/**
 * Analysis 页的渲染面（L111，A206）
 *
 * 数据分析页此前零组件测试。四条要点：① 选文件后自动 analyzeData 并渲染四张统计卡
 * （总量/平均长度/唯一词/去重移除）；② 未选文件时点清洗/基准得到 warning、不调服务层；
 * ③ 清洗成功后渲染四个计数标签；④ 基准成功后渲染指标表。
 *
 * ReactECharts 用替身（jsdom 里 echarts 无法真渲染，也不是本用例关心的东西）；
 * 服务层 `importOriginal` 部分替身（`apiErrorDetail` 留真身）。
 *
 * L225 起本页多了「语言画像」与「生成集对比评估」两张卡（A279 第一批接线），
 * 对应下面两个新 describe：画像的成功/失败两路，评估的必填闸/成功/失败三路。
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

vi.mock('echarts-for-react', () => ({ default: () => <div data-testid="echarts-stub" /> }))

const spies = vi.hoisted(() => ({
  getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
  analyzeData: null as unknown as ReturnType<typeof vi.fn>,
  cleanData: null as unknown as ReturnType<typeof vi.fn>,
  runBenchmark: null as unknown as ReturnType<typeof vi.fn>,
  visualizeData: null as unknown as ReturnType<typeof vi.fn>,
  languageProfile: null as unknown as ReturnType<typeof vi.fn>,
  evaluateDataset: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.getDataFiles = vi.fn().mockResolvedValue({ files: [{ name: 'seed.json' }] })
  spies.analyzeData = vi.fn()
  spies.cleanData = vi.fn()
  spies.runBenchmark = vi.fn()
  spies.visualizeData = vi.fn()
  spies.languageProfile = vi.fn()
  spies.evaluateDataset = vi.fn()
  return {
    ...actual,
    getDataFiles: spies.getDataFiles,
    analyzeData: spies.analyzeData,
    cleanData: spies.cleanData,
    runBenchmark: spies.runBenchmark,
    visualizeData: spies.visualizeData,
    languageProfile: spies.languageProfile,
    evaluateDataset: spies.evaluateDataset,
  }
})

import Analysis from './Analysis'

const ANALYSIS = {
  coverage_analysis: {
    total_items: 100,
    question_type_distribution: { how: 0.6, what: 0.4 },
    length_distribution: { short: 0.3, medium: 0.5, long: 0.2, avg_length: 18 },
    topic_distribution: { unique_words: 50, top_words: { 租房: 9, 押金: 5 }, coverage_score: 0.8 },
    complexity_distribution: { simple: 0.5, medium: 0.3, complex: 0.2, avg_complexity: 1.2 },
  },
  statistics: {
    total_items: 100,
    avg_length: 18.34,
    min_length: 3,
    max_length: 88,
    unique_words: 512,
    top_words: { 租房: 9 },
  },
  dedup_report: {
    original_count: 100,
    deduplicated_count: 93,
    removed_count: 7,
    removal_rate: 0.07,
    duplicate_groups_count: 3,
    avg_group_size: 2,
    threshold: 0.85,
  },
}

/** 选中文件的公共前奏：打开下拉、点第一个 option、等 analyzeData 渲染完 */
async function selectFileAndWait() {
  const { container } = render(<Analysis />)
  await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())
  fireEvent.mouseDown(container.querySelector('.ant-select-selector')!)
  await waitFor(() =>
    expect(document.querySelector('.ant-select-item-option')).toBeInTheDocument()
  )
  fireEvent.click(document.querySelector('.ant-select-item-option')!)
  await waitFor(() => expect(spies.analyzeData).toHaveBeenCalledWith('seed.json'))
  return container
}

/**
 * 在页面上第 index 个下拉里选中 label
 *
 * 不能沿用 `selectFileAndWait` 里那句全局 `document.querySelector('.ant-select-item-option')`：
 * 实测（jsdom + antd v5）关掉一只下拉时它带着 `ant-slide-up-leave` 仍留在 DOM 里，两份
 * 下拉的选项文案完全相同，全局查会点到那只正在关闭的、什么都没选中。本页现在有两个
 * Select（待分析文件 / 参照集），所以先用 `aria-expanded` 确认打开的是第 index 只
 * （两只 combobox 的 `id` 在 jsdom 下都是 `rc_select_TEST_OR_SSR`，靠 id 关联不了），
 * 再取 DOM 里最后创建的那份下拉（第二次打开的一定排在后面）。
 */
