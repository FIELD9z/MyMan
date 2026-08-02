import type { DashboardSummary, EntityType } from '../types'

export interface EntityTypeConfig {
  type: EntityType
  label: string
  description: string
  summaryKey: keyof DashboardSummary
}

export const entityTypeConfigs: EntityTypeConfig[] = [
  {
    type: 'note',
    label: '随手记',
    description: '快速捕捉想法、碎片信息和临时记录',
    summaryKey: 'notes',
  },
  {
    type: 'task',
    label: '任务',
    description: '可执行事项、截止时间和后续安排',
    summaryKey: 'tasks',
  },
  {
    type: 'event',
    label: '日程',
    description: '时间块、会议和应用内提醒',
    summaryKey: 'events',
  },
  {
    type: 'knowledge',
    label: '知识',
    description: 'Markdown 长文、资料沉淀和双向关联',
    summaryKey: 'knowledge',
  },
  {
    type: 'file',
    label: '文件',
    description: '文件路径、描述、标签和基础元数据',
    summaryKey: 'files',
  },
]

export function entityTypeLabel(type: EntityType): string {
  return entityTypeConfigs.find((item) => item.type === type)?.label ?? type
}

export function summaryKeyForType(type: EntityType): keyof DashboardSummary {
  return entityTypeConfigs.find((item) => item.type === type)?.summaryKey ?? 'notes'
}
