/**
 * 测试全局准备
 *
 * 三件事，全是**跨文件踩过的坑**，收在这里一处守护：
 *
 * 1. jest-dom 匹配器（`toBeInTheDocument` 等）。`api.ts` 的测试用不到它，
 *    但 React 组件测试的标准底座（规划 T1.9 要求引入 RTL），先就位。
 *
 * 2. 每个文件跑完统一清场：RTL `cleanup()` + antd 的 `message.destroy()` /
 *    `notification.destroy()`。为什么必须全局：antd 的静态 `message` 持有
 *    模块级单例 + **自动关闭计时器**（默认 3s）。某个用例触发过提示后，
 *    即使它自己的 `afterEach` 调了 destroy，计时器回调与 jsdom 环境销毁
 *    仍存在竞态——2026-10-11 CI 实测：`Settings.test.tsx` 的
 *    「保存成功后弹出提示」用例过后，3s 到点的关闭回调在环境已销毁的
 *    worker 里跑，`react-dom` 读 `window` 抛 `ReferenceError: window is
 *    not defined`，vitest 把这条未捕获异常判成步骤失败（测试本身 287
 *    个全过，红的是一条悬挂回调）。此前只有 Settings / DataManagement.upload
 *    两个文件手写了 destroy——漏一个文件就是一条 CI 红。
 *    vitest 的 setup afterEach 注册先于测试文件的 afterEach，因此这里的
 *    destroy 跑在文件自己的清理**之后**，是最后一道闸。
 *
 * 3. antd 的 `message.destroy()` 在测试环境里要能反复调用（无害）——
 *    这里包 try 是防极个别文件根本没有挂载过 antd 时的边缘情况。
 */
import '@testing-library/jest-dom/vitest'
import { cleanup } from '@testing-library/react'
import { afterEach } from 'vitest'
import { message, notification } from 'antd'

afterEach(() => {
  cleanup()
  try {
    message.destroy()
    notification.destroy()
  } catch {
    /* 没有挂载过 antd 时 destroy 是无操作；兜底不让清场自己炸 */
  }
})
