/**
 * `Settings` 的只读回显面与保存回执面（L237 / A281）
 *
 * 存在理由与 `System.*.test.tsx` 那几份同构：Python 侧的
 * `test_ts_contract_types_l237.py` 只能证明**类型面补齐了、源码里写了那几句口径**，
 * 证不出渲染面上真的看得见。这一份钉五件事：
 * ① 六节生效值逐格渲染，含本轮新补的 7 个节内键（`vector` 三键、`rag` 两键、
 *    `multimodal` 两键）——补了类型不渲染等于把死面从后端挪到前端；
 * ② 那张卡的内容是**从响应键推导**出来的，不是手抄的节名清单：往夹具里塞一节
 *    响应里没有过的 `web`，卡上必须自动多一行（手抄的话这一支恒绿不了）；
 * ③ 保存之后回执卡显示后端真回的 `message` 与 `ignored_keys`，逐条列出；
 * ④ `ignored_keys` 为空那一档说「0 个」，不编造键；
 * ⑤ 卡上写明四节可写但本页没做控件、两节根本不在写入清单里。
 *
 * 夹具是 `AppConfig` 的完整实例（标了类型），十节逐键都在，形状取自 `GET /api/config`
 * 的真响应。服务层用 `importOriginal` 做部分替身，`apiErrorDetail` 留真身。
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

const fx = vi.hoisted(() => ({
  spies: {
    getConfig: null as unknown as ReturnType<typeof vi.fn>,
    getModels: null as unknown as ReturnType<typeof vi.fn>,
    getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
    updateConfig: null as unknown as ReturnType<typeof vi.fn>,
    autoConfigRecommend: null as unknown as ReturnType<typeof vi.fn>,
    validateConfigFile: null as unknown as ReturnType<typeof vi.fn>,
  },
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  const s = fx.spies
  s.getConfig = vi.fn()
  s.getModels = vi.fn().mockResolvedValue({ models: ['ernie'], default: 'ernie' })
  s.getDataFiles = vi.fn().mockResolvedValue({ files: [{ name: 'seed.json' }] })
  s.updateConfig = vi.fn()
  s.autoConfigRecommend = vi.fn()
  s.validateConfigFile = vi.fn()
  return {
    ...actual,
    getConfig: s.getConfig,
    getModels: s.getModels,
    getDataFiles: s.getDataFiles,
    updateConfig: s.updateConfig,
    autoConfigRecommend: s.autoConfigRecommend,
    validateConfigFile: s.validateConfigFile,
  }
})

import Settings from './Settings'
import type { AppConfig } from '../types/api'

/** `AppConfig` 的完整实例：形状取自 `GET /api/config` 的真响应，十节逐键都在 */
const CONFIG: AppConfig = {
  default_model: 'ernie',
  augmentation: { variants_per_seed: 3, num_threads: 4 },
  quality: { enabled: true, threshold: 0.7 },
  dedup: { enabled: true, threshold: 0.85 },
  export: { default_format: 'jsonl', formats: ['jsonl', 'alpaca'] },
  vector: {
    enabled: false,
    backend: 'chromadb',
    dimension: 384,
    storage_dir: 'data/vectors',
    collection: 'default',
  },
  rag: { enabled: false, default_format: 'jsonl', chunk_size: 512, chunk_overlap: 64 },
  multimodal: {
    enabled: false,
    image_extensions: ['.jpg', '.png'],
    audio_extensions: ['.wav', '.mp3'],
  },
  versioning: { enabled: true },
  visualization: { enabled: true },
}

afterEach(() => {
  cleanup()
  message.destroy()
  vi.clearAllMocks()
})

