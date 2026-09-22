// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { createRequire } from 'node:module'
import { defineConfig, type Plugin } from 'vite'

// @dcloudio/vite-plugin-uni 为 CJS 包（main: dist/index.js），其默认导出是插件工厂函数。
// 直接 `import uni from '…'` 在部分 ESM/CJS interop 下会得到整个 exports 对象（"uni is not a function"），
// 故用 createRequire 显式取 .default，保证跨环境稳定。
const require = createRequire(import.meta.url)
// eslint-disable-next-line @typescript-eslint/no-var-requires
const uni = (
  require('@dcloudio/vite-plugin-uni') as {
    default: (options?: Record<string, unknown>) => Plugin[]
  }
).default

// OA4Rust 移动端（uni-app）构建配置。
// H5 目标经本地 dev server 代理 /api、/ws 到 oa4rust 后端（默认 :3000）。
// 原生 App / 小程序目标通过 setApiBase 指向后端绝对地址（见 src/main.ts）。
export default defineConfig({
  plugins: uni(),
  server: {
    port: 5174,
    host: true,
    proxy: {
      '/api': { target: 'http://localhost:3000', changeOrigin: true },
      '/ws': { target: 'ws://localhost:3000', ws: true },
    },
  },
  build: {
    sourcemap: false,
  },
})
