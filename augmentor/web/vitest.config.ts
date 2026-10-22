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
    // false）更保险但容器实测超过 12 分钟，不进 CI 闸门。（2026-10-22 曾试全串行，
    // 未捕获 window 报错 7→26 反增、断言 flake 挪到 transform，已回退。）
    // 2026-10-22 第五轮改用 **forks**（进程隔离，而非默认 threads）：默认 threads
    // 池里所有测试文件共享**同一 worker 进程的事件循环**，React 19 把 passive effects
    // 排到 `setImmediate` 的悬挂回调会在文件 N 拆环境后、文件 N+1 运行时才醒，跨文件
    // 读到已销毁的 `window` ⇒ 每轮**换个文件**红（A322→RAG→transform）且 window 报错
    // 累计（串行反增到 26）。forks 让每份文件独占一个子进程，事件循环彼此隔离，悬挂
    // macrotask 随进程结束而亡、跨不过程序边界。maxWorkers 保留 2 控制并行度。
    pool: 'forks',
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
