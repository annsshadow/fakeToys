/// <reference types="vite/client" />
/// <reference types="@dcloudio/types" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare const __UNI_PLATFORM__: string

/**
 * VITE_API_BASE 覆盖后端地址。
 *
 * ⚠️ 这一行是本轮（安全审计）加的。此前 `api/client.ts` 里是一个
 * 写死的回环地址字面量，既没有覆写点、构建链路里也没有任何地方读它
 * —— 于是「改地址」只能回去改源码，而那条路径上没有任何东西
 * 阻止人把 http:// 原样带到公网域名上。
 *
 * 非回环主机 + 明文 HTTP 会在模块加载时直接抛错（见 resolveBaseURL）。
 */
interface ImportMetaEnv {
  readonly VITE_API_BASE?: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
