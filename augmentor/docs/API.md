# API 文档

REST API 由 FastAPI 提供，默认监听 `http://localhost:8000`。交互式文档见 `/docs`（Swagger UI）。

```bash
uvicorn api.main:app --host 0.0.0.0 --port 8000
```

## 通用约定

| 项目 | 说明 |
|------|------|
| Base URL | `http://<host>:<port>` |
| 请求体 | `Content-Type: application/json`（上传接口除外） |
| 响应体 | JSON |
| 响应头 | 每个响应携带 `X-Process-Time`（秒，浮点数） |
| 文件参数 | 所有 `input_file` / `filename` 均为**服务端路径**，非 URL，且必须落在路径白名单内（见下） |

### 路径白名单

服务端不会打开白名单之外的任何路径。出厂默认只有一个根目录 —— 进程工作目录下的
`data/`。

**相对路径的解释顺序**：先在白名单各根目录内查找，再退回按工作目录解释。所以
`GET /api/data/load/train_data.json` 会命中 `data/train_data.json`，前端从
`/api/data/list` 拿到的裸文件名可以直接用。

**写入必须显式给出路径**（如 `data/out.json`）。目标文件不存在时服务端**不会**替你
猜该写到哪个根目录 —— 那种猜测会静默改掉产物落点，宁可返回 403。

优先级：环境变量 `AUGMENTOR_DATA_ROOTS`（`os.pathsep` 分隔）> `config.yaml` 的
`web.data_roots` > 出厂默认 `["data"]`。

**白名单本身写坏时落到出厂默认，不落到工作目录**。`web.data_roots` 必须是 YAML 序列，
标量写法（`data_roots: data`）会被逐字符拆成 `d/a/t/a` 四个根、`- null` 会变成一个名
叫 `None` 的根 —— 两种在 3.x 都在 `load_config` 处判掉（L51 / A80）。进程启动时配置不
合法则服务不起（`api/main.py` 在 import 期加载配置）；**运行中**改坏配置文件则由
`allowed_data_roots()` 兜住，记一条 warning 并回退出厂 `["data"]`，既不会 500、也不会
因为白名单为空而把所有请求变成 403。

| 输入 | 结果 |
|------|------|
| 空字符串 / 纯空白 | 400 |
| 含 `..` 组件 | 400 |
| resolve 后（含跟随符号链接）不在白名单内 | **403** |
| 在白名单内但文件不存在 | 404 |

`GET /api/data/list` 的扫描范围与白名单同源，因此它列出的文件一定都读得到。

**目录类参数的缺省值也在白名单内**。`/api/system/dependency/*` 的 `registry_path`
与 `/api/system/backups*` 的 `backup_dir` 都是可选参数；留空时分别使用白名单首个根目录下
的 `.dependency_registry` 与 `.backups`（出厂默认即 `data/.dependency_registry` 与
`data/.backups`），因此 CLI 用 `--registry-path data/.dependency_registry` 就能与 API
共用同一份登记。显式传入时照旧校验，越界 403；显式传空串（`?backup_dir=`）是非法入参
→ 400，不会被静默换成缺省值。

**服务自身的配置文件不走数据白名单**。`POST /api/system/validate-config` 的 `path`
可选：留空时校验服务端自己在用的那份配置（`AUGMENTOR_CONFIG_PATH`，未设置则是工作目录下
的 `config.yaml`）；显式传入时仍按数据白名单校验。`POST /api/config` 保存的也是同一个
路径，二者不会各写一遍字面量。

**`POST /api/config` 的越界值是 400，且整批不落**（3.x，L73 起）。改前该端点用裸
`setattr` 写字段，而运行时判据住在各配置节的 `__post_init__` 里、`setattr` 不会再触发它
⇒ 坏值当次回 200、还能被原样写进 `config.yaml`，症状是「服务跑得好好的，重启后起不来」。
现在每写一条就跑该节自己的判据（界仍然只住 `augmentor/config.py` 一处），任一条越界即
整批退回原值并回 **400**（`detail` 点名是哪个键、当前值多少），内存与磁盘都停在改前状态。
未知子键的既有契约不变：仍然 200，键名列在响应的 `ignored_keys` 里。
至今**没有**运行时判据的节（`quality` / `dedup` / `export` / `vector` / `rag` / `multimodal`）
照旧只写不判，见账本 A118。

### 配置的「写了没人读」反馈（3.x）

拼错的配置键**不会让服务起不来**：`load_config` 只按每节已知的字段取键，多出来的键被
直接忽略。所以过去的症状是「我明明改了数，行为一点没变」，而 `validate-config` 全程
沉默。现在这类反馈会进 `warnings`（CLI 的 `validate-config` 与
`POST /api/system/validate-config` 共用同一份结果），**并且不用你先跑诊断命令**：
`load_config` 在加载这一步会用 `logging.warning` 把同一批条目原样说出来（见下面「加载即出声」）。
**仓库里不带示例配置文件**：拿出厂
`config.yaml` 改两处即可逐字复现下面这份输出 —— 在 `augmentation` 节里加一行拼错的
`variant_per_seed: 3`（真键 `variants_per_seed` 那行留着），再另加一节拼错的节名：

```yaml
augmenation:
  max_retries: 3
```

