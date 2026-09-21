---
title: "audit: augmentor 多端全面优化与整改规划"
type: audit
status: active
date: 2026-09-21
scope: augmentor/ (Python SDK + CLI + REST API + Web UI + Docker/CI)
origin: 现场审计（实测基线：3061 passed / 99.46% coverage / 91.73s）
---

# augmentor 多端全面优化与整改规划

## Summary

对 `augmentor/` 做了一次带实测的全面审计，共确认 **12 项可复现问题**（5 项正确性/安全、7 项工程质量），
并识别出 **多端能力严重不对称**这一结构性缺口。

三条最重要的结论：

1. **API 的质量评估结果是不可信的**。`POST /api/quality/evaluate` 对一条"答案与问题完全无关"的样本
   给出 `total_score=0.600`、`passed=True`。原因是 `original` 与 `generated` 取了同一个字段，
   导致占 30% 权重的语义相似度恒为 `1.0`。**通过率虚高，前端质量页与质量报告全部失真。**
2. **存在两处静默的符号遮蔽**，`augmentor.QualityReport` 指向的类与 `ReportGenerator.generate()`
   实际返回的类不是同一个，`isinstance()` 判定为 `False`，类型契约已断裂且无测试发现。
3. **REST API 存在任意文件读写**。项目目录之外的 JSON 文件可被 `POST /api/quality/evaluate`
   以 200 正常读取（实测 `total_samples=7`）。同时已实现并测试过的限流中间件**根本没有注册**。

本规划按「先止血、再固本、后扩展」分 4 个阶段，每个任务都给出文件位置、具体改法、验收标准与验证命令。

---

## 实测基线（本规划全部结论的证据来源）

| 指标 | 实测值 | 采集方式 |
|------|--------|---------|
| 测试结果 | 3061 passed, 3 skipped | `pytest -q` |
| 语句覆盖率 | **99.46%**（10421 stmts / 56 miss） | `pytest --cov=augmentor --cov=api --cov=cli` |
| 测试耗时 | 91.73s | 同上 |
| 核心代码规模 | 23723 行（`augmentor/` + `api/`） | `find ... \| xargs wc -l` |
| CLI 规模 | 1308 行，44 个子命令 | `wc -l cli.py` |
| 测试规模 | 245 个测试文件 / 33620 行 / 3061 个测试 | `wc -l tests/**/*.py` |
| API 路由 | 44 个（11 个 router） | `grep '@router\.' api/routes/*.py` |
| Web 页面 | 9 个页面 + 5 个组件（2835 行） | `web/src` |
| 覆盖率最低模块 | `csv_excel_import.py` **24%** | coverage 报告 |

> 关键反差：覆盖率 99.46%，却漏掉了本规划 F1/F2/F3 三个真实缺陷。
> 说明当前测试在**度量语句执行**，而非**验证行为**——详见 F6。

---

## Problem Frame

### P0 组：正确性与安全（阻断级，必须先修）

#### F1 — API 质量评分退化：语义相似度恒为 1.0，垃圾数据判 pass

**位置**：`api/routes/quality.py:28-38`（`_build_scoring_items`）

```python
return [
    {
        "original": item.get("instruction", ""),   # ← 同一个字段
        "generated": item.get("instruction", ""),  # ← 赋给两个语义相反的入参
        "output": item.get("output", "")
    }
    for item in items
]
```

**根因**：`QualityScorer.batch_score`（`augmentor/quality.py:235-330`）中，
`original` 是「原始种子问题」、`generated` 是「生成的变体问题」，二者语义相似度是三个评分维度之一
（权重 `0.3`）。此处把同一个值传给两者，相似度恒等于 `1.0`。

**实测证据**：

```
样本: instruction="怎么申请租房？"  output="今天天气不错哈哈哈"（答案与问题完全无关）
→ 语义=1.000  相关性=0.000  多样性=1.000  总分=0.600  passed=True   ← 通过了 0.6 阈值

对照（pipeline.py:178-179 的正确做法，generated 用真实变体文本）：
→ 语义=0.143  总分=0.343   ← 正确地被过滤
```

**影响面**：`POST /api/quality/evaluate`、`POST /api/quality/report`（经 `_build_scoring_items`）
→ 前端 `web/src/pages/Quality.tsx` 的通过率、均分、分桶分布全部虚高约 `0.3` 分。

**修复方向**：
- 若输入数据保留种子来源（如 `source_instruction` / `seed_id`），用它作为 `original`；
- 若无法还原种子，**降级为二维评分**（相关性 + 多样性），按剩余权重 `[0.4, 0.3]` 重归一化，
  并在响应中显式标注 `"semantic_skipped": true`，而不是塞一个假值；
- 在 `QualityScorer` 中加断言：`original == generated` 时记录 WARNING，避免同类问题再次发生。

**验收**：无关答案的 `total_score < threshold`；新增回归测试在修复前必须失败。

---

#### F2 — 公开 API 符号被静默遮蔽（`QualityReport` / `ValidationResult`）

**位置**：`augmentor/__init__.py`

```python
from .report import ReportGenerator, QualityReport          # 第 15 行
...
from .quality_report import QualityReporter, QualityReport, ...  # 第 37 行 ← 覆盖前者
```

`augmentor/__init__.py` 中 **`QualityReport` 与 `ValidationResult` 各被导入两次**，
后一次静默覆盖前一次。

**实测证据**：

```
augmentor.QualityReport.__module__    -> augmentor.quality_report    （不是 augmentor.report）
augmentor.ValidationResult.__module__ -> augmentor.config_validator  （不是 augmentor.validation）

ReportGenerator.generate() 返回类型 -> augmentor.report.QualityReport
isinstance(r, augmentor.QualityReport) -> False        ← 类型契约断裂
augmentor.QualityReport().pass_rate    -> AttributeError（该字段只在 report.QualityReport 上）
```

**影响面**：任何 `from augmentor import QualityReport` 后做类型标注/`isinstance` 判断的调用方
（含 `api/` 与 `cli.py`）会拿到错误的类。两个 `QualityReport` 字段完全不同：
- `report.QualityReport`：`total_samples / pass_rate / score_distribution / improvement_suggestions`
- `quality_report.QualityReport`：`dataset_name / timestamp / metrics / overall_score`

**修复方向**：重命名为语义明确的独立名字（推荐 `ReportQualityReport` 与 `ConfigValidationResult`），
保留旧名作为 `DeprecationWarning` 别名过渡一个版本；`__init__` 中禁止同名二次导入。

**验收**：新增测试断言 `augmentor.QualityReport.__module__ == "augmentor.report"`（或改名后的确定值），
且 `isinstance(ReportGenerator().generate(...), augmentor.QualityReport) is True`。

---

#### F3 — REST API 任意文件读写（路径穿越 + 绝对路径逃逸）

**位置**：`api/deps.py:98-118`（`require_file` / `load_items` / `save_items`）、`api/routes/data.py:14-21`（`_safe_data_path`）

```python
def require_file(filename: str) -> Path:
    file_path = Path(filename)
    if not file_path.exists():      # ← 只判存在，无任何根目录约束
        raise HTTPException(status_code=404, detail="文件不存在")
    return file_path
```

`_safe_data_path` 只拦截 `..` 组件，但**不限定基目录**，绝对路径可直接穿透。

**实测证据**：在用户主目录（项目之外）放置一个 7 条记录的 JSON 文件后：

```
POST /api/quality/evaluate  {"input_file": "C:\\Users\\<user>\\OUTSIDE_PROJECT_SECRET.json"}
→ 200  {"total_samples": 7, "passed_samples": 1, "pass_rate": 0.14, ...}
```

`total_samples=7` 即证明项目目录之外的文件被成功打开并解析。

**受影响路由**（全部走 `load_items`，无校验）：

| 路由 | 风险 |
|------|------|
| `POST /api/quality/{evaluate,dedup,report,clean,annotate,benchmark,outliers,profiling}` | 任意读 |
| `POST /api/audit`、`POST /api/privacy/sanitize`、`POST /api/leakage/check` | 任意读 |
| `POST /api/export/preview`、`POST /api/export/batch` | 任意读 |
| `POST /api/data/export`、`POST /api/augment/start` | 任意**写** |
| `GET /api/data/load/{filename}`、`PUT`、`DELETE` | 绝对路径逃逸 |

**修复方向**：在 `api/deps.py` 中新增唯一入口 `resolve_data_path(name, *, for_write=False)`，
用 `Path(base).resolve()` + `candidate.is_relative_to(base)` 做白名单校验（基目录默认 `data/`，
可由 `web.data_root` 配置），`load_items` / `save_items` / `_safe_data_path` 全部收敛到它。
同时拒绝绝对路径与非 `.json` 扩展名。

**验收**：上述实测脚本返回 `400/403`；新增测试覆盖 `..`、绝对路径、符号链接三种逃逸。

---

#### F4 — 限流中间件已实现但未注册

**位置**：`api/main.py:26-37` 只注册了 `CORSMiddleware` / `RequestLoggingMiddleware` / `RequestTraceMiddleware`。
`api/middleware/rate_limit.py` 是完整实现（滑动窗口 + `Retry-After`），且 `tests/unit/test_rate_limit.py` 已覆盖，
但从未被接线。

**实测证据**：

```
app.user_middleware -> ['RequestTraceMiddleware', 'RequestLoggingMiddleware', 'CORSMiddleware']
RateLimitMiddleware 已注册? False
```

**影响**：44 个路由全部无速率保护，`/api/augment/start` 等重操作可被轻易打满线程池。

**修复方向**：在 `api/main.py` 注册 `RateLimitMiddleware`，参数从新增的 `web.rate_limit` 配置段读取
（`max_requests` / `window_seconds` / `exempt_paths: ["/api/health"]`）。

**验收**：`app.user_middleware` 含 `RateLimitMiddleware`；新增集成测试断言第 N+1 次请求返回 `429` 且带 `Retry-After`。

---

#### F5 — 无认证 / 无鉴权

**位置**：全部 44 个路由。`POST /api/config` 可改运行时配置，`DELETE /api/data/delete/{filename}` 可删数据，
`POST /api/versions/{id}/rollback` 可回滚版本——全部无需任何凭据。

**修复方向**：
- 新增 `api/deps.py:verify_api_key`（读 `X-API-Key`，比对环境变量 `AUGMENTOR_API_KEY`）；
- 读操作可选、写操作强制（由 `web.auth.require_for_writes` 控制）；
- 生产环境（`app.debug == False`）下若未配置密钥则**启动即失败**，而不是静默放行；
- 与 F4 的限流配合：未认证请求走更严格的限流档位。

**验收**：未带密钥的写请求返回 `401`；`/api/health` 与 `/docs` 保持匿名可达。