async function pickInNthSelect(container: HTMLElement, index: number, label: string) {
  fireEvent.mouseDown(container.querySelectorAll('.ant-select-selector')[index])
  await waitFor(() =>
    expect(container.querySelectorAll('[role=combobox]')[index]).toHaveAttribute(
      'aria-expanded',
      'true'
    )
  )
  const dropdown = () => document.querySelectorAll('.ant-select-dropdown')
  await waitFor(() => expect(dropdown().length).toBeGreaterThanOrEqual(1))
  const option = Array.from(dropdown()[dropdown().length - 1].querySelectorAll('.ant-select-item-option')).find(
    el => el.textContent === label
  )
  if (!option) throw new Error(`第 ${index} 个下拉里没有 ${label}`)
  fireEvent.click(option)
}

/**
 * 取页面上表头含 header 的那张表的数据行（按列文本）
 *
 * 本页有四张表（关键词 / 基准 / 画像 / 评估），全局 `.ant-table-row` 会串台，
 * 所以按表头文本定位。
 */
const rowsOfTable = (container: HTMLElement, header: string) => {
  const table = Array.from(container.querySelectorAll('.ant-table')).find(t =>
    t.querySelector('.ant-table-thead')?.textContent?.includes(header)
  )
  if (!table) throw new Error(`找不到表头含 ${header} 的表格`)
  return Array.from(table.querySelectorAll('.ant-table-tbody .ant-table-row')).map(row =>
    Array.from(row.querySelectorAll('td')).map(td => td.textContent)
  )
}

afterEach(() => {
  cleanup()
  message.destroy()
  vi.clearAllMocks()
})

describe('Analysis 选文件触发分析', () => {
  it('渲染四张统计卡的真实读数', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)

    const container = await selectFileAndWait()

    expect(await screen.findByText('总数据量')).toBeInTheDocument()
    // antd Statistic 会把数值拆进整数/小数多个 span，故按统计值节点整体读文本
    await waitFor(() =>
      expect(container.querySelectorAll('.ant-statistic-content-value').length).toBe(4)
    )
    const values = Array.from(container.querySelectorAll('.ant-statistic-content-value')).map(
      (el) => el.textContent
    )
    // 总量 100 / 平均长度 18.3 / 唯一词 512 / 去重移除 7
    expect(values).toEqual(['100', '18.3', '512', '7'])
  })
})

describe('Analysis 清洗与基准', () => {
  it('清洗成功后渲染四个计数标签', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.cleanData.mockResolvedValue({
      original_count: 100,
      cleaned_count: 95,
      dropped_count: 3,
      changed_count: 2,
      language_distribution: {},
      issues: { noise_removed: 1, format_normalized: 1, dropped: 3 },
    })

    await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行数据清洗/ }))

    expect(await screen.findByText('原始 100')).toBeInTheDocument()
    expect(screen.getByText('清洗后 95')).toBeInTheDocument()
    expect(screen.getByText('丢弃 3')).toBeInTheDocument()
    expect(screen.getByText('变更 2')).toBeInTheDocument()
  })

  it('基准成功后渲染指标表', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.runBenchmark.mockResolvedValue({
      sample_count: 50,
      metrics: { diversity: 0.7321, coverage: 0.9 },
      all_metrics: {},
      threshold: 0.6,
      timestamp: 't',
    })

    await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行质量基准/ }))

    await waitFor(() => expect(spies.runBenchmark).toHaveBeenCalledWith('seed.json'))
    expect(await screen.findByText('diversity')).toBeInTheDocument()
    // 数值 toFixed(4)
    expect(screen.getByText('0.7321')).toBeInTheDocument()
  })
})

