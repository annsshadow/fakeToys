/**
 * Settings 页的渲染面（L105，A206）
 *
 * 配置管理页此前零组件测试。本用例把「读配置→渲染到各控件→改字段→保存时
 * 把什么发给服务层」这条链从「源码里写了」升级成「真的连起来了」，尤其钉住
 * 三件容易断的接线：① 默认模型下拉用的是 `getModels` 的 default、不是 config；
 * ② 保存时 `default_model` 取自下拉的当前值，其余七节原样透传；③ 读取失败与
 * 保存失败都有专属可见文案。
 *
 * 服务层 `importOriginal` 部分替身（`apiErrorDetail` 留真身）；清场口径承
 * DataManagement.upload.test.tsx（vitest 没开 globals，显式 cleanup + message.destroy）。
 *
 * L226 起本页多了「按数据推荐配置」卡（A279 第二批接线），对应下面 `Settings 按数据推荐配置`。
 * L233 起多了「写操作鉴权」卡（收 A302 / A312），对应下面 `Settings 写操作鉴权（L233）`。
 * L234 起多了「配置文件校验」卡（A279 第九批接线），对应下面 `Settings 配置文件校验（L234）`。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { message } from 'antd'

window.matchMedia =
  window.matchMedia ||
  ((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: () => {},
    removeListener: () => {},
    addEventListener: () => {},
    removeEventListener: () => {},
    dispatchEvent: () => false,
  }) as unknown as MediaQueryList)
class RO {
  observe() {}
  unobserve() {}
  disconnect() {}
}
window.ResizeObserver = window.ResizeObserver || (RO as unknown as typeof ResizeObserver)

const spies = vi.hoisted(() => ({
  getConfig: null as unknown as ReturnType<typeof vi.fn>,
  getModels: null as unknown as ReturnType<typeof vi.fn>,
  updateConfig: null as unknown as ReturnType<typeof vi.fn>,
  getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
  autoConfigRecommend: null as unknown as ReturnType<typeof vi.fn>,
  validateConfigFile: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.getConfig = vi.fn()
  spies.getModels = vi.fn()
  spies.updateConfig = vi.fn()
  // L226 起本页多了一张「按数据推荐配置」卡，挂载时会拉文件列表；不给替身的话
  // 每条老用例都会在 jsdom 里发一次真请求。
  spies.getDataFiles = vi.fn().mockResolvedValue({ files: [{ name: 'seed.json' }] })
  spies.autoConfigRecommend = vi.fn()
  spies.validateConfigFile = vi.fn()
  return {
    ...actual,
    getConfig: spies.getConfig,
    getModels: spies.getModels,
    updateConfig: spies.updateConfig,
    getDataFiles: spies.getDataFiles,
    autoConfigRecommend: spies.autoConfigRecommend,
    validateConfigFile: spies.validateConfigFile,
  }
})

import Settings from './Settings'
import { API_KEY_STORAGE_KEY } from '../services/api'
import type { AppConfig } from '../types/api'

/**
 * `AppConfig` 的完整实例：十节逐键都在，形状取自 `GET /api/config` 的真响应
 *
 * 标了 `: AppConfig` 而不是裸字面量，这样类型面一改（少一节、键改名）这里当场
 * typecheck 红——夹具跟着契约漂，而不是各写各的。
 */
const CONFIG: AppConfig = {
  default_model: 'ernie',
  augmentation: { variants_per_seed: 3, num_threads: 4 },
  quality: { enabled: true, threshold: 0.7 },
  dedup: { enabled: true, threshold: 0.85 },
  export: { default_format: 'jsonl', formats: ['jsonl'] },
  vector: {
    enabled: false,
    backend: 'chromadb',
    dimension: 384,
    storage_dir: 'data/vectors',
    collection: 'default',
  },
  rag: { enabled: false, default_format: 'jsonl', chunk_size: 512, chunk_overlap: 64 },
  multimodal: {
    enabled: false,
    image_extensions: ['.jpg', '.png'],
    audio_extensions: ['.wav', '.mp3'],
  },
  versioning: { enabled: true },
  visualization: { enabled: true },
}

