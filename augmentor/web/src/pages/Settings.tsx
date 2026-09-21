import { useState, useEffect } from 'react'
import { Card, Form, InputNumber, Select, Switch, Button, message, Space } from 'antd'
import { SaveOutlined } from '@ant-design/icons'
import { getConfig, updateConfig, getModels } from '../services/api'

export default function Settings() {
  const [config, setConfig] = useState<any>(null)
  const [models, setModels] = useState<string[]>([])
  const [defaultModel, setDefaultModel] = useState('')
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    loadConfig()
    loadModels()
  }, [])

  const loadConfig = async () => {
    try {
      const result = await getConfig()
      setConfig(result)
    } catch {
      message.error('加载配置失败')
    }
  }

  const loadModels = async () => {
    try {
      const result = await getModels()
      setModels(result.models)
      setDefaultModel(result.default)
    } catch {
      message.error('加载模型列表失败')
    }
  }

  const handleSave = async () => {
    setLoading(true)
    try {
      await updateConfig({
        default_model: defaultModel,
        augmentation: config?.augmentation,
        quality: config?.quality,
        dedup: config?.dedup,
        export: config?.export,
        vector: config?.vector,
        rag: config?.rag,
        multimodal: config?.multimodal
      })
      message.success('配置已保存到 config.yaml，部分配置需要重启服务生效')
    } catch {
      message.error('保存配置失败')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div>
      <Card title="模型配置" style={{ marginBottom: 16 }}>
        <Form layout="vertical">
          <Form.Item label="默认模型">
            <Select
              value={defaultModel}
              onChange={setDefaultModel}
              style={{ width: 300 }}
            >
              {models.map(model => (
                <Select.Option key={model} value={model}>{model}</Select.Option>
              ))}
            </Select>
          </Form.Item>
        </Form>
      </Card>

      <Card title="增强配置" style={{ marginBottom: 16 }}>
        <Form layout="vertical">
          <Space size="large">
            <Form.Item label="每条种子生成的变体数量">
              <InputNumber
                value={config?.augmentation?.variants_per_seed}
                min={1}
                max={20}
                onChange={(value) => setConfig({
                  ...config,
                  augmentation: { ...config.augmentation, variants_per_seed: value }
                })}
              />
            </Form.Item>
            <Form.Item label="并发线程数">
              <InputNumber
                value={config?.augmentation?.num_threads}
                min={1}
                max={100}
                onChange={(value) => setConfig({
                  ...config,
                  augmentation: { ...config.augmentation, num_threads: value }
                })}
              />
            </Form.Item>
          </Space>
        </Form>
      </Card>

      <Card title="质量检查" style={{ marginBottom: 16 }}>
        <Form layout="vertical">
          <Space size="large">
            <Form.Item label="启用质量检查">
              <Switch
                checked={config?.quality?.enabled}
                onChange={(checked) => setConfig({
                  ...config,
                  quality: { ...config.quality, enabled: checked }
                })}
              />
            </Form.Item>
            <Form.Item label="质量阈值">
              <InputNumber
                value={config?.quality?.threshold}
                min={0}
                max={1}
                step={0.1}
                onChange={(value) => setConfig({
                  ...config,
                  quality: { ...config.quality, threshold: value }
                })}
              />
            </Form.Item>
          </Space>
        </Form>
      </Card>

      <Card title="智能去重" style={{ marginBottom: 16 }}>
        <Form layout="vertical">
          <Space size="large">
            <Form.Item label="启用水重">
              <Switch
                checked={config?.dedup?.enabled}
                onChange={(checked) => setConfig({
                  ...config,
                  dedup: { ...config.dedup, enabled: checked }
                })}
              />
            </Form.Item>
            <Form.Item label="相似度阈值">
              <InputNumber
                value={config?.dedup?.threshold}
                min={0}
                max={1}
                step={0.1}
                onChange={(value) => setConfig({
                  ...config,
                  dedup: { ...config.dedup, threshold: value }
                })}
              />
            </Form.Item>
          </Space>
        </Form>
      </Card>

      <Card title="配置说明">
        <p>配置文件位于: <code>config.yaml</code></p>
        <p>修改配置后需要重启服务才能生效。</p>
        <p>环境变量配置：</p>
        <ul>
          <li><code>BAIDU_API_KEY</code> - 百度 API Key</li>
          <li><code>BAIDU_SECRET_KEY</code> - 百度 Secret Key</li>
          <li><code>OPENAI_API_KEY</code> - OpenAI API Key</li>
        </ul>
      </Card>

      <div style={{ marginTop: 16, textAlign: 'right' }}>
        <Button type="primary" icon={<SaveOutlined />} onClick={handleSave} loading={loading}>
          保存配置
        </Button>
      </div>
    </div>
  )
}
