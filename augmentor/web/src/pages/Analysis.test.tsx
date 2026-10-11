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
 * L226 起再加一张「内容检索」（A279 第二批接线），对应 `Analysis 内容检索` 那个
 * describe：闸/成功/换方法/无高亮/零命中/失败六路。
 * L227 起再加「语义相似检索」与「增强前后影响」（A279 第三批接线），对应最后两个
 * describe。这一批多了一条判据方向：页面**递出去的是本次生效值**（`top_k`、`text_key`、
 * `min_scale_gain` 都取自控件当前值而不是初值），回执里那些「改了结果就不同」的字段
 * （backend/dimension/collection、判决门槛）必须原样出声。
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
  searchDataset: null as unknown as ReturnType<typeof vi.fn>,
  semanticSearch: null as unknown as ReturnType<typeof vi.fn>,
  assessImpact: null as unknown as ReturnType<typeof vi.fn>,
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
  spies.searchDataset = vi.fn()
  spies.semanticSearch = vi.fn()
  spies.assessImpact = vi.fn()
  return {
    ...actual,
    getDataFiles: spies.getDataFiles,
    analyzeData: spies.analyzeData,
    cleanData: spies.cleanData,
    runBenchmark: spies.runBenchmark,
    visualizeData: spies.visualizeData,
    languageProfile: spies.languageProfile,
    evaluateDataset: spies.evaluateDataset,
    searchDataset: spies.searchDataset,
    semanticSearch: spies.semanticSearch,
    assessImpact: spies.assessImpact,
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
 * 全局 `.ant-table-row` 会串台（本页选中文件后已有 关键词 / 基准 / 画像 / 评估 / 检索高亮 /
 * 语义命中 / 增益 / 基线侧快照 / 增强后侧快照 多张表），所以按表头文本定位。
 */
const tablesByHeader = (container: HTMLElement, header: string) => {
  const tables = Array.from(container.querySelectorAll('.ant-table')).filter(t =>
    t.querySelector('.ant-table-thead')?.textContent?.includes(header)
  )
  if (tables.length === 0) throw new Error(`找不到表头含 ${header} 的表格`)
  return tables.map(table =>
    Array.from(table.querySelectorAll('.ant-table-tbody .ant-table-row')).map(row =>
      Array.from(row.querySelectorAll('td')).map(td => td.textContent)
    )
  )
}

const rowsOfTable = (container: HTMLElement, header: string) => tablesByHeader(container, header)[0]

/** 两个 `InputNumber` 的 input：DOM 序 0=语义检索「前 K 条」，1=影响评估「规模增益门槛」 */
const numberInputs = (container: HTMLElement) =>
  Array.from(container.querySelectorAll('.ant-input-number-input'))

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

/**
 * 内容检索（L226，A279 第二批接线）
 *
 * 钉四件事：① 递出去的是（文件, 检索词, {method}），method 来自下拉当前值而不是初值；
 * ② 命中总数与本次返回条数分开渲染（limit 截断时两个数不同，只报一个会把「还有没回传的」
 * 说成「就这么些」）；③ 高亮行是 `[前, 命中, 后]` 拼出来的，所以「命中片段」列与
 * 「上下文」列必须不同——两列相同就说明拼接写错了；④ regex 那一路实测
 * `total_matches>0` 而 `highlights=[]`（L226 探针量到），此时既不能说「没搜到」，
 * 也不该渲染一张空表。
 */
describe('Analysis 内容检索', () => {
  const SEARCH_HITS = {
    query: '向量',
    method: 'contains',
    total_matches: 12,
    query_time_ms: 3.5,
    items: [{ instruction: '请解释什么是向量数据库', output: '按相似度检索。' }],
    highlights: [
      {
        index: 0,
        highlights: {
          instruction: ['请解释什么是', '向量', '数据库'],
          output: ['', '向量', '向量按相似度检索。'],
        },
      },
      { index: 7, highlights: { output: ['上文', '向量', '下文'] } },
    ],
    fuzzy_threshold: null,
    ngram_n: null,
    applied_filters: null,
    matches_before_filters: null,
  }

  it('检索成功：递出方法，命中数与返回数分开，高亮表逐字段一行', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.searchDataset.mockResolvedValue(SEARCH_HITS)

    const container = await selectFileAndWait()
    fireEvent.change(container.querySelector('input[placeholder="检索词"]')!, {
      target: { value: '向量' },
    })
    fireEvent.click(screen.getByRole('button', { name: /运行检索/ }))

    await waitFor(() =>
      expect(spies.searchDataset).toHaveBeenCalledWith('seed.json', '向量', { method: 'contains' })
    )
    // 12 命中 vs 本次返回 1 条：两个数都必须在场
    expect(await screen.findByText('命中 12 条')).toBeInTheDocument()
    expect(screen.getByText('本次返回 1 条')).toBeInTheDocument()
    expect(screen.getByText('耗时 3.50 ms')).toBeInTheDocument()
    // 第 0 条在两个字段各命中一行、第 7 条一行 ⇒ 三行，且顺序与响应一致
    expect(rowsOfTable(container, '条目下标')).toEqual([
      ['0', 'instruction', '向量', '请解释什么是向量数据库'],
      ['0', 'output', '向量', '向量向量按相似度检索。'],
      ['7', 'output', '向量', '上文向量下文'],
    ])
  })

  it('换方法真的换：下拉选 regex 后递出去的是 regex', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.searchDataset.mockResolvedValue({ ...SEARCH_HITS, method: 'regex', highlights: [] })

    const container = await selectFileAndWait()
    fireEvent.change(container.querySelector('input[placeholder="检索词"]')!, {
      target: { value: '请.*数据库' },
    })
    // 本页第三个下拉是「检索方法」（0=待分析文件，1=参照集）
    await pickInNthSelect(container, 2, '正则')
    fireEvent.click(screen.getByRole('button', { name: /运行检索/ }))

    await waitFor(() =>
      expect(spies.searchDataset).toHaveBeenCalledWith('seed.json', '请.*数据库', {
        method: 'regex',
      })
    )
  })

  it('regex 有命中但无高亮：说不产出高亮，而不是说没命中', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.searchDataset.mockResolvedValue({ ...SEARCH_HITS, method: 'regex', highlights: [] })

    const container = await selectFileAndWait()
    fireEvent.change(container.querySelector('input[placeholder="检索词"]')!, {
      target: { value: '请.*数据库' },
    })
    fireEvent.click(screen.getByRole('button', { name: /运行检索/ }))

    expect(await screen.findByText(/不产出高亮片段/)).toBeInTheDocument()
    expect(screen.getByText('命中 12 条')).toBeInTheDocument()
    expect(screen.queryByText('没有命中')).not.toBeInTheDocument()
    // 不能查「页面上有没有 .ant-table」：本页本来就有关键词表。查的是高亮表自己的表头。
    expect(screen.queryByText('条目下标')).not.toBeInTheDocument()
  })

  it('零命中才说「没有命中」', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.searchDataset.mockResolvedValue({
      ...SEARCH_HITS,
      total_matches: 0,
      items: [],
      highlights: [],
    })

    const container = await selectFileAndWait()
    fireEvent.change(container.querySelector('input[placeholder="检索词"]')!, {
      target: { value: '不存在的词' },
    })
    fireEvent.click(screen.getByRole('button', { name: /运行检索/ }))

    expect(await screen.findByText('没有命中')).toBeInTheDocument()
    expect(screen.getByText('命中 0 条')).toBeInTheDocument()
  })

  it('空检索词不发请求', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)

    await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行检索/ }))

    await waitFor(() => expect(document.body.textContent).toContain('请输入检索词'))
    expect(spies.searchDataset).not.toHaveBeenCalled()
  })

  it('检索失败时专属文案出声，且不渲染命中读数', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.searchDataset.mockRejectedValue(new Error('boom'))

    const container = await selectFileAndWait()
    fireEvent.change(container.querySelector('input[placeholder="检索词"]')!, {
      target: { value: '向量' },
    })
    fireEvent.click(screen.getByRole('button', { name: /运行检索/ }))

    await waitFor(() => expect(document.body.textContent).toContain('内容检索失败'), {
      timeout: 4000,
    })
    expect(screen.queryByText(/^命中 \d+ 条$/)).not.toBeInTheDocument()
  })
})