---

### P1 组：工程质量（覆盖率门禁形同虚设的根因）

#### F6 — 测试是"覆盖率刷分"，不是行为验证

> **2026-09-21 复核更正**：本节初版按文件数估计"删除 `test_round*.py` 与
> `test_micro_branches_l*.py` 后测试数从 3061 降至 ~2000"，**该估计错了约 15 倍**。
> 用 AST 逐函数分诊（`scripts/triage_tests.py`）实测后发现：这两个文件族**绝大多数是真实测试**，
> 真正的垃圾只有 28 个占位文件 / 68 个测试。详见下方"实测数据（复核后）"。

**实测数据（复核后，由 `scripts/triage_tests.py` 逐函数 AST 分析得出）**：

| 文件族 | 文件数 | 测试数 | 有效检查点 | 空洞断言 | 占位文件 |
|--------|-------|--------|-----------|---------|---------|
| `test_round*.py` | 49 | 132 | **226** | 0 | **0** |
| `test_micro_branches_l*.py` | 11 | 78 | **134** | 0 | **0** |
| `test_*_opt.py` | 32 | 44 | 33 | 25 | **25** |
| 其他 | 151 | 2851 | 5037 | 43 | 2 |
| **合计** | **243** | **3105** | **5430** | **68** | **27** |

> 初版把"文件命名难看"误判成了"测试无效"。`test_round52.py` 里是
> `batch_merge` / `shuffle_dataset` 的真实断言，`test_micro_branches_l*.py`
> 有 134 处真实检查点——它们是**组织问题**，不是**有效性问题**。
> 混为一谈会导致误删 210 个真实测试。

**真正的垃圾长这样**：

```python
# tests/unit/test_round_progress.py —— 41 个测试，45 处空洞断言，0 处有效检查
def test_round_62_integration_workflow(self): assert True
def test_100_round_goal(self):
    current_round = 62
    assert current_round <= 100          # 断言的是局部常量，不是系统行为
def test_coverage_threshold_met(self): assert 94.17 >= 80.0   # 两个字面量互比

# tests/unit/test_search_opt.py —— 全文
def test_search_opt():
    assert True
```

**另一种更隐蔽的写法**（`assert X or True`，X 是什么都恒真）：

```python
assert hasattr(cache, '_stats') or True          # _stats 写入后再无任何读取
assert "quality_a" in result.quality_diff or True # 该键实际不存在，判断永远为假
assert hasattr(config, '_load_cache') or True     # AppConfig 根本没有这个属性
```

**为什么这是问题**：F1（质量评分恒 pass）、F2（符号遮蔽）、F3（任意文件读）三个真实缺陷
全部逃过了 3061 个测试。`--cov-fail-under=80` 被"满足"了，但它保护的不是正确性。

**修复方向**：
1. 用机器判据（`scripts/triage_tests.py`）而非人工感觉来识别占位文件与空洞断言；
2. 删除占位文件；把 `assert X or True` 这类**掩盖了错误判断**的断言改写成真断言
   （其中 3 条改真后会暴露产品代码的实际情况，而不是删掉假装没看见）；
3. 覆盖率门禁从语句覆盖升级为 **branch 覆盖**（`--cov-branch`），并排除 `pragma: no cover` 滥用；
4. 引入 `mutmut` 做抽样变异测试，验证"测试能否杀死变异体"——这是唯一能度量
   「测试是否验证行为」的自动化手段；
5. 本规划 F1/F2/F3 的回归测试**必须在修复前失败**（红→绿），否则视为无效测试；
6. 把卫生判据接入 CI，防止假测试重新长回来。

**验收**：无占位文件、无空洞断言、无"不可能失败"的测试；`pytest --cov-branch` 仍 ≥ 80%；
新增回归测试在 `git stash` 掉修复代码后确实失败。

**执行结果（2026-09-21 完成）**：

| 指标 | 清理前 | 清理后 |
|------|--------|--------|
| 测试文件 | 243 | 215 |
| 测试函数（pytest 口径，含参数化） | 3108 | 3040 |
| 占位文件（无任何有效检查点） | 28 | **0** |
| 空洞断言（恒真） | 68 | **0** |
| 不可能失败的测试 | 2 | **0** |
| 有效检查点 | 5430 | 5441 |
| 覆盖率 | 99.41% | **99.50%** |
| `csv_excel_import.py` 覆盖率 | 24% | **37%** |

**删掉 68 个假测试后覆盖率反而上升**——这本身就是 F6 论断的直接证据：
假测试不但不保护正确性，还在稀释覆盖率的可信度。

具体动作：
- 删除 28 个占位文件（25 个 `*_opt.py` + `test_round_progress.py` +
  `test_final_integration.py` + `test_stream_perf_51.py`，共 68 个测试）；
- 删除 `test_thread_pool_optimization.py`（3 个测试全假；并行行为已由
  `tests/integration/test_pipeline.py::test_parallel_run_matches_serial_count` 覆盖）；
- 改写 7 处空洞断言为真断言（`test_cache_perf` / `test_comparison_branches` /
  `test_config_load_opt` / `test_csv_excel_import` / `test_multilingual_ext` /
  `test_export` / `test_thread_pool_optimization`）；
- 新增 `tests/unit/test_test_hygiene.py`（4 条门禁）+ `scripts/triage_tests.py --check`。

**判据设计上的两次自我纠错（值得记录）**：

1. 初版把"出现过 `name = <字面量>`"就当常量，把 `call_count = 0`（后续被闭包 `+=`）、
   `missing = []`（后续被 `append`）误判为常量，导致 `assert call_count == 1` 这类
   **有效断言被误报**。修正为"单次绑定 + 从不被变异 + 不作为参数传出"才算常量。
2. 用"有没有 assert"当判据同样错：`writer.close()` 这类"不抛异常"测试是合法的
   （`close` 若开始抛异常，测试就会失败）。改为判定"**这条测试能否失败**"——
   既无有效检查点、又没有任何未被 `try` 包裹的调用，才算空壳。

教训：**自动化判据必须先做误报校准再动手删除**。第一次误报如果不查，会删掉 210 个真实测试。

---

#### F7 — 模块重复实现，六对并行 API

| 模块 A | 模块 B | 重复内容 |
|--------|--------|---------|
| `augmentor/comparison.py` | `augmentor/compare_enhanced.py` | 数据集对比（`DatasetComparator` vs `EnhancedComparator`） |
| `augmentor/versioning.py` | `augmentor/version_control.py` | 版本管理（`VersionManager` vs `DatasetVersionManager`） |
| `augmentor/export.py` | `augmentor/export_enhanced.py` | 导出（**`ExportFormat` 枚举各定义一份**） |
| `augmentor/visualizer.py` | `augmentor/visualize_enhanced.py` | 可视化 |
| `augmentor/statistics.py` | `augmentor/analytics.py` | 统计（`FieldStatistics` vs `TextStatistics`） |
| `augmentor/cleaner.py` | `augmentor/data/cleaner.py` | 清洗（`CleaningResult` vs `CleanResult`，**类名仅差一词**） |

另外 `get_supported_formats` 在 `converter.py` 与 `export_enhanced.py` 各有一份，
`__init__.py` 靠 `as get_supported_export_formats` 别名掩盖了冲突。

**修复方向**：每对选一个作为唯一实现（判据：被 `pipeline.py` 引用、测试更完整、API 更清晰者），
另一个标注 `@deprecated` 并在下一个大版本删除；`__init__.py` 的导出面收敛为每能力一个入口。

**验收**：`augmentor/__init__.py` 中不再出现同名二次导入；`grep -c "class ExportFormat"` 结果为 1。

> **复核修正（T1.5 前置侦察）**：本条被**低估**了。两个同名 `ExportFormat` 不是「重复」而是
> **互不兼容的公开类型**——`augmentor.ExportFormat` 的 13 个成员**全部**无法传给
> `pipeline.export_dataset`（`ValueError`）。详见下方「T1.5–T1.7 前置侦察」。

---

#### F8 — CLI 44 个子命令、9 组功能重复、`main()` 单函数 922 行

**结构现状**：`cli.py` 仅 6 个顶层函数，其中
- `build_parser()` 占 58-386 行（**329 行**）
- `main()` 占 387-1308 行（**922 行**）

**功能重复的 9 组命令**（同一能力两个入口，用户无法判断该用哪个）：

| 基础命令 | 增强命令 |
|---------|---------|
| `compare` | `compare-enhanced` |
| `clean` | `clean-enhanced` |
| `export` | `export-enhanced` |
| `stats` | `stats-enhanced` |
| `search` | `search-enhanced` |
| `version` | `version-control` |
| `visualize` | `visualize-data` |
| `analyze` | `analyze-data` |
| `quality` | `quality-report` |

**修复方向**：
- 把 `cli.py` 拆为 `cli/` 包：`cli/__init__.py`（dispatch 表）、`cli/commands/<command>.py`（每命令一模块）、
  `cli/io.py`（`_load_items` / `_save_items` / `_dump_json` / `_print` 共用）；
- 9 组重复命令合并：保留基础命令，增强能力以 `--enhanced` / `--format` 开关暴露，
  旧命令名保留为隐藏别名并输出弃用提示；
- `main()` 收敛为 `{"command": handler}` 分发表 + 统一异常处理，目标 ≤ 60 行。

**验收**：`cli.py` 顶层 ≤ 80 行；`python cli.py --help` 子命令数从 44 降至 ≤ 30；
`tests/integration/test_cli_*.py`（11 个文件）全部通过。

> **复核修正（T1.5 前置侦察）**：上表「9 组重复命令」有 3 处不准确 ——
> `clean-enhanced` / `stats-enhanced` 调用的恰是**基础模块**（`-enhanced` 后缀名不副实）；
> `quality` / `quality-report` **不是重复**（一个筛数据、一个出报告），合并会丢功能。
> 按此表动手会把功能合并掉。详见下方「T1.5–T1.7 前置侦察」。

---

#### F9 — 前端零工程化：无测试、无 lint、`build` 不做类型检查

**位置**：`web/package.json`

```json
"scripts": { "dev": "vite", "build": "vite build", "preview": "vite preview" }
```

- `vite build` 基于 esbuild，**不做类型检查**——本地构建可放过类型错误；
- 无 ESLint / Prettier 配置（`find web -name ".eslintrc*" -o -name "eslint.config.*"` 为空）；
- 无任何测试文件（无 `*.test.tsx` / `*.spec.tsx` / `vitest.config.*`）；
- CI（`.github/workflows/ai-platform-ci.yml`）里有 `npx tsc --noEmit`，但本地 `npm run build` 与 CI 行为不一致。