afterEach(() => {
  cleanup()
  message.destroy()
  vi.clearAllMocks()
})

describe('Settings 读配置与渲染', () => {
  it('增强/质量/去重的数值控件渲染出配置里的真实值', async () => {
    spies.getConfig.mockResolvedValue(CONFIG)
    spies.getModels.mockResolvedValue({ models: ['ernie', 'openai'], default: 'ernie' })

    const { container } = render(<Settings />)

    await waitFor(() =>
      expect(container.querySelector('input[value="3"]')).toBeInTheDocument()
    )
    // 四个 InputNumber 的值分别是 variants=3 / threads=4 / quality.threshold=0.7 / dedup.threshold=0.85
    const values = Array.from(container.querySelectorAll('.ant-input-number-input')).map(
      (el) => (el as HTMLInputElement).value
    )
    expect(values).toEqual(['3', '4', '0.7', '0.85'])
  })
})

describe('Settings 保存时递给服务层的负载', () => {
  it('default_model 取自下拉当前值，其余七节原样透传', async () => {
    spies.getConfig.mockResolvedValue(CONFIG)
    spies.getModels.mockResolvedValue({ models: ['ernie', 'openai'], default: 'ernie' })
    spies.updateConfig.mockResolvedValue({ success: true, message: 'ok' })

    const { container } = render(<Settings />)

    // 等下拉真的把 default 显示出来，才证明 defaultModel 状态已就绪（否则保存拿到空串）
    await waitFor(() =>
      expect(container.querySelector('.ant-select-selection-item')?.textContent).toBe('ernie')
    )
    fireEvent.click(screen.getByRole('button', { name: /保存配置/ }))
    const payload = spies.updateConfig.mock.calls[0][0]
    expect(payload.default_model).toBe('ernie')
    expect(payload.quality).toEqual(CONFIG.quality)
    expect(payload.dedup).toEqual(CONFIG.dedup)
    expect(payload.export).toEqual(CONFIG.export)
    expect(payload.vector).toEqual(CONFIG.vector)
    expect(payload.rag).toEqual(CONFIG.rag)
    expect(payload.multimodal).toEqual(CONFIG.multimodal)
  })

  it('保存成功后弹出「已保存」提示', async () => {
    spies.getConfig.mockResolvedValue(CONFIG)
    spies.getModels.mockResolvedValue({ models: ['ernie'], default: 'ernie' })
    spies.updateConfig.mockResolvedValue({ success: true, message: 'ok' })

    render(<Settings />)

    await waitFor(() => expect(spies.getModels).toHaveBeenCalled())
    fireEvent.click(screen.getByRole('button', { name: /保存配置/ }))

    await waitFor(
      () => expect(document.body.textContent).toContain('配置已保存到 config.yaml'),
      { timeout: 4000 }
    )
  })
})

describe('Settings 失败路径的可见性', () => {
  it('加载配置失败时看得见「加载配置失败」', async () => {
    spies.getConfig.mockRejectedValue(new Error('boom'))
    spies.getModels.mockResolvedValue({ models: ['ernie'], default: 'ernie' })

    render(<Settings />)

    await waitFor(() => expect(document.body.textContent).toContain('加载配置失败'), {
      timeout: 4000,
    })
  })

  it('保存失败时看得见服务端判决', async () => {
    spies.getConfig.mockResolvedValue(CONFIG)
    spies.getModels.mockResolvedValue({ models: ['ernie'], default: 'ernie' })
    spies.updateConfig.mockRejectedValue({
      response: { data: { detail: '配置校验失败：quality.threshold 越界' } },
    })

    render(<Settings />)

    await waitFor(() => expect(spies.getModels).toHaveBeenCalled())
    fireEvent.click(screen.getByRole('button', { name: /保存配置/ }))

    await waitFor(
      () => expect(document.body.textContent).toContain('quality.threshold 越界'),
      { timeout: 4000 }
    )
  })
})

