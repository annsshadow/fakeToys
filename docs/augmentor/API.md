# API 文档

REST API 由 FastAPI 提供，监听地址与静态目录取自配置的 `web` 节（出厂默认 `0.0.0.0:8000`，即 `http://localhost:8000`）。交互式文档见 `/docs`（Swagger UI）。

```bash
python api/main.py             # 读配置：web.host / web.port / web.static_dir
```

也可以照旧用 uvicorn 起（`uvicorn api.main:app --host 127.0.0.1 --port 8000`），此时**命令行参数优先于配置**。`python api/main.py` 这条入口不传字面量：`web.host: 127.0.0.1` 就真的只绑回环。

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

`GET /api/data/list` 的扫描范围与白名单同源（默认档与 `all_files=true` 都一样），所以它列出的文件一定都读得到。

**目录类参数的缺省值也在白名单内**。`/api/system/dependency/*` 的 `registry_path`
与 `/api/system/backups*` 的 `backup_dir` 都是可选参数；留空时分别使用白名单首个根目录下
的 `.dependency_registry` 与 `.backups`（出厂默认即 `data/.dependency_registry` 与
`data/.backups`），因此 CLI 用 `--registry-path data/.dependency_registry` 就能与 API
共用同一份登记。显式传入时照旧校验，越界 403；显式传空串（`?backup_dir=`）是非法入参
→ 400，不会被静默换成缺省值。**默认值有三份且互不相同**：CLI `--registry-path` 与 SDK `DependencyManager` 形参的默认都是**相对工作目录**的 `.dependency_registry`，端点缺省走白名单首根下那一份 ⇒ 不显式带 `--registry-path` 时，CLI 登记的数据集页面看不见，反之亦然（A308）。

**服务自身的配置文件不走数据白名单**。`POST /api/system/validate-config` 的 `path`
可选：留空时校验服务端自己在用的那份配置（`AUGMENTOR_CONFIG_PATH`，未设置则是工作目录下
的 `config.yaml`）；显式传入时仍按数据白名单校验。`POST /api/config` 保存的也是同一个
路径，二者不会各写一遍字面量。

**`POST /api/config` 的越界值是 400，且整批不落**（3.x，L73 起）。改前该端点用裸
`setattr` 写字段，而运行时判据住在各配置节的 `__post_init__` 里、`setattr` 不会再触发它
⇒ 坏值当次回 200、还能被原样写进 `config.yaml`，症状是「服务跑得好好的，重启后起不来」。
现在整批写完跑一次该节判据（L73 起是「每写一条判一次」，**L82 改成批次终态判一次**，
理由见下面 `rag` 那一跨键界；界仍然只住 `augmentor/config.py` 一处），任一条越界即
整批退回原值并回 **400**（`detail` 点名是哪个键、当前值多少），内存与磁盘都停在改前状态。
未知子键的既有契约不变：仍然 200，键名列在响应的 `ignored_keys` 里。
`quality` / `dedup` 两节自 L76 起**也有**判据了：`enabled` 走新增的
`validation.require_bool`，`threshold` 走 `require_ratio`，且上界与静态规格共用
`QUALITY_THRESHOLD_RANGE` / `DEDUP_THRESHOLD_RANGE` 两个常数（界只住 `config.py` 一处），
所以这两节的坏值现在同样回 400。`enabled` 只认**真布尔**：YAML 里裸 `no` / `false` / `off`
（以及 `yes` / `true` / `on`）会被解析成布尔值，是合法写法；**一旦加引号**就变成字符串，
加载与 `POST /api/config` 两侧都拒（15 种拼法的实测分类见账本 L76）。
**L82 起 A118 那六节的账全清**：`export`（2 键）/ `vector`（5 键）/ `rag`（4 键）/
`multimodal`（3 键）四节共 14 键接上判据，于是 `POST /api/config` 可写的**七节全部有**
运行时判据，四面（SDK 直构 / `load_config` / 本端点 / `validate-config`）同判。新增两条
族员：`validation.require_choice`（管「从一张封闭清单里选一个」，清单**推导**自各权威模块
—— `EXPORT_FORMATS` 取 `ExportFormat`、`RAG_FORMATS` 取 `rag.SUPPORTED_FORMATS`、
`VECTOR_BACKENDS` 取 `vector.SUPPORTED_BACKENDS`）与 `validation.require_chunk_window`
（管一对互相约束的键）。两条对用户的可见差别：
- `vector.backend: milvus`、`export.default_format: xls` 这类从前要等到流水线跑到那一步
  才炸（或者根本不炸），现在四面全部拒；`Exporter` 那句裸 `ValueError` 走 API 时是 500，
  配置面先判 ⇒ 本端点回 400。
- `rag.chunk_size` / `chunk_overlap` 是**跨键**界（`chunk_overlap < chunk_size`，且两侧
  分别 `>= 1` / `>= 0`），所以「单看合法」的两个值配上现存的另一侧仍可能 400：出厂
  overlap 是 64，只写 `{"rag": {"chunk_size": 1}}` 会被拒；把两半放在同一批里
  （`{"chunk_size": 1, "chunk_overlap": 0}`）就 200。从前这类批次即使两半一起改也写不进去
  —— 判据看的是每写一条之后的**中间态**。

**空串与纯空白是两档，两档都 400（L83 / A142）**。字符串型配置键的判据从两刀变成三刀：
类型 → 空串 → 纯空白。改前第二刀只判 `if not value:`，于是 `web.host: "   "` 这类
**看起来有值**的写法四面全盲（构造、`load_config`、`validate-config`、本端点都不出声），
落到 `Path()` 就得到一个名字是若干空格的目录根。三处要点：

- 报错文案把两档分开：空串是 `xxx 不能是空字符串`，纯空白是 `xxx 不能是纯空白字符串，当前
  是 '\xa0\xa0'`（带 `repr`，因为空白字符本身不可见；NBSP 也在纯空白之列 —— 从网页或文档
  里复制 YAML 时最常见的隐形字符）。列表键的每一项吃同一把刀：`data_roots: ["  "]` 400。
- **含空格的合法值照旧通过，且不裁剪**：`web.static_dir: "my docs"` 仍然 200，落盘的值逐字
  不变。判的是「整串都是空白」，不是「值里有空格」；隐式 `.strip()` 等于替客户改配置。
- **唯一例外是 `logging.file`**：空串在这里是**合法值**，意思是「不落文件」（出厂默认就是
  空串），纯空白才是坏值（它会创建一个名叫空格的文件）。所以该键只挂第三刀。

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
- **模型条目的「没写」与「写了没给值」自 L75 起两侧同判**：`type` 是条目里唯一必填键
  （缺键 ⇒ `缺少必填字段: models.<名>.type`，写了留空 ⇒ `字段不能为null: ...`），其余八键
  不写就是由 `ModelConfig` 的字段默认回答 —— 加载器不再持有第二份回落值。加载期报错现在
  带**真实条目名**（如 `models.beta.temperature 必须是不大于 2.0 的比例，当前是 999`），
  而 SDK 直接构造 `ModelConfig` 时仍是 `models.<名字>.…` 占位（那一层拿不到条目名）。
  **一处旧形状作废**：`save_config` 以前会写出 `model: ''`，这样的文件现在加载即拒。
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

### 可写四节的「读者分档」（A138，L184 记档 / L214 接线第一批 / L216 清空 ③ 档）

上面 L82 段落给 `export` / `vector` / `rag` / `multimodal` 四节 14 键全接上了判据（写坏即 400），
但「写得进」不等于「有人读」。这 14 键按**现量**分三档，契约面与实现面的落差就摊开在这里：

