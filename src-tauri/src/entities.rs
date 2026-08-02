use crate::models::{
    CreateEntityRequest, DashboardSummary, Entity, EntityPage, EntityType, ListEntitiesRequest,
    SearchEntitiesRequest, UpdateEntityRequest,
};
use rusqlite::types::Type;
use rusqlite::{params, params_from_iter, Connection, OptionalExtension, Row};
use std::collections::BTreeSet;
use uuid::Uuid;

const DEFAULT_PAGE_LIMIT: u32 = 50;
const MAX_PAGE_LIMIT: u32 = 200;

pub fn create_entity(
    connection: &mut Connection,
    request: CreateEntityRequest,
) -> Result<Entity, String> {
    let title = required_title(&request.title)?;
    let summary = optional_trimmed(request.summary.as_deref());
    let content = optional_trimmed(request.content.as_deref());
    let tags = normalize_tags(&request.tags);
    let id = Uuid::new_v4().to_string();

    let transaction = connection
        .transaction()
        .map_err(|error| format!("Failed to start database transaction: {error}"))?;

    transaction
        .execute(
            "INSERT INTO entities (id, type, title, summary) VALUES (?1, ?2, ?3, ?4)",
            params![id, request.entity_type.as_str(), title, summary],
        )
        .map_err(|error| format!("Failed to create entity: {error}"))?;

    if content.is_some() {
        transaction
            .execute(
                "INSERT INTO entity_contents (entity_id, body, format) VALUES (?1, ?2, 'markdown')",
                params![id, content],
            )
            .map_err(|error| format!("Failed to create entity content: {error}"))?;
    }

    replace_tags(&transaction, &id, &tags)?;
    insert_search_index(
        &transaction,
        &id,
        title,
        summary.as_deref(),
        content.as_deref(),
        &tags,
    )?;

    transaction
        .commit()
        .map_err(|error| format!("Failed to commit entity: {error}"))?;

    load_entity(connection, &id)
}

pub fn update_entity(
    connection: &mut Connection,
    request: UpdateEntityRequest,
) -> Result<Entity, String> {
    let title = required_title(&request.title)?;
    let summary = optional_trimmed(request.summary.as_deref());
    let content = optional_trimmed(request.content.as_deref());
    let tags = normalize_tags(&request.tags);

    let transaction = connection
        .transaction()
        .map_err(|error| format!("Failed to start database transaction: {error}"))?;

    let rows = transaction
        .execute(
            "UPDATE entities SET title = ?1, summary = ?2, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?3 AND archived_at IS NULL",
            params![title, summary, request.id],
        )
        .map_err(|error| format!("Failed to update entity: {error}"))?;

    if rows == 0 {
        return Err(format!("Entity not found: {}", request.id));
    }

    if content.is_some() {
        transaction
            .execute(
                "INSERT OR REPLACE INTO entity_contents (entity_id, body, format, updated_at) VALUES (?1, ?2, 'markdown', strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))",
                params![request.id, content],
            )
            .map_err(|error| format!("Failed to update entity content: {error}"))?;
    } else {
        transaction
            .execute(
                "DELETE FROM entity_contents WHERE entity_id = ?1",
                params![request.id],
            )
            .map_err(|error| format!("Failed to remove entity content: {error}"))?;
    }

    replace_tags(&transaction, &request.id, &tags)?;
    transaction
        .execute(
            "DELETE FROM search_index WHERE entity_id = ?1",
            params![request.id],
        )
        .map_err(|error| format!("Failed to clear search index: {error}"))?;
    insert_search_index(
        &transaction,
        &request.id,
        title,
        summary.as_deref(),
        content.as_deref(),
        &tags,
    )?;

    transaction
        .commit()
        .map_err(|error| format!("Failed to commit entity update: {error}"))?;

    load_entity(connection, &request.id)
}

pub fn archive_entity(connection: &Connection, id: &str) -> Result<(), String> {
    let rows = connection
        .execute(
            "UPDATE entities SET archived_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1 AND archived_at IS NULL",
            params![id],
        )
        .map_err(|error| format!("Failed to archive entity: {error}"))?;

    if rows == 0 {
        return Err(format!("Entity not found or already archived: {id}"));
    }

    Ok(())
}

