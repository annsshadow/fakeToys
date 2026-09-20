import UnoCSS from '@unocss/vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [vue(), UnoCSS()],
  resolve: {
    alias: {
      '@oa4rust/sdk': resolve(__dirname, '../../packages/sdk/src'),
      '@oa4rust/ui': resolve(__dirname, '../../packages/ui/src'),
      '@oa4rust/locales': resolve(__dirname, '../../packages/locales/src'),
    },
  },
  server: {
    port: 5173,
    host: true,
    // Inject Content-Security-Policy-Report-Only header for safe debugging.
    // When ready to enforce, switch to 'Content-Security-Policy' and tighten.
    headers: {
      'Content-Security-Policy-Report-Only': [
        "default-src 'self'",
        "script-src 'self' 'report-sample'",
        "style-src 'self' 'unsafe-inline'",
        "img-src 'self' data: blob:",
        "connect-src 'self' ws://localhost:* wss://* *",
        "font-src 'self'",
        "object-src 'none'",
        "frame-src 'none'",
        "base-uri 'self'",
        "form-action 'self'",
        'report-uri /csp-report',
        'report-to csp-endpoint',
      ].join('; '),
      'Report-To': JSON.stringify({
        group: 'csp-endpoint',
        max_age: 86400,
        endpoints: [{ url: '/csp-report' }],
      }),
    },
    proxy: {
      '/api': { target: 'http://localhost:3000', changeOrigin: true },
      '/ws': { target: 'ws://localhost:3000', ws: true },
      '/openapi': { target: 'http://localhost:3000', changeOrigin: true },
    },
  },
  build: {
    outDir: '../../dist/web',
    emptyOutDir: true,
    sourcemap: false,
    chunkSizeWarningLimit: 1000,
    rollupOptions: {
      output: {
        // 手动分包：按第三方包分组（函数形态；对象形态在当前 rollup 类型下被判为只接受函数）。
        // 分组结果与原对象写法等价。
        manualChunks(id: string) {
          if (!id.includes('/node_modules/')) return undefined
          const groups: Record<string, string[]> = {
            vue: ['vue', 'vue-router', 'pinia'],
            naive: ['naive-ui'],
            query: ['@tanstack/vue-query'],
            echarts: ['echarts'],
            codemirror: [
              'codemirror',
              '@codemirror/state',
              '@codemirror/view',
              '@codemirror/language',
              '@codemirror/autocomplete',
              '@codemirror/commands',
              '@codemirror/lang-sql',
            ],
          }
          for (const [chunk, pkgs] of Object.entries(groups)) {
            if (pkgs.some((pkg) => id.includes(`/node_modules/${pkg}/`))) return chunk
          }
          return undefined
        },
      },
    },
  },
  optimizeDeps: {
    include: ['vue', 'vue-router', 'pinia', '@tanstack/vue-query', 'naive-ui'],
  },
})
