/**
 * Quality 页的渲染面（L112，A206）
 *
 * 质量中心聚合七项能力，此前零组件测试。四条要点：① `run` 守卫——未选数据集时任一
 * 动作都得到 warning、不调服务层；② 质量评估成功后渲染四张统计卡（含通过率百分比）；
 * ③ 去重结果 Tab 渲染四个计数标签；④ 离群点检测把（文件, 方法, 阈值）三参数递给服务层。
 *
 * DataList / QualityChart 用替身（后者内含 echarts），服务层 `importOriginal` 部分替身。
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
  evaluateQuality: null as unknown as ReturnType<typeof vi.fn>,
  dedupData: null as unknown as ReturnType<typeof vi.fn>,
  detectOutliers: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.evaluateQuality = vi.fn()
  spies.dedupData = vi.fn()
  spies.detectOutliers = vi.fn()
  return {
    ...actual,
    evaluateQuality: spies.evaluateQuality,
    dedupData: spies.dedupData,
    detectOutliers: spies.detectOutliers,
  }
})

// DataList 替身：一个按钮把 'a.json' 选进父组件
vi.mock('../components/DataList', () => ({
  default: (props: { onChange: (v: string) => void }) => (
    <button type="button" data-testid="pick-file" onClick={() => props.onChange('a.json')}>
      pick
    </button>
  ),
}))
vi.mock('../components/QualityChart', () => ({ default: () => <div data-testid="quality-chart" /> }))

import Quality from './Quality'

afterEach(() => {
  cleanup()
  message.destroy()
  vi.clearAllMocks()
})

describe('Quality run 守卫', () => {
  it('未选数据集就点质量评估得到 warning，不调服务层', async () => {
    render(<Quality />)

    fireEvent.click(screen.getByRole('button', { name: /质量评估/ }))

    await waitFor(() => expect(document.body.textContent).toContain('请先选择数据集'))
    expect(spies.evaluateQuality).not.toHaveBeenCalled()
  })
})

describe('Quality 质量评估', () => {
  it('评估成功后渲染四张统计卡（含通过率百分比）', async () => {
    spies.evaluateQuality.mockResolvedValue({
      total_samples: 100,
      passed_samples: 82,
      filtered_samples: 18,
      pass_rate: 0.82,
      avg_score: 0.7,
      threshold: 0.6,
      semantic_evaluated: false,
      effective_weights: [0.25, 0.25, 0.25, 0.25],
    })

    const { container } = render(<Quality />)
    fireEvent.click(screen.getByTestId('pick-file'))
    fireEvent.click(screen.getByRole('button', { name: /质量评估/ }))

    await waitFor(() => expect(spies.evaluateQuality).toHaveBeenCalledWith('a.json', 0.6))
    await waitFor(() =>
      expect(container.querySelectorAll('.ant-statistic-content-value').length).toBe(4)
    )
    const values = Array.from(container.querySelectorAll('.ant-statistic-content-value')).map(
      (el) => el.textContent
    )
    // 总数 100 / 通过 82 / 过滤 18 / 通过率 82.00%（后缀单独 span）
    expect(values.slice(0, 3)).toEqual(['100', '82', '18'])
    expect(values[3]).toContain('82.00')
  })
})

describe('Quality 去重结果', () => {
  it('去重成功后 Tab 渲染四个计数标签', async () => {
    spies.dedupData.mockResolvedValue({
      original_count: 100,
      deduplicated_count: 90,
      removed_count: 10,
      duplicate_groups: 4,
      kept_indices: [],
    })

    render(<Quality />)
    fireEvent.click(screen.getByTestId('pick-file'))
    fireEvent.click(screen.getByRole('button', { name: /智能去重/ }))

    await waitFor(() => expect(spies.dedupData).toHaveBeenCalledWith('a.json'))
    // 去重结果 Tab
    fireEvent.click(screen.getByRole('tab', { name: /去重结果/ }))
    expect(await screen.findByText('原始 100')).toBeInTheDocument()
    expect(screen.getByText('去重后 90')).toBeInTheDocument()
    expect(screen.getByText('移除 10')).toBeInTheDocument()
    expect(screen.getByText('重复组 4')).toBeInTheDocument()
  })
})

describe('Quality 离群点检测', () => {
  it('把（文件, 方法, 阈值）三参数递给服务层', async () => {
    spies.detectOutliers.mockResolvedValue({
      total_items: 100,
      outlier_count: 2,
      outlier_rate: 0.02,
      method: 'zscore',
      threshold: 3,
      field: 'instruction',
      outliers: [],
    })

    render(<Quality />)
    fireEvent.click(screen.getByTestId('pick-file'))
    fireEvent.click(screen.getByRole('button', { name: /离群点检测/ }))

    // 默认方法 zscore、默认阈值 3
    await waitFor(() =>
      expect(spies.detectOutliers).toHaveBeenCalledWith('a.json', 'zscore', 3)
    )
  })
})
