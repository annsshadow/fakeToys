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
    // 2026-10-22 第五轮改用**全串行**（`fileParallelism: false`）：
    // 2 worker 并行下，4 份重 antd 的 `DataManagement.*` 文件两两配对互抢 CPU，
    // React 19 的 passive-effect 提交被 scheduler 延迟，`fireEvent` 后的状态没在
    // 下一次 `click` 前 commit ⇒ 不同文件轮流 flake（A322 → RAG），且拆环境后残留的
    // setImmediate 读 `window` 抛错。串行后每份文件独占 worker、无互抢，提交及时、
    // 不再「换个文件又红」。旧「12 分钟」读数出自 2 核受限容器；当前 4 核 runner
    // 串行全量实测约 2~3 分钟（墙钟 < 30min 闸），换 CI 确定性值得。
    fileParallelism: false,
    maxWorkers: 2,
    minWorkers: 1,
    // 页面级组件测试（L104–L113）落地后，测试文件从 2 份增至 13 份。jsdom + antd
    // 的挂载渲染很重，CPU 紧张时挂载期的 `waitFor` 可能超过默认的 5s
    // testTimeout（单文件跑必过、多文件并行才偶发）。20s 给足挂载头寸，
    // 不掩盖真失败（真失败是断言不符、秒级即报，不会拖到 30s）。
    // 2026-10-22 第四轮再抬：`DataManagement.sample` RAG 用例的 `waitFor` 无显式预算，
    // 走 RTL 的 `asyncUtilTimeout`，在 2 worker + jsdom + antd 争用下整文件 31s（CI 实测），
    // 默认 10s 不够。真失败仍秒级报。testTimeout / hookTimeout 抬到 35s 给「一次 30s
    // RTL waitFor + 若干 setup 头寸」留余量。
    testTimeout: 35000,
    hookTimeout: 35000,
    // React 19 把 passive effects 的提交排到 scheduler 的 `setImmediate`，antd 的
    // `message` / `notification` 又用**独立的 React root**（RTL `cleanup()` 够不到）。
    // 文件拆环境之后那个 setImmediate 才跑，一跑就 `schedulerEvent = window.event`
    // 抛 `ReferenceError: window is not defined`——这是 React 19 + jsdom + antd 的
    // 已知悬挂回调，发生在所有用例结束之后，与用例成败无关。默认 'throw' 会把整轮
    // vitest 判红（7 条无害悬挂回调就能毙掉 311 个通过的用例）。降到 'warn' 让这些
    // 拆环境后的悬挂回调不再毙轮；根因侧已在 setup.ts 用同步 act 把能冲干净的都冲掉，
    // 这里只兜底那极少数的真实延迟回调。
    unhandledErrors: 'warn',
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
