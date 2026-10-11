/**
 * System 页的渲染面（L106，A206）
 *
 * 系统状态页此前零组件测试。它有三条分支需要真渲染才能证明：① 依赖齐备时
 * 顶部 Alert 是 success，缺依赖时是 warning 且列出缺失项；② 依赖明细表按
 * installed 映射渲染每行的「已安装 / 未安装」标签；③ 降级功能有则列 orange 标签、
 * 无则显示「没有功能因依赖缺失而降级」。全空态（尚未加载）四张卡显示占位符。
 *
 * 服务层 `importOriginal` 部分替身（`apiErrorDetail` 留真身）；清场口径承前。
 *
 * L225 起本页还会取向量库状态。`spies.getVectorStatus` 在替身工厂里给了默认回执：
 * 向量面与本页原有三条用例无关，让它们不必逐个显式安排；`clearAllMocks` 不清实现，
 * 默认值在用例之间留得住。向量面自己的两条用例覆盖「渲染生效值」与「失败时降级成
 * 警示、且不把整页服务状态一起拖掉」。
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
  getVectorStatus: null as unknown as ReturnType<typeof vi.fn>,
  listBackups: null as unknown as ReturnType<typeof vi.fn>,
  getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
  listDependencyDatasets: null as unknown as ReturnType<typeof vi.fn>,
  getDependencyGraph: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.getServiceStatus = vi.fn()
  spies.getVectorStatus = vi.fn().mockResolvedValue({
    enabled: false,
    exists: false,
    count: 0,
    backend: 'faiss',
    dimension: 768,
    collection: 'augmentor',
    storage_dir: '',
  })
  // L231 起本页还会取备份列表与数据文件候选。两面都给了默认回执：备份面与本页原有
  // 用例无关，让它们不必逐个显式安排（`clearAllMocks` 不清实现，默认值留得住）。
  spies.listBackups = vi.fn().mockResolvedValue({ backups: [] })
  spies.getDataFiles = vi.fn().mockResolvedValue({ files: [] })
  // L232 起本页还会取数据集血缘那一面（列表 + 图）。与本份用例无关，给空回执即可。
  spies.listDependencyDatasets = vi.fn().mockResolvedValue({ datasets: [] })
  spies.getDependencyGraph = vi.fn().mockResolvedValue({ nodes: [], edges: [], issues: [] })
  return {
    ...actual,
    getServiceStatus: spies.getServiceStatus,
    getVectorStatus: spies.getVectorStatus,
    listBackups: spies.listBackups,
    getDataFiles: spies.getDataFiles,
    listDependencyDatasets: spies.listDependencyDatasets,
    getDependencyGraph: spies.getDependencyGraph,
  }
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

describe('System 向量库状态（L225 接线）', () => {
  it('闸开、索引存在时四张统计卡与三个标签渲染生效值', async () => {
    spies.getVectorStatus.mockResolvedValue({
      enabled: true,
      exists: true,
      count: 1200,
      backend: 'faiss',
      dimension: 512,
      collection: 'augmentor_v1',
      storage_dir: 'data/vector',
    })

    const { container } = render(<System />)

    await waitFor(() => expect(spies.getVectorStatus).toHaveBeenCalled())
    expect(await screen.findByText('开')).toBeInTheDocument()
    expect(screen.getByText('已存在')).toBeInTheDocument()
    // antd Statistic 给整数加千分位分隔符：渲染的是 1,200 而不是 1200（实测）
    expect(
      Array.from(container.querySelectorAll('.ant-statistic-content-value')).map(
        (el) => el.textContent
      )
    ).toEqual(expect.arrayContaining(['1,200', '512']))
    expect(screen.getByText('backend: faiss')).toBeInTheDocument()
    expect(screen.getByText('collection: augmentor_v1')).toBeInTheDocument()
    expect(screen.getByText('storage_dir: data/vector')).toBeInTheDocument()
  })

  it('未配 storage_dir 时标签回显「未配置」，而不是空串', async () => {
    spies.getVectorStatus.mockResolvedValue({
      enabled: false,
      exists: false,
      count: 0,
      backend: 'faiss',
      dimension: 768,
      collection: 'augmentor',
      storage_dir: '',
    })

    render(<System />)

    expect(await screen.findByText('storage_dir: 未配置')).toBeInTheDocument()
  })

  it('向量面失败只降级它自己：服务状态照旧渲染，卡内换成警示', async () => {
    spies.getServiceStatus.mockResolvedValue(HEALTHY)
    spies.getVectorStatus.mockRejectedValue(new Error('vector down'))

    render(<System />)

    // 两面的取数各自独立发起，向量面失败不该把服务状态一起拖掉
    expect(await screen.findByText(/必需依赖齐备/)).toBeInTheDocument()
    await waitFor(
      () => expect(document.body.textContent).toContain('获取向量库状态失败'),
      { timeout: 4000 }
    )
    expect(screen.getByText('向量库状态未取到')).toBeInTheDocument()
    expect(screen.queryByText('自动建库')).not.toBeInTheDocument()
  })
})
