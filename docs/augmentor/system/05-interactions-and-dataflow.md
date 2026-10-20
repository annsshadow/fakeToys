# 05 · 端间交互与核心业务流程

> 本篇回答「**三端如何协作、数据/请求如何流动、关键接口与数据结构长什么样**」。
> 端内细节见 [`02`](02-server.md) / [`03`](03-client-web.md) / [`04`](04-cli.md)。

---

## 1. 三端关系总览

**核心引擎是唯一的实现来源，三端共享同一套行为**——这是平台最重要的架构事实：

```
                ┌─────────────────────────────────────┐
                │  核心引擎  augmentor/ (能力所在)        │
                │  pipeline / models / quality / dedup  │
                │  versioning / export / vector / …      │
                └───────▲──────────────▲───────────▲────┘
                         │              │           │
            (import)     │  (import)   │  (HTTP)   │  (import)
                         │              │           │
          ┌──────────────┴───┐  ┌──────┴───────┐  ┌┴──────────────┐
          │  CLI  cli.py     │  │ REST  api/    │  │ Web   web/    │
          │  37 子命令        │  │  77 端点       │  │  11 页面       │
          │  离线/脚本/CI      │  │  程序/他系统     │  │  浏览器人      │
          └──────────────────┘  └────────────────┘  └─────────────┘
```

| 驱动方式 | 路径 | 是否经 REST | 典型场景 |
|----------|------|:-----------:|----------|
| CLI | `cli.py → import 引擎` | 否 | 离线批处理、CI 门禁、脚本 |
| REST | `client → api/ → import 引擎` | 是 | 其它系统、自动化、Web UI |
| Web | `web/ → axios(/api) → api/ → import 引擎` | 是 | 人工操作台 |

> **没有移动端**；浏览器 Web UI 是唯一直面人的图形入口。

---

## 2. 客户端 ↔ 服务端（Web 主线）

这是**最主要的人机链路**，请求逐层下沉：

```
浏览器页 (pages/*.tsx)
   │ 调 services/api.ts 的某函数（axios 实例，baseURL "/api"）
   │ 写操作拦截器：若 localStorage 有 key，则附 X-API-Key 头
   ▼
服务端 FastAPI (api/main.py)
   │ 中间件链（自外向内）：
   │   RequestTrace(X-Request-ID) → RequestLogging → CORS
   │   → 上传尺寸闸(413) → 匿名写闸(403) → 限流(429)
   ▼
router (api/routes/*.py)
   │ Depends(verify_api_key)  —— 设了 AUGMENTOR_API_KEY 时，写操作缺 key → 401
   │ resolve_data_path(白名单) —— 路径越界 → 403，读时不存在 → 404
   ▼
get_pipeline() → AugmentorPipeline 方法 / 能力模块
   ▼
本地 JSON 数据 + 模型后端(LLM) + 向量库(faiss/chromadb)
   ▼
按 response_model 过滤后返回（所有 77 端点都声明契约）
```

**鉴权语义**（易混点，务必记牢）：
- **未设** `AUGMENTOR_API_KEY`：写操作**不鉴权**，但 `匿名写闸` 只放行**回环地址**来的写（非本机 → 403）。零配置可用（容器探针/本地/CI 冒烟依赖它）。
- **已设** `AUGMENTOR_API_KEY`：所有写操作要求 `X-API-Key` 匹配，否则 401。Web 端把 key 存浏览器 `localStorage`（明文、单机、非保密措施）。

**数据坐标契约**：读侧分页回 `indices`（盘上 0 基下标），写侧 `PUT/DELETE` 用 `indices` 而非页内下标。

---

## 3. 核心业务流程（端到端，主线）

