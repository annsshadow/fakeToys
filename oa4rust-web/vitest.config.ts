import { resolve } from 'node:path'
import { defineConfig } from 'vitest/config'

export default defineConfig({
  resolve: {
    alias: {
      '@oa4rust/sdk': resolve(__dirname, 'packages/sdk/src'),
      '@oa4rust/ui': resolve(__dirname, 'packages/ui/src'),
    },
  },
  test: {
    globals: true,
    environment: 'node',
    include: ['**/*.test.ts'],
    // mobile 使用独立 vitest 配置（apps/mobile/vitest.config.ts，含 @ → src 别名），
    // 见根 package.json 的 test:mobile 脚本。
    exclude: ['node_modules', 'dist', '**/node_modules/**', 'apps/mobile/**'],
  },
})