**修复方向**：
- `"build": "tsc -b && vite build"`，让本地与 CI 行为对齐；
- 引入 ESLint 9（flat config）+ `@typescript-eslint` + `eslint-plugin-react-hooks` + Prettier，
  加 `"lint": "eslint src --max-warnings=0"`；
- 引入 Vitest + React Testing Library，优先覆盖 `web/src/services/api.ts`（243 行，全部接口封装，
  是前后端契约的唯一收敛点）与 2 个核心页面；
- CI 增加 `npm run lint` 与 `npm run test` 两个 step。

**验收**：`npm run build` 在存在类型错误时失败；`npm run lint` 零 warning；
`api.ts` 的行覆盖率 ≥ 70%。

---

#### F10 — 版本号三处不一致 + 架构文档漂移

**版本号**：

| 位置 | 值 |
|------|-----|
| `augmentor/__init__.py:3` | `__version__ = "2.2.0"` |
| `api/main.py:22` | `FastAPI(version="2.0.0")` |
| `api/routes/status.py:38` | `version="2.0.0"` |
| `config.yaml:6` | `app.version: "1.0"` |
| `docker/docker-compose.yml:20` | `IMAGE_TAG:-2.0.0` |

**文档漂移**（`docs/ARCHITECTURE.md`）：

| 文档声称 | 实际 |
|---------|------|
| 「按领域拆分的 **7** 个 router」（第 100 行） | **11** 个（新增 privacy / leakage / audit / status） |
| 「`api/main.py` FastAPI 应用装配（**29** 行）」（第 97 行） | **70** 行 |
| 「`__init__.py` 对外导出面（版本 **2.0.0**）」（第 48 行） | **2.2.0** |
| 「`build_parser` + **11** 个子命令」（第 41 行） | **44** 个子命令 |

**修复方向**：
- 单一版本源：`api/main.py` 与 `status.py` 改为 `from augmentor import __version__`；
  `config.yaml` 的 `app.version` 删除或改为引用；docker tag 由 CI 注入而非硬编码；
- 文档中所有会漂移的**数字**改为「见 `ls api/routes/`」式的指引，或由脚本生成；
- CI 增加一步：校验 `augmentor.__version__` 与 `api.main.app.version` 一致。

**验收**：全仓库 `grep -rn '2\.0\.0'` 不再出现于代码中的版本声明；
新增 CI step `check-version-consistency` 通过。

---

#### F11 — 仓库卫生：冗余数据文件入库（原判"无 `.gitignore`"经复核**不成立**）

> **2026-09-21 复核更正**：本节初版称"`augmentor/` 没有 `.gitignore`，仓库根也无针对该子项目的规则"，
> 与实际不符。仓库根 `.gitignore` **存在且有效**，已覆盖 `__pycache__/`、`*.py[cod]`、`.coverage`、
> `coverage.xml`、`htmlcov/`、`test_output.json`、`node_modules/`、`dist/`、`*.log`、`*.orig`。
> 实测 `git check-ignore -v` 对 `augmentor/coverage.xml`、`augmentor/test_output.json`、
> `augmentor/__pycache__`、`augmentor/web/dist/`、`augmentor/web/node_modules/` **全部命中**。
>
> 因此 **T1.13（新增 `augmentor/.gitignore`）取消**——新建子目录规则只会重复根规则，
> 增加维护面而不解决任何问题。真正待处理的是下面这张表。

**真正的问题：已跟踪的冗余数据（合计约 11.5 MB）**

| 文件 | 体积 | 问题 |
|------|------|------|
| `train_data.json` | 3.5 MB | 与 `train_data_final.json` **md5 完全相同**（`a83320ab…`） |
| `train_data_final.json` | 3.5 MB | 同上 |
| `train_data_final02.json` | 3.0 MB | 中间产物 |
| `train_data_final01.json` | 595 KB | 中间产物 |
| `bak/`（4 文件） | 1.1 MB | `last_line.json`、`train_data-bd.txt`、`train_data.json`、`train_data.jsonl` |
| `enhanceTXT.py` ×2 | 19450 B ×2 | 根目录与 `archive/` **md5 相同**（`e74dc8a5…`） |

**未入库、仅需本地整理**：`身份证、工作证明`（438 B）、`选房、签约、付款`（303 B）
两个无扩展名 JSON 文件——`git ls-files` 查无记录，属 untracked，不涉及 git 变更。

**修复方向**：`git rm --cached` 移出跟踪 + 补 `.gitignore` 规则（只加真正缺失的：
`augmentor/bak/`、`augmentor/train_data*.json`），删除重复的 `enhanceTXT.py` 一份。

> **注意**：删除数据文件属破坏性操作，须先备份并由用户逐项确认后再执行，本规划不自动执行。
> 另需注意 `augmentor/train_data*.json` 可能被 `config.yaml` 的默认路径引用，
> 移出跟踪前须确认没有代码/文档依赖它们（`git rm --cached` 只影响版本控制，不影响本地文件）。

**验收**：`git status` 在跑完 `pytest` 与 `npm run build` 后保持干净；仓库体积下降 ≥ 10 MB。

---

#### F12 — `ModelBackend.__del__` 在构造失败时抛 `AttributeError`

**位置**：`augmentor/models/base.py:174-183`

```python
def close(self):
    if self._session:          # ← __init__ 早退时 _session 不存在
        self._session.close()
        self._session = None

def __del__(self):
    self.close()
```

**实测证据**（pytest 输出中的 `PytestUnraisableExceptionWarning`）：

```
Exception ignored in: <function ModelBackend.__del__ at 0x...>
  File ".../models/base.py", line 182, in __del__
    self.close()
  File ".../models/base.py", line 176, in close
    if self._session:
AttributeError: 'Bare' object has no attribute '_session'
```

**修复方向**：`session = getattr(self, "_session", None)`，并加 `try/except` 兜底
（析构函数中抛异常会污染解释器退出流程）。同类问题应全仓扫描 `__del__`。

**验收**：`pytest -W error::pytest.PytestUnraisableExceptionWarning` 通过。

---

#### F13 — `QualityScore.passed` 声明为 `bool`，实际是 `np.bool_`（Phase 0 执行中发现）

**位置**：`augmentor/quality.py`（`score()` 与 `batch_score()` 两处构造点）

```python
passed=total_score >= self.threshold          # np.float64 >= float → np.bool_
passed=total_scores[i] >= self.threshold      # 同上
```

**实测证据**：

```
>>> type(np.False_)
<class 'numpy.bool'>   isinstance(v, bool) → False
>>> json.dumps({"passed": np.False_})
TypeError: Object of type bool is not JSON serializable
>>> jsonable_encoder({"passed": np.False_})
ValueError: [TypeError("'numpy.bool' object is not iterable"), ...]
```

**为什么现在没炸**：现有 API 路由都不直接序列化 `s.passed`，而是先聚合
（`sum(1 for s in scores if s.passed)` → Python `int`），所以这条路径是**潜伏**的。
但 `float(...)` 在另外三个字段上被显式调用过（说明作者意识到 numpy 泄漏），
唯独 `passed` 漏了——只要有人把 `QualityScore` 直接放进响应或落盘，就会 500。

**修复方向**：构造点用 `bool(...)` 收敛。`total_score` 无需转换——
`np.float64` 是 `float` 的子类，可正常序列化。

**验收**：`isinstance(score.passed, bool)` 为真，且 `json.dumps` 可序列化。

---

### P2 组：结构性缺口 —— 多端能力严重不对称

这是本审计发现的**最大结构性问题**，也是"多端"这个词的核心：

| 能力端 | 入口数 | 覆盖能力 |
|--------|--------|---------|
| Python SDK（`augmentor/__init__.py`） | ~120 个导出符号 | 几乎全部 |
| CLI（`cli.py`） | **44 个子命令** | 几乎全部 |
| REST API（`api/routes/`） | **44 个路由** | 仅 ~15 项能力 |
| Web UI（`web/src/pages/`） | **9 个页面** | 仅 ~12 项能力 |

**只有 CLI/Python 可达、REST API 与 Web UI 完全缺失的能力**（抽样）：

| 模块 | 能力 | CLI 命令 | API | Web |
|------|------|---------|-----|-----|
| `streaming.py` | 大数据集流式处理 | `stream` | ✗ | ✗ |
| `dataset_ops.py` | 合并 / 采样 / 分割 | `merge` `sample` `split` | ✗ | ✗ |
| `converter.py` | 格式转换 | `convert` | ✗ | ✗ |
| `backup.py` | 备份 / 恢复 | `backup` | ✗ | ✗ |
| `version_control.py` | 版本控制（第二套） | `version-control` | ✗ | ✗ |
| `rag.py` | RAG 格式转换 | `rag` | ✗ | ✗ |
| `multilingual.py` | 多语言翻译增强 | — | ✗ | ✗ |
| `evaluation.py` | BLEU / ROUGE-L 评估 | — | ✗ | ✗ |
| `active_learning.py` | 主动学习循环 | — | ✗ | ✗ |
| `expander.py` | 领域扩展 | — | ✗ | ✗ |
| `sampler.py` | 覆盖度选样 | — | ✗ | ✗ |
| `tracker.py` | 实验追踪 | — | ✗ | ✗ |
| `quality_trend.py` | 质量趋势 | — | ✗ | ✗ |
| `health_score.py` | 健康度评分 | — | ✗ | ✗ |
| `quality_gate.py` | 质量门禁 | — | ✗ | ✗ |
| `aggregator.py` | 数据集聚合 | `aggregate` | ✗ | ✗ |
| `data_pipeline.py` | 多阶段流水线 | — | ✗ | ✗ |
| `impact.py` | 增强效果评估 | — | ✗ | ✗ |
| `frameworks/` | LangChain / LlamaIndex 互转 | — | ✗ | ✗ |
| `vector/` | 向量库 | — | ✗ | ✗ |
| `indexer.py` | 数据索引 | — | ✗ | ✗ |
| `cache.py` | 缓存 | — | ✗ | ✗ |
| `dependency.py` | 依赖图 | `dependency` | ✗ | ✗ |
| `migration.py` | 数据迁移 | `migrate` | ✗ | ✗ |
| `auto_config.py` | 参数推荐 | `auto-config` | ✗ | ✗ |
| `auto_test.py` | 自动化测试 | `auto-test` | ✗ | ✗ |
| `feature_detect.py` | 特征检测 | `features` | ✗ | ✗ |
| `data_splitter.py` | 数据集划分 | — | ✗ | ✗ |

**结论**：项目有约 60 个能力模块，但只有约 15 个能通过 Web 使用。
CLI 与 API/Web 之间存在 **3 倍以上的能力落差**，且没有任何机制保证三者同步演进
（`tests/e2e/test_web.py` 只校验「前端调用的路径后端是否存在」，**不校验反向**——
后端有能力但前端没入口时不会报错）。

