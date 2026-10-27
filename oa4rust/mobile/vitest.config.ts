// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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
      '@oa4rust/sdk': resolve(__dirname, '../frontend/packages/sdk/src'),
    },
  },
  test: {
    globals: true,
    environment: 'node',
    include: ['src/**/*.test.ts'],
    exclude: ['node_modules', 'dist', '**/node_modules/**'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html', 'json'],
      reportsDirectory: './coverage-mobile',
      include: ['src/**/*.ts'],
      // 结构豁免（2026-09-18 审计）：main.ts 是 uni-app 应用入口，模块顶层即
      // createApp/挂载，node 环境无法执行；其行为由 H5 E2E 实跑覆盖（见
      // oa4rust-mobile-gui-verified 记忆条目）。其余文件不设豁免。
      //
      // 2026-10-27 补充两条同类豁免：
      //   - `src/env.d.ts` 是纯类型声明（编译后无运行时代码），覆盖率恒 0%；
      //   - `src/devtools-stub.ts` 只在 devtools 构建分支被引用，测试环境不加载。
      // 两者留在分母里只会按比例拉低总分，而任何用例都不可能「覆盖」它们。
      exclude: ['src/main.ts', 'src/env.d.ts', 'src/devtools-stub.ts'],
    },
  },
})
