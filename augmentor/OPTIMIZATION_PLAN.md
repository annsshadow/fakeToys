# AI 训练数据增强工具 — 优化计划与完成记录

> 本文档既记录**计划**，也记录**实际结果**。所有数字都取自当前仓库的实跑输出，
> 不是估算。与旧版本（2.2.0 时期的 89% / 648 passed）已完全不同，故整篇重写。

## 一、项目现状

### 1.1 项目架构

```
augmentor/
├── augmentor/          # 核心增强包（Python）
│   ├── models/         # 模型后端
│   ├── data/           # 数据处理
│   ├── vector/         # 向量数据库
│   ├── frameworks/     # 框架集成
│   ├── config.py       # 配置管理
│   ├── pipeline.py     # 增强管道
│   ├── quality.py      # 质量评分
│   ├── dedup.py        # 智能去重
│   ├── export.py       # 多格式导出
│   ├── streaming.py    # 流式处理
│   ├── comparison.py   # 数据集对比
│   ├── dataset_ops.py  # 数据集操作
│   └── exceptions.py   # 统一异常体系
├── api/                # FastAPI 服务（routes/ 按能力分组）
├── web/                # 前端（React 18 + Ant Design 5 + Vite）
├── tests/              # 测试套件（unit / integration）
├── docs/               # 文档
├── docker/             # Docker 配置
├── archive/            # 归档文件
├── cli.py              # CLI 入口
└── config.yaml         # 配置文件
```

### 1.2 当前状态

| 维度 | 现状 |
|------|------|
| 版本 | **3.0.0**（`augmentor.__version__` 是全仓唯一声明） |
| 测试 | **3658 个**（3658 passed / 3 skipped），覆盖率 **98.55%**（门槛 80%） |
| 版本一致性 | FastAPI 元数据 / `web/package.json` / `docker-compose.yml` 默认 tag 三处均由测试锁定 |
| HTTP API | **68 个端点 / 13 个 tag**，全部声明 `response_model`，无「无 schema 的 200 响应」；23 条写/删路由挂 `verify_api_key`，由动态发现的路由清单守门 |
| 路径白名单 | 出厂默认 `web.data_roots` = **`["data"]`**（原 `["."]` 放行整个工作目录）；三处默认值由字面量钉住并交叉比对。`/api/data/list` 扫描范围与白名单同源；相对路径**先在根目录内查已存在的文件**、工作目录兜底，写入需显式 `data/x.json` |
| 前端 | **11 个功能页**，服务层 43 个函数；契约测试 + 接线守门共 45 个用例 |
| 模型后端 | ERNIE、OpenAI、Ollama、Claude、Gemini |
| 导出格式 | 以 `ExportFormat` 枚举为准（13 种，含 `raw`）；CLI `--format` 与转换图的支持面由 `TestExportFormatSurface` / `TestConvertFormatSurface` 双向锁定 |

---

## 二、3.0 已完成的工作

### 2.1 健壮性与正确性

| # | 内容 | 验证 |
|---|------|------|
| 1 | 接入统一异常体系（85 处 raise / 40 个文件），消除「裸 `Exception` + 字符串」的不可判别错误 | 3130 passed |
| 2 | 修静默吞异常（`except: pass` 之类）与 4 处契约违规 | 3130 passed |
| 3 | 修 `config_validator` 的配置闭环缺陷：`save_config` 写出的配置**必然校验失败** | 见 2.4 |
| 4 | 移除 `--enhanced` 双轨与 legacy 命令名/别名（8 对命令合并为一套实现，落败侧独有能力并入幸存侧） | 两轮共 17 项缺陷注入全部被捕获 |

### 2.2 性能（均为实测峰值/规模，非估算）

| # | 问题 | 结果 |
|---|------|------|
| 1 | dedup 的 N×N 相似度矩阵内存炸弹 | 峰值 **9536 MB → 20 MB** |
| 2 | quality 缓存键两个 bug（导致缓存几乎不命中） | 编码量 **2965 → 865** |
| 3 | 伪流式（先全量读进内存再分块）改为真流式 | 峰值 **22.70 MB → 1.66 MB** |
| 4 | 无界缓存改为有界 + 接入 `cache.py` | 3199 passed / 98.78% |
| 5 | 接入 `retry.py` + 限流识别 | 3157 passed |
| 6 | `DiskCache` opt-in 接入模型响应缓存 | 3217 passed |

### 2.3 契约与工程化

