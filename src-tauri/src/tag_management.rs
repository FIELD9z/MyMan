use rusqlite::{params, Connection};

pub fn rename_tag(connection: &Connection, old_name: &str, new_name: &str) -> Result<(), String> {
    let old_name = normalize(old_name)?;
    let new_name = normalize(new_name)?;

    if old_name == new_name {
        return Err("New tag name must be different".to_owned());
    }

    let transaction = connection
        .unchecked_transaction()
        .map_err(|error| format!("Failed to start transaction: {error}"))?;

    let changed = transaction
        .execute(
            "UPDATE tags SET name = ?1 WHERE name = ?2 COLLATE NOCASE",
            params![new_name, old_name],
        )
        .map_err(|error| format!("Failed to rename tag: {error}"))?;

    if changed == 0 {
        return Err(format!("Tag not found: {old_name}"));
    }

    transaction
        .commit()
        .map_err(|error| format!("Failed to commit rename: {error}"))
}

pub fn merge_tags(connection: &Connection, source: &str, target: &str) -> Result<(), String> {
    let source = normalize(source)?;
    let target = normalize(target)?;

    if source == target {
        return Err("Source and target tags must differ".to_owned());
    }

    let transaction = connection
        .unchecked_transaction()
        .map_err(|error| format!("Failed to start transaction: {error}"))?;

    let target_id: Option<String> = transaction
        .query_row(
            "SELECT id FROM tags WHERE name = ?1 COLLATE NOCASE",
            params![target],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| format!("Failed to find target tag: {error}"))?;

    let source_id: String = transaction
        .query_row(
            "SELECT id FROM tags WHERE name = ?1 COLLATE NOCASE",
            params![source],
            |row| row.get(0),
        )
        .map_err(|error| format!("Failed to find source tag: {error}"))?;

    let target_id = match target_id {
        Some(id) => id,
        None => {
            transaction
                .execute(
                    "UPDATE tags SET name = ?1 WHERE id = ?2",
                    params![target, source_id],
                )
                .map_err(|error| format!("Failed to rename source tag: {error}"))?;
            transaction.commit().map_err(|error| error.to_string())?;
            return Ok(());
        }
    };

    transaction
        .execute(
            "INSERT OR IGNORE INTO entity_tags(entity_id, tag_id) SELECT entity_id, ?1 FROM entity_tags WHERE tag_id = ?2",
            params![target_id, source_id],
        )
        .map_err(|error| format!("Failed to move tag links: {error}"))?;

    transaction
        .execute("DELETE FROM entity_tags WHERE tag_id = ?1", params![source_id])
        .map_err(|error| format!("Failed to remove old links: {error}"))?;

    transaction
        .execute("DELETE FROM tags WHERE id = ?1", params![source_id])
        .map_err(|error| format!("Failed to remove source tag: {error}"))?;

    transaction.commit().map_err(|error| error.to_string())
}

pub fn cleanup_unused_tags(connection: &Connection) -> Result<u64, String> {
    connection
        .execute(
            "DELETE FROM tags WHERE id NOT IN (SELECT DISTINCT tag_id FROM entity_tags)",
            [],
        )
        .map(|count| count as u64)
        .map_err(|error| format!("Failed to cleanup tags: {error}"))
}

fn normalize(value: &str) -> Result<String, String> {
    let value = value.trim().to_lowercase();
    if value.is_empty() {
        Err("Tag name cannot be empty".to_owned())
    } else {
        Ok(value)
    }
}
