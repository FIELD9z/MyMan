use crate::db::AppState;
use crate::entities;
use crate::models::{
    CreateEntityRequest, DashboardSummary, Entity, ListEntitiesRequest, SearchEntitiesRequest,
    UpdateEntityRequest,
};
use tauri::State;

#[tauri::command]
pub fn create_entity(
    state: State<'_, AppState>,
    request: CreateEntityRequest,
) -> Result<Entity, String> {
    let mut connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;
    entities::create_entity(&mut connection, request)
}

#[tauri::command]
pub fn update_entity(
    state: State<'_, AppState>,
    request: UpdateEntityRequest,
) -> Result<Entity, String> {
    let mut connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;
    entities::update_entity(&mut connection, request)
}

#[tauri::command]
pub fn archive_entity(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;
    entities::archive_entity(&connection, &id)
}

#[tauri::command]
pub fn list_entities(
    state: State<'_, AppState>,
    request: ListEntitiesRequest,
) -> Result<Vec<Entity>, String> {
    let connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;
    entities::list_entities(&connection, request)
}

#[tauri::command]
pub fn search_entities(
    state: State<'_, AppState>,
    request: SearchEntitiesRequest,
) -> Result<Vec<Entity>, String> {
    let connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;
    entities::search_entities(&connection, request)
}

#[tauri::command]
pub fn list_tags(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;
    entities::list_tags(&connection)
}

#[tauri::command]
pub fn dashboard_summary(state: State<'_, AppState>) -> Result<DashboardSummary, String> {
    let connection = state
        .db
        .lock()
        .map_err(|error| format!("Failed to lock database: {error}"))?;
    entities::dashboard_summary(&connection)
}
