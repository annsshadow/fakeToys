import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import UnoCSS from '@unocss/vite';
import { resolve } from 'path';

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
    proxy: {
      '/jaxrs': { target: 'http://localhost:3000', changeOrigin: true },
      '/ws': { target: 'ws://localhost:3000', ws: true },
      '/openapi': { target: 'http://localhost:3000', changeOrigin: true },
    },
  },
  build: {
    outDir: '../../dist/web',
    emptyOutDir: true,
    sourcemap: true,
    rollupOptions: {
      output: {
        manualChunks: {
          vue: ['vue', 'vue-router', 'pinia'],
          naive: ['naive-ui'],
          query: ['@tanstack/vue-query'],
        },
      },
    },
  },
});