```
$ python cli.py validate-config --config typo_config.yaml
配置验证结果: 通过
错误: 0, 警告: 7, 信息: 0

警告:
  - augmentation.variant_per_seed: 键没人读取: augmentation.variant_per_seed（值不会生效）；是否想写 variants_per_seed？
  - augmenation: 顶层段落没人读取: augmenation（写了不会生效）；是否想写 augmentation？
  - models.ernie.api_key: 环境变量未设置: BAIDU_API_KEY
  - models.ernie.secret_key: 环境变量未设置: BAIDU_SECRET_KEY
  - models.openai.api_key: 环境变量未设置: OPENAI_API_KEY
  - models.claude.api_key: 环境变量未设置: ANTHROPIC_API_KEY
  - models.gemini.api_key: 环境变量未设置: GOOGLE_API_KEY
```

> 上面这份输出是本轮在两套解释器（venv 3.13.14 与 `C:\\Python314` 3.14.4）各跑一遍、**逐字节相同**的实跑结果
> （Temp `l52q/doc_repro2_report.txt`，NONCE-45A0C1AB9510-DOC2）。两条读数顺带说明了判据的形状：
> **错节只报节名那一条**（`augmenation` 里的子键不再逐个判，节都没人读，键更没人读），
> 而**真节里的错键逐条报**（`augmentation.variant_per_seed`），两条都带「是否想写 X」。

> 那 5 条「环境变量未设置」是既有告警，**条数随本机环境而变**（对应键已 export 就不出现），
> 别把 `警告: 7` 当契约读；本轮新增的判据只是前两条。

- **这类反馈一律是 WARNING，不改 `is_valid`**：`is_valid` 同时是本端点响应的判决位和
  `validate-config` 的退出码判据（L53 起 CLI 那一半才真的接上），把多余的键判负会让
  「配置里夹了自己的段落」的用法凭空变红。
- **`validate-config` 的退出码跟着判决走**（L53 起，实测两解释器一致）：报 ERROR ⇒ `exit 1`；
  只有 WARNING（含上面那两条「没人读」）⇒ 不判负、也不抛 `SystemExit`，与 `health-gate` 同口径。
  配置坏到加载器拒收（如 `web.port: 99999`）时也不再是裸 traceback：CLI 统一走
  stderr 的「错误: …」+ `exit 1`，那一档 stdout 全空。
- **加载即出声（A84）**：同样这两条「没人读」由 `load_config` 用 `logging.warning` 在
  **加载配置的那一步**发出，所以任何一条命令（不止 `validate-config`）、以及 API 进程启动
  都会说，不用你先想到去跑诊断命令。落在 **stderr**，不写 stdout（那是各命令的输出契约），
  不改退出码（与下面「只有 WARNING 不判负」同口径）。出厂 `config.yaml` 零命中 ⇒ 正常一次
  运行**一行都不多**。目前**没有把它静音的配置旋钮**：`logging` 节的 `level` / `file` / `format`
  三键尚未接到任何出口（**A97**），API 进程用的是 `api/main.py` 里那次硬编码的
  `basicConfig(level=INFO)`，纯 CLI 进程则由 `logging` 的默认 handler 落 stderr。实跑（上面那份改过两处的 `typo_config.yaml`）：

  ```
  $ python cli.py --config typo_config.yaml doctor
  键没人读取: augmentation.variant_per_seed（值不会生效）；是否想写 variants_per_seed？
  顶层段落没人读取: augmenation（写了不会生效）；是否想写 augmentation？
  ```

  > 上面两行是 stderr 的全部内容，本轮在两套解释器（venv 3.13.14 与 `C:\\Python314` 3.14.4）
  > 各跑一遍、**逐字节相同**（各 199 字节，Temp `l54/doc_repro_aug.txt` 与
  > `l54/doc_repro_py314.txt`，NONCE-L54-DOCREPRO）；`doctor` 打到 stdout 的体检报告随
  > 本机装了什么包而变，故不引。两条通道由**同一个** `_warn_unread_keys` 产生而不是各抄
  > 一遍（同源性钉在 `tests/unit/test_config_unread_feedback.py`）。
  > 「环境变量未设置」那一路**不进**加载声：它的条数随本机 shell 变，接进来等于让同一份配置
  > 在不同机器上行数不同。
- **L57 勘误（上面那句「目前没有把它静音的配置旋钮」已为假，原文一字未删）**：`logging`
  节的三键从 L57 起真的接到了出口（新模块 `augmentor/logging_setup.py`，A97 结案）。
  现在的口径是：
  - `level: ERROR` 就能静音那两行「没人读取」—— 实测同一条命令的 stderr 从
    107 B / 1 行变 0 B / 0 行，而 rc 与 stdout 一字不变（Temp `l57/ab.txt` 判据 4）。
  - **只收大写级别名**（`WARNING` / `INFO` / `DEBUG` …）：`logging.setLevel` 对小写是
    `ValueError: Unknown level`，实测两套解释器同形，所以校验器与运行时用**同一份**
    允许集，谁也不比谁宽（Temp `l57/probe2.txt` P4）。
  - `file` 非空时**同时**落文件，**控制台不少字**：root 一有了 handler，`logging.lastResort`
    就不再兜底，不自己补一条 stderr handler 就等于把 CLI 的输出吃掉（16 条命令实测
    「控制台被改动的命令: 无」，同文件判据 3）。相对路径按**当前工作目录**解释。
  - `format` 必须在**加载期**渲染得开：`Formatter("%(nope)s")` 构造期不报错，到发第一条
    日志才抛，代价是每条日志一行 `--- Logging error ---` + 31 行 traceback。
  - **默认档一字未改**：14 条命令在「无 `logging` 节 / `logging:` / `logging: {}`」三臂下
    rc 与两个 stdout/stderr 逐字节相同（`l57/ab.txt` 判据 1），出厂 `config.yaml` 里这一节
    现在是一段**注释块**（写了才生效，没写就还是今天的形状）。
  详见 `docs/ARCHITECTURE.md` §3.32、`tests/unit/test_logging_wiring_a97.py`（82 例）与
  `tests/integration/test_cli_logging_wiring.py`（17 例，真子进程）。
