/**
 * `System` 的数据集血缘面（L232：A279 第八批接线）
 *
 * 存在理由与 L229 / L230 / L231 那三份同构：Python 侧的 `test_lineage_wiring_l232.py` 只能
 * 证明**源码写了**那三个调用形状与那几句口径，证不出渲染面上真的看得见。这一份钉七件事：
 * ① 已登记数据集按服务端回的**原样**渲染，空描述与空标签各有可见占位（不是空白格）；
 * ② 图的三格计数按回显渲染，`issues` 非空时逐条列出 `type` 与 `message`；
 * ③ 登记的必填闸两路（无名称 / 无源文件）只给一句 warning、**不发请求**；
 * ④ 登记成功按 `(名称, 文件, { description, tags })` 发，标签那一格的中英文逗号都切成数组、
 *    空段被丢掉，回执里的 id 与条数出现在提示上，列表与图**都重取**；
 * ⑤ 白名单外被判 403 时，页面上是服务端那句判决原话，不是页面自己编的「失败了」；
 * ⑥ 血缘那一面取不到只降级自己（出现「已登记数据集未取到」），服务状态与备份面照常渲染；
 * ⑦ 六条实测过的**诚实口径**留在卡上（A306 覆盖并刷新 created_at、A307 只读端点建目录、
 *    A308 三份默认注册表不是同一份、A309 边建不出来且登记删不掉、A310 issues 不查环、
 *    A312 写那一支挂 X-API-Key 而前端没有送头的通道）。
 *
 * 夹具里的 dataset_id、item_count、路径与时间戳逐字来自探针在真实端点上量到的回执
 * （`dsA` → `8c7f0201`、3 条；`dsB` → `0d2cace8`、1 条），不是编的。服务层用
 * `importOriginal` 做部分替身，`apiErrorDetail` 留真身，理由同前三份用例。
 */
import { afterEach, describe, expect, it, vi } from 'vitest'
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { message } from 'antd'

/** antd 的 `message.*` 回的是一个可 then 的关闭句柄；替身只需要一个哑值，`mockRestore` 收得掉 */
const DUMMY_MSG = (() => {}) as unknown as ReturnType<typeof message.warning>

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

const fx = vi.hoisted(() => {
  const DS_A = {
    dataset_id: '8c7f0201',
    name: 'dsA',
    description: '',
    path: 'D:\\data\\ds_a.json',
    item_count: 3,
    created_at: '2026-10-11T18:52:23.772871',
    updated_at: '2026-10-11T18:52:23.772871',
    tags: [] as string[],
    metadata: {},
  }
  const DS_B = {
    ...DS_A,
    dataset_id: '0d2cace8',
    name: 'dsB',
    description: '描述B',
    path: 'D:\\data\\ds_b.json',
    item_count: 1,
    tags: ['t1', 't2'],
  }
  const LIST = { datasets: [DS_A, DS_B] }
  // 悬空边那一条 issue 的形状逐字抄自探针：`dependency` 里是 SDK 的五键，`message` 是
  // 服务端那句「目标数据集不存在: <id>」。
  const GRAPH_DANGLING = {
    nodes: [
      { id: '8c7f0201', name: 'dsA', item_count: 3 },
      { id: '0d2cace8', name: 'dsB', item_count: 1 },
    ],
    edges: [{ source: '8c7f0201', target: 'ghost12345', type: 'derived' }],
    issues: [
      {
        type: 'missing_target',
        dependency: {
          source_dataset: '8c7f0201',
          target_dataset: 'ghost12345',
          dependency_type: 'derived',
          description: '',
          created_at: '2026-10-11T18:52:23.854090',
        },
        message: '目标数据集不存在: ghost12345',
      },
    ],
  }
  const GRAPH_EMPTY = { nodes: [], edges: [], issues: [] }
  const REGISTER_OK = { ...DS_A, name: 'dsC', dataset_id: '183fb9f0', item_count: 7 }
  const FILES = { files: [{ name: 'ds_a.json', path: 'ds_a.json' }] }
  return {
    DS_A,
    DS_B,
    LIST,
    GRAPH_DANGLING,
    GRAPH_EMPTY,
    REGISTER_OK,
    FILES,
    spies: {} as Record<string, ReturnType<typeof vi.fn>>,
  }
})

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  const s = fx.spies
  s.getServiceStatus = vi.fn().mockResolvedValue({
    status: 'ok',
    version: '3.0.0',
    model_default: 'ernie',
    model_available: true,
    dependencies: {
      installed: { pandas: true },
      missing: [],
      degraded_features: [],
      all_required_present: true,
      available_count: 1,
    },
  })
  s.getVectorStatus = vi.fn().mockResolvedValue({
    enabled: false,
    exists: false,
    count: 0,
    backend: 'faiss',
    dimension: 768,
    collection: 'augmentor',
    storage_dir: '',
  })
  s.listBackups = vi.fn().mockResolvedValue({ backups: [] })
  s.getDataFiles = vi.fn().mockResolvedValue(fx.FILES)
  s.listDependencyDatasets = vi.fn().mockResolvedValue(fx.LIST)
  s.getDependencyGraph = vi.fn().mockResolvedValue(fx.GRAPH_EMPTY)
  s.registerDependencyDataset = vi.fn().mockResolvedValue(fx.REGISTER_OK)
  return {
    ...actual,
    getServiceStatus: s.getServiceStatus,
    getVectorStatus: s.getVectorStatus,
    listBackups: s.listBackups,
    getDataFiles: s.getDataFiles,
    listDependencyDatasets: s.listDependencyDatasets,
    getDependencyGraph: s.getDependencyGraph,
    registerDependencyDataset: s.registerDependencyDataset,
  }
})

