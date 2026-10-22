/**
 * 测试全局准备
 *
 * 四件事，全是**跨文件踩过的坑**，收在这里一处守护：
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
 *
 * 4. antd 6 的两个坑（2026-10-18 升级 antd 5→6 时踩到，写在这里免得下次重踩）：
 *    - Trigger（Select / Dropdown / DatePicker…）依赖 `ResizeObserver`，jsdom 没有 →
 *      各页面测试在挂载前自行补桩（`window.ResizeObserver = window.ResizeObserver || ...`），
 *      `window.matchMedia` 同理。
 *    - Select 的 DOM 变了：v5 的 `.ant-select-selector` **已移除**，打开下拉的 `mouseDown`
 *      要打在根节点 `.ant-select` 上（探针实测：`.ant-select` / `.ant-select-content` /
 *      `[role=combobox]` 都能开，只有根节点不会随下拉内容变化，且下拉内部不含 `.ant-select`）。
 *      单选的选中值渲染在 `.ant-select-content`（v5 的 `.ant-select-selection-item` 同样已移除）。
 *      未变：选项类名 `.ant-select-item-option`、`[role=combobox]` 的 `aria-expanded`。
 */
import '@testing-library/jest-dom/vitest'
import { cleanup } from '@testing-library/react'
import { act } from 'react'
import { afterEach } from 'vitest'
import { message, notification } from 'antd'

afterEach(async () => {
  // `await act(async () => …)` 不是仪式：React 19 的调度器把渲染任务排到
  // `setImmediate`（`Immediate.performWorkUntilDeadline` → `processImmediate`），
  // 用例结束时若还有一次 pending render，它会在**环境已销毁之后**才跑，
  // `react-dom` 读 `window` 抛 `ReferenceError: window is not defined`，
  // vitest 记为 Unhandled Error（用例全过、步骤仍红）。
  // 2026-10-22 CI 实测：3 条这样的未捕获异常。act 的异步形态会把调度队列
  // 走完再返回，等于在拆环境前把 pending 渲染冲干净。
  await act(async () => {
    cleanup()
  })
  try {
    message.destroy()
    notification.destroy()
  } catch {
    /* 没有挂载过 antd 时 destroy 是无操作；兜底不让清场自己炸 */
  }
})
