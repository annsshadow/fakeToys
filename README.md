<div align="center">

<img src="docs/assets/banner.svg" alt="fakeToys" width="100%">

<br/>

### 一个装着若干「不那么玩具」的玩具箱

**从 Rust 重写的企业级 OA 后端，到塔防元素反应 H5 游戏、AI 训练数据增强工具、自动签到面板——**
**几个互相独立、各自能跑的实验性项目，共处一个 monorepo。**

<br/>

<a href="#projects"><kbd>📦</kbd> 项目</a>
&nbsp;·&nbsp;
<a href="#metrics"><kbd>📊</kbd> 指标</a>
&nbsp;·&nbsp;
<a href="#architecture"><kbd>🏗️</kbd> 架构</a>
&nbsp;·&nbsp;
<a href="#gates"><kbd>🚦</kbd> 门禁</a>
&nbsp;·&nbsp;
<a href="#quickstart"><kbd>🚀</kbd> 上手</a>

<br/>

[![Rust](https://img.shields.io/badge/Rust-1.85%2B-f0874a?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Go](https://img.shields.io/badge/Go-1.26%2B-00ADD8?logo=go&logoColor=white)](https://go.dev/)
[![Vue](https://img.shields.io/badge/Vue-3.5-42b883?logo=vuedotjs&logoColor=white)](https://vuejs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.8-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Python](https://img.shields.io/badge/Python-3.10%2B-3776AB?logo=python&logoColor=white)](https://python.org/)
[![Node](https://img.shields.io/badge/Node-20%2B-339933?logo=node.js&logoColor=white)](https://nodejs.org/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-14%2B-4169E1?logo=postgresql&logoColor=white)](https://www.postgresql.org/)
[![License](https://img.shields.io/badge/License-AGPL--3.0-blue?logo=gnu&logoColor=white)](LICENSE)

</div>

---

<a id="projects"></a>

## 📦 里面有什么

这是一个多语言、多技术栈的 monorepo。每个子项目自成一体，可单独构建、单独运行、单独测试。

| 子项目 | 一句话 | 技术栈 | 规模 |
| --- | --- | --- | --- |
| 🦀 **[oa4rust](oa4rust/)** | Rust 重写的企业 OA 系统 | Rust · Axum · Vue3 · uni-app · PostgreSQL | 97 crate · 652 `.rs` |
| 💥 **[laiyipao](laiyipao/)** | 塔防 + 元素反应 H5 游戏 | Go · Fiber · uni-app · Vue3 · PostgreSQL | 397 `.go` · 50 表 |
| 🧪 **[augmentor](augmentor/)** | AI 训练数据增强工具 v3.0 | Python · FastAPI · React · Ant Design | 81 端点 · 10737 单测 |
| ✅ **[auto-checkin](auto-checkin/)** | 大模型公益站自动签到面板 | Node · Playwright · Express | 3 站点 · 9 项棘轮 |
| 🧠 **[llm-lab](llm-lab/)** | 从零学大模型训练 | Python · NumPy · PyTorch | 11 讲义 · 95 用例 |

---

<a id="metrics"></a>

## 📊 质量仪表盘

<img src="docs/assets/dashboard.svg" alt="质量仪表盘" width="100%">

> 以上均为**实测值**，非宣称值。口径逐项标注：oa4rust 为 shared crate 行覆盖率（`cargo llvm-cov`），augmentor 为核心包行覆盖率（`coverage.xml`），laiyipao 为全包合并 profile 语句覆盖率（`go tool cover -func`）。

<div align="center">

| 指标 | 数值 | 证据来源 |
| :-- | :--: | :-- |
| Rust · oa4rust 行覆盖 | **95.01%** | `docs/plans/2026-10-29-remaining-work-plan.md`（容器内实测） |
| Python · augmentor 行覆盖 | **99.51%** | `augmentor/coverage.xml`（全量复跑，2026-11-15：TOTAL 16074/78/4480/52，门槛 99%） |
| Go · laiyipao 语句覆盖 | **85.8%** | `docs/laiyipao/audits/coverage-2026-10-28.md` |
| augmentor 单测 | **10737 passed / 0 failed / 2 skipped** | 本地全量复跑（`cd augmentor && python -m pytest tests -q`，648.61 s，覆盖率 99.37%）；`ai-platform-ci.yml` 的四格红已在本轮收口，见 [`docs/plans/2026-11-12-remaining-ledger-inventory.md`](docs/plans/2026-11-12-remaining-ledger-inventory.md) §三 |
| 编译告警门禁 | **13 通过 / 0 失败 / 0 跳过** | `node scripts/check-warnings.mjs`（本轮抽查 rust-lints / llm-lab / llm-lab-tests / augmentor-web / license 五项亦全绿） |
| 全仓提交 | **以 `git rev-list --count HEAD` 为准** | 浅克隆下读数会偏小，不写死数字 |

</div>

---

<a id="architecture"></a>

## 🏗️ 架构总览

<img src="docs/assets/architecture.svg" alt="架构总览" width="100%">

```mermaid
graph LR
    Gate["🚦 warnings-gate<br/>node scripts/check-warnings.mjs"] --> Rust["🦀 oa4rust<br/>Rust · Axum"]
    Gate --> Go["💥 laiyipao<br/>Go · Fiber"]
    Gate --> Py["🧪 augmentor<br/>Python · FastAPI"]
    Gate --> Ts["✅ auto-checkin<br/>Node · Playwright"]
    Gate --> Lab["🧠 llm-lab<br/>Python · NumPy"]

    Rust --> DB1[("PostgreSQL")]
    Go --> DB2[("PostgreSQL")]
    Rust --> FE1["Vue3 Desktop<br/>uni-app Mobile"]
    Go --> FE2["uni-app H5<br/>Vue Admin"]
    Py --> FE3["React 11 Pages"]

    style Gate fill:#1e1b4b,stroke:#818cf8,stroke-width:2px,color:#c7d2fe
    style Rust fill:#2a1a0e,stroke:#f0874a,color:#f6c453
    style Go fill:#07242a,stroke:#00b8d4,color:#5eead4
    style Py fill:#171442,stroke:#6d5efc,color:#a78bfa
    style Ts fill:#08240f,stroke:#22c55e,color:#86efac
    style Lab fill:#062a1e,stroke:#42b883,color:#6ee7b7
```

---

<a id="gates"></a>

## 🚦 编译告警零容忍

本仓库的门禁是**所有子项目编译 0 告警**。不是"尽量少"，是 0：出现一条告警即视为构建失败。

判据是**编译器自己的输出**，而不是"命令退出码为 0"——很多工具链的告警不改变退出码（Vite / Rollup / lightningcss / Python 的 `SyntaxWarning` 都是这样），所以每项检查除了要求 exit 0，还要对输出做正则匹配，命中即失败。

```bash
node scripts/check-warnings.mjs          # 全跑
node scripts/check-warnings.mjs --list   # 列出检查项
node scripts/check-warnings.mjs --only rust,admin
```

各子项目自身也把"告警=错误"写进了配置，所以直接跑各自的构建命令同样会失败：

| 子项目 | 门禁 |
|--------|------|
| oa4rust 后端（Rust） | `Cargo.toml` 的 `[workspace.lints.rust] warnings = "deny"`（96 个 workspace member 均 `[lints] workspace = true`；连同根包共 97 个 `Cargo.toml`），CI 另有 `cargo clippy -- -D warnings` |
| oa4rust desktop / mobile | CI 构建后由 `scripts/assert-no-warnings.sh` 断言日志无 `(!)` / `[lightningcss` / `warning:` |
| laiyipao server（Go） | `go vet` + `gofmt -l`（有输出即失败） |
| laiyipao admin / miniapp | 同上，构建日志断言 |
| augmentor / llm-lab（Python） | `python -W error::SyntaxWarning -m compileall` |
| augmentor/web | `eslint --max-warnings 0` + 构建日志断言 |
| auto-checkin（TS） | `tsc --noEmit` |

<details>
<summary><b>🔍 19 项检查清单（点击展开）</b></summary>

> 下表按 `scripts/check-warnings.mjs` 的 `CHECKS` 顺序逐项列出，编号与脚本一致。

| # | id | 检查内容 |
|---|---|---|
| 1 | `rust-lints` | oa4rust/backend 全部成员继承 `deny(warnings)` |
| 2 | `admin-config-drift` | laiyipao admin 枚举映射与服务端白名单一致 |
| 3 | `script-paths` | 各项引用的本地脚本存在且已入库（防空转） |
| 4 | `rust` | oa4rust/backend（Rust，96 workspace member / 连同根包 97 个 `Cargo.toml`） |
| 5 | `desktop` | oa4rust desktop（Vue + Vite） |
| 6 | `mobile` | oa4rust mobile（uni-app H5 + 微信小程序） |
| 7 | `go` | laiyipao/server（Go） |
| 8 | `admin` | laiyipao/admin（Vue + Vite） |
| 9 | `miniapp` | laiyipao/miniapp（uni-app H5 源码） |
| 10 | `auto-checkin` | auto-checkin（TypeScript，typecheck + 测试） |
| 11 | `augmentor-web` | augmentor/web（React + Vite） |
| 12 | `augmentor` | augmentor（Python 字节码编译） |
| 13 | `llm-lab` | llm-lab（Python 字节码编译） |
| 14 | `llm-lab-tests` | llm-lab（pytest 行为面） |
| 15 | `r7-readonly-cross-check` | oa4rust R7 只读挂写交叉验证（0 疑似含写） |
| 16 | `license` | 全仓源码 SPDX 标识符 |
| 17 | `openapi-paths` | oa4rust OpenAPI 生成物与路由源码不脱节（regen diff 为空） |
| 18 | `generated-tests` | oa4rust `tests_generated.rs` 与生成器逐字一致（O14 门禁） |
| 19 | `generated-artifacts` | oa4rust 其余 Python 生成器产物与源码一致（MCP 路由表） |

</details>

> ⚠️ 不要用 `VITE_CONFIG_NATIVE_IGNORE_WARNING` 这类开关压掉告警——那是把门禁关掉，不是修问题。

---

## 📚 各子项目速览

<details>
<summary><b>🦀 oa4rust — Rust 重写的企业 OA 系统</b></summary>

源自 O2OA 的重写，追求更强的性能、内存安全与更低的资源占用。

**架构特点**
- 后端：Axum + Tokio 异步栈，PostgreSQL 持久化，**97 个 crate** 的 Cargo workspace
- 前端：pnpm workspace 统一管理桌面端（Vue3 + Vite）和移动端（uni-app）
- 类型驱动 SDK：统一 API 客户端服务两端，跨账号缓存隔离保护
- 质量保障：TypeScript strict 模式、Vitest 单测、Playwright E2E、三端契约守卫

**工程成果**
- 前端 main bundle 优化：1124KB → 96KB（-91%）；当前实测 main chunk 41KB，vendor 独立分包
- 覆盖率突破：shared crate 行覆盖 **95.01%**
- 契约对齐：parity 测试对照原系统路由与行为
- 安全门禁：HttpOnly Cookie + CSRF/CORS、RustSec 供应链审计、口令不入 URL

```bash
# 后端
cd oa4rust/backend
cp .env.example .env && cargo run

# 前端
cd oa4rust
pnpm install && pnpm dev
```

> 详见 [oa4rust/README.md](oa4rust/README.md) 和 [oa4rust/backend/README.md](oa4rust/backend/README.md)

</details>

<details>
<summary><b>💥 laiyipao — 塔防 + 元素反应 H5 游戏</b></summary>

仿《向僵尸开炮》玩法的塔防 + 元素反应游戏，美术 100% Canvas 程序化绘制。

**核心创新**
- 元素反应矩阵：攻击力权重结构性 ≤30%，堆数值无法绕过搭配收益
- 确定性战斗：定点整数 + 确定性 PRNG，回放哈希验真
- 地形机制：5 类可交互地形（油桶/潮汐闸/风障/掩体/蓄能塔），25% 关卡使用
- 防线值守：构筑快照本地模拟，服务端不跑战斗引擎

**工程完善**
- CI 全面进仓：Go server / Vue admin / uni-app H5 三端自动化测试
- 50 张业务表 + 23 个 goose 迁移、端到端验收脚本
- 双端契约守卫、公式一致性测试、100 关零死锁验证

```bash
cd laiyipao/server
export DATABASE_URL="postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"
go run ./cmd/migrate && go run ./cmd/seed && go run ./cmd/api
```

> 详见 [laiyipao/README.md](laiyipao/README.md)

</details>

<details>
<summary><b>🧪 augmentor — AI 训练数据增强工具 v3.0</b></summary>

将少量种子数据扩充为高质量多轮问答对。**v3.0** 已发布。

**核心能力**
- 多模型支持：百度 ERNIE、OpenAI、Ollama、Claude、Gemini
- 质量控制：语义相似度、回答相关性、多样性多维评分 + 向量去重
- 数据安全：隐私脱敏、训练/测试泄漏检测、就绪审计
- 多格式导出：JSONL、Llama-Factory、Alpaca、ShareGPT、ChatML、CSV

**v3.0 升级**
- 性能：流式处理峰值 22.70MB→1.66MB、dedup 内存 9536MB→20MB
- 健壮性：**10737 单测全绿 / 2 skipped**（`coverage.xml` 行覆盖 **99.51%**、门禁汇总 **99.37%**），错误分类、自动重试、配置写入闭环
- API：**81 个 REST 端点**全部声明 `response_model`
- Web：React 11 个功能页 + Ant Design

```bash
cd augmentor
pip install -r requirements.txt
uvicorn api.main:app --port 8000
```

> 详见 [augmentor/README.md](augmentor/README.md)

</details>

<details>
<summary><b>✅ auto-checkin — 大模型公益站自动签到面板</b></summary>

用真实浏览器绕过反爬，带本地统计面板。

**已接入站点**
- VyceAI：自定义 API + PoW 工作量证明
- AgentRouter：阿里云 WAF，GitHub OAuth
- JustWoker：Cloudflare Turnstile，GitHub OAuth

**特性**
- 自动化测试：三站点测试 + 截图诊断 + 领奖验证
- 隐私优先：账号密码只存本地 `.env`，面板默认仅监听 `127.0.0.1`
- 覆盖率棘轮：9 个模块各钉一条行覆盖地板（justwoker 100% / vyceai 99.51%），任何回退直接红

```bash
cd auto-checkin
npm install && npm run setup:browser
npm start  # 启动定时签到 + Web 面板
```

> 详见 [auto-checkin/README.md](auto-checkin/README.md)

</details>

<details>
<summary><b>🧠 llm-lab — 从零学大模型训练</b></summary>

可运行、可验证、带完整中文讲义。目标不是堆术语，而是让你亲手把一个 GPT **从零造出来 → 训练起来 → 再优化**。

**两条代码轨**
- **minigrad**：纯 NumPy 手写 autograd + GPT，任何环境可跑
- **torchgpt**：PyTorch 复刻 nanoGPT，演示 AMP/梯度累积/LoRA/DPO

**学习资源**
- 11 篇中文讲义：分词、注意力、Transformer、训练、微调、对齐、评估
- 95 个验证用例：数值梯度校验、KV cache 等价性、端到端冒烟测试
- 可视化：注意力热力图、尺度实验（参数量 vs loss）

```bash
cd llm-lab
pip install -r requirements.txt
python -m minigrad.train  # loss ~4.1 → ~0.3
pytest tests/ -q          # 95 passed
```

> 详见 [llm-lab/README.md](llm-lab/README.md) · 讲义从 [`docs/llm-lab/00-学习路线.md`](docs/llm-lab/00-学习路线.md) 开始

</details>

---

## 🗂️ 仓库结构

```
fakeToys/
├── oa4rust/          # 🦀 Rust OA 后端 + 前端双端（97 crates + Vue3 + uni-app）
├── laiyipao/         # 💥 塔防元素反应游戏（Go server + uni-app + Vue admin）
├── augmentor/        # 🧪 AI 数据增强工具（Python + FastAPI + React）
├── auto-checkin/     # ✅ 自动签到面板（Node + Playwright）
├── llm-lab/          # 🧠 大模型训练教学（Python + NumPy/PyTorch）
├── docs/             # 📚 方案/评审/审计文档（约 270 篇 md / 360 余文件）
│   └── assets/       # 🎨 README 视觉资源（自绘 SVG）
├── scripts/          # 🛠️ 通用脚本（check-warnings.mjs 等）
└── .github/workflows # 🤖 10 条 CI 流水线
```

<a id="quickstart"></a>

## 🚀 快速开始

各子项目独立运行，请进入对应目录按其 README 操作：

| 子项目 | 环境要求 |
|--------|---------|
| **oa4rust** | Rust 1.85+（CI 用 1.98.1）、PostgreSQL 14+、Node + pnpm |
| **laiyipao** | Go 1.26+、PostgreSQL 15+、Node + pnpm |
| **augmentor** | Python 3.10+ |
| **auto-checkin** | Node 20+ + Playwright |
| **llm-lab** | Python 3.10+（torch 可选） |

## 📖 文档站

`docs/` 下的 **laiyipao / llm-lab / oa4rust** 三个目录会发布成可浏览的书页式站点（MkDocs + Material，带全文检索）：

**在线地址**：<https://annsshadow.github.io/fakeToys/>

其余 `docs/` 内容（plans、audits、solutions 等）不入站，由 `mkdocs.yml` 的 `exclude_docs` 排除；CI 构建后还会做产物白名单校验，新增顶层目录漏配会直接让构建失败。

```bash
# 本地预览（改完 Markdown 自动热更新）
python -m venv .venv-docs && .venv-docs/Scripts/pip install -r requirements-docs.txt
.venv-docs/Scripts/python -m mkdocs serve

# 发布：push main 即自动构建部署（.github/workflows/docs.yml）；
# 只想覆盖发布、不提交源码时，用下面这条直推 gh-pages
.venv-docs/Scripts/python -m mkdocs gh-deploy --force
```

新增页面时在 `mkdocs.yml` 的 `nav` 里补一行即可。

## 📄 开源协议

本仓库以 **[GNU AGPL-3.0](LICENSE)** 授权。

选择 AGPL-3.0 的原因：`oa4rust` 在重写过程中参考了 [O2OA](https://github.com/o2oa/o2oa)（AGPL-3.0）的源码与接口契约，作为其衍生作品，沿用上游协议以保持授权一致。

> **上游归属**：oa4rust 相关部分源于对 O2OA 的重写，原始版权归 O2OA 项目及其贡献者所有。

---

<div align="center">

**fakeToys** —— 名字叫「玩具」，里面的东西大多认真在做。

<sub>自绘 SVG · 无第三方服务 · 数据均来自仓库实测</sub>

</div>
