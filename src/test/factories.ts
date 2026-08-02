import type { DashboardSummary, Entity, EntityType } from '../types'

export const emptySummary: DashboardSummary = {
  notes: 0,
  tasks: 0,
  events: 0,
  knowledge: 0,
  files: 0,
  remindersDueToday: 0,
}

export function entity(overrides: Partial<Entity> = {}): Entity {
  const entityType: EntityType = overrides.entityType ?? 'note'

  return {
    id: 'entity-1',
    entityType,
    title: 'Alpha note',
    summary: 'First summary',
    content: 'First summary\nSecond line',
    tags: ['work'],
    createdAt: '2026-06-24T08:00:00.000Z',
    updatedAt: '2026-06-24T09:00:00.000Z',
    ...overrides,
  }
}
