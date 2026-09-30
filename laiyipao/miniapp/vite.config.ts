import { defineConfig } from 'vite'
import uni from '@dcloudio/vite-plugin-uni'

// vite 必须锁 5.2.8（@dcloudio/vite-plugin-uni 的 peer 硬锁），见 GAME_DESIGN 目录说明
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
