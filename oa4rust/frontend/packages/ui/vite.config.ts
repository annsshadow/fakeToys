// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { resolve } from 'node:path'
import UnoCSS from '@unocss/vite'
import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [vue(), UnoCSS()],
  resolve: {
    alias: {
      // import.meta.dirname：Vite 的 configLoader 'native' 不支持 __dirname（会告警）。
      '@oa4rust/sdk': resolve(import.meta.dirname, '../sdk/src'),
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