| # | 内容 |
|---|------|
| 1 | OpenAPI 元数据补全（13 个 tag 的描述、`version` 取自包版本） |
| 2 | **68 个端点全部补 `response_model`**，并加**双向守门**：既要每个端点都声明契约，也要每个已声明契约的端点都真实存在 |
| 3 | 守门测试自身防空转：数组响应同时识别 `items.$ref`；枚举类断言配非空元断言；测试夹具钉住 `api.deps._pipeline` 单例 |
| 4 | 前端服务层**接线守门**：`api.ts` 的每个导出函数都必须在页面源码里被真正引用（`import` 与注释都不算） |
| 5 | 版本号单一来源，三处副本（FastAPI / `web/package.json` / `docker-compose.yml`）全部由测试锁定 |

### 2.4 配置闭环缺陷修复（由新端点暴露）

| 问题 | 修复 |
|------|------|
| `save_config` 写顶层 `default_model`，而 `load_config` 只认 `models.default` | 写入时归一化进 `models.default`；旧文件的顶层键会被保留并合并 |
| `AppConfig` 不建模的顶层段（如 `app`）在写配置时被抹掉 | 保留既有文件中未建模的顶层键 |
| 模型密钥明文落盘 | 只在原文件是 `${ENV_VAR}` 占位符时保留，否则移除 |
| `ConfigValidator` 要求 `app` / `app.name` / `app.version` 必填，而 `AppConfig` 无对应字段、`load_config` 从不读它 → **写出的配置必然不合法** | 降为可选；`models` / `models.default` 保持必填 |

### 2.5 能力扩展

| 分组 | 数量 | 说明 |
|------|------|------|
| `dataset`（新增） | 12 | CLI 数据集工具的 API 化：stats / validate / search / compare / features / auto-config / convert / merge / sample / split / aggregate / rag |
| `system`（新增） | 13 | 依赖诊断 / 校验配置 / 监控 / 自动化测试 / 迁移 / 流式 / 依赖图 / 备份增删改查 |
| 既有分组补齐 | — | privacy（脱敏）、leakage（泄漏）、audit（就绪审计）、quality 的 outliers / profiling、status（综合状态）、demo（演示数据） |

约定：`dataset` 里**只读分析类**内联返回完整结果；**写盘变换类**必须给
`output_path`，响应只回传「写到哪、写了多少」。

### 2.6 前端

| 内容 | 说明 |
|------|------|
| 新增 `数据安全` 页 | 隐私脱敏（含 PII 模式清单）/ 泄漏检测 / 就绪审计 |
| 新增 `系统状态` 页 | 服务综合状态、依赖明细、降级功能 |
| 质量中心扩展 | 新增「离群点检测」（zscore / zscore_one_sided / iqr）与「数据画像」两个 Tab |
| 数据管理 | 「载入演示数据」改走服务层，不再裸 `fetch` |
| 类型层 | `src/types/api.ts` 增至 61 个接口 / 类型别名，全部来自对后端**真实响应**的探测 |
| 服务层 | `services/api.ts` 增至 43 个函数，13 个新契约用例 + 4 个接线守门用例 |

### 2.7 验证方法（这套方法本身是本次交付的一部分）

1. **探针优先于猜测**：写 `response_model` / TS 类型前，先用真实 `TestClient`
   逐个打端点、打印响应键。本轮因此当场抓到两处模型与真实返回不符
   （`/api/dataset/stats` 的 `summary` 是字符串不是 dict、
   `/api/dataset/features` 的 `field_features` 是列表不是 dict）。
2. **反事实验证（缺陷注入）**：把「缺陷」改回生产代码，确认测试真的变红，
   且失败信息里出现预期标记 —— 否则可能是「碰巧因为别的原因红了」。
   本轮各任务累计注入 40+ 项，全部被捕获。最近一次（P3 后端两项）用
   `_inject_check.py` 一次性验证 13 项，覆盖 `to_http_error` 的翻译分支、
   四处枚举白名单、`preset` 默认值、`except HTTPException` 透传、
   500 收尾分支，以及 `streaming.py` 的空白跳过 / 截断语义 / 补缓冲 /
   `close()` 条件。脚本在 `try/finally` 里还原源文件，任何一步崩溃都不会留下脏改动。
3. **同源预言机是假测试**：`set(payload) == _field_names(model)` 在模型漏字段时
   恒真（响应与字段集一起变小）。正确做法是用**领域契约**（handler 实际返回的
   `to_dict()` 键）作独立预言机。
