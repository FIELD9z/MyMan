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

export interface ListEntitiesRequest {
  entityType?: EntityType
  tag?: string
  archived?: boolean
  limit?: number
  offset?: number
}

export type SearchMode = 'and' | 'or'

export interface SearchEntitiesRequest extends ListEntitiesRequest {
  query: string
  searchMode: SearchMode
}

export interface RenameTagRequest {
  oldName: string
  newName: string
}

export interface MergeTagRequest {
  sourceName: string
  targetName: string
}

export interface TagSummary {
  name: string
  activeCount: number
  archivedCount: number
}

export interface EntityPage {
  items: Entity[]
  total: number
}

export interface DashboardSummary {
  notes: number
  tasks: number
  events: number
  knowledge: number
  files: number
  remindersDueToday: number
}