- **配置节写成 null 或标量（A101，L57）**：只写节名不给值（`logging:`）或写成标量
  （`logging: app.log`）在改前抛 `AttributeError: 'NoneType' object has no attribute 'get'`，
  而这不是 `logging` 一节的问题 —— 实测**每一节同形**（`web` / `quality` / `dedup` 全中，
  Temp `l57/probe1.txt` P4/P5）。现在的口径与 A94 同一档：**null = 该节全默认**（「写了节名
  没提任何要求」与「没写这节」同形），**标量 = 明确拒收**，报错文案给出该节应有的键集。
- **0 字节或只含注释的配置文件按全默认处理（A94）**：以前会抛
  `TypeError: argument of type 'NoneType' is not iterable`（该文案随解释器版本而变），
  CLI 上表现为「错误: …」+ `exit 1`；现在它与「路径不存在」同档 —— 拿到出厂默认、`exit 0`。
- 名单**从 `AppConfig` 的字段类型推导**，不是手抄的清单：它定义上就等于
  `load_config` 实际读走的那份键集（`tests/unit/test_config_validator.py::TestUnreadKeyWarnings`
  逐节钉住这个等式）。新增配置节、新增字段都会自动进入判据。
- `app` 与 `models` 两节豁免：前者是不被任何代码读取的历史元信息，后者的键是**模型名**。
  但 `models.<名字>` 的**子项**照判（`temperatur` 这类拼错一样出声）。
- 顶层写 `default_model: xxx` 也会被报出来：默认模型的唯一来源是 `models.default`。
- 想不到的名字不硬猜：只有存在高度相近项时才附「是否想写 X？」。
- **CLI 的退出码只有三种形状**（L55 起，唯一出口是 `augmentor/cli/verdict.py:verdict_exit`）：
  `0` = 判决通过或这条命令压根不判负；`1` = 判决未通过，**或** handler 抛异常被 `main()`
  翻译成 `错误: …` + `1`（⇒ 同样是 1，靠 stderr 有无「错误:」分「判负」与「崩溃」）；
  `2` = argparse 用法错误。本端点响应的判决位（`is_valid` / `passed` / `overall_passed`）在
  CLI 侧的对应物：校验形命令判负即 `1`，报告形命令（`quality-report` / `audit` /
  `check-leakage` / `doctor` / `auto-test` / `migrate`）**默认恒 0**，加 `--gate` 才判负
  ——「有问题时照样把报告出完」是 L53 定的口径，`--gate` 是把报告变成门禁的显式一档。
  哪些命令接了这套口径由 `tests/integration/test_cli_verdict_wiring.py` 从 **parser 声明与
  handler 源码双向推导**对账（声明 `--gate` 的命令 == handler 里真传 `enforce=args.gate` 的；
  源码出现 `verdict_exit(` == 子命令 help 里宣称「退出码」的），不抄清单。
- **编码不参与退出码**（L56 起）：CLI 入口把 stdout / stderr 的错误处理器从 `strict` 换成
  `replace`，产品侧文本读写一律显式带 `encoding=` ⇒ 用户数据里含 emoji（或任何当前 locale
  编不出的字符）时命令照样跑完，那个字符落成一个 `?`，JSON 输出仍可被 `json.loads` 解析。
  上面「`1` = 崩溃」那一支从此只对应真异常，不再对应编码事故；能被编码的字符逐字节不变。
  守卫：`tests/integration/test_cli_stdio_encoding.py`（真起子进程，`PYTHONIOENCODING=gbk`）
  与 `tests/unit/test_no_locale_text_io.py`（AST 扫产品包，禁止新增不带编码的文本 I/O）。

### 跨源（CORS）

出厂默认**不放行任何跨源**：`web.cors_origins: []` + `web.cors_credentials: false`，
即服务端不发任何 `access-control-*` 响应头。随包 Web UI 与后端同源（前端 axios 的
`baseURL` 是相对路径 `/api`，vite 开发模式走 proxy），curl / Python SDK 等非浏览器客户端
不受 CORS 约束，因此这两项默认对既有部署零影响。

需要让**别的站点上的浏览器页面**调本 API 时，显式列出来源：

```yaml
web:
  # 必须是序列：写成标量（`cors_origins: https://...`）在 starlette 下走子串匹配，
  # 会放行同名前缀的其它主机，因此 3.x 在加载时直接拒（L51 / A80）
  cors_origins: ["https://your-frontend.example"]
  # 只有确实要带 cookie / Authorization 跨源时才打开，且 origins 必须是精确白名单
  cors_credentials: true