| 档 | 键 | 读点 | 语义 |
|----|----|------|------|
| ① 有行为读者（12） | `export.default_format` | `augmentor/pipeline.py`（导出走默认格式）、`api/routes/export.py`（preview 默认格式、批量导出构造） | 真控行为 |
| | `export.formats` | `api/routes/export.py`（`/api/data/export`、`/api/export/batch` 未点名格式时按它导出） | 真控行为（L214） |
| | `rag.chunk_size`、`rag.chunk_overlap` | `augmentor/cli/commands/quality.py`（RAG 切块）、`api/routes/dataset_tools.py`（`/api/dataset/rag`） | 真控行为；**本表一直记着它们有读者，缺的是下面那份机器名单**——L216 的分区守卫当场抓出这两键没进 `_TIER_12_ECHO`，于是 `GET /api/config` 也从没回显过它们 |
| | `rag.default_format` | `api/routes/dataset_tools.py`（请求未点名格式时按它切形） | 真控行为（L214） |
| | `multimodal.image_extensions`、`multimodal.audio_extensions` | `api/routes/multimodal.py`（`scan` 的识别面、`formats` 的清单）；`process` **不读**——单条融合按文件内容解析，扩展名门只住在扫描侧（L214 量过之后撤掉了那里的字段） | 真控行为（L214） |
| | `vector.enabled` | `augmentor/pipeline.py`（增强结束时自动建库的闸，同步与异步两个入口同一道判；与既有 `auto_snapshot` 串联） | 真控行为（L216） |
| | `vector.backend` | `augmentor/semantic.py`（`create_vector_db` 的分派实参；未知值由工厂拒 400；没有 `persist`/`load` 面的后端由 `EXPLICIT_PERSISTENCE_BACKENDS` 在构造前拒 400） | 真控行为（L216） |
| | `vector.dimension` | `augmentor/semantic.py`（哈希嵌入宽度 ⇒ 相似度分值与排名跟着变） | 真控行为（L216） |
| | `vector.storage_dir` | `augmentor/semantic.py`（持久化落点，空值时 `persist()` 抛 `VectorError`）、`api/routes/vector.py`（`vector-status` 唯一去读的那份索引） | 真控行为（L216） |
| | `vector.collection` | `augmentor/semantic.py`（落盘文件名与向量 ID 前缀） | 真控行为（L216） |
| ② 仅回显（2） | `rag.enabled`、`multimodal.enabled` | `GET /api/config` 原样回显，**不改任何行为** | 可见但未消费：两只都是出厂 `false` 的启用闸门，接成端点拒绝门等于让随包 API 的一项既有能力当场消失（实测 13 支集成测试全红），要单独拍 |
| ③ 连回显都没有（0） | — | — | **L216 清空**（`vector` 那三只进了 ①）。这一档为空会让「③ 档键不得出现在回显里」退化成循环体为零的永真断言，所以同批把判据换成**分区**：可写四节的每一键必须逐键归入 ①② 或 ③，不重不漏——幽灵键防线从「点名不许出现」升成「新键不许无人归档」 |

**权威口径（L214 定稿）**：① 档那几只键的读取序是**请求参数 &gt; 配置键 &gt; 内置默认**——请求体点名
了 `formats` / `format` / `image_extensions` 就以它为准，没点名才落到配置，配置也没写才用各处理器自己的
内置默认。所以「配置里写了什么」只决定**不点名时**的行为，不缩小契约面承诺的能力。

向量库这节的提醒**在 L216 之后要改口**（旧文里那句「`vector.dimension` 配 768 还是 8 在今天的产物里一字不变」
从今天起是错的，先把它划掉再说清楚）：
- `vector.backend` 回显成 `faiss` 只表示「配置里写了 faiss」，**不等于**引擎此刻就是 faiss——
  `faiss-cpu` 没装时 `FAISSDB` 退化为 numpy 精确检索，那是依赖层事实，权威住在
  `/api/system/dependencies`。旧文里「真正的后端入口是 `vector.build_vector_db` 的 `backend` 形参与
  API 请求体」那半句作废：现在唯一的生产入口是 `augmentor/semantic.py`，它的 `backend` 实参只能来自
  `config.vector.backend`（`test_configured_backend_reaches_the_factory` 钉，喂进去的是非默认值，
  否则「写死默认值」这种缺陷与正确实现读到的是同一个字符串）。
- `backend` 的**落盘面只覆盖有 `persist` / `load` 契约的后端**：`augmentor/vector/chromadb.py` 那侧
  没有这两个方法，而它的构造阶段就会创建目录并注册集合。因此 `augmentor/semantic.py` 的
  `EXPLICIT_PERSISTENCE_BACKENDS` 在**构造之前**把 `chromadb` 拒成 400——闸的位置就是这条承诺的内容：
  写在构造之后，一条 GET 状态查询就能凭空造出目录。现量在
  `tests/integration/test_vector_semantic_consumption_l216.py::TestPersistenceSurfaceIsGated`。
- `vector.dimension` **真的改产物**：它是哈希嵌入的宽度，同一查询在 4 维与 1024 维下的相似度分值
  与排名都不同（`test_dimension_changes_scores_and_ranking` 现量）。旧文那句「维度由所选嵌入
  模型决定」描述的是**没有嵌入模型参与**时的假想接法——本轮用的是零依赖的确定性哈希嵌入，所以宽度
  就是配置值。真模型接法（模型维度与本键冲突怎么收）已立账待拍。
- `vector.storage_dir` / `vector.collection` 决定落盘点与文件名、向量 ID 前缀；`vector.enabled` 决定
  增强结束时建不建库（两个入口同一道判）。

**L216 接的是阶段闸，不是端点拒绝门**：`vector.enabled` 关掉时 `POST /api/dataset/semantic-search` 与
`GET /api/dataset/vector-status` **照常可用**（检索走内存索引，与「结束时自动落盘」是两件事），
所以下面那段「`enabled` 不做成端点拒绝门」的结论对本键同样成立。

**`enabled` 这一族为什么不做成端点拒绝门（L214 实测，并在此订正一句自己写错的话）**：第一版这一段写的是
「`enabled` 的既有语义是管道阶段开关，住在 `AugmentorPipeline`（与 `quality.enabled` / `dedup.enabled` 同族）」
——**那句是没实测就写的，而且是错的**。按 `config.<节>.enabled` 做全仓普查（扣掉 `config.py` 的校验、
`config_validator.py` 的静态规格与 `api/routes/config.py` 的回显），**16 只 `enabled` 在产品代码里零读者**；
管道阶段真正的开关是**调用参数**（`AugmentorPipeline.augment` 的 `use_quality_check` / `use_dedup` 两个形参），
阶段内部从来不看配置。⇒「接 `enabled`」不是给一条既有语义找位置，而是**新建一条消费路径**，
那就必须先量默认值再动：出厂 config 里 `quality` / `dedup` / `versioning` / `visualization` 四节是 `true`
（接上即零行为漂移），其余十二节是 `false`（接上等于把当前始终在跑的行为关掉 = 对外破坏面，要单独拍）。
端点拒绝门那一撤照旧成立，理由不变：`rag.enabled` / `multimodal.enabled` / `vector.enabled` 出厂全是 `false`，
按字面判则随包 API 的这三项既有能力当场消失，13 支集成测试全红。⇒ 四只 true 键的消费点**已在 L215 落地**
（`quality.enabled` / `dedup.enabled` 进同步与异步两个增强入口的默认解析、
`versioning.enabled` 与既有 `auto_snapshot` 串联、`visualization.enabled` 进出图阶段入口），
十二只 false 键立账待拍，端点面全部保持不变。调用参数在 L215 之后仍是最高一档：
`use_quality_check` / `use_dedup` 的默认值从 `True` 换成 `None` 哨兵，`None` 的意思是
「调用方没点，交给配置」，不是「关掉」。
（这一族不是新发现的病：Backlog **B5** 的 AST 尺子早已量出全仓 37 只死键，其中「16 格是各节的启用闸门」
与本节这次 grep 的 16 只 `enabled` 同一读数 —— 本节写它是因为**文档自己曾经把 B5 已登记的事实写成反话**。）

