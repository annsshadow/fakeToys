import { resolve } from 'node:path'
import vue from '@vitejs/plugin-vue'
import UnoCSS from 'unocss/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [vue(), UnoCSS()],
  resolve: {
    alias: {
      '@oa4rust/sdk': resolve(__dirname, '../sdk/src'),
    },
  },
  build: {
    lib: {
      entry: 'src/index.ts',
      name: 'Oa4RustUi',
      formats: ['es'],
      fileName: () => 'index.js',
    },
    rollupOptions: {
      external: ['vue', '@oa4rust/sdk', 'naive-ui'],
      output: {
        globals: {
          vue: 'Vue',
          '@oa4rust/sdk': 'Oa4RustSdk',
          'naive-ui': 'NaiveUI',
        },
      },
    },
  },
})
