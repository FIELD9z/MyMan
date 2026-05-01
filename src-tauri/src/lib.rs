use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tauri::{Manager, State};
use uuid::Uuid;

struct AppState {
    db: Mutex<Connection>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateEntityRequest {
    entity_type: String,
    title: String,
    summary: Option<String>,
    content: Option<String>,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateEntityRequest {
    id: String,
    title: String,
    summary: Option<String>,
    content: Option<String>,
    tags: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Entity {
    id: String,
    entity_type: String,
    title: String,
    summary: Option<String>,
    content: Option<String>,
    tags: Vec<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DashboardSummary {
    notes: i64,
    tasks: i64,
    events: i64,
    knowledge: i64,
    files: i64,
    reminders_due_today: i64,
}

fn setup_database(app: &tauri::App) -> Result<AppState, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Failed to resolve app data directory: {error}"))?;

    fs::create_dir_all(&data_dir)
        .map_err(|error| format!("Failed to create app data directory: {error}"))?;

    let db_path = data_dir.join("myman.sqlite3");
    let connection = Connection::open(db_path)
        .map_err(|error| format!("Failed to open SQLite database: {error}"))?;

    connection
        .execute_batch(include_str!("../migrations/0001_initial.sql"))
        .map_err(|error| format!("Failed to run database migrations: {error}"))?;

    Ok(AppState {
        db: Mutex::new(connection),
    })
}

fn parse_tags(tags: Option<String>) -> Vec<String> {
    tags.map(|value| {
        value
            .split(',')
            .map(str::trim)
            .filter(|tag| !tag.is_empty())
            .map(ToOwned::to_owned)
            .collect()
    })
    .unwrap_or_default()
}

fn load_entity(connection: &Connection, id: &str) -> Result<Entity, String> {
    connection
        .query_row(
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
            FROM entities e
            LEFT JOIN entity_contents c ON c.entity_id = e.id
            LEFT JOIN entity_tags et ON et.entity_id = e.id
            LEFT JOIN tags t ON t.id = et.tag_id
            WHERE e.id = ?1
            GROUP BY e.id
            "#,
            params![id],
            |row| {
                Ok(Entity {
                    id: row.get(0)?,
                    entity_type: row.get(1)?,
                    title: row.get(2)?,
                    summary: row.get(3)?,
                    content: row.get(4)?,
                    tags: parse_tags(row.get(5)?),
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        )
        .optional()
        .map_err(|error| format!("Failed to load entity: {error}"))?
        .ok_or_else(|| format!("Entity not found: {id}"))
}

#[tauri::command]
fn create_entity(state: State<'_, AppState>, request: CreateEntityRequest) -> Result<Entity, String> {
    let title = request.title.trim();
    if title.is_empty() {
        return Err("Title is required".to_owned());
    }

    let entity_type = request.entity_type.trim();
    if entity_type.is_empty() {
        return Err("Entity type is required".to_owned());
    }

    let id = Uuid::new_v4().to_string();
    let summary = request
        .summary
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);
    let content = request
        .content
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);
    let normalized_tags: Vec<String> = request
        .tags
        .iter()
        .map(|tag| tag.trim().to_lowercase())
        .filter(|tag| !tag.is_empty())
        .collect();

    let mut connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;
    let transaction = connection
        .transaction()
        .map_err(|error| format!("Failed to start database transaction: {error}"))?;

    transaction
        .execute(
            "INSERT INTO entities (id, type, title, summary) VALUES (?1, ?2, ?3, ?4)",
            params![id, entity_type, title, summary],
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

    for tag in &normalized_tags {
        let tag_id = Uuid::new_v4().to_string();
        transaction
            .execute(
                "INSERT OR IGNORE INTO tags (id, name) VALUES (?1, ?2)",
                params![tag_id, tag],
            )
            .map_err(|error| format!("Failed to create tag: {error}"))?;

        let existing_tag_id: String = transaction
            .query_row("SELECT id FROM tags WHERE name = ?1", params![tag], |row| row.get(0))
            .map_err(|error| format!("Failed to load tag: {error}"))?;

        transaction
            .execute(
                "INSERT OR IGNORE INTO entity_tags (entity_id, tag_id) VALUES (?1, ?2)",
                params![id, existing_tag_id],
            )
            .map_err(|error| format!("Failed to link tag: {error}"))?;
    }

    transaction
        .execute(
            r#"
            INSERT INTO search_index (entity_id, title, summary, content, tags)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![id, title, summary, content, normalized_tags.join(" ")],
        )
        .map_err(|error| format!("Failed to update search index: {error}"))?;

    transaction
        .commit()
        .map_err(|error| format!("Failed to commit entity: {error}"))?;

    load_entity(&connection, &id)
}

#[tauri::command]
fn update_entity(state: State<'_, AppState>, request: UpdateEntityRequest) -> Result<Entity, String> {
    let title = request.title.trim();
    if title.is_empty() {
        return Err("Title is required".to_owned());
    }

    let summary = request
        .summary
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);
    let content = request
        .content
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);
    let normalized_tags: Vec<String> = request
        .tags
        .iter()
        .map(|tag| tag.trim().to_lowercase())
        .filter(|tag| !tag.is_empty())
        .collect();

    let mut connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;
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

    transaction
        .execute("DELETE FROM entity_tags WHERE entity_id = ?1", params![request.id])
        .map_err(|error| format!("Failed to clear tags: {error}"))?;

    for tag in &normalized_tags {
        let tag_id = Uuid::new_v4().to_string();
        transaction
            .execute(
                "INSERT OR IGNORE INTO tags (id, name) VALUES (?1, ?2)",
                params![tag_id, tag],
            )
            .map_err(|error| format!("Failed to create tag: {error}"))?;

        let existing_tag_id: String = transaction
            .query_row("SELECT id FROM tags WHERE name = ?1", params![tag], |row| row.get(0))
            .map_err(|error| format!("Failed to load tag: {error}"))?;

        transaction
            .execute(
                "INSERT OR IGNORE INTO entity_tags (entity_id, tag_id) VALUES (?1, ?2)",
                params![request.id, existing_tag_id],
            )
            .map_err(|error| format!("Failed to link tag: {error}"))?;
    }

    transaction
        .execute("DELETE FROM search_index WHERE entity_id = ?1", params![request.id])
        .map_err(|error| format!("Failed to clear search index: {error}"))?;

    transaction
        .execute(
            "INSERT INTO search_index (entity_id, title, summary, content, tags) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![request.id, title, summary, content, normalized_tags.join(" ")],
        )
        .map_err(|error| format!("Failed to update search index: {error}"))?;

    transaction
        .commit()
        .map_err(|error| format!("Failed to commit entity update: {error}"))?;

    load_entity(&connection, &request.id)
}

#[tauri::command]
fn archive_entity(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;

    let rows = connection
        .execute(
            "UPDATE entities SET archived_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1 AND archived_at IS NULL",
            params![id],
        )
        .map_err(|error| format!("Failed to archive entity: {error}"))?;

    if rows == 0 {
        return Err(format!("Entity not found or already archived: {id}"));
    }

    Ok(())
}

#[tauri::command]
fn list_entities(state: State<'_, AppState>, entity_type: Option<String>) -> Result<Vec<Entity>, String> {
    let connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;

    let mut sql = String::from(
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
        FROM entities e
        LEFT JOIN entity_contents c ON c.entity_id = e.id
        LEFT JOIN entity_tags et ON et.entity_id = e.id
        LEFT JOIN tags t ON t.id = et.tag_id
        WHERE e.archived_at IS NULL
        "#,
    );

    if entity_type.is_some() {
        sql.push_str(" AND e.type = ?1");
    }

    sql.push_str(" GROUP BY e.id ORDER BY e.updated_at DESC LIMIT 100");

    let mut statement = connection
        .prepare(&sql)
        .map_err(|error| format!("Failed to prepare entity query: {error}"))?;

    let rows = if let Some(entity_type) = entity_type {
        statement
            .query_map(params![entity_type], map_entity_row)
            .map_err(|error| format!("Failed to list entities: {error}"))?
            .collect::<Result<Vec<_>, _>>()
    } else {
        statement
            .query_map([], map_entity_row)
            .map_err(|error| format!("Failed to list entities: {error}"))?
            .collect::<Result<Vec<_>, _>>()
    };

    rows.map_err(|error| format!("Failed to read entities: {error}"))
}

fn map_entity_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Entity> {
    Ok(Entity {
        id: row.get(0)?,
        entity_type: row.get(1)?,
        title: row.get(2)?,
        summary: row.get(3)?,
        content: row.get(4)?,
        tags: parse_tags(row.get(5)?),
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

#[tauri::command]
fn search_entities(state: State<'_, AppState>, query: String) -> Result<Vec<Entity>, String> {
    let query = query.trim();
    if query.is_empty() {
        return list_entities(state, None);
    }

    let connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;

    let fts_query = query
        .split_whitespace()
        .map(|token| format!("\"{}\"*", token.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(" AND ");

    let mut statement = connection
        .prepare(
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
            FROM search_index si
            JOIN entities e ON e.id = si.entity_id
            LEFT JOIN entity_contents c ON c.entity_id = e.id
            LEFT JOIN entity_tags et ON et.entity_id = e.id
            LEFT JOIN tags t ON t.id = et.tag_id
            WHERE search_index MATCH ?1
              AND e.archived_at IS NULL
            GROUP BY e.id
            ORDER BY rank
            LIMIT 50
            "#,
        )
        .map_err(|error| format!("Failed to prepare search query: {error}"))?;

    let entities = statement
        .query_map(params![fts_query], map_entity_row)
        .map_err(|error| format!("Failed to search entities: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Failed to read search results: {error}"))?;

    Ok(entities)
}

#[tauri::command]
fn dashboard_summary(state: State<'_, AppState>) -> Result<DashboardSummary, String> {
    let connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;

    let count_type = |entity_type: &str| -> Result<i64, String> {
        connection
            .query_row(
                "SELECT COUNT(*) FROM entities WHERE type = ?1 AND archived_at IS NULL",
                params![entity_type],
                |row| row.get(0),
            )
            .map_err(|error| format!("Failed to count {entity_type}: {error}"))
    };

    let reminders_due_today = connection
        .query_row(
            r#"
            SELECT COUNT(*)
            FROM reminders
            WHERE triggered_at IS NULL
              AND date(remind_at) <= date('now', 'localtime')
            "#,
            [],
            |row| row.get(0),
        )
        .map_err(|error| format!("Failed to count reminders: {error}"))?;

    Ok(DashboardSummary {
        notes: count_type("note")?,
        tasks: count_type("task")?,
        events: count_type("event")?,
        knowledge: count_type("knowledge")?,
        files: count_type("file")?,
        reminders_due_today,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let state = setup_database(app)?;
            app.manage(state);

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_entity,
            update_entity,
            archive_entity,
            list_entities,
            search_entities,
            dashboard_summary
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
