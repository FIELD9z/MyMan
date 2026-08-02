use crate::models::TagSummary;
use rusqlite::{params, Connection, OptionalExtension};

pub fn list_tag_summaries(connection: &Connection) -> Result<Vec<TagSummary>, String> {
    let mut statement = connection
        .prepare(
            r#"
            SELECT
                t.name,
                COALESCE(SUM(CASE WHEN e.id IS NOT NULL AND e.archived_at IS NULL THEN 1 ELSE 0 END), 0) AS active_count,
                COALESCE(SUM(CASE WHEN e.id IS NOT NULL AND e.archived_at IS NOT NULL THEN 1 ELSE 0 END), 0) AS archived_count
            FROM tags t
            LEFT JOIN entity_tags et ON et.tag_id = t.id
            LEFT JOIN entities e ON e.id = et.entity_id
            GROUP BY t.id, t.name
            ORDER BY t.name
            "#,
        )
        .map_err(|error| format!("Failed to prepare tag summary query: {error}"))?;

    statement
        .query_map([], |row| {
            Ok(TagSummary {
                name: row.get(0)?,
                active_count: row.get(1)?,
                archived_count: row.get(2)?,
            })
        })
        .map_err(|error| format!("Failed to list tag summaries: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Failed to read tag summaries: {error}"))
}

pub fn rename_tag(connection: &Connection, old_name: &str, new_name: &str) -> Result<(), String> {
    let old_name = normalize(old_name)?;
    let new_name = normalize(new_name)?;

    if old_name == new_name {
        return Err("New tag name must be different".to_owned());
    }

    let transaction = connection
        .unchecked_transaction()
        .map_err(|error| format!("Failed to start tag rename transaction: {error}"))?;

    let existing_target = transaction
        .query_row(
            "SELECT id FROM tags WHERE name = ?1 COLLATE NOCASE",
            params![new_name],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| format!("Failed to check target tag: {error}"))?;

    if existing_target.is_some() {
        return Err(format!(
            "Tag already exists: {new_name}. Use merge instead."
        ));
    }

    let changed = transaction
        .execute(
            "UPDATE tags SET name = ?1 WHERE name = ?2 COLLATE NOCASE",
            params![new_name, old_name],
        )
        .map_err(|error| format!("Failed to rename tag: {error}"))?;

    if changed == 0 {
        return Err(format!("Tag not found: {old_name}"));
    }

    refresh_search_index_tags(&transaction)?;
    transaction
        .commit()
        .map_err(|error| format!("Failed to commit tag rename: {error}"))
}

pub fn merge_tags(connection: &Connection, source: &str, target: &str) -> Result<(), String> {
    let source = normalize(source)?;
    let target = normalize(target)?;

    if source == target {
        return Err("Source and target tags must differ".to_owned());
    }

    let transaction = connection
        .unchecked_transaction()
        .map_err(|error| format!("Failed to start tag merge transaction: {error}"))?;

    let source_id = transaction
        .query_row(
            "SELECT id FROM tags WHERE name = ?1 COLLATE NOCASE",
            params![source],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| format!("Failed to find source tag: {error}"))?
        .ok_or_else(|| format!("Tag not found: {source}"))?;

    let target_id = transaction
        .query_row(
            "SELECT id FROM tags WHERE name = ?1 COLLATE NOCASE",
            params![target],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| format!("Failed to find target tag: {error}"))?;

    if let Some(target_id) = target_id {
        transaction
            .execute(
                "INSERT OR IGNORE INTO entity_tags (entity_id, tag_id) SELECT entity_id, ?1 FROM entity_tags WHERE tag_id = ?2",
                params![target_id, source_id],
            )
            .map_err(|error| format!("Failed to move tag links: {error}"))?;
        transaction
            .execute(
                "DELETE FROM entity_tags WHERE tag_id = ?1",
                params![source_id],
            )
            .map_err(|error| format!("Failed to remove old tag links: {error}"))?;
        transaction
            .execute("DELETE FROM tags WHERE id = ?1", params![source_id])
            .map_err(|error| format!("Failed to remove source tag: {error}"))?;
    } else {
        transaction
            .execute(
                "UPDATE tags SET name = ?1 WHERE id = ?2",
                params![target, source_id],
            )
            .map_err(|error| format!("Failed to rename source tag: {error}"))?;
    }

    refresh_search_index_tags(&transaction)?;
    transaction
        .commit()
        .map_err(|error| format!("Failed to commit tag merge: {error}"))
}

pub fn cleanup_unused_tags(connection: &Connection) -> Result<u64, String> {
    connection
        .execute(
            "DELETE FROM tags WHERE NOT EXISTS (SELECT 1 FROM entity_tags et WHERE et.tag_id = tags.id)",
            [],
        )
        .map(|count| count as u64)
        .map_err(|error| format!("Failed to cleanup tags: {error}"))
}

