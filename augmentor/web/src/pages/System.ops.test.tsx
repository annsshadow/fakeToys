/**
 * `System` 的质量监控 / 自测 / 迁移 / 流式四面（L234：A279 第九批接线）
 *
 * 存在理由与 L229–L232 那四份同构：Python 侧的 `test_system_ops_wiring_l234.py` 只能证明
 * **源码写了**那几个调用形状、那几句口径，以及端点在真链路上的读数；证不出渲染面上真的看得见。
 * 这一份钉九件事：
 * ① 快照的三个指标格按回显渲染，`metrics` 是空对象时显示「-」而不是 0（0 与「没算」是两件事）；
 * ② 告警非空时逐条列出 `severity` 与**服务端英文原文**，为 0 时一张 Alert 都不出；
 * ③ 自测五条按服务端顺序渲染，`pass_rate` 乘 100 显示，说明列照抄那句**截断过的**明细；
 * ④ 自测那一支**不送 suite**（服务端只认 default，空串是 400，所以页面连那一格都不给）；
 * ⑤ 两支只读动作的必填闸：没填源文件只给一句 warning、**不发请求**；
 * ⑥ 迁移走二次确认（Popconfirm），确认之前一发都不发；规则一格不选时递给服务层的是
 *    `{ rules: [] }`（由服务层丢成「整键不送」，那一半在 `api.test.ts` 钉）；
 * ⑦ 流式那一支把三个默认档原样递下去，「被丢掉」是本页自己算的（读入 − 写出），不是回执里的
 *    `processed`（那格四档实测都等于读入条数）；
 * ⑧ 下拉不是装饰：选了 `quality` 与 `json` 就照选的发；
 * ⑨ 写盘那两支挂门禁，401 时页面上是服务端那句判决并指到存 key 的那张卡；
 *    以及两张卡上那十几句实测脾气一句都不许被美化掉。
 *
 * 夹具里的 snapshot_id、时间戳、通过率、绝对路径逐字来自仓外探针在真实端点上量到的回执
 * （`l234_fixtures.py` 与 `l234_empty.py`，L234 跑完即删），不是编的。服务层用
 * `importOriginal` 做部分替身，`apiErrorDetail` 留真身，理由同前四份用例。
 */
import { afterEach, describe, expect, it, vi } from 'vitest'
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { message } from 'antd'

/** antd 的 `message.*` 回的是一个可 then 的关闭句柄；替身只需要一个哑值，`mockRestore` 收得掉 */
const DUMMY_MSG = (() => {}) as unknown as ReturnType<typeof message.warning>

