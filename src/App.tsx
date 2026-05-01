import { useEffect, useMemo, useState } from 'react'
import type { FormEvent } from 'react'
import './App.css'
import { Composer } from './components/Composer'
import { EntityCard } from './components/EntityCard'
import { archiveEntity, createEntity, dashboardSummary, listEntities, listTags, searchEntities, updateEntity } from './lib/entities'
import type { CreateEntityRequest, DashboardSummary, Entity, EntityType, UpdateEntityRequest } from './types'

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
  const [activeTag, setActiveTag] = useState<string | null>(null)
  const [entities, setEntities] = useState<Entity[]>([])
  const [summary, setSummary] = useState<DashboardSummary>(emptySummary)
  const [query, setQuery] = useState('')
  const [searchMode, setSearchMode] = useState<'and' | 'or'>('and')
  const [editing, setEditing] = useState<Entity | null>(null)
  const [allTags, setAllTags] = useState<string[]>([])
  const [status, setStatus] = useState<string>('正在连接本地数据库...')

  const activeTypeLabel = useMemo(() => {
    if (activeType === 'all') return '全部'
    return entityTypes.find((item) => item.type === activeType)?.label ?? activeType
  }, [activeType])

  async function refresh(nextType = activeType, nextQuery = query, nextTag = activeTag, nextSearchMode = searchMode) {
    try {
      const [nextSummary, nextEntities] = await Promise.all([
        dashboardSummary(),
        nextQuery.trim()
          ? searchEntities(nextQuery, nextSearchMode)
          : listEntities(nextType === 'all' ? undefined : nextType, nextTag ?? undefined),
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
    void (async () => {
      try {
        const [nextSummary, nextEntities, tags] = await Promise.all([
          dashboardSummary(),
          listEntities(),
          listTags(),
        ])
        setSummary(nextSummary)
        setEntities(nextEntities)
        setAllTags(tags)
        setStatus('本地数据已就绪')
      } catch (error) {
        setStatus(`无法连接 Tauri 后端：${String(error)}`)
        setEntities([])
      }
    })()
  }, [])

  async function handleFilter(nextType: EntityType | 'all') {
    setActiveType(nextType)
    await refresh(nextType, query, activeTag)
  }

  async function handleTagFilter(tag: string | null) {
    setActiveTag(tag)
    await refresh(activeType, query, tag)
  }

  async function handleSearch(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    await refresh(activeType, query, activeTag)
  }

  function toggleSearchMode() {
    const next = searchMode === 'and' ? 'or' : 'and'
    setSearchMode(next)
  }

  async function handleCreate(request: CreateEntityRequest) {
    setStatus('正在保存...')
    await createEntity(request)
    setStatus('已保存')
    await refresh(activeType, query, activeTag)
  }

  async function handleUpdate(request: CreateEntityRequest | UpdateEntityRequest) {
    setStatus('正在更新...')
    await updateEntity(request as UpdateEntityRequest)
    setEditing(null)
    setStatus('已更新')
    await refresh(activeType, query, activeTag)
  }

  async function handleArchive(id: string) {
    setStatus('正在归档...')
    await archiveEntity(id)
    setStatus('已归档')
    await refresh(activeType, query, activeTag)
  }

  async function handleSave(request: CreateEntityRequest | UpdateEntityRequest) {
    if ('id' in request) {
      await handleUpdate(request)
    } else {
      await handleCreate(request)
    }
  }

  function handleEdit(entity: Entity) {
    setEditing(entity)
  }

  function handleCancelEdit() {
    setEditing(null)
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

        {allTags.length > 0 ? (
          <nav className="tag-nav" aria-label="标签筛选">
            <p className="tag-nav-label">标签</p>
            <div className="tag-list filter">
              <button className={activeTag === null ? 'active' : ''} onClick={() => void handleTagFilter(null)}>
                全部
              </button>
              {allTags.map((tag) => (
                <button
                  key={tag}
                  className={activeTag === tag ? 'active' : ''}
                  onClick={() => void handleTagFilter(tag)}
                >
                  {tag}
                </button>
              ))}
            </div>
          </nav>
        ) : null}

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
            placeholder={searchMode === 'or' ? '搜索任意词...' : '搜索全部词...'}
          />
          <button
            type="button"
            className={`mode-toggle ${searchMode}`}
            onClick={toggleSearchMode}
          >
            {searchMode === 'or' ? 'OR' : 'AND'}
          </button>
          <button type="submit">搜索</button>
        </form>

        <div className="content-grid">
          <Composer editing={editing} onSave={handleSave} onCancelEdit={handleCancelEdit} />

          <section className="entity-list" aria-live="polite">
            {entities.length === 0 ? (
              <div className="empty-state">
                <h3>还没有内容</h3>
                <p>先创建一条随手记，后续可以把它转为任务、知识条目或关联文件。</p>
              </div>
            ) : (
              entities.map((entity) => (
                <EntityCard key={entity.id} entity={entity} onEdit={handleEdit} onArchive={handleArchive} />
              ))
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

function summaryKey(type: EntityType): keyof DashboardSummary {
  if (type === 'note') return 'notes'
  if (type === 'task') return 'tasks'
  if (type === 'event') return 'events'
  if (type === 'knowledge') return 'knowledge'
  return 'files'
}

export default App