**修复方向**：
1. 建立**单一能力清单** `augmentor/capabilities.py`（或 `docs/capabilities.yaml`），
   声明每项能力的 `{id, sdk_symbol, cli_command, api_route, web_page}`；
2. `tests/e2e/test_web.py` 增加**双向**校验：清单中的能力若声明了 `api_route` 则必须注册，
   声明了 `cli_command` 则 `build_parser()` 必须包含，声明了 `web_page` 则 `web/src/pages/` 必须存在；
3. 按「价值 / 成本」排序补齐 API 与 Web 入口（见 Phase 3 的优先级表）。

---

## Requirements

- **R1** 修复 F1：API 质量评分恢复语义维度或显式降级，通过率不再虚高。
- **R2** 修复 F2：消除 `QualityReport` / `ValidationResult` 符号遮蔽，恢复类型契约。
- **R3** 修复 F3：所有文件路径入参收敛到白名单校验，消除任意文件读写。
- **R4** 修复 F4：限流中间件注册并可通过配置调整。
- **R5** 修复 F5：写操作强制认证，生产环境未配置密钥时启动失败。
- **R6** 修复 F6：测试回归行为验证，覆盖率门禁升级为 branch 覆盖 + 变异测试抽检。
- **R7** 修复 F7/F8：消除重复模块与重复 CLI 命令，`cli.py` 拆包。
- **R8** 修复 F9：前端补齐类型检查门禁、lint、单测。
- **R9** 修复 F10/F11/F12：版本号单一来源、文档数字不漂移、仓库卫生、析构安全。
- **R10** 建立能力清单与三端一致性校验（P2），并补齐高价值能力的 API/Web 入口。

## Scope Boundaries

### In Scope
- `augmentor/` 下的 Python 核心、`api/`、`cli.py`、`web/`、`docker/`、`docs/`
- 本规划中 F1-F12 的具体整改与验收
- 三端（CLI / REST API / Web UI）+ Python SDK 的能力一致性机制

### Out of Scope
- 多用户 / 多租户权限体系（当前定位为内部工具，F5 只做单密钥认证）
- 分布式部署与水平扩展（版本、断点、向量库均为本地文件，单副本是有意设计）
- 更换 LLM 供应商或重写模型后端
- 前端 UI 视觉改版

### Deferred to Follow-Up Work
- 模型后端的原生异步化（`httpx.AsyncClient`），当前以 `run_in_thread` 隔离阻塞已够用
- `chromadb` 后端的生产化（当前缺依赖即抛 `ImportError`，属预期行为）
- 多语言 / 多模态能力的 API 与 Web 暴露（依赖 Phase 3 的能力清单先落地）

---

## Context & Research

### 已有基础设施（本规划复用，不重建）

- **CI/CD 已完整**：`.github/workflows/ai-platform-ci.yml`（后端测试+覆盖率门禁+前端 tsc/build+Docker 健康检查）、
  `ai-platform-cd.yml`（GHCR 推送 + SSH 部署 + release notes）。
  **本规划只需在现有 CI 上增量添加 step，不新建流水线。**
- **测试夹具完善**：`tests/conftest.py` 已注入假环境变量、强制 `model_manager` 走 fallback（禁止测试期下载模型）、
  提供 `sample_items` 与 `tmp_data_file`。
- **契约测试雏形**：`tests/e2e/test_web.py` 已能从 `app.openapi()["paths"]` 与
  `web/src/services/api.ts` 双向提取路径并比对——P2 只需把它从「路径级」升级为「能力级」。
- **降级设计**：全仓可选依赖均为「惰性加载 + 降级」，本规划的改动不得破坏这一约定。
- **异常体系**：`augmentor/exceptions.py` 已有完整层级（`ConfigError` / `ModelError` / `DataError` / …），
  API 层的错误转换应复用它而非直接 `HTTPException(500, str(e))`。

### 本机验证环境（已就绪）

```
Python: C:\Users\Administrator\.workbuddy-ai\binaries\python\envs\aug\Scripts\python.exe
        （已按 requirements-dev.txt 安装，可跑全量测试）
基线：  pytest -q  →  3061 passed, 3 skipped, 99.46% coverage, 91.73s
```

### 外部参考

- OWASP Path Traversal / Broken Access Control（F3 / F5 的判据）
- `mutmut` 变异测试（F6 的验证手段）
- FastAPI `dependencies` 与 `middleware` 官方文档（F4 / F5 的实现方式）

---

## Phase 0 — 止血：正确性与安全（P0）

> 目标：修掉「结果不可信」与「可被任意读写」两类问题。**本阶段不新增任何功能。**

| 任务 | 对应 | 主要改动 | 验收 |
|------|------|---------|------|
| T0.1 | F2 | `augmentor/__init__.py` 消除同名二次导入；冲突类改名 + 弃用别名 | `isinstance(generate(...), augmentor.QualityReport) is True` |
| T0.2 | F1 | `api/routes/quality.py` 重写 `_build_scoring_items`：有种子则用之，无则二维降级 + 显式标记 | 无关答案 `total_score < threshold` |
| T0.3 | F3 | `api/deps.py` 新增 `resolve_data_path()`，全部路径入参收敛 | 项目外文件返回 400/403 |
| T0.4 | F3 | `api/routes/data.py:_safe_data_path` 改为委托 `resolve_data_path` | 绝对路径 / `..` / 符号链接均被拒 |
| T0.5 | F4 | `api/main.py` 注册 `RateLimitMiddleware` + `config.yaml` 新增 `web.rate_limit` | 超限返回 429 + `Retry-After` |
| T0.6 | F5 | `api/deps.py` 新增 `verify_api_key`，写路由挂载；生产未配置密钥则启动失败 | 无密钥写请求返回 401 |
| T0.7 | F12 | `models/base.py:close()` 改 `getattr` + `try/except` | `-W error::PytestUnraisableExceptionWarning` 通过 |
| T0.8 | F1/F2/F3 | **为以上每一项补"修复前必须失败"的回归测试** | 在 `git stash` 掉修复后测试确实变红 |

**阶段门禁**：T0.8 的回归测试全部为红→绿；`pytest -q` 全绿；`docs/ARCHITECTURE.md`
新增「安全边界」小节说明路径白名单与认证模型。

### 执行状态（已完成，含与原规划的偏差）

T0.1–T0.8 全部落地。**偏差项必须显式记录，不能当作"按计划完成"**：

| 项 | 原规划 | 实际实现 | 原因 |
|----|--------|---------|------|
| T0.1 | 冲突类改名 | 四处导入全部加显式别名（`PipelineQualityReport` / `DataValidationResult` / `ConfigValidationResult` / `DatasetQualityReport`），原歧义名保留为 PEP 562 惰性别名并发 `DeprecationWarning` | 只改真正冲突的四个名字，避免 `__all__` 209 项大面积改名打穿下游；旧名仍可用 |
| T0.2 | "有种子则用之，无则二维降级" | API 侧固定 `include_semantic=False`（落盘数据集不存在种子问题），并新增 `QualityScorer.effective_weights()` 做权重**重归一化**而非硬编码二维权重 | 权重可配置，硬编码会让 `quality.weights` 配置项对降级路径失效 |
| T0.3 | 新增 `resolve_data_path()` | 另加 `resolve_within_roots()` / `resolve_data_dir()` 分层，并引入 `web.data_roots` 配置与 `AUGMENTOR_DATA_ROOTS` 环境变量 | 需要"只校验边界、不要求存在"的中间层（多模态降级路径），且白名单必须可配 |
| T0.6 | **生产未配置密钥则启动失败** | **改为启动 WARNING**，未配置时写操作放行 | 与既有"零配置开箱可用"约定及既有测试冲突。已在 `api/main.py` 打印明确告警，并在 `ARCHITECTURE.md` §3.12.2 说明该模型的边界 |
| T0.8 | 补回归测试 | 另发现 `tests/integration/test_api_export_multimodal.py::test_export_missing_input_500` **固化了错误契约**（缺文件报 500），已改为断言 404 | 调用方无法区分"我传错了"与"服务挂了" |
| — | 未在规划内 | 新增 **F13**（`QualityScore.passed` 是 `np.bool_`，不可 JSON 序列化），一并修复 | 见上文 F13 |

**验证结果**：

- 红→绿：临时还原 F1/F3/F4 的修复后，`tests/integration/test_api_security.py` 有 **8 例失败**；恢复后 **24 例全通过**。证明回归测试确实锁定缺陷。
- 全量：`3103 passed / 3 skipped`（基线 `3061 passed / 3 skipped`，新增 **42 例**：
  安全回归 24 + 语义维度可选 8 + 符号完整性 7 + 析构安全 3）。
- 覆盖率：**99.41%**，62 行未覆盖。其中 54 行来自 `csv_excel_import.py`
  （缺 `pandas` 导致相关用例 skip，属环境限制而非回归），其余 8 行是各路由的
  `except` 兜底与 `__del__` 兜底分支。远高于 80% 门禁。
- `docs/ARCHITECTURE.md` 新增 §3.12「安全边界」（路径白名单 / 写操作鉴权 / 限流）。

**已知残留（Phase 1 处理，不属 Phase 0 范围）**：

- `AUGMENTOR_API_KEY` 只在写路由逐个挂载，**新增写路由时容易漏挂**——Phase 1 应改为
  按 HTTP 方法在中间件层统一拦截，或加一条"所有非 GET 路由必须挂鉴权"的元测试。
- `ARCHITECTURE.md` 仍有 Phase 0 之外的漂移未修：称 `cli.py` 11 个子命令（实际 44）、
  `api/routes/` 7 个 router（实际 11）、`__init__.py` 版本 2.0.0（实际 2.2.0）——
  对应 F10，留待 Phase 1。

---

## Phase 1 — 固本：工程质量（P1）

> 目标：让"通过测试"重新等价于"行为正确"。

