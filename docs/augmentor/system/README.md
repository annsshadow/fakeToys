# AI 训练数据增强平台 — 系统文档（分端总纲）

> 本目录（`docs/augmentor/system/`）是 **按「端」组织的系统级文档集**，面向开发、测试与维护人员，
> 用于快速理解 **服务端 / 客户端（Web 管理端）/ 命令行端（CLI）** 三端的整体架构、模块划分、
> 核心业务流程、关键接口与数据结构、配置与部署，以及端与端之间的交互关系。
>
> 内容全部以 `augmentor/` 仓内**当前代码**为准；若与代码不一致，**以代码为准**并在代码侧提交修正。
> 仓内另有几份深度专项文档（与本目录互补，不重复）：
> - [`docs/augmentor/README.md`](../README.md) — 使用指南 / 快速上手
> - [`docs/augmentor/ARCHITECTURE.md`](../ARCHITECTURE.md) — 逐层架构与审计账本（体量大，侧重缺陷追溯）
> - [`docs/augmentor/API.md`](../API.md) — 77 个 REST 端点逐条契约
> - [`docs/augmentor/DEPLOYMENT.md`](../DEPLOYMENT.md) — 部署细节
>
> 本目录聚焦「**端**」这一视角，把上述材料重新组织成可查阅的系统手册。

---

## 0. 这是什么平台

一个用于 **AI 模型训练数据增强** 的工程平台，典型场景是房产客服问答。
它把**少量种子数据**（JSON 问答对）经 LLM 扩充为**大量多样化变体**，并在
**质量控制、去重、版本管理、多格式导出、多模态、隐私脱敏、泄漏检测、就绪审计**等
数据工程环节上提供一致的能力面。

同一套能力可通过 **三种端** 驱动，行为一致（详见 [`05-interactions-and-dataflow.md`](05-interactions-and-dataflow.md)）：

| 端 | 位置 | 形态 | 主要用户 |
|----|------|------|----------|
| **服务端** | `augmentor/` + `api/` | Python 核心引擎 + FastAPI REST | 程序、其他系统、本仓其它端 |
| **客户端（Web 管理端）** | `web/` | React + TypeScript 浏览器 SPA | 数据 / 算法 / 运维人员 |
| **命令行端（CLI）** | `cli.py` + `augmentor/cli/` | argparse 子命令 | 脚本、CI、批处理 |

> 平台**没有独立移动端**（无 React Native / Flutter / Ionic / 原生 App 目录或产物）。
> 浏览器端 Web UI 是唯一的图形化入口；若需移动端，应基于服务端 REST 另行开发。

---

## 1. 文档索引（本目录）

| 文件 | 内容 | 适合读者 |
|------|------|----------|
| [`01-overview-and-terminology.md`](01-overview-and-terminology.md) | 平台定位、三端划分、分层架构、**术语表** | 所有人（先读） |
| [`02-server.md`](02-server.md) | 服务端：核心引擎模块划分、REST API 结构、中间件、配置中心、目录结构 | 后端 / 维护 |
| [`03-client-web.md`](03-client-web.md) | 客户端 Web 管理端：页面、服务层、类型、测试守门、与后端的接线 | 前端 / 维护 |
| [`04-cli.md`](04-cli.md) | 命令行端：37 个子命令分组、退出码约定、输出契约 | 脚本 / CI / 运维 |
| [`05-interactions-and-dataflow.md`](05-interactions-and-dataflow.md) | 端与端交互、**核心业务流程**、关键接口与数据结构 | 所有人 |
| [`06-deployment-and-ops.md`](06-deployment-and-ops.md) | 配置与部署（裸机 / Docker / Compose）、环境变量、安全、运维、测试 | 运维 / 交付 |

**建议阅读顺序**：`01 → 05 → 02/03/04（按角色取）→ 06`。

---

## 2. 关键路径速查

```
augmentor/
├── augmentor/            # ★ 核心引擎（纯 Python，不依赖 Web 框架）
│   ├── pipeline.py       #   编排中枢 AugmentorPipeline
│   ├── config.py         #   dataclass 配置中心 + YAML/环境变量解析
│   ├── models/           #   LLM 后端（工厂：ernie/openai/ollama/claude/gemini）
│   ├── vector/           #   向量库（工厂：faiss/chromadb，缺失回退 numpy）
│   ├── data/             #   数据加工（cleaner/annotator/image/audio/multimodal）
│   ├── frameworks/       #   LLM 框架互转（langchain / llamaindex）
│   ├── cli/              #   CLI 实现（parser + commands/ 九组 + verdict）
│   └── *.py              #   质量/去重/导出/版本/审计/脱敏/泄漏/检索/备份… 能力模块
├── api/                  # ★ REST 层（FastAPI 薄适配）
│   ├── main.py           #   应用装配（中间件 + 路由 + 静态挂载 + 异常兜底）
│   ├── deps.py           #   管道单例、路径白名单、鉴权、异步 IO、异常映射
│   ├── middleware/       #   限流 / 请求追踪 / 请求日志 / 上传尺寸闸 / 匿名写传输闸
│   └── routes/           #   15 个按领域拆分的 router（77 端点，全部声明 response_model）
├── web/                  # ★ 客户端 Web 管理端（React 19 + TS + Ant Design 6 + Vite 8）
│   ├── src/pages/        #   11 个功能页
│   ├── src/services/     #   api.ts（axios 封装 + 写操作鉴权拦截器 + 接线守门）
│   └── src/types/api.ts  #   与后端契约对齐的 TS 类型
├── cli.py                # ★ 命令行入口（薄壳：解析 → 查分发表 → handler）
├── config.yaml           # 默认配置
├── requirements*.txt     # 依赖分层（core / 全量 / dev）
└── docker/              # Dockerfile（多阶段）+ docker-compose.yml + .env.example
```

> 各端更细的目录说明见 [`02-server.md § 目录结构`](02-server.md)、
> [`03-client-web.md § 目录结构`](03-client-web.md)、[`04-cli.md § 目录结构`](04-cli.md)。

---

## 3. 与代码的一致性声明

- 版本单一来源：`augmentor/__init__.py` 的 `__version__`（当前 **3.0.0**），全仓唯一版本声明。
- 端点数量与契约：以 `api/main.py` 注册的 15 个 router + `/api/health` 为准，
  **全部端点都声明 `response_model`**；逐条契约详见 [`docs/augmentor/API.md`](../API.md)。
- CLI 子命令数量与退出码：以 `augmentor/cli/parser.py` 与 `augmentor/cli/verdict.py` 为准（**37 个规范命令**）。
- 配置键清单：以 `augmentor/config.py` 的 `AppConfig.fields()` 为准，`config.yaml` 出厂值与之对账。
