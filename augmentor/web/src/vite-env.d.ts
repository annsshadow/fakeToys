/// <reference types="vite/client" />

// 引入 Vite 的客户端类型（`import.meta.glob` / `import.meta.env` / 静态资源模块声明）。
// 没有这个文件时 `import.meta.glob` 在 `tsc --noEmit` 下是未知属性，
// `services/api.wiring.test.ts` 无法通过类型检查。