| 任务 | 对应 | 主要改动 | 验收 |
|------|------|---------|------|
| T1.1 | F6 | ~~删除 50 个 `test_round*.py` + 11 个 `test_micro_branches_l*.py` 中的占位文件~~ → **经复核这两个文件族是真实测试，实际只删了 28 个占位文件**（见 F6 复核） | 测试文件数 ↓ ≥ 30% → **实际 ↓ 12%**（243→214），但覆盖率**上升** |
| T1.2 | F6 | ~~合并 32 个 `*_opt.py` 到主测试文件~~ → 25 个是占位（已删），7 个有真断言（已按模块归并/改写） | 无 `assert True` 残留 → **达成（0 处）** |
| T1.3 | F6 | `pytest.ini` 加 `--cov-branch`；~~清理 `pragma: no cover` 滥用~~ → **复核发现是伪问题（生产源码 0 处）** | branch 覆盖率 ≥ 80% → **实际 98.63%** |
| T1.4 | F6 | ~~引入 `mutmut`~~ → **Windows 不支持，改用 `cosmic-ray`**（显式偏差，见下），对 `quality.py` / `dedup.py` / `export.py` 做分层抽样变异测试 | 变异杀死率 ≥ 60% |
| T1.5a | F7 | **拆分出的高优先子项**：修 `ExportFormat` 同名冲突（前置侦察定性为**用户可见故障**，非"重复"） | `grep -c "class ExportFormat"` == 1 且三处入口同一对象；公开枚举成员可端到端消费 |
| T1.5b | F7 | 六对重复模块各保留一个实现，另一个标 `@deprecated` | `__init__.py` 无同名二次导入；全量测试通过 |
| T1.6 | F8 | `cli.py` 拆为 `cli/` 包（dispatch 表 + `commands/` + `io.py`） | `cli.py` 顶层 ≤ 80 行 |
| T1.7 | F8 | 9 组重复命令合并为 `--enhanced` 开关，旧名保留弃用别名 | 子命令 44 → ≤ 30，CLI 集成测试全绿 |
| T1.8 | F9 | `web/package.json`：~~`build` 加 `tsc -b`~~ → **改为 `tsc --noEmit && vite build`**（见偏差）；新增 `typecheck` / `lint` / `test` / `test:coverage` / `format` | 类型错误时 `npm run build` 失败 → **已验证（退出码 2）** |
| T1.9 | F9 | 引入 ~~ESLint 9~~ **ESLint 10** flat config + Prettier；Vitest + RTL 覆盖 `api.ts` | `npm run lint` 零 warning → **达成**；`api.ts` 覆盖 ≥ 70% → **实际 100%** |
| T1.10 | F9 | `ai-platform-ci.yml` 的 `frontend-build` 增加 Lint 与 Test-with-coverage-gate 两步 | CI 两 step 通过 → **本地等价命令全绿**（未在 GitHub 上实跑） |
| T1.11 | F10 | 版本号单一来源；`api` 从 `augmentor.__version__` 读取 | 全仓无硬编码版本声明 |
| T1.12 | F10 | 修正 `docs/ARCHITECTURE.md` 的 4 处数字漂移；CI 加版本一致性检查 | `check-version-consistency` step 通过 |
| ~~T1.13~~ | F11 | ~~新增 `augmentor/.gitignore`~~ → **取消**（根 `.gitignore` 已覆盖，见 F11 复核） | — |
| T1.14 | F11 | `git rm --cached` 移出冗余数据（`train_data*.json`、`bak/`、重复的 `enhanceTXT.py`）（**需用户逐项确认**） | `git status` 干净且无数据丢失 |
| T1.15 | F9 | **新增**：前端 API 响应类型层 —— 为约 24 个端点定义响应接口，消除 `src/pages` / `src/components` 里约 30 处 `useState<any>` | `no-explicit-any` 在页面/组件层恢复为 `error` |

### Phase 1 偏差登记

与原规划不一致的项必须显式记录，**不能当作"按计划完成"**（与 Phase 0 同规矩）：

| 项 | 原规划 | 实际实现 | 原因 |
|----|--------|---------|------|
| T1.4 工具 | `mutmut` | **`cosmic-ray` 8.7.0** | `mutmut` 3.8 在原生 Windows 上直接拒绝运行（`To run mutmut on Windows, please use the WSL.`）；WSL 被本机安全策略阻断（`wsl.exe` 在程序黑名单中）。`cosmic-ray` 跨平台且在维护 |
| T1.4 范围 | "变异抽检" | 分层抽样 **173 / 1320** 个变异体 | 全量约 92 分钟且 70% 是等价变异体，信息量低 |
| T1.3 范围 | "清理 `pragma: no cover` 滥用" | **取消**（生产源码 0 处） | 与 F11 同类的**伪问题**，复核后不成立 |
| T1.4 门槛 | "变异杀死率 ≥ 60%" | 改为**逐模块**判定 + 合计参考 | 合计 60.1% 会掩盖 export 的 51.7%，只看汇总会放掉薄弱模块 |
| — | 未在规划内 | 新增 `scripts/mutation/{run.py,sample.py}` 与三个 TOML | 需要一个把 `{python}` 占位符解析为 `sys.executable` 的运行器，否则变异得分全是假的（见 T1.4「工具链上的一个坑」） |
| T1.8 | `build` 加 `tsc -b` | **`tsc --noEmit && vite build`** | 实测 `tsc -b` 会在仓库根生成 `vite.config.js`，**遮蔽 `vite.config.ts`**（vite 优先解析 `.js`）——这是个真实隐患；还会留下 `tsconfig*.tsbuildinfo`。`tsc --noEmit` 与既有 CI 步骤一致且零产物 |
| T1.9 | ESLint 9 | **ESLint 10.11** | 当前最新稳定版；`typescript-eslint` 8.70 的 peer 已声明支持 `^10.0.0` |
| T1.9 | 用 `reactHooks.configs.flat.recommended` | **只启用经典两规则**（`rules-of-hooks` / `exhaustive-deps`） | v7 的 recommended 额外含一批 **React Compiler 专用**规则（immutability / purity / static-components / preserve-manual-memoization / set-state-in-effect）。本项目用 React 18、无 `babel-plugin-react-compiler`，那 10 处 immutability 报的是 "prevents the compiler from…"（编译器无法优化）而非缺陷 |
| T1.9 | "`npm run lint` 零 warning" | **前提不成立**：首次运行 91 处问题（89 error / 2 warning）。修掉 45 处，对 `no-explicit-any` 在页面/组件层**显式豁免**并登记 T1.15 | 44 处 `any` 里约 30 处是 `useState<any>` 承载 API 响应载荷；消除它们需先建响应类型层，属独立工程，硬塞进本次改动会把可能错误的类型假设固化下来 |
| T1.9 | 引入 Prettier | 引入配置与 `format` / `format:check` 脚本，但**不接入 CI 门禁** | 既有 18 个文件从未格式化，`format:check` 必然失败；强行 `--write` 会制造一个淹没真实改动的巨大 diff（违反"外科手术式改动"） |

### Phase 1 已完成部分（2026-09-21）

**T1.1 + T1.2 测试去水分** — 详见 F6 的"执行结果"。要点：

- 用 `scripts/triage_tests.py`（逐函数 AST 常量传播）替代人工判断；
- 删除 28 个占位文件 + `test_thread_pool_optimization.py`（并行行为已被集成测试覆盖）；
- 改写 7 处空洞断言为真断言，其中 3 处 `assert X or True` 改真后**暴露了产品代码的真实情况**：
  - `quality_diff` 的键是 `avg_score`/`pass_rate`/`min_score`/`max_score`，**不含 `quality_a`**；
  - `AppConfig` **没有** `_load_cache` 属性；
  - `MemoryCache._stats` 是只写不读的死状态（`stats` property 用的是 `_hits`/`_misses`）。
- 新增 `tests/unit/test_test_hygiene.py`（4 条门禁）+ `scripts/triage_tests.py --check`
  （可接入 CI），并用临时探针验证过门禁确实会红。

**T1.3 分支覆盖** — `pytest.ini` 的 `addopts` 加 `--cov-branch`（阈值仍是
`--cov-fail-under=80`，即门禁口径由语句改为分支）：

| 口径 | 数值 |
|------|------|
| 语句 | 10528 stmts / 53 miss → **99.50%** |
| 分支 | 3070 branch / 117 partial → **98.63%** |

**只低 0.87 个百分点**，远小于"分支覆盖通常比语句覆盖低 10–20pp"的经验预期——
说明这套测试在分支层面同样扎实，F6 批评的是**断言空洞**，不是**执行不足**。

> ~~清理 `pragma: no cover` 滥用~~ → **复核发现是伪问题**，与 F11 同类：
> 生产源码（`augmentor/` + `api/` + `cli.py`）**0 处** `pragma: no cover`；
> 唯一一处在 `tests/unit/test_models.py`，是析构测试用的 `_call_api` 空实现，合法。
> `.coveragerc` 另豁免两类，也都有正当理由：`raise NotImplementedError`（3 处，全在
> `augmentor/vector/base.py` 的 `VectorDB(ABC)` 抽象方法里，方法体永不执行）、
> `if TYPE_CHECKING:`（**0 处**）。

**T1.4 变异测试抽检** — 原规划用 `mutmut`，**在原生 Windows 上不可用**（见下方偏差），
改用 `cosmic-ray`。

工具与规模：

- 三个目标模块共生成 **1320 个变异体**（quality 676 / dedup 514 / export 130）。
  逐个体跑一遍测试约 4.2 秒 → 全量约 92 分钟。
- 按**算子族分层等距抽样**压到 **173 个**（52 / 63 / 58）。抽样的必要性（`NumberReplacer`
  与 `ReplaceBinaryOperator_*` 占 quality.py 变异体的约 70%，且大量等价）与规则
  见 `scripts/mutation/sample.py` 的模块 docstring。
- 前置校验——单模块测试文件对该模块的语句覆盖率：
  `test_quality.py`→`quality.py` **97%**、`test_dedup.py`→`dedup.py` **99%**、
  `test_export.py`→`export.py` **98%**。因此"每个变异体只跑本模块的测试文件"
  足以代表该模块的测试强度，不会因测试面太窄而虚低。

结果：

| 模块 | killed | survived | 原始得分 | 门槛 60% |
|------|-------:|---------:|---------:|:--:|
| quality | 35 | 17 | 67.3% | ✅ |
| dedup | 39 | 24 | 61.9% | ✅ |
| export（首轮） | 30 | 28 | **51.7%** | ❌ |
| export（补测试后） | 40 | 18 | **69.0%** | ✅ |
| **合计** | **114** | **59** | 60.1% → **65.9%** | ✅ |

#### 发现 1：汇总数字会掩盖单模块失败

首轮整体 60.1%「达标」，但那是 quality/dedup 把 export 拖上来的，**export 单独只有 51.7%**。
只看汇总就会放掉一个薄弱模块。因此门槛必须**逐模块**判定，而不是只看合计。

#### 发现 2：原始得分被等价变异体系统性压低

export 首轮 28 个存活变异体里约 20 个是**等价变异体**（改了代码但不改行为）：