window.matchMedia =
  window.matchMedia ||
  ((query: string) =>
    ({
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
  // 5 条混合夹具（一组重复、一条过短、一条带全角标点与 HTML）上量到的三份回执
  const SNAPSHOT_MIXED = {
    snapshot_id: 'ae0f6ddd',
    timestamp: '2026-10-11T20:52:18.680935',
    metrics: { completeness: 1.0, diversity: 0.8, consistency: 0.8 },
    alerts: [] as unknown[],
  }
  // 空数组那一档：metrics 是**空对象**，不是三个 0
  const SNAPSHOT_EMPTY = {
    snapshot_id: '3eff2592',
    timestamp: '2026-10-11T20:55:50.360474',
    metrics: {},
    alerts: [] as unknown[],
  }
  // 三条都过短那一档：consistency 触阈值，告警正文是服务端英文原文
  const SNAPSHOT_ALERT = {
    snapshot_id: 'a222c0ed',
    timestamp: '2026-10-11T20:52:18.676058',
    metrics: { completeness: 1.0, diversity: 1.0, consistency: 0.0 },
    alerts: [
      {
        alert_id: 'consistency_below',
        metric_name: 'consistency',
        current_value: 0.0,
        threshold_value: 0.7,
        alert_type: 'below',
        severity: 'critical',
        message: 'consistency below threshold: 0.00 < 0.70',
        timestamp: '2026-10-11T20:52:18.676013',
      },
    ],
  }
  const SUITE_MIXED = {
    name: 'default',
    description: '默认测试套件',
    total_tests: 5,
    passed_tests: 3,
    failed_tests: 2,
    pass_rate: 0.6,
    results: [
      {
        test_id: 'format_check',
        test_name: '格式检查',
        passed: true,
        message: '格式检查通过，共 5 条数据',
        execution_time_ms: 0.0035762786865234375,
        timestamp: '2026-10-11T20:52:18.685067',
      },
      {
        test_id: 'field_check',
        test_name: '字段检查',
        passed: true,
        message: '字段检查通过，所有数据都包含必填字段',
        execution_time_ms: 0.0030994415283203125,
        timestamp: '2026-10-11T20:52:18.685077',
      },
      {
        test_id: 'quality_check',
        test_name: '质量检查',
        passed: false,
        message: '发现 1 个过短的问题',
        execution_time_ms: 0.0021457672119140625,
        timestamp: '2026-10-11T20:52:18.685081',
      },
      {
        test_id: 'diversity_check',
        test_name: '多样性检查',
        passed: true,
        message: '多样性检查通过: 80.00%',
        execution_time_ms: 0.0040531158447265625,
        timestamp: '2026-10-11T20:52:18.685087',
      },
      {
        test_id: 'duplicate_check',
        test_name: '重复检查',
        passed: false,
        message: '发现 1 个重复问题',
        execution_time_ms: 0.0026226043701171875,
        timestamp: '2026-10-11T20:52:18.685091',
      },
    ],
  }
  // 空数据集那一档：五条里有三条判「数据为空」，通过率 0.4
  const SUITE_EMPTY = {
    ...SUITE_MIXED,
    passed_tests: 2,
    failed_tests: 3,
    pass_rate: 0.4,
    results: SUITE_MIXED.results.map((row, index) =>
      index < 2
        ? { ...row, message: index === 0 ? '格式检查通过，共 0 条数据' : row.message }
        : { ...row, passed: false, message: '数据为空' },
    ),
  }
  const STREAM_QUALITY = { total_input: 5, total_output: 2, processed: 5 }
  const MIGRATION = {
    migration_id: 'fd606983',
    source_path: 'D:\\data\\ds\\mixed.json',
    target_path: 'D:\\data\\ds\\mig.json',
    total_items: 5,
    migrated_items: 5,
    failed_items: 0,
    rules_applied: ['rename_instruction', 'rename_output', 'flatten_conversations'],
    errors: [] as unknown[],
    timestamp: '2026-10-11T20:52:18.698832',
  }
  return {
    SNAPSHOT_MIXED,
    SNAPSHOT_EMPTY,
    SNAPSHOT_ALERT,
    SUITE_MIXED,
    SUITE_EMPTY,
    STREAM_QUALITY,
    MIGRATION,
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
    model_retry_count: 0,
    model_retry_wait_seconds: 0,
    // 契约必填（无后端时回 null）；夹具走的是未标注类型的 mockResolvedValue，
    // tsc 管不到，漏了就会在渲染面崩 —— 2026-10-25 就是这么红的
    model_generate_seconds: 3.2,
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
  s.getBackupSourceCandidates = vi
    .fn()
    .mockResolvedValue({ files: [{ name: 'mixed.json', path: 'mixed.json' }] })
  s.listDependencyDatasets = vi.fn().mockResolvedValue({ datasets: [] })
  s.getDependencyGraph = vi.fn().mockResolvedValue({ nodes: [], edges: [], issues: [] })
  // L236 起依赖诊断自取一面。不给回执 ⇒ 页面退回综合状态里那一份，本份用例渲染结果不变
  s.systemDependencies = vi.fn()
  s.runQualityMonitor = vi.fn().mockResolvedValue(fx.SNAPSHOT_MIXED)
  s.runDatasetAutoTest = vi.fn().mockResolvedValue(fx.SUITE_MIXED)
  s.migrateDataset = vi.fn().mockResolvedValue(fx.MIGRATION)
  s.streamProcess = vi.fn().mockResolvedValue(fx.STREAM_QUALITY)
  return {
    ...actual,
    getServiceStatus: s.getServiceStatus,
    getVectorStatus: s.getVectorStatus,
    listBackups: s.listBackups,
    getBackupSourceCandidates: s.getBackupSourceCandidates,
    listDependencyDatasets: s.listDependencyDatasets,
    getDependencyGraph: s.getDependencyGraph,
    systemDependencies: s.systemDependencies,
    runQualityMonitor: s.runQualityMonitor,
    runDatasetAutoTest: s.runDatasetAutoTest,
    migrateDataset: s.migrateDataset,
    streamProcess: s.streamProcess,
  }
})

import System from './System'

const {
  SNAPSHOT_MIXED,
  SNAPSHOT_EMPTY,
  SNAPSHOT_ALERT,
  SUITE_MIXED,
  SUITE_EMPTY,
  STREAM_QUALITY,
  MIGRATION,
  spies,
} = fx

afterEach(() => {
  cleanup()
  message.destroy()
  // `clearAllMocks` 不清实现，所以逐只把默认回执重新钉回去
  spies.runQualityMonitor.mockClear().mockResolvedValue(SNAPSHOT_MIXED)
  spies.runDatasetAutoTest.mockClear().mockResolvedValue(SUITE_MIXED)
  spies.migrateDataset.mockClear().mockResolvedValue(MIGRATION)
  spies.streamProcess.mockClear().mockResolvedValue(STREAM_QUALITY)
})

/**
 * 取某一格的**输入框**
 *
 * antd 的 AutoComplete 把 `aria-label` 同时挂在外层 div 与内层 input 上（L231 实测），
 * InputNumber 则只挂在内层 input 上 ⇒ 两种形状都要认，统一在这里挑出 INPUT。
 */
function fieldOf(container: HTMLElement, label: string): HTMLInputElement {
  const holder = container.querySelector(`[aria-label="${label}"]`)
  const input = holder?.tagName === 'INPUT' ? holder : holder?.querySelector('input')
  expect(input, `按 ${label} 取不到 INPUT`).toBeTruthy()
  return input as HTMLInputElement
}

/**
 * 在第 `label` 那一只 Select 里挑一项
 *
 * 不按「本页第几只 Select」数：antd 的 AutoComplete 也建在 Select 上，同样带
 * `.ant-select`（v5 时代这个类叫 `.ant-select-selector`，antd 6 已移除），首跑就是那么红的（按序号点开的是文件下拉，里面当然没有
 * 「quality · 按质量分过滤」这一项）。按 aria-label 落到那一格自己才是这里要钉的接线。
 * 下拉也只认**没被收起**的那一只：点过一次的 dropdown 会留在 body 上。
 */
function pickOption(container: HTMLElement, label: string, optionLabel: string) {
  const holder = container.querySelector(`[aria-label="${label}"]`)
  expect(holder, `页面上没有「${label}」这一格`).toBeTruthy()
  const select = holder!.closest('.ant-select')
  expect(select, `「${label}」不是一只 Select`).toBeTruthy()
  fireEvent.mouseDown(select!)
  const dropdowns = document.querySelectorAll(
    '.ant-select-dropdown:not(.ant-select-dropdown-hidden)',
  )
  expect(dropdowns.length, `「${label}」的下拉没打开`).toBeGreaterThan(0)
  const option = Array.from(
    dropdowns[dropdowns.length - 1].querySelectorAll('.ant-select-item-option'),
  ).find((node) => node.textContent === optionLabel)
  expect(option, `下拉里没有「${optionLabel}」这一项`).toBeTruthy()
  fireEvent.click(option!)
}

/** 按卡标题取那一张 Card：本页多张表都在渲 `.ant-table-row`，按全文取会串到别的卡上 */
function cardOf(container: HTMLElement, title: string): HTMLElement {
  const card = Array.from(container.querySelectorAll('.ant-card')).find(
    (node) => node.querySelector('.ant-card-head-title')?.textContent === title,
  )
  expect(card, `页面上没有「${title}」这张卡`).toBeTruthy()
  return card as HTMLElement
}

/**
 * 按标题取某一格 Statistic 的值
 *
 * 不用 `getByText('0.8')`：本页十几格 Statistic 与上面几张卡（向量状态、备份、依赖）都在
 * 渲裸数字，按文本取会撞「多个匹配」，而那正是把判据放松成 `getAllByText` 的开头。
 * 按标题定位才是这里要钉的东西——「完整度那一格是不是服务端回的那个数」。
 */
function statValue(container: HTMLElement, title: string): string {
  const box = Array.from(container.querySelectorAll('.ant-statistic')).find(
    (node) => node.querySelector('.ant-statistic-title')?.textContent === title,
  )
  expect(box, `页面上没有「${title}」这一格`).toBeTruthy()
  return box!.querySelector('.ant-statistic-content-value')!.textContent!
}

/** 挂上页面并等文件列表落地（两张新卡的下拉选项都来自它） */
async function mount() {
  const view = render(<System />)
  await waitFor(() => expect(spies.getBackupSourceCandidates).toHaveBeenCalled())
  expect(await screen.findByText('质量监控与自测')).toBeInTheDocument()
  return view.container
}

describe('System 质量监控快照', () => {
  it('三个指标格照回显渲染，告警为 0 时一张 Alert 都不出', async () => {
    const container = await mount()
    fireEvent.change(fieldOf(container, '监控数据文件'), { target: { value: 'mixed.json' } })
    fireEvent.click(screen.getByLabelText('生成质量快照'))

    await waitFor(() => expect(spies.runQualityMonitor).toHaveBeenCalledWith('mixed.json'))
    expect(await screen.findByText('完整度')).toBeInTheDocument()
    // 1.0 / 0.8 / 0.8 与告警数 0：四格都是服务端回显，不是页面自己算出来的
    expect(statValue(container, '完整度')).toBe('1')
    expect(statValue(container, '多样性')).toBe('0.8')
    expect(statValue(container, '一致性')).toBe('0.8')
    expect(statValue(container, '触发告警')).toBe('0')
    expect(screen.queryByText(/触发 \d+ 条告警/)).not.toBeInTheDocument()
  })

  it('metrics 是空对象时三格显示「-」而不是 0（0 与「没算」是两件事）', async () => {
    spies.runQualityMonitor.mockResolvedValue(SNAPSHOT_EMPTY)
    const container = await mount()
    fireEvent.change(fieldOf(container, '监控数据文件'), { target: { value: 'empty.json' } })
    fireEvent.click(screen.getByLabelText('生成质量快照'))

    await waitFor(() => expect(spies.runQualityMonitor).toHaveBeenCalled())
    expect(await screen.findByText('完整度')).toBeInTheDocument()
    expect(statValue(container, '完整度')).toBe('-')
    expect(statValue(container, '多样性')).toBe('-')
    expect(statValue(container, '一致性')).toBe('-')
  })

  it('告警非空时逐条列出 severity 与服务端英文原文', async () => {
    spies.runQualityMonitor.mockResolvedValue(SNAPSHOT_ALERT)
    const container = await mount()
    fireEvent.change(fieldOf(container, '监控数据文件'), { target: { value: 'bad.json' } })
    fireEvent.click(screen.getByLabelText('生成质量快照'))

    expect(await screen.findByText(/快照 a222c0ed 触发 1 条告警/)).toBeInTheDocument()
    expect(screen.getByText('critical')).toBeInTheDocument()
    // 告警正文是服务端英文原文，页面上没有中文那一份
    expect(screen.getByText('consistency below threshold: 0.00 < 0.70')).toBeInTheDocument()
    // consistency 那一格是 0（真算出来的 0），与上面「空对象显示 -」那一档必须能分开
    expect(statValue(container, '一致性')).toBe('0')
    expect(statValue(container, '触发告警')).toBe('1')
  })

  it('没填源文件就点两支：各一句 warning，两支都不发请求', async () => {
    const warn = vi.spyOn(message, 'warning').mockImplementation(() => DUMMY_MSG)
    await mount()
    fireEvent.click(screen.getByLabelText('生成质量快照'))
    fireEvent.click(screen.getByLabelText('运行自动化测试'))

    expect(warn).toHaveBeenCalledWith('先填要做质量监控的那份服务端数据文件')
    expect(warn).toHaveBeenCalledWith('先填要跑自动化测试的那份服务端数据文件')
    expect(spies.runQualityMonitor).not.toHaveBeenCalled()
    expect(spies.runDatasetAutoTest).not.toHaveBeenCalled()
    warn.mockRestore()
  })

  it('取不到时页面上是服务端那句判决，不是页面自己编的「失败了」', async () => {
    const err = vi.spyOn(message, 'error').mockImplementation(() => DUMMY_MSG)
    spies.runQualityMonitor.mockRejectedValue({
      response: { status: 404, data: { detail: '文件不存在: ghost.json' } },
    })
    const container = await mount()
    fireEvent.change(fieldOf(container, '监控数据文件'), { target: { value: 'ghost.json' } })
    fireEvent.click(screen.getByLabelText('生成质量快照'))

    await waitFor(() =>
      expect(err).toHaveBeenCalledWith(expect.stringContaining('文件不存在: ghost.json')),
    )
    err.mockRestore()
  })
})

describe('System 自动化测试', () => {
  it('五条按服务端顺序渲染，通过率乘 100，说明列照抄明细，且**不送 suite**', async () => {
    const ok = vi.spyOn(message, 'success').mockImplementation(() => DUMMY_MSG)
    const container = await mount()
    fireEvent.change(fieldOf(container, '监控数据文件'), { target: { value: 'mixed.json' } })
    fireEvent.click(screen.getByLabelText('运行自动化测试'))

    // 只递一个实参：suite 那一格页面根本不给（服务端只认 default，空串是 400）
    await waitFor(() => expect(spies.runDatasetAutoTest).toHaveBeenCalledWith('mixed.json'))
    expect(await screen.findByText('通过率')).toBeInTheDocument()
    expect(statValue(container, '通过率')).toBe('60') // pass_rate 0.6 是 0–1 的小数，页面乘 100
    expect(statValue(container, '套件')).toBe('default')
    const rows = Array.from(
      cardOf(container, '质量监控与自测').querySelectorAll('.ant-table-tbody tr.ant-table-row'),
    )
    expect(rows.map((row) => row.querySelector('td')?.textContent)).toEqual([
      '格式检查',
      '字段检查',
      '质量检查',
      '多样性检查',
      '重复检查',
    ])
    // 结果那一列按行渲成 Tag，顺序与上面五条一一对应
    expect(rows.map((row) => row.querySelector('.ant-tag')?.textContent)).toEqual([
      '通过',
      '通过',
      '失败',
      '通过',
      '失败',
    ])
    // 说明那一列是服务端**截断过的**明细，页面照抄而不是自己概括
    expect(screen.getByText('发现 1 个过短的问题')).toBeInTheDocument()
    expect(screen.getByText('发现 1 个重复问题')).toBeInTheDocument()
    expect(ok).toHaveBeenCalledWith('通过率 60%（3/5）')
    ok.mockRestore()
  })

  it('空数据集那一档：通过率 40%，三条判「数据为空」', async () => {
    spies.runDatasetAutoTest.mockResolvedValue(SUITE_EMPTY)
    const container = await mount()
    fireEvent.change(fieldOf(container, '监控数据文件'), { target: { value: 'empty.json' } })
    fireEvent.click(screen.getByLabelText('运行自动化测试'))

    await waitFor(() => expect(spies.runDatasetAutoTest).toHaveBeenCalled())
    await waitFor(() => expect(statValue(container, '通过率')).toBe('40'))
    expect(screen.getAllByText('数据为空')).toHaveLength(3)
  })
})

describe('System 迁移与流式', () => {
  it('迁移走二次确认，确认之前一发都不发；规则不选时递 { rules: [] }', async () => {
    const ok = vi.spyOn(message, 'success').mockImplementation(() => DUMMY_MSG)
    const container = await mount()
    fireEvent.change(fieldOf(container, '源数据文件'), { target: { value: 'mixed.json' } })
    fireEvent.change(fieldOf(container, '迁移产物文件'), { target: { value: 'mig.json' } })
    fireEvent.click(screen.getByLabelText('迁移数据集结构'))

    // Popconfirm 那一句是这一支的判决面：破坏性改名 + 整份覆盖
    expect(await screen.findByText('迁移会改字段名，产物已存在则整份覆盖')).toBeInTheDocument()
    expect(spies.migrateDataset).not.toHaveBeenCalled()
    // antd 会给「恰好两个汉字」的按钮文案中间插一个空格（accessible name 成了 `照 改`）
    fireEvent.click(screen.getByText(/照\s*改/))

    await waitFor(() =>
      expect(spies.migrateDataset).toHaveBeenCalledWith('mixed.json', 'mig.json', { rules: [] }),
    )
    await waitFor(() => expect(statValue(container, '迁移 id')).toBe('fd606983'))
    expect(statValue(container, '失败条数')).toBe('0')
    expect(
      screen.getByText(/rename_instruction \/ rename_output \/ flatten_conversations/),
    ).toBeInTheDocument()
    expect(ok).toHaveBeenCalledWith(`已迁移 5/5 条 ⇒ ${MIGRATION.target_path}`)
    ok.mockRestore()
  })

  it('没填源文件或产物名就点两支：各一句 warning，两支都不发请求', async () => {
    const warn = vi.spyOn(message, 'warning').mockImplementation(() => DUMMY_MSG)
    const container = await mount()
    fireEvent.click(screen.getByLabelText('流式处理'))
    expect(warn).toHaveBeenCalledWith('先填要流式处理的那份服务端数据文件')

    fireEvent.change(fieldOf(container, '源数据文件'), { target: { value: 'mixed.json' } })
    fireEvent.click(screen.getByLabelText('流式处理'))
    expect(warn).toHaveBeenCalledWith('先填流式产物的文件名')
    expect(spies.streamProcess).not.toHaveBeenCalled()

    // 迁移那一支的必填闸在 `handleMigrate` 里，而它挂在 Popconfirm 的 onConfirm 上 ⇒
    // 要点到「照改」才够得着。这也是量出来的一格脾气：没填产物名时用户会先看见二次确认，
    // 确认之后才得到那句 warning（不是先拦下来）。
    fireEvent.click(screen.getByLabelText('迁移数据集结构'))
    fireEvent.click(await screen.findByText(/照\s*改/))
    await waitFor(() => expect(warn).toHaveBeenCalledWith('先填迁移产物的文件名'))
    expect(spies.migrateDataset).not.toHaveBeenCalled()
    warn.mockRestore()
  })

  it('流式默认档原样递下去，「被丢掉」是本页算的读入 − 写出而不是 processed', async () => {
    const ok = vi.spyOn(message, 'success').mockImplementation(() => DUMMY_MSG)
    const container = await mount()
    fireEvent.change(fieldOf(container, '源数据文件'), { target: { value: 'mixed.json' } })
    fireEvent.change(fieldOf(container, '流式产物文件'), { target: { value: 'out.jsonl' } })
    fireEvent.click(screen.getByLabelText('流式处理'))

    // 三个默认档（'' / '' / null）由服务层丢成「整键不送」，那一半在 api.test.ts 钉
    await waitFor(() =>
      expect(spies.streamProcess).toHaveBeenCalledWith('mixed.json', 'out.jsonl', {
        operation: '',
        chunkSize: undefined,
        outputFormat: '',
      }),
    )
    // 「被丢掉」= 5 − 2 = 3，而回执里的 processed 是 5 ⇒ 这一格必须是页面自己算的
    await waitFor(() => expect(statValue(container, '被丢掉')).toBe('3'))
    expect(statValue(container, '写出条数')).toBe('2')
    expect(ok).toHaveBeenCalledWith('读入 5 条，写出 2 条')
    ok.mockRestore()
  })

  it('下拉不是装饰：选了 quality / json / 250 就照选的发', async () => {
    const container = await mount()
    fireEvent.change(fieldOf(container, '源数据文件'), { target: { value: 'mixed.json' } })
    fireEvent.change(fieldOf(container, '流式产物文件'), { target: { value: 'out.json' } })
    pickOption(container, '流式操作', 'quality · 按质量分过滤')
    pickOption(container, '产物格式', 'json · 一整个数组')
    fireEvent.change(fieldOf(container, '分块大小'), { target: { value: '250' } })
    fireEvent.click(screen.getByLabelText('流式处理'))

    await waitFor(() =>
      expect(spies.streamProcess).toHaveBeenCalledWith('mixed.json', 'out.json', {
        operation: 'quality',
        chunkSize: 250,
        outputFormat: 'json',
      }),
    )
  })

  it('门禁升起时 401 的那一句是服务端原文，并指到存 key 的那张卡', async () => {
    const err = vi.spyOn(message, 'error').mockImplementation(() => DUMMY_MSG)
    spies.streamProcess.mockRejectedValue({
      response: { status: 401, data: { detail: '缺少或无效的 X-API-Key' } },
    })
    const container = await mount()
    fireEvent.change(fieldOf(container, '源数据文件'), { target: { value: 'mixed.json' } })
    fireEvent.change(fieldOf(container, '流式产物文件'), { target: { value: 'out.jsonl' } })
    fireEvent.click(screen.getByLabelText('流式处理'))

    await waitFor(() => expect(err).toHaveBeenCalled())
    const text = err.mock.calls[0][0] as string
    expect(text).toContain('缺少或无效的 X-API-Key')
    expect(text).toContain('配置管理 → 写操作鉴权')
    err.mockRestore()
  })
})

describe('System 两张新卡的诚实口径', () => {
  it('十句实测脾气都留在卡上，一条都不许被美化掉', async () => {
    await mount()
    // 监控与自测那张卡
    expect(screen.getByText(/不是「质量合格」/s)).toBeInTheDocument()
    expect(screen.getByText(/completeness 0\.7 \/ diversity 0\.4/s)).toBeInTheDocument()
    expect(screen.getByText(/服务端英文原文/s)).toBeInTheDocument()
    expect(screen.getByText(/空对象/s)).toBeInTheDocument()
    expect(screen.getByText(/0 与「没算」是两件事/s)).toBeInTheDocument()
    expect(screen.getByText(/suite 这一格页面\*\*不送\*\*/s)).toBeInTheDocument()
    expect(screen.getByText(/截断过的/s)).toBeInTheDocument()
    expect(screen.getByText(/这一族没有只读 GET/s)).toBeInTheDocument()
    // 迁移与流式那张卡
    expect(screen.getByText(/各档实测都等于读入条数/s)).toBeInTheDocument()
    expect(screen.getByText(/none 5 \/ quality 2 \/ dedup 3 \/ clean 5/s)).toBeInTheDocument()
    expect(screen.getByText(/一条不丢，改写内容/s)).toBeInTheDocument()
    expect(screen.getByText(/以格式为准、扩展名不改/s)).toBeInTheDocument()
    expect(screen.getByText(/chunk_size 必须是不小于 1 的整数/s)).toBeInTheDocument()
    expect(screen.getByText(/留空等于三条全用/s)).toBeInTheDocument()
    expect(screen.getByText(/静默无操作/s)).toBeInTheDocument()
    expect(screen.getByText(/没有 409、也没有「已存在」提示/s)).toBeInTheDocument()
  })
})
