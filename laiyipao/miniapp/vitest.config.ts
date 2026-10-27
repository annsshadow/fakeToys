import { defineConfig, coverageConfigDefaults } from 'vitest/config'
import { fileURLToPath, URL } from 'node:url'
import vue from '@vitejs/plugin-vue'

// 独立配置：game/ 是纯 TS 内核，不依赖 uni 运行时，
// 也不需要 vite:5.2.8 的锁定（那只对 uni 构建有意义）。

// uni-app 模板标签（view/text/scroll-view...）在测试环境没有组件实现，
// 按自定义元素处理即可 —— 真实编译由 @dcloudio/vite-plugin-uni 负责，
// 测试只关心 <script setup> 里的页面逻辑。
const UNI_CUSTOM_TAGS = new Set([
  'view',
  'text',
  'scroll-view',
  'swiper',
  'swiper-item',
  'canvas',
  'input',
  'textarea',
  'image',
  'picker',
  'form',
  'button',
  'label',
  'navigator',
  'block',
  'checkbox',
  'radio',
  'switch',
  'slider',
  'progress',
  'rich-text',
  'video',
  'map',
  'web-view',
  'cover-view',
  'cover-image',
])

export default defineConfig({
  plugins: [
    vue({
      template: {
        compilerOptions: {
          isCustomElement: (tag) => UNI_CUSTOM_TAGS.has(tag),
        },
      },
    }),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
      // 契约向量的绝对路径别名。放在 server/ 之外的相对 import 会被 vite 拒绝，
      // 用别名可以绕开 root 限制，避免每个测试文件都写 '../..'。
      '@vectors': fileURLToPath(new URL('../server/testdata', import.meta.url)),
      // ⚠️ @dcloudio/uni-app 的生命周期钩子从 'vue' 导入内部 API `injectHook`，
      // 该导出在 vue 3.4 存在、vue 3.5（本仓库实际解析到的版本）已移除 ——
      // 真包一被调用就抛 `vue.injectHook is not a function`。
      // 测试用替身：把回调注册到组件实例上，由测试手动触发。
      '@dcloudio/uni-app': fileURLToPath(new URL('./src/test/uni-app-stub.ts', import.meta.url)),
    },
  },
  test: {
    include: ['src/**/*.test.ts'],
    environment: 'node',
    globals: true,
    // ⚠️ 默认 5s 超时对本项目是错的：`src/game/**` 的平衡性用例要跑完整 100 关
    // 战斗模拟，单条实测 10~40s（armor_shape 34s、elem_coef_diag 28s）。
    // 默认值下这些用例会**随机**超时（2026-10-27 实测 revive_extra_hp 在并发
    // 负载下红过一次，串行跑又绿）—— 这种「负载型假红」最坏的地方是它会顺带
    // 让覆盖率报告整份作废：vitest 在用例失败时跳过 coverage 输出，
    // 于是「覆盖率没数」和「真失败」被混成同一件事。
    // 放到 120s 仍然能抓住真的死循环（那些会远超 2 分钟）。
    testTimeout: 120000,
    hookTimeout: 60000,
    coverage: {
      // src/test/** 是测试基建（uni mock / 生命周期替身 / 夹具），不是产品代码。
      // src/game/types.ts 是纯类型声明（interface / type / 字面量联合），
      // 编译后不产生任何运行时代码 —— 覆盖率对它恒为 0%，纳入统计只会
      // 拉低总分而无法被任何测试"覆盖"，故按纯类型模块豁免。
      exclude: [...coverageConfigDefaults.exclude, 'src/test/**', 'src/game/types.ts'],
      // 覆盖率 ratchet（2026-10-27 建立）：实测 语句 94.35 / 分支 89.95 / 函数 93.32 /
      // 行 95.26，各留 ~1pt 余量（v8 计数随 Node 版本有零点几个百分点抖动）。
      // 注意 `src/game/**` 的平衡性用例要跑完整 100 关模拟，覆盖率跑一次约 1 分钟，
      // 所以本地验证时用它自己的 reportsDirectory（见 .gitignore 的 coverage-run*）。
      thresholds: {
        statements: 93,
        branches: 88,
        functions: 92,
        lines: 94,
      },
    },
  },
  // 契约向量在 server/testdata/ 下，物理路径必须解析到那里。
  // vite 的 root 是 miniapp/，'..' 恰好是 laiyipao/，再拼 server/testdata 正确。
  server: {
    fs: {
      allow: ['..'],
    },
  },
  optimizeDeps: {
    exclude: [],
  },
})