| 等价变异类型 | 个数 | 为什么杀不死 |
|------|-----:|------|
| `json.dump(..., indent=2)` → `indent=1/3` | 6 | 纯缩进，任何解析型断言都测不出 |
| `ensure_ascii=False` → `True` | 4 | 用例用 `json.load` 往返比较，转义与否解码后相同 |
| `use_parallel: bool = True` → `False` | 2 | 串行/并行两条路径输出本就等价 |
| `except Exception` → `except CosmicRayTestingException` | 2 | 该异常永不抛出，正常路径下等价 |
| `max_workers=4` → `5` | 1 | 线程数不影响输出 |

`cosmic-ray` 的默认算子集（尤其 `NumberReplacer`）对这类代码会产生大量等价变异体。
**结论：变异得分必须结合人工甄别，不能当绝对指标**——这正是它没被写进 CI 门禁的原因。

#### 发现 3：5 个真实盲区，全部是「断言被错误来源满足」

| 存活变异体 | 原断言为什么测不出 |
|------|------|
| `if not use_parallel:` → `if use_parallel:`（4 个） | `test_serial_mode` 只断言"6 种格式都导出了"；两条路径输出一致，条件写反不可观测 |
| `ext = ".csv" if fmt == CSV.value else ".json"` → 翻转 | 只断言 `Path(path).exists()`，而返回值就是刚写入的路径，后缀写错照样存在 |
| `mkdir(parents=True)` → `parents=False`（2 个） | 现有父目录用例只覆盖 `export()`，没覆盖 `export_all_formats` / `export_batch` 的多级目录 |
| `for turn in history:` → `for turn in []:` | `assert "user" in roles` 被"当前问题"那条 user 消息满足，历史整段丢掉也通过 |

修复：新增 `tests/unit/test_export.py::TestExportContractGaps`（**7 条**），每条注明它杀死的变异体。
export 得分 **51.7% → 69.0%**。

#### 留在 quality.py / dedup.py 的盲区（已记录，未修）

这两模块各自已过 60% 门槛，按"外科手术式改动"原则未在本轮改动，登记为后续项：

- `quality.py` `if not include_semantic:` → 翻转后请求语义时 `semantic_similarity` 恒为 0，
  但现有用例都传 `include_semantic=False`，没有断言这个维度非零；
- `quality.py` `existing_generated or []` → `and []`：多样性参照集初始化条件；
- `dedup.py` `norms[norms == 0] = 1` → `norms[norms != 0] = 1`：零向量守卫，无用例覆盖零向量输入；
- `dedup.py` `visited[i] = True` → `False`：去重访问标记；
- `dedup.py` `pairs.sort(key=..., reverse=True)` → `False`：相似度排序方向，无用例断言顺序。

> 这 5 处都是"代码正确但没测到"，不是产品缺陷。

#### 工具链上的一个坑（必须记下来）

`test-command` 里写裸 `python` 会让**所有变异体被误判为 killed**，产出**假的 100% 杀死率**：

1. 裸 `python` 解析到托管解释器（没装 pytest）→ `ModuleNotFoundError` → 退出码非 0 →
   cosmic-ray 判定为 `killed`；
2. 实测**把 venv 的 `Scripts` 目录前置到 PATH 也无效**——即使排第一位，`python` 仍解析到托管版；
3. cosmic-ray 用 `shlex.split(command)`（**posix 模式**）解析命令，**反斜杠会被当转义符吃掉**，
   所以路径必须写正斜杠。

因此新增 `scripts/mutation/run.py`：把 TOML 里的 `{python}` 占位符替换成 `sys.executable`
（**只替换 `test-command` 那一行**），并顺带做抽样与报告。

> 这个坑是在用 `--per-family 1` 的小样本做端到端验证时才暴露的。
> **教训：变异测试必须先验证"验尸官"本身可信**——否则 100% 杀死率看起来像好消息，
> 比失败更危险。

**T1.8–T1.10 前端工程化** — 新增 `eslint.config.js` / `vitest.config.ts` /
`.prettierrc.json` / `.prettierignore` / `src/test/setup.ts` / `src/services/api.test.ts`。

`web/package.json` 的 scripts：

| 脚本 | 命令 |
|------|------|
| `build` | `tsc --noEmit && vite build` |
| `typecheck` | `tsc --noEmit` |
| `lint` | `eslint . --max-warnings 0` |
| `test` / `test:coverage` | `vitest run` / `vitest run --coverage` |
| `format` / `format:check` | `prettier --write src` / `prettier --check src` |

结果：

| 门禁 | 结果 |
|------|------|
| `npm run lint` | **0 problem**（含 `--max-warnings 0`） |
| `npm run test:coverage` | 29 用例通过；`api.ts` **100%**（语句/分支/函数/行），门槛 70% |
| `npm run build` | 通过（`tsc --noEmit` 先行） |

**`api.ts` 的测试为什么这么写**：这一层全是 HTTP 调用、没有分支，测"覆盖了几行"没有意义。
34 个函数的价值在于**锁定与后端的约定**——URL、HTTP 方法、查询参数名、请求体字段名、默认值。
例如 `loadData` 把 `pageSize` 映射成 `page_size`、`uploadData` 的 multipart 字段名必须是 `file`、
`dedupData` 的默认阈值是 0.9 而 `evaluateQuality` 是 0.6。断言方式是检查
`get/post/put/delete` 收到的**实参**，而不是"被调用过"。

**三个门禁都做了红→绿验证**（否则门禁只是装饰）：

| 门禁 | 红（注入违规） | 绿（清理后） |
|------|------|------|
| `build` 的类型检查 | 临时加 `const broken: number = "str"` → 退出码 **2**，且 `vite build` 未执行 | 退出码 **0** |
| `lint` | 临时加未使用变量 → 退出码 **1**，报错点名 `@typescript-eslint/no-unused-vars` | 退出码 **0** |
| `test:coverage` 门槛 | `--coverage.include='src/pages/**'`（未测试层）→ 退出码 **1**，四条 threshold 报错 | 退出码 **0** |

> 第一次做 lint 红绿验证时，我写的探针**本身是干净的**（`x` 被 `return` 用到了），
> 于是"门禁没拦住"的结论是错的；换了个真违规的探针才验证成功。
> 这与 T1.4 的教训同源：**验证手段本身要先被验证。**

**为让 lint 变绿所做的 45 处改动**（全部机械，且经 `tsc --noEmit` 验证）：

- 32 处 `catch (error)` → `catch`（绑定未被使用；可选 catch 绑定自 ES2019 起可用）；
- 10 处 `useEffect` 调用了在其下方用 `const` 声明的函数 → 把 effect 移到声明之后；
- 2 处 `exhaustive-deps` **真问题** → 用 `useCallback` 稳定引用并补全依赖数组
  （`Analysis.tsx` 的 `loadAnalysis`、`DataManagement.tsx` 的 `loadDataList`）。
  注意**不能**把函数直接塞进依赖数组了事——每次渲染都会产生新函数身份，会变成无限重渲染；
  `useCallback` 的依赖恰好是这些函数读取的值，因此 effect 的触发时机与改动前完全一致。

**T1.10 CI**：`frontend-build` 作业在 Install 之后插入两步 ——
`Lint`（`npm run lint`）与 `Test with coverage gate`（`npm run test:coverage`）。
必须用 `test:coverage` 而不是 `test`：覆盖率门槛配在 `vitest.config.ts` 里，只有前者会真正校验。

**T1.11 版本号单一来源** — `augmentor.__version__`（2.2.0）为唯一权威声明：

| 位置 | 原值 | 现值 |
|------|------|------|
| `api/main.py` `FastAPI(version=)` | `"2.0.0"` 硬编码 | `from augmentor import __version__` |
| `api/routes/status.py` 响应 `version` | `"2.0.0"` 硬编码 | 同上 |
| `web/package.json` `version` | `"1.0.0"` | `"2.2.0"` |
| `docker/docker-compose.yml` 默认镜像 tag | `2.0.0` | `2.2.0` |
| `docs/DEPLOYMENT.md` 示例镜像 tag | `2.0.0` | `2.2.0` |
| `.github/workflows/ai-platform-cd.yml` 触发示例 | `ai-platform-v2.0.0` | `ai-platform-v2.2.0` |

> **注意（两个版本轴不要混淆）**：Docker 镜像 tag 来自**发布用的 git tag**
> （`ai-platform-v*`，由 CD 工作流剥离前缀得到），而不是 `augmentor.__version__`。
> 因此发布 2.2.0 时必须打 `ai-platform-v2.2.0`，否则镜像 tag 与包版本会不一致。
> `docker-compose.yml` 里的 `${IMAGE_TAG:-2.2.0}` 只是本地默认值。

新增 `tests/unit/test_package_exports.py::TestVersionConsistency`：
断言 `app.version == augmentor.__version__` 与 `package.json.version == augmentor.__version__`；
`tests/integration/test_api_status.py` 的断言从硬编码 `"2.0.0"` 改为 `augmentor.__version__`。

**T1.12 文档漂移修正** — `docs/ARCHITECTURE.md`：

- `cli.py` 子命令 11 → **44**；
- `api/routes/` router 数 7 → **11**（补上 `status` / `audit` / `leakage` / `privacy` 四个模块的职责说明）；
- `api/main.py` 行数 29 → 改为不写具体行数（避免再次漂移）；
- `__init__.py` 版本 2.0.0 → 2.2.0，并注明"全仓唯一版本声明"。

**新增（不在原规划内，但补上了 Phase 0 的残留风险）**：
`tests/integration/test_api_security.py::TestWriteRouteAuthCoverage` —— 把全部 **42 条路由**的
鉴权状态固化为元测试（11 条受保护 / 31 条只读白名单）。新增路由会让测试变红，迫使改动者
显式分类。已验证：临时摘掉 `/api/config` 的 `Depends(verify_api_key)` 后两条用例均失败，
且报错直接点名 `('POST', '/api/config')`。

**顺带修掉一处既有测试缺陷**（`compileall` 的 `SyntaxWarning` 暴露）：
`tests/unit/test_model_backends.py:498` 用 `is` 比较字符串字面量
（`assert manager.get_sentence_model() is "cached-model"`）。它靠字符串驻留（interning）
**恰好**通过，但语义上应为 `==`，且 CPython 会为此发 `SyntaxWarning`。全仓仅此一处。

**T1.13 取消** — 复核发现根 `.gitignore` 已完整覆盖构建产物，新建子目录规则属重复维护。

**Phase 1 已验证**：全量 `3108 passed / 3 skipped`，覆盖率 99.41%。

**T1.14 待用户确认** — 已量化的冗余入库文件（约 11.5 MB）：

```
train_data.json          3.5 MB   md5 a83320ab…（与 train_data_final.json 完全相同）
train_data_final.json    3.5 MB   同上
train_data_final02.json  3.0 MB
train_data_final01.json  595 KB
bak/（4 文件）            1.1 MB
enhanceTXT.py（重复一份） 19450 B  md5 e74dc8a5…（根目录与 archive/ 相同）
```

