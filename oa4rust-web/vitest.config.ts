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
    exclude: ['node_modules', 'dist', '**/node_modules/**'],
  },
})
