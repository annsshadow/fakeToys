import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

/**
 * 前端测试配置
 *
 * 与 `vite.config.ts` 分开：`vitest.config.ts` 一旦存在，vitest 就只读它、
 * 不再合并 `vite.config.ts`，所以这里必须自己带上 react 插件（否则 .tsx 无法解析）。
 * 分开的好处是构建配置保持干净，测试相关改动不会碰到产物构建。
 */
export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    include: ['src/**/*.{test,spec}.{ts,tsx}'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html'],
      // 只统计服务层：组件层目前没有测试，纳入统计会得到一个虚低的全项目数字
      include: ['src/services/**/*.ts'],
      exclude: ['src/services/**/*.{test,spec}.ts'],
      // 规划 T1.9 的验收门槛
      thresholds: {
        lines: 70,
        functions: 70,
        branches: 70,
        statements: 70,
      },
    },
  },
})