**分档的机器口径**：`tests/integration/test_api_config_extended.py` 的 `TestConfigEchoReaderSplitL184`
钉三件事。① ① /② 档 **14 键必须**回显（防止回显面被顺手摘掉）；② `test_consumed_keys_are_echoed`
反向钉「已有读者的键必须可见」（名单 15 只：可写四节里已有读者的那些 + `enabled` 族已接进管道阶段入口的
四只）；③ L216 新加的 `test_the_two_tiers_partition_the_writable_sections`——可写四节的每一键必须逐键
归入 ①② 或 ③，不重不漏，键名从 `AppConfig` 的字段**实推**而不抄清单。
③ 是 ③ 档清空之后幽灵键防线的新形状（旧那条「③ 档键不得出现在回显里」此时循环体为空、恒真），
并且它**当场抓出一处历史漏计**：`rag.chunk_size` / `rag.chunk_overlap` 一直有读者、本表也一直记着，
却没进过那份机器名单，于是 `GET /api/config` 也从没回显过它们（L216 补上）。
① 档每只键的**行为差**由 `tests/integration/test_api_config_consumption_l214.py` 逐键证明（同一端点、
只换配置、产物必须不同），`enabled` 四只由 `tests/integration/test_pipeline_config_consumption_l215.py`，
`vector` 五只由 `tests/integration/test_vector_semantic_consumption_l216.py`。`GET /api/config` 的**节清单**不手抄：`tests/integration/test_api_openapi_contract.py` 逐键核对，
L215 新增两只只读节时红点就落在它身上，L216 扩 vector 整节时同样落在它身上。

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
| 400 | 请求参数错误 | 索引越界、扩展名不支持、`chunk_overlap >= chunk_size`、**路径名被操作系统判死**（`OSError` 的 `errno` 是 `EINVAL` / `ENAMETOOLONG`：名字里有当前系统禁止的字符或名字过长，L271 / A351。`detail` 用固定文案 `文件路径含当前系统不接受的字符或超出长度上限`，**不含**部署根路径；同族的 NUL 那一形由操作系统抛 `ValueError`，走「原文照回」支，所以它的 `detail` 是英文原文——两形文案不同是已登记的余面，不是笔误）。**判决文案里嵌请求原文**那一格同形会崩（`source_format` 喂一枚孤立代理 ⇒ `detail` 带原文 ⇒ `JSONResponse.render` 抛 `UnicodeEncodeError`）：`api/main.py` 为 `HTTPException` 也兜了一层（L284），默认那支抛了才把编不出的字符换成反斜杠原文重渲染，状态码、响应头与其余字节不动；而成功路径把原文回显进响应体那一格（`POST /api/dataset/search` 的 `query`）**未兜**，实测仍 500，已立 A385 待拍 |
| 401 | 鉴权失败 | 服务端已设置 `AUGMENTOR_API_KEY`，而写/删请求未携带或携带了错误的 `X-API-Key`（前端那份 key 在「配置管理 → 写操作鉴权」卡上存，由请求拦截器随每个请求送出；401 的 `detail` 会被服务层 `apiErrorDetail` 追加一句指路） |
| 403 | 路径超出白名单 | 见上「路径白名单」（`detail` 为「路径超出允许的数据目录范围」，与 401 的鉴权失败可区分） |
| 404 | 资源不存在 | 文件不存在、版本不存在 **409** 不在这张表的行里，它是「备份内容坏了」而不是「资源不在」：备份在、id 也在而校验和对不上，恢复被拒且零写入。备份一族六员的错误档自 **L252** 起逐档声明在 OpenAPI 的 `responses` 里（A330 已收），本表加一整行会平移本文件被账本引用的那一族行号，所以按净 0 同行追加在这里；而「文件不存在」这一档在备份一族里既可能是源数据文件不在场，也可能是 `backup_id` 不在索引里，两种都由 404 出声。 |
| 422 | 请求体校验失败 | 缺少必填字段、字段类型错误（由 FastAPI 自动返回）。载荷里写 JSON 的 `NaN` / `Infinity` / `-Infinity` 字面量时，错误项的 `input` 会回显那个非有限浮点，而 starlette 渲染 JSON 响应钉的是 `allow_nan=False` ⇒ 回执自己渲染不动；本仓在 `api/main.py` 文末兜了一层（L284）：默认处理器抛 `ValueError` 才把非有限浮点换成**同名字符串 token** 重渲染，状态码仍是 422，`type` / `msg` / `ctx` 与其余字节一字未动——**成功路径刻意不脱敏**，否则会掩盖服务端自己算出来的 NaN。同一层还兜 JSON 里解得出来、却编不成 UTF-8 的**孤立代理对**（`"\ud800"`）：渲染抛的是 `UnicodeEncodeError`（它是 `ValueError` 的子类，所以那一支 `except` 天然接得住），编不出的那枚字符换成反斜杠原文（六个可见字符）后重渲染，`query` / `pattern` / `regex_pattern` 三只回显原文的字段实测同形 |
| 500 | 服务端错误 | 模型调用失败、依赖缺失、磁盘写入失败（指**服务端状态**那一种：磁盘满 / 权限缺失 / 只读卷，`errno` 为 `ENOSPC` / `EACCES` / `EROFS`，或测试注入的无 `errno` `OSError`；「这个名字在任何账户下都不可能成立」那一档在上一行是 400，L271 之前它混在这一格里） |
| — | **配置越界不走 HTTP** | `web` / `augmentation` 两节的取值越界在 `load_config` 即抛 `DataValidationError`：启动期表现为进程不起，运行期表现为「回退出厂默认 + warning」（见上「路径白名单」）—— 访问控制项宁可退回最小范围也不静默放宽 |

错误响应格式统一为：

```json
{"detail": "文件不存在"}
```

### 慢请求

中间件会记录每个请求的处理耗时。超过 **2.0 秒**会写入 WARNING 日志，便于定位性能瓶颈。

### 响应契约

**每个端点都声明了 `response_model`**（共 77 个），因此 `/openapi.json` 里不存在
「无 schema 的 200 响应」。这条由 `tests/integration/test_api_openapi_contract.py`
双向守门：既要每个端点都声明契约，也要**每个已声明契约的端点都真实存在**
（防止文档里留着一个早就删掉的端点）。

需要精确到字段的契约时，以 `/openapi.json` 为准 —— 本文档只对下面第 1~7 节
的端点给出人读说明，其余端点的字段请查 schema。**不要从本文档手抄字段名**：
它会漂移，而 schema 由代码生成。

### 易混字段词典

以下字段跨多个端点复现、且**字面直觉常与实际语义不符**，逐条给出人读说明。
字段名以 schema（`api/routes/*.py` 的 `*Response`）为准，这里只解释含义，不复述类型。

| 字段 | 出现处 | 人读说明（陷阱在括号里） |
|------|--------|--------------------------|
| `pass_rate` | quality/evaluate、quality/report、health-gate | 通过阈值的样本占比 = `passed_samples / total_samples`。（**健康门禁 health-gate 判负读的是它、不是 `avg_score`**：想「平均分不达标就让 CI 红」的人要另判 `avg_score`。） |
| `avg_score` | quality/evaluate | 全体样本质量分的算术平均。（与 `pass_rate` 无固定关系：高均分也可能低通过率，反之亦然。） |
| `threshold` | quality/evaluate、benchmark | 本次评分实际生效的阈值（可能来自请求覆盖，不一定等于 `config.yaml` 的 `quality.threshold`）。 |
| `effective_weights` | quality/evaluate | **重归一化后**的权重，不是配置原值。API 拿不到原始种子问题 ⇒ 语义相似度维度不参与评分，其权重被摊到其余维度上；返回它是为了让调用方能自行复算总分。 |
| `semantic_evaluated` | quality/evaluate | 语义相似度维度**是否真的参与了**评分。API 面恒为 `false`（见上一行原因），CLI/SDK 面在有原始问题且装了 `sentence-transformers` 时才 `true`。 |
| `duplicate_groups` | quality/dedup | 重复**组的数量**（一个整数），**不是**分组明细列表。要保留下来的条目下标看 `kept_indices`。 |
| `removed_count` / `dropped_count` | dedup 用前者 / quality/clean 用后者 | 都是「被去掉的条数」，但**不同名是既有契约**：去重器（`removed_count`）与清洗器（`dropped_count`）是两个不同组件，不在 API 层统一。 |
| `is_valid` | dataset/validate | 「数据集整体是否通过校验」。（**空数据集 `[]` 恒为 `true`**——一条问题都没有是因为一条都没看，见账本 A92；上游可能把语料写空却三面全绿，别把它当「数据没问题」。） |
| `overall_passed` | quality/report | 报告级总判决位。报告形端点**默认不把它翻译成退出码**（只出报告不判负，见账本 A91 / §3.28），要门禁语义走 health-gate 或 CLI 的 `--gate`。 |
| `metrics` / `all_metrics` | benchmark | `metrics` 只含**配置里声明**的指标；`all_metrics` 含全部已计算指标。CI 该盯的是前者。 |

---

## 端点总览

按 OpenAPI tag 分组，共 **77** 个端点。

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

### dataset（18）

| 方法 | 路径 | 说明 |
|------|------|------|
| `POST` | `/api/dataset/aggregate` | 多源聚合 |
| `POST` | `/api/dataset/auto-config` | 自动配置推荐 |
| `POST` | `/api/dataset/compare` | 数据集对比 |
| `POST` | `/api/dataset/convert` | 格式转换 |
| `POST` | `/api/dataset/evaluate` | 生成文本指标评估（BLEU / ROUGE-L / 相似度） |
| `POST` | `/api/dataset/features` | 字段特征检测 |
| `POST` | `/api/dataset/impact` | 增强前后影响评估（四项增益） |
| `POST` | `/api/dataset/language-profile` | 语言分布画像（按 `config.multilingual.*` 生效值，不打模型） |
| `POST` | `/api/dataset/merge` | 合并数据集 |
| `POST` | `/api/dataset/rag` | 转换为 RAG 格式 |
| `POST` | `/api/dataset/sample` | 数据集采样（`method="stratified"` 时按 `stratify_key` 分组） |
| `POST` | `/api/dataset/search` | 数据集搜索 |
| `POST` | `/api/dataset/semantic-search` | 语义相似检索（按 `config.vector.*` 生效值） |
| `POST` | `/api/dataset/split` | 分割数据集（`stratify` 分层 / `stratify_key` 选字段 / `shuffle` 控段内次序） |
| `POST` | `/api/dataset/translate` | 批量翻译数据集（受 `multilingual.enabled` 闸控制，挂鉴权） |
| `POST` | `/api/dataset/stats` | 数据集统计 |
| `POST` | `/api/dataset/validate` | 数据集验证 |
| `GET` | `/api/dataset/vector-status` | 向量库状态（是否已建、多少条） |