```

两点实测（starlette 1.6.0 与 1.2.1 形状一致）：`cors_origins: ["*"]` 配
`cors_credentials: true` **不是**「不开放」，上游会把请求方的 Origin 原样回显并附
`access-control-allow-credentials: true`（预检一并 200）⇒ 任意网站都能带用户凭据读本
API；而 `cors_credentials: true` 即使来源不在白名单里，响应里也会出现
`access-control-allow-credentials: true`（没有 `allow-origin`，因此仍读不到）。本仓 API
不用 cookie，鉴权是 `X-API-Key` 头，所以带凭据这一项对合法用法没有收益。

> 3.x 的**破坏性默认变更**：旧出厂默认是 `["*"]` + `true`。此前即使在 `config.yaml` 里
> 写了收紧值也不生效（`load_config` 的映射表缺这两个键，见 `ARCHITECTURE.md` §3.25），
> 升级到 3.x 后同样的写法会真的生效。

### 错误码

| 状态码 | 含义 | 触发场景 |
|--------|------|----------|
| 200 | 成功 | — |
| 400 | 请求参数错误 | 索引越界、扩展名不支持、`chunk_overlap >= chunk_size` |
| 401 | 鉴权失败 | 服务端已设置 `AUGMENTOR_API_KEY`，而写/删请求未携带或携带了错误的 `X-API-Key` |
| 403 | 路径超出白名单 | 见上「路径白名单」（`detail` 为「路径超出允许的数据目录范围」，与 401 的鉴权失败可区分） |
| 404 | 资源不存在 | 文件不存在、版本不存在 |
| 422 | 请求体校验失败 | 缺少必填字段、字段类型错误（由 FastAPI 自动返回） |
| 500 | 服务端错误 | 模型调用失败、依赖缺失、磁盘写入失败 |
| — | **配置越界不走 HTTP** | `web` / `augmentation` 两节的取值越界在 `load_config` 即抛 `DataValidationError`：启动期表现为进程不起，运行期表现为「回退出厂默认 + warning」（见上「路径白名单」）—— 访问控制项宁可退回最小范围也不静默放宽 |

错误响应格式统一为：

```json
{"detail": "文件不存在"}
```

### 慢请求

中间件会记录每个请求的处理耗时。超过 **2.0 秒**会写入 WARNING 日志，便于定位性能瓶颈。

### 响应契约

**每个端点都声明了 `response_model`**（共 70 个），因此 `/openapi.json` 里不存在
「无 schema 的 200 响应」。这条由 `tests/integration/test_api_openapi_contract.py`
双向守门：既要每个端点都声明契约，也要**每个已声明契约的端点都真实存在**
（防止文档里留着一个早就删掉的端点）。

需要精确到字段的契约时，以 `/openapi.json` 为准 —— 本文档只对下面第 1~7 节
的端点给出人读说明，其余端点的字段请查 schema。**不要从本文档手抄字段名**：
它会漂移，而 schema 由代码生成。

---

## 端点总览

按 OpenAPI tag 分组，共 **70** 个端点。

### audit（1）

| 方法 | 路径 | 说明 |
|------|------|------|
| `POST` | `/api/audit` | 数据集就绪审计 |

### augment（3）

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/api/augment/checkpoints` | 列出断点 |
| `GET` | `/api/augment/progress` | 获取增强进度 |
| `POST` | `/api/augment/start` | 启动增强任务 |

### config（3）

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/api/config` | 获取配置 |
| `POST` | `/api/config` | 更新配置 |
| `GET` | `/api/models` | 列出可用模型 |

### data（8）

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/api/analyze/{filename}` | 分析数据集 |
| `DELETE` | `/api/data/delete/{filename}` | 删除单条数据 |
| `GET` | `/api/data/list` | 列出数据文件 |
| `GET` | `/api/data/load/{filename}` | 分页加载数据 |
| `PUT` | `/api/data/update/{filename}` | 更新单条数据 |
| `POST` | `/api/data/upload` | 上传数据文件 |
| `GET` | `/api/demo/data` | 内置演示数据集（**裸数组**） |
| `GET` | `/api/visualize/{filename}` | 生成可视化图表 |

### dataset（14）

| 方法 | 路径 | 说明 |
|------|------|------|
| `POST` | `/api/dataset/aggregate` | 多源聚合 |
| `POST` | `/api/dataset/auto-config` | 自动配置推荐 |
| `POST` | `/api/dataset/compare` | 数据集对比 |
| `POST` | `/api/dataset/convert` | 格式转换 |
| `POST` | `/api/dataset/evaluate` | 生成文本指标评估（BLEU / ROUGE-L / 相似度） |
| `POST` | `/api/dataset/features` | 字段特征检测 |
| `POST` | `/api/dataset/impact` | 增强前后影响评估（四项增益） |
| `POST` | `/api/dataset/merge` | 合并数据集 |
| `POST` | `/api/dataset/rag` | 转换为 RAG 格式 |
| `POST` | `/api/dataset/sample` | 数据集采样（`method="stratified"` 时按 `stratify_key` 分组） |
| `POST` | `/api/dataset/search` | 数据集搜索 |
| `POST` | `/api/dataset/split` | 分割数据集（`stratify` 分层 / `stratify_key` 选字段 / `shuffle` 控段内次序） |
| `POST` | `/api/dataset/stats` | 数据集统计 |
| `POST` | `/api/dataset/validate` | 数据集验证 |