/**
 * 按数据推荐配置（L226，A279 第二批接线）
 *
 * 钉三件本页别处没有的事：① 推荐值递的是下拉里选中的那份文件；② `source_stats` 的
 * `null` 观测渲染成「无观测」而不是 0 或空白（实测真响应里 `quality_pass_rate` 就是 null）；
 * ③ 「填进表单」只改表单、不落盘——`updateConfig` 必须仍然零调用，保存这一步留给用户。
 */
describe('Settings 按数据推荐配置', () => {
  const RECOMMENDATION = {
    quality_threshold: 0.62,
    dedup_threshold: 0.91,
    recommended_sample_size: 250,
    reasoning: ['重复率 0.03 较低，去重阈值下调至 0.85', '长度分布偏长，建议抽样 250 条'],
    source_stats: { duplicate_rate: 0.03, quality_pass_rate: null, total_items: 1200 },
  }

  /** 选中数据集下拉（本页第二个 Select：0=默认模型，1=数据集） */
  async function pickRecommendFile(container: HTMLElement) {
    fireEvent.mouseDown(container.querySelectorAll('.ant-select-selector')[1])
    await waitFor(() =>
      expect(container.querySelectorAll('[role=combobox]')[1]).toHaveAttribute(
        'aria-expanded',
        'true'
      )
    )
    const dropdown = () => document.querySelectorAll('.ant-select-dropdown')
    await waitFor(() => expect(dropdown().length).toBeGreaterThanOrEqual(1))
    const option = Array.from(
      dropdown()[dropdown().length - 1].querySelectorAll('.ant-select-item-option')
    ).find(el => el.textContent === 'seed.json')
    if (!option) throw new Error('数据集下拉里没有 seed.json')
    fireEvent.click(option)
  }

  it('取推荐值：三个读数、null 观测与逐条理由都渲染；填进表单但不落盘', async () => {
    spies.getConfig.mockResolvedValue(CONFIG)
    spies.getModels.mockResolvedValue({ models: ['ernie'], default: 'ernie' })
    spies.autoConfigRecommend.mockResolvedValue(RECOMMENDATION)

    const { container } = render(<Settings />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())
    await pickRecommendFile(container)
    fireEvent.click(screen.getByRole('button', { name: /取推荐值/ }))

    await waitFor(() => expect(spies.autoConfigRecommend).toHaveBeenCalledWith('seed.json'))
    const stats = Array.from(container.querySelectorAll('.ant-statistic-content-value')).map(
      el => el.textContent
    )
    // 250 是整数、无千分位；0.62 / 0.91 原样显示，不四舍五入成 0.6 / 0.9
    expect(stats).toEqual(['0.62', '0.91', '250'])
    expect(screen.getByText('duplicate_rate: 0.03')).toBeInTheDocument()
    expect(screen.getByText('quality_pass_rate: 无观测')).toBeInTheDocument()
    expect(screen.getByText('total_items: 1200')).toBeInTheDocument()
    expect(screen.getByText('长度分布偏长，建议抽样 250 条')).toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: /填进上面的表单/ }))
    const values = Array.from(container.querySelectorAll('.ant-input-number-input')).map(
      el => (el as HTMLInputElement).value
    )
    // 前两个是增强配置（3 / 4），后两个被推荐值覆盖：0.7→0.62、0.85→0.91
    expect(values).toEqual(['3', '4', '0.62', '0.91'])
    expect(spies.updateConfig).not.toHaveBeenCalled()
  })

  it('没选数据集就点取推荐值：得到提示，不调服务层', async () => {
    spies.getConfig.mockResolvedValue(CONFIG)
    spies.getModels.mockResolvedValue({ models: ['ernie'], default: 'ernie' })

    render(<Settings />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())
    fireEvent.click(screen.getByRole('button', { name: /取推荐值/ }))

    await waitFor(() => expect(document.body.textContent).toContain('请先选择要观测的数据集'))
    expect(spies.autoConfigRecommend).not.toHaveBeenCalled()
  })

  it('推荐失败时专属文案出声，推荐面回落到「还没有推荐值」', async () => {
    spies.getConfig.mockResolvedValue(CONFIG)
    spies.getModels.mockResolvedValue({ models: ['ernie'], default: 'ernie' })
    spies.autoConfigRecommend.mockRejectedValue({
      response: { data: { detail: '文件不存在' } },
    })

    const { container } = render(<Settings />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())
    await pickRecommendFile(container)
    fireEvent.click(screen.getByRole('button', { name: /取推荐值/ }))

    await waitFor(() => expect(document.body.textContent).toContain('文件不存在'), {
      timeout: 4000,
    })
    expect(screen.getByText('还没有推荐值')).toBeInTheDocument()
    expect(container.querySelector('.ant-statistic')).toBeNull()
  })
})

