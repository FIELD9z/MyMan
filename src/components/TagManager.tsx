import { useEffect, useState } from 'react'
import { cleanupUnusedTags, listTagSummaries, mergeTags, renameTag } from '../lib/entities'
import type { TagSummary } from '../types'
import './TagManager.css'

interface Props {
  onChanged?: () => void | Promise<void>
}

export function TagManager({ onChanged }: Props) {
  const [tags, setTags] = useState<TagSummary[]>([])
  const [selected, setSelected] = useState('')
  const [renameTo, setRenameTo] = useState('')
  const [mergeTarget, setMergeTarget] = useState('')
  const [status, setStatus] = useState('正在加载标签...')
  const [isBusy, setIsBusy] = useState(false)

  async function reload() {
    const nextTags = await listTagSummaries()
    setTags(nextTags)
    if (selected && !nextTags.some((tag) => tag.name === selected)) {
      setSelected('')
    }
  }

  useEffect(() => {
    void reload()
      .then(() => setStatus(''))
      .catch((error) => setStatus(`标签加载失败：${String(error)}`))
  }, [])

  async function runChange(action: () => Promise<void>, successMessage: string) {
    setIsBusy(true)
    setStatus('正在处理...')
    try {
      await action()
      await reload()
      await onChanged?.()
      setRenameTo('')
      setMergeTarget('')
      setStatus(successMessage)
    } catch (error) {
      setStatus(`操作失败：${String(error)}`)
    } finally {
      setIsBusy(false)
    }
  }

  function handleRename() {
    void runChange(
      () => renameTag({ oldName: selected, newName: renameTo }),
      '标签已重命名',
    )
  }

  function handleMerge() {
    void runChange(
      () => mergeTags({ sourceName: selected, targetName: mergeTarget }),
      '标签已合并',
    )
  }

  function handleCleanup() {
    setIsBusy(true)
    setStatus('正在清理...')
    void cleanupUnusedTags()
      .then(async (removed) => {
        await reload()
        await onChanged?.()
        setStatus(`已清理 ${removed} 个无引用标签`)
      })
      .catch((error) => setStatus(`操作失败：${String(error)}`))
      .finally(() => setIsBusy(false))
  }

  return (
    <section className="tag-manager" aria-label="标签管理面板">
      <div className="tag-manager-header">
        <div>
          <p className="eyebrow">组织工具</p>
          <h3>标签管理</h3>
        </div>
        <button className="secondary" onClick={handleCleanup} disabled={isBusy}>
          清理孤立标签
        </button>
      </div>

      {tags.length === 0 ? (
        <p className="tag-manager-empty">暂无标签。创建带标签的内容后，可在这里统一整理。</p>
      ) : (
        <>
          <label>
            选择要管理的标签
            <select
              aria-label="选择标签"
              value={selected}
              onChange={(event) => setSelected(event.target.value)}
              disabled={isBusy}
            >
              <option value="">选择标签</option>
              {tags.map((tag) => (
                <option key={tag.name} value={tag.name}>
                  {tag.name}
                </option>
              ))}
            </select>
          </label>

          <div className="tag-manager-actions">
            <label>
              新标签名
              <input
                aria-label="新标签名"
                value={renameTo}
                onChange={(event) => setRenameTo(event.target.value)}
                placeholder="输入新标签名"
                disabled={isBusy}
              />
              <button onClick={handleRename} disabled={isBusy || !selected || !renameTo.trim()}>
                重命名
              </button>
            </label>

            <label>
              合并到
              <input
                aria-label="合并目标"
                value={mergeTarget}
                onChange={(event) => setMergeTarget(event.target.value)}
                placeholder="输入目标标签"
                disabled={isBusy}
              />
              <button onClick={handleMerge} disabled={isBusy || !selected || !mergeTarget.trim()}>
                合并
              </button>
            </label>
          </div>

          <div className="tag-summary-table" role="table" aria-label="标签使用统计">
            <div className="tag-summary-row header" role="row">
              <span role="columnheader">标签</span>
              <span role="columnheader">当前内容</span>
              <span role="columnheader">归档内容</span>
            </div>
            {tags.map((tag) => (
              <div className="tag-summary-row" role="row" key={tag.name}>
                <strong role="cell">{tag.name}</strong>
                <span role="cell">{tag.activeCount}</span>
                <span role="cell">{tag.archivedCount}</span>
              </div>
            ))}
          </div>
        </>
      )}

      <p className="tag-manager-status" aria-live="polite">
        {status}
      </p>
    </section>
  )
}
