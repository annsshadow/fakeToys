/**
 * Quality 页的渲染面（L112，A206）
 *
 * 质量中心聚合八项能力，此前零组件测试。四条要点：① `run` 守卫——未选数据集时任一
 * 动作都得到 warning、不调服务层；② 质量评估成功后渲染四张统计卡（含通过率百分比）；
 * ③ 去重结果 Tab 渲染四个计数标签；④ 离群点检测把（文件, 方法, 阈值）三参数递给服务层。
 *
 * L226 起第八项「结构校验」进本页（A279 第二批接线），对应下面 `Quality 结构校验`：
 * run 守卫、默认预设、换预设、失败四路。
 *
 * DataList / QualityChart 用替身（后者内含 echarts），服务层 `importOriginal` 部分替身。
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

const spies = vi.hoisted(() => ({
  evaluateQuality: null as unknown as ReturnType<typeof vi.fn>,
  dedupData: null as unknown as ReturnType<typeof vi.fn>,
  detectOutliers: null as unknown as ReturnType<typeof vi.fn>,
  validateDataset: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.evaluateQuality = vi.fn()
  spies.dedupData = vi.fn()
  spies.detectOutliers = vi.fn()
  spies.validateDataset = vi.fn()
  return {
    ...actual,
    evaluateQuality: spies.evaluateQuality,
    dedupData: spies.dedupData,
    detectOutliers: spies.detectOutliers,
    validateDataset: spies.validateDataset,
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
vi.mock('../components/QualityChart', () => ({
  default: () => <div data-testid="quality-chart" />,
}))

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
      expect(container.querySelectorAll('.ant-statistic-content-value').length).toBe(4),
    )
    const values = Array.from(container.querySelectorAll('.ant-statistic-content-value')).map(
      (el) => el.textContent,
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
    await waitFor(() => expect(spies.detectOutliers).toHaveBeenCalledWith('a.json', 'zscore', 3))
  })
})

/**
 * 结构校验（L226，A279 第二批接线）
 *
 * 结果在 Tabs 的第八个 pane 里，所以每条用例都要先点 tab 再断言——pane 未激活时
 * antd 根本不渲染它，「查不到文本」在这里既可能是接线坏了，也可能是忘了切 tab，
 * 因此成功那条把「切 tab 前查不到、切到后查得到」都钉住，证明文本确实来自这个 pane。
 */