import System from './System'

const { LIST, GRAPH_DANGLING, GRAPH_EMPTY, REGISTER_OK, spies } = fx

afterEach(() => {
  cleanup()
  message.destroy()
  // `clearAllMocks` 不清实现，所以逐只把默认回执重新钉回去
  spies.listDependencyDatasets.mockClear().mockResolvedValue(LIST)
  spies.getDependencyGraph.mockClear().mockResolvedValue(GRAPH_EMPTY)
  spies.registerDependencyDataset.mockClear().mockResolvedValue(REGISTER_OK)
})

/** 等已登记数据集落地（挂载即取，所以只需等第一条的名字渲染出来） */
async function waitDatasets() {
  await waitFor(() => expect(spies.listDependencyDatasets).toHaveBeenCalled())
  expect(await screen.findByText('dsA')).toBeInTheDocument()
}

/**
 * 取某一格的**输入框**
 *
 * antd 的 AutoComplete 把 `aria-label` 同时挂在外层 div 与内层 input 上（L231 实测：
 * `getAllByLabelText` 回两个元素），所以按标签取会撞「多个匹配」。这里显式挑 INPUT。
 */
function inputOf(label: string): HTMLInputElement {
  const input = screen
    .getAllByLabelText(label)
    .find((element) => element.tagName === 'INPUT')
  expect(input, `按 ${label} 取不到 INPUT`).toBeTruthy()
  return input as HTMLInputElement
}

