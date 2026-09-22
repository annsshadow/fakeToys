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
        manualChunks: {
          'antd-vendor': ['antd', '@ant-design/icons'],
          'echarts-vendor': ['echarts', 'echarts-for-react'],
          'react-vendor': ['react', 'react-dom', 'react-router-dom']
        }
      }
    }
  }
})
