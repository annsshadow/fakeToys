<div align="center">

# 🧸 fakeToys

**一个装着若干「不那么玩具」的玩具箱**

从 Rust 重写的企业级 OA 后端，到塔防元素反应小程序游戏、AI 训练数据增强工具、自动签到面板、免登 Token 生成器——
几个互相独立、各自能跑的实验性项目，共处一个 monorepo。

<br/>

![Rust](https://img.shields.io/badge/Rust-1.85+-000000?logo=rust&logoColor=white)
![Go](https://img.shields.io/badge/Go-1.26+-00ADD8?logo=go&logoColor=white)
![Vue](https://img.shields.io/badge/Vue-3.5-4FC08D?logo=vuedotjs&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-5.8-3178C6?logo=typescript&logoColor=white)
![Python](https://img.shields.io/badge/Python-3.10+-3776AB?logo=python&logoColor=white)
![Node](https://img.shields.io/badge/Node-Playwright-339933?logo=node.js&logoColor=white)
![Java](https://img.shields.io/badge/Java-8+-ED8B00?logo=openjdk&logoColor=white)
![License](https://img.shields.io/badge/License-AGPL--3.0-blue?logo=gnu&logoColor=white)

</div>

---

## 📦 里面有什么

这是一个多语言、多技术栈的 monorepo。每个子项目自成一体，可单独构建、单独运行，也各自带有更详细的 README。

| 子项目 | 一句话 | 技术栈 | 详情 |
| --- | --- | --- | --- |
| 🦀 **[oa4rust](#-oa4rust--rust-重写的-oa-后端)** | OA 三端 monorepo：Rust 后端 + 桌面 Web 端 + 移动端（uni-app），后端为 96 个 crate 的 workspace | Rust · Axum · Vue3 · uni-app · PostgreSQL | [README](oa4rust/README.md) |
| 💥 **[laiyipao](#-laiyipao--塔防元素反应小游戏)** | 仿《向僵尸开炮》玩法内核的塔防 + 元素反应小程序游戏 | Go · uni-app · Vue3 · PostgreSQL | [README](laiyipao/README.md) |
| 🧪 **[augmentor](#-augmentor--ai-训练数据增强)** | 把少量种子数据扩充成多轮问答对的训练数据增强工具 | Python · FastAPI · React | [README](augmentor/README.md) |
| ✅ **[auto-checkin](#-auto-checkin--自动签到面板)** | 给多个大模型公益站自动签到领奖，带本地统计面板 | Node · Playwright · Express | [README](auto-checkin/README.md) |
| 🔐 **[cool](#-cool--免登-token-生成)** | Cool College OA 的免登 Token 生成工具 | Java · AES/MD5 | [README](cool/README.md) |

---

## 🦀 oa4rust — Rust 重写的 OA 后端

把一套 Java 企业 OA 系统（O2OA）用 Rust 逐模块重写，追求同等能力下更强的性能、内存安全与更低的资源占用。

- **架构**：Axum + Tokio 异步栈，PostgreSQL 持久化，按业务域拆成 **96 个 crate** 的 Cargo workspace（认证、流程、门户、CMS、会议、考勤、即时通讯、AI、脑图、组织……）。
- **契约对齐**：以 parity 测试对照原系统路由与行为，保证接口级别的一致性。
- **实时能力**：内置 WebSocket 实时通道，支撑 IM 与在线状态。
- **安全门禁**：全路径 HttpOnly Cookie + CSRF/CORS、RustSec 供应链审计、OpenAPI 漂移门禁。

```bash
cd oa4rust/backend
cp .env.example .env          # 配置数据库连接
cargo run                     # 启动服务端
cargo test --workspace --lib  # 运行单元测试
```

> 更多开发环境、质量门禁与目录结构见 [oa4rust/backend/README.md](oa4rust/backend/README.md)。

## 🖥️ oa4rust 前端 — 桌面 Web + 移动端

oa4rust 的前端两端与共享包同住一个 pnpm workspace（`oa4rust/` 为 workspace 根），一套 SDK/类型驱动桌面与移动两端。

- **frontend/apps/desktop**：Vue3 + Vite + Naive UI 的桌面 Web 端。
- **mobile**：基于 uni-app 的移动端，可编译到 H5 / 微信小程序 / 原生 App。
- **frontend/packages**：`sdk`（类型化 API 客户端）、`apis`（覆盖后端路由的 API 模块）、`ui`、`locales` 共享包。
- **质量栈**：TypeScript 严格模式、Biome、Vitest 单测、Playwright E2E。

```bash
cd oa4rust
pnpm install
pnpm dev          # 启动桌面端开发服务器
pnpm test         # Vitest（pnpm test:mobile 跑移动端）
pnpm build        # 生产构建（产物 frontend/dist/web）
```

## 💥 laiyipao — 塔防元素反应小游戏

仿微信小游戏《向僵尸开炮》玩法内核自研的**塔防 + 元素反应**小程序游戏。美术 100% Canvas 程序化绘制，不使用任何原作名称、素材、UI 或文案。

- **三端子工程**：`server/`（Go Fiber + PostgreSQL）、`miniapp/`（uni-app，微信小程序 / H5）、`admin/`（Vue 3 运营后台 + 玩法文档站）。
- **设计内核**：元素反应矩阵（攻击权重结构性 ≤30%，堆数值无法绕过搭配收益）、插槽与热量、专精树 + 装备契合度、地形机制关、防线值守、构筑评分。
- **确定性战斗**：定点整数 + 确定性 PRNG，回放哈希验真——任何人可取回种子重放并证伪可疑分数。
- **工程契约**：48 张业务表 + goose 迁移、端到端验收脚本、双端契约与文档漂移守卫。

```bash
cd laiyipao/server
export DATABASE_URL="postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"
go run ./cmd/migrate && go run ./cmd/seed && go run ./cmd/api
```

> 玩法创新、公式推导与部署运维见 [laiyipao/README.md](laiyipao/README.md) 与 `laiyipao/docs/`。

## 🧪 augmentor — AI 训练数据增强

用于生成 AI 模型训练数据的工具：将少量种子数据，通过多种 LLM API 与 NLP 技术，扩充为高质量多轮问答对。

- **多模型**：百度 ERNIE、OpenAI、Ollama、Claude、Gemini。
- **质量控制**：语义相似度、回答相关性、多样性多维评分 + 基于向量的智能去重。
- **数据安全**：隐私脱敏、训练/测试泄漏检测、就绪审计（go / no-go）。
- **工程能力**：断点续传、流式分块处理、数据集对比、画像与离群点检测。
- **多格式导出**：JSONL、Llama-Factory、Alpaca、ShareGPT、ChatML、CSV。
- **HTTP API + Web**：FastAPI 68 个 REST 端点（全部声明响应契约）+ `web/`（React 18 + Ant Design）可视化界面。

> 详见 [augmentor/README.md](augmentor/README.md)。

## ✅ auto-checkin — 自动签到面板

给多个大模型公益站（new-api / one-api 系面板）自动签到领奖，用真实浏览器绕过反爬，带一个 2026 科技风的本地统计面板（趋势图 / 周月 / 成功率）。

- 已实测接入 **VyceAI**、**AgentRouter**、**JustWoker** 三站，各自适配了 PoW、WAF、Cloudflare Turnstile 等反爬。
- **隐私优先**：账号密码只存本地 `.env`（已 gitignore），面板默认只监听 `127.0.0.1`，接口不返回任何凭据。

```bash
cd auto-checkin
npm install
npm run setup:browser   # 安装 Playwright Chromium
npm start               # 启动定时签到 + Web 面板
```

> 站点机制、隐私说明与配置见 [auto-checkin/README.md](auto-checkin/README.md)。

## 🔐 cool — 免登 Token 生成

Cool College（coolcollege）OA 系统的免登 Token 生成工具，支持 AES 加密/解密、MD5、Base64，兼容 .NET 的 AES-CBC 解密算法，可生成用户 ID / 工号 / 手机号 / 邮箱四种免登 Token。

> 依赖、密钥规则与用法见 [cool/README.md](cool/README.md)。

---

## 🗂️ 仓库结构

```
fakeToys/
├── oa4rust/          # 🦀🖥️📱 OA 三端 monorepo（backend Rust 后端 + frontend 桌面端 + mobile 移动端）
├── laiyipao/         # 💥 塔防元素反应小游戏（Go server + uni-app + admin）
├── augmentor/        # 🧪 AI 训练数据增强工具（Python + React）
├── auto-checkin/     # ✅ 公益站自动签到 + 统计面板（Node）
├── cool/             # 🔐 免登 Token 生成（Java）
├── docs/             # 📚 方案 / 评审 / 审计 / 已归档计划文档
├── scripts/          # 🛠️ 通用脚本
└── .github/workflows # 🤖 各子项目 CI（oa4rust / web / 供应链 / 三端契约 / AI 平台）
```

## 🚀 快速开始

各子项目环境要求不同，请进入对应目录按其 README 操作。总体上：

- **oa4rust**：Rust 1.85+、PostgreSQL 14+；前端两端另需 Node + pnpm
- **laiyipao**：Go 1.26+、PostgreSQL 15+、Node 20+ + pnpm
- **augmentor**：Python 3.10+
- **auto-checkin**：Node + Playwright
- **cool**：JDK 8+

## 📄 开源协议

本仓库以 **[GNU AGPL-3.0](LICENSE)** 授权。你可以自由使用、修改与分发，但任何衍生作品——**包括仅通过网络对外提供服务的场景（如部署 oa4rust）**——都必须以同一协议公开完整源码。

选择 AGPL-3.0 的原因：`oa4rust` 在重写过程中参考了 [O2OA](https://github.com/o2oa/o2oa)（同为 AGPL-3.0）的源码与接口契约，作为其衍生作品，沿用上游协议以保持授权一致。

> **上游归属**：oa4rust / oa4rust-web 相关部分源于对 O2OA 的重写，原始版权归 O2OA 项目及其贡献者所有。

## 🤝 关于

`fakeToys` 是一个个人实验性项目集合——名字叫「玩具」，但里面的东西大多认真在做。