pub fn restore_entity(connection: &Connection, id: &str) -> Result<(), String> {
    let rows = connection
        .execute(
            "UPDATE entities SET archived_at = NULL, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1 AND archived_at IS NOT NULL",
            params![id],
        )
        .map_err(|error| format!("Failed to restore entity: {error}"))?;

    if rows == 0 {
        return Err(format!("Entity not found or not archived: {id}"));
    }

    Ok(())
}

pub fn list_entities(
    connection: &Connection,
    request: ListEntitiesRequest,
) -> Result<EntityPage, String> {
    let (limit, offset) = pagination(request.limit, request.offset);
    let mut count_sql = String::from("SELECT COUNT(*) FROM entities e WHERE ");
    append_archive_filter(&mut count_sql, request.archived);

    let mut count_params = Vec::new();
    append_entity_filters(
        &mut count_sql,
        &mut count_params,
        request.entity_type,
        request.tag.as_deref(),
    );
    let total = query_count(connection, &count_sql, count_params)?;

    let mut sql = entity_select_sql(&format!(
        r#"
        FROM entities e
        LEFT JOIN entity_contents c ON c.entity_id = e.id
        LEFT JOIN entity_tags et ON et.entity_id = e.id
        LEFT JOIN tags t ON t.id = et.tag_id
        WHERE {}
        "#,
        archive_condition(request.archived)
    ));

    let mut params = Vec::new();
    append_entity_filters(
        &mut sql,
        &mut params,
        request.entity_type,
        request.tag.as_deref(),
    );
    sql.push_str(" GROUP BY e.id ORDER BY e.updated_at DESC LIMIT ? OFFSET ?");
    params.push(limit.to_string());
    params.push(offset.to_string());

    Ok(EntityPage {
        items: query_entities(connection, &sql, params)?,
        total,
    })
}

pub fn search_entities(
    connection: &Connection,
    request: SearchEntitiesRequest,
) -> Result<EntityPage, String> {
    let query = request.query.trim().to_owned();
    if query.is_empty() {
        return list_entities(
            connection,
            ListEntitiesRequest {
                entity_type: request.entity_type,
                tag: request.tag,
                archived: request.archived,
                limit: request.limit,
                offset: request.offset,
            },
        );
    }

    let (limit, offset) = pagination(request.limit, request.offset);
    let fts_query = build_fts_query(&query, request.search_mode.operator());

    let mut count_sql = format!(
        r#"
        SELECT COUNT(DISTINCT e.id)
        FROM search_index si
        JOIN entities e ON e.id = si.entity_id
        WHERE search_index MATCH ?
          AND {}
        "#,
        archive_condition(request.archived)
    );
    let mut count_params = vec![fts_query.clone()];
    append_entity_filters(
        &mut count_sql,
        &mut count_params,
        request.entity_type,
        request.tag.as_deref(),
    );
    let total = query_count(connection, &count_sql, count_params)?;

    let mut sql = entity_select_sql(&format!(
        r#"
        FROM search_index si
        JOIN entities e ON e.id = si.entity_id
        LEFT JOIN entity_contents c ON c.entity_id = e.id
        LEFT JOIN entity_tags et ON et.entity_id = e.id
        LEFT JOIN tags t ON t.id = et.tag_id
        WHERE search_index MATCH ?
          AND {}
        "#,
        archive_condition(request.archived)
    ));

    let mut params = vec![fts_query];
    append_entity_filters(
        &mut sql,
        &mut params,
        request.entity_type,
        request.tag.as_deref(),
    );
    sql.push_str(" GROUP BY e.id ORDER BY rank LIMIT ? OFFSET ?");
    params.push(limit.to_string());
    params.push(offset.to_string());

    Ok(EntityPage {
        items: query_entities(connection, &sql, params)?,
        total,
    })
}

pub fn list_tags(connection: &Connection) -> Result<Vec<String>, String> {
    let mut statement = connection
        .prepare(
            r#"
            SELECT DISTINCT t.name
            FROM tags t
            JOIN entity_tags et ON et.tag_id = t.id
            JOIN entities e ON e.id = et.entity_id
            WHERE e.archived_at IS NULL
            ORDER BY t.name
            "#,
        )
        .map_err(|error| format!("Failed to prepare tag query: {error}"))?;

    let tags = statement
        .query_map([], |row| row.get(0))
        .map_err(|error| format!("Failed to list tags: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Failed to read tags: {error}"))?;

    Ok(tags)
}

