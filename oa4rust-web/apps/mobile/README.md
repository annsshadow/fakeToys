# @oa4rust/mobile — OA4Rust 跨平台移动端

基于 **uni-app（Vue 3 + Vite + TypeScript）** 的跨多平台移动端，作为 `oa4rust-web`
monorepo 的一个 workspace 成员（`apps/mobile`），与桌面端 `apps/desktop` 共享
`oa4rust` 后端的 API 契约。

## 平台目标

| 目标 | 命令 | 说明 |
| --- | --- | --- |
| H5 | `pnpm dev:h5` / `pnpm build:h5` | 浏览器，Cookie 鉴权开箱即用 |
| 微信小程序 | `pnpm dev:mp-weixin` / `pnpm build:mp-weixin` | 需配置 mp-weixin appid |
| 原生 App | `pnpm dev:app` / `pnpm build:app` | HBuilderX / 云打包 |

产物输出到 `apps/mobile/unpackage/dist/`。

## 架构决策

- **仅复用契约类型**：从 `@oa4rust/sdk` `import type`（`O2User` / `OrgGroup` /
  `PagedResponse` / `ApiResponse`），编译期擦除，避免把桌面端 SDK 的
  `vue-router` / `@tanstack/vue-query` / 浏览器 `fetch` 运行时拖进移动端，
  也规避 uni-app 自带 Vue 构建（`@dcloudio/uni-h5-vue`）与桌面 `vue` 的双份 Vue 冲突。
- **uni.request 传输层**（`src/services/http.ts`）：与桌面 `ApiClient` 语义对齐
  （401 自动 refresh 重试、403 权限错误、统一 `ApiError` / `ApiResponse`），
  底层换成 `uni.request` / `uni.uploadFile` 以跨平台运行。
- **精选端点**（`src/services/index.ts`）：只暴露适合移动端的高频模块
  （auth / portal / message / process / file / org / general），路径与
  `@oa4rust/apis` 保持一致，不把 3892 条路由全量打进移动包。

## 鉴权模型（重要）

`oa4rust` 后端采用 **HttpOnly Cookie** 会话：

- **H5 目标**：同源部署或经 dev-server 代理（`/jaxrs` → `http://localhost:3000`），
  请求携带 `withCredentials`，Cookie 鉴权直接可用。
- **原生 / 小程序目标**：需 `setApiBase` 指向后端绝对地址（`import.meta.env.VITE_OA_API_BASE`）。
  Cookie 会话在原生容器 / 小程序中不会自动跨域携带，须由后端提供配套的
  令牌（token）鉴权通道后，在此传输层注入 `Authorization` 头。

## 开发

```bash
# 从 monorepo 根
pnpm install                                        # 安装全部 workspace 依赖
pnpm --filter @oa4rust/mobile typecheck             # tsc 检查（SDK 经 tsconfig paths 走源码）
pnpm --filter @oa4rust/mobile dev:h5                # 启动 H5（默认 :5174）
```

页面：`login`（登录）、`index`（工作台）、`message`（消息）、`process`（审批）、
`doc`（文档）、`mine`（我的），底部 tabBar 4 项。
