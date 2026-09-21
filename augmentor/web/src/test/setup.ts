/**
 * 测试全局准备
 *
 * 引入 jest-dom 的匹配器（`toBeInTheDocument` 等）。当前 `api.ts` 的测试用不到，
 * 但它是 React 组件测试的标准底座（规划 T1.9 要求引入 RTL），先就位，
 * Phase 2 的组件测试可直接用。
 */
import '@testing-library/jest-dom/vitest'
