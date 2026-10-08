// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ApiClient } from './api'

/** 最小 XHR 桩：可手动触发 upload.progress / onload。 */
class FakeXHR {
  static instances: FakeXHR[] = []
  upload = { onprogress: null as ((e: { lengthComputable: boolean; loaded: number; total: number }) => void) | null }
  onload: (() => void) | null = null
  onerror: (() => void) | null = null
  ontimeout: (() => void) | null = null
  onreadystatechange: (() => void) | null = null
  withCredentials = false
  status = 200
  responseText = ''
  timeout = 0
  sent: FormData | null = null
  method = ''
  url = ''

  open(method: string, url: string) {
    this.method = method
    this.url = url
    FakeXHR.instances.push(this)
  }
  setRequestHeader() {}
  send(body: FormData | null) {
    this.sent = body
  }

  /** 测试驱动：模拟上传字节进度 */
  progress(loaded: number, total: number) {
    this.upload.onprogress?.({ lengthComputable: true, loaded, total })
  }
  /** 测试驱动：模拟响应完成 */
  complete(status: number, responseText: string) {
    this.status = status
    this.responseText = responseText
    this.onload?.()
  }
}

let client: ApiClient

beforeEach(() => {
  FakeXHR.instances = []
  vi.stubGlobal('window', { location: { origin: 'https://web.example.test' } })
  vi.stubGlobal('XMLHttpRequest', FakeXHR as unknown as typeof XMLHttpRequest)
  client = new ApiClient()
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('uploadWithProgress（XHR 真实上传进度）', () => {
  it('上报 lengthComputable 进度百分比并解析 JSON 响应', async () => {
    const percents: number[] = []
    const p = client.uploadWithProgress('/api/file/upload', new FormData(), (pc) => percents.push(pc))
    const xhr = FakeXHR.instances[0]
    expect(xhr.method).toBe('POST')
    expect(xhr.url).toBe('https://web.example.test/api/file/upload')
    expect(xhr.withCredentials).toBe(true)

    xhr.progress(25, 100)
    xhr.progress(100, 100)
    xhr.complete(200, JSON.stringify({ success: true, data: 'ok' }))
    const resp = await p
    expect(resp.data).toBe('ok')
    expect(percents).toEqual([25, 100])
  })

  it('非 2xx 映射为 ApiError（携带状态码）', async () => {
    const p = client.uploadWithProgress('/api/file/upload', new FormData())
    const xhr = FakeXHR.instances[0]
    xhr.complete(500, 'server error')
    await expect(p).rejects.toMatchObject({ status: 500 })
  })

  it('不带进度回调也可用（onProgress 可选）', async () => {
    const p = client.uploadWithProgress('/api/file/upload', new FormData())
    const xhr = FakeXHR.instances[0]
    xhr.progress(50, 100) // 无回调时不得抛错
    xhr.complete(200, JSON.stringify({ success: true, data: 1 }))
    await expect(p).resolves.toMatchObject({ data: 1 })
  })
})