describe('System 数据集血缘列表与图', () => {
  it('已登记数据集按回显原样渲染，空描述与空标签各有可见占位', async () => {
    render(<System />)
    await waitDatasets()
    expect(screen.getByText('dsB')).toBeInTheDocument()
    // 描述那一格：dsB 有值、dsA 是空串 ⇒ 空的那格要出占位，不能留白
    expect(screen.getByText('描述B')).toBeInTheDocument()
    expect(screen.getByText('（空）')).toBeInTheDocument()
    // 标签那一格：dsB 的两只标签渲成 Tag，dsA 无标签出占位
    expect(screen.getByText('t1')).toBeInTheDocument()
    expect(screen.getByText('t2')).toBeInTheDocument()
    expect(screen.getByText('（无）')).toBeInTheDocument()
    // 条数与服务端绝对路径照抄回显
    expect(screen.getByText('3')).toBeInTheDocument()
    expect(screen.getByText('D:\\data\\ds_a.json')).toBeInTheDocument()
  })

  it('图的三格计数按回显渲染，issues 非空时逐条列出 type 与 message', async () => {
    spies.getDependencyGraph.mockResolvedValue(GRAPH_DANGLING)
    render(<System />)
    await waitDatasets()
    await waitFor(() => expect(spies.getDependencyGraph).toHaveBeenCalled())
    expect(await screen.findByText('图中节点')).toBeInTheDocument()
    expect(screen.getByText('图中边')).toBeInTheDocument()
    expect(screen.getByText('校验问题')).toBeInTheDocument()
    // 悬空边那一条 issue 要带着服务端那句原话出现在页面上
    expect(
      await screen.findByText(/依赖关系上有 1 处问题/)
    ).toBeInTheDocument()
    expect(
      screen.getByText('missing_target：目标数据集不存在: ghost12345')
    ).toBeInTheDocument()
  })

  it('血缘那一面取不到只降级自己，服务状态与备份面照常渲染', async () => {
    spies.listDependencyDatasets.mockRejectedValue(new Error('boom'))
    spies.getDependencyGraph.mockRejectedValue(new Error('boom'))
    render(<System />)
    expect(await screen.findByText('已登记数据集未取到')).toBeInTheDocument()
    // 上面那两张卡不受影响：服务状态照渲，备份面那句白名单口径还在
    expect(screen.getByText('服务状态')).toBeInTheDocument()
    expect(screen.getByText('数据备份')).toBeInTheDocument()
  })
})

describe('System 登记数据集', () => {
  it('未填名称就点登记：只给一句 warning，不发请求', async () => {
    const warn = vi.spyOn(message, 'warning').mockImplementation(() => DUMMY_MSG)
    render(<System />)
    await waitDatasets()
    fireEvent.click(screen.getByLabelText('登记数据集'))
    expect(warn).toHaveBeenCalledWith('先填数据集名称——登记 id 只由名字算出来')
    expect(spies.registerDependencyDataset).not.toHaveBeenCalled()
    warn.mockRestore()
  })

  it('填了名称没填源文件：仍是 warning，不发请求', async () => {
    const warn = vi.spyOn(message, 'warning').mockImplementation(() => DUMMY_MSG)
    render(<System />)
    await waitDatasets()
    fireEvent.change(inputOf('数据集名称'), { target: { value: 'dsC' } })
    fireEvent.click(screen.getByLabelText('登记数据集'))
    expect(warn).toHaveBeenCalledWith('先填要登记的那份服务端数据文件')
    expect(spies.registerDependencyDataset).not.toHaveBeenCalled()
    warn.mockRestore()
  })

  it('填齐后按 (名称, 文件, { description, tags }) 发，逗号切成数组且空段被丢，回执报 id 与条数，列表与图都重取', async () => {
    const ok = vi.spyOn(message, 'success').mockImplementation(() => DUMMY_MSG)
    render(<System />)
    await waitDatasets()
    const before = spies.listDependencyDatasets.mock.calls.length
    const graphBefore = spies.getDependencyGraph.mock.calls.length

    fireEvent.change(inputOf('数据集名称'), { target: { value: 'dsC' } })
    fireEvent.change(inputOf('数据集源文件'), { target: { value: 'ds_a.json' } })
    fireEvent.change(inputOf('数据集描述'), { target: { value: '第三份' } })
    // 中英文逗号混着写、还带首尾空格与一个空段：切完必须是 ['t1','t2']，不许把空串送上去
    fireEvent.change(inputOf('数据集标签'), { target: { value: 't1， t2 ,, ' } })
    fireEvent.click(screen.getByLabelText('登记数据集'))

    await waitFor(() =>
      expect(spies.registerDependencyDataset).toHaveBeenCalledWith('dsC', 'ds_a.json', {
        description: '第三份',
        tags: ['t1', 't2'],
      })
    )
    expect(ok).toHaveBeenCalledWith('已登记 dsC（183fb9f0，7 条）')
    // 登记成功后列表与图都要重取（不是只刷一边）
    await waitFor(() =>
      expect(spies.listDependencyDatasets.mock.calls.length).toBeGreaterThan(before)
    )
    await waitFor(() =>
      expect(spies.getDependencyGraph.mock.calls.length).toBeGreaterThan(graphBefore)
    )
    ok.mockRestore()
  })

  it('白名单外被判 403 时，页面上是服务端那句判决而不是「失败了」', async () => {
    const err = vi.spyOn(message, 'error').mockImplementation(() => DUMMY_MSG)
    spies.registerDependencyDataset.mockRejectedValue({
      response: { status: 403, data: { detail: '路径超出允许的数据目录范围' } },
    })
    render(<System />)
    await waitDatasets()
    fireEvent.change(inputOf('数据集名称'), { target: { value: 'dsC' } })
    fireEvent.change(inputOf('数据集源文件'), { target: { value: 'D:\\outside\\x.json' } })
    fireEvent.click(screen.getByLabelText('登记数据集'))
    await waitFor(() =>
      expect(err).toHaveBeenCalledWith(expect.stringContaining('路径超出允许的数据目录范围'))
    )
    err.mockRestore()
  })
})

