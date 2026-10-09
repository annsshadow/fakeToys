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
    chunkSizeWarningLimit: 800,
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
