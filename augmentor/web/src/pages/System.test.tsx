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
 * L236 起依赖诊断改读 `GET /api/system/dependencies`（不碰管道那条），因此「status 拉不动
 * ⇒ 明细整块不渲染」不再是通则：只有两面都空才不渲染。替身里 `systemDependencies` 默认
 * **不给回执**，让既有三组用例继续走综合状态里那一份（两面数据逐字节相同，L236 实测）。
 *
 * L225 起本页还会取向量库状态。`spies.getVectorStatus` 在替身工厂里给了默认回执：
 * 向量面与本页原有三条用例无关，让它们不必逐个显式安排；`clearAllMocks` 不清实现，
 * 默认值在用例之间留得住。向量面自己的两条用例覆盖「渲染生效值」与「失败时降级成
 * 警示、且不把整页服务状态一起拖掉」。
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
  getServiceStatus: null as unknown as ReturnType<typeof vi.fn>,
  getVectorStatus: null as unknown as ReturnType<typeof vi.fn>,
  listBackups: null as unknown as ReturnType<typeof vi.fn>,
  getBackupSourceCandidates: null as unknown as ReturnType<typeof vi.fn>,
  listDependencyDatasets: null as unknown as ReturnType<typeof vi.fn>,
  getDependencyGraph: null as unknown as ReturnType<typeof vi.fn>,
  systemDependencies: null as unknown as ReturnType<typeof vi.fn>,
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
  spies.getBackupSourceCandidates = vi.fn().mockResolvedValue({ files: [] })
  // L232 起本页还会取数据集血缘那一面（列表 + 图）。与本份用例无关，给空回执即可。
  spies.listDependencyDatasets = vi.fn().mockResolvedValue({ datasets: [] })
  spies.getDependencyGraph = vi.fn().mockResolvedValue({ nodes: [], edges: [], issues: [] })
  // L236 起依赖诊断自取一面。**默认不给回执**（resolved undefined）⇒ 页面 `deps` 落空 ⇒
  // 渲染面退回 `status.dependencies` 那一份，上面三组既有用例的语义一字未变；
  // 独立取数那几条用例自己显式安排回执。
  spies.systemDependencies = vi.fn()
  return {
    ...actual,
    getServiceStatus: spies.getServiceStatus,
    getVectorStatus: spies.getVectorStatus,
    listBackups: spies.listBackups,
    getBackupSourceCandidates: spies.getBackupSourceCandidates,
    listDependencyDatasets: spies.listDependencyDatasets,
    getDependencyGraph: spies.getDependencyGraph,
    systemDependencies: spies.systemDependencies,
  }
})

import System from './System'
import type { StatusResponse } from '../types/api'

/**
 * 两只夹具都标了 `: StatusResponse`
 *
 * 契约里 `model_retry_count` / `model_retry_wait_seconds` / `model_generate_seconds`
 * 都是**必填**（管道缺失时回 `null`，不是省掉这一键），所以夹具必须带上；
 * 标了类型之后少一键当场 typecheck 红，不会等到渲染面上出现一只空格子才发现。
 *
 * ⚠️ 2026-10-25：`model_generate_seconds`（累计生成耗时）加进契约时只改了类型与渲染，
 * 忘了这两个夹具 —— `tsc --noEmit` 于是在 Docker 镜像构建里红了三天
 * （ai-platform-ci 的 Docker image smoke test，从 10-24 起），并卡住 Dependabot PR。
 * 教训：**加必填字段时，带类型标注的夹具就是编译器替你守的那道门，别绕过它**。
 */
const HEALTHY: StatusResponse = {
  status: 'ok',
  version: '3.0.0',
  model_default: 'ernie',
  model_available: true,
  model_retry_count: 0,
  model_retry_wait_seconds: 0,
  // 12.5 而不是整数：渲染面做 toFixed(1)，用带小数的值才能钉住「没被截成 12」
  model_generate_seconds: 12.5,
  dependencies: {
    installed: { pandas: true, chromadb: true },
    missing: [],
    degraded_features: [],
    all_required_present: true,
    available_count: 2,
  },
}

