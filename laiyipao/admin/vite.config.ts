import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'

// ⚠️ API 地址：生产与开发同源，靠 proxy 反代，不必在构建时硬编码
const API_TARGET = process.env.VITE_API_TARGET || 'http://127.0.0.1:8080'

export default defineConfig({
  // ⚠️ 顺序有讲究：Components 必须排在 vue() **之后** ——
  // 它靠分析编译后的 render 函数来收集模板里用到的组件，
  // 排在前面就什么也收集不到（表现为「构建成功但组件全丢」）。
  plugins: [
    vue(),
    Components({
      // 按需引入：只打进模板里**真正用到**的 el-* 组件。
      //
      // 为什么做：改之前 `app.use(ElementPlus)` 整包注册，
      // element-plus 一个 chunk 就是 **901 KB**（占产物 1981 KB 的 45%）。
      // 后台是运营每天要开的页面，这个体积直接等于首屏等待。
      //
      // 依赖（devDependencies 里早就装了，**但一直没接进配置**）：
      // unplugin-vue-components + ElementPlusResolver。
      resolvers: [ElementPlusResolver({ importStyle: 'css' })],
      // 生成 src/components.d.ts，让 vue-tsc 认识这些全局组件
      dts: 'src/components.d.ts',
    }),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  server: {
    port: 5175,
    host: '127.0.0.1',
    proxy: {
      '/api': { target: API_TARGET, changeOrigin: true },
    },
  },
  build: {
    chunkSizeWarningLimit: 1500,
    rollupOptions: {
      output: {
        // ⚠️ Vite 8 是 Rolldown 构建，manualChunks 只接受函数形式
        // （Rollup 的对象映射 { vue: [...] } 在 Rolldown 类型里已不支持，
        //  vue-tsc 报 TS2769）。语义与旧对象形式一致：
        // vue 组 = vue/vue-router/pinia/@vue 框架层，charts 组 = echarts。
        // ⚠️ 这一行是「整包」的根因：手写 manualChunks 把整个
        // element-plus 强行圈成一个 chunk，**绕过了按需引入的裁剪**。
        // 改成按需之后不能再这么圈，否则前面的优化全部作废。
        // 组件由 resolver 自动 import，交给打包器自行分块即可（element-plus 不进组）。
        manualChunks: (moduleId) => {
          if (moduleId.includes('/echarts/')) return 'charts'
          if (
            moduleId.includes('/vue/') ||
            moduleId.includes('/vue-router/') ||
            moduleId.includes('/pinia/') ||
            moduleId.includes('/@vue/')
          ) {
            return 'vue'
          }
          return null
        },
      },
    },
  },
})
