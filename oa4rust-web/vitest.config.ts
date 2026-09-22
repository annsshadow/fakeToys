// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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
    exclude: ['node_modules', 'dist', '**/node_modules/**', 'apps/mobile/**'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html', 'json', 'lcov'],
      reportsDirectory: './coverage',
      // Coverage for files that can be tested with Node environment
      include: [
        'packages/sdk/src/**/*.ts',
        'packages/apis/src/index.ts',
        'apps/desktop/src/views/**/*.ts',
        'apps/desktop/src/contracts/**/*.ts',
        'apps/desktop/src/utils/**/*.ts',
      ],
      exclude: [
        '**/*.test.ts',
        '**/node_modules/**',
        '**/vite-env.d.ts',
        '**/vite.config.ts',
        '**/unocss.config.ts',
        'packages/apis/src/export.ts',
        'packages/apis/src/locales/**',
        'packages/apis/src/api-coverage-generated.test.ts',
        'packages/apis/src/api-functional-tests.test.ts',
        'types.ts',
        'index.ts',
        'index.js',
      ],
      // Per-file thresholds
      // ── 覆盖率豁免策略（2026-09-18 审计，详见 oa4rust/docs/unit-test-coverage-exemptions.md）：
      //  - 有可执行分支的模块 → 设真实 floor，防止覆盖回退；
      //  - 结构不可测（生成器产物 / uni-app 入口）→ 显式 exclude 并注释理由；
      //  - 剩余未覆盖语句逐文件判定：可测的已补测，不可测的在此声明豁免。
      perFile: true,
      thresholds: {
        // 生成的一行 HTTP 包装器（62 个 API 模块）无可测分支：api-coverage-generated.test.ts
        // 已实跑每个方法触发 V8 覆盖，剩余 ~3% 未覆盖行是生成器模板的纯转发行，
        // 属结构不可测——维持 90% floor 即豁免线（低于此说明生成测试集坏了）。
        'packages/apis/src/index.ts': {
          lines: 90,
          functions: 90,
          branches: 80,
          statements: 90,
        },
        // SDK 模块 2026-09-18 实测 97-100%（app/widget/theme/router 已 100%）；
        // 80% floor 锁住已达成水平，防回退。session.ts 1 行（legacy storage catch）与
        // api.ts 个别超时分支属难触发路径，豁免在 audit 文档记录。
        'packages/sdk/src/**/*.ts': {
          lines: 80,
          functions: 80,
          branches: 50,
          statements: 80,
        },
        // contracts 三文件（designer/xform/process-definition）纯逻辑分支已补测
        // （designer.test.ts / xform.test.ts / xform.dom.test.ts / process-definition.test.ts）；
        // 70% floor 锁住可执行覆盖，声明/配置层（接口定义）不计入。
        'apps/desktop/src/contracts/**/*.ts': {
          lines: 70,
          functions: 70,
          branches: 60,
          statements: 70,
        },
        // utils：toast.ts（showToast 管道 + confirmMsg 全路径）与 sandbox.ts 已 90%+；
        // 80% floor 锁住，防回退。
        'apps/desktop/src/utils/**/*.ts': {
          lines: 80,
          functions: 80,
          branches: 50,
          statements: 80,
        },
        // Views: Vue single-file component logic
        'apps/desktop/src/views/**/*.ts': {
          lines: 50,
          functions: 50,
          branches: 45,
          statements: 50,
        },
      },
    },
  },
})

/**
 * Coverage strategy notes:
 *
 * 1. UNIT TEST COVERAGE (this config):
 *    - SDK modules: 75-100% (achieved)
 *    - Views: 62-100% (achieved for testable ones)
 *    - API declarations: ~20% (requires browser E2E for full coverage)
 *
 * 2. PLAYWRIGHT E2E COVERAGE (requires live backend):
 *    - 启用 COLLECT_COVERAGE=true
 *    - 当前E2E测试覆盖关键API路径（BBS、IM、工作流）
 *    - 通过 /app/ 路由覆盖大量前端代码
 *
 * 3. To enable Playwright coverage:
 *    COLLECT_COVERAGE=true npx playwright test --project=coverage-chromium
 *    输出到 coverage-e2e/lcov.info
 *
 * 4. RUST COVERAGE:
 *    - Run: cargo tarpaulin --workspace --out Lcov
 *    - Output: coverage-rust/lcov.info
 *    - Merge with JS coverage using lcov
 */
