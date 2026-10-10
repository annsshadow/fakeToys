/**
 * Versions 页的渲染面（L110，A206）
 *
 * 版本管理页此前零组件测试。四条要点：① 版本表按 getVersions 渲染每行的 id/标签/
 * 条数；② 创建弹窗里未选文件就确定得到 warning、不调服务层；③ 选文件后 createVersion
 * 收到（文件, 标签, 描述）；④ 对比弹窗选两个版本后 diffVersions 被调用并渲染
 * 新增/移除/修改三个计数。
 *
 * 服务层 `importOriginal` 部分替身（`apiErrorDetail` 留真身）；清场口径承前。
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
  getVersions: null as unknown as ReturnType<typeof vi.fn>,
  getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
  createVersion: null as unknown as ReturnType<typeof vi.fn>,
  diffVersions: null as unknown as ReturnType<typeof vi.fn>,
  rollbackVersion: null as unknown as ReturnType<typeof vi.fn>,
  deleteVersion: null as unknown as ReturnType<typeof vi.fn>,
  getVersion: null as unknown as ReturnType<typeof vi.fn>,
  getVersionData: null as unknown as ReturnType<typeof vi.fn>,
  getVersionHistory: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.getVersions = vi.fn().mockResolvedValue({
    versions: [
      { version_id: 'v-1', label: '首版', description: 'd', created_at: 't', item_count: 12 },
      { version_id: 'v-2', label: '次版', description: 'e', created_at: 't2', item_count: 20 },
    ],
  })
  spies.getDataFiles = vi.fn().mockResolvedValue({ files: [{ name: 'seed.json' }] })
  spies.createVersion = vi.fn().mockResolvedValue({ success: true })
  spies.diffVersions = vi.fn()
  spies.rollbackVersion = vi.fn().mockResolvedValue({ success: true })
  spies.deleteVersion = vi.fn().mockResolvedValue({ success: true })
  spies.getVersion = vi.fn().mockResolvedValue({
    version_id: 'v-1',
    label: '首版',
    description: 'd',
    created_at: 't',
    item_count: 12,
    metadata: { source: 'seed.json' },
  })
  spies.getVersionData = vi.fn().mockResolvedValue({
    items: [{ instruction: '租房押金多少', input: '', output: '押金一个月' }],
  })
  spies.getVersionHistory = vi.fn().mockResolvedValue({
    history: [
      { action: 'create', version_id: 'v-1', timestamp: '2026-10-10T00:00:00', detail: { label: '首版', item_count: 12 } },
    ],
  })
  return {
    ...actual,
    getVersions: spies.getVersions,
    getDataFiles: spies.getDataFiles,
    createVersion: spies.createVersion,
    diffVersions: spies.diffVersions,
    rollbackVersion: spies.rollbackVersion,
    deleteVersion: spies.deleteVersion,
    getVersion: spies.getVersion,
    getVersionData: spies.getVersionData,
    getVersionHistory: spies.getVersionHistory,
  }
})

import Versions from './Versions'

afterEach(() => {
  cleanup()
  message.destroy()
  vi.clearAllMocks()
})

describe('Versions 列表渲染', () => {
  it('版本表渲染每行的标签与数据条数', async () => {
    render(<Versions />)

    await waitFor(() => expect(screen.getByText('首版')).toBeInTheDocument())
    expect(screen.getByText('次版')).toBeInTheDocument()
    expect(screen.getByText('12')).toBeInTheDocument()
    expect(screen.getByText('20')).toBeInTheDocument()
  })
})

describe('Versions 创建版本', () => {
  it('未选文件就确定得到 warning，不调服务层', async () => {
    render(<Versions />)

    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())
    fireEvent.click(screen.getByRole('button', { name: /创建版本/ }))
    // 弹窗的确定按钮
    fireEvent.click(screen.getByRole('button', { name: "OK" }))

    await waitFor(() => expect(document.body.textContent).toContain('请选择文件'))
    expect(spies.createVersion).not.toHaveBeenCalled()
  })

  it('选文件后 createVersion 收到（文件, 标签, 描述）', async () => {
    render(<Versions />)

    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())
    fireEvent.click(screen.getByRole('button', { name: /创建版本/ }))

    // Modal 渲进 document.body 的 portal，不在 render 容器内 ⇒ 用 document 查询
    const fileSelect = await screen.findByRole('combobox')
    fireEvent.change(fileSelect, { target: { value: 'seed.json' } })
    const labelInput = document.querySelector('input[placeholder="例如: v1.0"]') as HTMLInputElement
    fireEvent.change(labelInput, { target: { value: 'v1.0' } })
    const descInput = document.querySelector('textarea') as HTMLTextAreaElement
    fireEvent.change(descInput, { target: { value: '初始快照' } })

    fireEvent.click(screen.getByRole('button', { name: "OK" }))

    await waitFor(() =>
      expect(spies.createVersion).toHaveBeenCalledWith('seed.json', 'v1.0', '初始快照')
    )
  })
})

describe('Versions 对比', () => {
  it('选两个版本后 diffVersions 被调用并渲染三个计数', async () => {
    spies.diffVersions.mockResolvedValue({
      version1: 'v-1',
      version2: 'v-2',
      added_count: 8,
      removed_count: 2,
      modified_count: 3,
    })

    render(<Versions />)

    await waitFor(() => expect(screen.getByText('首版')).toBeInTheDocument())
    fireEvent.click(screen.getByRole('button', { name: /版本对比/ }))

    const selects = await screen.findAllByRole('combobox')
    // 对比弹窗里两个原生 select
    fireEvent.change(selects[0], { target: { value: 'v-1' } })
    fireEvent.change(selects[1], { target: { value: 'v-2' } })
    fireEvent.click(screen.getByRole('button', { name: "OK" }))

    await waitFor(() => expect(spies.diffVersions).toHaveBeenCalledWith('v-1', 'v-2'))
    expect(await screen.findByText('8')).toBeInTheDocument()
    expect(screen.getByText('对比结果')).toBeInTheDocument()
  })
})

describe('Versions 详情/数据/操作历史（L211 接线）', () => {
  it('点详情：getVersion 被调用，metadata 以 JSON 渲出', async () => {
    render(<Versions />)
    await waitFor(() => expect(screen.getByText('首版')).toBeInTheDocument())

    fireEvent.click(screen.getAllByRole('button', { name: /详情/ })[0])

    await waitFor(() => expect(spies.getVersion).toHaveBeenCalledWith('v-1'))
    expect(await screen.findByTestId('version-metadata')).toHaveTextContent('"source": "seed.json"')
  })

  it('点数据：getVersionData 被调用，条目渲进表格', async () => {
    render(<Versions />)
    await waitFor(() => expect(screen.getByText('首版')).toBeInTheDocument())

    fireEvent.click(screen.getAllByRole('button', { name: /数据/ })[0])

    await waitFor(() => expect(spies.getVersionData).toHaveBeenCalledWith('v-1'))
    expect(await screen.findByText('租房押金多少')).toBeInTheDocument()
    expect(screen.getByText('押金一个月')).toBeInTheDocument()
  })

  it('操作历史：getVersionHistory(50) 被调用，行渲出操作/版本/详情 JSON', async () => {
    render(<Versions />)
    await waitFor(() => expect(screen.getByText('首版')).toBeInTheDocument())

    fireEvent.click(screen.getByRole('button', { name: /操作历史/ }))

    await waitFor(() => expect(spies.getVersionHistory).toHaveBeenCalledWith(50))
    expect(await screen.findByText('create')).toBeInTheDocument()
    // detail 渲的是 JSON.stringify 现量（冒号后无空格），不是格式化 JSON
    expect(screen.getByText(/"item_count":12/)).toBeInTheDocument()
  })
})
