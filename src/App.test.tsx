import { render, screen, waitFor, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import App from './App'
import * as api from './lib/entities'
import { emptySummary, entity } from './test/factories'

vi.mock('./lib/entities', () => ({
  archiveEntity: vi.fn(),
  cleanupUnusedTags: vi.fn(),
  createEntity: vi.fn(),
  dashboardSummary: vi.fn(),
  listEntities: vi.fn(),
  listTagSummaries: vi.fn(),
  listTags: vi.fn(),
  mergeTags: vi.fn(),
  renameTag: vi.fn(),
  restoreEntity: vi.fn(),
  searchEntities: vi.fn(),
  updateEntity: vi.fn(),
}))

describe('App', () => {
  const page = (items = [entity()]) => ({ items, total: items.length })

  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(api.dashboardSummary).mockResolvedValue({
      ...emptySummary,
      notes: 1,
      tasks: 1,
      files: 1,
    })
    vi.mocked(api.listEntities).mockResolvedValue(page())
    vi.mocked(api.listTags).mockResolvedValue(['work'])
    vi.mocked(api.listTagSummaries).mockResolvedValue([
      { name: 'work', activeCount: 1, archivedCount: 0 },
    ])
    vi.mocked(api.searchEntities).mockResolvedValue(page([entity({ id: 'task-1', entityType: 'task', title: 'Alpha task' })]))
    vi.mocked(api.createEntity).mockResolvedValue(entity())
    vi.mocked(api.updateEntity).mockResolvedValue(entity())
    vi.mocked(api.archiveEntity).mockResolvedValue(undefined)
    vi.mocked(api.restoreEntity).mockResolvedValue(undefined)
    vi.mocked(api.renameTag).mockResolvedValue(undefined)
    vi.mocked(api.mergeTags).mockResolvedValue(undefined)
    vi.mocked(api.cleanupUnusedTags).mockResolvedValue(0)
  })

  it('loads dashboard data on startup', async () => {
    render(<App />)

    expect(await screen.findByText('Alpha note')).toBeInTheDocument()
    expect(screen.getByText('本地数据已就绪')).toBeInTheDocument()
    expect(api.listEntities).toHaveBeenCalled()
  })

  it('keeps filters when searching', async () => {
    const user = userEvent.setup()
    render(<App />)

    await screen.findByText('Alpha note')
    await user.click(screen.getByRole('button', { name: /任务/ }))

    await waitFor(() => {
      expect(api.listEntities).toHaveBeenLastCalledWith(expect.objectContaining({ entityType: 'task' }))
    })

    const tagNav = screen.getByRole('navigation', { name: '标签筛选' })
    await user.click(within(tagNav).getByRole('button', { name: 'work' }))

    await user.type(screen.getByPlaceholderText('搜索全部词...'), 'alpha')
    await user.click(screen.getByRole('button', { name: '搜索' }))

    await waitFor(() => {
      expect(api.searchEntities).toHaveBeenLastCalledWith(expect.objectContaining({
        query: 'alpha',
        entityType: 'task',
        tag: 'work',
      }))
    })
  })

  it('shows archive box and restores an archived item', async () => {
    const user = userEvent.setup()
    const archived = entity({ id: 'archived-1', title: 'Archived note' })
    vi.mocked(api.listEntities).mockResolvedValueOnce(page())
    vi.mocked(api.listEntities).mockResolvedValue(page([archived]))

    render(<App />)
    await screen.findByText('Alpha note')

    await user.click(screen.getByRole('button', { name: '归档箱' }))
    expect(await screen.findByText('Archived note')).toBeInTheDocument()

    await user.click(screen.getByRole('button', { name: '恢复' }))
    await waitFor(() => expect(api.restoreEntity).toHaveBeenCalledWith('archived-1'))
  })

  it('opens the tag management workspace', async () => {
    const user = userEvent.setup()
    render(<App />)

    await screen.findByText('Alpha note')
    await user.click(screen.getByRole('button', { name: '标签管理' }))

    expect(await screen.findByRole('region', { name: '标签管理面板' })).toBeInTheDocument()
    expect(screen.queryByPlaceholderText('搜索全部词...')).not.toBeInTheDocument()
    expect(api.listTagSummaries).toHaveBeenCalledTimes(1)
  })
})
