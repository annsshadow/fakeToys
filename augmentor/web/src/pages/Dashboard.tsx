import { useEffect, useState } from 'react'
import { Row, Col, Card, Statistic, Tag, Space, message } from 'antd'
import AugmentForm from '../components/AugmentForm'
import VersionTimeline from '../components/VersionTimeline'
import { getConfig, getModels, getVersions, healthCheck } from '../services/api'
import type { AppConfig, HealthResponse, ModelsResponse, VersionInfo } from '../types/api'

/**
 * 平台仪表盘
 *
 * 汇总健康状态、模型配置、版本历史与快速增强入口，
 * 对应需求 R20 中的「平台管理和监控」视图。
 */
export default function Dashboard() {
  const [health, setHealth] = useState<HealthResponse | null>(null)
  const [config, setConfig] = useState<AppConfig | null>(null)
  const [models, setModels] = useState<ModelsResponse | null>(null)
  const [versions, setVersions] = useState<VersionInfo[]>([])
  const [currentVersion, setCurrentVersion] = useState<string | null>(null)

  const loadVersions = () => {
    getVersions()
      .then(result => {
        const list = result.versions || []
        setVersions(list)
        setCurrentVersion(list.length ? list[0].version_id : null)
      })
      .catch(() => message.error('加载版本列表失败'))
  }

  useEffect(() => {
    healthCheck().then(setHealth).catch(() => message.error('服务健康检查失败'))
    getConfig().then(setConfig).catch(() => message.error('加载配置失败'))
    getModels().then(setModels).catch(() => message.error('加载模型列表失败'))
    loadVersions()
  }, [])

  return (
    <div>
      <Row gutter={16} style={{ marginBottom: 16 }}>
        <Col span={6}>
          <Card>
            <Statistic
              title="服务状态"
              value={health?.status === 'ok' ? '正常' : '未知'}
              valueStyle={{ color: health?.status === 'ok' ? '#3f8600' : '#cf1322' }}
            />
            <div style={{ color: '#888', fontSize: 12 }}>版本 {health?.version || '-'}</div>
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic title="默认模型" value={config?.default_model || '-'} />
            <div style={{ color: '#888', fontSize: 12 }}>
              质量阈值 {config?.quality?.threshold ?? '-'}
            </div>
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic title="可用模型数" value={models?.models?.length || 0} />
            <div style={{ color: '#888', fontSize: 12 }}>
              去重阈值 {config?.dedup?.threshold ?? '-'}
            </div>
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic title="版本快照数" value={versions.length} />
            <div style={{ color: '#888', fontSize: 12 }}>
              并发线程 {config?.augmentation?.num_threads ?? '-'}
            </div>
          </Card>
        </Col>
      </Row>

      <Row gutter={16}>
        <Col span={14}>
          <AugmentForm />
          <Card title="可用模型" style={{ marginTop: 16 }}>
            <Space wrap>
              {(models?.models || []).map((name: string) => (
                <Tag
                  key={name}
                  color={name === models?.default ? 'green' : 'blue'}
                >
                  {name}
                </Tag>
              ))}
            </Space>
          </Card>
        </Col>
        <Col span={10}>
          <VersionTimeline
            versions={versions}
            currentVersion={currentVersion}
            onRollback={loadVersions}
          />
        </Col>
      </Row>
    </div>
  )
}