/**
 * 写操作鉴权那张卡（L233，收 A302 / A312）
 *
 * 服务层那一半（拦截器挂不挂头、空串等于清除、401 那句指路）钉在 `api.test.ts`；
 * 这一份钉的是**渲染面上真的能填、填了真的落到 localStorage**。两边都不碰网络：
 * 「存 / 清」这两个按钮刻意不发请求——服务端只在写操作请求上校验，本层没有能力
 * 证明这份 key 是对的，所以页面上不许出现「校验通过」这类话（诚实口径那一条钉住）。
 */
describe('Settings 写操作鉴权（L233）', () => {
  const quietPage = () => {
    spies.getConfig.mockResolvedValue(CONFIG)
    spies.getModels.mockResolvedValue({ models: ['ernie'], default: 'ernie' })
  }

  const keyInput = () => screen.getByLabelText('写操作鉴权 API Key') as HTMLInputElement

  beforeEach(() => {
    localStorage.clear()
  })

  it('没存过 key 时如实说「没有存」，输入框是空的', async () => {
    quietPage()
    render(<Settings />)

    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())
    expect(screen.getByText('这台浏览器没有存 key')).toBeInTheDocument()
    expect(keyInput().value).toBe('')
    expect(localStorage.getItem(API_KEY_STORAGE_KEY)).toBeNull()
  })

  it('填一份存进去：落 localStorage、状态翻成「已存了一份」', async () => {
    quietPage()
    render(<Settings />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())

    fireEvent.change(keyInput(), { target: { value: '  s3cret  ' } })
    fireEvent.click(screen.getByRole('button', { name: /存进这台浏览器/ }))

    // trim 后才落盘：存进去的必须是能直接与环境变量比对的那一份
    expect(localStorage.getItem(API_KEY_STORAGE_KEY)).toBe('s3cret')
    expect(screen.getByText('这台浏览器已存了一份')).toBeInTheDocument()
    await waitFor(
      () => expect(document.body.textContent).toContain('已存进这台浏览器'),
      { timeout: 4000 }
    )
  })

  it('重开页面时输入框反映已经存下的那份（不是空串）', async () => {
    quietPage()
    localStorage.setItem(API_KEY_STORAGE_KEY, 'already-here')

    render(<Settings />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())

    expect(keyInput().value).toBe('already-here')
    expect(screen.getByText('这台浏览器已存了一份')).toBeInTheDocument()
  })

  it('清除：localStorage 那一格被摘掉，状态翻回「没有存」', async () => {
    quietPage()
    localStorage.setItem(API_KEY_STORAGE_KEY, 'already-here')

    render(<Settings />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())
    // antd 的 Button 会给「恰好两个汉字」的文案中间插一个空格（accessible name 成
    // 了 `清 除`），实测第一次跑就是这么红的 —— 所以这里用 `\s*` 而不是逐字匹配。
    fireEvent.click(screen.getByRole('button', { name: /清\s*除/ }))

    expect(localStorage.getItem(API_KEY_STORAGE_KEY)).toBeNull()
    expect(keyInput().value).toBe('')
    expect(screen.getByText('这台浏览器没有存 key')).toBeInTheDocument()
  })

  it('空输入点「存」不报假话：说的是「等于清除」，不是「已存进」', async () => {
    quietPage()
    render(<Settings />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())

    fireEvent.click(screen.getByRole('button', { name: /存进这台浏览器/ }))

    await waitFor(
      () => expect(document.body.textContent).toContain('等于清除'),
      { timeout: 4000 }
    )
    expect(localStorage.getItem(API_KEY_STORAGE_KEY)).toBeNull()
  })

  it('诚实口径留在卡上：明文存放、影响面 25/25、不是登录、不许说「校验通过」', async () => {
    quietPage()
    render(<Settings />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())

    const text = document.body.textContent || ''
    expect(text).toContain('AUGMENTOR_API_KEY')
    expect(text).toContain('明文')
    // 影响面是 L233 探针在真契约面上量到的两个数，不是估的（L234：21 → 23；L236 接走 translate ⇒ 24，尾巴清空；
    // L243 给清理旧备份加了一只挂门禁的端点 ⇒ 25 / 25，未接线那一档仍为空）
    expect(text).toContain('77 条操作里有 25 条挂着这道门禁')
    expect(text).toContain('25 条是页面上已')
    expect(text).toContain('不是登录')
    expect(text).toContain('不替代 HTTPS')
    expect(text).not.toContain('校验通过')
  })
})