**T1.3 + T1.4 已完成**：分支覆盖率 **98.63%**（语句 99.50%），变异杀死率
**逐模块** 67.3% / 61.9% / 69.0%，合计 **65.9%**（均 ≥ 60%）。
**全量已验证**：`3047 passed / 3 skipped`，覆盖率 98.63%（门禁 80%）。

---

#### T1.5–T1.7 前置侦察（只读，尚未动手）

**F7 定性升级：从「模块重复」升级为「可复现的用户可见故障」。**

两个同名 `ExportFormat`：

| 位置 | 成员数 | 成员 |
|---|---:|---|
| `augmentor/export.py:14` | 6 | JSONL / LLAMA_FACTORY / ALPACA / **SHARE_GPT** / CHATML / CSV |
| `augmentor/export_enhanced.py:17` | 13 | 上述 6 个的值 + JSON / TSV / **SHAREGPT** / VICUNA / BELLE / OPENAI / HUGGINGFACE / RAW |

`augmentor/__init__.py:38` 把 **`export_enhanced` 的那一份**导出为公开名 `augmentor.ExportFormat`，
而 `pipeline.py` / `api/routes/export.py` / `preview.py` 走的是 `export.py` 的那一份。

复现（`pipeline.export_dataset`，输入文件写临时目录）：

```
[FAIL] ExportFormat.OPENAI   -> ValueError: <ExportFormat.OPENAI: 'openai'> is not a valid ExportFormat
[FAIL] ExportFormat.CHATML   -> ValueError: <ExportFormat.CHATML: 'chatml'> is not a valid ExportFormat
[FAIL] "openai"              -> ValueError: 'openai' is not a valid ExportFormat
[OK]   "chatml"              -> {'chatml': '...\\train_data_chatml.json'}
```

即 **`augmentor.ExportFormat` 的 13 个成员没有一个能传给 `pipeline.export_dataset`** ——
连值完全相同的 `CHATML` 也不行（`Enum.__call__` 按成员身份匹配，跨枚举类的成员一律不认）。
用户唯一可用的写法是裸字符串，且只能是 6 成员子集；`SHARE_GPT` / `SHAREGPT` 是同一格式的两个拼写。

**六对模块的收敛判据**（按规划 F7 定的「被 `pipeline.py` 引用、测试更完整、API 更清晰」）：

| 对 | A（行数 / 类数 / 引用方） | B | 建议保留 | 依据 |
|---|---|---|---|---|
| comparison / compare_enhanced | 323 / 2 / cli | 383 / 4 / cli | **compare_enhanced** | 无 pipeline 引用；B 的 API 更完整（`ComparisonMetrics`/`FieldComparison`） |
| versioning / version_control | 385 / 3 / **pipeline** + 13 测试 | 364 / 2 / cli + 3 测试 | **versioning** | `pipeline` 是唯一主链路消费者；13 个集成测试直接依赖 |
| export / export_enhanced | 364 / 2 / **pipeline + api + preview** | 365 / 3 / cli | **export**（枚举取 B 的超集，见上） | 两条主链路都在 A 上；枚举是唯一冲突点 |
| visualizer / visualize_enhanced | 333 / 1 / **pipeline** | 246 / 2 / cli | **visualizer** | `pipeline` 是唯一主链路消费者 |
| statistics / analytics | 353 / 3 / cli | 527 / 4 / cli | **analytics** | 无 pipeline 引用；B 明显更完整 |
| cleaner / data.cleaner | 470 / 4 / **data/__init__ + cli** + 8 测试 | 229 / 2 / **包内零引用** + 3 测试 | **cleaner** | `data/cleaner.py` 在包内无任何消费者，仅测试引用 → 实为死代码 |

**F8 的「9 组重复命令」描述不准确**（Rule 7：暴露冲突而非折中）：

| 组 | 基础命令实现 | 增强命令实现 | 实际情况 |
|---|---|---|---|
| export / export-enhanced | `pipeline.export_dataset`（→ `export.py`） | `export_enhanced.export_dataset` | 真重复 |
| analyze / analyze-data | `pipeline.analyze_dataset` | `analytics.analyze_dataset` | 真重复 |
| visualize / visualize-data | `pipeline.visualize_dataset`（→ `visualizer.py`） | `visualize_enhanced.visualize_dataset` | 真重复 |
| version / version-control | `pipeline.version_manager`（→ `versioning`） | `version_control` 模块函数 | 真重复 |
| compare / compare-enhanced | `comparison.compare_datasets` | `compare_enhanced.compare_datasets_enhanced` | 真重复 |
| search / search-enhanced | `pipeline` 侧实现 | `search_enhanced.search_dataset` | 真重复 |
| clean / clean-enhanced | `pipeline` 侧实现 | **`cleaner.clean_dataset`（基础模块！）** | 命名误导 |
| stats / stats-enhanced | `pipeline` 侧实现 | **`statistics.calculate_statistics`（基础模块！）** | 命名误导 |
| quality / quality-report | 内联约 48 行：打分 → 过滤 → 存盘 | `quality_report.generate_quality_report` | **不是重复**：一个筛数据、一个出报告 |

即 `-enhanced` 后缀**不代表「用了增强实现」**：`clean-enhanced` 与 `stats-enhanced` 调的恰恰是基础模块；
`quality` / `quality-report` 是两种不同能力，合并会丢功能。

`cli.py` 结构现状：1308 行 / 6 个顶层函数 / 44 个 `add_parser`；
`build_parser()` 占 329 行、`main()` 占 **922 行**；分发是 **44 段 `if/elif args.command == ...` 长链**，
且每个分支内部用**函数内 import**（这也是 `main()` 膨胀到 922 行的直接原因）。

**结论**：T1.5–T1.7 不宜作为「机械重命名」推进。`ExportFormat` 那一条是**修 bug**，
优先级最高且可独立完成；其余五对是收敛；T1.7 必须先按上表把「真重复 / 命名误导 / 非重复」三类
分开处理，否则会把 `quality` 的能力合并掉。

---

#### T1.5a — 修 `ExportFormat` 同名冲突（已完成）

**性质**：**修 bug**，不是清理重复。复现见上一节。

**根因**：`Enum.__call__` 按**成员身份**匹配（`cls._value2member_map_`），
跨枚举类的成员一律不认——所以公开名的 13 个成员**没有一个**能传给
`pipeline.export_dataset`，连值完全相同的 `CHATML` 也不行。

**修法**：让枚举成为全包唯一来源，并**显式声明**「原生支持」的边界。

| 改动 | 文件 |
|---|---|
| 删除本地枚举定义，改为 `from .export_enhanced import EnhancedExporter, ExportFormat, ExportOptions` | `export.py` |
| 新增 `NATIVE_FORMATS`（原生 6 种）与 `_FORMAT_EXTENSIONS` | `export.py` |
| `Exporter.export` 对非原生格式委托 `EnhancedExporter` | `export.py` |
| `export_all_formats` 改为迭代 `NATIVE_FORMATS`（保持「一次 6 个文件」） | `export.py` |
| 保留 `SHARE_GPT = "sharegpt"` 为同值**别名**（非独立成员，不影响清单长度） | `export_enhanced.py` |
| 预览的字典索引改 `.get()` + 明确 `ValueError`（否则退化成 `KeyError`） | `preview.py` |
| `export_dataset` 把成员归一化为 `.value`（否则文件名变成 `train_data_ExportFormat.OPENAI.json`） | `pipeline.py` |

**三处必须一起改的理由**（少任何一处都会引入**新**缺陷）：

1. 只删枚举、不加委托 → 7 种非原生格式落空 `if/elif` 链，**静默不写任何文件却返回成功**，
   比原来的 `ValueError` 更难排查；
2. 只改 `Exporter`、不改 `export_all_formats` → 调用方（`pipeline` / CLI）的产物
   从 6 个变 13 个，属破坏性变更；
3. 只改 `export.py`、不改 `preview.py` → 预览的报错从约定的 `ValueError`
   退化成 `KeyError`。

**未做（刻意）**：`export_all_formats` 的**名字**仍暗示「所有格式」，但它现在只写 6 种。
改名属破坏性变更，留待与 T1.7 的命令合并一并处理；当前以 `NATIVE_FORMATS` 的文档字符串
说明边界。

---

**阶段门禁**：~~测试数从 3061 降至 ~2000~~ → **改为"测试卫生门禁全绿 + 覆盖率不下降"**
（原目标基于错误估计，见 F6 复核；实际 3108 → 3047，覆盖率 99.41% → 99.50% 语句 / 98.63% 分支）；
变异杀死率达标（逐模块 ≥ 60%）；前端三门禁（lint / test:coverage / build）全绿且都做过红→绿验证；
CI 增加 lint / test / 版本一致性 / 测试卫生四个门禁；`docs/ARCHITECTURE.md` 与代码一致。

**Phase 1 剩余**：T1.5–T1.7（会动公共接口）、T1.14（冗余数据，**需用户逐项确认**）、
T1.15（前端 API 响应类型层）。其中 **T1.5 内部已可拆出独立子项**：
`ExportFormat` 同名冲突是**修 bug**（用户可见、有复现），建议先做且单独验证；
六对模块收敛是纯内部重构，可随后。T1.7 需先按「真重复 / 命名误导 / 非重复」分类再动手。

---

## Phase 2 — 对齐：多端能力一致性（P2）

> 目标：把 CLI 与 Web 之间 3 倍的能力落差变成"有清单、有校验、可度量"。

| 任务 | 对应 | 主要改动 | 验收 |
|------|------|---------|------|
| T2.1 | P2 | 新建 `augmentor/capabilities.py`，声明每项能力的 `{id, sdk, cli, api, web}` 映射 | 覆盖全部 ~60 个能力模块 |
| T2.2 | P2 | `tests/e2e/test_web.py` 升级为**双向**能力校验 | 声明了 `web_page` 但页面缺失时测试失败 |
| T2.3 | P2 | 输出能力矩阵报告（`docs/CAPABILITY_MATRIX.md`，由 T2.1 生成） | 与 `ls api/routes/` / `cli.py --help` 一致 |
| T2.4 | P2 | 按优先级补齐 API 路由（见下表） | 每条路由有集成测试 |
| T2.5 | P2 | 按优先级补齐 Web 页面 | 每条页面有契约测试 |

**API/Web 补齐优先级**（依据：用户价值 × 已有实现成熟度 × 成本）：

