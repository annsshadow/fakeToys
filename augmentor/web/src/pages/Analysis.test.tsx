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
 * L228 起再加「字段级统计」「特征普查」「双集对比」（A279 第四批接线），对应末尾三个
 * describe。这一批的新判据是**候选清单只能来自回执**：实测 `fields` 给一个数据集里没有的
 * 名字照样回 200 并填整套全零，`key_fields` 给一个两侧都没有的名字更糟——两份毫不相干的
 * 数据会回 `common_items: 1`、`similarity_score: 1.0`（幽灵匹配键把「毫无重叠」谎报成
 * 「完全重叠」），所以页面不写死字段名，只在回执给出的键里让用户勾。
 */
import { afterEach, describe, expect, it, vi } from 'vitest'
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { message } from 'antd'

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

vi.mock('echarts-for-react/lib/core', () => ({ default: () => <div data-testid="echarts-stub" /> }))

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
  datasetStats: null as unknown as ReturnType<typeof vi.fn>,
  detectFeatures: null as unknown as ReturnType<typeof vi.fn>,
  compareDatasets: null as unknown as ReturnType<typeof vi.fn>,
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
  spies.datasetStats = vi.fn()
  spies.detectFeatures = vi.fn()
  spies.compareDatasets = vi.fn()
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
    datasetStats: spies.datasetStats,
    detectFeatures: spies.detectFeatures,
    compareDatasets: spies.compareDatasets,
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
  fireEvent.mouseDown(container.querySelector('.ant-select')!)
  await waitFor(() => expect(document.querySelector('.ant-select-item-option')).toBeInTheDocument())
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
  fireEvent.mouseDown(container.querySelectorAll('.ant-select')[index])
  await waitFor(() =>
    expect(container.querySelectorAll('[role=combobox]')[index]).toHaveAttribute(
      'aria-expanded',
      'true',
    ),
  )
  const dropdown = () => document.querySelectorAll('.ant-select-dropdown')
  await waitFor(() => expect(dropdown().length).toBeGreaterThanOrEqual(1))
  const option = Array.from(
    dropdown()[dropdown().length - 1].querySelectorAll('.ant-select-item-option'),
  ).find((el) => el.textContent === label)
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
  const tables = Array.from(container.querySelectorAll('.ant-table')).filter((t) =>
    t.querySelector('.ant-table-thead')?.textContent?.includes(header),
  )
  if (tables.length === 0) throw new Error(`找不到表头含 ${header} 的表格`)
  return tables.map((table) =>
    dataRowsOf(table).map((row) =>
      Array.from(row.querySelectorAll('td')).map((td) => td.textContent),
    ),
  )
}

/**
 * 一张表里的数据行，排除「展开行里内嵌那张表」的行
 *
 * 字段对照表与字段统计表都会展开内嵌表（逐条值差异 / 高频取值），内嵌表的行同样带
 * `.ant-table-row`，不排除会把明细行混进主表行数里。
 */
const dataRowsOf = (table: Element) =>
  Array.from(table.querySelectorAll('.ant-table-tbody .ant-table-row')).filter(
    (row) => !row.closest('.ant-table-expanded-row'),
  )

const rowsOfTable = (container: HTMLElement, header: string) => tablesByHeader(container, header)[0]

/**
 * 取「表格标题（`title` 插槽）含 fragment」的那张表的数据行
 *
 * 本页现在有三张表头完全相同的对称表（去重面 / 长度面 / 词汇面，都是 指标 / A 侧 / B 侧），
 * 按表头定位会三张混成一张；它们的区分量在标题上，所以按标题定位。
 */
