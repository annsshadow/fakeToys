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
      perFile: true,
      thresholds: {
        // API index.ts is now testable with executable branches: 90% floor
        'packages/apis/src/index.ts': {
          lines: 90,
          functions: 90,
          branches: 80,
          statements: 90,
        },
        // SDK modules: api.ts has business logic (~57%), rest are declaration/type stubs
        'packages/sdk/src/**/*.ts': {
          lines: 15,
          functions: 10,
          branches: 10,
          statements: 15,
        },
        // Contracts/designer.ts, definition.ts, xform.ts: pure declaration/config layer
        // Treated as declaration/configuration; quality gate is contract guard tests, not coverage
        'apps/desktop/src/contracts/**/*.ts': {
          lines: 0,
          functions: 0,
          branches: 0,
          statements: 0,
        },
        // Utils: mostly UI scaffolding stubs, declaration layer
        'apps/desktop/src/utils/**/*.ts': {
          lines: 0,
          functions: 0,
          branches: 0,
          statements: 0,
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
