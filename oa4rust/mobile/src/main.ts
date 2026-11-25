// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { createPinia } from 'pinia'
import { createSSRApp } from 'vue'
import { setApiBase } from '@/services/http'
import App from './App.vue'

/**
 * R537：H5 表单控件可访问名透传到 AT 实际聚焦的原生控件（WCAG 2.1 SC 4.1.2 / 3.3.2）。
 * uni-app H5 把 <input>/<textarea>/<picker> 编译为宿主 <uni-*> aria-label=X 包裹内层原生控件；
 * aria-label 只落宿主（AX role=generic, focusable=false），AT 聚焦播报的内层 textbox 名仍为 null。
 * 用 MutationObserver 把宿主 aria-label 透传到内层可聚焦原生控件；R535 宿主层标记保留作 label 来源。
 * 仅写 aria-label 属性，不改 v-model / 事件 / 结构 / 样式，业务零改。
 */
function installH5NativeInputNameA11y(): void {
  const HOST_SELECTOR = 'uni-input, uni-textarea, uni-picker'
  const NATIVE_SELECTOR = 'input, textarea'

  function labelHost(host: Element): void {
    const label = host.getAttribute('aria-label')
    if (!label) return
    host.querySelectorAll(NATIVE_SELECTOR).forEach((native) => {
      if (native.getAttribute('aria-label') !== label) native.setAttribute('aria-label', label)
    })
  }

  function labelFromNative(native: Element): void {
    if (native.hasAttribute('aria-label')) return
    const host = native.closest(HOST_SELECTOR)
    if (!host) return
    const label = host.getAttribute('aria-label')
    if (label) native.setAttribute('aria-label', label)
  }

  function handleAdded(node: Node): void {
    if (node.nodeType !== 1) return
    const el = node as Element
    if (el.matches?.(HOST_SELECTOR)) labelHost(el)
    if (el.matches?.(NATIVE_SELECTOR)) labelFromNative(el)
    el.querySelectorAll?.(HOST_SELECTOR).forEach(labelHost)
    el.querySelectorAll?.(NATIVE_SELECTOR).forEach(labelFromNative)
  }

  const root: Node | null = document.body ?? document.documentElement
  if (!root) return
  document.querySelectorAll(HOST_SELECTOR).forEach(labelHost)
  document.querySelectorAll(NATIVE_SELECTOR).forEach(labelFromNative)

  const observer = new MutationObserver((records) => {
    records.forEach((record) => {
      if (record.type === 'attributes') {
        const target = record.target
        if (target instanceof Element && target.matches?.(HOST_SELECTOR)) labelHost(target)
        return
      }
      record.addedNodes.forEach(handleAdded)
    })
  })
  observer.observe(root, {
    childList: true,
    subtree: true,
    attributes: true,
    attributeFilter: ['aria-label']
  })
  // 增强器随应用生命周期常驻；无需 dispose。
}

export function createApp() {
  const app = createSSRApp(App)
  // 全局兜底（与 desktop main.ts 同口径）：未捕获渲染/Promise 错误进 console，不静默吞
  app.config.errorHandler = (err, _instance, info) => {
    console.error('[unhandled]', info, err)
  }
  // H5 的事件处理器与异步回调不必经过 Vue：Promise 拒绝兜底防静默丢失；
  // 小程序/原生目标无 window，须守卫（uni-app SSR 创建阶段亦无）。
  if (typeof window !== 'undefined') {
    window.addEventListener('unhandledrejection', (event) => {
      console.error('[unhandled]', 'unhandledrejection', event.reason)
    })
    // R537：H5 表单控件可访问名透传到 AT 实际聚焦的原生控件（WCAG 2.1 SC4.1.2）。
    installH5NativeInputNameA11y()
  }
  app.use(createPinia())
  // 原生 App / 小程序目标：后端为跨域绝对地址（由 VITE_OA_API_BASE 注入）。
  // H5 目标：测试服挂在 testmall 子路径下，API 走 /oa4rust-api 前缀（nginx 反代去掉前缀到后端）。
  const base = import.meta.env.VITE_OA_API_BASE || '/oa4rust-api'
  if (base) setApiBase(base)
  return { app }
}