const DEGRADED: StatusResponse = {
  status: 'ok',
  version: '3.0.0',
  model_default: 'ernie',
  model_available: false,
  model_retry_count: null,
  model_retry_wait_seconds: null,
  model_generate_seconds: null,
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

/**
 * 两只重试读数（L237 / A281）
 *
 * 这两格唯一容易做错的地方是 `null` 与 `0` 折叠成一档：后端在管道缺失时回 `null`，
 * 在「后端在、没重试过」时回 `0`，两句话在页面上必须长得不一样。所以两档各测一次，
 * 而不是只测有值那一档。
 */
describe('System 模型重试读数（L237 补渲染）', () => {
  const statistic = (container: HTMLElement, title: string) =>
    Array.from(container.querySelectorAll('.ant-statistic')).find(
      (s) => s.querySelector('.ant-statistic-title')?.textContent === title
    )?.querySelector('.ant-statistic-content-value')?.textContent

  it('后端在、重试过 0 次时两格显示 0（不是「无后端」）', async () => {
    spies.getServiceStatus.mockResolvedValue(HEALTHY)

    const { container } = render(<System />)

    await waitFor(() => expect(statistic(container, '模型重试次数')).toBe('0'))
    expect(statistic(container, '重试等待（秒）')).toBe('0')
    // 累计生成耗时同一套 null 语义：有后端就该是数字，且保留一位小数
    expect(statistic(container, '累计生成耗时（秒）')).toBe('12.5')
    expect(screen.queryAllByText('无后端')).toHaveLength(0)
  })

  it('管道缺失（两键为 null）时两格显示「无后端」，不折成 0', async () => {
    spies.getServiceStatus.mockResolvedValue(DEGRADED)

    const { container } = render(<System />)

    await waitFor(() => expect(statistic(container, '模型重试次数')).toBe('无后端'))
    expect(statistic(container, '重试等待（秒）')).toBe('无后端')
    expect(statistic(container, '累计生成耗时（秒）')).toBe('无后端')
    expect(statistic(container, '模型重试次数')).not.toBe('0')
  })

  it('卡上写明这两键只在 status 上、health 刻意不含', async () => {
    spies.getServiceStatus.mockResolvedValue(HEALTHY)

    render(<System />)

    await waitFor(() =>
      expect(document.body.textContent).toContain('「无后端」是 null，不是 0')
    )
    expect(document.body.textContent).toContain('/api/health')
  })
})

describe('System 失败路径', () => {
  it('综合状态与依赖诊断都拉不动时才不渲染明细区', async () => {
    spies.getServiceStatus.mockRejectedValue(new Error('boom'))

    render(<System />)

    await waitFor(
      () => expect(document.body.textContent).toContain('获取服务状态失败'),
      { timeout: 10000 }
    )
    // L236 起门是 diag（自取一面 ?? 综合状态里那份）：这一格两面都空 ⇒ 整块不渲染
    expect(screen.queryByText('依赖明细')).not.toBeInTheDocument()
  })
})

describe('System 依赖诊断独立取数（L236 接线）', () => {
  const DIAG = {
    installed: { pandas: true, faiss: false },
    missing: ['faiss'],
    degraded_features: ['向量检索'],
    all_required_present: false,
    available_count: 1,
  }

  // `clearAllMocks` 不清实现：本组安排的 DIAG 回执会漏进后面的用例（实测把向量面那条
  // 「必需依赖齐备」判成缺失态 ⇒ 1 支红）。这一面在替身工厂里的默认就是「不给回执」，
  // 所以 mockReset 恰好还原成默认，不会顺手清掉别人的默认值
  afterEach(() => {
    spies.systemDependencies.mockReset()
  })

  it('综合状态失败时明细照旧渲染，并说明这份读数不来自它', async () => {
    spies.getServiceStatus.mockRejectedValue(new Error('pipeline down'))
    spies.systemDependencies.mockResolvedValue(DIAG)

    render(<System />)

    // L236 之前这一格是「status 为 null ⇒ 依赖明细整块不渲染」；现在依赖诊断走的是
    // 不碰管道的那条端点，所以它必须还在，而且要说清自己不是从综合状态里读的
    expect(await screen.findByText('依赖明细')).toBeInTheDocument()
    expect(screen.getByText(/这份依赖诊断来自不碰管道的那条端点/)).toBeInTheDocument()
    expect(screen.getByText(/缺失：faiss/)).toBeInTheDocument()
  })

  it('综合状态正常时不重复那句话（两面数据相同，只有取法不同）', async () => {
    spies.getServiceStatus.mockResolvedValue(HEALTHY)
    spies.systemDependencies.mockResolvedValue({
      ...HEALTHY.dependencies,
    })

    render(<System />)

    expect(await screen.findByText(/必需依赖齐备/)).toBeInTheDocument()
    expect(
      screen.queryByText(/这份依赖诊断来自不碰管道的那条端点/)
    ).not.toBeInTheDocument()
  })

  it('卡上那颗刷新只打依赖诊断那条端点，不再重取综合状态', async () => {
    spies.getServiceStatus.mockResolvedValue(HEALTHY)
    spies.systemDependencies.mockResolvedValue(DIAG)

    render(<System />)
    await screen.findByText('依赖明细')
    spies.getServiceStatus.mockClear()
    spies.systemDependencies.mockClear()

    // 按钮跑过一轮之后会残留 loading 图标，可及名变成「loading刷新依赖诊断」⇒ 只能按正则取
    fireEvent.click(screen.getByRole('button', { name: /刷新依赖诊断/ }))

    await waitFor(() => expect(spies.systemDependencies).toHaveBeenCalledTimes(1))
    expect(spies.getServiceStatus).not.toHaveBeenCalled()
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
      { timeout: 10000 }
    )
    expect(screen.getByText('向量库状态未取到')).toBeInTheDocument()
    expect(screen.queryByText('自动建库')).not.toBeInTheDocument()
  })
})