> `dataset` 分组的约定：**只读分析类**（stats / validate / search / compare /
> features / auto-config / impact / evaluate / semantic-search / vector-status /
> language-profile）内联返回完整结果；**写盘变换类**（convert / merge /
> sample / split / aggregate / rag）必须给输出路径字段 —— `output_file`，只有
> `split` 是 `output_dir`；响应只回传「写到哪、
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
>
> `convert` 的 `target_format` 自 L84 起合法值是 12 个：容器四形 `json` / `jsonl` / `csv` /
> `tsv`、六个行内 schema，加上 Excel 写边的 `excel` / `xlsx`（同一份权威
> `converter.OUTPUT_FORMAT_CHOICES`，CLI 的 `--format` 与它由 `TestConvertTargetFormatSurface`
> 对账）。**写 Excel 只产 `.xlsx`**，需要可选依赖 pandas + openpyxl；缺依赖时回 **400** 并给出
> 安装指令，不是 500 栈。三种「名字与声明互不点名」的写法一律 **400 且不留下文件**：输出名是
> `.xls`（老格式要 `xlwt`，不在依赖表里）、名字是 `.xlsx` 而 `target_format` 不是 Excel（会写出
> 一份 Excel 打不开的 JSON 文本）、`target_format` 是 Excel 而名字不是 `.xlsx`。`xls` 仍然只在
> **读边**合法（`source_format`）—— 读写两张表刻意分开，差的恰好只有它。
>
> `semantic-search` / `vector-status`（L216）是 `augmentor/vector/` 那一族后端第一次接到产品入口。
> 两面的参数一律取自 `config.vector.*`，并把**生效值回显**在响应里：`dimension` 改的是哈希嵌入的宽度，
> 同一查询的分值与排名会跟着变；`collection` 决定持久化文件名与向量 ID 前缀；`storage_dir` 是
> `vector-status` 唯一去读的那份索引的位置。嵌入用的是**确定性字符二元组哈希**，不依赖
> `sentence-transformers`——接真模型等于写一条任何没装它的环境都跑不到的产品路径。自动建库的闸是
> `vector.enabled`（出厂 `false` ⇒ 与 L216 之前的产物一字不差），建库动作住在
> `AugmentorPipeline.build_vector_index`，回执落在增强报告的 `vector_index` 键里（`null` 的意思是
> 「配置没让建」，不是「建了但空」）。
>
> 两条端点都**不挂鉴权**，这个归类是逐后端核过的而不是按方法名想当然：`semantic-search` 走内存索引
> （`storage_dir=None`），`vector-status` 走 `load()`，而 `FAISSDB` 的构造与 `load()` 都不写盘。
> 会写盘的是 `persist()`，它只在 `vector.enabled=true` 的建库阶段跑，而那条路只能由已挂鉴权的增强端点
> 触发。至于「构造即落盘」的后端（chromadb），它在到达文件系统之前就被能力闸拒成 400。
>
> `language-profile` / `translate`（L217）是 `augmentor/multilingual.py` 第一次在产品侧被实例化——
> 在那之前 `MultilingualSupport` 的调用者只有测试，`multilingual` 四键全部只写不读。四只键的落点：
> `supported_langs` 是构造参数（画像的清单外集合、翻译的准入名单），`default_target_lang` 是请求
> 省略 `target_lang` 时的生效值（权威序沿用 L214 的**请求参数 > 配置键 > 内置默认**），
> `translate_batch_size` 是一次提交的条数上界，`enabled` 是翻译那条按 token 计费路径的闸。
> 鉴权分档同样是判断而非照抄：画像是纯正则、不花钱 ⇒ 只读白名单；翻译逐条打模型 ⇒ 与
> `/api/augment/start` 同档挂 `verify_api_key`。

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

### quality（9）

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
> 可选依赖，因此能回答「当前环境缺什么、哪些功能会降级」，也可能更慢或失败。它另带 `model_retry_count` 与 `model_retry_wait_seconds` 两只后端重试统计（`health` 刻意不含这两键）：**管道缺失时它们是 `null` 而不是 0**——「没有后端」与「后端重试过 0 次」是两句话，折叠成一个数字就丢掉了前半句。前端自 L237 起在系统状态页按这两档分别渲染（`null` ⇒ 「无后端」）。

### system（15）

| 方法 | 路径 | 说明 |
|------|------|------|
| `POST` | `/api/system/auto-test` | 运行数据集自动化测试 |
| `GET` | `/api/system/backups` | 列出备份 |
| `POST` | `/api/system/backups` | 创建备份 |
| `DELETE` | `/api/system/backups/{backup_id}` | 删除备份 |
| `POST` | `/api/system/backups/{backup_id}/restore` | 恢复备份 |
| `GET` | `/api/system/backups/verify` | 校验备份完整性 |
| `POST` | `/api/system/backups/cleanup` | 清理旧备份 |
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

### dataset 数据集工具（18）