/**
 * 配置文件校验那张卡（L234，A279 第九批接线）
 *
 * 这一支与本页其余动作有一处根本不同：**校验不通过也回 HTTP 200**，判决只在 body 的
 * `is_valid` 上 ⇒ 页面不能沿用「没抛错就是成了」，否则一份坏配置会被渲成绿色。四件事
 * 逐个钉：① 不合法那一档渲成红 Alert + 三个计数 + 逐条 issue；② 合法那一档**不是**
 * 「没有问题清单」（最小合法配置实测 is_valid 为 true 却带 1 条警告、出厂那份带 5 条）；
 * ③ 路径留空是一个合法档位，照空串递给服务层、由服务层丢成整键不送（那一半钉在
 * `api.test.ts`）；④ 403/404 走 catch，页面显示服务端那句判决、结果面回落成「还没有」。
 * 夹具逐字来自仓外探针（`l234_fixtures.py` / `l234_minvalid.py`，L234 跑完即删）。
 */
describe('Settings 配置文件校验（L234）', () => {
  const quietPage = () => {
    spies.getConfig.mockResolvedValue(CONFIG)
    spies.getModels.mockResolvedValue({ models: ['ernie'], default: 'ernie' })
  }

  /** 一份 YAML 都解析不了的配置：路径那一格是 `yaml`，行号恒为 null（探针逐字） */
  const BROKEN = {
    is_valid: false,
    summary: { errors: 1, warnings: 0, info: 0 },
    errors: [
      {
        path: 'yaml',
        message:
          'YAML解析错误: while parsing a flow sequence\n  in "D:\\tmp\\broken.yaml", line 2, column 14',
        severity: 'error',
        line: null,
      },
    ],
    warnings: [],
    info: [],
  }

  /** 最小合法配置 `models:\n  default: ernie`：is_valid 为 true，却仍带一条警告（探针逐字） */
  const VALID_WITH_WARNING = {
    is_valid: true,
    summary: { errors: 0, warnings: 1, info: 0 },
    errors: [],
    warnings: [
      {
        path: 'models.default',
        message: "模型 'ernie' 未配置。可用模型: []",
        severity: 'warning',
        line: null,
      },
    ],
    info: [],
  }

  const pathInput = () => screen.getByLabelText('配置文件路径') as HTMLInputElement

  /** 按标题取统计格：页面上裸数字太多，逐文本找会撞多重匹配 */
  const stats = (container: HTMLElement) =>
    Array.from(container.querySelectorAll('.ant-statistic')).map(
      (box) =>
        `${box.querySelector('.ant-statistic-title')?.textContent}=` +
        `${box.querySelector('.ant-statistic-content-value')?.textContent}`
    )

  beforeEach(() => {
    spies.validateConfigFile.mockReset()
  })

  it('不合法那一档：红 Alert、三个计数、逐条 issue，都是服务端那份原文', async () => {
    quietPage()
    spies.validateConfigFile.mockResolvedValue(BROKEN)
    const { container } = render(<Settings />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())

    fireEvent.change(pathInput(), { target: { value: 'broken.yaml' } })
    fireEvent.click(screen.getByRole('button', { name: /校验配置/ }))

    await screen.findByText('这份配置不合法——下面逐条列出')
    expect(spies.validateConfigFile).toHaveBeenCalledWith({ path: 'broken.yaml' })
    expect(stats(container)).toEqual(['错误=1', '警告=0', '提示=0'])
    expect(screen.getByText('yaml')).toBeInTheDocument()
    expect(screen.getByText(/YAML解析错误: while parsing a flow sequence/)).toBeInTheDocument()
    expect(screen.getByText('error')).toBeInTheDocument()
    await waitFor(
      () => expect(document.body.textContent).toContain('这份配置不合法：1 个错误、0 个警告'),
      { timeout: 4000 }
    )
  })

  it('合法那一档：判决照 is_valid 渲成绿的，但警告清单照样列出，且不许说「校验通过」', async () => {
    quietPage()
    spies.validateConfigFile.mockResolvedValue(VALID_WITH_WARNING)
    const { container } = render(<Settings />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())

    // 路径留空是一个合法档位：照空串递出去，由服务层丢成「整键不送」
    fireEvent.click(screen.getByRole('button', { name: /校验配置/ }))

    await screen.findByText('这份配置合法（判决是 body 里的 is_valid）')
    expect(spies.validateConfigFile).toHaveBeenCalledWith({ path: '' })
    expect(screen.getByText(/不等于没有问题清单/)).toBeInTheDocument()
    expect(stats(container)).toEqual(['错误=0', '警告=1', '提示=0'])
    expect(screen.getByText('models.default')).toBeInTheDocument()
    expect(screen.getByText(/模型 'ernie' 未配置/)).toBeInTheDocument()
    expect(screen.getByText('warning')).toBeInTheDocument()
    // L233 那支禁令在这一档同样成立：本层没有能力证明这份 key/配置「对」，只转述服务端判决
    expect(document.body.textContent).not.toContain('校验通过')
  })

  it('白名单外那一档（403）：走 catch，页面是服务端那句判决，结果面回落成「还没有」', async () => {
    quietPage()
    spies.validateConfigFile.mockRejectedValue({
      response: { status: 403, data: { detail: '路径超出允许的数据目录范围' } },
    })
    const { container } = render(<Settings />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())

    fireEvent.change(pathInput(), { target: { value: '../../etc/passwd' } })
    fireEvent.click(screen.getByRole('button', { name: /校验配置/ }))

    await waitFor(
      () => expect(document.body.textContent).toContain('路径超出允许的数据目录范围'),
      { timeout: 4000 }
    )
    expect(screen.getByText('还没有校验结果')).toBeInTheDocument()
    expect(container.querySelector('.ant-statistic')).toBeNull()
  })

  it('诚实口径留在卡上：留空校验谁、三档实测判决、行号恒为空、只读不写', async () => {
    quietPage()
    render(<Settings />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())

    // placeholder 不在 textContent 里，得单独按属性取
    expect(pathInput().placeholder).toContain('留空 = 校验服务自己正在用的那份')
    const text = document.body.textContent || ''
    expect(text).toContain('AUGMENTOR_CONFIG_PATH')
    expect(text).toContain('不是 404')
    expect(text).toContain('文件路径 不能为空')
    expect(text).toContain('恒为空')
    expect(text).toContain('跳到出错行')
    expect(text).toContain('只读配置、不写配置')
    // 这一支的判决通道与本页其余动作不同，卡上必须写着，否则用户会把 200 当成「配置没问题」
    expect(text).toContain('校验不通过也回 200')
  })
})