4. **当一次改动同时触及实现与它的测试时，绿灯不再构成证据**：修
   `config_validator` 时有 5 个既有测试变红，其中 4 个**固化了缺陷语义**。
   更新它们时必须同时把判据换成真正必填的 `models.default`，并写明旧断言为何错。
5. **shell 引号坑（已栽三次）**：`python -c "...\n..."` 在 git-bash 下会把
   `\n` 当字面量传进去，产生**假的失败**。本轮据此误判「最小合法配置校验不通过」，
   换成真实文件后确认 `is_valid: true`。凡是要构造多行文本的探针，一律写
   `.py` 文件再执行。

### 2.8 新端点业务分支与错误语义收口

契约测试只保证「响应形态对」，不保证「业务分支对」。本节补齐 25 个新端点的
业务分支，并顺手收口了三处**同一语义两种结果**的不一致。

| 内容 | 说明 |
|------|------|
| `api/deps.py` 新增 `to_http_error()` | 把 404/400/500 的判据收到一处。此前同一个「文件不是合法 JSON」在有的端点是 400、有的是 500，且都把 `"Expecting property name enclosed in double quotes: line 1 column 3"` 这种解析器措辞原样回给客户端。现在只回「数据文件不是合法 JSON（第 N 行第 M 列）」 |
| `_sync_read_json` → 公开 `read_items()` | 收**已 resolve 的 Path**，供「一次校验、多次读取」的路由复用（对比读 2 个文件、聚合读任意多个）。同时删掉重复定义的 `read_json_file` |
| 坏 JSON 泄漏端点 6 → **0** | 探针实测：`validate` / `convert` / `merge` / `sample` / `split` / `migrate` 曾把英文原文透出 |
| 4 组枚举白名单 | `preset` / 搜索 `method` / 自动测试 `suite` / 迁移 `rules`。下游对未知值是**静默降级或静默过滤**的：`DatasetValidator` 回退 basic、`EnhancedSearcher` 回退 contains、`DatasetMigrator` 静默丢弃未知规则 id、`run_dataset_tests` 回退 default。调用方会以为「strict 校验通过了」「迁移跑了」 |
| `ValidateRequest.preset` 默认值 `"default"` → `"basic"` | 旧默认值**根本不存在**于 `PRESET_RULES`，于是默认请求也在静默回退 |
| 恢复不存在的备份 400 → **404** | 同一组路由里 `DELETE /api/system/backups/{id}` 对同样情况回 404，不该有两种状态码。为此给 `to_http_error` 加了 `not_found_types`（`BackupError` 在库里唯一的抛出点就是「备份不存在」） |
| 折叠 24 处重复 `except` 块 | 脚本化替换，带三重自检（匹配数、残留 500 收尾数、`ast.parse`） |
| `tests/integration/test_api_dataset_system_tools.py` | **151 个用例**：只读分析 / 写盘变换 / 依赖注册表 / 备份生命周期 / 跨端点错误语义 / HTTPException 透传 / 意外失败 → 500 故障注入 |
| `tests/unit/test_streaming_edge_branches.py` | **22 个用例**：`_peek_first_non_space` 分块读取、`_is_single_json_value` 补缓冲、数组两种截断语义、JSONL 空行、`StreamWriter` 收尾分支 |
| 覆盖率 | `api/routes/dataset_tools.py` 67% → **100%**；`api/routes/system_ops.py` 62% → **100%**；`augmentor/streaming.py` 89% → **100%** |
| 反事实验证 | 13 项缺陷注入全部被捕获 |

**流式端点为什么**不**并入「坏 JSON 一律 400」**：JSONL 是逐行格式，单行语法错误
跳过并 `logger.warning`；而 JSON **数组**结构明确，截断必须报 400。两者行为相反，
因此 `/api/system/stream` 单独钉住，包括「整份文件非法且以 `{` 开头 → 200 且产物为空」
这个边界 —— 把它写清楚，避免日后被当成「静默吞异常」误改。

---

### 2.9 3.0.0 复审：新缺陷与整改（本轮）

对 3.0.0 重扫一遍，按「可复现实测证据」确认 8 项新缺陷（F-07 是需产品决策的出厂
默认值，F-08 是收紧它时自己引出的前端回归）。基线：本轮开始前
3605 passed / 98.37%，本轮结束 **3658 passed, 3 skipped / 98.55%**
（+53 用例，全部经红→绿校验：把修复代码退回缺陷态，新测试必须失败）。