| 方法 | 路径 | 说明 | 人读字段提示 |
| --- | --- | --- | --- |
| `POST` | `/api/dataset/stats` | 数据集统计 | 只读分析。`summary` 是**字符串摘要**（不是 dict）；逐字段统计在 `field_statistics`（列表）。 |
| `POST` | `/api/dataset/validate` | 数据集验证 | 只读。`is_valid` 对**空数据集恒为 true**（一条问题都没有是因为一条都没看，A92）。它的职责是报告坏数据，所以对坏输入回 200 而不是 4xx。 |
| `POST` | `/api/dataset/search` | 数据集搜索 | 只读。`total_matches` 是**分页前**的全集口径；`fuzzy_threshold` / `ngram_n` / `filters` 三个旋钮的**生效值**在响应里回显（不传时是 `null`，不是默认值）。`method="regex"` 的模式串由调用方给出：编译不过回 **400** 并点名坏模式与位置（`无效的正则表达式 …（位置 N）`），**不是** 200 配 `total_matches: 0`——那是把参数错读成语料里没有（L267）。同一支还带两把静态闸（L278 / A345）：模式串长度 > 1024 ⇒ **400** 并点名两侧数字；形状是「可变长的组被重复到 ≥2 次」（`(a+)+b`、`(.*)*`、`(\w+\s?)*`、`((a+))+` 这一族）⇒ **400** 并点名组所在列、组内可变长片段与套在外面的量词，判在读语料**之前**（空数据集也拒）。`contains` / `exact` 那一半不判形状也不判长度——把 `(a+)+b` 当普通字符串搜是合法请求。已知误伤 2 形（`(\[[^\]]*\])+`、`(?:[^,]+,){3}`）与不判的两族（交替分支共享首字符即 `a` 与 `aa` 两支整体被 `+` 重复、可选量词 `(a?)+`）见 `tests/unit/test_pattern_blowup_l278.py`；这是缓解不是根除，请求级超时与线性引擎那一半仍待拍。 |
| `POST` | `/api/dataset/compare` | 数据集对比 | 只读。返回质量向 A/B 结论 + 重叠度 + 建议，`diff_datasets` 的成员语义见 §11 排查。 |
| `POST` | `/api/dataset/features` | 字段特征检测 | 只读。`field_features` 是**列表**（不是 dict），每项一个字段的画像。 |
| `POST` | `/api/dataset/auto-config` | 自动配置推荐 | 只读。返回推荐配置 + 理由；推荐值不落盘，要 `POST /api/config` 才生效。 |
| `POST` | `/api/dataset/impact` | 增强前后影响评估 | 只读。`duplicate_rate` 是**增强后**的重复率；语义分数在 `semantic_evaluated=false` 时不参与（API 面拿不到原始问题）。`text_field` 空串 / 纯空白 / 那两份数据里根本没有这一列都回 400，detail 带首条实际字段（L238 / A321，与健康门禁、批量翻译共引 `validation.require_item_field`）；空串这一档的措辞从「没有任何一条含字段 ''」改成「`text_field` 不能是空字符串」——同一个 400，但后者说的是真凶。 |
| `POST` | `/api/dataset/evaluate` | 生成文本指标评估 | 只读。响应键就是 `metrics` / `sample_count` / `details`。请求省略 `metrics` ⇒ 取 `config.evaluation.metrics`（L219 接线，权威序 请求参数 > 配置键 > SDK 内置默认）；生效清单统一过 `METRIC_FUNCTIONS` 白名单，越界名 400，空清单也 400（SDK 的 `metrics or [内置三只]` 会把空值静默补成三项，所以在本层出声）。两侧字段名 `generated_field` / `reference_field` 同规（L220 接线，配置那两键出厂值与被替换的硬编码默认同为 `output` ⇒ 零漂移）：请求侧默认是 `null` 哨兵，**省略与显式传 `"output"` 从此是两种不同的事**；取不到的列名（含空串）由逐条门口报 400 并带下标，不静默补空。 |
| `POST` | `/api/dataset/convert` | 格式转换 | **写盘变换类**：必须给 `output_file`，响应只回「写到哪、写了多少」。 |
| `POST` | `/api/dataset/merge` | 合并数据集 | **写盘变换类**：同上。 |
| `POST` | `/api/dataset/sample` | 数据集采样 | **写盘变换类**：同上。 |
| `POST` | `/api/dataset/split` | 分割数据集 | **写盘变换类**：输出字段是 `output_dir`（本组其余几条叫 `output_file`）。`stratify_distribution` 在按 `instruction` 分层时可能有**数千个键**（紧凑 JSON 数百 KB），所以写盘类只回计数不回明细。 |
| `POST` | `/api/dataset/aggregate` | 多源聚合 | **写盘变换类**，但**偏了本组「只回写到哪、写了多少」那条口径**：响应里 `conflicts` 是**明细整份**（每项 `{key, values, sources}`，实测 133 字节），而且端点手拼回执、没调 `AggregationResult.to_dict()` ⇒ 两份键集各自漂（`total_count` 改名 `aggregated_count`、`conflict_count` 只在线上缺席、`output_file` 只在线上有）。`target_size` **只对 `weighted` 生效**，配其余三支完全空转（实测填 1 配 union 仍交全量，A294）；**省略不是 400**——实测 200，走 `DEFAULT_TARGET_SIZE = 100`，够不着就交全量（旧文案那句「省略会 400」与端点行为不符，L230 按实测订正）。`consistent` 连同源自比也算冲突，且冲突条目整份不落盘（A297）。 |
| `POST` | `/api/dataset/rag` | 转换为 RAG 格式 | **写盘变换类**：同上。`format` 省略时落 `config.rag.default_format`（出厂默认 **`llamaindex`**，`augmentor/config.py:533`；路由那第三个兜底字面量才是 `langchain`，只在配置被设成空串时才轮得到它，A295），响应里的 `format` 是**生效值**（L214）。旧文案把出厂默认写成 `langchain`，L230 按实测订正。请求里**多送** `chunk_size` / `chunk_overlap` 一律静默忽略——两个切块旋钮只从服务端配置读。 |
| `POST` | `/api/dataset/semantic-search` | 语义相似检索 | 只读分析（L216）。按 `config.vector.*` 的生效值在**内存**建索引再检索，不落盘；`top_k` 非正整数 400（判据住在 `VectorDB.search`，不在路由里重复一遍）。`text_key` 换的是参与嵌入的字段，默认 `instruction`。 |
| `GET` | `/api/dataset/vector-status` | 向量库状态 | 只读（L216）。`exists` / `count` 读的是 `config.vector.storage_dir` 里那份持久化索引；`enabled` 是「增强结束时自动建库」那只闸的生效值，两者分开设（`enabled=false` + `exists=true` = 上轮开着建过、这轮关了闸）。`backend` 是配置选择，`faiss-cpu` 没装时的引擎退化看 `/api/system/dependencies`。配 `chromadb` 时回 **400**：那侧没有 `persist`/`load` 面，闸在构造之前，所以既不会 500 也不会凭空建目录。 |
| `POST` | `/api/dataset/language-profile` | 语言分布画像 | 只读分析（L217）。纯正则检测，**不打模型、不落盘**，所以 `multilingual.enabled` 关掉也照常可用（闸只管花钱那条路）。`supported_langs` / `default_target_lang` 回显的是生效值；`outside_supported` 里出现 `mixed` 不是错误——它是检测标签而非语言码。 |
| `POST` | `/api/dataset/translate` | 批量翻译数据集 | **挂鉴权**（L217）：逐条调用模型后端，按 token 计费，与 `/api/augment/start` 同档——判据是「会不会替调用方花钱」，不是 HTTP 动词。`multilingual.enabled=false`（出厂值）直接 400，且闸在**读盘之前**。`target_lang` 省略时落 `default_target_lang`；`batches` / `batch_size` 一起才看得出分批确实发生（飞行中的翻译任务数被压到 `min(批宽, max_workers)` 以内）。**失败回执**（L221）：`failed` 是逐条失败的计数，`failed_indices` 是全量下标（可直接指回 `input_file`，跨批也已折算成全局坐标），`errors` 是前 20 条 `{index, error}` 样本、`errors_truncated` 说明样本是否收窄。部分失败仍回 200——它既不是客户端错误也不是服务端故障，重发整份只会重复花钱；诚实读数在响应体里，不在状态码里。只读 `items` 读不出这件事：失败条目回退原文，形状与成功一致。**落盘口**（L236 / A283①）：`output_file` 可选——省略时整份译文只在响应体里（刷新即失），给了就把 `items` 原子写进白名单数据根、响应回一个**已 resolve 的绝对路径**（键恒在，没给时为 `null`；用「键在不在」区分不了「没落盘」与「落了盘没回显」）。顺序是量过的：**落点先解析、再动模型**，越界落点 403 出声时模型调用 **0** 次。**`text_key` 判三档**（A322，L238 收口）：空串、纯空白、数据集里不存在的键都在**动模型之前**回 400（detail 带首条实际字段），模型调用 0 次且一个字节都不落盘。改前这一档回的是 200、回执四键全零，产物里凭空多出 `<key>`、`<key>_original` 与 `language` 三键而原字段一个字没动——`language` 是无条件盖上的目标语言戳，所以「200 且 failed=0」本来就读不出「翻成了」，仍得核对产物里那一列。 |

### system 运维面（13）

