# OA4Rust 项目文档

> 本目录是 OA4Rust（`oa4rust/`）的统一文档入口，覆盖项目定位、架构、模块、接口、流程、环境、运行、示例、FAQ 与维护约定。
> 所有内容基于当前代码实测梳理（2026-10-14 快照，分支 `optimism20261012`）。路径引用均以仓库根（`fakeToys/`）为基准。

## 文档导航

| 分册 | 内容 | 对应问题 |
| --- | --- | --- |
| [01-architecture.md](01-architecture.md) | 项目定位与整体架构 | 这是什么项目？怎么组织的？ |
| [02-modules.md](02-modules.md) | 模块划分与核心功能 | 96 个 crate / 三端各负责什么？ |
| [03-data-and-api.md](03-data-and-api.md) | 关键数据结构与接口说明 | 响应信封、错误类型、核心 API 路径 |
| [04-flows.md](04-flows.md) | 主要业务流程 | 登录、审批、IM、部署回滚等全链路 |
| [05-environment.md](05-environment.md) | 环境依赖与配置 | 数据库 / Redis / 环境变量清单 |
| [06-install-and-run.md](06-install-and-run.md) | 安装与运行方式 | 从零到跑通三端的操作步骤 |
| [07-examples.md](07-examples.md) | 典型使用示例 | 登录调用、审批、WebSocket、MCP 示例 |
| [08-faq.md](08-faq.md) | 常见问题与注意事项 | 64MB 栈、CSRF 403、Cookie 跨域等坑 |
| [09-maintenance.md](09-maintenance.md) | 后续维护与更新约定 | 门禁命令、文档同步、审计节奏 |

## 30 秒速览

- **OA4Rust = O2OA（Java 开源 OA，o2server）的 Rust 重写 + 配套三端**，对标 o2server v9 的接口契约与信封（使 O2OA 生态存量前端可无感接入）；项目独立交付，从未上线、无 Java 存量与迁移包袱。
- **三端 monorepo**：`oa4rust/backend/`（Rust axum workspace，96 crates，单二进制）、`oa4rust/frontend/`（Vue3 桌面 Web + 共享包）、`oa4rust/mobile/`（uni-app：H5 / 微信小程序 / App）。
- **迁移状态**：90 个业务 crate 中 88 个已真实化（`done`，无桩），2645 个真实 handler；跟踪文件为 `oa4rust/backend/docs/brainstorms/oa4rust-migration-status.md` 与 `oa4rust-endpoint-inventory.md`（脚本自动生成，单一信息源）。
- **运行形态**：单进程监听 `:3000`，启动时自动执行幂等数据库迁移（顶层 205 个 `.sql` 迁移文件 + `archive/` 归档），默认同时托管前端静态产物（SPA 回退）；可选 `--http` 挂载 MCP 端点 `/mcp`；WebSocket 实时通道 `/ws/realtime`。
- **依赖**：PostgreSQL 14+（主库，MySQL 8 经方言抽象兼容）、Redis 7（会话 / 限流，不可达时自动降级内存+DB）。
- **许可**：AGPL-3.0-or-later（见各源码文件头）。

## 一图看懂目录

```
fakeToys/
├── docs/oa4rust/          # ← 本文档集
└── oa4rust/
    ├── backend/           # 🦀 Rust 后端（axum workspace，96 crates）
    │   ├── crates/        #    业务/实体/基础设施 crate
    │   ├── src/           #    单二进制入口（main.rs / lib.rs）
    │   ├── migrations/    #    205 个顶层 SQL 迁移（成对含 *_rollback；历史归档在 archive/）
    │   ├── deploy/        #    nginx 反代模板 + 早期灰度切流脚本（历史归档）
    │   └── docs/          #    审计 / 迁移状态 / 运维与安全文档
    ├── frontend/          # 🖥️ 桌面 Web（apps/desktop）+ 共享包（packages/{apis,sdk,ui,locales}）
    ├── mobile/            # 📱 uni-app 移动端（16 页面，H5/微信小程序/App）
    ├── tests/contracts/   # 三端 API 契约守卫（vitest）
    ├── e2e/               # Playwright E2E（需 live 后端）
    └── vendor/            # Windows 本机 GNU 工具链（不入 git，README 可复现）
```
