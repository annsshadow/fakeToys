# oa4rust

OA4Rust 三端 monorepo 根：服务端、桌面 Web 端、移动端共用一个目录。

```
oa4rust/
├── backend/     # 🦀 Rust 后端（axum workspace，96 crates，含 migrations/deploy）
├── frontend/    # 🖥️ 桌面 Web 端（Vue3 + Vite）与共享包（packages/{apis,sdk,ui,locales}）
├── mobile/      # 📱 移动端（uni-app：H5 / 微信小程序 / 原生 App）
├── package.json / pnpm-workspace.yaml   # 前端 workspace 根（frontend + mobile 同一 workspace）
├── vitest.config.ts / biome.json / tsconfig.base.json   # 前端共同门禁
├── tests/contracts/   # 三端 API 契约守卫（desktop/mobile 端点 × 后端注册路由）
├── e2e/               # Playwright E2E（需 live 后端）
└── scripts/ / docs/   # 前端脚本与文档
```

## 常用命令（在本目录执行）

- 前端全量测试：`pnpm test`（desktop + packages + 契约守卫）
- 移动端测试：`pnpm test:mobile`
- 类型检查 / Lint：`pnpm typecheck` / `pnpm lint`
- 构建桌面端（产物 `frontend/dist/web`）：`pnpm build`
- 构建移动端 H5：`pnpm --filter @oa4rust/mobile build:h5`
- 后端：见 [backend/README.md](backend/README.md)（`cd backend && cargo run`）

后端服务默认从 `frontend/dist/web` 读取前端构建产物（可用 `OA4RUST_WEB_DIST` 覆盖）。
三端契约门禁脚本位于 `../docs/audits/three-ends-2026-09-20/`。