| 方法 | 路径 | 说明 | 人读字段提示 |
| --- | --- | --- | --- |
| `GET` | `/api/system/dependencies` | 可选依赖诊断 | 只读。返回已装/缺失两栏 + **降级功能清单**（缺哪个包会导致哪个功能退化）。 |
| `POST` | `/api/system/validate-config` | 校验配置文件 | 只读。**不传 `path` 时校验服务自己的 `config.yaml`**；显式传 `path` 仍按数据白名单校验（越界 403）。**不挂 `verify_api_key`**：服务端设了 `AUGMENTOR_API_KEY` 时它照样 200（实测，它是 POST 但只读，门禁只看写操作）。前端自 L234 起在「配置管理 → 配置文件校验」卡上接线，路径留空即校验服务自己那份（空串由服务层丢成整键不送）；卡上写明本支**校验不通过也回 200**、红绿只看 body 里的 `is_valid`，而「合法」不等于没有问题清单——出厂那份实测 `is_valid` 为 true 却带着 5 条「环境变量未设置: …_API_KEY」警告，最小合法配置（只有 `models.default`）为 true 带 1 条 `models.default` 警告；`line` 三档实测恒为 null，所以页面不按行号排序、也不提供「跳到出错行」。 |
| `POST` | `/api/system/monitor` | 质量监控快照 | 只读。返回质量监控快照。不挂 `verify_api_key`（实测门禁升起时仍 200）。前端自 L234 起在「系统状态 → 质量监控与自测」卡上接线：三指标与告警逐条照回显渲染，阈值是 completeness 0.7 / diversity 0.4 / consistency 0.7；空数据集那一档 `metrics` 是**空对象**而不是三个 0，页面按「键在不在」渲成「-」；告警正文是**服务端英文原文**（形如 `consistency below threshold: 0.00 < 0.70`），没有中文那一份；「触发告警」为 0 只说明这一份没触那三个阈值，**不是「质量合格」**。 |
| `POST` | `/api/system/auto-test` | 运行数据集自动化测试 | 只读判定。`suite` 走**封闭清单**：未知套件名 400 而不是静默回落 default。不挂 `verify_api_key`（实测门禁升起时仍 200）。前端同一张卡上接线，`suite` 这一格页面**不送**（送空串是 400，只有 `null` 与省略才走 default）；五条判定按服务端顺序渲出，通过率是 `pass_rate * 100`（0.6 ⇒ 60%），说明列照抄服务端**已截断**的 `message`；空数据集那一档实测 2/3、通过率 40%，后三条 `message` 是「数据为空」。 |
| `POST` | `/api/system/migrate` | 迁移数据集结构 | 只读判定（产物另给 `output_file`）。`rules` 里未知 id **400 并列出全部合法 id**，不静默丢弃。挂 `verify_api_key`：设了 `AUGMENTOR_API_KEY` 而请求没带对的 `X-API-Key` 就是 401「缺少或无效的 X-API-Key」，前端那份 key 在「配置管理 → 写操作鉴权」卡上存。前端自 L234 起在「系统状态 → 迁移与流式」卡上接线：规则多选一条不选就是递 `rules: []`（实测与省略等价 = 三条全用，**不是「不迁移」**），破坏性改名与整份覆盖走 Popconfirm 二次确认；产物已存在时**整份覆盖**，没有 409、也没有「已存在」提示；回执里的 `source_path` / `target_path` 都是**服务端绝对路径**。 |
| `POST` | `/api/system/stream` | 流式处理数据集 | 只读判定（产物另给 `output_file`）。JSONL 单行语法错**跳过并告警**；JSON 数组截断**报 400**——两者行为相反，是格式决定的。挂 `verify_api_key`（401 与 key 的存放位置同 `migrate`）。前端同一张卡上接线：`operation` 四档与 `output_format` 两档都取自服务端权威清单（后者是 `augmentor/streaming.py` 的 `WRITER_FORMATS`），页面不给 parquet 那一档（400），`chunk_size` 留空即不送（0 与负数是 400）。同一份 5 条夹具上四档的实测写出条数是 none 5 / quality 2 / dedup 3 / clean 5 —— clean **一条不丢**，改写只落在内容里（只动 `instruction`：全角标点换半角、剥首尾空白，`output` 里的 HTML 标签与空白原样留着）；回执里的 `processed` 各档都等于读入条数，它数的是「逐条看过」而不是「改动过」，页面上的「被丢掉」因此是本页自己算的读入 − 写出。CLI 侧 `stream --operation` 自 L234 起与端点共用同一份清单 `quality / dedup / clean / none`（改前 CLI 写的是 `export`——那个名字承诺的导出从来没发生过，而 HTTP 侧合法的 `none` 被 argparse 直接拒，A313）。 |
| `GET` | `/api/system/dependency/datasets` | 已登记数据集列表 | 只读**且不写盘**（**A307 已于 L249 收口**）：注册表目录不存在时构造函数不再 `mkdir(parents=True)`，实测请求前后目录都不在场，此时回 **200** `{"datasets": []}` 而不是 404。`registry_path` 省略时用白名单首个根目录下的 `.dependency_registry`（**与 CLI `--registry-path`、SDK 形参那两份同名默认不是同一个目录**，A308）；四档：空串 400「目录路径 不能为空」、`..` 400「路径包含非法组件」、白名单外绝对路径 403、白名单内绝对路径 200 且那一份注册表与默认那份**互不可见**。元素九键 = `POST` 回执 = `DatasetInfo.to_dict()`。 |
| `POST` | `/api/system/dependency/datasets` | 登记数据集 | **写**（落 `datasets.json`），改前那行「只读」是错的。挂 `verify_api_key`：服务端设了 `AUGMENTOR_API_KEY` 而无 `X-API-Key` 头就是 401「缺少或无效的 X-API-Key」；前端那份 key 在「配置管理 → 写操作鉴权」卡上存，由请求拦截器随每个请求送出（A312 已收，与备份族缺口 A302 同源）。回执九键（`dataset_id` / `name` / `description` / `path` / `item_count` / `created_at` / `updated_at` / `tags` / `metadata`），`dataset_id = md5(name)[:8]` 只由名字算 ⇒ **同名再登记是覆盖**，无 409 也无任何提示，`created_at` 被刷新，第二次送空 `description`/`tags` 会把旧值**抹成空**（A306；与备份族「覆盖磁盘、索引多一行」的形状不同，这里索引也只留 1 条）。`description`/`tags` 改前**回显即丢**：端点在 `register_dataset()`（内部已落盘）**之后**才改返回对象，改的是内存 —— 响应带着这两格而随后的 GET 与磁盘上都是空；L232 把两格随登记一起传下去，三处一致（A305）。`input_file` 五档：不存在 404「文件不存在」、白名单外 403、带 `..` 400「路径包含非法组件」、空串 400「文件路径 不能为空」、顶层不是 JSON 数组 400「数据文件的顶层必须是 JSON 数组，当前是对象」；缺 `name`/`input_file` 则是 FastAPI 英文 **422** `Field required` ⇒ 两种错误体都要认（A311）。登记过的数据集在产品面上**删不掉**：无 DELETE、无注销，SDK 也没有注销数据集的方法（A309）。`registry_path` 同 GET 那四档。 |
| `GET` | `/api/system/dependency/graph` | 数据集依赖图与校验问题 | 只读**同样不建注册表目录**（A307 已于 L249 收口，与上面那只 GET 同源）。回 `{nodes: [{id, name, item_count}], edges: [{source, target, type}], issues: […]}`，空注册表 ⇒ **200** 三个空数组。`issues` 只报悬空两端（`missing_source` / `missing_target`，元素 `{type, dependency{五键}, message}`），**不查环**：实测用 SDK 直写 A→B（merged）与 B→A（filtered）成环后读图，`issues` 仍只有悬空那 1 条 ⇒「issues 为空」不能读成「依赖关系没问题」（A310）。边没有 HTTP 也没有 CLI 创建入口（`dependency` 的 4 个 action 里没有建边，`add_dependency`/`remove_dependency`/`get_dependencies`/`validate_dependencies` 四个 SDK 方法一个都没上线），只能由 SDK 的 `add_dependency` 建立 ⇒ 纯前端用户的 `edges` 恒为空（A309）。`registry_path` 同 GET 列表四档。 |
| `GET` | `/api/system/backups` | 列出备份 | GET 列出 / POST 创建。`backup_dir` 省略时用白名单首个根目录下的 `.backups`（**与 CLI `--backup-dir` 的默认 `.backups`、SDK 形参默认 `.backups` 不是同一个目录**，A301）；四档：空串 400「目录路径 不能为空」、`..` 400「路径包含非法组件」、白名单外绝对路径 403、白名单内绝对路径 200。空目录回 **200** `{"backups": []}`，不是 404 也不是 null；目录**不在场**时同样回空列表而**不把它建出来**（A332 / L249：六只只读面同源，构造零磁盘副作用）。索引里可以躺着**两条同 id** 的记录曾是实测形状（`timestamp` 不同而 `backup_path` 逐字相同）；**A298 已于 L250 收口**——同名再创建时那一行跟着更新，列表条数与磁盘份数一致，而 L250 之前写出的 `index.json` 里的同名多行读边照原样交回，前端行键因此继续拼时间戳。 错误档 **400 / 403** 自 **L252** 起逐档写进 OpenAPI 的 `responses`（A330 已收），本员不挂鉴权所以没有 401。 |
| `POST` | `/api/system/backups` | 创建备份 | 回执七键（`backup_id` / `timestamp` / `source_path` / `backup_path` / `item_count` / `file_size` / `checksum`），两条路径都是**服务端绝对路径**。`name` 省略**或给空串**都生成 `backup_<YYYYmmdd_HHMMSS>`（秒级；它和回执 `timestamp` 自 **L250** 起来自**同一次** `datetime.now()`，改前是两次调用、跨秒时对不上，A303 已收）。同名再创建**覆盖磁盘那一份**，索引里那一行也跟着更新成新读数 ⇒ 行数与磁盘份数一致（**A298 已于 L250 收成 ①**）；未做的是 ②「让 `backup_id` 真的唯一」，所以同一秒内的两只**匿名**备份仍共享 id、只留一份，那一格由 `tests/unit/test_backup_index_collapse_l250.py` 当场钉住。`name` 必须是**一个可用的 URL 路径段**：带 `/` 或反斜杠、`.`/`..`、`#`/`?`、控制字符、尾随点或空白、超 255 字符一律 **400**（**A366 / L277** 收的那一格：改前 `sub/deep` 实测 **200**，文件落 `.backups/sub/deep.json`、索引与清单里都有这一行，而 `DELETE /api/system/backups/{backup_id}` 与 `…/restore` 的 `{backup_id}` 只匹配 `[^/]+`，percent 编码也救不回来 ⇒ 四档全 **404**，那份备份谁都动不了——建得出来却寻不了址；含 `#` 的名字坏在**另一维**：段形状合法，但裸插值的客户端把 `#` 之后当 fragment 丢掉，实测 restore **405** / delete **404**，只有 `frag%23ment` 寻得着，所以出生时一并拒，前端两只入口自 L277 起改走 `encodeURIComponent` 好让**存量**含 `#` 的备份仍可寻址）。落在根内且形状合法的名字照旧放行，判决只问名字不问落点；`path_lands_within` 那道旧闸保留当第二线，`../../escaped` 现在由新闸先点名（同是 400，文案换成「不能含路径分隔符」）。 —— L231 补的门只判「写出根外」，本轮把它收成「单段可寻址」。源文件不存在 404「文件不存在」。本族三只写操作都挂 `verify_api_key`，`web/src` 自 L233 起由请求拦截器送 `X-API-Key`，key 在「配置管理 → 写操作鉴权」卡上存 ⇒ 设了 key 而浏览器里没存（或存的不一致）就一律 401（A302 已收）。SDK 侧的 `clean_old_backups` / `verify_backup_integrity` 自 **L243** 起有 HTTP 入口（A300 已收），见下面 `…/cleanup` 与 `…/verify` 两行。 错误档 **400 / 401 / 403 / 404** 自 **L252** 起写进 `responses`，每一档都由一个真请求实测过。 |
| `POST` | `/api/system/backups/{backup_id}/restore` | 恢复备份 | 写。备份不存在回 **404**（与 DELETE 同组同码、同文案「备份不存在」）。回执三键 `{backup_id, output_path, item_count}`，落点键叫 **`output_path`**（本组其余五条写盘端点叫 `output_file`，`split` 叫 `output_dir`）。目标已存在时**整份覆盖，不问**。把备份文件改坏再恢复 ⇒ **409**「备份文件校验和不匹配，文件可能已损坏，未执行恢复: <backup_id>」，且**一个字节都不写**：库里在覆盖输出路径之前抛 `BackupIntegrityError`（**A299 已于 L243 收口**；改前这一档是 200 加一句警告日志，交回来的就是改坏的内容）。想不等恢复就知道哪份坏了，用下面那只只读的 `…/verify`。 错误档 **400 / 401 / 403 / 404 / 409** 自 **L252** 起写进 `responses`，409 那句说明带上「目标文件与索引都未被改写」。 |
| `DELETE` | `/api/system/backups/{backup_id}` | 删除备份 | 写。同上，404。按 id 过滤索引 ⇒ 同 id 的**全部**行一起消失（L250 起新写出的同名备份只有一行，多行只剩存量索引），磁盘那一份也删掉；此后同 id 再恢复 / 再删都是 404（A298 已收）。 错误档 **400 / 401 / 403 / 404** 自 **L252** 起写进 `responses`，其中 404 的说明点名「索引有条目而备份文件已丢不算这一档，仍回 200」。 |
| `GET` | `/api/system/backups/verify` | 校验备份完整性 | 只读，**不挂鉴权**（同族另一只 GET 一样不挂）。逐只重读备份文件算 sha256 并与索引登记值比对，回执 `results` 一键，每项五键（`backup_id` / `exists` / `checksum_match` / `item_count` / `timestamp`）。`backup_id` 给一个索引里没有的名字 ⇒ **200 空列表**而不是 404：这只端点报的是「目录里哪些备份是坏的」，不存在的 id 既不坏也好，报不了任何事。目录不存在 ⇒ **200 `{"results": []}` 且不把目录建出来**（改前靠路由那道挡板，L249 起 `DatasetBackup` 的构造本身不再 mkdir 落 index.json，A307 / A332 同源收口）。`exists` 判的是「存在**且非空**」，零字节文件也算丢，此时 `checksum_match` 为 false 且无从比。结果以 `backup_id` 为键 ⇒ 存量索引里的同名多行在这里只出**一行**；自 **L250** 起列表也只剩一行，两面因此对上（A298 的另一面已收）。`backup_dir` 四档与 `GET /api/system/backups` 相同。 错误档 **400 / 403** 自 **L252** 起写进 `responses`；只读面不声明 401，也不给幽灵 id 或坏备份声明 404 / 409。 |
| `POST` | `/api/system/backups/cleanup` | 清理旧备份 | 写，挂 `verify_api_key`（本族第四只写操作，走 A302 那条通道）。请求体 `max_backups` 是**保留份数**而不是删除份数，出厂默认 10；0 与负数一律 **400**「max_backups 必须是不小于 1 的整数，当前是 0」——按库里的删除算术，负数等于把目录里的备份全删光，而「保留 0 份」与手滑想打 10 无从分辨，所以判参排在构造**之前**（判在一切磁盘动作之前，而 L249 起构造本身零副作用）。按索引 `timestamp` 从旧到新排，多出来的连文件一起删；不足保留数时删 0 只并回 **200** `{deleted_count: 0, max_backups: 10}`。回执两键 `deleted_count` / `max_backups`。**两格实测口径**：其一，保留数按**索引行数**算而不是按磁盘文件数算——自 **L250** 起行数与磁盘份数对得上（同名不再留两行），所以「行数 − 保留数」就是删除份数；改前同 id 的两行会占掉两个删除名额却只删一次（`delete_backup` 按 id 清全部匹配项），`deleted_count` 可以小于「行数 − 保留数」——4 行索引 3 个文件保留 1 实测回 2，而 L250 之前写出的存量索引里若还躺着同名多行，今天走的仍是这套旧算术；其二，目录不存在时这只**写**端点**也不再把目录建出来**（L249 摘掉 `DatasetBackup` 构造的 mkdir，`index.json` 只在真删了东西时由原子写落下），与上面 verify 同口径，而「清理一个还不存在的目录」仍然回 **200** 空结果而不是 500。`backup_dir` 走 query，四档同上。 错误档 **400 / 401 / 403** 自 **L252** 起写进 `responses`，400 那句同时说清「整数但小于 1」与「不是整数走 422」的分界。 |

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
  "multimodal": {"enabled": false, "image_extensions": [".jpg", ".jpeg", ".png", ".bmp", ".webp"], "audio_extensions": [".wav", ".mp3", ".flac", ".ogg", ".m4a"]},
  "versioning": {"enabled": true},
  "visualization": {"enabled": true}
}
```

> **末两节是只读节（L215）**：`versioning` / `visualization` 整节都不在
> `POST /api/config` 的写入清单（`WRITABLE_SECTIONS` 七节）里，回显它们的意思是
> 「操作者看得见生效值」，不是「这里可以改它」——要改只能改 `config.yaml`。
> 这两节各只回显**已接上读者的那一只键**：同一节里的 `visualization.types` 至今零读者
> （见下方 `enabled` 一段与账本 A267），把它塞进回显就是亲手造一格 A138 型幽灵面。
>
> 顺带订正一处本文件的陈旧：上面 `multimodal` 的两只扩展名键是 L214 才进回显面的，
> 而这份示例当时没跟着改——本轮重读才抓到，形状与「回显面被钉而示例漂着」同源。

### `GET /api/models`

列出配置文件中定义的模型后端。

```json
{"models": ["ernie", "openai", "ollama", "claude", "gemini"], "default": "ernie"}
```

> **这两个字段是「条目名」不是「`type`」** —— 值是配置里 `models:` 节的键名
> （用户自己起的），`default` 是 `models.default` 那根指针指向的条目名。
> 出厂 `config.yaml` 把条目名起成了后端名的样子，于是「两处同名不同物」：
> `ernie` 这条的 `type` 其实是 `baidu`。`type` 的封闭清单只有
> `baidu` / `openai` / `ollama` / `claude` / `gemini` 五个值，写别的
> （包括把条目名当 `type` 写）在加载时就当场拒。

---

## 3. 数据管理

### `GET /api/data/list`

扫描**路径白名单**各根目录（出厂默认即 `data/`）下以 `train_data` 开头的 JSON 文件；
`?all_files=true` 那一档只放开**名字**这道闸，列出根目录下全部 JSON，扫描范围一字未动（A304 / L251）。
`name` 是文件名，`path` 是服务端拼出来的完整路径 —— 两者都能直接喂给 `/api/data/load/{filename}`。

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
| `page` | query | int | `1` | 页码，从 1 开始；`< 1` ⇒ `400` |
| `page_size` | query | int | `20` | 每页条数；`< 1` ⇒ `400` |
| `search` | query | string | `""` | 在 `instruction` 与 `output` 中做不区分大小写的子串匹配 |

```json
{
  "total": 200,
  "page": 1,
  "page_size": 20,
  "items": [{"instruction": "如何申请入住？", "input": "", "output": "通过 App 提交申请。"}],
  "indices": [0]
}
```

**`indices` 是写端点唯一的权威坐标**（A365 / L277）：与 `items` 同长同序，逐格是「这一条在
盘上那份文件里的 0 基下标」。本端点交回的是**过滤后切片**，而 `PUT /api/data/update` 与
`DELETE /api/data/delete` 读的是整份文件并按 `items[index]` 直接写盘 ⇒ 页内行号从来不是坐标：
翻到第 2 页时行号 0 指的是第 1 页第 1 条，`search` 生效时更是对不上号（实测 25 条夹具里
第 2 页第 1 行发 `index=1` 改掉了第 1 页第 2 条，还回 200「保存成功」）。

`page` / `page_size` 的判据在门口（共引 SDK 的 `validation.require_count`）：它们落到
`items[start:end]` 这把切片，而 Python 切片对越界值不报错、只换语义 —— 改前实测
`page_size=-10` 回 200 并交出 **15 条**、`page=-1` 回 200 并交出**第 5~14 条**、
`page=0` / `page_size=0` 回 200 交出空页。四种都是静默错答，故一律 400。页码超过末尾仍是
`200` + 空页（合法请求，只是没数据）。

`400` 分页参数非法；`404` 文件不存在；`500` 读取或解析失败。

### `PUT /api/data/update/{filename}`

替换指定索引的单条数据。

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `filename` | path | string | 数据文件路径 |
| `index` | query | int | 待替换的索引——**盘上 0 基下标**，取自 `GET /api/data/load` 的 `indices` |
| body | body | object | 新的数据条目（任意 JSON 对象） |

```json
{"success": true}
```

`400` 索引越界（含负数：`items[-1]` 会改掉最后一条而不是报错）。

### `DELETE /api/data/delete/{filename}`

删除指定索引的数据。参数同 `PUT`。返回 `{"success": true}`，`400` 索引越界。

### `POST /api/data/upload`

上传数据文件（`multipart/form-data`）。产物**恒为 JSON 数据集**：JSON 原样收下，表格与
JSONL 进料交给转换器的读边解析成规范记录（`instruction` / `input` / `output` 那一套）再落盘。

| 字段 | 类型 | 说明 |
|------|------|------|
| `file` | file | 必填。按扩展名认 `.json` / `.jsonl` / `.csv` / `.tsv` / `.xlsx` / `.xls`；扩展名认不出（`.txt`、无扩展名等）时按 JSON 处理，与既有客户端同路 |
| `input_format` | form 字段 | 可选，**优先于扩展名**。合法值：`json`、`jsonl`、`csv`、`tsv`、`alpaca`、`sharegpt`、`chatml`、`llama_factory`、`vicuna`、`belle` 与 Excel 的三种拼法（`excel` / `xlsx` / `xls`）；不合法直接 `400` 并列出全表 |

落点由配置 `web.data_roots`（出厂默认 `data/`）圈住，**不写服务端当前目录**：`file.filename`
当**白名单内的相对路径**用（`data/sub/x.json` 就落到那里），裸文件名落进白名单根目录，含
`..` 的路径 `400`，越界的绝对路径 `403`。源是表格时落名一律换成 `.json`（不把 JSON 字节
写进 `.csv` 这种假容器）。

| 状态码 | 触发条件 |
|--------|----------|
| `200` | 成功，`count` 为落盘条数 |
| `400` | 非 UTF-8 文本；不是合法 JSON；顶层不是数组或元素不是对象；按表格解析出 0 条；内容整份其实是 JSON 却按表格格式进料；`input_format` 不合法；路径含 `..` |
| `403` | 目标路径在白名单之外 |
| `413` | 超过 `web.max_upload_bytes`（默认 256 MiB）；文案注明「未读取、未落盘」 |

```json
{"success": true, "path": "/srv/augmentor/data/train_data_uploaded.json", "count": 120}
```

`path` 是落盘后的绝对路径。Excel 那一档需要服务端装 `pandas` 与 `openpyxl`，缺依赖时回
`400` 并说明装什么（不是 `500`）。

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

字段说明：

- `use_quality` / `use_dedup`（L215 起是 `bool | 缺席`，缺席**不等于**关掉）：请求里给值就是
  调用方点名，端点原样转发；**不写**这两只字段时管道读 `config.quality.enabled` /
  `config.dedup.enabled`。权威序 请求参数 > 配置键 > 内置默认。出厂两键都是 `true`，
  所以省略方拿到的结果与 L215 之前一致；随包前端两只字段总是显式送，也不受影响。
  端点自己在缺席时补 `True` 是错的——那会让配置臂永远拿不到控制权。
- `use_checkpoint`：仍是普通 `bool`，默认 `true`，没有配置对应键。

### `GET /api/augment/progress`

返回当前断点进度——口径是**管理器此刻最后加载的那一份断点**，不是「你这次请求启动的那只任务」：本端点没有任何任务身份可传（响应模型里也没有 `task_id` 以外的选择器），并发跑多只增强时它只能报其中一只；要精确读某只任务，请用 `GET /api/augment/checkpoints` 拿到任务 ID 后按断点文件对账，或把增强串行跑。无进行中的任务时返回 `{"status": "no_checkpoint"}`。

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

`weights` 是健康分四项（完整性 / 多样性 / 质量分布均衡性 / 覆盖率）的权重：必须 4 个、逐项必须是
[0, 1] 内的数值（布尔不算、NaN 不算）、四项之和落在 1.0 ± 0.01 **含端点**（判和之前先把浮点噪声量化到 9 位，所以 0.99 与 1.01 放行、0.98 与 1.02 才拒；L238 / A320），省略则各 0.25。非法值在**读文件之前**就返回 400，detail 点名到 `weights`
的下标或实测和；`text_field` 决定重复率按哪个字段算（默认 `instruction`），空串 / 纯空白 / 数据集里不存在的键一律 400 且带上首条实际字段（L238 / A321：改前这一档一句判决都没有，所有条目取到空值、`duplicate_rate` 静默变成 1.0 并在那条规则上判负）。`health_score` 的值域是 [0, 1]（出口钳制；越界只可能来自那 0.01 的容差）。

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
{"formats": ["json", "jsonl", "csv", "tsv", "alpaca", "sharegpt", "chatml", "llama_factory", "vicuna", "belle", "openai", "huggingface", "raw"]}
```

