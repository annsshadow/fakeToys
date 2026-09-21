import { Card, Timeline, Tag, Empty, Button, message } from 'antd'
import { rollbackVersion } from '../services/api'

interface VersionTimelineProps {
  /** 版本列表 */
  versions: any[]
  /** 回滚成功后的回调 */
  onRollback?: () => void
  /** 当前版本 ID */
  currentVersion?: string | null
}

/**
 * 版本时间线
 *
 * 按时间倒序展示版本快照，并支持一键回滚。
 */
export default function VersionTimeline({
  versions,
  onRollback,
  currentVersion
}: VersionTimelineProps) {
  if (!versions?.length) {
    return (
      <Card title="版本历史">
        <Empty description="暂无版本记录" />
      </Card>
    )
  }

  const handleRollback = async (versionId: string) => {
    try {
      await rollbackVersion(versionId)
      message.success(`已回滚到 ${versionId}`)
      onRollback?.()
    } catch {
      message.error('回滚失败')
    }
  }

  return (
    <Card title="版本历史">
      <Timeline
        mode="left"
        items={versions.map(version => ({
          color: version.version_id === currentVersion ? 'green' : 'blue',
          children: (
            <div>
              <div style={{ fontWeight: 500 }}>
                {version.label}
                {version.version_id === currentVersion && (
                  <Tag color="green" style={{ marginLeft: 8 }}>
                    当前
                  </Tag>
                )}
              </div>
              <div style={{ color: '#888', fontSize: 12 }}>
                {version.created_at} · {version.item_count} 条
              </div>
              {version.description && (
                <div style={{ fontSize: 12 }}>{version.description}</div>
              )}
              {version.version_id !== currentVersion && (
                <Button
                  size="small"
                  type="link"
                  style={{ paddingLeft: 0 }}
                  onClick={() => handleRollback(version.version_id)}
                >
                  回滚到此版本
                </Button>
              )}
            </div>
          )
        }))}
      />
    </Card>
  )
}
