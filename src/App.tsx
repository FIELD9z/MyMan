import { useEffect, useMemo, useRef, useState } from 'react'
import type { FormEvent } from 'react'
import './App.css'
import { Composer } from './components/Composer'
import { EntityCard } from './components/EntityCard'
import { entityTypeConfigs, summaryKeyForType } from './lib/entityTypes'
import {
  archiveEntity,
  createEntity,
  dashboardSummary,
  listEntities,
  listTags,
  restoreEntity,
  searchEntities,
  updateEntity,
} from './lib/entities'
import type {
  CreateEntityRequest,
  DashboardSummary,
  Entity,
  EntityType,
  ListEntitiesRequest,
  SearchMode,
  UpdateEntityRequest,
} from './types'

const PAGE_SIZE = 50

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
  const [showArchived, setShowArchived] = useState(false)
  const [entities, setEntities] = useState<Entity[]>([])
  const [total, setTotal] = useState(0)
  const [summary, setSummary] = useState<DashboardSummary>(emptySummary)
  const [query, setQuery] = useState('')
  const [searchMode, setSearchMode] = useState<SearchMode>('and')
  const [editing, setEditing] = useState<Entity | null>(null)
  const [allTags, setAllTags] = useState<string[]>([])
  const [status, setStatus] = useState<string>('正在连接本地数据库...')
  const [isLoadingMore, setIsLoadingMore] = useState(false)
  const latestRequest = useRef(0)

  const activeTypeLabel = useMemo(() => {
    if (activeType === 'all') return '全部'
    return entityTypeConfigs.find((item) => item.type === activeType)?.label ?? activeType
  }, [activeType])

  async function refresh(
    nextType = activeType,
    nextQuery = query,
    nextTag = activeTag,
    nextSearchMode = searchMode,
    nextArchived = showArchived,
    offset = 0,
    append = false,
  ) {
    const requestId = ++latestRequest.current
    const filters = {
      ...entityFilters(nextType, nextTag),
      archived: nextArchived,
      limit: PAGE_SIZE,
      offset,
    }

    if (append) {
      setIsLoadingMore(true)
    }

    try {
      const [nextSummary, page] = await Promise.all([
        dashboardSummary(),
        nextQuery.trim()
          ? searchEntities({ query: nextQuery, searchMode: nextSearchMode, ...filters })
          : listEntities(filters),
      ])

      if (requestId !== latestRequest.current) return

      setSummary(nextSummary)
      setEntities((current) => (append ? [...current, ...page.items] : page.items))
      setTotal(page.total)
      setStatus('本地数据已就绪')
    } catch (error) {
      if (requestId !== latestRequest.current) return
      setStatus(`无法加载本地数据：${String(error)}`)
      if (!append) {
        setEntities([])
        setTotal(0)
      }
    } finally {
      if (requestId === latestRequest.current) {
        setIsLoadingMore(false)
      }
    }
  }

  async function refreshTags(currentTag = activeTag): Promise<string | null> {
    try {
      const tags = await listTags()
      setAllTags(tags)
      if (currentTag && !tags.includes(currentTag)) {
        setActiveTag(null)
        return null
      }
      return currentTag
    } catch (error) {
      setStatus(`标签加载失败：${String(error)}`)
      return currentTag
    }
  }

  useEffect(() => {
    const requestId = ++latestRequest.current
    void (async () => {
      try {
        const [nextSummary, page, tags] = await Promise.all([
          dashboardSummary(),
          listEntities({ archived: false, limit: PAGE_SIZE, offset: 0 }),
          listTags(),
        ])
        if (requestId !== latestRequest.current) return
        setSummary(nextSummary)
        setEntities(page.items)
        setTotal(page.total)
        setAllTags(tags)
        setStatus('本地数据已就绪')
      } catch (error) {
        if (requestId !== latestRequest.current) return
        setStatus(`无法连接 Tauri 后端：${String(error)}`)
        setEntities([])
        setTotal(0)
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

  async function handleArchivedToggle(nextArchived: boolean) {
    setShowArchived(nextArchived)
    setEditing(null)
    setActiveTag(null)
    await refresh(activeType, query, null, searchMode, nextArchived)
  }

  async function handleSearch(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    await refresh(activeType, query, activeTag)
  }

  function toggleSearchMode() {
    const next = searchMode === 'and' ? 'or' : 'and'
    setSearchMode(next)
    void refresh(activeType, query, activeTag, next)
  }

  async function handleCreate(request: CreateEntityRequest) {
    setStatus('正在保存...')
    try {
      await createEntity(request)
      const nextTag = await refreshTags(activeTag)
      await refresh(activeType, query, nextTag)
      setStatus('已保存')
    } catch (error) {
      setStatus(`保存失败：${String(error)}`)
      throw error
    }
  }

  async function handleUpdate(request: CreateEntityRequest | UpdateEntityRequest) {
    setStatus('正在更新...')
    try {
      await updateEntity(request as UpdateEntityRequest)
      setEditing(null)
      const nextTag = await refreshTags(activeTag)
      await refresh(activeType, query, nextTag)
      setStatus('已更新')
    } catch (error) {
      setStatus(`更新失败：${String(error)}`)
      throw error
    }
  }

  async function handleArchive(id: string) {
    setStatus('正在归档...')
    try {
      await archiveEntity(id)
      const nextTag = await refreshTags(activeTag)
      await refresh(activeType, query, nextTag)
      setStatus('已归档')
    } catch (error) {
      setStatus(`归档失败：${String(error)}`)
    }
  }

  async function handleRestore(id: string) {
    setStatus('正在恢复...')
    try {
      await restoreEntity(id)
      await refresh(activeType, query, null, searchMode, true)
      await refreshTags(null)
      setStatus('已恢复')
    } catch (error) {
      setStatus(`恢复失败：${String(error)}`)
    }
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

  function handleLoadMore() {
    void refresh(activeType, query, activeTag, searchMode, showArchived, entities.length, true)
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

        <nav className="view-nav" aria-label="内容视图">
          <button className={!showArchived ? 'active' : ''} onClick={() => void handleArchivedToggle(false)}>
            当前内容
          </button>
          <button className={showArchived ? 'active' : ''} onClick={() => void handleArchivedToggle(true)}>
            归档箱
          </button>
        </nav>

        <nav className="type-nav" aria-label="实体类型">
          <button className={activeType === 'all' ? 'active' : ''} onClick={() => void handleFilter('all')}>
            全部
            <span>{total}</span>
          </button>
          {entityTypeConfigs.map((item) => (
            <button
              key={item.type}
              className={activeType === item.type ? 'active' : ''}
              onClick={() => void handleFilter(item.type)}
            >
              {item.label}
              <span>{showArchived ? '—' : summary[summaryKeyForType(item.type)]}</span>
            </button>
          ))}
        </nav>

        {!showArchived && allTags.length > 0 ? (
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

        <section className="status-card" aria-live="polite">
          <strong>状态</strong>
          <p>{status}</p>
        </section>
      </aside>

      <section className="workspace">
        <header className="workspace-header">
          <div>
            <p className="eyebrow">{showArchived ? '归档管理' : '今日视图雏形'}</p>
            <h2>{showArchived ? `${activeTypeLabel}归档` : `${activeTypeLabel}工作台`}</h2>
          </div>
          <div className="summary-grid">
            <SummaryItem label="随手记" value={summary.notes} />
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
          <button type="button" className={`mode-toggle ${searchMode}`} onClick={toggleSearchMode}>
            {searchMode === 'or' ? 'OR' : 'AND'}
          </button>
          <button type="submit">搜索</button>
        </form>

        <div className={`content-grid${showArchived ? ' archived' : ''}`}>
          {!showArchived ? (
            <Composer editing={editing} onSave={handleSave} onCancelEdit={handleCancelEdit} />
          ) : null}

          <section className="entity-list" aria-live="polite">
            {entities.length === 0 ? (
              <div className="empty-state">
                <h3>{showArchived ? '归档箱为空' : '还没有内容'}</h3>
                <p>
                  {showArchived
                    ? '归档后的内容会显示在这里，并可以随时恢复。'
                    : '先创建一条随手记，后续可以把它转为任务、知识条目或关联文件。'}
                </p>
              </div>
            ) : (
              entities.map((entity) => (
                <EntityCard
                  key={entity.id}
                  entity={entity}
                  archived={showArchived}
                  onEdit={handleEdit}
                  onArchive={handleArchive}
                  onRestore={handleRestore}
                />
              ))
            )}

            {entities.length < total ? (
              <button className="load-more" onClick={handleLoadMore} disabled={isLoadingMore}>
                {isLoadingMore ? '加载中...' : `加载更多（${entities.length}/${total}）`}
              </button>
            ) : null}
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

function entityFilters(activeType: EntityType | 'all', activeTag: string | null): ListEntitiesRequest {
  return {
    entityType: activeType === 'all' ? undefined : activeType,
    tag: activeTag ?? undefined,
  }
}

export default App
