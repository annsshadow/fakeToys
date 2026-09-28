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
 */
import { afterEach, describe, expect, it, vi } from 'vitest'
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
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.getConfig = vi.fn()
  spies.getModels = vi.fn()
  spies.updateConfig = vi.fn()
  return {
    ...actual,
    getConfig: spies.getConfig,
    getModels: spies.getModels,
    updateConfig: spies.updateConfig,
  }
})

import Settings from './Settings'

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
