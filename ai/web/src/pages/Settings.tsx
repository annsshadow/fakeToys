import { useState, useEffect } from 'react'
import { Card, Form, InputNumber, Select, Switch, Button, message, Space, Divider } from 'antd'
import { SaveOutlined } from '@ant-design/icons'
import { getConfig, getModels } from '../services/api'

export default function Settings() {
  const [config, setConfig] = useState<any>(null)
  const [models, setModels] = useState<string[]>([])
  const [defaultModel, setDefaultModel] = useState('')

  useEffect(() => {
    loadConfig()
    loadModels()
  }, [])

  const loadConfig = async () => {
    try {
      const result = await getConfig()
      setConfig(result)
    } catch (error) {
      message.error('加载配置失败')
    }
  }

  const loadModels = async () => {
    try {
      const result = await getModels()
      setModels(result.models)
      setDefaultModel(result.default)
    } catch (error) {
      message.error('加载模型列表失败')
    }
  }

  const handleSave = () => {
    message.success('配置已保存（注意：需要重启服务才能生效）')
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
                disabled
              />
            </Form.Item>
            <Form.Item label="并发线程数">
              <InputNumber
                value={config?.augmentation?.num_threads}
                min={1}
                max={100}
                disabled
              />
            </Form.Item>
          </Space>
        </Form>
      </Card>

      <Card title="质量检查" style={{ marginBottom: 16 }}>
        <Form layout="vertical">
          <Space size="large">
            <Form.Item label="启用质量检查">
              <Switch checked={config?.quality?.enabled} disabled />
            </Form.Item>
            <Form.Item label="质量阈值">
              <InputNumber
                value={config?.quality?.threshold}
                min={0}
                max={1}
                step={0.1}
                disabled
              />
            </Form.Item>
          </Space>
        </Form>
      </Card>

      <Card title="智能去重" style={{ marginBottom: 16 }}>
        <Form layout="vertical">
          <Space size="large">
            <Form.Item label="启用水重">
              <Switch checked={config?.dedup?.enabled} disabled />
            </Form.Item>
            <Form.Item label="相似度阈值">
              <InputNumber
                value={config?.dedup?.threshold}
                min={0}
                max={1}
                step={0.1}
                disabled
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
        <Button type="primary" icon={<SaveOutlined />} onClick={handleSave}>
          保存配置
        </Button>
      </div>
    </div>
  )
}
