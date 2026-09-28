import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vitest/config'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  test: {
    environment: 'jsdom',
    include: ['tests/**/*.test.ts'],
    setupFiles: ['tests/setup.ts'],
    // 覆盖率模式下全量并行时，element-plus 首次收集较慢，放宽默认 5s/10s
    testTimeout: 20000,
    hookTimeout: 40000,
    coverage: {
      provider: 'v8',
      include: ['src/**'],
      // 纯类型声明与纯样式无运行时代码，无可测语句
      exclude: ['src/env.d.ts', 'src/styles/**'],
      reporter: ['text'],
    },
  },
})