fn refresh_search_index_tags(connection: &Connection) -> Result<(), String> {
    let entity_ids = {
        let mut statement = connection
            .prepare("SELECT entity_id FROM search_index")
            .map_err(|error| format!("Failed to prepare search index refresh: {error}"))?;
        statement
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|error| format!("Failed to query indexed entities: {error}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("Failed to read indexed entities: {error}"))?
    };

    for entity_id in entity_ids {
        let tags = connection
            .query_row(
                r#"
                SELECT COALESCE(group_concat(t.name, ' '), '')
                FROM entity_tags et
                JOIN tags t ON t.id = et.tag_id
                WHERE et.entity_id = ?1
                "#,
                params![entity_id],
                |row| row.get::<_, String>(0),
            )
            .map_err(|error| format!("Failed to rebuild indexed tags: {error}"))?;
        connection
            .execute(
                "UPDATE search_index SET tags = ?1 WHERE entity_id = ?2",
                params![tags, entity_id],
            )
            .map_err(|error| format!("Failed to update indexed tags: {error}"))?;
    }

    Ok(())
}

fn normalize(value: &str) -> Result<String, String> {
    let value = value.trim().to_lowercase();
    if value.is_empty() {
        Err("Tag name cannot be empty".to_owned())
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::run_migrations;
    use crate::entities::{create_entity, list_entities, search_entities};
    use crate::models::{
        CreateEntityRequest, EntityType, ListEntitiesRequest, SearchEntitiesRequest, SearchMode,
    };
    use uuid::Uuid;

    fn test_connection() -> Connection {
        let connection = Connection::open_in_memory().expect("open in-memory database");
        run_migrations(&connection).expect("run migrations");
        connection
    }

    fn create_tagged_entity(
        connection: &mut Connection,
        title: &str,
        tags: &[&str],
    ) -> String {
        create_entity(
            connection,
            CreateEntityRequest {
                entity_type: EntityType::Note,
                title: title.to_owned(),
                summary: None,
                content: None,
                tags: tags.iter().map(|tag| (*tag).to_owned()).collect(),
            },
        )
        .expect("create tagged entity")
        .id
    }

    fn search_by_tag_text(connection: &Connection, query: &str) -> i64 {
        search_entities(
            connection,
            SearchEntitiesRequest {
                query: query.to_owned(),
                search_mode: SearchMode::And,
                entity_type: None,
                tag: None,
                archived: false,
                limit: None,
                offset: None,
            },
        )
        .expect("search entities")
        .total
    }

    #[test]
    fn renames_tag_and_refreshes_entity_and_search_results() {
        let mut connection = test_connection();
        create_tagged_entity(&mut connection, "Tagged item", &["oldlabel"]);

        rename_tag(&connection, "oldlabel", "newlabel").expect("rename tag");

        let page = list_entities(
            &connection,
            ListEntitiesRequest {
                tag: Some("newlabel".to_owned()),
                ..Default::default()
            },
        )
        .expect("filter renamed tag");
        assert_eq!(page.total, 1);
        assert_eq!(page.items[0].tags, vec!["newlabel"]);
        assert_eq!(search_by_tag_text(&connection, "oldlabel"), 0);
        assert_eq!(search_by_tag_text(&connection, "newlabel"), 1);
        assert!(rename_tag(&connection, "newlabel", "newlabel").is_err());
    }

    #[test]
    fn merges_tag_links_without_duplicates_and_refreshes_search() {
        let mut connection = test_connection();
        create_tagged_entity(&mut connection, "Source only", &["sourcelabel"]);
        create_tagged_entity(&mut connection, "Target only", &["targetlabel"]);
        let both_id = create_tagged_entity(
            &mut connection,
            "Both labels",
            &["sourcelabel", "targetlabel"],
        );

        merge_tags(&connection, "sourcelabel", "targetlabel").expect("merge tags");

        let page = list_entities(
            &connection,
            ListEntitiesRequest {
                tag: Some("targetlabel".to_owned()),
                ..Default::default()
            },
        )
        .expect("filter merged tag");
        assert_eq!(page.total, 3);
        assert_eq!(search_by_tag_text(&connection, "sourcelabel"), 0);
        assert_eq!(search_by_tag_text(&connection, "targetlabel"), 3);

        let duplicate_count: i64 = connection
            .query_row(
                r#"
                SELECT COUNT(*)
                FROM entity_tags et
                JOIN tags t ON t.id = et.tag_id
                WHERE et.entity_id = ?1 AND t.name = 'targetlabel'
                "#,
                params![both_id],
                |row| row.get(0),
            )
            .expect("count merged links");
        assert_eq!(duplicate_count, 1);
        assert!(merge_tags(&connection, "missing", "targetlabel").is_err());
    }

    #[test]
    fn lists_usage_counts_and_removes_only_unused_tags() {
        let mut connection = test_connection();
        let active_id = create_tagged_entity(&mut connection, "Active", &["used"]);
        let archived_id = create_tagged_entity(&mut connection, "Archived", &["used"]);
        connection
            .execute(
                "UPDATE entities SET archived_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
                params![archived_id],
            )
            .expect("archive entity directly");
        connection
            .execute(
                "INSERT INTO tags (id, name) VALUES (?1, 'orphan')",
                params![Uuid::new_v4().to_string()],
            )
            .expect("insert orphan tag");

        let summaries = list_tag_summaries(&connection).expect("list tag summaries");
        let used = summaries
            .iter()
            .find(|tag| tag.name == "used")
            .expect("used summary");
        assert_eq!(used.active_count, 1);
        assert_eq!(used.archived_count, 1);
        let orphan = summaries
            .iter()
            .find(|tag| tag.name == "orphan")
            .expect("orphan summary");
        assert_eq!(orphan.active_count, 0);
        assert_eq!(orphan.archived_count, 0);

        assert_eq!(cleanup_unused_tags(&connection).expect("cleanup tags"), 1);
        let summaries = list_tag_summaries(&connection).expect("list summaries after cleanup");
        assert!(!summaries.iter().any(|tag| tag.name == "orphan"));
        assert!(summaries.iter().any(|tag| tag.name == "used"));

        let active_exists: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM entities WHERE id = ?1",
                params![active_id],
                |row| row.get(0),
            )
            .expect("check active entity");
        assert_eq!(active_exists, 1);
    }
}