const rowsOfTitledTable = (container: HTMLElement, fragment: string) => {
  const table = Array.from(container.querySelectorAll('.ant-table')).find((t) =>
    t.querySelector('.ant-table-title')?.textContent?.includes(fragment),
  )
  if (!table) throw new Error(`找不到标题含 ${fragment} 的表格`)
  return dataRowsOf(table).map((row) =>
    Array.from(row.querySelectorAll('td')).map((td) => td.textContent),
  )
}

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
      expect(container.querySelectorAll('.ant-statistic-content-value').length).toBe(4),
    )
    const values = Array.from(container.querySelectorAll('.ant-statistic-content-value')).map(
      (el) => el.textContent,
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
      charts: {
        length_distribution: 'visualizations/length.png',
        topics: 'visualizations/topics.png',
      },
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
      timeout: 10000,
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
      expect(document.body.textContent).toContain('评估需要同时选中生成集与参照集'),
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
      expect(spies.evaluateDataset).toHaveBeenCalledWith('seed.json', 'seed.json'),
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
      timeout: 10000,
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
      expect(spies.searchDataset).toHaveBeenCalledWith('seed.json', '向量', { method: 'contains' }),
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
      }),
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
      timeout: 10000,
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
      {
        index: 3,
        score: 0.8123456,
        text: '租房合同要写明押金。',
        item: { instruction: '押金怎么退' },
      },
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
      }),
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
    spies.semanticSearch.mockResolvedValue({
      ...SEMANTIC,
      total: 1,
      results: [SEMANTIC.results[0]],
    })

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
      }),
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
      }),
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
      timeout: 10000,
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
      expect(document.body.textContent).toContain('影响评估需要同时选中基线集与增强后数据集'),
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
      }),
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
      }),
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
      timeout: 10000,
    })
    expect(screen.queryByText('判为有增益')).not.toBeInTheDocument()
    expect(screen.queryByText('规模增益')).not.toBeInTheDocument()
  })
})

/**
 * 字段级统计（L228 接线）
 *
 * 断言强度按实测回执来：`nope` 那一整行全零是**端点真的会发**的形状（幽灵字段），
 * `novelty` 是清单外的键，两条都必须落地而不是消失。
 */
