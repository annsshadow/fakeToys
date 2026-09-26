import { defineConfig } from 'vitest/config'
import { fileURLToPath, URL } from 'node:url'

// 独立配置：game/ 是纯 TS 内核，不依赖 uni 运行时，
// 也不需要 vite:5.2.8 的锁定（那只对 uni 构建有意义）。
export default defineConfig({
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
      // 契约向量的绝对路径别名。放在 server/ 之外的相对 import 会被 vite 拒绝，
      // 用别名可以绕开 root 限制，避免每个测试文件都写 '../..'。
      '@vectors': fileURLToPath(new URL('../server/testdata', import.meta.url)),
    },
  },
  test: {
    include: ['src/**/*.test.ts'],
    environment: 'node',
    globals: true,
  },
  // 契约向量在 server/testdata/ 下，物理路径必须解析到那里。
  // vite 的 root 是 miniapp/，'..' 恰好是 laiyipao/，再拼 server/testdata 正确。
  server: {
    fs: {
      allow: ['..'],
    },
  },
  optimizeDeps: {
    exclude: [],
  },
})
