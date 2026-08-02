import { useEffect, useState, type FormEvent } from 'react'
import { entityTypeConfigs, entityTypeLabel } from '../lib/entityTypes'
import type { CreateEntityRequest, Entity, EntityType, UpdateEntityRequest } from '../types'

interface Props {
  editing?: Entity | null
  onSave: (request: CreateEntityRequest | UpdateEntityRequest) => Promise<void>
  onCancelEdit: () => void
}

export function Composer({ editing, onSave, onCancelEdit }: Props) {
  const [entityType, setEntityType] = useState<EntityType>('note')
  const [title, setTitle] = useState('')
  const [body, setBody] = useState('')
  const [tags, setTags] = useState('')
  const [isSaving, setIsSaving] = useState(false)
  const [saveError, setSaveError] = useState<string | null>(null)

  const isEditing = editing != null

  useEffect(() => {
    setSaveError(null)
    if (editing) {
      setTitle(editing.title)
      setBody(editing.content ?? '')
      setTags(editing.tags.join(', '))
      setEntityType(editing.entityType)
    } else {
      setTitle('')
      setBody('')
      setTags('')
      setEntityType('note')
    }
  }, [editing])

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setIsSaving(true)
    setSaveError(null)

    try {
      if (editing) {
        await onSave({
          id: editing.id,
          title,
          summary: body.split('\n').find(Boolean) ?? '',
          content: body,
          tags: tags.split(',').map((tag) => tag.trim()).filter(Boolean),
        })
      } else {
        await onSave({
          entityType,
          title,
          summary: body.split('\n').find(Boolean) ?? '',
          content: body,
          tags: tags.split(',').map((tag) => tag.trim()).filter(Boolean),
        })
      }

      if (!isEditing) {
        setTitle('')
        setBody('')
        setTags('')
      }
    } catch (error) {
      setSaveError(`无法保存：${String(error)}`)
    } finally {
      setIsSaving(false)
    }
  }

  return (
    <form className="composer" onSubmit={handleSubmit}>
      <div className="form-row">
        {isEditing ? (
          <p className="edit-label">
            类型：{editing ? entityTypeLabel(editing.entityType) : ''}
          </p>
        ) : (
          <label>
            类型
            <select value={entityType} onChange={(event) => setEntityType(event.target.value as EntityType)}>
              {entityTypeConfigs.map((item) => (
                <option key={item.type} value={item.type}>
                  {item.label}
                </option>
              ))}
            </select>
          </label>
        )}
        <label>
          标签
          <input value={tags} onChange={(event) => setTags(event.target.value)} placeholder="工作, 灵感" />
        </label>
      </div>
      <label>
        标题
        <input value={title} onChange={(event) => setTitle(event.target.value)} placeholder="标题..." />
      </label>
      <label>
        Markdown 内容 / 文件描述
        <textarea
          value={body}
          onChange={(event) => setBody(event.target.value)}
          placeholder="写下内容..."
        />
      </label>
      {saveError ? (
        <p className="form-error" role="alert">
          {saveError}
        </p>
      ) : null}
      <div className="composer-actions">
        <button type="submit" disabled={isSaving || !title.trim()}>
          {isSaving ? '保存中...' : isEditing ? '更新' : '保存'}
        </button>
        {isEditing ? (
          <button type="button" className="secondary" onClick={onCancelEdit}>
            取消
          </button>
        ) : null}
      </div>
    </form>
  )
}