describe('Quality 结构校验', () => {
  const VALIDATION = {
    is_valid: false,
    total_items: 200,
    valid_items: 180,
    error_count: 20,
    warning_count: 37,
    issues: [
      { field: 'output', message: '字段 output 不能为空', severity: 'error', index: 7 },
      // 实测 SDK 会回 index=null 的问题（整份文件级的问题没有条目下标），必须渲成 '-'
      { field: 'instruction', message: '长度不能小于 5', severity: 'warning', index: null },
    ],
  }

  const rowsOfIssueTable = (container: HTMLElement) => {
    const table = Array.from(container.querySelectorAll('.ant-table')).find((t) =>
      t.querySelector('.ant-table-thead')?.textContent?.includes('级别'),
    )
    if (!table) throw new Error('找不到「级别」表头的 issues 表')
    return Array.from(table.querySelectorAll('.ant-table-tbody .ant-table-row')).map((row) =>
      Array.from(row.querySelectorAll('td')).map((td) => td.textContent),
    )
  }

  it('未选数据集就点结构校验得到 warning，不调服务层', async () => {
    render(<Quality />)

    fireEvent.click(screen.getByRole('button', { name: /结构校验/ }))

    await waitFor(() => expect(document.body.textContent).toContain('请先选择数据集'))
    expect(spies.validateDataset).not.toHaveBeenCalled()
  })

  it('默认预设 basic 递出去；统计卡、判决与 issues 表逐列渲染', async () => {
    spies.validateDataset.mockResolvedValue(VALIDATION)

    const { container } = render(<Quality />)
    fireEvent.click(screen.getByTestId('pick-file'))
    fireEvent.click(screen.getByRole('button', { name: /结构校验/ }))
    await waitFor(() => expect(spies.validateDataset).toHaveBeenCalledWith('a.json', 'basic'))

    // 还没切到那个 pane：这一步让「下面查到了」不可能来自默认 pane
    expect(screen.queryByText('条目总数')).not.toBeInTheDocument()

    fireEvent.click(screen.getByRole('tab', { name: /结构校验/ }))

    expect(await screen.findByText('条目总数')).toBeInTheDocument()
    const stats = Array.from(container.querySelectorAll('.ant-statistic-content-value')).map(
      (el) => el.textContent,
    )
    // 顺序即页面顺序：总数 / 合格 / 错误 / 警告。四个数都来自响应，没有一个是从 issues 行数算的
    expect(stats).toEqual(['200', '180', '20', '37'])
    // is_valid=false 时说的是「未通过」，同时把错误条数带在文案里
    expect(screen.getByText('校验未通过：20 条错误')).toBeInTheDocument()
    expect(rowsOfIssueTable(container)).toEqual([
      ['error', 'output', '7', '字段 output 不能为空'],
      ['warning', 'instruction', '-', '长度不能小于 5'],
    ])
  })

  it('换预设真的换：下拉选 strict 后递出去的是 strict', async () => {
    spies.validateDataset.mockResolvedValue({
      ...VALIDATION,
      is_valid: true,
      error_count: 0,
      warning_count: 3,
      issues: [],
    })

    const { container } = render(<Quality />)
    fireEvent.click(screen.getByTestId('pick-file'))
    // 本页两个下拉：0=离群点方法，1=验证预设
    fireEvent.mouseDown(container.querySelectorAll('.ant-select')[1])
    await waitFor(() =>
      expect(container.querySelectorAll('[role=combobox]')[1]).toHaveAttribute(
        'aria-expanded',
        'true',
      ),
    )
    const dropdown = () => document.querySelectorAll('.ant-select-dropdown')
    await waitFor(() => expect(dropdown().length).toBeGreaterThanOrEqual(1))
    const option = Array.from(
      dropdown()[dropdown().length - 1].querySelectorAll('.ant-select-item-option'),
    ).find((el) => el.textContent === 'strict（加长度下限与禁用片段）')
    if (!option) throw new Error('验证预设下拉里没有 strict')
    fireEvent.click(option)
    fireEvent.click(screen.getByRole('button', { name: /结构校验/ }))

    await waitFor(() => expect(spies.validateDataset).toHaveBeenCalledWith('a.json', 'strict'))
    fireEvent.click(screen.getByRole('tab', { name: /结构校验/ }))
    // is_valid=true 且有警告：说「没有错误，但有 3 条警告」，不能整句报成「全部条目通过校验」
    expect(await screen.findByText('没有错误，但有 3 条警告')).toBeInTheDocument()
    expect(screen.queryByText('全部条目通过校验')).not.toBeInTheDocument()
    // 零问题时要明确说没问题，而不是留一张空表让人自己猜
    expect(screen.getByText('这份文件没有报出任何问题')).toBeInTheDocument()
  })

  it('校验失败时专属文案出声，且不渲染统计卡', async () => {
    spies.validateDataset.mockRejectedValue(new Error('boom'))

    render(<Quality />)
    fireEvent.click(screen.getByTestId('pick-file'))
    fireEvent.click(screen.getByRole('button', { name: /结构校验/ }))

    await waitFor(() => expect(document.body.textContent).toContain('结构校验失败'), {
      timeout: 10000,
    })
    fireEvent.click(screen.getByRole('tab', { name: /结构校验/ }))
    expect(screen.queryByText('条目总数')).not.toBeInTheDocument()
  })
})
