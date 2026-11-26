// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { createSSRApp, type Directive } from 'vue'
import App from './App.vue'
import { createPinia } from 'pinia'

// R61 修复：uni-app H5 原生 input.uni-input-input 的属性白名单丢弃 aria-label，
// 全局指令在 mounted/updated 把可访问名桥接到原生 input（只改名，不碰值/事件/class）。
function applyAriaNative(el: HTMLElement, label: string) {
  const name = label || el.getAttribute('aria-label')
  const native: Element | null =
    el.tagName === 'INPUT' && el.classList.contains('uni-input-input')
      ? el
      : el.querySelector('input.uni-input-input')
  if (native && name) native.setAttribute('aria-label', name)
}

const ariaNative: Directive<HTMLElement, string> = {
  mounted(el, binding) { applyAriaNative(el, binding.value) },
  updated(el, binding) { applyAriaNative(el, binding.value) },
}

export function createApp() {
  const app = createSSRApp(App)
  app.use(createPinia())
  app.directive('aria-native', ariaNative)
  return { app }
}
