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
