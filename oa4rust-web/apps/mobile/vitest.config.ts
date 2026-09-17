import { resolve } from 'node:path'
import { defineConfig } from 'vitest/config'

/**
 * 移动端独立 vitest 配置：mobile 的 `@` 别名指向 apps/mobile/src（uni-app 约定），
 * 与根 vitest.config.ts（覆盖 desktop / packages / contracts）互不干扰。
 * 运行：pnpm --filter @oa4rust/mobile test（根 test:mobile 脚本）。
 */
export default defineConfig({
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
      '@oa4rust/sdk': resolve(__dirname, '../../packages/sdk/src'),
    },
  },
  test: {
    globals: true,
    environment: 'node',
    include: ['src/**/*.test.ts'],
    exclude: ['node_modules', 'dist', '**/node_modules/**'],
  },
})
