# augmentor 优化循环进度追踪（2026-09-23 启动，目标 100 轮）

约定（沿用 `.backups/opt100_progress.md` 那次的做法）：每轮 = 一次「实测定位 → 整改 →
全量测试 → 提交」，红→绿必须先验（把改动退回缺陷态，新用例必须失败），全绿才提交。

- 基线（L0 起点）：**3668 passed / 3 skipped，覆盖率 98.55%**，`pytest -q` 约 50–95 s
- 测试解释器：`C:/Users/Administrator/.workbuddy-ai/binaries/python/envs/aug/Scripts/python.exe`，
  cwd = `augmentor/`，`PYTHONIOENCODING=utf-8`
- 硬约束：不往仓库工作目录写产物；不擅自搬动用户数据；破坏性默认值变更需逐项确认

## 已完成

- [x] **L1** `fix(api)`: F-09 依赖注册表缺省目录跟白名单走 + 测试不再污染版本树（`d5601369b`）
- [x] **L2** `fix(api)`: F-10 备份目录缺省值、F-11 服务配置文件路径收敛到 `deps.config_file_path()`（`13ec11eaa`）
- [x] **L3** `perf(analytics)`: A1 多样性均值提出生成器 → 长度方差 O(n²)→O(n)
- [x] **L4** `perf(api)`: A2 白名单两条来源都按「值会变的东西」建缓存 → 7.06 ms/次 → 0.19 ms/次

## Backlog A — 性能（含 file:line 与实测线索）

| # | 位置 | 问题 | 量级 |
|---|------|------|------|
| ~~A1~~ | ~~`augmentor/analytics.py:368`~~ | ~~均值在生成器里重算 → O(n²)；实测 n=20000 时 **1043 ms vs 2 ms**~~ 已修（L3） | S |
| ~~A2~~ | ~~`api/deps.py` `allowed_data_roots()`~~ | ~~每个路径参数都 `load_config()` 重新解析 YAML，实测 **6.8 ms/次**~~ 已修（L4，含环境变量分支 0.29 ms/次） | S |
| A3 | `augmentor/analytics.py:403-412` | `get_duplicate_candidates()` 纯 O(n²) 且每对重建字符集合 | M |
| A4 | `api/routes/quality.py:141,185,217,253,285,309,370,403` + `audit.py:31` `export.py:109,132` `leakage.py:46` `privacy.py:52` | `async def` 里同步 `load_items()` 阻塞事件循环，应走 `run_in_thread` | M |
| A5 | `augmentor/pipeline.py:205-210` + `quality.py:133,153` | 逐条 `encode()`/`predict()` 打分，已有 `batch_score()` 却没用 | M |
| A6 | `augmentor/dedup.py:94-123,242-290` | 回退路径构造 **稠密 float64** n×vocab 矩阵（大词表 OOM）；应用稀疏 CSR / faiss range_search | L |
| A7 | `augmentor/search_enhanced.py:448,82` | 每次查询重建索引（n=3000 重建 8 ms vs 查询 2 ms），且 contains/fuzzy 不查索引 | M |
| A8 | `api/routes/data.py:146-164` | 整文件解析 + 全量过滤后再切片，无早停 | M |
| A9 | `augmentor/tracker.py:114` → `:87-99` | 每个指标点整文件重写 → O(points²) 字节 | S |
| A10 | `augmentor/quality.py:175-179` | 多样性回退里参照文本 n-gram 集合反复重建 | S |
| A11 | `augmentor/indexer.py:117-123,345,352,372-399` | 每次 `DatasetView` 操作重建全部索引（filter 26 ms @ n=3000） | M |
| A12 | `augmentor/validation.py:217-222`、`sampler.py:296-299` | 循环内未缓存的正则、`list.index`/`in` 线性扫描 | S |

## Backlog B — 功能增强（价值 ÷ 工作量）

| # | 内容 | 证据 | 量级 |
|---|------|------|------|
| B1 | 把 `/api/system/*`(13) 与 `/api/dataset/*`(12) 接入 UI | `web/src/services/api.ts` 对二者零引用；`System.tsx:3` 只用了 status | M |
| B2 | Excel/CSV 摄取端到端（上传 + `convert --input-format xlsx`） | SDK `csv_excel_import.py:61,152` 可达性为零；`data.py:236-243` 只认 JSON | M |
| B3 | 反向转换边 `alpaca/sharegpt/chatml/vicuna/belle/tsv → json` | `converter.py:60-66` 只读 json/jsonl/csv，导出侧 13 种 → 不可回环 | M |
| B4 | `QualityGate` / `DatasetHealthScore` 暴露为 CLI 子命令 + 端点 | 两者在 `augmentor/__init__.py:38+` 导出但无人可达 | S |
| B5 | 死配置项：读取或显式废弃（`config.py:75-167` 的 sampler/expander/tracker/visualization/multilingual/evaluation/vector/active_learning 等） | 无代码路径读取 | M |
| B6 | `/api/dataset/evaluate`（BLEU/ROUGE）+ `/api/dataset/impact`（前后对比） | `evaluation.py:74,155,211`、`impact.py:49,96` 纯函数已就绪 | S |
| B7 | 主动学习 + 实验跟踪回路（`active_learning.py:46`、`tracker.py:29`） | 公开导出但无 CLI/API | L |
| B8 | 前端 5 个未接线服务函数 + 运行时 schema 校验 | `api.ts:141,152,175,180,206` | M |

## 循环日志

（每轮追加一行：`L<n>` 提交哈希 · 做了什么 · 实测数字 · 测试计数）

- **L3** `e7e051279` `perf(analytics)` A1 —— `_calculate_diversity_score()` 长度方差把均值提出生成器，
  O(n²)→O(n)。n=20000 实测 **1147.7 ms → 2 ms**（不带 trace）/ **1130 ms → 143 ms**（带 trace）。
  新增 `tests/unit/test_analytics_performance.py` 2 例：一条锁语义（手算 0.6/0.4 权重对照），
  一条锁量级（预算 500 ms；docstring 说明预算为何按**带覆盖率 trace** 的实测值取，而不是 2 ms）。
  红→绿：把均值塞回生成器 → 预算用例红在 1130.3 ms。
- **L4** `perf(api)` A2 —— `allowed_data_roots()` 两条来源都加缓存：config 分支
  `(路径, mtime_ns)` 键、环境变量分支 `(原值, os.getcwd())` 键。实测 7.06 ms/次 → **0.19 ms/次**、
  0.29 ms/次 → **0.0006 ms/次**。新增 `TestConfigDataRootsCache` 4 例 + `TestEnvDataRootsCache` 4 例；
  `docs/ARCHITECTURE.md` §3.12.1 补「缓存键都带值会变的东西」一节。红→绿两处注入：
  ①缓存键去掉 cwd → 相对根目录换目录用例红（`first/data` ≠ `second/data`）；
  ②关掉缓存读 → 「200 次查询解析了 201 次配置」红。
- 全量：**3679 passed / 3 skipped**（89.2 s，覆盖率门禁通过；基线 3668）。

> **操作纪律**（L4 踩过）：验红用的是**定向反向 patch**，绝不用 `git checkout <file>` 撤注入 ——
> 本轮 `api/deps.py` 有未提交工作，一次 `git checkout` 把整段缓存实现清掉了，只能重写。
