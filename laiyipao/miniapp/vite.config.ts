import { defineConfig } from 'vite'
import uni from '@dcloudio/vite-plugin-uni'

// vite 必须锁 5.2.8（@dcloudio/vite-plugin-uni 的 peer 硬锁），见 GAME_DESIGN 目录说明
// ⚠️ 这条约束有可执行守卫：src/test/toolchain_pin.test.ts。dependabot 在
// 2026-09-30 / 2026-10-08 两次自动升级（5.2.8→6.4.3→8.3.4）把 type-check
// （TS2307，vite 8 无根 types 字段而 TS4.9 不读 exports）、build:h5
// （rolldown 解析不了 pinia 的 vue-demi）、npm install（ERESOLVE）同时打坏。
// 依赖机器人再动 package.json 的 vite/vitest 两行时，那个测试会先红。
export default defineConfig({
  plugins: [uni()],
  build: {
    chunkSizeWarningLimit: 1024,
    /**
     * target 必须是 es2020 或更高 —— 定点整数数学全靠 BigInt 字面量
     * （6364136223846793005n），es2015 下 esbuild 会直接报
     * "Big integer literals are not available" 并中止构建。
     *
     * 微信小程序基础库 2.32+ / iOS 14+ 均支持 BigInt，
     * 但如果需要兼容更老的机型，得改用 BigInt 构造函数而非字面量 ——
     * 那会牺牲可读性，先不做这个妥协。
     */
    target: 'es2020',
  },
  esbuild: {
    target: 'es2020',
  },
})
