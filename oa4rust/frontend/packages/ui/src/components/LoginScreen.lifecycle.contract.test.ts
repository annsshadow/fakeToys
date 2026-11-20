// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R195 UI 包事件监听器生命周期契约：登录页 resize 监听器必须成对注册/解绑。
 *
 * 缺陷证据：LoginScreen.vue 在 onMounted 里用「匿名内联箭头函数」注册 window
 * resize，而 onBeforeUnmount 只取消粒子动画帧（cancelAnimationFrame），从不解绑
 * resize。匿名函数没有引用，即便想解绑也做不到。LoginScreen 是路由组件（/login，
 * meta.requiresAuth:false），用户每次登出/会话过期再次进入都会再挂一个永久监听器，
 * 旧闭包捕获 canvasRef，长期积累造成内存泄漏与 resize 时多余回调。
 *
 * 现有 lifecycleAudit.test.ts（R142）虽然钉死了 add/removeEventListener 必须配对，
 * 但扫描根只有 apps/desktop/src 与 mobile/src，packages/ui 是盲区——本测试补上这块，
 * 专门锁住登录页这一处真实泄漏，防止回归。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const loginScreen = readFileSync(path.join(here, 'LoginScreen.vue'), 'utf8')

/** 截取 onBeforeUnmount 之后的内容，用于验证解绑确实发生在卸载钩子内。 */
function afterHook(name: string): string {
  const idx = loginScreen.indexOf(name)
  expect(idx, `未找到生命周期钩子 ${name}`).toBeGreaterThan(-1)
  return loginScreen.slice(idx)
}

describe('LoginScreen resize 监听器生命周期（事件监听泄漏防回归）', () => {
  it('resize 处理器必须是具名稳定引用，而非无法解绑的匿名内联箭头函数', () => {
    // 匿名 () => {...} 一旦注册就无法 removeEventListener —— 这是泄漏的根因。
    expect(loginScreen).not.toMatch(/addEventListener\(\s*['"]resize['"]\s*,\s*\(\)\s*=>/)
    expect(loginScreen).toMatch(/function handleResize\s*\(/)
  })

  it('onMounted 注册的 resize 监听器必须能在 onBeforeUnmount 中按同一引用解绑', () => {
    expect(loginScreen).toMatch(/addEventListener\(\s*['"]resize['"]\s*,\s*handleResize\s*\)/)
    const unmount = afterHook('onBeforeUnmount')
    expect(
      unmount,
      'onBeforeUnmount 内缺少 removeEventListener("resize", handleResize)：路由反复进入会累积监听器',
    ).toMatch(/removeEventListener\(\s*['"]resize['"]\s*,\s*handleResize\s*\)/)
  })

  it('注册与解绑使用同一个处理器标识符（避免解绑到不同函数导致无效）', () => {
    const addHandler = loginScreen.match(/addEventListener\(\s*['"]resize['"]\s*,\s*(\w+)/)?.[1]
    const unmount = afterHook('onBeforeUnmount')
    const removeHandler = unmount.match(/removeEventListener\(\s*['"]resize['"]\s*,\s*(\w+)/)?.[1]
    expect(addHandler).toBe('handleResize')
    expect(removeHandler).toBe(addHandler)
  })
})
