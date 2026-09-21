import { useEffect, useState } from 'react'
import {
  Card,
  Form,
  Input,
  Button,
  Space,
  Table,
  Tag,
  Alert,
  Row,
  Col,
  Statistic,
  message
} from 'antd'
import {
  getMultimodalFormats,
  processMultimodal,
  scanMultimodal
} from '../services/api'

/**
 * 多模态数据处理
 *
 * 支持单条融合与目录扫描，对应需求 R19。
 */
export default function Multimodal() {
  const [formats, setFormats] = useState<any>(null)
  const [singleResult, setSingleResult] = useState<any>(null)
  const [scanResult, setScanResult] = useState<any>(null)
  const [loading, setLoading] = useState(false)
  const [form] = Form.useForm()

  useEffect(() => {
    getMultimodalFormats()
      .then(setFormats)
      .catch(() => message.error('加载多模态格式失败'))
  }, [])

  const handleProcess = async (values: any) => {
    setLoading(true)
    try {
      setSingleResult(
        await processMultimodal(values.text, values.image, values.audio)
      )
    } catch (error) {
      message.error('多模态处理失败')
    } finally {
      setLoading(false)
    }
  }

  const handleScan = async (values: any) => {
    setLoading(true)
    try {
      setScanResult(await scanMultimodal(values.directory))
    } catch (error) {
      message.error('目录扫描失败，请确认目录存在')
    } finally {
      setLoading(false)
    }
  }

  const recordColumns = [
    { title: '文本', dataIndex: 'text', key: 'text', ellipsis: true },
    {
      title: '模态',
      dataIndex: 'modalities',
      key: 'modalities',
      render: (modalities: string[]) => (
        <>
          {(modalities || []).map(item => (
            <Tag key={item} color="blue">
              {item}
            </Tag>
          ))}
        </>
      )
    },
    {
      title: '状态',
      dataIndex: 'valid',
      key: 'valid',
      render: (valid: boolean) => (
        <Tag color={valid ? 'green' : 'red'}>{valid ? '有效' : '无效'}</Tag>
      )
    },
    {
      title: '错误',
      dataIndex: 'errors',
      key: 'errors',
      render: (errors: string[]) => (errors || []).join('; ')
    }
  ]

  return (
    <div>
      <Alert
        type="info"
        showIcon
        style={{ marginBottom: 16 }}
        message="多模态数据处理"
        description={
          formats
            ? `支持图像格式：${formats.image_extensions.join('、')}；支持音频格式：${formats.audio_extensions.join('、')}`
            : '正在加载支持的格式…'
        }
      />

      <Row gutter={16}>
        <Col span={12}>
          <Card title="单条处理">
            <Form form={form} layout="vertical" onFinish={handleProcess}>
              <Form.Item name="text" label="文本说明">
                <Input.TextArea rows={3} placeholder="可选，作为该记录的文本描述" />
              </Form.Item>
              <Form.Item name="image" label="图像路径">
                <Input placeholder="例如 data/images/sample.png" />
              </Form.Item>
              <Form.Item name="audio" label="音频路径">
                <Input placeholder="例如 data/audio/sample.wav" />
              </Form.Item>
              <Button type="primary" htmlType="submit" loading={loading}>
                处理
              </Button>
            </Form>

            {singleResult && (
              <div style={{ marginTop: 16 }}>
                <Space wrap>
                  {(singleResult.modalities || []).map((item: string) => (
                    <Tag key={item} color="blue">
                      {item}
                    </Tag>
                  ))}
                  <Tag color={singleResult.valid ? 'green' : 'red'}>
                    {singleResult.valid ? '有效' : '无效'}
                  </Tag>
                </Space>
                {singleResult.image && (
                  <Card size="small" title="图像信息" style={{ marginTop: 12 }}>
                    <Statistic
                      title="尺寸"
                      value={`${singleResult.image.width} × ${singleResult.image.height}`}
                    />
                    <div style={{ color: '#888', fontSize: 12 }}>
                      格式 {singleResult.image.format} · {singleResult.image.size_bytes} 字节
                    </div>
                  </Card>
                )}
                {singleResult.audio && (
                  <Card size="small" title="音频信息" style={{ marginTop: 12 }}>
                    <Statistic
                      title="时长"
                      value={singleResult.audio.duration?.toFixed?.(2) ?? 0}
                      suffix="秒"
                    />
                    <div style={{ color: '#888', fontSize: 12 }}>
                      采样率 {singleResult.audio.sample_rate} Hz · {singleResult.audio.channels} 声道
                    </div>
                  </Card>
                )}
              </div>
            )}
          </Card>
        </Col>

        <Col span={12}>
          <Card title="目录扫描">
            <Form layout="vertical" onFinish={handleScan}>
              <Form.Item
                name="directory"
                label="目录路径"
                rules={[{ required: true, message: '请输入目录路径' }]}
              >
                <Input placeholder="例如 data/multimodal" />
              </Form.Item>
              <Button type="primary" htmlType="submit" loading={loading}>
                扫描目录
              </Button>
            </Form>

            {scanResult && (
              <>
                <Row gutter={16} style={{ marginTop: 16 }}>
                  <Col span={8}>
                    <Statistic title="记录数" value={scanResult.total_records} />
                  </Col>
                  <Col span={8}>
                    <Statistic title="有效" value={scanResult.valid_records} />
                  </Col>
                  <Col span={8}>
                    <Statistic title="错误" value={scanResult.error_count} />
                  </Col>
                </Row>
                <Table
                  size="small"
                  style={{ marginTop: 16 }}
                  rowKey={(_, index) => String(index)}
                  columns={recordColumns}
                  dataSource={scanResult.records || []}
                  pagination={{ pageSize: 5 }}
                />
              </>
            )}
          </Card>
        </Col>
      </Row>
    </div>
  )
}
