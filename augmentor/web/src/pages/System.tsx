import { useEffect, useState } from 'react'
import { Alert, Button, Card, Col, Row, Space, Statistic, Table, Tag, message } from 'antd'
import { apiErrorDetail, getServiceStatus, getVectorStatus } from '../services/api'
import type { StatusResponse, VectorStatusResponse } from '../types/api'

/**
 * 系统状态
 *
 * 展示 `GET /api/status`：与仪表盘上的 `GET /api/health` 是两件事 ——
 * health 只报存活（给容器探针用，刻意不含可能失败的字段），status 会真的
 * 构造管道并逐个检查可选依赖，因此能回答「当前环境缺什么、哪些功能会降级」。
 *
 * 下面那张卡是 `GET /api/dataset/vector-status`：向量库的闸门生效值与磁盘上到底有没有
 * 一份可读回的索引。它和服务状态是两回事，所以单独取、单独刷新。
 */
export default function System() {
  const [status, setStatus] = useState<StatusResponse | null>(null)
  const [loading, setLoading] = useState(false)
  const [vector, setVector] = useState<VectorStatusResponse | null>(null)
  const [vectorLoading, setVectorLoading] = useState(false)

  const refresh = async () => {
    setLoading(true)
    try {
      setStatus(await getServiceStatus())
    } catch (err) {
      message.error(apiErrorDetail(err, '获取服务状态失败，请确认后端已启动'))
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    refresh()
  }, [])

  const refreshVector = async () => {
    setVectorLoading(true)
    try {
      setVector(await getVectorStatus())
    } catch (err) {
      message.error(apiErrorDetail(err, '获取向量库状态失败'))
    } finally {
      setVectorLoading(false)
    }
  }

  // 向量库那一面单独取、单独报错：它失败不该把整页服务状态一起拖掉
  useEffect(() => {
    refreshVector()
  }, [])

  const installedRows = Object.entries(status?.dependencies.installed || {}).map(
    ([name, available]) => ({ name, available })
  )

  return (
    <div>
      <Card
        title="服务状态"
        style={{ marginBottom: 16 }}
        extra={
          <Button onClick={refresh} loading={loading}>
            刷新
          </Button>
        }
      >
        <Row gutter={16}>
          <Col span={6}>
            <Statistic
              title="运行状态"
              value={status?.status === 'ok' ? '正常' : '未知'}
              valueStyle={{ color: status?.status === 'ok' ? '#3f8600' : '#cf1322' }}
            />
          </Col>
          <Col span={6}>
            <Statistic title="版本" value={status?.version || '-'} />
          </Col>
          <Col span={6}>
            <Statistic title="默认模型" value={status?.model_default || '-'} />
          </Col>
          <Col span={6}>
            <Statistic
              title="模型可用"
              value={status ? (status.model_available ? '是' : '否') : '-'}
              valueStyle={{
                color: status?.model_available ? '#3f8600' : '#cf1322'
              }}
            />
          </Col>
        </Row>
      </Card>

      {status && (
        <>
          <Alert
            type={status.dependencies.all_required_present ? 'success' : 'warning'}
            showIcon
            style={{ marginBottom: 16 }}
            message={
              status.dependencies.all_required_present
                ? `必需依赖齐备（共 ${status.dependencies.available_count} 项可用）`
                : '缺少必需依赖，部分功能将不可用'
            }
            description={
              status.dependencies.all_required_present ? undefined : (
                <span>缺失：{status.dependencies.missing.join('、') || '未知'}</span>
              )
            }
          />

          <Row gutter={16}>
            <Col span={12}>
              <Card title="依赖明细">
                <Table
                  size="small"
                  rowKey="name"
                  pagination={false}
                  columns={[
                    { title: '依赖', dataIndex: 'name' },
                    {
                      title: '状态',
                      dataIndex: 'available',
                      width: 100,
                      render: (available: boolean) => (
                        <Tag color={available ? 'green' : 'red'}>
                          {available ? '已安装' : '未安装'}
                        </Tag>
                      )
                    }
                  ]}
                  dataSource={installedRows}
                />
              </Card>
            </Col>
            <Col span={12}>
              <Card title="降级功能">
                {(status.dependencies.degraded_features || []).length > 0 ? (
                  <Space wrap>
                    {status.dependencies.degraded_features.map(feature => (
                      <Tag key={feature} color="orange">
                        {feature}
                      </Tag>
                    ))}
                  </Space>
                ) : (
                  <Alert type="success" showIcon message="没有功能因依赖缺失而降级" />
                )}
              </Card>
            </Col>
          </Row>
        </>
      )}

      <Card
        title="向量库状态"
        style={{ marginTop: 16 }}
        extra={
          <Button onClick={refreshVector} loading={vectorLoading}>
            刷新
          </Button>
        }
      >
        {vector ? (
          <>
            <Row gutter={16}>
              <Col span={6}>
                <Statistic
                  title="自动建库"
                  value={vector.enabled ? '开' : '关'}
                  valueStyle={{ color: vector.enabled ? '#3f8600' : '#cf1322' }}
                />
              </Col>
              <Col span={6}>
                <Statistic
                  title="磁盘索引"
                  value={vector.exists ? '已存在' : '不存在'}
                  valueStyle={{ color: vector.exists ? '#3f8600' : '#cf1322' }}
                />
              </Col>
              <Col span={6}>
                <Statistic title="向量条数" value={vector.count} />
              </Col>
              <Col span={6}>
                <Statistic title="维度" value={vector.dimension} />
              </Col>
            </Row>
            <Space wrap style={{ marginTop: 12 }}>
              <Tag color="blue">backend: {vector.backend}</Tag>
              <Tag color="purple">collection: {vector.collection}</Tag>
              <Tag>storage_dir: {vector.storage_dir || '未配置'}</Tag>
            </Space>
          </>
        ) : (
          <Alert
            type="warning"
            showIcon
            message="向量库状态未取到"
            description="上面那张服务状态不受影响：两面各自独立取。"
          />
        )}
      </Card>
    </div>
  )
}
