import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vitest/config'
import vue from '@vitejs/plugin-vue'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'

export default defineConfig({
  // ⚠️ 顺序与 `vite.config.ts` 一致：Components 必须在 vue() 之后。
  //
  // 为什么测试也需要这个插件（原来这里只有 `vue()`）：
  // 之前 `main.ts` 里 `app.use(ElementPlus)` 让测试**不需要**解析器 ——
  // 整包注册已经把所有 `el-*` 挂到全局了。
  // 改成按需引入后 `main.ts` 不再整包注册，
  // 于是 `tests/bootstrap.test.ts`（它 import 的就是真实的 `src/main`）
  // 会因为 `el-config-provider` / `el-card` 未注册而**渲染不出东西**。
  //
  // 加了解析器之后，测试与构建走**同一套**组件解析规则 ——
  // 这比「构建按需、测试整包」更可靠：后者会让「测试通过但页面是白的」
  // 成为可能（测试里组件来自整包，构建里来自按需，两者不是一回事）。
  plugins: [
    vue(),
    Components({
      resolvers: [
        // ⚠️ `importStyle: false` —— 与 `vite.config.ts` 的 `css` **故意不同**。
        //
        // 打开样式引入会让 resolver 注入 `element-plus/theme-chalk/*.css`，
        // 而 vitest 的模块加载走 Node ESM，它不认裸 `.css`：
        //
        //   TypeError: Unknown file extension ".css"
        //     for node_modules/.../element-plus/theme-chalk/base.css
        //   code: 'ERR_UNKNOWN_FILE_EXTENSION'
        //
        // 结果是 **14 个测试文件整片失败**（不是断言红，是加载就炸）。
        //
        // 关掉是正确的：jsdom 里样式表**不影响任何断言**，
        // 我们断言的是文案、数字、DOM 结构。
        // 样式是否正确由「构建产物能出得来」+ 人工看页面负责，
        // 不用一个必然报错的导入把它变成测试的噪声。
        ElementPlusResolver({ importStyle: false }),
      ],
      // 测试环境不写 components.d.ts —— 那是给编辑器/vue-tsc 用的，
      // 与运行时无关，而且会在跑测试时产生文件写入。
      dts: false,
    }),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  test: {
    environment: 'jsdom',
    include: ['tests/**/*.test.ts'],
    setupFiles: ['tests/setup.ts'],
    // 整包注册去掉后，组件是逐个真实 import 的，收集阶段略慢
    testTimeout: 20000,
    hookTimeout: 40000,
    coverage: {
      provider: 'v8',
      include: ['src/**'],
      // 生成型声明文件与样式，忽略掉免得算进覆盖率分母
      exclude: ['src/env.d.ts', 'src/styles/**', 'src/components.d.ts'],
      reporter: ['text'],
    },
  },
})
