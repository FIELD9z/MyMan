import { render, screen, waitFor, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import App from './App'
import * as api from './lib/entities'
import { emptySummary, entity } from './test/factories'

vi.mock('./lib/entities', () => ({
  archiveEntity: vi.fn(),
  createEntity: vi.fn(),
  dashboardSummary: vi.fn(),
  listEntities: vi.fn(),
  listTags: vi.fn(),
  searchEntities: vi.fn(),
  updateEntity: vi.fn(),
}))

describe('App', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(api.dashboardSummary).mockResolvedValue({
      ...emptySummary,
      notes: 1,
      tasks: 1,
      files: 1,
    })
    vi.mocked(api.listEntities).mockResolvedValue([entity()])
    vi.mocked(api.listTags).mockResolvedValue(['work'])
    vi.mocked(api.searchEntities).mockResolvedValue([entity({ id: 'task-1', entityType: 'task', title: 'Alpha task' })])
    vi.mocked(api.createEntity).mockResolvedValue(entity())
    vi.mocked(api.updateEntity).mockResolvedValue(entity())
    vi.mocked(api.archiveEntity).mockResolvedValue(undefined)
  })

  it('loads dashboard data on startup', async () => {
    render(<App />)

    expect(await screen.findByText('Alpha note')).toBeInTheDocument()
    expect(screen.getByText('本地数据已就绪')).toBeInTheDocument()
    expect(api.dashboardSummary).toHaveBeenCalled()
    expect(api.listEntities).toHaveBeenCalledWith()
    expect(api.listTags).toHaveBeenCalled()
  })

  it('keeps type and tag filters when searching and toggling search mode', async () => {
    const user = userEvent.setup()
    render(<App />)

    await screen.findByText('Alpha note')
    await user.click(screen.getByRole('button', { name: /任务/ }))

    await waitFor(() => {
      expect(api.listEntities).toHaveBeenLastCalledWith({ entityType: 'task', tag: undefined })
    })

    const tagNav = screen.getByRole('navigation', { name: '标签筛选' })
    await user.click(within(tagNav).getByRole('button', { name: 'work' }))

    await waitFor(() => {
      expect(api.listEntities).toHaveBeenLastCalledWith({ entityType: 'task', tag: 'work' })
    })

    await user.type(screen.getByPlaceholderText('搜索全部词...'), 'alpha')
    await user.click(screen.getByRole('button', { name: '搜索' }))

    await waitFor(() => {
      expect(api.searchEntities).toHaveBeenLastCalledWith({
        query: 'alpha',
        searchMode: 'and',
        entityType: 'task',
        tag: 'work',
      })
    })

    await user.click(screen.getByRole('button', { name: 'AND' }))

    await waitFor(() => {
      expect(api.searchEntities).toHaveBeenLastCalledWith({
        query: 'alpha',
        searchMode: 'or',
        entityType: 'task',
        tag: 'work',
      })
    })
  })
})