| 编号 | 缺陷 | 实测证据 | 整改 |
|------|------|---------|------|
| F-01 | **6 条写盘路由漏挂 `verify_api_key`**，且为此设的鉴权守门测试**自身失查** | 配了 `AUGMENTOR_API_KEY`、不带 `X-API-Key` 调 `POST /api/dataset/convert` → **200 并真的写出文件** | `dataset_tools.py` 6 条 + `system_ops.py` 2 条补挂；`test_api_security.py` 的模块清单由硬编码 11 项改为 `pkgutil.iter_modules` 动态发现，`PROTECTED` 11→23、`OPEN_ALLOWLIST` 31→44 |
| F-02 | 数据文件**顶层形态不校验**，下游 Python 异常原文回给客户端 | 顶层是对象 / 元素是标量的文件进 `/api/dataset/stats` → 500，`detail` 里是 `'str' object has no attribute 'get'` | `deps.read_items()` 单点收口：非数组顶层 / 非对象元素 → 400 中文文案（`第 N 个数据项必须是 JSON 对象，当前是字符串`）。`/api/dataset/validate` 例外——它的职责就是**报告**坏数据，行为已由测试钉住 |
| F-03 | CSV/TSV 导出三处产物缺陷 | `convert --format csv` 产物字节实测 `b'instruction,output\r\r\n...'`；异构条目 → `ValueError: dict contains fields not in fieldnames`；空数据集 → 0 字节却报成功 | 写侧补 `newline=''`（POSIX 不显形，只在 Windows 露头）；新增 `csv_fieldnames()` 取**全量键并集**，`converter` / `export_enhanced` 的 CSV+TSV 四条路径共用 |
| F-04 | 公开「支持格式」清单与实现两处对不上 | `converter.get_supported_formats()` 报 `tsv`，但转换图没有 `json -> tsv` 边 → `UnsupportedFormatError`；CLI `export --format` 少了 SDK/API 都支持的 `raw` | 转换侧清单改为由 `_converters` **反推**（枚举不再是事实来源）；`EXPORT_FORMATS` 补 `raw`；新增三方一致门禁 `TestExportFormatSurface` / `TestConvertFormatSurface` |
| F-05 | **6 处 SDK 随机操作污染进程级 RNG** | `DataSplitter.split()` 之后调用方的全局随机序列被改写（参照序列对照实测：`0.9097 != 0.2929`） | `dataset_ops`（sample/split/shuffle/merge/模块级 helper）、`data_splitter`、`indexer`、`export_enhanced` 全改局部 `random.Random(seed)`；新增 `tests/unit/test_rng_hygiene.py` 门禁。CLI 侧同一坑 3.0 已修，SDK 侧当时漏了 |
| F-06 | 文档/注释漂移 4 处 | `docs/README.md` 教用户用已删除的 `--no-url-removal`；`parser.py` 注释写「`history` 取消」而 `choices` 与实现都在；导出格式数写死 12 | 改为真实可执行命令；注释与实现对齐；文档里的**会漂移数字**改成「以枚举/清单为准」的写法 |
| F-08 | **收紧默认值会让前端文件选择器集体 403**（收紧动作自己引出的缺陷） | 实测：把 `data_roots` 收紧后按旧解析规则跑闭环 —— 列表给出 `train_data_ui.json`，`GET /api/data/load/train_data_ui.json` 返回 **403**（相对路径只按工作目录解释，而工作目录已不在白名单内）。前端 8 处（`DataList.tsx`/`Analysis.tsx`/`AugmentForm.tsx`/…）都把 `files[].name` 原样拼回 URL，且 `{filename}` 只匹配单个路径段，改传 `data/x.json` 也走不通 | `resolve_within_roots` 对相对路径生成候选：**白名单根目录在前、工作目录兜底**，取第一个存在的解释（`_relative_candidates`）。闭环由 `TestBareNameResolvesInsideRoots::test_listed_name_can_be_loaded_back` 钉住（红→绿已验：403 → 200）。**写**一侧保持显式（`data/out.json`）：目标不存在时不替调用方猜目录，那种猜测会静默改掉产物落点，宁可 403 |

**方法学收获**（两条，都已写进对应用例的 docstring）：

1. **守门测试自己需要守门**。`test_api_security.py` 用一份手抄的路由模块清单来
   检查「新路由有没有挂鉴权」，于是新增 `dataset_tools.py` 时它静默失明。修法是
   加一条**独立**判据：OpenAPI 声明的每条路由必须落在「已保护 ∪ 已豁免 ∪ 应用级」
   之内，否则失败——这条不依赖那份清单，攻击者同时改两张表也骗不过去。
