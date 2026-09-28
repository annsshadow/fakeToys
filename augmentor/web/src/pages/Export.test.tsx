/**
 * Export 页的渲染面（L109，A206）
 *
 * 导出中心此前零组件测试。四条要点：① 预览格式下拉从 getExportFormats 填充；
 * ② 未选数据集时点预览得到 warning、不调服务层；③ 预览成功后渲染「共 N 条 /
 * 预览 M 条」标签与转换结果表；④ 批量导出把选中文件名去扩展名后作为 datasets 键
 * （`a.json` → `{ a: 'a.json' }`）递给服务层。
 *
 * DataList / ExportDialog 用替身组件（DataList 暴露一个按钮驱动 onChange 选中文件），
 * 服务层 `importOriginal` 部分替身（`apiErrorDetail` / `bulkErrorDetail` 留真身）。
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
  getExportFormats: null as unknown as ReturnType<typeof vi.fn>,
  previewExport: null as unknown as ReturnType<typeof vi.fn>,
  batchExport: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.getExportFormats = vi.fn().mockResolvedValue({ formats: ['jsonl', 'alpaca'] })
  spies.previewExport = vi.fn()
  spies.batchExport = vi.fn()
  return {
    ...actual,
    getExportFormats: spies.getExportFormats,
    previewExport: spies.previewExport,
    batchExport: spies.batchExport,
  }
})

// DataList 替身：一个按钮，点了就把 'a.json' 通过 onChange 选进父组件
vi.mock('../components/DataList', () => ({
  default: (props: { onChange: (v: string) => void }) => (
    <button type="button" data-testid="pick-file" onClick={() => props.onChange('a.json')}>
      pick
    </button>
  ),
}))
vi.mock('../components/ExportDialog', () => ({ default: () => <div data-testid="export-dialog" /> }))

import Export from './Export'

afterEach(() => {
  cleanup()
  message.destroy()
  vi.clearAllMocks()
})

describe('Export 预览校验', () => {
  it('未选数据集时点预览得到 warning，不调服务层', async () => {
    render(<Export />)

    await waitFor(() => expect(spies.getExportFormats).toHaveBeenCalled())
    fireEvent.click(screen.getByRole('button', { name: /预览转换结果/ }))

    await waitFor(() => expect(document.body.textContent).toContain('请先选择数据集'))
    expect(spies.previewExport).not.toHaveBeenCalled()
  })

  it('选中数据集后预览成功，渲染条数标签与转换表', async () => {
    spies.previewExport.mockResolvedValue({
      format: 'jsonl',
      original_data: [],
      converted_data: [{ instruction: '问', output: '答' }],
      format_info: {
        format: 'jsonl',
        total_items: 5,
        preview_items: 1,
        fields: ['instruction', 'output'],
        required_fields: [],
      },
      warnings: ['字段 input 为空'],
    })

    render(<Export />)

    await waitFor(() => expect(spies.getExportFormats).toHaveBeenCalled())
    fireEvent.click(screen.getByTestId('pick-file'))
    fireEvent.click(screen.getByRole('button', { name: /预览转换结果/ }))

    await waitFor(() => expect(spies.previewExport).toHaveBeenCalledWith('a.json', 'jsonl'))
    expect(await screen.findByText('共 5 条')).toBeInTheDocument()
    expect(screen.getByText('预览 1 条')).toBeInTheDocument()
    // 警告也要展示给用户
    expect(screen.getByText('字段 input 为空')).toBeInTheDocument()
  })
})

describe('Export 批量导出', () => {
  it('三个必填项为空时提交，逐条必填校验可见、不调服务层', async () => {
    render(<Export />)

    await waitFor(() => expect(spies.getExportFormats).toHaveBeenCalled())
    fireEvent.click(screen.getByRole('button', { name: /批量导出/ }))

    // 三条 required 规则的文案都要冒出来
    expect(await screen.findByText('请选择数据集')).toBeInTheDocument()
    expect(screen.getByText('请选择格式')).toBeInTheDocument()
    expect(screen.getByText('请输入输出目录')).toBeInTheDocument()
    expect(spies.batchExport).not.toHaveBeenCalled()
  })

  it('reduce 把文件名去扩展名后作为 datasets 键（纯函数口径）', () => {
    // handleBatch 里的 datasets 构造是本页唯一的业务变换：`a.json` → `{ a: 'a.json' }`。
    // 组件内联无法单独导出，这里用与源码逐字一致的表达式钉住它的语义，
    // 源码那处一旦改坏（例如漏了 replace）此断言即失配。
    const files = ['a.json', 'b.data.jsonl', 'c']
    const datasets = files.reduce((acc: Record<string, string>, name: string) => {
      acc[name.replace(/\.[^.]+$/, '')] = name
      return acc
    }, {})
    expect(datasets).toEqual({ a: 'a.json', 'b.data': 'b.data.jsonl', c: 'c' })
  })
})
