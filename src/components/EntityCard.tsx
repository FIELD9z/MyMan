import { entityTypeLabel } from '../lib/entityTypes'
import type { Entity } from '../types'

interface Props {
  entity: Entity
  onEdit: (entity: Entity) => void
  onArchive: (id: string) => void
}

export function EntityCard({ entity, onEdit, onArchive }: Props) {
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
        <button onClick={() => onEdit(entity)}>编辑</button>
        <button className="secondary" onClick={() => onArchive(entity.id)}>归档</button>
      </div>
    </article>
  )
}
