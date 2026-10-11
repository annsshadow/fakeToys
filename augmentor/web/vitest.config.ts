import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

/**
 * 前端测试配置
 *
 * 与 `vite.config.ts` 分开：`vitest.config.ts` 一旦存在，vitest 就只读它、
 * 不再合并 `vite.config.ts`，所以这里必须自己带上 react 插件（否则 .tsx 无法解析）。
 * 分开的好处是构建配置保持干净，测试相关改动不会碰到产物构建。
 */
export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    include: ['src/**/*.{test,spec}.{ts,tsx}'],
    // 文件级并行度收敛到 2：jsdom + antd 的挂载渲染是重 CPU 负载，全并行时
    // 争用会让挂载期的 `waitFor` 偶发超时（单文件必过）。2026-10-11 CI 实测：
    // 4 核 runner 上 20 文件并行，DataManagement.transform 的转换点击用例
    // 20s 内没观察到调用（受限容器更糟：worker 启动超时、总耗时 257s vs
    // 本机 37s）。抬 testTimeout 治标（5s→20s 已经抬过一次），并行度才是根因。
    // GitHub runner 是 4 核，留一半给主进程与 I/O；本机多核 Worker 会被限到 2，
    // 总墙钟约 +30s（37s→68s），换 CI 上的确定性。全串行（fileParallelism:
    // false）更保险但容器实测超过 12 分钟，不进 CI 闸门。
    maxWorkers: 2,
    minWorkers: 1,
    // 页面级组件测试（L104–L113）落地后，测试文件从 2 份增至 13 份。jsdom + antd
    // 的挂载渲染很重，CPU 紧张时挂载期的 `waitFor` 可能超过默认的 5s
    // testTimeout（单文件跑必过、多文件并行才偶发）。20s 给足挂载头寸，
    // 不掩盖真失败（真失败是断言不符、秒级即报，不会拖到 20s）。
    testTimeout: 20000,
    hookTimeout: 20000,
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html'],
      // 只统计服务层：组件层目前没有测试，纳入统计会得到一个虚低的全项目数字
      include: ['src/services/**/*.ts'],
      exclude: ['src/services/**/*.{test,spec}.ts'],
      // 规划 T1.9 的验收门槛
      thresholds: {
        lines: 70,
        functions: 70,
        branches: 70,
        statements: 70,
      },
    },
  },
})