| 优先级 | 能力 | 理由 |
|--------|------|------|
| P0 | `dataset_ops`（merge / sample / split） | CLI 已有，Web 无入口，是日常高频操作 |
| P0 | `converter`（格式转换） | 导出格式是核心卖点，Web 只能导出不能转换 |
| P0 | `backup`（备份 / 恢复） | 数据安全相关，Web 完全缺失 |
| P1 | `quality_trend` + `health_score` | 已实现但无出口，前端质量页可直接消费 |
| P1 | `rag`（RAG 格式） | 配置段已有 `rag.enabled`，但无 API |
| P1 | `evaluation`（BLEU / ROUGE-L） | 配置段已有 `evaluation.enabled`，但无 API |
| P2 | `streaming`（流式处理） | 大数据集场景，Web 需要 SSE 进度 |
| P2 | `sampler` + `expander` + `active_learning` | 主动学习闭环，配置已就绪但全无出口 |
| P2 | `aggregator` / `data_pipeline` / `data_splitter` | 组合能力，依赖前述基础能力先暴露 |

---

## Phase 3 — 演进：架构与性能（可选，视使用规模触发）

| 任务 | 触发条件 | 内容 |
|------|---------|------|
| T3.1 | 单次增强 > 5 万条 | 模型后端原生异步化（`httpx.AsyncClient`），替代 `run_in_thread` 线程池 |
| T3.2 | 语义检索延迟 > 1s | 向量索引持久化 + 分块矩阵计算的显存/内存上限保护 |
| T3.3 | 多实例部署需求出现 | 断点 / 版本 / 向量库从本地文件迁移到共享存储（需重新评估锁策略） |
| T3.4 | 持续 | `asyncio.get_event_loop()` → `get_running_loop()`（`api/deps.py` 3 处，Python 3.12+ 已弃用） |

---

## 端到端验收标准

### 功能正确性
- [ ] `POST /api/quality/evaluate` 对无关答案返回 `total_score < threshold`，`passed=false`
- [ ] `isinstance(ReportGenerator().generate(...), augmentor.QualityReport) is True`
- [ ] `augmentor.QualityReport.__module__` 与 `augmentor.ValidationResult.__module__` 各自唯一确定
- [ ] 项目目录外的文件无法通过任何 API 路由读写

### 安全
- [ ] 未带 API Key 的写请求返回 401
- [ ] 超过 `web.rate_limit.max_requests` 的请求返回 429 且带 `Retry-After`
- [ ] 生产环境（`debug=false`）未配置 `AUGMENTOR_API_KEY` 时启动失败
- [ ] `..` / 绝对路径 / 符号链接三种逃逸方式均有测试覆盖且被拒

### 工程质量
- [ ] `pytest -q` 全绿，`--cov-branch` ≥ 80%
- [ ] 变异测试杀死率 ≥ 60%（`quality.py` / `dedup.py` / `export.py`）
- [ ] 测试文件中无 `assert True` 占位
- [ ] `npm run build` / `npm run lint` / `npm run test` 三条命令本地与 CI 行为一致
- [ ] 版本号全仓一致，CI 有自动校验
- [ ] 跑完 `pytest` + `npm run build` 后 `git status` 干净

### 多端一致性
- [ ] `docs/CAPABILITY_MATRIX.md` 与 `cli.py --help`、`ls api/routes/`、`ls web/src/pages/` 完全一致
- [ ] 清单中声明了 `api_route` / `web_page` 的能力，对应实现必须存在（双向校验）
- [ ] 声明为"仅 CLI 可达"的能力有显式标记，而非静默缺失

---

## 风险与回滚

| 风险 | 影响 | 缓解 |
|------|------|------|
| **T0.3 路径白名单会切断现有前端调用** | 前端传绝对路径时 400 | 先统计 `web/src/services/api.ts` 实际传入的路径形态，确认均为文件名后再启用；保留 `web.data_root` 配置以兼容外部目录 |
| **T1.1 删除占位测试文件** | 可能连带删除有效断言 | 已完成：先复核（原估"50 个 round 文件"错约 15 倍，那两个文件族是真实测试），只删 28 个占位文件 + 1 个全假文件；分诊脚本 `scripts/triage_tests.py` 经两轮误报校准后才动手 |
| **T1.5 合并重复模块** | 调用方 `import` 路径失效 | 只标 `@deprecated` + `DeprecationWarning`，**不在同一版本删除**；`__init__` 导出面保持不变 |
| **T1.7 合并 CLI 命令** | 用户脚本中的旧命令名失效 | 旧名保留为隐藏别名并打印弃用提示，至少跨一个版本 |
| **T1.14 清理数据文件** | **不可逆数据丢失** | 必须先备份；`train_data.json` 与 `train_data_final.json` md5 相同才可去重；逐项由用户确认 |
| **T1.4 变异测试的命令配置错误** | 产出**假的 100% 杀死率**——比失败更危险，因为它看起来像好消息 | `run.py` 强制 `{python}` 占位符（缺失即报错退出）；用 A/B 对照（`-c "import pytest"` / `-c "pass"` / `-c "raise SystemExit(1)"`）验证过退出码管道；小样本（`--per-family 1`）先行验证 |
| **T1.4 变异得分被等价变异体压低** | 误判测试质量、白写无用测试 | 得分**只作参考不作 CI 门禁**；存活变异体必须逐条人工甄别是"真盲区"还是"等价变异" |
| **T2.x 能力清单与实现脱节** | 清单变成新的漂移源 | 清单必须由测试强制校验，不允许人工维护的"文档型清单" |

**回滚策略**：Phase 0 与 Phase 1 的每个任务独立成 commit，`git revert` 单点可回退；
Phase 2 的能力清单校验以 `pytest -m capabilities` 标记，可用 marker 临时跳过而不阻塞主线。

---

## 执行顺序建议

```
Phase 0 (P0 止血)  ──→  Phase 1 (工程质量)  ──→  Phase 2 (多端对齐)  ──→  Phase 3 (演进)
   T0.1 ~ T0.8           T1.1 ~ T1.14            T2.1 ~ T2.5           按触发条件
   正确性与安全           测试与结构              能力一致性            按需
```

**建议先做 T0.1 / T0.2 / T0.3**：这三项改动小、收益最大，
分别恢复「类型契约」「评分可信度」「文件访问边界」，且互不依赖，可并行。

---

## 附录：证据索引

| 编号 | 结论 | 复现方式 |
|------|------|---------|
| F1 | 无关答案 `total_score=0.600` / `passed=True` | `QualityScorer().batch_score([{original:q, generated:q, output:"今天天气不错哈哈哈"}])` |
| F2 | `isinstance(r, augmentor.QualityReport) == False` | `ReportGenerator().generate(items, scores)` 后比对 |
| F3 | 项目外文件被 200 读取，`total_samples=7` | `TestClient(app).post("/api/quality/evaluate", json={"input_file": <外部绝对路径>})` |
| F4 | `app.user_middleware` 无 `RateLimitMiddleware` | 打印 `app.user_middleware` |
| F6 | 60 处 `assert True`、50 个 round 文件 | `grep -rn "assert True" tests/ \| wc -l` |
| F7 | 六对重复模块 | `grep -n "^class " augmentor/{comparison,compare_enhanced,...}.py` |
| F8 | `main()` 922 行、44 子命令 | `grep -n "^def " cli.py` + `grep -c add_parser cli.py` |
| F9 | 无 lint / 无测试 / build 不类型检查 | `find web -name "*.test.*" -o -name "eslint.config.*"` 为空 |
| F10 | 2.2.0 / 2.0.0 / 1.0 三值并存；文档称 7 个 router（实际 11） | `grep -rn "__version__"` + `ls api/routes/*.py \| wc -l` |
| F11 | `train_data.json` 与 `train_data_final.json` md5 相同 | `md5sum train_data*.json` |
| F12 | `AttributeError: 'Bare' object has no attribute '_session'` | `pytest -q` 输出的 `PytestUnraisableExceptionWarning` |
| F6 / T1.3 | 分支覆盖 **98.63%**（3070 分支 / 117 partial），仅比语句低 0.87pp | `python -m pytest -q` 读 `--cov-branch` 汇总行 |
| F6 / T1.3 | 生产源码 `pragma: no cover` **0 处**（原判"滥用"不成立） | `grep -rn "no cover" --include=*.py augmentor/ api/ cli.py` |
| F6 / T1.4 | export 变异得分 **51.7% → 69.0%**（补 7 条测试后）；三模块合计 65.9% | `python scripts/mutation/run.py quality dedup export` |
| F6 / T1.4 | 裸 `python` 作 test-command 会让所有变异体被误判 killed | 把 TOML 的 `{python}` 改回 `python`，再 `--per-family 1` 跑一次，看是否 100% killed |
| F9 / T1.8 | `tsc -b` 会生成 `vite.config.js` 并**遮蔽** `vite.config.ts` | `cd augmentor/web && npx tsc -b && ls vite.config.js` |
| F9 / T1.9 | 首次 lint 有 **91 处**问题（89 error / 2 warning），"零 warning"前提不成立 | `cd augmentor/web && npx eslint . -f json` 统计 |
| F9 / T1.9 | `api.ts` 覆盖率 **100%**（29 用例），门槛 70% | `cd augmentor/web && npm run test:coverage` |
| F9 / T1.9 | 三门禁均可红→绿（类型错误/未使用变量/覆盖率不足都会让对应命令非 0 退出） | 见 T1.8–T1.10 一节的验证表 |
| F7 / T1.5 | `augmentor.ExportFormat` 的 13 个成员**全部**被 `pipeline.export_dataset` 拒绝（`ValueError`） | 见「T1.5–T1.7 前置侦察」的复现块 |
| F7 / T1.5 | 两个同名枚举成员数 6 vs 13；`SHARE_GPT`/`SHAREGPT` 同值不同名 | `augmentor/export.py:14`、`augmentor/export_enhanced.py:17` |
| F7 / T1.5 | 六对模块的引用方与行数（用于选主实现） | 见「T1.5–T1.7 前置侦察」的收敛判据表 |
| F8 / T1.7 | `clean-enhanced` 调 `cleaner`（基础模块）、`stats-enhanced` 调 `statistics`（基础模块） | `cli.py:863`、`cli.py:1074` |
| F8 / T1.7 | `quality` 是内联 48 行筛选逻辑，与 `quality-report` 不是重复 | `cli.py:479-526` vs `cli.py:895-919` |
| F8 / T1.6 | `cli.py` 1308 行 / 6 顶层函数 / 44 `add_parser` / `main()` 922 行 / 44 段 if-elif | `cd augmentor && grep -c add_parser cli.py` |
| F13 | `json.dumps({"passed": np.False_})` → `TypeError` | `type(np.False_)` + `isinstance(..., bool)` |
