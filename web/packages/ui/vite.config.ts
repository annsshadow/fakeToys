import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import UnoCSS from 'unocss/vite';

export default defineConfig({
  plugins: [vue(), UnoCSS()],
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
});