describe('System 血缘卡的诚实口径', () => {
  it('六条实测脾气都留在卡上，一条都不许被美化掉', async () => {
    render(<System />)
    await waitDatasets()
    // A309：边在产品面上建不出来 + 登记了删不掉
    expect(screen.getByText(/恒为 0.*不是没数据/s)).toBeInTheDocument()
    expect(screen.getByText(/add_dependency 建立（A309）/s)).toBeInTheDocument()
    expect(screen.getByText(/在产品面上删不掉/s)).toBeInTheDocument()
    expect(screen.getByText(/连 SDK 都没有 remove_dataset/s)).toBeInTheDocument()
    // A310：issues 为空不能读成「没问题」
    expect(screen.getByText(/不查环/s)).toBeInTheDocument()
    expect(screen.getByText(/0 不能读成「依赖关系没问题」（A310）/s)).toBeInTheDocument()
    // A306：同名覆盖 + created_at 被刷新，且没有 409
    expect(screen.getByText(/md5\(名称\) 的前 8 位/s)).toBeInTheDocument()
    expect(screen.getByText(/created_at 会被刷新成第二次的时间/s)).toBeInTheDocument()
    expect(screen.getByText(/（A306）/s)).toBeInTheDocument()
    // A305：描述与标签曾回显即丢，本轮已修
    expect(screen.getByText(/回显即丢/s)).toBeInTheDocument()
    expect(screen.getByText(/随登记一起写下去（A305）/s)).toBeInTheDocument()
    // A308：三份默认注册表不是同一份；A307：只读端点会建目录
    expect(screen.getByText(/两边不是同一份/s)).toBeInTheDocument()
    expect(screen.getByText(/（A308）/s)).toBeInTheDocument()
    expect(screen.getByText(/把它建出来/s)).toBeInTheDocument()
    expect(screen.getByText(/（A307）/s)).toBeInTheDocument()
    // A312 / A302：写那一支挂鉴权门禁；**L233 起前端有了送头的通道**（key 存在「配置管理」
    // 页那张卡上，由 `api.ts` 的请求拦截器挂到 X-API-Key）——这两条原先钉的是「没有通道」，
    // 前提被本轮改掉，按新世界重钉（通道的形状由 test_api_key_channel_l233.py 与 api.test.ts 钉）。
    expect(screen.getByText(/登记这一支挂鉴权门禁（两条 GET 不挂）/s)).toBeInTheDocument()
    // 备份卡、血缘卡、迁移与流式卡各指一次（L234 起是 3 处，与 test_api_key_channel_l233.py 的
    // `page.count(CARD) == 3` 同一份读数）
    expect(screen.getAllByText(/配置管理 → 写操作鉴权/s)).toHaveLength(3)
    // 这一格 L234 起页面上有两处（血缘卡与新的迁移/流式卡都写着同一批号），所以按数目钉
    expect(screen.getAllByText(/（A302 \/ A312 已收）/s)).toHaveLength(2)
    expect(screen.getByText(/两种错误体都要认/s)).toBeInTheDocument()
  })
})