> 上表是 2026-10-09 实跑结果。清单由 `ExportFormat` 枚举驱动，**会随实现增长**，
> 精确值以 `ExportFormat` / `GET /api/export/formats` 为准，不要从本文档手抄。
> （注：`GET /api/config` 回显里的 `export.formats` 是**五值出厂默认**，
> 与这里的「全部支持格式」不是同一件事：本端点报的是**能力清单**，因此不受
> `export.formats` 收窄（那是「不给点名时导出哪几只」的**策略**，不是「本装配支持哪几只」）；
> 清单里出现 `csv` 而出厂默认五值没有它，正是这一区分的实测证据。）

### `POST /api/data/export`

导出单个数据集。

```json
{"input_file": "train_data.json", "output_dir": "exports", "formats": ["jsonl", "alpaca"]}
```

`formats` 的取值序是**请求参数 &gt; `config.export.formats` &gt; 全部格式**（L214）：点名了就只导那几只；
没点名而配置里写了 `export.formats` 就按配置导；两边都没写才导出全部格式。
**这对省略方是行为变更**：出厂默认 `export.formats` 是五值非空清单，所以从 L214 起「省略 `formats`」
得到的是那 5 只而不是全部 13 只。要旧行为就显式点名，或把配置里的 `formats` 写成 `[]`。
（管道层 `AugmentorPipeline.export_dataset(..., formats=None)` 的「None = 全部」不变，本档只讲 HTTP 入口。）

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

