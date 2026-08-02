import { invoke } from '@tauri-apps/api/core'
import type {
  CreateEntityRequest,
  DashboardSummary,
  Entity,
  EntityPage,
  ListEntitiesRequest,
  SearchEntitiesRequest,
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

export function dashboardSummary(): Promise<DashboardSummary> {
  return invoke<DashboardSummary>('dashboard_summary')
}