2. **判据必须先做可失败性校准**。第一版 RNG 门禁写成「seed → 取值 → 调用 → 再
   seed → 取值」，两次都重新播种所以恒等，注入缺陷后仍然绿灯。改成「同一序列中
   插入调用，比较后续值」才真正可失败。这与 §2.7 的「同源预言机是假测试」同源。

**3.2 出厂默认已收紧（F-07，2026-09-23 由维护者决策执行）**：`web.data_roots` 由
`["."]` 改为 `["data"]`。**代价是破坏性的，尚未替用户迁移数据**：

- **读**一侧的写法不变：裸文件名会在白名单各根目录内查找（见 F-08），所以
  `/api/data/load/train_data.json` 与前端列表都能继续用；命中不了工作目录解释时才 403。
- **写**新文件必须显式给白名单内的路径，如 `data/out.json`；服务端不猜写入目录。
- 工作目录（`augmentor/`）根下的 4 个 `train_data*.json`（共约 10.3 MB，
  `train_data.json` / `train_data_final.json` / `train_data_final01.json` / `train_data_final02.json`）
  以及 `up.json`、`test_output.json` 从此**对 API 不可见**（它们不在 `data/` 里，
  裸文件名查找也找不到）。要继续用需自行移动：`mv train_data*.json data/`，
  或把该目录并进 `AUGMENTOR_DATA_ROOTS`。
  这些文件由 `.gitignore` 显式忽略（`augmentor/train_data.json` 等条目），**不在版本库里**，
  所以移动后需同步 `.gitignore` 的路径，且换机器不会自动恢复。
  文档示例（`docs/README.md`、`enhanceTXT.py` 等）里引用这些根路径的地方需同步。
- Docker 部署不受影响：镜像工作目录 `/app`、数据在 `/app/data`，新默认与之一致。
- 需要旧行为时显式声明 `web.data_roots: ["."]` 或设 `AUGMENTOR_DATA_ROOTS`——
  白名单收紧后这两条是唯一的逃生阀，且都有测试覆盖。

---

## 三、待办

### 3.1 前端

- [ ] 页面级组件测试（RTL 已就位，`src/test/setup.ts` 已引入 jest-dom，但尚无组件测试）
- [ ] 5 个历史遗留的未接线服务函数：`getCheckpoints`、`visualizeData`、`getVersion`、`getVersionData`、`getVersionHistory`（已列入 `api.wiring.test.ts` 的显式白名单）
- [ ] 运行时 schema 校验（如 zod），把「编译期声明」升级为「前后端契约有机器保证」
- [ ] 前端 bundle 拆分：`antd-vendor`（946 kB）与 `echarts-vendor`（1146 kB）超过 800 kB 告警线

### 3.2 后端

- [x] **`web.data_roots` 出厂默认 `["."]` → `["data"]`**（旧默认放行整个工作目录子树，
      含 `config.yaml` 与备份目录）。已由维护者决策、本轮收紧并配三重钉住测试，
      迁移代价与待办见 §2.9 末「3.2 出厂默认已收紧」
- [x] 新增 `dataset` / `system` 两组共 25 个端点的业务分支集成测试
      → `tests/integration/test_api_dataset_system_tools.py`（151 用例，
      两个模块均达 100%）
- [x] `augmentor/streaming.py` 覆盖率补齐 → `tests/unit/test_streaming_edge_branches.py`
      （22 用例，89% → 100%）

### 3.3 文档

- [ ] 为 `dataset` / `system` 两组的 25 个端点补人读字段说明（目前只给端点总览，字段以 `/openapi.json` 为准）
- [ ] FAQ

---

## 四、优先级

### P0 — 已完成 ✅

统一异常体系、静默吞异常、配置闭环缺陷、dedup 内存炸弹、伪流式

### P1 — 已完成 ✅

缓存键 bug / 有界缓存 / DiskCache、`--enhanced` 双轨合并、OpenAPI 元数据与双向守门、版本号一致性锁定

### P2 — 已完成 ✅

68 个端点全量响应契约、`dataset` / `system` 能力补齐、前端数据安全页与系统状态页、接线守门

### P3 — 待完成

前端：页面级组件测试、5 个未接线服务函数、运行时 schema 校验、bundle 拆分
文档：25 个端点的人读字段说明、FAQ

（后端两项已于本轮完成：新端点业务分支测试、`streaming.py` 覆盖率补齐）
