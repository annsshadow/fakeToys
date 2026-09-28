/**
 * Dashboard 页的渲染面（L104，A206）
 *
 * 11 个页面里 Dashboard 是入口页，此前零组件测试。本用例把四张统计卡、
 * 阈值行、模型标签、以及「把什么 props 递给了时间线子组件」从「源码里写了」
 * 升级成「真的渲染出来了」。
 *
 * 服务层用 `importOriginal` 做**部分替身**（`apiErrorDetail` 留真身，
 * 否则错误路径断言的是替身的行为而不是页面的行为）；四个读取函数在
 * 每条用例里分别 resolve / reject。子组件（AugmentForm / VersionTimeline）
 * 用替身组件代替——它们自己的行为由各自用例管，这里只关心接线。
 *
 * 清场口径承 DataManagement.upload.test.tsx：vitest 没开 globals，RTL 的
 * 自动 cleanup 不生效，必须显式 `cleanup()`；antd 的 message 容器挂在
 * RTL 容器之外，必须显式 `destroy()`。
 */
import { afterEach, describe, expect, it, vi } from 'vitest'
import { cleanup, render, screen, waitFor } from '@testing-library/react'
import { message } from 'antd'

// antd v5 的主题与响应式 hook 依赖这两个浏览器 API，jsdom 不提供。
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
  getVersions: null as unknown as ReturnType<typeof vi.fn>,
  healthCheck: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.getConfig = vi.fn()
  spies.getModels = vi.fn()
  spies.getVersions = vi.fn()
  spies.healthCheck = vi.fn()
  return {
    ...actual,
    getConfig: spies.getConfig,
    getModels: spies.getModels,
    getVersions: spies.getVersions,
    healthCheck: spies.healthCheck,
  }
})

vi.mock('../components/AugmentForm', () => ({
  default: () => <div data-testid="augment-form-stub" />,
}))

vi.mock('../components/VersionTimeline', () => ({
  default: (props: { versions: unknown[]; currentVersion: string | null }) => (
    <div
      data-testid="timeline-stub"
      data-current={props.currentVersion ?? ''}
      data-count={String(props.versions.length)}
    />
  ),
}))

import Dashboard from './Dashboard'

/** AppConfig 的合法最小实例：页面只读 default_model / quality / dedup / augmentation */
const CONFIG = {
  default_model: 'ernie',
  augmentation: { variants_per_seed: 3, num_threads: 4 },
  quality: { enabled: true, threshold: 0.7 },
  dedup: { enabled: true, threshold: 0.85 },
  export: { default_format: 'jsonl', formats: ['jsonl'] },
  vector: { enabled: false, backend: 'chromadb' },
  rag: { enabled: false, default_format: 'jsonl' },
  multimodal: { enabled: false },
}

afterEach(() => {
  cleanup()
  message.destroy()
  vi.clearAllMocks()
})

describe('Dashboard 统计卡的渲染面', () => {
  it('健康正常时四张卡渲染真实读数与三个阈值', async () => {
    spies.healthCheck.mockResolvedValue({ status: 'ok', version: '3.0.0' })
    spies.getConfig.mockResolvedValue(CONFIG)
    spies.getModels.mockResolvedValue({ models: ['ernie', 'openai'], default: 'ernie' })
    spies.getVersions.mockResolvedValue({
      versions: [{ version_id: 'v-1', label: 'l', description: 'd', created_at: 't', item_count: 2 }],
    })

    render(<Dashboard />)

    await waitFor(() => expect(screen.getByText('正常')).toBeInTheDocument())
    expect(screen.getByText('版本 3.0.0')).toBeInTheDocument()
    expect(screen.getByText('默认模型')).toBeInTheDocument()
    expect(screen.getByText('质量阈值 0.7')).toBeInTheDocument()
    expect(screen.getByText('去重阈值 0.85')).toBeInTheDocument()
    expect(screen.getByText('并发线程 4')).toBeInTheDocument()
    // 时间线子组件收到版本列表，且首个版本是当前版本
    const timeline = screen.getByTestId('timeline-stub')
    expect(timeline).toHaveAttribute('data-count', '1')
    expect(timeline).toHaveAttribute('data-current', 'v-1')
    expect(screen.getByTestId('augment-form-stub')).toBeInTheDocument()
  })

  it('默认模型标签是绿色，其余模型是蓝色', async () => {
    spies.healthCheck.mockResolvedValue({ status: 'ok', version: '3.0.0' })
    spies.getConfig.mockResolvedValue(CONFIG)
    spies.getModels.mockResolvedValue({ models: ['ernie', 'openai'], default: 'ernie' })
    spies.getVersions.mockResolvedValue({ versions: [] })

    const { container } = render(<Dashboard />)

    await waitFor(() =>
      expect(container.querySelectorAll('.ant-tag')).toHaveLength(2)
    )
    // 「默认模型」统计卡的值也是 ernie，所以按标签元素查而不是按文本查
    const tags = Array.from(container.querySelectorAll('.ant-tag'))
    expect(tags.map((t) => t.textContent)).toEqual(['ernie', 'openai'])
    expect(tags[0].className).toContain('ant-tag-green')
    expect(tags[1].className).toContain('ant-tag-blue')
  })

  it('健康异常时状态卡显示「未知」而不是「正常」', async () => {
    spies.healthCheck.mockResolvedValue({ status: 'degraded', version: '3.0.0' })
    spies.getConfig.mockResolvedValue(CONFIG)
    spies.getModels.mockResolvedValue({ models: [], default: 'ernie' })
    spies.getVersions.mockResolvedValue({ versions: [] })

    render(<Dashboard />)

    await waitFor(() => expect(screen.getByText('未知')).toBeInTheDocument())
  })
})

describe('Dashboard 四条读取失败的可见性', () => {
  const CASES = [
    { fn: 'healthCheck' as const, text: '服务健康检查失败' },
    { fn: 'getConfig' as const, text: '加载配置失败' },
    { fn: 'getModels' as const, text: '加载模型列表失败' },
    { fn: 'getVersions' as const, text: '加载版本列表失败' },
  ]

  for (const { fn, text } of CASES) {
    it(`${fn} 失败时用户看得见专属文案，而不是静默空白`, async () => {
      spies.getConfig.mockResolvedValue(CONFIG)
      spies.getModels.mockResolvedValue({ models: [], default: 'ernie' })
      spies.getVersions.mockResolvedValue({ versions: [] })
      spies.healthCheck.mockResolvedValue({ status: 'ok', version: '3.0.0' })
      spies[fn].mockRejectedValue(new Error('boom'))

      render(<Dashboard />)

      // antd 的 message 渲进 body 而不是组件容器
      await waitFor(() => expect(document.body.textContent).toContain(text), { timeout: 4000 })
    })
  }
})
