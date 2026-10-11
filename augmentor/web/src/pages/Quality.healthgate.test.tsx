/**
 * `Quality` 的健康门禁一面（L235：A279 第十批接线）
 *
 * 存在理由与 L229–L234 那几份同构：Python 侧的 `test_health_gate_wiring_l235.py` 只能证明
 * **源码写了**那些调用形状、那几句口径，以及端点在真链路上的读数；证不出渲染面上真的看得见。
 * 这一份钉八件事：
 * ① `run` 守卫——没选数据集时点「健康门禁」只给一句 warning、**不发请求**；
 * ② 默认档递参：五个初值原样递下去，`weights` 与 `passRate` 是 **undefined**（不是 null、
 *    不是 0）—— 服务端那四个非 Optional 字段送 null 是 422，而 weights/pass_rate 留空才是
 *    「用后端默认」与「不判这一项」；
 * ③ 一键喂通过率：**0 也喂得进去**（服务层那句「pass_rate 不许用 `||`」禁令的渲染面证据），
 *    且没有评估结果时那颗按钮是禁用的；
 * ④ 权重「要么四个全填、要么全留空」：只填一个 ⇒ 按钮禁用 + 说明文案，四个全填 ⇒ 按
 *    完整性/多样性/质量均衡/覆盖率的顺序递下去；
 * ⑤ verdict 三档各有渲染证据，且 warned 那一档把 `passed = true` **明写出来**；
 * ⑥ `block_on_warning` 那一档：`failed_rules` 是空的、名字住在 warned 行里，页面照实渲染；
 * ⑦ 指标表把「健康分那四项」与「门禁实际看到的那一份」并排列出（后者多 duplicate_rate）；
 * ⑧ 失败时显示服务端那句判决原文，以及那几句实测脾气一句都不许被美化掉。
 *
 * 夹具里的 health_score、四项指标、duplicate_rate、verdict 与四个名单逐字来自仓外探针在
 * 真实端点上量到的回执（`l235_probe6.py`，L235 跑完即删；小数按探针打印的 4 位取），不是编的。
 * 服务层用 `importOriginal` 做部分替身，`apiErrorDetail` 留真身，理由同前几份用例。
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
  // dup.json（4 条，前两条完全重复）在默认阈值下的回执：verdict failed
  const FAILED = {
    health: {
      health_score: 0.8927055856521438,
      level: 'healthy',
      metrics: { completeness: 1.0, diversity: 0.75, quality_balance: 0.8208, coverage: 1.0 },
      weights: [0.25, 0.25, 0.25, 0.25],
      total_samples: 4,
    },
    gate: {
      verdict: 'failed',
      passed: false,
      failed_rules: ['duplicate_rate'],
      warned_rules: [] as string[],
      errored_rules: [] as string[],
      metrics: {
        completeness: 1.0,
        diversity: 0.75,
        quality_balance: 0.8208,
        coverage: 1.0,
        duplicate_rate: 0.5,
      },
    },
    skipped_rules: ['pass_rate'],
  }
  // sparse.json（4 条互不重复、output 一空一缺）：只有 warning 级判负 ⇒ warned 而 passed 仍 true
  const WARNED = {
    health: {
      health_score: 0.6250000833333056,
      level: 'moderate',
      metrics: { completeness: 0.5, diversity: 1.0, quality_balance: 0.0, coverage: 1.0 },
      weights: [0.25, 0.25, 0.25, 0.25],
      total_samples: 4,
    },
    gate: {
      verdict: 'warned',
      passed: true,
      failed_rules: [] as string[],
      warned_rules: ['completeness'],
      errored_rules: [] as string[],
      metrics: {
        completeness: 0.5,
        diversity: 1.0,
        quality_balance: 0.0,
        coverage: 1.0,
        duplicate_rate: 0.0,
      },
    },
    skipped_rules: ['pass_rate'],
  }
  // 同一份 dup.json，只把 duplicate_rate 上限放到 99.0 ⇒ 三档里的 passed
  const PASSED = {
    ...FAILED,
    gate: {
      ...FAILED.gate,
      verdict: 'passed',
      passed: true,
      failed_rules: [] as string[],
    },
  }
  // sparse.json + block_on_warning=true：verdict 变 failed，而 failed_rules 是**空的**
  const BLOCKED_BY_WARNING = {
    ...WARNED,
    gate: { ...WARNED.gate, verdict: 'failed', passed: false },
  }
  return {
    FAILED,
    WARNED,
    PASSED,
    BLOCKED_BY_WARNING,
    spies: {} as Record<string, ReturnType<typeof vi.fn>>,
  }
})

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  const s = fx.spies
  s.evaluateQuality = vi.fn().mockResolvedValue({
    total_samples: 4,
    passed_samples: 0,
    filtered_samples: 4,
    pass_rate: 0.0,
    issues: [],
  })
  s.healthGate = vi.fn().mockResolvedValue(fx.FAILED)
  return { ...actual, evaluateQuality: s.evaluateQuality, healthGate: s.healthGate }
})

// DataList 替身：一个按钮把 'dup.json' 选进父组件
vi.mock('../components/DataList', () => ({
  default: (props: { onChange: (v: string) => void }) => (
    <button type="button" data-testid="pick-file" onClick={() => props.onChange('dup.json')}>
      pick
    </button>
  ),
}))
vi.mock('../components/QualityChart', () => ({ default: () => <div data-testid="quality-chart" /> }))

import Quality from './Quality'

const { FAILED, WARNED, PASSED, BLOCKED_BY_WARNING, spies } = fx

afterEach(() => {
  cleanup()
  message.destroy()
  spies.evaluateQuality.mockClear()
  spies.healthGate.mockClear().mockResolvedValue(FAILED)
})

/** 按标题取某一格 Statistic 的值（本页十几格都在渲裸数字，按文本取会撞「多个匹配」） */
function statValue(container: HTMLElement, title: string): string {
  const box = Array.from(container.querySelectorAll('.ant-statistic')).find(
    (node) => node.querySelector('.ant-statistic-title')?.textContent === title
  )
  expect(box, `页面上没有「${title}」这一格`).toBeTruthy()
  return box!.querySelector('.ant-statistic-content-value')!.textContent!
}

