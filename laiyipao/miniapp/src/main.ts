// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { createSSRApp } from 'vue'
import App from './App.vue'
import { createPinia } from 'pinia'

export function createApp() {
  const app = createSSRApp(App)
  app.use(createPinia())
  return { app }
}