pub fn dashboard_summary(connection: &Connection) -> Result<DashboardSummary, String> {
    let count_type = |entity_type: EntityType| -> Result<i64, String> {
        connection
            .query_row(
                "SELECT COUNT(*) FROM entities WHERE type = ?1 AND archived_at IS NULL",
                params![entity_type.as_str()],
                |row| row.get(0),
            )
            .map_err(|error| format!("Failed to count {entity_type}: {error}"))
    };

    let reminders_due_today = connection
        .query_row(
            r#"
            SELECT COUNT(*)
            FROM reminders r
            JOIN entities e ON e.id = r.entity_id
            WHERE r.triggered_at IS NULL
              AND e.archived_at IS NULL
              AND date(r.remind_at, 'localtime') <= date('now', 'localtime')
            "#,
            [],
            |row| row.get(0),
        )
        .map_err(|error| format!("Failed to count reminders: {error}"))?;

    Ok(DashboardSummary {
        notes: count_type(EntityType::Note)?,
        tasks: count_type(EntityType::Task)?,
        events: count_type(EntityType::Event)?,
        knowledge: count_type(EntityType::Knowledge)?,
        files: count_type(EntityType::File)?,
        reminders_due_today,
    })
}

fn load_entity(connection: &Connection, id: &str) -> Result<Entity, String> {
    connection
        .query_row(
            &format!("{} FROM entities e LEFT JOIN entity_contents c ON c.entity_id = e.id LEFT JOIN entity_tags et ON et.entity_id = e.id LEFT JOIN tags t ON t.id = et.tag_id WHERE e.id = ?1 GROUP BY e.id", entity_select_columns()),
            params![id],
            map_entity_row,
        )
        .optional()
        .map_err(|error| format!("Failed to load entity: {error}"))?
        .ok_or_else(|| format!("Entity not found: {id}"))
}

fn query_entities(
    connection: &Connection,
    sql: &str,
    params: Vec<String>,
) -> Result<Vec<Entity>, String> {
    let mut statement = connection
        .prepare(sql)
        .map_err(|error| format!("Failed to prepare entity query: {error}"))?;

    let entities = statement
        .query_map(params_from_iter(params.iter()), map_entity_row)
        .map_err(|error| format!("Failed to list entities: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Failed to read entities: {error}"))?;

    Ok(entities)
}

fn query_count(
    connection: &Connection,
    sql: &str,
    params: Vec<String>,
) -> Result<i64, String> {
    connection
        .query_row(sql, params_from_iter(params.iter()), |row| row.get(0))
        .map_err(|error| format!("Failed to count entities: {error}"))
}

fn map_entity_row(row: &Row<'_>) -> rusqlite::Result<Entity> {
    let entity_type_value: String = row.get(1)?;
    let entity_type = EntityType::from_db_value(&entity_type_value).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(
            1,
            Type::Text,
            Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, error)),
        )
    })?;

    Ok(Entity {
        id: row.get(0)?,
        entity_type,
        title: row.get(2)?,
        summary: row.get(3)?,
        content: row.get(4)?,
        tags: parse_tags(row.get(5)?),
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

fn entity_select_columns() -> &'static str {
    r#"
        SELECT
            e.id,
            e.type,
            e.title,
            e.summary,
            c.body,
            COALESCE(group_concat(t.name, ','), '') AS tags,
            e.created_at,
            e.updated_at
    "#
}

fn entity_select_sql(from_clause: &str) -> String {
    format!("{} {}", entity_select_columns(), from_clause)
}

fn archive_condition(archived: bool) -> &'static str {
    if archived {
        "e.archived_at IS NOT NULL"
    } else {
        "e.archived_at IS NULL"
    }
}

fn append_archive_filter(sql: &mut String, archived: bool) {
    sql.push_str(archive_condition(archived));
}

