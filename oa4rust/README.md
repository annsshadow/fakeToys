# oa4rust

O2OA 后端的 Rust 实现，以认证模块为第一个迁移试点。

## 开发环境

- Rust 1.75+
- PostgreSQL 14+
- 推荐使用 `rust-analyzer` 插件

## 快速开始

1. 克隆仓库
2. 复制 `.env.example` 到 `.env` 并配置数据库连接
3. 运行 `cargo run`

## 安全与质量门禁

- 浏览器会话 Cookie、CSRF 与代理部署：[`docs/security/browser-session-cookie.md`](docs/security/browser-session-cookie.md)
- RustSec 依赖审计与例外流程：[`docs/security/rustsec-audit.md`](docs/security/rustsec-audit.md)
- 后端验证：`cargo check --workspace --locked`、`cargo test --workspace --lib --locked`
- 前端验证：在 `../oa4rust-web` 运行 `pnpm test && pnpm typecheck && pnpm lint:check && pnpm build`

供应链扫描器由 CI 使用 Rust 1.88.0 运行；这与应用在 `[workspace.package]` 中声明的最低 Rust 版本是两个独立约束。

## 项目结构

```
oa4rust/
├── Cargo.toml              # workspace 根
├── crates/
│   ├── shared/             # 共享基础设施（响应格式、错误处理、数据库）
│   ├── auth/               # 认证模块业务逻辑
│   └── personal/           # 个人设置模块
├── src/
│   └── main.rs             # 入口
├── migrations/             # 数据库迁移文件
├── scripts/                # 数据迁移脚本
├── deploy/                 # 部署配置
└── .env.example
```

## 端口

- Rust 服务：3000
- Java 服务：20020
