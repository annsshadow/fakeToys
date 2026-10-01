// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { createPinia } from 'pinia'
import { createSSRApp } from 'vue'
import { setApiBase } from '@/services/http'
import App from './App.vue'

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
  }
  app.use(createPinia())
  // 原生 App / 小程序目标：后端为跨域绝对地址；H5 目标经 dev-server 同源代理，无需设置。
  const base = import.meta.env.VITE_OA_API_BASE
  if (base) setApiBase(base)
  return { app }
}
