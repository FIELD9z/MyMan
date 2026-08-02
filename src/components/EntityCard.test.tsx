import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, expect, it, vi } from 'vitest'
import { entity } from '../test/factories'
import { EntityCard } from './EntityCard'

describe('EntityCard', () => {
  it('shows entity details and triggers archive action', async () => {
    const user = userEvent.setup()
    const testEntity = entity({ id: 'file-1', entityType: 'file', title: 'Project brief', tags: ['work', 'docs'] })
    const onEdit = vi.fn()
    const onArchive = vi.fn()

    render(<EntityCard entity={testEntity} onEdit={onEdit} onArchive={onArchive} />)

    expect(screen.getByText('文件')).toBeInTheDocument()
    expect(screen.getByText('Project brief')).toBeInTheDocument()
    expect(screen.getByText('First summary')).toBeInTheDocument()

    await user.click(screen.getByRole('button', { name: '编辑' }))
    expect(onEdit).toHaveBeenCalledWith(testEntity)

    await user.click(screen.getByRole('button', { name: '归档' }))
    expect(onArchive).toHaveBeenCalledWith('file-1')
  })

  it('shows restore action for archived entities', async () => {
    const user = userEvent.setup()
    const onRestore = vi.fn()

    render(
      <EntityCard
        entity={entity({ id: 'archived-1' })}
        archived
        onEdit={vi.fn()}
        onArchive={vi.fn()}
        onRestore={onRestore}
      />,
    )

    await user.click(screen.getByRole('button', { name: '恢复' }))
    expect(onRestore).toHaveBeenCalledWith('archived-1')
  })
})