`formats` 省略时与 `/api/data/export` 同一档序（请求 &gt; `config.export.formats` &gt; 全部，L214）。
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

`format` 省略或传 `null` 时落 `config.export.default_format`（L214）——响应里的 `format` 就是**生效值**，
不是请求原值。

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

上表是**出厂默认**下的清单。从 L214 起它由 `config.multimodal.image_extensions` /
`audio_extensions` 驱动：配置里改了识别面，这里就跟着变（大小写与缺失的前导点会先归一成
`Path.suffix` 口径）。配置留空时才回落处理器的内置默认——内置默认额外认 `.gif`，
而出厂配置不认，这正是「配置能改变行为」的可证证据。

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

扫描目录，把同名 stem 的图片与音频融合为一条记录。请求可带
`image_extensions` / `audio_extensions` 覆盖本轮识别面，取值序同样是**请求参数 &gt; 配置键 &gt; 内置默认**
（L214）——同名 stem 配对看的是「这一轮认哪些扩展名」，所以覆盖扩展名会直接改变 `total_records`。

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

`config.visualization.enabled` 为 `false` 时（L215 起这一键真的有读者）返回
`{"charts": {}}` 而**不是** 4xx：这一族键是阶段开关，关掉的意思是「这一步别做」，
不是「这个能力不存在」，所以响应形状保持不变。出厂是 `true`，随包行为不变。

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

# 上传表格 / JSONL（服务端解析成规范记录后落成 .json；扩展名认不出时用 input_format 声明）
curl -X POST http://localhost:8000/api/data/upload \
  -F "file=@seeds.csv"
curl -X POST http://localhost:8000/api/data/upload \
  -F "file=@dump.dat" -F "input_format=jsonl"
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

## 11. 排查（API 面）

只列 REST 面特有的坑；依赖缺失、词云乱码等通用问题见 `docs/README.md` §9。

**Q：`POST` 请求返回 400，报「路径不在白名单内」**
A：所有 `input_file` / `filename` 都是**服务端路径**、不是 URL，且必须落在路径
白名单里（出厂默认只有工作目录下的 `data/`）。传绝对路径或 `../` 逃逸都会被拒。
把文件放进 `data/`、用相对名即可（见 §通用约定 · 路径白名单）。

**Q：写入类端点（导出 / 版本创建）报「目标目录不存在」而不是自动建目录**
A：写入必须显式给出已存在的目标路径，服务端**不替你造父目录**（防手滑把数据写到
意料之外的地方）。先建目录或指到已存在的目录下。

**Q：`dataset/validate` 对一份空数据集回 200 且 `is_valid: true`**
A：这是**已知语义**（账本 A92）：没有任何条目 ⇒ 没有任何校验失败 ⇒ 判为通过。
它不代表「数据没问题」。要拦空语料，调用方需自行判 `total_items > 0`。

**Q：质量类端点（report / audit / leakage）即使有问题也回 200，怎么让 CI 变红**
A：报告形端点默认只出报告、不判负（§响应契约、§易混字段词典 `overall_passed` 行）。
门禁语义走 `POST /api/quality/health-gate`（读 `pass_rate`）或 CLI 的 `--gate` 开关。

**Q：并发跑增强时 `GET /api/augment/progress` 的数字对不上**
A：进度是**单进程内的全局状态**，多个并发增强会互相覆盖。API 面一次只应跑一个增强
任务；需要并行请用独立进程 + 独立工作目录。

**Q：响应头里的 `X-Process-Time` 是什么单位**
A：秒（浮点）。它计的是服务端处理耗时，不含网络往返，可用来定位慢端点（见 §慢请求）。