/** 「规则名单」那张卡里某一行标签后面的名字（空名单渲染成「（无）」） */
function ruleRow(container: HTMLElement, label: string): string {
  const row = Array.from(container.querySelectorAll('.ant-list-item')).find((node) =>
    node.textContent?.includes(label)
  )
  expect(row, `名单里没有「${label}」这一行`).toBeTruthy()
  return row!.textContent!.replace(label, '')
}

/**
 * 「健康门禁」那颗按钮
 *
 * 按**正则**取而不是精确名：本页所有动作共用一个 `loading`，跑过任何一个动作之后，antd 会在
 * 按钮里留一只 `ant-btn-loading-icon-motion-leave` 的图标（自带 `aria-label="loading"`），
 * 而 jsdom 不放动画 ⇒ 那只图标永远不走，可及名就永久变成「loading健康门禁」（实测：
 * 精确名 1s 内一直取不到，正则恰好 1 个匹配）。按钮此时并没有 disabled，点得动。
 */
const gateButton = () => screen.getByRole('button', { name: /健康门禁/ })

/** 挂上页面、选好文件（`run` 守卫那一支不选，所以拆成两步） */
async function mount(pickFile = true) {
  const view = render(<Quality />)
  if (pickFile) fireEvent.click(screen.getByTestId('pick-file'))
  return view.container
}

/** 点门禁并等结果落地，然后切到那一个 Tab */
async function runAndOpenTab(container: HTMLElement) {
  fireEvent.click(gateButton())
  await waitFor(() => expect(spies.healthGate).toHaveBeenCalled())
  fireEvent.click(screen.getByRole('tab', { name: '健康门禁' }))
  await waitFor(() => expect(document.querySelector('[aria-label="健康门禁回执"]')).toBeTruthy())
  return container
}

