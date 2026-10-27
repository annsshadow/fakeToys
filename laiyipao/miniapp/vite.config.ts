import { defineConfig } from 'vite'
import uni from '@dcloudio/vite-plugin-uni'

// vite 声明值必须与 pnpm-workspace.yaml 里 override 的值一致（当前 6.4.4），
// 见 GAME_DESIGN 目录说明。
// 历史：@dcloudio/vite-plugin-uni 的 peer 是精确值 5.2.8，2026-10-22 起为了修
// 15 条 vite advisory 用 override 顶到 6.4.4（5.4.x 线没有后续修复版本）。
// 2026-10-27 起 package.json 的声明值也改成 6.4.4 —— 声明 5.2.8、实际装 6.4.4
// 会让 dependabot 按声明值反复报已修的漏洞（一次 20 条）。
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
