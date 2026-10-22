import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://localhost:8000',
        changeOrigin: true
      }
    }
  },
  build: {
    outDir: 'dist',
    // 1200 kB 而非默认 500 kB：本项目的两个 vendor chunk 是**第三方库本体**
    // （antd-vendor ≈1055 kB、echarts-vendor ≈1134 kB，均为未压缩体积），
    // 应用自身代码的最大 chunk 只有 ≈59 kB。阈值定在"刚好覆盖这两个已测量
    // 的 vendor 体积"上，而不是无脑放大：一旦应用代码或新引入的库把 chunk
    // 顶到 1200 kB 以上，构建仍会告警。
    chunkSizeWarningLimit: 1200,
    rollupOptions: {
      output: {
        // ⚠️ Vite 8（Rolldown 构建）只接受函数形式的 manualChunks，
        // Rollup 的对象映射 { 'x-vendor': [...] } 会抛 "manualChunks is not a function"。
        // 语义不变：antd/echarts/react 三组各自成 vendor chunk。
        manualChunks: (id: string) => {
          if (id.includes('/node_modules/antd/') || id.includes('/node_modules/@ant-design/')) {
            return 'antd-vendor'
          }
          if (id.includes('/node_modules/echarts')) return 'echarts-vendor'
          if (
            id.includes('/node_modules/react/') ||
            id.includes('/node_modules/react-dom/') ||
            id.includes('/node_modules/react-router-dom/')
          ) {
            return 'react-vendor'
          }
          return null
        }
      }
    }
  }
})