describe('Quality 健康门禁 · 发之前', () => {
  it('没选数据集就点门禁得到一句 warning，不发请求', async () => {
    const warn = vi.spyOn(message, 'warning').mockImplementation(() => DUMMY_MSG)
    await mount(false)

    fireEvent.click(gateButton())

    await waitFor(() => expect(warn).toHaveBeenCalledWith('请先选择数据集'))
    expect(spies.healthGate).not.toHaveBeenCalled()
    warn.mockRestore()
  })

  it('默认档递的是契约默认那五个值，weights 与 passRate 是 undefined 而不是 null/0', async () => {
    const container = await mount()

    fireEvent.click(gateButton())

    await waitFor(() => expect(spies.healthGate).toHaveBeenCalled())
    expect(spies.healthGate.mock.calls[0][0]).toBe('dup.json')
    expect(spies.healthGate.mock.calls[0][1]).toEqual({
      textField: 'instruction',
      weights: undefined,
      passRate: undefined,
      passRateMin: 0.6,
      duplicateRateMax: 0.3,
      completenessMin: 0.8,
      blockOnWarning: false,
    })
    // undefined 与 null 在这一只端点上不是同一件事：后三个非 Optional 字段送 null 是 422
    expect(container.querySelector('[aria-label="健康门禁回执"]')).toBeNull()
  })

  it('一键喂通过率：评估回 0.0 也喂得进去（不是被当成「没填」丢掉）', async () => {
    await mount()
    expect(screen.getByRole('button', { name: /用上面那次评估的通过率填入/ })).toBeDisabled()

    fireEvent.click(screen.getByRole('button', { name: /质量评估/ }))
    await waitFor(() => expect(spies.evaluateQuality).toHaveBeenCalled())

    const filler = screen.getByRole('button', { name: /用上面那次评估的通过率填入/ })
    await waitFor(() => expect(filler).not.toBeDisabled())
    fireEvent.click(filler)
    // step=0.05 ⇒ antd 按两位小数显示，0 渲染成 '0.00'；要紧的是下面那句：递下去的是 0
    expect(screen.getByLabelText('门禁通过率')).toHaveProperty('value', '0.00')

    fireEvent.click(gateButton())
    await waitFor(() => expect(spies.healthGate).toHaveBeenCalled())
    expect(spies.healthGate.mock.calls[0][1].passRate).toBe(0)
  })

  it('权重只填一个 ⇒ 按钮禁用并说明原因；四个全填 ⇒ 按顺序递下去', async () => {
    await mount()

    fireEvent.change(screen.getByLabelText('健康分权重 完整性'), { target: { value: '0.5' } })
    await waitFor(() => expect(gateButton()).toBeDisabled())
    expect(document.body.textContent).toContain('权重已填 1 / 4 个')
    expect(document.body.textContent).toContain('个数必须是 4')

    fireEvent.change(screen.getByLabelText('健康分权重 多样性'), { target: { value: '0.2' } })
    fireEvent.change(screen.getByLabelText('健康分权重 质量均衡'), { target: { value: '0.2' } })
    fireEvent.change(screen.getByLabelText('健康分权重 覆盖率'), { target: { value: '0.1' } })
    await waitFor(() => expect(gateButton()).not.toBeDisabled())

    fireEvent.click(gateButton())
    await waitFor(() => expect(spies.healthGate).toHaveBeenCalled())
    expect(spies.healthGate.mock.calls[0][1].weights).toEqual([0.5, 0.2, 0.2, 0.1])
  })

  it('阈值与 warning 开关改了照改的发（页面不当二传手把默认值塞回去）', async () => {
    await mount()

    fireEvent.change(screen.getByLabelText('pass_rate 下限'), { target: { value: '0.9' } })
    fireEvent.change(screen.getByLabelText('duplicate_rate 上限'), { target: { value: '0.05' } })
    fireEvent.change(screen.getByLabelText('completeness 下限'), { target: { value: '0.95' } })
    fireEvent.click(screen.getByLabelText('warning 也阻断'))

    fireEvent.click(gateButton())
    await waitFor(() => expect(spies.healthGate).toHaveBeenCalled())
    expect(spies.healthGate.mock.calls[0][1]).toMatchObject({
      passRateMin: 0.9,
      duplicateRateMax: 0.05,
      completenessMin: 0.95,
      blockOnWarning: true,
    })
  })
})

