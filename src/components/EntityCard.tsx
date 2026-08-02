import { entityTypeLabel } from '../lib/entityTypes'
import type { Entity } from '../types'

interface Props {
  entity: Entity
  archived?: boolean
  onEdit: (entity: Entity) => void
  onArchive: (id: string) => void
  onRestore: (id: string) => void
}

export function EntityCard({ entity, archived = false, onEdit, onArchive, onRestore }: Props) {
  return (
    <article className="entity-card">
      <div>
        <span className="entity-type">{entityTypeLabel(entity.entityType)}</span>
        <h3>{entity.title}</h3>
        {entity.summary ? <p>{entity.summary}</p> : null}
      </div>
      {entity.tags.length > 0 ? (
        <div className="tag-list">
          {entity.tags.map((tag) => (
            <span key={tag}>{tag}</span>
          ))}
        </div>
      ) : null}
      <small>{new Date(entity.updatedAt).toLocaleString()}</small>
      <div className="entity-actions">
        {archived ? (
          <button className="secondary" onClick={() => onRestore(entity.id)}>
            恢复
          </button>
        ) : (
          <>
            <button onClick={() => onEdit(entity)}>编辑</button>
            <button className="secondary" onClick={() => onArchive(entity.id)}>
              归档
            </button>
          </>
        )}
      </div>
    </article>
  )
}
