import { invoke } from '@tauri-apps/api/core'
import type { CreateEntityRequest, DashboardSummary, Entity, EntityType } from '../types'

export function createEntity(request: CreateEntityRequest): Promise<Entity> {
  return invoke<Entity>('create_entity', { request })
}

export function listEntities(entityType?: EntityType): Promise<Entity[]> {
  return invoke<Entity[]>('list_entities', { entityType })
}

export function searchEntities(query: string): Promise<Entity[]> {
  return invoke<Entity[]>('search_entities', { query })
}

export function dashboardSummary(): Promise<DashboardSummary> {
  return invoke<DashboardSummary>('dashboard_summary')
}
