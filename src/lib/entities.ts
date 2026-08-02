import { invoke } from '@tauri-apps/api/core'
import type {
  CreateEntityRequest,
  DashboardSummary,
  Entity,
  EntityPage,
  ListEntitiesRequest,
  MergeTagRequest,
  RenameTagRequest,
  SearchEntitiesRequest,
  TagSummary,
  UpdateEntityRequest,
} from '../types'

export function createEntity(request: CreateEntityRequest): Promise<Entity> {
  return invoke<Entity>('create_entity', { request })
}

export function updateEntity(request: UpdateEntityRequest): Promise<Entity> {
  return invoke<Entity>('update_entity', { request })
}

export function archiveEntity(id: string): Promise<void> {
  return invoke('archive_entity', { id })
}

export function restoreEntity(id: string): Promise<void> {
  return invoke('restore_entity', { id })
}

export function listEntities(request: ListEntitiesRequest = {}): Promise<EntityPage> {
  return invoke<EntityPage>('list_entities', { request })
}

export function searchEntities(request: SearchEntitiesRequest): Promise<EntityPage> {
  return invoke<EntityPage>('search_entities', { request })
}

export function listTags(): Promise<string[]> {
  return invoke<string[]>('list_tags')
}

export function listTagSummaries(): Promise<TagSummary[]> {
  return invoke<TagSummary[]>('list_tag_summaries')
}

export function renameTag(request: RenameTagRequest): Promise<void> {
  return invoke('rename_tag', { request })
}

export function mergeTags(request: MergeTagRequest): Promise<void> {
  return invoke('merge_tags', { request })
}

export function cleanupUnusedTags(): Promise<number> {
  return invoke<number>('cleanup_unused_tags')
}

export function dashboardSummary(): Promise<DashboardSummary> {
  return invoke<DashboardSummary>('dashboard_summary')
}
