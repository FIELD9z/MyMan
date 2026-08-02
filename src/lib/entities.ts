import { invoke } from '@tauri-apps/api/core'
import type {
  CreateEntityRequest,
  DashboardSummary,
  Entity,
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

export function listEntities(request: ListEntitiesRequest = {}): Promise<Entity[]> {
  return invoke<Entity[]>('list_entities', { request })
}

export function searchEntities(request: SearchEntitiesRequest): Promise<Entity[]> {
  return invoke<Entity[]>('search_entities', { request })
}

export function listTags(): Promise<string[]> {
  return invoke<string[]>('list_tags')
}

export function dashboardSummary(): Promise<DashboardSummary> {
  return invoke<DashboardSummary>('dashboard_summary')
}
