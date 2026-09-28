/**
 * Multimodal 页的渲染面（L108，A206）
 *
 * 多模态处理页此前零组件测试。四条要点：① 顶部 Alert 从 formats 渲染出图像/音频
 * 扩展名，未加载时显示「正在加载」；② 单条处理提交后按结果渲染模态标签与图像信息卡；
 * ③ 目录扫描提交后渲染三张统计（记录数/有效/错误）与记录表；④ 处理失败有可见文案。
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
  getMultimodalFormats: null as unknown as ReturnType<typeof vi.fn>,
  processMultimodal: null as unknown as ReturnType<typeof vi.fn>,
  scanMultimodal: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.getMultimodalFormats = vi
    .fn()
    .mockResolvedValue({ image_extensions: ['.png', '.jpg'], audio_extensions: ['.wav'] })
  spies.processMultimodal = vi.fn()
  spies.scanMultimodal = vi.fn()
  return {
    ...actual,
    getMultimodalFormats: spies.getMultimodalFormats,
    processMultimodal: spies.processMultimodal,
    scanMultimodal: spies.scanMultimodal,
  }
})

import Multimodal from './Multimodal'

afterEach(() => {
  cleanup()
  message.destroy()
  vi.clearAllMocks()
})

describe('Multimodal 格式提示', () => {
  it('Alert 从 formats 渲染图像/音频扩展名', async () => {
    render(<Multimodal />)

    await waitFor(() =>
      expect(document.body.textContent).toContain('支持图像格式：.png、.jpg')
    )
    expect(document.body.textContent).toContain('支持音频格式：.wav')
  })
})

describe('Multimodal 单条处理', () => {
  it('处理成功后渲染模态标签与图像信息卡', async () => {
    spies.processMultimodal.mockResolvedValue({
      text: '一张图',
      image: { width: 640, height: 480, format: 'PNG', size_bytes: 1024 },
      audio: null,
      modalities: ['text', 'image'],
      valid: true,
      errors: [],
    })

    render(<Multimodal />)

    await waitFor(() => expect(spies.getMultimodalFormats).toHaveBeenCalled())
    fireEvent.click(screen.getByRole('button', { name: /处.*理/ }))

    await waitFor(() => expect(screen.getByText('640 × 480')).toBeInTheDocument())
    expect(screen.getByText(/格式 PNG/)).toBeInTheDocument()
    expect(screen.getByText('有效')).toBeInTheDocument()
  })

  it('处理失败时看得见专属文案', async () => {
    spies.processMultimodal.mockRejectedValue(new Error('boom'))

    render(<Multimodal />)

    await waitFor(() => expect(spies.getMultimodalFormats).toHaveBeenCalled())
    fireEvent.click(screen.getByRole('button', { name: /处.*理/ }))

    await waitFor(() => expect(document.body.textContent).toContain('多模态处理失败'), {
      timeout: 4000,
    })
  })
})

describe('Multimodal 目录扫描', () => {
  it('扫描成功后渲染统计与记录表', async () => {
    spies.scanMultimodal.mockResolvedValue({
      total_records: 3,
      valid_records: 2,
      invalid_records: 1,
      modality_counts: {},
      error_count: 1,
      errors: ['坏文件'],
      records: [
        { text: 'a', image: null, audio: null, modalities: ['text'], valid: true, errors: [] },
      ],
    })

    const { container } = render(<Multimodal />)

    await waitFor(() => expect(spies.getMultimodalFormats).toHaveBeenCalled())
    const dirInput = container.querySelector('input[id="directory"]') as HTMLInputElement
    fireEvent.change(dirInput, { target: { value: 'data/mm' } })
    fireEvent.click(screen.getByRole('button', { name: /扫描目录/ }))

    await waitFor(() => expect(spies.scanMultimodal).toHaveBeenCalledWith('data/mm'))
    // 「有效」在统计标题与记录标签里都出现，故按统计标题类精确定位三张统计
    await waitFor(() => expect(screen.getByText('记录数')).toBeInTheDocument())
    const statTitles = Array.from(
      container.querySelectorAll('.ant-statistic-title')
    ).map((el) => el.textContent)
    expect(statTitles).toEqual(['记录数', '有效', '错误'])
  })
})