> `dataset` 分组的约定：**只读分析类**（stats / validate / search / compare /
> features / auto-config / impact / evaluate）内联返回完整结果；**写盘变换类**（convert / merge /
> sample / split / aggregate / rag）必须给 `output_path`，响应只回传「写到哪、
> 写了多少」，不回传数据本身。
>
> `impact` / `evaluate` 的口径细节（哪一侧空算 400、哪一侧空算结论，字段名拼错为什么
> 必须报错而不是给一份「全空」统计）写在 `api/routes/dataset_tools.py` 对应函数的
> 文档串里，并由 `tests/integration/test_api_dataset_system_tools.py` 逐条钉住。
>
> `sample` / `split` 的分层旋钮自 L42 起在请求体上可达：`split` 收 `stratify`（默认
> `false`）、`stratify_key`（默认 `"instruction"`）、`shuffle`（默认 `true`），`sample` 收
> `stratify_key`（默认 `"instruction"`，只在 `method="stratified"` 生效）。三个新字段全省略
> 与 L42 之前的请求交出同一条三段产物，**响应形态一字未改** —— `stratify_distribution` 不回传：真实
> 6,902 条按 `instruction` 分层时它有 6,531 个键、紧凑 JSON 404,362 字节，是四计数响应的
> 5,119 倍，也违反上面那条约定。空 `stratify_key` 一律 400，不静默退化成随机或不分层。
> `shuffle` 在两条支路上的口径**不同**（分层支路只回排段内顺序、默认支路连成员一起改且
> `seed` 空转），细节见 `SplitRequest` 文档串与 `docs/ARCHITECTURE.md` §3.17。

### export（4）

| 方法 | 路径 | 说明 |
|------|------|------|
| `POST` | `/api/data/export` | 导出数据 |
| `POST` | `/api/export/batch` | 批量导出多个数据集 |
| `GET` | `/api/export/formats` | 列出支持的导出格式 |
| `POST` | `/api/export/preview` | 预览导出格式转换结果 |

### leakage（1）

| 方法 | 路径 | 说明 |
|------|------|------|
| `POST` | `/api/leakage/check` | 训练/测试集泄漏检测 |

### multimodal（3）

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/api/multimodal/formats` | 列出支持的多模态格式 |
| `POST` | `/api/multimodal/process` | 处理单条多模态数据 |
| `POST` | `/api/multimodal/scan` | 扫描目录并处理多模态文件 |

### privacy（2）

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/api/privacy/patterns` | PII 模式清单 |
| `POST` | `/api/privacy/sanitize` | PII 脱敏 |

### quality（8）

| 方法 | 路径 | 说明 |
|------|------|------|
| `POST` | `/api/quality/annotate` | 自动标注 |
| `POST` | `/api/quality/benchmark` | 运行数据质量基准 |
| `POST` | `/api/quality/clean` | 数据清洗 |
| `POST` | `/api/quality/dedup` | 智能去重 |
| `POST` | `/api/quality/evaluate` | 质量评分 |
| `POST` | `/api/quality/health-gate` | 数据集健康度评分 + 质量门禁 |
| `POST` | `/api/quality/outliers` | 长度离群点检测 |
| `POST` | `/api/quality/profiling` | 数据集画像 |
| `POST` | `/api/quality/report` | 生成质量报告 |

### status（2）

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/api/health` | 健康检查（轻量存活，给容器探针） |
| `GET` | `/api/status` | 服务综合状态（含可选依赖诊断） |

> `health` 刻意只报存活，不含可能失败的字段；`status` 会真的构造管道并检查
> 可选依赖，因此能回答「当前环境缺什么、哪些功能会降级」，也可能更慢或失败。

### system（13）

| 方法 | 路径 | 说明 |
|------|------|------|
| `POST` | `/api/system/auto-test` | 运行数据集自动化测试 |
| `GET` | `/api/system/backups` | 列出备份 |
| `POST` | `/api/system/backups` | 创建备份 |
| `DELETE` | `/api/system/backups/{backup_id}` | 删除备份 |
| `POST` | `/api/system/backups/{backup_id}/restore` | 恢复备份 |
| `GET` | `/api/system/dependencies` | 可选依赖诊断 |
| `GET` | `/api/system/dependency/datasets` | 已登记数据集列表 |
| `POST` | `/api/system/dependency/datasets` | 登记数据集 |
| `GET` | `/api/system/dependency/graph` | 数据集依赖图与校验问题 |
| `POST` | `/api/system/migrate` | 迁移数据集结构 |
| `POST` | `/api/system/monitor` | 质量监控快照 |
| `POST` | `/api/system/stream` | 流式处理数据集 |
| `POST` | `/api/system/validate-config` | 校验配置文件 |

### version（8）

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/api/versions` | 列出所有版本 |
| `POST` | `/api/versions/create` | 创建新版本 |
| `POST` | `/api/versions/diff` | 对比两个版本 |
| `GET` | `/api/versions/history` | 获取版本操作历史 |
| `GET` | `/api/versions/{version_id}` | 获取版本信息 |
| `DELETE` | `/api/versions/{version_id}` | 删除版本 |
| `GET` | `/api/versions/{version_id}/data` | 获取版本数据 |
| `POST` | `/api/versions/{version_id}/rollback` | 回滚到指定版本 |

---

## 1. 健康检查

### `GET /api/health`

返回服务状态。

```json
{"status": "ok", "version": "3.0.0"}
```