```
┌─ 阶段 0 · 数据准备 ─────────────────────────────────────────────┐
│ 上传/载入 (POST /api/data/upload, GET /api/data/list)           │
│   → 清洗 (POST /api/quality/clean) / 校验 (POST /api/dataset/validate)│
│   → 格式转换/合并/采样/分割 (dataset 工具族)                        │
└──────────────────────────────────────────────────────────────────┘
                              │
┌─ 阶段 1 · 数据增强 ──────────────────────────────────────────────┐
│ 启动 (POST /api/augment/start)                                   │
│   → LLM 生成变体 (models/factory) + 多轮上下文 (context)           │
│   → 质量评分 (QualityScorer 三维加权) + 过滤                        │
│   → 向量去重 (Deduplicator / vector)                               │
│   → 断点续传 (CheckpointManager, auto_save_interval)              │
│   → 轮询进度 (GET /api/augment/progress)                          │
│   → 自动版本快照 (versioning.auto_snapshot)                        │
└──────────────────────────────────────────────────────────────────┘
                              │
┌─ 阶段 2 · 质量与安全 ───────────────────────────────────────────┐
│ 报告/基准/离群/画像 (quality 族)                                  │
│ 健康门禁 (POST /api/quality/health-gate) → go/no-go               │
│ 脱敏 (POST /api/privacy/sanitize)                                 │
│ 泄漏检测 (POST /api/leakage/check)                                │
│ 就绪审计 (POST /api/audit) → go/no-go 综合判决                     │
└──────────────────────────────────────────────────────────────────┘
                              │
┌─ 阶段 3 · 导出与交付 ───────────────────────────────────────────┐
│ 多格式导出 (POST /api/export, /api/export/batch, /preview)        │
│ RAG 格式 (POST /api/dataset/rag)   翻译 (POST /api/dataset/translate)│
│ 版本回滚 (POST /api/versions/{id}/rollback)                        │
└──────────────────────────────────────────────────────────────────┘
```

**CLI 等价驱动**（同一条主线，走引擎不走 REST）：
`augment → quality(-report) → sanitize / check-leakage / audit → export / rag`，
报告形命令加 `--gate` 即成 CI 门禁。

---

## 4. 关键数据结构（跨端一致）

### 4.1 数据集记录（平台第一等公民）

```json
{"instruction": "用户提问", "output": "客服回答", "input": ""}
```
多轮追加：
```json
{"history": [{"role":"user","content":"…"},{"role":"assistant","content":"…"}],
 "system": "你是…智能客服"}
```
**形态契约**：顶层数组、元素对象（`assert_dataset_shape` 在读/上传两侧共用，违反 → 400）。

### 4.2 质量三维评分

`QualityScorer` 输出 `[语义相似度, 回答相关性, 多样性]`，按 `quality.weights`
（默认 `[0.3,0.4,0.3]`）加权得总分；低于 `quality.threshold`(0.6) 过滤。

### 4.3 断点

任务 ID `augment_<md5(输入路径)前12位>`；断点按 `auto_save_interval` 增量落盘，
已完成条目跳过、上一轮输出合并而非覆盖。重跑前可 `GET /api/augment/checkpoints` 查看。

### 4.4 版本

`DatasetVersionManager` 独立 `--versions-dir`；`history.jsonl` 记 create/delete/rollback，按时间倒序。

### 4.5 依赖关系

`DependencyManager` 把数据集登记进注册表并记录依赖边，`GET /api/system/dependency/graph`
出依赖图。**注意**：默认注册表目录 CLI 与 API 不同（见 [`04 § 5`](04-cli.md)）。

---

## 5. 横切机制（影响所有端）

| 机制 | 作用 | 关键配置 |
|------|------|----------|
| **数据白名单** | 客户端路径 resolve 后须落在白名单根，否则 403 | `web.data_roots`（默认 `["data"]`）、env `AUGMENTOR_DATA_ROOTS` |
| **写操作鉴权** | 设 key 后写操作要求 `X-API-Key` | `AUGMENTOR_API_KEY`（环境变量，非配置项） |
| **限流** | 单客户端窗口内超限 → 429 + `Retry-After` | `web.rate_limit_max_requests` / `window_seconds` / `exempt_paths` |
| **上传体闸** | 解析前判 `Content-Length` → 413（未落盘） | `web.max_upload_bytes`（默认 256MiB） |
| **原子写** | 整份数据集落盘唯一咽喉，读者看不到半份 | `atomic_write_json`（`deps._sync_write_json` 收敛） |
| **读-改-写加锁** | 同路径读写互斥，避免并发丢更新 | `deps.mutate_json_file`（`path_lock`，仅同进程） |

---

## 6. 故障与降级（端间一致性视角）

- **500 不回传原文**：未分类异常收敛为固定文案 `INTERNAL_ERROR_DETAIL`，
  原文只落服务端日志（防部署路径/文件系统状态外泄）。`to_http_error` 与 `raise_internal_error` 共用此口径。
- **400 可行动**：参数/数据不合法时照回原文（可修复）；但「OS 拒绝的名字形状」（errno 22/36）
  回固定文案 `INVALID_PATH_NAME_DETAIL`。
- **可选依赖降级**：无语义模型 → n-gram 相似度；无 faiss → numpy 精确检索；
  无 matplotlib/wordcloud → 跳过图表返回空路径。**降级不崩进程**。
- **配置损坏**：读 `web.data_roots` / `max_upload_bytes` 失败 → 记 warning 并回落出厂默认（不放宽边界）。