describe('Analysis 可视化图表文件（L211 接线）', () => {
  it('点生成图表文件：visualizeData 收到所选文件，落盘路径渲成表格', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.visualizeData.mockResolvedValue({
      charts: { length_distribution: 'visualizations/length.png', topics: 'visualizations/topics.png' },
    })

    await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /生成图表文件/ }))

    await waitFor(() => expect(spies.visualizeData).toHaveBeenCalledWith('seed.json'))
    expect(await screen.findByText('length_distribution')).toBeInTheDocument()
    expect(screen.getByText('visualizations/topics.png')).toBeInTheDocument()
  })
})

describe('Analysis 语言画像（L225 接线）', () => {
  it('渲染总数/支持清单标签、支持外警示与按条数降序的分布表', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.languageProfile.mockResolvedValue({
      total_items: 120,
      // 故意让插入序与条数序相反：这样「按条数降序」这条断言才有牙齿
      // （缺陷注入 A4：把页面里的 .sort 摘掉，本用例必红）
      distribution: { en: 30, zh: 90 },
      ratios: { zh: 0.75, en: 0.25 },
      outside_supported: ['mixed', 'ja'],
      supported_langs: ['zh', 'en'],
      default_target_lang: 'en',
    })

    const container = await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行语言画像/ }))

    await waitFor(() => expect(spies.languageProfile).toHaveBeenCalledWith('seed.json'))
    expect(await screen.findByText('共 120 条')).toBeInTheDocument()
    expect(screen.getByText('支持 zh、en')).toBeInTheDocument()
    expect(screen.getByText('默认目标语 en')).toBeInTheDocument()
    // 落在清单外要出声，并说明 mixed 是检测标签不是语言码
    expect(screen.getByText(/落在支持清单之外：mixed、ja/)).toBeInTheDocument()
    expect(screen.getByText(/mixed 是检测标签而不是语言码/)).toBeInTheDocument()
    // 分布表：条数降序 ⇒ zh 行在前，占比是 ratios*100 保留一位
    expect(rowsOfTable(container, '检测标签')).toEqual([
      ['zh', '90', '75.0'],
      ['en', '30', '25.0'],
    ])
  })

  it('画像失败时专属文案出声，且不渲染标签与表', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.languageProfile.mockRejectedValue(new Error('boom'))

    await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行语言画像/ }))

    await waitFor(() => expect(document.body.textContent).toContain('语言画像失败'), {
      timeout: 4000,
    })
    expect(screen.queryByText('共 120 条')).not.toBeInTheDocument()
  })
})

describe('Analysis 生成集对比评估（L225 接线）', () => {
  it('只选了生成集时点评估得到双侧必填 warning，不调服务层', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)

    await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行批量评估/ }))

    await waitFor(() =>
      expect(document.body.textContent).toContain('评估需要同时选中生成集与参照集')
    )
    expect(spies.evaluateDataset).not.toHaveBeenCalled()
  })

  it('两侧都选中后评估：回执的样本数与指标表渲染，数值保留四位', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.evaluateDataset.mockResolvedValue({
      metrics: { rouge_l_f1: 0.73214, bleu: 0.5 },
      sample_count: 42,
      details: [],
    })

    const container = await selectFileAndWait()
    await pickInNthSelect(container, 1, 'seed.json')
    fireEvent.click(screen.getByRole('button', { name: /运行批量评估/ }))

    // 契约允许两侧同文件（同一份数据的两个字段做对照），故这里两次都是 seed.json
    await waitFor(() =>
      expect(spies.evaluateDataset).toHaveBeenCalledWith('seed.json', 'seed.json')
    )
    expect(await screen.findByText('对齐样本数 42')).toBeInTheDocument()
    expect(screen.getByText('0.7321')).toBeInTheDocument()
    expect(screen.getByText('0.5000')).toBeInTheDocument()
  })

  it('评估失败时专属文案出声，且不渲染指标表', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.evaluateDataset.mockRejectedValue(new Error('boom'))

    const container = await selectFileAndWait()
    await pickInNthSelect(container, 1, 'seed.json')
    fireEvent.click(screen.getByRole('button', { name: /运行批量评估/ }))

    await waitFor(() => expect(document.body.textContent).toContain('批量评估失败'), {
      timeout: 4000,
    })
    expect(screen.queryByText(/^对齐样本数/)).not.toBeInTheDocument()
  })
})
