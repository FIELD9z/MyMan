import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { TagManager } from './TagManager'
import * as api from '../lib/entities'

vi.mock('../lib/entities', () => ({
  cleanupUnusedTags: vi.fn(),
  listTagSummaries: vi.fn(),
  mergeTags: vi.fn(),
  renameTag: vi.fn(),
}))

const summaries = [
  { name: 'personal', activeCount: 1, archivedCount: 0 },
  { name: 'work', activeCount: 2, archivedCount: 1 },
]

describe('TagManager', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(api.listTagSummaries).mockResolvedValue(summaries)
    vi.mocked(api.renameTag).mockResolvedValue(undefined)
    vi.mocked(api.mergeTags).mockResolvedValue(undefined)
    vi.mocked(api.cleanupUnusedTags).mockResolvedValue(1)
  })

  it('loads tag usage statistics', async () => {
    render(<TagManager />)

    expect(await screen.findByRole('table', { name: '标签使用统计' })).toBeInTheDocument()
    expect(screen.getByText('work')).toBeInTheDocument()
    expect(screen.getByText('personal')).toBeInTheDocument()
  })

  it('renames a selected tag and refreshes data', async () => {
    const user = userEvent.setup()
    const onChanged = vi.fn()
    render(<TagManager onChanged={onChanged} />)

    await screen.findByRole('option', { name: 'work' })
    await user.selectOptions(screen.getByLabelText('选择标签'), 'work')
    await user.type(screen.getByLabelText('新标签名'), 'project')
    await user.click(screen.getByRole('button', { name: '重命名' }))

    await waitFor(() => {
      expect(api.renameTag).toHaveBeenCalledWith({ oldName: 'work', newName: 'project' })
      expect(onChanged).toHaveBeenCalledTimes(1)
      expect(api.listTagSummaries).toHaveBeenCalledTimes(2)
    })
    expect(screen.getByText('标签已重命名')).toBeInTheDocument()
  })

  it('merges a selected tag into the target tag', async () => {
    const user = userEvent.setup()
    render(<TagManager />)

    await screen.findByRole('option', { name: 'personal' })
    await user.selectOptions(screen.getByLabelText('选择标签'), 'personal')
    await user.type(screen.getByLabelText('合并目标'), 'work')
    await user.click(screen.getByRole('button', { name: '合并' }))

    await waitFor(() => {
      expect(api.mergeTags).toHaveBeenCalledWith({ sourceName: 'personal', targetName: 'work' })
    })
    expect(screen.getByText('标签已合并')).toBeInTheDocument()
  })

  it('cleans unused tags and reports API errors', async () => {
    const user = userEvent.setup()
    render(<TagManager />)

    await screen.findByRole('option', { name: 'work' })
    await user.click(screen.getByRole('button', { name: '清理孤立标签' }))
    expect(await screen.findByText('已清理 1 个无引用标签')).toBeInTheDocument()
    expect(api.cleanupUnusedTags).toHaveBeenCalledTimes(1)

    vi.mocked(api.renameTag).mockRejectedValueOnce(new Error('rename failed'))
    await user.selectOptions(screen.getByLabelText('选择标签'), 'work')
    await user.type(screen.getByLabelText('新标签名'), 'broken')
    await user.click(screen.getByRole('button', { name: '重命名' }))

    expect(await screen.findByText('操作失败：Error: rename failed')).toBeInTheDocument()
  })
})
