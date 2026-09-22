// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/// <reference types="@dcloudio/types" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'

  const component: DefineComponent<Record<string, unknown>, Record<string, unknown>, unknown>
  export default component
}

interface ImportMetaEnv {
  /** 后端 API 绝对地址（原生 App / 小程序目标使用；H5 走同源代理，留空即可）。 */
  readonly VITE_OA_API_BASE?: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