describe('Settings 只读回显卡（L237 补渲染）', () => {
  it('六节生效值逐格渲染，含本轮新补的 7 个节内键', async () => {
    fx.spies.getConfig.mockResolvedValue(CONFIG)

    render(<Settings />)

    await waitFor(() => expect(screen.getByText('其余节的生效值（本页只读）')).toBeInTheDocument())
    const text = document.body.textContent ?? ''
    // 改前这 7 键在类型面上根本不存在，页面上也就无从渲染
    for (const cell of [
      'dimension: 384',
      'storage_dir: data/vectors',
      'collection: default',
      'chunk_size: 512',
      'chunk_overlap: 64',
      'image_extensions: .jpg / .png',
      'audio_extensions: .wav / .mp3',
    ]) {
      expect(text).toContain(cell)
    }
    // 两只只读节整节可见
    expect(text).toContain('versioning')
    expect(text).toContain('visualization')
    // 有表单控件的三节不在这一张卡里重复出现（它们在上面三张卡上可改）
    expect(screen.queryByText('augmentation')).not.toBeInTheDocument()
  })

  it('响应里多出一节时卡上自动多一行——那张卡不是手抄的节名清单', async () => {
    // `web` 不在 `AppConfig` 上，也不在本页任何清单里：它只能是被推导出来的
    fx.spies.getConfig.mockResolvedValue({
      ...CONFIG,
      web: { max_upload_bytes: 10485760 },
    })

    render(<Settings />)

    await waitFor(() => expect(screen.getByText('web')).toBeInTheDocument())
    expect(document.body.textContent).toContain('max_upload_bytes: 10485760')
  })

  it('配置没取到时那一格如实说空，不渲染一排 undefined', async () => {
    fx.spies.getConfig.mockRejectedValue(new Error('boom'))

    render(<Settings />)

    await waitFor(() => expect(screen.getByText(/还没取到配置/)).toBeInTheDocument())
    expect(document.body.textContent).not.toContain('undefined')
  })

  it('卡上写明四节可写但本页无控件、两节根本不在写入清单里', async () => {
    fx.spies.getConfig.mockResolvedValue(CONFIG)

    render(<Settings />)

    await waitFor(() => expect(screen.getByText('这些键本页只能看、不能改')).toBeInTheDocument())
    const text = document.body.textContent ?? ''
    expect(text).toContain('在 POST /api/config 的写入清单里，但本页没给它们做输入控件')
    expect(text).toContain('versioning / visualization 两节根本不在写入清单里')
    expect(text).toContain('后端往响应里加一节会自动出现在这里')
  })
})

describe('Settings 保存回执卡（L237 补渲染）', () => {
  const save = async () => {
    // 按钮带 SaveOutlined，可访问名实测是 "save保存配置"，所以这里只能按子串匹配
    fireEvent.click(screen.getByRole('button', { name: /保存配置/ }))
    await waitFor(() => expect(fx.spies.updateConfig).toHaveBeenCalled())
  }

  it('没保存过时那一格说「还没保存过」，不预先编一份回执', async () => {
    fx.spies.getConfig.mockResolvedValue(CONFIG)

    render(<Settings />)

    await waitFor(() => expect(screen.getByText('还没保存过')).toBeInTheDocument())
    expect(screen.queryByText(/ignored_keys:/)).not.toBeInTheDocument()
  })

  it('后端回了 ignored_keys 时逐条列出，并显示后端自己那句 message', async () => {
    fx.spies.getConfig.mockResolvedValue(CONFIG)
    fx.spies.updateConfig.mockResolvedValue({
      success: true,
      message: '配置已保存（2 个键被忽略）',
      ignored_keys: ['web', 'augmentation.variants_per_item'],
    })

    render(<Settings />)
    await waitFor(() => expect(screen.getByText('上次保存的回执')).toBeInTheDocument())
    await save()

    await waitFor(() => expect(screen.getByText('配置已保存（2 个键被忽略）')).toBeInTheDocument())
    const text = document.body.textContent ?? ''
    expect(text).toContain('ignored_keys: 2 个')
    expect(text).toContain('web')
    expect(text).toContain('augmentation.variants_per_item')
    // 两种形状的区别写在卡上：有没有那个点
    expect(text).toContain('按有没有那个点就能分开看')
  })

  it('ignored_keys 为空那一档说「0 个」，不编造键', async () => {
    fx.spies.getConfig.mockResolvedValue(CONFIG)
    fx.spies.updateConfig.mockResolvedValue({
      success: true,
      message: '配置已保存到 config.yaml',
      ignored_keys: [],
    })

    render(<Settings />)
    await waitFor(() => expect(screen.getByText('上次保存的回执')).toBeInTheDocument())
    await save()

    await waitFor(() => expect(screen.getByText('ignored_keys: 0 个')).toBeInTheDocument())
  })

  it('后端没回 ignored_keys 这一键时也不炸（契约里它非必填）', async () => {
    fx.spies.getConfig.mockResolvedValue(CONFIG)
    fx.spies.updateConfig.mockResolvedValue({ success: true, message: '配置已保存' })

    render(<Settings />)
    await waitFor(() => expect(screen.getByText('上次保存的回执')).toBeInTheDocument())
    await save()

    await waitFor(() => expect(screen.getByText('ignored_keys: 0 个')).toBeInTheDocument())
  })

  it('卡上写明 success 恒为 true、落没落盘看 message 与 ignored_keys', async () => {
    fx.spies.getConfig.mockResolvedValue(CONFIG)
    fx.spies.updateConfig.mockResolvedValue({ success: true, message: '配置已保存' })

    render(<Settings />)
    await waitFor(() => expect(screen.getByText('上次保存的回执')).toBeInTheDocument())
    // 这句话只住在「已保存」那一档的 description 里：没保存过时页面说的是「还没保存过」，
    // 不预先编一份回执，也就无从谈起「success 恒为 true」这件事
    await save()

    await waitFor(() =>
      expect(
        screen.getByText(/success 两边都是 true（它只表示「请求被正常处理」）/),
      ).toBeInTheDocument(),
    )
  })
})