fn append_entity_filters(
    sql: &mut String,
    params: &mut Vec<String>,
    entity_type: Option<EntityType>,
    tag: Option<&str>,
) {
    if let Some(entity_type) = entity_type {
        sql.push_str(" AND e.type = ?");
        params.push(entity_type.as_str().to_owned());
    }

    if let Some(tag) = tag {
        if let Some(tag) = normalize_filter_tag(tag) {
            sql.push_str(
                " AND e.id IN (SELECT et2.entity_id FROM entity_tags et2 JOIN tags t2 ON t2.id = et2.tag_id WHERE t2.name = ?)",
            );
            params.push(tag);
        }
    }
}

fn pagination(limit: Option<u32>, offset: Option<u32>) -> (u32, u32) {
    (
        limit.unwrap_or(DEFAULT_PAGE_LIMIT).clamp(1, MAX_PAGE_LIMIT),
        offset.unwrap_or(0),
    )
}

fn replace_tags(connection: &Connection, entity_id: &str, tags: &[String]) -> Result<(), String> {
    connection
        .execute(
            "DELETE FROM entity_tags WHERE entity_id = ?1",
            params![entity_id],
        )
        .map_err(|error| format!("Failed to clear tags: {error}"))?;

    for tag in tags {
        let tag_id = Uuid::new_v4().to_string();
        connection
            .execute(
                "INSERT OR IGNORE INTO tags (id, name) VALUES (?1, ?2)",
                params![tag_id, tag],
            )
            .map_err(|error| format!("Failed to create tag: {error}"))?;

        let existing_tag_id: String = connection
            .query_row("SELECT id FROM tags WHERE name = ?1", params![tag], |row| {
                row.get(0)
            })
            .map_err(|error| format!("Failed to load tag: {error}"))?;

        connection
            .execute(
                "INSERT OR IGNORE INTO entity_tags (entity_id, tag_id) VALUES (?1, ?2)",
                params![entity_id, existing_tag_id],
            )
            .map_err(|error| format!("Failed to link tag: {error}"))?;
    }

    Ok(())
}

fn insert_search_index(
    connection: &Connection,
    entity_id: &str,
    title: &str,
    summary: Option<&str>,
    content: Option<&str>,
    tags: &[String],
) -> Result<(), String> {
    connection
        .execute(
            "INSERT INTO search_index (entity_id, title, summary, content, tags) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![entity_id, title, summary, content, tags.join(" ")],
        )
        .map_err(|error| format!("Failed to update search index: {error}"))?;

    Ok(())
}

fn required_title(title: &str) -> Result<&str, String> {
    let title = title.trim();
    if title.is_empty() {
        return Err("Title is required".to_owned());
    }

    Ok(title)
}

