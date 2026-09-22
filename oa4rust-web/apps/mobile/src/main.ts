// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { createPinia } from 'pinia'
import { createSSRApp } from 'vue'
import { setApiBase } from '@/services/http'
import App from './App.vue'

export function createApp() {
  const app = createSSRApp(App)
  app.use(createPinia())
  // 原生 App / 小程序目标：后端为跨域绝对地址；H5 目标经 dev-server 同源代理，无需设置。
  const base = import.meta.env.VITE_OA_API_BASE
  if (base) setApiBase(base)
  return { app }
}
