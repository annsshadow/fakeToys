// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<Record<string, unknown>, Record<string, unknown>, any>
  export default component
}

interface ImportMetaEnv {
  readonly VITE_API_BASE?: string
  /** t33：显式开启后才在顶栏显示完整 API 地址（排障用，默认关闭）。 */
  readonly VITE_SHOW_API_HOST?: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