describe('Quality 健康门禁 · 回执渲染', () => {
  it('failed 那一档：verdict、passed=false、健康分与等级照回显，skipped 里是 pass_rate', async () => {
    const container = await mount()
    await runAndOpenTab(container)

    expect(document.body.textContent).toContain('门禁判定：failed（passed = false）')
    expect(statValue(container, '健康分')).toBe('0.893')
    expect(
      statValue(container, '等级（≥0.7 healthy / ≥0.4 moderate / 否则 poor）')
    ).toBe('healthy')
    expect(statValue(container, '样本总数')).toBe('4')
    expect(
      statValue(container, '生效权重（完整性 / 多样性 / 质量均衡 / 覆盖率）')
    ).toBe('0.25 / 0.25 / 0.25 / 0.25')
    expect(ruleRow(container, '判负（error 级，阻断）')).toContain('duplicate_rate')
    expect(ruleRow(container, '没参与判决（skipped）')).toContain('pass_rate')
    expect(ruleRow(container, '算不出来（errored）')).toContain('（无）')
  })

  it('warned 那一档把 passed = true 明写出来（「放行」不等于「干净」）', async () => {
    spies.healthGate.mockResolvedValue(WARNED)
    const container = await mount()
    await runAndOpenTab(container)

    expect(document.body.textContent).toContain('门禁判定：warned（passed = true）')
    expect(document.body.textContent).toContain('带警告放行')
    expect(statValue(container, '健康分')).toBe('0.625')
    expect(
      statValue(container, '等级（≥0.7 healthy / ≥0.4 moderate / 否则 poor）')
    ).toBe('moderate')
    expect(ruleRow(container, '警告（warning 级）')).toContain('completeness')
    expect(ruleRow(container, '判负（error 级，阻断）')).toContain('（无）')
  })

  it('passed 那一档渲染成功色文案，三个判负名单都是（无）', async () => {
    spies.healthGate.mockResolvedValue(PASSED)
    const container = await mount()
    await runAndOpenTab(container)

    expect(document.body.textContent).toContain('门禁判定：passed（passed = true）')
    expect(document.body.textContent).toContain('放行：参与判决的规则全部满足')
    expect(ruleRow(container, '判负（error 级，阻断）')).toContain('（无）')
    expect(ruleRow(container, '警告（warning 级）')).toContain('（无）')
  })

  it('block_on_warning 那一档：verdict=failed 而 failed_rules 空，名字住在 warned 行', async () => {
    spies.healthGate.mockResolvedValue(BLOCKED_BY_WARNING)
    const container = await mount()
    await runAndOpenTab(container)

    expect(document.body.textContent).toContain('门禁判定：failed（passed = false）')
    expect(ruleRow(container, '判负（error 级，阻断）')).toContain('（无）')
    expect(ruleRow(container, '警告（warning 级）')).toContain('completeness')
    expect(document.body.textContent).toContain('failed_rules 是空的')
  })

  it('指标表把健康分那四项与门禁那一份并排列出（后者多 duplicate_rate）', async () => {
    const container = await mount()
    await runAndOpenTab(container)

    const rows = Array.from(
      document.querySelectorAll('[aria-label="健康门禁回执"] .ant-table-row')
    ).map((row) => Array.from(row.querySelectorAll('td')).map((td) => td.textContent))
    expect(rows).toHaveLength(9) // 健康分 4 项 + 门禁 5 项（多一个 duplicate_rate）
    expect(rows[0]).toEqual(['健康分', 'completeness', '1.0000'])
    expect(rows[4]).toEqual(['门禁', 'completeness', '1.0000'])
    expect(rows.some((row) => row[1] === 'duplicate_rate' && row[2] === '0.5000')).toBe(true)
    expect(container.querySelector('[aria-label="健康门禁回执"]')).toBeTruthy()
  })

  it('那几句实测脾气一句都不许被美化掉', async () => {
    const container = await mount()
    await runAndOpenTab(container)
    const text = document.querySelector('[aria-label="健康门禁回执"]')!.textContent!

    // 这一句住在上方那张卡的 Alert 里（不在回执区），所以按整页取
    expect(document.body.textContent).toContain('健康分本身不参与门禁判决')
    expect(text).toContain('没参与判决')
    expect(text).toContain('duplicate_rate 变成 1.0')
    expect(text).toContain('先怀疑字段名')
    expect(text).toContain('1.17')
    expect(text).toContain('超过 1')
    expect(text).toContain('0.99 与 1.01')
    expect(text).toContain('三个阈值没有范围闸')
    expect(text).toContain('恒空')
  })

  it('门禁失败时显示服务端那句判决原文（权重之和那一句）', async () => {
    const err = vi.spyOn(message, 'error').mockImplementation(() => DUMMY_MSG)
    spies.healthGate.mockRejectedValue({
      response: { status: 400, data: { detail: '权重之和必须为 1.0' } },
    })
    await mount()

    fireEvent.click(gateButton())

    await waitFor(() => expect(err).toHaveBeenCalledWith('权重之和必须为 1.0'))
    err.mockRestore()
  })

  it('空数据集那一句也照原样显示（400 而不是 500）', async () => {
    const err = vi.spyOn(message, 'error').mockImplementation(() => DUMMY_MSG)
    spies.healthGate.mockRejectedValue({
      response: { status: 400, data: { detail: '数据集为空，门禁无法给出有意义的判定' } },
    })
    await mount()

    fireEvent.click(gateButton())

    await waitFor(() =>
      expect(err).toHaveBeenCalledWith('数据集为空，门禁无法给出有意义的判定')
    )
    err.mockRestore()
  })
})
