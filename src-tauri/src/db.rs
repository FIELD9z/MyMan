use rusqlite::{params, Connection, OptionalExtension};
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use tauri::Manager;

const INITIAL_MIGRATION_ID: &str = "0001_initial";
const INITIAL_MIGRATION: &str = include_str!("../migrations/0001_initial.sql");

pub struct AppState {
    pub(crate) db: Mutex<Connection>,
}

pub fn setup_database(app: &tauri::App) -> Result<AppState, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Failed to resolve app data directory: {error}"))?;

    fs::create_dir_all(&data_dir)
        .map_err(|error| format!("Failed to create app data directory: {error}"))?;

    let connection = open_database(data_dir.join("myman.sqlite3"))?;

    Ok(AppState {
        db: Mutex::new(connection),
    })
}

pub fn open_database(path: impl AsRef<Path>) -> Result<Connection, String> {
    let connection = Connection::open(path)
        .map_err(|error| format!("Failed to open SQLite database: {error}"))?;
    run_migrations(&connection)?;
    Ok(connection)
}

pub fn run_migrations(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            r#"
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS schema_migrations (
              id TEXT PRIMARY KEY,
              applied_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
            );
            "#,
        )
        .map_err(|error| format!("Failed to prepare database migrations: {error}"))?;

    let migration_applied = connection
        .query_row(
            "SELECT 1 FROM schema_migrations WHERE id = ?1",
            params![INITIAL_MIGRATION_ID],
            |_| Ok(()),
        )
        .optional()
        .map_err(|error| format!("Failed to inspect database migrations: {error}"))?
        .is_some();

    if !migration_applied {
        connection
            .execute_batch(INITIAL_MIGRATION)
            .map_err(|error| {
                format!("Failed to run database migration {INITIAL_MIGRATION_ID}: {error}")
            })?;
        connection
            .execute(
                "INSERT OR IGNORE INTO schema_migrations (id) VALUES (?1)",
                params![INITIAL_MIGRATION_ID],
            )
            .map_err(|error| {
                format!("Failed to record database migration {INITIAL_MIGRATION_ID}: {error}")
            })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn migration_count(connection: &Connection) -> i64 {
        connection
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| {
                row.get(0)
            })
            .expect("migration count")
    }

    #[test]
    fn runs_migrations_on_empty_database() {
        let connection = Connection::open_in_memory().expect("open in-memory database");

        run_migrations(&connection).expect("run migrations");

        let entity_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM entities", [], |row| row.get(0))
            .expect("entities table exists");
        assert_eq!(entity_count, 0);
        assert_eq!(migration_count(&connection), 1);
    }

    #[test]
    fn records_initial_migration_for_legacy_database() {
        let connection = Connection::open_in_memory().expect("open in-memory database");
        connection
            .execute_batch(INITIAL_MIGRATION)
            .expect("create legacy schema without migration table");

        run_migrations(&connection).expect("run migrations");

        let migration_id: String = connection
            .query_row("SELECT id FROM schema_migrations", [], |row| row.get(0))
            .expect("migration row");
        assert_eq!(migration_id, INITIAL_MIGRATION_ID);
    }
}
