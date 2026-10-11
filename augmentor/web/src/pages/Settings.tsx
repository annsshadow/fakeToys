import { useState, useEffect } from 'react'
import {
  Alert,
  Button,
  Card,
  Col,
  Form,
  Input,
  InputNumber,
  Row,
  Select,
  Space,
  Statistic,
  Switch,
  Tag,
  message
} from 'antd'
import { SaveOutlined } from '@ant-design/icons'
import {
  apiErrorDetail,
  autoConfigRecommend,
  getApiKey,
  getConfig,
  getDataFiles,
  getModels,
  setApiKey,
  updateConfig
} from '../services/api'
import type { AppConfig, AutoConfigResponse } from '../types/api'

export default function Settings() {
  const [config, setConfig] = useState<AppConfig | null>(null)
  const [models, setModels] = useState<string[]>([])
  const [defaultModel, setDefaultModel] = useState('')
  const [loading, setLoading] = useState(false)
  const [files, setFiles] = useState<string[]>([])
  const [recommendFile, setRecommendFile] = useState('')
  const [recommendation, setRecommendation] = useState<AutoConfigResponse | null>(null)
  const [recommending, setRecommending] = useState(false)
  /**
   * 写操作鉴权那一格：输入值与「这台浏览器现在存没存」分开记
   *
   * 初值取 `getApiKey()`（读 localStorage）而不是空串：页面重开后输入框必须如实反映
   * 「已经存过」，否则用户会以为每次都要重填。`apiKeyStored` 单独一格是因为输入框是
   * 密码型、看不见内容，而「存着的那份不对」与「根本没存」在 401 的提示里是两句话
   * （见 `apiErrorDetail` 的 401 那一支）。
   */
  const [apiKeyInput, setApiKeyInput] = useState(getApiKey)
  const [apiKeyStored, setApiKeyStored] = useState(() => Boolean(getApiKey()))

  /**
   * 局部更新配置
   *
   * 两点不能省：
   * - 用**函数式更新**而不是展开闭包里的 `config`：同一屏上连着改几个字段时，
   *   后一次不会把前一次覆盖掉。
   * - `config` 还没加载完（或加载失败）时原样返回。原来直接写
   *   `setConfig({ ...config, quality: { ...config.quality, ... } })`，
   *   `config` 为 null 时会在用户第一次交互时抛 `Cannot read properties of null`。
   */
  const patch = (fn: (prev: AppConfig) => AppConfig) =>
    setConfig(prev => (prev ? fn(prev) : prev))

  useEffect(() => {
    loadConfig()
    loadModels()
    loadFiles()
  }, [])

  const loadConfig = async () => {
    try {
      const result = await getConfig()
      setConfig(result)
    } catch (err) {
      message.error(apiErrorDetail(err, '加载配置失败'))
    }
  }

  const loadModels = async () => {
    try {
      const result = await getModels()
      setModels(result.models)
      setDefaultModel(result.default)
    } catch (err) {
      message.error(apiErrorDetail(err, '加载模型列表失败'))
    }
  }

  const loadFiles = async () => {
    try {
      const result = await getDataFiles()
      setFiles(result.files.map(f => f.name))
    } catch {
      // 只影响「按数据推荐配置」这一张卡：配置表单本身走的是 loadConfig，不受牵连
      console.error('加载文件列表失败')
    }
  }

  const handleRecommend = async () => {
    // 没选文件就不发：`input_file` 是必填，空串到服务端只会换来一句「文件不存在」，
    // 而那句话并不说明真正的原因是我们没给用户选文件。
    if (!recommendFile) {
      message.warning('请先选择要观测的数据集')
      return
    }
    setRecommending(true)
    try {
      setRecommendation(await autoConfigRecommend(recommendFile))
    } catch (err) {
      message.error(apiErrorDetail(err, '推荐配置失败'))
    } finally {
      setRecommending(false)
    }
  }

  /**
   * 把推荐值填进**表单**，不直接保存
   *
   * 故意分两步：`auto-config` 说的是「按这份数据的观测该用哪档」，写盘要过
   * `POST /api/config`。中间留着「我看了建议、我决定不改」那一步，用户也可以只认
   * 其中一个阈值再保存。`recommended_sample_size` 不在这里落地——本表单没有对应字段，
   * 它只作为读数显示。
   */
  const applyRecommendation = () => {
    if (!recommendation) return
    patch(c => ({
      ...c,
      quality: { ...c.quality, threshold: recommendation.quality_threshold },
      dedup: { ...c.dedup, threshold: recommendation.dedup_threshold }
    }))
    message.success('两个阈值已填进表单，确认无误后再点「保存配置」')
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
    } catch (err) {
      message.error(apiErrorDetail(err, '保存配置失败'))
    } finally {
      setLoading(false)
    }
  }

  /**
   * 存 / 清 写操作鉴权用的那份 key
   *
   * 两个动作都不发请求：`setApiKey` 只写 localStorage，真正被校验是在**下一个**写操作
   * 请求上（拦截器把这份 key 挂到请求头，服务端 `verify_api_key` 拿它与环境变量
   * `AUGMENTOR_API_KEY` 做定长比较）。所以这里刻意**不**报「校验通过」——本层没有
   * 能力证明这份 key 是对的，说「已存进这台浏览器」才是真话；填错了要等第一次写操作
   * 才 401，那句错误会指回这张卡。
   */
  const saveApiKey = () => {
    const trimmed = apiKeyInput.trim()
    setApiKey(apiKeyInput)
    setApiKeyStored(Boolean(trimmed))
    // 空输入走的是 `removeItem`，这时候说「已存进」就是假话
    message.success(
      trimmed
        ? '已存进这台浏览器（localStorage 明文），后续每个请求都会带上它'
        : '输入是空的：等于清除，后续请求不再送鉴权头'
    )
  }

  const clearApiKey = () => {
    setApiKey('')
    setApiKeyInput('')
    setApiKeyStored(false)
    message.success('已清除：后续请求不再送鉴权头')
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
                onChange={(value) =>
                  patch(c => ({
                    ...c,
                    augmentation: {
                      ...c.augmentation,
                      variants_per_seed: value ?? c.augmentation.variants_per_seed
                    }
                  }))
                }
              />
            </Form.Item>
            <Form.Item label="并发线程数">
              <InputNumber
                value={config?.augmentation?.num_threads}
                min={1}
                max={100}
                onChange={(value) =>
                  patch(c => ({
                    ...c,
                    augmentation: {
                      ...c.augmentation,
                      num_threads: value ?? c.augmentation.num_threads
                    }
                  }))
                }
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
                onChange={(checked) =>
                  patch(c => ({ ...c, quality: { ...c.quality, enabled: checked } }))
                }
              />
            </Form.Item>
            <Form.Item label="质量阈值">
              <InputNumber
                value={config?.quality?.threshold}
                min={0}
                max={1}
                step={0.1}
                onChange={(value) =>
                  patch(c => ({
                    ...c,
                    quality: { ...c.quality, threshold: value ?? c.quality.threshold }
                  }))
                }
              />
            </Form.Item>
          </Space>
        </Form>
      </Card>

      <Card title="智能去重" style={{ marginBottom: 16 }}>
        <Form layout="vertical">
          <Space size="large">
            <Form.Item label="启用去重">
              <Switch
                checked={config?.dedup?.enabled}
                onChange={(checked) =>
                  patch(c => ({ ...c, dedup: { ...c.dedup, enabled: checked } }))
                }
              />
            </Form.Item>
            <Form.Item label="相似度阈值">
              <InputNumber
                value={config?.dedup?.threshold}
                min={0}
                max={1}
                step={0.1}
                onChange={(value) =>
                  patch(c => ({
                    ...c,
                    dedup: { ...c.dedup, threshold: value ?? c.dedup.threshold }
                  }))
                }
              />
            </Form.Item>
          </Space>
        </Form>
      </Card>

      <Card title="按数据推荐配置" style={{ marginBottom: 16 }}>
        <Space wrap style={{ marginBottom: 12 }}>
          <Select
            style={{ width: 300 }}
            value={recommendFile}
            onChange={setRecommendFile}
            placeholder="选择用来观测的数据集"
            options={files.map(f => ({ value: f, label: f }))}
          />
          <Button loading={recommending} onClick={handleRecommend}>
            取推荐值
          </Button>
        </Space>
        {recommendation ? (
          <>
            <Row gutter={16}>
              <Col span={8}>
                <Card>
                  <Statistic title="质量阈值" value={recommendation.quality_threshold} />
                </Card>
              </Col>
              <Col span={8}>
                <Card>
                  <Statistic title="去重阈值" value={recommendation.dedup_threshold} />
                </Card>
              </Col>
              <Col span={8}>
                <Card>
                  <Statistic
                    title="建议抽样条数"
                    value={recommendation.recommended_sample_size}
                  />
                </Card>
              </Col>
            </Row>
            <Space wrap style={{ marginTop: 12 }}>
              {/* `source_stats` 是观测值原样带回，实测 `quality_pass_rate` 可以为 null
                  （那份夹具没有可判质量的字段）。null 要显式写成「无观测」，
                  留空或写成 0 都会把「没测出来」报成「测出来是零」。 */}
              {Object.entries(recommendation.source_stats).map(([key, value]) => (
                <Tag key={key} color="blue">
                  {key}: {value === null ? '无观测' : String(value)}
                </Tag>
              ))}
            </Space>
            {recommendation.reasoning.length > 0 && (
              <ul>
                {recommendation.reasoning.map((line, i) => (
                  <li key={i}>{line}</li>
                ))}
              </ul>
            )}
            <Button onClick={applyRecommendation}>填进上面的表单（不直接保存）</Button>
          </>
        ) : (
          <Alert
            type="info"
            showIcon
            message="还没有推荐值"
            description="这一步只读数据、不写配置；取到建议后仍要点「保存配置」才落盘。"
          />
        )}
      </Card>

      <Card title="写操作鉴权" style={{ marginBottom: 16 }}>
        <Form layout="vertical">
          <Form.Item label="API Key">
            <Space.Compact style={{ width: 520 }}>
              <Input.Password
                aria-label="写操作鉴权 API Key"
                value={apiKeyInput}
                onChange={e => setApiKeyInput(e.target.value)}
                placeholder="服务端设了 AUGMENTOR_API_KEY 时才需要填"
              />
              <Button type="primary" onClick={saveApiKey}>存进这台浏览器</Button>
              <Button onClick={clearApiKey}>清除</Button>
            </Space.Compact>
          </Form.Item>
        </Form>
        <Tag color={apiKeyStored ? 'green' : 'default'}>
          {apiKeyStored ? '这台浏览器已存了一份' : '这台浏览器没有存 key'}
        </Tag>
        <div style={{ marginTop: 8, fontSize: 12, opacity: 0.75 }}>
          什么时候要填：只有服务端设了环境变量 <code>AUGMENTOR_API_KEY</code> 才校验。没设时
          留空即可，填了也不会被拒——服务端那一格看见环境变量为空就直接放过。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          影响面（L233 实测）：契约面 75 条操作里有 24 条挂着这道门禁，其中 21 条是页面上已
          接线的动作——保存配置、上传、导出与批量导出、启动增强、版本创建与回滚、改删单条、
          六只数据集变换、备份那三只、数据集登记、质量画像。key 不对或没填，它们一律回 401，
          错误提示会指回这张卡。
        </div>
        <div style={{ marginTop: 4, fontSize: 12, opacity: 0.75 }}>
          存哪儿：这台浏览器的 localStorage，<strong>明文</strong>。换浏览器、换机器、清站点数据
          都要重填；同源脚本读得到它，所以它不是保密措施，也不替代 HTTPS。它更不是登录——
          服务端只做一次定长比较，没有用户身份、没有过期、没有权限分级。
        </div>
      </Card>

      <Card title="配置说明">
        <p>配置文件位于: <code>config.yaml</code></p>
        <p>修改配置后需要重启服务才能生效。</p>
        <p>环境变量配置：</p>
        <ul>
          <li><code>BAIDU_API_KEY</code> - 百度 API Key</li>
          <li><code>BAIDU_SECRET_KEY</code> - 百度 Secret Key</li>
          <li><code>OPENAI_API_KEY</code> - OpenAI API Key</li>
          <li>
            <code>AUGMENTOR_API_KEY</code> - 写操作鉴权（服务端那一半；填进上面那张卡才生效）
          </li>
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
