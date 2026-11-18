// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 测试环境全局垫片。
 * jsdom 没有 ResizeObserver，Element Plus 的表格/抽屉等组件会用到它。
 */
class ResizeObserverStub {
  observe() {}
  unobserve() {}
  disconnect() {}
}
;(globalThis as unknown as Record<string, unknown>).ResizeObserver = ResizeObserverStub

// jsdom 的 Blob 没有标准 text()，且 FileReader/Response 是宏任务，flushPromises 等不到。
// 用带同步 text() 的子类替换全局 Blob，text() 返回微任务 Promise，测试可稳定断言。
if (typeof Blob !== 'undefined') {
  class BlobWithText extends Blob {
    private readonly parts: BlobPart[]
    constructor(parts: BlobPart[] = [], opts?: BlobPropertyBag) {
      super(parts, opts)
      this.parts = parts
    }
    text(): Promise<string> {
      return Promise.resolve(this.parts.map((p) => String(p)).join(''))
    }
  }
  ;(globalThis as unknown as Record<string, unknown>).Blob = BlobWithText
}