describe('Analysis 字段级统计（L228 接线）', () => {
  const STATS = {
    dataset_name: 'p',
    total_items: 4,
    summary: "数据集 'p' 共 4 条数据\n包含 2 个字段",
    field_statistics: {
      instruction: {
        field_name: 'instruction',
        total_count: 4,
        filled_count: 4,
        empty_count: 0,
        fill_rate: 1.0,
        avg_length: 8.0,
        min_length: 5,
        max_length: 11,
        median_length: 8.0,
        std_deviation: 2.23606797749979,
        unique_count: 4,
        top_values: [['正则怎么用', 1]],
      },
      nope: {
        field_name: 'nope',
        total_count: 4,
        filled_count: 0,
        empty_count: 4,
        fill_rate: 0.0,
        avg_length: 0,
        min_length: 0,
        max_length: 0,
        median_length: 0,
        std_deviation: 0,
        unique_count: 0,
        top_values: [],
      },
    },
    content_statistics: {
      text_length: { avg: 8.0, min: 5, max: 11, total: 32 },
      vocabulary: { total_words: 28, unique_words: 28, top_words: [['正则怎么用', 1]] },
    },
    quality_metrics: { completeness: 1.0, diversity: 1.0, consistency: 1.0, novelty: 0.5 },
  }

  it('分析成功之前三张新卡根本不出现，所以门口那道必填闸在这三张卡上是够不到的', async () => {
    render(<Analysis />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())

    // 三张卡都住在 `{analysis && …}` 里面（沿用本页既有卡片的摆法），未选文件 ⇒ 没有
    // analysis ⇒ 连按钮都没有。因此 handler 里那句 `!selectedFile` 与 handleClean 的同款
    // 守卫一样是「与兄弟卡片对齐」的写法，而不是本页可达的用户路径；双集对比的必填闸
    // 才是真 reachable 的那一道（选了 A 侧没选 B 侧），在下面的对比 describe 里断。
    expect(screen.queryByRole('button', { name: /运行字段统计/ })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /运行特征普查/ })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /运行双集对比/ })).not.toBeInTheDocument()
    expect(spies.datasetStats).not.toHaveBeenCalled()
    expect(spies.detectFeatures).not.toHaveBeenCalled()
    expect(spies.compareDatasets).not.toHaveBeenCalled()
  })

  it('默认档位一个字段都不发：幽灵字段整行照渲，质量三口径带口径列，未知键不丢', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.datasetStats.mockResolvedValue(STATS)

    const container = await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行字段统计/ }))

    // 一个都没勾 ⇒ `fields` 这一档以 undefined 递出去（服务层再把它从 JSON 里丢掉），
    // 页面不替端点写第二套默认字段清单。
    await waitFor(() =>
      expect(spies.datasetStats).toHaveBeenCalledWith('seed.json', { fields: undefined }),
    )
    expect(await screen.findByText(/数据集 'p' 共 4 条数据/)).toBeInTheDocument()
    // 可展开的表在第一列前面多一个空的展开图标格，行首那个 '' 就是它
    expect(rowsOfTable(container, '填充率')).toEqual([
      ['', 'instruction', '4', '4 / 0', '100.00%', '8', '8', '5 – 11', '2.2361', '4'],
      // 幽灵字段：端点回的是整套全零，页面不把它摘掉也不补 0
      ['', 'nope', '4', '0 / 4', '0.00%', '0', '0', '0 – 0', '0', '0'],
    ])
    expect(rowsOfTable(container, '质量指标')).toEqual([
      [
        '完整性',
        'completeness',
        '（instruction 非空数 + output 非空数）÷（2 × 条目数），与 fields 无关',
        '100.00%',
      ],
      [
        '多样性',
        'diversity',
        '唯一 instruction 数 ÷ instruction 非空条数，与 fields 无关',
        '100.00%',
      ],
      [
        '一致性',
        'consistency',
        'instruction 与 output 都非空且互不相等的条数 ÷ 条目数，与 fields 无关',
        '100.00%',
      ],
      ['novelty', 'novelty', '未知口径', '50.00%'],
    ])
    expect(rowsOfTitledTable(container, '文本长度面')).toEqual([
      ['平均长度', '8'],
      ['最短长度', '5'],
      ['最长长度', '11'],
      ['长度合计', '32'],
    ])
    expect(screen.getByText('词数合计 28')).toBeInTheDocument()
    expect(rowsOfTitledTable(container, '高频词')).toEqual([['正则怎么用', '1']])
  })

  it('候选清单来自回执：勾定 output 后递出去的是勾选值', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.datasetStats.mockResolvedValue(STATS)

    const container = await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行字段统计/ }))
    await waitFor(() => expect(spies.datasetStats).toHaveBeenCalledTimes(1))

    // 第 6 只下拉＝限定字段；它的选项在第一次回执之前是空的（清单只能来自回执）
    await pickInNthSelect(container, 6, 'instruction')
    fireEvent.click(screen.getByRole('button', { name: /运行字段统计/ }))

    await waitFor(() =>
      expect(spies.datasetStats).toHaveBeenLastCalledWith('seed.json', {
        fields: ['instruction'],
      }),
    )
  })

  it('空数据集：三个字典全空时按「无内容」渲染而不补 0', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.datasetStats.mockResolvedValue({
      dataset_name: 'e',
      total_items: 0,
      summary: "数据集 'e' 共 0 条数据\n包含 0 个字段",
      field_statistics: {},
      content_statistics: {},
      quality_metrics: {},
    })

    const container = await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行字段统计/ }))

    expect(await screen.findByText(/空数据集：实测三个统计字典全为/)).toBeInTheDocument()
    expect(screen.getByText('回执没有质量指标（空数据集实测如此）')).toBeInTheDocument()
    // 字段表还在（antd 会渲一张带表头的空表 + 「暂无数据」），但零行；文本长度面整张不出现
    // （`content_statistics` 实测是 `{}`，两个子键一个都没有）——这两种「空」的形状不同，
    // 分开断言才不会把「空表」和「没表」混为一谈。页面顶部本来就有一张「平均长度」统计卡，
    // 所以这里只能按容器内容判，不能全局查词。
    expect(rowsOfTable(container, '填充率')).toEqual([])
    expect(container.textContent).not.toContain('文本长度面')
  })

  it('统计失败时专属文案出声，且不渲染字段表', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.datasetStats.mockRejectedValue(new Error('boom'))

    await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行字段统计/ }))

    await waitFor(() => expect(document.body.textContent).toContain('字段统计失败'), {
      timeout: 10000,
    })
    expect(screen.queryByText('填充率')).not.toBeInTheDocument()
  })
})

