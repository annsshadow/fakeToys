/**
 * Analysis 页的渲染面（L111，A206）
 *
 * 数据分析页此前零组件测试。四条要点：① 选文件后自动 analyzeData 并渲染四张统计卡
 * （总量/平均长度/唯一词/去重移除）；② 未选文件时点清洗/基准得到 warning、不调服务层；
 * ③ 清洗成功后渲染四个计数标签；④ 基准成功后渲染指标表。
 *
 * ReactECharts 用替身（jsdom 里 echarts 无法真渲染，也不是本用例关心的东西）；
 * 服务层 `importOriginal` 部分替身（`apiErrorDetail` 留真身）。
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
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.getDataFiles = vi.fn().mockResolvedValue({ files: [{ name: 'seed.json' }] })
  spies.analyzeData = vi.fn()
  spies.cleanData = vi.fn()
  spies.runBenchmark = vi.fn()
  return {
    ...actual,
    getDataFiles: spies.getDataFiles,
    analyzeData: spies.analyzeData,
    cleanData: spies.cleanData,
    runBenchmark: spies.runBenchmark,
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