/**
 * 语义相似检索（L227，A279 第三批接线）
 *
 * 钉四件事：① 递出去的是**本次生效值**（`topK` / `textKey` 取自控件当前值，不是初值）；
 * ② 三个「本次生效值」回显标签（backend / dimension / collection）在**任何**有回执的路径上
 * 都必须在场，包括 `top_k=0` 那种「只要回显、不要命中」的路径——那一档存在的理由就是核对
 * 「配了什么、算了什么」，把它渲染成空表等于把这条能力删了；
 * ③ 相似度是相对排序量，按四位原样给，不折算成百分比；
 * ④ 空查询必须拦在门口：实测 `query=""` 照样回条目且 `score` 全 0，那是一份退化读数。
 */
describe('Analysis 语义相似检索', () => {
  const SEMANTIC = {
    query: '租房押金',
    total: 2,
    backend: 'faiss',
    dimension: 384,
    collection: 'default',
    results: [
      // 一条长小数（验 toFixed(4) 的截断），一条整 1（验它渲染成 1.0000 而不是「1」或「100%」）
      { index: 3, score: 0.8123456, text: '租房合同要写明押金。', item: { instruction: '押金怎么退' } },
      { index: 0, score: 1, text: '合同里的押金条款', item: { instruction: '租房合同' } },
    ],
  }

  it('按默认档位检索：递出 topK=5·textKey=instruction，回显标签与命中表都出声', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.semanticSearch.mockResolvedValue(SEMANTIC)

    const container = await selectFileAndWait()
    fireEvent.change(container.querySelector('input[placeholder="查询文本"]')!, {
      target: { value: '租房押金' },
    })
    fireEvent.click(screen.getByRole('button', { name: /运行语义检索/ }))

    await waitFor(() =>
      expect(spies.semanticSearch).toHaveBeenCalledWith('seed.json', '租房押金', {
        topK: 5,
        textKey: 'instruction',
      })
    )
    expect(await screen.findByText('命中 2 条')).toBeInTheDocument()
    expect(screen.getByText('后端 faiss')).toBeInTheDocument()
    expect(screen.getByText('维度 384')).toBeInTheDocument()
    expect(screen.getByText('集合 default')).toBeInTheDocument()
    // 顺序跟随响应而不是按分数重排：排名是后端给的，页面再排一次就是把两份口径叠在一起
    expect(rowsOfTable(container, '相似度')).toEqual([
      ['3', '0.8123', '租房合同要写明押金。'],
      ['0', '1.0000', '合同里的押金条款'],
    ])
  })

  it('改前 K 条与编码字段：递出去的是改后的那两个值', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.semanticSearch.mockResolvedValue({ ...SEMANTIC, total: 1, results: [SEMANTIC.results[0]] })

    const container = await selectFileAndWait()
    fireEvent.change(container.querySelector('input[placeholder="查询文本"]')!, {
      target: { value: '押金' },
    })
    fireEvent.change(numberInputs(container)[0]!, { target: { value: '2' } })
    // 本页第四个下拉是「编码字段」（0=待分析文件，1=参照集，2=检索方法）
    await pickInNthSelect(container, 3, '编码 output')
    fireEvent.click(screen.getByRole('button', { name: /运行语义检索/ }))

    await waitFor(() =>
      expect(spies.semanticSearch).toHaveBeenCalledWith('seed.json', '押金', {
        topK: 2,
        textKey: 'output',
      })
    )
  })

  it('前 K 条取 0：没有命中，但三个生效值照旧回显（这一档的存在理由就是核对配置）', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.semanticSearch.mockResolvedValue({ ...SEMANTIC, total: 0, results: [] })

    const container = await selectFileAndWait()
    fireEvent.change(container.querySelector('input[placeholder="查询文本"]')!, {
      target: { value: '租房押金' },
    })
    fireEvent.change(numberInputs(container)[0]!, { target: { value: '0' } })
    fireEvent.click(screen.getByRole('button', { name: /运行语义检索/ }))

    await waitFor(() =>
      expect(spies.semanticSearch).toHaveBeenCalledWith('seed.json', '租房押金', {
        topK: 0,
        textKey: 'instruction',
      })
    )
    expect(await screen.findByText(/没有命中/)).toBeInTheDocument()
    expect(screen.getByText('后端 faiss')).toBeInTheDocument()
    expect(screen.getByText('维度 384')).toBeInTheDocument()
    expect(screen.getByText('命中 0 条')).toBeInTheDocument()
    // 命中表不该在零命中时渲染出来（本页另有表，查的是语义表自己的表头）
    expect(screen.queryByText('相似度')).not.toBeInTheDocument()
  })

  it('空查询不发请求', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)

    await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行语义检索/ }))

    await waitFor(() => expect(document.body.textContent).toContain('请输入查询文本'))
    expect(spies.semanticSearch).not.toHaveBeenCalled()
  })

  it('检索失败时专属文案出声，且不渲染回显标签', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.semanticSearch.mockRejectedValue(new Error('boom'))

    const container = await selectFileAndWait()
    fireEvent.change(container.querySelector('input[placeholder="查询文本"]')!, {
      target: { value: '租房押金' },
    })
    fireEvent.click(screen.getByRole('button', { name: /运行语义检索/ }))

    await waitFor(() => expect(document.body.textContent).toContain('语义检索失败'), {
      timeout: 4000,
    })
    expect(screen.queryByText('后端 faiss')).not.toBeInTheDocument()
  })
})