describe('Analysis 特征普查（L228 接线）', () => {
  const FEATURES = {
    total_items: 4,
    // 故意让插入序与覆盖率序相反：这样「按覆盖率升序」这条断言才有牙齿
    field_features: [
      {
        name: 'instruction',
        feature_type: 'text',
        coverage: 1.0,
        unique_ratio: 1.0,
        description: '常规特征',
      },
      {
        name: 'output',
        feature_type: 'numeric',
        coverage: 0.75,
        unique_ratio: 0.5,
        description: '常规特征',
      },
      {
        name: 'input',
        feature_type: 'unknown',
        coverage: 0.0,
        unique_ratio: 0.0,
        description: '稀疏特征',
      },
      // 清单外的类型值：页面必须原样报而不编一个中文名
      {
        name: 'novel',
        feature_type: 'quantum',
        coverage: 0.5,
        unique_ratio: 1.0,
        description: '常规特征',
      },
    ],
    sparse_fields: ['input'],
    intent_distribution: { what: 2, how: 1 },
  }

  it('按覆盖率升序渲五列，稀疏清单与意图条数各自出声', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.detectFeatures.mockResolvedValue(FEATURES)

    const container = await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行特征普查/ }))

    await waitFor(() => expect(spies.detectFeatures).toHaveBeenCalledWith('seed.json'))
    expect(rowsOfTable(container, '推断类型')).toEqual([
      ['input', '未识别', '0.00%', '0.00%', '稀疏特征'],
      ['novel', 'quantum', '50.00%', '100.00%', '常规特征'],
      ['output', '数值', '75.00%', '50.00%', '常规特征'],
      ['instruction', '文本', '100.00%', '100.00%', '常规特征'],
    ])
    expect(screen.getByText('稀疏字段：')).toBeInTheDocument()
    // 意图这一档是条数而不是占比，页面得把两副口径的关系说清
    expect(rowsOfTitledTable(container, '意图条数')).toEqual([
      ['what', '2'],
      ['how', '1'],
    ])
  })

  it('无稀疏字段与空意图分布各自说清，不重述阈值', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.detectFeatures.mockResolvedValue({
      total_items: 2,
      field_features: [
        {
          name: 'instruction',
          feature_type: 'text',
          coverage: 1.0,
          unique_ratio: 0.5,
          description: '常规特征',
        },
      ],
      sparse_fields: [],
      intent_distribution: {},
    })

    const container = await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行特征普查/ }))

    await waitFor(() => expect(spies.detectFeatures).toHaveBeenCalled())
    expect(screen.getByText('无稀疏字段')).toBeInTheDocument()
    expect(screen.getByText(/意图分布为空/)).toBeInTheDocument()
    expect(container.textContent).not.toContain('稀疏字段：')
  })

  it('普查失败时专属文案出声，且不渲染特征表', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.detectFeatures.mockRejectedValue(new Error('boom'))

    await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行特征普查/ }))

    await waitFor(() => expect(document.body.textContent).toContain('特征普查失败'), {
      timeout: 10000,
    })
    expect(screen.queryByText('推断类型')).not.toBeInTheDocument()
  })
})

/**
 * 双集对比（L228 接线）
 *
 * 三个死面/形状坑都在这里坐实：① `similarity_to_a` / `similarity_to_b` 全仓无赋值点、
 * 恒为 0.0（账本 A287），页面不渲它们；② `size_ratio` 在 B 侧 0 条时线上是 `null`
 * （A288），必须先接住 null；③ `winner` 有三态，`tie` 不是「没有胜者」而是判据本身。
 */
