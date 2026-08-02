import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, expect, it, vi } from 'vitest'
import { entity } from '../test/factories'
import { Composer } from './Composer'

describe('Composer', () => {
  it('creates an entity request from form fields', async () => {
    const user = userEvent.setup()
    const onSave = vi.fn().mockResolvedValue(undefined)

    render(<Composer onSave={onSave} onCancelEdit={vi.fn()} />)

    await user.selectOptions(screen.getByLabelText('类型'), 'task')
    await user.type(screen.getByLabelText('标签'), 'Work, home')
    await user.type(screen.getByLabelText('标题'), 'Ship baseline')
    await user.type(screen.getByLabelText('Markdown 内容 / 文件描述'), 'First line\nSecond line')
    await user.click(screen.getByRole('button', { name: '保存' }))

    await waitFor(() => expect(onSave).toHaveBeenCalledTimes(1))
    expect(onSave).toHaveBeenCalledWith({
      entityType: 'task',
      title: 'Ship baseline',
      summary: 'First line',
      content: 'First line\nSecond line',
      tags: ['Work', 'home'],
    })
  })

  it('keeps form values and shows an error when saving fails', async () => {
    const user = userEvent.setup()
    const onSave = vi.fn().mockRejectedValue(new Error('database unavailable'))

    render(<Composer onSave={onSave} onCancelEdit={vi.fn()} />)

    await user.type(screen.getByLabelText('标题'), 'Unsaved note')
    await user.type(screen.getByLabelText('Markdown 内容 / 文件描述'), 'Keep this body')
    await user.click(screen.getByRole('button', { name: '保存' }))

    expect(await screen.findByRole('alert')).toHaveTextContent('database unavailable')
    expect(screen.getByLabelText('标题')).toHaveValue('Unsaved note')
    expect(screen.getByLabelText('Markdown 内容 / 文件描述')).toHaveValue('Keep this body')
    expect(screen.getByRole('button', { name: '保存' })).toBeEnabled()
  })

  it('disables create submit until a title exists', () => {
    render(<Composer onSave={vi.fn()} onCancelEdit={vi.fn()} />)

    expect(screen.getByRole('button', { name: '保存' })).toBeDisabled()
  })

  it('edits an existing entity and supports cancel', async () => {
    const user = userEvent.setup()
    const onSave = vi.fn().mockResolvedValue(undefined)
    const onCancelEdit = vi.fn()

    render(
      <Composer
        editing={entity({ id: 'task-1', entityType: 'task', title: 'Old task', tags: ['work'] })}
        onSave={onSave}
        onCancelEdit={onCancelEdit}
      />,
    )

    expect(screen.getByText('类型：任务')).toBeInTheDocument()
    await user.clear(screen.getByLabelText('标题'))
    await user.type(screen.getByLabelText('标题'), 'Updated task')
    await user.click(screen.getByRole('button', { name: '更新' }))

    await waitFor(() => expect(onSave).toHaveBeenCalledTimes(1))
    expect(onSave).toHaveBeenCalledWith({
      id: 'task-1',
      title: 'Updated task',
      summary: 'First summary',
      content: 'First summary\nSecond line',
      tags: ['work'],
    })

    await user.click(screen.getByRole('button', { name: '取消' }))
    expect(onCancelEdit).toHaveBeenCalledTimes(1)
  })
})
