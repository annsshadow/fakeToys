/**
 * Augmentation 页的渲染面（L107，A206）
 *
 * 数据增强页此前零组件测试。六条要点：① 未选输入文件就点「开始增强」得到
 * warning、不调服务层；② 选了文件后 startAugmentation 收到五个参数（输入/输出/
 * 三个开关）且默认输出名是 augmented_output.json；③ 加载文件列表失败有可见文案；
 * ④ 轮询拿到带 task_id 的进度后渲染任务详情（否则显示「暂无进行中的任务」）；
 * ⑤（L277）进度到终态那一拍之后定时器被关掉，再多跑十拍一次都不许多打；
 * ⑥（L277）停表之后第二次点「开始增强」把表重新起起来——改前⑤恒真、⑥靠泄漏侥幸。
 *
 * 轮询用 `setInterval(2s)`：进度用例用假定时器推进一拍，其余用例把 getProgress
 * 固定成 `{ status: 'no_checkpoint' }`（不带 task_id ⇒ 不进详情分支）。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
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
  startAugmentation: null as unknown as ReturnType<typeof vi.fn>,
  getProgress: null as unknown as ReturnType<typeof vi.fn>,
  getCheckpoints: null as unknown as ReturnType<typeof vi.fn>,
  getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.startAugmentation = vi.fn().mockResolvedValue({ success: true, message: 'ok' })
  spies.getProgress = vi.fn().mockResolvedValue({ status: 'no_checkpoint' })
  spies.getCheckpoints = vi.fn().mockResolvedValue({ checkpoints: [] })
  spies.getDataFiles = vi.fn().mockResolvedValue({ files: [{ name: 'seed.json' }] })
  return {
    ...actual,
    startAugmentation: spies.startAugmentation,
    getProgress: spies.getProgress,
    getCheckpoints: spies.getCheckpoints,
    getDataFiles: spies.getDataFiles,
  }
})

import Augmentation from './Augmentation'

afterEach(() => {
  cleanup()
  message.destroy()
  vi.clearAllMocks()
  vi.useRealTimers()
})

describe('Augmentation 启动校验与负载', () => {
  beforeEach(() => {
    spies.getProgress.mockResolvedValue({ status: 'no_checkpoint' })
  })

  it('未选输入文件时点开始得到 warning，不调服务层', async () => {
    render(<Augmentation />)

    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())
    fireEvent.click(screen.getByRole('button', { name: /开始增强/ }))

    await waitFor(() => expect(document.body.textContent).toContain('请选择输入文件'))
    expect(spies.startAugmentation).not.toHaveBeenCalled()
  })

  it('选文件后 startAugmentation 收到五参数与默认输出名', async () => {
    const { container } = render(<Augmentation />)

    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())
    // 直接驱动受控 Select：mousedown 打开后点选项（用 option 类精确定位，避免与
    // 选中态回显的同名节点冲突）
    fireEvent.mouseDown(container.querySelector('.ant-select')!)
    await waitFor(() =>
      expect(document.querySelector('.ant-select-item-option')).toBeInTheDocument()
    )
    fireEvent.click(document.querySelector('.ant-select-item-option')!)

    fireEvent.click(screen.getByRole('button', { name: /开始增强/ }))

    await waitFor(() => expect(spies.startAugmentation).toHaveBeenCalledTimes(1))
    expect(spies.startAugmentation).toHaveBeenCalledWith(
      'seed.json',
      'augmented_output.json',
      true,
      true,
      true
    )
  })
})

describe('Augmentation 文件列表失败', () => {
  it('加载失败时看得见专属文案', async () => {
    spies.getDataFiles.mockRejectedValue(new Error('boom'))

    render(<Augmentation />)

    await waitFor(() => expect(document.body.textContent).toContain('加载文件列表失败'), {
      timeout: 4000,
    })
  })
})

describe('Augmentation 进度轮询', () => {
  it('轮询拿到带 task_id 的进度后渲染任务详情', async () => {
    vi.useFakeTimers()
    spies.getProgress.mockResolvedValue({
      task_id: 't-42',
      total_items: 10,
      processed_items: 4,
      failed_items: 1,
      progress: 0.4,
      progress_percent: '40.0%',
      elapsed_time: '0:00:12',
      remaining_time: '0:00:18',
      start_time: 's',
      last_update: 'u',
      avg_quality_score: 0.812,
    })

    render(<Augmentation />)
    // 推进一拍触发第一次 checkProgress，并放行其 await 链
    await vi.advanceTimersByTimeAsync(2100)
    vi.useRealTimers()

    await waitFor(() => expect(screen.getByText('t-42')).toBeInTheDocument())
    expect(screen.getByText('平均质量评分: 0.812')).toBeInTheDocument()
  })
})

describe('Augmentation 轮询的停表与重启（L277）', () => {
  beforeEach(() => {
    // 上一条用例把 getDataFiles 设成了 reject，`clearAllMocks` 不清实现 —— 漏进来就没有
    // 文件可选，重启用例点不到「开始增强」。这一族跨用例的实现泄漏按格显式重设。
    spies.getDataFiles.mockResolvedValue({ files: [{ name: 'seed.json' }] })
    spies.startAugmentation.mockResolvedValue({ success: true, message: 'ok' })
  })

  const detail = (progress: number) => ({
    task_id: 't-9',
    total_items: 10,
    processed_items: Math.round(progress * 10),
    failed_items: 0,
    progress,
    progress_percent: `${Math.round(progress * 100)}%`,
    elapsed_time: '0:00:01',
    remaining_time: '0:00:02',
    start_time: 's',
    last_update: 'u',
    avg_quality_score: 0.7,
  })

  it('到终态那一拍之后不再打进度端点（改前它每 2 秒一直打）', async () => {
    vi.useFakeTimers()
    const sequence = [0.4, 0.6, 1]
    spies.getProgress.mockImplementation(async () => detail(sequence.shift() ?? 1))

    render(<Augmentation />)
    await vi.advanceTimersByTimeAsync(2100 * 3)

    expect(spies.getProgress).toHaveBeenCalledTimes(3)
    // 终态 toast 由 React 调度器在假定时器下渲染不出（实测 DOM 里没有「增强任务完成！」），
    // 所以这里判的是可控的那半：进度卡片停在终态、并且端点不再被戳。
    expect(document.body.textContent).toContain('增强进度100%')

    // 停表判据：再多跑 10 拍，一次都不许多打。改前 `checkProgress` 够不着定时器句柄，
    // 这十拍会原样变成十次请求，并且「增强任务完成！」每拍重发一次。
    await vi.advanceTimersByTimeAsync(2100 * 10)
    expect(spies.getProgress).toHaveBeenCalledTimes(3)
    vi.useRealTimers()
  })

  it('上一轮到终态停了表，第二次点「开始增强」把表重新起起来', async () => {
    spies.getProgress.mockResolvedValue(detail(1))

    const { container } = render(<Augmentation />)

    fireEvent.mouseDown(container.querySelector('.ant-select')!)
    await waitFor(() =>
      expect(document.querySelector('.ant-select-item-option')).toBeInTheDocument()
    )
    fireEvent.click(document.querySelector('.ant-select-item-option')!)

    // 第一拍的 mock 就是 `progress: 1` ⇒ 立刻到终态、表被关掉
    await waitFor(() => expect(spies.getProgress).toHaveBeenCalled(), { timeout: 6000 })

    spies.getProgress.mockClear()
    fireEvent.click(screen.getByRole('button', { name: /开始增强/ }))
    await waitFor(() => expect(spies.startAugmentation).toHaveBeenCalledTimes(1))

    // 真定时器：轮询节奏是 2 秒一拍，所以这一等最长跨一拍
    await waitFor(() => expect(spies.getProgress).toHaveBeenCalled(), { timeout: 6000 })
  })
})

describe('Augmentation 断点列表（L211 接线）', () => {
  it('挂载即拉断点并把每个断点 ID 渲成标签；刷新再拉一次', async () => {
    spies.getCheckpoints.mockResolvedValue({ checkpoints: ['ck-a', 'ck-b'] })

    render(<Augmentation />)

    await waitFor(() => expect(screen.getByText('ck-a')).toBeInTheDocument())
    expect(screen.getByText('ck-b')).toBeInTheDocument()
    expect(spies.getCheckpoints).toHaveBeenCalledTimes(1)

    fireEvent.click(screen.getByRole('button', { name: /刷新/ }))
    await waitFor(() => expect(spies.getCheckpoints).toHaveBeenCalledTimes(2))
  })

  it('无断点时显示「暂无断点」而不是空卡片', async () => {
    // clearAllMocks 只清调用记录、不清 mockResolvedValue —— 上一用例设的
    // ['ck-a','ck-b'] 会漏进本用例，必须显式重设空清单
    spies.getCheckpoints.mockResolvedValue({ checkpoints: [] })

    render(<Augmentation />)

    await waitFor(() => expect(spies.getCheckpoints).toHaveBeenCalled())
    expect(await screen.findByText('暂无断点')).toBeInTheDocument()
  })
})