响应模型是 `HealthResponse`，两个字段都在（实测 `TestClient` 读到
`{'status': 'ok', 'version': '3.0.0'}`）。`version` 就是包的 `__version__`，会随发布
变化，所以容器探针与前端启动检查只该依赖 `status`；本条示例在 3.0.0 之前少写了
`version` 键（文档滞后于契约，见 `OPTIMIZATION_LOOP.md` 的 A81）。

---

## 2. 配置与模型

### `GET /api/config`

返回当前生效的关键配置。

```json
{
  "default_model": "ernie",
  "augmentation": {"variants_per_seed": 5, "num_threads": 40},
  "quality": {"enabled": true, "threshold": 0.6},
  "dedup": {"enabled": true, "threshold": 0.9},
  "export": {"default_format": "jsonl", "formats": ["jsonl", "llama_factory", "alpaca", "sharegpt", "chatml"]},
  "vector": {"enabled": false, "backend": "faiss"},
  "rag": {"enabled": false, "default_format": "llamaindex"},
  "multimodal": {"enabled": false}
}
```

### `GET /api/models`

列出配置文件中定义的模型后端。

```json
{"models": ["ernie", "openai", "ollama", "claude", "gemini"], "default": "ernie"}
```

---

## 3. 数据管理

### `GET /api/data/list`

扫描**路径白名单**各根目录下以 `train_data` 开头的 JSON 文件（出厂默认即 `data/`）。
`name` 是文件名，`path` 是服务端拼接出来的完整路径 —— 两者都可以直接喂给
`/api/data/load/{filename}`。

```json
{
  "files": [
    {"name": "train_data.json", "path": "/app/data/train_data.json", "size": 123456}
  ]
}
```

### `GET /api/data/load/{filename}`

分页加载数据，支持关键词搜索。

| 参数 | 位置 | 类型 | 默认 | 说明 |
|------|------|------|------|------|
| `filename` | path | string | — | 数据文件路径 |
| `page` | query | int | `1` | 页码，从 1 开始 |
| `page_size` | query | int | `20` | 每页条数 |
| `search` | query | string | `""` | 在 `instruction` 与 `output` 中做不区分大小写的子串匹配 |

```json
{
  "total": 200,
  "page": 1,
  "page_size": 20,
  "items": [{"instruction": "如何申请入住？", "input": "", "output": "通过 App 提交申请。"}]
}
```

`404` 文件不存在；`500` 读取或解析失败。

### `PUT /api/data/update/{filename}`

替换指定索引的单条数据。

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `filename` | path | string | 数据文件路径 |
| `index` | query | int | 待替换的索引 |
| body | body | object | 新的数据条目（任意 JSON 对象） |

```json
{"success": true}
```

`400` 索引越界。

### `DELETE /api/data/delete/{filename}`

删除指定索引的数据。参数同 `PUT`。返回 `{"success": true}`，`400` 索引越界。

### `POST /api/data/upload`

上传 JSON 文件（`multipart/form-data`）。

| 字段 | 类型 | 说明 |
|------|------|------|
| `file` | file | JSON 文件，须为 UTF-8 编码的 JSON 数组 |

文件以 `file.filename` 为名保存到服务端当前目录。

```json
{"success": true, "path": "train_data_uploaded.json", "count": 120}
```

---

## 4. 数据增强

### `POST /api/augment/start`

以**后台任务**方式启动增强，立即返回。进度通过 `GET /api/augment/progress` 轮询。

```json
{
  "input_file": "train_data.json",
  "output_file": "train_data_augmented.json",
  "use_quality": true,
  "use_dedup": true,
  "use_checkpoint": true
}
```

```json
{"success": true, "message": "增强任务已启动"}
```

### `GET /api/augment/progress`

返回当前断点进度。无进行中的任务时返回 `{"status": "no_checkpoint"}`。

```json
{
  "task_id": "augment_9f2c1ab34d5e",
  "total_items": 200,
  "processed_items": 120,
  "failed_items": 2,
  "progress": 0.6,
  "progress_percent": "60.0%",
  "elapsed_time": "2.5分钟",
  "remaining_time": "1.7分钟",
  "start_time": "2026-09-16T17:00:00",
  "last_update": "2026-09-16T17:02:30",
  "avg_quality_score": 0.78
}
```

### `GET /api/augment/checkpoints`

列出已有断点的任务 ID。

```json
{"checkpoints": ["augment_9f2c1ab34d5e"]}
```

---

## 5. 质量评估

以下端点均以 `POST` + JSON body 调用。`QualityRequest` 结构：

```json
{"input_file": "train_data.json", "threshold": 0.6}
```

`DedupRequest` 结构相同，但 `threshold` 默认 `0.9`。

### `POST /api/quality/evaluate`

计算质量评分统计。

```json
{
  "total_samples": 200,
  "passed_samples": 156,
  "filtered_samples": 44,
  "pass_rate": 0.78,
  "avg_score": 0.72,
  "threshold": 0.6
}
```

### `POST /api/quality/dedup`

基于向量相似度去重，返回保留的索引。

```json
{
  "original_count": 200,
  "deduplicated_count": 181,
  "removed_count": 19,
  "duplicate_groups": 12,
  "kept_indices": [0, 1, 2, 4]
}
```

### `POST /api/quality/report`

生成完整质量报告（评分分布、过滤统计、改进建议）。

