/**
 * Security 页的渲染面（L113，A206）
 *
 * 数据安全中心聚合脱敏/泄漏/审计三项，此前零组件测试。四条要点：① 挂载时拉 PII
 * 模式清单并渲染默认/增强两组标签与计数；② 隐私脱敏成功后渲染命中统计卡且入参带
 * includeExtra；③ 就绪审计成功后按 ready 渲染结论；④ 泄漏检测把（训练集, 测试集,
 * 阈值）三参数递给服务层。
 *
 * DataList 用替身（把 'a.json' 选进父组件），服务层 `importOriginal` 部分替身。
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
  getPiiPatterns: null as unknown as ReturnType<typeof vi.fn>,
  getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
  sanitizeData: null as unknown as ReturnType<typeof vi.fn>,
  auditDataset: null as unknown as ReturnType<typeof vi.fn>,
  checkLeakage: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.getPiiPatterns = vi
    .fn()
    .mockResolvedValue({ default: ['email', 'phone'], extra: ['idcard'] })
  spies.getDataFiles = vi
    .fn()
    .mockResolvedValue({ files: [{ name: 'train.json' }, { name: 'test.json' }] })
  spies.sanitizeData = vi.fn()
  spies.auditDataset = vi.fn()
  spies.checkLeakage = vi.fn()
  return {
    ...actual,
    getPiiPatterns: spies.getPiiPatterns,
    getDataFiles: spies.getDataFiles,
    sanitizeData: spies.sanitizeData,
    auditDataset: spies.auditDataset,
    checkLeakage: spies.checkLeakage,
  }
})

vi.mock('../components/DataList', () => ({
  default: (props: { onChange: (v: string) => void }) => (
    <button type="button" data-testid="pick-file" onClick={() => props.onChange('a.json')}>
      pick
    </button>
  ),
}))

import Security from './Security'

afterEach(() => {
  cleanup()
  message.destroy()
  vi.clearAllMocks()
})

describe('Security PII 模式清单', () => {
  it('挂载时渲染默认/增强两组标签与计数', async () => {
    render(<Security />)

    await waitFor(() => expect(screen.getByText('email')).toBeInTheDocument())
    expect(screen.getByText('phone')).toBeInTheDocument()
    expect(screen.getByText('idcard')).toBeInTheDocument()
    // 计数标签：默认 2 项 · 增强 1 项
    expect(screen.getByText(/默认 2 项/)).toBeInTheDocument()
  })
})

describe('Security 隐私脱敏', () => {
  it('脱敏入参带 includeExtra，成功后渲染命中统计', async () => {
    spies.sanitizeData.mockResolvedValue({
      items: [],
      report: { total_items: 100, touched_items: 30, matches: { email: 12 }, total_matches: 12 },
    })

    const { container } = render(<Security />)
    await waitFor(() => expect(spies.getPiiPatterns).toHaveBeenCalled())
    fireEvent.click(screen.getByTestId('pick-file'))
    fireEvent.click(screen.getByRole('button', { name: /隐私脱敏/ }))

    // 未勾选增强模式 ⇒ 第三参 false
    await waitFor(() =>
      expect(spies.sanitizeData).toHaveBeenCalledWith('a.json', undefined, false)
    )
    await waitFor(() => expect(screen.getByText('命中样本')).toBeInTheDocument())
    const values = Array.from(container.querySelectorAll('.ant-statistic-content-value')).map(
      (el) => el.textContent
    )
    expect(values).toContain('30')
    expect(values).toContain('12')
  })
})

describe('Security 就绪审计', () => {
  it('审计成功后按 ready 渲染结论', async () => {
    spies.auditDataset.mockResolvedValue({
      total_items: 100,
      pii_matches: 0,
      duplicate_count: 1,
      duplicate_rate: 0.01,
      empty_field_rate: 0,
      leak_count: 0,
      leak_rate: 0,
      findings: ['发现 1 条重复'],
      ready: true,
    })

    render(<Security />)
    await waitFor(() => expect(spies.getPiiPatterns).toHaveBeenCalled())
    fireEvent.click(screen.getByTestId('pick-file'))
    fireEvent.click(screen.getByRole('button', { name: /就绪审计/ }))

    await waitFor(() => expect(spies.auditDataset).toHaveBeenCalledWith('a.json', undefined))
  })
})

describe('Security 泄漏检测', () => {
  it('把（训练集, 测试集, 阈值）三参数递给服务层', async () => {
    spies.checkLeakage.mockResolvedValue({
      train_size: 80,
      test_size: 20,
      exact_leaks: 0,
      fuzzy_leaks: 0,
      total_leaks: 0,
      leak_rate: 0,
      is_clean: true,
      leaked_examples: [],
    })

    render(<Security />)
    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())

    // 切到泄漏检测 Tab
    fireEvent.click(screen.getByRole('tab', { name: /泄漏检测/ }))

    // 两个 antd Select（训练集/测试集）
    const selectors = document.querySelectorAll('.ant-select-selector')
    fireEvent.mouseDown(selectors[0])
    await waitFor(() =>
      expect(document.querySelector('.ant-select-item-option')).toBeInTheDocument()
    )
    fireEvent.click(document.querySelectorAll('.ant-select-item-option')[0]) // train.json

    fireEvent.mouseDown(document.querySelectorAll('.ant-select-selector')[1])
    await waitFor(() =>
      expect(document.querySelectorAll('.ant-select-item-option').length).toBeGreaterThan(1)
    )
    // 第二个下拉的第二个选项 test.json
    const opts = document.querySelectorAll('.ant-select-item-option')
    fireEvent.click(opts[opts.length - 1])

    fireEvent.click(screen.getByRole('button', { name: /检测泄漏/ }))

    await waitFor(() => expect(spies.checkLeakage).toHaveBeenCalledTimes(1))
    const [train, test, threshold] = spies.checkLeakage.mock.calls[0]
    expect(train).toBe('train.json')
    expect(test).toBeTruthy()
    expect(threshold).toBe(0.8)
  })
})
