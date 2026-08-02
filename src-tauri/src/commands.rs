use crate::db::AppState;
use crate::entities;
use crate::models::{
    CreateEntityRequest, DashboardSummary, Entity, EntityPage, ListEntitiesRequest, MergeTagRequest,
    RenameTagRequest, SearchEntitiesRequest, TagSummary, UpdateEntityRequest,
};
use crate::tag_management;
use tauri::State;

#[tauri::command]
pub fn create_entity(state: State<'_, AppState>, request: CreateEntityRequest) -> Result<Entity, String> {
    let mut connection = state.db.lock().map_err(|e| format!("Failed to lock database: {e}"))?;
    entities::create_entity(&mut connection, request)
}

#[tauri::command]
pub fn update_entity(state: State<'_, AppState>, request: UpdateEntityRequest) -> Result<Entity, String> {
    let mut connection = state.db.lock().map_err(|e| format!("Failed to lock database: {e}"))?;
    entities::update_entity(&mut connection, request)
}

#[tauri::command]
pub fn archive_entity(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let connection = state.db.lock().map_err(|e| format!("Failed to lock database: {e}"))?;
    entities::archive_entity(&connection, &id)
}

#[tauri::command]
pub fn restore_entity(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let connection = state.db.lock().map_err(|e| format!("Failed to lock database: {e}"))?;
    entities::restore_entity(&connection, &id)
}

#[tauri::command]
pub fn list_entities(state: State<'_, AppState>, request: ListEntitiesRequest) -> Result<EntityPage, String> {
    let connection = state.db.lock().map_err(|e| format!("Failed to lock database: {e}"))?;
    entities::list_entities(&connection, request)
}

#[tauri::command]
pub fn search_entities(state: State<'_, AppState>, request: SearchEntitiesRequest) -> Result<EntityPage, String> {
    let connection = state.db.lock().map_err(|e| format!("Failed to lock database: {e}"))?;
    entities::search_entities(&connection, request)
}

#[tauri::command]
pub fn list_tags(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let connection = state.db.lock().map_err(|e| format!("Failed to lock database: {e}"))?;
    entities::list_tags(&connection)
}

#[tauri::command]
pub fn list_tag_summaries(state: State<'_, AppState>) -> Result<Vec<TagSummary>, String> {
    let connection = state.db.lock().map_err(|e| format!("Failed to lock database: {e}"))?;
    tag_management::list_tag_summaries(&connection)
}

#[tauri::command]
pub fn rename_tag(state: State<'_, AppState>, request: RenameTagRequest) -> Result<(), String> {
    let connection = state.db.lock().map_err(|e| format!("Failed to lock database: {e}"))?;
    tag_management::rename_tag(&connection, &request.old_name, &request.new_name)
}

#[tauri::command]
pub fn merge_tags(state: State<'_, AppState>, request: MergeTagRequest) -> Result<(), String> {
    let connection = state.db.lock().map_err(|e| format!("Failed to lock database: {e}"))?;
    tag_management::merge_tags(&connection, &request.source_name, &request.target_name)
}

#[tauri::command]
pub fn cleanup_unused_tags(state: State<'_, AppState>) -> Result<u64, String> {
    let connection = state.db.lock().map_err(|e| format!("Failed to lock database: {e}"))?;
    tag_management::cleanup_unused_tags(&connection)
}

#[tauri::command]
pub fn dashboard_summary(state: State<'_, AppState>) -> Result<DashboardSummary, String> {
    let connection = state.db.lock().map_err(|e| format!("Failed to lock database: {e}"))?;
    entities::dashboard_summary(&connection)
}