```json
{
  "total_samples": 200,
  "passed_samples": 156,
  "filtered_samples": 44,
  "pass_rate": 0.78,
  "score_distribution": {"0.0-0.2": 3, "0.2-0.4": 8, "0.4-0.6": 33, "0.6-0.8": 96, "0.8-1.0": 60},
  "filter_statistics": {},
  "metric_summary": {},
  "dedup_summary": {},
  "improvement_suggestions": ["语义相似度均值偏低（0.65），建议调整增强提示词以保持原意"],
  "charts": {}
}
```

`improvement_suggestions` 的判定规则：

| 条件 | 建议 |
|------|------|
| 通过率 < 0.3 | 阈值过严，建议放宽 |
| 通过率 > 0.95 | 阈值过松，建议提高 |
| 语义相似度均值 < 0.7 | 调整提示词以保持原意 |
| 回答相关性均值 < 0.7 | 提升回答与问题的相关性 |
| 多样性均值 < 0.3 | 增加变体多样性 |
| 去重移除率 > 0.3 | 重复度过高，建议检查种子数据 |
| 均不触发 | `"各项指标均在合理区间，数据质量良好"` |

### `POST /api/quality/clean`

数据清洗（去噪、格式标准化、语言检测）。

```json
{
  "original_count": 200,
  "cleaned_count": 198,
  "dropped_count": 2,
  "changed_count": 87,
  "language_distribution": {"zh": 195, "en": 2, "mixed": 1},
  "issues": []
}
```

### `POST /api/quality/annotate`

自动标注（实体识别、意图分类、情感分析）。

```json
{
  "total": 200,
  "entities": {"phone": 12, "email": 8, "money": 45},
  "intents": {"how_to": 88, "what_is": 42, "price": 30},
  "sentiments": {"positive": 90, "neutral": 100, "negative": 10}
}
```

### `POST /api/quality/benchmark`

运行数据质量基准，返回标准化指标。

```json
{
  "pass_rate": 0.78,
  "avg_total_score": 0.72,
  "diversity": 0.54,
  "duplication_rate": 0.09
}
```

### `POST /api/quality/health-gate`

数据集健康度评分 + 质量门禁：把「这份数据能不能进下游训练」压成一次只读判定。

```json
{
  "input_file": "train_data.json",
  "text_field": "instruction",
  "weights": [0.4, 0.3, 0.2, 0.1],
  "pass_rate": 0.78,
  "pass_rate_min": 0.6,
  "duplicate_rate_max": 0.3,
  "completeness_min": 0.8,
  "block_on_warning": false
}
```

`weights` 是健康分四项（完整性 / 多样性 / 质量分布均衡性 / 覆盖率）的权重，必须 4 个
且之和为 1.0，省略则各 0.25；非法值在**读文件之前**就返回 400。`text_field` 决定重复率
按哪个字段算（默认 `instruction`）。

```json
{
  "health": {
    "health_score": 0.74,
    "level": "healthy",
    "metrics": {"completeness": 1.0, "diversity": 0.667, "quality_balance": 0.222, "coverage": 1.0},
    "weights": [0.4, 0.3, 0.2, 0.1],
    "total_samples": 3
  },
  "gate": {
    "verdict": "failed",
    "passed": false,
    "failed_rules": ["duplicate_rate"],
    "warned_rules": [],
    "metrics": {"completeness": 1.0, "diversity": 0.667, "quality_balance": 0.222,
                 "coverage": 1.0, "duplicate_rate": 0.667, "pass_rate": 0.78}
  },
  "skipped_rules": []
}
```

门禁规则为 `pass_rate >= pass_rate_min`（error）、`duplicate_rate <= duplicate_rate_max`
（error）、`completeness >= completeness_min`（warning）。`verdict` 取
`passed` / `warned` / `failed`，`passed` 字段对前两者为 `true`；`block_on_warning` 为
`true` 时 warning 也判负。

**`pass_rate` 必须由调用方自带**（即 `POST /api/quality/evaluate` 的返回值）。数据集里
不含原始种子问题，本端点无法自行算出通过率；而规则对**缺失的指标**判为不满足，所以省略
`pass_rate` 时该规则会被摘掉并出现在 `skipped_rules` 里 —— 端点不会把「没测这一项」表达成
「这一项不及格」。空数据集（`[]`）返回 400 而不是放行。

CLI 侧等价命令：`augmentor health-gate --input data/train_data.json --pass-rate 0.78`，
门禁判负时退出码为 1（warning 默认不阻断，加 `--block-on-warning` 才阻断）。

---

## 6. 数据导出

### `GET /api/export/formats`

列出支持的导出格式。

```json
{"formats": ["jsonl", "llama_factory", "alpaca", "sharegpt", "chatml"]}
```

### `POST /api/data/export`

导出单个数据集。

```json
{"input_file": "train_data.json", "output_dir": "exports", "formats": ["jsonl", "alpaca"]}
```

`formats` 为 `null` 或省略时导出全部格式。

```json
{
  "success": true,
  "files": {"jsonl": "exports/train_data.jsonl", "alpaca": "exports/train_data_alpaca.json"}
}
```

### `POST /api/export/batch`

一次导出多个数据集 × 多个格式。

```json
{
  "datasets": {"main": "train_data_final.json", "extra": "train_data_extra.json"},
  "output_dir": "exports",
  "formats": ["jsonl", "sharegpt"]
}
```

