import { useEffect, useState } from 'react'
import { cleanupUnusedTags, listTagSummaries, mergeTags, renameTag } from '../lib/entities'
import type { TagSummary } from '../types'

interface Props {
  onChanged?: () => void
}

export function TagManager({ onChanged }: Props) {
  const [tags, setTags] = useState<TagSummary[]>([])
  const [selected, setSelected] = useState('')
  const [renameTo, setRenameTo] = useState('')
  const [mergeTarget, setMergeTarget] = useState('')
  const [status, setStatus] = useState('')

  async function reload() {
    setTags(await listTagSummaries())
  }

  useEffect(() => {
    void reload()
  }, [])

  async function handleRename() {
    await renameTag({ oldName: selected, newName: renameTo })
    setStatus('标签已重命名')
    await reload()
    onChanged?.()
  }

  async function handleMerge() {
    await mergeTags({ sourceName: selected, targetName: mergeTarget })
    setStatus('标签已合并')
    await reload()
    onChanged?.()
  }

  async function handleCleanup() {
    const removed = await cleanupUnusedTags()
    setStatus(`已清理 ${removed} 个无引用标签`)
    await reload()
    onChanged?.()
  }

  return (
    <section className="tag-manager">
      <h3>标签管理</h3>
      <select aria-label="选择标签" value={selected} onChange={(event) => setSelected(event.target.value)}>
        <option value="">选择标签</option>
        {tags.map((tag) => <option key={tag.name}>{tag.name}</option>)}
      </select>
      <input aria-label="新标签名" value={renameTo} onChange={(event) => setRenameTo(event.target.value)} placeholder="新标签名" />
      <button onClick={handleRename} disabled={!selected || !renameTo}>重命名</button>
      <input aria-label="合并目标" value={mergeTarget} onChange={(event) => setMergeTarget(event.target.value)} placeholder="目标标签" />
      <button onClick={handleMerge} disabled={!selected || !mergeTarget}>合并</button>
      <button onClick={handleCleanup}>清理孤立标签</button>
      <ul>
        {tags.map((tag) => <li key={tag.name}>{tag.name}: {tag.activeCount} / {tag.archivedCount}</li>)}
      </ul>
      <p aria-live="polite">{status}</p>
    </section>
  )
}
