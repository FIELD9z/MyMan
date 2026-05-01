export type EntityType = 'note' | 'task' | 'event' | 'knowledge' | 'file'

export interface Entity {
  id: string
  entityType: EntityType
  title: string
  summary?: string | null
  content?: string | null
  tags: string[]
  createdAt: string
  updatedAt: string
}

export interface CreateEntityRequest {
  entityType: EntityType
  title: string
  summary?: string
  content?: string
  tags: string[]
}

export interface UpdateEntityRequest {
  id: string
  title: string
  summary?: string
  content?: string
  tags: string[]
}

export interface DashboardSummary {
  notes: number
  tasks: number
  events: number
  knowledge: number
  files: number
  remindersDueToday: number
}
