import { invoke } from '@tauri-apps/api/core'
import type { CreateEntityRequest, DashboardSummary, Entity, EntityType, UpdateEntityRequest } from '../types'

export function createEntity(request: CreateEntityRequest): Promise<Entity> {
  return invoke<Entity>('create_entity', { request })
}

export function updateEntity(request: UpdateEntityRequest): Promise<Entity> {
  return invoke<Entity>('update_entity', { request })
}

export function archiveEntity(id: string): Promise<void> {
  return invoke('archive_entity', { id })
}

export function listEntities(entityType?: EntityType, tag?: string): Promise<Entity[]> {
  return invoke<Entity[]>('list_entities', { entityType, tag })
}

export function searchEntities(query: string, searchMode?: 'and' | 'or'): Promise<Entity[]> {
  return invoke<Entity[]>('search_entities', { query, searchMode })
}

export function listTags(): Promise<string[]> {
  return invoke<string[]>('list_tags')
}

export function dashboardSummary(): Promise<DashboardSummary> {
  return invoke<DashboardSummary>('dashboard_summary')
}