```json
{
  "success": true,
  "results": {
    "main": {"jsonl": "exports/main.jsonl", "sharegpt": "exports/main_sharegpt.json"},
    "extra": {"jsonl": "exports/extra.jsonl", "sharegpt": "exports/extra_sharegpt.json"}
  }
}
```

格式名非法时在开启线程前即校验并返回 `500`，不会产生部分写入的文件。

### `POST /api/export/preview`

预览格式转换结果（不写磁盘）。

```json
{"input_file": "train_data.json", "format": "sharegpt", "size": 5}
```

```json
{
  "format": "sharegpt",
  "total": 200,
  "preview_count": 5,
  "items": [],
  "warnings": ["数据集为空"]
}
```

`warnings` 会在以下情况出现：数据集为空、必填字段为空、目标格式需要 `history` 但数据未提供。

---

## 7. 版本管理

### `GET /api/versions`

```json
{
  "versions": [
    {"version_id": "20260916_170000_123456", "label": "增强数据", "description": "", "created_at": "2026-09-16T17:00:00", "item_count": 28000}
  ]
}
```

版本 ID 形如 `%Y%m%d_%H%M%S_%f`，精确到微秒并做存在性去重，避免同秒创建时互相覆盖。

### `POST /api/versions/create?filename=<path>`

从数据文件创建快照。

```json
{"label": "v1 基线", "description": "增强前的原始数据"}
```

```json
{"success": true, "version_id": "20260916_170000_123456"}
```

### `POST /api/versions/diff`

对比两个版本。

```json
{"version1": "20260916_170000_123456", "version2": "20260916_180000_654321"}
```

```json
{"version1": "...", "version2": "...", "added_count": 120, "removed_count": 3, "modified_count": 8}
```

### `GET /api/versions/history`

版本操作历史（create / delete / rollback），按时间倒序。

| 参数 | 类型 | 默认 | 说明 |
|------|------|------|------|
| `limit` | query | `20` | 返回条数上限 |

```json
{"history": [{"action": "create", "version_id": "20260916_170000_123456", "detail": "增强数据"}]}
```

### `GET /api/versions/{version_id}`

```json
{
  "version_id": "...", "label": "...", "description": "...",
  "created_at": "...", "item_count": 28000, "metadata": {}
}
```

### `GET /api/versions/{version_id}/data`

```json
{"items": [{"instruction": "...", "output": "..."}]}
```

### `POST /api/versions/{version_id}/rollback`

```json
{"success": true}
```

### `DELETE /api/versions/{version_id}`

```json
{"success": true}
```

> **路由顺序注意**：`/api/versions/history` 必须定义在 `/api/versions/{version_id}` 之前，
> 否则会被路径参数吞掉。

---

## 8. 多模态处理

### `GET /api/multimodal/formats`

```json
{
  "image_extensions": [".jpg", ".jpeg", ".png", ".bmp", ".webp"],
  "audio_extensions": [".wav", ".mp3", ".flac", ".ogg", ".m4a"]
}
```

### `POST /api/multimodal/process`

融合单条多模态记录。`image` / `audio` 为可选的文件路径。

```json
{"text": "客户上传的报修图片", "image": "samples/leak.png", "audio": null}
```

```json
{
  "text": "客户上传的报修图片",
  "image": {"path": "samples/leak.png", "format": "PNG", "width": 800, "height": 600, "size": 20480},
  "audio": null,
  "modalities": ["text", "image"]
}
```

### `POST /api/multimodal/scan`

扫描目录，把同名 stem 的图片与音频融合为一条记录。

```json
{"directory": "samples"}
```

```json
{
  "total_records": 12,
  "modality_distribution": {"text+image": 7, "text+audio": 3, "text+image+audio": 2},
  "records": []
}
```

`400` 目录不存在或不支持的文件扩展名。

---

## 9. 分析与可视化

### `GET /api/analyze/{filename}`

返回覆盖度分析、统计信息与去重报告。

```json
{
  "coverage_analysis": {"total_items": 200, "question_type_distribution": {"how": 0.4}},
  "statistics": {"avg_length": 18.5, "top_words": {}},
  "dedup_report": {"removal_rate": 0.09}
}
```

### `GET /api/visualize/{filename}`

生成图表并返回文件路径。依赖缺失时对应项返回空字符串，会被过滤掉。

```json
{"charts": {"wordcloud": "visualizations/wordcloud.png", "length_distribution": "visualizations/length_distribution.png"}}
```

---

## 10. 调用示例

```bash
# 健康检查
curl http://localhost:8000/api/health

# 启动增强
curl -X POST http://localhost:8000/api/augment/start \
  -H "Content-Type: application/json" \
  -d '{"input_file":"train_data.json","output_file":"out.json"}'

# 轮询进度
curl http://localhost:8000/api/augment/progress

# 质量评估
curl -X POST http://localhost:8000/api/quality/evaluate \
  -H "Content-Type: application/json" \
  -d '{"input_file":"train_data.json","threshold":0.6}'

# 上传文件
curl -X POST http://localhost:8000/api/data/upload \
  -F "file=@train_data_new.json"
```

Python：

```python
import requests

base = "http://localhost:8000"

print(requests.get(f"{base}/api/health").json())

resp = requests.post(
    f"{base}/api/quality/report",
    json={"input_file": "train_data.json", "threshold": 0.6},
    timeout=300,
)
report = resp.json()
print(report["pass_rate"], report["improvement_suggestions"])
```