describe('Analysis 双集对比（L228 接线）', () => {
  const COMPARE = {
    quality_comparison: {
      dataset_a_name: 'p',
      dataset_b_name: 'q',
      dataset_a_count: 4,
      dataset_b_count: 5,
      quality_a: {
        avg_score: 0.6450461160073786,
        pass_rate: 1.0,
        min_score: 0.6195121951219512,
        max_score: 0.68,
      },
      quality_b: { avg_score: 0.5760368928059029, pass_rate: 0.8, min_score: 0.3, max_score: 0.68 },
      quality_diff: {
        avg_score: -0.06900922320147573,
        pass_rate: -0.2,
        min_score: -0.3195121951219512,
        max_score: 0.0,
      },
      dedup_a: { duplication_rate: 0.0, unique_count: 4 },
      dedup_b: { duplication_rate: 0.2, unique_count: 4 },
      length_a: { avg: 8.0, min: 5, max: 11, std: 2.23606797749979 },
      length_b: { avg: 8.6, min: 5, max: 11, std: 2.33238075793812 },
      vocabulary_a: { unique_chars: 28, total_chars: 32 },
      vocabulary_b: { unique_chars: 28, total_chars: 43 },
      similarity_to_a: 0.0,
      similarity_to_b: 0.0,
      winner: 'a',
      summary: '## 数据集对比: p vs q\n- 获胜者: A',
    },
    overlap_comparison: {
      dataset_a_name: 'p',
      dataset_b_name: 'q',
      metrics: {
        size_a: 4,
        size_b: 5,
        size_diff: -1,
        size_ratio: 0.8,
        common_items: 4,
        unique_a: 0,
        unique_b: 0,
        similarity_score: 1.0,
        field_overlap: 1.0,
      },
      field_comparisons: {
        instruction: {
          field_name: 'instruction',
          in_a_only: 0,
          in_b_only: 0,
          in_both: 4,
          type_mismatches: 0,
          value_differences: [],
        },
        output: {
          field_name: 'output',
          in_a_only: 0,
          in_b_only: 0,
          in_both: 4,
          type_mismatches: 0,
          // 明细条目实测固定三键，第一个永远是 instruction：匹配键不影响这一张表
          value_differences: [{ instruction: '正则怎么用', value_a: '甲', value_b: '乙' }],
        },
      },
      summary: '## 重叠度\n  相似度: 100.00%',
      recommendations: ['数据集相似度较高，可考虑合并去重'],
    },
  }

  it('只选了 A 侧时点对比得到双侧必填 warning，不调服务层', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)

    await selectFileAndWait()
    fireEvent.click(screen.getByRole('button', { name: /运行双集对比/ }))

    await waitFor(() =>
      expect(document.body.textContent).toContain('双集对比需要同时选中 A 侧与 B 侧'),
    )
    expect(spies.compareDatasets).not.toHaveBeenCalled()
  })

  it('默认不发匹配键：三态 winner、分数与率分档、九项指标都出声，恒 0 的死面不落 DOM', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.compareDatasets.mockResolvedValue(COMPARE)

    const container = await selectFileAndWait()
    await pickInNthSelect(container, 7, 'seed.json')
    fireEvent.click(screen.getByRole('button', { name: /运行双集对比/ }))

    await waitFor(() =>
      expect(spies.compareDatasets).toHaveBeenCalledWith('seed.json', 'seed.json', {
        keyFields: undefined,
      }),
    )
    expect(await screen.findByText('A 侧平均质量分占优')).toBeInTheDocument()
    expect(
      screen.getByText('本次匹配键：instruction（请求未发 key_fields，取 SDK 默认）'),
    ).toBeInTheDocument()
    expect(rowsOfTable(container, '质量面指标')).toEqual([
      ['平均质量分', 'avg_score', '单条 total_score 的均值', '0.6450', '0.5760', '-0.0690'],
      [
        '通过率',
        'pass_rate',
        '达阈值（SDK 固定 0.6）条数 ÷ 条目数',
        '100.00%',
        '80.00%',
        '-20.00%',
      ],
      ['最低分', 'min_score', '单条最低', '0.6195', '0.3000', '-0.3195'],
      ['最高分', 'max_score', '单条最高', '0.6800', '0.6800', '0'],
    ])
    // 通过率渲成百分数而不是「1」——混在一列里会被读成条数
    expect(rowsOfTitledTable(container, '去重面')).toEqual([
      ['重复率', '0', '0.2000'],
      ['唯一条数', '4', '4'],
    ])
    expect(rowsOfTitledTable(container, '长度面')).toEqual([
      ['平均长度', '8', '8.6000'],
      ['最短长度', '5', '5'],
      ['最长长度', '11', '11'],
      ['长度标准差', '2.2361', '2.3324'],
    ])
    expect(rowsOfTitledTable(container, '词汇面')).toEqual([
      ['唯一字符数', '28', '28'],
      ['字符总数', '32', '43'],
    ])
    expect(rowsOfTitledTable(container, '重叠面九项指标')).toEqual([
      ['A 侧条数', 'size_a', '4'],
      ['B 侧条数', 'size_b', '5'],
      ['条数差（A − B）', 'size_diff', '-1'],
      ['条数比（A ÷ B）', 'size_ratio', '80.0%'],
      ['匹配键交集条数', 'common_items', '4'],
      ['仅 A 侧匹配键数', 'unique_a', '0'],
      ['仅 B 侧匹配键数', 'unique_b', '0'],
      ['键相似度（交集 ÷ 并集）', 'similarity_score', '100.0%'],
      ['字段重叠（交集 ÷ 并集）', 'field_overlap', '100.0%'],
    ])
    // 两套摘要各自成段（口径不同，合并会把结论混起来）
    expect(screen.getByText(/质量面摘要/)).toBeInTheDocument()
    expect(screen.getByText(/重叠面摘要/)).toBeInTheDocument()
    expect(screen.getByText('数据集相似度较高，可考虑合并去重')).toBeInTheDocument()
    // A287：恒 0.0 的死面既不渲键也不渲值
    expect(screen.queryByText('similarity_to_a')).not.toBeInTheDocument()
    expect(screen.queryByText('similarity_to_b')).not.toBeInTheDocument()
  })

  it('同一份文件自己比自己：winner 渲成平手而不是「没有胜者」', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.compareDatasets.mockResolvedValue({
      quality_comparison: { ...COMPARE.quality_comparison, winner: 'tie' },
      overlap_comparison: COMPARE.overlap_comparison,
    })

    const container = await selectFileAndWait()
    await pickInNthSelect(container, 7, 'seed.json')
    fireEvent.click(screen.getByRole('button', { name: /运行双集对比/ }))

    expect(await screen.findByText('平手（两侧平均质量分之差 < 0.01）')).toBeInTheDocument()
    expect(screen.queryByText('A 侧平均质量分占优')).not.toBeInTheDocument()
    // 平手不是「什么都不渲」：九项指标表照样出声
    expect(rowsOfTitledTable(container, '重叠面九项指标').length).toBe(9)
  })

  it('B 侧 0 条时条数比是 null：接住 null 而不是渲成 NaN', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.compareDatasets.mockResolvedValue({
      quality_comparison: COMPARE.quality_comparison,
      overlap_comparison: {
        ...COMPARE.overlap_comparison,
        metrics: { ...COMPARE.overlap_comparison.metrics, size_b: 0, size_ratio: null },
      },
    })

    const container = await selectFileAndWait()
    await pickInNthSelect(container, 7, 'seed.json')
    fireEvent.click(screen.getByRole('button', { name: /运行双集对比/ }))

    expect(
      await screen.findByText(/—（B 侧 0 条：SDK 存 inf，JSON 面为 null）/),
    ).toBeInTheDocument()
    expect(container.textContent).not.toContain('NaN')
    expect(container.textContent).not.toContain('Infinity')
  })

  it('换匹配键发得出去、候选来自回执字段并集；明细列头按条目键渲，实测就是 instruction 三键', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.compareDatasets.mockResolvedValue(COMPARE)

    const container = await selectFileAndWait()
    await pickInNthSelect(container, 7, 'seed.json')
    fireEvent.click(screen.getByRole('button', { name: /运行双集对比/ }))
    await waitFor(() => expect(spies.compareDatasets).toHaveBeenCalledTimes(1))

    // 第 8 只下拉＝匹配键，候选是第一次回执的 field_comparisons 键（两侧字段并集）
    await pickInNthSelect(container, 8, 'output')
    fireEvent.click(screen.getByRole('button', { name: /运行双集对比/ }))
    await waitFor(() =>
      expect(spies.compareDatasets).toHaveBeenLastCalledWith('seed.json', 'seed.json', {
        keyFields: ['output'],
      }),
    )
    expect(screen.getByText('本次匹配键：output')).toBeInTheDocument()

    // 展开有值差异的那一行：明细列头来自条目键而不是写死的字段名
    const fieldTable = Array.from(container.querySelectorAll('.ant-table')).find((t) =>
      t.querySelector('.ant-table-title')?.textContent?.includes('字段对照'),
    )!
    const expandableRows = Array.from(fieldTable.querySelectorAll('.ant-table-row'))
    fireEvent.click(expandableRows[1].querySelector('.ant-table-row-expand-icon')!)
    await waitFor(() =>
      expect(fieldTable.querySelectorAll('.ant-table-expanded-row').length).toBe(1),
    )
    const detailHeader = Array.from(
      fieldTable.querySelectorAll('.ant-table-expanded-row .ant-table-thead th'),
    ).map((th) => th.textContent)
    expect(detailHeader).toEqual(['instruction', 'value_a', 'value_b'])
    expect(rowsOfTitledTable(container, '字段对照')).toEqual([
      ['', 'instruction', '0', '0', '4', '0', '0'],
      ['', 'output', '0', '0', '4', '0', '1'],
    ])
  })

  it('对比失败时专属文案出声，且不渲染质量面表', async () => {
    spies.analyzeData.mockResolvedValue(ANALYSIS)
    spies.compareDatasets.mockRejectedValue(new Error('boom'))

    const container = await selectFileAndWait()
    // 文件清单只有一份，B 侧选的就是同一只文件：必填闸过了，走「端点报错」那条支路
    await pickInNthSelect(container, 7, 'seed.json')
    fireEvent.click(screen.getByRole('button', { name: /运行双集对比/ }))

    await waitFor(() => expect(spies.compareDatasets).toHaveBeenCalled())
    await waitFor(() => expect(document.body.textContent).toContain('双集对比失败'), {
      timeout: 10000,
    })
    expect(screen.queryByText('质量面指标')).not.toBeInTheDocument()
  })
})