fn optional_trimmed(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn normalize_tags(tags: &[String]) -> Vec<String> {
    tags.iter()
        .filter_map(|tag| normalize_filter_tag(tag))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn normalize_filter_tag(tag: &str) -> Option<String> {
    let tag = tag.trim().to_lowercase();
    if tag.is_empty() {
        None
    } else {
        Some(tag)
    }
}

fn parse_tags(tags: Option<String>) -> Vec<String> {
    let mut tags = tags
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|tag| !tag.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    tags.sort();
    tags
}

fn build_fts_query(query: &str, operator: &str) -> String {
    query
        .split_whitespace()
        .map(|token| format!("\"{}\"*", token.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(operator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::run_migrations;
    use crate::models::SearchMode;
    use serde_json::json;
    use std::collections::BTreeSet;

    fn test_connection() -> Connection {
        let connection = Connection::open_in_memory().expect("open in-memory database");
        run_migrations(&connection).expect("run migrations");
        connection
    }

    fn create_request(
        entity_type: EntityType,
        title: &str,
        content: &str,
        tags: &[&str],
    ) -> CreateEntityRequest {
        CreateEntityRequest {
            entity_type,
            title: title.to_owned(),
            summary: Some(content.to_owned()),
            content: Some(content.to_owned()),
            tags: tags.iter().map(|tag| (*tag).to_owned()).collect(),
        }
    }

    fn update_request(id: &str, title: &str, content: &str, tags: &[&str]) -> UpdateEntityRequest {
        UpdateEntityRequest {
            id: id.to_owned(),
            title: title.to_owned(),
            summary: Some(content.to_owned()),
            content: Some(content.to_owned()),
            tags: tags.iter().map(|tag| (*tag).to_owned()).collect(),
        }
    }

    fn search_request(query: &str, search_mode: SearchMode) -> SearchEntitiesRequest {
        SearchEntitiesRequest {
            query: query.to_owned(),
            search_mode,
            entity_type: None,
            tag: None,
            archived: false,
            limit: None,
            offset: None,
        }
    }

    fn titles(page: EntityPage) -> BTreeSet<String> {
        page.items
            .into_iter()
            .map(|entity| entity.title)
            .collect()
    }

    #[test]
    fn creates_updates_archives_and_restores_entities() {
        let mut connection = test_connection();

        let created = create_entity(
            &mut connection,
            create_request(EntityType::Note, "  First note  ", "Body", &["Work"]),
        )
        .expect("create entity");
        assert_eq!(created.title, "First note");
        assert_eq!(created.entity_type, EntityType::Note);
        assert_eq!(created.tags, vec!["work"]);

        let listed =
            list_entities(&connection, ListEntitiesRequest::default()).expect("list entities");
        assert_eq!(listed.total, 1);
        assert_eq!(listed.items.len(), 1);

        let updated = update_entity(
            &mut connection,
            update_request(&created.id, "Updated note", "Updated body", &["Home"]),
        )
        .expect("update entity");
        assert_eq!(updated.title, "Updated note");
        assert_eq!(updated.content.as_deref(), Some("Updated body"));
        assert_eq!(updated.tags, vec!["home"]);

        archive_entity(&connection, &created.id).expect("archive entity");
        let visible =
            list_entities(&connection, ListEntitiesRequest::default()).expect("list visible");
        assert_eq!(visible.total, 0);

        let archived = list_entities(
            &connection,
            ListEntitiesRequest {
                archived: true,
                ..Default::default()
            },
        )
        .expect("list archived");
        assert_eq!(archived.total, 1);
        assert_eq!(archived.items[0].id, created.id);

        restore_entity(&connection, &created.id).expect("restore entity");
        let restored =
            list_entities(&connection, ListEntitiesRequest::default()).expect("list restored");
        assert_eq!(restored.total, 1);
        assert!(restore_entity(&connection, &created.id).is_err());
    }

    #[test]
    fn rejects_blank_titles_and_invalid_entity_types() {
        let mut connection = test_connection();

        let blank_title = create_entity(
            &mut connection,
            create_request(EntityType::Task, "   ", "Body", &[]),
        );
        assert_eq!(
            blank_title.expect_err("blank title error"),
            "Title is required"
        );

        let invalid_type = serde_json::from_value::<CreateEntityRequest>(json!({
            "entityType": "unknown",
            "title": "Bad type",
            "summary": null,
            "content": null,
            "tags": []
        }));
        assert!(invalid_type.is_err());
    }

    #[test]
    fn normalizes_and_filters_tags_and_types() {
        let mut connection = test_connection();

        let note = create_entity(
            &mut connection,
            create_request(
                EntityType::Note,
                "Tagged note",
                "Alpha body",
                &[" Work ", "work", "Home"],
            ),
        )
        .expect("create note");
        create_entity(
            &mut connection,
            create_request(EntityType::Task, "Tagged task", "Beta body", &["work"]),
        )
        .expect("create task");

        assert_eq!(note.tags, vec!["home", "work"]);
        assert_eq!(
            list_tags(&connection).expect("list tags"),
            vec!["home", "work"]
        );

        let work_entities = list_entities(
            &connection,
            ListEntitiesRequest {
                tag: Some("WORK".to_owned()),
                ..Default::default()
            },
        )
        .expect("filter by tag");
        assert_eq!(work_entities.total, 2);

        let note_entities = list_entities(
            &connection,
            ListEntitiesRequest {
                entity_type: Some(EntityType::Note),
                tag: Some("work".to_owned()),
                ..Default::default()
            },
        )
        .expect("filter by type and tag");
        assert_eq!(note_entities.total, 1);
        assert_eq!(note_entities.items[0].title, "Tagged note");

        archive_entity(&connection, &note.id).expect("archive tagged note");
        assert_eq!(list_tags(&connection).expect("active tags"), vec!["work"]);
    }

    #[test]
    fn paginates_entities_and_reports_total() {
        let mut connection = test_connection();
        for title in ["First", "Second", "Third"] {
            create_entity(
                &mut connection,
                create_request(EntityType::Note, title, "Body", &[]),
            )
            .expect("create entity");
        }

        let first_page = list_entities(
            &connection,
            ListEntitiesRequest {
                limit: Some(2),
                offset: Some(0),
                ..Default::default()
            },
        )
        .expect("first page");
        let second_page = list_entities(
            &connection,
            ListEntitiesRequest {
                limit: Some(2),
                offset: Some(2),
                ..Default::default()
            },
        )
        .expect("second page");

        assert_eq!(first_page.total, 3);
        assert_eq!(first_page.items.len(), 2);
        assert_eq!(second_page.total, 3);
        assert_eq!(second_page.items.len(), 1);
        assert_ne!(first_page.items[0].id, second_page.items[0].id);
        assert_ne!(first_page.items[1].id, second_page.items[0].id);
    }

    #[test]
    fn searches_with_and_or_modes_and_filters() {
        let mut connection = test_connection();

        create_entity(
            &mut connection,
            create_request(
                EntityType::Note,
                "Alpha beta note",
                "Shared words",
                &["project"],
            ),
        )
        .expect("create note");
        create_entity(
            &mut connection,
            create_request(EntityType::Task, "Alpha task", "Gamma words", &["work"]),
        )
        .expect("create task");
        create_entity(
            &mut connection,
            create_request(
                EntityType::Knowledge,
                "Beta guide",
                "Reference",
                &["project"],
            ),
        )
        .expect("create knowledge");

        let and_results = search_entities(
            &connection,
            search_request("alpha beta", SearchMode::And),
        )
        .expect("and search");
        assert_eq!(and_results.total, 1);
        assert_eq!(
            titles(and_results),
            BTreeSet::from(["Alpha beta note".to_owned()])
        );

        let or_results =
            search_entities(&connection, search_request("alpha beta", SearchMode::Or))
                .expect("or search");
        assert_eq!(or_results.total, 3);
        assert_eq!(
            titles(or_results),
            BTreeSet::from([
                "Alpha beta note".to_owned(),
                "Alpha task".to_owned(),
                "Beta guide".to_owned(),
            ])
        );

        let mut filtered_by_type_request = search_request("alpha", SearchMode::And);
        filtered_by_type_request.entity_type = Some(EntityType::Task);
        let filtered_by_type =
            search_entities(&connection, filtered_by_type_request).expect("type filtered search");
        assert_eq!(
            titles(filtered_by_type),
            BTreeSet::from(["Alpha task".to_owned()])
        );

        let mut filtered_by_tag_request = search_request("beta", SearchMode::And);
        filtered_by_tag_request.tag = Some("project".to_owned());
        let filtered_by_tag =
            search_entities(&connection, filtered_by_tag_request).expect("tag filtered search");
        assert_eq!(
            titles(filtered_by_tag),
            BTreeSet::from(["Alpha beta note".to_owned(), "Beta guide".to_owned()])
        );
    }

    #[test]
    fn empty_search_uses_list_filters_and_paging() {
        let mut connection = test_connection();

        create_entity(
            &mut connection,
            create_request(EntityType::Note, "Visible note", "Body", &["work"]),
        )
        .expect("create note");
        create_entity(
            &mut connection,
            create_request(EntityType::Task, "Hidden task", "Body", &["work"]),
        )
        .expect("create task");

        let results = search_entities(
            &connection,
            SearchEntitiesRequest {
                query: "   ".to_owned(),
                search_mode: SearchMode::And,
                entity_type: Some(EntityType::Note),
                tag: Some("work".to_owned()),
                archived: false,
                limit: Some(1),
                offset: Some(0),
            },
        )
        .expect("empty search");

        assert_eq!(results.total, 1);
        assert_eq!(
            titles(results),
            BTreeSet::from(["Visible note".to_owned()])
        );
    }
}