/**
 * 增强前后影响（L227，A279 第三批接线）
 *
 * 钉四件事：① 双侧必填闸；② 四项增益**按项**渲染并带量纲，遇到响应里新增的键要落
 * 「未知口径」而不是被 `GAIN_PRESENTATION` 的清单吃掉（后端加一项增益时页面不许静默丢数据）；
 * ③ 两侧快照必须读出**不同的数**（把 `after` 误写成 `impact.before` 是这类双列布局最常见的错），
 * 且 `extra` 里出现的新键不进表；④ 判决标签旁边要说清「只看两项」并把**本次门槛**报出来，
 * 因为门槛现在是用户可调的控件。
 */
describe('Analysis 增强前后影响', () => {
  const IMPACT = {
    before: {
      total_items: 4,
      unique_instructions: 4,
      avg_length: 12.5,
      length_std: 2.48746859276655,
      duplicate_rate: 0,
      extra: {},
    },
    after: {
      total_items: 8,
      unique_instructions: 6,
      avg_length: 14.25,
      length_std: 3.1,
      duplicate_rate: 0.05,
      // 故意给非空：渲染清单里没有 extra，这一档多出来的键必须不落地
      extra: { novel_metric: 9 },
    },
    gains: {
      scale_gain: 1.0,
      diversity_gain: 0.5,
      dedup_gain: -0.06,
      length_spread_gain: 1.29128784747792,
      // 清单外的键：页面必须照报（label 退回键名），而不是消失
      novelty_gain: 0.5,
    },
    beneficial: true,
  }

  it('只选了基线时点评估得到双侧必填 warning，不调服务层', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)

    await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行影响评估/ }))

    await waitFor(() =>
      expect(document.body.textContent).toContain('影响评估需要同时选中基线集与增强后数据集')
    )
    expect(spies.assessImpact).not.toHaveBeenCalled()
  })

  it('两侧齐备：增益逐项带量纲，未知键不丢，两侧快照读出不同的数', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.assessImpact.mockResolvedValue(IMPACT)

    const container = await selectFileAndWait()
    await pickInNthSelect(container, 4, 'seed.json')
    fireEvent.click(screen.getByRole('button', { name: /运行影响评估/ }))

    await waitFor(() =>
      expect(spies.assessImpact).toHaveBeenCalledWith('seed.json', 'seed.json', {
        textField: 'instruction',
        minScaleGain: 0,
      })
    )
    expect(await screen.findByText('判为有增益')).toBeInTheDocument()
    expect(rowsOfTable(container, '增益项')).toEqual([
      ['规模增益', 'scale_gain', '比值', '1.0000'],
      ['多样性增益', 'diversity_gain', '比值（按所选字段的唯一值）', '0.5000'],
      ['去重增益', 'dedup_gain', '绝对差，正=重复率下降', '-0.0600'],
      ['长度离散增益', 'length_spread_gain', '绝对差（标准差）', '1.2913'],
      ['novelty_gain', 'novelty_gain', '未知口径', '0.5000'],
    ])
    // 表头含「指标」的表在本页恰好两张（基线侧 / 增强后侧）；先坐实数量，再比内容，
    // 否则「两张其实渲染了同一侧」这种错会被只查第一张的写法放过去。
    const snapshots = tablesByHeader(container, '指标')
    expect(snapshots.length).toBe(2)
    expect(snapshots[0]).toEqual([
      ['条目数', '4'],
      ['唯一值数（按所选字段）', '4'],
      ['平均长度', '12.5000'],
      ['长度标准差', '2.4875'],
      ['重复率', '0.0000'],
    ])
    expect(snapshots[1]).toEqual([
      ['条目数', '8'],
      ['唯一值数（按所选字段）', '6'],
      ['平均长度', '14.2500'],
      ['长度标准差', '3.1000'],
      ['重复率', '0.0500'],
    ])
    // extra 里的新键不进表；计数渲染成 4 而不是 4.0000（假精度）
    expect(screen.queryByText('novel_metric')).not.toBeInTheDocument()
    expect(screen.queryByText('4.0000')).not.toBeInTheDocument()
  })

  it('门槛抬到 0.5 且换看 output：递出生效值，判为无增益时把本次门槛一起说清', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.assessImpact.mockResolvedValue({ ...IMPACT, beneficial: false })

    const container = await selectFileAndWait()
    await pickInNthSelect(container, 4, 'seed.json')
    await pickInNthSelect(container, 5, '看 output')
    fireEvent.change(numberInputs(container)[1]!, { target: { value: '0.5' } })
    fireEvent.click(screen.getByRole('button', { name: /运行影响评估/ }))

    await waitFor(() =>
      expect(spies.assessImpact).toHaveBeenCalledWith('seed.json', 'seed.json', {
        textField: 'output',
        minScaleGain: 0.5,
      })
    )
    expect(await screen.findByText('判为无增益')).toBeInTheDocument()
    // 门槛现在是可调的，所以口径文案必须跟着报本次值而不是出厂默认
    expect(screen.getByText(/本次 0\.5/)).toBeInTheDocument()
  })

  it('评估失败时专属文案出声，且不渲染增益表', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.assessImpact.mockRejectedValue(new Error('boom'))

    const container = await selectFileAndWait()
    await pickInNthSelect(container, 4, 'seed.json')
    fireEvent.click(screen.getByRole('button', { name: /运行影响评估/ }))

    await waitFor(() => expect(document.body.textContent).toContain('影响评估失败'), {
      timeout: 4000,
    })
    expect(screen.queryByText('判为有增益')).not.toBeInTheDocument()
    expect(screen.queryByText('规模增益')).not.toBeInTheDocument()
  })
})
