import { useEffect, useMemo, useState } from 'react'
import type { FormEvent } from 'react'
import './App.css'
import { createEntity, dashboardSummary, listEntities, searchEntities } from './lib/entities'
import type { DashboardSummary, Entity, EntityType } from './types'

const entityTypes: Array<{ type: EntityType; label: string; description: string }> = [
  { type: 'note', label: '随手记', description: '快速捕捉想法、碎片信息和临时记录' },
  { type: 'task', label: '任务', description: '可执行事项、截止时间和后续安排' },
  { type: 'event', label: '日程', description: '时间块、会议和应用内提醒' },
  { type: 'knowledge', label: '知识', description: 'Markdown 长文、资料沉淀和双向关联' },
  { type: 'file', label: '文件', description: '文件路径、描述、标签和基础元数据' },
]

const emptySummary: DashboardSummary = {
  notes: 0,
  tasks: 0,
  events: 0,
  knowledge: 0,
  files: 0,
  remindersDueToday: 0,
}

function App() {
  const [activeType, setActiveType] = useState<EntityType | 'all'>('all')
  const [entities, setEntities] = useState<Entity[]>([])
  const [summary, setSummary] = useState<DashboardSummary>(emptySummary)
  const [query, setQuery] = useState('')
  const [title, setTitle] = useState('')
  const [entityType, setEntityType] = useState<EntityType>('note')
  const [body, setBody] = useState('')
  const [tags, setTags] = useState('')
  const [status, setStatus] = useState<string>('正在连接本地数据库...')
  const [isSaving, setIsSaving] = useState(false)

  const activeTypeLabel = useMemo(() => {
    if (activeType === 'all') return '全部'
    return entityTypes.find((item) => item.type === activeType)?.label ?? activeType
  }, [activeType])

  async function refresh(nextType = activeType, nextQuery = query) {
    try {
      const [nextSummary, nextEntities] = await Promise.all([
        dashboardSummary(),
        nextQuery.trim()
          ? searchEntities(nextQuery)
          : listEntities(nextType === 'all' ? undefined : nextType),
      ])

      setSummary(nextSummary)
      setEntities(nextEntities)
      setStatus('本地数据已就绪')
    } catch (error) {
      setStatus(`无法连接 Tauri 后端：${String(error)}`)
      setEntities([])
    }
  }

  useEffect(() => {
    async function loadInitialData() {
      try {
        const [nextSummary, nextEntities] = await Promise.all([dashboardSummary(), listEntities()])
        setSummary(nextSummary)
        setEntities(nextEntities)
        setStatus('本地数据已就绪')
      } catch (error) {
        setStatus(`无法连接 Tauri 后端：${String(error)}`)
        setEntities([])
      }
    }

    void loadInitialData()
  }, [])

  async function handleFilter(nextType: EntityType | 'all') {
    setActiveType(nextType)
    await refresh(nextType)
  }

  async function handleSearch(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    await refresh(activeType, query)
  }

  async function handleCreate(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setIsSaving(true)
    setStatus('正在保存...')

    try {
      await createEntity({
        entityType,
        title,
        summary: body.split('\n').find(Boolean) ?? '',
        content: body,
        tags: tags
          .split(',')
          .map((tag) => tag.trim())
          .filter(Boolean),
      })

      setTitle('')
      setBody('')
      setTags('')
      setStatus('已保存到本地数据库')
      await refresh(activeType)
    } catch (error) {
      setStatus(`保存失败：${String(error)}`)
    } finally {
      setIsSaving(false)
    }
  }

  return (
    <main className="app-shell">
      <aside className="sidebar">
        <div>
          <p className="eyebrow">Myman</p>
          <h1>本地优先个人助理</h1>
          <p className="subtitle">
            从统一实体模型开始，把随手记、任务、日程、知识库和文件分类放进同一套搜索与关联体系。
          </p>
        </div>

        <nav className="type-nav" aria-label="实体类型">
          <button className={activeType === 'all' ? 'active' : ''} onClick={() => void handleFilter('all')}>
            全部
            <span>{entities.length}</span>
          </button>
          {entityTypes.map((item) => (
            <button
              key={item.type}
              className={activeType === item.type ? 'active' : ''}
              onClick={() => void handleFilter(item.type)}
            >
              {item.label}
              <span>{summary[summaryKey(item.type)]}</span>
            </button>
          ))}
        </nav>

        <section className="status-card">
          <strong>状态</strong>
          <p>{status}</p>
        </section>
      </aside>

      <section className="workspace">
        <header className="workspace-header">
          <div>
            <p className="eyebrow">今日视图雏形</p>
            <h2>{activeTypeLabel}工作台</h2>
          </div>
          <div className="summary-grid">
            <SummaryItem label="笔记" value={summary.notes} />
            <SummaryItem label="任务" value={summary.tasks} />
            <SummaryItem label="日程" value={summary.events} />
            <SummaryItem label="知识" value={summary.knowledge} />
            <SummaryItem label="文件" value={summary.files} />
            <SummaryItem label="提醒" value={summary.remindersDueToday} />
          </div>
        </header>

        <form className="search-bar" onSubmit={handleSearch}>
          <input
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            placeholder="搜索标题、正文、标签或文件描述"
          />
          <button type="submit">搜索</button>
        </form>

        <div className="content-grid">
          <form className="composer" onSubmit={handleCreate}>
            <div className="form-row">
              <label>
                类型
                <select value={entityType} onChange={(event) => setEntityType(event.target.value as EntityType)}>
                  {entityTypes.map((item) => (
                    <option key={item.type} value={item.type}>
                      {item.label}
                    </option>
                  ))}
                </select>
              </label>
              <label>
                标签
                <input value={tags} onChange={(event) => setTags(event.target.value)} placeholder="工作, 灵感" />
              </label>
            </div>
            <label>
              标题
              <input value={title} onChange={(event) => setTitle(event.target.value)} placeholder="记录一个想法" />
            </label>
            <label>
              Markdown 内容 / 文件描述
              <textarea
                value={body}
                onChange={(event) => setBody(event.target.value)}
                placeholder="写下内容、任务背景、日程说明，或描述一个文件方便之后找回..."
              />
            </label>
            <button type="submit" disabled={isSaving || !title.trim()}>
              {isSaving ? '保存中...' : '保存'}
            </button>
          </form>

          <section className="entity-list" aria-live="polite">
            {entities.length === 0 ? (
              <div className="empty-state">
                <h3>还没有内容</h3>
                <p>先创建一条随手记，后续可以把它转为任务、知识条目或关联文件。</p>
              </div>
            ) : (
              entities.map((entity) => <EntityCard key={entity.id} entity={entity} />)
            )}
          </section>
        </div>
      </section>
    </main>
  )
}

function SummaryItem({ label, value }: { label: string; value: number }) {
  return (
    <div className="summary-item">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  )
}

function EntityCard({ entity }: { entity: Entity }) {
  const type = entityTypes.find((item) => item.type === entity.entityType)

  return (
    <article className="entity-card">
      <div>
        <span className="entity-type">{type?.label ?? entity.entityType}</span>
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
    </article>
  )
}

function summaryKey(type: EntityType): keyof DashboardSummary {
  if (type === 'note') return 'notes'
  if (type === 'task') return 'tasks'
  if (type === 'event') return 'events'
  if (type === 'knowledge') return 'knowledge'
  return 'files'
}

export default App
