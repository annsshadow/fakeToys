/**
 * System 页的渲染面（L106，A206）
 *
 * 系统状态页此前零组件测试。它有三条分支需要真渲染才能证明：① 依赖齐备时
 * 顶部 Alert 是 success，缺依赖时是 warning 且列出缺失项；② 依赖明细表按
 * installed 映射渲染每行的「已安装 / 未安装」标签；③ 降级功能有则列 orange 标签、
 * 无则显示「没有功能因依赖缺失而降级」。全空态（尚未加载）四张卡显示占位符。
 *
 * 服务层 `importOriginal` 部分替身（`apiErrorDetail` 留真身）；清场口径承前。
 */
import { afterEach, describe, expect, it, vi } from 'vitest'
import { cleanup, render, screen, waitFor } from '@testing-library/react'
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
  getServiceStatus: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.getServiceStatus = vi.fn()
  return { ...actual, getServiceStatus: spies.getServiceStatus }
})

import System from './System'

const HEALTHY = {
  status: 'ok',
  version: '3.0.0',
  model_default: 'ernie',
  model_available: true,
  dependencies: {
    installed: { pandas: true, chromadb: true },
    missing: [],
    degraded_features: [],
    all_required_present: true,
    available_count: 2,
  },
}

const DEGRADED = {
  status: 'ok',
  version: '3.0.0',
  model_default: 'ernie',
  model_available: false,
  dependencies: {
    installed: { pandas: true, faiss: false },
    missing: ['faiss'],
    degraded_features: ['向量检索'],
    all_required_present: false,
    available_count: 1,
  },
}

afterEach(() => {
  cleanup()
  message.destroy()
  vi.clearAllMocks()
})

describe('System 依赖齐备态', () => {
  it('Alert 为 success 并报可用项数，明细表把已安装依赖标绿', async () => {
    spies.getServiceStatus.mockResolvedValue(HEALTHY)

    const { container } = render(<System />)

    await waitFor(() => expect(screen.getByText(/必需依赖齐备/)).toBeInTheDocument())
    expect(screen.getByText(/共 2 项可用/)).toBeInTheDocument()
    expect(screen.getByText('没有功能因依赖缺失而降级')).toBeInTheDocument()
    // 明细表两行都是「已安装」
    const installedTags = Array.from(container.querySelectorAll('.ant-tag-green'))
    expect(installedTags.map((t) => t.textContent)).toEqual(['已安装', '已安装'])
  })
})

describe('System 依赖缺失态', () => {
  it('Alert 为 warning 并列出缺失项，降级功能以 orange 标签呈现', async () => {
    spies.getServiceStatus.mockResolvedValue(DEGRADED)

    const { container } = render(<System />)

    await waitFor(() =>
      expect(screen.getByText('缺少必需依赖，部分功能将不可用')).toBeInTheDocument()
    )
    expect(screen.getByText(/缺失：faiss/)).toBeInTheDocument()
    // 明细里 faiss 一行标红「未安装」
    expect(
      Array.from(container.querySelectorAll('.ant-tag-red')).map((t) => t.textContent)
    ).toContain('未安装')
    // 降级功能标签
    const orange = container.querySelector('.ant-tag-orange')
    expect(orange?.textContent).toBe('向量检索')
  })

  it('模型不可用时「模型可用」卡显示「否」', async () => {
    spies.getServiceStatus.mockResolvedValue(DEGRADED)

    render(<System />)

    await waitFor(() => expect(screen.getByText('否')).toBeInTheDocument())
  })
})

describe('System 失败路径', () => {
  it('获取状态失败时看得见专属文案且不渲染明细区', async () => {
    spies.getServiceStatus.mockRejectedValue(new Error('boom'))

    render(<System />)

    await waitFor(
      () => expect(document.body.textContent).toContain('获取服务状态失败'),
      { timeout: 4000 }
    )
    // status 仍为 null ⇒ 依赖明细整块不渲染
    expect(screen.queryByText('依赖明细')).not.toBeInTheDocument()
  })
})
