<div align="center">

# 🧸 fakeToys

**一个装着若干「不那么玩具」的玩具箱**

从 Rust 重写的企业级 OA 后端，到塔防元素反应小程序游戏、AI 训练数据增强工具、自动签到面板、免登 Token 生成器——
几个互相独立、各自能跑的实验性项目，共处一个 monorepo。

<br/>

![Rust](https://img.shields.io/badge/Rust-1.98+-000000?logo=rust&logoColor=white)
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

这是一个多语言、多技术栈的 monorepo。每个子项目自成一体，可单独构建、单独运行。

| 子项目 | 简介 | 技术栈 |
| --- | --- | --- |
| 🦀 **[oa4rust](oa4rust/)** | Rust 重写的企业 OA 系统，97 个 crate workspace；前端桌面/移动双端，bundle 优化至 96KB；跨账号缓存隔离、TypeScript strict | Rust · Axum · Vue3 · uni-app · PostgreSQL |
| 💥 **[laiyipao](laiyipao/)** | 塔防+元素反应小程序游戏，CI 全面落地（Go/Vue/uni-app 三端自动化测试）；确定性战斗引擎、回放验真机制 | Go · Fiber · uni-app · Vue3 · PostgreSQL |
| 🧪 **[augmentor](augmentor/)** | AI 训练数据增强工具 v3.0，68 个 REST 端点、7872 单测 97% 覆盖；React 11 页面、配置写入闭环、性能优化 | Python · FastAPI · React · Ant Design |
| ✅ **[auto-checkin](auto-checkin/)** | 大模型公益站自动签到面板，三站点（vyceai/agentrouter/justwoker）成熟；自动化测试+诊断完整、隐私本地化 | Node · Playwright · Express |
| 🧠 **[llm-lab](llm-lab/)** | 从零学大模型训练教学项目，11 篇中文讲义 + 32 个验证用例；手写 autograd + GPT 训练栈、梯度校验机制 | Python · NumPy · PyTorch |
| 🔐 **[cool](cool/)** | Cool College OA 免登 Token 生成工具 | Java · AES/MD5 |

---

## 🦀 oa4rust

用 Rust 重写的企业 OA 系统（源自 O2OA），追求更强的性能、内存安全与更低的资源占用。

**架构特点**
- 后端：Axum + Tokio 异步栈，PostgreSQL 持久化，**97 个 crate** 的 Cargo workspace
- 前端：pnpm workspace 统一管理桌面端（Vue3 + Vite）和移动端（uni-app）
- 类型驱动 SDK：统一 API 客户端服务两端，跨账号缓存隔离保护
- 质量保障：TypeScript strict 模式、Vitest 单测、Playwright E2E、三端契约守卫

**工程成果**
- 前端 main bundle 优化：1124KB → 96KB（-91%）
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

---

## 💥 laiyipao

仿《向僵尸开炮》玩法的塔防+元素反应小程序游戏，美术 100% Canvas 程序化绘制。

**核心创新**
- 元素反应矩阵：攻击力权重结构性 ≤30%，堆数值无法绕过搭配收益
- 确定性战斗：定点整数 + 确定性 PRNG，回放哈希验真
- 地形机制：5 类可交互地形（油桶/潮汐闸/风障/掩体/蓄能塔），41% 关卡使用
- 防线值守：构筑快照本地模拟，服务端不跑战斗引擎

**工程完善**
- CI 全面进仓：Go server / Vue admin / uni-app miniapp 三端自动化测试
- 48 张业务表 + goose 迁移、端到端验收脚本
- 双端契约守卫、公式一致性测试、100 关零死锁验证

```bash
cd laiyipao/server
export DATABASE_URL="postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"
go run ./cmd/migrate && go run ./cmd/seed && go run ./cmd/api
```

> 详见 [laiyipao/README.md](laiyipao/README.md)

---

## 🧪 augmentor

AI 训练数据增强工具，将少量种子数据扩充为高质量多轮问答对。**v3.0** 已发布。

**核心能力**
- 多模型支持：百度 ERNIE、OpenAI、Ollama、Claude、Gemini
- 质量控制：语义相似度、回答相关性、多样性多维评分 + 向量去重
- 数据安全：隐私脱敏、训练/测试泄漏检测、就绪审计
- 多格式导出：JSONL、Llama-Factory、Alpaca、ShareGPT、ChatML、CSV

**v3.0 升级**
- 性能：流式处理峰值 22.70MB→1.66MB、dedup 内存 9536MB→20MB
- 健壮性：7872 单测 97% 覆盖、错误分类、自动重试、配置写入闭环
- API：68 个 REST 端点全部声明 `response_model`
- Web：React 11 个功能页 + Ant Design

```bash
cd augmentor
pip install -r requirements.txt
uvicorn api.main:app --port 8000
```

> 详见 [augmentor/README.md](augmentor/README.md)

---

## ✅ auto-checkin

大模型公益站自动签到面板，用真实浏览器绕过反爬，带本地统计面板。

**已接入站点**
- VyceAI：自定义 API + PoW 工作量证明
- AgentRouter：阿里云 WAF，GitHub OAuth
- JustWoker：Cloudflare Turnstile，GitHub OAuth

**特性**
- 自动化测试：三站点测试 + 截图诊断 + 领奖验证
- 隐私优先：账号密码只存本地 `.env`，面板默认仅监听 127.0.0.1
- 断签保护：vyceai 端点迁移修复、日志/余额追踪完整

```bash
cd auto-checkin
npm install && npm run setup:browser
npm start  # 启动定时签到 + Web 面板
```

> 详见 [auto-checkin/README.md](auto-checkin/README.md)

---

## 🧠 llm-lab

从零学大模型训练教学项目，可运行、可验证、带完整中文讲义。

**两条代码轨**
- **minigrad**：纯 NumPy 手写 autograd + GPT，任何环境可跑
- **torchgpt**：PyTorch 复刻 nanoGPT，演示 AMP/梯度累积/LoRA/DPO

**学习资源**
- 11 篇中文讲义：分词、注意力、Transformer、训练、微调、对齐、评估
- 32 个验证用例：数值梯度校验、KV cache 等价性、端到端冒烟测试
- 可视化：注意力热力图、尺度实验（参数量 vs loss）

```bash
cd llm-lab
pip install -r requirements.txt
python -m minigrad.train  # loss ~4.1 → ~0.3
pytest tests/ -q          # 32 passed, 1 skipped
```

> 详见 [llm-lab/README.md](llm-lab/README.md)

---

## 🔐 cool

Cool College OA 免登 Token 生成工具，支持 AES 加密/解密、MD5、Base64，兼容 .NET 的 AES-CBC 算法。

> 详见 [cool/README.md](cool/README.md)

---

## 🗂️ 仓库结构

```
fakeToys/
├── oa4rust/          # 🦀 Rust OA 后端 + 前端双端（97 crates + Vue3 + uni-app）
├── laiyipao/         # 💥 塔防元素反应游戏（Go server + uni-app + Vue admin）
├── augmentor/        # 🧪 AI 数据增强工具（Python + FastAPI + React）
├── auto-checkin/     # ✅ 自动签到面板（Node + Playwright）
├── llm-lab/          # 🧠 大模型训练教学（Python + NumPy/PyTorch）
├── cool/             # 🔐 免登 Token 生成（Java）
├── docs/             # 📚 方案/评审/审计文档
├── scripts/          # 🛠️ 通用脚本
└── .github/workflows # 🤖 CI 配置
```

## 🚀 快速开始

各子项目独立运行，请进入对应目录按其 README 操作：

- **oa4rust**：Rust 1.98+、PostgreSQL 14+、Node + pnpm
- **laiyipao**：Go 1.26+、PostgreSQL 15+、Node + pnpm
- **augmentor**：Python 3.10+
- **auto-checkin**：Node 20+ + Playwright
- **llm-lab**：Python 3.10+（torch 可选）
- **cool**：JDK 8+

## 📄 开源协议

本仓库以 **[GNU AGPL-3.0](LICENSE)** 授权。

选择 AGPL-3.0 的原因：`oa4rust` 在重写过程中参考了 [O2OA](https://github.com/o2oa/o2oa)（AGPL-3.0）的源码与接口契约，作为其衍生作品，沿用上游协议以保持授权一致。

> **上游归属**：oa4rust 相关部分源于对 O2OA 的重写，原始版权归 O2OA 项目及其贡献者所有。

---

`fakeToys` 是一个个人实验性项目集合——名字叫「玩具」，但里面的东西大多认真在做。
