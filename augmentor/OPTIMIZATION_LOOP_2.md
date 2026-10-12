# augmentor 优化循环 · 第二本账（2026-09-28 启动，目标 100 轮：L101–L200）

第一本账 `OPTIMIZATION_LOOP.md` 在 L100 显式收口（「末轮不等 L101」），本账承接其全部纪律与
轮次编号，从 L101 起算；Backlog 编号新开 A201+/B201+ 命名空间，避免与第一本账的 A1–A185
混淆。主题：**更全面提升质量** —— 覆盖率缺数与偏支、前端页面级测试、容错路径、文档守卫、
性能热点并行推进。每轮一格、判据只降不升的棘轮、数字必须来自同一条命令链、探针先于结论、
引用必须真的指得到东西，全部沿袭第一本账。

## 基线（L100 终态复核，2026-09-28）

- 解释器：`C:\Python314\python.exe`（Python 3.14.4，pandas 3.0.2 / chromadb 1.5.9）
- 全量：`pytest -q` → 7276 passed / 3 skipped，exit 0，107.32 s
- 覆盖率：TOTAL 13,009 语句缺 37 / 分支 3,808 偏支 85（99%，门禁 80）
- 缺数分布（11 支文件共 37 条）：`augmentor/cli/commands/version.py` 8、`augmentor/retry.py` 6、`augmentor/version_control.py` 5、`augmentor/models/base.py` 5、`api/routes/quality.py` 3、`augmentor/config.py` 3、`api/routes/export.py` 2、`api/deps.py` 2、`api/routes/augment.py` 1、`api/routes/multimodal.py` 1、`augmentor/preview.py` 1
- 前端：11 页仅 1 页有组件级测试（DataManagement 上传一页）；门禁 = vitest + eslint（--max-warnings 0）+ tsc
- 分支：augmentor-opt100 与远端同步于 587392a91

> **上面是 L100 的起点读数，早已过期，不作为当前基准。** 缺数 37 → **0**（L172 清零）、
> 偏支 85 → **19**（L205 后由本轮 L196 复核为 19）、全量 7276 → **7897 passed / 3 skipped**
> （L196 终态）。当前基准一律以**最近一轮循环日志末尾的全量门禁行**为准。

## 已完成

> **本清单只到 L113，L114–L193 的逐轮条目在下面的「循环日志」里**（那是权威处）；
> 本清单在 L197 補記 L194–L196 三条，其余历史轮次不再回填——补 80 行的边际价值
> 低于让两处数字不一致的风险。

- [x] **L101** `015b2ca2e` + （批② docs 即本条所在提交）`test,docs(retry,version_control,models,loop)`：**A201 容错路径族 16 条缺数清零，偏支 85 → 80** —— 三支文件（`augmentor/retry.py`、`augmentor/version_control.py`、`augmentor/models/base.py`）的守卫支第一次被踩，9 例新用例全绿，全量 7290 passed / 3 skipped / exit 0 ⇒ 详见循环日志 L101
- [x] **L102** `c1734be35` + （账本批即本条所在提交）`test(api,config)`：**A202 路由/依赖/配置错误路径族 11 条缺数清零** —— health-gate 的 FileNotFoundError 防御支与 500 收尾、`/api/data/export` 500 收尾、augment/multimodal 的 HTTPException 复位支、`_config_data_roots` 降级不冻缓存、`save_config` 对坏旧文件的「继续保存」两支；8 例新增定向全绿，**全量读数被并行会话撞库污染（9 failed 全部归因对方在途改动），干净全量待补** ⇒ 详见循环日志 L102 与撞库记录
- [x] **L103** `28e3a1264` + （同上）`test(cli,preview)`：**A203/A204——version delete 动作三支（缺参 exit / 删除成功 / 版本不存在 exit）与非原生导出格式守卫支清零**，9 条缺数（version.py 98-106 + preview 177）定向覆盖实证从缺数清单消失 ⇒ 详见循环日志 L103
- [x] **L104–L113** 前端页面级组件测试从零建立（A206）：10 个页面各一份 一份 `Dashboard.test.tsx` 同型文件，逐页提交 —— L104 Dashboard `21c3ae1f5`、L105 Settings `39eaa30b0`、L106 System `51096d9fc`、L107 Augmentation `41024a400`、L108 Multimodal `5559e8768`、L109 Export `6d800cb2f`、L110 Versions `2d71358df`、L111 Analysis `b85aacf32`、L112 Quality `e0de32b94`、L113 Security `7ab30f256`；配套 `284d89e29` 抬高 vitest testTimeout。**前端全量 13 文件 / 112 例全绿，tsc 0 错，各文件 eslint --max-warnings 0 通过** ⇒ 详见循环日志 L104–L113
- [x] **L194** `1e4df8881` +（行号刷新即本条所在提交）`29b715600` +（棘轮重钉）`2b2d8fb03` `fix(augmentor)`：**B264 套件名封闭清单下沉 SDK 直构面** —— `run_tests()` 对显式未知套件名静默回落内置 default 套件，拼错的名字拿到 default 的判决不出声；`DEFAULT_SUITE_NAME` 成单一权威并共引 API 白名单，6 例回归。**本条是 L197 补记：L194 当时漏写了「已完成」行，代码与测试早已提交** ⇒ 详见循环日志 L194
- [x] **L195** `f531e62e3` `test(augmentor)`：**子进程文本解码与 locale 解耦** —— `text=True` 不传 `encoding=` 让 `test_warning_when_key_absent` 在文档约定的测试命令（`PYTHONIOENCODING=utf-8`）下必红、在 CI 上必绿；两侧同时钉编码，并建 AST 同族守门 7 例 + 行为例 1 例。全量 7877 passed / 3 skipped / exit 0 ⇒ 详见循环日志 L195
- [x] **L196** `63ef9368e` `perf(augmentor)`：**五处「同一件事算两遍」** —— sampler 的 `analyze_coverage`、`merge_files` 的第二次 `_deduplicate`、iqr 的两次排序、`diff_datasets` 的三个差集、`get_statistics` 的双趟；真实 6902 条合计 **1.73×**，44872 字节输出与 HEAD 逐键相同；新增 20 例**计数式**守卫。全量 7897 passed / 3 skipped / exit 0 ⇒ 详见循环日志 L196
- [x] **L197**（账本批即本条所在提交）`docs(augmentor)`：**文档数字对账 + 账本补记** —— 端点总数 68/70 → **71**、`quality` tag 8 → **9**、导出格式 5 → **13**、router 11 → **13**、前端页面 9 → **11**、事件循环守门家族 12 → **13** 条；routes 目录树补 `dataset_tools.py` / `system_ops.py` 两行；补 L194/L195/L196 三轮账本条目。**Python 版本口径三处不一致（3.8+ / 3.10+ / 3.11）留作维护者决策，本轮只记不改** ⇒ 详见循环日志 L197
- [x] **L198** `d510340f8` `docs(augmentor)`：**统一 Python 版本口径为 3.10+ 并上机器守卫** —— 三处旧读数（README `3.8+` / docs 表行 `3.10+（验证环境 3.13）` / DEPLOYMENT 镜像与服务器段 `3.11`）合一。四条实测依据：产品码语法下限用 `ast.parse(feature_version=)` 逐版本试得 **3.8 即全通过**、测试套件因一处 `str | None` 运行期注解需 **3.10**、本机已装 `fastapi`/`uvicorn`/`requests`/`pytest`/`coverage` 自己都声明 `>=3.10`、Dockerfile 钉 `python:3.11-slim`；不拍 3.11+ 是因为**核心依赖在 3.10 上确实装得起来**，说 3.11+ 是夸大。守卫 11 例（实推下限 + 六种会抬高地板的语法形状 + 三处文档一致 + 部署钉不低于下限 + 全仓不许出现个位数次版本的要求），注入 5 模式全绿 ⇒ 详见循环日志 L198
- [x] **L199** `c772f9a5b` `feat(augmentor)`：**封闭清单族最后一轮，7 处「枚举形参静默换语义」收口** —— `visualize_dataset` 的 `format`、`DatasetIndexer.search` 的 `method`（连 `index_used` **谎报实际方法**一起修）、`DatasetView.to_file` 的 `format`、`remove_duplicates` 的 `keep`、`get_dependencies` 的 `direction`（未知值返回 `[]`，给出**错的结论**）、`StreamWriter` 的 `mode`/`format`、`GateRule` 的 `severity`（`"warn"` 被 else 分支静默**升为 error 级**）。清单一律立为模块级常数；`INDEXER_SEARCH_METHODS` 刻意**不**共引 `SEARCH_METHODS`（5 项 vs 索引器只实现 3 项，照抄会让 fuzzy/regex 通过判据后继续静默回落）。新增 62 例 ⇒ 详见循环日志 L199
- [x] **L200** `a6e4748c6` `test(augmentor)`：**A266 收口 —— 契约面数字从 `app.openapi()` / 枚举实推** —— 端点总数（README 两处 + API 文档两处）、13 个 tag 逐个计数、导出格式清单、router 数、前端页面数、事件循环守门家族，全部变成「实推值 == 文档断言值」。每个断言走「模式必须恰好命中一处」防退化；`_one()` 命中 0 次或多次都炸。15 例，注入 7 模式全绿 ⇒ 详见循环日志 L200
- [x] **L201** `a561c0835` `fix(augmentor)`：**B266① 流式写入器不再把写崩的产物伪装成半份合法 JSON** —— 探针实测产物 `'[{"a": 1},]'`，`json.loads` 报 Illegal trailing comma。两处必须一起改（先序列化再写分隔符 + 异常穿过 with 块时不补闭合括号），注入逐半退回分别红 1/2 条正是这个理由；另把 `mode='a'` 的既有破损写成**钉住已知限制**的用例而非假装它合法。新增 9 例 ⇒ 详见循环日志 L201
- [x] **L202** `1b74bab3b` `fix(augmentor)`：**B267 预览缓存键覆盖整份样本** —— 旧键只哈希 `sample[:5]` 而 `sample` 是 `items[:preview_size]` 的全部内容 ⇒ 「前 5 条相同、第 6 条起不同」的两个数据集撞键，第二个调用方拿到**第一个的 original_data / converted_data**；同族还有命中时把缓存对象就地改完按引用返回。这是 L123 修过的那一半的另一半。L127 那条 `assert r2 is r1`（钉的正是共享可变对象）按惯例改写并写明旧断言为何错。新增 10 例 ⇒ 详见循环日志 L202
- [x] **L203** `396e4baf6` `test(augmentor)`：**A265 收口 —— 账本「已完成」清单与循环日志的进度一致性守卫**。同型事故发生过两次都靠人眼发现（L194 只写日志块漏写清单行、L50 也漏过）。判据全部从账本推导：LOG ⊆ LIST、max(LOG) 必须在 LIST、LIST 在边界以上必须是一路连到最新轮的完整尾巴 ⇒ 详见循环日志 L203
- [x] **L204** `e6c14dfaf` `docs(augmentor)`：**README 补全 37 个 CLI 子命令索引 + 三个承诺功能的入口示例**。实测 README 只覆盖 21/37，且「功能特性」承诺的 sanitize / check-leakage / audit 在 CLI 一节**一个示例都没有**。表由 parser 源码现提，「表 == parser」钉成集合相等 ⇒ 详见循环日志 L204
- [x] **L205** `dd4c1b500` `feat(augmentor)`：**B266③ 门禁分开「规则算不出来」与「算出来不及格」**。三面同改（SDK `GateReport.errored_rules` / API `GateReportResponse` / 守卫）——只改 SDK 那一面会让新键被 FastAPI 按 response_model 静默裁掉。契约守门第一次跑就抓到第三处手抄副本，改成从 `to_dict()` 实推 ⇒ 详见循环日志 L205
- [x] **L206** `2fc72b8f2` `feat(augmentor)`：**B266④ 失败明细从「只写不读」变成有出口**。`_process_errors` 全仓零读者，且跨调用累积不清理。**顺带修掉第三处（实跑才发现）**：串行支路的 except 压根不记明细 ⇒「失败明细有出口」此前只在并行档成立 ⇒ 详见循环日志 L206
- [x] **L207** `5461ba6b1` `feat(augmentor)`：**B266⑤ 管道阶段失败有出口 + 新增 `run_strict()`**。`stop_on_failure` 默认 False，阶段 B 炸了阶段 C 拿到的是阶段 A 的输出，整条管道少跑一环却毫无信号。不动既有默认值，只补可见性 ⇒ 详见循环日志 L207
- [x] **L208** `b949f7929` `feat(augmentor)`：**B266② 追加 JSON 数组改成拒绝**。实测产物 `'[{"a": 1}]{"b": 2}'`（json.loads 报 Extra data）——非法 JSON 而调用方以为写成功了。三条候选都量过，取「拒绝」 ⇒ 详见循环日志 L208
- [x] **L209** `0778cb240` `feat(augmentor)`：**B265 备注收口 —— `dependency_type` 纳入封闭清单**。实测全仓零分支读取、测试只用过清单成员 ⇒ 收紧无行为影响，所谓「契约变更」的风险在可达面上为零 ⇒ 详见循环日志 L209
- [x] **L210** `06e253bc9` `docs(augmentor)`：**§3.3 收口 —— dataset/system 两组 27 个端点补人读字段说明**。守卫分两层（总览层全路径集合相等 / 详细层恰好覆盖两组），且守卫自身两次被实测抓出漏洞后修正 ⇒ 详见循环日志 L210
- [x] **L211** `deb9bb3c9` `feat(augmentor)`：**前端消费面收口 —— 五只未接线服务函数全部接进页面**。KNOWN_UNWIRED 白名单清空；顺带修掉接线守门自身的失查（语料面含 `*.test.tsx`，替身字符串被当成接线）⇒ 详见循环日志 L211
- [x] **L212** `9abc72c67` `test(augmentor)`：**账本标尺的谱系口径换成全祖先 topo-order**。换序前先逐档重量六档读数全部复现才动刀 ⇒ 详见循环日志 L212（与 L213 同批提交）
- [x] **L213** `9abc72c67` `test(augmentor)`：**符号链接能力交叉对账**。第一版「按探针分臂」不可证伪，撤掉重写 ⇒ 详见循环日志 L213
- [x] **L214** `ec0668115 + bccfeff85 + e6b67238a` `feat(augmentor)`：**A138 ②③ 档接线第一批 —— 五只键 / 六条端点支路，判据一律取行为差**。权威序定稿「请求参数 > 配置键 > 内置默认」；`enabled` 三只按实测撤回（端点门 = 改契约），process 的扩展名字段本轮自己造又自己拆；收口批被自己的一句未实测断言推翻 ⇒ 详见循环日志 L214
- [x] **L215** `2f994fbc7` `feat(augmentor)`：**`enabled` 族第一批 —— 出厂四只 `true` 键接进管道阶段入口**，零读者名单 16 → 12。默认值换成 `None` 哨兵之后调用参数仍是最高一档；顺带把两只新有读者的键接进 `GET /api/config` 的只读回显面 ⇒ 详见循环日志 L215
- [x] **L216** `af03b67b3` `feat(augmentor)`：**A138 ③ 档清空 —— vector 五键接进新建的语义消费层**，两条只读端点 + 出厂 `false` 的自动建库闸 ⇒ 零行为漂移。分区守卫当场抓出 `rag.chunk_*` 是名单历史漏计；读 chromadb 时撞出本轮新造成的 `AttributeError`／GET 落盘缺陷并补能力闸 ⇒ 详见循环日志 L216
- [x] **L217** `d171524a0` `feat(augmentor)`：**multilingual 四键接进产品入口**（新建 `augmentor/language.py` + 画像/翻译两条端点，`enabled` 闸在读盘之前），并把 B5 那把住在 `Temp/` 的死键尺子升成仓内守卫。普查现量 69 键 / 47 只有读者 / 23 只零读者，`config.*.enabled` 16 → 10 只。顺带撤掉本层那道与 SDK 重复的语言准入闸（注入显示摘掉它全绿 ⇒ 两存必有一盲），SDK 吞异常导致的「失败冒充成功」新立 A270 ⇒ 详见循环日志 L217
- [x] **L218** `334e94a40` `test(augmentor)`：**死键普查的「理由」补上机器可证伪维**——20 只零读者键逐键挂一条由 `inspect.signature` 与路由模型 AST 验真的旋钮证据（五种形状），外加散文一致性两规则与两把尺子的正控制；实测 L217 那批理由有 **8 条与证据矛盾**（我在提交信息里数成 7 条，那个数本身也是散文数）。据此订正 A271 的旋钮口径、新立 A273（管道构造 `self.expander` / `self.tracker` 之后产品侧从未调用）⇒ 详见循环日志 L218
- [x] **L219** `fbf5052ec` `feat(augmentor)`：**`evaluation.metrics` 接进 `/api/dataset/evaluate` 的省略支路**（权威序 请求 > 配置 > SDK 默认；出厂值与内置回落值逐字同 ⇒ 零漂移，判据取「改配置 ⇒ 指标集合跟着变」再加一整份字典逐值相等的等价钉）。接线顺带把两格死角第一次变成可达：空清单被 SDK 的 `metrics or [内置三只]` 静默补成三项（本层改 400），白名单过去只判请求侧、现在判生效值。普查现量随档位改：有读者 47 → 48、零读者 23 → 22、归档待处置 20 → 19；同节 `reference_field` **接不动**（请求默认是硬值 `"output"`，`or` 永远落不到配置），立 **A274**；L218 那句假理由计数按机器比对从 7 订正为 **8** ⇒ 详见循环日志 L219
- [x] **L220** `68b929bb8` `feat(augmentor)`：**A274 的「接不动」用 `None` 哨兵解开——`evaluation.reference_field` 接进 `/api/dataset/evaluate`，并按对称口径新补 `evaluation.generated_field`**（权威序 请求 > 配置 > 内置默认；两只键出厂同为被替换的硬编码值 `output` ⇒ 零漂移，前提本身由三臂机器钉住）。坏字段名两处出声（加载期 `require_string` + 门口逐条带下标 400），请求侧空串与省略不可区分、落回配置，与 L219 的 `metrics` 语义同式。普查现量：schema 69 → **70 键**、有读者 48 → **49**、零读者 22 → **21**、归档待处置 19 → **18**（非闸门键 9 → 8）。既有面五处按现量重钉（含注册表门铃 69 → 70 与两支写死 29 的测试名改为点名归因），注入取证 **八臂全部有牙齿**（5/4/7/2/1/6/4/5 红，其中一臂是普查尺子自己看见读者消失）。门禁 **8224 passed / 11 skipped / 0 failed / 99.80%**，用例增量 `+28` 用「HEAD 临时换入再逐字节复原」的对照跑法逐文件实测。新立 **A275**（README 配置表与 schema 无机器对账）⇒ 详见循环日志 L220
- [x] **L221** `5d37639ee` `feat(augmentor)`：**A270 收口——逐条翻译失败加回执面**：`batch_translate` 从「返回清单」改成「返回报告」（`failed` / `failed_indices` / 本批全量 `errors`），聚合层把批内下标折算成全数据集坐标并按 20 截样本，`TranslateResponse` 声明回执四键。部分失败**仍是 200**，诚实读数在响应体而不是状态码。普查面零变动（本轮不新增键）；门禁 **8239 passed / 11 skipped / 0 failed / 99.80%**，注入八臂各有牙齿（3/1/1/2/1/7/6/4 红），用例增量 **+15**（三个既有文件逐文件实测计数一字未变）⇒ 详见循环日志 L221
- [x] **L222** `932c5885a` + `c6209ab67`（归因订正）+（账本批即本条所在提交）`test,docs(augmentor)`：**A275 收口——配置面三侧同源对账守卫落成机器**（`docs/README.md` 配置表 / `fields(AppConfig)` / `config.yaml` 三角互核）。README 表格 16 行 34 键 → **22 行 70 键**，权威只有一份：节与键全从 `fields()` 实推，测试侧唯一手写名单 `META_ROWS`（`models` / `app`）逐员机器验真。`config.yaml` 那一侧的判据被文件自己纠正过一次——不是「与 schema 逐节全等」而是**整节写全或整节注释列全**（`logging` 三键刻意不落地，写出来才装配）。用最终版尺子对 HEAD 那份文档量出的真漂移是 **21 红 / 38 绿**：5 个整节缺席、13 节只列半节、恰好列全的只有 `export` 与 `multilingual`。注入七臂各有牙齿（4/2/3/3/5/2/3 红），A5「schema 新增一只 init 键」复现的正是 L220 那一步的形状。顺带量出一格普查盲区：`config.yaml` 的 `app` 三键既无 `AppConfig` 字段、又被校验器 `META_TOP_SECTIONS` 免检 ⇒ 「只覆盖 schema 键」的死键普查**结构上看不见**它（零读者没有读者这件事仓内三处早有记录，历史处置是降为可选；新的是盲区与处置未立账）⇒ 立 **A276**。l97 / l98 两把参数化棘轮按现量回填（CALLS 399→402、盲面 111→114、plain 57→58、derived 36→38、盲面用例 106→109）。门禁 **8298 passed / 11 skipped / 0 failed / 99.80%**，用例增量 `+59` 与新文件支数吻合 ⇒ 详见循环日志 L222
- [x] **L223** `6c53d986c` +（账本批即本条所在提交）`test,docs(augmentor)`：**A277 收口（同轮立同轮关）——`docs/API.md` 逐端点「人读字段提示」与 OpenAPI 契约面落成机器对账**（`tests/unit/test_api_doc_field_hints_l223.py`，23 支）。详细表 31 格里提取到 **58** 个标识符形状的字段声称，其中 **40** 个（69%）直接落在该端点的契约面上；其余由四档**有位置的**证据接住（作用域内字面量 / 定义名 / 配置键 / CLI 选项）加一份封闭 JSON 词表。字面量档按「路由文件 + 两跳 import 闭包」限作用域，**两跳这个数字是量的**：一跳会把 `stratify_distribution` 误判成无据，三跳时 convert 的闭包涨到 70 个文件、`output_path` 反而有据 ⇒ 判据失去区分力。尺子首跑报出 3 格无据，**其中 2 格是尺子自己的 bug**（相对导入层级换算写错，`augmentor` 包内部的 `from .x` 一条都没解析进闭包），修完只剩 `output_path` 那一格，也就是本轮要修的真文档缺陷 4 处（总览散文 1 + 详细表 3；字段实为 `output_file`，`split` 是 `output_dir`，`output_path` 只是路由局部变量名）。`docs/README.md` 与 `docs/API.md` 全文再无该名字。注入九臂各有牙齿（2/2/6/1/7/1/3/1/3 红），其中 A1/A2 成对：同一处文档缺陷，作用域退回全仓后**主判据转绿** ⇒ 那档作用域是承重的。本轮**零 parametrize 新增** ⇒ l97 / l98 两把棘轮读数一格未动，未回填。门禁 **8321 passed / 11 skipped / 0 failed / 99.80%**（195.43 s），用例增量 `+23` 与新文件支数吻合。顺带量到 `additionalProperties: true` 的自由子对象（如 `ImpactResponse.before/after`）在契约面上给不出键名 ⇒ 立 **A278** ⇒ 详见循环日志 L223
- [x] **L224** `08bc4f0c8` +（账本批即本条所在提交）`test,docs(augmentor)`：**A279 立账——后端 75 条操作与前端服务层 43 条调用形状的消费对账落成机器**（`tests/unit/test_api_endpoint_consumption_l224.py`，26 支）。两侧权威各一份：契约面从 `app.openapi()` 实推（**75 条操作 / 71 条路径**），前端面从 `web/src/services/api.ts` 的 `api.<verb>('路径')` 形状实推（**43 条**），路径前缀从同一份源码的 `baseURL` 读，不在测试里写死第二份。**缺口 32 条**落成名单且与实测集合**相等** ⇒ 只会随接线变短，新增一条没接的端点当场红；32 条里 **26 条有 CLI 子命令入口**（映射去重后 **21** 个名字，逐个由 parser 的 `add_parser` 名单验真）、**6 条两边都没有**（`dataset/evaluate`、`impact`、`language-profile`、`semantic-search`、`translate`、`vector-status`）。query 一维量出**两处来源**：写在路径串里的 1 条 + axios `params:` 对象里的 4 条 / 5 个键，只认前者会漏掉 `page_size` 这个「TS 侧改名后的对外名」；归属判据用「最近的前一个调用」，被实测否掉的备选是往后看固定 400 字符（候选从 4 涨到 7，多出三桩冤案）。去歧义同样有实测：43 条形状里恰 1 条（`GET /api/versions/history`）同时落在字面量与参数化两条路由上，绑定只认占位符最少那一档，并拿 `TestClient` 的 200 + 响应体形状证明赢的那条确实是它——依据是运行时行为，不是匹配规则。注入十臂各有牙齿（1/4/6/3/2/2/1/7/2/1 红），其中 A0 只响 1 支：真把两条路由的声明顺序对调，**只有那条运行时判据会响**。本轮**零 parametrize 新增** ⇒ l97 / l98 两把棘轮读数一格未动，未回填。门禁 **8347 passed / 11 skipped / 0 failed / 99.80%**（196.80 s），用例增量 `+26` 与新文件支数吻合。顺手量到 8 个路由文件把下游异常一刀切收敛成 500 ⇒ 立 **A280** ⇒ 详见循环日志 L224
- [x] **L225** `e7dfa43ab` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第一批接线——6 条「两边都没入口」里先接 3 条，消费面 43 → 46**（`language-profile` 与 `evaluate` 进数据分析页、`vector-status` 进系统状态页；服务层三只 fetchers 加三个响应类型，后端字段名与响应形状一字未改，`include_details` 在页面上显式送 `false`——契约默认是 `true`，见 `api/routes/dataset_tools.py:231`）。守卫按实测回填四格：调用形状 **43 → 46**、缺口名单 **32 → 29**（划掉的正是这三条）、已消费 **43 → 46**、`NO_ENTRY` 队列 **6 → 3**（剩 `impact` / `semantic-search` / `translate`）；`CLI_ENTRY` 的 26 条 / 21 个名字一格未动，因为本轮只动前端。A279 的三件收口判据齐：名单与实测相等且变短、`api.wiring.test.ts` 的 `KNOWN_UNWIRED` 仍为空、每员覆盖成功与失败两路（前端 vitest **13 文件 127 支，+8**，`tsc --noEmit` 与 `eslint --max-warnings 0` 干净）。注入五臂各有牙齿（3/1/3/1/1 红）：A0 把服务层路径拼错 ⇒ 消费守卫响 3 支（幽灵方向 + 名单相等 + 名单规模）；A1 只 import 不调用 ⇒ 接线守门响 1 支；A2 删掉整张向量卡 JSX ⇒ 该页响 3 支且原有 4 支不动；A3 把「双侧必填」削弱成单侧 ⇒ 守卫用例响 1 支；A4 摘掉分布表的 `.sort` ⇒ 排序用例响 1 支。**A4 那一支是先量出假绿才补的**：画像 fixture 的插入序原本恰好与条数序同向，摘掉排序照样绿，改成故意反向之后 A4 才响。顺手量两件事，都按「不动，只立账」处置：**A281**（45 只带类型的 fetchers 里 40 只的 TS 响应类型字段集与契约 200 响应的属性集逐字相等，3 只缺字段——`AppConfig` 少 `versioning` / `visualization`、`UpdateConfigResponse` 少 `ignored_keys`、`StatusResponse` 少 `model_retry_count` / `model_retry_wait_seconds`，方向一致都是「后端在回、前端类型上不存在」）、**A282**（`prettier --check src` 的仓内基线本来就是红的：32 个文件不合规，把本轮碰过的 5 个文件从 `HEAD` 取到仓外用同一份配置复查同样全红 ⇒ 本轮没引入漂移，而 CI 里根本没有 format 门）。本轮**零 parametrize 新增** ⇒ l97 / l98 两把棘轮读数一格未动，未回填；后端用例总数与 L224 持平（**8347 passed / 11 skipped / 0 failed / 99.80%**，198.98 s），增量全在前端那一侧 ⇒ 详见循环日志 L225
- [x] **L226** `dfb9ab0ae` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第二批接线——`search` / `validate` / `auto-config` 进页面，消费面 46 → 49**（内容检索进数据分析页、结构校验进质量中心新 tab、按数据推荐配置进配置管理页；三只 fetchers + 三只响应类型 + 三处页面入口 + 三页各补成功/失败两路，后端字段名与响应形状一字未改）。守卫按实测回填五格：调用形状与已消费 **46 → 49**、缺口名单 **29 → 26**、`CLI_ENTRY` **26 条/21 名 → 23 条/18 名**、`NO_ENTRY` 仍 **3**。新立跨语言对账守卫 `test_wired_dropdown_choices_l226.py`（23 支）：两张下拉清单与 `SEARCH_METHODS` / `VALIDATION_PRESETS` / `PRESET_RULES` 逐项相等、CLI 的 `--method` choices 首次入尺、label 写死的数字真读规则表验证、端点面回执 5+3 档全 200 与清单外两档回字符串 detail；五档注入臂红 3/1/1/1/4 格，跑完按字节还原。**明确拒接两员并记账**：`GET /api/system/dependencies`（与 `/api/status` 的 dependencies 同数据两面，接了只涨指标）与 `POST /api/dataset/translate`（整份译文只在响应体、无 `output_file` 可落盘 ⇒ 另立 **A283**）。顺手修掉去重卡片「启用水重」错字。census 两把棘轮按实测重钉（CALLS 402→404、READABLE 288→290、盲面 114 未动）。门禁：后端 **8370 passed / 11 skipped / 0 failed / 99.80%**（189.99 s），前端 **13 文件 / 140 支全绿**（+13）、`tsc` 与 `eslint` 均干净 ⇒ 详见循环日志 L226
- [x] **L227** `835d15d4d` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第三批接线——`impact` / `semantic-search` 进数据分析页，消费面 49 → 51，并把「抄进页面的默认值」钉回请求模型**（`NO_ENTRY` 队列里最后两只「SDK 做完、web 与 CLI 都没人取」的缺口，只动 `web/src`，后端字段名与响应形状一字未改；`translate` 留给 A283）。守卫按实测回填四格：调用形状与已消费 **49 → 51**、缺口名单 **26 → 24**、`NO_ENTRY` **3 → 1**（只剩 `translate`）、`CLI_ENTRY` 的 **23 条 / 18 名一格未动**（本轮只动前端）。新立跨语言对账守卫 `test_impact_semantic_wiring_l227.py`（20 支）：**这两只没有「字段名权威常数」可引**（l226 那种封闭清单在这里不存在，`["instruction", "input", "output"]` 只是 SDK/CLI 里五处手写字面量），权威换成三处来源——请求模型 `model_fields[...].default`、真实回执的 `gains` 键集、`AugmentationMetrics` 的 dataclass 字段；页面四档初值逐个与契约默认同值，服务层不写第二套默认。新暴露两档此前任何产品面都调不到的旋钮（`text_key`、`min_scale_gain`），`前 K 条` 的下界按实测从 1 放宽到 0（`top_k=0` 是合法的「只要回显、不要命中」档，端点 200 且 `results` 为空）。顺手量到 `AugmentationMetrics.unique_instructions` 是误名（`augmentor/impact.py:76` 数的是 `len(set(texts))`，`texts` 取自 `text_field`）⇒ 标签按诚实口径写「唯一值数（按所选字段）」并立 **A284**，那五处重复字面清单立 **A285**。census 按实测重钉（CALLS 404→405、READABLE 290→291、盲面 114 与 l98 档位和未动——本轮唯一新增的 parametrize 值位是内联字符串清单）。注入十二臂各有牙齿（Python 面 2/2/2/3/1/1/3，vitest 面各 1）：**第一批有五臂被取证脚本自己读成「空转」，那是 ANSI 色码截断了汇总行，剥色重跑才是真读数，假读数没有入账**。前端 **13 文件 / 149 支全绿**（+9：Analysis 15 → 24）、`tsc` 与 `eslint` 均干净；门禁：后端 **8390 passed / 11 skipped / 0 failed / 99.80%**（188.88 s，`+20` = 新守卫 20 支）；账本批复跑时 A184 那把注释引用尺子把账本第一版里一个「尚不存在的新模块名」红了两支 ⇒ 改口不指名，阈值一格未动；同轮立 **A286**（比值型计时守卫在负载下会假红：那一次全量 3 failed / 8387 passed、比值 4.38 对阈值 3.0，同条用例隔离复跑四次全绿、整轮 257.50 s 比 188.88 s 高 36%） ⇒ 详见循环日志 L227
- [x] **L228** `ed2409452` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第四批接线——`stats` / `features` / `compare` 三条数据集操作进数据分析页，消费面 51 → 54**（字段级统计、特征普查、双集对比三张新卡 + 三处页面入口，只动 `web/src`，后端字段名与响应形状一字未改）。守卫按实测回填四格：调用形状与已消费 **51 → 54**、缺口名单 **24 → 21**（划掉的正是这三条）、`CLI_ENTRY` **23 条 / 18 名 → 20 条 / 15 名**（三条各有 CLI 子命令，所以这轮动的正是这一半）、`NO_ENTRY` 仍 **1**。新立守卫 `tests/unit/test_stats_features_compare_wiring_l228.py`（30 支），**这批换了一条判据**：l226 量「下拉 vs Python 封闭常数」、l227 退到「页面发得出每档都被端点接受」，这一批连那条都不够用——实测这两档旋钮给错值时后端**不出声**：`fields:["nope"]` ⇒ **200** 且回执里多出一整只十二键全零的**幽灵字段**（`summary` 还说「包含 1 个字段」），`key_fields:["nope"]` ⇒ **200** 且把两份毫不相干的数据回成 `common_items 1 / similarity_score 1.0`，而同一条数据默认键回的是 `0 / 0.0`（真话）⇒ **幽灵匹配键是假阳性，比零更贵**。因此页面这两处不许出现任何手写字段名，候选只能来自上一次 200 回执的键集，由 `TestCandidatesComeFromTheReceipt` 守。**又一条被探针推翻的想象**：初稿断言「换匹配键会让逐条值差异从空变成逐条列出」，读了 `augmentor/comparison.py:217` 与 `augmentor/compare_enhanced.py:289` 才知道 `_compare_fields(items_a, items_b)` 压根不接 `key_fields`，明细恒按 `instruction` 配对、条目键固定 `{instruction, value_a, value_b}`，`key_fields` **只**动 `metrics` ⇒ 订正落在 `types/api.ts` 与页面注释两处，并钉成正向不变量用例。顺手立两笔：**A287**（`similarity_to_a` / `similarity_to_b` 全仓只有 dataclass 默认与 `to_dict()` 两处出现、无赋值点 ⇒ 真数据上恒 `0.0`，契约面死面，页面不渲它）、**A288**（B 侧 0 条时 `size_ratio` 在 SDK 是 `float('inf')`（`augmentor/compare_enhanced.py:190`），经 JSON 到前端是 **`null`** ⇒ TS 类型与页面读数按线上真相写成 `number | null` 并显示「—（B 侧 0 条）」）。census 两把棘轮按现量重钉（CALLS 405→406、READABLE 291→292、盲面 114 与 l98 档位和未动——本轮唯一新增的 parametrize 值位是内联字符串清单）。注入八臂（Python 面 6 臂各点名单支、vitest 面 2 臂各单支）逐臂 rc=1 且输出含 `failed`，三个被改文件逐臂字节复原 + 收尾重新验盘一致。前端三门禁：**`tsc` 干净 / `eslint` 干净 / vitest 13 文件 149 → 163 支全绿**（Analysis 24 → 38）。门禁：**8420 passed / 11 skipped / 0 failed / 99.81%**（238.55 s，`+30` = 新守卫 30 支）。一条「尺子咬到我」的实测：A184 那把注释引用尺子把我写的平局判据行号（259）红了两支——259 是空行，那句判据实落在 **260**（`augmentor/comparison.py:260`）；两处同值就地改口、净行数零（`git diff --numstat` 复核），阈值一格未动 ⇒ 详见循环日志 L228
- [x] **L229** `820821fda` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第五批接线——`convert` / `merge` / `split` 三只写盘操作进数据管理页，消费面 54 → 57**（格式转换、合并、分割三张新卡；**只动 `web/src`**，后端字段名与响应形状一字未改；唯一一处产品面改动是把转换回执补成「响应模型的每一格都读得出来」，同行追加、`git diff --numstat` 复核净行数零）。守卫按实测回填四格：调用形状与已消费 **54 → 57**、缺口名单 **21 → 18**、`CLI_ENTRY` **20 条 / 15 名 → 17 条 / 12 名**（三条各有 CLI 子命令，所以动的又是这一半，与 L226 / L228 同形）、`NO_ENTRY` 仍 **1**（`translate`，归 **A283**）。新立 `tests/unit/test_transform_wiring_l229.py`（**55 支**）与渲染面 `web/src/pages/DataManagement.transform.test.tsx`（**13 支**）。**这批的判据换到「写边/读边两张格式清单各只有一个权威」**：转换目标下拉逐项等于 `OUTPUT_FORMAT_CHOICES`（`augmentor/converter.py:91`），源格式下拉复用上传那份 `UPLOAD_FORMAT_CHOICES`（读边比写边多一个 `xls`，`augmentor/converter.py:56` 的 `EXCEL_TARGET_NAMES` 正是那道差的由来），端点拒 `xls` 时自己枚举的十二档与页面清单逐项相等 ⇒ 页面编不出第十三档。八臂「初值 == 契约默认」逐个对账，`seed` / `source_format` 从空开始（留空即不发那一格），服务层不写第二套默认（全文只有一处 `|| undefined` 哨兵）。**又两条被探针推翻/证实的静默形状立成账**：**A289**（分割两条支路判据不同：容差默认支 0.01、分层支 1e-6，负数只有分层支拦，两句判决文案也不同）、**A290**（合并端点四档旋钮只接一档、SDK 六键只上线五键，而响应模型的 docstring 写着「一一对应」）、**A291**（空 `source_format` 的判决缺主语）。**还有一条被探针推翻的想象**：初稿断言「输出写绝对路径 ⇒ 403」，实测在白名单根内是 **200**，根外才是 403，`..` 则是 400 ⇒ 判据改成三档分开钉。census 两把按现量重钉：CALLS 406 → **411**、READABLE 292 → **293**、盲面 114 → **118**；l98 档位和同步 **118**（opaque 10 → 11、ids-derived 8 → 11，plain / derived 未动）、盲面用例 109 → **113**、豁免用数 17 → **18**。注入八臂各有牙齿（Python 面五臂：回执吞掉 `output_file` / 服务层路径拼错 / 页面往目标清单加 `xls` / 契约默认改一格 / 删掉诚实口径那句；vitest 面三臂：目标清单多一档 / `train` 初值替契约编默认 / 删掉分支判据差那句），逐臂 rc=1 且输出含 `failed`，被改的三个文件逐臂字节复原 + 收尾重新验盘一致；**中间读数不美化**：vitest 探针 v1 用「输出尾部 700 字含 failed」判红，把 vitest 的失败摘要截掉了，三臂全被读成 NOT-RED（其中一臂的输出明明带着红），订正成对全量输出判之后才是真读数。前端三门禁 `tsc` 干净 / `eslint` 干净 / vitest **14 文件 163 → 176 支全绿**。门禁：**8475 passed / 11 skipped / 0 failed / 覆盖率 99.81%**（244.78 s，与 L228 的 8420 相减 `+55` = 新守卫 55 支，接线本身零后端用例增量）；六把账本与文档守卫 **149 passed / 8 skipped** ⇒ 详见循环日志 L229
- [x] **L230** `32028f73b` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第六批接线——`sample` / `aggregate` / `rag` 三只写盘操作进数据管理页，消费面 57 → 60**（采样、多源聚合、转成 RAG 检索格式三张新卡；**只动 `web/src`**，后端字段名与响应形状一字未改）。守卫按实测回填四格：调用形状与已消费 **57 → 60**（75 条的 80%）、缺口名单 **18 → 15**、`CLI_ENTRY` **17 条 / 12 名 → 14 条 / 9 名**（三条各有 CLI 子命令，动的又是这一半，与 L226 / L228 / L229 同形）、`NO_ENTRY` 仍 **1**（`translate`，归 **A283**）——**至此全仓六只会落盘产物的端点（`convert` / `merge` / `split` / `sample` / `aggregate` / `rag`）在页面上都有了入口**。新立 `tests/unit/test_sample_aggregate_rag_wiring_l230.py`（**74 支**）与渲染面 `web/src/pages/DataManagement.sample.test.tsx`（**19 支**）。**这批又换了一档判据**：这三只端点背后的两个后端权威——`DatasetOperations.sample` 的三种 `method`、`DataAggregator.aggregate` 的四种 `strategy`——**在仓内都没有可引常数**（不像 L226 的 `SEARCH_METHODS`、L229 的 `OUTPUT_FORMAT_CHOICES`），只有 if/elif 分发臂与散落的字面量；照抄字面量等于自造第二套权威，所以守卫改成**读 AST 数分发臂**（`dispatch_arms()` 逐条读 `config.method == 「字面量」` / `strategy == 「字面量」` 的比较臂），**一条都数不到就抛 `LookupError` 而不是当成干净**（数不到 ≠ 没有）。八臂「页面初值 == 契约默认」逐个对账，服务层三只 fetchers 全文无第二套默认（只有转换卡那处 `|| undefined` 哨兵仍在，而这一批实测**送空串与省略等价**，所以那一处是形状卫生不是躲判决，与 **A291** 那处动机不同）。**三处被自己的夹具推翻的中间读数**：① 初稿断言「SDK 只报 `conflict_count`、线上只报 `conflicts`」，实测两套键集**各有两格对方没有**（SDK 多 `total_count` / `conflict_count`，线上多 `output_file` / `aggregated_count`）⇒ 改写成「两套手维护的键集已经漂移」并立 **A293**；② 初稿把 RAG 默认档引成 `RagConfig.model_fields`，实测真名是 `RAGConfig` 且是 dataclass（`augmentor/config.py:533`）⇒ 改引 `__dataclass_fields__` 并删掉那条永远走不到的双分支；③ 初稿断言「`target_size` 省略会 400」，实测省略是 **200** 并走 `DEFAULT_TARGET_SIZE = 100`（回落点 `augmentor/aggregator.py:159`），只有 weighted 那一支真的会被它咬 ⇒ 立 **A294**。**顺手订正 `docs/API.md` 两句假话**（aggregate 行的「省略会 400」与 rag 行的默认档），两处都是**同行追加、`git diff --numstat` 复核净行数零**，并新立四支「文档行必须说端点真的会做的事」的用例把订正钉住（判据咬到自己那一处：新用例的第一版被我自己写进 API.md 的那句订正说明红了一次）。**六笔新账全部只立不做**：**A292**（`ratio` 无判决，三种方法三种结局）、**A293**（聚合端点构造的是无参 `DataAggregator()`，三只旋钮不可达 + 两套回执键集漂移 + `conflicts` 整份上线）、**A294**（`target_size` 只咬 weighted）、**A295**（默认格式有第三处字面量 `langchain`）、**A296**（分层抽样 `value[:10]` 静默折叠分组）、**A297**（`consistent` 把同源内部分歧算成冲突且那些条目永不落地）。census 两把按现量重钉：CALLS 411 → **419**、READABLE 293 → **301**、盲面 **118 一格未动**（本轮八处新增 parametrize 的值位全是内联字面量清单 ⇒ 全落可读面）；l98 豁免用数 18 → **23**（五支新用例的值位来自 AST 分发臂与请求模型字段集，逐支带理由入册；豁免名单变长不是放松，判据仍逐行判值位形状）、档位和与盲面用例 **118 / 113 未动**。注入九臂各有牙齿（Python 面六臂 2/5/3/1/1/1 红，基线 70 passed；vitest 面三臂 2/1/1 红），逐臂 rc=1 且输出含 `failed`，被改文件逐臂字节复原 + 收尾重新验盘一致（sha256 `8b17a662` 起）；**其中一臂被拦在写盘之前**：W3 的锚点 `完全空转` 在文件里出现两次（模块注释与卡片文案），唯一性断言当场响 ⇒ 磁盘未被触碰，换成更长的锚点后才注入成功——这正是注入器该有的反应。前端三门禁 `tsc` 干净 / `eslint` 干净 / vitest **15 文件 176 → 195 支全绿**。门禁：**8549 passed / 11 skipped / 0 failed / 覆盖率 lines 99.93%、branches 99.39%**（179.78 s，与 L229 的 8475 相减 `+74` = 新守卫 74 支，接线本身零后端用例增量）；六把账本与文档守卫 **149 passed / 8 skipped** ⇒ 详见循环日志 L230
- [x] **L231** `d90148b38` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第七批接线——备份那一族四只端点进系统状态页，消费面 60 → 64**（一张「数据备份」卡把 `GET /api/system/backups`、`POST /api/system/backups`、`POST …/{backup_id}/restore`、`DELETE …/{backup_id}` 四只全接上：源文件候选取自 `/api/data/list` 也允许手填路径，恢复目标手填，删除走 Popconfirm 二次确认，回执把响应模型每一格都读出来，失败一律显示服务端那句判决）。守卫按实测回填四格：调用形状与已消费 **60 → 64**（75 条的 85%）、缺口名单 **15 → 11**、`CLI_ENTRY` **14 条 / 9 名 → 10 条 / 8 名**（`backup` 这一个子命令此前吃掉四条操作，本轮**整名出名单**；一次减 4 条是现量最大的一批）、`NO_ENTRY` 仍 **1**（`translate`，归 **A283**）。新立 `tests/unit/test_backup_wiring_l231.py`（**33 支**）与渲染面 `web/src/pages/System.backup.test.tsx`（**11 支**）。**这一批同时改了一处产品面**：`DatasetBackup.backup()` 的 `name` 直接决定落点，而它来自调用方——仓外探针实测 `name="../../escaped"` 打创建是 **200** 且真把文件写到备份目录之外（白名单只管到 `backup_dir` 那一格），补门之后同一条请求 **400**「备份名称不能指向备份目录之外」、白名单外零文件；`name` 带 `/` 仍在备份目录内建子目录（实测 `sub/deep` ⇒ `.backups/sub/deep.json`），那一档照旧放行。这处 +4 行把 `augmentor/backup.py:188` 平移到 **`:192`**，两处历史引用（本账 A277 行与 L223 日志块）同轮重钉、`git diff --numstat` 复核净行数 **0**。新立七条 **A298~A304**（同名备份在索引里留两行且删一次全清、校验和从不拦截恢复、两只 SDK 函数没有 HTTP 入口、「默认备份目录」仓内三份且端点那份与前两份不是同一个目录、前端没有送 `X-API-Key` 的通道、`backup()` 里两次 `datetime.now()`、候选清单窄于白名单）。**A277 那道「全文不许出现 `output_path`」的禁令按现量收窄而不是摘掉**：那个名字在 `POST …/restore` 的**响应**里是真的，收窄后 dataset 七条写盘族一行仍不许出现、全文只许 restore 那一行（注入臂 I5 实证原缺陷照旧喊红）。注入六臂逐臂点名喊红（**3 / 1 / 6 / 1 / 2 / 3** 支），被改的六个文件逐臂字节复原、收尾重新验盘并断言哈希与进入时一致。本轮**零 parametrize 新增**（两支新文件里 `parametrize` 出现 0 次）⇒ l97 / l98 两把普查棘轮读数一格未动；哈希槽那把尺子只读**第一本账**（`LEDGER = OPTIMIZATION_LOOP.md`），本轮动的是第二本 ⇒ 六档现量（15/24/15/69/占位 0/R6 白名单四格）同样一格未动。门禁：**8582 passed / 11 skipped / 0 failed / 覆盖率 lines 99.93%、branches 99.39%**（180.57 s，与 L230 的 8549 相减 `+33` = 新守卫 33 支）；前端 `tsc --noEmit` 与 `eslint .` 均 rc=0，`vitest run` **16 文件 / 195 → 206 支全绿** ⇒ 详见循环日志 L231
- [x] **L232** `d17cd5b56` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第八批接线——数据集血缘那一族三条进系统状态页，消费面 64 → 67**（第四张卡「数据集血缘」把 `GET` / `POST /api/system/dependency/datasets` 与 `GET /api/system/dependency/graph` 三条全接上：登记表单（名称 / 源文件候选取自 `/api/data/list` 也允许手填 / 描述 / 标签，中英文逗号都切成数组且空段被丢）、已登记列表按服务端原样渲染（空描述与空标签各有可见占位）、图的三格计数与 `issues` 逐条列出）。守卫按实测回填五格：调用形状与已消费 **64 → 67**（75 条的 89%）、缺口名单 **11 → 8**、`CLI_ENTRY` **10 条 / 8 名 → 7 条 / 7 名**（`dependency` 一个子命令此前吃掉三条操作，本轮**整名出名单**）、`NO_ENTRY` 仍 **1**（`translate`，归 **A283**）、`params` 归属 **8 → 11 条 / 去重后 7 → 8 个键名**（`registry_path` 第一次进这一维）。新立 `tests/unit/test_lineage_wiring_l232.py`（**39 支**）与渲染面 `web/src/pages/System.lineage.test.tsx`（**8 支**）。**这一批同时改掉一处产品缺陷（A305）**：`POST …/dependency/datasets` 的 `description` / `tags` 改前**回显即丢**——端点在 `register_dataset()`（内部已落盘）**之后**才改返回对象，改的是内存，于是响应带着这两格而随后的 `GET` 与磁盘 `datasets.json` 里都是空；改成随登记一起传下去之后三处一致（仓外探针实测响应 / GET / 磁盘都带着 `描述B` 与 `['t1','t2']`）。**只看响应的那把尺子在改前也是绿的**，所以新守卫那一条判据只读磁盘与 GET。`api/routes/system_ops.py` 净 **+1 行**（6/5），而这一处之上的历史行号引用（150/295/397/398/400/415）无一平移，`git diff --numstat` 复核；`docs/API.md` 与 `README.md` 各**净 0 行**（同行追加），所以本账第 734 行那条 `docs/API.md:952` 引用仍指得准。新立七条 **A306~A312**（同名覆盖并刷新 `created_at` 且空值抹掉旧值、两支只读端点会把注册表目录建出来、「默认注册表目录」仓内三份且端点那份与另两份不是同一个地方、登记过的数据集在产品面上删不掉且边没有产品级创建入口、`validate_dependencies` 不查环、422 英文与 4xx 中文双轨错误体、只有 `POST` 挂鉴权而前端没有送头的通道），**A305 本轮已修**故只立不待拍。**census 两把棘轮本轮动了三格**：新守卫两支 `parametrize`（`input_file` 四档坏值 / `registry_path` 两档坏值）值位都是内联字符串字面清单 ⇒ l97 的 `MEASURED_CALLS` **419 → 421**、`MEASURED_READABLE` **301 → 303**、盲面 **118 一格未动**；读数出自仓外探针 `l232_census.py` 直接 `import` 那把尺子逐处打印桶归属，不是按「字面量 ⇒ 可读」推的（L204 那次正是这么想当然写错过一回）；l98 从 l97 `import` 同两个常量故一并生效，其六档分布与豁免 `23` 原地绿。**另外三把按各自 docstring 的处方重钉，没有一把被改松**：`test_api_doc_field_hints_l223.py` 首跑红 1 格——我在 `docs/API.md` 那格里反引号引了 `remove_dataset` / `unregister_dataset` 两个**根本不存在**的方法名（它们不存在正是 A309 的内容），而那把尺子要求提示里每个名字都绑得到证据 ⇒ 改自己的散文（去掉反引号与那两个名字），不动尺子；`l224` 首跑红 5 格全是「按实测回填」类；本轮新守卫首跑红 3 格，其中 2 格是**我自己把族过滤写宽了**（`"dependenc" in p` 同时命中另一族的 `GET /api/system/dependencies`，那是 L226 明确拒接的「可选依赖包诊断」）⇒ 收窄成前缀 `/api/system/dependency/`，并留一支注入臂（臂5）证明这次收窄是吃重的。注入五臂逐臂点名喊红（**1 / 4 / 1 / 1 / 1** 支，臂2 同时打红 l224 的三支 query 判据），被改的四个文件逐臂字节复原、收尾重新验盘并断言 sha256 与进场值相等、基线复跑 65 passed。门禁：**8621 passed / 11 skipped / 0 failed / 覆盖率 99.81%**（186.34 s；改前同一条命令是 8618 passed + 3 failed，两边总数一致 ⇒ 那三格全是重钉类而不是用例数变了）；前端 `tsc --noEmit` 与 `eslint . --max-warnings 0` 均 rc=0（首跑 typecheck 红 8 格：两份既有替身文件的 `vi.hoisted` 声明块里没有新键；lint 红 5 格：新用例里 5 处 `any` ⇒ 都是改自己的测试代码收口，没有一处放宽规则），`vitest run` **17 文件 / 206 → 214 支全绿** ⇒ 详见循环日志 L232
- [x] **L233** `5d821c32b` +（账本批即本条所在提交）`feat,docs(augmentor)`：**前端补上 `X-API-Key` 通道——收 A302 / A312（本轮不接新端点，消费面仍 67/75）**。用户拍板走 A302 的修法①：`web/src/services/api.ts` 一处通道（`API_KEY_HEADER` / `API_KEY_STORAGE_KEY` / `getApiKey` / `setApiKey` + 请求拦截器，留空即不送头 ⇒ 零配置照旧可用）、key 的输入界面落在「配置管理 → 写操作鉴权」卡（存 / 清除 / 已存标记，卡上写明这是明文存储、不是登录、也不替代 HTTPS，且刻意**不**报「校验通过」——本层没能力证明那份 key 是对的）、401 的指路加在 `apiErrorDetail` 这个**唯一漏斗**上（一处改动，21 个页面动作同时被指路，并把「存着却不对」与「根本没存」两种成因分开说；服务层不引入 antd，所以不写响应拦截器弹窗）。**规模是被探针更正的，不是推的**：75 条操作里 **24** 条挂 `verify_api_key`，其中 **21** 条正是前端已接线的形状（L231 立账时只数了备份那一族的 3 只，**低估了一个量级**），另 **3** 条挂门禁但还没接线（`translate` / `migrate` / `stream`），门禁里**没有一条 GET**。新立 `tests/unit/test_api_key_channel_l233.py`（**18 支**：规模 / 通道形状 / 口径 / 真链路四类），行为面仍在 `api.test.ts`（+7 支，共 **68**）与 `Settings.test.tsx`（+6 支，共 **14**），两面各钉一半。**l231 那一维的前提被本轮改掉**：「前端零通道」当场成了假话，按新世界重钉为只留后端门禁事实（**不是放宽**，前端通道不在那边重复钉一份）。**注入 14 臂逐臂点名喊红**，收尾逐份 sha256 与进场值相等、基线复跑 18 passed；**其中一臂真的在仓里留了残留**（第一版 harness 在 `finally` 复原时抛 `OSError [Errno 22] EINVAL` 死掉，`api.ts` 少了一行 `if (response?.status !== 401) {`），靠 1 支守卫红 + vitest transform 失败才发现，按原文修回并复验（`tsc` rc=0、101 支绿、numstat 65/10）⇒ 第二版 harness 加三件事：跑前把 7 份原样备份进 Temp、写后读回比 sha256 且 EINVAL 重试三次、逐臂即时打印。**这是 L218 那处「注入残留」同一缺陷类第二次发生**。文档四处（README / API.md / ARCHITECTURE.md / DEPLOYMENT.md）与页面两处口径全部**同行改写、`git diff --numstat` 复核净零行**（2/2、3/3、1/1、1/1、System.tsx 7/7）。门禁：**8639 passed / 11 skipped / 0 failed / 覆盖率 99.81%**（166.24 s，与 L232 的 8621 相减 `+18` = 新守卫 18 支）；前端 `tsc --noEmit` 与 `eslint . --max-warnings 0` 均 rc=0，`vitest run` **17 文件 / 214 → 227 支全绿**；census 两把棘轮一格未动（新守卫零 `parametrize`），l98 哈希槽只读第一本账故同样未动 ⇒ 详见循环日志 L233
- [x] **L234** `aafbcbcc4` + `1ebd2c8ae`（A314 / A315）+（账本批即本条所在提交）`feat,fix,docs(augmentor)`：**A279 第九批接线——系统那一族五条进页面，消费面 67 → 72/75；并把接线途中量出的三处产品缺陷（A313 / A314 / A315）同轮收掉**。`monitor` 与 `auto-test` 进系统状态页第五张卡（都只读、都**不挂**门禁），`migrate` 与 `stream` 进第六张卡（都写盘、都**挂**门禁），`validate-config` 进配置管理页新卡（只读、不挂门禁）——**至此全仓六只落盘产物端点早已在 L230 收齐，本轮收的是「系统运维那一面」**。`UNCONSUMED` 8 → **3**（只剩 `dependencies` / `translate` / `health-gate`）、已消费 67 → **72**（75 条的 **96%**）、`NO_ENTRY` 仍 **1**（`translate`，归 A283 的契约缺口）、`CLI_ENTRY` 7 条/7 名 → **2 条/2 名**（`health-gate` / `doctor`）；l233 立的那格「挂门禁但没接线」3 → **1**。**A313（同轮收）**：`stream` 的 `operation` 在两端不是同一份清单——CLI 写 `quality / dedup / clean / export`，端点认 `quality / dedup / clean / none`；`export` 被 argparse 收下后落进 handler 的 `else`，而那个分支只是 `lambda items: items`（**原样搬运**，名字承诺的导出从来没发生过），连既有测试的 docstring 都逐字写着「export 操作为直通处理」把这句谎话钉成了事实；反过来 HTTP 侧合法的 `none` 在 CLI 里被 argparse 直接拒（退出码 2）。修法是把 CLI 那份字面改成与端点同一份，**没有新插 Python 常数**——把权威搬进 `augmentor/streaming.py` 会在 `WRITER_FORMATS` 之后插行、平移账本 A147 那条活引用，而仓内既有惯例（l226 / l175 / l189）也是「CLI 留字面、守卫对账」。**A314（同轮收）**：全量门禁首跑那**唯一一支红**倒推出来的产品缺陷，不是设计出来的——`OSError: [WinError 1920] 系统无法访问此文件。: 'data\versions\current'`，触发点是 `create_version` 里那句「先看那一格有没有东西」；现场取证量到它是 WSL 在 drvfs 上留下的 **LX 链接**（`is_symlink()` 是 **False**、`os.readlink()` 报 *not a symbolic link*、只有 `os.stat` 抛 1920 ⇒ 「先判 is_symlink 再判 exists」这类重排**治不了它**），且链接目标写的是**整段相对路径** `data/versions/v_…` ⇒ 按自己所在目录解析成 `data/versions/data/versions/v_…` 而**恒断链**（成因是 `storage_dir` 的出厂默认就是相对路径）。**A315（同轮收）**：A168 残留守卫的范围欠账——`data/versions/` 装的不是产品结构而是用例跑出来的运行期产物（现量 **67** 个 `v_*` 目录，那份 LX 链接就住在里面），而原口径「往 `data/` 更深一层写不判」把它整片放过了；补进 `WATCHED` 之后实测代价是**泄漏 1 条用例**（1 499 条里），扩范围后全量 8 687 条**泄漏 0 条** ⇒ 与 L94 那次入库同形、**不需要白名单**。新守卫 `tests/unit/test_system_ops_wiring_l234.py`（**36 支**，五面：清单同源 / 能力真有差 / 哨兵是躲判决的 / 口径如实 / 真链路形状）与 `tests/unit/test_version_current_link_l234.py`（**10 支**，四面：连创两版不炸 / 目标是裸名 / stat 抛 1920 也不许炸 / `.txt` 不会停在旧版本上）。**注入取证十三臂逐臂点名喊红**（接线六臂 + 前端三臂 + versioning 与残留守卫四臂，后四臂走零写入通道），仓库前后哈希相同（`cc1a80b8…` 与 `c17d1224…` 两次各验一遍）。门禁：**8687 passed / 0 failed / 11 skipped / 覆盖率 99.83%**（182.41 s；与 L233 的 8639 相减 `+48` = 接线守卫 36 + l233 新棘轮 1 + versioning 10 + l94 新棘轮 1）；前端 `tsc --noEmit` 与 `eslint . --max-warnings 0` 均 rc=0，`vitest run` **17 文件 / 227 → 18 文件 / 248 支全绿**（`+21` = System.ops 13 + Settings 4 + api.test 4）；census 三数（421 / 303 / 118）一格未动（两份新文件零 `parametrize`）⇒ 详见循环日志 L234
- [x] **L235** `e7c39adc1` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第十批接线——`health-gate` 进质量中心，消费面 72 → 73/75；同轮量出 A319 / A320 / A321 三条新账**。质量中心从八项能力变成九项：新卡「健康门禁」把 `POST /api/quality/health-gate` 的三面旋钮全接上（文本字段、四个健康分权重、`pass_rate` / `duplicate_rate` / `completeness` 三档阈值、`block_on_warning`），并让 `/api/quality/evaluate` 的 `pass_rate` **一键喂进门禁**——那一格刻意不用 `||` 哨兵，因为 evaluate 真会回 `0.0`，而 `0.0` 是判决不是「没填」（新守卫与渲染面各有一支专门钉这件事）。`UNCONSUMED` 3 → **2**（只剩 `dependencies` / `translate`）、已消费 72 → **73**（75 条的 **97%**）、`CLI_ENTRY` 2 条/2 名 → **1 条/1 名**（只剩 `dependencies` → `doctor`）、`NO_ENTRY` 仍 **1**；这一条**不带 query** ⇒ `params` 那一维一个数没动（仍 11 条 / 8 键）。新守卫 `tests/unit/test_health_gate_wiring_l235.py`（**33 支**，五面：页面初值＝契约默认 / 服务层只有一份权威 / TS 面＝pydantic 面 / 判决是页面说的那样 / 散文如实）与渲染面 `web/src/pages/Quality.healthgate.test.tsx`（**13 支**）。**A319**：四个权重既不校验非负也不校验上界，`health_score` 也不钳 [0,1]——实测 `weights=[2.0,-0.5,-0.25,-0.25]` 把健康分顶到 **1.1698**（`level` 仍 `healthy`），而门禁那三条规则**根本不看权重** ⇒ 同一份回执里「healthy」与「verdict=failed」同时为真。**A320**：权重之和的容差写的是 `abs(sum-1.0) > 0.01`，而 `[0.24,0.25,0.25,0.25]` 的浮点差是 **0.010000000000000009** ⇒ 0.99 与 1.01 两档**都被拒**、`[0.255,…]`（和 1.005）反而放行，「±0.01 以内算 1.0」这句在恰好 0.01 那一档是假话。**A321**：`text_field` 送空串或送一个数据集里不存在的键，后端**零判决**、`duplicate_rate` 一律回 **1.0** ⇒ 与「真的全重复」在响应里同形，而健康分那四项压根不看 `text_field`。三条都写在页面那张「读这一页之前要知道的六件事」里，不是只写在账本里。**守卫首跑红 1 格是我自己的尺子范围错**（`assert "0.25" not in 整页` 被页面里那句如实解释后端默认的文案判红）⇒ 按「扫描面与禁令面同族」收窄到 `healthGate(file, {…})` 调用点区块，并留一支反证（`pass_rate` 那一格确实设了 `min={0}` / `max={1}`）免得禁令恒真。**渲染面首跑红 3 格全是判据与取证问题**：`InputNumber step={0.05}` 把 0 显示成 `'0.00'`；那句「健康分本身不参与门禁判决」住在卡上的 Alert 而不在回执 div 里；第三支反复红的根因用两份一次性 `Dbg.test.tsx`（建完同一条命令里删掉）取到——antd 会在按钮里留一只 `ant-btn-loading-icon-motion-leave-start` 图标（自带 `aria-label="loading"`），jsdom 不放动画 ⇒ 可及名**永久**变成「loading健康门禁」，精确名查询恒失败，改按正则取（仓内 `DataManagement.transform.test.tsx` 早有同一惯例）。**注入取证十臂逐臂点名喊红**（Python 侧八臂 + 前端两臂，都走字节级注入；被改的三份文件逐臂复原并断言 sha256 与进场值逐份相等、基线复跑全绿），其中前端那两臂第一版判据是**恒真的**（「用例名出现在 vitest 输出里」——vitest 会把全部 13 个用例名连同 ✓/× 都打出来），改成「`FAIL` 行点名 + 失败计数恰好 1」，而 `FAIL` 行第一次匹配不上是因为**输出带 ANSI 色码**。**读数**：门禁 `PYTHONIOENCODING=utf-8 …python.exe -m pytest tests -q -p no:cacheprovider` ⇒ **8721 passed / 0 failed / 10 skipped / 覆盖率 99.83%**（285.65 s；与 L234 的 8687 / 11 相比 passed `+34`、skipped `-1`，而**收集总数 8698 → 8731 恰好 `+33`** = 新守卫 33 支 ⇒ 那 `-1` 是一格 skip 变成实跑，已归因到 `test_serve_config_wiring_l97.py:119`：它的跳过条件正是「`web/dist` 不存在」，而 `web/dist` 在 22:45 出现了（本会话只跑 typecheck / lint / vitest，都不产出 dist ⇒ 是并发会话的构建），属环境读数不是产品面变化）。前端三门禁：`tsc --noEmit` 与 `eslint . --max-warnings 0` 均 rc=0，`vitest run` **18 文件 / 248 → 20 文件 / 276 支全绿**（`+28`：本轮那一支文件 13 支，另 15 支来自并发会话新增的 `api.optional-defaults.test.ts`，该文件此刻仍在被另一会话改写、单独复跑读 14 支，所以那个 15 是全量跑当时的读数，不追平）；census 三数（421 / 303 / 118）一格未动（两份新文件零 `parametrize`）。`README.md` 的页面表那一行同行补上健康门禁（`git diff --numstat` 1/1 净零行）⇒ 详见循环日志 L235
- [x] **L236** `890965d32` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 收口——最后两员接完，消费面 73 → 75/75（100%）；同轮收 A283①（translate 的落盘口契约）、新立 A322**。① `POST /api/dataset/translate`：先给契约补上 `TranslateRequest.output_file` 与 `TranslateResponse.output_file`（都可选，键**恒在**、没给时为 `null`——用「键在不在」区分不了「没落盘」与「落了盘没回显」，所以两态都要可读），再把「批量翻译」卡接进数据管理页（输出文件名 / 目标语言 / 字段名 / 保留原文四格 + 计费确认闸），失败回执四键（`failed` / `failed_indices` / `errors` / `errors_truncated`）逐格可见。**顺序是量过的：落点先 `resolve_data_path(for_write=True)`、再动模型**，越界落点 403 出声时模型调用 **0** 次（注入臂实证：摘掉白名单判决之后那一份响应体里赫然是根外绝对路径，且三次模型调用都花了）。② `GET /api/system/dependencies`：把依赖诊断从系统状态页那道「综合状态取到才渲染」的整体门里摘出来，改成自己取数、卡上自己一只刷新按钮，并留 `deps ?? status?.dependencies ?? null` 这条退路。**取法是论证过的，不是顺手接一格指标**：两条端点的诊断逐格相同（同出 `augmentor.diagnostics.check_dependencies`），但 `/api/status` 要真构造管道——把 `api.routes.status.get_pipeline` 换成抛异常的替身之后，dependencies 读数一格未变、status 退成 `model_default='unknown'` / `model_available=False`（两支都仍 200）⇒ 管道拉不动时这一面没有理由跟着黑掉，退路也不是空的（两面数据相同是实测）。`UNCONSUMED` 2 → **0**、已消费 73 → **75**（**100%**）、`CLI_ENTRY` 1 条/1 名 → **0 条/0 名**（`doctor` 整名出名单）、`NO_ENTRY` 1 → **0**；l233 那格「挂门禁但没接线」1 → **0**（**24 / 24**），`Settings.tsx` 卡片上那句影响面口径同轮从 24/23 改成 24/24（守卫用 f-string 绑死，改不动就红）。新守卫 `tests/unit/test_last_two_wiring_l236.py`（**18 支**，四面：页面初值＝契约默认 / 落盘口 / A322 / 系统页自取依赖诊断）与渲染面 `web/src/pages/DataManagement.translate.test.tsx`（**9 支**），System 那一份 7 → **10** 支。**L235 挂着的那句「收口轮必须复验空集相等时仍会响」已复验**：l224 那把尺子带两支牙齿（`test_the_empty_claims_are_not_a_blind_ruler`、用 monkeypatch 摘掉一处调用形状的 `test_unwiring_one_operation_turns_the_empty_roster_red`），注入第五臂把 l233 的 `GATED_NOT_YET_CONSUMED` 塞回一条也当场红 ⇒ 收口那一步没有把尺子读成恒绿。**A322 本轮新立、后端一行未改**：`text_key` 那一格零判决——空串或幽灵键都回 **200**、模型调用 **0** 次、`failed=0` / `failed_indices=[]` / `errors=[]`，产物凭空多出**三**键（那一列、它的 `_original`，还有一个 `language` 目标语言戳），而原字段一个字没动。**第三键是本轮首跑量出来的新事实**：守卫首跑 **2 failed**，多出来的那一格是 `language`，源头是 `augmentor/multilingual.py:190` 那句无条件盖戳（探针当时只报了「多两键」）⇒ 「200 且 failed=0 看起来像全成功」这条风险从「回执全零」升级为「回执全零 + 产物带语言戳」，四处同轮写清（守卫 / 页面卡片 / 前端用例 / `docs/API.md`）。**注入取证五臂逐臂点名喊红**：前三臂走**零写入通道**（pytest 插件在导入后的模块对象上打补丁：`atomic_write_json` 变哑 ⇒ 4 支红、`resolve_data_path` 去掉白名单判决 ⇒ 越界那一支红、l233 的空尾巴塞回一条 ⇒ 1 支红），后两臂必须动源码故走字节级注入、**复原后重新验盘**（`disabled={!trAck}` → `disabled={false}` ⇒ vitest 1 支红；`{diag && (` → `{status && (` ⇒ vitest 1 支红 + Python 静态那一支红；两份 sha256 与进场值逐份相等）。**中间读数不美化**：第二臂那个注入器首跑在**打印环节**被 gbk 打断（`UnicodeEncodeError: 'gbk' codec can't encode '\u23af'`），finally 已复原并验盘（`System.tsx:432` 仍是 `{diag && (`），单臂重跑才取到读数。文档三处（README 页面表两行 + HTTP API 段一句、`docs/API.md` translate 说明行）全部**同行改写**，`git diff --numstat` 复核 **3/3 与 1/1 净零行**，故本账那条 `README.md:65` 引用仍指得准（第 65 行仍是 `/system` 那一行）。门禁：**8741 passed / 0 failed / 10 skipped / 覆盖率 lines 99.83%、branches 99%**（363.76 s；与 L235 的 8721 / 10 相比 passed `+20`，收集总数 8731 → **8751** 恰好 `+20` = 新守卫 18 支 + 并发会话 `8041df804` 给 `test_version_current_link_l234.py` 补的参数化 2 支）——**这一跑不带 `--ignore`，L235 记下的那笔欠账已还清**（那个文件已被其作者修好并提交）。前端三门禁：`tsc --noEmit` rc=0、`eslint . --max-warnings 0` rc=0、`vitest run` **21 文件 / 287 支全绿**（`+11` = 本轮 translate 9 支 + System 3 支 − 并发会话 `8041df804` 在 `api.optional-defaults.test.ts` 删掉的 1 支；276 + 12 − 1 = 287，账对得平）；census 三数（421 / 303 / 118）一格未动（两份新文件零 `parametrize`）⇒ 详见循环日志 L236
- [x] **L237** `043ee69d3` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A281 收口——三只 TS 响应类型补齐 12 处缺口 + 建「TS ↔ OpenAPI 契约」两层跨语言守卫（21 支）+ 补两处渲染**。A281 立账写的是 3 处，本轮把尺子做深一层重量实测 **12 处**（`AppConfig` 缺 9、`StatusResponse` 缺 2、`UpdateConfigResponse` 缺 1，反向幽灵 0）——立账那把尺子没错，只是浅一层：`ConfigResponse` 九个节在契约里全是 `additionalProperties: true` 的自由字典、**没有 `properties`**，节内 7 键根本不进契约 ⇒ 守卫第二层必须打只读的 `GET /api/config` 递归比键树。新守卫 `tests/unit/test_ts_contract_types_l237.py`（**21 支 / 5 面**：尺子先自证、fetcher 按**操作**配对、自由字典面走活响应、改前读数复现、渲染面认领类型新带的那些格），**75 只**带类型 fetchers 第一层全数相等、`unannotated` 与 `unresolved` 皆空集，`UNMEASURABLE` 空集（立账时点名的两只「naive 正则解析不动」`AugmentProgress` 联合别名与 `VersionDetail extends` 在新口径里都解析动了），路径配对按**段级**规则收敛（`/versions/${versionId}` 唯一命中 `{version_id}` 而排除 `/versions/history`）。渲染面同轮补：系统状态页「模型重试次数 / 重试等待」两格（`null` ⇒ 显式「无后端」，不折成 0），配置管理页「其余节的生效值（本页只读）」与「上次保存的回执」两张卡（后者渲染后端真回的 `message` 与逐条 `ignored_keys`）。**本轮零对外契约改动**（只动 TS 与页面）⇒ l200 契约数字守卫原地绿。注入取证 11 臂：守卫内 5 臂走内存字符串手术（零写入），渲染面 6 臂字节级注入、逐臂 sha256 复原（S1=1 / S2=2 / T1=1 / T2=1 / T3=4 / T4=2 支红；T1 只红 1 支太窄所以补 T4）。**新立 A323**（`handleSave` 的 toast 是第三处措辞、不带后端的 `ignored_keys` 判决）只立账不改，可达性已按实测收窄：`{}` 那一档从本页**不可达**（页面恒送 `default_model`）⇒ 详见循环日志 L237
- [x] **L238** `88837aa2a` + `049dea678`（api.ts 两处注释跟上后端新判决）+（账本批即本条所在提交）`fix,test,docs(augmentor)`：**A319 / A320 / A321 / A322 「判决缺失面」四条收口——同一条判据从两份合成一份**。权重族共引 `validation.require_ratio_list`（逐项 [0, 1] 点名 `weights[i]`、出口 `min(1.0, max(0.0, …))` 钳进 [0, 1]、容差抽成 `WEIGHT_SUM_TOLERANCE` 且量化到 9 位使「±0.01 含端点」成真尺子）；字段存在性新建 `validation.require_item_field`，health-gate / impact / translate 三面共引 ⇒ 空串、纯空白、幽灵键一律 400 并带回首条实际字段，`text_key` 那一刀走在模型与写盘之前（0 次调用、0 字节落盘）。新守卫 `test_decision_gates_l238.py`（30 支 / 5 面）专量行为面测不出的那一维：判据身份 `is`、判序（400 先于 404）、两面同文、反手抄与迟到绑定哨身。全量 **8802 passed / 0 failed / 11 skipped / 99.83%**。⇒ 详见循环日志 L238
- [x] **L239** `1c5cdb0b6` +（账本批即本条所在提交）`test,docs(augmentor)`：**A286 收口 + 3.12 解释器名册现量钉版 + 性能热点现量入账（零对外行为变化的一轮）**：① `test_parametrize_blind_face_l98` 的 `MEASURED_HAZARD_ROSTER` 补 `(3, 12)` 档，按 uv 托管 cpython **3.12.14 整跑 8813 条采集现量**钉版——读数与该版上 `dynamic_risky` 的整跑逐字相同，且与 `(3, 13)` 那格**同串**（`typing._UnionGenericAlias:Union`）⇒ 原来那句「同一份源码在两版上读数一样就是有把尺子失效了」只对 **(3,13) vs (3,14)** 成立，劈叉发生在 3.14（PEP 604 之后别名换了类型名），据此把 `test_hazard_roster_is_pinned_per_interpreter` 的 docstring 从「两版」改成「逐版」、并把那条「不许两两相同」的推断显式限死在 3.13/3.14 之间；3.12 上那格 skip 消掉 ⇒ **skipped 11 → 10**。② **A286**：比值型计时守卫按立案时给的处方 ② 收口——「数内层重算次数」取代墙钟比值，`_calculate_diversity_score()` 整趟 `builtins.sum` 调用数在 n=1000/2000/10000/20000 **四档同为 2**（`MEASURED_SUM_CALLS`），判据不再看这台机器此刻给不给 CPU；`RATIO_CEILING`、`MIN_MEANINGFUL_MS` 与那条把阈值冻成断言的用例一并退役（**撤下不是放宽**：统计判据换成精确判据），保留 20 倍松的绝对兜底与「计时把手」自检；牙齿三支：复刻退化档现量 `n+1`（5001/10001）、空档早退 0 次、替身必复原。三臂字节级注入取证全部点名喊红且逐字节复原（`analytics.py` 进出 sha `13e78b589e8d` 一致）。③ 20k 合成条性能现量入账并立 **A325 / A326 / A327**（最大一格是 `LeakageDetector.detect` **45.4 s**，41.0 s 落在 `_collections._count_elements` 的 9988 次调用上），以及 einsum 向量化**量化后否决**（快 3.73× 但产物不位相同）。前端三门禁**不适用**（`web/` 零改动）。⇒ 详见循环日志 L239
- [x] **L240** `807a2beec` +（账本批即本条所在提交）`perf,test,docs(augmentor)`：**A325 收口——泄漏近似计数加一条「按代价选路 + numpy 0/1 矩阵乘法」稠密档，判决逐条不变**：10k×10k 塌缩档从 **48 929.0 ms** 降到 **1 742.1 ms**（同一次运行内 auto vs 强制倒排 **28.1×**），六档形状的报告 `to_dict()` 与倒排**逐字节相同**；选路判据照 `dedup.py` 的「两条独立账」纯整数（时间账 摊平次数 × 单价 1000 vs 乘加格数、内存账 n_train × vocab × 4 ≤ 64 MiB），判决链条靠「`ov` 是 float32 精确整数 + `best >= t` 等价于 ∃候选 J ≥ t ⇒ 剪枝不承重 + argmax 在 float64 选 + 最终 J 仍由 `_jaccard_from_counts` 现算」四步。A325 原案处方 ①（长度带剪枝）实测只省 **53.7%** ⇒ 量化后否决，走处方 ③；铺开档（random5）自然判负给倒排、判负之后仍慢 **8%**（块化簿记是两条路共用的代价，如实入账）。新增守卫 `test_leakage_dense_route_l240.py` 29 支（**全计数判据，不看墙钟**）；两支 l95 尺子（内存上界、剪枝牙齿）被新判据路由成空转，按本轮点名被测路线强制倒排、阈值一字未动。八臂字节级注入取证 **7 臂红**（去剪枝 / 判据恒真 / 恒假 / 旁路权威算式 / 错位回填 / 块大小不看列数 / 删内存账），A6「argmax 换 float32」**不红**——float64 那一手只管并列选择、不承重，如实记为阴性臂并把真正把手换成「稠密不许旁路 `_jaccard_from_counts`」的计数钉子。普查按现量重钉 CALLS 426→429、READABLE 308→311、盲面 118 未动；`leakage.py` 净 +196 行把卷一 L55 的 `leakage.py:124` 平移，按处方 3 重钉为 `leakage.py:352` 并写明归因。全量门禁 **8835 passed / 0 failed / 10 skipped、99.84%**（= L239 基准 + 本轮新立 29 支）。前端三门禁**不适用**（`web/` 零改动）。⇒ 详见循环日志 L240
- [x] **L241** `9d417eda7` +（账本批即本条所在提交）`test,docs(augmentor)`：**代价预言机选路家族入册——文档 §3.34 + 跨家族机械守卫 24 支，零对外行为变化**：`docs/ARCHITECTURE.md` 新增 §3.34（两条独立账的定义与四个单价常数、收益 8.1×~32.0× / 10k×10k **28.1×**、铺开档判负之后仍慢 8% 如实入账、「一条从注入里长出来的纪律」）与 §7.1 第四颗子弹（性能判据一律计数，L239 负载差 46% / L240 −18% 两轮正面证据）；新守卫 `tests/unit/test_cost_oracle_routing_l241.py` 24 支钉五件事——文档 §3.34 反引号点名的预言机名与 AST 实推集合**两个方向精确相等**（刻意不用 L76/L77 那套只降不升的棘轮，否则「新增路线却不写文档」那一半被放过）、三条独立通道（名字形状 / 比较式里的 `*_COST` / `-> bool` + 乘法比较 + 同模块 `if`-test 消费）各自实推必须给出同一集合、判据本体零浮点/零调用/零属性访问、每支预言机的调用点坐在 `if` 的 test 位且两侧生产者同住一个函数、每支都在 tests 树里有把它焊死的 `setattr` / `patch` 接缝。第三通道先量过误伤面（全包只命中那两支）才采用。六臂真文件注入取证**全部按预期红**，逐字节复原后复验哈希。普查按现量重钉 l97 CALLS 429→430、READABLE 311→312、盲面 118 未动；l98 豁免用数 23→24（涨它不等于放松：四档 58/38/11/11、盲面 118 与名册 113 一格未动）。产品码一字未改 ⇒ 契约数字守卫一格未动、前端三门禁不适用；全量门禁 **8859 passed / 0 failed / 10 skipped、99.84%**（= L240 基准 8835 + 本轮新立 24 支）。⇒ 详见循环日志 L241
- [x] **L242** `16b5bf4c6` +（账本批即本条所在提交）`test,docs(augmentor)`：**Backlog 表关闭标记与进度声明面对账——量出 20 笔「说收了却没标」、补标 17 格、新守卫 24 支、新立 A328（定级 M 待拍）**：A265 那把尺子管「一轮写两遍自己」，账本的第三面（Backlog 表定级格）此前**没有任何机械通道**——开工前实测欠账 20 笔 = 17 真漏标 + 3 合法半程。口径全部现量选出：ID 认到一位数（三档读数 gap 11 / 20 / 20，账目面 190 / 234 / 247 行）、声明面只收清单行与日志标题行（宽窄口径同 gap）、「全清」与划掉的数字一律不算标记（A298 / A176 现场反例）、判据写成**名册精确相等**而不是棘轮。同轮重钉 `test_doc_line_refs_l79` 的结案口径 **107 → 119**（+12 归因：14 格里 2 格行内本已含删除线；行首口径 87 未动，差值 20 → 32）。本轮零对外行为变化：产品码一字未动，改动面 = 1 支新守卫 + 1 处重钉 + `docs/ARCHITECTURE.md` +2 行 + 两卷账本 18 格。

## Backlog A — 质量缺口（缺数 / 偏支 / 健壮性）

| # | 位置 | 内容 | 量级 |
|---|------|------|------|
| A201 | `augmentor/retry.py`、`augmentor/version_control.py`、`augmentor/models/base.py` | **已关闭（L101）**：16 条缺数 + 3 条隐藏偏支弧全清 | S |
| A202 | `api/routes/quality.py`、`api/routes/export.py`、`api/routes/augment.py`、`api/routes/multimodal.py`、`api/deps.py`、`augmentor/config.py` | **已关闭（L102）**：11 条缺数（路由错误/降级支 + 依赖层降级键 + 配置保存容错）全清 | S |
| A203 | `augmentor/cli/commands/version.py` | **已关闭（L103）**：delete 动作 8 条缺数（98-106）全清 | S |
| A204 | `augmentor/preview.py` | **已关闭（L103）**：非原生格式的 DataValidationError 守卫支（177）清零 | XS |
| A205 | 全仓约 40 支 | **已关闭（L114–L126）**：偏支 85 → 17，46 条逐支判定（24 条可达补测清零 + L126 删 5 条恒真守卫 + 余 17 条永久记档结构族） | M |
| A206 | `web/src/pages` | **已关闭（L104–L113）**：10 页各建组件测试，前端全量 13 文件 112 例全绿 | L |
| A207 | `web` | **已关闭（L128）**：路由懒加载 + 三 vendor 拆分本体已就位，补 `check-dist.mjs` 六格产物体检 + `npm run build:check` 入口 | M |
| A208 | `docs` | **已关闭（L129）**：API 易混字段词典（9 字段）+ API 面排查 FAQ（6 问） | M |
| A265 | `OPTIMIZATION_LOOP_2.md` | **新立（L197）**：账本「已完成」清单与循环日志的进度一致性**没有机器守卫**——L194 与 L50 两次漏写清单行都靠人眼发现。修法方向：从循环日志的 `### L\d+` 标题集合推导应有轮次，与清单行对账（防空转断言同 `test_cli_verdict_wiring.py` 的 S1==S2 型式）。**这是第二次同类，按纪律升级为待办而不是再忍一次** | ~~S~~ **已收口（L203）**：清单行与日志块三条不变量有机器守卫，进度声明面不再靠人眼 |
| A266 | `docs/API.md` / `docs/ARCHITECTURE.md` / augmentor 顶层 README | **新立（L197）**：契约面数字（端点总数、tag 计数、导出格式清单、前端页面数、事件循环守门条数）全靠人肉对账，L197 一次就抓出六处历史遗留错误。修法方向：从 `app.openapi()` / `ExportFormat` 枚举 / 前端页面目录**实推**后与文档断言相等（同 `TestExportFormatSurface` 先例） | ~~M~~ **已收口（L200）**：端点总数 / tag 计数 / 导出格式清单 / 前端页面数一律从实推对象对账 |
| A267 | `augmentor/visualizer.py`、`augmentor/config.py` 的 `VisualizationConfig.types` | **新立（L215）**：可视化这一族里除 `enabled`（L215 已接）之外还剩一对互证的死面。三条现量：① `visualization.types` 出厂五值 `["wordcloud", "length_distribution", "topic_cluster", "timeline", "quality_distribution"]`，全仓产品读者 **0**（grep 只命中 `config.py` 的定义与 `require_string_list` 校验，配置校验不算读者）；② `DataVisualizer.generate_timeline` 产品读者 **0**，只有 3 支单测调它；③ `generate_all_visualizations` 只出四类图，其中 `quality_distribution` 要 `scores` 形参，而两个产品调用点（`pipeline.visualize_dataset` 与 `cli/commands/analysis.py` 的报告模式）**都不传** ⇒ 出厂配置声称五类、阶段实际最多三类。**为什么 L215 不顺手接**：把 `types` 接成出图清单是一次**行为变更**而非零漂移接线——按出厂值接上，`timeline` 会今天第一次真的生成，产物多出那张图；`quality_distribution` 还要先定「scores 从哪来」。**判据要的是行为差，这里的「行为差」是新增产物而不是消费既有开关**，所以它属功能增强，得单独拍：要么按 `types` 生成（对外产物变化），要么承认清单虚假并从回显/文档降级。定级 M，待用户拍。 | M |
| A268 | `augmentor/semantic.py`、`config.vector.dimension` | **新立（L216）**：嵌入用的是零依赖的确定性字符二元组哈希，所以 `dimension` 就是配置值本身。接**真模型**嵌入（可选依赖 `sentence-transformers`）时要先回答两件事：① 模型输出宽度与本键冲突怎么收（模型给 384、配置写 768 时是拒配置、忽略配置，还是另开一只 `model_name` 键）；② 本机与 CI 都不装这个包，接上去等于写一条**任何没装它的环境都跑不到**的产品路径——正是本仓在收的「只写不读」形状。所以这条的前置条件是「先有一个装了包的定标环境」，不是先写代码。定级 M，待用户拍。 | M |
| A269 | `augmentor/vector/chromadb.py`、`augmentor/semantic.py` 的 `EXPLICIT_PERSISTENCE_BACKENDS` | **新立（L216）**：`persist` / `load` 只由 `FAISSDB` 实现，`ChromaDB` 那侧是 `PersistentClient` 自管（构造阶段就 `mkdir` 并注册集合）。L216 把落盘建库与状态读取定义为「有显式 `persist`/`load` 面」的后端专用，chromadb 在**构造之前**拿 400。要让 `vector.backend: chromadb` 真的可用，得先在装了 `chromadb` 的环境里量两件事：① 该后端的「索引是否已存在、多少条」怎么写成可测判据（今天的 `exists` / `count` 语义靠 `load()` 的布尔返回，那侧没有）；② 「GET 不落盘」这条承诺如何与「构造即落盘」的后端共存（要么给它一条只读的探查路径，要么把 `vector-status` 划成 faiss 专用面并回显清楚）。定级 M，待用户拍。 | M |
| A270 | `augmentor/multilingual.py` 的 `batch_translate` | **新立（L217）**：逐条翻译的异常被 `except Exception as e: logger.error(...)` **吞成「回退原文」**，返回清单与全部成功时**形状完全相同**——没有失败计数、没有逐条状态、没有非零退出信号。L217 接消费层时实测坐实：`model_backend=None` 时端点回 **200 + 整份原文**，调用方读到的是「翻译完成」。本层已在分批之前自保抛 `ModelNotConfiguredError`（后端整体缺失这一档出声了），但**逐条失败**这一档仍然沉默：真后端只对部分条目超时/被限流时，产物里混着原文而无人知道。修法是给 SDK 加**失败回执面**（`failures` 计数 + 索引列表，或逐条 `translation_status`），面一改就要同步 `TranslateResponse`、`test_api_openapi_contract` 与 `test_api_report_keys` 那把键集尺子——所以它是一次产品改动而不是接线。定级 M，待拍。**· 已关闭（L221）**：三面各落一层——`batch_translate` 从「返回清单」改成「返回报告」（`failed` / `failed_indices` / 本批**全量** `errors`；回退原文与长度、顺序对齐原样保留，只是不再无声），`translate_dataset` 把批内下标加 `start` 折成全数据集坐标并**独占**样本上限 20，`TranslateResponse` 声明回执四键。两条口径决策：① **部分失败仍走 200**——整批失败已由 `ModelNotConfiguredError` 落 400，而「25 条里 3 条没翻」既不是客户端错误也不是服务端故障，重发整份只会重复花钱，诚实读数放响应体；② 样本上限**不做成第五只配置键**——它是响应体尺寸的保护栏而不是能力开关，接一只键要规格表 / `config.yaml` / 节注册计数 / 普查尺子四处同步，却没有任何「按部署不同」的需求。本条目原文那句「同步 `test_api_report_keys`」**在仓内没有对应文件**（立账时凭印象写的），真正的键集尺子是 `test_api_openapi_contract.py` 的 `TestResponseModelDoesNotDropFields`，它守两个向（响应实到 + OpenAPI 声明），注入臂 A7 同时把这两支打红。 | M |
| A271 | `augmentor/config.py` 余 10 只 `config.*.enabled` + 10 只非闸门键 | **新立（L217）**：B5 的前沿现在有了机器名单（`tests/unit/test_config_reader_census_l217.py` 的零读者档，20 键逐键带理由）。两族的处置方向不同，**都不许顺手做**：① 10 只 `enabled`（active_learning / benchmark / context / evaluation / expander / frameworks / multimodal / rag / sampler / tracker）出厂全为 `false`，接进阶段入口等于**关掉当前始终在跑的行为**，是对外破坏面（L214 已实测：接成端点拒绝门当场红 13 支），要拍的是「破坏面要不要开」而不是「怎么接」；② 10 只非闸门键**按旋钮到底在不在分三族**（L218 用 `inspect.signature` 与 `api/routes` 的模型字段逐键验真，本条第一版那句「各自对应的模块确有同名构造参数」是错的，七只键里多数根本没有那种参数）：**请求侧已有同名旋钮** 2 只——`evaluation.metrics` / `evaluation.reference_field` 就在 `/api/dataset/evaluate` 的请求模型上，把它们改成「请求 > 配置 > SDK 默认」的缺省来源多半零漂移（出厂清单与 SDK 回落值同为 `bleu / rouge_l / similarity`）。**这一格 L219 已收一半**：`evaluation.metrics` 接进省略支路（顺带把空清单在门口 400，因为 SDK 那行 `metrics or [内置三只]` 会把空值**静默补成三项**），余 `reference_field` 因请求默认是硬值 `"output"` 而必须先改契约，另立 **A274**；**SDK 有同名旋钮但产品侧没有调用者** 4 只——`active_learning.strategy` / `batch_size` 在 `ActiveLearningLoop.__init__`、`max_iterations` 对应 `run(iterations=...)`（默认 `None` ⇒ 1 轮，与出厂 10 不同，接线即行为变更）、`expander.strategies` 在 `DomainExpander.batch_expand`，但 `ActiveLearningLoop` 全树零实例化、`self.expander` 构造之后从未被用（**A273**），所以要先造消费者再谈接线；**根本没有同名旋钮** 4 只——`frameworks.frameworks`（两个集成类的构造器只收 `text_key` / `answer_key`）、`sampler.dimensions`（`analyze_coverage` 只收 `items`，四个分布硬编码）、`tracker.metrics`（只有逐条 `log_metric(metric_name, value)`）与 `visualization.types`（**A267** 在案，接上去是产物新增不属接线）。前两族是接线、第三族是加功能，逐族拍完再按族批量收，别做半族（L214 的「修了 5 处漏 1 处」教训）。定级 M。**· L220 之后第②族只剩「SDK 有旋钮但产品侧无调用者」4 只与「根本没有同名旋钮」4 只**：「请求侧已有同名旋钮」那一格（2 只）已整格收完——`evaluation.metrics` L219、`evaluation.reference_field` L220，并同轮补了原本不存在的 `evaluation.generated_field`（新增键自带读者，未给普查添新账）。现量：schema **70 键**、有读者 **49**、零读者 **21**、归档待处置 **18** = 闸门 `enabled` **10** + 非闸门 **8**（4 只「有旋钮无调用者」+ 4 只「无同名旋钮」）。第①族那 10 只 `enabled` 与 A273 那两个无调用者的属性仍是「要拍」而不是「要写」 | M |
| A272 | `api/routes/config.py` 的可写四节 / `multilingual` 节 | **新立（L217）**：L217 接的四只键里除 `enabled` 之外，**没有任何一只可以经 `POST /api/config` 改**——可写面至今只有 export / rag / vector / multimodal 四节，`multilingual` 要改只能改 `config.yaml` 重启。要么把 `multilingual` 开成第 5 个可写节（连带 `test_the_two_tiers_partition_the_writable_sections` 的键名单扩员、节内判据已有所以不用补），要么在 `docs/API.md` 与回显面上写明「这节只读」。这是口径决策不是缺陷，所以先立账：现状下翻译的目标语言与批宽在部署期不可调，而 L217 的两条端点恰恰是运行时才消费它们。定级 S，待拍。 | S |
| A273 | `augmentor/pipeline.py` 的 `self.expander` / `self.tracker` | **新立（L218）**：管道在构造阶段建 `DomainExpander(self.model_backend) if self.model_backend else None`（`pipeline.py:146`）与 `ExperimentTracker()`（`pipeline.py:149`），而**产品树里再没有任何一处引用这两个属性**——grep `self.expander` 与 `self.tracker` 只命中那两行赋值，`api/`、`cli.py`、`augmentor/cli/` 全树对 `expander` / `tracker` 零命中，测试侧只有 `test_pipeline.py:412` 断言「无后端时 `instance.expander is None`」和两处给它塞 `model_backend` 的夹具。于是 `DomainExpander.batch_expand`（收 `strategies`、`topics_per_strategy`、`use_parallel`）与 `ExperimentTracker.log_metric` / `save_experiment` 这些**已写好并单测过的能力今天没有任何产品入口**，也正是 **A271** 里「SDK 有旋钮但没人转」那一族的根因。两条路要拍：① 接消费者（扩展走一条 `/api/dataset/expand` 一类入口、指标随增强结果落到版本面），② 承认死面并把构造删掉——后者会缩 `AugmentorPipeline` 的公开属性面，属对外契约变化，那 3 处测试引用要跟着改。构造本身近乎零成本（`ExperimentTracker` 明写「构造时不建目录，第一次记实验才建」），所以**这是消费条目不是性能条目**，别拿性能理由顺手删。定级 M，待拍。 | M |
| A274 | `api/routes/dataset_tools.py` 的 `EvaluateRequest.reference_field` / `config.evaluation.reference_field` | **新立（L219）**：L219 接 `evaluation.metrics` 时把同节的 `evaluation.reference_field` 留在原地，原因是它**接不动**。请求模型的默认是硬值 `reference_field: str = "output"`，调用方省略与显式传 `"output"` 在 API 面上是同一个形状，`request.reference_field or config....` 的 `or` 永远不会落到配置那一档——写出来就是一只假接线（配置改了读数不变，且没有任何测试能发现，正是本轮普查在收的形状）。真接法只有一条：把字段改成 `Optional[str] = None` 再走「请求 > 配置 > SDK 默认」，而这会同时改掉 OpenAPI 里该字段的**可空性**（定义上就变，不是推测），动到 `test_api_openapi_contract` 那把字段面尺子时要按现量重钉（尚未实测，故本条只立账不预判红几支），也改掉既有客户端「省略即 `output`」的语义。生成侧 `generated_field` 在配置里**没有对应键**，接完会出现「参考侧跟配置、生成侧跟硬编码」的不对称默认，要不要一起补一只 `evaluation.generated_field` 也是同一个口径决策。定级 S，待拍。**· L220 已收口**：两只字段名的请求默认改成 `Optional[str] = None` 哨兵，权威序 请求 > 配置 > 内置默认 才真的可达（省略与显式 `"output"` 从此是两种形状）；对称口径拍的是「补键」而不是留一对不对称默认 ⇒ 新增 `config.evaluation.generated_field`（出厂 `output`，运行时 `require_string` + 规格表 `non_empty` 与兄弟键同规）。零漂移前提不再由散文承担：dataclass 默认、仓库 YAML、规格表形状三臂各自钉住，另加「四列文本两两相似度不同」当地基。坏字段名两处出声（加载期拒空串、门口逐条带下标 400）。OpenAPI 可空性确实改了，但**没有任何一支既有尺子红**（`test_api_openapi_contract` 守的是响应面与端点覆盖，请求字段类型不在它刀口上），所以那把尺子的位置由新章 `TestRequestFieldsAreNowOmissible` 自己钉。⇒ 见循环日志 L220 | S |
| A275 | `docs/README.md` 的配置段表格 | **新立（L220）**：这张表的「关键项」列与 `AppConfig` 的键清单**没有任何机器对账**——L220 新增 `evaluation.generated_field` 时，`docs/README.md` 与 `config.yaml` 两处全靠人肉同步，而 `test_doc_contract_numbers_l200.py` 只核端点数、导出格式数、页面数那一类数字。这正是 **A266** 那句「契约面数字全靠人肉对账」在配置面上的具体形状，且它比端点面更容易烂：本仓 20 节 70 键每轮接线都在动。修法方向（与 A266 同族、可并表执行）：逐节从 `dataclasses.fields()` 实推键清单，与表格该列**逐节比对相等**，并同时核 `config.yaml` 的节/键集合（三侧同源的三角对账）。定级 S。**· 已关闭（L222）**：三侧对账落成 `tests/unit/test_config_three_side_l222.py`（59 支）。权威只有一份——节清单与键清单全从 `dataclasses.fields()` + `getattr` 实推（与 `test_section_registry_l77.py` 同口径），测试侧唯一手写名单是 `META_ROWS = {models, app}`，且每员要**同时**过两条机器验真（确实不是 dataclass 节 + 确在校验器 `META_TOP_SECTIONS` 白名单里），所以「把一只真节塞进例外名单」蒙不过去。`config.yaml` 那侧的判据不是「与 schema 逐节全等」而是**整节写全或整节注释列全**——这条是被文件自己纠正出来的：`logging` 三键刻意留在注释里（整节不存在时日志一字不变），第一版把它当漂移是我读错了产品口径。README 表格随之从 16 行 34 键补全为 22 行 70 键，并把「哪只键有人读」的权威明确留在普查守卫而不是文档里。对 HEAD 那份文档量出的真漂移 **21 红 / 38 绿**（5 整节缺席、13 节半节、恰好列全 2 节）；注入七臂各有牙齿，其中 A5「给 dataclass 加一只 init 键」复现的正是立账本条的那一步（L220 新增 `evaluation.generated_field`）。本轮量到的 `app` 节是另一码事，另立 **A276**。⇒ 见循环日志 L222 | S |
| A276 | `config.yaml` 顶部的 `app` 节（`name` / `version` / `debug`） | **新立（L222）**：**形状本身不是新事实**——`OPTIMIZATION_PLAN.md` §2.4 的表、`README.md` 变更说明第 6 条与 `tests/integration/test_cli_validate_config.py` 的 docstring 三处都记着「`AppConfig` 不建模 `app` 段、没有任何代码读它」，而历史上做过一次处置是把它从必填**降为可选**（因为 `save_config` 写出的配置否则必然校验失败）。**本条立的是另一码事：这只死面的*处置*从未进过账，而且它对现有的死键普查尺子结构上不可见**——`AppConfig` 没有 `app` 字段（`dataclasses.fields()` 实测不含它），`ConfigValidator` 又把它列进 `META_TOP_SECTIONS` 免检，于是 B5 那把「只覆盖 schema 键」的普查尺子永远不会把它报成零读者键，它也不会被加载器装配。当前后果：改这三键不影响任何输出，`GET /api/config` 的响应面里也没有它们（该端点逐字段从 `p.config` 即 `AppConfig` 拼装，见 `api/routes/config.py` 的 `get_config`，没有字段就没有格子）。三条处置方向都动得到对外文件，所以先立账：**① 接成节**（新增 `AppMetaConfig` 三键并接进 `/api/config` 与 `--version` 一类读取面 ⇒ schema 70 → 73 键，l77 节注册、规格表、普查尺子与本把三侧尺子都要按现量重钉）、**② 从出厂文件删掉那一段**（改动用户可见的配置面，且 `ConfigValidator.META_TOP_SECTIONS` 要跟着收窄、`save_config` 的「保留未建模顶层键」那条行为也一并失去对象）、**③ 维持现状并把「产品侧零读者」写明**（本轮 `docs/README.md` 表格已按③落了说明，但键本身仍在那儿）。本轮选③而没有顺手删或顺手接：前者是无声地改变出厂形状，后者是一轮做不完的对外契约变化。`META_ROWS` 的验真臂把这个形状钉在明处——白名单成员必须「确实不是节」。定级 S，待拍。 | S |
| A277 | `docs/API.md` 逐端点的「人读字段提示」列 | **新立并同轮收口（L223）**：这张四列说明表是 68 条端点里 dataset 与 system 两组共 31 条的**字段级承诺**面（哪个字段省略会落配置、哪个键可能几千个键、哪种坏值 400），但仓内此前没有任何机器读过它的字段名——`test_api_doc_endpoint_coverage_l210.py` 只钉「端点路径集合与说明存在性」，`test_doc_contract_numbers_l200.py` 只核数字。缺这一格的直接后果本轮就撞上了：总览散文与三条说明行把写盘类端点的输出字段写成 `output_path`，而真实字段是 `output_file`（`split` 是 `output_dir`），`output_path` 只是路由里的**局部变量名**——文档在教调用方发一个会被 pydantic 拒掉的键。这是 A266 / A275 那句「契约面数字与字段全靠人肉对账」在逐端点说明面上的最后一格，同一族、同一种修法：**权威只有一份**，字段名从 `app.openapi()` 实推（请求体属性递归过 `$ref` / `allOf` / `additionalProperties` / `items`，再加 `parameters[].name`），契约面接不住的名字再要求落进四档有位置的证据（该端点 import 闭包内的字符串字面量、`def` / `class` / 模块名、`fields(AppConfig)` 的键、CLI parser 的选项）。定级 S。**· 已关闭（L223）**：守卫 `tests/unit/test_api_doc_field_hints_l223.py` 23 支，文档 4 处一并订正（全文再无 `output_path`）。两件事值得留在账上而不是埋在日志里：① 字面量档**必须逐格限作用域**，全仓字面量会把 `output_path` 洗白（它确实是 `augmentor/backup.py:192` 与 `augmentor/export_enhanced.py:143` 的返回键），跳数预算由 `test_hop_budget_is_load_bearing` 钉住；② 尺子首跑的 3 格无据里有 2 格是**尺子自己的 bug**（相对导入层级换算错，`augmentor` 包内 `from .x` 全没解析进闭包），判据修好之后才剩下那 1 格真缺陷。⇒ 见循环日志 L223 **· L231 补记**：本轮把 `POST /api/system/backups/{backup_id}/restore` 的响应落点键 `output_path` 照实写进 `docs/API.md`——它是那条端点**真实的响应键**（同族其余五条写盘端点叫 `output_file`，`split` 叫 `output_dir`），与本条订正的「请求侧教人发幽灵键」是两件事。那道「整篇不许出现」的禁令因此按现量**收窄**成「dataset 七条写盘族一行不许出现 + 全文只许 restore 那一行出现，多一行即红」，原缺陷照旧喊红（注入臂 I5：把 convert 那行写回 `output_path` ⇒ 2 支红）。⇒ 见循环日志 L231 | S |
| A278 | OpenAPI 里 `additionalProperties: true` 的自由子对象 | **新立（L223）**：契约面不是全知的。响应模型里凡是 `Dict[str, Any]` 的子对象，在 OpenAPI 上就是 `additionalProperties: true` —— **键名压根不在 schema 里**，机器推不出来。本轮实测的形状：`ImpactResponse.before` 与 `after` 两个子对象（`api/routes/dataset_tools.py:383-384` 声明为 `Dict[str, Any]`，内容是 SDK 那份统计画像的原始 dict），它们内部有哪些键只能由散文说明，新尺子对这类名字只能走「字面量档」，而字面量档是**逐格限作用域**的——一旦某个自由子对象的键来自更深的模块（超过两跳），它就会被报成无据，而那不是文档错。当前后果：客户端只看 OpenAPI 拿不到这些子对象的形状，文档写漏也没有机器兜。两条方向：**① 给自由子对象立最小形状**（哪怕只声明 `count` / `types` 这一类稳定键，其余留在 `additionalProperties` 里），**② 维持自由并由 A277 那把尺子加一档「自由子对象白名单」**，把「这一格只能靠散文」写成可证伪的名单而不是含糊过去。本轮选②的现状（白名单还没立，靠字面量档接住）而没有动形状：改响应模型是对外契约变化。定级 S，待拍。 | S |
| A279 | 前端服务层对后端 15 条操作没有入口（接线队列） | **十一批共接 32 条（L225 三条 + L226 三条 + L227 两条 + L228 三条 + L229 三条 + L230 三条 + L231 四条 + L232 三条 + L234 五条 + L235 一条 + L236 两条）**：`language-profile` 与 `evaluate` 进了数据分析页、`vector-status` 进了系统状态页；L226 把 `search` 进分析页、`validate` 进质量中心、`auto-config` 进配置管理；L227 把 `impact` 与 `semantic-search` 接进数据分析页（四档旋钮全可调：被编码字段、前 K 条、`text_key`、规模增益门槛）；L228 把 `stats`（字段级统计）、`features`（特征普查）、`compare`（双集对比）接进数据分析页；L229 把 `convert`（格式转换）、`merge`（合并）、`split`（分割）接进数据管理页；L230 把 `sample`（采样）、`aggregate`（多源聚合）、`rag`（转成 RAG 检索格式）接进数据管理页——**至此全仓六只会落盘产物的端点全部有页面入口**；L231 把备份那一族四条（列出 / 创建 / 恢复 / 删除）接进系统状态页——**`backup` 这一个子命令名下的四条操作从此在页面上都有入口**，而且是**第一次有一批接线同时改掉一处产品面缺陷**（`name` 越界写盘，见循环日志 L231）；L232 把数据集血缘那一族三条（已登记列表 / 登记 / 依赖图与校验问题）接进系统状态页第四张卡——**这是第二次「接线同轮收掉一处产品缺陷」**（`description` / `tags` 回显即丢，A305 本轮已修，见循环日志 L232）；L234 把系统运维那一面五条接上——`monitor` 与 `auto-test` 进系统状态页第五张卡（都只读、都**不挂**门禁），`migrate` 与 `stream` 进第六张卡（都写盘、都**挂**门禁），`validate-config` 进配置管理页新卡（只读、不挂门禁），这是**第三次「接线同轮收掉产品缺陷」**，而且一次收了三处（A313 / A314 / A315，见循环日志 L234）；L235 把 `health-gate` 接进质量中心新卡「健康门禁」（文本字段 / 四个健康分权重 / 三档门禁阈值 / `block_on_warning` 全可调，并让 `/api/quality/evaluate` 的 `pass_rate` 一键喂进来，**含 `0.0`**），这一批不改产品代码、只量出三条脾气并立成 **A319 / A320 / A321**（权重不校验非负与上界且健康分不钳 [0,1]、和为 1 的容差在恰好 0.01 那一档是假话、`text_field` 空串与幽灵键零判决 ⇒ `duplicate_rate` 1.0 的假判决），三条都同时写在页面正文里，见循环日志 L235。`UNCONSUMED` 32 → 29 → 26 → 24 → 21 → 18 → 15 → 11 → 8 → 3 → 2 → **0**（两员都在 L236 接完）、已消费 43 → 46 → 49 → 51 → 54 → 57 → 60 → 64 → 67 → 72 → 73 → **75**（75 条的 **100%**），`NO_ENTRY` 从 6 一路收到 1、再到 **0**（L236 收掉 A283① 之后 `POST /api/dataset/translate` 有了页面入口）；`CLI_ENTRY` **26 条/21 名 → 23 条/18 名 → 20 条/15 名 → 17 条/12 名 → 14 条/9 名 → 10 条/8 名 → 7 条/7 名 → 2 条/2 名 → 1 条/1 名 → 0 条/0 名**（L236 接掉 `dependencies`，`doctor` 整名出名单）——L225/L227 都只动前端，L226、L228、L229、L230、L231、L232、L234 与 L235 接的条目各有 CLI 子命令，所以动这一半的是那八批；**L234 一批减 5 条 5 名（系统那一族五条各有子命令）是现量最大的单批**，L231 减 4 条（`backup` 整名出名单）次之，L232 减 3 条（`dependency` 整名出名单）第三；剩下那 1 条/1 名就是 `doctor`（L235 把 `health-gate` 接掉之后）。修法照旧按页面分组分批：一批 = 一个页面的 fetchers + 页面入口 + 该页用例。**三件收口判据从第一批起没换过，本轮第十次全部命中**——`test_api_endpoint_consumption_l224.py` 的名单与实测集合相等且变短、`api.wiring.test.ts` 的 `KNOWN_UNWIRED` 仍为空、每员覆盖成功与失败两路（外加一条经验：接线改动本身要让消费守卫先红再回填，红的那几次就是证据）。**L226 立的那条纪律在 L234 被用户显式撤销**（拍板原话「两员都接，做到 75/75」）：`GET /api/system/dependencies` 与 `POST /api/dataset/translate` 都要接。撤销**只针对这两员**——「指标涨而能力不涨的接线一律不做」这句在别处仍然成立，账上留着原话以便日后回看是谁拍的、拍了什么。历史那一段照旧记着：L232 把血缘一族的过滤从 `"dependenc" in p` 收窄成前缀 `/api/system/dependency/`，否则会把 `dependencies` 吞进「血缘一族只有三条」那格判据（首跑实红 2 处，见循环日志 L232）；那次收窄与接不接它无关，接进来之后仍要保持两族分开数。L230 那三员的理由也仍然成立：它们是「SDK 与 CLI 都做完了、页面上连入口都没有」的真缺口，而且是落盘一族。**上一格那条「下一批建议」里点名的系统那一族五条（`monitor` / `stream` / `migrate` / `validate-config` / `auto-test`）L234 全部接完**，而 L234 那一格点名的三员里 `health-gate` 已在 **L235 接完**（质量中心新卡「健康门禁」+ 一键喂 `pass_rate`）。**L236 已收口到 75/75**：① `dependencies` 进系统状态页——不是「多一格指标」，而是把依赖诊断从「综合状态取到才渲染」那道整体门里摘出来（管道拉不动时这一面照旧可读、单独刷新），l224 名册里那条拒接理由同轮改掉；② A283① 的 `output_file` 契约就位后接 `translate`（卡上写明「逐条调模型、按 token 计费」，失败面 `failed` / `failed_indices` / `errors` / `errors_truncated` 四键各有可见读数）。两员收完 `UNCONSUMED` 归 **0**，而 L235 挂着的那句「必须复验空集相等时仍会响」**已复验**：l224 那把尺子带两支牙齿（`test_the_empty_claims_are_not_a_blind_ruler`，以及用 monkeypatch 摘掉一处调用形状的 `test_unwiring_one_operation_turns_the_empty_roster_red`），注入那一臂把 l233 的 `GATED_NOT_YET_CONSUMED` 塞回一条也当场红 ⇒ 收口没有把尺子读成恒绿。**这一维本轮已顺手验过一次**：把 `health-gate` 从名单里删掉之前，`test_unconsumed_matches_the_roster` 首跑就报「已接好或已消失 ['POST /api/quality/health-gate']」，说明它读的是实测集合而不是名单自身；空集那一档仍要在收口轮再验一次。同批可顺带处理 **A281** 里 `AppConfig` 少 `versioning` / `visualization` 两处（配置管理页旁边已有新卡，**仍待拍**）。**· 已收口（L236）**：两员都接完，消费面 **75/75**，l224 那三个名单（`UNCONSUMED` / `CLI_ENTRY` / `NO_ENTRY`）同时归空集。 | ~~M~~ 已收口（L236） |
| A280 | 8 个路由文件把下游异常一刀切收敛成 500，「资源不存在」拿不到 404 | **新立（L224）**：本轮为核对 `GET /api/versions/history` 的运行时归属顺手实测——`GET /api/versions/v_ghost` 返回 **500** + 固定文案（`api/deps.py` 的 `INTERNAL_ERROR_DETAIL`），而 SDK 抛的是 `VersionError`（`augmentor/exceptions.py:133`，`AugmentorError` + `ValueError` 双继承）；同一份路由文件里三种归宿并存：`/api/versions/create` 源文件缺失 → 404（走 `require_file`）、`POST /api/versions/v_ghost/rollback` → **200 且 `success:false`**、`DELETE /api/versions/v_ghost` → **200 且 `success:true`**（`VersionManager` 静默成功）。**这不是无声 bug，是已被钉死的行为**：`tests/integration/test_api.py:479` 两支用例的 docstring 明写「应返回 500」。普查（按调用点数，不含 import 行）：只用 `to_http_error` 分档的 4 个文件 30 处（dataset_tools 14 / system_ops 12 / language 2 / vector 2），`config.py` 与 `data.py` 两把混用（分档 3 处 + 一刀切 8 处），**只用 `raise_internal_error` 一刀切的 8 个文件 29 处**（quality 9 / export 4 / augment 3 / multimodal 2 / audit 1 / leakage 1 / privacy 1 / version 8）。两条方向：**① 把语义明确的「不存在」类异常接进 `to_http_error(not_found_types=...)`**（`BackupError` 已是这一档的先例，见 `api/routes/system_ops.py`），逐文件收窄——改的是对外状态码，上面那两支钉死 500 的用例要同批改；**② 维持现状并把「除 `FileNotFoundError` 外一律 500」写成契约**，`docs/API.md` 里相应把 404 档从这些端点去掉。本轮选**不动，只立账**：状态码是对外契约，一刀切里混着的另一半（400 档 `to_http_error` 会把 `str(exc)` 原样回传）正是 A155① 盯住的泄漏面，一次改八个文件不是一轮的量。定级 S，待拍。 | S |
| A281 | 3 只服务层 fetchers 的 TS 响应类型缺契约里已经存在的字段（后端在回、前端读不到） | **新立（L225）**：接线时顺手量了一遍「`web/src/services/api.ts` 里 `Promise<Xxx>` 的字段集，与该操作契约 200 响应的属性集是否相等」（仓外一次性脚本：正则解析类型 + `app.openapi()` 的 `components.schemas`）。45 只带类型标注的 fetchers 里 **40 只逐字相等**（含本轮新增的三只），**3 只缺字段**，另有 **2 只 naive 正则解析不动**。三处缺口的方向完全一致，都是「契约有、TS 没有」：① `AppConfig` 少 `versioning` 与 `visualization` 两节 ⇒ 配置页拿不到这两节的当前值（与 **A276** / **A271** 相关但不等价——那两条问「这些键怎么处置 / 要不要可写」，这一条问「读侧的类型里根本没有」）；② `UpdateConfigResponse` 只有 `success` 与 `message`，少 `ignored_keys` ⇒ 后端明确回了「哪些键被忽略」，页面不显示，用户会以为自己已经写进去了；③ `StatusResponse` 少 `model_retry_count` 与 `model_retry_wait_seconds` ⇒ 系统状态页四张卡看不到重试的生效值。解析不动的那两只不是缺口，而是**这把尺子进仓前必须先解决的两种形状**：`AugmentProgress` 是联合类型别名（`types/api.ts:133`，由 `NoCheckpointProgress` 与 `AugmentProgressDetail` 二选一），`VersionDetail` 用 `extends VersionInfo` 把字段拆进继承（`types/api.ts:218`）——一行一字段的读法对这两只直接失效。修法两步：先补类型（纯前端，零风险），再决定是否真显示（只显示不渲染仍是死面，那要按 A138 的接线档继续走）。**为什么本轮不补**：本轮已含三张新卡与八支新用例，再掺类型改动会把「接线」这条判据的证据链掺浑；且这条测量目前只是仓外脚本，要做成守卫得先定下别名、继承、嵌套对象三条解析口径，并配 `KNOWN_UNMATCHED` 白名单加规模钉（形状与 `api.wiring.test.ts` 同构）——那是一轮的活。定级 S。**· 已收口（L237）**：立账的 3 处不是错，是浅——把尺子做深一层重量是 **12 处**（`AppConfig` 缺 9 / `StatusResponse` 缺 2 / `UpdateConfigResponse` 缺 1，反向幽灵 0），因为 `ConfigResponse` 九个节在契约里是 `additionalProperties: true` 的自由字典、没有 `properties`，节内 7 键不进契约 ⇒ 守卫分两层（第一层静态比契约属性集、第二层打只读 `GET /api/config` 递归比键树）。12 处全补齐，且**补齐之后同轮渲染**（系统状态页两格重试读数、配置管理页两只新卡），不是「类型上有了、页面上看不见」。守卫 `tests/unit/test_ts_contract_types_l237.py`（21 支 / 5 面）落成：75 只带类型 fetchers 全数相等，`unannotated` / `unresolved` / `UNMEASURABLE` 三格皆空集，每格都配注入臂证明仍会响。同轮量出并新立 **A323**（toast 措辞与后端 `message` 不同源）。 | ~~S~~ **已收口（L237）** |
| A282 | 前端 `npm run format:check` 的仓内基线是红的，而 CI 里没有这道门 | **新立（L225）**：本轮跑格式检查时实测 `prettier --check src` 报 **32 个文件**不合 `.prettierrc.json`（`semi: false` / `singleQuote: true` / `printWidth: 100` / `trailingComma: all` / `arrowParens: always`），而页面代码普遍是省略箭头函数括号的旧写法。**本轮没有引入漂移**，取证方式是把碰过的 5 个文件用 `git show` 取到仓外临时目录、拿同一份配置复查，`HEAD` 版本同样 5 个全红 ⇒ 跟着周围代码写才是对的选择。CI 侧 `.github/workflows/ai-platform-ci.yml` 的前端 job 只跑 `npm run lint`、`npx tsc --noEmit` 与 vitest，**没有 format 门**，所以这道红可以长期存在而不被任何人看见。两条方向：**① 一次性 `npm run format` 全量重排，并把 `format:check` 加进 CI**——此后 diff 与 IDE 一致；代价是那一个提交会淹没同批的真实改动，且重排要在事后复测一遍行号类文档守卫。**② 承认现状，把 `.prettierrc.json` 改成与仓内实际风格一致**（至少 `arrowParens: "avoid"`），让这道门从「恒红」变成可用。本轮选**不动，只立账**：两种改法都是全仓重排或配置改，与接线一批混在一起只会污染 diff。定级 S，待拍。 | S |
| A283 | `POST /api/dataset/translate` 把整份译文只放在响应体里，没有任何落盘口，前端接了也留不住 | **已关闭（L236，①②都做完了）**：①契约——`TranslateRequest` 与 `TranslateResponse` 各加可选 `output_file`（键**恒在**、没给时 `null`），给了就 `resolve_data_path(for_write=True)` **先解析落点**、`atomic_write_json` 原子写进白名单数据根、响应回显**已 resolve 的绝对路径**；越界落点 403 且模型调用 **0** 次。②接线——数据管理页新卡「批量翻译」，带计费确认闸、四格输入与失败回执四键。同轮量出 **A322**（`text_key` 零判决）并立账，后端一行未改。原立账原文照录 —— **新立（L226）**：A279 第二批挑接线对象时实测出来的契约层缺口。① `TranslateResponse`（立账时引 `api/routes/language.py:88`；那一格在 L236 之前就已漂到 `:94`，本轮加了两段 docstring 与一只请求字段之后是 `:105` ⇒ 同轮重钉。**这处漂移此前没人发现是有原因的，但原因不是「没人扫」**：`test_doc_line_refs_l79` 只读 `docs/ARCHITECTURE.md` 与第一本账，而 A184 那把尺子**是扫第二本账的**——它判的是**落点形状**（那一行是不是代码跨度），不核**行号是否还指着当初那一格**，所以 `:88` 从 `items` 那一行漂进 docstring 里它照样放过）里 `items: List[Dict[str, Any]]` 就是**整份翻译后的数据集**，请求模型没有 `output_file`，路由也不调 `_dump`——同仓的写盘变换类端点（`convert` / `merge` / `aggregate` 等）都走 `api/routes/dataset_tools.py` 的 `_dump` 落到白名单数据根，这一只没有；② CLI 侧同样没有 `translate` 子命令（`augmentor/cli/parser.py` 的 `add_parser` 名单里查不到），所以它是 A279 的 `NO_ENTRY` 三员之一，「SDK 做完、web 与 CLI 两边都没人取」；③ 花钱面：这条路径逐条调 `model_backend.generate`，挂了 `verify_api_key`（同文件 13 行的鉴权分档说明），`multilingual.enabled` 出厂 `false`。**为什么因此不接**：UI 若只是「点一下把译文显示出来」，用户刷新页面产物就没了，而每一次点击都在按 token 付费；要做成能用的入口，先得让这次花钱留下东西。修法两步：**① 契约**——给 `TranslateRequest` 加可选 `output_file`，非空时走 `_dump` 落白名单根并让响应带一个可核对的落点字段（与 `convert` 同形，顺带把 `failed_indices` 的「指回 `input_file`」这句话变成可操作的：用户看得见原文也看得见译文）；**② 接线**——契约就位后再进多模态/数据管理那一侧的页面，成功与失败两路各自要有可见读数（`failed` / `errors_truncated` 是 L221 刚补的失败面，只显示 `total` 等于把它再吞一次）。**本轮不做 ①**：它改的是对外请求与响应形状，A279 立项时写死「不改后端字段名或响应形状」，混进接线轮会让「接线」这条判据的证据链掺浑。⇒ **①②均在 L236 做完**，见循环日志 L236。 | ~~M~~ 已修（L236） |
| A284 | `AugmentationMetrics.unique_instructions` 的键名与实现不符 | **新立（L227）**：`augmentor/impact.py:76` 那一格赋的是 `len(set(texts))`，而 `texts` 取自 `self.text_field`（默认 `instruction`，见同文件 65 行）。换字段就是在数另一个东西，名字却还写着 instructions；`to_dict()`（同文件 24 行，键在 27 行）把它原样送到 API 响应面上，所以这是**契约面上的谎**而不是局部变量名。L227 只做了前端侧的诚实化：页面标签改成「唯一值数（按所选字段）」，并由 `test_impact_semantic_wiring_l227.py` 钉住「标签必须含『字段』、不得含『指令』」，**键名本身没动**——它同时是 SDK dataclass 属性和响应字段，改名是破坏性变更，不属于「接线」这一档工作量。两条候选处置：① 加 `unique_values` 别名 + 弃用旧名，跨版本删；② 只改 `docs/API.md` 的口径说明，把键名当既成契约。判据上的差别是 ①：`_dump` 出来的 JSON 键会变。定级 S，待拍。 | S |
| A285 | `["instruction", "input", "output"]` 这份字段清单在仓内有五处手写字面量，没有权威常数 | **新立（L227）**：`augmentor/audit.py:68`、`augmentor/privacy.py:76`、`augmentor/cli/parser.py:173 / 405 / 430` 各写一遍，语义都是「默认看这三格」。直接后果是 L227 那把跨语言尺子**建不出来**：`text_field` / `text_key` 的合法取值来自数据集本身，而这份字面量既不是常数也不能代表合法域，引它当权威等于自造一套——所以守卫只能退到「页面发得出的每一档都被端点接受」这种运行时判据（`TestEndpointAcceptsWhatTheUiSends`）。处置建议：抽一份模块级常数（比如叫 `DEFAULT_TEXT_FIELDS`）并把五处改为引用，归属模块待拍——本轮刻意不指名新模块，因为 **A184 那把注释引用尺子会把「指向仓内并不存在的文件」记成 dead_path**（第一版给的就是一个还没建的新模块名，被它当场红两支，这正是这把尺子该有的反应）。常数就位之后，l226 那把「下拉清单 vs 后端封闭清单」的尺子就能复用到这一族字段上，L227 守卫里那档运行时判据也可以升级成逐项对账。定级 S，待拍。 | S |
| A286 | `tests/unit/test_analytics_performance.py` 的比值型计时守卫在共享工作机上会假红 | **新立（L227）**：全量门禁复跑实测到一次假红——`stays_linear` 那条比值量到 **4.38**（阈值 3.0，线性档实测 2.0–2.5、退化到 O(n²) 实测 3.1–3.9），而同一条用例本轮第一次全量是绿的、之后隔离复跑四次全绿；红那一次整轮耗时 **257.50 s**，比绿那次的 188.88 s 高 36% ⇒ 是并发负载压在小档计时上，不是算法退化。**为什么这条值得立账**：退化档与噪声档在墙钟上是同一个数（4.38 比 O(n²) 档还大），意味着真来一次退化也可能被当成噪声放过。三条候选：**① 每档取 N 次的中位数或最小值**而不是单次；**② 用计数代替墙钟**（数内层重算次数，确定性、与负载无关）；**③ 把两档规模一起放大**让噪声占比下降（文件里已有这条口径：实现变快了就先放大规模，不许放宽比值阈值）。建议 ①+② 一起做——② 才是判据，① 只是让它稳。本轮**既不动阈值也不改实现**。定级 S，待拍。**L239 收口（按 ②，① 只留作兜底）**：判据换成「整趟 `builtins.sum` 被调几次」——`counted_sum_calls()` 把 `builtins.sum` 临时换成原样转发并计数的替身，`finally` 里复原并断言复原成功；现量 `_calculate_diversity_score()` 在 n=1000/2000/10000/20000 **四档同为 2**（一次求均值、一次求方差和），n=0 走空档早退 ⇒ 0 ⇒ 判据与负载、与机器快慢彻底无关。三条牙齿：复刻退化档（均值写在生成器里）现量调用数 **n+1**（5000→5001、10000→10001）、空档 0 次且分数 0.0、窗口内抛异常也必须复原真函数。`RATIO_CEILING` / `MIN_MEANINGFUL_MS` 与 `test_ratio_ceiling_separates_linear_from_quadratic` 退役；**撤下它不是放宽**——那条用例量的是「这台机器此刻的 CPU 给多少」，现在量的是「算法跑了几次求和」。保留的两条各测一轴：`ABSOLUTE_BACKSTOP_MS`（2000 ms，现实现约 70 ms，20 倍松、只在数量级出事时红）与 `test_the_timer_can_see_scale`（2000→8000 必须明显更慢，证明计时器仍有分辨力、别让任何读数把「耗时不随规模变」当证据）。注入取证：把均值改回生成器里，**返回值一字不变**、只有计数抓得到 ⇒ 这条判据测的正是旧判据测不到的那一维。 | ~~S~~ **已收口（L239）** |
| A287 | `augmentor/comparison.py` 的 `similarity_to_a` / `similarity_to_b`（双集对比质量面的两个键） | **新立（L228）**：接线 `POST /api/dataset/compare` 时顺手普查这两键的写者与读者。机器锁是**双向**的：源码面全仓（不含 import 行）**只有两处出现**——dataclass 默认值 `similarity_to_a: float = 0.0`（声明在同文件 60、61 两行，送出在 87、88 两行的 `to_dict()`），**没有任何赋值点**；行为面同锁——真数据（4 条 vs 2 条、instruction 一句不重）跑一次回执里两键都是 `0.0`。所以这不是「暂时没算」而是**结构上恒零的死面**，而它住在对外契约里：`docs/API.md` 全文查不到这两个名字，OpenAPI 的响应面上却有格子。当前后果：客户端读到 `0.0` 会理解成「A 与 B 毫不相似」，真相是「这一档从未被计算」——一个恒零的数比没有这个数更会骗人。两条处置方向：**① 接上计算**，但语义得先拍清楚（`similarity_to_a` 是「B 相对 A 的相似度」还是别的？重叠面已经有 `similarity_score` 一个数，再挂两个近义数是给消费面添混乱，不是补功能）；**② 从响应面删掉这两键**（破坏性变更：同时动 SDK dataclass、`to_dict()` 与契约面，既有用例里按位置构造这两键的点要跟着改）。L228 只做了前端侧的诚实化——页面**不渲染**这两个键，并由 `tests/unit/test_stats_features_compare_wiring_l228.py` 的 `TestDeadAndNullableFaces` 同时钉住「无赋值点」与「页面上没有它们的名字」，键本身一格未动。定级 S，待拍。 | S |
| A288 | `augmentor/compare_enhanced.py:190` 的 `size_ratio`：B 侧 0 条时 SDK 存 `float('inf')`，到线上是 `null` | **新立（L228）**：那一格写的是 `len(items_a) / len(items_b) if items_b else float('inf')`，而同一份 dataclass 把它声明成 `size_ratio: float`（同文件 32 行的声明、45 行的 `to_dict()` 原样送出）。实测的形状是三段式的：SDK 里是 `inf` ⇒ 经过 JSON 到浏览器是 **`null`**（响应体本身仍是合法严格 JSON，探针逐条验过）⇒ 而契约面上它写的是 `number`。也就是说**声明、运行时值、线上形状三者互不相同**，任何一方单独看都会读错：只看 OpenAPI 的客户端当它是数字，`null` 进到 `toFixed` / 算术里直接变 `NaN`；只看 SDK 的人会以为它可能是 `Infinity`（那不是 JSON 的值）。当前后果与处置：本轮把前端改成按**线上真相**说话——`web/src/types/api.ts` 里收成 `size_ratio: number | null`，页面读到 `null` 时显示「—（B 侧 0 条：SDK 存 `inf`，JSON 面为 `null`）」而不是一个假数；并由新守卫同时钉住三处（类型里有 `| null`、页面有 `null` 分支、真跑 B 侧空集那一对时回执里确实是 `null`）。两条后续方向：**① 让 SDK 不再发 `inf`**（显式 `None` 或一个可判的上限哨兵），拍的是「B 侧为空时 A ÷ B 该表示什么」这个口径；**② 维持现状并把「`inf` 上线为 `null`」写进 `docs/API.md` 的该端点字段说明**——注意现在文档里根本没有 `size_ratio` 这个名字，所以 L223 那把逐端点字段提示尺子对它没有任何覆盖。本轮选**不动后端形状，只把前端与守卫改成诚实的**：改响应值域是对外契约变化。定级 S，待拍。 | S |
| A289 | `POST /api/dataset/split` 的两条支路（`stratify=False` / `True`）用的不是同一套判据，同一份输入在两侧会得到不同的判决甚至不同的产物 | **新立（L229）**：接线分割卡时逐档实测出来的三处不对称。① **比例之和的容差不同**：默认支是 `abs(total_ratio - 1.0) > 0.01`（`augmentor/dataset_ops.py:427`），分层支委托 `DataSplitter`，那一支是 `abs(total - 1.0) > 1e-6`（`augmentor/data_splitter.py:63`）⇒ 实测 0.5/0.25/0.255 这种「差 0.005」的比例默认支**放行**、分层支**拒绝**。② **负数只有一支拦**：`比例不能为负数` 那句判据只在分层支（`augmentor/data_splitter.py:67`），默认支拿到负比例照切不误。③ **判决文案是两句不同的话**，客户端按字符串匹配就会在两侧各写一套。④ 还有一处更贵的：**`shuffle` 在两支上语义不同**（20 条语料 0.5/0.25/0.25 实测）——分层支只把每段成员按输入的原始相对顺序回排，成员与条数都不变、`seed` 仍决定谁进哪段；默认支是「先整份打乱再按位置切片」，所以 `shuffle=False` 会**连同成员一起改变**，且三段恒为输入的前 50%/25%/25%，此时 `seed` 完全空转。**当前处置**：页面卡片上把 ①② 两句写死（「比例之和的容差默认支是 0.01、分层支是 1e-6，负数只有分层支会拦」），④ 记在请求模型的 docstring 里；守卫 `tests/unit/test_transform_wiring_l229.py` 同时钉住「两句判决的原文」与「+0.005 两支一过一拒、负数两支一过一拒」四格行为。**修法方向**：① 把两支的校验合并成一处（谁先判、用哪个容差要拍），② 或者至少让两侧文案同源，③ `shuffle` 那面要么在两支上统一语义，要么在契约面上改名（`shuffle` 在分层支上其实是「保持原始相对顺序」）。定级 M，待拍。 | M |
| A290 | `POST /api/dataset/merge` 只转发 `MergeConfig` 四档旋钮里的一档，回执只上线 SDK 六键里的五键，而响应模型的 docstring 写着「一一对应」 | **新立（L229）**：① **旋钮面**：`MergeConfig` 有四档（`deduplicate` / `dedup_threshold` / `preserve_order` / `max_items`，声明在 `augmentor/dataset_ops.py:27`），而端点构造时只喂了第一档（`api/routes/dataset_tools.py:787` 那一行就是 `MergeConfig(deduplicate=request.deduplicate)`）⇒ `dedup_threshold` 的宽松/保守两档（L150 立的语义：`< 0.9` 启用大小写折叠 + 空白归一并删近似重复）、`preserve_order`、`max_items` 三档在 HTTP 面上**调不到**，CLI 与页面都一样。② **回执面**：`merge_files` 返回六键，其中 `truncated_by_max_items`（`augmentor/dataset_ops.py:220`）被 `MergeResponse`（`api/routes/dataset_tools.py:270`，五个字段）丢掉 ⇒ 而该模型的 docstring 明写「与 `DatasetOperations.merge_files()` 的返回键一一对应」，**这句是谎**。两件事叠起来有一个具体后果：即使哪天把 `max_items` 接上，用户也看不见「被截断了几条」，只会看到 `total_output` 变小。**当前处置**：页面卡片写明「这一档只控『开 / 关』：SDK 里那个宽松 / 保守两档的去重阈值在这只端点上不可调（见账本 A290）」，守卫 `test_the_merge_endpoint_forwards_one_knob_of_four_and_reports_five_of_six` 把两半各钉一格（请求模型字段集 vs `MergeConfig` 字段集、响应模型字段集 vs `merge_files` 返回键集），并另有一支钉住「页面上不许出现假的『预览合并结果』」。**修法方向**：① 最小改动——`MergeResponse` 补 `truncated_by_max_items` 并把 docstring 改成实话；② 接旋钮——请求模型补三档并逐个接线（注意 `max_items` 会改变产物条数，回执不带截断数就是骗人，所以 ① 必须先于 ②）；③ 反向收窄（「把 SDK 那三档删掉，缺口就没了」）**不成立**：`merge_files` 内部三档都有真消费者（`augmentor/dataset_ops.py:157` 的 `preserve_order`、`:163` 的 `dedup_threshold`、`:169` 的 `max_items`），所以缺口只能在入口面上补。定级 M，待拍。 | M |
| A291 | 转换端点在 `source_format` 是空串时回的判决缺主语：「无法从  转换到 JSON」 | **新立（L229）**：请求模型里 `source_format` 的默认是 `None`（＝「按扩展名推断」），但**空串不是 `None`**：实测 `{"source_format": ""}` ⇒ **400**，detail 是 `无法从  转换到 JSON`——那句 f-string 在 `augmentor/converter.py:327`，主语位置插进去的就是那个空串，于是句子里留着两个连续空格且**说不出是什么格式转不了**。缺省不发这一格则是 **200**（走扩展名推断），所以两者不等价：空串不是「未声明」的同义词。**为什么页面碰不到**：服务层全文只有一处 `|| undefined` 哨兵，把页面上的空串丢掉再发；能踩到这句判决的是直接调 HTTP 面的人（脚本、第三方客户端、curl）。**当前处置**：守卫钉住三面——请求模型默认是 `None` 而不是 `""`、服务层那一处哨兵存在且只有一处、空串与缺省在线上不等价（一个 400 一个 200）。**修法方向**：① 把空串归一成「未声明」（校验器里 `"" → None`），判决就不再有缺主语那一档；② 保留 400 但把文案改成能读的（「未声明源格式，且扩展名推不出输入格式：`<文件名>`」）；③ 两者都做——① 是行为变化（原本 400 的调用会变成 200），要先拍。定级 S，待拍。 | S |
| A292 | `augmentor/dataset_ops.py` 的 `SampleConfig.ratio`：这一格没有任何判决，同一个越界值在三种方法上给出三种结局 | **新立（L230）**：接线采样卡时逐档实测（20 条语料）。`size` 有域校验 `require_count("size", config.size)`（`augmentor/dataset_ops.py:291`），而 `ratio` 那一支只有 `elif config.ratio is not None:`（`augmentor/dataset_ops.py:296`）后面一句 `int(len(items) * config.ratio)`——**既不判上界也不判负数**。于是：① `ratio=2` ⇒ `random` 支 **400**，detail 是 CPython 自己的英文 `Sample larger than population or is negative`（不是本仓的中文判决），`systematic` 与 `stratified` 两支 **200 且回整份 20 条**；② `ratio=-0.5` ⇒ `random` **400**、`systematic` **200 且回 10 条**（负数被读成切片下标，「去掉末尾 10 条」）、`stratified` **200 且回 0 条**；③ `ratio=0.0001` ⇒ 三支都 **200 且回 0 条**（`int()` 截断）。**当前处置**：页面卡片写明「`ratio` 越界只有 random 支会拦，另外两支要么给整份要么给零份」，守卫把三档 × 三支的行为逐格钉住；`ratio` 与 `size` 在页面上是二选一（给一个就清另一个），所以踩不到「两格都发」那一档。**修法方向**：① 把 ratio 的域校验提到三支之前（`0 < ratio <= 1`），一处判决三支共用；② 或者至少让三支对同一个越界值给同一种结局。注意 ① 是行为变更：今天 `systematic` 上 `ratio=2` 是 200。定级 M，待拍。 | M |
| A293 | `api/routes/dataset_tools.py:887` 构造的是无参 `DataAggregator()`，三只旋钮在 HTTP 面调不到；回执是两套手维护的键集且已漂移 | **新立（L230）**：① **旋钮面**：`DataAggregator.__init__` 收 `key_fields` / `value_fields` / `random_state` 三档（`augmentor/aggregator.py:57`），而端点构造的是**无参** `DataAggregator()`（`api/routes/dataset_tools.py:887`）⇒ 三档在 HTTP 面与 CLI 面都调不到。直接后果三条：去重永远按 `instruction`、`consistent` 永远按 `output` 比答案、weighted 那一支的抽样**不可复现**（每次跑可能给不同产物，而写盘端点给不出可复现产物是硬伤）。② **回执面**：`AggregationResult.to_dict()`（`augmentor/aggregator.py:37`）与 `AggregateResponse` 是**两套手维护的键集**，实测已经漂移——SDK 侧多 `total_count` 与 `conflict_count`，线上侧多 `output_file` 与 `aggregated_count`；端点没有走 `to_dict()` 而是手抄了四格（`api/routes/dataset_tools.py:891` 起）⇒ 与 **A290** 同族：「与 SDK 返回键一一对应」这句话在这一只端点上也不成立。③ **`conflicts` 整份上线**：那一格把每条冲突的原文都塞进响应体（实测一组冲突 133 字节起，随语料线性涨），与本仓写盘类端点「只回写到哪、写了多少」的回执口径不一致。**当前处置**：页面把「去重键 / 一致性键不可调」与「冲突明细逐条渲出」两句写死在卡上，守卫同时钉住两套键集的**两个差集**与「端点不走 `to_dict()`」这一格。**修法方向**：① 请求模型补三档旋钮并接进构造（`random_state` 接上才谈得上可复现）；② 回执改成引用 `to_dict()`，或把两套键集合并成一份权威；③ `conflicts` 要么截断（带 `conflicts_truncated` 计数，同 L221 的翻译失败面）、要么落盘只回条数。定级 M，待拍。 | M |
| A294 | 聚合端点的 `target_size` 只咬 weighted 那一支，另外三支给什么都一样 | **新立（L230）**：四种策略里**只有 weighted 真的读 `target_size`**（回落点 `augmentor/aggregator.py:159` 的 `target_size = DEFAULT_TARGET_SIZE`），union / intersection / consistent 三支拿它完全空转。实测（20 条语料）：`target_size=1` ⇒ weighted **1 条**、其余三支都 **20 条**；`target_size=0` ⇒ **200 且 0 条**（0 是合法值，不是「未设置」）；`target_size=-1` ⇒ **400**；**省略 ⇒ 200**，走 `DEFAULT_TARGET_SIZE = 100`——这一格此前 `docs/API.md` 写的是「省略会 400」，是假话，本轮已订正并由四支文档用例钉住。**为什么值得立账**：契约面上四支共用一只字段，而它只在一支上有语义 ⇒ 客户端把 `target_size` 当成「聚合结果条数上限」就会读错三份产物。**当前处置**：页面卡片写明「`target_size` 只咬 weighted 那一支」，留空即不发那一格（服务层不写第二套默认，`DEFAULT_TARGET_SIZE = 100` 全仓只有 SDK 一处）。**修法方向**：① 把这一格改成 weighted 专用（请求模型上说明，或拆成 `weighted_target_size`）；② 让四支都尊重它（是行为变更：今天 union 给 20 条，接上会变）；③ 维持现状但把「只对 weighted 生效」写进 `docs/API.md` 的字段说明（本轮已按 ③ 落了订正）。定级 S，待拍。 | S |
| A295 | 「转成 RAG 检索格式」的默认档在仓内有三处字面量、两个不同的值，且请求侧多送的分块旋钮被静默忽略 | **新立（L230）**：配置出厂默认是 `"llamaindex"`（`augmentor/config.py:533`）、SDK 的 `RAGFormatter.format` 形参默认也是 `"llamaindex"`（`augmentor/rag.py:187`），而端点那一行是 `fmt = request.format or rag_config.default_format or "langchain"`（`api/routes/dataset_tools.py:927`）⇒ **第三档兜底写的是另一个格式名**。今天它不可达（配置那格永远非空，除非有人把 `rag.default_format` 显式设成空串并绕过校验），但它是一只埋着的谎：真兜到那一档时产物会是 langchain 形状，而文档与配置都说 llamaindex。另两处实测：① **空串 `format` 与省略等价**（两者都回 `llamaindex`），所以服务层那一处 `|| undefined` 在这一只端点上是**形状卫生**而不是躲判决——与转换卡那处 `source_format` 的动机不同（**A291**：那一处送空串会 400）；② 请求里多送的 `chunk_size` / `chunk_overlap` **被静默忽略**：端点构造 `RAGFormatter` 时只读配置那两格（`api/routes/dataset_tools.py:929`），调用方以为能按请求调分块。**当前处置**：页面只给三档下拉（含「服务端默认」那一档，留空即不发），守卫钉住「服务层不写第二套默认」与「页面默认档同源于 `RAGConfig` 的 dataclass 默认」；`docs/API.md` 的 rag 行已按实测订正（写明出厂默认 `llamaindex` 与那第三处字面量）。**修法方向**：① 删掉 `or "langchain"` 那一档兜底（配置校验已保证非空）；② 或把它改成与出厂默认同源的一个常数；③ 分块两格要么接进请求模型、要么在文档里写明只读配置。定级 S，待拍。 | S |
| A296 | 分层抽样只按字段值的**前 10 个字符**分组，前缀相同的不同类会静默塌进同一组 | **新立（L230）**：`group_key = value[:10] if value else "empty"`（`augmentor/dataset_ops.py:354`，源码注释自己写着「简化：使用前10个字符作为分组键」）。实测（夹具 `AAAAAAAAAA甲` / `AAAAAAAAAA乙` / `BBBBBBBBBB丙`，`size=2`、`seed=7`）⇒ 只回 `['AAAAAAAAAA乙', 'BBBBBBBBBB丙']` 两条，`甲` 那条**永远采不到**，而回执里没有任何一格说明发生过折叠。相邻的一格是**空键出声、幽灵键不出声**：`stratify_key` 给空串是 **400**（`augmentor/dataset_ops.py:347` 的 `分层采样需要非空的 stratify_key`），给一个数据集里**根本不存在的字段名**却是 **200**——所有条目的 `item.get(key, "")` 都是空串，落进同一个 `"empty"` 组，配额等于全量池子的一次抽样（这一格 L229 已在卡片上写明「塌成一个分组」）。**为什么这条比 A289 那类更贵**：折叠是**按数据内容**发生的，同一份代码在短标签数据上完全正常，在长文本字段（比如用 `instruction` 分层）上就会把不同的类当成一类，而产物条数看起来正常。**当前处置**：页面在分层字段那一格写明「只按前 10 个字符分组」，守卫钉住折叠行为与「空键 400 / 幽灵键 200」两格判决。**修法方向**：① 用完整值当分组键（是行为变更：分组数变多、每组配额变小）；② 保留截断但在回执里加一格「发生了折叠的组数」；③ 把截断长度做成可配。定级 M，待拍。 | M |
| A297 | `consistent` 策略把**同一来源内部**的答案分歧也算成冲突，而那些条目永不落地 | **新立（L230）**：这一支的判据是「同一个 key 下的所有 (来源, 答案) 对，答案集合大小为 1 才落地」（`augmentor/aggregator.py:227` 起），**它不区分分歧是跨来源还是同来源内部**。实测两格：① 同一个来源里出现两条同 key 不同答案（回执 `sources` 是 `["甲", "甲"]`）⇒ 被算成**一条冲突**，且这些条目**永不落地**（`aggregated_count` 是 **0**）；② 同 key 同答案但 `input` 不同 ⇒ **正常落地 1 条且不算冲突**（因为只比 `output`，见 **A293** 的「一致性键不可调」）。**为什么值得立账**：docstring 写的是「仅保留对相同 key 答案一致的数据」，用户读成「跨来源交叉验证」，而实现是「同 key 的所有出现都必须一致，包括同一份文件里的重复提问」⇒ 一份内部有重复提问的数据集会让整组产物为空，而回执只说 `aggregated_count: 0` 加一串冲突，不告诉你冲突的两边其实是**同一个来源**。**当前处置**：页面把冲突明细**逐条渲出**（含来源名），用户至少能看见 `["甲", "甲"]` 这种形状；守卫钉住「同源分歧算冲突且条目不落地」与「同答案不同 `input` 不算冲突」两格行为。**修法方向**：① 冲突判据改成按来源去重后再比（同源内部的多答案另立一格回执）；② 或在冲突明细里显式标注「同源内部」，让 0 条产物可解释。定级 S，待拍。 | S |
| A298 | `POST /api/system/backups` 的 `name` 不查重复：同名创建两次 ⇒ 磁盘一份、索引两行，而 `DELETE` 按 id 把两行一起清掉 | **新立（L231）**：接线备份卡时逐档实测。① 同一个 `name`（实测 `mybak`）创建两次 ⇒ `GET /api/system/backups` 回**三条**（时间戳 id 一条 + `mybak` 两条），磁盘上只有**一份** `mybak.json`（第二次 `atomic_write_json` 覆盖第一次），两行的 `timestamp` 不同（实测 `.991804` 与 `.997820`）而 `backup_path` **逐字相同** ⇒ 前端行键不能拿 `backup_id` 当唯一键（本轮页面用 `backup_id#timestamp`）。② `DELETE /api/system/backups/{backup_id}` 按 id 过滤索引 ⇒ **两行一起消失**，磁盘那一份也删掉；此后同 id 再恢复 / 再删都是 **404**「备份不存在」。③ 相邻的一格：`name` 带路径分隔符**放行**——实测 `sub/deep` ⇒ **200**，在备份目录下建 `sub/` 再落 `deep.json`；带 `..` 才 400（L231 补的那道门）。**为什么值得立账**：索引是「备份有哪些」的唯一权威，而它可以合法地含有两行指向同一份文件的记录，删除的粒度（按 id 全清）与创建的粒度（按 name 追加）不是同一把尺子 ⇒ 用户看到的条数与磁盘上的份数可以长期不等。**当前处置**：页面行键拼时间戳、卡片写明「同名再备份会覆盖磁盘那一份、索引里却多一行，删一次两行一起走」，守卫把「两行一文件」与「一次删除清全部」两格行为各自钉住。**修法方向**：① 创建时按 `name` 查重并覆盖索引行（行为变更：列表条数变少）；② 或让 `backup_id` 真的唯一（同名追加时间戳后缀），索引与磁盘一一对应；③ 维持现状但在回执里带一格「本 name 已有 N 行」。定级 M，待拍。 | M |
| A299 | 备份的 `checksum` 算了、存了、比了，但**从不拦截**：改坏的备份照样恢复成 200 | **新立（L231）**：`DatasetBackup.restore()` 会重算校验和并与索引里那一格比对，不匹配时只 `logger.warning("备份文件校验和不匹配，文件可能已损坏")` 然后**继续恢复**。实测把 `.backups/ck.json` 的内容整份换成一条别的记录再打 `POST …/restore` ⇒ **200**，回执 `item_count 1`，落盘的就是被改坏的内容；`GET /api/system/backups` 里那一行的 `checksum` 仍是**创建时**的旧值（不会随文件重算）。**为什么值得立账**：备份的语义是数据保全，而「保全下来的那份已经坏了」这件事只写在服务端日志里——HTTP 面与页面上都读不到，用户拿到 200 就会认为恢复成功。**当前处置**：页面卡片写明「校验和不匹配只留一句 WARNING 日志、不拦恢复」，守卫钉住「改坏的备份仍 200」与「回执不带校验和那一格」两格现量。**修法方向**：① 恢复时不匹配就 409 / 400（是行为变更：今天 200）；② 维持 200 但在回执里补一格 `checksum_ok: false`（响应模型要加字段，`test_api_openapi_contract` 那把键集尺子随之重钉）；③ 给 `verify_backup_integrity` 一只端点，让用户能主动查（与 **A300** 同一件事）。定级 M，待拍。 | M |
| A300 | `clean_old_backups` / `verify_backup_integrity` 两只 SDK 函数没有任何 HTTP 入口 | **新立（L231）**：两只函数在 `augmentor/backup.py` 里都存在且可调用（实测 `callable` 皆真），而契约面上一个都不认：`app.openapi()["paths"]` 里以 `/api/system/backups` 开头的路径现量只有**三条**（`/api/system/backups`、`/{backup_id}`、`/{backup_id}/restore`），打 `GET /api/system/backups/ck/verify` 实测 **404**。CLI 侧同样没有子命令（`backup` 那个子命令只覆盖四条端点对应的动作）。**为什么值得立账**：这两只正是 **A299**（校验和不拦恢复）与「备份目录无限增长」两件事的现成解法，能力在库里躺着、产品面上够不着。**当前处置**：守卫按 (方法, 路径) 集合精确钉住备份面现量（多一条少一条都红），页面卡片写明这两只函数没有 HTTP 入口。**修法方向**：① 各加一条端点（`POST …/verify`、`POST …/cleanup`，注意 cleanup 是删除类操作要挂 `verify_api_key`）；② 或把 `verify` 的结果并进 `GET /api/system/backups` 的每一行（一次列表就带完整性，代价是每次列表都要重算全部校验和）。定级 S，待拍。 | S |
| A301 | 「默认备份目录」在仓内有三份，而端点那一份与另外两份**不是同一个目录** | **新立（L231）**：三份实测——① CLI `--backup-dir` 的默认是相对路径 `.backups`；② SDK 侧全部形参默认也是 `backup_dir: str = ".backups"`（`augmentor/backup.py` 里现量若干处，守卫按「集合恰为 `{.backups}`」钉）；③ 端点省略 `backup_dir` 时走 `default_backup_dir()`，返回的是**白名单首个根目录下的 `.backups`**（出厂即 `data/.backups`，实测是绝对路径）。⇒ 同一句「默认备份目录」在 CLI/SDK 面上解析成**当前工作目录**下的 `.backups`，在 HTTP 面上解析成 `data/.backups`；`cwd` 恰好是 `data/` 时两者重合，其余时候是两份不同的目录。**为什么值得立账**：这不是「有三个默认值」那么轻——用户用 CLI 建了备份、再到页面上找，看到的是空列表（反之亦然），而两边都各自「按默认」行事，谁都没写错路径。**当前处置**：服务层不抄第二套默认（守卫钉住 `api.ts` 里不出现 `backup_dir` 的字面量默认、页面上也不给这一格的输入），页面卡片与 `docs/API.md` 各写明三份默认的关系。**修法方向**：① CLI/SDK 的相对默认改成解析到白名单首根（与端点同源，是行为变更：`cwd` 下不再生成 `.backups`）；② 或把三份默认收敛成一个函数，三面都调它；③ 维持现状但在 CLI 帮助与文档里写明「相对当前工作目录」。定级 S，待拍。 | S |
| A302 | 三只写操作挂 `verify_api_key`，而前端没有送 `X-API-Key` 的通道 ⇒ 设了 key 之后页面上的写操作一律 401 | **已关闭（L233，走修法①；原立 L231）**：实测 `POST /api/system/backups`、`POST …/restore`、`DELETE …/{backup_id}` 三条都挂 `Depends(verify_api_key)`，`GET /api/system/backups` 没挂（守卫按 `route.dependant.dependencies` 逐条读，`gated == seen - {GET}`）。而立账当时 `web/src` 里带引号的 `X-API-Key` 字面量现量**零处**（全文提到这个头名的只有 `System.tsx` 那一处老实文案），axios 实例也没有拦截器注入任何鉴权头。**后果**：`AUGMENTOR_API_KEY` 一旦设上，页面上所有写操作（不止备份这一族：改配置 / 删数据 / 上传 / 导出 / 回滚版本同规）都会 401，而界面只能显示服务端那句判决，用户没有任何地方能填 key。**当时的处置**：页面卡片写明这一条，守卫同时钉住「三条写操作确实挂了闸」与「前端确实没有送头的通道」两格现量。**修法方向**：① 前端加一处 key 输入（localStorage + axios 请求拦截器），并在 401 时引导；② 或改成同源 Cookie / 会话（部署面改动更大）；③ 或明确记档「设了 key 就只走 API 客户端，页面只读」并把页面上的写操作在设 key 时禁用掉。**注意 ①③ 都是产品决策**，不是接线。**L233 收口（用户拍板走 ①）**：仓外探针把整张契约面重量了一遍，规模比立账时以为的大一个量级——75 条操作里 **24** 条挂 `verify_api_key`，其中 **21** 条正是前端已接线的形状（立账时只数了备份那一族的 3 只），另 **3** 条挂门禁但还没接线（`translate` / `migrate` / `stream`），且门禁里**没有一条 GET**。通道落在 `web/src/services/api.ts` 一处：`API_KEY_HEADER` / `API_KEY_STORAGE_KEY` / `getApiKey` / `setApiKey` + 请求拦截器（留空即不送头 ⇒ 零配置照旧可用），401 的指路加在 `apiErrorDetail` 这个唯一漏斗上（一处改动，21 个动作同时被指路），key 的输入界面在「配置管理 → 写操作鉴权」卡上。l231 那支守卫的「前端零通道」一维**前提已变**，按新世界重钉为只留后端门禁事实（不是放宽），前端通道改由 `tests/unit/test_api_key_channel_l233.py`（18 支）与 `api.test.ts` / `Settings.test.tsx` 两面钉。注入 14 臂逐臂点名喊红。定级 M ⇒ **关闭**。 | M |
| A303 | `backup()` 里两次 `datetime.now()`：秒级 `backup_id` 与微秒级 `timestamp` 跨秒时对不上，且 `name` 不在回执里 | **新立（L231）**：`backup_id` 由 `datetime.now().strftime("%Y%m%d_%H%M%S")` 生成（秒级），回执里的 `timestamp` 由**另一次** `datetime.now().isoformat()` 生成（微秒级）——实测两次调用之间隔着读文件、写盘、算 sha256，跨秒时会出现「id 写着 175833、timestamp 写着 …175834」。守卫按 AST 数 `backup()` 里的 `datetime.now()` 调用次数（现量 **2**）。相邻的一格：回执七键里**没有 `name`**（`backup_id` 就是 `name or 时间戳`），所以调用方给过 `name` 之后无法从回执里区分「这是我起的名字」还是「服务端生成的 id」。**为什么值得立账**：id 与 timestamp 在同一条记录里语义上应该是同一时刻，而它们不是；排序、去重、按时间清理（**A300** 的 `clean_old_backups`）都读这两格。**修法方向**：① 取一次 `now()` 复用（一行改动，行为上更自洽）；② 回执补 `name` 那一格（响应模型加字段，键集尺子随之重钉）。定级 S，待拍。 | S |
| A304 | `/api/data/list` 只列 `train_data*.json` ⇒ 备份源的候选口径窄于白名单 | **新立（L231）**：页面「备份源文件」那一格的候选取自 `GET /api/data/list`，而它的过滤口径实测只认 `train_data*.json` 形状的名字——白名单根里放 `p.json` 与 `train_data1.json`、`train_dataX.json` 三份时，列表只回后两份（守卫用同一组夹具钉住这一格）。⇒ 用户想备份一份不叫这个形状的文件（比如 `convert` / `merge` / `sample` 的产物，它们的名字由调用方给）时，下拉里没有它。**当前处置**：那一格做成 `AutoComplete`（既能选也能手填任意路径），卡片写明「候选只列 `train_data*.json`，别的名字要手填」，手填的路径仍受白名单管（实测白名单外绝对路径 403、`..` 400、空串 400）。**修法方向**：① 给 `/api/data/list` 加一个「不按名字过滤」的档（query 参数或新端点），候选与白名单同宽；② 或让写盘端点的产物统一落一个可列举的目录；③ 维持现状（页面已给手填通道）。定级 S，待拍。 | S |
| A305 | `POST /api/system/dependency/datasets` 的 `description` / `tags` **回显即丢**：响应带着这两格，随后的 `GET` 与磁盘 `datasets.json` 里都是空 | **新立并同轮收口（L232）**：接线血缘卡时仓外探针逐档实测。请求体 `{name, input_file, description: "描述B", tags: ["t1","t2"]}` ⇒ **200**，回执九键里 `description` 与 `tags` **都在**；紧接着 `GET …/datasets` 里同一条的这两格是 `""` 与 `[]`，磁盘 `datasets.json` 同样是空。根因在路由里读得出来：`register_dataset()` 内部就 `atomic_write_json` 落了盘，端点却在这之后才 `info.description = …` / `info.tags = …`，改的是**内存里那个返回对象**。⇒ 契约声明得进、存不下来，而**只看响应的那把尺子在改前也是绿的**（回执确实回显了），这就是本条值得单独记账的理由：「响应形状对」不等于「写进去了」。**修法（本轮已落）**：把两格随登记一起传下去 `manager.register_dataset(name, path, item_count, description=…, tags=list(… or []))`（SDK 形参本来就有这两格，`augmentor/dependency.py:142-144`），删掉事后赋值那四行；`api/routes/system_ops.py` 净 **+1 行**。**判据**：`tests/unit/test_lineage_wiring_l232.py::test_description_and_tags_reach_the_disk_not_just_the_receipt` 只读磁盘与 `GET`、**不读响应**，注入臂1（把修法改回改前形状）实证它点名喊红。 | ~~S~~ 已修（L232） |
| A306 | 同名再登记是**覆盖**：`created_at` 被刷新、第二次送空值会把旧值抹成空，且没有 409 也没有任何提示 | **新立（L232）**：`dataset_id = md5(name)[:8]`（实测 `dsA` → `8c7f0201`、`dsB` → `0d2cace8`），注册表顶层是 `dict: id → 条目`，所以同名同文件再 `POST` 一次 ⇒ **200**、索引与磁盘各 **1** 条（与备份那一族「磁盘一份、索引两行」的形状**不同**，A298），但 `created_at` 被**刷新**成第二次的时间，`updated_at` 同步；第二次送空 `description` / 空 `tags` 会把第一次写进去的值**抹成空**。**为什么值得立账**：`created_at` 在语义上是「这条登记什么时候建的」，而它现在等于「最后一次覆盖的时间」，且调用方拿不到任何「你刚才覆盖了一条已有登记」的信号 ⇒ 前端上「登记」按钮的行为是幂等的假象，误点两次就静默改掉一条既有登记。**当前处置**：卡片写明「id 只由名字算（md5 前 8 位）、同名再登记是覆盖、`created_at` 会被刷新」，守卫钉住「同名两次 ⇒ 索引仍 1 条」这一格现量。**修法方向**：① 已存在同 id 时回 **409**（行为变更：今天 200）；② 或 200 但回执里补一格 `overwrote: true`（响应模型加字段，键集尺子随之重钉）；③ 或把 `created_at` 真的保住、只动 `updated_at`（最贴合语义，是行为变更）。定级 M，待拍。 | M |
| A307 | 两支**只读**端点会把注册表目录建出来（读面写盘） | **新立（L232）**：`GET …/dependency/datasets` 与 `GET …/dependency/graph` 在注册表不存在时都回 **200**（`{"datasets": []}` / `{"nodes": [], "edges": [], "issues": []}`，不是 404 也不是 null），而探针量到**请求之前目录不存在、请求之后存在**（空目录，此时还没有 `datasets.json`）——`DependencyManager.__init__` 里 `mkdir(parents=True)`。**为什么值得立账**：`GET` 在只读白名单语义下不该产生文件系统副作用；这一格的实际后果是「探活一次就多一个目录」，而在只读挂载 / 配额受限的部署面上它会变成 500（`to_http_error` 兜底），页面上只会看到一句服务端判决。**当前处置**：新守卫 `test_the_two_read_endpoints_create_the_registry_dir` 把这一格现量钉住（断言「请求前不存在、请求后是目录、且此时没有 `datasets.json`」），卡片与 `docs/API.md` 各写明「只读但会建目录」。**修法方向**：① 构造管理器时不 `mkdir`，改成写之前才建（读面就真的只读了，是行为变更）；② 或给 `DependencyManager` 一个 `readonly` 档，两条 `GET` 走它；③ 维持现状但把这一格写进契约说明（本轮已做）。定级 S，待拍。 | S |
| A308 | 「默认注册表目录」在仓内有**三份**，而端点那一份与另外两份不是同一个地方 | **新立（L232）**：与 A301（备份目录）同型、同根。三份实测——① CLI `--registry-path` 默认 `.dependency_registry`（`augmentor/cli/parser.py:472`，相对**工作目录**）；② SDK `DependencyManager.__init__` 形参默认也是 `.dependency_registry`（`augmentor/dependency.py`，相对**工作目录**）；③ 端点省略 `registry_path` 时走 `default_registry_dir()`＝**白名单首个根目录下**的 `.dependency_registry`（出厂 `data/.dependency_registry`，实测是绝对路径）。⇒ 用户用 CLI 登记了数据集、再到页面上找，看到的是空列表（反之亦然），而两边都各自「按默认」行事，谁都没写错路径。**当前处置**：服务层不抄第二套默认（守卫钉住 `api.ts` **代码**里不出现 `.dependency_registry` 字面量、页面上也不给这一格的输入；页面正文里出现它是**诚实口径**，判据只看 `strip_comments` 之后的代码面），`docs/API.md` 白名单那一节与卡片各写明三份的关系。**修法方向**：① CLI/SDK 的相对默认改成解析到白名单首根（与端点同源，是行为变更：`cwd` 下不再生成 `.dependency_registry`）；② 或三份默认收敛成一个函数、三面都调它；③ 维持现状但在 CLI 帮助与文档里写明「相对当前工作目录」。**与 A301 合并处置更省一次破坏性变更评估**（同一族、同一个修法、同一批用户可见后果）。定级 S，待拍。 | S |
| A309 | 登记过的数据集在产品面上**删不掉**，而依赖边没有任何产品级创建入口 | **新立（L232）**：HTTP 面上 dependency 路由现量**只有三条**（两支 `GET` + 一支 `POST`，守卫按 (方法, 路径) 集合精确钉住，多一条少一条都红）：无 `DELETE`、无注销、无建边入口。SDK 侧 `DependencyManager` 的公开方法有八个，而 `add_dependency` / `remove_dependency` / `get_dependencies` / `validate_dependencies` **四个一个都没有 HTTP 入口**，且 SDK 连注销数据集的方法都没有 ⇒ 登记了只能手改 `datasets.json`。相邻的一格：`edges` 只能由 SDK 的 `add_dependency` 建立，所以**纯前端用户的 `edges` 恒为空**，页面上「图中边」那一格恒为 0 而「依赖图」这个标题会让人以为坏了。**为什么值得立账**：这是「能力在库里躺着、产品面上够不着」的又一例（与 A300 同型），而且它是**成对**的——登记有入口、注销没有；图有入口、建边没有 ⇒ 用户能把注册表写脏却清不干净。**当前处置**：卡片写明「边只能由 SDK `add_dependency` 建立、纯前端 `edges` 恒为 0、登记过的数据集在产品面上删不掉（SDK 也没有 `remove_dataset`）」，守卫钉住「四条 SDK 方法都没有 HTTP 入口」与「三条端点里没有 DELETE」两格现量（前者按 openapi 的 `operationId` 查，因为按路径串查 `add_dependency` 是恒真的——那是一把不会响的尺子）。**修法方向**：① 补 `DELETE …/dependency/datasets/{dataset_id}` 与 `POST …/dependency/edges`（两只都要挂 `verify_api_key`，与 A312 一起拍）；② 或先在 SDK 补 `remove_dataset`，再各加一条端点；③ 维持现状但把「边」那一格从页面上撤掉（少一格恒 0 的死面）。定级 M，待拍。 | M |
| A310 | `validate_dependencies()` **不查环**：A→B 与 B→A 成环时 `issues` 里零报告 | **新立（L232）**：探针用 SDK 直写三条边（`dsA → ghost12345` derived、`dsA → dsB` merged、`dsB → dsA` filtered）后打 `GET …/graph` ⇒ **200**，`edges` 三条都在，而 `issues` 只有**悬空那 1 条**（`{"type": "missing_target", …, "message": "目标数据集不存在: ghost12345"}`）；环那一格 `issues` 里查不到任何 `cycle`/环相关的 `type`（实测 `type` 集合就是 `["missing_target"]`）。读实现也一致：`validate_dependencies` 只发 `missing_source` / `missing_target` 两种。**为什么值得立账**：页面上「校验问题」那一格是 **0 就绿**的口径，而 0 只证明「没有悬空端点」，不证明「依赖关系没问题」——一个环会让下游任何按依赖顺序重建的流程死循环或取到过期数据，而这件事在产品面上完全静默。**当前处置**：卡片写明「`issues` 只报悬空端点、**不查环**，0 不能读成依赖关系没问题」，守卫钉住「成环时 `issues` 仍只有悬空那 1 条」这一格现量。**修法方向**：① 给 `validate_dependencies` 加环检测（DFS 染色），`issues` 多一种 `type: "cycle"`（响应模型不变，但 TS 侧 `DependencyGraphIssue.type` 的取值域要跟着写进文档与用例）；② 或只在文档与页面上写清边界（本轮已做）。定级 M，待拍。 | M |
| A311 | 这一族的错误体是**双轨**的：缺必填字段是 FastAPI 英文 **422**，其余坏参数一律中文 4xx | **新立（L232）**：实测缺 `input_file` ⇒ **422**，`detail` 是 FastAPI 那份英文结构（`Field required`）；而 `input_file` 的其余四档坏值全是中文：不存在 **404**「文件不存在」、白名单外绝对路径 **403**「路径超出允许的数据目录范围」、带 `..` **400**「路径包含非法组件」、空串 **400**「文件路径 不能为空」（label 与「不能为空」之间那个空格是 f-string 原样），外加一档这一族独有的：顶层不是 JSON 数组 **400**「数据文件的顶层必须是 JSON 数组，当前是对象」；`registry_path` 那四档同样是中文。**为什么值得立账**：前端只有一处 `apiErrorDetail` 取错误文案，而 422 的 `detail` 是**数组**、4xx 的是**字符串** ⇒ 「填漏了必填项」这一种最常见的用户错误，界面上拿到的是一段英文结构体而不是那句话。**当前处置**：卡片写明「两种错误体都要认」，守卫把 422 那一档与五档中文判决逐格钉住（参数化四臂 + 顶层非数组一臂）。**修法方向**：① 全局装一个 `RequestValidationError` 处理器，把 422 也翻成中文同一形状（是行为变更：状态码可保持 422，`detail` 的形状要变，`apiErrorDetail` 与所有认 422 的用例随之重钉）；② 或只在前端把 422 的 `detail` 数组渲染成中文一句；③ 与 **A280**（错误分档一刀切）合并拍——两者是同一件事的两面。定级 S，待拍。 | S |
| A312 | 三条里只有 `POST` 挂 `verify_api_key`，而前端没有送 `X-API-Key` 的通道 ⇒ 设了 key 之后页面上的登记一律 401 | **已关闭（L233，随 A302 一起走修法①；原立 L232）**：静态读 + 行为双证——`route.dependant.dependencies` 逐条读出来是 `POST …/dependency/datasets` 挂着 `verify_api_key`、两支 `GET` 都是空；设上 `AUGMENTOR_API_KEY` 后无头 `POST` ⇒ **401**「缺少或无效的 X-API-Key」、带对头 ⇒ 200，两支 `GET` 无头照样 200。而立账当时 `web/src` 里带引号的 `X-API-Key` 字面量现量**零处**（全文提到这个头名的只有 `pages/System.tsx` 一处老实文案，那是 L231 立的判据），axios 实例也没有拦截器注入任何鉴权头。**与 A302 是同一个缺口的第二例**（备份族三只写操作同规），差别只在这一族**只有一支写操作**、两支读操作不设防 ⇒ 设了 key 之后页面变成「看得见、动不了」。**当时的处置**：卡片写明这一条（头名逐字只写在卡片正文里，服务层刻意不复述——「提到这个头名的文件只有一处」是 L231 那条判据的量法，多一处回声就会把它判红），守卫同时钉住「只有 POST 挂闸」与「前端确实没有送头的通道」两格现量。**修法方向**：与 **A302** 一起拍（① 前端加一处 key 输入 + axios 请求拦截器 + 401 引导；② 或改同源 Cookie / 会话；③ 或明确记档「设了 key 就只走 API 客户端，页面只读」并把页面上的写操作在设 key 时禁用）——**三个都是产品决策**，不是接线。**L233 收口**：走 ①。这一族那支 `POST` 与另外 20 条已接线的写操作共用同一条通道（`api.ts` 的请求拦截器），页面上不必逐族再补什么；「带引号字面量只有一处」这条量法**保留并搬家**——现在那一处是 `api.ts` 的 `API_KEY_HEADER`（L231 当时是零处），`System.tsx` 卡片正文里那句仍是**不带引号**的老实文案，所以不算第二处通道；血缘卡与备份卡各指一次「配置管理 → 写操作鉴权」，由新守卫按 `count == 2` 钉住。定级 M ⇒ **关闭**。 | M |
| A313 | `POST /api/system/stream` 的 `operation` 清单在 CLI 与端点两端不是同一份，而 CLI 独有的那一档 `export` 是**原样搬运** | **已关闭（L234）**：接线流式处理卡时两端对读量出来。CLI 那份 `--operation` 的 `choices` 写 `quality / dedup / clean / export`，端点认的是 `quality / dedup / clean / none` ⇒ 两头各错一半：① `export` 被 argparse 收下之后落进 handler 的 `else` 臂，而那一句只是 `lambda items: items`——**名字承诺的「导出」从来没发生过**，产物就是输入原样；② 反过来 HTTP 侧合法的 `none` 在 CLI 里被 argparse 直接拒（退出码 2）。**最贵的一格是它已经被钉成了事实**：既有测试的 docstring 逐字写着「export 操作为直通处理」，所以那把尺子改前是绿的、且它在为谎话作证。**修法**：把 CLI 那份字面改成与端点同一份（`export` → `none`），**没有新插 Python 常数**——把权威搬进 `augmentor/streaming.py` 会在 `WRITER_FORMATS` 之后插行、平移账本 A147 那条活引用（`test_doc_line_refs_l79` 与 `test_comment_refs_a184` 同时红），而仓内既有惯例（l226 / l175 / l189）也是「CLI 留字面、守卫对账」；同源由 `tests/unit/test_system_ops_wiring_l234.py` 的清单同源那一面钉住，注入臂 `cli_choices` 实证它点名喊红。**残留一格**：`export` 这个动作在产品面上仍然不存在（CLI 上曾经「看起来存在」），要真做导出得走 `POST /api/export/*` 那一族。⇒ 见循环日志 L234 | ~~S~~ 已修（L234） |
| A314 | `data/versions/current` 那一格：断链上 `Path.exists()` 抛 WinError 1920，把 `create_version` 与 `rollback` 整个炸掉；而链接目标写的是整段相对路径 ⇒ 恒断链 | **新立并同轮收口（L234）**：**这一条不是设计出来的，是全量门禁首跑那唯一一支红倒推出来的**——`OSError: [WinError 1920] 系统无法访问此文件。: 'data\versions\current'`，触发点是 `create_version` 里那句「先看那一格有没有东西」。现场取证量到三件：① 那份 `current` 是 **WSL 在 drvfs 上留下的 LX 链接**，`is_symlink()` 是 **False**、`os.readlink()` 报 *not a symbolic link*、只有 `os.stat` 抛 1920 ⇒ **「先判 is_symlink 再判 exists」这类重排治不了它**（我最初的修法正是这一版，被探针当场推翻）；② 链接目标是**整段相对路径** `data/versions/v_…`，而 `Path.symlink_to` 的相对目标按**链接自己所在目录**解析 ⇒ 实际指向 `data/versions/data/versions/v_…`，**恒断链**（成因是 `storage_dir` 的出厂默认就是相对路径）；③ 那个目录里当时堆着 **67** 个用例跑出来的 `v_*` 快照（⇒ A315）。**修法两处**：把「判存在 + 清除」搬进新方法 `_clear_current_link()`（先 `lstat` 再 `stat`，**stat 抛出来就当作「那里有东西」照清**；清除按「先当文件删、删不掉再当目录删」两步走，两步都失败就放过——这一格是便利索引不是数据，建不出新的也不该让 create/rollback 失败），链接目标改成**只写版本目录名**。**两处都刻意做成净零行**：账本里有三处活引用指着这只文件（`:197` / `:281` / `:372`），插行会把它们平移出代码跨度。**刻意没做的一件事**：不加 `target_is_directory=True`——本机 `os.symlink` 无特权直接吃 WinError 1314，没法验证「目录符号链接在 Windows 上能否被 `unlink` 掉」（`DeleteFileW` 对目录符号链接会失败、要 `RemoveDirectory`），引入一个无法验证的失效形状违反「失败要响」。**修完的现场证据**：那支用例转绿、原来读不了的 LX 链接被 `unlink` 自愈（`ls data/versions/current` 报 No such file）、`current.txt` 重写成最新版本 id。判据四面由 `tests/unit/test_version_current_link_l234.py`（10 支）钉住，注入臂 `old_clear` 红 4 支、`no_txt_clear` 红 2 支、`full_path_target` 红 3 支。⇒ 见循环日志 L234 | ~~M~~ 已修（L234） |
| A315 | A168 仓库残留守卫的范围欠账：`data/versions/` 与 `checkpoints/` 里堆的是用例跑出来的运行期产物，而原口径「往 `data/` 更深一层写不判」把它们整片放过 | **已关闭（L234）**：A314 那支红暴露出来的第二件事。原判据只盯仓库根 / `data/` / `.backups/` **三处顶层**，理由是「`data/` 更深一层是产品自己的目录结构」——这话对 `data/nested/` 成立，对 `data/versions/` 不成立：那里现量 **67** 个 `v_*` 快照全是测试跑出来的，A314 那份读不了的 LX 链接就住在里面。**也就是说 A168 那把尺子入库时（L94）量到的「全套跑下来 `RESIDUE_COUNT 0`」是真的，但它量的范围漏掉了产物真正落的那一层**——守卫不红不等于树是干净的。**修法**：`WATCHED` 三处 → **五处**（加 `data/versions` 与 `checkpoints`），口径表那一行同轮改成「写进 `data/` 更深一层（`data/versions/` 与 `checkpoints/` 除外）或 `tests/` 里 ⇒ 不判」。**代价是量出来的不是推的**：先跑「碰管道/版本」那一族 47 个文件 / 1 499 条用例，实测**泄漏 1 条**（`test_partial_branches_l120.py::test_malformed_variants_filtered_out`），就地补 `monkeypatch.chdir(tmp_path)` 收掉；扩范围之后全量 **8 687 条泄漏 0 条** ⇒ 与 L94 那次入库同形、**不需要白名单**。**新棘轮**：`test_the_runtime_dirs_are_watched_one_level_down`（把 `WATCHED` 改回三处它当场红），注入臂 `watched_three` 实证红 2 支。**这一支为什么住在文件末尾**：`test_repo_residue_l94.py` 第 227 行那处子进程调用是 `test_subprocess_text_decoding.py` 的 `BYTE_MODE_ALLOWLIST` 按「文件 + 行号」登记的调用点，在它前面插行会把登记指到空处、两条规则同时判红（实测插 23 行 ⇒ `227 → 250`，全量门禁 2 failed）⇒ 按处方优先级走「换点」而不是「重钉登记」。⇒ 见循环日志 L234 | ~~S~~ 已修（L234） |
| A316 | 页面把三个告警阈值写死在文案里（`completeness 0.7 / diversity 0.4 / consistency 0.7`），与 SDK 的 `_setup_default_thresholds()` 之间没有任何机器对账 | **新立（L234）**：接线自动质检卡时实测出来的。`web/src/pages/System.tsx` 的卡片正文写着「『触发告警』为 0 只说明这一份没触三个阈值（实测 completeness 0.7 / diversity 0.4 / consistency 0.7）」，那三个数来自 `augmentor/quality_monitor.py` 的 `_setup_default_thresholds()`（`alert_below` 逐档），**读数当时是用探针量出来的、写进文案之后就再没有任何尺子回头看它**。这与 A266 / A275 同族（契约面数字全靠人肉对账），但比那两条更容易烂：阈值在 SDK 里是一串构造实参、不是可引常数，改一个数不会让任何测试红，页面上那句话就静默变成假话。相邻两格也是同一次实测到的：① 告警文案是**英文单轨**（形如 `completeness below threshold: 0.50 < 0.70`），与本仓「4xx 中文判决」的口径不一致（⇒ 与 A311 同族）；② 这一支只比这三个数，**字段缺失、重复、长度异常都不在判据里**，所以「触发告警 0」不能读成「质量合格」——这句已写进卡片正文，但它同样是散文承诺。**修法方向**：① 阈值抽成 SDK 侧可引常数（`DEFAULT_QUALITY_THRESHOLDS` 一类），守卫逐项对账页面文案里的三个数；② 或页面不写死数字，改成从回执/端点读当前生效阈值（要求后端把阈值放进响应面，是契约变化）；③ 维持现状但把「这三个数是 L234 那次实测的快照」写进文案（最省，也最不抗漂移）。定级 S，待拍。 | S |
| A317 | 迁移卡的必填闸挂在 Popconfirm 的 `onConfirm` 上 ⇒ 用户先看见「照改」二次确认、点完才得到一句 warning | **新立（L234）**：`web/src/pages/System.tsx` 的 `handleMigrate` 开头两格判空（源文件、产物名）都在 `onConfirm` 指向的那个函数里，而 Popconfirm 包在按钮外面 ⇒ 交互顺序是「点『迁移结构』→ 弹出『迁移会改字段名，产物已存在则整份覆盖』→ 点『照改』→ 才发现自己没填东西」。**那句二次确认的措辞是有分量的**（它在为一次破坏性写盘要授权），而它在信息不全时照样弹出来，用户读完一段警告、确认了一次破坏性操作，得到的是「先填要迁移的那份服务端数据文件」。**为什么值得立账**：这不是样式问题，是**授权语义被空表单消耗掉了一次**——真正要拦的那次点击（填全了、确认覆盖）与这次白确认在界面上长得一模一样。同批接的其余四条里没有这一格（`auto-test` / `monitor` / `validate-config` 只读，`stream` 没有二次确认）。**修法方向**：① 把两格判空提到 Popconfirm 之前（按钮 `disabled` 或把确认框做成条件渲染），确认只在真要写盘时出现；② 或在确认框文案里带上当前两格的值，让用户在确认时就看见自己在迁移什么；③ 维持现状但把 warning 换成确认框内的一句提示。①最贴合语义、也是最小改动。定级 S，待拍。 | S |
| A318 | `validate-config` 的 `is_valid` 没有可引权威规则，而它与「问题清单条数」不是互补关系 | **新立（L234）**：接线配置校验卡时逐档实测。这一支校验不通过也回 **HTTP 200**，判决只在 body 的 `is_valid` 上 ⇒ 页面红绿不能看状态码（这一点已写进卡片正文）。真正没底的一格是 **`is_valid` 的确切规则在仓内没有可引权威**：本轮只观测到「`errors` 为 0 ⇒ `true`、`errors` ≥ 1 ⇒ `false`」这一族读数，而**出厂那份配置实测 `is_valid` 为 `true` 却带着 5 条「环境变量未设置: …_API_KEY」的警告** ⇒ `true` 不等于「没有问题清单」，`errors` / `warnings` / `info` 三档里只有第一档参与判决。**当前处置**：页面文案刻意**不写成定义**——它写的是「合法也不等于没有问题清单」加上那 5 条实测警告，而不是「`is_valid` 等于 errors 为 0」；守卫只钉观测到的行为（三档都渲出、判决来自 body 而不是状态码），**不钉一条推出来的规则**。**修法方向**：① 给 `is_valid` 写一句权威定义（在响应模型的 docstring 与 `docs/API.md` 里各一份，并由守卫对账）；② 或把三档计数一起上线一个 `blocking` 标记，让「哪几条会翻判决」变成可读的；③ 维持现状但把「warnings 不参与判决」写进文档（本轮已在页面上写了，文档面还没有）。定级 S，待拍。 | S |
| A319 | 四个健康分权重既不校验非负也不校验上界，`health_score` 也不钳 [0,1]，而门禁三条规则根本不看权重 ⇒ 同一份回执里「healthy」与「verdict=failed」同时为真 | **新立（L235）**：接线健康门禁卡时逐档实测。① `POST /api/quality/health-gate` 的 `weights` 只校验两件事——「必须 4 个元素」与「之和 ≈ 1.0」（那两句 400 的原文是「权重必须包含 4 个元素」/「权重之和必须为 1.0」），**负数与大于 1 的值全收**；② `DatasetHealthScore` 加权求和之后**不做 [0,1] 钳制**，实测 `weights=[2.0,-0.5,-0.25,-0.25]`（和恰为 1.0）把健康分顶到 **1.1697944143478562**，`weights=[0,0,0,1]` 顶到 **1.0**（同一份夹具、默认权重下是 **0.8927055856521438**）；③ `level` 的三档阈值（healthy ≥0.7 / moderate ≥0.4 / 否则 poor，`augmentor/health_score.py:121`）只跟这个**未钳制**的分数比 ⇒ 1.1698 读成 `healthy`；④ 而门禁那三条规则（`pass_rate` / `duplicate_rate` / `completeness`）**一个都不看权重**，所以同一份回执里 `level=healthy` 与 `verdict=failed` 同时成立（本轮夹具上 `duplicate_rate` 恒 0.5）。**为什么值得立账**：页面上「健康分 1.17 / healthy」与「判定 failed」并排显示，用户读到的是一句自相矛盾的话，而两侧都没说谎——真正缺的是**一句权威说明「健康分不参与门禁判决」**（本轮已写进卡片正文与回执 Alert，但那是散文承诺，没有机器对账，与 A316 同族）。**修法方向**：① `health_score` 出口钳到 [0,1] 并把钳制写进 docstring（破坏性：会让既有读数变小）；② 权重加「非负」校验（同样破坏性，且会挡住「刻意用负权重压低某一维」这类用法——本轮实测这种用法确实能用）；③ 维持现状但在响应里加一个显式标记（如 `score_is_unclamped`），让前端不必靠散文解释。**L238 收口（选 ①+② 两半，③ 刻意没做）**：判据搬进 `validation.require_ratio_list`，健康分权重与 `quality.weights` 共引同一份（三元与四元同一个容差、同一句措辞），逐项 [0, 1] 并点到 `weights[i]` 的下标，`score()` 出口钳 `min(1.0, max(0.0, float(raw)))` ⇒ 实测那一档 **1.1697944143478562** 今天到不了；破坏性已承担并写进文档：L235 量出的「刻意用负权重压低某一维」这一用法自本轮起 400。**没做的那一半要说清**：钳制只消灭了「分数越界」，`level=healthy` 与 `verdict=failed` 并排仍然可能出现（门禁三条规则一条都不看权重，本轮夹具上 `duplicate_rate` 恒 0.5），页面文案因此从「自相矛盾」改成「两套判据各说各话」，`score_is_unclamped` 那一格没加 ⇒ 「健康分不参与门禁判决」仍是散文承诺，与 **A316** 同族、仍待拍。守卫 `test_decision_gates_l238.py::TestTheClampBites`（4 支，含「四项必须全 1.0 才顶得出越界」的自证臂 `raw > 1.0`，防恒真）+ `test_health_score.py::test_each_weight_must_be_a_ratio`（4 档点名下标）。 | ~~S~~ **已收口（L238）** |
| A320 | 权重之和的容差写成 `abs(sum-1.0) > 0.01`，而「±0.01 以内算 1.0」这句在恰好 0.01 那一档是假话 | **新立（L235）**：同批实测。`[0.24,0.25,0.25,0.25]` 与 `[0.26,0.25,0.25,0.25]` **都被拒**（400「权重之和必须为 1.0」），因为它们的浮点差是 **0.010000000000000009** > 0.01；而 `[0.255,0.25,0.25,0.25]`（和 1.005）**放行**。⇒ 实际边界不是文档与直觉里的「0.99 与 1.01 都算 1.0」，而是「比 0.01 严格更小」，且这条边界**由浮点表示决定、不由十进制决定**（0.99 那档刚好越界，1.005 那档刚好在内）。**为什么值得立账**：这是一条**用户会照着写、然后被拒**的规则——「四个权重之和留两位小数凑 1.0」是最自然的用法，而 0.24/0.26 正是两位小数里最常见的一档；错误信息只说「必须为 1.0」，不说是容差判的、也不说容差多大。本轮页面文案刻意写成实测口径（「0.99 与 1.01 都 400、1.005 才 200」），并由守卫 `test_a320_the_sum_tolerance_is_not_what_the_docstring_says` 钉住。**修法方向**：① 把容差抽成可引常数（`WEIGHT_SUM_TOLERANCE` 一类）并在 400 的 `detail` 里带上「实测和 = X，容差 = Y」；② 或改用 `math.isclose` 一类的显式口径并把边界说成「四舍五入到 N 位」；③ 维持现状但把「恰好 0.01 会被拒」写进 `docs/API.md` 与响应模型 docstring（最省，也最不抗漂移）。**L238 收口（选 ①，并把 ② 的口径做实）**：容差抽成 `validation.WEIGHT_SUM_TOLERANCE = 0.01`，判和之前先 `round(abs(total - 1.0), WEIGHT_SUM_QUANTIZE_DIGITS)`（9 位）把浮点噪声消掉 ⇒ 「±0.01 **含端点**」第一次成为真尺子：实测 0.99 / 1.005 / 1.01 三档放行、0.98 与 1.02 拒，400 的 detail 带回实测和（`… 之和必须是 1.0（容差 0.01，含端点），当前 0.98`，守卫逐档 `endswith` 钉死）。`0.24,0.25,0.25,0.25` 的十进制差是 0.01 而 IEEE-754 求和是 **0.010000000000000009**——量化到 9 位正是为它，那条「留两位小数凑 1.0」最自然的用法不再被假拒。页面与 `docs/API.md` 同轮换口径，L235 钉的旧断言「0.99 与 1.01 都 400」按新读数重钉。守卫 `TestTheToleranceIsRuled`（5 支，含 `DEFAULT_HEALTH_WEIGHTS` 不被调用方就地改写那一支）。 | ~~S~~ **已收口（L238）** |
| A321 | `text_field` 送空串或送一个数据集里不存在的键，后端零判决 ⇒ `duplicate_rate` 回 1.0，与「真的全重复」在响应里同形 | **新立（L235）**：同批实测，也是本轮前端**唯一一处刻意用 `||` 躲判决**的地方。① `text_field=""` 与 `text_field="no_such_field"` 都**不报错**（200），`gate.metrics.duplicate_rate` 一律 **1.0**、`failed_rules == ["duplicate_rate"]`；② 同一份数据用真实存在的 `text_field="output"` 时 `duplicate_rate` 是 **0.5** ⇒ 那 1.0 不是数据性质，是**取不到字段**的产物；③ 健康分那四项（`completeness` / `diversity` / `quality_balance` / `coverage`）**压根不看 `text_field`**，实测 `completeness` 仍 1.0 ⇒ 回执里没有任何一格能告诉用户「你字段名写错了」。**为什么值得立账**：这是一条**假判决**——它长得与真判决（数据真的全重复）逐字节相同，而门禁的用途正是「按判决放行/阻断」，所以写错一个字段名的后果是**整份数据被拦下**，且拦下的理由看起来完全合法。与 **A285**（五处手写字段清单没有权威）同根：字段名既没有可引权威、也没有存在性校验。**当前处置**：前端 fetcher 把空串转成 `undefined`（不发这一格 ⇒ 走后端默认），页面文案写「`duplicate_rate` 变成 1.0 时先怀疑字段名」，守卫 `test_a321_a_ghost_text_field_produces_a_fake_duplicate_verdict` 钉住行为。**修法方向**：① 端点侧校验 `text_field` 存在于首条记录，不存在就 400 并点名（最贴合「假判决」这个缺陷类）；② 或在响应里回一个 `text_field_found: false`、或把该规则挪进 `errored_rules`（那一格实测**恒空**，正好是它该住的地方）；③ 维持现状但把这句写进 `docs/API.md`。**L238 收口（选 ①，② 没做）**：新建 `validation.require_item_field(name, items, field, label)`——两刀各有分工：空串 / 纯空白 / 非字符串由 `require_string` 点名**参数名**，名字成立但这里没有这一列由存在性判据点名**首条实际字段**（拼错的人真正需要的是「那一份里到底有什么」）；存在性取「**至少一条**含该字段」而不是每条，因为数据集异构是常态（增强与合并的产物天然多列）。health-gate、`/api/dataset/impact` 的 `_require_any_field`（降级成一层 HTTP 外衣）、批量翻译**三面共引**，四档（空串 / 纯空白 / `no_such_field` / `Instruction`）SDK 与 HTTP 两面逐字节同文。判序也钉死：**参数错在读盘之前**（坏权重 + 文件不存在 ⇒ 400，不是 404），**字段错在读盘之后**（幽灵字段 + 文件不存在 ⇒ 404，没有数据就没有「首条实际字段」可点）。**② 没做**：既没有 `text_field_found`，`errored_rules` 也仍恒空 ⇒ 这一族是被 400 **拦住**的，不是被**如实标出**的。守卫 `TestTheTwoFacesSayTheSameSentence` + `TestTheOrderOfVerdicts`（含迟到绑定哨身：SDK 侧是函数体内 import，`is` 判身份对它不成立，只能按「迟到绑定」验）。 | ~~M~~ **已收口（L238）** |
| A322 | `POST /api/dataset/translate` 的 `text_key`：空串与幽灵键都零判决，产物凭空多三键而原字段一字未动 | **新立（L236）**：接翻译卡时逐档实测（3 条中文语料、`multilingual.enabled=true`、管道用替身好数模型调用）。① `text_key=""` 与 `text_key="no_such_field"` **都回 200**，且 `failed=0` / `failed_indices=[]` / `errors=[]` / `errors_truncated=false` —— **回执四键全是「全成功」的形状**；② 模型调用 **0** 次（替身数出来的）⇒ 这一次不花钱，但也不产出任何译文；③ 产物里每条凭空多出**三**个键：`<text_key>` 那一列（值为空串）、它的 `_original`（同样空串），以及一个 `language` 目标语言戳（实测 `en`）——第三键来自 `augmentor/multilingual.py:190` 那句无条件 `new_item["language"] = target_lang`，**一个字没翻也照盖**；④ 原字段（`instruction`）一个字没动，落盘那一份与响应体里的 `items` 逐格相同（实测键集 `{instruction, output, "", _original, language}` 与 `{instruction, output, no_such_field, no_such_field_original, language}`）。**为什么值得立账**：与 **A321**（`text_field` 幽灵键 ⇒ `duplicate_rate` 假判决）同一缺陷类，但这一条更会骗人——A321 至少在响应里留了一个可疑的 `1.0`，而这一条的**回执与真成功逐字节同形**，唯一的线索在产物里那一列空值上；又因为 0 次模型调用，它连「花钱买到了什么」这条通常的追查路径都不会触发。**当前处置**（后端一行未改）：页面自己拦下空值（`if (!trTextKey.trim())` 那道必填闸 + 一句点名 A322 的提示），卡上写明「**别把「200 且 failed=0」读成翻成了**——先核对产物里那一列」，守卫 `TestAThreeTwoTwo` 三支钉住行为与文案（其中静态那一支的注释写明「改了后端那天这两句要一起动」），`docs/API.md` 的 translate 说明行同轮写清。**修法方向**：① 端点侧校验 `text_key` 非空且存在于首条记录，不满足就 400 并点名（最贴合缺陷类，与 A321 可并表执行）；② 或在响应里回一个 `text_key_found: false`、或把这一档挪进 `errors`（那一格实测恒空，正好是它该住的地方）；③ `language` 那一格要么只在真翻了的时候盖，要么改名成 `target_lang`——今天这个名字读起来像「检测到的语言」，而它是**目标**语言且无条件写入（改名是对外契约变化，故只立账）。**L238 收口（选 ①，③ 没做）**：`translate_dataset` 在分批之前共引 `require_item_field`（`name` 送 `text_key`），与 `backend is None` 那道闸同一条纪律——花钱路径上的参数错误不该先花一笔才知道。实测四档坏 `text_key` 一律 **400**、模型调用 **0** 次、**一个字节都不落盘**（写盘类探针先把 `AUGMENTOR_CONFIG_PATH` 指向仓外副本）；改前那份「200 / `failed=0` / 四键全零」的同形回执从源头消失。**③ 没做**：`language` 仍是 `multilingual.py:190` 无条件盖上的**目标**语言戳，改名是对外契约变化、不在本轮；而且存在性判的是「有这一列」，所以**列存在而整列为空串**时仍会走 SDK 的「回退原文并计入成功」⇒ 页面那句「别把『200 且 failed=0』读成翻成了」留着，它不是历史包袱。守卫 `test_last_two_wiring_l236.py::TestAThreeTwoTwo`（本轮把 L236 刻意留给「A322 自己那一轮」的三格判据接成真判据）+ `test_decision_gates_l238.py` 的同文面（CLI `run_health_gate` 与 impact 的「基线数据集」label 也在这条链上）。 | ~~M~~ **已收口（L238）** |
| A323 | `Settings.handleSave` 的 toast 是自己拼的第三处措辞，不读后端 `message` ⇒ 后端把 `ignored_keys` 判决拼进 `message` 那一档，页面上看不见 | **新立（L237）**：补 `UpdateConfigResponse.ignored_keys` 时逐档实测。后端两档措辞是「配置已保存，部分配置需要重启服务生效」与被忽略时的「配置已保存，部分配置需要重启服务生效（已忽略未写入的配置项：qualty；是否想写 quality？）」，另有空写入档「本次请求未写入任何配置项，config.yaml 保持不变」；页面 toast 写死的却是**第三处**「配置已保存到 config.yaml，部分配置需要重启服务生效」——它不读响应，所以被忽略的那串判决在 toast 里必然丢失。**可达性按实测收窄，不夸大**：空写入那一档从本页不可达——`handleSave` 恒带 `default_model`，而实测只发 `default_model` 那一格就 `applied` 非空、回「配置已保存」并真落盘（三档探针都跑在仓外副本上：`{}` ⇒ 「未写入任何配置项」且磁盘 sha256 不动；只有 `default_model` ⇒ 「已保存」且磁盘变）。**当前处置**：L237 新立的「上次保存的回执」卡把后端真回的 `message` 与逐条 `ignored_keys` 渲染出来，并写明 `success` 两边都是 true、它只表示「请求被正常处理」⇒ 判决在页面上已经可见，只有 toast 那一句还在说固定措辞。**为什么本轮不改 toast**：`Settings.test.tsx:142` 断言的正是那句硬编码文案，改措辞要连判据一起重钉，而本轮的证据链是「类型 ↔ 契约」，掺一条改判据的前端行为差会把两件事的证据混着算。**修法方向**：① toast 直接显示 `result.message`（一行改动，`Settings.test.tsx` 那句断言随之重钉）；② 或把「已保存」那句改成条件句（`ignored_keys` 非空时改口为「部分未写入」）；③ 或删掉 toast 只留回执卡。定级 S，待拍。 | S |
| A324 | `dataset_ops.split` 的三个分割比例仍是**手抄容差 + 不判长度**：完全合法的 40/30/29 切分被拒，SDK 直构喂 4 个元素则过校验后死在 unpack | **新立（L238）**：给权重族做「一条权威只住一处」时顺手量了同族的另一处——`split` 那三个比例**没有**走 `require_ratio_list`。① `train_ratio=0.4, val_ratio=0.3, test_ratio=0.29`（十进制和 0.99，浮点差 `0.010000000000000009`）实测被判「分割比例之和必须为 1.0，当前为 0.99」⇒ **A320 刚收掉的那个假拒在这一处还没收**，而「留两位小数凑 1.0」正是最自然的用法；② 同一处不判长度，SDK 直构把 `ratios` 喂成 4 个元素时**通过校验**，随后死在 `ValueError: too many values to unpack (expected 3)` ⇒ 一个 500 形状而不是 400；③ 三面可达性不同：API 与 CLI 由三个标量拼 tuple，所以长度那一半**只在直构面可达**，容差那一半**三面都可达**，这与 A320「同一份 YAML 在一面能加载、另一面被判拒」是同一个缺陷类的两种形状。**修法方向**：两刀都搬进 `require_ratio_list`（长度 3、逐项 [0, 1]、和为 1 ± 容差含端点）并三面共引，`split` 的比例形参改成显式具名校验。**为什么本轮不夹带**：A320 的四档实测、页面文案与 `docs/API.md` 是同一条证据链，改 `split` 要牵动 `dataset_ops` 的既有读数与 CLI 帮助文本，另起一轮才守得住「数字必须来自同一条命令链」。定级 M，待拍。 | M |

| A325 | `LeakageDetector.detect` 的近似匹配把**整份倒排按每条测试样本摊平重数一遍**：10k×10k 实测 **45.4 s**，是全项目当前最大的一格热点 | **新立（L239）**：性能现量探针（`Temp/l239_perf.py`，20000 条合成数据、uv 托管 cpython 3.12.14、sentence-transformers 未安装⇒走 fallback）。按 10k/10k 切给 `detect()` 的 wall time best **45411.2 ms**，cProfile 里 **41.047 s 的 tottime 全在 `_collections._count_elements`**、调用次数 **9988**（⇒ 每次约 4.1 ms），出处是 `augmentor/leakage.py:233` 那句 `Counter(chain.from_iterable(gram_index.get(gram, ()) for gram in test_grams))`。形状解释：Jaccard 只用到「共享三元组数」，所以每条测试样本都要把自己全部三元组的 posting set **串起来数一遍出现次数**；训练侧 posting 越宽（真实指令文本里常见子串能进几千个 sid），单条摊平就越接近「重跑一遍全索引」。对比同批读数：`Deduplicator.deduplicate` 20k 才 1065 ms ⇒ leakage 这一格比它大 **42 倍**。**为什么值得立账**：泄漏检测是「增强产物还能不能进评估」的守门面，API 面可达，而 45 s 量级会让一次页面点击看起来像卡死；且它随 posting 宽度而不是随数据条数恶化，**合成随机数据已经这么慢，真实中文语料只会更慢**（子串共享率远高于随机五字母词）。**修法方向**：① 按长度带剪枝——`J >= t` 蕴含候选 sid 的三元组数 `|b| ∈ [t·|a|, |a|/t]`（t=0.8⇒`[0.8|a|, 1.25|a|]`），把 posting 按 `sig_gram_counts` 有序化后用两次 `searchsorted` 夹出带内段（`dedup.py` 的倒排块就是这套 CSR 做法，仓内已有先例）；② 或给「出现次数低于 `need`」的 gram 先做 posting 长度预筛；③ 或按 dedup 同款代价预言机在「摊平计数」与「逐候选求交」之间选路。**为什么 L239 不动手**：本轮定义是「零对外行为变化」，而 ① 要重写 `leakage.py` 的索引形状，判据「产物逐字节相同」需要新的对照守卫与注入取证，`leakage.py:182/186/233/236` 四处行号还被现存注释点名，另起一轮才守得住口径。定级 **M**（性能面，本仓当前最大可量化收益），L240 主项。**· 已收口（L240）**：落地走的是原案处方 ③（dedup 同款代价预言机在「摊平计数」与「稠密矩阵乘法」之间选路），处方 ①（长度带剪枝）量化后否决——`Temp/l240_leakprobe2.py` 现量 4000×4000 档 posting 摊平 37,328 → 带内 17,298 条目/条测试 ⇒ 只省 **53.7%**，常数减半而量级不变。本行原框那句「`leakage.py:182/186/233/236` 四处行号还被现存注释点名」是**改写前的现场读数**，按 A127「账本允许时间戳式引用」保留不批量改写；实测那四处号里没有一条被产品注释点名（`test_comment_refs_a184` 硬 0 档本轮未红），真正需要重钉的是卷一 L55 那一处 `leakage.py:124`，本轮净 +196 行把它平移出跨度 ⇒ 已按处方 3 重钉 `leakage.py:352`。10k×10k 收益 **48 929.0 → 1 742.1 ms（28.1×）**、六档报告逐字节相同、八臂注入 7 臂红；新守卫 29 支全部计数判据。⇒ 详见 L240 日志块。 | ~~M~~ **已收口（L240）** |

| A326 | `Deduplicator` 在 fallback 编码器且代价预言机判稠密赢时，**同一批文本被逐字符扫两遍**（`_inverted_view` 选路 + `_batch_encode` 正式编码） | **新立（L239）**：同批探针 profile `deduplicate` 20k 的 cumtime：`_ngram_entries` **1.176 s** / `_inverted_view` **1.242 s** / `_fallback_encode` **0.775 s**（内部 2 511 040 次 `dict.get` + 2 499 756 次 `list.append`）。`_similarity_blocks`（`dedup.py:451-481`）在 `self._model == "fallback"` 时**先建一份倒排视图喂 `_inverted_blocks_win` 选路**，判负才把 `inverted = None` 松手、走 `_batch_encode` 的稠密路 ⇒ 稠密那一条把 n×vocab 的字符扫描做了两遍。实测真语料 6902 条 / vocab 23033 判**倒排赢**（969 ms vs 10484 ms），所以这条重复扫描在生产主路径上**不成立**，只在「短文档 / 高重复 / 小词表」这类稠密档里出现。**为什么值得立账**：它是「为了选路而付出的成本在选路失败时被完整丢弃」的形状——预言机的读数越保守，被浪费的那一份越大。**修法方向**：① 让稠密路复用 `_ngram_entries` 已经产出的词条/文档流（同一趟扫描同时喂两边）；② 或把预言机改成不建实体索引、只用 n 与词表规模的估计量（现在它真读了 `doc_count`/`vocab_size`/`pair_adds`）。**为什么 L239 不动手**：`_batch_encode` 是**测试注入向量的接缝**（`dedup.py` 的注释与多支守卫按它下钉子），合并成单趟这条路 A19/L2/L10 已记录被显存护栏 `test_fallback_encode_peak_holds_only_one_matrix` 否决过一次；且 `dedup.py` 身上挂着 **6 处** `dedup.py:NNN` 历史行号引用（104/132/193/242/607/830），插行必平移。定级 **L**，待拍。 | L |

| A327 | `QualityScorer._calculate_diversity` 每选一个候选都重算一遍已有向量集合的**逐行范数**：FakeModel 顶上 n=4000 实测 28.5 ms ≈ 整个向量侧的 18% | **新立（L239）**：同轮 quality 探针（`Temp/l239_quality.py`，FakeModel 用 `zlib.crc32` 顶掉 sentence-transformers，每轮重置 `_embedding_cache` 以免 `hasattr` 短路让第二次跑白捡缓存）。`batch_score` n=4000 best **156.9 ms**，其中 `encode` 只被调 **3 次 / 8030 行**⇒ 惰性批量编码那层（L231 收的重复编码）确实生效、没有重复编码；剩余可定位成本是 `_calculate_diversity` 里对 `_existing_embeddings` 每次候选重算 `np.linalg.norm(..., axis=1)`，单点插桩 **28.5 ms**。顺手量了同族的另一刀并**量化后否决**：语义相似度那个逐元素 `np.dot(a[i], b[i])` 循环换成 `np.einsum("ij,ij->i")` 快 **3.73×**（14.26 ms → 3.82 ms），但**产物不位相同**——float32 最大差 **2.98e-07**、float64 最大差 **8.88e-16** ⇒ 按本仓「性能改动必须产物逐字节相同」的口径不予采纳。**为什么值得立账**：1–2% 的端到端收益在真实语义模型（n×dim 更大）下会放大成可观常数，而行范数缓存是纯内部改动、可零对外行为变化。**为什么 L239 不动手**：收益量级（真模型下端到端 1–2%）撑不起一次改动——`quality.py` 身上挂着**十余处** `quality.py:NNN` 历史行号引用（417-433、312、133/153、85-89、81、80 等），任一处插行都要重钉一串历史引用；本轮先把读数钉进账，动手留给能同时把「缓存失效边界」（`_embedding_cache` 与 `_existing_embeddings` 的寿命）说清楚的那一轮。定级 **L**，待拍。 | L |

| A328 | 两卷 Backlog 表的关闭标记词汇与轮号可追溯性 | **新立（L242）**：本轮那把守卫只钉「进度声明面说收了 ⇒ 表行有标记」这一个方向。反方向（表行有标记 ⇒ 标记里的轮号能在**同一卷**的进度声明面查到）**本轮没钉**，现量三条：①补标之后 132 行有标记，按严格式「左括号 + L + 数字」解析轮号，有 **10 行**解析不出（限定词插在括号与轮号之间，例如卷一 A122 写作「已关闭 + 第二本账 + 轮号」的形状），另有 **3 行**（A46 / A130 / A138）解析得出但那一轮不在本卷——跨卷语义本身是对的，只是同一卷查不到；②放宽到行内任意轮号则 132 行全部「可查」，但那量的已经不是标记里的轮号而是行内任一提及，不足以当判据；③标记词汇形状现量 **7 种**且**没有封闭清单**（五个动词「已关闭 / 已收口 / 关闭 / 收口 / 已修」各配一个括号加轮号，再加两种划掉的定级字母 S 与 M），换一种新写法（例如未来的「已了结 + 轮号」）对守卫结构性失明。**本行自己就撞上过一次**：初稿把七种形状逐字写进格子里，于是那一格被标记判据当成「已关闭」，现量从 132 涨到 133——这就是形状失明方向的证据。修法方向：先给标记词汇立封闭清单（与 A77 单一权威口径同族），再把轮号查找做成「括号后允许限定词」的可解析式并跨卷对账。定级 M，待拍。 | M |

## Backlog B — 功能增强与体验（价值 ÷ 工作量）

| # | 内容 | 证据 | 量级 |
|---|------|------|------|
| B201 | **已关闭（L131）**：`DatasetAnalyzer.get_duplicate_candidates`（SDK 公共方法）O(n²) 里逐对重建字符集是真热点 | 实测 6000 条 **71.8 s**；预计算后 2500 条 A/B new/old ×0.36 / ×0.34（双序），等价性 5 阈值逐元素一致 | S |
| B202 | **已关闭（L132）**：`ActiveLearningLoop._diversity_scores` 逐对调用 `compute_similarity`，每对把两侧重新 tokenize，n 条共约 2n² 次切词 | 预计算词元集合后 n=800 A/B new/old ×0.088（≈11×，双序），等价性 4 数据集（含重复/空串/单条/多词）逐元素一致 | S |
| B203 | **已关闭（L133）**：`DataCleaner._normalize_punctuation`（`normalize_punctuation` 规则）此前无任何直达行为用例；候选「14 道 replace → str.translate」优化经 A/B **证否**（长文本稀疏匹配 ×8.7 变慢）故不改产品，改补首个行为守卫 | A/B：重匹配 ×0.87、但真实场景（长文本少量全角）translate 逐字符查表 ×8.7 慢于 replace 的无命中短路；新增映射表 14 项 + 顺序 replace 参照逐字符等价的 3 用例 | S |
| B204 | **已关闭（L134）**：`ContextAugmentor.generate_multi_turn` 在「每轮 × 每条历史」里各自重建 user 问题列表再 `in` 线性查，且重复判定散在循环内不易推理；循环不变量 `existing_histories` 被反复重算 | 改为循环外一次性摊平成单一 `existing_questions` 集合（并集语义），内层 O(1) 命中；新增 2 条跨多条历史的去重用例钉死并集语义 | S |
| B205 | **已关闭（L135）**：`DataSanitizer.remove_duplicates(keep="last")` 每次遇到重复都在**全 result 上**逐条 `r.get(key)` 线性找槽位，O(n×distinct)；重复多的数据上显著热点 | N=40000/D=400 实测 new/old **×0.0235**（≈42×，双序 min-of-2）；seen 改存 result 下标、O(1) 覆盖；等价性 300 dup 密集集 + 50 keep=first 集逐元素一致；1 条新槽位不变量用例 | S |
| B206 | **已关闭（L136）**：`EnhancedComparator._compare_fields` 的 instruction 匹配是 O(F×A×B) 逐条扫 B、每字段再各扫一遍 A/B 算 count；且 `type_mismatches`/`value_differences`/`diff_datasets` 成员语义无直达行为用例 | 双层索引 `instruction→{field:首个含该field的item_b}` + 一次 O(A+B) 字段计数，匹配变 O(A)；N=1500/F=12 A/B new/old ×0.097；等价性 200 随机数据集逐字段逐差异一致；补 3 条行为用例 | M |
| B207 | **已关闭（L137）**：`AugmentorPipeline._process_single_item` 的 except 分支里 `_process_errors` 惰性「没有就建 + append」非原子——并行路径（ThreadPoolExecutor 多工作线程）并发触发时，两线程都读到 `hasattr` 为 False 各自建 `[]`，后建者吞掉先 append 的那条；而 `__init__` 里的 `self._lock` 建了却全程无人用（dead lock） | 持 `self._lock` 包住 check-then-act；补 `TestProcessErrorsThreadSafety` 2 条（并发 N 条错误一条不丢 + 源码级守卫钉死锁确实被用上，防未来把锁拿掉却因 GIL 窗口窄测不出） | S |
| B208 | **已关闭（L138）**：`ActiveSampler.generate_report` 里 `recommended_seed_indices` 用 `items.index(seed)`（== 相等）取「seed 的下标」，内容相等的兄弟 item 会误报第一个的下标；候选「改 id(seed) 索引」优化经 A/B **证否**（真实 k 很小，O(n) 建表 > per-seed 2×O(n) 扫描，反而更慢）故不改产品，改补下标精确性守卫 | A/B：n=3000/50 seeds 下 new(id-map)/old ×3.56（变慢）；新增 1 条钉死「下标＝被选中对象本身的下标，不是内容相等里的第一个」的守卫 | S |
| B209 | **已关闭（L139）**：`EnhancedComparator._generate_recommendations` 的 4 条字符串分支（size_diff>±100 / 相似度<0.5 / 类型不匹配 / 值差异）此前**无任何直达断言**——既有用例只 `assert "recommendations" in d`（key 存在），全分支实际盲跑 | 补 3 条字符串守卫：size_diff=150⇒「数据集A比B多 150 条数据」；similarity=0.1⇒「相似度较低」；type_mismatches=2/value_diffs=1⇒各一条「字段 X 存在 N 个…」（含「高度相似」顺带路径） | S |
| B210 | **已关闭（L140）**：`turns_to_canonical`（chatml/vicuna/sharegpt 逆运算）末尾两轮角色检查用裸 `["role"]`，缺 role 键时抛**裸 `KeyError`** 而非契约承诺的 `DataFormatError`（函数 docstring Raises 段明写抛 DataFormatError）；且该逆运算此前**无直达单测**（只在集成 CLI 用例里被间接触发） | `["role"]` 改 `.get("role")`（缺键 → 比对 `None` ≠ "user"/"assistant" → 走既有 DataFormatError 分支，顺带把 f-string 里的裸引用一起改）；补 `TestTurnsToCanonicalContract` 2 条（缺 role 键 / 角色顺序错），实证改前 KeyError 红、改后 DataFormatError 绿 | S |
| B211 | **已关闭（L141）**：`VersionManager.diff()` 用 `(instruction, output)` 组合键做身份 ⇒ 「原地改某条 output」被拆成「删旧+增新」两条，`modified` 分支**恒为空**（`modified_count` 永远 0），而 `DiffResult` 字段注释与 docs/API.md 示例都承诺 `modified_count` 非零；既有唯一守卫还是空转的 `assert modified_count >= 0` | 身份键改成 `instruction` 单键，同 instruction 两版都在 ⇒ 整条 dict 不等即计入 `modified`；原地修改 now 报 modified=1/added=0/removed=0。既有 4 条 diff 用例（added/removed/identical/empty）全绿无回归；强化 `test_diff_modified_items` 断言 + 新增 1 条原地修改守卫 | M |
| B212 | **已关闭（L142）**：`QualityTrendTracker.compare_trends()` 的 `comparison` 用「A > B else B_higher」二态判据——**平局**与「某侧根本没记过该指标」都误报 `dataset_b_higher`（缺指标被静默当 0 比，语义错）；既有唯一用例只钉了 A>B 一条 | 改成三态：双侧都有最新值且 A>B ⇒ `dataset_a_higher`，B>A ⇒ `dataset_b_higher`，其余（含平局、任一侧无记录）⇒ `tie`；补 3 条用例（平局 / 双侧缺指标 / 单侧缺指标）钉死 `tie` | S |
| B213 | **已关闭（L143）**：`ActiveSampler.identify_underrepresented` 把 `length_distribution` 的全部键都当占比桶遍历（`.items()`），但 `_analyze_length_distribution` 会额外塞一个 `avg_length` **均值**键（绝对值，非占比）；当 `avg_length < threshold` 时会误产出伪桶 `length:avg_length`，污染 `recommend_seeds` 的 underrepresented 列表并白占一个 top_k 名额（其匹配循环因没有该 bucket 分支而永远打不中） | 长度桶改为显式只遍历 `("short","medium","long")`；`avg_length` 不再是候选。等价性 200 随机数据集 new⊆old 且排除 avg_length 全过；新增 1 条守卫（空指令 avg_length=0 < 0.1 时不产出 `length:avg_length`），改前红/改后绿 | S |
| B214 | **已关闭（L144）**：`DatasetOperations.merge()` 的 `max_items` 截断用 falsy 判据 `if config.max_items and ...`——`max_items=0` 语义是「一条不留」，但 0 被读成「不限」而返回全量（`merge_files` 委托 `self.merge()`，同一截断点一并覆盖） | 判据改 `is not None` 并加注释钉住 falsy 回归防护；补 `test_merge_max_items_zero_keeps_none`（改前 0 返回全量，改后 0→0 条；对照 None→全量、3→3 条防修过头） | S |
| B215 | **已关闭（L145）**：A184 引用普查的索引跳过集只有 .git/__pycache__/.pytest_cache/node_modules——测试套件自写的运行时快照目录 tests/.backups/（gitignored，门禁的 fixture 运行期写入 index.json、snap.json、opt100_progress.md）进索引：跑过一轮全量门禁后，index.json、snap.json 这类文件名有多候选，下一次门禁普查 9 条引用被翻 live→ambiguous（336 vs 钉值 327），棘轮测试红——门禁状态依赖、自伤 | 快照目录加入索引跳过集（语料面跳过集一直有、索引面漏配；L79 的 build_index 只收 .py 不受影响）；补 1 条守卫（改前红：3 条泄漏候选；改后绿）；按「动案面要重跑普查再改这里」条款逐格重钉：scratch_missing 289→302（排除快照目录候选后 13 条翻进本档，逐格归因），ambiguous 327 与其余各档原地未动；幂等性实证：普查含/不含快照目录逐格同读 | M |
| B216 | **已关闭（L146）**：QualityTrendTracker 的趋势历史文件加载失败（JSON 损坏、`trends` 非列表）时**静默置空**历史并只告警一条；随后记录指标触发的保存会用 `open` 写模式**截断覆盖原文件**成「仅含新条目」——原始数据永久丢失、零备份；且原实现不校验 `trends` 类型，它是字符串时下一步追加直接崩 | 畸形文件先备份为带时间戳的损坏副本再置空（同秒重名自动加序号；Windows 句柄被占用挡掉 rename 时自动退化成复制式备份）；两条备份路径全断才置禁写标记、保存跳过（原文件逐字保留、内存历史不丢）；`trends` 非列表改抛领域异常 DataFormatError（对齐裸内置异常 raise 守卫）；新增损坏防护测试类 3 条，改前红 3 failed（旧版文件沙盒实证）/ 改后绿 | M |
| B217 | **已关闭（L147）**：增强导出器的条数限制分支写成 falsy 判据——max_items=0 语义是「一条不导」，但 0 被读成「不限」而导全量（与 L144 的 merge() 缺陷同族，全仓漏网的最后一处） | 判据改 `is not None` 加 `>= 0`（负数仍读「不限」维持旧行为，防修过头）；补 1 条回归（0→0 条；对照 None→全量、负数→全量），改前红（实测导了 3 条）/ 改后绿 | S |
| B218 | **已关闭（L148）**：merge_files 的 `removed_duplicates` 统计把「去重删除数 + max_items 截断数」混报成一个值（`total_input - total_output`），用户读到的「去重数」在带截断的配置下虚高（实测 8 条入、去重 2、截 3：旧报 5 新报 2） | 在 merge_files 内部用既有 _deduplicate 复算去重删除数、截断数 = 去重后基数 - 输出数，两数分报（新增键 `truncated_by_max_items`，既有键语义改为只记去重）；实现刻意控制在 242 行参考点以下净增 8 行——历史文档对 dataset_ops.py:242/:243 的行引用平移后仍落在代码行（棘轮 58/58 绿），首版把统计逻辑放进 merge() 净增 15 行、把 :243 翻进空行致 A184 硬 0 红，已回退换点；补 1 条回归（去重+截断双开分报 + 无去重无截断全 0 对照），改前红/改后绿 | M |
| B219 | **已关闭（L149）**：sample() 的比例分支 `elif config.ratio:` 用 falsy 判据——显式 ratio=0 语义是「采 0 条」，但 0 被读成「未设置」直接落 else 采全量（与 L144 max_items、L147 导出 max_items 同族，本处是 ratio 位） | 判据改 `is not None`（1 行替换 + 2 行注释，插入点在被历史行引用的 242/243 之下，棘轮不受扰）；补 1 条回归（ratio=0⇒0 条；对照 None⇒全量），改前红/改后绿 | S |
| B220 | **已关闭（L150）**：_deduplicate 的 threshold 形参从不被读——`MergeConfig.dedup_threshold` 文档承诺「去重阈值」、auto_config 按重复率逐条推荐数值并写理由（「上调/下调去重阈值」），但任何取值结果都相同（死旋钮，全仓 0 消费点） | threshold 分档消费：≥ 0.9 只删 instruction 完全相同（原默认行为，保守档），< 0.9 追加宽松键（大小写折叠 + 全空白删除）把近似重复一并删；字段注释与 docstring 同步改写；宽松键归一化首版用单空格 join（多 token 折叠回一个空格、与无空格形态仍不等，测试当场抓出），改全空白删除。auto_config 的推荐数值（0.85–0.98 区间）自此真有其效：高重复率→0.98 保守档、低重复率→0.85 宽松档，方向与其既有理由文字一致，零改动。补 2 条回归（宽松/保守档对照 + merge 层配置真消费），改前红 2 failed/改后绿 | M |
| B221 | **已关闭（L151，收口第一本账 A125）**：两个组件构造器的阈值判据各缺一半——Deduplicator 的 `threshold < 0 or threshold > 1` 对 NaN 双假放行（去重整条静默零动作）、对 True/False 读成 1.0/0.0（False 实测误删 2 条真数据）、对 None/非数值延后到比较处炸 TypeError；QualityScorer 七档坏值（None/字符串/NaN/bool/越界）全放行、判负迟到 score()。配置层（DedupConfig/QualityConfig）L76 起就拒，组件层 15 轮未收 | 两组件阈值统一接 `require_ratio`（界与配置侧共引 `DEDUP_THRESHOLD_RANGE` / QUALITY_THRESHOLD_RANGE，A77 同式）+ None 显式拒；Deduplicator 仍抛 DedupError 且文案不带键名（消费侧公开契约由 L76 钉子钉住）。L76 两条「洞本体」钉子按 docstring 红字约定翻转（改前红 11 / 改后绿），新增 QualityScorer 阈值守卫 11 例 | M |
| B222 | **已关闭（L152，收口第一本账 A46①）**：`create_stream_processor(processor_func, chunk_size=1000)` 的 `chunk_size` 形参声明了从不被读——工厂闭包逐条处理、无状态，块边界无任何可观测后果，「设了没生效」是静默的，且全仓 0 调用点 | 删除形参（结构性填不了⇒删，不发明语义）；补签名守卫（断言形参不在）；更新钉死该形参的既有调用。A46②（StreamConfig 两字段断线，产品内 0 使用者）留档后续轮 | S |
| B223 | **已关闭（L153，收口第一本账 A124）**：`quality.weights` 的形状判据权威住组件（`QualityScorer` 两行手抄只判长度与和）——配置层只判 null，两面对同一键不同判；三档洞：`'abc'` 长度恰 3 死在 `sum()` 的 TypeError、`[-1.0, 2.0, 0.0]` 静默放行（权重符号反了排序整个反过来不出声）、`[True, True, False]` 被 sum 当 `[1,1,0]` | 判据权威搬进 validation 族（新族员 `require_ratio_list`：逐项 bool/NaN/越界 + 长度 + 和=1±0.01），配置层 / 组件层 / 静态回放三面共引同一份（A77）；`quality.weights` 进静态规格表、L76 的「唯一豁免」钉子按 docstring 红字约定翻转为「规格在场 + 坏值两面同拒 + 合法 6/6 两面放行」；`[]` 由静默回落默认改为即拒。新增 6 条组件层守卫 + 翻转 1 条，改前红/改后绿 | M |
| B224 | **已关闭（L154，收口第一本账 A140）**：A140 余 11 节 29 键（context / versioning / sampler / expander / tracker / visualization / multilingual / evaluation / benchmark / active_learning / frameworks）四面零反馈——运行时零判据（一个节内判据都没有）、静态规格表零行、不可经 API 写入 ⇒ 坏值全静默进加载面；三节有真读者（pipeline 直读 num_turns / storage_dir / auto_snapshot，CLI 基准子命令读 metrics / baseline_file），假零 / 字符串假真 / 标量拆字符等档症状要到消费日才出声 | 11 节整批接节（同式同族一批收掉，A140 原框「单节约等于 S」）：节内判据引 validation 族既有成员（12 布尔键、4 计数键取正、5 字符串键、8 清单键、null 全拒），不新造判据不新造常数；静态面补 11 节 dict 行 + 29 键规格行与运行时同档；加载面构造器自动吃到。新守卫 8 类（普查名单翻转 + 坏值 × 面矩阵 + 合法反向 + 删规格行独立性 + 加载面 + 消费面）；l83 分档 13 键移档、validator 两计数扩员、l73 棘轮第三次翻空 + 1 用例按其 docstring 明文处方退役（假臂无来源 ⇒ B225 留档）；行引用 4 条降名锚 + L79 / A184 / l97 逐格重钉 | M |
| B225 | **已关闭（L155，留防御支 + 钉死其契约）**：`apply_section_update` 的假臂（节无节内判据时「写直通、不复查」那一支）自 L154 全 20 节接上判据起仓内无来源 | 处置二选一拍定为「留」：写入路径拿到的是任意 dataclass 节对象，无判据节（含用户自定义节）是合法入参，删臂 = 对外行为变更；l73 新增 `TestTheUngatedWriteThroughIsAPinnedContract`（2 例）把「写直通、不吞键、不做判决（有判据节会拒的 0 / 负数照落）」钉成契约，谁删臂或顺手接判据就红。测试-only 一轮，产品码零改动、无行引用位移 | S |
| B265 | **新立（L197 立案，未开工）**：**封闭清单族还剩 6 处 SDK 直构面漏网**——与 L175/L176/L189/L192/L193/L194 同族，全部实测过症状：① `visualize_enhanced.visualize_dataset(format=)` 未知值静默走 text 支还按 text 落盘（`"txt"` / `"JSON"` / `"json "` / `None` 全中）；② `indexer.DatasetIndexer.search(method=)` 未知值静默回落 contains，而 `QueryResult.index_used` 回显 `"regex:instruction,output"` —— **结果主动谎报实际方法**（比静默默默更糟，读结果的人无法自证）；③ `validation.DataSanitizer.remove_duplicates(keep=)` 的 `"Last"` / `"firts"` 静默按 first 走（B205/L135 两轮优化过这条路径的复杂度，却没补判据）；④ `dependency.DependencyManager.get_dependencies(direction=)` 未知值三条分支全不命中 ⇒ 返回 `[]`，用户读到的是「这个数据集没有任何依赖」这个**正面论断**；同文件 `add_dependency(dependency_type=)` 同样零判据；⑤ `streaming.StreamWriter(mode=, format=)` 两个字符串形参各只与一个字面量比一次 ⇒ `format="csv"` 把 JSON 数组写进用户声称为 csv 的产物、`mode='a' + format='json'` 写出无括号的半份 JSON；⑥ `quality_gate` 的 `rule.severity == "warning"` 漏网 ⇒ `"warn"`（少个 ing）静默升级为 error 级门禁阻断流水线。另 `indexer.DatasetView` 的 `format=`（:499）同形 | 一律接 `validation.require_choice`，合法值清单立为 A77 单一权威并与 API 面共引（L175/L189/L193 先例）。**建议一批收掉：同族同式，分开做会出现「修了 5 处漏 1 处」的 L194 型二次欠账** | ~~M~~ **已收口（L199 七站点 + L209 把立案不做的 dependency_type 也纳入清单）** |
| B266 | **部分关闭（L201 收 StreamWriter 那一处；其余仍在案）**：C 族「except 后继续用半成品状态」——`streaming.StreamWriter.__exit__` 不看 `exc_type` 无条件补 `]`，而 `write_chunk` 又先写 `,` 再 `json.dumps`，一条记录序列化失败时产物是 `[{"a": 1},]`，**非法 JSON 却看起来像写到一半**（探针实测 `json.loads` 报 Illegal trailing comma）——**L201 已修**（先序列化再写分隔符 + 异常时不补闭合括号，注入逐半退回都红）；`mode='a' + format='json'` 不参与方括号记账，追加写出 `[{"a": 1}]{"b": 2}`（已写成一条**钉住已知限制**的用例而非假装它合法）；`quality_gate` 把「规则算不出来」（TypeError）与「算出来不及格」塞进同一个 `failed_rules`，`verdict=FAILED` 可能来自一条坏规则而不是坏数据；`pipeline._process_errors` 收进状态却**零读者**（`augment_dataset` 返回报告里没有 `errors` 键）且跨调用累积；`data_pipeline` 的 `stop_on_failure` 默认 False，阶段 B 抛异常时阶段 C 拿到的是**阶段 A 的输出**，整条管道少跑一环却照常跑完 | **订正一条原记录**：`context.batch_generate`「并行支吞失败、串行支抛异常」**经复核不是活缺陷**——`generate_multi_turn` 内部已捕获后端异常并降级返回空历史（`test_parallel_failure_does_not_drop_other_items` 钉的正是这个），并行支的 `except` 对后端错误不可达；原记录是「读到 except 就算缺陷」没追到上一层。**其余处置前提**：quality_gate 要同时改 SDK 的 `GateReport` / `api/routes/quality.py` 的 `GateReportResponse` / 守卫三面——只改 SDK 那一面会让新键被 FastAPI 按 `response_model` 静默裁掉（L27 记过的坑）；追加语义要先拍「追加 JSON 数组」的形态（口径决策）；`_process_errors` 要往返回报告加键（改 API 响应面） | M |
| B267 | **新立（L197 立案，未开工）**：`preview.PreviewGenerator` 的缓存两个问题——① 命中后把 `ExportPreview` **就地改后按引用返回**，同实例所有调用方共享同一可变对象；② 缓存键只哈希 `sample[:min(5, len(sample))]`，`preview_size > 5` 时「前 5 条相同、后续不同」的两个数据集**撞键**，第二个调用方拿到第一个数据集的 `original_data` / `converted_data`（只有两个计数被刷新）。L127 的注释自称「三者缺一即换键」，但数据因子被截断到 5 条 | 键改整份 `sample`（或加长度 + 全量哈希）；命中时返回 `dataclasses.replace(cached, format_info={...})` 的副本 | S |




## 循环日志

### L101（2026-09-28）— A201 容错路径族 16 条缺数清零

- 三支文件的守卫支第一次被踩：`augmentor/retry.py`（HTTP-date 解析器返回 None、headers.get 抛
  TypeError、5xx 不在可重试集合仍判重试、1xx/3xx 不重试）、`version_control.py`（历史落盘
  OSError 只告警不炸操作、空行与坏 JSON 行跳过）、`models/base.py`（生成缓存缺席时
  _cache_get 返回 None / _cache_put 惰性自建、__del__ 吞 close 异常——该例以「close 确实被
  调用」断言防空转，异常逃逸则由 pytest unraisable 插件接住）。
- **全量（第二轮，干净）**：`C:\Python314\python.exe -m pytest -q` → **7290 passed / 3 skipped，
  exit 0，178.37 s**；TOTAL 13,009 语句缺 **21**（37−16，全部落在本轮三支，逐支 0 缺数）/
  分支 3,808 偏支 **80**（85−5：base −2、retry −2、version_control −1，全是挂在被本轮覆盖的
  缺失行上的隐藏弧；其余约 40 支偏支与 L100 逐支对齐）。测试数 +14 = 本轮 9 例新用例 +
  文档守卫对新账本的逐文档参数化 +5（`--collect-only` 实证：移走账本 110 例、放回 115 例）。
- 操作坑三笔（本账开账即撞，全部当场归因）：
  1. 给定向跑追加 `--cov=augmentor.retry` 这类子模块目标会与 pytest.ini addopts 里的全包
     `--cov` 叠加，numpy 3.0.2 的「每进程只许加载一次」保护当场引爆（ImportError: cannot
     load module more than once per process）⇒ 定向跑要么 `--no-cov`、要么只吃 ini 自带的
     全包 `--cov`，不要手拼子模块目标。
  2. **全量套件跑着的时候，不许在仓库根 / `data/` / `.backups/` 顶层创建新文件**：第一轮
     全量是我在后台跑着、同时写下本账本文件，A168 残留守卫（逐用例快照差集）把新文件判成
     「当前跑到的那条用例留下的产物」，error 记到了 test_dependency 头上（7285 passed +
     1 error）。守卫只盯顶层新名字的存在性，内容与编辑不可见 ⇒ 账本必须先于套件存在，
     套件运行期间只许编辑已存在的文件。重跑第二轮干净（本格读数即第二轮）。
  3. **偏支对账必须按 brpart 列数，不许数显示出来的 `->`**：coverage 对「挂在本就缺失的行
     上的弧」不显示（L100 的 base 行 brpart=3 只显示 1 条弧）⇒ 按显示数对账会差出 5 条
     且方向可疑，差点记成一笔「归因不明」。


### L102（2026-09-28）— A202 路由/依赖/配置错误路径族 11 条缺数清零（含撞库记录）

- 8 例新增（5 例路由注入 + 1 例 deps + 2 例 config），定向 15 passed（含 7 例既有）。
  命中证据（定向覆盖读数，ini 自带全包 `--cov`）：六支文件的目标行从缺数清单消失——
  `api/routes/quality.py` 535/541-542、`api/routes/export.py` 90-91、`api/routes/augment.py` 89、
  `api/routes/multimodal.py` 90、`api/deps.py` 188-189（连同 196->199 偏支）、
  `augmentor/config.py` 1110-1113。
- 立项时的两处口径订正：① health-gate 的 `except FileNotFoundError` 是**防御支**——
  `load_items` 对缺失文件抛的是 HTTPException(404)（走复位支），原生 FileNotFoundError
  要用假 load_items 注入才命中；② augment/multimodal 复位支的触发器是 `..` 组件的
  **400**（不是 403）——`resolve_within_roots` 对 `..` 直接判 400「路径包含非法组件」。
- **撞库记录（本账最重要的一笔，旧账本 L8 事故的同族再现）**：18:46–18:52 检测到
  **另一活动会话与本会话共享同一工作树**（同在 augmentor-opt100 分支）：其改动为
  产品三支（`augmentor/checkpoint.py`/`augmentor/tracker.py`/`augmentor/visualizer.py` 改「构造不碰盘、首次写才建目录」）
  + 新测试 `test_lazy_output_dir.py`（非本会话所建）+ `test_tracker.py` + 旧账本一行
  + 两份 README，全部未提交、仍在途。
- 撞库当日全量的 9 failed 逐例归因（**没有一例属于本会话改动**）：4 例文档守卫 =
  瞬态（本会话全量读到对方**写了一半**的旧账本；隔离复跑 57 passed 全绿）；2 例既有
  visualizer 测试（`test_output_dir_is_created` / `test_output_dir_attribute`）与产品新行为
  相逆（断言构造期建目录，产品已改惰性）+ 对方新测试 2 例全量序下失败 + tracker 1 例 =
  全部是对方在途工作的中间态。**处置**：不碰对方文件、不代其修复、只用显式路径提交
  本会话自己的文件；干净全量读数待工作树安静后由后续轮次补跑（缺数 37→12 的总趋势
  已由 L102 前的一次全量 + 两次定向覆盖读数双向印证）。
- **新纪律（本轮起每轮开工先跑）**：`git status` 先看有没有别人的在途改动——有则在本轮
  账本记一笔、只碰自己的文件、全量读数改用「定向覆盖读数 + 撞库注记」的组合口径，
  等工作树安静后再补一次干净全量。

### L103（2026-09-28）— A203/A204：CLI version delete 三支 + 预览非原生格式守卫

- 4 例新增，定向 4 passed（含一处断言修正：`_run` helper 的 code 语义是「None = main()
  自然走完未判负」——成功路径不触发 SystemExit，`code == 0` 是我抄错既有用例的形状；
  修法是验输出标记 + 用 `list_versions()` 复核版本真被删掉，不放宽成对 None 装没看见）。
- 命中证据（定向覆盖读数）：`augmentor/cli/commands/version.py` 98-106 与
  `augmentor/preview.py` 177 从缺数清单消失（version.py 仅余 `98->exit` 形状的偏支弧，
  与 L100 基线同形）。
- 一个有趣的形状：`preview` 里同样的「不支持的导出格式」消息有两处 raise（155 与 177）
  ——前者管「不是合法 ExportFormat 成员」，后者管「成员合法但没有原生转换器」；
  只测前者时 177 依然黑着，两格要分别喂 `openai`（合法成员、非原生）与真正的坏串。


### L104–L113（2026-09-28）— A206 前端页面级组件测试从零建立（10 页）

- 覆盖 10 页各一份 一份 `Dashboard.test.tsx` 同型文件，逐页一轮一提交：Dashboard（统计卡/阈值/模型标签配色/时间线接线/四读取失败可见性）、Settings（数值控件回填/保存负载 default_model 取下拉值+七节透传/加载与保存失败）、System（依赖齐备与缺失两态 Alert 与明细表标签/降级功能/模型可用否/失败不渲染明细）、Augmentation（未选文件 warning 不调服务/五参数负载/文件列表失败/轮询进度详情）、Multimodal（格式提示/单条结果卡与图像信息/目录扫描统计与记录表/处理失败）、Export（格式下拉/未选 warning/预览条数标签与转换表/批量三必填校验 + 去扩展名 reduce）、Versions（版本表/创建未选文件 warning/创建负载/对比三计数）、Analysis（选文件触发分析与四统计卡/未选清洗基准 warning/清洗四计数/基准指标表）、Quality（run 守卫/评估四统计卡与通过率/去重 Tab 四计数/离群点三参数）、Security（PII 清单/脱敏 includeExtra 入参与命中统计/就绪审计结论/泄漏三参数）。
- 终态门禁：前端全量 `vitest run` **13 文件 / 112 例全绿**（此前 2 文件），`tsc --noEmit` 0 错，每份新测试 `eslint --max-warnings 0` 通过。
- 复用的一套接线口径（承既有 DataManagement.upload.test）：① `vi.mock('../services/api', importOriginal)` 部分替身——只替换要断言的函数，`apiErrorDetail`/`bulkErrorDetail` 留真身，否则断的是替身行为；② jsdom 补 `matchMedia`/`ResizeObserver`（antd v5 主题与响应式 hook 依赖，jsdom 不提供）；③ vitest 未开 globals ⇒ RTL 自动 cleanup 不生效，每轮 `afterEach` 显式 `cleanup()` + `message.destroy()`（antd message 容器挂在 RTL 容器之外）。
- 本轮踩到并落账的六个前端测试真坑（都当场归因、非放宽断言）：
  1. **antd 两 CJK 字按钮名带插入空格**：`处理` 的 accessible name 是「处 理」（antd 给双中文按钮插 `\u2005`）⇒ 用 `/处.*理/` 匹配；「扫描目录」「开始增强」这类 ≥4 字或非纯双字的不插。
  2. **默认 antd Modal 按钮是英文**：本仓未配 `ConfigProvider locale=zhCN` ⇒ Modal 的确定/取消是「OK」「Cancel」而非「确定」。
  3. **Modal 渲进 document.body 的 portal**：`container.querySelector` 找不到弹窗内的输入，必须用 `document.querySelector` / `screen`。
  4. **antd Statistic 拆数值**：数字被拆进整数/小数多个 span，`getByText('18.3')` 必失配 ⇒ 读 `.ant-statistic-content-value` 节点整体文本。
  5. **同名文本多处命中**：统计标题「有效」与记录标签「有效」同时在 DOM ⇒ 按 `.ant-statistic-title` 类精确定位，不用 `getByText`。
  6. **antd 多选 Select 在 jsdom 里点选不稳**：Export 批量导出的多选交互点不进值 ⇒ 改测「三必填校验消息可见 + 不调服务」这条稳态守卫 + 用与源码逐字一致的表达式钉住去扩展名 reduce 的纯函数语义。
- **门禁工程性一笔（`284d89e29`）**：测试文件从 2 份增至 13 份后，13 文件并行时 antd 挂载渲染的挂载期 `waitFor` 偶发超过 vitest 默认 5s testTimeout（**单文件跑必过、6 文件子集必过、13 文件并行才偶发**，失败者耗时 5–7s 卡在 testTimeout 边界 ⇒ 判定为 CPU 争用而非跨文件泄漏）。处置：`vitest.config.ts` 抬 `testTimeout`/`hookTimeout` 到 20s（真失败是断言不符、秒级即报，不会拖到 20s，所以不掩盖真错）。抬高后 13/13 稳定复绿两次。
- 口径说明：coverage.include 仍只统计 `src/services/**`（组件层不计入服务层阈值），所以这 10 份页面测试不改覆盖率数字，价值在**行为守卫**（挂载读取、失败可见性、提交负载形状）而非覆盖率百分比——与后端 A201–A204 的「缺数清零」是两类指标，分开记。

### L114–L115（2026-09-28）— A205 偏支逐支审计（首两批：可达假支清零 + 不可达记档）

- **前置读数订正（重要）**：L101–L103 之后的干净全量（`C:\Python314` / `pytest -q`）实测
  **7313 passed / 3 skipped / exit 0，TOTAL 13,011 语句缺 0 / 分支 3,808 偏支 77**。
  即后端**语句缺数已在 L101–L103 清零**（37 → 0，基线 L100 记的 37 是含 L101 前状态），
  偏支 85 → 77。本账基线段那句「缺 37」是 L100 终态、已被 L101–L103 推平，此处订正。
- **L114（`b7f1b6f17`）export_enhanced 6 条可达假支**：sharegpt/chatml/openai 三个对话式
  转换器里 `if instruction:` / `if output_text:` 的假支——只有半条记录（空问题或空答案）
  才走「跳过该轮消息」。3 例参数化用例喂「只有问题」「只有答案」两条数据，实证六条偏支
  （241->247 / 247->253 / 268->274 / 274->280 / 345->347 / 347->350）覆盖。
- **L115（`87174e26b`）validation 2 条可达假支**：`remove_duplicates(keep="last")` 让重复值
  排在 result 第二位，替换循环必须先跳过第 0 条不匹配记录（864->863）；`sanitize_dataset(
  remove_duplicates=False)` 跳过去重直接返回（899->902）。
- **A205 方法学（本轮拍定，承第一本账「判据只盯可观测行为」）**：偏支分两类处置——
  ① **可达假支**：构造触发它的真实数据形态补用例（本轮 8 条）；② **结构性不可达支**：
  不许用「调私有方法传不可能的参数」硬凑覆盖，而是逐条记档说明为何不可达。本轮已判定的
  不可达支：`augmentor/validation.py` 的 `822->826` / `826->816`（`_sanitize_item` 里的 `if fix:`
  假支——该方法只在 `sanitize(fix=True)` 时被调用，`fix else item` 那支根本不进函数 ⇒
  函数内 `fix` 恒真，两条假支是冗余防御码，属「可简化」而非「可测」）、`863->855`
  （去重替换的内层循环「找不到匹配就退出」那支——`value` 在 `seen` 里 ⟺ 它一定在 `result`
  里，内层循环必然 break，退出支不可达）。这三条其后已由后续轮次关闭：恒真 `if fix:` 守卫在 L126 删除（`_sanitize_item` 注释就地标记），去重替换循环的退出支在 O(1) 下标重写后
  结构性消失（`value` 在 `seen` 里 ⟺ 必在 `result` 里，内层循环不再存在）。
- 全量待下一次干净全量落账（前端 13/13 已绿、不受影响）。

### L114 补记（census 基线联动，本账新纪律）

- 新增/删除任何 `@pytest.mark.parametrize` 站点会动到第一本账 L97/L98 立的**全仓 parametrize 普查基线**：`tests/unit/test_parametrize_ids_l97.py` 的 `MEASURED_CALLS` / `MEASURED_READABLE` / `MEASURED_BLIND`（l98 从 l97 import 这三个数）。L114 那支对话式空字段参数化桶=plain（可读、无 `ids=`）⇒ `MEASURED_CALLS` 356→357、`MEASURED_READABLE` 263→264，`MEASURED_BLIND` 93 与 L98 六档分布一字未动（`d3ab8c78f`）。
- **纪律**：本账任何新增 parametrize 的轮次，scoped 集必须带上 `test_parametrize_ids_l97.py` + `test_parametrize_blind_face_l98.py`（后者的动态面测试需全量 ≥3000 用例才不报 RulerNotProven，所以定向跑只验 l97 的计数校验 + l98 的 `test_census_agrees_with_the_l97_ruler`，六档分布随全量核）。本轮就是漏带它、靠干净全量才抓到（2 failed 全是这两条计数校验），已就地补平。

### L116–L119（2026-09-28）— A205 偏支逐支审计（可达假支批量清零）

承 L114–L115 的 A205 方法学（可达补测 / 不可达记档），本四轮按「退化/非常规/空输入触发的假支」
逐支补真行为用例，每轮一提交：
- **L116（`4ea67aaac`）quality_report 3 条**：to_markdown 无改进建议跳章节（87->96）、一致性统计跳过空 output（217->213）与自我重复条（219->213）。
- **L117（`4c979939e`）六模块退化输入 6 条**：privacy 重复命中类型不重复入表（119->118）、quality_monitor/quality_trend 缺指标跳过（194->193 / 93->90）、data.cleaner remove_urls=False 跳 URL 清洗（97->99）、data.multimodal 空文本不记 text 模态（96->99）、migration 对话项非字典跳过（124->123）。
- **L118（`0bf96fbec`）六模块非常规值 6 条**：feature_detect 列表值三档 elif 全不中（130->122）、cleaner 未知规则跳过（153->152）、compare 缺字段跳过（251->250）、audit 无泄漏不追加 finding（131->137）、indexer 非串值不入索引（147->145）与已小写值不重复记键（152->145）。
- **L119（`2fd83c9b2`）空/不可解析输入 3 条**：report 空 scores 摘要为空（115->113）、json_extract 围栏内不可解析 JSON 回落平衡扫描（136->143）、平衡片段解析失败回退下一开括号（150->143）。
- 合计本四轮清 18 条可达假支（偏支 69 起，逐轮下降，全量读数待阶段性干净全量落账）；测试组织：L117–L119 用独立的 test_partial_branches_l11N（各轮一支）批量文件（每条用例注释点名对应偏支坐标），L116 就地并入 test_quality_report.py。
- **A205 进度口径**：偏支两类处置继续执行——已补测的都是「真实数据形态触发的假支」（空/缺字段/非常规类型/重复/开关关闭），未纳入本批的多为需第三方库（sklearn/faiss）或并发竞态（model_manager 双检锁 23->26）或循环 break/exhaust 的结构性不可达支，留后续轮次逐条判定记档。

### L120（2026-09-28）— A205 可达假支第二批 + 不可达判据落档（`9af6ded88`）

- 补测 7 条可达假支（`test_partial_branches_l120.py`）：export chatml `system` 显式空串跳过系统
  消息（173->180）、CSV 空项不落文件（277->286）、pipeline 变体过滤（195->194 两个假支：缺
  instruction 键的 dict + 非 dict）、version_control 删「当前版本」三向（目录缺失 321->326 /
  有其它版本回退列表尾 329 真支 / 最后一条置 None 329->335）、versioning 列版本时遇到
  「目录在、metadata.json 缺」的目录跳过（256->253）。
- **判定为结构性不可达、记档不测**（不许用绕过入口的构造硬凑）：
  - `augmentor/quality.py` 222->230：`_calculate_diversity` 入口有 `if not existing_texts: return 1.0`
    （:181），`diversity_sample_size` 判据 `require_count(minimum=1)` ⇒ 走到 :219 时切片
    `existing_texts[:n]` 恒非空 ⇒ `if sample_texts:` 假支不可达（冗余防御，保留）。
  - `augmentor/search_enhanced.py` 221->exit：`_ensure_indexes` 公共路径 :218 已判 `_index_ready` 为
    False 才进锁，锁内无其它置位路径 ⇒ 内层双检的假支不可达（防御式 DCL，保留不改）。
  - `augmentor/model_manager.py` 23->26：双检锁内层假支需线程竞态触发，单线程测试构造不出 ⇒ 记档。
  - `augmentor/visualizer.py` 46->exit：懒加载 `if self._wordcloud is None:` 假支需同一实例两次调用
    才见，属「懒加载族」，留后续轮次统一补。
  - `augmentor/pipeline.py` 352/369 的 `if use_checkpoint:`：成功侧公共路径可达已覆盖；失败侧需
    `success=False`，而 `_process_single_item` 对模型异常是「catch 后 put(idx, True, [])」
    （:245 恒 True）⇒ 公共路径构造不出失败 ⇒ 记档。
- **A184 棘轮本批一度红、根因是探针残留不是语料位移**：全量门禁起前 A184 的
  `test_bucket_matches_its_ceiling` 报 `scratch` 731（常数 729）/ `scratch_missing` 287（常数 289）。
  逐条对照现量普查锁定翻转源：本轮回调探针往 `Temp/l120probe*/` 写了 4 份带 in/out 扩展名的
  JSON 输入输出件，而带 in 扩展名的输入件这个名字在案面里有 2 处反引号引用
  （`OPTIMIZATION_LOOP.md:2483`、`tests/unit/test_write_landing_l93.py:4`），`name_bucket` 按
  「Temp 里有同名工件」判 scratch 档，于是 2 处从 scratch_missing 翻进 scratch —— 正好 +2/−2。
  处置是**清掉探针残留目录**（`Temp/l120probe` ~ `l120probe4`，均为本轮 15:09–15:10 生成、
  语料零引用、未入库），棘轮即复原 729/289，CEILING 一字未动。**新纪律**：A184 的 scratch
  两档会随**磁盘上的 Temp 同名工件**漂移（案面引用不变、桶读数变），后续每轮全量门禁前，
  凡往 Temp 写了与案面已有反引号引用同名的 JSON 输入输出件（第一本账高频名还有带 out 扩展名
  的输出件、`config.yaml` 出厂模板、带 census_report 扩展名的报表一族），必须清残留或换探针
  目录名再跑。本条补记自身即按此纪律书写：凡名字会进 scratch 档的一律不套反引号，以免账本
  自己给自己造新引用（两条带行号引用均为 live 档 + 区间 live，已现量核验）。
- 全量偏支读数待阶段门禁落账。

### L121（2026-09-29）— A205 偏支逐支审计·第三批（`61d075bf3`）

- **权威清单重建**：L120 后剩余偏支以全量门禁 coverage.xml 的 missing-branches 重读，
  得 46 条（L116 旧清单已过时）。坑：XML 的 `class.filename` 不带包前缀，首读把
  `api/routes/export.py` 误并入 `augmentor/export.py`；拼上 `package.name` 才是 46 条
  真身。L121–L124 全部以这份 46 条为基准。
- `test_partial_branches_l121.py` 清 8 条可达 + 记 3 条不可达：
  - analytics 182->181（停用词夹在 bigram 对 ⇒ 该对跳过；无停用词对照组必出 bigram 钉判据）
  - analytics 192->190（纯空白文本句法切分无句 ⇒ `if sentences:` 假支）
  - analytics 266->276（150 条数据集落在 [100,1000) ⇒ 数据量洞察两档全不中）
  - analytics 422->418（重复候选 instruction 为空一方被挡；threshold=0.0 把「挡掉」变可观测）
  - config_validator 753->752（列表项环境变量**已设置**不告警 + 未设置对照）
  - config_validator 759->exit / 764->exit（`_validate_dict`/`_validate_list` 喂正确类型零错误）
  - sampler 279->282（六问题类型×长度三档全覆盖 ⇒ `underrepresented` 为空）
- 记档：analytics 349->353（入口 :325 空早退 ⇒ :349 `if self._items:` 恒真，冗余防御）、
  sampler 41->exit（`self._model` 只置 None、`_load_model` 从不回填 ⇒ 死守卫）、
  sampler 265->260（`identify_underrepresented` 产出口径封闭 {question_type,length} ⇒
  elif 恒真支）。
- 附带记档（死代码，无偏支弧）：config_validator `__init__` 登记的 `_validators`
  分发表（dict/list/str/int/float/bool → 六个 `_validate_*`）**没有任何走查调用点**，
  类型检查全在 `_validate_known_fields` 就地 isinstance ⇒ 六个方法只能被测试直调。
  留 L126 简化轮收编。

### L122（2026-09-29）— 第四批（`925610098`）

- `test_partial_branches_l122.py` 清 5 条可达 + 记 3 条不可达：
  - backup 219->223（备份文件外部缺失：跳 unlink、索引照清、仍成功）
  - backup 331->330（索引**重复 backup_id** 的真实损坏形态：同 id 两条目，第一条
    删光、第二条返回 False，`clean_old_backups` 计数只进一；走公共入口不碰私有方法）
  - version_control 245->244（`get_version` 找列表尾 / 找不存在 id 两种扫过形态）
  - version_control 382->384（同实例比较**不同版本对**：缓存键不同不进 :354 早退，
    `hasattr` 已存在 ⇒ 跳缓存初始化）
  - indexer 168->166（n-gram 字段混入 int 值被 `isinstance` 挡出索引、其余照索）
- 记档：rag 61->68（分块循环每轮 `start+chunk_size ≥ len+overlap > len` ⇒ :66 `break`
  恒先于 range 回边触发，回边不可达）、rag 63->65（range 的 start 恒 `< len(text)`，
  空切片不可达，防御式判据）、export 277->286（ExportFormat 六成员封闭枚举 + :240
  非原生委托早退 ⇒ 穷举 elif 链尾穿到 :286 不可达）。

### L123（2026-09-29）— 第五批·九条单偏支（`0c61ba3dd`）

- `test_partial_branches_l123.py` 清 5 条可达 + 记 4 条：
  - performance_benchmark 59->63（`HAS_MEMORY_MONITOR` 假支：翻开关跳过采样、原样返回）
  - preview 216->218（第二次预览必须**换数据**：缓存键是前 5 条样本的 md5、**不含
    格式**，同数据换格式走 :191 命中早退到不了 :216。附带产品缺陷记档：同数据换
    格式返回旧格式缓存结果，缓存键设计漏了 format 维，留后续轮次处置）
  - tracker 126->125（指标登记了但从未记值（空值列表）⇒ 不进 final_metrics）
  - logging_setup 133->135（handler 被外部从目标 root 摘走 ⇒ 撤装跳 remove、仍 close）
  - models/base 223->258（DCL 内层假支的**本意场景**：锁等待中的 worker 见到并发
    写者已置好的 `_session` ⇒ 返回预建会话；双线程确定性配方，不是记档项）
- 记档：report 145->138（`metrics` 写死三元组，走到 :145 时 metric 必为 diversity，
  elif 恒真支）、retry 197->237（最后一轮必走 :218 `break`，range 自然耗尽不可达）、
  cache 291->297（`total > _max_bytes` 前提保证 entries 非空，淘汰循环零轮回边不可达）、
  api/main 193->207（模块**导入期**环境分支：用例在子进程设/不设
  `AUGMENTOR_API_KEY` 各跑一遍 import 断言 stderr，不在测试进程 reload 生产模块
  （会替换全套 TestClient 持有的 app 对象）；**测量面该弧仍挂着**，属「子进程实证、
  主进程不可达」形态，记档不硬凑）。

### L124（2026-09-29）— 第六批·cli 家族 + faiss（`94f7f09a2`）

- `test_partial_branches_l124.py` 清 6 条可达 + 记 4 条：
  - cli data_ops 66->73（`validate` 不带 `--output`）、ops 118->exit（`dependency
    --action graph` 不带 `--output`）、profiling 35->37（`outliers --field score` 走
    「非 length 不挂长度字段」支）；三条都走真 argparse 流程（`from cli import main`；
    顶层 `cli.py` 是壳、真正分发表在 `augmentor.cli.commands`）
  - faiss 192->199（查询缓存命中后 `_ids` 被**绕过写接口**直接改 ⇒ 滤失效条目、
    整条作废重算；触发要点：缓存键是 `(digest, min(top_k, len(_ids)))`，top_k 必须
    小于删后条数才同键命中；faiss 缺失环境沿用测试树既有的伪 faiss 注入）
  - faiss 268->271（faiss 可用时删空全部向量 ⇒ 不往空索引 add）
  - faiss 334->337（load 零向量持久化 ⇒ 跳 `index.add`）
- 记档：cli version 98->exit / 142->exit（`run_version` / `run_backup` 的 action 由
  parser choices 封闭、全被 if/elif 覆盖 ⇒ 链尾假支不可达）、cli ops 125->exit
  （dependency action 同形）、cli quality 86->91（`_generate_recommendations` 零建议时
  兜底追加「质量良好」一行 ⇒ recommendations 恒非空）。

### L125（2026-09-29）— 阶段全量门禁 + A205 判定收官读数

- **全量门禁（`.coverage` 清空重跑，防 scoped 混数）**：`7373 passed / 3 skipped /
  exit 0`，语句+分支总覆盖 **99.87%**；分支 3808 valid / 3786 covered / **22 条
  missing**。较 L120 的 46 条：L121–L124 四轮以 24 条测试清零（8+5+5+6），余 22 条
  **全部已逐条记档**，A205「偏支逐支审计」在本批收官。
- 余 22 条分两类：
  - **可简化族（5 条弧 + 1 处死代码）**：validation 822->826 / 826->816
    （`_sanitize_item` 里恒真的 `if fix:` 冗余守卫，L115 起记档待删）、quality 222->230
    （`if sample_texts:` 恒真冗余）、analytics 349->353（`if self._items:` 恒真冗余）、
    sampler 41->exit（`self._model` 死守卫）、config_validator 死分发表 + 六个只能
    测试直调的 `_validate_*`（L121 记档）。L126 以「删守卫/死码 + 回归护栏」收掉
    前 5 条弧（删后对应假支消失、行为不变 ⇒ 全量 17 条 missing）。
  - **永久记档的结构性不可达（17 条）**：rag 61/63（break 抢跑/空切片族）、export
    277 + cli version 98/142 + cli ops 125（封闭词表链尾族）、cli quality 86（兜底
    恒非空族）、retry 197（break 抢跑族）、cache 291（前提保非空族）、report 145
    （写死三元组族）、api/main 193（导入期环境分支族，子进程实证）、model_manager
    23 + search_enhanced 221 + visualizer 46（DCL/懒加载防御族）、pipeline 369
    （并行失败侧恒 True 族）、sampler 265（封闭词表族）、validation 863->855
    （去重内层循环耗尽出边：value 在 seen 里 ⟺ 必在 result 里、内层循环必然
    break，耗尽支不可达——L115 起记档，属结构族而非可删守卫）。
- 附带发现的产品缺陷 2 处已记档（preview 缓存键漏 format 维、config_validator 死
  分发表），均不属偏支审计范围，留专项轮。
- 本批无前端改动；前端状态仍为 L113 阶段 13 文件 / 112 例绿。

### L126（2026-09-29）— 简化轮：删 5 条恒真/死守卫 + 1 处死分发表

- 产品码四处手术（全部「入口前提保证恒真」类，删除后行为不变，回归护栏 = 全量门禁）：
  - validation.py：`_sanitize_item` 两个恒真 `if fix:` 守卫（:794 只在 fix=True 支
    调用本方法）⇒ 偏支 822->826 / 826->816 消失；863->855 归结构族不动
  - quality.py：`if sample_texts:`（入口 :181 空早退 + sample_size 判据 ≥1 ⇒ 恒非空）
    ⇒ 222->230 消失
  - analytics.py：`if self._items:`（入口 :325 空早退 ⇒ 恒非空）⇒ 349->353 消失
  - sampler.py：`self._model` 死属性（唯一写入点 `__init__` 置 None、`_load_model`
    从不回填）连同 `_load_model` 的恒真守卫一并删 ⇒ 41->exit 消失
  - config_validator.py：`__init__` 的 `_validators` 死分发表（六个 `_validate_*`
    方法零调用点、类型检查全在 `_validate_known_fields` 就地 isinstance）连表带方法
    整体删
- 测试侧随行清理（不删行为、只删打已删对象的脸）：
  - test_config_validator.py 的 `TestConfigValidatorEdgeCases` 二十条里十六条直调六个
    已删方法 ⇒ 删十六条，留四条打公共入口的（`test_error_to_dict` 等）
  - test_partial_branches_l121.py 的 `TestConfigValidatorTypeGuardsPositive` 两条随方法
    删除而失效 ⇒ 留空壳类 + docstring 历史注记（759/764 两条弧线随方法消失）
  - test_sampler.py 两条打 `_model` 死属性的用例改以 `_use_sklearn` 为判据（重复
    调用状态一致性）、降级通道改打「降级仍可用」的真实行为
  - test_micro_branches_l58.py 两条直调 `_validate_int/_validate_float` 的范围用例
    改走公共入口 `validate_config` 的内联 isinstance 路径（KNOWN_FIELDS 规格 max
    与 config 常量同源，判据不变、入口更真）
  - 账本 A92 行号引用因 validation.py 删行平移（:637 空行 → :639 现量落点）⇒
    L79 漂移桶 161 → 160（棘轮只降不升，MEASURED 留一句「谁清的」）
- 验收预期：全量偏支 22 → **17**（5 条可简化弧清零，余 17 条全为永久记档结构族）；
  实际读数以下一轮全量门禁落账。

### L127（2026-09-29）— 缺陷修复轮：preview 缓存键补全三因子（L123 记档缺陷闭环）

- **缺陷**（L123 附带发现）：`PreviewGenerator.preview` 的缓存键只哈希前 5 条样本
  数据（`md5(str(sample[:5]))`），**不含格式与条数** ⇒ 同数据换格式 / 换预览条数
  会命中旧缓存：`preview(items, "jsonl")` 之后 `preview(items, "csv")` 直接返回
  jsonl 的缓存对象（`format_info["format"]` 还是 jsonl）；`preview_size=5` 与
  `preview_size=8`（前 5 条相同）同理撞键，8 条请求拿到 5 条预览。
- **修法**（一行键改）：键 = `md5(f"{fmt}|{size}|{前5条样本}")` —— 同一次预览的
  三个决定因子全进键，缺一即换键。命中支的「总数信息刷新」行为原样保留。
- **回归**（`test_preview_cache_key_l127.py` 3 例，两格在旧键下必红）：
  换格式不串味（`format_info["format"]` 各自正确 + 两次结果不是同一缓存对象）、
  换条数不串味（同前 5 条、size 5 vs 8 ⇒ 后者 `converted_data` 必须 8 条）、
  同三因子仍命中（优化没被修掉：命中支照走、总数照刷、缓存字典只长 1 格）。
- 全量门禁：`7358 passed / 3 skipped / exit 0`（较 L126 的 7355 + 3 新例，无一回归）。
- L123 记档的两处产品缺陷至此已闭环 1 处（preview 缓存键）；另一处
  （config_validator 死分发表）已在 L126 删掉。

### L128（2026-09-29）— A207 构建产物体检收口（web/scripts/check-dist.mjs）

- A207 的「bundle 拆分」本体（路由级懒加载 + antd/echarts/react 三 vendor
  manualChunks）此前已就位，本轮回**产物体检**：新增 `web/scripts/check-dist.mjs`
  六格判据 + `npm run build:check` 入口（先构建再体检，红即 exit 1）：
  1. 入口壳 ≤50 kB（实测 6.8 kB，壳里全是懒加载路由）
  2. 11 个页面 chunk 齐全（懒加载没被摇掉）
  3. 页面 chunk 均 ≤100 kB（vendor 没漏进业务块；实测最大 9.2 kB）
  4. 首屏 modulepreload 名单无 echarts（实测仅 react-vendor + antd-vendor 两项；
     echarts-vendor 1.15 MB 只由 Analysis 路由动态 import 拉取）
  5. 大 vendor 只降不升基线：antd ≤1200 / echarts ≤1500 / react ≤300 kB
     （现状 945 / 1145 / 180 kB；构建期 800 kB 告警阈值对这两个 vendor 是常态
     噪声，判据收在体检脚本里不靠告警）
  6. index.html 引用的产物全部在场（防半截提交）
- 红能力实证：往入口 js 注 60 kB 后体检红在第一格（「壳被业务逻辑污染」）、
  exit 1，重建即复原。
- 前端门禁维持 vitest + eslint + tsc 三件套不动（体检是 build 后置检查，
  不进 CI 常规链，`npm run build:check` 一轮一跑）。

### L129（2026-09-29）— A208 收口：API 易混字段词典 + API 面排查 FAQ

- **A208 = 「FAQ + 端点人读字段说明」**。既有 `docs/API.md` 第 1~7 节已逐端点给出
  响应示例，缺的是两块：跨端点复现字段的**人读语义**、以及 REST 面特有的**排查**。
- 新增 `docs/API.md` **易混字段词典**（9 行表，插在「响应契约」后）：只收字面直觉
  与实际语义不符的字段，逐条标注陷阱，字段名以 `api/routes/*.py` 的 `*Response`
  schema 为准（读源核过、未手抄）——`pass_rate` vs `avg_score`（门禁读前者）、
  `effective_weights` 重归一化、`semantic_evaluated` API 面恒 false、`duplicate_groups`
  是组数非明细、`removed_count`/`dropped_count` 不同名是既有契约、`is_valid` 空集
  恒真（A92）、`overall_passed` 默认不判负（A91）、`metrics`/`all_metrics` 之别。
- 新增 `docs/API.md` **§11 排查（API 面）**（6 问）：路径白名单 400、写入不自动建
  目录、空集 is_valid=true、报告端点如何让 CI 变红、并发 progress 串号、
  X-Process-Time 单位；只收 REST 面特有坑，通用依赖问题指回 README §9。
- **A184 漂移连带修**：易混字段词典插在 API.md 中部（+约 18 行），把账本 L49 三处
  历史行号引用（API.md 加冒号 849 那种写法）顶到空行（dead_line 0→3）。按 A153/A127
  纪律«历史行号不是对今天文件的断言» 降成裸文件锚点 `docs/API.md`（file_tokens 侧仍
  live，行号不再随 API.md 增删漂移）⇒ 三档硬 0 复零。**本条自己也踩了同一坑**：初稿把
  那个「加冒号行号」的写法照抄成反引号引用，本账本身在扫描面里 ⇒ 记账行当场变成新
  dead_line（A153 记录过的形状第 N 次复现）；改成裸文本描述、不成形为可解析引用即消。
- 全量门禁：`7358 passed / 3 skipped / exit 0`（纯文档轮，与 L127 同数无回归）；
  A184 + L79 + markdown 结构三守卫 115/115 绿。
- **Backlog A 全清**：A201–A208 八项全部关闭（A205 偏支审计 L114–L126、A206 前端
  L104–L113、A207 产物体检 L128、A208 文档 L129）。

### L130（2026-09-30）— 回归修复：L128 新增脚本触发前端 lint 门禁转红

- **自查抓到的本轮自造回归**：L128 新增的 `web/scripts/check-dist.mjs` 用了 Node 全局
  `console` / `process`，而 `eslint.config.js` 只给 `*.config.{js,ts}` 与
  `eslint.config.js` 配了 Node globals，`scripts/**` 落进默认（浏览器）环境 ⇒
  `npm run lint`（`--max-warnings 0`）报 **8 处 `no-undef`**、前端 lint 门禁转红。
  L128 当时只跑了 `build:check`（脚本能跑），漏跑 lint ⇒ 门禁红了一轮才被本轮
  `npm run lint` 抓到。
- **修法**：`eslint.config.js` 加一条 `scripts/**/*.{js,mjs,cjs}` 的 Node-env override
  （与既有 `*.config` 那条同形）。lint 复零。
- **前端三件套复验全绿**：lint exit 0 / tsc exit 0 / vitest **13 文件 112 例**通过。
- **纪律补记**：新增任何被 `eslint .` 扫到的文件（不止 `src/`），当轮必须连
  `npm run lint` 一起复验，不能只验「脚本能跑」——lint 环境判定与运行时是两回事。

### L131（2026-09-30）— B201 立项 + 关闭：get_duplicate_candidates O(n²) 热点预计算

- **性能热点普查**（贴近真实的房产客服语料，`Temp/l131q`）：`DatasetAnalyzer.analyze()`
  在 6000 条上 61.5 ms（text_statistics 36.5 / quality 19.3 / diversity 6.1，都健康），
  但 `get_duplicate_candidates()` 单独 **71.8 s / 244225 候选** —— 数量级离群，是本轮热点。
- **根因**：全对 Jaccard 是 O(n²) 无法回避，但改前把 `set(inst_i)` / `set(inst_j)` 写在
  **内层循环里逐对重建** ⇒ str→set 构造被跑了 ~n² 次（长文本上这是主成本），`dict.get`
  也在内层跑 n² 次。这是本仓 SDK 公共方法（`augmentor/__init__.py` 导出 `DatasetAnalyzer`）。
- **修法**（行为完全不变，只把构造提到循环外）：每条 instruction 的字符集**预计算一次**
  （空 instruction 记 `None`、与改前 `if inst_i and inst_j` 同判跳过），内层只做集合运算。
  n² 次 set 构造降到 n 次；相似度公式、`>=` 阈值、配对顺序、降序排序一字未动。
- **A/B（同进程双序 min-of-2，2500 条）**：old 13727 / 14067 ms，new 4957 / 4825 ms ⇒
  **new/old ×0.361 / ×0.343**（约 2.8× 快）。等价性：1200 条 × 5 阈值（0.3/0.5/0.8/0.9/1.0）
  **逐元素一致**，含空 instruction 边界与相似度平局。
- **回归护栏**：`test_analytics.py::TestGetDuplicateCandidates::test_matches_reference_semantics_l131`
  ——内嵌一个「朴素双循环 + 逐对重建 set」的参照实现，6 阈值逐元素比对 + 断言候选下标
  不牵扯空 instruction 条目。改写只要动了「空跳过 / Jaccard / 降序」任一就红。
- 全量门禁：`7359 passed / 3 skipped / exit 0`（L129 的 7358 + 1 新守卫，无回归）；
  analytics 三测件 75/75、A184 21/21 绿。**B201 关闭**。

### L132（2026-09-30）— B202 立项 + 关闭：_diversity_scores 词元集合预计算

- **热点普查**（延续 B201 的全对相似度族）：`ActiveLearningLoop._diversity_scores`
  （`strategy="diversity"/"hybrid"` 主动学习选样的打分核）对 n 条文本两两求词元级
  Jaccard，内层**逐对调用 `compute_similarity(text, other)`**。该函数每次都
  `set(tokenize(generated))` + `set(tokenize(reference))`，即每对切两次词，n 条共约
  2n² 次 tokenize —— 相似度对称、每行只取 max，切词被白算了一个数量级。
- **修法**（行为完全不变，只把切词提到循环外）：先一次性算好每条的词元集合
  `token_sets = [set(tokenize(t)) for t in texts]`（n 次 tokenize），内层只做
  `len(a & b) / len(a | b)`；空集合仍返 0.0（与 `compute_similarity` 的
  `if not gen_tokens or not ref_tokens: return 0.0` 同判）。`compute_similarity`
  导入随之移除（不再有调用点），改导 `tokenize`。
- **A/B（同进程双序 min-of-3，n=800）**：new 708 ms、old 8033 ms ⇒ **new/old ×0.088**
  （≈11× 快）。等价性：4 数据集（含重复条目 / 空串 / 单条 / 多词）与「逐对
  compute_similarity」参照实现**逐元素一致**（脚本内 `assert abs<1e-9` 全过）。
- **回归护栏**：`test_active_learning.py::TestDiversityScoresReference::`
  `test_matches_pairwise_compute_similarity_l132` —— 内嵌逐对 `compute_similarity`
  参照，在含重复/空串/单条/多词的 4 数据集上逐元素对照；任何切词或 Jaccard 语义
  漂移当场红。
- 全量门禁：`7360 passed / 3 skipped / exit 0`（L131 的 7359 + 1 新守卫，无回归）；
  active_learning 49/49、A184 21/21 绿。**B202 关闭**。

### L133（2026-09-30）— B203 立项 + 关闭：normalize_punctuation 优化证否 + 补首个行为守卫

- **候选**（延续标点/字符串处理族）：`DataCleaner._normalize_punctuation` 对每个字段跑
  **14 遍顺序 `str.replace`**，直觉上「合成一次 `str.translate`」更快。
- **A/B 证否（诚实记录，不作提速主张）**：str.translate 逐字符查表、无命中不短路；
  `str.replace` 在子串不存在时 C 层几乎瞬返。实测三种负载 —
  - 50k × 80char 混排：new/old **×1.05**（基本持平）
  - 50k × 80char 重匹配（几乎全是全角标点）：×0.87（translate 略快）
  - 2k × 4000char 稀疏匹配（长文本、零星全角，**最贴近真实 instruction/output**）：
    **×8.74（translate 大幅变慢）**
  → 真实数据是「长文本 + 少量全角」，translate 是**净回归**。**保留原 14-replace 实现**，
  不改产品代码（符合「不为风格改动能工作的代码」）。
- **真实收获**：该规则此前**无任何直达行为用例**（仅 TextNormalizer 的叠字合并被测）。
  补 `test_cleaner.py::TestNormalizePunctuation` 三条：① 映射表 14 项逐项断言半角落点 +
  `modified_count==1`；② 与「顺序 replace」参照实现在混排/纯全角/无全角/空串/中英混排
  5 样本上逐字符等价；③ 无全角时不误报修改。参照实现＝原实现语义，故对现产品代码有效。
- 全量门禁：`7363 passed / 3 skipped / exit 0`（L132 的 7360 + 3 新守卫，无回归）；
  cleaner 40/40、A184 21/21 绿。**B203 关闭**（产品未改，账本如实记优化证否）。

### L134（2026-09-30）— B204 立项 + 关闭：generate_multi_turn 已有问题集合提环 + 并集语义守卫

- **缺口**：`ContextAugmentor.generate_multi_turn` 的去重写在每轮循环里——对**每条**
  `existing_histories` 各重建一遍 `[t["content"] for t in existing if t["role"]=="user"]`
  再 `in` 线性查，而 `existing_histories` 在整个循环内是常量，等于把循环不变量反复重算
  约 `num_turns × len(existing_histories)` 次；且「跨多条历史」的并集语义此前没有被任何
  用例钉死（既有用例的 `existing_histories` 都只有 1 条历史，重复项恰好落在唯一那条）。
- **修法**（语义等价、结构收紧）：循环前一次性把全部已有 user 问题摊平成单一
  `existing_questions` 集合（并集语义），内层改成 `if follow_up in existing_questions:
  continue`。旧写法「按条重建列表再 in」在**语义上等价**（每条历史都扫到），但把不变量
  算在循环内、且嵌套两层更难读；新写法天然按并集判、O(1) 命中，缺 `role`/`content`
  键的错误语义与原实现保持一致。
- **回归护栏**（锁死等价性，非修 bug）：`test_context.py::TestGenerateMultiTurn` 新增
  2 条 —— ① `test_duplicate_across_multiple_existing_histories`：两条历史（第二条混入
  3 条 user/assistant 轮次），follow_up 只落在**第二条**历史即须命中；②
  `test_cross_history_partial_accept`：部分命中不误伤、非命中项正常采纳。改写只要把
  「摊平到单一集合」退回逐条重建就仍绿（等价），但一旦误改成「只看首条历史」这类
  并集外的判据，①当场红——正是把「并集」这层语义显式冻进用例。
- **A/B 不作提速主张**（真实 `existing_histories` 通常很小，循环不变量重算不构成热点）；
  本轮价值在语义收紧 + 简化，非计时。
- 全量门禁：`7365 passed / 3 skipped / exit 0`（L133 的 7363 + 2 新守卫，无回归）；
  context 30/30、A184 21/21 绿。**B204 关闭**。

### L135（2026-09-30）— B205 立项 + 关闭：remove_duplicates keep="last" O(n×D) → O(n) 槽位覆盖

- **缺口**：`DataSanitizer.remove_duplicates(keep="last")` 每次撞上重复值，就在全
  `result` 上逐条 `r.get(key,"")==value` 线性找槽位再 `result[i]=item`，重复多的数据上
  是 O(n × distinct) 的显式热点（distinct 越大、重复越密集越糟）；`seen` 明明已经按值
  建了索引，却把 items 下标存进去、再回头全表扫，索引白建了。
- **修法**（语义等价、复杂度收紧）：`seen[value]` 改存 `len(result)`（首次出现时记录的
  result 槽位），重复时直接 `result[seen[value]] = item`，O(1) 覆盖。保留「保留最后一次
  的内容 + 落在首次出现位置 + 不动其它记录」三点语义不变。
- **A/B（同进程双序 min-of-2，N=40000 / D=400 dup 密集）**：old 289 ms、new 6.8 ms ⇒
  **new/old ×0.0235（≈42× 快）**。等价性：300 组随机 dup 密集集（D=15）逐元素一致 +
  50 组 keep=first 回归集不变。
- **回归护栏**：`test_validation.py` 既有的 L114 用例（重复值居第二、需跳过第 0 条）
  改述为「O(1) 槽位不变量」——语义本身（更新到新内容、落回原槽、不动其它）仍是它守的，
  但旧 docstring 里「替换循环假支 864->863」已随循环消失而失效，改为直接钉三点语义；
  另新增 1 条「三次以上同值重复」的槽位不变量用例（内容取最后一次 3，只写一次进槽）。
- 全量门禁：`7366 passed / 3 skipped / exit 0`（L134 的 7365 + 1 新守卫，无回归）；
  validation 63/63、A184 21/21 绿。**B205 关闭**。

### L136（2026-09-30）— B206 立项 + 关闭：_compare_fields O(F×A×B) 匹配 → 双层索引 + 补行为守卫

- **缺口**：`EnhancedComparator._compare_fields`（`compare()`/`compare_datasets_enhanced`
  公共路径）对每条字段、每条 A 项都**全扫一遍 B**找「同 instruction」记录，O(F×A×B)；
  另外每个字段各扫一遍 A、B 算 `count`/`count_b`，又叠一层 O(F×(A+B))。同时
  `type_mismatches` / `value_differences` / `diff_datasets` 成员语义此前**无任何直达行为
  用例**——既有用例只 `assert "field_comparisons" in d`，全 O(F×A×B) 匹配逻辑实际是盲跑。
- **修法**（行为等价、匹配降一档）：
  - 一次 O(B×F) 建 `first_b_by_field[instruction][field] = 首个含该 field 的 item_b`
    两层索引（旧内层是「取同 instruction 且**首个含该 field** 的 item_b 就 break」，
    不能退成「取该 instruction 的整体首条」，须按字段维度记 first）。
  - 一次 O(A+B) 累加 `field_count_a` / `field_count_b`，替代每字段各扫一遍的 O(F×(A+B))。
  - 内层改成 O(1) 查表。`in_*` / `type_mismatches` / `value_differences` 计算一字未改。
- **A/B（同进程双序 min-of-2，N=1500、F≈12）**：old 127.5 ms、new 12.4 ms ⇒
  **new/old ×0.097（≈10× 快）**。等价性：200 组随机数据集（含重复 instruction、缺 field、
  int/str 混型、空集）逐字段逐差异与旧 O(F×A×B) 参照实现全量一致。
- **回归护栏**（先补后改）：`test_compare_enhanced.py::TestFieldComparisonBehavioral`
  3 条 —— ① 共享 instruction、output 值不同 ⇒ 精确产生 1 条 value_difference；
  ② 同字段 int vs str ⇒ `type_mismatches==1`；③ 部分重叠数据集 `diff_datasets` 的
  `only_in_a`/`only_in_b`/`in_both` 成员精确（不是只查 key 存在）。这三条把「匹配逻辑
  真正在算」钉死，任何索引化只要把「取哪个 item_b」搞错就当场红。
- 全量门禁：`7369 passed / 3 skipped / exit 0`（L135 的 7366 + 3 新守卫，无回归）；
  compare_enhanced 19/19、A184 21/21 绿。**B206 关闭**。

### L137（2026-09-30）— B207 立项 + 关闭：_process_single_item 惰性 _process_errors 竞态 → 持锁

- **并发竞态专项**（普查：`grep ThreadPoolExecutor` 命中 pipeline/context/export/
  expander/multilingual；逐个核谁在共享状态上竞写，pipeline 是唯一「多工作线程写同一
  惰性 list」的真竞态）：`_process_single_item` 由 `augment_dataset` 的 `ThreadPoolExecutor`
  多工作线程并发调用，其 except 分支对共享的 `self._process_errors` 走「`hasattr` 没有就
  建 `[]` 再 `append`」两步非原子操作——两个线程都可能读到「还没有这个属性」，各自建一份
  `[]`，后建者把先 append 的错误记录整份覆盖丢一条。
- **dead lock 佐证**：`__init__` 里 `self._lock = threading.Lock()`（:58）建了之后**全程
  无人 `with self._lock`**（`grep "with self._lock" pipeline.py` 改前为 0 命中）——锁是
  白建的，惰性 list 恰好没被它保护，竞态就裸露着。本轮把 check-then-act 包进这把锁，
  顺带把 dead lock 转成活锁。
- **修法**（行为不变，只补并发安全）：`error_info` 构造仍在锁外（无共享），锁只包
  「`hasattr`→建→append」三步；串行路径（单次调用）行为逐字节等价。
- **回归护栏**：`test_pipeline_error_recovery.py::TestProcessErrorsThreadSafety` 2 条 ——
  ① `test_concurrent_error_records_not_lost`：40 个真实线程同时走 except 分支（注入
  RuntimeError），断言 `_process_errors` 恰好 N 条、index 集合完整（无丢失/无重复）；
  ② `test_error_record_mutation_is_guarded_by_lock`：源码级钉死「record 分支必须持
  `self._lock`」，防止未来有人把锁拿掉、却因 CPython GIL 窗口窄到行为测试测不出丢失而
  静默退化。
- 全量门禁：`7371 passed / 3 skipped / exit 0`（L136 的 7369 + 2 新守卫，无回归）；
  pipeline 12/12、A184 21/21 绿。**B207 关闭**。

### L138（2026-09-30）— B208 立项 + 关闭：generate_report 下标取法 id 化证否 + 补下标精确性守卫

- **候选**（延续「一次建表替代 per-item 线性扫」族）：`ActiveSampler.generate_report`
  里 `recommended_seed_indices = [items.index(seed) for seed in ... if seed in items]`
  用 `==` 相等取「seed 在 items 里的下标」；`items.index` 是 **first-equal** 语义——若
  items 里有两个内容相等但对象不同的 dict，会把「第一个」的下标当成「被选中那个」的下标
  报出来。直觉上改成 `id(seed)` 一次建表 O(n)+O(1) 查更快更准。
- **A/B 证否（诚实记录，不作提速主张）**：真实 `top_k` 很小（默认 10，实测多数场景
  连 1 条 seed 都难触发），per-seed 的 `in`+`.index` 合计 2×O(n) 在 k 很小时**比**
  建一遍 O(n) id-map 更便宜。实测 n=3000、50 seeds：new(id-map)/old **×3.56（更慢）**。
  即 O(n) 建表的固定成本在 k 小时吃掉了 per-seed 扫描的便宜。
- **语义改进也不成立**：`recommend_seeds` 对每个 underrepresented 类型都是「遇到第一个
  匹配就 `break`」——所以选中的 seed 在 items 里**永远是该内容的首个对象**，first-equal
  与 exact-object 两个下标实际**恒等**。id 化的「精确到对象」语义增益在真实选择逻辑下是
  空的（只有人为构造「先被跳过、后被选中的同内容对象」才分得出，但 recommend_seeds 永远
  不会那样选）。
- **结论**：不改产品代码（保留 `items.index(seed)`，符合「不为无意义风格改能工作的代码」）。
  真实收获是补一条守卫把「下标必须对应被选中对象本身」这层契约显式钉进用例——
  `test_sampler.py::test_seed_index_uses_exact_object_not_first_equal_l138`，用
  `it is seed` 取 expected，未来若有人把 `recommended_seeds` 换成新构造的同内容 dict 再
  查下标、或改用 first-equal 的宽松实现，这条当场红。
- 全量门禁：`7372 passed / 3 skipped / exit 0`（L137 的 7371 + 1 新守卫，无回归）；
  sampler 58/58、A184 21/21 绿。**B208 关闭**（产品未改，账本如实记优化证否）。

### L139（2026-09-30）— B209 立项 + 关闭：_generate_recommendations 字符串分支补断言

- **缺口**（纯测试补齐，无产品改动）：`_generate_recommendations` 有 4 条字符串分支
  （`size_diff>100` / `similarity<0.5` / 字段类型不匹配 / 字段值差异），此前**无任何直达
  断言**——既有 `test_diff_datasets`/`test_to_dict` 只查 `"recommendations" in d`（key 存在），
  分支里的具体文案、计数插值全是盲跑。哪条阈值/文案改错都不会红。
- **修法**：补 `test_compare_enhanced.py::TestRecommendationStrings` 3 条，直接构造
  `ComparisonMetrics`/`FieldComparison` 调 `_generate_recommendations`，逐字钉死：
  ① `size_diff=150` ⇒ 出现「数据集A比B多 150 条数据」；
  ② `similarity=0.1` ⇒ 出现「相似度较低」；
  ③ `type_mismatches=2` + `value_differences` 1 条 ⇒ 各出现「字段 'output' 存在 2 个类型
     不匹配」「字段 'output' 存在 1 个值差异」（顺带覆盖「高度相似」分支的相邻文案）。
  三条都是「具体文案 + 具体计数」，任一分支改文案/改阈值/漏 append 就当场红。
- 全量门禁：`7375 passed / 3 skipped / exit 0`（L138 的 7372 + 3 新守卫，无回归）；
  compare_enhanced 22/22、A184 21/21 绿。**B209 关闭**。

### L140（2026-09-30）— B210 立项 + 关闭：turns_to_canonical 裸 KeyError 违反 Raises 契约 → DataFormatError

- **真缺陷（契约违反，非性能）**：`turns_to_canonical`（chatml/vicuna/sharegpt 的逆运算，
  把多轮对话折回 instruction/output）docstring 明写 `Raises: DataFormatError: 对话不足两轮，
  或末尾不是「用户 → 助手」`。但末尾两轮角色检查 `turns[-2]["role"]` / `turns[-1]["role"]`
  用**裸下标**：任一末轮缺 `role` 键时抛裸 `KeyError: 'role'`，违反契约承诺的
  `DataFormatError`（调用方按契约捕获 `DataFormatError` 做降级/重试，会被 KeyError 打穿）。
- **修法**（行为最小化、契约归位）：`["role"]` → `.get("role")`（缺键 ⇒ `None` ≠
  `"user"`/`"assistant"` ⇒ 落入既有的 `DataFormatError` 分支，报错文案也同步把裸引用改
  `.get` 以防文案里再 KeyError）。正常「user→assistant」结尾路径逐字节等价（`"user".get`
  语义不变）。
- **缺测试佐证**：该逆运算此前**无直达单测**（只在 `test_cli_dataset_tools.py` 集成用例里
  被完整 pipeline 间接触发，且都喂的是规整数据）。补 `test_converter.py::
  TestTurnsToCanonicalContract` 2 条 —— ① 缺 role 键 ⇒ `DataFormatError`（改前 `KeyError`
  红，实证可复现）；② 角色顺序错（user→user）⇒ `DataFormatError`。
- 全量门禁：`7377 passed / 3 skipped / exit 0`（L139 的 7375 + 2 新守卫，无回归）；
  converter 181/181、A184 21/21 绿。**B210 关闭**。

### L141（2026-09-30）— B211 立项 + 关闭：VersionManager.diff 死代码 modified → 真识别原地修改

- **真缺陷（死分支 + 恒 0）**：`VersionManager.diff()` 用 `(instruction, output)` **组合键**
  做记录身份——`item_to_key` 把 output 也算进 key。结果「原地改了某条 instruction 的
  output」在新旧两版里各是一个**不同的组合键**，于是被 diff 拆成「删旧 (instr, out1)」+
  「增新 (instr, out2)」两条，而 `modified` 列表**结构上永远空**（`modified_count` 恒 0）。
  既有的 `test_diff_modified_items` 只 `assert diff.modified_count >= 0`（空转断言，永远真），
  缺陷被 100% 盖住。`DiffResult` 字段注释（`modified_count  # 修改数量`）与 docs/API.md:952
  示例（`"modified_count": 8`）都承诺非零，实现从不兑现。
- **修法**（身份键降为 instruction）：记录身份只按 `instruction` 匹配（SFT 记录的自然主键）；
  同一 instruction 两版都在时，整条 dict 不等即计入 `modified`。原地修改现在报
  `modified=1 / added=0 / removed=0`；纯新增/纯移除语义不变。
- **等价性核查**：既有 4 条 diff 用例（`added`/`removed`/`identical`/`empty`）全部继续绿，
  说明新键在「无原地修改」场景与原组合键结果一致，只在「同 instruction 改 output」这一
  此前误报的场景纠正了归属。
- **回归护栏**：强化 `test_diff_modified_items` 断言（`>= 0` → `== 1`），新增
  `test_diff_in_place_edit_is_modified_not_added_removed`（原地改 output + 混合一条不动项，
  钉死 modified=1 且 added/removed 都 0）。改前该新用例会红（modified=0、added/removed 各 1）。
- 全量门禁：`7378 passed / 3 skipped / exit 0`（L140 的 7377 + 1 新守卫，无回归）；
  versioning 59/59、A184 21/21 绿。**B211 关闭**。

### L142（2026-09-30）— B212 立项 + 关闭：compare_trends 平局/缺指标误报 dataset_b_higher → 三态 tie

- **真缺陷（错误结论，非风格）**：`QualityTrendTracker.compare_trends` 的 `comparison`
  用「A > B else B_higher」二态判据——**平局**（最新值相等）与「某侧根本没记录过该指标」
  都被误报成 `dataset_b_higher`（缺指标被静默当 0 参与比较，语义错）。既有唯一用例
  `test_compare_trends` 只钉了 A>B 一条，平局/缺指标路径全盲跑。
- **修法**（结论收紧为三态）：双侧都有最新值时 A>B ⇒ `dataset_a_higher`、B>A ⇒
  `dataset_b_higher`；其余（含平局、任一侧无记录）⇒ `tie`。严格比大小，只有 `>` 才判
  higher。
- **回归护栏**：`test_quality_trend.py::TestTrendComparison` 补 3 条 —— ① 相等 ⇒ `tie`；
  ② 某指标两侧都从未记录 ⇒ `tie`（且 `dataset_*_latest` 均为 None）；③ 严格大于才判
  a_higher、对称地 b 更大才判 b_higher。改前 ①②会红（误报 b_higher），③会绿。
- 全量门禁：`7381 passed / 3 skipped / exit 0`（L141 的 7378 + 3 新守卫，无回归）；
  quality_trend 13/13、A184 21/21 绿。**B212 关闭**。

### L143（2026-09-30）— B213 立项 + 关闭：identify_underrepresented 误把 avg_length 当占比桶

- **真缺陷（伪类型污染）**：`ActiveSampler.identify_underrepresented` 用
  `.items()` 遍历 `length_distribution`，但 `_analyze_length_distribution` 除 short/medium/long
  三个占比桶外还塞了一个 `avg_length` **均值**键（绝对值，非占比）。`isinstance(ratio, float)`
  判不了它是均值——当数据集平均指令长度 < threshold（默认 0.1，即几乎全空指令）时，会误把
  伪桶 `length:avg_length` 列进 underrepresented，进而：① 污染 `recommend_seeds` 的建议列表；
  ② 白占一个 `top_k` 名额（`underrepresented[:top_k]`）；③ `recommend_seeds` 的匹配循环里
  只有 short/medium/long 三个分支，`avg_length` 永远打不中，浪费一整轮迭代。
- **修法**：长度桶显式只遍历 `("short","medium","long")`，`avg_length` 不再进候选。
- **等价性**（证明只少报 avg_length、不动真实桶）：200 组随机数据集（含全空/混长短）
  new ⊆ old 且 new 恒不含 `length:avg_length`，全过。
- **回归护栏**：`test_sampler.py::test_never_flags_avg_length_as_length_bucket` —— 40 条全空
  指令（avg_length=0 < 0.1），断言 underrep 不含 `length:avg_length` 且 length 前缀只能是
  short/medium/long。**改前该用例红**（实测 1 failed）、**改后绿**（sampler 59/59）。
- 全量门禁：`7382 passed / 3 skipped / exit 0`（L142 的 7381 + 1 新守卫，无回归）；
  sampler 59/59、A184 21/21 绿。**B213 关闭**。


### L144（2026-09-30）— B214 立项 + 关闭：merge() 的 max_items=0 被 falsy 判据读成「不限」

- **真缺陷（falsy 假零）**：`DatasetOperations.merge()` 截断分支写的是
  `if config.max_items and len(merged) > config.max_items:`——Python falsy 语义下
  `max_items=0` 直接短路，**「一条不留」被当成「不限条数」而返回全量**。
  单文件截断点只此一处：`merge_files` 读文件后委托 `self.merge(datasets, config)`，
  故显式传 `max_items=0` 的 CLI/HTTP 调用同样拿回全量。
- **修法**：判据改 `if config.max_items is not None and len(merged) > config.max_items:`，
  并加注释钉住「`is not None` 而非 falsy」的回归防护（防止后人改回 `if config.max_items`）。
- **回归护栏**：`test_dataset_ops.py::test_merge_max_items_zero_keeps_none`——
  `max_items=0` ⇒ 0 条；同用例内对照 `max_items=None` ⇒ 全量（防修过头把未设置也截成 0）。
  既有 `test_merge_max_items`（=5）与 `test_merge_max_items_truncates`（=3）保持全绿。
- 已随 `cf35b5984` 入库（product+test）；本轮回填 B214 行 + 本条目。全量门禁复核先后受阻
  两次：v1 双门禁并发时 `.coverage` 落进残留守卫误伤（环境问题）；v2 红在 A184 棘轮
  `ambiguous`（336 vs 钉值 327）——根因是套件自写的运行时快照目录 `tests/.backups`
  进了 `full_index()`，L145 修复。L145 修复后全量门禁绿，本条「无回归」结论由该绿门禁佐证。
  **B214 关闭**。
- **遗留（已由 L148 处理）**：`merge_files` 的 `removed_duplicates` 混报问题由 L148
  （B218）以「merge_files 局部净增 8 行、避开 242/243 历史行引用区」的方案关闭。


### L145（2026-10-01）— B215 立项 + 关闭：A184 普查索引收进套件运行时快照目录，全量门禁自伤

- **真缺陷（门禁状态依赖）**：`full_index()`（A184 引用索引）的跳过集只有
  .git/__pycache__/.pytest_cache/node_modules；但测试套件**自己写的**运行时快照目录
  tests/.backups/（gitignored，门禁的 fixture 运行期写入 index.json、snap.json、
  opt100_progress.md）未被排除。实测时序：门禁第一轮绿 → fixture 写入快照目录 →
  下一轮门禁普查里 index.json、snap.json 等裸文件名多候选，9 条引用被翻
  live→ambiguous（336 vs 钉值 327），`test_bucket_matches_its_ceiling`[ambiguous] 红。
  删掉快照目录门禁恢复绿、下轮又红——状态依赖、自伤。
- **修法**：索引跳过集加入 `.backups`（语料面跳过集一直排除了它，本次为两面对齐；
  L79 的 `build_index` 只索引 .py、快照目录当前无 .py，不动，记档即可）。
- **账本重钉（逐格归因，按「动案面要重跑普查再改这里」条款）**：排除快照目录候选后
  13 条「只能靠快照目录落定」的引用翻进 scratch_missing（289→302）：裸名两条、
  全路径两条、余为守卫文件自身引用；ambiguous 327、runtime_ns 30、scratch 729 与
  三格硬 0 原地未动。
- **幂等性证据**：普查「含快照目录 / 不含快照目录」两跑，读数逐格相同
  （327/30/729/302，硬 0 三格为 0，引用共 3881 条）——全量门禁不再依赖运行时工件态。
- **守卫**：`test_runtime_backup_dir_is_invisible_to_the_index`——断言索引里无任何
  含快照目录段的路径（改前红：3 条泄漏；改后绿）。
- 全量门禁：`7384 passed / 3 skipped / exit 0`（原套件 7383 + 1 新守卫，无回归；
  首跑 1 红为性能测时用例在负载下的 flake——单跑 4/4 两次全绿、复跑全量绿，
  非产品回归）。**B215 关闭**。


### L146（2026-10-01）— B216 立项 + 关闭：quality_trend 损坏文件被静默截断 = 历史数据永久丢失

- **真缺陷（静默数据丢失）**：趋势历史文件加载失败（JSON 损坏、`trends` 不是列表）时，
  加载器只打一条告警就把历史置空；随后任何一次记录指标触发的保存都会以写模式打开
  原文件、**截断**成「仅含这条新条目」——用户对 JSON 手改错一行、或存储坏一块，
  全部历史质量趋势数据永久丢失、无备份、不可恢复。原实现还有一处连带缺陷：
  加载结果不做类型校验，`trends` 是字符串时下一步 `append` 直接 AttributeError。
- **修法**：畸形文件先备份为带时间戳的损坏副本再置空（同秒重名自动加序号，
  避免覆盖更早的副本；Windows 上句柄被占用会挡掉 rename——集成门禁实测
  WinError 32——故退化成复制式备份，原文件留在原地、后续保存可写新文件）；
  两条备份路径全断才置禁写标记，保存直接跳过、原文件逐字保留，内存历史不丢
  只是不落盘；`trends` 非列表改抛领域异常 DataFormatError（对齐「不得再出现
  裸内置异常 raise」守卫）；测试 3 用 monkeypatch 双阻塞（rename + 复制源）做到平台无关。
- **守卫**：新增损坏防护测试类 3 条（损坏 JSON 备份+新文件干净 / 非列表备份 /
  不可备份时原文件保留），改前红 3 failed（旧版文件沙盒实证）、改后绿
  （quality_trend 16/16）。
- 全量门禁：`7387 passed / 3 skipped / exit 0`（L145 的 7384 + 3 新守卫，无回归）。
  **B216 关闭**。


### L147（2026-10-01）— B217 立项 + 关闭：导出器 max_items=0 被 falsy 判据读成「不限」（L144 同族漏网点）

- **真缺陷（falsy 假零，全仓最后一处）**：增强导出器 `EnhancedExporter` 的条数限制分支写的是
  `if options.max_items and options.max_items > 0`——Python falsy 语义下 `max_items=0` 直接
  短路，**「一条不导」被当成「不限条数」而导全量**。与 L144 修掉的 merge() 缺陷同形
  同族：当轮只修了 dataset_ops 那处，本文件的同族点漏网。
- **修法**：判据改 `if options.max_items is not None and options.max_items >= 0`，
  注释钉住 falsy 回归防护；负数仍读「不限」（维持旧行为，防修过头）。
- **回归护栏**：`test_export_with_max_items_zero_exports_nothing`——max_items=0 ⇒ 0 条；
  同用例对照 None ⇒ 全量（3 条）、负数 ⇒ 全量（3 条）。改前红（旧代码实测 0 条断言失败、
  实际导了 3 条）、改后绿（export_enhanced 45/45）。
- 全量门禁：`7388 passed / 3 skipped / exit 0`（L146 的 7387 + 1 新守卫，无回归）。
  **B217 关闭**。


### L148（2026-10-01）— B218 立项 + 关闭：merge_files 的 removed_duplicates 混报去重与截断（L144 遗留项）

- **真缺陷（统计混报）**：merge_files 返回的 removed_duplicates 直接算
  total_input - total_output，把「去重删掉的」与「max_items 截掉的」混成一个数——
  带截断的配置下（8 条入、去重 2、截 3）报「去重 5」，用户按此读数判断数据质量会被误导。
  该缺陷 L144 修 max_items 假零时已发现并显式留档（行号平移风险），本轮作为独立轮处理。
- **修法**：merge_files 内部用既有的 _deduplicate 对拼接全量复算去重删除数
  （去重是哈希集合操作，与 shuffle 次序无关，复算与 merge 内部结果逐条一致），
  截断数 = 去重后基数 - 输出数；新增返回键 truncated_by_max_items，既有键
  removed_duplicates 语义改为「只记去重」。
- **行引用平移的坑（本轮最值钱的取证）**：首版把统计逻辑放进 merge()（净增 15 行），
  历史文档里 3 条 dataset_ops.py:243 引用平移到空行，A184 硬 0（dead_line）与 L79 基线
  当场红。回退后改为只在 merge_files 局部净增 8 行，242/243 平移后仍落代码行
  （A184+L79 58/58 绿）——凡动被历史行引用钉住的区域，先量平移再动手。
- **回归护栏**：test_merge_files_reports_dedup_and_truncation_separately——
  去重+截断双开分报（2 与 3，旧实现报 5 与无截断键）+ 无去重无截断全 0 对照。
  改前红（旧产品文件实测 1 failed）、改后绿（dataset_ops 87/87）。
- 全量门禁：`7389 passed / 3 skipped / exit 0`（L147 的 7388 + 1 新守卫，无回归）。
  **B218 关闭**。


### L149（2026-10-01）— B219 立项 + 关闭：sample() 的 ratio=0 被 falsy 判据读成「未设置」

- **真缺陷（falsy 假零，ratio 位）**：sample() 的比例分支写的是
  `elif config.ratio:`——显式传 ratio=0（语义「采 0 条」）被 falsy 短路当成「未设置」，
  直接落 else 采全量。与 L144（merge max_items）、L147（导出 max_items）同族：
  那两轮扫 `if config.x` 形状时没扫到 `elif` 形状与 ratio 这个字段。
- **修法**：判据改 `elif config.ratio is not None:`，注释钉住 falsy 回归防护；
  ratio=0 算出的 0 条由下方既有短路守卫（`sample_size == 0` 前置于 method 分发）接住，
  三种 method 一律交空列表。改动插入点在 242/243 历史行引用之下，棘轮 58/58 原地绿。
- **回归护栏**：test_sample_ratio_zero_samples_none——ratio=0 ⇒ 0 条；同用例对照
  None ⇒ 全量。改前红（旧产品文件实测 1 failed）、改后绿（dataset_ops 88/88）。
- 全量门禁：`7390 passed / 3 skipped / exit 0`（L148 的 7389 + 1 新守卫，无回归）。
  **B219 关闭**。


### L150（2026-10-01）— B220 立项 + 关闭：dedup_threshold 死旋钮变真（分档消费）

- **真缺陷（死旋钮）**：_deduplicate 的 threshold 形参从不被读（docstring 自己写着
  「此处未使用」）：MergeConfig.dedup_threshold 承诺「去重阈值」、auto_config 按重复率
  插值推荐 0.85–0.98 并写「上调/下调去重阈值」的理由文字——但产品侧任何取值结果相同，
  整条推荐管线空转。
- **口径拍定（一轮一类）**：threshold 分档近似相似度——≥ 0.9 只删 instruction 完全相同
  （= 原默认行为，保守档），< 0.9 追加宽松键（大小写折叠 + 全空白删除）把近似重复一并删。
  边界 0.9 刻意压在 auto_config 默认区间（0.85, 0.98）之内：高重复率→0.98 保守档
  （与其「避免误删」理由一致）、低重复率→0.85 宽松档；默认 0.9 = 原行为，零回归。
- **坑（测试抓出）**：宽松键归一化首版用单空格 join——多 token 会被折叠回一个空格，
  「如何 申请」与「如何申请」仍不同键；改全空白删除后归一化才真正等价。
- **行平移记账**：_deduplicate 净增约 10 行，3 条 dataset_ops.py:243 历史引用随被引行换位
  退出 L79 的 code_but_no_name_match 档（160→157，常数按「数字搬进常量」条款重钉、逐格归因）；
  A184 棘轮 22/22 原地绿。
- **回归护栏**：test_deduplicate_loose_tier_catches_case_whitespace_duplicates（宽松档归并
  近似重复 / 保守档全留）+ test_merge_consumes_dedup_threshold（配置真被消费，改前 0.5 与
  0.9 同结果 ⇒ 红）；改前红 2 failed、改后绿（dataset_ops 90/90）。
- **顺带回填**：L144 的「merge_files 混报遗留」标记为已由 L148 处理；L115 三条「留待收
  冗余守卫」标记为已关闭（L126 删守卫 + O(1) 重写消退出支）。
- 全量门禁：`7392 passed / 3 skipped / exit 0`（L149 的 7390 + 2 新守卫，无回归）。
  **B220 关闭**。


### L151（2026-10-01）— B221 立项 + 关闭：A125 收口（两组件阈值判据补全）

- **真缺陷（跨两代账本的记档待收项）**：第一本账 A125（L76 立）记着两格组件层判据缺口：
  Deduplicator 的阈值界 `threshold < 0 or threshold > 1` 对 **NaN 双假放行**——去重
  整条静默零动作（与 retry 负 jitter 同形）；**True/False 被算术读成 1.0/0.0**，
  False 档实测误删 2 条互不相同的真数据。QualityScorer 的 threshold **七档全放行**
  （None/'x'/NaN/bool/越界），判负迟到 score() 的 >= 比较（None 那档直接 TypeError）。
  配置层 L76 起就拒这些值，组件层 15 轮未收。
- **修法（一轮一类）**：两组件构造期统一接 `require_ratio`，界与配置侧**共引同一常数**
  （DEDUP_THRESHOLD_RANGE / QUALITY_THRESHOLD_RANGE，A77 一条界只住一处）；None 单独
  显式拒（require_ratio 的 None = 「没传参」语义，组件默认值在签名里，显式传 None 属
  坏值不是缺省）。Deduplicator 仍抛 DedupError 且**文案不带键名**——消费侧公开契约
  由 L76 的钉子（「两边同判不等于两边同文案」）钉住，改抛 DataValidationError 或带键名
  都会拆掉它。
- **钉子翻转（L76 docstring 红字约定）**：`TestDedupGateHasABoolNanHole` 两条按收口翻转——
  「配置拒、组件放」的同判对用例改成「两面同拒」；NaN 静默零动作用例改成构造期拒绝。
  改前红 11（旧产品上全组 152 里 11 failed）、改后绿。
- **行引用平移记账**：dedup.py 顶部插行使 L1 账本 3 条 dedup.py 历史行引用（:111/:626/:431）
  落空行，按 A127/L100 先例就地降名锚；L79 的 MEASURED 三格随现量重钉（157→155 /
  296→293 / 1640→1643，逐格归因），A184 棘轮原地绿。
- 全量门禁：`7403 passed / 3 skipped / exit 0`（L150 的 7392 + 11 新守卫，无回归）。
  **B221 关闭，第一本账 A125 收口**。


### L152（2026-10-01）— B222 立项 + 关闭：A46① 收口（create_stream_processor 死形参删除）

- **真缺口（静默死面）**：`create_stream_processor(processor_func, chunk_size=1000)` 的
  `chunk_size` 形参声明后从不被读——工厂闭包 `stream_process(items)` 逐条调用
  `processor_func`、无状态，块边界在结构上没有任何可观测后果（L35 记档的 A46①，
  全仓 0 调用点，L35 给 `StreamReader` 接的 `require_count` 判据也够不到这一层）。
- **修法拍板**：判据家族先例（L35「修法要么读签名默认、要么入口拒 None」与 L36
  「删字段/报错」）里，唯一不发明行为的选择是**删除死形参**——把它接上「真读」
  需要给无状态逐条处理造一个块边界语义，属新特性，超「一轮一类」边界。
  签名改 `create_stream_processor(processor_func)`，docstring 就地记 A46① 收口。
- **守卫**：新增 `test_dead_chunk_size_param_is_gone`（`inspect.signature` 断言
  形参不在——死形参回来即红）；既有钉死该形参的调用（`chunk_size=10`）更新为无参
  （改前该用例红、改后绿）。
- **行引用平移**：改动在 streaming.py 524+ 行（全部历史行引用落点 :374 之下），
  A184/L79 棘轮原地绿。
- **留档**：A46②（`StreamConfig.chunk_size`/`buffer_size` 从不喂 `StreamReader`，
  产品内 0 使用者、只算 SDK 面）留后续轮；A46③（require_count 的 None）L41 已收。
- 全量门禁：`7404 passed / 3 skipped / exit 0`（L151 的 7403 + 1 新守卫，无回归）。
  **B222 关闭，第一本账 A46① 收口**。


### L153（2026-10-01）— B223 立项 + 关闭：A124 收口（weights 形状判据权威搬家 + 三面共引）

- **真缺陷（判据权威错位 + 三档静默洞）**：第一本账 A124（L76 立）记着
  `quality.weights` 的形状判据住在 `QualityScorer` 两行手抄（只判长度与和），
  配置层只判 null ⇒ 加载面与直构面对同一键**不同判**；且手抄判据有三个洞：
  ① `'abc'` 长度恰 3 ⇒ 穿过 len 判据、死在 `sum()` 的 TypeError（与「权重写错」
  无关的栈）；② `[-1.0, 2.0, 0.0]` 和恰为 1.0 ⇒ **两层面静默放行**，权重符号反了
  会让排序整个反过来不出声（本仓最贵的一族症状）；③ `[True, True, False]` 被
  `sum()` 当 `[1,1,0]`（bool 是 int 子类的老家族）。
- **修法（按 A124 记档的「两侧同批」口径）**：新族员 `validation.require_ratio_list`
  （逐项 bool/NaN/越界 + 长度 + 和=1±0.01；`None` 视为未传由调用点回落，字符串显式
  挡掉）；配置层 `__post_init__`、组件层 `__init__`、静态面 `config_validator` 回放
  **三面共引同一份判据**（A77：一条权威只住一处）；`quality.weights` 进静态规格表。
  组件层手抄两行删除（权威搬家，不留第二份）；连带收掉 `[]` 的静默回落
  （`weights or 默认` 的 falsy 读法 ⇒ 空列表现在即拒，fail-loud 方向）。
- **钉子翻转（L76 约定）**：`test_weights_is_the_only_waived_field_of_quality` 按
  docstring 红字翻转为 `test_weights_spec_exists_and_both_faces_agree`（规格在场 +
  null 仍拒 + 6 档坏值两面同拒 + 6 档合法值两面放行——A124 记档的「容差 0.01
  六档 6/6 不误拒」反向护栏）；对账测试的 weights 豁免移除。
- **行引用平移记账**：config/quality 插行使 L1 账本 4 条 config.py 历史行引用换位
  （3 条退出 code_but_no_name_match 档，config.py 原 311 行那条按先例降名锚）；L79 MEASURED
  三格重钉（ARCH 37→34、LEDGER 155→150 / 293→292 / 1643→1644，逐格归因），A184 棘轮绿。
- 全量门禁：`7410 passed / 3 skipped / exit 0`（L152 的 7404 + 6 新守卫，无回归）。
  **B223 关闭，第一本账 A124 收口**。


### L154（2026-10-01）— B224 立项 + 关闭：A140 收口（余 11 节 29 键整批接上四面判据）

- **真缺陷（第一本账 A140 原框，L82 现量房推导）**：A140 余 11 节（context /
  versioning / sampler / expander / tracker / visualization / multilingual /
  evaluation / benchmark / active_learning / frameworks）共 29 键：运行时零判据、
  静态规格表零行、不可经 API 写入 ⇒ 坏值在 YAML 加载 / SDK 直构 / 静态校验三面
  全静默放行。其中 3 节有真读者：pipeline 直读 context 的轮数（0 与负数在
  range(轮数 - 1) 上答「零轮」，对话上下文整件静默失效，假零家族 L144 同式）、
  versioning 的目录名（空目录名让版本根落在当前目录）与自动快照开关（加引号的
  字符串假值在 if 上恒真）、CLI 基准子命令读 benchmark 的指标清单与基线文件名。
  余 8 节属性面 0 读者，判据买「写时就报」不买房子。
- **修法（A140 原框的 A118 同式，一批收掉 11 节）**：11 节各加节内判据，全引
  validation 族既有成员（布尔族 12 键、计数族 4 键取正、字符串族 5 键、
  字符串清单族 8 键、null 全拒），不新造判据、不新造常数（计数的 1 是调用点
  字面量，无既有常数可共引，数值相等由新守卫钉死）；静态面补 11 节 dict 行 +
  29 键规格行与运行时同档；加载面走构造器自动吃到。
- **新守卫**：新增配置判据守卫一份（8 类）——A140 普查名单翻转（11 节全有节内
  判据、29 键全有规格行、逐节键数现量 29 防删键凑数）、坏值 × 面矩阵
  （布尔 12 键 × 6 档 / 计数 4 键 × 5 档 / 字符串 5 键 × 5 档 / 清单 8 键 × 6 档）、
  合法值反向护栏、静态面同档对账、加载面最小 YAML、消费面合法集同判、「删规格
  行运行时仍红」独立性。
- **棘轮与先例执行**：L83 分档 13 个字符串键从「两档全放」移入「已安全」（该
  分档自带的「动了 A140 就得同时改这里」条款兑现）；配置校验器两把精确集计数
  随承诺自动扩员（数值规格 17 → 21、布尔规格 7 → 19，L82 / L87 同式）；写面
  棘轮「无判据节名单」第三次翻空（按条款同款推导重测），那条行为面用例按其
  docstring 明文处方退役（全 20 节有判据 ⇒ 假臂无来源），假臂本身留档 B225
  待下一轮判删。
- **行引用平移记账**：config.py 插 11 个节内判据块 + 校验器补 40 行规格 ⇒
  L1 两条（A102 行、A119 行）与架构文档两条指向 config.py 尾段的行引用换位、
  按 L151 / L153 先例降名锚；L79 实测三格重钉（架构 42 → 40 与 244 → 246、
  账本 292 → 290 / 1644 → 1646 / 150 → 147，逐格归因）；A184 两格（裸名形状
  入歧义桶 +1、新提普查脚本名入 scratch 桶 +1）；l97 普查 359 → 377（新守卫
  18 处参数化，其中 8 处无 ids 入盲桶 93 → 101，可读差值 266 → 276）。
- 全量门禁：`7692 passed / 3 skipped / exit 0`（覆盖率 99.85%；L153 的 7410 + 新守卫 283
  − 退役的 1 条 l73 行为面用例 = 净增 282，无回归）。
  **B224 关闭，第一本账 A140 收口**。


### L155（2026-10-01）— B225 立项 + 关闭：假臂留防御支、契约钉死（测试-only 轮）

- **立项即处置**：L154 退役 l73 那条行为面用例时留档 B225（假臂二选一待拍）。
  侦察结论：写入路径（api 路由）对「任意 dataclass 节对象」调 `apply_section_update`，
  全 20 节接上判据后这支仓内无来源，但该函数不是 20 节私有件——用户自定义的
  无判据节是合法入参，删臂是对外行为变更而收益仅 2 行 ⇒ 按「简与安」取留：
  防御支保留，把它的行为钉成契约。
- **钉子**：l73 新增 `TestTheUngatedWriteThroughIsAPinnedContract`（2 例，纯断言
  不新设参数化位）：① 无判据节写直通——已知键落值、未知键原样返回、不抛；
  ② 不复查的完整含义——有判据节会拒的 0 / 负数在无判据节照落（这条把「哪一轮
  把判据顺手接到假臂 / 删臂」都变成当场红）。
- **行引用零位移**：本轮产品码零改动（只动 l73 测试文件），L79 / A184 / l97 / l98
  各棘轮全部原地绿（全量门禁证实）。
- 全量门禁：`7694 passed / 3 skipped / exit 0`（L154 的 7692 + 2 新例，无回归）。
  **B225 关闭**。
| B242 | **已关闭（L172，全仓未覆盖语句清零）**：死码删后 coverage 余 4 处**真**未覆盖语句——atomic_write 两个写工具的 finally 清理（写失败时临时件不残留，旧半份窗口防护的半边）、quality_trend 检疫备份名碰撞（同秒双损坏不覆盖第一份备份）、require_ratio_list 非数值元素档（`[1, "abc"]` 逐元素拒）、validate_config 面 quality.weights 坏清单由专项回放承接（非已删死码）——全部补成行为面用例 | 新守卫 5 例；终态全仓未覆盖语句 **0 条**（语句级全覆盖，余豁免全是不可构造异常路径记档）；全量 7763 零回归 | S |

| B247 | **已关闭（L177，死码清扫第三轮：SDK 导出面）**：`__init__.py` 的 `__all__` 220 个导出符号逐条对全仓消费面普查——唯一零消费方的是死别名 `get_supported_export_formats`（`export_enhanced.get_supported_formats` 的 `as` 别名导出，全仓无一处消费；原版由测试面直 import 保活，别名是「为了避开与 converter 版同名函数的重名」而造的中间产物，造出来没人用） | 别名 import 尾删 + `__all__` 条目删（220→219）；**普查机器化为常驻哨兵**：新守卫 2 例——`__all__` 逐条必须可核到产品面或测试面消费方（语料库模块级缓存，单次全量代价受控）+ 死别名禁回流钉；变异自证两种死符号形状（已删符号回流 / 塞不存在符号）全红、sha256 逐字节还原；A184/L79 各棘轮原地绿（__init__.py 的行号引用全在被删两行之前，零位移） | M |

| B248 | **已关闭（L178，A184 索引口径变更：未入仓用户数据目录移出索引）**：A184 的定位引用索引收下顶层两个未入仓用户数据目录（data/ 与 bak/ 两个顶层目录）——data/versions/ 每跑一次全量门禁被 create_version 测试追加新版本目录、bak/ 是手动备份——磁盘态随门禁跑动漂移，ambiguous 快照不可复现（L178 实锤 ambiguous 现量 330 vs 钉值 328，train_data 与 data.json 一名多候选被这两目录的同名件灌入） | 按 L145 的 .backups 先例把两目录移出索引——但**只按「恰好是 ROOT 直接子目录」逐层判**（_walks 新增 top_skip 形参，仅 base==ROOT 时匹配），不误伤嵌套的 augmentor/data 包（其 image.py 等是入仓产品码、被 web 侧类型注释引用，必须留在索引才解析成 live；首版用任意深度同名跳法把它也抹掉、翻成 dead_path 红 2 条，已回退）；指向这两个目录的引用改判「指向不入仓工件」两档 scratch 与 scratch_missing；四桶按口径变更逐格重钉：ambiguous 328→300、runtime_ns 30→0（30 条全是 data/* 示例路径）、scratch 730→768、scratch_missing 302→339（含本条账目自身新引散名 +1/+5/+1）；自检例 data/xxx.json 期望由 runtime_ns 改 scratch_missing；三档硬 0 与 live 面（2488）未动；测试-only 轮零新用例；全量门禁 7787 passed / 3 skipped / exit 0 | M |

| B250 | **已关闭（L180，A155②：路由裸 500 全面普查收边，A155 全清）**：A155 原框点名的「40 处路由里裸写 detail=str(e) 绕过归类器」实测 37 处（api/routes 十个路由文件，quality 9 / version 8 / data 5 / export 4 / augment 3 / config 3 / 余 5）——L179 只修了 to_http_error 这一份权威，这 37 处收尾分支仍把异常原文（部署路径 / 文件系统状态）直接回给客户端，修的是象征不是面 | 新增 raise_internal_error 助手（api/deps.py，与 to_http_error 500 支同一口径 A77：原文落 logger.exception、客户端拿 INTERNAL_ERROR_DETAIL）；37 处裸 500 全部改调助手（十路由 import 补齐，字节级保 CRLF/LF）；tests/integration/test_api_route_500_branches.py 8 处 + test_api_data_500_branches.py 1 处「500 回异常原文」断言按红字重述为 == 固定文案（404 版本不存在 / 文件不存在、400 非法组件等非泄漏档一字不动）；新守卫 2 例含一条**静态形状棘轮**（api/routes 下裸 500 detail=str(异常) 回流当场红）；A155 三格（to_http_error + 37 裸 500 + 404/400 非泄漏档）自此全清；A184 棘轮原地绿（全路径 live 形引用）、L79 漂移桶 145→146 逐格重钉（十路由补 import 行使产品行号下移、账本一条行号格翻漂移档）；全量 7792 passed / 3 skipped / exit 0 | M |

| B251 | **已关闭（L181，A172 收口：泄漏 fuzzy_threshold 区间判据 (0,1] 三面共引）**：leakage 的 fuzzy_threshold 在 SDK 直构 / API 请求模型 / CLI 三面**零区间判据**（请求模型裸 float=0.8 无 gt/ge、构造器不判、CLI 裸 float 位置参）——Jaccard 下界语义可用区间是 0<t<=1：t<=0 时**每条**非精确样本都判泄漏（干净测试集报 100% 泄漏，网络面就能造出假读数），t>1 反向永远报不出近似泄漏（静默关闭）；L95 那条 legacy 守卫当年钉的是「t<=0 保持旧判决」，本轮按 A172 原框翻成「钉住拒绝」 | 判据权威住构造器（augmentor/leakage.py，A77 一条判据一处）：显式 None 专判 + require_ratio(0,1) + 下界取开（0<t 拒假零家族 L144 同式），detect_leakage 与 CLI 面（main 的 except 转「错误:…」+exit 1）经它自动吃到；API 请求面补 Pydantic Field(gt=0,le=1) 边界 422（实测 0.0/1.5/-1 均 422、0.8 放行）；新守卫 11 例（合法 (0,1] 四档放行 1 例 + 坏形状 9 档全拒 9 例 + detect 委托 1 例）；legacy 守卫按红字翻转为「t<=0 构造期即 DataValidationError」（0.3/0.7/0.9 正区间逐字段一致档不动）；A184/L79 各棘轮原地绿（全路径 live 形引用）、l97 普查 CALLS 381→382 / READABLE 280→281 逐格重钉（新守卫坏形状 9 档字面清单 parametrize 进可读桶、盲面 101 不动，l98 随之回稳）；全量 7803 passed / 3 skipped / exit 0 | M |

| B252 | **已关闭（L182，A147 收口：流式单值文件 total_input/processed 分叉记档钉死）**：`augmentor/streaming.py` 的 `read_chunks` 与 `_count_items` 对「整个文件是单个 JSON 值」的读数分叉——一份只写 `{"a": 1}`（单行合法 JSON 对象、非数组）的文件，`_count_items` 走单值支计 **0 条**（它数的是数据项、单值不是数据项），而 `read_chunks` 逐行 `json.loads` 成功产出 **1 条** ⇒ `process()` 报告 `total_input=0` 而 `processed=1`。这是老形状（L86 已逐形状钉住 0/1 两格），但用户看到两读数不一致会当 bug | 按 A147 候选③「原样承认并写进文档」拍定（① 单值算 1 条 / ② 读取循环拒收 都是对外读数变更、会动 API/CLI 报告文案与既有断言，留单独轮不动）：process() docstring 加「单值文件分叉」note 写明 total_input=0/processed=1 是既有契约、读到时不当 bug；tests/unit/test_stream_single_pass_l86.py 新增守卫 1 例把分叉两侧各自钉死（total_input==0、processed==1、两者不等）——谁把任一侧改了都要先翻这一格。测试-only 面（docstring + 新守卫），A184/L79/l97 各棘轮原地绿（docstring 无散名引用、新守卫非参数化位）；全量 7804 passed / 3 skipped / exit 0 | S |

| B253 | **已关闭（L183，收口第一本账 A130：哈希占位符 14 条历史欠账永久记档）**：第一本账「循环日志」标题行的「哈希待 L{n+1} 回填」占位符，L58..L71 共 14 条从未回填，L79 起已钉成精确相等棘轮（PLACEHOLDER_CEILING=14，只降不升）；原框留了「只回填哈希 vs 整段重述 vs 不动」三格待拍 | 拍定取「不动、永久记档」：补那 14 条历史正文属改历史账（哈希虽可从 git log 现取，但那几轮本账的 numstat 与当时秒数已不可重算，只回填哈希会留「半条已填」新形状；整段重述又直接改历史）三格皆不可取 ⇒ 14 条钉成终态欠账。落点：tests/unit/test_doc_line_refs_l79.py 的 test_only_the_current_round_leaves_a_hash_placeholder docstring 订正（原写「加本轮=15」旧读数，现稳态恰 14）+ 记 L183 拍定；第一本账 A130 行就地标「已关闭（L183）、14 条永久记档」。测试-only/文档轮零新用例；L79/A184 各棘轮原地绿（首账改动全为散文、零新反引号引用，全量门禁证实）；全量 7804 passed / 3 skipped / exit 0 | S |

| B254 | **已关闭（L184，A138 收口：可写四节 14 键「读者分档」记档 + 回显面机器钉死）**：export/vector/rag/multimodal 四节 14 键 L82 起全接判据（写坏即 400），但「写得进」不等于「有人读」——按现量分三档：① 有行为读者 3 键（export.default_format→augmentor/pipeline.py；rag.chunk_size 与 rag.chunk_overlap→augmentor/cli/commands/quality.py）、② 仅回显 6 键（GET /api/config 原样回显、不改行为）、③ 连回显都没有 5 键（vector.dimension/storage_dir/collection、multimodal.image/audio_extensions，无处消费）；契约面承诺能力、实现面不消费，用户配 vector.backend: faiss 会以为向量库在用 faiss（实际后端入口是调用参数） | 候选①（接消费入口=功能增强，要先拍「配置键 vs 请求参数谁是权威」）与②（降级=对外破坏面）均不在本轮；取**记档 + 机器钉面**：docs/API.md 新增「可写四节的读者分档」小节（三档表 + 权威口径）；tests/integration/test_api_config_extended.py 新增守卫 2 例钉回显面——③ 档 5 键不得出现在 GET /api/config、①② 档 7 键是既定回显面不得被顺手摘掉（谁接了读者/回显都要先翻这一格）。A184 新增引用全用全路径 live 形、①②③ 键为点号键名（无文件扩展名、不进 REF）⇒ A184/L79 各棘轮原地绿；全量 7806 passed / 3 skipped / exit 0 | M |

| B255 | **已关闭（L185，A95/A122 合轮收口：模型凭证 ${ENV} 占位符展开语义钉成契约）**：模型条目三凭证键（api_key/secret_key/base_url）经 _resolve_env 解析 ${ENV} 占位符，**未设置的环境变量静默回落 ''**（与 None 等价，都是「这条没配凭据」的合法状态，由后端 ModelNotConfiguredError 在建后端时判）；A122 原框把「${ENV} 展开失败静默」与「三键零判据」留待拍，与 A95（展开出声口径）是同一件事的两半 | 拍定取**非破坏面收口（记档 + 机器钉死，不改加载期行为）**：加载期「环境变量未设置」条数随本机 shell 而变、接进加载声会让同一份配置不同机器行数不同（docs/API.md 3.x 既定口径），故静默 '' 是**刻意契约**、出声归 validate-config 的 warning 通道；本轮把该语义钉死——augmentor/config.py 的 _resolve_env docstring 写明四档（已设置/未设置静默''/非 ${...} 完整形状原样/非串原样）；tests/unit/test_model_entry_defaults_l75.py 新增守卫 13 例（已设置取值 + 未设置静默'' + 坏形状 5 档原样 + 非串 4 档原样 + 合法占位未设置'' + MODEL_CREDENTIAL_KEYS 权威清单 == 三键）。l97 普查 CALLS 382→384 / READABLE 281→283 逐格重钉（两支 parametrize 字面清单进可读桶、盲面 101 不动、l98 随之回稳）；A184/L79 各棘轮原地绿（全路径 live 形引用）；全量 7819 passed / 3 skipped / exit 0 | M |

| B256 | **已关闭（L186，cache TTL falsy 假零收口）**：augmentor/cache.py 的 MemoryCache.set（:121）与 DiskCache.set（:268）用 `ttl or self._default_ttl` 把显式 0.0 读成「没传」而落到默认 TTL——但 0.0 本是合法「立即过期」值（CacheEntry.is_expired 对 ttl=0 恒真）；同族第三处在 DiskCache.get（:235）`if ttl and ...` 把读侧 ttl=0.0 读成「不过期」。三处 falsy 假零让「立即过期」这个档位静默失效 | 三处改 `is not None` 判型（set×2 + get×1，零值保留为「立即过期」、None 才回落默认/不过期）；tests/unit/test_cache.py 新增守卫 2 例（MemoryCache 与 DiskCache 各钉「ttl=0.0 立即过期 + None 回落默认」两侧对照）。CRLF 文件全程二进制写保真；A184/L79/l97 各棘轮原地绿（无散名引用、非参数化位、cache.py 插行使 0 条活行引用顶歪）；全量 7821 passed / 3 skipped / exit 0 | S |

| B257 | **已关闭（L187，benchmark delta_ratio 除零档自相矛盾收口）**：augmentor/benchmark.py 的 compare_with_baseline（:240）对基准读数为 0.0 的指标（pass_rate / diversity 合法读成 0）用 `if reference else 0.0` 记 delta_ratio=0.0——把「相对变化未定义（除零）」读成「无相对变化」，与同条目 improved/regressed 状态自相矛盾（baseline=0、current=0.5 判 improved，delta_ratio 却 0.0） | 改 `if reference != 0.0 else None`：reference==0.0 时记 None（与 :217 no_baseline 哨兵同口径），非 0 照旧相除；下游 Markdown 报告只读 current/baseline/delta/status（delta_ratio 不进表格）、无测试面读 delta_ratio ⇒ 零回归。tests/unit/test_benchmark.py 新增守卫 3 例（零基准 improved→None、零基准零当前 unchanged→None、非零基准照旧 0.8 防修过头）；A184/L79/l97 各棘轮原地绿；全量 7824 passed / 3 skipped / exit 0 | S |

| B258 | **已关闭（L188，PiiSanitizer 构造器 falsy 假零收口：fields/patterns/placeholders 三处）**：augmentor/privacy.py 的 PiiSanitizer.__init__ 三处 `x or default`——显式空容器（fields=[] / patterns={} / placeholders={}）本是「零元素」的合法意图，却被 falsy 读成「没传」而悄悄套上默认（脱敏范围 / 字段面 / 占位符表被无声放大）；最典型 patterns={}（「我只要这几条自定义规则、其余不脱」）被读成「用全量 DEFAULT_PATTERNS」 | 三处改 `x if x is not None else default`（空容器保留为「零元素」、None 才回落默认）；tests/unit/test_privacy.py 新增守卫 4 例（patterns={} 零模式不脱敏 + None 回落默认全量防修过头 + fields=[] 零字段 + placeholders={} 保留空表）。CRLF/LF 保真；A184/L79/l97 各棘轮原地绿（无散名引用、非参数化位）；全量 7828 passed / 3 skipped / exit 0 | S |

| B259 | **已关闭（L189，验证预设封闭清单下沉 SDK 直构面 + `rules` falsy 假零）**：augmentor/validation.py 的 DatasetValidator.__init__ 同格两缺陷：① `if rules:` 把显式 `rules={}`（「零规则」的合法意图）读成「没传」，静默落 preset 规则面——strict 档传 {} 拿到 basic 面（falsy 假零族，L144/L147/L149/L186/L188 先例）；② 未知 preset 静默回落 basic——拼错 strict/chat 被读成 basic，规则面整档放宽不出声（L175/L176 封闭清单族同型：API 面 400、CLI 面 argparse choices 早有收口，SDK 直构面独缺这一格） | ① 改 `is not None` 判型（`rules={}` 保留零规则、None 才走 preset）；② 回落支改抛 DataValidationError 并列出 PRESET_RULES 全部键（清单权威住 SDK 层，A77）；API 面 VALIDATION_PRESETS 由手抄字面改共引 `tuple(DatasetValidator.PRESET_RULES)`（L175 的 SEARCH_METHODS 共引先例，400 判据与文案不变）；CLI 面 parser.py 维持「不 import 业务模块、字面清单」既有惯例，同一性由新守卫钉；test_validation.py 旧「未知回落 basic」例红字重述为拒收例；tests/unit/test_validation_preset_closed_list_l189.py 新增守卫 10 例（rules={} 零规则判别探针 + None 回落 preset 浅拷贝防修过头 + 坏预设 5 档 parametrize 字面清单 + None 预设拒 + API 共引派生钉（源码级读法防回流手抄）+ CLI choices 与 PRESET_RULES 同一集合钉）；l97 重钉 CALLS 384→385、READABLE 283→284（盲面 101 不动）；A184 账本既有活行引用 validation.py:639 因 __init__ 插入 4 行注释档平移至 :697（修引用，非新增散名）；L79 原地绿。全量门禁：7838 passed / 3 skipped / exit 0（L188 的 7828 + 10 新守卫，无回归） | S |

| B260 | **已关闭（L190，leakage min_examples 计数旋钮收口）**：augmentor/leakage.py 的 LeakageDetector.__init__ 对 min_examples 零判据（`self.min_examples = min_examples` 原样入切片下标）：负数让 `[:n]` 切片换语义（`[:-2]` = 丢掉最后 2 条，「要 2 条」变「要 N-2 条」，silent 错答族；与 L144 的「取前 N 条」计数旋钮同式，此前漏在这一格），None / 非整数 / bool 原样进切片下标。0 是合法「报告不带示例」意图（与「没传参数」区分开，require_count minimum=0 口径） | 构造器加 `require_count("min_examples", min_examples, minimum=0)` + 显式 None 专判（L181 的 fuzzy_threshold None 专判先例）；tests/unit/test_leakage.py 新增守卫 13 例（合法 4 档 parametrize + 坏形状 8 档 parametrize + 0 档语义钉死：精确泄漏也零示例但统计照出）；l97 重钉 CALLS 385→387、READABLE 284→286（盲面 101 不动）；A184/L79/markdown-structure 各棘轮原地绿（B 表新行须与相邻行留空行，GFM 表格判据）；全量门禁：7851 passed / 3 skipped / exit 0（L189 的 7838 + 13 新守卫，无回归） | S |

| B261 | **已关闭（L191，sampler recommend_seeds 问题类型面 O(k×n) 重复分析→预计算）**：augmentor/sampler.py 的 recommend_seeds 在「每个覆盖不足类型 × 每条数据」的内层循环里对同一条 item 反复调 _analyze_question_type（纯函数、同输入同输出），k 个 question_type 覆盖不足档 = k 倍重复调用（98 条语料实测：top_k=1 付 2n+91 次、top_k=20 付 2n+186 次，差 95 次；修后两档同收在 3n=294 次） | 循环前预计算一次成索引数组 qtypes（O(n)），内层循环改 `enumerate` + `qtypes[i]` 查表（O(1)），语义不变（首匹配 seed 仍是数据里第一条匹配条目，守卫钉死 items[90]）；tests/unit/test_sampler.py 新增守卫 2 例（形状棘轮：同数据 top_k=1 与 top_k=20 两档调用数必须相同且 ≤3n——旧 k×n 形状当场分叉变红；+ 首匹配语义钉）；l121 docstring 的分支弧 `sampler.py 265->260` 因插入 5 行平移 `270->265`（红字重述）；变异自证：换回旧形 1 红（调用数分叉）、sha256 逐字节还原；l97/A184/L79/markdown 各棘轮原地绿（无新参数化位、无新增散名引用）；全量门禁：7853 passed / 3 skipped / exit 0（L190 的 7851 + 2 新守卫，无回归） | S |

| B262 | **已关闭（L192，cleaner 规则名封闭清单下沉 SDK 直构面）**：augmentor/cleaner.py 的 DatasetCleaner.clean 对未知规则名静默跳过（`for rule_name in rules: if rule_name in self._default_rules:`），拼错 normalize_whitespace = 该规则无声不生效、清洗报告照常出（checkpoint 症状族语义漂移档；L175/L176/L189 封闭清单族同式）。清单权威住 `_default_rules` 键（A77），CLI 面 parser.CLEAN_RULES 字面清单早有 argparse choices 收口，SDK 直构面独缺这一格 | clean() 入口补封闭清单判据（unknown = 非 str 或不在 `_default_rules`，一并拒；列出全部 11 个合法规则名），clean_dataset / clean_batch_optimized 经委托自动吃档；test_cleaner.py 新增守卫 5 例（未知名拒+全量文案 + 坏形状 4 档 + 合法名照旧生效防修过头 + 两条入口委托继承 + CLI 字面清单与 SDK 键同一集合钉）；test_partial_branches_l118 的旧「未知名跳过不报错」例红字重述为拒收例、docstring 勘误注 L192 假支不可达；l97/A184/L79/markdown 各棘轮原地绿（守卫无 parametrize、无新增散名行引用）；全量门禁：7858 passed / 3 skipped / exit 0（L191 的 7853 + 5 新守卫，无回归） | S |

| B263 | **已关闭（L193，迁移规则 id 封闭清单下沉 SDK 直构面）**：augmentor/migration.py 的 DatasetMigrator.migrate 对未知规则 id 静默丢弃（`r.rule_id in rules` 过滤，全拼错时迁移照跑、rules_applied 空数组，「迁移跑了个寂寞」不出声；L175/L176/L189/L192 封闭清单族同式）。API 面 system_ops.MIGRATION_RULES 早有 400 判据，SDK 直构面（migrate / migrate_dataset / migrate_file / CLI migrate）独缺这一格 | migration.py 加内置清单常量 `BUILTIN_MIGRATION_RULE_IDS`（A77 权威，守卫钉与 `_create_builtin_rules` 逐一对上）+ `migrate()` 入口拒未知/坏形状 id（列全部合法 id；自定义规则经 add_rule 注册后仍是合法 id，不修过头）；API 面 MIGRATION_RULES 由手抄字面改共引同一常量（is 钉，400 判据不变）；test_migration.py 新增守卫 5 例（常量追平内置规则 + 未知 id 拒 + 坏形状 3 档 + 自定义 id 放行 + API 共引 is 钉），旧「未知跳过不崩溃」例红字重述为拒收例；L193 顶部插 1 行共引 import 使 system_ops.py 全部行号 +1：OPTIMIZATION_LOOP.md 既有 6 处 `system_ops.py:147/:292×3/:397×2` 活行引用平移 :148/:293×3/:398×2（修引用，非新增散名）、docs/ARCHITECTURE.md 2 处同移（:147→:148、:292→:293，L79 散文数字重钉随修引用回稳而非升档）；l97 原地绿（守卫无 parametrize）；全量门禁：7863 passed / 3 skipped / exit 0（L192 的 7858 + 5 新守卫，无回归） | S |

| B249 | **已关闭（L179，A155①：to_http_error 500 档不再转发异常原文）**：api/deps.py 的 to_http_error 最后一支把未分类异常原样 str(exc) 塞进 500 响应体——OSError(13,'Permission denied','/srv/...') 实测把部署根绝对路径与文件系统状态（Errno 28 磁盘满 / Errno 13 权限缺失）一并回给客户端，是可被利用的探测信号；A155 原框两格候选拍定取①（固定文案 + 原文落服务端日志，安全收紧方向，非脱敏白名单②） | 500 支改回固定文案常量 INTERNAL_ERROR_DETAIL（api/deps.py 模块级唯一产地，A77）+ logger.exception 把原文留服务端日志；400 支（ValueError 可行动文案）一字不动防修过头；tests/unit/test_api_deps.py 新增守卫 3 例（OSError 不泄漏部署路径/errno + 原文进日志 + ValueError 仍 400 原样回传）；tests/integration/test_api_dataset_system_tools.py 两处「500 回原文」断言按红字重述为「== 固定文案 + 原文不在响应体」（TestUnexpectedFailureBecomes500 五处 + dataset_impact/evaluate 各一，docstring 同步改口径）；A184/L79 各棘轮原地绿（本轮新增引用全用全路径 live 形、零散名）；全量 7790 passed / 3 skipped / exit 0（L178 的 7787 + 3 新守卫，零回归） | M |

| B246 | **已关闭（L176）**：策略旋钮「封闭清单拒」同型普查（L175 的 method 静默回落修完后沿同型全仓扫）——sample 的 method / outlier 的 method / expander 的strategy 三处**已有** else 拒（普查实证）；唯一缺口 `save_quality_report` 的 format：未知 format **静默落 JSON 支**（请求 yaml 的拿到 json 文件不出声，checkpoint 症状族语义漂移档） | 缺口收边（format 封闭清单 `QUALITY_REPORT_FORMATS` + 入口 require_choice + None 专判）；**普查结论钉成棘轮**：新守卫 6 例含一条「五旋钮 × 未知值全部出 DataValidationError」合体例——任一旋钮未来退化回静默回落当场红；变异自证 2 红（剪 format 判据）、sha256 逐字节还原；A184/L79 各棘轮原地绿（quality_report.py 插行使 0 条活行引用顶歪，全量门禁证实） | M |

| B245 | **已关闭（L175）**：搜索方法封闭清单只住 API 面（dataset_tools 路由的 400 判据）、CLI 面靠 argparse choices——但 **SDK 直构面静默**：`search_dataset(method='regex_typo')` 不进分支落 contains，请求 regex 的拿到 contains 结果与分数，不出任何声（checkpoint 症状族的语义漂移档；L174 探针九档里唯一真缺口） | 清单权威下沉 SDK 层（search_enhanced.SEARCH_METHODS），API 路由改共引（is 钉）；search 入口 require_choice 判封闭清单 + None 单独判（必填参数不适用「未传回落」语义）+ 空串走 require_string 那刀；新守卫 14 例（未知/形状错/None 三档 + 五合法值parametrize + API 共引 is）；parametrize 值位全部字面清单（动态 list() 调用进盲面桶，同轮字面化 + l97 重钉）；变异自证 7 红、sha256 逐字节还原；A184/L79/l97/l98 各棘轮逐格重钉（全量门禁证实） | M |

| B244 | **已关闭（L174）**：ngram 热路径的**相对**性能形状棘轮缺失——L 轮性能工作把 `_search_ngram` 从「每条文档物化整串 gram 集合」（O(文档长度×gram) 对象创建）换成「查询侧 k 个 gram × 子串判定」（约 6 倍提速）后，只有**同分**有守卫，**形状**没有：谁把实现换回朴素形状不会红（结果逐条同分，同分用例拦不住慢实现）。绝对毫秒预算又是 A157 记过的环境性假红 ⇒ 要的是**同机 A/B 相对比** | 新守卫 2 例：同进程交替量当前实现与朴素物化实现的耗时比（≤ 0.7 系数，形状回退时比值 → ≈1 当场红；系数留环境余量，小语料保棘轮便宜）+ 小语料同分护栏（不与既有全量同分用例重复付账）；变异自证：把 `_search_ngram` 真换回朴素形状 → 棘轮 1 红（形状可检测被实证）、sha256 逐字节还原；A184/L79/docs 各棘轮原地绿（全量门禁证实） | M |

| B243 | **已关闭（L173，死码清扫第二轮：无引用常数）**：全仓普查（root + cli + api，加二次全树 grep 双保险）撞出 3 个零引用模块级常数——converter 的 FLAT_FORMATS / CONVERSATION_FORMATS（L77 起格式清单改由枚举推导后无人消费的两组分类元组）、feature_detect 的 FEATURE_TYPES（消费点直接内联 isinstance 三行后的孤儿判定字典）——死码整组移除（共 10 行） | 全量 7763 零回归实证删的是真死码；converter.py 删行使账本 L23 那行两条行号格落空行，按先例降级名锚 + L79 三桶逐格重钉（line_refs −2 / file_tokens +2 / 漂移 +2 / FLOOR 同步）；A184/docs 各棘轮原地绿（全量门禁证实）。坑记档：普查文件列表口径缺子包，死码删除前必须全树 grep双保险（_find_duplicates_faiss 一类测试面公开方法靠它才排掉误报） | S |

| B241 | **已关闭（L171）**：L157 撤判决维后 config_validator 规格走查段残留的**四个永久死分支**（min/max 范围检查、item_choices 元素成员、choices 允许集合——两表已无这四维，分支条件恒假）——A139 收口时保守留着走查结构、未同步清码，本轮回放承接已实锤（四档坏值在 validate_config 面上仍红），死码删除 | 三块死分支整块移除（约 −30 行）；全量 7758 零回归实证「删除没删判据」（承接在回放）；删除后规格走查只留活判据（类型/NaN/items/non_empty/non_blank/renderable 七维）；A184/L79/docs 各棘轮原地绿（无行号引用变动——删行在活引用区间之后） | S |

| B240 | **已关闭（L170，A182 收口裁定 + API 裸数值字段普查）**：L162 记档的「同名覆盖写中途崩毁既有」待裁档在本轮裁定——备份快照与恢复输出两边收原子写。裁定理由不是扩判据而是**语义自洽**：备份/恢复的语义就是数据保全，「备份操作毁旧备份」「恢复动作毁用户输出路径上的既有文件」自相矛盾，os.replace 前旧内容完好的性质恰好是这两边要的保全（工具零成本覆盖）。同轮完成 API 裸数值字段普查（18 处）：全部链路完整或已有契约——比例三键（DataSplitter 判据→DataValidationError→ValueError 子类→400 可行动）、quality/dedup 三 threshold（组件层 L48/L76 判据同 400 形态）、leakage fuzzy_threshold 属 A172 待拍不动、分页/size 杂项无错误答案路径 ⇒ 本普查未撞出新缺陷，面扫清记档 | backup.py 两边改调 atomic_write_json（快照边 + 恢复输出边，写失败仍由既有 except 路径承担）；l99 棘轮 21→19，**A182 逐面收实质完成**（剩余 19 处全为有据豁免：CLI 报告面 10、快照响亮档 4、工具本体 2、cache 值边 1、其余豁免记档）；新守卫 4 例（旧备份完好 + 用户输出完好 + 回环 + 静态形状）；变异自证 1 红、sha256 还原；A184/L79/docs 各棘轮原地绿（全量门禁证实） | M |

| B239 | **已关闭（L169，A182 逐面收路线第十一面 + 契约保持型收边）**：实验记录与趋势历史两写边收原子写。这两处与 L160–L168 判定路径不同——读侧契约**本来就是静默的**（load_experiment 损坏→None、「不应崩溃」、L146 检疫，test_tracker 三处测试钉住）——侦察修正 L168 的记档：这不是待裁欠账而是已有契约；收原子**不翻案**：半份窗口消失后损坏只剩余磁盘故障一途，读侧静默路径更难触达、写失败仍被 except 吞（契约保持），L146 检疫保留为纵深防御 | 两边改调 atomic_write_json；l99 棘轮 23→21；新守卫 3 例（写中断契约保持+旧文件完好 / 检疫标志不被正常写触发 / 静态形状）；变异自证 1 红、sha256 还原；**一处集成测试随收边修正**：NamedTemporaryFile 借名习惯在原子替换下撞 Windows 句柄拒绝访问，改 mkstemp 建后即关（测试基建适配，产品语义更安全）；A184/L79/docs 各棘轮原地绿（全量门禁证实） | S |

| B238 | **已关闭（L168，A182 逐面收路线第十面）**：管线交付口两边（`AugmentorPipeline` 的 process 与异步变体的最终输出——产品核心交付面，API 链落数据根）与迁移输出边（`/api/system/migrate` 路径经白名单 for_write 校验）三边同判据必收；同轮完成剩余候选的读侧行为对账归档：benchmark 基准边读侧裸抛响亮、comparison/quality_report/visualize_enhanced 报告面、faiss meta 边读侧裸抛——全部有据豁免；**记档待裁**：tracker 实验记录是「写侧 except 吞成 log + 读侧吞成 None」的 checkpoint 症状完全体（修它要改两侧异常语义，属 fail-loud 行为变更另轮），quality_trend 同族复合形状（有检疫交互）一并待裁 | 三边改调 atomic_write_json；l99 棘轮 26→23；新守卫 4 例（迁移写中断旧文件完好 + 回环 + 管线双锚静态 + 迁移静态形状）；变异自证 2 红、sha256 还原；pipeline.py 插行使账本三条行号格（A138 一条落空行 + _init_components 两条落别行）降级名锚 + L79 三桶逐格重钉（line_refs 276→273 / file_tokens 1655→1658 / 漂移 142→141 / FLOOR 同步）；A184/docs 各棘轮原地绿（全量门禁证实） | M |

| B237 | **已关闭（L167，A182 逐面收路线第九面 + 工具参数化）**：增强导出的九格式共用落盘口（`_export_json`，alpaca/sharegpt/chatml/llama/vicuna/belle/openai/hf/json 九个分支一处收口覆盖全交付面）与 L163 同判据必收；与 L163 不同点：本边的 ensure_ascii / indent 是**活选项**（ExportOptions 契约，既有测试断言过 indent=4）——不能直换紧凑工具 | `atomic_write_json` 参数化（可选 ensure_ascii/indent，默认紧凑与既有调用字节零变化，实证：默认档输出逐字节不变）；选项原样透传；l99 棘轮修正一轮：现量 28 = 预期 27 + 1（参数化把工具本体单行拆两支的形状噪声）——工具本体豁免显式化（atomic_write.py 写的是自身临时件非产品写边，普查排除并记档），棘轮 27→26；新守卫 4 例（默认紧凑字节断言 + 写中断旧文件完好 + 选项透传 + 静态形状）；变异自证 2 红、sha256 还原；A143 那行 ExportFormat 行号格降级名锚（账本 + l79 自注释两处）+ L79 两桶重钉 + FLOOR 同步；A184/docs 各棘轮原地绿（全量门禁证实） | M |

| B236 | **已关闭（L166，A182 逐面收路线第八面）**：数据集工具路由的落盘口（`_dump` helper，写盘变换类六端点 convert/merge/sample/split/aggregate/rag 共用）与 L163/L164 同判据必收——产物经白名单落数据根、list_data_files 扫描读者在写窗口内拿半份；data_ops 命令的 --output 报告边同轮分档为豁免（CLI 报告一次性写，L165 口径） | `_dump` 改调 atomic_write_json（父目录创建由工具承担）；l99 棘轮 29→28（api/routes 现量降 1）；新守卫 3 例（写中断旧文件完好 + 回环建父目录 + 静态形状）；变异自证 3 红、sha256 还原；A184/L79/docs 各棘轮原地绿（全量门禁证实） | S |

| B235 | **已关闭（L165，A182 逐面收路线第七面 + 逐面分档完成）**：依赖登记两边（DependencyManager 的 _save_datasets / _save_dependencies）与 L160–L162 同构第四份——读侧 _load_file 无 except 且 __init__ 必调，半份登记表的写窗口里任何新构造实例被当场炸掉；**另一路也封死**：若有人顺手给 _load_file 加 except，截断会静默回落 default——已登记的数据集/依赖凭空消失（L99 checkpoint 症状），两条路都指向收原子 | 两边改调 atomic_write_json；l99 棘轮 31→29；新守卫 3 例（写中断旧登记完好 + 回环 + 静态形状）；变异自证 2 红、sha256 还原；**同轮完成 A182 剩余 29 处的逐面分档**：CLI 报告一次性写边（ops/analysis/quality/io 的 --output 面）按判据豁免（同步等待、任意路径、无扫描链），versioning 快照两边豁免（读侧响亮抛异常，同 L161 快照判），atomic_write.py 自身豁免（工具本体）；剩余必收候选（dataset_tools 路由边、export_enhanced 交付面、data_ops 命令输出边）已点名待下轮；A184/L79/docs 各棘轮原地绿（全量门禁证实） | M |

| B234 | **已关闭（L164，A182 逐面收路线第六面）**：数据集变换三边（`DatasetOperations` 的 merge_files / sample_file / split_file）与 L163 交付面同判据——API 侧（`dataset_tools` 路由）与 CLI 侧输出路径都经白名单落进数据根，`list_data_files` 按 *.json 扫同一目录，写窗口内读者拿半份；三边输入读侧 `json.load` 裸抛（响亮失败）不属缺陷面 | 三边改调 `atomic_write_json`；l99 棘轮 34→31；新守卫 4 例（写中断旧文件完好 + 三变换回环 + 同名覆盖写 + 静态形状）；变异自证 2 红、sha256 逐字节还原；dataset_ops.py 插行使账本六条历史读数格（入参判定清单格 ×1、分配器调用方格 ×5）落漂移桶，按先例全部降级名锚 + L79 三桶逐格重钉（line_refs 283→277 / file_tokens 1648→1654 / 漂移回 142 / FLOOR 同步）；A184 / docs 各棘轮原地绿（全量门禁证实） | M |

| B233 | **已关闭（L163，A182 逐面收路线第五面）**：导出五边（JSONL 文本边 + LLAMA_FACTORY / ALPACA / SHAREGPT / CHATML 四条 JSON 边）是**导出物交付面**——输出落 `web.data_roots` 白名单目录，数据管理端点按 *.json 扫同一目录，文件在写窗口一开始就出现在列表里，预览 / 下载在窗口内读到的就是半份「文件不是合法 JSON」（atomic_write 模块 docstring 点名的读者形状），必收 | 工具面新增 `atomic_write_text`（JSONL 是文本写，替换语义与临时件命名与 JSON 版逐字同口径——临时件名避开两种扫描）；四条 JSON 边改调 `atomic_write_json`、文本边改调新工具；写盘字节从 indent=2 变紧凑序列化（读侧只解析不读格式，既有测试面无格式断言，实证于全量门禁）；l99 棘轮 38→34；新守卫 12 例（五格式写中断旧文件完好 + 回环 + 临时件不残留 + 工具口径 + 静态形状）；l97 重钉（CALLS 377→379、READABLE 276→278，盲面 101 不动）；两份活文档三条历史行号格按先例降级名锚 + L79 三桶逐格重钉（ARCH line_refs 40→39/file_tokens 246→247、LEDGER line_refs 284→283/file_tokens 1647→1648、漂移回 145；FLOOR 下界同步）；变异自证由写中断行为面承担（每格式断言旧文件完好 + 临时件清掉）；A184 / docs 各棘轮原地绿（全量门禁证实） | M |

| B232 | **已关闭（L162，A182 逐面收路线第四面）**：BackupManager 索引两边（默认索引写 + `_save_index`）与 L160/L161 逐字同构——读侧 `_load_index` 无 except 且在 `__init__` 必调，半份索引窗口 = 新实例构造当场崩 = 错误答案档，必收；**另一半有据不收**：快照与恢复输出两边读侧响亮抛异常（restore 前校验和哨兵先出声）= 可恢复档；它们的「同名覆盖写中途崩毁既有」窗口是判据口径未覆盖的另一档，记档待裁不在本轮擅自扩判据 | 两边改调既有 `atomic_write_json`；l99 棘轮 40→38 重钉逐格归因（backup.py 的 4 处计数边现量降 2）；新守卫 4 例（索引写中断旧账完好 + 临时件不残留 + 快照豁免条件机器化 + 静态形状）；变异自证 2 红、sha256 逐字节还原；backup.py 插行使账本一条历史读数清单的行号格落非代码行，按先例降级名锚 + L79 三桶（line_refs −1 / file_tokens +1 / 漂移 −1）逐格重钉；A184 / docs 各棘轮原地绿 | S |

| B231 | **已关闭（L161，A182 逐面收路线第三面）**：DatasetVersionManager 索引两边（`_load_index` 的默认索引写 + `_save_index`）是 version_control.py 四处计数写边里的**必收一半**——读侧 `_load_index` 无 except 且在 `__init__` 必调（L160 DiskCache 同款形状），半份索引窗口里任何新构造实例被 JSONDecodeError 当场炸掉 = 错误答案档；**另一半刻意不收**——`create_version` 的 data/version 快照两边读侧 `load_version` 响亮抛异常（fail loud = 判据口径的可恢复代价），且快照写一次不再改、写后即算 checksum，豁免条件已机器化进守卫（读侧改静默吞 ⇒ 红 ⇒ 重新对账） | 两边改调既有 `atomic_write_json`（工具同源）；l99 棘轮 42→40 重钉逐格归因（version_control 的 4 处计数边现量降 2）；新守卫 4 例（索引写中断旧账完好 + 临时件不残留 + 快照豁免条件机器化 + 静态形状）；变异自证 2 红、sha256 逐字节还原；CRLF 文件全程二进制写保真；A184 / L79 / docs 各棘轮原地绿（全量门禁证实） | S |

| B230 | **已关闭（L160，A182 逐面收路线第二面）**：DiskCache 元数据写边（`_save_metadata`）是 A182 原文点名的「缓存写」类里的**必收半边**——读侧 `_load_metadata` 无 except 且在 `__init__` 必调，旧写法的写窗口里共享同一 cache_dir 的第二个实例构造被半份元数据当场炸掉（JSONDecodeError 冒泡 = 错误答案档）；同类的**另一半刻意不收**——`set` 的缓存值写边读侧 `get` 把坏 JSON 按未命中重算（可恢复代价，atomic_write 判据口径明文「不必动」），这条条件豁免已机器化进守卫（get 读侧行为变了 ⇒ 红 ⇒ 重新对账） | `_save_metadata` 改调既有 `atomic_write_json`（工具同源，不手搓 os.replace）；l99 棘轮 43→42 重钉并逐格归因（cache.py 的 2 处里现量只降必收的这 1 处）；新守卫 4 例（写中断旧元数据完好 + 临时件不残留 + 豁免条件机器化 + 静态形状）；变异自证 2 红、sha256 逐字节还原；cache.py 为 CRLF 全程二进制写保真；A184 / L79 / docs 各棘轮原地绿（全量门禁证实） | S |

| B229 | **已关闭（L159，A85 族第四侧）**：悬空 `models.default`（指向不存在的条目）在**校验面零错零警**——消费点 `get_model_config`（pipeline 构造必经）在「要建默认后端」时抛可行动的 ConfigError，但校验工具对这一档完全沉默；且这一档**不是必炸**（显式按名取模型的调用方不受影响）。v2 曾在校验面判 ERROR，被 56 例既有测试面当场实证过度收紧（测试底座大量使用「只带 default 无条目」的形状），v3 按既有「值不会生效」族先例降为 **warning**（`_warn_unread_model_keys` 同档） | 「模型未配置」文案提为 `_missing_model_message` 唯一产地（A77），消费点与校验面共引（运行时判决与文案零变化）；校验面悬空档真调产地产 warning。新守卫 6 例（ghost/空串 warning 面 + 合法零噪声 + 类型档不叠 + 缺席档归必填通道 + 消费点判决零变化 + 产地唯一/校验面不抄）；变异自证：短路新分支 1 红、sha256 逐字节还原；两处既有断言按红字重述（一处「警告总数恰一」改「全是 warning 档 + 期望路径在场」、一处缺席档期望更正）；config.py 末尾追加零位移 ⇒ A184 / L79 / l97 / l98 各棘轮全部原地绿（全量门禁证实） | S |

| B228 | **已关闭（L158，A85 族第三侧漏网）**：`models.<名字>` 条目写成标量 / 列表 / 数的档在**校验面完全静默**——`validate_config` 对 `models` 条目给 `[1]` / `'x'` / `None` / `42` 报 is_valid=True（回放段 `not isinstance(entry, dict)` 直接 continue），而运行时 `load_config` 抛 ConfigError「必须是映射」；同一份配置校验工具说好、应用启动才炸。A85 / A101 当年补的是运行时两侧（`_load_section` 与 models 条目），校验面这一侧漏网；HEAD~1 复跑同盲——不是 L157 回归，是被 L157 回放重写之后的系统面普查撞出的长期欠账 | 「必须是映射」判据提升为 `_require_mapping` 唯一产地（A77 终极形），运行时两侧与校验面回放段**三处共引**（运行时行为与文案零变化，实测逐字相同）；校验面对非 dict 条目真调产地取文案、不再静默。新守卫 6 例：四档判决 + 两面文案逐字同源 + default 豁免不叠报 + 合法条目零噪声 + 产地唯一计数（config 源里该文案恰 1 处）+ 校验面不抄文案；变异自证：删新分支 2 红（分支承重被证）、sha256 逐字节还原；既有一条按旧口径钉「零错误」的用例按红字重述（立意不变：不叠噪声，收紧为一条对的错）。config.py 末尾追加零位移、validator 插行区间在全部活行引用之后 ⇒ A184 / L79 / l97 / l98 各棘轮全部原地绿（全量门禁证实） | S |

| B227 | **已关闭（L157，收口第一本账 A139）**：`KNOWN_FIELDS`（24 行）+ `MODEL_ENTRY_FIELDS`（5 行）的判决维（min/max/choices/item_choices）与各节 `__post_init__` 的运行时判据是**两份实现**——加一节/一键要改两处，L82 补静态面时每行都要重抄「类型·下界·上界·清单」四件事 | 判决维整批撤出两张规格表（只留类型层+形状层 type/items/non_empty/non_blank/required/nullable/renderable），改由 `_validate_replay_sections` 逐键 `dataclasses.replace(默认底, 单键覆盖)` 回放各节 `__post_init__`（判决权威只住运行时一处，A77 终极形）；类型层由规格走查先短路、按路径（含元素档 key[0]、节级同文案）去重；跨键窗口 rag + 权重由既有 `_validate_rag_window`/`_validate_quality_weights` 专管不重探。38 条既有文案断言按红字逐条重述（值过大/值过小→不大于/不小于/比例+带常量端点、choices 维→运行时真拒），测试-only 无新用例 | M |

| B226 | **已关闭（L156，收口第一本账 A46②）**：`StreamConfig` 三键（`chunk_size` / `buffer_size` / `max_memory_mb`）零判据——读侧 `StreamReader` 的 `chunk_size` 形参早接 `require_count(minimum=1)`，配置类这边 0 / 负数 / 字符串 / bool 全静默构造成功，同一档下界住两处（A77 形状） | 按「不发明语义、不删公开类」取判据档：`StreamConfig.__post_init__` 三键接 `require_count(minimum=1)`（chunk_size 与读侧形参同源同档，两侧同判由新守卫钉死；buffer / memory 无读侧权威，下界 1 按「正数量」读法记档）。本类无产品消费方（A46② 留档口径），判据买「写时就报」。A46 三子项自此全清（① L152、③ L41、② 本轮）。新增 3 例（无新参数化位，l97 / l98 原地不动） | S |

### L156（2026-10-01）— B226 立项 + 关闭：A46② 收口（StreamConfig 三键接判据，第一本账 A46 三子项全清）

- **真缺陷（A46② 原框，L35 立 / B222 留档）**：`StreamReader` 的 `chunk_size`
  形参自 L51 起就按 `require_count(minimum=1)` 判负，而 `StreamConfig` 的
  `chunk_size` 字段零判据——同一档下界住两处且配置侧全盲：
  `StreamConfig(chunk_size=0)` 静默构造成功（假零家族 L144 同式），
  `buffer_size` / `max_memory_mb` 同样无档。本类无产品消费方（0 使用者，
  只算 SDK 面），缺陷形状是「配置类是死旋钮 + 判据缺面」而非现行故障。
- **处置（三档拍一）**：① 删类 = 公开面破坏变更（SDK 导出类，跨轮不可逆），
  不做；② 发明消费语义（把三键喂给 StreamReader）= A46 留档明文的「不发明
  语义」红线，不做；③ 取判据档（与 L154 对无读者节同一口径）：
  `__post_init__` 三键接 `require_count(minimum=1)`，chunk_size 与读侧形参
  同一字面下界、两侧同判由新用例钉死，buffer / memory 的 1 按「正数量」读法
  记档。
- **钉子**：test_streaming 新增 3 例（坏值 0 / 负 / bool / 字符串 × 三键、
  0 档假零专钉、两侧同档对账）；无新参数化位 ⇒ l97 / l98 / A184 / L79 各棘轮
  仅 BACKLOG_A_CLOSED 一格 +1（A46 行收口划格，先例同 L154）。
- 全量门禁：`7697 passed / 3 skipped / exit 0`（L155 的 7694 + 3 新例，无回归）。
  **B226 关闭，第一本账 A46 三子项全清**。


### L157（2026-10-01）— B227 立项 + 关闭：A139 收口（规格表判决维整批撤下，改逐键回放）

- **真缺陷（第一本账 A139 原框，L82 立）**：`ConfigValidator.KNOWN_FIELDS`（24 行判决维）
  与 `MODEL_ENTRY_FIELDS`（5 行）跟各节 `__post_init__` 的运行时判据是「哪几键要判、
  按什么形状判」的**两份实现**——界本身没抄两份（A77 由 is 共引守卫钉），但判决维
  （min/max/choices/item_choices）与运行时调用点各写一遍，加一节/一键要同改两处，
  L82 补静态面时每行都重抄四件事（类型·下界·上界·清单）。
- **修法（A139 记档的「候选修法」落地，三条「先拍」顾虑逐条兑现）**：判决维整批
  撤出两张规格表（只留类型层 type + 形状层 items/non_empty/non_blank/required/
  nullable/renderable）；新增 `_validate_replay_sections` 逐键
  `dataclasses.replace(默认底, 单键覆盖)` 回放各节 `__post_init__`，把运行时判据
  原样投影到静态面——判决权威只住运行时一处（A77 终极形）。
  - 顾虑①（文案位置会变 ⇒ 既有断言要重述）：38 条文案断言按红字逐条重述
    （值过大/值过小 → 不大于/不小于/比例、choices 维 → 运行时真拒 + 带常量端点）。
  - 顾虑②（YAML 形状错须类型层先短路）：规格走查的「类型错误」先报，回放只对
    「节在场且是映射」构造、值类型错时与规格层同路径去重，不叠 TypeError。
  - 顾虑③（无 `__post_init__` 节回放等于不判 ⇒ 收益边界）：A140 已于 L154 给
    那 11 节全补判据，回放对全 20 节都真判。
- **坑**：① 整批构造在第一个坏键上停手（六键写坏只出五声），改 `dataclasses.replace`
  **逐键探针**；② 跨键窗口（rag chunk_size/overlap）与权重三件套按整批终态回放，
  单键探针会把「单独合法、跨键非法」误判 ⇒ 由既有 `_validate_rag_window` /
  `_validate_quality_weights` 专管，通用回放 `cross_key_owned` 跳过那三键；
  ③ 去重要含元素路径（key[0]）与节级同文案（跨键回放报在节名上）两档。
- 全量门禁：`7697 passed / 3 skipped / exit 0`（测试-only 轮无新用例，38 条断言
  重述；A184 / L79 棘轮绿——本轮只动 config_validator.py，config.py 零改动无行位移）。
  **B227 关闭，第一本账 A139 收口**。


### L158（2026-10-03）— B228 立项 + 关闭：A85 族第三侧漏网（models 条目非映射档，判据提升为唯一产地三处共引）

- **真缺陷（新侦察撞出）**：`models` 条目写成标量 / 列表 / 数（`m1: [1]` / `m1: 'x'` / `m1:` / `m1: 42`）时，校验面 `validate_config` **零报错**——回放段的 models 循环对非 dict 条目静默 continue；同一份配置运行时 `load_config` 抛 ConfigError「必须是「键: 值」的映射」。症状 = 校验工具（CLI validate-config）报 is_valid=True、应用启动时才炸，A85 / A101 族「两侧不同判」的第三侧（A85 当年补运行时两侧时校验面未同步）。侦察排除了同族假嫌疑：非法 type / type 缺席 / 非 str type / 顶层 models 非映射 / 未知节未知键五档都已判（探针逐档实证）；HEAD~1 复跑同盲 ⇒ 长期欠账非回归。
- **修法（A77 终极形）**：判据提升为 `_require_mapping` 唯一产地（config 模块级，放文件末尾使既有行引用零位移），运行时两侧改调它（行为与文案零变化）、校验面回放段对非 dict 条目真调它取文案（try/except ConfigError 转 add_error）——三处共引后文案改一处三侧同步，校验面源码里该文案计数为 0（投影不抄）。
- **钉子**：新守卫 6 例（四面判决 / 文案同源 / default 豁免 / 合法零噪声 / 产地唯一 / 校验面不抄）；l75 一条旧口径用例按红字重述（零错误 → 恰一条对的错，立意「不叠噪声」保留）。变异自证 2 红 + sha256 还原（Temp 的 l158q 工件）。
- **棘轮**：config.py 末尾追加 + validator 插行区间在全部活行引用之后 ⇒ A184 census、L79 两桶 MEASURED 与 BACKLOG_A_CLOSED、l97 / l98 全部原地绿（无新参数化位、无行号位移、记账文本零 token 形状）。
- 全量门禁：`7703 passed / 3 skipped / exit 0`（L157 的 7697 + 6 新例，无回归）。
  **B228 关闭**。


### L159（2026-10-03）— B229 立项 + 关闭：A85 族第四侧（悬空 default_model 校验面零反馈，文案提为唯一产地 + warning 档两翻定档）

- **真缺陷（L159 侦察撞出）**：`models.default` 指向不存在的条目时，校验面零错零警；运行时消费点（get_model_config，pipeline 构造必经）抛可行动的 ConfigError「模型 X 未配置。可用模型: [...]」。侦察先排除了探针噪声：后端工厂第一形参是 ModelConfig，裸 AttributeError 是探针把 AppConfig 传错了位——产品链无此缺陷。
- **定档两翻（本轮主叙事）**：v2 按两侧同判口径判 **ERROR** → 同族合跑 56 例当场红，实证「悬空 default 不是必炸」（显式按名取模型的调用方完全不受影响；测试底座大量使用只带 default 的形状）⇒ ERROR 是过度收紧的伪判据；v3 降 **warning**（「值不会生效」族先例同档），底座变绿。教训入账：**消费点才判的引用完整性，校验面投影时档位必须跟着消费点的触发条件走，不是跟着错误类型走**。
- **修法**：文案提为唯一产地（config 模块级、文件末尾追加零位移），消费点与校验面共引；校验面悬空档真调产地产 warning（is_valid 不受影响 = CLI 退出码不变）。
- **钉子**：新守卫 6 例；两处既有断言按红字重述；变异自证 1 红 + sha256 还原。
- 全量门禁：`7709 passed / 3 skipped / exit 0`（L158 的 7703 + 6 新例，无回归）。
  **B229 关闭**。


### L160（2026-10-03）— B230 立项 + 关闭：A182 逐面收第二面（DiskCache 元数据写边收原子写，缓存值写边按判据口径有据豁免）

- **侦察路径**：A182 原文留了「按有并发读者逐面收」的既拍路线（L99 走过第一面），并点名「缓存写」为待判类之一 ⇒ 本轮判 cache.py 的两处写边：读侧行为对账后**一处必收、一处有据不收**——逐面收的粒度由判据决定，不是凑数字。
- **必收的形状**：`_load_metadata` 无 except 且在 `__init__` 必调 ⇒ 半份元数据的窗口里任何第二实例构造当场崩（错误答案档）；修法 = 改调既有 `atomic_write_json`（L99 的工具，带同名并发替换重试）。
- **有据豁免的形状**：`set` 的缓存值边读侧 `get` 按未命中重算（`return None`）⇒ 重算即可恢复；但豁免是**条件性**的——挂在 get 读侧行为上，守卫把这条条件钉成机器（「get 必须按未命中处理坏 JSON」断言），行为变了当场红。
- **钉子**：新守卫 4 例；l99 棘轮 43→42 重钉 + 归因；变异 2 红 + sha256 还原；cache.py 是 CRLF，全程二进制写保真（Windows 文本模式翻行的坑不入）。
- 全量门禁：`7713 passed / 3 skipped / exit 0`（L159 的 7709 + 4 新例，无回归）。
  **B230 关闭**。


### L161（2026-10-03）— B231 立项 + 关闭：A182 逐面收第三面（VersionControl 索引两边收原子写，快照两边有据豁免）

- **侦察路径**：延续 L160 的逐面收路线，判 version_control.py 的四处计数写边。分判与 L160 完全同构：索引两边必收（读侧无 except + `__init__` 必调 = 错误答案档），快照两边有据不收（读侧响亮抛异常 = 可恢复档）——逐面收的粒度由判据决定。
- **判据口径的第二种豁免形状**：L160 的豁免挂「读侧按未命中重算」，本轮的豁免挂「读侧响亮抛异常」——两档都是 atomic_write 模块 docstring 明文的可恢复代价；守卫同样把豁免条件钉成机器（`load_version` 对坏内容必须抛 JSONDecodeError）。
- **修法**：两边改调既有 `atomic_write_json`；多实例并发初始化同一目录的窗口也顺带被同名并发替换重试覆盖。
- **钉子**：新守卫 4 例；l99 棘轮 42→40 重钉 + 归因；变异 2 红 + sha256 还原。
- 全量门禁：`7717 passed / 3 skipped / exit 0`（L160 的 7713 + 4 新例，无回归）。
  **B231 关闭**。


### L162（2026-10-03）— B232 立项 + 关闭：A182 逐面收第四面（BackupManager 索引两边收原子写）+ L79 首次三桶联动重钉

- **侦察路径**：延续逐面收路线，backup.py 与 version_control.py 逐字同构（读侧无 except 且 `__init__` 必调），分判零悬念：索引两边必收、快照与恢复输出有据豁免。
- **新记档格**：快照/输出两边的「同名覆盖写中途崩毁既有文件」窗口是 atomic_write判据口径（按读侧行为分档）没有覆盖的形状——它属于写侧数据保全而非读者可见性，扩不扩判据留给单独轮裁，本轮不擅动。
- **L79 三桶联动**：backup.py 插行使第一本账一条历史读数清单的 backup.py 行号格落非代码行 → 按先例降级名锚（「文件名（原 N 行，降级名锚）」）→ line_refs −1、file_tokens +1（文件名反引号 token 转桶，先例同形）、漂移桶 −1，三桶逐格重钉。
- **钉子**：新守卫 4 例；l99 棘轮 40→38 重钉 + 归因；变异 2 红 + sha256 还原。
- 全量门禁：`7721 passed / 3 skipped / exit 0`（L161 的 7717 + 4 新例，无回归）。
  **B232 关闭**。


### L163（2026-10-03）— B233 立项 + 关闭：A182 逐面收第五面（导出物交付面五边收原子写，工具面扩文本版）

- **侦察路径**：延续逐面收路线。export.py 四处计数写边的「读侧」按语义判——读侧在仓外（训练管线/用户），但**仓内有目录扫描读者**：`POST /api/data/export` 把导出物写进 `web.data_roots` 白名单目录，数据管理端点按 *.json 扫同一目录 → atomic_write docstring 点名的读者形状逐字命中（「按 *.json 扫目录的消费者在窗口里都会拿到截断的 JSON」）⇒ 四条 JSON 边必收；JSONL 文本边同缺陷面（半份 = 行截断）。
- **工具面**：新增 `atomic_write_text`（JSONL 不能走 JSON 序列化）；与 JSON 版共用「前导点 + tmp 中段」临时件命名——避 *.json 也避 *.jsonl 的目录扫描。
- **字节变化记档**：四条 JSON 边 indent=2 → 紧凑序列化；既有测试面全部解析断言无格式断言（全量门禁实证）。
- **坑两记**：① export.py 是混合行尾文件（CRLF 397 + LF 3），heredoc 转义层让锚点计数假 0，改脚本文件 + slice 定位替换收口；② l97 的 MEASURED_BLIND 是「no-literal-list 桶」口径，与本轮两支字面清单 parametrize 无关（盲面 101 不动），第一次复算用错口径（no-ids 站点数 332）差点乱钉。
- **钉子**：新守卫 12 例；l99 棘轮 38→34；l97 379/278/101；L79 两文档三桶 + FLOOR 下界逐格重钉。
- 全量门禁：`7733 passed / 3 skipped / exit 0`（L162 的 7721 + 12 新例，无回归）。
  **B233 关闭**。


### L164（2026-10-03）— B234 立项 + 关闭：A182 逐面收第六面（数据集变换三边收原子写）+ 账本单文件六格联动降名锚

- **侦察路径**：延续逐面收路线。dataset_ops 三边的输出链实链核实：API 侧 `dataset_tools` 路由的 merge/sample/split 端点路径全经白名单校验落数据根、CLI 侧 data_ops 命令同链 ⇒ L163 判据逐字命中（目录扫描读者 + 半份窗口）。
- **行为面侦察的两个假信号**：merge_files 断言失败是 MergeConfig 既有语义（默认去重 + 截断），非收边缺陷；split 方法实名 split_file。测试数据改大差异后过。
- **L79 六格联动**：dataset_ops.py 插行使账本单文件六条历史读数格整体落漂移桶（L36/L42/L150 三个时代的落点各一条/两条/三条），全部按先例降级名锚——单文件单轮顶歪六格是本守卫立档以来最大一批，三桶逐格重钉。
- **钉子**：新守卫 4 例；l99 棘轮 34→31；变异 2 红 + sha256 还原。
- 全量门禁：`7737 passed / 3 skipped / exit 0`（L163 的 7733 + 4 新例，无回归）。
  **B234 关闭**。


### L165（2026-10-03）— B235 立项 + 关闭：A182 逐面收第七面（依赖登记两边）+ 剩余 29 处逐面分档完成

- **侦察路径**：全量普查 31 处现量分布后逐面分档。dependency.py 两边必收（同构第四份）；ops.py 3 处与 cli/io.py 2 处核实为 CLI 报告一次性写（--output 面同步等待，无并发读者）豁免；versioning.py 2 处快照响亮档豁免。
- **分档结论**：剩余必收候选已点名（dataset_tools.py 路由边、export_enhanced.py 交付面、data_ops.py 命令输出边、pipeline/tracker/quality_report 等面待逐面核），豁免档全部有据记档——A182 的「哪些边算有并发读者按面分别判」从口径变成清单。
- **坑**：混合行尾文件（dependency.py CRLF 361+LF 3）变异脚本单形态锚点没命中（assert 变异 != 原文挡住），双形态探测重跑——变异自证不得因脚本笔误跳过。
- **钉子**：新守卫 3 例；l99 棘轮 31→29；变异 2 红 + sha256 还原。
- 全量门禁：`7740 passed / 3 skipped / exit 0`（L164 的 7737 + 3 新例，无回归）。
  **B235 关闭**。


### L166（2026-10-03）— B236 立项 + 关闭：A182 逐面收第八面（数据集工具路由落盘口）

- **侦察路径**：L165 分档点名的必收候选逐个核实——dataset_tools 的 `_dump` 是写盘变换类六端点共用落盘口（一处收口覆盖全部调用面）；data_ops 两边核实为CLI 报告面豁免；export_enhanced 交付面留给下一轮。
- **钉子**：新守卫 3 例；l99 棘轮 29→28；变异 3 红 + sha256 还原。
- **记账自检口径修正**（L163/L164 两次假红教训）：自检只查反引号 file:行号 形态与竖线数，「按 *.json 扫描规则」这类叙述不再误伤。
- 全量门禁：`7743 passed / 3 skipped / exit 0`（L165 的 7740 + 3 新例，无回归）。
  **B236 关闭**。


### L167（2026-10-03）— B237 立项 + 关闭：A182 逐面收第九面（增强导出九格式落盘口）+ 工具参数化 + 工具本体豁免显式化

- **侦察路径**：L165 分档点名的最后一个必收候选。`_export_json` 一处收口覆盖九个格式分支——交付面收边的杠杆格。
- **活选项的处置**：ensure_ascii/indent 不能丢（ExportOptions 契约）——工具加可选参数向后兼容，默认路径字节级零变化（守卫断言逐字节）。
- **l99 棘轮的两轮修正**：① 参数化把工具本体 json.dump 单行拆两支（+1 形状噪声）暴露了「工具本体一直在计数里」这个漏档——atomic_write.py 写的是自身临时件，普查排除并记档；② 棘轮 28→26（28 - 2 排除，L167 净收 1 体现在其中）。
- **坑**：账本 A143 与 l79 自注释里各有一份 export_enhanced 行号格——守卫文件自己的注释也是普查对象（A77 用在测试注释上的自我适用），两处都要降名锚。
- 全量门禁：`7747 passed / 3 skipped / exit 0`（L166 的 7743 + 4 新例，无回归）。
  **B237 关闭**。


### L168（2026-10-03）— B238 立项 + 关闭：A182 逐面收第十面（管线交付口 + 迁移落盘三边）+ 剩余候选读侧行为对账归档完成

- **侦察路径**：剩余 10 处候选逐个读侧对账——必收 3 边（pipeline×2 核心交付口、migration API 白名单面）；豁免 7 处全部有据（benchmark/faiss 读侧响亮、comparison/quality_report/visualize_enhanced 报告面）；**两个特殊形状记档**：tracker（写吞+读吞 None 完全体）与 quality_trend（检疫交互）超出 l99 收边类型，留给 fail-loud 行为变更单独轮。
- **L79 三桶重钉的曲折**：pipeline 插行使三条格漂移（一条 dead_line + 两条漂移桶转入），重钉时三桶数字算错两轮（274→273、1657→1658、140→141）——每轮都被「散文总数 == 现量」守卫当场抓回，守卫的精确相等纪律就是为这种时刻立的。
- **钉子**：新守卫 4 例；l99 棘轮 26→23；变异 2 红 + sha256 还原。
- 全量门禁：`7751 passed / 3 skipped / exit 0`（L167 的 7747 + 4 新例，无回归）。
  **B238 关闭**。


### L169（2026-10-03）— B239 立项 + 关闭：A182 逐面收第十一面（契约保持型收边：实验记录与趋势历史）

- **侦察修正**：L168 把 tracker 标为「记档待裁」，本轮深挖后发现读侧静默语义是 L66 时代有意设计且被三处测试钉成契约——**不是欠账是契约**。收边方式随之调整：只收写边（半份窗口消失），读/写异常语义一字不动（「不应崩溃」保持）。
- **连带适配一处集成测试**：NamedTemporaryFile 借名习惯（文件句柄保持打开）在 Windows 原子替换下必然拒绝访问——改为 mkstemp 建后即关。这是测试基建适配新语义，不是放宽断言。
- **钉子**：新守卫 3 例（重点验证契约保持）；l99 棘轮 23→21；变异 1 红 + sha256 还原。
- 全量门禁：`7754 passed / 3 skipped / exit 0`（L168 的 7751 + 3 新例，无回归）。
  **B239 关闭**。


### L170（2026-10-03）— B240 立项 + 关闭：A182 收口裁定（backup 两边）+ API 裸数值字段普查扫清

- **裁定**：「同名覆盖毁既有」档不扩判据、按语义自洽收边——备份/恢复即数据保全，旧写法下写中途崩会把旧备份/用户输出截成半份（备份操作毁掉自己保护的东西）。收边后 os.replace 保证替换前旧内容完好。
- **普查方法论**：18 处裸数值逐一探消费点判据与 HTTP 形态——比例三键/quality/dedup 全是「组件层已判 + ValueError 子类 → 400 可行动文案」的完整链（A77 一判据一处 + to_http_error 归一）。普查撞不出新缺陷本身是结论：这一面在 L48/L76/L82 配置判据族落地后已被组件层兜住。
- **A182 实质完成**：剩余 19 处全有豁免依据（CLI 报告面 / 响亮读侧 / 工具本体 / 值边钉死），分档清单住 l99 棘轮注释。
- **钉子**：新守卫 4 例；l99 棘轮 21→19；变异 1 红 + sha256 还原。
- 全量门禁：`7758 passed / 3 skipped / exit 0`（L169 的 7754 + 4 新例，无回归）。
  **B240 关闭，A182 收口**。


### L171（2026-10-03）— B241 立项 + 关闭：config_validator 判决维死码清除

- **发现路径**：coverage 全仓仅余 10 条未覆盖语句，其中 6 条在 config_validator——逐条对照 L157 撤维后的规格表（两表 min/max/choices/item_choices 残留 = 0）确认是**永久死分支**而非真缺口。删前先实锤承接：四档坏值经回放仍红（既有测试面全绿即证）。
- **删而不乱**：整块移除三处（min/max 块 / item_choices 分支 / choices 块），规格走查段从「类型 + 形状 + 判决维（死）」缩成「类型 + 形状（活）」——A77 判决权威只住运行时的终极形在规格走查段也落地。
- 全量门禁：`7758 passed / 3 skipped / exit 0`（删除零回归）。
  **B241 关闭**。


### L172（2026-10-03）— B242 立项 + 关闭：全仓未覆盖语句清零

- **余 4 条真实缺口**（死码删后）：atomic_write 两个写工具的 finally 清理（写失败临时件不残留——旧半份窗口防护的半边）、quality_trend 检疫备份名碰撞（同秒第二损坏不覆盖第一备份）、require_ratio_list 非数值元素档、validate_config 面 quality.weights 坏清单由专项回放承接（非死码）——全部补成行为面用例。
- **终态**：全仓未覆盖语句行 = **0 条**（语句级全覆盖，豁免清单里「真活代码」那一格清空；余豁免全是不可构造异常路径的记档）。
- 全量门禁：`7763 passed / 3 skipped / exit 0`（L171 的 7758 + 5 新例，无回归）。
  **B242 关闭**。


### L173（2026-10-03）— B243 立项 + 关闭：死码清扫第二轮（全仓无引用常数）

- **侦察路径**：L171 死码清除的轮次化。普查 3 候选，逐个经全 augmentor 树二次grep 核真——3 常数全部真死码（测试面零引用）。
- **坑记档**：普查文件列表口径（root + cli + api，缺子包）会漏报「子包有引用」与「全仓无引用」两类，死码删除前必须全树 grep 双保险。
- **记账补全注**：B242 行本应在 L172 轮写、当时漏写（L172 条目在账但 B242 行缺），本轮一并补齐——B 表行与 L 条目同轮齐整。
- 全量门禁：`7763 passed / 3 skipped / exit 0`（零回归）。
  **B243 关闭**。


### L174（2026-10-03）— B244 立项 + 关闭：ngram 热路径性能形状棘轮（相对 A/B）

- **立项动机**：L173 的 CLI 参数面探针（ngram-n / fuzzy / limit / offset 九档）全部实锤无缺口（判据在消费点齐配，bool 档也单独拒）——本轮负载换成既定待办的性能形状棘轮。
- **口径**（L77 先例 + A157 教训）：不钉绝对毫秒（环境性假红），钉**同机 A/B 相对比**——形状完好时比值稳定在 0.15–0.3（本机实测 0.174），阈值 0.7 留余量；实现被换回朴素形状（比值 → ≈1）或换得更慢（比值 ↑）都会当场红。
- **同分不拦形状**的教训入账：性能优化轮若只买「结果同分」不买「形状」，慢实现可以被无感换回来——同分用例与形状棘轮是两种守卫，各买各的。
- 全量门禁：`7765 passed / 3 skipped / exit 0`（L173 的 7763 + 2 新例，无回归）。
  **B244 关闭**。


### L175（2026-10-03）— B245 立项 + 关闭：搜索方法封闭清单下沉 SDK 层（未知 method静默回落 contains 清零）

- **发现路径**：L174 的 CLI 参数面九档探针全部实锤无缺口（判据在消费点齐配、bool 档也单独拒），唯一漏网是 method 维度——CLI 有 argparse choices、API 有 400、SDK 静默。三面对账出「一面松」的形状。
- **修法（A77 一条界一处）**：清单权威从 API 路由提进库层，API 改共引；库层 `require_choice` + None 专判（require_choice 的「未传回落默认」语义对必填参数不适用——同轮抓出并补判）。CLI 面自动吃库层判据（错误响亮）。
- **坑两记**：① parametrize 值位写 `list(SEARCH_METHODS)` 动态调用进 l97 盲面桶，字面化后 l97/l98 读数各自回稳（同轮修）；② 字面化前后各量了一次数读（381/279/102 对 381/280/101），混用会钉错——重钉前必须按**当前文件状态**复算。
- 全量门禁：`7779 passed / 3 skipped / exit 0`（L174 的 7765 + 14 新例，无回归）。
  **B245 关闭**。


### L176（2026-10-03）— B246 立项 + 关闭：策略旋钮封闭清单普查 + 质量报告 format下沉判据

- **普查方法**：L175 的 method 静默回落修完后，同型全仓扫「用户可见的字符串策略旋钮」——判据形状是「if/elif 匹配 + else 拒（列全合法取值）」。五个旋钮实探：search（L175 已修）/ sample / outlier / expander 四处的 else 拒实锤在场，save_quality_report 的 format 双分支**无拒**（唯一真缺口）。
- **普查棘轮形态**：修完缺口后把普查结论本身钉成守卫（五旋钮合体例）——普查不是做完就蒸发的，它变成「未来新增旋钮忘拒未知值 / 既有旋钮退化」的机械红线。
- 全量门禁：`7785 passed / 3 skipped / exit 0`（L175 的 7779 + 6 新例，无回归）。
  **B246 关闭**。


### L177（2026-10-03）— B247 立项 + 关闭：SDK 导出面死符号清扫 + 普查哨兵化

- **普查结果**：220 导出符号逐一核消费面，仅 1 个死别名。别名本身是历史重名避让的中间产物（converter 版与 export_enhanced 版同名，`__init__` 用别名分流），造出来 20 多轮无人消费。
- **哨兵设计**：普查一次就蒸发的教训（L173/L175 靠每轮人工重扫）——把「`__all__` 逐条有消费方」写成常驻测试，未来任何死符号入库当场红；语料库缓存化把代价从 77s 压到 22s（import + 建库）。
- **坑记档**：`__init__.py` 是 CRLF/LF 混合文件，行尾探测（sed 显示会吞 
，须用字节级 repr 定行尾）——本轮第一稿锚点因此没中。
- 全量门禁：`7787 passed / 3 skipped / exit 0`（L176 的 7785 + 2 新例，无回归）。
  **B247 关闭**。


### L178（2026-10-03）— B248 立项 + 关闭：A184 索引口径变更（未入仓用户数据目录 data/bak 移出索引，棘轮对磁盘运行期漂移免疫）

- **缺陷（棘轮自伤，快照不可复现）**：A184 的引用索引 `full_index()` 收下顶层两个**未入仓**用户数据目录 `data/` 与 `bak/`。`data/versions/` 每跑一次全量门禁就被 create_version 测试追加一个新版本目录（`data.json`/`metadata.json`/`current.txt`/`history.jsonl`），`bak/` 是手动备份——两者内容随磁盘态漂移。实测门禁起前 `ambiguous` 现量 330、钉值 328（+2）：`train_data.json`、`data.json` 一类散名因 data//bak/ 的同名件被灌进「一名多候选」而翻进歧义档，且**下次门禁跑完数据目录又变、快照永远回不到 328** ⇒ 棘轮自伤（与 L145 的 `.backups` 同族，那轮只排了快照目录、漏了 data/bak 这两个更大的未入仓数据面）。
- **修法（L145 先例 + 一层修正）**：把 `data`/`bak` 移出 A184 索引。**首版**用 `IDX_SKIP` 那种「任意深度同名目录」跳法，结果把嵌套的 `augmentor/data` 包（`image.py`/`cleaner.py` 等是**入仓产品码**、被 `web/src/types/api.ts` 等注释引用）也一并抹掉，`augmentor/data/image.py` 翻成 `dead_path` 红 2 条；**回退**改为 `_walks` 新增 `top_skip` 形参、**仅当 base==ROOT 时**匹配目录名，嵌套包不受影响。指向 data//bak/ 的引用改判「指向不入仓工件」两档（scratch / scratch_missing，语义正确——用户数据本就是工件而非案面）。
- **四桶逐格重钉（口径变更，按 L145 同式「动案面要重跑普查再改这里」条款归因）**：`ambiguous` 328→299、`runtime_ns` 30→0（30 条全是 `data/*` 示例路径、data/ 出索引后首段不在源码目录集）、`scratch` 730→763、`scratch_missing` 302→338；三档硬 0（dead_path/dead_line/symbol_dead）与 live 面（2488）一字未动；自检例 `data/xxx.json` 期望由 runtime_ns 改 scratch_missing（指向用户数据目录里的不存在文件 = 不入仓工件档，非死链）。
- **正面证据**：重钉后棘轮只量已入仓案面，`data/versions/` 再怎么随门禁追加、`bak/` 再怎么手动改，四个欠账桶读数都不再漂移（本轮全量门禁本身就会写一个新版本目录，门禁绿即证免疫）。
- 测试-only 轮零新用例（改的是守卫的索引口径与重钉常数 + 一条自检例的期望档），全量 7787 与 L177 同数、无回归。
  全量门禁：`7787 passed / 3 skipped / exit 0`（零回归）。**B248 关闭**。


### L179（2026-10-03）— B249 立项 + 关闭：A155① 收口（to_http_error 500 档不再转发异常原文）

- **真缺陷（安全泄漏，A155 原框待拍档，本轮按保守方向拍定）**：api/deps.py 的 to_http_error 最后一支把未分类异常原样 str(exc) 塞进 500 响应体。实测形状：OSError(13, 'Permission denied', 'D:/deploy/sensitive/secret.json') 走这一支，响应 detail 是 [Errno 13] Permission denied: 'D:/deploy/sensitive/secret.json'——把部署根目录的绝对路径与文件系统状态（磁盘满 / 权限缺失）一并回给客户端，是 A155 记档的「可被利用的探测信号」。A155 原框给两格候选（①固定文案+落日志 / ②脱敏白名单），本轮拍定取 ①——安全收紧方向、不引入易漏的脱敏白名单，且原文仍有服务端日志可查（不丢排障信息）。
- **修法（A77 一条权威一处）**：500 支改回模块级常量 INTERNAL_ERROR_DETAIL（api/deps.py 唯一产地）+ logger.exception 把原文留服务端日志；400 支（ValueError 可行动文案）与 404/JSONDecodeError 支一字不动，防修过头把客户端可行动信息也抹掉。
- **红字重述（A155 预测的「先跑全量看红几条」）**：tests/integration/test_api_dataset_system_tools.py 里凡钉死「500 回原文」的断言全翻——TestUnexpectedFailureBecomes500 五处 assert FAULT_MESSAGE in detail 改 == INTERNAL_ERROR_DETAIL（含反向护栏：原文不在响应体），TestDatasetImpact / evaluate 两处 500 用例同改；docstring「且带上原始信息」改「不外泄原始异常（固定文案 + 原文只进服务端日志）」。改前实测 24 failed（全为断言原文泄漏的用例）、改后全绿。
- **新守卫**：tests/unit/test_api_deps.py 新增 TestToHttpError500LeakClosed 3 例——① OSError 的 500 支不回部署路径/errno/文件名（逐词反向断言）；② 原文必进服务端日志（caplog 断言 ERROR 级记录在场，证明没被静默吞）；③ ValueError 仍 400 且原样回传（防修过头反向护栏）。
- **CRLF 坑**：tests/integration/test_api_dataset_system_tools.py 是 CRLF 文件，批量改写断言用字节级读写保真（LF-only 计数验零），避免文本模式翻行。
- **棘轮**：本轮账本与测试新增引用全用全路径 live 形（api/deps.py、tests/unit/test_api_deps.py 等），零散名 ⇒ A184/L79 各棘轮原地绿（全量门禁证实）。
- 全量门禁：7790 passed / 3 skipped / exit 0（L178 的 7787 + 3 新守卫，无回归）。**B249 关闭，A155① 收口**。


### L180（2026-10-03）— B250 立项 + 关闭：A155② 收口（路由裸 500 全面普查收边，A155 全清）

- **立项**：L179 修完 to_http_error 这一份权威后，A155 原框点名的另一半「40 处路由裸写 detail=str(e) 绕过归类器」仍未收——修的是象征不是面。实普 api/routes 十个路由文件，裸 500 detail=str(异常) 共 **37 处**（quality 9 / version 8 / data 5 / export 4 / augment 3 / config 3 / audit·leakage·multimodal·privacy 各 1），A155 记的「40」含 3 处已走 to_http_error 的旧计数，真裸写 37。
- **修法（A77 一条 500 泄漏判据住一处）**：新增 raise_internal_error 助手（api/deps.py，与 to_http_error 500 支同一口径：原文落 logger.exception、客户端拿 INTERNAL_ERROR_DETAIL，区别是直接 raise 供 except 分支调）；37 处裸 500 全部改调助手（十路由 import 补齐，字节级保 CRLF/LF 行尾）。404（版本不存在/文件不存在）与 400（非法组件）等非泄漏档一字不动。
- **红字重述**：tests/integration/test_api_route_500_branches.py 8 处（fuse/scan/batch export/formats/create/rollback/data export/gate boom）+ test_api_data_500_branches.py 1 处（磁盘读失败）的「500 回异常原文」断言改 == 固定文案；改前这 9 例红（断言泄漏），改后全绿。
- **新守卫 2 例**：① raise_internal_error raise 500 + 固定文案、OSError 部署路径/errno 不进 detail、原文进 caplog；② **静态形状棘轮**——api/routes 下裸 500 detail=str(异常) 一行回流即当场红（把「37 处已清」钉成机械红线，谁改回转发原文立刻红）。
- **A155 全清**：to_http_error（L179）+ 37 裸 500（本轮）+ 非泄漏档（404/400 不动）三格闭环；A155 自此关闭。
- **棘轮**：全路径 live 形引用 ⇒ A184 棘轮原地绿；**L79 漂移桶 145→146 逐格重钉**（十路由补 raise_internal_error 的 import，config/data/export 三个多行块各 +1 行使产品行号下移、账本一条行号格翻「指向真代码行但非所名」，按 L175 先例重钉，line_refs 270 / file_tokens 1661 未动）；全量门禁证实。
- 全量门禁：7792 passed / 3 skipped / exit 0（L179 的 7790 + 2 新守卫，无回归）。**B250 关闭，A155 全清**。


### L181（2026-10-04）— B251 立项 + 关闭：A172 收口（泄漏 fuzzy_threshold 区间判据 (0,1] 三面共引）

- **真缺陷（A172 原框待拍档，本轮按 fail-loud 方向拍定）**：leakage 的 fuzzy_threshold 在 SDK 直构 / API 请求模型 / CLI 三面**零区间判据**（请求模型裸 float=0.8 无 gt/ge、LeakageDetector 构造器不判、CLI 裸 float 位置参）。Jaccard 下界语义可用区间 0<t<=1：t<=0 时**每条**非精确样本都判泄漏（干净测试集报 100% 泄漏，网络面即可造出假读数）；t>1 反向永远报不出近似泄漏（静默关闭）。
- **修法（A77 一条判据一处）**：判据权威住构造器（augmentor/leakage.py）——显式 None 专判（必填旋钮无未传回落语义）+ require_ratio(0,1) 吃类型/NaN/bool/越界 + **下界取开**（`<=0` 拒假零家族 L144 同式，0 档把干净数据全判泄漏）。detect_leakage 模块工厂与 CLI 面（main 的 except 转「错误:…」+exit 1）经它自动吃到同一档；API 请求面补 Pydantic Field(gt=0, le=1) 边界 422（实测 0.0/1.5/-1 均 422、0.8 放行 404）。
- **legacy 守卫翻转（A172 原框「那条 legacy 守卫要从钉住无意义判决改成钉住拒绝」）**：tests/unit/test_leakage_counts_l95.py 的 test_non_positive_threshold_keeps_legacy_verdict 按红字翻转为 test_non_positive_threshold_is_rejected（t<=0 构造期即 DataValidationError）；正区间 0.3/0.7/0.9 逐字段一致档（test_verdicts_match_brute_force）一字不动防修过头。
- **新守卫 11 例**：合法 (0,1] 四档（0.1/0.8/1.0/int 1）放行 + 坏形状 9 档（0.0/-0.5/1.5/2.0/None/True/False/NaN/'0.8'）全拒 + detect 委托 + 防修过头（0.5 正常出报告）。
- **棘轮**：全路径 live 形引用 ⇒ A184/L79 各棘轮原地绿（leakage.py 插行使 0 条活行引用顶歪、全量门禁证实）。
- 全量门禁：7803 passed / 3 skipped / exit 0（L180 的 7792 + 11 新守卫，无回归）。**B251 关闭，A172 收口**。


### L182（2026-10-04）— B252 立项 + 关闭：A147 收口（流式单值文件 total_input/processed 分叉记档钉死）

- **真缺口（既有分叉，L86 起逐形状钉住但用户面未写明）**：streaming.py 的 read_chunks 与 _count_items 对「整个文件是单个 JSON 值」的读数分叉——一份只写 {"a": 1}（单行合法 JSON 对象、非数组）的文件，_count_items 走单值支计 0 条（它数的是数据项、单值不是数据项），而 read_chunks 逐行 json.loads 成功产出 1 条 ⇒ process() 报 total_input=0 / processed=1。L86 已在测试面逐形状钉住 0/1 两格，但用户看到两读数不一致会当 bug。
- **定档（A147 候选③）**：三格候选里 ① 单值算 1 条、② 读取循环拒收 都是**对外读数变更**（会动 API system_stream 与 CLI 报告文案、既有断言），本轮不动；取 ③「原样承认并写进文档」——把分叉写明成契约，读者读到时不当 bug。
- **落点**：augmentor/streaming.py 的 process() docstring 加「单值文件分叉」note；tests/unit/test_stream_single_pass_l86.py 新增守卫 1 例（total_input==0、processed==1、两者不等，两侧各自钉死）——谁把任一侧顺手修了都要先翻这一格并重新对账。
- **棘轮**：docstring 无散名引用、新守卫非参数化位 ⇒ A184/L79/l97 各棘轮原地绿（全量门禁证实）。
- 全量门禁：7804 passed / 3 skipped / exit 0（L181 的 7803 + 1 新守卫，无回归）。**B252 关闭，A147 收口**。


### L183（2026-10-04）— B253 立项 + 关闭：A130 收口（哈希占位符 14 条历史欠账永久记档，第一本账 A130 关闭）

- **立项即处置**：A130 是「上一轮哈希必须当轮回填」的欠账——L58..L71 共 14 条占位符从未回填，L79 起已钉成精确相等棘轮（PLACEHOLDER_CEILING=14，只降不升）。原框留三格待拍：① 只回填哈希（哈希可从 git log 现取，但那几轮本账的 numstat 与当时秒数已不可重算 ⇒ 留「半条已填」新形状）；② 整段重述（直接改历史账）；③ 不动。
- **拍定取 ③「不动、永久记档」**：①② 都碰历史，③ 把 14 条钉成终态欠账（棘轮只降不升，将来谁清了它要留一句「谁清的」，机械通道已在 L79）。这是「历史欠账不改写、只记档 + 用棘轮锁死不再新增」的既定纪律（同 L120 对 scratch 两档的记档口径）。
- **落点**：tests/unit/test_doc_line_refs_l79.py 的 test_only_the_current_round_leaves_a_hash_placeholder docstring 订正（原文「加本轮 = 15」是 L79 当时读数、现稳态已回填本轮占位符故恰 14）+ 记 L183 拍定；第一本账 OPTIMIZATION_LOOP.md 的 A130 行就地标「已关闭（L183）、14 条永久记档」。
- **零新用例**（文档 + docstring 订正），L79/A184 各棘轮原地绿（首账改动全为散文、零新反引号文件引用，全量门禁证实）。
- 全量门禁：7804 passed / 3 skipped / exit 0（与 L182 同数，无回归）。**B253 关闭，第一本账 A130 收口**。


### L184（2026-10-04）— B254 立项 + 关闭：A138 收口（可写四节 14 键读者分档记档 + 回显面机器钉死）

- **真缺口（契约面承诺、实现面不消费，A138 原框待拍档）**：export/vector/rag/multimodal 四节 14 键 L82 起全接判据（写坏即 400），但只有 3 键有行为读者（export.default_format、rag.chunk_size、rag.chunk_overlap）；余 11 键里 6 键仅被 GET /api/config 原样回显、5 键连回显都没有（无处消费）。用户按 docs 配 vector.backend: faiss 会以为向量库在用 faiss，而真正的后端入口是调用参数（vector.build_vector_db 的 backend 形参 / API 请求体），不是配置键——契约面与实现面的落差没有写明。
- **定档（非破坏面收口）**：候选①（把 ②③ 档接到消费入口）属功能增强、要先拍「配置键与请求参数谁是权威」；候选②（从文档与回显里降级）属对外破坏面——均不在本轮。本轮取**记档 + 机器钉面**：把三档分类写成权威清单，并钉成「③ 档 5 键不出现在 GET /api/config 回显」的机械红线。
- **落点**：docs/API.md 新增「可写四节的读者分档（A138）」小节（三档表 + 权威口径 + 非破坏面说明）；tests/integration/test_api_config_extended.py 新增 TestConfigEchoReaderSplitL184 守卫 2 例（③ 档 5 键不回显 + ①② 档 7 键既定回显面不得被摘）。
- **棘轮**：A184 新增引用全用全路径 live 形（augmentor/pipeline.py、augmentor/cli/commands/quality.py、tests/integration/test_api_config_extended.py）、①②③ 键为点号键名（无文件扩展名、不进 REF）⇒ A184/L79 各棘轮原地绿（全量门禁证实）；API.md 不在 L79 两文档扫描面、只进 A184 语料面。
- 全量门禁：7806 passed / 3 skipped / exit 0（L183 的 7804 + 2 新守卫，无回归）。**B254 关闭，A138 收口**。


### L185（2026-10-04）— B255 立项 + 关闭：A95/A122 合轮收口（模型凭证 ${ENV} 占位符展开语义钉成契约）

- **真缺口（A122 原框，与 A95 是同一件事的两半）**：模型条目三凭证键经 _resolve_env 解析 ${ENV} 占位符，未设置的环境变量**静默回落 ''**（与 None 等价，都是「没配凭据」的合法状态，由后端 ModelNotConfiguredError 在建后端时判）。A122 原框把「${ENV} 展开失败静默」与「三键两侧零形状判据」留待拍；A95 是「展开失败的出声口径」，两格合并处置。
- **定档（非破坏面收口）**：加载期「环境变量未设置」的条数随本机 shell 而变，接进加载声会让同一份配置在不同机器上行数不同（docs/API.md 3.x 既定口径），故**静默 '' 是刻意契约**、出声归 validate-config 的 warning 通道（A76/A84）。本轮不改加载期行为，把该语义**钉死**：_resolve_env docstring 写明四档；新增守卫 13 例钉「已设置取值 / 未设置静默'' / 坏形状原样 / 非串原样 / 合法占位未设置'' / 权威清单 == 三键」——谁把静默改成加载期 fail-loud（或把 '' 换成报错）都要先翻这一格并重新对账对外契约。
- **棘轮**：l97 普查 CALLS 382→384、READABLE 281→283 逐格重钉（两支 parametrize 字面清单进可读桶、盲面 101 不动、l98 随之回稳）；A184/L79 各棘轮原地绿（全路径 live 形引用，config.py 插行 0 条活行引用顶歪）。
- 全量门禁：7819 passed / 3 skipped / exit 0（L184 的 7806 + 13 新守卫，无回归）。**B255 关闭，A122/A95 合轮收口**。


### L186（2026-10-04）— B256 立项 + 关闭：cache TTL falsy 假零收口（set×2 + get×1 三处）

- **真缺陷（falsy 假零，L144/L147/L149 同族漏网点）**：cache.py 的 `MemoryCache.set`（:121）与 `DiskCache.set`（:268）写 `ttl or self._default_ttl`——显式 `ttl=0.0` 语义是「立即过期」，但被 falsy 读成「没传」而落到默认 TTL；`CacheEntry.is_expired`（:33）对 ttl=0 恒真，0.0 本是合法的「立即过期」档。同族第三处漏网在 `DiskCache.get`（:235）`if ttl and ...`——读侧把 ttl=0.0 读成「不过期」。三处一起让「立即过期」档位静默失效。
- **修法（判据 `is not None`）**：set×2 + get×1 共三处改 `ttl if/… is not None else`，零值保留为「立即过期」、None 才回落默认（set）/不过期（get）；与 L144 merge / L147 导出 / L149 sample 的假零修法同式。
- **守卫**：tests/unit/test_cache.py 新增 TestCacheTtlFalsyZeroL186 2 例（MemoryCache 与 DiskCache 各钉「ttl=0.0 立即过期 + None 回落默认」两侧对照，防修过头把 None 也当成 0）。
- **棘轮**：cache.py 为 CRLF 全程二进制写保真；无散名引用、非参数化位 ⇒ A184/L79/l97 各棘轮原地绿（全量门禁证实）。
- 全量门禁：7821 passed / 3 skipped / exit 0（L185 的 7819 + 2 新守卫，无回归）。**B256 关闭**。


### L187（2026-10-04）— B257 立项 + 关闭：benchmark delta_ratio 除零档自相矛盾收口

- **真缺陷（除零档自相矛盾）**：benchmark.py 的 compare_with_baseline（:240）写 `"delta_ratio": (delta / reference) if reference else 0.0`。当基准读数 reference=0.0（pass_rate / diversity 合法读成 0）且当前 current>0 时，delta/reference 是除零，`if reference else 0.0` 把它记成 0.0（「无相对变化」），但同条目状态已判 improved（delta>0 且 higher_is_better）⇒ 同一格自相矛盾：说「改善了」又说「相对变化 0」。
- **修法**：`if reference != 0.0 else None`——reference==0.0 时相对变化未定义（除零），记 None（与 :217 no_baseline 档的 None 哨兵同口径），机器面读到 None 即知「无相对基准」；非 0 照旧相除。下游 Markdown 报告只读 current/baseline/delta/status（delta_ratio 不进表格），无测试面读 delta_ratio ⇒ 零回归。
- **守卫**：tests/unit/test_benchmark.py 新增 TestDeltaRatioZeroBaselineL187 3 例（零基准 improved→None、零基准零当前 unchanged→None、非零基准照旧 0.8 防修过头）。
- **棘轮**：无散名引用、非参数化位 ⇒ A184/L79/l97 各棘轮原地绿（全量门禁证实）。
- 全量门禁：7824 passed / 3 skipped / exit 0（L186 的 7821 + 3 新守卫，无回归）。**B257 关闭**。


### L188（2026-10-04）— B258 立项 + 关闭：PiiSanitizer 构造器 falsy 假零收口（fields/patterns/placeholders 三处）

- **真缺陷（falsy 假零，构造器三处同型）**：privacy.py 的 PiiSanitizer.__init__ 三处 `x or default`（fields / patterns / placeholders）。显式空容器（`fields=[]` / `patterns={}` / `placeholders={}`）本是「零元素」的合法意图，却被 falsy 读成「没传」而悄悄套上默认：`patterns={}`（「我只要这几条自定义规则、其余不脱」）被读成「用全量 DEFAULT_PATTERNS」，脱敏范围被无声放大；`fields=[]` 被读成默认三字。
- **修法（判据 `is not None`）**：三处改 `x if x is not None else default`——空容器保留为「零元素」、None 才回落默认。与 L186 cache TTL 同族（falsy 假零收口）。
- **守卫**：tests/unit/test_privacy.py 新增 TestPiiSanitizerFalsyZeroL188 4 例（patterns={} 零模式不脱敏 + None 回落默认全量防修过头 + fields=[] 零字段 + placeholders={} 保留空表）。
- **棘轮**：无散名引用、非参数化位 ⇒ A184/L79/l97 各棘轮原地绿（全量门禁证实）。
- 全量门禁：7828 passed / 3 skipped / exit 0（L187 的 7824 + 4 新守卫，无回归）。**B258 关闭**。

### L189（2026-10-04）— B259 立项 + 关闭：验证预设封闭清单下沉 SDK 直构面 + `rules` falsy 假零

- **真缺陷①（falsy 假零）**：validation.py 的 DatasetValidator.__init__ `if rules:` 把 `rules={}`（「零规则」合法意图）读成「没传」，静默落 preset 面——strict 档传 {} 拿到 basic 规则面。判别探针：「hi」（短于 strict 档 min_instruction_length=5）strict 档必告警、零规则面必零告警，改前 {} 与 preset 无法区分。
- **真缺陷②（静默回落）**：未知 preset（拼错 strict/chat）回落 basic，规则面整档放宽不出声；API/CLI 面早有封闭清单收口，SDK 直构面独缺。
- **修法**：① `is not None` 判型；② 未知预设抛 DataValidationError 并列出全部合法值（PRESET_RULES 即清单权威，A77）；API 面 VALIDATION_PRESETS 改共引派生（400 判据不变）；CLI 面维持字面清单惯例、同一性由守卫钉。
- **守卫**：tests/unit/test_validation_preset_closed_list_l189.py 新增 10 例；test_validation.py 旧「未知回落 basic」例红字重述为拒收例。
- **棘轮**：l97 CALLS 384→385、READABLE 283→284（新守卫 5 档 parametrize 全字面）；A184 既有活行引用 validation.py:639 因 __init__ 插入 4 行注释档平移至 :697；L79 原地绿。
- 全量门禁：7838 passed / 3 skipped / exit 0（L188 的 7828 + 10 新守卫，无回归）。**B259 关闭**。

### L190（2026-10-04）— B260 立项 + 关闭：leakage min_examples 计数旋钮收口

- **真缺陷（计数旋钮静默错答族）**：leakage.py 的 LeakageDetector.__init__ 对 min_examples 零判据。负数 `[:-2]` 切片 = 丢掉最后 2 条——「要 2 条」读成「要 N-2 条」，内容相反不出声；None / 非整数 / bool 原样进 `[:n]` 下标（bool 在 Python 里是 int，`[:True]` = 要 1 条）。L144 起的计数旋钮 require_count 收口漏在这一格（同族 require_count 口径：0 = 「一条都不要」合法，须与「没传」区分开）。
- **修法**：构造器 `require_count("min_examples", min_examples, minimum=0)` + 显式 None 专判（L181 的 fuzzy_threshold None 专判先例，必填语义旋钮）；API/CLI 面未暴露 min_examples（grep 实证），只补 SDK 直构面。
- **守卫**：tests/unit/test_leakage.py 新增 TestMinExamplesGuardL190 13 例（合法 4 档 + 坏形状 8 档 parametrize 全字面清单 + 0 档语义钉死：精确泄漏 total_leaks≥1 但 leaked_examples==[]，统计照出）。
- **棘轮**：l97 CALLS 385→387、READABLE 284→286（盲面 101 不动）；A184/L79/markdown-structure 原地绿（B 表新行与相邻行须留空行——首轮门禁就撞出 GFM 表格判据红，补空行复绿）。
- 全量门禁：7851 passed / 3 skipped / exit 0（L189 的 7838 + 13 新守卫，无回归）。**B260 关闭**。

### L191（2026-10-04）— B261 立项 + 关闭：sampler recommend_seeds 问题类型面 O(k×n) 重复分析→预计算

- **性能形状（k×n 重复）**：recommend_seeds 内层循环对每条 item 重复调 _analyze_question_type——k 个覆盖不足档 × n 条数据，同一 item 被同一纯函数扫 k 遍（结果恒同）。改前 98 条语料实测：top_k=1 付 2n+91 次（=287）、top_k=20 付 2n+186 次（=382），差 95 次；修后两档同收在 3n=294 次（3 遍既有过法 + 预计算一遍）。
- **修法**：循环前预计算 `qtypes = [_analyze_question_type(...) for item in items]`（一次 O(n)），内层 `for i, item in enumerate(items)` + `qtypes[i]` 查表；语义不变（首匹配不变）。
- **守卫（形状棘轮，不钉绝对次数）**：tests/unit/test_sampler.py 新增 2 例——① 同数据 top_k=1 与 top_k=20 的调用数必须相等且 ≤3n（旧 k×n 形状下两档分叉、当场红）；② 首匹配 seed = items[90] 语义钉死（防预计算改出排序漂移）。变异自证：换回旧形 1 红，sha256 逐字节还原。
- **棘轮**：l121 的 `sampler.py 265->260` 分支弧 docstring 随插入 5 行平移 `270->265`（红字重述）；无新参数化位（守卫不参化）⇒ l97 原地绿；A184/L79/markdown 原地绿。
- 全量门禁：7853 passed / 3 skipped / exit 0（L190 的 7851 + 2 新守卫，无回归）。**B261 关闭**。

### L192（2026-10-04）— B262 立项 + 关闭：cleaner 规则名封闭清单下沉 SDK 直构面

- **真缺陷（静默跳过）**：DatasetCleaner.clean 的 `if rule_name in self._default_rules:` 对未知名静默跳过，拼错规则名 = 该规则无声不生效、清洗报告照常出（与 L175 搜索 method 静默回落 contains 同式）。CLI 面 parser.CLEAN_RULES（argparse choices）与 L189 的 API/CLI 收口先例一样早有 400/choices 判据，SDK 直构面（clean / clean_dataset / clean_batch_optimized）独缺。
- **修法**：clean() 入口补封闭清单判据——unknown 同时收「非 str 坏形状」与「不在 `_default_rules`」，拒并列出全部 11 个合法规则名（A77 权威住 `_default_rules` 键）；clean_dataset / clean_batch_optimized 经委托自动吃档。CLI 面维持「不 import 业务模块、字面清单」惯例，同一集合由新守卫钉。
- **守卫**：tests/unit/test_cleaner.py 新增 TestUnknownRuleNamesClosedListL192 5 例；tests/unit/test_partial_branches_l118.py 旧「未知跳过」例红字重述为拒收例 + docstring 勘误（假支自此不可达）。
- **棘轮**：守卫无 parametrize、无新增散名行引用 ⇒ l97/A184/L79/markdown 各棘轮原地绿（全量门禁证实）。
- 全量门禁：7858 passed / 3 skipped / exit 0（L191 的 7853 + 5 新守卫，无回归）。**B262 关闭**。

### L193（2026-10-04）— B263 立项 + 关闭：迁移规则 id 封闭清单下沉 SDK 直构面

- **真缺陷（静默丢弃）**：DatasetMigrator.migrate 的 `r.rule_id in rules` 过滤把未知 id 无声丢掉——全拼错时迁移照跑、`rules_applied` 是空数组，「迁移跑了个寂寞」而调用方不看该字段就发现不了（与 L175 搜索 method 回落、L192 cleaner 规则跳过同族）。API 面早有 400 白名单，SDK 直构面独缺。
- **修法**：① migration.py 立 `BUILTIN_MIGRATION_RULE_IDS` 常量（A77 单一权威，与内置三条规则逐一钉）；② `migrate()` 入口拒未知/坏形状 id 并列出全部合法 id（自定义规则经 add_rule 注册后合法，不误伤）；③ API 面 `MIGRATION_RULES` 改共引该常量（is 钉，L175/L189 共引先例），400 判据与文案不变。
- **守卫**：tests/unit/test_migration.py 新增 5 例 + 旧「未知跳过」例红字重述；常量与内置规则集逐一对上、未知拒（文案含全量清单）、坏形状 3 档、自定义 id 放行、API 共引 is 钉。
- **棘轮（行号位移房）**：system_ops.py 顶部插 1 行共引 import ⇒ 全文件行号 +1，OPTIMIZATION_LOOP.md 既有 6 处 `system_ops.py:` 活行引用（:147/:292×3/:397×2）与 docs/ARCHITECTURE.md 2 处（:147/:292）同步平移，L79 `code_but_no_name_match` 两格（账本 146、架构 34）随修引用**回稳而非升档**；l97 原地绿（守卫不参化）。
- 全量门禁：7863 passed / 3 skipped / exit 0（L192 的 7858 + 5 新守卫，无回归）。**B263 关闭**。

### L194（2026-10-08，L197 补记）— B264 立项 + 关闭：套件名封闭清单下沉 SDK 直构面

- **真缺陷（静默回落）**：`augmentor/auto_test.py` 的 `DatasetTestRunner.run_tests()` 对**显式未知套件名**静默回落到内置 default 套件 —— `_test_suites` 初始为空且没有注册入口，`if suite_name and suite_name in self._test_suites:` 不中就往 `create_test_suite("default", …)` 落。拼错的套件名拿到 default 套件的判决**不出声**（checkpoint 症状族语义漂移档；L175 / L176 / L189 / L192 / L193 封闭清单族同式）。API 面 `AUTO_TEST_SUITES` 早有 400 白名单，SDK 直构面（`run_tests` / `run_dataset_tests` / CLI `auto-test`）独缺这一格。
- **修法**：① `auto_test.py` 立模块级 `DEFAULT_SUITE_NAME = "default"`（A77 单一权威，字面 "default" 全仓不再有两份）；② `run_tests()` 入口按 `set(self._test_suites) | {DEFAULT_SUITE_NAME}` 判封闭清单，未知名 / 非 str 抛 `DataValidationError` 并列出全部合法名；`None` = 「未指定」仍走 default、显式 `"default"` 在注册前也合法、`create_test_suite` 注册过的自定义套件名照旧放行（三条防修过头对照同轮钉死）；③ API 面 `AUTO_TEST_SUITES` 由手抄 `("default",)` 改共引 SDK 常量（`is` 钉，400 判据与文案不变）。
- **守卫**：`tests/unit/test_auto_test.py` 新增 `TestSuiteNameClosedListL194` **6 例**；旧 `test_to_dict` 里靠静默回落才没红的套件名 `"test"` 改为 `"default"`。
- **棘轮（行号位移房）**：顶部插 1 行共引 import ⇒ `system_ops.py` 全文件行号 +1，`OPTIMIZATION_LOOP.md` 既有活行引用与 `docs/ARCHITECTURE.md` 2 处（`:147→:148`、`:292→:293`）随移（`29b715600`，纯引用漂移）；L79 `code_but_no_name_match` 漂移桶按「数字搬进常量」条款重钉 **146 → 149**（`2b2d8fb03`）；`line_refs 270 / file_tokens 1661` 未动；l97 原地绿。
- 全量门禁：7869 passed / 3 skipped / exit 0（L193 的 7863 + 6 新守卫，无回归）。**B264 关闭**。
- **本条的欠账**：L194 当时只写了循环日志块、漏写「已完成」清单行，直到 L197 才补（同型事故 L50 也发生过一次）。判据本身没有守卫 ⇒ 新立一条：见 L197 的「账本自洽」小节。

### L195（2026-10-09）— 子进程文本解码与 locale 解耦（缺陷由基线全量实测发现）

- **真缺陷（环境相依的假绿/假红）**：`tests/unit/test_partial_branches_l123.py` 的两处 `subprocess.run(..., text=True)` 没传 `encoding=`。父进程按 `locale.getpreferredencoding()` 解码子进程输出 —— 中文 Windows 是 **cp936**；子进程那侧的编码却取决于环境里有没有 `PYTHONIOENCODING`，而**本仓文档 `OPTIMIZATION_LOOP.md` 的基线恰恰要求测试命令带 `PYTHONIOENCODING=utf-8`**。于是子进程写 UTF-8、父进程按 GBK 解，subprocess 读线程 `UnicodeDecodeError` 把 `proc.stderr` 变成 `None`，`test_warning_when_key_absent` 直接 TypeError 崩。同一条用例在 Linux CI（UTF-8 locale）必绿 ⇒ **在项目自己写的那条测试命令下必红、在 CI 上必绿**。探针实测字节：子进程写出 UTF-8 的「未设置」，cp936 解到第 49 字节的 `0xaa` 即 `illegal multibyte sequence`。
- **修法**：两侧同时钉。① 子进程 `env["PYTHONIOENCODING"] = "utf-8"`（不设时它的字节形态随本机 locale 变）；② 父进程 `encoding="utf-8", errors="replace"`。既有正确先例三处（`test_config_unread_feedback.py` / `test_ledger_hash_slots_l98.py` / `test_excel_write_native_l85.py`），本轮补齐最后一处，另把两处原本靠「恰好是 ASCII」蒙混的调用点（`test_model_cache_optimization.py` / `test_comment_refs_a184.py`）一并补上。
- **新增守卫** `tests/unit/test_subprocess_text_decoding.py`（7 例，AST 推导不抄清单，承 L56 纪律）：**A** 文本模式（`text=True` / legacy `universal_newlines=True`）必须给 `encoding=`，无豁免；**B** 字节模式调用必须登记在 `BYTE_MODE_ALLOWLIST` 并写明「父进程不解码文本 + 用哪个具名编解码器 + 子进程编码怎么钉」；**C** 登记不许过期（指向的调用点已不存在或已补 `encoding=` ⇒ 红）；**D** 登记文件必须自己出现 `PYTHONIOENCODING`；**E** 扫描下限 13 防 glob 静默扫空。另有一条「AST 与 grep 双向互证」——两个方向都不许多或少（排除判据自身，它的 docstring 必然写出被检形状）。
- **新增行为例** `test_decoded_warning_is_the_real_sentence_not_mojibake`：单靠 `errors="replace"` 会把「未设置」换成一串替换字符，于是 `"未设置" not in stderr` **假绿**；必须连编码一起钉，替换字符一出现就红。
- **红→绿注入 6 模式**（一次性注入脚本，还原后 4 文件 sha 逐字节一致）：M1 整体退回缺陷态 **4 红**（2 行为例 + 守门 A/B）、M2 只退父进程编码（保留 `errors="replace"`）**4 红**、M3 删一条登记 **1 红**、M4 登记过期 **2 红**、M5 子进程编码不再钉 **1 红**、M6 无害对照 **0 红**。红名单逐条与预定 node-id 同名。
- **过程缺陷如实记（三处，都只属于注入脚本）**：① 首版把还原写最外层 `finally`，于是各模式补丁**叠加**而不是逐模式还原，第二个模式的锚点必然脱靶；② 更早一版崩在还原之前，把守卫文件留在缺陷态（登记块被删），手工复原后才继续 —— 自此还原必须在**同一个模式内**完成，且还原失败只打印不让它盖掉后续模式；③ 还原循环的解包顺序写反（`patches` 项是 `(key, old, new)`，写成了 `(key, new, old)`）。
- **A184 棘轮未被触动**：守卫 docstring 原引两个不入仓工件（一次性探针脚本、被 `.gitignore` 覆盖的真实语料文件），都会记进 `scratch_missing`。测试本身只用语料内数据、不依赖任何不入仓文件，故**删掉路径引用而不是去改常数** —— 改常数是「把欠账记下来」，删引用是「不欠这笔账」，后者更便宜。探针脚本本身不写路径名，理由是同一个。
- 全量门禁：基线 7872 collected（7868 passed + **1 failed**）→ **7880 collected，7877 passed / 3 skipped / exit 0**，覆盖率 99.89%。**+8 = 7 条守门 + 1 条行为例，两路独立闭合（numstat 与新例数精确对上）**。（`f531e62e3`）

### L196（2026-10-09）— 五处「同一件事算两遍」（代价轮，行为零变化）

- **共同形状**：同一个纯函数在同一轮里被完整算了两遍，第二次的结果要么直接丢掉、要么只为了拿一个 `len()`。这类改动的难点是**验收**——行为必须一字不变，所以每条都配**计数式预言机**而不是只断言「结果没变」（旧实现也算得出同样结果）。
- **五处**：① `sampler.recommend_seeds` 把 `analyze_coverage`（4 趟遍历）算了两遍（自己一次、`identify_underrepresented` 内部一次）⇒ 后者新增可选形参 `analysis`，不传仍自算，老调用方形状不变；② `dataset_ops.merge_files` 为拿一个 `removed_duplicates` 计数把整份数据又去重了一遍 ⇒ 抽出 `_merge_and_count()` 作为 `merge()` 与 `merge_files` 的**唯一实现**（不抄第二份合并口径），顺带把 `preserve_order` 的 if/else 收成一行（两个分支本来就只差末尾那次洗牌）；③ `outlier` 的 iqr 分支对同一份 `valid` 排两遍 ⇒ 共一次排序，`_quantile` 新增可选形参 `ordered`；④ `compare_enhanced.diff_datasets` 三个集合差 `list()` 与 `len()` 各算一次 ⇒ 三个变量先算好两侧复用；⑤ `dataset_ops.get_statistics` 长度清单一趟、唯一 instruction 集合又一趟 ⇒ 单趟同时累积。
- **`config=None` 的口径刻意不动**：产物侧照旧去重（`MergeConfig.deduplicate` 默认 True），报表的 `removed_duplicates` 仍是 0 ——「没传配置就不统计」是 L148/B218 起的旧行为，对齐它属行为变更轮，本轮不动并在代码注释里写明。
- **等价性不靠推理**（一次性探针脚本，两个独立测量链互证）：真实 6902 条房产客服语料，`PYTHONHASHSEED=0`，把改动前**整包内容**经 `git show HEAD:` 物化成另一个同名包当**独立预言机**（不是新代码的副本），双进程跑同一份 dump 后比字节 ⇒ **44872 字节输出逐键相同**，仅 3 个「新能力标记」按预期不同。计时 min-of-3：`recommend_seeds` **0.0484 → 0.0266 s（1.82×）**、`detect(method="iqr")` **0.0024 → 0.0013 s（1.85×）**、`get_statistics` 0.0013 → 0.0012、`diff_datasets` 0.0016 → 0.0014，**合计 0.0555 → 0.0321 s（1.73×）**。
- **新增守卫** `tests/unit/test_double_compute_l196.py`（20 例）：`analyze_coverage` / `_deduplicate` / `sorted` / `items` 迭代各必须**只发生一次**（旧实现必然多一次，退回去必须红）；另含新形参等价性（`analysis` / `ordered` 两形状同答）、`merge_files` 报表恒等式（其中「产物必须与 `merge()` 逐条一致」才是「内联没改口径」的真判据）、`diff_datasets` 的 `stats` 与清单同源。
- **三轮自我纠错，逐条如实记**：
  1. **探针首版物化错了包**：用 `git ls-files` 拿路径却 `REPO / rel` 读盘，物化出的「OLD」其实是**新代码本身** ⇒「完全一致」和那次计时全是自我印证（旧 0.0395 vs 新 0.0221 那个读数就是这么来的，作废）。改成真取 `git show HEAD:` 后重跑，并加两条自检断言（HEAD 不该有 `analysis` 形参、`_quantile` 不收 3 参）防再犯。
  2. **探针的离群段空转**：`_extract_values` 读的是**名为该字段的数值**，传 `field="instruction"` 得到全 `None`，`detect` 在排序之前就早退 ⇒ 一直在比两个空报表。换成真数值语料后 iqr 的受益才量到（1.85×）。
  3. **`shuffle_same_multiset` 一度 OLD=True / NEW=False，看着像回归**：把同一探针喂两个包根各跑 8 次，两侧同为 **1/8** —— 未播种的 `random.Random().shuffle` 让「洗后去重保留哪一份重复」本身随机（语料里有 instruction 相同而 output 不同的记录），**不是回归**。已把该键换成确定性判据（条数不变 + 洗牌确实发生），并把「探针键本身不确定就不许当等价性判据」立为纪律。
- **棘轮未被触动**：守卫 docstring 原引两个不入仓工件（一次性探针脚本、被 `.gitignore` 覆盖的真实语料文件），会使 A184 `scratch_missing` +2；测试只用语料内数据，故删路径引用而非改常数（同 L195 的取舍）。
- 全量门禁：7880 collected → **7900 collected，7897 passed / 3 skipped / exit 0**，覆盖率 **99.89%**（13240 语句 **0 missed**、3870 分支偏 19）。**+20 精确对上**；A184 / L79 / markdown / l97 各棘轮原地绿。（`63ef9368e`）

### L197（2026-10-09）— 文档数字对账 + 账本补记（docs 轮，产品码 0 改动）

- **动机**：L196 收尾后重扫文档，发现 6 处 A 级「契约面数字直接错」。全部先用代码实测再改，**不采信任何二手读数**：数 `api/routes/*.py` 的 `@router.*` 装饰器（70）+ `api/main.py` 的 `@app.get("/api/health")`（1）= **71**；进程内生成 OpenAPI 得 `paths 67 / operations 71 / tag 13`；`ExportFormat` 枚举与 `GET /api/export/formats` 实调均得 **13** 种；`include_router` **13** 次；`web/src/pages/*.tsx`（非测试）**11** 个、`App.tsx` **11** 条 `<Route>`；`test_api_event_loop_blocking.py` 的 `BLOCKING_SITES` 数到 **13** 条其中 quality **9** 条。
- **改动的六处数字**：README 端点 68 → **71**（两处）、README `dataset` 分组 12 → **14**、`docs/API.md` 端点总数 70 → **71**（两处）、`docs/API.md` `### quality（8）` → **（9）**（该节表格本来就有 9 行，是标题写错）、`docs/API.md` 导出格式示例 5 → **13**、`docs/ARCHITECTURE.md` router 11 → **13** 并补 `dataset_tools.py` / `system_ops.py` 两行、页面 9 → **11**（两处）、事件循环守门 12 → **13** 条且 quality 8 → **9** 条。
- **会漂移的数字改成「以枚举为准」的写法**（承第一本账 F-06 纪律）：README 的导出格式行与 API.md 的 `GET /api/export/formats` 示例都加了「清单由 `ExportFormat` 驱动、精确值以枚举/实时接口为准、不要从文档手抄」；并**顺手堵掉一个会让人改错的相邻陷阱** —— `GET /api/config` 回显里的 `export.formats` 是**五值出厂默认**，与这里的「全部支持格式」不是同一件事（探针实测两者分别为 5 与 13，第二本账 L184 也把前者归入「仅回显」档）。
- **账本自洽三件**：① 补 L194 的「已完成」行与循环日志段（L194 当时只写日志块、漏写清单行，同型事故 L50 也发生过一次 ⇒ 判据本身没有守卫，这是**第二次**同类，按纪律升级为一条待办而不是再忍一次）；② 在「已完成」段首加指针，说明清单只到 L113、L114–L193 以循环日志为权威，不再回填 80 行（两处数字长期不一致的风险高于补行的边际价值）；③ 全量读数从 7869 更新到 7897。
- **留作维护者决策、本轮只记不改的一处**：Python 版本口径三处不一致 —— README `3.8+`、`docs/README.md` `3.10+（开发验证环境为 3.13）`、`docs/DEPLOYMENT.md` 镜像与服务器段 `3.11`。实测代码侧无 3.10 独有语法（无 `match`、无内置泛型运行期注解），依赖上限也按 3.8 钉（`numpy<3.0.0` / `fastapi<1.0.0`），但两个验证解释器是 3.13.14 与 3.14.4、部署镜像是 3.11-slim ⇒ 「最低支持哪个版本」是产品决策，按「先记后问」处理，不擅自拍数。
- **新立待办**：**A265** 账本「已完成」清单与循环日志的进度一致性没有机器守卫（L194 / L50 两次漏写清单行都靠人眼发现）；**A266** 文档里的契约面数字（端点总数、tag 计数、格式清单、页面数）全靠人肉对账，本轮六处错全部是历史遗留 ⇒ 应收敛成一条从 `app.openapi()` / 枚举实推的门禁。
- 文档守卫全绿：`test_docs_markdown_structure.py` / `test_doc_line_refs_l79.py` / `test_comment_refs_a184.py` 共 116 passed；本轮只改数字与目录树，**不动任何「文件名 + 行号」形的行引用**，故 L79 与 A184 棘轮未被触动。（账本批即本条所在提交）

### L198（2026-10-10）— 统一 Python 版本口径为 3.10+ 并上机器守卫（A266 之外独立立案的那处）

- **立项来源**：L197 把「Python 版本口径三处不一致」立为「留作维护者决策、本轮只记不改」。本轮由维护者拍定，取 **3.10+**。
- **定这个数的四条实测依据**（不是拍脑袋、也不是抄任何文档）：① **产品码语法下限**：把 `augmentor/`、`api/`、`cli.py` 全部 `.py` 用 `ast.parse(source, feature_version=(3, N))` 从 N=8 起逐版本试，实测 **3.8 即全部通过**（`match`/PEP604/`slots=True`/`except*`/`typing.Self`/PEP695 全仓 0 命中）；② **测试套件真正的地板**：`tests/unit/test_micro_branches_l66.py` 有一处 `storage: str | None` 运行期注解 ⇒ 测试面需要 **3.10**；③ **依赖面**：`requirements-core.txt` 声明的区间在 3.8 上装得起来，但**本机实际装上的** `fastapi` / `uvicorn` / `requests` / `pytest` / `coverage` 自己都声明 `>=3.10`（`numpy 2.x` / `pandas 3.0` / `chromadb 1.5` 更是 `>=3.11`）；④ **部署面**：`docker/Dockerfile` 钉 `python:3.11-slim`。
- **为什么不拍 3.11+**：核心依赖在 3.10 上确实装得起来，说 3.11+ 是夸大。同时显式写出「装全套可选依赖需 3.11、部署镜像钉 3.11、验证环境 3.13.14 与 3.14.4」，读者拿到的是完整决策树而不是一个孤零零的数字。
- **守卫** `tests/unit/test_python_version_floor_l198.py`（11 例）：产品码语法下限**实推**且不许高于文档声称值；防空转例（另造一个只有 3.12 才有的 PEP695 `type` 别名，floor 必须真被抬到 12）；静态扫六种会抬高地板的形状；三处文档必须各自出现统一口径、过期读数一律不许复活；部署镜像钉的版本不许低于声称下限；全仓文档不许出现「Python 个位数次版本」这种更低的要求。
- **判据两次自我纠错**：第一版「不许有第四个版本号」用宽正则扫所有 `3.x`，实测把 `### 3.8` 这类**章节号**与 `Python 3.13.14` 这类**验证环境**全扫进来 ⇒ 收窄成只认 `Python` 后面跟**个位数次版本号**这一种形状；另一次是 DEPLOYMENT 的注记写成「3.10」漏了 `+`，被自己的 `CLAIM in text` 断言当场抓住。
- 红→绿注入 5 模式（一次性注入脚本）：README 退回 3.8+ **3 红** / docs 表行改 3.9+ **1 红** / 部署镜像降 3.9 **1 红** / 产品码引入 PEP695 **2 红** / 无害对照 **0 红**；还原后 5 文件 sha 逐字节一致。

### L199（2026-10-10）— B265 封闭清单族最后一轮：7 处「枚举形参静默换语义」收口

- **共同形状**：参数只与一个字面量比一次，比不上就**静默走默认分支**。七种变体全部实测过症状：`visualize_dataset(format=)` 走 text 支还按 text 落盘；`DatasetIndexer.search(method=)` 回落 contains 而 `index_used` 回显 `"regex:..."`（**结果主动谎报**，读结果的人无法自证）；`DatasetView.to_file(format=)` 静默写成 JSON 数组；`remove_duplicates(keep=)` 的 `"Last"` / `"firts"` 静默按 first 走；`get_dependencies(direction=)` 三条分支全不命中 ⇒ 返回 `[]`，用户读到的是「这个数据集没有任何依赖」这个**正面论断**；`StreamWriter(mode=/format=)` 把 JSON 数组写进声称为 csv 的产物；`GateRule(severity=)` 的 `"warn"` 被 else 分支**静默升为 error 级**门禁。
- **判据一律不新造清单**：清单一律立为模块级常数（A77 单一权威），`validation.require_choice` 只判「选中项存不存在」。`INDEXER_SEARCH_METHODS` **刻意不共引** `search_enhanced.SEARCH_METHODS` —— 后者有 5 项而 `DatasetIndexer` 只实现 3 项，照抄会让 fuzzy / regex 通过判据后继续静默回落 contains，等于把 L175 刚堵上的口子原样开回来（有一条用例专门钉这个真子集关系）。
- **立案不做的一处**：`add_dependency(dependency_type=)` 全仓没有任何分支读它（只在 `to_dict` 与依赖图里原样回显），且既存测试已在使用清单外的 `"derived_from"` ⇒ 收紧它是**对外契约变更**，不是静默回落修复。
- **守卫** `tests/unit/test_closed_list_knobs_l199.py`（62 例）：每站点三档坏值（未知串 / 显式 null / 非字符串形状）必须抛 `DataValidationError` 且文案列出**全部**合法值；每站点合法值一个不许被挡下；`index_used` 必须回显**实际执行**的方法；fuzzy / regex 必须被拒而不是静默降级；常数必须是非空无重复元组且是 `SEARCH_METHODS` 的真子集。
- **一次必须如实记的事故**：`visualize_enhanced.py` 的判据代码一度**只留下注释、代码行没写进去**，而 L199 用例同期报 62/62 全绿。我是靠注入脚本报「锚点命中 0 次」才发现源码里根本没有那段代码。随后做判决实验（临时摘掉判据再跑）：6 条 `[visualize]` 用例**确实全红** ⇒ 用例本身是响的，「全绿」与「判据缺失」不可能同时为真，说明中间发生过一次未被察觉的状态回退；注入脚本用**空串当还原锚点**、还原失败只打印不中断，而它自报的「sha 逐字节一致」不可信，是最大嫌疑。已把占位符改成沿用原缩进的注释、跑前先 `compile` 自检、替换后无法编译就整轮读数作废，并用**行为面**单独核验当前状态（7 站点 × 3 档坏值全部抛 `DataValidationError`）。教训：**注入脚本的还原必须由外部行为验证，不能信它自己的 sha 自报**。
- **棘轮逐格实测后重钉**：l97/l98 参数化普查 CALLS 387 → **393**、READABLE 286 → **287**、盲面 101 → **106**、盲面用例名 96 → **101**、档位 plain 51→52 / derived 33→36 / opaque 9→10（+6 = L198 两处 + L199 四处，后者有两处叠在同一用例上，l97 按装饰器计、l98 按展示键计）；L79 `line_refs` 270 → **268**、`file_tokens` 1661 → **1663** —— 本轮给 `validation.py`（+7）与 `streaming.py`（+21）顶部插判据档，账本 A92 与 A145 两行行号引用的被引行换位，`dead_line` 硬 0 档当场抓住 ⇒ 按 A127 / L180 / L193 既有口径降成名字锚点。**归因注释自己又造出 2 条死引用**（把被降级的旧行号原样抄进注释），已改为只描述不抄形状 —— 这是账本记过的第三次同型复现。
- 全量门禁：7970 passed / 3 skipped / exit 0，覆盖率 99.89%（**L198 与 L199 合用一次全量读数**：7897 + 11 + 62 = 7970，两轮各自的新例数可从 numstat 独立闭合）。

### L200（2026-10-10）— A266 收口：契约面数字从 `app.openapi()` / 枚举实推

- **动机**：L197 一次人肉对账抓出六处历史遗留错误，且这些数**没有一条会随产品变坏而报警** —— 端点加了一个、文档不跟上，全量测试照样绿。本守卫把它们变成实推值与文档断言值的相等关系。
- **判据**：端点总数（README 两处 + API 文档两处，四路都等于 OpenAPI 实推的 operation 数）；13 个 tag 逐个与实推相等（含「缺一个」与「多一个」两个方向，重复标题会炸）；导出格式示例逐个列全且顺序一致、个数等于 `ExportFormat` 成员数；README 那行必须既写对个数**又**声明「以 `ExportFormat` 为准」；router 模块数、前端页面数（ARCHITECTURE 两处 + README 一处）；事件循环守门家族改判三件自洽的事（句内枚举加起来等于自称总数、`/api/quality/*` 条数等于实推的 quality tag 数、点名的每个路径在 OpenAPI 里真实存在），另有一条单独判整份 `BLOCKING_SITES` 全是活路径。
- **为什么守门家族不拿清单总条数去对那句话**：该句是历史叙述、只描述第一个家族，而全量 `BLOCKING_SITES` 后来扩到含 dataset / system 两族（实推 25 条 vs 文档 13 条）—— 第一版就是这么写的，实测当场红，已改判据。
- **防空转**：每个断言都走 `_one()`（模式命中 0 次或多次都炸，文档写法变了就改判据，不许退化成恒真）；另有三例证明实推面真的取到东西（openapi paths / 格式数 / 页面与 router 数都超过 10）。
- 红→绿注入 7 模式 + 无害对照：README 端点退回 68 / API 文档退回 70 / quality 标题退回 8 / 导出格式示例退回 5 种 / router 退回 11 / 页面退回 9 / README 格式数退回 6 —— 逐模式恰好打在对应那条断言上；无害对照 0 红。全部还原后 sha 逐字节一致。
- 全量门禁：7985 passed / 3 skipped / exit 0，覆盖率 99.89%（7970 + 15）。

### L201（2026-10-10）— B266① 流式写入器不再把写崩的产物伪装成半份合法 JSON

- **缺陷（探针实测产物字节）**：`StreamWriter.write_chunk` 是「先写分隔符 `,` 再 `json.dumps(item)`」，而 `__exit__` 无条件补 `]`。一条记录序列化失败时盘上字节是 `'[{"a": 1},]'`，`json.loads` 报 **Illegal trailing comma**。用户先看到 `TypeError`，回头看文件还以为「写到一半了」—— 实际是**永久损坏**，且没有任何信号说明它非法。与 L55/L56 修过的「GBK 编码崩占掉退出码」同族：崩溃本身不可怕，可怕的是崩溃后留下的东西看起来是好的。
- **两处必须一起改，分开任何一半都还会坏**（注入 M1/M2 逐半退回分别红 1/2 条，正是这个理由）：① `write_chunk` 改成**先序列化再写分隔符**；② `__exit__` 在 `exc_type is not None` 时**不补**闭合括号。**口径**：这不是「失败也要产出合法文件」—— 崩了就是崩了；这里争取的是**崩掉的产物不许看起来像好的**：不补括号的 `'[{"a": 1}'` 一看就是截断，补了括号的会被 `json.loads` 当成「差一点」而被反复重试。`close()` 维持原语义（显式 close = 成功路径）。
- **守卫** `tests/unit/test_stream_writer_crash_l201.py`（9 例）：崩时无悬挂逗号、不补括号、异常照常传播、崩前已落盘的记录不被抹掉（截断 ≠ 清空）；正常路径五个反向守卫一个不许被挡下。
- **顺带如实记两件**：① `mode='a' + format='json'` 至今是坏的（第一次 `w` 已写过 `]`，第二次 `a` 直接把记录接在后面 ⇒ `'[{"a": 1}]{"b": 2}'`）—— 这是 B266② 的剩余 Scope，本轮不动追加语义（要先拍「追加 JSON 数组」的形态，属口径决策），把它写成一条**钉住已知限制**的用例而不是假装它合法，谁改追加语义这条先红；我第一版写成「追加后仍是合法 JSON」，实测当场红，那正说明它从来没合法过。② B266 原记录的 `context.batch_generate`「并行支吞失败」**经复核不是活缺陷**：`generate_multi_turn` 内部已捕获后端异常并降级返回空历史，并行支的 `except` 对后端错误不可达 —— 原记录是「读到 except 就算缺陷」没追到上一层，账本已订正。
- 红→绿注入 4 模式：M1 只退序列化顺序 1 红 / M2 只退闭合括号判据 2 红 / M3 两处都退 2 红 / M4 无害对照 0 红。**过程缺陷**：注入锚点最初按 `\n` 写，而 `streaming.py` 是**纯 CRLF** ⇒ 永远匹配不上；且 `write_chunk` 那段夹着新加的注释，手抄锚点两次脱靶，最后改成按唯一标记从文件现提区间。
- 全量门禁：7994 passed / 3 skipped / exit 0，覆盖率 99.89%（7985 + 9）。

### L202（2026-10-10）— B267 预览缓存键覆盖整份样本：第二个调用方曾拿到第一份数据集的产物

- **缺陷**：`PreviewGenerator.preview` 的缓存键只哈希 `sample[:5]`，而 `sample` 是 `items[:preview_size]` 的**全部**内容。于是「前 5 条相同、第 6 条起不同」的两个数据集撞键，第二个调用方拿到**第一个数据集的 `original_data` / `converted_data`**。这是 L123 修过的那一半的**另一半**：L123 把「只哈希样本」补成「格式 + 条数 + 样本」三因子，但样本那个因子本身被截断了 ⇒ 「数据」这个因子等于又弄丢一次。同族还有命中时把缓存对象**就地改完按引用返回**：同实例所有调用方共享同一个可变 `ExportPreview`。
- **修法**：键覆盖整份样本（外加 `len(sample)`，让「条数不同但前若干条相同」也分开）；命中时返回 `dataclasses.replace` 出的副本，只刷可能不同的 `total_items` / `preview_items` —— 键覆盖整份样本 ⇒ 命中即数据相同，`fields` 不可能不同，无需刷。
- **一处既有断言被本次改动打红，按惯例改写并写明旧断言为何错**：`test_preview_cache_key_l127.py` 的 `assert r2 is r1` 钉的正是「命中时把缓存对象就地改完按引用返回」。判据换成「内容相同、对象不同、总数已刷新」；该文件 docstring 里「键 = (格式, 条数, 前 5 条样本)」那句也一并订正。这一格正是账本 §2.7 那条「当一次改动同时触及实现与它的测试时，绿灯不再构成证据」的现场 —— 不更新它，L202 的修法会被自己的旧用例挡在门外。
- **守卫** `tests/unit/test_preview_cache_key_l202.py`（10 例）：同前缀不同尾不许撞键、真相同的数据仍要命中（防修过头）、条数不同不许撞键、L123 那半（格式换键）不许被改回去、命中不污染缓存对象、命中后计数按本次数据集算、`fields` 在命中后仍对；另三条既有功能反向守卫。
- 红→绿注入 4 模式：键退回截断态 1 红 / 命中退回共享对象 1 红 / 两处都退 2 红 / 无害对照 0 红；全部还原后逐字节一致。
- 全量门禁：8004 passed / 3 skipped / exit 0，覆盖率 99.89%（7994 + 10；L127 那条是**改写**不是新增）。（`1b74bab3b`）

### L203（2026-10-10）— A265 收口：账本「已完成」清单与循环日志的进度一致性守卫

- **为什么有这一支**：同一型事故发生了两次都靠人眼发现 —— L194 只写了循环日志块、
  漏写「已完成」清单行（L197 才补记），更早的 L50 也漏过一次。而「每轮回看循环机制」
  是用户定下的硬要求，那一整段可以消失、全量测试照样绿。
- **判据全部从账本推导、不抄清单**：`LOG` = 循环日志所有 `### L<n>` 标题解析出的轮次集合；
  `LIST` = 「已完成」段所有 `- [x] **L<n>**` 解析出的轮次集合；`BOUNDARY` = 指针句
  「本清单只到 L<m>」里的 m（**从文档现读**，不抄第二份）。三条不变量逐条对应一种
  真实失效形状：① `max(LOG) ∈ LIST`（L194 那次的形状当场红）；② `LIST ⊆ LOG`
  （清单不许指向一段不存在的日志）；③ LIST 在边界以上那一段必须是从 `max(LIST)` 一路
  连续到 `max(LOG)` 的**完整尾巴**（既抓漏最新一轮，也抓「最近几轮中间被跳掉一轮」）。
- **第一版判据想错了，实测当场红并暴露设计问题**：原写「LOG - LIST 必须是边界及以前的
  轮次」，实测发现**边界之前根本就没有「只在日志里」的轮次**（清单完整覆盖 L101–L113），
  而**边界之后**那一大片 L114–L193 才是刻意不回填的 ⇒「认边界」这个设计在当前账本形态下
  不起作用。已按上面三条重写，并把「LOG - LIST 非空」单独留成一条防空转（证明普查真的
  解析到了 L114–L193 那片区域，而不是漏掉半边后 0==0 绿）。
- **本轮自己就犯了一次要防的事**：写守卫的过程中发现 **L198–L202 五轮的循环日志段一直
  没记**——正是它要防的形状。已连同 L203–L210 一起补齐（见 L211 补记）。
- 红→绿注入 5 模式：漏写最新一轮清单行 2 红 / 中间跳掉一轮 1 红 / 编造一轮 2 红 /
  指针边界过期 2 红 / 无害对照 0 红。

### L204（2026-10-10）— README 补全 37 个 CLI 子命令索引 + 三个承诺功能的入口示例

- **实测缺口**：README 只以 `python cli.py <name>` 的形式覆盖 **21/37** 个子命令；更糟的是
  「功能特性」里承诺的三个能力——隐私脱敏、泄漏检测、就绪审计——在 CLI 一节
  **一个示例都没有**（承诺在 README 里、入口却查不到）。这类缺口不会让任何测试变红：
  docs 守卫只管形状与行引用，契约守卫只管 HTTP 端点。
- **改动**：① 「CLI 使用方法」开头加**完整命令索引表**：37 行，help 文案由
  `augmentor/cli/parser.py` **现提**而非手抄，并标出哪些命令下方有详细示例；权威出处写明
  是 `python cli.py --help`；② 补退出码三形状的口径说明，并指向由 parser 声明与 handler
  源码双向推导的 `test_cli_verdict_wiring.py`（不抄清单）；③ 新增「数据安全：三个承诺
  功能的 CLI 入口」小节，三个示例的参数逐个实测过。
- **守卫** `tests/unit/test_readme_cli_index_l204.py`（12 例）把「表 == parser」钉成集合
  相等，两个方向都不许多或少，help 文案也不许手抄后漂掉；三个承诺命令必须既有索引行又有
  可运行示例，且示例里的旗标必须真的被该子命令接受（防止文档教一个不存在的参数）。
- 红→绿注入 5 模式：表里漏一个子命令 3 红 / help 文案与 parser 不一致 1 红 / 删掉承诺
  功能示例 1 红 / 示例用不存在的旗标 1 红 / 无害对照 0 红。

### L205（2026-10-10）— B266③ 门禁分开「规则算不出来」与「算出来不及格」

- **缺陷**：`QualityGate.run()` 的 except 分支把求值失败的规则记成 `satisfied = False`，
  于是它和真不达标的规则进**同一栏** `failed_rules`。后果：`verdict=FAILED` 分不清是
  **数据不达标**还是**规则本身写错了**——而后者该修的是规则。
- **修法（B266③ 的处置前提：三面必须一起改，缺一面比不修更糟）**：① SDK `GateReport`
  新增 `errored_rules` 字段并导出到 `to_dict()`；② API `GateReportResponse` 同步声明该键
  ——否则 FastAPI 按 `response_model` 过滤返回值，新键会被**静默裁掉**（L27 记过的坑），
  「求值失败」反而看不见了；③ 守卫钉住三面键集相等。
- **契约守门第一次跑就抓到第三处手抄副本**（这正是它该干的）：`test_api_openapi_contract.py`
  里 `gate` 子对象的期望键集是**手抄的**五键，加了 `errored_rules` 后它报
  「多出: ['errored_rules']」。修法不是把新键抄进去，而是新增 `_gate_report_keys()` 从
  `GateReport.to_dict()` **实推**——与 L200 建的「契约数字不手抄」同源。
- **一处测试预期被实跑推翻并当场订正**：原打算把「指标键不在 metrics 里」也归到求值失败，
  实测 `evaluate` 对缺失键是 `return False`（仓内既有契约）⇒ 它进 `failed_rules`。
  L205 只改「抛异常」那一支的口径，不动这一支——动了它就是行为变更轮。
- 红→绿注入 3 模式：SDK 退回把求值失败当不达标 3 红 / 响应模型漏声明新键 1 红 /
  无害对照 0 红。

### L206（2026-10-10）— B266④ 失败明细从「只写不读」变成有出口（并补上串行支路的缺口）

- **缺陷两条**：① `_process_errors` 只在失败分支里写，**全仓零读者**——整批增强里哪些
  条目失败了、为什么失败，除了 logger.error 一行之外没有任何可编程出口，而那段注释明明
  写着「以便后续分析和重试」；② **跨调用累积不清理**——同一个实例跑第二次
  `augment_dataset()`，第一批的错误记录还在里面。
- **修法**：`__init__` 建好列表（不再惰性首建 ⇒「有读者」在结构上可见）、`augment_dataset`
  开头持锁清空、报告里加 `errors` 键（追加语义，既有键集不改）。
- **顺带修掉第三处，是动手写测试时实跑才发现的**：**串行支路的 except 压根不记明细**
  （`use_parallel=False` 或只剩一条数据时只打日志）⇒「失败明细有出口」这件事此前
  **只在并行档成立**。两支现在写同一形状的字典，且都持锁。
- **测试方法学**：并行支路的**行为**改用源码级判据覆盖而不是跑并行——并行支路的
  `_process_single_item` 既要干活又要往 `result_queue` put，stub 它就得连 queue 语义一起仿，
  那就是在测自己的桩而不是测产品（账本 §2.7「同源预言机是假测试」的同款）。
- **一处既有用例按惯例改写**：`test_pipeline_error_recovery.py` 的 L137 并发用例**故意不建**
  `_process_errors` 来验证惰性首建的并发安全；本轮移除了惰性首建，它因此显式补上初始
  列表。被测的性质（并发 append 不丢记录）一字未变。
- 红→绿注入 4 模式：报告去掉 errors 键 5 红 / 不清空列表 2 红 / 串行支路退回只打日志
  5 红 / 无害对照 0 红。

### L207（2026-10-10）— B266⑤ 管道阶段失败有出口 + 新增 run_strict()

- **缺陷**：`DataPipeline.run()` 的 except 分支只记 `result.error` 并打日志。
  `stop_on_failure` 是 `add_stage` 的**默认 False**，所以阶段 B 抛异常时 `current` 从未被
  重新赋值 ⇒ **阶段 C 拿到的是阶段 A 的输出**，整条管道少跑一环。而 `run()` 的返回值
  形状与条数和正常跑完完全一致，唯一信号是 logger.error 与 `self.results` 里那条 error。
- **为什么不动既有默认值**：`run()` 的「跑完能跑的阶段」在强健链路里正是要的（让强项继续跑）。
  本轮缺的不是这个语义，而是**「有阶段失败了」没有出口**。所以：① 新增 `PipelineOutcome`，
  `run()` 把失败清档写进 `self.last_outcome.failed_stages`；② 新增 `run_strict()`：任何阶段
  失败即抛 `PipelineError` 并 `from e` 链接原始异常；③ `run()` 的返回形状与条数、
  `stop_on_failure` 的既有效果一字未改（4 条反向守卫钉着）。
- `test_stage_c_sees_stage_a_output_not_stage_b` 直接钉缺陷本身：给每条数据打上「经过哪些
  阶段」的标记，b 炸了 ⇒ c 的输入是 a 的输出 ⇒ seen 里没有 "b"。
- 红→绿注入 3 模式：退回不记 last_outcome 5 红 / run_strict 退回静默跳过 2 红 /
  无害对照 0 红。

### L208（2026-10-10）— B266② 追加 JSON 数组改成拒绝：静默产出非法 JSON 是最坏的第三种

- **缺陷（探针实测产物字节）**：第一次 `w` 写出 `'[{"a": 1}]'`（合法、已闭合），第二次
  `a` 把 `'{"b": 2}'` 接在后面 ⇒ `'[{"a": 1}]{"b": 2}'`，`json.loads` 报 `Extra data`。
  一份非法 JSON，而调用方以为写成功了。
- **口径决策（三条候选都量过）**：① **拒绝**（采纳）——追加一份已闭合的 JSON 数组，任何
  写法都产不出合法 JSON；② 读回原数组、追加后整体重写：O(n) 读 + O(n) 写且要求原文件
  合法，违背「流式写入器」的初衷；③ 保持现状 + 文档写明（L201 那条 pinned
  known-limitation 用例已经做过）——「明确登记一个坏行为」仍然让调用方踩坑。
- **正确姿势**：要么一次性写，要么 `format='jsonl'`（天生可追加）。报错文案因此必须点名
  jsonl，否则用户只知道「不让干」不知道该干什么。
- **三处既有用例按惯例改写并写明旧断言为何错**：L123 的 `test_close_append_mode_does_not_add_bracket`
  原本钉的正是「追加 json 不闭合括号」——那恰是本轮判定为缺陷的行为；L201 的 pinned
  known-limitation 用例转成「钉住拒绝」；L199 的 mode × format 组合表分成三种可用一种拒绝。
- 红→绿注入 2 模式：把拒绝换成恒假条件 3 红 / 无害对照 0 红。

### L209（2026-10-10）— B265 备注收口：dependency_type 纳入封闭清单

- **立项时为什么没做**：L199 判它是「对外契约变更、需维护者决策」，因为**测试里用过
  清单外的值**（`derived_from`），怕收紧会打断既有调用。
- **本轮先量影响面再动手**：`dependency_type` 全仓**零分支读取**——只在 `to_dict()` 与
  依赖图里原样回显，没有任何 `==` / `in` 消费它；测试里用过的值只有 `derived`（正是源码
  注释声明的四个合法值之一），另一处 `derived_from` 与断言无关（那条用例断言的是
  「目标数据集不存在 ⇒ exit 1」）⇒ 收紧**不改变任何行为**，所谓「契约变更」的风险在
  实际可达面上为零。
- **修法**：`DEPENDENCY_TYPES` 常量（A77 单一权威）+ `require_choice` + `None` 单独先判
  （必填位置参数，`require_choice` 对 None 放行是「没传」的语义，显式传 null 是另一回事
  ——L176 `save_quality_report` 的同款处理）。并进 L199 的站点表（`SURFACE` 现 9 个），
  那套「三档坏值 + 合法值不误伤」的守卫自动覆盖它。
- **棘轮**：dependency.py 顶部插 9 行 ⇒ 4 处指向 `add_dependency` 的行引用被引行换位，
  `dead_line` 硬 0 档当场抓住 ⇒ 按 A127 口径降级为名字锚点。**归因注释自己又造出 1 条死
  引用**（把被降级的旧行号原样抄进注释）——这是账本记过的第三次同型复现，已改为只描述
  不抄形状。重钉：ARCHITECTURE `line_refs` 39 → 38、LEDGER 268 → 265。
- 红→绿注入 2 模式：摘掉判据 5 红 / 无害对照 0 红。

### L210（2026-10-10）— §3.3 收口：dataset/system 两组 27 个端点补人读字段说明

- **缺口规模**：**27 个端点操作**（dataset 14 + system 13）摊在 **25 个路径**上
  （`/api/system/backups` 同时挂 GET/POST、`/api/system/dependency/datasets` 同时挂两个 GET）
  ——比立项时写的 25 多两条：`dataset/impact` 与 `dataset/evaluate` 是 L8 之后新增的。
  这 27 个端点此前**只有总览表的一行**，而 API.md 的详细章节（1–11）按旧分组写，整整少两组。
- **这类缺口不会让任何测试变红**：端点总览有行、OpenAPI 有 schema、契约测试只验
  「响应键 == 模型键」，没有一个判据看「有没有人读说明」。
- **新增两节**：dataset 14 + system 13，每行四列（方法/路径/说明/**人读字段提示**）。
  提示的取舍标准是「按字面直觉理解会理解错」的那些，不是字段全集（文档已写明以
  `/openapi.json` 与 `response_model` 为准）。
- **守卫自身两次被实测抓出漏洞，都当场修了**：① 第一版把总览表与详细表混在一起匹配
  ⇒ 退化成「路径在文档里出现过就行」，**详细说明整段删光仍然全绿**（注入实测删行 0 红）。
  改成按「行列数」区分——总览表三列（4 个 `|`）、详细表四列（5 个 `|`）；用列数而不是
  在正则里数后续列，是因为 CRLF + 中文会让 `[^|]*` 的行为难猜（探针实测第一版正则
  匹配 0 行）。② 第一版过度主张「全量 71 个端点都要详细说明」。实测其他 46 个端点历史上
  就只在总览表里有一行 ⇒ 收窄到 §3.3 点名的两组，并加防空转证明总览层确实比详细层大。
- **一次误判的如实记录**：首轮全量跑出 2 个 `test_section_registry_l77.py` 的 teardown ERROR，
  残留物是 `coverage.PC-<日期>.<pid>.<随机>` 瞬态文件——coverage 多探针/多进程时序下的
  临时文件被 A168 残留守卫逮到。两种单独跑都不复现、干净重跑后消失 ⇒ 判为时序噪声而非
  本轮引入的缺陷，**没有去「修」一个不存在的问题**。
- 红→绿注入 3 模式：删 system 详细表一行 1 红 / 删 dataset 详细表一行 1 红 / 无害对照 0 红。
- 全量门禁：8075 passed / 3 skipped / exit 0，覆盖率 99.89%。

### L211（2026-10-10）— 前端消费面收口：五只未接线服务函数接进页面 + 守门语料面失查修复

- **缺口**：`api.wiring.test.ts` 的 KNOWN_UNWIRED 白名单长期挂着 5 只函数
  （`getVersion` / `getVersionData` / `getVersionHistory` / `getCheckpoints` / `visualizeData`）——
  封装层写好了、后端端点契约都在，页面却没有一个调用点；README 页面表早已承诺
  断点列表 / 版本历史 / 可视化图表三块，文档与现实不符。
- **接线取舍**：`GET /api/versions/{id}` 只回元数据、`/{id}/data` 回全量条目——点开一行
  不该默认付后者的代价，所以 Versions 页拆成「详情」「数据」两个显式入口。
  Augmentation 加「断点列表」卡（含刷新按钮与空态文案），Analysis 加「可视化图表文件」卡。
  新增页面测试 7 个（Versions 3 + Augmentation 2 + Analysis 1 + 语料守卫 1）。
- **本轮最大的发现不是接线，而是守门本身是瞎的**：第一次缺陷注入把 5 只函数从页面源码
  全部摘除后，接线守门照样 4 passed 全绿。根因：`import.meta.glob('../pages/*.tsx')`
  把 `*.test.tsx` 也扫进语料面，页面测试里 `vi.mock` 的替身字符串被判定成「已接线」。
  ⇒ 语料面剔除测试文件，并加一条专属守卫（断言语料面不含 `.test.tsx` 且原始 glob 确实含）
  防这个修复本身将来被改回去。重注入：五只全被抓住（`expected [ 'getCheckpoints', …(4) ]
  to deeply equal []`），恢复后转绿。
- **换机环境的如实读数**：本轮在一台新机器上执行，先重建了开发环境（venv 移到仓库外，
  避免污染 L79 文档行引用普查）。Python 全量门禁 `1 failed / 8051 passed / 10 skipped /
  16 errors，覆盖率 99.84%`。逐支归因后三类剩余项全部是本机环境、与 L211 改动零交集：
  ① 16 个 ERROR 来自 L98 账本标尺守卫的 first-parent 谱系断言——本机 main 的历史经 graft
  重建，origin/augmentor-opt100 的 1582 笔合并在 first-parent 链之外，`git_history()`
  只数到 429 笔低于 ≥500 下限而 RulerNotProven（尺子没证明自身可靠时报错，行为正确）；
  ② 1 个 FAILED 是 L68 微分支的 symlink rmtree 用例——本机无符号链接权限，
  `versioning.py` 按设计落 `current.txt` 回退，测试却写死符号链接臂；
  ③ 10 个 skip vs 基线 3：本机未装 chromadb / sentence-transformers 等可选依赖。
  ①② 各立一轮（L212 标尺谱系适配、L213 符号链接能力探测），均在下一轮以
  「不削弱断言强度」的方式修复。**Web 门禁本机实测：13 files / 119 tests passed
  （上轮 112 → +7），eslint --max-warnings 0 干净，tsc 通过。**
- 红→绿注入：摘除 5 只接线（修复守门前 0 红 ⇒ 实证守门失查；修复后 5 只全红）/
  把 `.test.tsx` 放回语料面 ⇒ 语料守卫 1 红 / 无害对照 0 红。

### L212（2026-10-10）— 账本标尺的谱系口径换成全祖先 topo-order

- **症状**：换机后首次全量门禁，`tests/unit/test_ledger_hash_slots_l98.py` 交回 16 个 ERROR，
  全部同一句 `RulerNotProven: first-parent 历史只有 431 笔，不像本仓的真值`（L211 块里记的 429
  是当时那笔提交之前的读数，两笔差在中间又落了几笔提交 —— 尺子读数按当下重量，不靠回忆）。
- **根因（实测不是推理）**：本机 main 的历史经 graft 重建（`4f0cf03d1`），origin/augmentor-opt100
  的那 1582 笔循环提交挂在**合并的第二父链**上，first-parent 一条直线走不到它们。
  同一棵树换成全祖先 `--topo-order` 现量 **2552 笔**。
- **为什么允许换**：这一族尺子最容易在换口径时静默失真，所以「六档读数与谱系口径无关」这一格
  **必须先自证再改**。换序前后逐档重量：带槽行 prog 15 / log 24、两侧一致 15、唯一哈希 69、
  「只有块」的轮次 9 格**集合相等**、主题白名单 4 格**集合相等** ⇒ 六档逐档复现，才动
  `git_history()`；下限报错文案同步改成「全祖先历史」，免得下一位读者按 first-parent 去理解那个数。
- **不削弱强度**：「读不到 git 就不给读数」的 `RulerNotProven` 语义一字未动；能红性由文件里
  两条既有自证用例承担（带已知答案的合成样本 + monkeypatch 把 R3 换成恒不报破口 ⇒ 自证必抛）。
  本轮复跑 19 passed。
- **顺带记一次自己差点踩的坑**：第一版探针读的是 `OPTIMIZATION_LOOP_2.md`，读数 4 / 0 / 4 与
  钉值全线不符，我差点把「探针读错文件」判成「尺子坏了」。真相是本守卫的 `LEDGER` 常量指向
  **第一本账** `OPTIMIZATION_LOOP.md`（`REPO_ROOT = parents[2]`）。记这一格的理由：读错文件
  同样能「证明」任何东西 —— 这条纪律对本仓所有普查型守卫成立。

### L213（2026-10-10）— 符号链接能力交叉对账：第一版不可证伪，撤掉重写

- **症状**：L68 微分支的 `test_rollback_replaces_plain_dir_current` 假红。本机 Windows 无特权、
  未开开发者模式 ⇒ `Path.symlink_to` 抛 WinError 131，而 `versioning.py` 的 except 支按**设计**
  回退成把版本 ID 写进 `current.txt`。测试却把符号链接那一臂写死成唯一形状。
- **处置**：加 `_can_symlink()` 能力探针，并把它与现实**交叉对账**：
  `assert current.is_symlink() is can`；探针说不能时额外断言 `current` 压根不存在、
  且 `.txt` 里躺着本轮的 `version_id`。另立一支
  `test_symlink_failure_falls_back_to_txt_marker`（monkeypatch 让 `symlink_to` 抛 131），
  让回退分支在**有能力**的机器上也有正牌住户，不再只靠机器环境碰运气。
- **本轮真正的收获是一次设计失败**：第一版写成「`if _can_symlink(): 断言链接臂 else: 断言回退臂`」。
  缺陷注入把探针换成说谎（恒 `return False`）之后 **8 passed 全绿** ⇒ 断言跟着探针的话走，
  而不是跟着现实，那正是不可证伪的形状。改成对账形状才有牙。
- **剩余不确定（明写，不藏）**：在**这台**无特权机器上，「探针谎称不能」那一注仍然红不了
  （`False is False` 成立），它只在有能力机器上红；反方向（探针说能而实际没链接成）由另一注
  注入证明会红。⇒ 这一格的可红性随机器能力变化，本地不可全证，别把它当成两端都已闭合。

### L214（2026-10-10）— A138 ②③ 档接线第一批：五只键 / 六条端点支路，判据一律取行为差

- **缺口**：可写四节（export / vector / rag / multimodal）14 键里 **11 只「只写不读」**：
  ② 档 6 键只在 `GET /api/config` 回显、③ 档 5 键连回显都没有。L184 把回显面钉死之后，
  「键在响应里」有守卫、「键有用」仍然零守卫 ⇒ 本轮动的是后者。
- **权威序定稿**：**请求参数 > 配置键 > 内置默认**。这一格必须先拍，否则每接一只键都要重吵一次谁赢。
  （本块第一版在这里写过一句「与 `quality.enabled` / `dedup.enabled` 的管道阶段同一口径」——
  那是没实测就引用的说法，实测证明是错的，订正见下面「本轮写错并被自己推翻的那一句」。）
- **接线与证据**（新文件 `tests/integration/test_api_config_consumption_l214.py`，13 例）：
  - `export.formats` → `/api/data/export` 与 `/api/export/batch` 的「省略 formats」支路；
  - `export.default_format` → `/api/export/preview` 的省略 `format` 支路（响应里的 `format` 是**生效值**）；
  - `rag.default_format` → `/api/dataset/rag`。这一格刻意不只断言回显字符串，还断言产物形状真的换
    了（`{id, query, answer}` 是 custom 的形状、`text` 是 langchain 的形状）⇒ echo 骗不过这一档；
  - `multimodal.image_extensions` / `audio_extensions` → `/api/multimodal/scan` 的识别面与
    `/api/multimodal/formats` 的清单。最强证据是两臂互证：内置默认认 `.gif` 而出厂配置不认，
    同一份文件 `total_records` 一边 1 一边 0；请求再点名 `.gif` 盖掉配置时同名 stem 集合只剩 `clip`。
- **对外行为变更（写进文档，不让它只活在 diff 里）**：HTTP 侧「省略 `formats`」从「导出全部 13 族」
  变成「导出 `config.export.formats` 那 5 族」。出厂默认是非空清单 ⇒ 收窄对所有省略方真实生效；
  要旧行为就显式点名或把配置写成 `[]`。管道层 `export_dataset(formats=None)` 的「None = 全部」**不变**，
  随包 UI 总是点名单只格式所以不受影响 —— 三条都在 API 文档里逐条写明。
- **两次被实测打回的设计**（都是「改契约」与「消费配置」的边界，判据是现量而不是洁癖）：
  ① 把 `enabled` 接成端点拒绝门 ⇒ **13 支既有集成测试当场红**（三只 enabled 出厂默认全 false），
  那等于把随包 API 的三项既有能力关掉。⇒ 挪给 L215 在 `AugmentorPipeline` 的阶段入口上消费，
  端点面保持不变；② 用 `config.export.formats` 去收窄 `GET /api/export/formats` ⇒ 与既有断言
  `"csv" in formats` 冲突，而冲突是对的：那只端点报的是**能力清单**，配置那是**导出策略**，
  两个语义不许混。撤掉代码，把这条区分写进 docstring 与文档。
- **本轮写错并被自己推翻的那一句（订正，不留原文误导）**：上面这些文字与两处 docstring 都写过
  「`enabled` 是本仓既有的**管道阶段开关**，住在 `AugmentorPipeline`，与 `quality.enabled` /
  `dedup.enabled` 同族」。收尾普查时按 `config.<节>.enabled` 全仓搜（扣掉 `config.py` 的校验、
  `config_validator.py` 的静态规格、`api/routes/config.py` 的回显三处），命中数是 **0**——
  **16 只 `enabled` 键在产品代码里一只都没有读者**，管道阶段真正的开关从来是调用参数
  （`augment(use_quality_check=…)` / `augment(use_dedup=…)`），阶段内部不看配置。
  ⇒ 结论跟着变：接 `enabled` 不是「给既有语义找位置」而是**新建消费路径**，于是必须先按出厂默认
  分档——`quality` / `dedup` / `versioning` / `visualization` 四节是 `true`，接上零行为漂移；
  其余十二节是 `false`，接上等于把当前始终在跑的行为关掉 = 对外破坏面，单独拍。
  **这一格不是新发现的缺陷，是既有账目的复现**：Backlog **B5** 的 AST 尺子早就量出全仓 37 只死键，
  其中「16 格是各节的启用闸门」与我这次 grep 的 16 只 `enabled` **一字不差**。本轮的病是把 B5 已登记的
  事实反过来写进文档与 docstring ⇒ 教训与本仓既有纪律同一形：**「听起来合理的语义」不是证据，
  凡是写进文档的语义断言都必须带一条现量普查**，而且要先查账——L215 按四只 true 键起步，
  B5 那 37 只键的「读取或显式废弃」是消费面收口的真正前沿。
- **第三次是本批 commit 落盘之后自己抓自己的**：`ec0668115` 给 `/api/multimodal/process` 的请求体
  加了 `image_extensions` / `audio_extensions`，实测 `fuse_modalities` 走 `process_image` /
  `process_audio`，两者按**文件内容**解析、压根不查 `supported_extensions`（那道门只住在
  `is_supported` / `process_directory`，即扫描侧）⇒ 那是本轮**新造的幽灵面**，正是 A138 的形状。
  撤销在 `bccfeff85`，并把「接线必须先举出行为差」写进路由 docstring、请求模型 docstring 与
  API 文档的分档表三处 —— 判据不许只待在这一段散文里。
- **顺带的两处必要修正**：`GET /api/config` 补回显 multimodal 两只扩展名键（有读者的键不许只活在
  代码里），`TestConfigEchoReaderSplitL184` 的 ③ 档名单随之 5 → 3，并新增 `test_consumed_keys_are_echoed`
  从正向钉「有读者 ⇒ 必须可见」，让「名单改了但回显没跟上」这种做一半的改写当场红；账本 A143 那一格
  指向 `list_export_formats` 的行引用被本轮插行换位、落到空行，由 dead_line 硬 0 档当场抓住 ⇒ 按
  A127 口径降为名字锚点，普查三档同步重量（`line_refs` 265 → 264、`file_tokens` 1666 → 1667、
  名不匹配档 149 → 148）。
- **红→绿注入（收口批重量过一遍，因为本块的③拆掉了 process 那两处读点，注入面对象变了）**：
  一次摘掉**五处配置读点**（`/api/data/export` 与 `/api/export/batch` 的 `or p.config.export.formats`、
  preview 的 `or …export.default_format`、rag 的 `or rag_config.default_format`、
  `_resolve_extensions` 的配置臂、`supported_formats` 的两只处理器构造）⇒ **13 例红 7**。
  剩下 6 例不红是**形状正确**而不是漏网：它们守的是「请求参数赢」那一档双臂（拆掉配置臂不影响
  请求臂成立），加上一例在拆配置后仍成立（内置默认本来也认 `.gif`，那一例的对照对象是识别面而不是读者）。
  逐注都先核对「注入确实落进文件」（打印残留读点数 = 0）再跑，这是本仓 L211 之后立的规矩。
  恢复后 13 passed，三个被改过的产品文件用 `git status` 逐字节核对无残留。
- **既有面**：`311 passed`（multimodal/config/openapi/doc 选择集复跑）与本轮改动前的 `618 passed`
  （integration 全量）都是零用例改动通过。
- **门禁**：L212/L213 收口后的全量读数是 `8069 passed / 10 skipped / exit 0 / 覆盖率 98.60%`；
  L214 这批改动之后的全量复跑读数：**`8083 passed / 10 skipped / 0 failed / 0 errors / exit 0 / 覆盖率 99.82% / 191.26s`**。
  用例数对得上：`8069 + 13`（`test_api_config_consumption_l214.py`）`+ 1`（`test_consumed_keys_are_echoed`）
  `= 8083`，无凭空多出的通过项。（这一格在 L215 轮被重排过一次：反引号代码跨度原本跨在换行上，
  加粗配对守卫 `test_bold_markers_paired` 当场抓住 —— 订正见 L215 块的「本轮修掉的既有格」。）
- **第一次复跑不干净，读数如实记在这里**：`2 failed / 8081 passed / 10 skipped / 1 error`。
  两支红是 L214 的代码把两处行引用顶成了死线（账本 A143 的 `api/routes/export.py`
  `list_export_formats`、`test_doc_line_refs_l79.py` 自己的注释），按 A127 口径降为名字锚点并把普查三档重量之后复绿；
  A184 硬 0 档抓到的那一处就在同一支测试文件的注释里，是同根因。
  那一条 `ERROR tests/integration/test_cli_merged_commands.py::TestEnhancedFlagIsRemoved::test_enhanced_flag_is_rejected[stats]`
  **没有复现**：参数化id `[stats]`，单独复跑该文件 `117 passed`，第二轮全量也是 0 error。
  ⇒ 记为「一次未能复现的偶发，位置在 CLI 集成链的 stats 用例」，不当成已定位的缺陷立账，
  也不当作不存在——若后续任何一轮门禁再抓到它，这条要升级为待修项。

### L215（2026-10-10）— enabled 族第一批：出厂四只 true 键接进管道阶段入口，零读者名单 16 → 12

- **缺口与动手顺序**：L214 的现量普查量出 16 只 `config.<节>.enabled` 在产品代码里一只读者都没有
  （与 Backlog **B5** 的 AST 尺子一字不差）。既然接 `enabled` 是**新建消费路径**而不是给既有语义找位置，
  就必须先按出厂默认分档：`quality` / `dedup` / `versioning` / `visualization` 四节出厂是 `true`，接上
  零行为漂移 ⇒ 本轮做；其余十二节出厂是 `false`，接上等于把当前始终在跑的行为关掉 = 对外破坏面 ⇒ 单独拍。
- **四只键的落点与判据**（新文件 `tests/integration/test_pipeline_config_consumption_l215.py`，18 例，
  判据一律取行为差）：
  - `quality.enabled` / `dedup.enabled` 进 `augment_dataset` 与 `augment_async` 两个入口的默认解析。做法
    是把 `use_quality_check` / `use_dedup` 的默认从 `True` 换成 `None` 哨兵 ⇒ 权威序仍是「调用参数 >
    配置 > 内置默认」，而 `None` 是「调用方没点」不是「关掉」。实测：质量关掉 15 条进产物、打开 0 条被滤；
    去重关掉 5 条、打开 1 条。
  - `versioning.enabled` 与既有 `auto_snapshot` **串联**而不是互相覆盖：总闸关了还自动打点，等于用户说
    「版本管理别跑」而代码回答「但自动打点你没关」。实测 1 个快照 vs 0 个，`auto_snapshot=False` 那一臂
    仍然不打点（既有语义没被顶掉）。
  - `visualization.enabled` 进出图阶段入口，关掉返回空字典而不是 4xx —— 这一族是**阶段开关**，「关掉」的意思
    是「这一步别做」，不是「这个能力不存在」，响应形状不变。这正是 L214「把 enabled 接成端点拒绝门 ⇒ 十三支
    既有集成测试当场红」那条教训的落点。
  - **两个入口同一道判**：`test_both_entries_agree_on_the_same_config` 钉同一份配置下同步与异步出同一种结果，
    专防「只接一个入口」这种做一半。
- **报告面写解析后的值**：`augment` 返回里的 `quality_check` / `dedup` 两键现在回显**解析结果**而不是入参原样
  ⇒ 既有那些 `is True` 断言仍成立（出厂两键都是 `true`，省略方拿到的解析结果就是 `True`）。
- **转发面同步收口**：API 请求模型 `use_quality` / `use_dedup` 改 `Optional[bool] = None`（缺席 ≠ 关掉，
  端点不许自己补 `True`）；CLI 的 `--no-quality` / `--no-dedup` 没点时传 `None`；`GET /api/config` 新增
  `versioning` / `visualization` 两只**只读**节 ⇒ `test_api_openapi_contract.py` 的段清单与
  `test_consumed_keys_are_echoed` 的名单（4 → 8 键）同步。**刻意只回显已接上读者的那一只键**，
  `visualization.types` 零读者不进回显 —— 塞进去就是 A138 的幽灵形状。
- **本轮不动的那一半，说清楚**：vector 五键（`enabled` / `backend` / `dimension` / `storage_dir` /
  `collection`）挪 **L216**，它要先定「配置键 vs 请求参数」谁赢；十二只出厂 `false` 的 `enabled` 每一只都是
  行为移除决策，待用户拍；`visualization.types` 那一对互证死面（声称五类图、阶段最多出三类）新立 **A267**，
  接上它属功能增强而不是零漂移消费 ⇒ 不在本轮做。
- **零既有用例改动**：18 例首跑即绿，且本轮没改任何一支既有用例的断言——对既有用例只做两处「追加」
  （8 键名单、契约段清单）。
- **红→绿注入取证（本轮全部重跑现量，不引用上一轮的记忆读数）**：
  - 六个管道读点一起摘 ⇒ **18 例红 6**，红的正是六支各自的对照臂（`test_false_in_config_disables_filtering`
    / `test_false_in_config_keeps_duplicates` / `test_disabled_creates_no_snapshot` / `test_disabled_skips_stage`
    / `test_absent_params_follow_config` / `test_both_entries_agree_on_the_same_config`）。
  - 逐点单摘（每次只摘一处，其余五点保持）：同步质量 2 红、同步去重 2 红、版本闸 1 红、可视化闸 1 红、
    异步质量 2 红、异步去重 2 红 ⇒ 六个读点各有独立取证对象，不存在「两点互掩」。
  - 转发臂三注：API 字段默认退回 `True` ⇒ 该臂 2 例红 1；端点自己 `or True` 补默认 ⇒ 同一臂红 2，含
    `test_explicit_false_is_forwarded_verbatim`（「显式 False 被吃掉」正是这一臂要防的形状）；CLI 旗标退回
    `not args.no_quality` ⇒ 该臂 2 例红 1。
  - 回显面一注：`GET /api/config` 两节整体摘掉 ⇒ 该选择集 **163 例红 4**（① / ② 档两处回显面钉子、
    `test_consumed_keys_are_echoed`、响应模型与领域契约字段一致性那一支）。
  - 注入器纪律：只按**字节**改写（`read_bytes`/`write_bytes`），改前断言注入面唯一命中、改后断言无 CRLF 混入，
    跑完逐字节复原并断言与基线相等。
- **本轮写错、被重测推翻的四处（原文不改口，订正留在这里）**：
  ① commit `2f994fbc7` 正文写「六个管道读点一起摘 **13 例红 6**」——13 是 L214 那只文件的例数，本轮文件是
  18 例，实测 **6 failed / 12 passed**。按本仓「优先新 commit、不改已落盘 commit」的规矩，错的正文留着、
  此处订正，且后续任何引用这个数字处以本块为准。
  ② 同一段写「回显两节摘掉则契约面红 3 例」，实测 **4 例**。
  ③ 一次可视化注入的极性写反：给闸门写 `if not False:` 等于「永远跳过」，红的却是
  `test_enabled_enters_stage`（错的臂）；换成 `if not True:`（= 不门 = L215 之前的行为）才抓到
  `test_disabled_skips_stage`。两次都记 —— 「红的那支不是我要的那支」本身就是注入器有病的信号。
  ④ 一次注入器把自己改坏：早先用 `Path.write_text(encoding="utf-8")` 复原时把 `pipeline.py` 整文件刷成
  CRLF（703 行 diff），且一次 `finally` 里抛 `OSError: [Errno 22] Invalid argument` 中断复原 ⇒ 从备份回灌、
  按去掉 `\r` 后逐字节核内容等价、核 CRLF 计数归零。另：`api/routes/config.py` 的基线**本来就含 CRLF**，
  「注入前断言基线无 CRLF」这条过严的断言当场把它拦下、没有误改文件 —— 断言要看被测对象的事实，不看我的假设。
- **本轮修掉的既有格**：L214 块的「门禁」那一格把反引号代码跨度写在换行处，`test_bold_markers_paired` 在本轮
  全量里红。已重排该格（代码跨度不跨行）并在原格留一句指向本块。这不是本轮新引入的语义，是 `4d177e3f3`
  写读数时留下的形状病 —— **根因是我把 8083 读数写进账本后没有复跑守卫就提交**。教训：账本改动本身也要过
  Markdown 结构守卫，回填数字这一格不能当「只有散文」免检。
- **既有面**：`web` 侧本轮零改动，回归面复跑 `119 passed`（13 个测试文件）+ `tsc` 退出 0 + `eslint` 退出 0。
  普查三档：`line_refs` 264 不变、`file_tokens` 1667 → **1668**（新测试文件名进案面多一个文件 token）、
  `code_but_no_name_match` 148 不变 —— 本轮没动任何行引用，是纯计数位移。
- **门禁**：补本块**之前**的第一次全量复跑读数如实记在这里：**3 failed / 8098 passed / 10 skipped /
  覆盖率 99.82% / 201.99s**。三支红逐条对账：两支 `test_ledger_progress_consistency_l203.py`（清单行已写、
  日志块未补 —— 正是 A265 那把尺子的设计对象，补完本块即复绿），一支上面「本轮修掉的既有格」那条加粗配对，
  它在 HEAD 上就红、与工作树新增无关（同一把尺子跑 `git show HEAD` 版本复现）。用例数对得上：
  `8083 + 18 = 8101 = 3 + 8098`，无凭空多出的通过项。补完本块、修好加粗格之后的全量复跑读数见下一段。
- **收口复跑（本轮最终门禁读数）**：补完本块、修好加粗格之后的定标全量复跑：
  **`8101 passed / 10 skipped / 0 failed / 覆盖率 99.82% / 186.16s`**。用例数与上一段对得上
  （`8083 + 18 = 8101`，且 3 红全部转绿），覆盖率语句分母 13976 与 L214 那一轮同口径可比。
- **覆盖率读数差点写错，根因是命令口径不是仓库口径**：第一次收口复跑我在 `pytest.ini` 的
  `addopts`（`--cov=augmentor --cov=api --cov=cli`）之外又加了一个**裸 `--cov`**，分母从 13976 语句
  涨到 54863，读数落到 98.80% —— 看上去像本轮掉了 1 个百分点，实际是口径被我自己改了。
  按定标命令重跑才拿到 99.82%。**新纪律**：门禁读数只认 `python -m pytest tests -q`（不加任何
  `--cov` 变体），跨轮比较前必须先核 TOTAL 那一行的语句数是否同分母，不同分母的百分比不许并列写进账本。

### L216（2026-10-10）— vector 五键接进新建的语义消费层：A138 ③ 档清空，顺带抓出两只历史漏计键

- **缺口的定性决定了动手形状**：`vector` 那五只键的问题不是「忘了回显」，而是**整个向量子系统没有产品读者**
  ——`create_vector_db` 的调用者只有测试。所以本轮做**加法**：新建消费层 `augmentor/semantic.py`
  （`embed_texts` / `build_index` / `search_similar` / `persist_index` / `index_status` 四面）、两条只读端点
  （`POST /api/dataset/semantic-search`、`GET /api/dataset/vector-status`）、`vector.enabled` 接进增强结束时的
  自动建库闸。出厂 `false` ⇒ 默认一字不建，改前改后产物相同（零漂移那一档）。L214「把 `enabled` 接成端点拒绝门
  ⇒ 十三支既有集成测试当场红」的教训在这里同样落地：**`enabled` 只闸自动建库，两条端点与它无关**。
- **五只键的判据一律取行为差**（`tests/integration/test_vector_semantic_consumption_l216.py`，24 例）：
  `dimension` ⇒ 哈希嵌入宽度，同一查询在 4 维与 1024 维下分值与排名都不同（`test_dimension_changes_scores_and_ranking`）；
  `backend` ⇒ 值流到工厂分派 + 未知值被工厂拒 + 无持久化面者被能力闸拒；`collection` ⇒ 落盘文件名
  `{collection}_vectors.npy` 与向量 ID 前缀；`storage_dir` ⇒ 落点跟着变且另一侧目录保持空；`enabled` ⇒
  报告里 `vector_index` 是 `null` 还是回执，同步与异步两个入口同一道判。
- **为什么用确定性哈希嵌入而不是真模型**：`sentence-transformers` 在本机与 CI 都没装，接上模型臂就是写一条
  **任何没装它的环境都跑不到**的产品路径——正是本仓一路在收的「只写不读」形状。真模型接法（模型输出宽度与
  本键冲突怎么收）新立 **A268** 待拍。
- **分区守卫当场抓出历史漏计**：`test_the_two_tiers_partition_the_writable_sections` 要求可写四节每一键逐键归档
  （键名从 `AppConfig` 字段实推，不抄清单），它第一次跑就报出 `rag.chunk_size` / `rag.chunk_overlap`：
  这两键**一直有读者**（`cli/commands/quality.py` 与 `/api/dataset/rag`），A138 的人读表也一直记着，
  缺的是那份**机器名单**，于是 `GET /api/config` 从没回显过它们。本轮补进可见面（回显面 14 键、
  ① 档名单 15 只）。防线同时从「③ 档键不得出现在回显里」（③ 清空后循环体为空、恒真）升成「新键不许无人归档」。
- **本轮自己造、又被自己读代码抓出的缺陷**：`persist` / `load` 只由 `FAISSDB` 实现，`ChromaDB` 是
  `PersistentClient` 自管、**构造阶段**就 `mkdir` 并注册集合。配 `vector.backend: chromadb`（合法取值，清单推导自
  `SUPPORTED_BACKENDS`）会让新代码撞上 `AttributeError`（→ 500），并让一条只读 GET 凭空建目录。处置是
  `EXPLICIT_PERSISTENCE_BACKENDS` 能力闸，且**闸的位置就是这条承诺的内容**——写在构造之前才守得住「GET 不落盘」，
  写在之后等于没守。chromadb 那侧到底怎么表达「索引存在/多少条」新立 **A269**。
- **鉴权归类是判断，不是登记**：`test_api_security.py` 的只读白名单要求逐后端核过——`semantic-search` 走内存索引
  （`storage_dir=None`，连 chromadb 那侧也只拿临时客户端），`vector-status` 走 `load()`，`FAISSDB` 的构造与
  `load()` 都不写盘；会写盘的 `persist()` 只在 `vector.enabled=true` 的建库阶段跑，而那条路只能由已挂鉴权的增强端点
  触发。结论：两条都属只读，理由写进那两行注释，而不是让白名单变成「名字看起来是 GET/查询就放行」。
- **红→绿注入取证（七臂 + 三臂，全部本轮现量）**：`dimension` 写死 384 ⇒ **11 红**；`backend` 写死 faiss ⇒ 2 红；
  `collection` 写死 default ⇒ 2 红；`storage_dir` 不进工厂 ⇒ 6 红；`enabled` 闸两个入口一起摘 ⇒ 1 红
  （红的正是 `test_disabled_builds_nothing_and_reports_none` 那支对照臂）；`GET /api/config` 摘掉 `vector.dimension`
  回显 ⇒ 2 红；端点不转发配置维度 ⇒ 1 红。能力闸三臂：摘 `persist_index` 的闸 ⇒ 1 红、摘 `index_status` 的闸 ⇒ 2 红
  （SDK 面 + 端点面各一）、名单写成两只后端 ⇒ 3 红。注入器只按字节改写、注入前断言唯一命中、跑完逐字节复原并断言相等。
- **`backend` 判据的第一版偏弱，这条最值得记**：spy 那支原本喂 `backend="faiss"` 再断言工厂收到 `"faiss"`——
  与「写死默认值」的缺陷臂读到的是**同一个字符串**，所以注入只红 1 支。改成喂非默认值 `chromadb`
  （spy 替身接住构造，本机没装包时真构造会 `ImportError`）之后红数升到 2。**纪律**：spy / 转发类判据的输入必须与
  被写死的那个值**可区分**，否则断言的是「值恰好等于默认」而不是「配置被消费」。
- **本轮写错、被重测或重算推翻的三处**：① 我在类 docstring 里把可写四节写成 16 键，`dataclasses.fields` 实推是
  **14**（export 2 / rag 4 / vector 5 / multimodal 3）；② `docs/API.md` 引用了一支不存在的测试名
  （`test_narrow_and_wide_dimensions_score_differently`），真实名是 `test_dimension_changes_scores_and_ranking` ——
  教训是**写文档前先 grep 真实测试名**，别照记忆拼；③ Backlog A 表第一次插入把 A268/A269 排在 A267 之前，
  还把 A267 的行首吞进了同一行，重排并按列数复核。
- **顺带修掉的既有标尺两处**：`api/main.py` 因新 router +1 行，把 `test_api_logging_wiring.py` 模块 docstring 里
  两条 `api/main.py:NN` 引用顶成死线，`test_comment_refs_a184.py` 当场抓住——按 L154 先例降为名字锚；
  `test_doc_line_refs_l79.py` 的逐档精确计数判据补上同族通用的 `require_authoring_tree()` 门，证据是同一把尺子跑
  HEAD 树与当前树同为 33 而钉值是 34，差额来自未入仓的 `augmentor/Temp/` 取证底料在当前工作树缺失（那目录
  `.gitignore` 明写「不入仓，也不可清」）。
- **既有面**：`web` 侧本轮**零文件改动**，故未复跑前端测试与 `tsc`（不适用，不是漏跑）。契约面登记两条新端点、
  事件循环守门登记一条新站点（语义检索的读取与建库都走 `run_in_thread`）。
- **门禁**：定标全量跑两遍，读数一字不差：**`8131 passed / 11 skipped / 0 failed / 覆盖率 99.79%`**
  （补本块之前 193.48s，补完之后复跑 177.81s，墙钟差与用例数无关）。用例对象对账：
  L215 收口 8111 ⇒ 8142，`+31` = 新文件 24 + 契约面两条端点各两支（149→153）+ 事件循环一条站点两支（51→53）+
  分区判据 1 支（14→15）。skip `10 → 11` 那一支正是 l79 新加的作者树门（同一文件同域跑：HEAD 2 skip ⇒ 当前 3 skip）。
- **中间读数留着不美化**：本轮第一次全量是 `2 failed / 8125 passed / 11 skipped`。两支红都是新增面未登记——
  只读白名单那一支是本文件的分内事，报告键集那一支是 L206 那把「键集不许悄悄变」的尺子**按设计生效**：
  `vector_index` 是签字过的新增，处置是登记进名单并在注释里写清「只认逐条签字的新增，不是永远不许变」，
  而不是把尺子改松。

### L217（2026-10-10）— multilingual 四键接进产品入口，并把 B5 那把死键尺子升成仓内守卫

- **缺口的定性与 L216 同形，所以形状照抄已验证的那一条**：`augmentor/multilingual.py` 的
  `MultilingualSupport` 在产品侧**一个实例都没建过**（`api/`、`augmentor/`、`cli.py` 全树零调用者，
  只有测试 import），能力早就写好了（语言检测、批量翻译、语言分布），缺的是消费者。于是本轮做**加法**：
  新建消费层 `augmentor/language.py`（`language_profile` / `translate_dataset` 两面）+ 两条端点
  （`POST /api/dataset/language-profile`、`POST /api/dataset/translate`），`multilingual.enabled` 那三只
  出厂 `false` 的兄弟键一个都没碰。**除 multilingual 之外零行为变更**，这是 L214「把 `enabled` 接成端点
  拒绝门 ⇒ 十三支既有测试当场红」的教训换来的形状。
- **四只键各取一条行为差**（`tests/integration/test_multilingual_consumption_l217.py`，21 例）：
  `supported_langs` ⇒ 画像的清单外集合跟着名单变窄而变宽，越界目标语言在**任何模型调用之前**被拒
  （`backend.calls == 0`）；`default_target_lang` ⇒ 配置写 `ja` 时产物才出现 `ja`，**取 `ja` 而不是 `en`
  是有意的**——SDK 内置默认就是 `en`，同名值做校验时「写死 en」的缺陷臂与正确实现读到同一个字符串；
  `translate_batch_size` ⇒ 批长序列 `seen == [7, 7, 7, 4]`，批宽 1 时并发上界真的是 1；`enabled` ⇒
  闸的位置在**读盘之前**，所以关掉闸的部署不会被一条越界文件名拖着去做白名单解析与整份读盘。
- **分批的理由是并发上界，不是调用次数**：`MultilingualSupport.batch_translate` 把整份清单一次性 submit 给
  `ThreadPoolExecutor`，5000 条就在飞行中挂 5000 个 future。本层按批切之后，瞬时并发被压到
  `min(批宽, max_workers)`；`max_workers` 是 SDK 构造参数、出厂 4，本轮不动它（动它是另一次接线）。
  **假模型后端的预言机数的是并发峰值而不是调用数**——两种写法调用数一样，只有峰值能区分。
- **三处期望被真实 SDK 行为否证，全部以实测改判据而不是改产品**：① 原本断言 5 条 ⇒ 实际只打 3 次模型，
  因为 `translate` 在源语言==目标语言时短路；为此新增全中文清单 `ALL_ZH` 与一支专门钉短路的用例，并发量测
  面也不许混进英文项（注释里写明了原因）。② 原本断言批宽 5 ⇒ 并发峰值 5，实测 4——`max_workers=4` 封顶，
  判据改成批宽 3 才量得出「峰值 = 批宽」，这条也写进了 `docs/API.md`。③ 原本断言 `backend=None` 即抛 ⇒
  实测 **200 回原文**：`batch_translate` 把逐条异常吞成「回退原文」，缺后端在下游根本不出声。
- **由此发现的真缺陷立账，本层先自保**：SDK 那条吞异常的路径让「一条都没翻成」与「全部翻完」在回执面上
  不可区分（冒充成功）。本轮在 `translate_dataset` **分批之前**补 `ModelNotConfiguredError`（→ 400），
  并加了一支「不许冒充成功」的用例；SDK 侧缺的是**失败回执面**（哪些条目回退了原文、为什么），那是产品改动，
  新立 **A270**。
- **撤掉一道我自己写的重复闸**：`translate_dataset` 里原本还有一道「目标语言在不在名单内」的闸，与 SDK
  `batch_translate` 顶部那道同判据。注入取证当场把它判死——**摘掉本层那道闸，21 例全绿**，因为下游那道
  永远先出声。两存必有一盲，盲的是多余的那道，所以删；按仓库「判据住在 SDK / config 层」的惯例只留 SDK 那道，
  并把那一臂换成有牙齿的新臂（画像侧丢掉配置名单 ⇒ 红 2 支）。路由与 SDK 的 docstring 同步改了口径，
  免得注释继续宣称一个已经不在这层的判据。
- **B5 那把尺子从 Temp/ 搬进仓库**：搬的理由是复现性，不是新奇——`Temp/` 与 `data/` 的取证底料**当前工作树里
  根本不存在**（`.gitignore` 明写不入仓），所以「37 只死键」这类读数在门禁里**无法复跑**，只能靠人肉重跑探针。
  `tests/unit/test_config_reader_census_l217.py`（20 例）把尺子入仓，并把它的盲区写进判据而不是留给记忆：
  三档互斥（直接属性链 / 经别名读到 / 零读者）+ 撞车桶 + **正控制**（有正牌读者的键若看不见，尺子自己红）+
  引用验真（归档理由里点名的那个文件与那一行，必须真的是读它的那一句）+ 反向防腐（零读者名单不许悄悄少归档一键，
  也不许悄悄多——接上读者就得改档）。尺子看不见的东西同样立了字面：加载器的鸭子形参装配（`logging` 三键走
  `_load_section`，属「有装配无判决」，另开一档）、回显面、以及 A84 的 `unread_key_messages`（那是「写了没人读
  的 YAML 键」，与本尺子的「schema 里的键有没有产品读者」是两个轴，不许混谈）。
- **现量，替掉 Backlog B5 那行 37/16/12 的旧读数**：schema 共 20 节 / **69 键**，其中 **47 只**有产品读者、
  **23 只**零读者；23 只里 `logging` 三键属装配面，**真正归档待处置的是 20 只**。`config.*.enabled` 共 16 只，
  本轮接走 `multilingual.enabled` 之后剩 **10 只**零读者。普查第一版（v2）是错的：别名臂的判据写成
  `len(seq) >= 2`，而 `ast.walk` 对 `cfg.key` 只返回 `["key"]`，那一支**永不触发**，于是 4 只 vector 键加
  rag/multilingual 键被误报成死键（假阳性 4 只）。v3 改用「链根是别名 ⇒ 取 `node.attr`」才对。这件事的
  价值全在那两支正控制上——它把「尺子说死」与「尺子没看见」区分开了。
- **鉴权分档同样是判断**：画像纯正则、不打模型、不落盘 ⇒ 进只读白名单；翻译逐条按 token 花钱 ⇒ 与
  `/api/augment/start` 同档挂 `verify_api_key`。判据记的是「会不会替调用方花钱/改状态」，不是 HTTP 动词
  （本仓早有 POST 只读的先例）。
- **红→绿注入取证（十臂，全部现量，无空转臂）**：尺子整体失明（别名解析被短路）⇒ **9 红**；尺子偏臂
  （只认别名、看不见直接链）⇒ 6 红；零读者名单少归档一键 ⇒ 3 红；schema 新增一只无人归档的键 ⇒ 2 红；
  批尺寸写成一次性全放 ⇒ 3 红；默认目标语言写死 `en` ⇒ 3 红；画像丢掉配置名单 ⇒ 2 红；后端缺失不出声 ⇒
  2 红；`enabled` 闸摘掉 ⇒ 1 红；路由把批宽写成常量 ⇒ 1 红。注入器逐臂只按字节改写、注入前断言锚点唯一命中、
  跑完逐字节复原并断言与原文相等；**第一版那条「摘本层准入门」的臂红 0 支，被当成缺陷证据用掉了**（见上面撤闸那条）。
- **本轮被自己的守卫抓住的第二次机会**：`docs/ARCHITECTURE.md` 的模块树里插两行新模块，把历史账本引用的
  `ARCHITECTURE.md:1232` 平移进非代码跨度，`test_comment_refs_a184.py` 硬 0 档当场报 2 处。处置按 L148 先例
  **换点而不是重钉历史行**：改成在既有行上同行追加，`git diff --numstat` 复核净行数为零。
- **既有面登记**：契约面（`CALLS` 两条新键 × 两支参数化 ⇒ +4 例）、事件循环守门（+1 站点 ×2 支 + 1 支翻译
  分批离线用例 ⇒ +3 例）、文档端点覆盖规模钉（dataset 16→18）、只读白名单与受保护名单各 +1，全部登记后复跑。
  `web/` 侧本轮**零文件改动**，故未复跑前端测试与 `tsc`（不适用，不是漏跑）。
- **门禁**：定标全量跑 `python -m pytest tests -q -p no:cacheprovider` ⇒ **8179 passed / 11 skipped / 0 failed / 覆盖率 99.80%**（179.56s）。
  用例增量对账，L216 收口 8131 ⇒ 8179 的 `+48` = 普查守卫 20 + l217 集成 21 +
  契约面 4 + 事件循环 3，逐文件 `--collect-only` 实测非推算。skip 数 11 一字未动。
- **中间读数留着不美化**：本轮第一次全量是 **`12 failed / 8166 passed / 11 skipped`**，没有一处是产品逻辑坏：
  7 支是文档契约计数（端点总数 73→75 四处、dataset tag 16→18、router 14→15），3 支是 l97/l98 参数化棘轮
  （新增 3 处无 ids 参数化 ⇒ `CALLS` 396→399、盲面 108→111、plain 档 54→57、用例名册 103→106），
  2 支就是上面那条 A184 平移。逐格按各自 docstring 的处方重钉并写明归因，**没有任何一把尺子被改松**。
  钉值第一次写错过一处：把 `READABLE` 记成 293，实为差值口径 399−111=**288**，重跑当场红、已订正。

### L218（2026-10-11）— 死键普查的「理由」补上机器可证伪维：我自己的散文被实码推翻 8 条

本轮没有产品代码改动，改的是 L217 立起来的那把尺子的**可信度**。它查「键有没有产品读者」是机器判据，
但同一档里那句「等价旋钮住在哪」是散文——而散文没有断言对象，写错了也不会红。

- **触发点很难看**：按纪律复核实码时发现 L217 那 20 条理由里 **8 条与证据矛盾**。全部是同一个形状——
  宣称有一个并不存在的请求参数或构造形参：`active_learning.batch_size`「住在请求参数」、
  `expander.strategies`「扩展请求走参数」、`frameworks.enabled`「框架导出走请求参数」、
  `frameworks.frameworks`「框架名由调用方在请求里给」、`sampler.enabled`「采样端点走请求参数」、
  `sampler.dimensions`「维度清单由 `/api/dataset/sample` 的请求参数提供」、`tracker.metrics`
  「指标名由调用方在请求里给」、`visualization.types`「可视化类型清单走请求参数」。实码现量：
  `api/routes/*.py` 的模型类注解里**没有** `strategies` / `frameworks` / `dimensions` / `types` 这些字段名，
  `ActiveSampler.__init__` 除 `self` 不收任何形参。20 条里有 12 条文字被改写。
- **连「假了几条」这个数都是散文数**：上面的 8 是本轮用机器比对旧提交与证据表跑出来的，我先前写进
  代码提交信息的是 7——少算了一条。修正只在账本这一侧（提交信息不 amend），这也是把判据交给机器而不是
  交给人眼的直接代价。
- **修法：理由与证据成对存放**（`_UNREAD_KNOB`），证据只有五种形状，每种都能被机器否证——`ctor` / `method`
  由 `inspect.signature` 问解释器（不读源码文本，否则签名折行和 `*args` 就骗过去了），`absent` 用同一个
  签名反着判（那里**不许**出现这个名字），`request` 由 AST 扫 `api/routes/*.py` 的 `ClassDef` 体内
  `AnnAssign` 字段名，`none` 表示产品侧不存在任何同名旋钮。目标写法与 `_LOADER_CONSUMED` 同规：
  路径相对仓库根、符号名逐字可查。
- **`none` 只许用于闸**：10 只 `*.enabled` 才允许声称「没有旋钮可接」。这一条专门防假理由换个堆放处——
  写不出证据就往 `none` 一扔，等于回到散文。
- **散文一致性只做两条窄规则**：理由里点了「请求」就必须真是 `request` 档，点了「形参」就必须真是
  `ctor` / `method` 档；反向不管（证据是 `request` 而理由只写「端点侧提供」照样放行）。为什么不做成
  「理由必须准确描述证据」——那种判决性断言 L217 已经删过一次，它会把守卫变成对文案写作水平的意见。
  残余盲区因此留着：`active_learning.strategy` 旧理由把旋钮点到了 `ActiveSampler` 名下（真旋钮在
  `ActiveLearningLoop.__init__`），形状合法、目标写错，窄规则抓不到它，是这次人工复核抓的。
- **正控制两把**：`_request_fields()` 非空、字段总数 ≥ 40、且 `dataset_tools.py` 里读到 `metrics` 与
  `reference_field`；`_callable_params("augmentor.active_learning:ActiveLearningLoop.__init__")` 读到
  `strategy` 与 `batch_size`。没有这两条，把任一 helper 改成恒空就能让 `absent` / `none` / `request`
  三档集体空转还全绿。
- **A271 据此改写**：原判据里那句「各自对应的模块确有同名构造参数」不实，现按三族重列——请求侧已有
  旋钮 2 只（`evaluation.metrics` / `reference_field`，L219 的候选）、SDK 有旋钮但产品侧无调用者 4 只
  （先造消费者才谈接线）、根本没有同名旋钮 4 只（那是加功能不是接线）。新立 **A273**：`pipeline.py:146`
  与 `:149` 把 `DomainExpander` / `ExperimentTracker` 构造出来之后产品树再无引用，`batch_expand` 与
  `log_metric` 今天没有任何入口——A271 那一族的根因就在这儿。
- **红→绿注入取证（十臂，全部有牙齿，每臂 1 红）**：A1 `request` 字段名说谎、A2 `ctor` 形参名说谎、
  A3 `absent` 谎称没有（`analyze_coverage` 真收 `items`）、A4 **原样复原 L217 那条假理由**、A5 少挂一只键
  的证据、A6 非闸键扔进 `none`、A7 新造一档没人验真、A8 请求字段扫描臂瞎掉（正控制抓住）、A9 把「没有」
  谎称成「有」、A10 签名读取臂瞎掉。A4 是本轮的靶心：新守卫抓得住我刚犯过的那个错。
- **探针通道的教训（工具形状，不是代码形状）**：第一版注入器逐臂改写磁盘上的测试文件，中途一次
  `OSError [Errno 22]` 把「跑完复原」打断，仓库文件里留下 `metricz` 与 `items` 两处注入残留。发现它的
  不是脚本的自我报告，而是复跑该文件时那 1 支红。第二版改成 pytest 插件在**导入后的模块对象**上打补丁
  （`pytest_collection_modifyitems` 里按 `__file__` 找回模块），仓库文件全程零写入。规则：会改仓库文件的
  探针必须自带「跑完重新验磁盘」这一步，且失败要响；能不改文件就别改。
- **门禁**：定标全量跑 `python -m pytest tests -q -p no:cacheprovider` ⇒ **8187 passed / 11 skipped /
  0 failed / 覆盖率 99.80%**（180.61s）。用例增量对账 8179 → 8187 的 `+8` 全部来自普查守卫
  （该文件 20 → 28 例），逐支数过。skip 11 一字未动。
- **两把参数化棘轮一格未动**：本轮八条新判据一律用循环而不是 `parametrize`——要报的失败面是
  「一张 20 行的对账表一次报全」，逐键分片反而看不见其余 19 条，顺带也就没给 l97 / l98 添新形状。
  产品代码零改动，故 `web/` 侧、`docs/` 契约计数与 README 面本轮**不适用**（不是漏跑）。

### L219（2026-10-11）— `evaluation.metrics` 接进 `/api/dataset/evaluate`：一次零漂移接线，顺带让两格死角第一次可达

L218 把「等价旋钮住在哪」逐键验真之后，这一族里唯一「旋钮现成、接线近乎免费」的一格就是
`evaluation.metrics`。本轮按 A271 的排序把它收掉；**同节的 `reference_field` 没收**——它接不动，
理由立成 A274 而不是硬接一只假线。

- **接的是省略支路**：`api/routes/dataset_tools.py` 的 `dataset_evaluate` 过去把 `request.metrics`
  原样交给 `ModelEvaluator`，配置那只清单在这条路径上从未参与（普查因此判它零读者）。现在生效值按
  **请求参数 > `config.evaluation.metrics` > SDK 内置默认** 解析，与 L214 定稿、L215/L216/L217 沿用的权威序同形。
- **零漂移不是口头话**：出厂 `config.yaml` 与 SDK 那行内置回落值**逐字同**（`bleu` / `rouge_l` /
  `similarity`），所以按出厂配置部署的读数一字不变。判据取两侧——改配置 ⇒ 响应的指标集合跟着变
  （专打「写死内置三只」那一臂），以及一整份 `metrics` 字典逐值相等
  （`test_factory_config_is_the_same_three_metrics` 比「响应恰好三个键」强，因为它拿显式清单的结果做对照）。
  只断言集合不断言数值，是因为短中文串上 `bleu` 与 `rouge_l` 会同为 0.0，那种数值断言咬不住东西。
- **接线让两格死角第一次可达，所以出声与接线同轮做**：其一，SDK 的 `metrics or [内置三只]` 把**空列表当假值**，
  配置留空会被静默补成三项——配置说「一个都不算」而响应三个指标齐全；本层现在对空清单 400，并在 detail
  里点名两个来源（请求与配置），因为只回「指标为空」会让人去查错的那一侧。其二，白名单过去只判
  `request.metrics`，配置里的错名会原样落进 SDK 的 `DataValidationError`；现在判**生效值**，同一句 400
  同一份可选项提示。这两格在接线前都不可达，属「接线顺手暴露的既有缺陷要同轮收」的既有口径。
- **请求侧空列表的语义由实测固定**：`metrics: []` 与省略在 pydantic 层**不可区分**（都是 falsy），
  所以按权威序落回配置而不是报错。这条写进测试（`test_empty_request_list_falls_back_to_config`）
  而不留给读者猜——它是本轮唯一一处「形状不是我想要的，但只能这样」。
- **普查档位跟着改**：`evaluation.metrics` 从第 3 档挪出，并挂进 `_MUST_BE_ALIAS_ONLY` 当别名臂的
  **正控制**（新读者确实是 `evaluation_config = get_pipeline().config.evaluation` 之后再取
  `evaluation_config.metrics` 这一形状，纯属性链抓不到）；L218 那句「理由第一版有 7 条是假的」按机器比对
  订正为 **8 条**——把旧提交文本与证据表逐条对跑得到 8，我先前那个 7 连数都数错了。
- **现量**：schema 20 节 / 69 键里，有产品读者 47 → **48**、零读者 23 → **22**，归档待处置 20 → **19**
  （非闸门键 10 → **9**，`config.*.enabled` 仍是 10 只未接）。第一本账 B5 那行按同一读数就地订正。
- **A274 为什么是「接不动」而不是「没空接」**：`EvaluateRequest.reference_field` 的默认是硬值 `"output"`，
  调用方省略与显式传 `"output"` 在 API 面上是同一个形状，`request.x or config.x` 的 `or` 永远落在请求那一侧——
  照抄 metrics 的写法只会得到一只**改了配置而读数不变**的假接线，正是本仓在收的形状。真接法要把字段改成
  `Optional[str] = None`，那是契约变化（OpenAPI 可空性、既有客户端省略语义、`test_api_openapi_contract`
  的字段面尺子），另拍。生成侧 `generated_field` 在配置里压根没有对应键，接完会留一对不对称默认。
- **红→绿注入取证（四臂，全部有牙齿）**：I1 配置不参与（回退成只认请求）⇒ **5 红**；I2 权威序写反
  （配置赢过请求）⇒ **2 红**；I3 空清单不出声 ⇒ **1 红**；I4 白名单只判请求 ⇒ **1 红**。目标文件是
  **纯 CRLF**（933 处行尾、零个孤立 LF），所以注入器改成「读时归一成 LF、写时回译成 CRLF」并逐臂
  读回比字节、收尾断言与进入时一致；这套自证在 L218 那次 `OSError` 留残留之后已经是硬性要求。
- **被自己的守卫第二次抓住同一件事**：路由那处 docstring 第一版**多插了一行**，把 `OPTIMIZATION_LOOP.md:294`
  引用的 `api/routes/dataset_tools.py:497` 平移成非代码跨度 ⇒ `test_comment_refs_a184.py` 硬 0 档与
  `test_doc_line_refs_l79.py` 同时红。处置不是重钉历史行，而是把 docstring 压回一行、让该处净行数归零
  （路由主体那 21 行插在 646 之后，历史引用没有落在受影响的区间里，三把尺子当场转绿）。这与 L217 那条
  「插行会平移历史引用」是同一个缺陷类的**第二次**，且这次是我先写了注释再改的形状。
- **一次不干净的读数留着**：第一次全量跑是 **`3 failed / 8193 passed / 11 skipped / 2 errors`**——
  三支红就是上面那条插行平移（其中一支还是守卫自己的注入臂被同一处污染成 `2 == 1`），而那 2 个 error
  是我**在同一次门禁运行期间继续改账本文件**造成的：跑测试期间不许再动被守卫读取的文件，这条记成操作口径。
  清掉之后复跑 ⇒ **8196 passed / 11 skipped / 0 failed / 覆盖率 99.80%**（183.29s）。
- **用例增量对账**：8187 → 8196 的 `+9` = L219 集成 8 支 + 普查守卫里 `_MUST_BE_ALIAS_ONLY` 多出的 1 个参数化
  case。skip 11 一字未动；**l97 / l98 两把参数化棘轮一格未红**（新增 case 的 ids 由字面串自动导出，
  不落在盲面名册里）。

### L220（2026-10-11）— `evaluation` 两侧字段名接进省略支路：A274 那句「接不动」用 `None` 哨兵解开，并按对称口径补一只新键

- **接的是同一格，用的是不同钥匙**：L219 收 `metrics` 时把同节 `reference_field` 判成**接不动**并立 A274，
  原因是请求默认写死 `"output"`、与出厂配置同值，`request.x or config.x` 的 `or` 永远落在请求那一档。
  本轮把两只字段名请求默认改成 `Optional[str] = None` 哨兵，权威序 **请求参数 > 配置键 > 内置默认** 才真的可达；
  这不是「换个写法」，是把 API 面上两种不可区分的形状（省略 / 显式 `"output"`）拆开成两件事。
- **口径决策：补 `evaluation.generated_field` 而不是留一对不对称默认**：只接参考侧会得到「参考侧跟配置、
  生成侧跟硬编码」，而这两只字段在同一条请求模型上、语义完全对称。新增键必须有读者，所以它与接线同轮做、
  与同节兄弟键同规（运行时 `require_string` + 规格表 `non_empty`），不留「写进 schema 却没人读」的新账。
  `config.*.enabled` 一族本键不涉及，出厂闸仍 `false` 且仍未接（A271 第①族，未动）。
- **零漂移的前提是逐字同值，不是我的断言**：两只键出厂都是 `"output"`，与它们所替换的那句硬编码默认同值，
  所以按出厂配置部署的读数一字不变。等价臂（`test_factory_defaults_are_zero_drift`）拿**整份响应**逐值比，
  而前提本身由 `TestFactoryPremise` 三臂钉住（dataclass 默认、仓库 `config.yaml`、规格表形状），
  外加 `TestColumnValuesAreDistinguishable` 先核对四列文本两两不同相似度 ⇒ 「配置改了读数不变」这种
  假绿在这套桩子里没有藏身处。
- **坏字段名一律出声，两处各管一段**：配置指向取不到的列 ⇒ `_as_column` 逐条带下标 400（`第 0 条没有字段 'nope'`），
  配置写空串 ⇒ 加载期 `require_string` 拒，绕过加载直接改属性 ⇒ 门口照样 400 并报出 `''`。
  请求侧空串与省略不可区分（pydantic 层就是 falsy），按权威序落回配置——这条与 L219 给 `metrics: []`
  定下的语义同式，不是各自发明的两套规矩。
- **消费路径只有 API 一条**：`augmentor/cli/` 与 `cli.py` 全树对 `ModelEvaluator` / `evaluate` 零命中，
  `web/src` 也没有打这条端点 ⇒ 本轮不存在「同一判据在两处实现」的第二现场，`web/` 侧零文件改动
  （所以未复跑前端测试与 `tsc`，属不适用而非漏跑）。
- **普查档位跟着改**：`evaluation.reference_field` 从第 3 档（零读者）挪出并挂进 `_MUST_BE_ALIAS_ONLY`
  当别名臂正控制，`evaluation.generated_field` 作为新增键直接进同一档；`_UNREAD` 18 行、
  `_UNREAD_KNOB` 同步摘一条（配对的机器断言当场逼我改档，不是我主动想起来改的）；
  「尺子不空转」那支的正控制从两字段扩到三字段（`metrics` / `generated_field` / `reference_field`），
  并把理由表说明里那句「一张 20 行的对账表」订正成现量 18 行。
- **现量**：schema 20 节 / **69 → 70 键**，有产品读者 **48 → 49**、零读者 **22 → 21**，
  归档待处置 **19 → 18**（`config.*.enabled` 仍 10 只未接、非闸门键 9 → **8**）。第一本账 B5 那行按同一读数就地订正。
- **红→绿注入取证（八臂，全部有牙齿、零空转）**：摘掉 `generated_field` 的 `None` 哨兵 ⇒ **5 红**；
  摘掉 `reference_field` 的哨兵 ⇒ **4 红**；路由不再读配置那侧的参考字段名 ⇒ **7 红**（其中一支是普查守卫的
  `test_alias_readers_are_seen[evaluation.reference_field]`，即尺子自己看见读者消失了）；权威序颠倒成
  配置压过请求 ⇒ **2 红**；把新键出厂值改成 `"answer"` ⇒ **1 红**（前提臂红，等价臂不陪）；
  摘掉新键的运行时判据 ⇒ **6 红**；删掉新键的规格行 ⇒ **4 红**；反向防腐臂——把已接上读者的键仍留在零读者档 ⇒
  **5 红**（分区、档位精确相等、理由非占位、证据配对四支一起红）。注入器逐臂只按字节改写、锚点唯一命中先断言、
  跑完逐字节复原并断言与进入时一致；脚本与备份都写在仓库外（`D:/WORKSPACE/`），用完即删。
- **既有面登记（新增一只 schema 键的账，全部按现量重钉、没有一把尺子被改松）**：`test_config_gates_l154.py`
  三处名单（`STR_KEYS` / 合法取值 / 删规格行后运行时仍拒）+ 那支「逐节键数」计数 `evaluation` 3 → 4、
  总数 29 → 30，并把两支测试名从写死 29 改成点名归因（`test_all_listed_keys_have_static_spec_rows` /
  `test_the_key_census_is_29_plus_the_l220_key`）；`test_blank_string_l83.py` 的 `ALREADY_SAFE` +1 与并集计数
  33 → 34；`test_config_validator.py` 的 `non_empty` 精确集合 +1；`test_section_registry_l77.py` 的
  「新增字段常驻门铃」69 → 70；普查守卫的 `69 只键` 说明文字 → 70。
- **一次不干净的读数留着**：第一次全量是 **`1 failed / 8223 passed / 11 skipped / 覆盖率 99.80%`**（180.83s），
  唯一那支红正是 `test_section_registry_l77.py` 的门铃计数（`assert total == 69` 读到 70）——它自己的注释就写着
  「红的只有这个必须精确相等的计数，它是新增字段的常驻门铃，不是缺陷」，而落地档（真写 YAML、真落进节对象、
  值与出厂不同那条反空转断言）当场判绿。按现量重钉后复跑 ⇒ **8224 passed / 11 skipped / 0 failed / 99.80%**（186.16s）。
- **用例增量对账（实测，不是推算）**：8196 → 8224 的 `+28` = L220 集成 17 + `test_config_gates_l154.py` 8
  （新键逐档 5 支坏值 + 合法取值 + 静态面合法值 + 删规格行后运行时仍拒）+ 普查守卫 2（`_MUST_BE_ALIAS_ONLY`
  两支参数化各多一只键）+ 注册表门铃 1。对账做法是新的：把 HEAD 版那 7 个文件**临时换入**跑一次全量
  `--collect-only`，再逐字节复原并断言相等，得到逐文件改前/改后两张计数表（其余文件零差别）；
  换入前先落一份盘外备份。skip 11 一字未动；l97 / l98 两把参数化棘轮一格未红。
- **插行这次没有将历史引用平移出代码跨度**：路由文件净增 14 行，`test_comment_refs_a184.py` 与
  `test_doc_line_refs_l79.py` 当场绿（那把尺子判的是「引用落点是不是代码跨度」，不是逐符号比对），
  所以不必像 L217 / L219 那样换点。这条不意味着插行安全——L219 的教训仍然成立。
- **新立 A275**：`docs/README.md` 的配置段表格「关键项」列与 `AppConfig` 的键清单**没有机器对账**，本轮两处文档
  全靠人肉同步；这正是 A266 那句「契约面数字靠人肉对账」在配置面上的具体形状，方向是从 `fields()` 实推逐节比对。

### L221（2026-10-11）— 逐条翻译失败不再冒充成功：SDK 回报告、聚合层跨批折算、端点带回执的 200

- **补的是 A270 那一格**：`MultilingualSupport.batch_translate` 的 `except Exception as e: logger.error(...)`
  把逐条异常吞成「回退原文」，返回形状与全部成功时**完全相同**。本轮把一面拆成三层各管一件事：
  SDK 管「本批哪几条失败、失败原因原文照回」，聚合层管「跨批坐标与响应体尺寸」，端点管「这四键出现在
  HTTP 响应与 OpenAPI 里」。**回退原文这个行为一个字没改**（长度与顺序对齐是调用方的既有依赖），
  改掉的只是它的沉默。
- **判据一律取行为差，四条各钉一格**：① 下标是**输入下标**而不是 `as_completed` 的完成顺序——失败那条
  刻意做成**最先返回**的那条（`SelectiveBackend` 的失败路径不 sleep），按完成顺序记数的实现会拿到 `0`
  而不是 `2`；② 批宽 2 时失败落在第 2、3 批，回执必须是 `[3, 4]` 而不是批内的 `[1, 0]`；③ 样本边界取
  **正好 20 条 ⇒ `errors_truncated` 是 False**（`>` 而不是 `>=`），25 条 ⇒ 样本 20 + 截断为真 + `failed`
  仍是 25；④ 响应模型少写任一只键 ⇒ 端点回执测试与契约尺子同时红。
- **一处口径决策，写进代码而不是只写在这里**：部分失败**仍走 200**。整批失败（后端缺失）由 `language.py`
  在分批之前抛 `ModelNotConfiguredError` 落 400，那一档 L217 已经收过；而「25 条里 3 条没翻」既不是客户端
  错误也不是服务端故障，把它做成 4xx/5xx 会让调用方重发整份、重复按 token 花钱。所以诚实读数放响应体
  （`failed` / `failed_indices`），路由 docstring 明写「状态码不是这一档的载体」。
- **截断只住一层，是判断而不是顺手**：SDK 的 `errors` 是本批**全量**记录（批宽 25 天然有界），聚合层
  `translate_dataset` 才做「前 20 条样本 + `errors_truncated`」。理由是「20」这个数字如果两层各写一份就成了
  第二事实来源（F-04 那族教训）；而计数取 SDK 显式的 `failed` 而不是 `len(errors)`，这样将来样本面再怎么收窄，
  「失败了几条」都不会跟着失真。`failed_indices` 到响应侧可能只剩样本，所以它是全量下标的权威来源。
- **第一版我自己写坏过一次，如实记**：`batch_translate` 的 except 分支初稿是
  `{"index": getattr(future, "_ann_index", None)}`——`Future` 没有这个私有字段，于是下标恒 `None`，
  `failures.sort(key=...)` 会直接 TypeError。它是本轮写作过程产物而不是既有缺陷，但注入臂 A1（换成
  完成顺序计数）正是这一族的邻居，红 3 支说明这一格的牙齿是真的。
- **A270 原文有一句不准**：它说要同步的 `test_api_report_keys` **在仓内根本不存在**（L217 立账时凭印象写的
  文件名）。真正的键集尺子是 `test_api_openapi_contract.py` 的 `TestResponseModelDoesNotDropFields`，
  它守两个向（实际响应命中契约 + OpenAPI 声明同一份），注入臂 A7 一次打红两支
  （`test_response_matches_domain_contract[route_key46]` 与 `test_openapi_advertises_domain_contract[route_key46]`）。
  契约那份键清单**刻意不从 `TranslateResponse` 实推**——两边同源就永远抓不到「模型写了而响应静默丢字段」，
  与 `_gate_report_keys()` 那条（从 SDK `to_dict()` 实推）方向相反，差别在此处记明免得后人当成不一致来「统一」。
- **既有面迁移（全部按现量，没有一把尺子被改松）**：`tests/unit/test_multilingual.py` 的 **15** 处译文取值点
  （现量由 `grep -c 'report\["items"\]'` 数出，旧写法是 `result[...]`）迁到报告形状，两支空数据集断言改成
  **等形的零值报告**（旧写法 `== []` 与新形状之间没有中间态可言），`test_failed_items_fall_back_to_original`
  从「只钉长度」升级成同时钉 `failed` / `failed_indices` /
  `errors` 逐值；`test_multilingual_consumption_l217.py` 那支整份字典相等（空数据集）补四键——它本来就是
  键集权威，新增/丢失任一只都会当场红；该文件的 `Spy(real)` 子类一字未改就仍然绿，因为它是包装而非形状依赖。
- **门禁读数**：**8239 passed / 11 skipped / 0 failed / 覆盖率 99.80%**（180.49s）。与 L220 的 8224 相减正好
  `+15`，即新文件 `tests/integration/test_translate_failure_receipt_l221.py` 的 15 支；逐文件对照用 L220 那套
  「HEAD 版临时换入跑 `--collect-only`，再逐字节复原并断言相等」的跑法实测：`test_multilingual.py` 33 → 33、
  `test_multilingual_consumption_l217.py` 21 → 21、`test_api_openapi_contract.py` 157 → 157，
  即三个既有文件本轮**只改形状不改数量**。skip 11 一字未动。
- **注入八臂（每臂单点改、锚点唯一命中先断言、跑完逐字节复原并断言与进入时一致）**：
  A1 完成顺序当下标 **3 红** / A2 聚合层不加 `start` **1 红** / A3 `>` 写成 `>=` **1 红** /
  A4 聚合层不截样本 **2 红** / A5 SDK 层就截成 20 **1 红** / A6 计数恒零（原缺陷的 ghost） **7 红** /
  A7 响应模型少写四键 **6 红** / A8 失败条目被丢掉而非回退原文 **4 红**。A2 只红一支是诚实读数而非弱点：
  `test_sample_keeps_the_global_indices_it_reports` 钉的是「样本下标与 `failed_indices` 同轴且升序」这一内部
  一致性，加不加 `start` 它都成立——全局坐标这一格由 A2 那支专门的正控制独占。脚本与备份都在仓库外
  （`D:/WORKSPACE/`），用完即删。
- **插行没有平移历史引用**：`multilingual.py` / `language.py` 的改动全部落在既有函数体内且**在
  `docs/ARCHITECTURE.md` 引用的 `multilingual.py:43` 之下**，`test_comment_refs_a184.py` 与
  `test_doc_line_refs_l79.py` 当场绿；这也是本轮刻意不新增模块级常量（`sample_limit` 写为函数内局部量）的原因。
- **配置普查本轮零变动**：没有新增、删除或接线任何配置键，现量仍是 schema **70 键** / 有读者 **49** /
  零读者 **21** / 归档待处置 **18**。`multilingual` 四键的读者面仍是 L217 那一条消费层。
- **文档同步**：`docs/API.md` 翻译行补上失败回执四键的读法，并把「为什么部分失败仍 200」与「只读 `items`
  读不出失败」写进同一格。`docs/README.md` 的 `multilingual` 行讲的是四只键的落点，本轮未涉配置面，故不改。
- **仍在账上的邻居**：**A272**（`multilingual` 要不要开成第 5 个可写配置节——本轮明确回答了「样本上限」那半边：
  它不该成为键，但 `default_target_lang` 等四键是否运行时可改仍是待拍项）；以及 L220 之后量到的
  「**32 个端点没有前端调用者**」这一消费面缺口，`/api/dataset/translate` 本身就在其中。

### L222（2026-10-11）— 配置面三侧同源对账：README 表格、`AppConfig`、`config.yaml` 三角互核，权威只有一份

- **补的是 A275 那一格**，而且是它自己立的账被自己的尺子量了一遍：L220 新增
  `evaluation.generated_field` 时 `docs/README.md` 与 `config.yaml` 全靠人肉同步，当时我在日志里
  写了「没有机器守」，本轮把这句话落成 59 支守卫。**用最终版尺子对 HEAD 那份文档的读数是
  21 红 / 38 绿**（取证脚本 `evidence_l222.py` 在仓库外跑）：表里只有 16 行，schema 节只覆盖 34 只键，
  `context` / `expander` / `sampler` / `tracker` / `visualization` **5 个整节缺席**，13 节的键清单只写了一半上下，
  恰好列全的只有 `export` 与 `multilingual` 两节。也就是说 A275 那句「容易烂」不是推测，是量出来的。
- **权威只有一份，是本轮最重要的一条判据**：节清单与键清单全部从 `dataclasses.fields(AppConfig)` 与
  `getattr` 出来的节实例实推（与 `test_section_registry_l77.py` 同一口径），`init=False` 的键按定义写不进
  文件所以不算清单。测试侧唯一手写的名单是 `META_ROWS = {models, app}`，而它每一员要**同时**过两条机器验真
  （确实不是 dataclass 节 + 确在校验器 `ConfigValidator.META_TOP_SECTIONS` 白名单里）⇒ 把一只真节塞进例外名单
  来绕开逐节比对，这条路是堵的（注入 A6 当场红 2 支，其中一支就是验真臂）。
- **三侧而不是两两**：只比 README↔schema 会放过「文档说有了而文件里没有」，只比 schema↔yaml 会放过
  「文档少了整整一节」。三条读数各自与 `fields()` 推出来的那份 70 键集合相等，两两自然一致，所以
  `TestThreeSidesAgree` 那两条是合成读数而不是第三条独立刀口——它红得比前两类的组合更晚，是设计如此。
- **`config.yaml` 那一侧教本尺子改了一次判据，如实记**：第一版把「文件里没有 `logging` 节」当成漂移，
  被文件自己那段注释推翻——整节不存在时 CLI 与 API 的日志一字不变，写出来才装配，那是**刻意的产品决策**。
  于是 yaml 侧判据从「与 schema 逐节全等」换成**「整节写全，或整节注释列全」**：一节可以整个不落地，
  但不许只写半节（A4 红 3 支）、不许写幽灵节（A3 的形状在 yaml 侧同判）、也不许新增一只键而文件里
  连提都没提。为了让「整节注释」这一档不是靠人记得那段注释，另加两臂：`commented_section_keys()`
  扫描器自己有正控制（抹掉一行注释就少一键，A7 红 3 支），以及缺席节必须在文件里有一句说明它缺席时
  行为是什么的判据。
- **尺子自己也有前提臂**：`len(SCHEMA) >= 20` 且键和 == 70（A5 一加键就红，正是这条在替「70」这个数字
  守着）、README 与 yaml 两侧解析出的成员数各设下限，防的是「解析器一行没解析出来，两侧都是空集于是
  等式恒成立」这一族空转（A171 口径：数不到不判干净）。
- **第一版我自己写坏过两处，都是没读代码就下判**：① 把 `models` 当成「不是 `AppConfig` 字段」——
  它**是**字段（一个 dict），待在 `META_ROWS` 的理由是「它的键不住在 dataclass 里」而不是「它不是节」，
  验真臂对它的判据因此必须是「不是 dataclass 节」这一条而不是「不是字段」；② 前提臂写成
  `len(README) >= 20`，把「修好的文档形状」当成了尺子的前提——真漂移状态下这条本该红的臂反而先炸给我看。
  两处都在本轮改掉，且改后对 HEAD 文档的读数（21 红）是重量的，不是沿用第一版那个被推翻的数。
- **既有面迁移（按现量，没有一把尺子被改松）**：`tests/unit/test_parametrize_ids_l97.py`
  CALLS 399 → **402**、盲面 111 → **114**（READABLE 288 是差值口径，未动）；
  `tests/unit/test_parametrize_blind_face_l98.py` 六档 plain 57 → **58**、derived 36 → **38**、
  盲面用例 106 → **109**（`MEASURED_BLIND_SEEN` 同步、豁免 17 未动）。第一次全量门禁就是这 4 支红
  （`test_census_agrees_with_the_l97_ruler` / `test_tier_counts_are_pinned` /
  `test_the_dynamic_face_sees_every_blind_site_by_name` / `test_census_totals_match_this_rounds_measurement`），
  归因清楚：新文件三支无 ids 的 `parametrize` 全落盲面。档位不是猜的——用 l98 的 `static_rows()`
  单独对本文件量过：`sorted(META_ROWS)` 入 **plain**（frozenset 里的字面串看得见），
  两支 `sorted(SCHEMA)` 入 **derived**（元素来自函数返回值，Scope 看不见）⇒ 1 + 2 = 3 与档位和、
  盲面、CALLS 三处独立闭合。
- **门禁读数**：**8298 passed / 11 skipped / 0 failed / 覆盖率 99.80%**（末次 182.27s，账本合并后重量一遍）。
  与 L221 的 8239 相减
  `+59`，正好等于新文件 `tests/unit/test_config_three_side_l222.py` 单独跑出来的 59 支，
  即本轮除两把参数化棘轮改常量之外**没有既有测试改形状**。skip 11 一字未动。
- **注入七臂（每臂单点改、锚点唯一命中先断言、跑完逐字节复原并断言与进入时一致）**：
  A1 README 少一整行 **4 红** / A2 README 键名改名 **2 红** / A3 README 多写幽灵键 **3 红** /
  A4 `config.yaml` 少一只键（半节） **3 红** / A5 schema 新增一只 init 键 **5 红** /
  A6 例外名单塞进真节名 **2 红** / A7 注释扫描器少读一键 **3 红**。
  A5 是这一族的靶心：它复现的正是立 A275 的那一步（L220 给 dataclass 加 `generated_field`），
  五红分别落在前提档的 70 键和、README 逐节臂、yaml 逐节臂、三侧合成读数、两两差值——
  也就是「加一只键而三处文档/文件没人跟着动」在这把尺子下**不可能无声通过**。
  A2 只红 2 支是诚实读数：改名不影响行数，所以逐节存在性臂不参与，红的是逐节键清单与两两差值。
  脚本与备份都在仓库外（`D:/WORKSPACE/probe_l222.py`、`probe_backup_l222/`），用完即删。
- **文档同步**：`docs/README.md` §7 的配置表从 16 行重排为 22 行，列名「关键项」改成「键清单」——
  旧列名是这轮漂移的语义挡箭牌（「关键项」天然允许挑着写，而逐节比对要求列全）。表格上方另加一句
  明确「本节不复制『哪只键有人读』的结论」，读者身份权威仍住在
  `test_config_reader_census_l217.py`，免得两处各写一份结论（A266 那一族的第二个事实来源）。
  `app` 那一格按本轮量到的事实写明「出厂文件里写了而产品侧零读者」，并把处置指向 A276。
- **新立 A276，但第一版把它写成了「本轮量到的新事实」——这句是本轮的第二次口径错，订正如下**：
  `app` 那三键**没有读者**这件事仓内早就记着三处（`OPTIMIZATION_PLAN.md` §2.4 的
  「`AppConfig` 不建模 `app` 段、`load_config` 从不读它 → 写出的配置必然不合法」、`README.md`
  变更说明第 6 条「由必填降为可选」、`tests/integration/test_cli_validate_config.py` 的 docstring
  「也没有任何代码读它们」），历史上的那次处置是**降为可选**而不是接上读者。本条真正新的只有两格：
  ① 它对 B5 那把「只覆盖 schema 键」的死键普查尺子**结构上不可见**（没有字段就没有格子，
  校验器又按 `META_TOP_SECTIONS` 免检），所以零读者名单里从来数不到它；
  ② 这只死面的**处置**（接成节 / 从出厂文件删掉 / 维持现状并写明）从未立过账。
  本轮只做了「立账 + 文档写明 + 验真臂钉住形状」，没有顺手删那一段（动对外文件）也没有顺手接成节
  （schema 70 → 73 键，节注册 / 规格表 / 两把普查尺子 / 本把三侧尺子四处要重钉，一轮做不完）。
  `api/routes/config.py` 里那段写入清单注释本来就把 `app` / `models` 叫「两个元节」，
  与本轮 README 表格的 22 行（20 节 + 2 元节）同一口径——这条不是我发明的分类，是仓内既有说法。
- **配置普查本轮零变动**：没有新增、删除或接线任何配置键，现量仍是 schema **70 键** / 有读者 **49** /
  零读者 **21** / 归档待处置 **18**。本轮动的是「文档与出厂文件怎么描述这 70 键」，不是这 70 键本身。
- **插行与引用**：`docs/README.md` §7 表格净增 12 行（+29 / −17），而 `test_doc_line_refs_l79.py`
  的扫描面是 `docs/ARCHITECTURE.md` 与本账本这两份活文档，README 不在刀口上；本轮代码文件一行未动，
  所以历史 `file:line` 引用不会被平移——`test_comment_refs_a184.py` 与 l79 在全量里当场绿。
  **反过来，账本这一侧是它的刀口，而且本轮真的被它红过一次**：新写的 A276 行我在散文里换了行，
  于是那条表格行被劈成两行，`test_docs_markdown_structure.py::test_tables_have_consistent_column_count`
  在全量里报 `1 failed, 8297 passed`（列数 3 对表头 5）。这是本账本自身的形状缺陷，不是尺子过严，
  修法是把那一行并回单行（GFM 与这把尺子都要求一行一格）。合并后同批守卫 **134 passed / 8 skipped**。
  本日志块不写任何 `` `path:行号` `` 形式的定位引用；仓外取证脚本与备份目录在本块里以反引号名出现，
  实测 l79 不把它们算成断链（改前我担心这一格会红，跑出来 59 passed / 3 skipped —— 这一条是量过的，
  不是推测的）。
- **仍在账上的邻居**：**A276**（本轮新立，`app` 那三键的三条处置方向都要拍，见上）；**A271** 的
  18 只待处置键（闸门 `enabled` 10 + 非闸门 8）一字未动；**A272**（`multilingual` 要不要开成第 5 个
  可写配置节）与 **A273**（管道构造 `self.expander` / `self.tracker` 之后产品侧从未调用）仍是待拍而不是待写。
  **A275 已收口**，不再占账。

### L223（2026-10-11）— 逐端点字段提示与契约面同源对账：`output_path` 这个名字在路由里是局部变量，在文档里冒充了 4 处字段

- **补的是 A277 那一格**，而且本轮是「立账与收口同一轮」：先按 A275 的修法量了一遍现状，确认
  `docs/API.md` 的四列说明表里那些反引号字段名**没有任何机器读过**（l210 只核路径集合与说明
  存在性，l200 只核数字），确认之后才动手，不是先动手再找理由。
- **权威只有一份**：字段名从 `app.openapi()` 实推，契约面 = 请求体属性（递归过 `$ref` / `allOf` /
  `additionalProperties` / `items`）+ `parameters[].name` + 响应体属性。现量：详细表 **31** 格
  （dataset 18 + system 13）、提取到 **58** 个标识符形状的声称、其中 **40** 个（**69%**）直接落在
  所属端点的契约面上。剩下 18 个不许靠「散文嘛」过关，各自要落进一档**有位置**的证据。
- **字面量档的作用域是量出来的，不是拍出来的**：`output_path` 确实存在于产品代码，但只是
  `augmentor/backup.py:192` 与 `augmentor/export_enhanced.py:143` 的返回键加一堆局部变量名；
  按「全仓字面量」判据它就成了合法，convert 那条文档错永远红不了。于是这一档限定为
  「路由文件 + 两跳 import 闭包」内的字符串常量。三档跳数都量过：一跳时 convert 闭包 17 个文件，
  `stratify_distribution`（权威在 `augmentor/data_splitter.py:40`）落不进去 ⇒ 误判；两跳 24 个文件，
  正解；三跳 70 个文件，`output_path` 反过来进了 ⇒ 判据失去区分力。两跳由
  `test_hop_budget_is_load_bearing` 两侧同时钉死。
- **首跑三格无据，其中两格是尺子自己的 bug，这点按原样记**：`stratify_distribution` 与 `mixed`
  都不是文档错，是我的相对导入层级换算写错了（`[:level - 1]` 应为 `[:len(parts) - (level - 1)]`），
  于是 `augmentor` 包内部那些 `from .x import y` 一条都没解析进闭包，两跳实际只走成了一跳。
  判据修好之后无据清单只剩 `output_path` 一格 —— **先排除尺子冤枉文档的可能，再谈文档错**。
- **真缺陷的定位与修法**：`/api/dataset/convert` 那格写「必须给 `output_path`」，而请求模型字段是
  `output_file`；`split` 的字段是 `output_dir`；`/api/system/migrate` 与 `/api/system/stream` 两格
  同名字错。四处一起改（总览散文 1 处 + 详细表 3 处），顺手把「同上」这种在 `split` 行上并不
  成立的指代改成点名。散文那一处**不在这把尺子的扫描面里**（它只逐格读详细表），所以另补一支
  整篇文档级判据，防止「表格修干净、注里还写着旧名」。改后 `docs/API.md` 与 `docs/README.md`
  全文再无 `output_path`（grep 空读数）。
- **注入九臂各有牙齿**（红数 2 / 2 / 6 / 1 / 7 / 1 / 3 / 1 / 3）：A1 把文档退回旧字段名（2 红：
  主判据 + 整篇判据）；A2 同一处文档缺陷 **加上** 字面量档退回全仓作用域 ⇒ 主判据**转绿**，
  这就是那档作用域承重的直接证据，A1/A2 必须成对读；A3 不展开 `$ref`（6 红，契约面塌了大半）；
  A4 不收 `parameters`（1 红）；A5 路由装饰器索引扫不到文件（7 红）；A6 摘掉封闭 JSON 词表
  （1 红，`null` 立刻无据）；A7 把局部变量名也算成定义（3 红，正是 `output_path` 当年混进来的路）；
  A8 提取器不再要求小写蛇形（1 红）；A9 跳数放宽一档（3 红）。九臂之后两份文件逐字节复原，
  快照一致。
- **两把参数化棘轮本轮一格未动**：新守卫不用 `parametrize`（31 行的判据按行循环比按行参数化更
  合适，参数化会把 31 格值位变成派生清单、白添盲面）。l97 的 CALLS 402 / 盲面 114 与 l98 的六档
  分布一字未改，**没有回填不是因为豁免，是因为读数确实没变**——全量跑过验证了这一点。
- **新立 A278（自由子对象的盲区）**：响应模型里 `Dict[str, Any]` 的子对象在 OpenAPI 上就是
  `additionalProperties: true`，键名压根不在 schema 里（本轮实测的形状是
  `api/routes/dataset_tools.py:383-384` 那对 `before` / `after`）。这类名字只能靠散文或靠限作用域
  的字面量档接住，一旦来源模块超过两跳就会被误报成无据。方向两条（立最小形状 / 加一档可证伪的
  白名单）都动得到对外面，所以只立账不顺手做。
- **门禁**：**8321 passed / 11 skipped / 0 failed / 覆盖率 99.80%**（195.43 s）。用例增量 `+23`
  与新文件的 23 支正好吻合，既没有别的文件被顺手动过，也没有支数蒸发。账本批写完再复跑同一条
  命令：8321 passed / 11 skipped / 196.95 s，支数一字未变（覆盖率读数取自前一次运行——本轮
  账本批只动 `.md`，而覆盖率统计面只有 `.py`）。
- **仍在账上的邻居**：**A276**（`app` 三键的处置要拍）、**A271** 的 18 只待处置键、**A272**
  （`multilingual` 要不要开成第 5 个可写节）、**A273**（`self.expander` / `self.tracker` 无消费者）
  一字未动；**A277 已在本轮收口**，不再占账。

### L224（2026-10-11）— 消费面第一次有数：75 条操作里前端调到 43 条，缺的 32 条落成名单（A279）

- **这一轮的产出是一把尺子、一个数、一条队列，不是三页新功能。** 目标那句「消费能力达到
  100%」要的是能数出来的数，而此前仓内没有任何机器读过「后端 75 条操作，前端调了几条」。
  `web/src/services/api.wiring.test.ts`（L211 那批）钉的是「`api.ts` 导出的每个函数都在页面
  上被引用」——它管封装之后那一环，管不到「服务层压根没写这条调用」。实测 **43 调 / 32 缺**
  之后，接线本身仍是一串对外功能改动，塞进同一轮只会得到「加了三页但没人验证」的产物；所以
  缺口立成 **A279**，按页面分组分批做。
- **两侧权威各只有一份。** 后端面 = `app.openapi()` 的 `(方法, 路径模板)` 集合（75 条操作 /
  71 条路径，backups 与 dependency 有同路径多方法）；前端面 = `web/src/services/api.ts` 里
  `api.<verb>('路径')` 的形状（43 条）。`baseURL` 从同一份源码读（且断言它恰好出现一次，
  A9 注入臂就是往那儿复制一行，当场红），不在测试里写第二份前缀。语料排除 `*.test.*`，
  与 L211 那条「接线判据不吃测试替身」的教训同口径。
- **query 那一维是量出来才补进去的，而且补的过程本身被实测纠了一次。** 只认写在路径串里的
  `?a=${x}` 会漏掉 axios 的 `params: {…}` 对象——现量 4 条 / 5 个键，其中 `data/load` 的分页
  三键里 `page_size` 恰好是 TS 侧改名（`page_size: pageSize`）后对外的真名；判据必须认冒号
  左边那一半，否则「有人把键写成 `{ pageSize }`」这一类就会蒙过去（钉在
  `test_query_judgment_uses_the_contract_name_not_the_ts_name`）。归属规则最初写成「往后看 400
  字符找 `params:` 块」，被实测否掉：它把 `/versions/history` 的 `limit` 同时算进 rollback 与
  DELETE version 两条根本没有 params 的调用，候选从 **4 涨到 7**。换成「最近的前一个调用」后
  收回 4 条。**间隔上限 60 这个数的诚实读数也写在测试里**：放宽到无穷今天一条都不多收（语料里
  每个 `params:` 块之前都站着另一条调用），收到 3 则把 4 条真归属全判没——所以牙齿在下界，
  上限守的是尚未出现的那种形状。第一版实现还有个把元组解包写错的 bug（`{k: v for k, v, _, _
  in ...}` 把路径当成了值），是「带 query 的调用现量 5 条」这条断言把它逼出来的。
- **一个形状能吃下两条路由时，绑定认运行时而不是认我的匹配规则。** 43 条里恰 1 例：
  `GET /api/versions/history` 逐段看同时符合 `/api/versions/{version_id}`。不做「占位符最少的
  那一档」收敛，字面量调用会把参数化路由也算成已消费，缺口往少里报错一格——尺子往松的方向错
  是最不能接受的方向。为什么字面量那一档是赢的：`api/routes/version.py:156` 的 history 声明在
  `api/routes/version.py:170` 的参数化路由之前，Starlette 按声明顺序取先命中的路由，
  `TestClient` 实测 200 且响应体是 `{"history": […]}`。这条判据的依据是运行时行为加声明顺序，
  不是我的正则；A0 注入臂（把两个路由块整体对调）十臂里红得最少，只响这一支——
  匹配规则本身是顺序无关的，这正是它必须拿运行时证据的原因。
- **`21` 这个数字被实测纠正过一次。** 侦察阶段我把手写映射去重读成 20 个名字，首跑断言
  `21 == 20` 当场红；重数后确认是 **26 条映射 → 21 个不同子命令**（`backup` 4 员共用、
  `dependency` 3 员共用，我漏算了一员）。钉的是复算过的数，不是记忆里的数——这条和本轮
  `page_size` 那次是同一件事的两面：凡手写名单，成员与数量都要机器验。
- **名单的方向性**：`UNCONSUMED`（32 员）与实测未消费集合**相等**，不是子集。所以它只会随接线
  变短；后端新加一条没接的端点当场红（A5 臂：加一条 `GET /api/dataset/injection-probe`，
  2 红——面尺寸 + 名单）。`CLI_ENTRY` 每员指向的子命令名逐个由 `augmentor/cli/parser.py` 的
  `add_parser` 名单验真（A6 臂：把 `aggregate` 改成 `aggregat`，1 红），`NO_ENTRY` 那 6 员的
  判据取「路径末段确实不在 37 个子命令清单里」，两档并起来正好覆盖 32 员且互不相交。
- **被实测否掉的第二条判据**（没做，理由是数）：想加「CLI 子命令与该端点做的是同一件事」这一维，
  做法是比两边的旋钮命名。实测 26 条有 CLI 入口的映射里只有 **15** 条存在 dest/字段名交集
  （CLI 侧 `--input` / `--output`，API 侧字段 `input_file` / `output_file`），硬做交集会把另外
  11 条明明有入口的操作错报成无入口。所以本轮只验「名字存在」，「语义等同」留在 A279 的散文里。
- **注入十臂逐个**：A0 对调两个路由块（1 红，只响运行时那支）、A1 端点名拼错 `/data/list` →
  `/data/lst`（4 红）、A2 撤掉一条已接的调用（6 红）、A3 params 里塞未声明键（3 红）、
  A4 params 用 TS 内部名（2 红）、A5 后端新增无人接的操作（2 红）、A6 CLI 名单指向不存在的
  子命令（1 红）、A7 把字面量路由改名（7 红，最广）、A8 间隔收到 3（2 红）、A9 第二处 `baseURL`
  （1 红）。十臂各有牙齿，脚本与备份在仓外，跑完逐字节还原并核对快照相等。
- **顺手量到 A280（错误分档盲区），本轮不动它。** 核对 `versions/history` 运行时行为时撞见
  `GET /api/versions/v_ghost` → 500，而 SDK 抛的是 `ValueError` 后裔 `VersionError`；普查到
  仓内 8 个路由文件 29 处一刀切、4 个文件 30 处分档、2 个文件混用，且同一份 version.py 里
  404 / 500 / 「200 且 success=false」三种归宿并存。**不修的理由不是「不该修」，是「这是对外
  状态码，且有两支既有用例的 docstring 明写『应返回 500』把现状钉着」**——改它是一批契约变化，
  得与那两支用例同批做，属于 A280 待拍的范围。
- **棘轮未回填，且这不是豁免**：本轮零 `@pytest.mark.parametrize` 新增，l97 / l98 两把参数化
  棘轮的读数（CALLS 402、盲面 114、plain 58、derived 38、盲面用例 109）一格未动。
- **门禁**：**8347 passed / 11 skipped / 0 failed / 覆盖率 99.80%**（196.80 s）。用例增量
  `+26` 与新文件支数吻合（上一轮 L223 收口时是 8321）。
- **仍在账上的邻居**：**A279**（本轮新立的 32 条接线队列，待按批拍）、**A280**（本轮新立的
  错误分档，待拍）、**A278**（自由子对象的形状 vs 可证伪白名单）、**A276**（`app` 三键的处置）、
  **A271** 的 18 只待处置键、**A272**（`multilingual` 要不要可写）、**A273**
  （`self.expander` / `self.tracker` 的消费者还是删除）。
### L225（2026-10-11）— A279 第一批接线：三条「两边都没入口」的操作进页面，消费面 43 → 46

- **这一轮消费的是 L224 那把尺子，不是又造一把。** 只动前端 `web/src`，回填守卫四格
  （形状 43 → 46、缺口 32 → 29、已消费 43 → 46、`NO_ENTRY` 6 → 3），后端源码、后端契约、
  后端测试**一行未改**——A279 立项时写死的「改后端字段名或响应形状一律不做」在这一批里
  字面执行了。

- **第一批为什么是 3 条而不是 6 条。** 优先队列里剩下 `impact` / `semantic-search` / `translate`
  三只：前两只的响应是数组套对象（逐条影响清单、`results` 命中列表），要的是可滚动列表那种
  控件形态，跟本轮「标签卡 + 小表」不是一种东西；`translate` 虽与已接的 `language-profile`
  同属 `multilingual` 一节，但它是**写盘变换类**（产出文件路径 + 逐条失败下标），页面入口要做
  的是「选输入 → 跑 → 把产物接回文件列表」，与本轮三只纯读回的不同族。三只是量得出来的
  一批：三张卡、三只 fetchers、三个类型、八支用例、五支注入臂。

- **一次把契约默认值读错，被自己的注释逮住。** 初稿在 `api.ts` 的注释里写「`include_details`
  契约默认 `false`」，实际 `api/routes/dataset_tools.py:231` 是 `include_details: bool = True`。
  订正后的事实是：页面不取 `details` 那一大坨逐条明细，所以服务层**显式送 `false`**——这句话
  只有在默认值是 `true` 时才有意义，写错的话这条显式传参会被下一个读代码的人当成多余。

- **`vector-status` 单独取、单独报错。** 系统状态页原本一条 `GET /api/status` 撑起整页，
  新增的向量卡走自己的 `useState` + 自己的 `useEffect` + 自己的 `message.error`：它失败只把
  自己降级成一张「向量库状态未取到」的警示，不把已经拿到的服务状态一起拖掉。注入臂 A2
  （删掉整张卡的 JSX）实测只响该页三支新用例、原有四支不动，这条隔离性是量出来的而不是设计的巧合。

- **antd 在 jsdom 下的三条实测事实，是 `pickInNthSelect` 这个 helper 的全部理由。**
  ① 关掉的下拉**不带** `ant-select-dropdown-hidden`，它带的是 `ant-slide-up-leave`，而且新旧两份
  下拉里的选项文案**完全相同**——照原写法用全局 `document.querySelector('.ant-select-item-option')`
  会点到正在关闭的那只，选中失败但一声不响（本轮第一次跑就是这样：`evaluateDataset` 零调用）。
  ② 两只 Select 的 combobox 在 jsdom 下 `id` 都是 `rc_select_TEST_OR_SSR`（`useId` 在测试环境被
  钉成常量），靠 id 把触发器与下拉关联起来这条路走不通，能区分的只有 `aria-expanded`。
  ③ `Statistic` 给整数加千分位分隔符，`1200` 渲染成 `1,200`——断言按渲染文本写而不是按数据写，
  这条也实测过一次（第一版断言 `1200` 直接红）。

- **A4 那一支是先量出假绿才补上的，本轮最值钱的一条。** 画像分布表断言「按条数降序」，
  而初稿 fixture 是 `{ zh: 90, en: 30 }`——插入序恰好与条数序同向，把页面里的 `.sort` 整个摘掉
  照样全绿。改成 `{ en: 30, zh: 90 }`（与注入臂 A4 同一件事的两半）之后，摘掉 `.sort` 才响 1 支。
  如果没跑 A4，这条断言会一直挂着「测了排序」的名字而什么都没测。

- **一支用例写到一半被删掉。** 初稿给语言画像也配了「未选文件时点按钮得到 warning」，但
  整个卡片区块在 `{analysis && ...}` 里，**未选文件时那颗按钮根本不存在** ⇒ 页面里 `handleProfile`
  开头那句 `if (!selectedFile)` 从 UI 不可达（`handleClean` / `handleBenchmark` / `handleVisualize`
  三支同构，是既有风格，本轮不动它们）。可达的那条闸是「生成集已选、参照集未选」，
  于是只把它写成用例（A3 证明它响）。

- **回填的因果要说清：`CLI_ENTRY` 一个数没动。** 本轮只动前端，26 条 CLI 入口映射与去重后的
  21 个名字都保持原值；变的只有 `NO_ENTRY`（6 → 3）和缺口名单。把这条写出来是因为「名单变短」
  很容易被读成「CLI 那边也接了一点」，实际没有。

- **注入五臂（逐个，改真源码 ⇒ 跑守卫 ⇒ 记红数 ⇒ 字节级还原）：** A0 服务层路径拼错
  （`/dataset/language-profile` → `/dataset/lang-profile`）⇒ 消费守卫 **3** 红；A1 保留 import 但
  删掉调用 ⇒ 接线守门 **1** 红（`expected [ 'getVectorStatus' ] to deeply equal []`）；A2 删整张向量卡
  ⇒ 该页 **3** 红、原 4 支绿；A3 双侧必填闸改成单侧 ⇒ **1** 红；A4 摘掉 `.sort` ⇒ **1** 红。
  四个页面/服务层文件还原后用 md5 逐个核过（`git status --porcelain -- augmentor` 只剩本轮真实改动）。

- **顺手量到 A281 与 A282，两条都只立账。** A281 是三处「后端在回、前端类型上不存在」的字段，
  另有两只类型是 naive 正则读不动的形状（联合别名 `types/api.ts:133`、`extends` 继承 `types/api.ts:218`）——
  把它做成守卫要先定解析口径，那是一轮自己的活。A282 是 `prettier --check src` 的基线本身就红
  32 个文件、CI 又没这道门；取证用 `git show` 把 HEAD 版本取到仓外复查，确认本轮没引入漂移，
  所以选「跟周围代码写」而不是顺手重排 32 个文件污染这一批的 diff。

- **棘轮：本轮零 `@pytest.mark.parametrize` 新增**，l97 / l98 两把读数一格未动，未回填；
  这不是豁免，是实测（后端测试总数与 L224 同为 8347 passed / 11 skipped，`+0`）。

- **门禁**：后端全量 **8347 passed / 11 skipped / 0 failed / 99.80%**（198.98 s，账本批之后同命令
  复跑读数不变）；前端 `tsc --noEmit` 与 `eslint . --max-warnings 0` 干净，`vitest run`
  **13 文件 / 127 支全绿**（基线 119，本轮 +8），19.21 s。

- **仍在账上的邻居**：**A279**（本轮划掉 3 条，剩 29 条 / 无入口的 3 条，待拍下一批）、
  **A281**（本轮新立的 3 处响应类型缺字段）、**A282**（本轮新立的 format 基线）、
  **A280**（错误分档一刀切）、**A278**（自由子对象的形状 vs 可证伪白名单）、
  **A276**（`app` 三键的处置）、**A271** 的 18 只待处置键、**A272**（`multilingual` 要不要可写）、
  **A273**（`self.expander` / `self.tracker` 的消费者还是删除）。

### L226（2026-10-11）— A279 第二批接线：检索 / 校验 / 推荐三只进页面，消费面 46 → 49，并新立一张「下拉清单不许与后端各说各话」的尺子

- **这一轮做两件事，主次分明**：① 把三只操作接进已有页面（同 L225 的口径：只动 `web/src`，
  后端字段名与响应形状一字未改）；② 给「接进来的两个下拉框」补一把跨语言对账尺子
  `tests/unit/test_wired_dropdown_choices_l226.py`。第二件事不是顺手加的：接线把后端的一张
  封闭清单搬成了 TSX 里的字面量数组，而**这份字面量与 Python 权威表之间没有任何编译期连接**，
  后端加一档方法而页面不动，用户不会收到任何报错，只会「永远选不到那一档」——
  这正是 L92 / A162 量过的同一形状（能力做完了，产品面只走完一半）。

- **三只分别落在哪、为什么那样渲染**（每一条都是契约要求的两侧读数，不是审美选择）：
  `POST /api/dataset/search` → 数据分析页。响应里 `total_matches` 是**全量命中数**、`items` 是
  **本次返回的那一页**（`limit` 默认 100），两个数分开渲染成两个标签，写成一个是说谎；
  `highlights` 那支实测**只对 contains / ngram / fuzzy 产出**，`regex` 与 `exact` 有命中而高亮清单为空
  （仓外探针实测，见下条），所以那一档显式出一句「方法 regex 不产出高亮片段，命中数以
  『命中 N 条』那个计数为准」，而不是让用户以为零高亮等于零命中。
  `POST /api/dataset/validate` → 质量中心新 tab。`error_count` / `warning_count` 按全量算，
  `issues` 只是前 100 条样本，所以那张卡的标题直接把这件事写出来；`is_valid` 与「有警告」
  是两回事，三态 Alert 分开（未通过 / 没有错误但有警告 / 全部通过）。
  `POST /api/dataset/auto-config` → 配置管理页。它回的是**建议值**，不是生效值，所以
  「填进上面的表单」那支按钮只写表单状态、明确不调 `updateConfig`（提示语就写着「确认无误后再点
  『保存配置』」）；`source_stats.quality_pass_rate` 实测可以是 `null`（那份数据集没有可观测的
  质量面），渲染成「无观测」而不是 `0`，`recommended_sample_size` 不进表单——表单里没有这一格，
  硬塞一个字段就是把推荐面冒充成配置面。

- **先量后写**：接线前用仓外探针（`D:/WORKSPACE/probe_l226*.py` 三档）拿真实 200 响应逐字段读过一遍，
  三条结论直接决定了上面的文案与类型：regex 的 `highlights` 为空数组、`issues` 是样本不是全量、
  `auto-config` 的 `reasoning` 是**清单**（逐项列成 `<li>`）而不是字符串。
  探针文件与它写过的 `data/l226d_probe.json` 全部删除，`data/` 整目录本就在 `.gitignore:123` 之外不入仓。

- **守卫回填五格，全部按实测**（一把尺子都没放松）：`test_api_endpoint_consumption_l224.py` 的
  前端面规模 **46 → 49**、缺口名单 **29 → 26**（划掉 search / validate / auto-config 三格）、
  已消费 **46 → 49**、`CLI_ENTRY` **26 → 23 条 / 21 → 18 个不同子命令**、`NO_ENTRY` 仍 **3**。
  回填前它红了三格（`test_frontend_face_scale`、`test_roster_scale_is_pinned`、`test_cli_roster_scale`），
  正是这把尺子在只改前端时想要的反应；`test_unconsumed_matches_the_roster` 在划掉名单之后转绿。

- **新尺子的三层判据**：① 双向集合相等 + 顺序相等 + 扫描非空转（只抠到一两项就说明正则在空转）；
  ② **label 里写死的数字必须真是规则表里的值**（`basic（上限 1000 / 5000）` 那两个数直接读
  `PRESET_RULES["basic"]` 验证，`strict` 那句「加长度下限与禁用片段」验 `min_*_length` 与
  `forbidden_patterns` 真在场，`chat` 那句验 `history` 真在字段清单里）——比集合相等更严，
  因为它是「UI 对用户的宣称」；③ 端点面回执：5 档方法 + 3 档预设各自真发一次请求拿 200，
  清单外两档（`method=substring`、`preset=default`）必须回**字符串** detail，
  这样前端 `apiErrorDetail` 的主支才不是想象出来的形状。
  另加两格「页面初值 == 请求模型的默认值」（`SearchRequest.method` / `ValidateRequest.preset`），
  钉住 TSX 注释里那句「初值与契约默认同值」——`default` 曾长期是 `preset` 的默认值而它根本不存在
  于预设表（L189 的事故），这一格就是那道防线的前端侧。
  **与既有尺子的分工写进文件头**：`l175` 管 SDK/API 共引、`l189` 管 CLI `--preset` 与派生白名单，
  本文件不重复；CLI 的 `--method` choices 那份手写字面**从没被比过**，本文件第一次把它入尺。

- **五档注入臂，逐臂红格数按实测记**：M1 页面删掉 `ngram` 那一档 ⇒ **3** 红（非空转 + 每档选得到 + 顺序）；
  M2 把页面默认值从 `contains` 改成 `exact` ⇒ **1** 红（初值 vs 契约默认）；
  M3 把 `basic` 的 label 上限改成假数字 ⇒ **1** 红（label 宣称）；
  M4 把 CLI `--method` choices 删一档 ⇒ **1** 红（三面对账的那只新面）；
  M5 预设下拉删掉 `chat` ⇒ **4** 红（非空转 + 对 API 白名单相等 + 每档选得到 + chat 那句宣称）。
  变异前每档都先 `assert count(old) == 1` 保证替换点唯一；跑完用 md5 逐文件核对还原
  （`还原校验: True`），工作树只剩本轮真实改动。

- **拒接两员，理由可证伪**：`GET /api/system/dependencies` 的数据与已接的 `/api/status` 里的
  `dependencies` 字段是同一份东西的两面，接进来只把「消费率」这个数往上涨、不涨任何用户能力
  ——**这条判断本身是新立的一档纪律：指标涨而能力不涨的接线一律不做**。
  `POST /api/dataset/translate` 的问题在契约层：`TranslateResponse.items` 是整份译文，
  请求模型没有 `output_file`、路由不走 `_dump`，而同仓所有写盘变换类端点都有落盘口；
  CLI 侧也没有 `translate` 子命令（它是 `NO_ENTRY` 三员之一）。这条另立 **A283**（先补契约、再接前端），
  本轮不做契约改动。

- **前端面的三件收口判据齐**（沿用 L225 那批）：名单与实测集合相等且变短、
  `web/src/services/api.wiring.test.ts` 的 `KNOWN_UNWIRED` 仍为空、每员覆盖成功与失败两路。
  前端 `vitest run` **13 文件 / 140 支全绿**（基线 127，本轮 **+13**：Analysis 9 → 15、
  Quality 4 → 8、Settings 5 → 8）。本轮量到的三条 jsdom/antd 事实都写进了用例注释：
  两字 CJK 按钮的文本被 antd 渲染成「检 索」，`getByRole('button', {name: /检索/})` 直接落空
  （第一版 6 支全红就是这个原因，改成「运行检索」后全绿）；关闭中的 dropdown 仍带同一份
  option 文本，只能靠 `aria-expanded` 与「最后创建的那层」区分；`Tabs` 只渲染活动面板，
  所以校验那张表要「切 tab 前查不到、切 tab 后才查得到」这样断。

- **棘轮：本轮新增 2 支 `@pytest.mark.parametrize`**（5 个方法 / 3 个预设，值位是内联**字符串**字面清单，
  pytest 直接渲染成 `[exact]`、`[basic]` 这样的可读 id）⇒ l97 的 CALLS **402 → 404**、
  READABLE **288 → 290**、盲面 **114 一格未动**；l98 的档位和与四档名册实测复跑一字未动，
  两把尺子的读数各由自己的判据独立咬合（回填前第一次全量门禁就是这 2 支红）。

- **门禁**：后端全量 **8370 passed / 11 skipped / 0 failed / 覆盖率 99.80%**（189.99 s；
  与 L225 的 8347 相减 `+23` = 新守卫文件 23 支，接线本身零后端用例增量）；
  前端 `tsc --noEmit` 与 `eslint .` 均 0 退出、`vitest run` 21.84 s 全绿。
  账本批之后再同命令复跑一遍，读数不变。

- **顺手一格**：配置管理页「智能去重」那张卡的开关标签一直写着「启用水重」（错字，从 `HEAD` 起就在），
  本轮改回「启用去重」。它没有任何测试覆盖，因为文案本身不是判据；记在这里是为了让下一轮
  「页面文案与术语表对账」这类活有个起点。

- **仍在账上的邻居**：**A279**（剩 26 条，其中 `NO_ENTRY` 3 条；下一批建议 `impact` + `semantic-search`）、
  **A283**（本轮新立：translate 无落盘口）、**A281**（3 处响应类型缺字段）、**A282**（format 基线红且 CI 无门）、
  **A280**（错误分档一刀切）、**A278**（自由子对象的形状 vs 可证伪白名单）、
  **A276**（`app` 三键的处置）、**A271** 的 18 只待处置键、**A272**（`multilingual` 要不要可写）、
  **A273**（`self.expander` / `self.tracker` 的消费者还是删除）。

### L227（2026-10-11）— A279 第三批接线：impact / semantic-search 进数据分析页，消费面 49 → 51

- **这一轮做两件事**：① 把 `NO_ENTRY` 队列里最后两只「SDK 做完、web 与 CLI 都没人取」的操作接进
  数据分析页（口径同前两批：只动 `web/src`，后端字段名与响应形状一字未改；`translate` 的问题在契约层，
  留给 A283）；② 新立 `tests/unit/test_impact_semantic_wiring_l227.py`（**20 支**），管住这一批特有的
  漂移面——**页面把请求模型的默认值抄进了 `useState`**。抄的东西会漂，而它一漂用户不会收到报错，
  只会拿到一个和契约不一致的「本次生效值」。

- **两张卡各按契约渲染成什么**（每一条都是响应本身的要求，不是审美选择）：
  语义相似检索：`query=""` 端点回 **200**、`total` 等于条目数、每条 `score` 全 `0.0`——一份「全都相关」的
  退化读数，所以门口拦空串而不是发出去；`top_k=0` 同样 **200** 且 `results=[]`，是合法的「只要回显、
  不要命中」那一档，所以 UI 下界从第一版写的 `min={1}` 放宽到 **0**（第一版把 0 挡在门外，等于页面自己的
  控件推翻了自己注释里的契约口径）；**后端 / 维度 / 集合**三个生效值标签即使 0 命中也照样渲染，
  否则用户分不清空的是检索面还是数据面。`text_key="nope"` 实测是 **200 + `text` 全空串的静默退化**，
  这一档只能靠「下拉里只放数据集真有的字段」来防，不能指望后端出声。
  增强前后影响：`gains` 四项逐项标量纲（比值与绝对差的方向不同，混为一谈就是谎），清单外的键不静默丢而是
  显式出「未知口径」；判定卡的文案随门槛实时写「本次 `${minScaleGain}`」——门槛可调之后还写死「默认 0.0」
  就是一句过期话；快照两侧各一张表，整数与小数分开格式化（`formatSnapshotValue`），
  **条目数渲染成 `4.0000` 是假精度**，第一轮就是这样量出来才改的。

- **为什么不能扩 l226 那把尺子**（本轮最重要的一条判断，写进了文件头）：l226 靠的是 Python 侧有可 import 的
  封闭清单常数（`SEARCH_METHODS` / `VALIDATION_PRESETS`）。这两只操作**没有**这种清单——`text_field` / `text_key`
  的合法取值来自数据集本身，而 SDK 里那份 `["instruction", "input", "output"]` 只是 `augmentor/audit.py:68`、
  `augmentor/privacy.py:76` 与 `augmentor/cli/parser.py:173 / 405 / 430` 的**五处手写字面量**，
  引其中任何一处当权威就是自造一套。所以权威换成三处：**默认值 → 请求模型 `model_fields[...].default`**；
  **增益键集 → 真实回执里的 `gains` 键**（双向对账，页面不许漏说明也不许多说）；
  **快照键集 → `AugmentationMetrics` 的 dataclass 字段**，唯一豁免 `extra`，豁免用
  `Exclude<keyof ImpactSnapshot, 'extra'>` 由**类型**声明而不是运行时 if。
  字段名那一档不做清单守卫，改做「页面发得出的每一档都被端点接受」（`TestEndpointAcceptsWhatTheUiSends`）。
  **没有权威常数就不假装有一把尺子**——这条取舍比多写几支断言更值得记账。

- **新暴露的两档旋钮**：`text_key`（拿哪个字段做向量匹配）与 `min_scale_gain`（抬高 `beneficial` 的门槛）
  此前在**任何产品面都调不到**——CLI 的 `add_parser` 选项里查不到这两个 flag（实测 grep 零命中），
  SDK 侧只作为构造参数存在。接线之后它们是页面上可读可改的档位，`beneficial` 的判定口径
  （规模增益 ≥ 门槛 ∧ 去重增益 ≥ −0.05）也随门槛实时显示，用户看得见自己定的门槛。

- **`unique_instructions` 是误名**（实测，不是推断）：`augmentor/impact.py:76` 那一格赋的是 `len(set(texts))`，
  而 `texts` 取自 `self.text_field`（默认 `instruction`）；把字段换成 `output`，它数的就是 output 的唯一值，
  名字却还写着 instructions。本轮**只改标签不改键名**——键名在契约面上，改它就不是接线了。
  页面标签写「唯一值数（按所选字段）」，守卫钉住「标签必须含『字段』、不得含『指令』」；
  键名处置另立 **A284**，那五处重复字面清单立 **A285**。

- **守卫回填四格 + census 重钉，全部按实测**（一把尺子都没放松）：
  `test_api_endpoint_consumption_l224.py` 前端面规模 **49 → 51**、缺口名单 **26 → 24**（划掉这两格）、
  已消费 **49 → 51**、`NO_ENTRY` **3 → 1**；`CLI_ENTRY` 的 **23 条 / 18 个名字**一格未动，因为本轮只动前端。
  l97 的 CALLS **404 → 405**、READABLE **290 → 291**、盲面 **114 未动**；l98 的档位和与盲面名册（109 支）
  实测复跑一字未动。原因是本轮唯一新增的那支 parametrize（`["impactField", "semanticTextKey"]`）
  值位是内联**字符串**字面清单 ⇒ 只进 CALLS / READABLE 两档，不进盲面——两把尺子各自独立闭合。

- **注入十二臂，逐臂红格数按实测记**（Python 面 7 臂、vitest 面 5 臂）：
  M1 页面 `top_k` 初值漂到 3 ⇒ **2** 红（契约对账 + 扫描非空转）；M2 门槛初值漂到 1 ⇒ **2**；
  M3 摘掉 `dedup_gain` 的量纲条目 ⇒ **2**（键覆盖 + 「哪几项是比值」那句宣称）；
  M4 摘掉快照里的唯一值行 ⇒ **3**；M5 让标签跟着键名说谎 ⇒ **1**；
  M6 服务层替端点写第二套默认 ⇒ **1**；M9 删掉 `impact` 的服务层调用 ⇒ **3**（幽灵方向 + 名单相等 + 名单规模）。
  vitest 面每臂各 **1** 红：M7 双侧快照都读 `before`、M8 撤掉空查询门口闸、M9b 语义检索不再递编码字段、
  M10 门槛变了而口径文案还说默认值、M11 清单外的增益键被静默丢掉。
  **第一批把 M7 / M8 / M9b / M10 / M11 读成了「空转」，那是取证脚本自己的 bug 而不是结果的 bug**：
  pytest 与 vitest 的汇总行带 ANSI 色码，正则截到 `Tests 行：1 ` 就断了、用例名的 `×` 也匹配不上；
  剥色（去掉 ANSI SGR 转义序列）并置 `FORCE_COLOR=0` / `NO_COLOR=1` 重跑，五臂全部各点名为 1 支红。
  **那条假读数没有入账**。变异前每臂先 `assert count(old) == 1` 保证替换点唯一，跑完按 sha256 逐文件核对还原
  （十二臂全部字节一致），工作树只剩本轮真实改动。

- **前端面的三件收口判据齐**（沿用 L225 / L226）：名单与实测集合相等且变短、
  `web/src/services/api.wiring.test.ts` 的 `KNOWN_UNWIRED` 仍为空、每员覆盖成功与失败两路。
  前端 `vitest run` **13 文件 / 149 支全绿**（基线 140，本轮 **+9**：Analysis 15 → 24），
  `tsc --noEmit` 与 `eslint .` 均 0 退出。
  本轮量到的一条 antd 事实：`InputNumber` 的 `addonBefore` 会打 deprecation 警告，而**全仓只有本轮那两只新旋钮在打**
  ⇒ 按仓内既有写法改成控件前一个 `<span>标签</span>`，警告归零；测试侧为这两只数字框加了 `numberInputs(container)`
  （index 0 = 前 K 条，1 = 规模增益门槛），因为 `getByRole('spinbutton')` 分不开同页多只。

- **门禁**：后端全量 **8390 passed / 11 skipped / 0 failed / 覆盖率 99.80%**（188.88 s；
  与 L226 的 8370 相减 `+20` = 新守卫文件 20 支，接线本身零后端用例增量）；
  账本批之后再同命令复跑一遍：**8390 passed / 11 skipped / 0 failed**，读数一格未动，而这一次整轮
  **270.65 s**（比首次的 188.88 s 高 43%）却照样绿——「更慢的一轮反而不红」正是 A286 那格归因的依据之一。

- **两条「尺子咬到我」的实测**（都不放宽阈值，只记账）：① 账本第一版在 A285 的处置建议里写了一个**尚不存在的新模块名**，`tests/unit/test_comment_refs_a184.py` 当场红两支（dead_path 硬零 + 注入臂那支的计数也对不上）⇒ 按这把尺子的口径改口「不指名新模块」，注释引用面重新归零——这是尺子按设计咬住了散文里的想当然。② 全量门禁复跑第一次是 **3 failed / 8387 passed / 11 skipped**：除 ① 的两支，`test_analytics_performance.py` 的 `stays_linear` 比值实测 **4.38**（阈值 3.0）也红。同一条用例本轮第一次全量（8390 全绿）是绿的，之后**隔离复跑四次全绿**，而红那一次整轮 **257.50 s**（比 188.88 s 高 36%）⇒ 判为并发负载下的墙钟噪声而不是算法退化，**阈值一格未动**。这条噪声本身立成 **A286**，因为「退化档与噪声档撞在同一个数上」会让下一次真退化被当成噪声放过。
- **仍在账上的邻居**：**A279**（剩 24 条，其中 `NO_ENTRY` 1 条 = `translate`，归 A283）、**A283**、
  **A284**（本轮新立：`unique_instructions` 误名）、**A285**（本轮新立：五处 `["instruction", "input", "output"]`
  手写字面清单没有权威）、**A286**（比值型计时守卫会假红）、**A281**（3 处响应类型缺字段）、**A282**（format 基线红且 CI 无门）、
  **A280**（错误分档一刀切）、**A278**（自由子对象的形状 vs 可证伪白名单）、**A276**（`app` 三键的处置）、
  **A271** 的 18 只待处置键、**A272**（`multilingual` 要不要可写）、**A273**（`self.expander` / `self.tracker`
  的消费者还是删除）。

### L228（2026-10-11）— A279 第四批：`stats` / `features` / `compare` 接进数据分析页，并把「后端不出声的两档旋钮」钉回真实回执

- **这一轮做两件事**：① `POST /api/dataset/stats`、`POST /api/dataset/features`、`POST /api/dataset/compare` 三条进数据分析页（三张新卡 + 三处入口 + 该页用例），**只动 `web/src`**，后端字段名与响应形状一字未改；② 新立守卫 `tests/unit/test_stats_features_compare_wiring_l228.py`（30 支），并把顺手量到的两笔契约面问题按「不动，只立账」处置成 **A287**（`similarity_to_a` / `similarity_to_b` 恒零死面）与 **A288**（`size_ratio` 的 `inf` 上线为 `null`）。

- **三张卡各按契约渲染成什么**：字段级统计表把逐字段那十二个键（`total_count` / `filled_count` / `empty_count` / `fill_rate` / `avg_length` / `min_length` / `max_length` / `median_length` / `std_deviation` / `unique_count` / `top_values` 加 `field_name`）与三项质量指标分开展示，**每项指标各带一行口径**，其中三行都写明「与 `fields` 无关」；特征普查卡只读 `field_features` 的 `feature_type` 与 `coverage`，**页面不装 `min_coverage` 旋钮**（出厂 `0.1`，`augmentor/feature_detect.py:59`，而该端点请求模型只有 `input_file`——实测多送 `text_field` 被静默忽略，装了就是假可调）；双集对比卡是 A/B 双侧必填，重叠面九项按三种量纲分组排（计数 / 带符号差 / 0–1 比值，`size_ratio` 的 80% 不是「80 条」），字段对照明细做成可展开表。

- **为什么这批必须换一条判据**：l226 那把尺子量的是「下拉清单 vs 可 import 的 Python 封闭常数」，这一族没有那种常数（合法取值来自数据集本身）；l227 退到「页面发得出的每一档都被端点接受」，而这一批连那条都不够用——**实测这两档旋钮给错值时后端不出声**：
  - `fields:["nope"]` ⇒ **200**，回执 `field_statistics` 里多出一整只**十二键全零的幽灵字段**（`filled_count 0`、`empty_count 4`、`top_values []`），而 `summary` 散文还照它写「包含 1 个字段」；
  - `key_fields:["nope"]` ⇒ **200**，而且方向更坏：4 条与 2 条、`instruction` 一句都不重的两份数据回 `common_items 1`、`similarity_score 1.0`，同一条数据默认键给的是 `0` 与 `0.0`（真话）⇒ **幽灵匹配键把「毫无重叠」谎报成「完全重叠」，这是假阳性，比一格零更贵**。
  所以判据换成**候选只能来自回执**：两支无字面清单的 parametrize 钉住两个下拉里不许出现任何手写字段名（`TestCandidatesComeFromTheReceipt`），再加一支拿真实回执的键集逐个发一遍、全部要 200，证明「页面发得出的每一档」这条不会自己烂掉。

- **一条被探针推翻的想象**（留在账上，不是埋在日志里）：守卫初稿断言「换匹配键会让逐条值差异从空变成逐条列出」，读了 `augmentor/comparison.py:217` 与 `augmentor/compare_enhanced.py:289` 才知道 `_compare_fields(items_a, items_b)` **压根不接 `key_fields` 参数**——配对永远按 `instruction`，明细条目键固定是 `{instruction, value_a, value_b}`，`key_fields` **只**动 `metrics` 那一组数。订正落在三处（`web/src/types/api.ts` 的类型说明、`Analysis.tsx` 的两条注释、页面用例的 fixture 与列头期望），并钉成**正向不变量**用例 `test_key_fields_do_not_move_the_per_field_detail_table`（三跑逐字节相等）。这条之所以值得单独写一段：它不是「文档少写一格」，而是**上一轮我自己写进类型注释的一句假话**，如果不发探针就会一直是对外的口径。

- **其余按探针入账的事实**：空数据集（`[]`）⇒ `field_statistics` / `content_statistics` / `quality_metrics` **三个都是 `{}`**，`content_statistics` 不是「有结构但值为零」而是两个子键根本不出现 ⇒ TS 里两个子键收成可选；两侧同一份文件 ⇒ `winner == "tie"`、`quality_diff` 四项全 `0.0`（平局判据是 `abs(avg_a − avg_b) < 0.01`，`augmentor/comparison.py:260`）⇒ 页面按**三态**渲染而不是「有没有胜者」；`similarity_to_a` / `similarity_to_b` 全仓无赋值点 ⇒ 恒 `0.0`，页面不渲它（**A287**）；B 侧 0 条时 `size_ratio` 在 SDK 是 `float('inf')`（`augmentor/compare_enhanced.py:190`）而线上是 **`null`**（响应体仍是严格 JSON，探针逐条验过）⇒ TS 收成 `number | null`，页面读到 `null` 显示「—（B 侧 0 条：SDK 存 `inf`，JSON 面为 `null`）」（**A288**）。

- **守卫回填五格 + census 重钉，全部按实测**：调用形状与已消费 **51 → 54**、缺口名单 **24 → 21**（划掉的正是这三条）、`CLI_ENTRY` **23 条 / 18 名 → 20 条 / 15 名**（L228 三条都各有 CLI 子命令，所以动的正是这一半——与 L225 / L227 两批「只动前端、这一半一格未动」是对照的两类形状）、`NO_ENTRY` 仍 **1**、契约总数 **75 / 71** 未动。census 两把：CALLS 405 → **406**、READABLE 291 → **292**、盲面 **114** 与 l98 档位和**未动**（本轮唯一新增的 parametrize 值位是内联字符串清单，落可读档）。

- **注入八臂，逐臂红格按实测记**：Python 面六臂各点名**单支**（① 质量口径删掉「与 `fields` 无关」② 限定字段下拉改成手写字面量 option ③ 重叠九项清单摘掉 `size_ratio` 一行 ④ TS 把 `size_ratio` 收成纯 `number` ⑤ 页面把恒 0 的死面渲出来 ⑥ 服务层给 `key_fields` 编第二套默认），vitest 面两臂各单支（⑦ `raw === null` 改写成 `undefined` ⇒ B 侧空集渲成 `NaN` ⑧ 双集对比的必填闸只看 A 侧）。八臂 rc 全 1 且输出含 `failed`；**判红的口径刻意是「rc≠0 且输出含 `failed`」**——node id 写错时 pytest rc=4、vitest 的 `-t` 一条没匹配上时压根没有 `failed` 行，那些都不是取证。三个被改文件（`Analysis.tsx` / `types/api.ts` / `services/api.ts`）逐臂字节复原，收尾再重新验盘一次并断言与进入时一致。**行为锁没有注入臂**（`field_comparisons` 不随 `key_fields` 变、`size_ratio` 上线为 `null`）：那两臂要改的是 SDK 而不是页面，改 SDK 来证前端守卫不是恒真，归因就错了。

- **前端面的三件收口判据齐**（A279 从第一批起没换过，本轮第四次命中）：消费守卫的名单与实测集合**相等且变短**、`api.wiring.test.ts` 的 `KNOWN_UNWIRED` **仍为空**、三员各覆盖**成功与失败两路**。三门禁 `npx tsc --noEmit` / `npx eslint . --ext .ts,.tsx` / `npx vitest run`：前两者干净，vitest **13 文件 149 → 163 支全绿**（`Analysis.test.tsx` 24 → 38）。接线改动本身让消费守卫先红再回填——红的那几次就是证据。

- **中间读数留着不美化**：全量门禁**第一次**是 **2 failed / 8418 passed / 11 skipped**（223.36 s），两支都来自 `test_comment_refs_a184.py`（硬 0 档的 `dead_line` + 那只「逐桶注入各红一次」的计数臂，后者跟着前者的格数动）。归因见下一条。修完两处同值改口后：六把账本与文档守卫 **149 passed / 8 skipped**，新守卫 30 支、l226 + l227 + l228 三把合跑 73 支、l224 26 支全绿。第二次全量 **8420 passed / 11 skipped / 0 failed / 覆盖率 99.81%**（238.55 s；与 L227 的 8390 相减 `+30` = 新守卫 30 支，接线本身零后端用例增量）。

- **一条「尺子咬到我」的实测**：我在类型注释与新守卫 docstring 里都把平局判据的行号写成 259，而 **259 行是空行**，那句 `if abs(score_a - score_b) < 0.01:` 实落在 **260**（`augmentor/comparison.py:260`）⇒ A184 的硬 0 档当场红两支，把那两条注入臂的计数一起带动。处置走的是纪律里的第 2 优先：**同值就地改口、把多插的行收回**，`git diff --numstat` 复核净行数为零，阈值一格未动。这把尺子的反应是对的——它判的是「引用落点必须是代码跨度」，而我写下的那个数来自上一次读数之后的漂移。本条自己也注意别再落一个 `:259` 进去：那种形状一旦被 A184 读到就是新的 dead_line，叙述一个已修的坏引用不需要复刻它的字面形状。

- **仍在账上的邻居**：**A279**（剩 21 条，其中 `NO_ENTRY` 1 条 = `translate`，归 **A283**）、**A287** 与 **A288**（本轮新立）、**A284**（`unique_instructions` 误名，本轮继续只动前端标签）、**A285**（五处 `["instruction", "input", "output"]` 手写字面清单没有权威 ⇒ 正是这批「候选只能来自回执」这条判据建不起来的根）、**A286**（比值型计时守卫会假红）、**A281**（3 处响应类型缺契约已有字段：`AppConfig` 少 `versioning` / `visualization`、`UpdateConfigResponse` 少 `ignored_keys`、`StatusResponse` 少重试两键）、**A282**（format 基线红且 CI 无门）、**A280**（错误分档一刀切）、**A278**（自由子对象的形状 vs 可证伪白名单）、**A276**（`app` 三键的处置）、**A271** 的 18 只待处置键、**A272**（`multilingual` 要不要可写）、**A273**（`self.expander` / `self.tracker` 的消费者还是删除）。


### L229（2026-10-11）— A279 第五批：`convert` / `merge` / `split` 进数据管理页，并把「写边与读边各只有一份格式清单」钉成判据

- **这一轮做两件事**：① `POST /api/dataset/convert`、`POST /api/dataset/merge`、`POST /api/dataset/split` 三条进数据管理页（三张新卡 + 三处入口 + 该页用例），**只动 `web/src`**，后端字段名与响应形状一字未改；② 新立 `tests/unit/test_transform_wiring_l229.py`（**55 支**）与渲染面 `web/src/pages/DataManagement.transform.test.tsx`（**13 支**），并把顺手量到的三笔契约面问题按「不动，只立账」处置成 **A289**（分割两支判据不同）、**A290**（合并四档只接一档、六键只上线五键、docstring 写着「一一对应」）、**A291**（空 `source_format` 的判决缺主语）。

- **为什么这批要换判据**：l226 量「下拉清单 vs Python 封闭常数」、l227 退到「页面发得出的每档都被端点接受」、l228 再退到「候选只能来自回执」。这一族**两张格式清单都各有唯一权威**，所以判据回到最硬的那一档——**逐项相等**：转换目标下拉 == `OUTPUT_FORMAT_CHOICES`（`augmentor/converter.py:91`，十二档），源格式下拉复用上传那份 `UPLOAD_FORMAT_CHOICES`（十三档），两者**只差一个 `xls`**，而那道差不是笔误：`EXCEL_TARGET_NAMES`（`augmentor/converter.py:56`）正是它的由来——写边刻意不认老 BIFF 容器。端点拒 `xls` 时**自己枚举的那十二档**与页面清单逐项相等 ⇒ 页面编不出第十三档，也删不掉一档。

- **改前实测（这一批的语料，全部现量，没有一条是推算）**：目标格式十二档逐个发过去**全 200**，`xlsx` 的回执报的是内部名 `excel`（写 Excel 认两种拼法，但落盘扩展名必须 `.xlsx`，两者不匹配是 **400**，而不是「落一份名字与内容不符的文件」）；合并的去重开关**真的动产物**（同一对文件：开 ⇒ `(2, 10, 6, 4)`、关 ⇒ `(10, 10, 0)`，四个数分别是输入份数 / 总进 / 总出 / 删掉）；分割三段键**恒在**，`test` 那一段是 `count: 0` 而不是缺键；`inputs: []` 是 **400** 而不是「写一个空文件」；`output_dir: ""` 是 **400**；三条路径缺输入文件都是 **404 且 detail 是字符串**（不是对象）。

- **一条被探针推翻的想象**：初稿断言「输出写绝对路径 ⇒ 403」。实测**在白名单根内的绝对路径是 200**，根外才是 403，而 `..` 是 400（`路径包含非法组件`）⇒ 三档分开钉成一支带字面 `ids` 的 parametrize（convert / merge / split 各一格，字段名与合法值 / 穿越值随格给）。这条如果没发探针，守卫就会把「根内绝对路径」这个**合法**用法判成缺陷。

- **两条静默形状写进卡片（页面替用户把坑说出口）**：**A289** 的容差与负数两条判据差（默认支 `0.01`、分层支 `1e-6`；负数只有分层支拦）、**A290** 的「去重阈值在这只端点上不可调」、以及分层字段给一个不存在的名字**不报错**——它把全部条目**塌成一个分组**，分段数看起来与用默认键时一样。少写这几句，用户会把「塌成一个分组」读成「分层生效了」，把一个开关读成整套去重策略。

- **八臂「初值 == 契约默认」逐个对账**：`target_format="jsonl"`、`source_format=None`、`deduplicate=True`、`0.8/0.1/0.1`、`seed=None`、`stratify=False`、`stratify_key="instruction"`、`shuffle=True`；页面留空的那两格（`seed` / 源格式）在契约里默认就是 `None`，`output_file` / `output_dir` 契约里**必填**，页面刻意**不给默认**（空着就发不出去，并给一句「点名输出文件名」的提示）。服务层全文只有**一处** `|| undefined` 哨兵（`source_format: opts.sourceFormat || undefined`）——这一处正是 A291 那道坑的挡板，多一处就是第二套默认。

- **census 两把按现量重钉（不许照抄上一轮）**：l97 CALLS 406 → **411**、盲面 114 → **118**、READABLE 是差值口径 ⇒ 292 → **293**；l98 档位和同步 **118**（plain 58 与 derived 38 未动，**opaque 10 → 11** ＝值位是 `OUTPUT_FORMAT_CHOICES` 这个名字引用、Scope 彻底解析不到；**ids-derived 8 → 11** ＝三支写了 `ids=` 但 id 由推导式现造），盲面用例名册 109 → **113**，整支豁免用数 17 → **18**（「绝对路径 / `..`」那一支的每支 parametrize 都写了字面 ids）。**五支新 parametrize 里只有一支落可读档**，所以 READABLE 只 +1 而盲面 +4——两边独立闭合，不是同一个数抄两遍。

- **注入八臂，逐臂红格按实测记**：Python 面五臂各点名**单支或单类**（① 回执吞掉 `output_file` 那一格 ⇒ `TestReceiptCoversEveryResponseKey` 红；② 服务层把 convert 的路径写成 `/dataset/convertr` ⇒ 「三只 fetchers 只打这三条写盘路径」那支红；③ 页面自己往目标清单里加 `xls` ⇒ 格式清单那一类红；④ 契约默认改一格（`train_ratio` 0.8 → 0.7）⇒ 八臂初值对账那一类红；⑤ 删掉「塌成一个分组」那句诚实口径 ⇒ 卡片文案那一类红），vitest 面三臂各**单支**（⑥ 目标清单多一档 `xls` ⇒ 「渲染十二项且不含 xls」红；⑦ `train` 初值替契约编一个默认 0.7 ⇒ 「初值就是契约默认」红；⑧ 删掉「负数只有分层支会拦」⇒ 「卡上写着两条支路判据差」红）。八臂 rc 全 1 且输出含 `failed`（vitest 三臂各 `1 failed | 12 passed (13)`），被改的三个文件（`DataManagement.tsx` / `services/api.ts` / `api/routes/dataset_tools.py`）逐臂字节复原，收尾重新验盘并断言哈希与进入时一致（`da10bb97b864` / `b0d77707eb37` / `89877d7bdbbf`）。

- **中间读数留着不美化**：① **vitest 注入探针 v1 的红判据是坏的**——它只对输出**尾部 700 字**判「含 `failed`」，而 vitest 把失败摘要印在错误转储里被截掉了，三臂全被读成 NOT-RED，其中第三臂的输出明明带着 `expect(page).toContain('负数只有分层支会拦')` 那一格红。订正成对**全量**输出判、每臂输出落盘、并把 vitest 自己报的 `Tests N failed` 摘出来之后才是真读数（三臂各 `1 failed | 12 passed`）。**假读数没有入账**，但这条要记下来：判红的口径本身就是取证的一部分，截尾等于自废。② 渲染面用例第一版连撞四次形状，全部按实测改判据而不是改页面：`vi.mock` 的工厂里碰不到顶层常量（`Cannot access 'CONVERT_OK' before initialization`）⇒ 夹具与 spies 一起搬进 `vi.hoisted`，且工厂里只能走 `fx.spies.*` 属性访问（解构语句本身也是被提升的顶层语句）；`selectedFile` 是**异步**落的（`loadFiles` 之后才有），没等就点按钮 ⇒ 页面弹的是「需要选中源文件」那句提示，于是每支用例先等文件落位；antd 的 loading 图标会**并进按钮的可达名**（role dump 里是 `Name "loading转换选中的文件"`），改成子串正则匹配；`InputNumber` 按 step 补零，初值读出来是 `['0.80', '0.10', '0.10', '']` 而不是 `['0.8', ...]`（线上发的仍是数字 0.8，就地重钉并写明理由）。③ 合并回执的期望文案第一版是我**编的**（「10 条进」），页面上的真话是「2 份输入共 10 条 / 8 条出 / 删掉 2 条」⇒ 按页面原文重钉。④ 分割失败那一档的夹具文案也换成真判决：`分割比例之和必须为1.0，当前为 0.8999999999999999`——那个浮点尾巴是端点原文，写成 `0.9` 就是把判决改成我的想象。

- **前端面的三件收口判据齐**（A279 从第一批起没换过，本轮第五次命中）：消费守卫的名单与实测集合**相等且变短**、`api.wiring.test.ts` 的 `KNOWN_UNWIRED` **仍为空**、三员各覆盖**成功与失败两路**。三门禁 `npx tsc --noEmit` / `npx eslint . --ext .ts,.tsx` / `npx vitest run`：前两者 rc=0，vitest **14 文件 163 → 176 支全绿**（新增那一支文件 13 支）。

- **门禁读数**：全量 `python -m pytest tests -q -p no:cacheprovider` ⇒ **8475 passed / 11 skipped / 0 failed / 覆盖率 99.81%**（244.78 s；与 L228 的 8420 相减 `+55` = 新守卫 55 支，接线本身零后端用例增量）。这一跑同时是 `MEASURED_BLIND_SEEN = 113` 的**实测点**——那一格依赖 `collected`（下限 3000 条），单跑子集只会抛 `RulerNotProven`，所以它只能在全量里量。六把账本与文档守卫 **149 passed / 8 skipped**（8 支 skip 是 `test_doc_line_refs_l79.py` 要作者工作树那一档，与本轮无关）。**账本批之后复跑同一份全量，读数一字未动**（8475 passed / 11 skipped / 0 failed / 99.81%，235.34 s；两次都是 0 failed，差值只在墙钟上）。本轮**没有**踩到 A184 那把注释引用尺子：账本与新守卫里落的每一处行号引用都逐条验过落在代码行上（`augmentor/converter.py` 的 91 / 56 / 327、`augmentor/dataset_ops.py` 的 27 / 157 / 163 / 169 / 220 / 427、`augmentor/data_splitter.py` 的 63 / 67、`api/routes/dataset_tools.py` 的 270 / 787）。

- **仍在账上的邻居**：**A279**（剩 18 条，其中 `NO_ENTRY` 1 条 = `translate`，归 **A283**）、**A289** / **A290** / **A291**（本轮新立）、**A287** 与 **A288**（L228 立的契约面死面与值域）、**A284**（`unique_instructions` 误名）、**A285**（五处手写字段清单没有权威）、**A286**（比值型计时守卫会假红）、**A281**（3 处响应类型缺契约已有字段）、**A282**（format 基线红且 CI 无门）、**A280**（错误分档一刀切）、**A278**（自由子对象的形状 vs 可证伪白名单）、**A276**（`app` 三键的处置）、**A271** 的 18 只待处置键、**A272**（`multilingual` 要不要可写）、**A273**（`self.expander` / `self.tracker` 的消费者还是删除）。


### L230（2026-10-11）— A279 第六批：`sample` / `aggregate` / `rag` 进数据管理页，并把「后端没有可引常数时怎么钉同源」换成读 AST 分发臂

- **这一轮做两件事**：① `POST /api/dataset/sample`、`POST /api/dataset/aggregate`、`POST /api/dataset/rag` 三条进数据管理页（采样、多源聚合、转成 RAG 检索格式三张新卡 + 三处入口 + 该页用例），**只动 `web/src`**，后端字段名与响应形状一字未改；② 新立 `tests/unit/test_sample_aggregate_rag_wiring_l230.py`（**74 支**）与渲染面 `web/src/pages/DataManagement.sample.test.tsx`（**19 支**），并把顺手量到的六笔契约面问题按「不动，只立账」处置成 **A292**~**A297**。**一个里程碑读数**：这三只也是落盘一族，接完之后全仓六只会写产物的端点（`convert` / `merge` / `split` / `sample` / `aggregate` / `rag`）**在页面上全部有入口**，消费面 57 → **60**（75 条操作的 80%）。

- **为什么这批又要换一档判据**：l226 量「下拉清单 vs Python 封闭常数」、l227 退到「页面发得出的每档都被端点接受」、l228 再退到「候选只能来自回执」、l229 回到「写边/读边各只有一份格式清单」。**这一批连 l229 那条都不成立**：三种 `method` 与四种 `strategy` 在仓内**没有任何可引常数**——它们只以 if/elif 分发臂和散落的字面量存在。照抄一份清单进测试就是自造第二套权威（正是本仓在收的形状），所以判据换成**读 AST 数分发臂**：`dispatch_arms()` 走 `DatasetOperations.sample` 与 `DataAggregator.aggregate` 的函数体，逐条收集 `config.method == 「字面量」` / `strategy == 「字面量」` 的比较臂，页面两张下拉逐项与它相等。**并且一条都数不到就抛 `LookupError`**——分发结构哪天被改成查表，这把尺子会当场响而不是 silently 判绿（数不到 ≠ 没有，这条写在函数自己的错误文案里）。

- **改前实测（这一批的语料，全部现量）**：采样的 `size` 与 `ratio` 是二选一（页面上给一个就清另一个），`size=0` 与 `ratio=0` 都是**合法的「采 0 条」**而不是「未设置」（`augmentor/dataset_ops.py:291` 与 `:296` 那两行的`is not None` 判据正是为此，源码注释里记着 L149/B219 的旧缺陷）；`method` 三档 × `ratio` 三档的九格矩阵逐格发过去，读数是 **A292** 那三条；分层的分组键只取前 10 字（**A296**）；聚合四策略在 20 条语料上各回各的条数，`target_size=1` 只让 weighted 变 1 条（**A294**）；RAG 三档格式（`llamaindex` / `langchain` / 留空）都 200，留空与送空串**等价**（**A295**）。

- **三处被自己的夹具推翻的中间读数（按纪律留着不美化）**：① 初稿断言「SDK 只报 `conflict_count`、线上只报 `conflicts`，两边各有一半」——实测**两套键集各有两格对方没有**：`AggregationResult.to_dict()` 是 `{total_count, source_counts, removed_duplicates, conflict_count, conflicts}`，`AggregateResponse` 是 `{output_file, aggregated_count, source_counts, removed_duplicates, conflicts}`。用例改写成「两套手维护的键集已经漂移」并把**两个差集**都钉住，那条错判据的原文留在用例 docstring 里；② 初稿把 RAG 默认档引成 `RagConfig.model_fields["default_format"]` ⇒ `ImportError: cannot import name 'RagConfig'`，真名是 `RAGConfig` 且是 **dataclass**（`augmentor/config.py:533`）⇒ 改引 `__dataclass_fields__[...].default`，并删掉那条为「万一是 pydantic」写的、永远走不到的双分支；③ 初稿断言「`target_size` 省略会 400」⇒ 实测省略是 **200**，走 `DEFAULT_TARGET_SIZE = 100`（回落点 `augmentor/aggregator.py:159`），而且 `min(DEFAULT_TARGET_SIZE, len(ITEMS))` 才是真条数。**这三条如果没发探针就会变成三支恒真用例**：①③ 断言的是不存在的行为，② 是一支永远走不到的分支。

- **另两次红档**：① 新守卫第一版在**收集阶段**就 SyntaxError——f-string 的**字面文本**里嵌了双引号（PEP 701 只放开表达式位，不放开字面文本），换成 「」 之后才收得进去；② A184 那把注释引用尺子红了**两支**：docstring 里给 `augmentor/aggregator.py` 指的第 20 行落在**注释行**上（这把尺子要求每一处行号引用都落在代码行），改指 `augmentor/aggregator.py:159`（`target_size = DEFAULT_TARGET_SIZE`，正是那一格判据的真落点）之后转绿，阈值一格未动。

- **`docs/API.md` 两句订正（都是同行追加，`git diff --numstat` 复核 2/2 与 1/1 净行数零）**：① aggregate 行原本写着「`target_size` 省略会 400」——假话，实测 200；订正后写明**省略不是 400**、走 `DEFAULT_TARGET_SIZE`、且这一格只咬 weighted（指向 **A294**），并把回执偏离 SDK 键集这件事写进行里（指向 **A293** 与 **A297**）；② rag 行原本没写默认档是哪个，订正后写明出厂默认 **`llamaindex`**（`augmentor/config.py:533`）与端点里那第三处字面量 `langchain`（**A295**），并写明多送的 `chunk_size` / `chunk_overlap` 会被静默忽略。**订正之后新立四支文档用例**把这两行钉住：文档行必须说端点**真的会做的事**，并且与一次真跑的 200 回执交叉核对。**判据咬到自己那一处**：新用例的第一版断言「aggregate 行不得含『`target_size` 省略会 400』」，而我自己刚写进去的订正说明里**引用了这句被推翻的原文**，于是当场红 ⇒ 改成断言整句原文的形状（订正说明可以引，独立声称不行）。

- **六笔新账（全部只立不做，逐条都有实测读数）**：**A292**（`ratio` 无判决，三种方法三种结局：`ratio=2` 在 random 是 400 且 detail 是 CPython 的英文原文、在 systematic/stratified 是 200 且回整份；`ratio=-0.5` 在 systematic 是 200 且回 10 条）、**A293**（端点构造无参 `DataAggregator()`，`key_fields` / `value_fields` / `random_state` 三档不可达 ⇒ 去重恒按 `instruction`、一致性恒按 `output`、weighted 不可复现；外加两套回执键集漂移与 `conflicts` 整份上线）、**A294**（`target_size` 只咬 weighted，0 合法、-1 是 400、省略走 100）、**A295**（默认格式的第三处字面量写的是 `langchain`；空串与省略等价；分块两格静默忽略）、**A296**（分层分组键只取前 10 字，前缀相同的不同类静默折叠，且折叠在回执上无痕迹）、**A297**（`consistent` 把同源内部分歧算成冲突，那些条目永不落地 ⇒ 一份内部有重复提问的数据集会让整组产物为空）。**六条一条都没动产品代码**：改判决、改回执键集、改分组键都是对外契约变化，按纪律待拍。

- **八臂「页面初值 == 契约默认」逐个对账**：`method="random"`、`size` / `ratio` / `seed` / `stratify_key` 四格从空开始（留空即不发那一格）、`strategy="union"`、`target_size` 留空、`format` 留空（＝「服务端默认」那一档）。**`DEFAULT_TARGET_SIZE = 100` 全仓只有 SDK 一处**，页面**不抄那个数当默认**——由一支用例同时钉住「常数确实是 100」与「页面 state 初值是 `null`」。服务层三只 fetchers 全文**没有第二套默认**（既不写 `= '…'` 也不写 `??`），唯一一处默认表达式是转换卡那处 `format: opts.format || undefined` 形状卫生哨兵；这一批实测送空串与省略等价，所以那一处在**这一只端点上不是为了躲判决**（与 A291 那处不同），动机差别写进了用例 docstring。

- **census 两把按现量重钉（不许照抄上一轮）**：l97 CALLS 411 → **419**、READABLE 293 → **301**、盲面 **118 一格未动**——本轮八处新增 `@pytest.mark.parametrize` 的值位**全是内联字面量清单**，所以八格全落可读面，两边独立闭合（+8 / +8 / +0）。l98 档位和 **118** 与盲面用例名册 **113** 未动，豁免用数 18 → **23**：五支新用例的值位来自 AST 分发臂与请求模型字段集（不是手写清单），逐支带理由入册。**豁免名单变长不是把尺子放松**——l98 判的仍是**逐行**的值位形状，整支豁免只适用于「值位由机器实推」那一种理由。**一处读数口径的坑**：l98 的动态面依赖 `collected`（下限 3000 条），单跑子集会抛 `RulerNotProven` 报成 **5 errors**，所以那两格只能在全量里量；第一次全量读到的是 `assert 23 == 18` **1 failed**，按实测把五支新豁免点数清楚之后重钉成 23，不是先改数再看。

- **注入九臂，逐臂红格按实测记**：Python 面六臂（① 摘掉 `dispatch_arms()` 的 `LookupError` 那一档 ⇒ 响 **2** 支；② 页面 `method` 下拉多编一档 ⇒ 响 **5** 支；③ 把「两套键集漂移」的两个差集断言削弱成一个 ⇒ 响 **3** 支；④ 服务层给 `target_size` 抄一份默认 100 ⇒ 响 **1** 支；⑤ 删掉卡片上「只咬 weighted」那句 ⇒ 响 **1** 支；⑥ 把文档用例的交叉核对摘掉 ⇒ 响 **1** 支），基线 **70 passed**；vitest 面三臂（W1 采样卡的目标清单多一档 ⇒ **2** 支、W2 把 `seed` 初值替契约编一个默认 ⇒ **1** 支、W3 删掉「只按前 10 个字符分组」那句 ⇒ **1** 支）。九臂 rc 全 1 且输出含 `failed`，被改文件逐臂字节复原、收尾重新验盘并断言哈希与进入时一致（sha256 `8b17a662` 起），复原后基线复跑 **2 文件 / 32 支**全绿。**一臂被拦在写盘之前**：W3 的锚点 `完全空转` 在文件里出现**两次**（模块注释一处、卡片文案一处），注入器的唯一性断言当场响 ⇒ **磁盘未被触碰**，换成更长的锚点（`配其余三支**完全空转**`）并按 argv 过滤到单臂之后才注入成功。这正是「锚点必须唯一」这条该有的反应：少写一步就会改错地方并把红归因到错的臂上。

- **前端面的三件收口判据齐**（A279 从第一批起没换过，本轮第六次命中）：消费守卫的名单与实测集合**相等且变短**、`api.wiring.test.ts` 的 `KNOWN_UNWIRED` **仍为空**、三员各覆盖**成功与失败两路**。三门禁 `npx tsc --noEmit` / `npx eslint . --ext .ts,.tsx` / `npx vitest run`：前两者 rc=0，vitest **15 文件 176 → 195 支全绿**（新增那一支文件 19 支）。**渲染面用例第一版撞的三处地雷，全部按实测改判据而不是改页面**：① antd 的 loading 图标会并进按钮的可达名 ⇒ 断言改成子串正则；② 按钮文案在成功/失败两路上不同 ⇒ 用 role + 名字前缀定位而不是整串相等；③ `InputNumber` 的 spinbuttons 读出来带 step 补零，且那一页的 spinbutton 数量比 L229 多 ⇒ 按 `slice(4)` 取本卡那几格并写明为什么是这个偏移。

- **门禁读数**：全量 `python -m pytest tests -q -p no:cacheprovider` ⇒ **8549 passed / 11 skipped / 0 failed**，覆盖率 **lines 14186/14196 = 99.93%、branches 3912/3936 = 99.39%**（179.78 s；与 L229 的 8475 相减 `+74` = 新守卫 74 支，接线本身零后端用例增量）。这一跑同时是 l98 动态面的实测点。**本轮全量跑了三次，中间两次读数留着**：第一次 **8544 passed / 1 failed**（169.66 s，红的是 l98 豁免用数 `assert 23 == 18`）；第二次是修完 A184 那两支之后的 **8545 passed / 0 failed**（112.82 s，带 `--no-cov`，只为确认收集面干净，不作为定标读数）；第三次才是上面那份定标读数。六把账本与文档守卫 **149 passed / 8 skipped**（8 支 skip 是 `test_doc_line_refs_l79.py` 要作者工作树那一档，与本轮无关）。**账本批之后复跑同一份全量，读数一字未动**（8549 passed / 11 skipped / 0 failed，163.18 s，覆盖率仍是 lines 14186/14196 = 99.93%、branches 3912/3936 = 99.39%；两次都是 0 failed，差值只在墙钟上：179.78 s → 163.18 s）。

- **仍在账上的邻居**：**A279**（剩 15 条，其中 `NO_ENTRY` 1 条 = `translate`，归 **A283**）、**A292**~**A297**（本轮新立）、**A289** / **A290** / **A291**（L229 立的三条同族契约面缺口，其中 A290 与本轮 A293 是同一形状：回执手抄而不是引用 SDK）、**A287** 与 **A288**（契约面死面与值域）、**A284**（`unique_instructions` 误名）、**A285**（五处手写字段清单没有权威——本轮这两个后端权威同样没有可引常数，只是这两个能用 AST 数出来，A285 那五处不能）、**A286**（比值型计时守卫会假红）、**A281**（3 处响应类型缺契约已有字段）、**A282**（format 基线红且 CI 无门）、**A280**（错误分档一刀切）、**A278**（自由子对象的形状 vs 可证伪白名单）、**A276**（`app` 三键的处置）、**A271** 的 18 只待处置键、**A272**（`multilingual` 要不要可写）、**A273**（`self.expander` / `self.tracker` 的消费者还是删除）。

### L231（2026-10-11）— A279 第七批：备份那一族四条进系统状态页，并补掉 `name` 越界写盘那一处产品缺陷

- **这一轮做三件事**：① `GET` / `POST /api/system/backups`、`POST …/{backup_id}/restore`、`DELETE …/{backup_id}` 四条进系统状态页（一张「数据备份」卡：列表 + 创建 + 恢复 + 删除，源文件候选取自 `/api/data/list` 也允许手填，删除走 Popconfirm 二次确认）；② 新立 `tests/unit/test_backup_wiring_l231.py`（**33 支**）与渲染面 `web/src/pages/System.backup.test.tsx`（**11 支**）；③ **改一处产品面**——`DatasetBackup.backup()` 的 `name` 越界写盘。前六批接线都只动 `web/src`，这一批动不了：接线之前先量后端，量出来一处真缺陷。

- **那处产品缺陷的取证与修法**：`backup_path = self._backup_dir / f"{backup_id}.json"`，而 `backup_id = name or 时间戳`，`name` 直接来自请求体 ⇒ 仓外探针实测 `name="../../escaped"` 打 `POST /api/system/backups` 是 **200**，且临时目录**顶层**真的多出 `escaped.json`（白名单只管到 `backup_dir` 那一格，管不到 `name` 这一段）。修法是四行：算出 `backup_path` 之后断言 `self._backup_dir.resolve()` 在它的 `parents` 里，不在就抛 `DataValidationError`（经 `to_http_error` 落 **400**）。改后同一条请求 **400**「备份名称不能指向备份目录之外: ../../escaped」、白名单外零文件；`name` 带 `/` 那一档**照旧放行**（实测 `sub/deep` ⇒ 200，在备份目录下建 `sub/` 再落 `deep.json`）——它没有逃出备份目录，拦它就是砍能力。**这四处 +4 行把 `augmentor/backup.py:188` 平移到 `:192`**，两处历史行号引用（本账 A277 行、L223 日志块）同轮重钉，另加 `tests/unit/test_api_doc_field_hints_l223.py` 的 docstring 一处；`git diff --numstat` 复核三处净行数 **0**（`2/2`、`1/1`）。**插行安全不可外推**这条纪律本轮又验了一次：先 grep 过全仓，指向 `System.tsx` 的历史引用只有 `System.tsx:3` 一格，所以页面新增的 import 一律放在第 4 行及以后，第 3 行仍是原来那句服务层 import。

- **消费面按实测回填四格**：调用形状与已消费 **60 → 64**（75 条的 **85%**）、缺口名单 **15 → 11**、`CLI_ENTRY` **14 条 / 9 名 → 10 条 / 8 名**、`NO_ENTRY` 仍 **1**（`translate`，归 A283）。**这一批的名字侧动的是「一个子命令整名出名单」**：`backup` 此前吃掉四条操作（`GET` / `POST` / `DELETE` / `restore`），本轮四条一起接走 ⇒ 条目减 4、名字减 1。**我第一版把这件事写成「第一次出现一个子命令吃掉多条操作」，那是假话**——`dependency` 早就映射 3 条，只是没被接走；按实测改成「一次减 4 条是现量最大的一批」。判据 4 那一维同时涨：带 query 的调用 **5 → 9** 条、去重后键名 **6 → 7** 个（多的一个是 `backup_dir`）、`params: {…}` 归属 **4 → 8** 条、最大间隔 **14 → 41** 字符（`PARAMS_MAX_GAP = 60` 一格未动）。

- **那把窗口尺子本轮真的咬人了**：`createBackup` 的第一版把请求体写在 `api.post(…)` 的实参位上，`params` 块离所属调用超过 60 字符 ⇒ `backup_dir` **绑不上**，query 那一维对这一条失明（实测：8 条里只有它没绑上）。修法是**改产品侧而不是放宽尺子**——把请求体提成一行 `const body = {…}`，间隔回到 14 字符，`backup_dir` 当场绑上。这一格的教训与 L224 立尺子时那句「上限守的是尚未出现的形状」正好接上：形状真的出现了，而它出现在**我新写的代码**里。

- **新守卫首跑 3 failed / 30 passed，三格全是尺子自己的 bug**：① 路径字面量对账没剥 `/api` 前缀（契约面是 `/api/system/backups`，`api.ts` 里写的是 `/system/backups`，`baseURL` 补前缀）⇒ 补 `.removeprefix("/api")`；② 「服务层不写第二套默认」那一支被**页面上的老实文案**判红（`.backups` 出现在 JSX 正文里，那句话正是在向用户解释三份默认的关系）⇒ 收窄成「只扫 `api.ts` + 页面里不许出现 `backup_dir` / `backupDir` 的字面量赋值」；③ `verify_api_key` 门禁普查拿 `app.routes` 当语料得到**空名单**——这版 Starlette 不展平 included router（l224 里为路由顺序那一格已经量过一次同一个坑）⇒ 换成 `api.routes.system_ops.router.routes` + `route.dependant.dependencies`，并补一格 `seen` 断言证明四条备份路由确实都在语料里（否则「零条未挂闸」与「零条被读到」长得一样）。

- **复跑又 2 failed / 31 passed，还是尺子的范围写错**：④ A300 那一支我写成「全契约里不许出现 `verify` 或 `clean` 字样」，被 `/api/quality/clean` 判红——那是一条与备份毫不相干的端点 ⇒ 收窄到备份面这一族的 (方法, 路径) 集合**精确相等**（四条，多一条少一条都红）；⑤ A302 那一支写成「`web/src` 全文零处 `X-API-Key`」，被 `System.tsx` 里那句「前端目前没有送 X-API-Key 的通道（A302）」判红 ⇒ 只认**带引号的字面量**（真有通道时那个头名必然写在 `headers: { 'X-API-Key': … }` 的字符串位上），同时把「提到这个头名的文件恰好只有 `System.tsx` 一处」钉成现量。**两次都是同一个错误形状**：把「这一族里没有」写成「全仓没有」，语料一宽就撞上无关住户；五格全在尺子这一侧，产品面一处未改。

- **渲染面第一版撞的两处 antd 形状，都按实测改判据而不是改页面**：① `AutoComplete` 把 `aria-label` 同时挂在**外层 div 与内层 input** 上 ⇒ `getByLabelText('备份源文件')` 命中 2 个直接抛，加一个只取 `tagName === 'INPUT'` 的 helper 才定位到；② antd 报 `[antd: Table] index parameter of rowKey function is deprecated` ⇒ 行键从 `backup_id#index` 改成 `backup_id#timestamp`（这也更老实：索引里两条同 id 的记录本来就是靠时间戳区分的，见 **A298**），并因此要求夹具里那两条同 id 记录带上**实测的两个不同时间戳**（`.991804` / `.997820`）——否则 React 键会撞，夹具就不老实了。

- **注入六臂，逐臂红格按实测记**：① 摘掉 `backup.py` 那道越界门 ⇒ **3** 支红（HTTP 面 400 那一支、异常类型那一支、SDK 直构面那一支）；② TS 侧把恢复回执的落点键改名 `output_path` → `output_file` ⇒ **1** 支红（两侧键集对账）；③ 服务层把恢复路径拼错一个字母 `restore` → `recover` ⇒ **6** 支红（l231 的 fetcher 路径那一支 + l224 的幽灵调用、query 声明、params 归属、名单相等、规模档五支）；④ 页面摘掉删除前的 Popconfirm ⇒ vitest **1** 支红；⑤ 文档把 convert 那行的输出字段写回 `output_path` ⇒ **2** 支红（收窄后的禁令 + 无据 token 那一支）；⑥ 往 l224 的 `UNCONSUMED` 里塞回一条已接好的操作 ⇒ **3** 支红（名单相等、规模档、两分档）。六臂 rc 全 1，被改的六个文件（`augmentor/backup.py`、`web/src/types/api.ts`、`web/src/services/api.ts`、`web/src/pages/System.tsx`、`docs/API.md`、`tests/unit/test_api_endpoint_consumption_l224.py`）逐臂字节复原、收尾重新验盘并断言 sha256 与进入时一致，复原后基线复跑 **82 支**（33 + 26 + 23）全绿 + vitest rc=0。**一臂被拦在写盘之前**：⑥ 的第一版锚点是「把名单里那条 `DELETE /api/system/backups/{backup_id}` 删掉」，而那条本轮已经不在名单里了 ⇒ 注入器的唯一性断言命中 0 次当场抛，**磁盘未被触碰**；换成「塞回一条已接好的 `GET /api/system/backups`」才注入成功。

- **A277 的全文禁令与本轮的真话冲突，处置是收窄而不是摘掉**：L223 立的判据是「整篇 `docs/API.md` 不许出现 `output_path`」，起因是那个名字当时只是路由里的局部变量名、文档在教调用方发一个会被 pydantic 拒掉的键。本轮量出它在**另一族**里是真的：`POST /api/system/backups/{backup_id}/restore` 的响应落点键就叫 `output_path`（同族其余五条写盘端点叫 `output_file`，`split` 叫 `output_dir`）。照实写进文档就撞上禁令 ⇒ 判据改成「dataset 七条写盘族一行不许出现 + 全文只许 restore 那一行出现，多一行即红」，并把那七条路径提成 `WRITE_SIDE_PATHS` 供两支判据共用（原缺陷那一侧**没有变松**：注入臂 ⑤ 实证把 convert 那行写回幽灵名仍 2 支红）。

- **census 两把棘轮本轮一格未动，且这不是「没量」**：两支新文件里 `parametrize` 出现 **0** 次（`grep -c` 实测），l97 的 `MEASURED_CALLS = 419` / `READABLE = 301` / 盲面 `118` 与 l98 的四档分布、豁免 `23` 全部原地绿；l98 的动态面只在整跑里可量（子集跑会以 `RulerNotProven` 报 5 个 error，那是判据在自证而不是坏了），本轮全量跑过 ⇒ 那一面也是实测绿。**这一族里还有一格假读数，是我自己写出来又划掉的**：清单行第一版写「哈希槽 prog 侧 15 → 16、唯一哈希 69 → 70」，理由是「本轮往清单里加了一行带哈希的条目」——六把守卫跑完全绿才发现那把尺子的语料是 `LEDGER = OPTIMIZATION_LOOP.md`（**第一本账**，`tests/unit/test_ledger_hash_slots_l98.py:53`），而 L231 写在第二本 ⇒ 六档现量（15/24/15/69/占位 0/R6 白名单四格）一格未动，守卫 **149 passed / 8 skipped** 原地绿。已按实测改回。教训与 L224 那句「尺子的语料面要先读判据再报读数」是同一件事：**加一行清单条目并不必然动哈希槽，动的是它读的那一本账**。

- **门禁读数**：全量 `python -m pytest tests -q -p no:cacheprovider` ⇒ **8582 passed / 11 skipped / 0 failed**（180.57 s，覆盖率 **lines 99.93% / branches 99.39%**，TOTAL 行 99%）；与 L230 的 8549 相减 `+33` 恰等于新守卫支数（接线本身零后端用例增量，产品面那四行的行为由新守卫的 3 支覆盖）。前端三门禁：`npx tsc --noEmit` rc=0、`npx eslint . --ext .ts,.tsx` 零输出、`npx vitest run` **16 文件 / 206 支全绿**（基线 15 文件 / 195 支，`+11` = 新那支文件的 11 支）。六把账本与文档守卫在账本批改完后复跑 ⇒ **149 passed / 8 skipped**（与 L230 同一读数；8 个 skip 全是 l79 那支「需要作者工作树」的既有豁免）。

- **下一批的口径**：**A279**（剩 11 员：`CLI_ENTRY` 10 员 + `NO_ENTRY` 1 员 = `translate`；`GET /api/system/dependencies` 仍按 L226 的纪律拒接）、**A298~A304**（本轮新立的七条备份面缺口，其中 A299 与 A300 是同一件事的两半）、**A283**（`translate` 的契约缺口）、**A280**（错误分档一刀切）、**A278**（自由子对象的形状 vs 可证伪白名单）、**A276**（`app` 三键的处置）、**A271** 的 18 只待处置键、**A272**（`multilingual` 要不要可写）、**A273**（`self.expander` / `self.tracker` 的消费者还是删除）。

### L232（2026-10-11）— A279 第八批：数据集血缘那一族三条进系统状态页，并收掉 `description`/`tags` 回显即丢那一处产品缺陷

- **这一轮做三件事**：① `GET` / `POST /api/system/dependency/datasets` 与 `GET /api/system/dependency/graph` 三条进系统状态页（第四张卡「数据集血缘」：登记表单 + 已登记列表 + 图的三格计数与 `issues` 逐条列出）；② 把接线时量出来的一处产品缺陷（**A305**：`description` / `tags` 回显即丢）当场修掉；③ 新立 `tests/unit/test_lineage_wiring_l232.py`（39 支）与渲染面 `web/src/pages/System.lineage.test.tsx`（8 支），五臂注入取证。

- **卡片为什么叫「数据集血缘」而不是「依赖」**：页面上已经有一张「依赖明细」卡，那是 `GET /api/system/dependencies`（**可选依赖包诊断**：已装/缺失两栏 + 降级功能清单），与这一族（数据集之间的血缘/派生关系）**同前缀不同族**。两个标题同时在场由判据钉住（`test_the_card_is_named_after_lineage_not_after_packages`），模块 docstring 里也写清了区别。**这个「同前缀不同族」本轮咬了我一口**，见下面第 6 条。

- **A305 的取证与修法**：请求体带上 `description: "描述B"` / `tags: ["t1","t2"]` ⇒ **200**，回执九键里两格都在；紧接着 `GET …/datasets` 里同一条是 `""` 与 `[]`，磁盘 `datasets.json` 同样为空。根因在路由里读得出来：`register_dataset()` 内部就 `atomic_write_json` 落了盘，端点却在这之后才 `info.description = …` / `info.tags = …` —— 改的是内存里那个返回对象。修法是随登记一起传下去（SDK 形参本来就有这两格），删掉事后赋值那四行，`api/routes/system_ops.py` 净 **+1 行**（6/5）。**这一条最值得留在账上的是「哪把尺子在改前也是绿的」**：只看响应形状的判据（回执确实回显了两格）在改前照样绿 ⇒ 新守卫那一条判据**只读磁盘与随后的 GET、不读响应**；注入臂1（把修法改回改前形状）实证它点名喊红。

- **插行有没有平移历史行号**：`system_ops.py` 那处改动**之上**的历史引用现量在 150/295/397/398/400/415，净 +1 行落在它们全部之下 ⇒ 无一平移；`docs/API.md` 与 `README.md` 都是**同行追加、净 0 行**（4/4 与 1/1，`git diff --numstat` 复核），所以本账第 734 行那条 `docs/API.md:952` 引用仍指得准——L231 那次是 +4 行真的平移了两处历史引用、当轮重钉，这一次是**先躲开**。

- **census 两把棘轮动了三格，读数是量出来的不是推的**：新守卫两支 `parametrize`（`input_file` 四档坏值 / `registry_path` 两档坏值）。仓外探针 `l232_census.py` 直接 `import` l97 那把尺子、对 `tests/` 现量并逐处打印桶归属 ⇒ 两处都落 `plain`（可读档），`MEASURED_CALLS` **419 → 421**、`MEASURED_READABLE` **301 → 303**、盲面 `no-literal-list` **118 一格未动**；l98 从 l97 `import` 同两个常量故一并生效，其六档分布与豁免 `23` 原地绿。**没有按「值位是字面量 ⇒ 可读」想当然写**——L204 那次正是这么推的，实测当场红（那两支的值位是模块级名字引用）。l98 的动态面只在整跑里可量（子集跑以 `RulerNotProven` 报 5 个 error，那是判据在自证而不是坏了；本轮隔离跑实测 54 passed / 5 errors，全量跑里 5 支都绿）。

- **三把尺子首跑红，逐格按各自 docstring 的处方重钉，没有一把被改松**：① `test_api_doc_field_hints_l223.py` 红 1 格 —— 我在 `docs/API.md` 那格里用反引号引了 `remove_dataset` / `unregister_dataset` 两个**根本不存在**的方法名（「它们不存在」正是 A309 的内容），而那把尺子要求提示里每个名字都绑得到证据（契约面 / 字面量 / 符号 / 配置键 / CLI / JSON）。处方是改自己的散文：去掉反引号与那两个名字，写成「SDK 也没有注销数据集的方法」；**尺子没动**，而且页面上那句原文（`System.tsx` 里带 `remove_dataset`）不在它的语料面内、照旧留着。② `l224` 红 5 格，全是「按实测回填」类（形状 67、query 12 条 / 8 键、`params` 归属 11、缺口名单 8、名册规模），逐格用 `l232_measure.py` 的读数重钉。③ **本轮新守卫自己首跑红 3 格**：1 格是诚实口径那句短语我写成了服务端判决原文（卡上其实写的是「白名单外是 403」），另 **2 格是我自己把族过滤写宽了** —— `"dependenc" in p` 同时命中另一族的 `GET /api/system/dependencies`，于是「血缘一族只有三条」那两格判据多出一员。收窄成前缀 `/api/system/dependency/` 并把理由写进常量注释；**这与 L231 那次「把『不在这一族』写成『不在仓里』」是同一个缺陷形状**：判据的作用域写宽，就会把邻族当成本族。顺手把同一支里另一把**恒真的尺子**换成会响的：原本断言 `add_dependency` 等三个方法名不出现在 openapi 的**路径串**里（路径从来不长这样，永远绿），改成查 `operationId`（FastAPI 的默认 operationId 同时含函数名与路径，新增一条以这三个方法命名的路由会红）。另外我自己写出过 5 处「永远为真」的断言（`… or True`、`… if … else True`、`assert not x.is_absolute() is False` 一类），全部删掉或换成真的 `in` / `==`。

- **注入五臂（逐臂点名喊红、逐臂字节复原）**：臂1 把 A305 的修法改回改前形状 ⇒ 1 支红（磁盘与 GET 那一条）；臂2 把 POST 的 query 键 `registry_path` 改名成 `registryPath` ⇒ 4 支红（新守卫 1 支 + `l224` 的 3 支 query 判据）；臂3 把卡上「不查环」那句改成「不做这一维」⇒ 1 支红；臂4 把卡片标题改成「血缘」⇒ 1 支红；臂5 把族过滤放宽回 `"dependenc" in p` ⇒ 1 支红（证明这次收窄是吃重的、不是装饰）。四个被改文件进场都是**纯 LF**（逐文件量过 CRLF=0），锚点各恰好命中 1 次，每臂写完读回比 sha256 与「变异已生效」两件事，`finally` 复原并断言与进场值相等；收尾对五个文件做一次总账比对（全 OK），基线复跑 **65 passed**。

- **门禁读数（同一条命令，改前改后各跑一次）**：改前 `8618 passed / 3 failed / 11 skipped`（三格全是上面那三把重钉类），重钉后 **8621 passed / 11 skipped / 0 failed**，覆盖率 **99.81%**，186.34 s。两边总数一致（8621）⇒ 三格是「读数错了」而不是「用例数变了」。前端三门禁：`tsc --noEmit` rc=0、`eslint . --max-warnings 0` rc=0、`vitest run` **17 文件 / 214 支全绿**（改前 16 文件 / 206 支）。**前端首跑也不是一遍过**：typecheck 红 8 格 —— 两份既有替身文件（`System.test.tsx` / `System.backup.test.tsx`）的 `vi.hoisted` 声明块里没有新键，而我在 `vi.mock` 工厂里已经用了它们；lint 红 5 格 —— 新用例里 5 处 `any`（`Record<string, any>` 与四处 `({}) as any`）。两处都是**改自己的测试代码**收口（补声明块的键；`Record<string, ReturnType<typeof vi.fn>>` 与 `as unknown as ReturnType<typeof message.warning>`），没有一处放宽 `tsconfig` 或 eslint 规则。

- **这一批第一次让 `params` 那一维涨到 11 条 / 去重后 8 个键名**：三条调用都带 `registry_path`，而 `POST` 那一条**先把请求体提到调用之前**（间隔 14 字符，落在 `PARAMS_MAX_GAP = 60` 的窗口内）——L231 的 `createBackup` 就是因为请求体写在调用里、`params` 块离得太远而**绑不上**（四条里唯一漏的一员），当时的修法是改源码而不是放宽窗口。这一批照那个处方**先躲开**：全批 `params` 归属间隔实测 4~41，最大仍是 `restoreBackup` 的 41，一格未涨。

- **待拍口径**：**A279**（剩 8 员：`CLI_ENTRY` 7 员 + `NO_ENTRY` 1 员 = `translate`；`GET /api/system/dependencies` 仍按 L226 的纪律拒接）、**A306~A312**（本轮新立的七条血缘面缺口，其中 **A308 与 A301 同型同根、A312 与 A302 是同一个缺口的第二例、A311 与 A280 是同一件事的两面**，三对各自合并拍更省一次破坏性变更评估）、**A298~A304**（L231 那七条备份面缺口）、**A283**（`translate` 的契约缺口）、**A280**（错误分档一刀切）、**A278**（自由子对象的形状 vs 可证伪白名单）、**A276**（`app` 三键的处置）、**A271** 的 18 只待处置键、**A272**（`multilingual` 要不要可写）、**A273**（`self.expander` / `self.tracker` 的消费者还是删除）。

### L233（2026-10-11）— 前端补上 `X-API-Key` 通道：24 条挂门禁里 21 条是前端已接线的形状，收 A302 / A312

- **这一轮做四件事（一件都不是接新端点）**：① `web/src/services/api.ts` 补一条 key 通道（常量 + 存取 + 请求拦截器），并在 `apiErrorDetail` 这个唯一漏斗里给 401 加一句指路；② `Settings.tsx` 新增「写操作鉴权」卡（存 / 清除 / 已存标记）；③ 新立 `tests/unit/test_api_key_channel_l233.py`（18 支）+ 前端两侧行为面（`api.test.ts` 61 → 68、`Settings.test.tsx` 8 → 14），注入 14 臂取证；④ 把 L233 之后成了假话的口径全部订正（l231 那一维、`System.tsx` 两张卡、README / API.md / ARCHITECTURE.md / DEPLOYMENT.md）。**消费面一格未动，仍是 67/75**——这一轮修的是「设了 key 之后页面还能不能用」，不是「页面上有多少端点」。

- **规模是被探针更正的，不是推的**：仓外探针 `l233_probe.py` 逐条读 `route.dependant.dependencies`，量到契约面 75 条操作里 **24** 条挂 `verify_api_key`；与 l224 那份「前端已消费」名单求交得 **21** 条（逐条冻结进守卫的 `CONSUMED_AND_GATED`），差集 **3** 条是挂门禁但还没接线的尾巴（`POST /api/dataset/translate`、`POST /api/system/migrate`、`POST /api/system/stream`）。**L231 立 A302 时只数了备份那一族的 3 只，规模低估了一个量级**：开鉴权之后页面上点不动的不是 3 个动作而是 **21** 个。门禁里**没有一条 GET**（守卫单独钉这一格），所以开了鉴权页面仍然读得到东西，只是动不了。

- **探针首跑读到的是「1 条操作 / 0 条门禁」——走法是瞎的**：这版 FastAPI 的 `include_router` 不在 `app.routes` 上摊平，而是留一个 `_IncludedRouter` 占位，真路由挂在它的 `original_router.routes` 上。先试 `route.router` 仍然读不到，`dir()` 查出真名之后改成递归展平才读到 75 / 24。**要是照首跑那个读数写账，「21 个动作会 401」会被写成「0 个」，而所有交集判据都空转成全绿**（L231 也踩过这一格，那边改用 `app.openapi()`，但 openapi 里没有依赖信息，量不到门禁）。守卫里因此留了一支**反向元断言**：展平后的 (方法, 路径) 集合必须**等于** l224 的 `operations()`，走法一漂这里先红，而不是让后面每条判据静默变绿。

- **真链路五档逐个量（`TestClient`，白名单根指向临时目录）**：无头 ⇒ **401**，`detail` 逐字「缺少或无效的 X-API-Key」；错值 ⇒ 同样 **401**（小写头名 `x-api-key` 与全大写 `X-API-KEY` 同判）；对值 ⇒ 放行，且**头名大小写无关**（`X-API-Key` / `X-Api-Key` / `x-api-key` 三种写法都放行——HTTP 头名不区分大小写，代码里仍与后端 `Header(alias=...)` 逐字一致，免得读代码时以为是两个不同的头）；`AUGMENTOR_API_KEY` 没设 ⇒ 那道门禁**整个不存在**（`verify_api_key` 直接 return，这是「零配置可用」的既有行为，本轮不动）；门禁开着时 `GET /api/system/backups` 仍 **200**。**验放行那一支换过打法**：初稿用 `POST /api/system/backups`，那会在 `data/.backups` 里真落一份产物（探针不许在仓里留东西），改成 `DELETE` 一个幽灵 id（`20990101_000000_probe`）——放行之后它只会 404「备份不存在」，零写盘。

- **通道形状的三个决定**：① **401 的指路落在 `apiErrorDetail` 一处**，不在 21 个页面各自的 `catch` 里——页面全部走这一个漏斗，一处改动 21 个动作同时被指路；② **服务层不引入 antd**（`services/` 无 UI 依赖是本仓既有惯例），所以不写响应拦截器弹窗，而是让错误文案自己指路；③ **留空即不送头**（`setApiKey('')` 走 `removeItem` 而不是存空串），服务端没设 key 时零配置照旧可用，多送一个空头没有意义。指路文案把两种成因**分开说**：存着却 401 是「浏览器里存着的那一份与服务端不一致」，没存是「这台浏览器还没有存 key」——让用户去对一台自己没填过的输入框是错的方向。

- **卡上刻意不报「校验通过」**：本层没有能力证明那份 key 是对的（服务端只在写操作请求上校验，而存 key / 清 key 这两个动作**一个请求都不发**），所以守卫把 `"校验通过" not in code` 钉成一格，卡上写的是「已存进这台浏览器（localStorage 明文），后续每个请求都会带上它」；同一张卡上写明这是**明文存储**、**不是登录**、**也不替代 HTTPS**。

- **与 A162 那条架构决定不冲突，且已写进 ARCHITECTURE.md**：账上有一条「取文案选 helper，不选 axios 拦截器」（A162）。本轮挂的是**请求**拦截器，只管把头挂上去；**文案仍然全部住在 `apiErrorDetail` 这个纯函数里**，那一拍没被推翻。这层关系同行写进 `docs/ARCHITECTURE.md` 密钥那一条（净零行），免得后来读的人以为两处互相打脸。

- **两处首跑假红 + 一次前端真红，逐个归因**：① 新守卫首跑 **2 failed / 49 passed**。第一格是我自己把尺子写宽了——`localStorage` 那一维原本数「文件里提到这个词」，于是 `pages/Settings.tsx` 被判红，而那里出现 `localStorage` 是 JSX 里的**诚实正文**（「存哪儿：这台浏览器的 localStorage，明文」）。**尺子把回声当成了信号**，与 L231 那条「带引号字面量才算通道」是同一个教训 ⇒ 收窄成只数**存取动作** `localStorage.(get|set|remove)Item`，并补一臂注入（在 Settings.tsx 里塞一句真的 `setItem`）证明收窄之后尺子仍然吃重。② 第二格是**真红**：`README.md:65` 还写着「设了 `AUGMENTOR_API_KEY` 时前端没有送头的通道」⇒ 同行改写，并在 `/settings` 那一行同行补上这张新卡。③ 前端全量首跑 **1 failed / 226 passed**：`System.lineage.test.tsx` 那支「六条实测脾气都留在卡上」里有两条断言钉的正是我改掉的旧口径 ⇒ 按新世界重钉为三条（**不是删掉**）。重钉之后又红一次，这次是 `getByText` 命中**两处**（备份卡与血缘卡各指一次那张卡）⇒ 改成 `getAllByText(...).toHaveLength(2)`，把「各指一次」这个事实本身钉住。

- **两个前端实测坑（都写进注释了）**：① **vitest 的 mock 调用历史在每个测试开跑前被清**——`mocks.instance.interceptors.request.use.mock.calls.length` 在测试体内读是 0，而模块作用域捕获的那一份照常对得上；所以「拦截器注册过几次」这类事实只能在模块作用域取（沿用既有 `createArgs` 那个模式）。为此写过一支 `dbg.test.ts` 复现、定位之后删掉，仓里零残留。② **antd 的 `Button` 会给恰好两个汉字的文案自动插空格**，于是「清除」的 accessible name 是「清 除」，`getByRole('button', { name: /清除/ })` 找不到 ⇒ 改 `/清\s*除/`。

- **注入 14 臂逐臂点名喊红，但第一版 harness 真的在仓里留了残留（本轮最该记档的一条）**：臂序覆盖四面——后端（`raise HTTPException` → `return`、摘掉 `POST /api/config` 的 `Depends`）、走法（`inner = None` 让展平失效）、前端源码（拦截器不读 key、空 key 走不到 `removeItem`、`removeItem` 整行删掉、存取多第二处、头名字面量多第二处、页面自称「校验通过」、401 不再单独指路、前端多接一条挂门禁的写操作）、口径（页面把老假话说回来、两张卡只剩一张指路、README 把老假话说回来）。**第一版跑到「401 不再单独指路」那一臂时，`finally` 里的 `write_bytes(original)` 抛 `OSError [Errno 22] EINVAL`，脚本当场死掉，`api.ts` 里留下一处注入残留**（少一行 `if (response?.status !== 401) {`，文件因此语法不通）。发现它靠的是两支独立的红：守卫那 1 支「401 不再单独指路」，以及 vitest 的 transform 失败（`2 failed / no tests`）。按原文修回并复验：`git diff --numstat` 65/10（与进场一致）、`tsc` rc=0、四份相关测试 101 支全绿、守卫 18 passed。**第二版 harness 加三件事**：跑前把 7 份被注入文件原样备份进 `Temp/l233_inject_backup`（复原失败也能修回）、写后读回比 sha256 且 EINVAL 重试三次（0 / 0.3 / 1.0 s）、逐臂即时打印（脚本死掉也不丢已得读数）。重跑 **14 臂全 OK**、收尾逐份 sha256 与进场值一致、基线复跑 18 passed。**这与 L218 那次「注入器被 OSError 打断、在仓里留两处残留」是同一缺陷类第二次发生** ⇒ 记档：注入器「跑完重新验磁盘」这一条早已写进纪律，但**只有仓外备份 + 写后重试才真的兜得住**。

- **门禁读数（同一条命令）**：全量 **8639 passed / 11 skipped / 0 failed / 覆盖率 99.81%**（166.24 s；与 L232 的 8621 相减 `+18` = 新守卫 18 支，接线与产品面改动**零后端用例增量**）。六把账本与文档守卫 **149 passed / 8 skipped**（写清单行而**还没写日志块**的中间态首跑红 2 格：`test_the_checklist_does_not_invent_rounds` 与 `test_the_tail_above_the_boundary_is_contiguous`——那正是「双写缺一处即红」在起作用，补上日志块后全绿）。前端三门禁：`tsc --noEmit` rc=0、`eslint . --max-warnings 0` rc=0、`vitest run` **17 文件 / 214 → 227 支全绿**。**census 两把棘轮一格未动**（421 / 303 / 118）：新守卫**零 `parametrize`**（刻意不用，免得又去动 l97 的 census 常量），l98 从 l97 `import` 同两个常量故一并原地绿；哈希槽那把尺子只读**第一本账**，本轮动的是第二本 ⇒ 同样未动。

- **净零行管控**：四处文档全部同行改写（`git diff --numstat` README 2/2、API.md 3/3、ARCHITECTURE.md 1/1、DEPLOYMENT.md 1/1），`System.tsx` 7/7、`System.backup.test.tsx` 1/1、`api.ts` 里 `registerDependencyDataset` 那段注释也是同行改写 ⇒ 历史行号引用无一平移（a184 与 l79 两把原地绿）。**净增行只落在本轮新增的那几处**：`api.ts` 65/10、`Settings.tsx` +79、三份测试文件与新守卫；已确认仓内没有 `Settings.tsx:NNN` / `test_backup_wiring_l231.py:NNN` 形式的引用，`api.ts:NNN` 形式的引用只出现在账本 `.md` 里（l79 只匹配 `.py:NNN`）。

- **待拍口径**：**A279** 剩 8 员不变（`CLI_ENTRY` 7 员 + `NO_ENTRY` 1 员 = `translate`）；**其中 `translate` / `migrate` / `stream` 三条正是「挂门禁但还没接线」那一格的全体成员**，接进页面时新守卫的 `GATED_NOT_YET_CONSUMED` 要跟着挪（那一支注入臂已经证明它会响）。**A302 / A312 本轮关闭**。**A298~A301 / A303 / A304**（L231 那六条备份面缺口）、**A306~A311**（L232 那六条血缘面缺口）、**A283**、**A280**、**A278**、**A276**、**A271** 的 18 只待处置键、**A272**、**A273** 仍待拍。**用户本轮已拍板两件**（记在这里免得下一轮又当成未决）：① **`GET /api/system/dependencies` 与 `POST /api/dataset/translate` 两员都接，消费面做到 75/75**——L226 那条「指标涨而能力不涨的接线一律不做」的纪律被用户**撤销**（至少对 `dependencies` 这一员），l224 名册里那条拒接理由要同轮改掉；② `translate` 那一员**先收掉 A283**（它的契约缺口）再接，且页面上要写明「逐条调模型、按 token 计费」。

### L234（2026-10-11）— A279 第九批接线：系统那一族五条进页面，消费面 67 → 72/75；同轮收 A313 / A314 / A315

- **这一轮做三件事**：① 五条接线——`monitor` 与 `auto-test` 进系统状态页第五张卡（都只读、都**不挂**门禁），`migrate` 与 `stream` 进第六张卡（都写盘、都**挂**门禁），`validate-config` 进配置管理页新卡（只读、不挂门禁）；② 接线途中量出的三处产品缺陷同轮收掉（**A313** CLI 的 `export` 是原样搬运、**A314** `data/versions/current` 那一格把 `create_version` 整个炸掉、**A315** A168 残留守卫的范围欠账）；③ 两份新守卫（`test_system_ops_wiring_l234.py` 36 支、`test_version_current_link_l234.py` 10 支）+ 前端三面（System.ops 13、Settings 4、api.test 4）。**消费面 67 → 72（75 条的 96%）**、`UNCONSUMED` 8 → **3**、`CLI_ENTRY` 7 条/7 名 → **2 条/2 名**（只剩 `health-gate` / `doctor`）、`NO_ENTRY` 仍 **1**（`translate`，归 A283）。

- **五条的门禁归属是逐条读出来的，不是按「写操作都挂门禁」推的**：`migrate` / `stream` 挂 `verify_api_key`，而 `monitor` / `auto-test` / `validate-config` **都是 POST 却都不挂**。按 HTTP 方法推会把三条只读端点也算成「要送 key」，页面上就会多三处假口径。l233 立的 `GATED_NOT_YET_CONSUMED` 那一格随之 **3 → 1**（只剩 `translate`），新守卫的 `key_numbers` 注入臂实证它会响。

- **A313：`stream` 的 `operation` 清单两端各错一半，而 CLI 独有的那一档是原样搬运**。CLI 的 `--operation` `choices` 写 `quality / dedup / clean / export`，端点认 `quality / dedup / clean / none`。`export` 被 argparse 收下之后落进 handler 的 `else` 臂，而那一句只是 `lambda items: items`——名字承诺的「导出」从来没发生过；反过来 HTTP 侧合法的 `none` 在 CLI 里被 argparse 直接拒（退出码 2）。**最贵的一格是它已经被钉成了事实**：既有测试的 docstring 逐字写着「export 操作为直通处理」，那把尺子改前是绿的、而且它在为谎话作证。修法是把 CLI 那份字面改成与端点同一份，**没有新插 Python 常数**——把权威搬进 `augmentor/streaming.py` 会在 `WRITER_FORMATS` 之后插行、平移账本 A147 那条活引用（`test_doc_line_refs_l79` 与 `test_comment_refs_a184` 会同时红），而仓内既有惯例（l226 / l175 / l189）也是「CLI 留字面、守卫对账」。**残留一格记在账上**：`export` 这个动作在产品面上仍然不存在（CLI 上曾经「看起来存在」），要真做导出得走 `POST /api/export/*` 那一族。

- **A314 是全量门禁首跑那唯一一支红倒推出来的，不是设计出来的**：`OSError: [WinError 1920] 系统无法访问此文件。: 'data\versions\current'`，触发点是 `create_version` 里那句「先看那一格有没有东西」。现场取证量到三件：① 那份 `current` 是 **WSL 在 drvfs 上留下的 LX 链接**——`is_symlink()` 是 **False**、`os.readlink()` 报 *not a symbolic link*、只有 `os.stat` 抛 1920；② 链接目标写的是**整段相对路径** `data/versions/v_…`，而 `Path.symlink_to` 的相对目标按**链接自己所在目录**解析 ⇒ 实际指向 `data/versions/data/versions/v_…`，**恒断链**（成因是 `storage_dir` 的出厂默认就是相对路径）；③ 那个目录里当时堆着 **67** 个用例跑出来的 `v_*` 快照（⇒ A315）。**我最初的修法被探针当场推翻**：原打算「先判 `is_symlink()` 再判 `exists()`」，而 ① 量到的正是 `is_symlink()` 也认不出来 ⇒ **重排治不了它**。改成把「判存在 + 清除」搬进 `_clear_current_link()`，**stat 抛出来就当作「那里有东西」照清**；清除按「先当文件删、删不掉再当目录删」两步走，两步都失败就放过（这一格是便利索引不是数据，建不出新的也不该让 create/rollback 失败）。**两处改动都刻意做成净零行**：账本里有三处活引用指着这只文件（`:197` / `:281` / `:372`），插行会把它们平移出代码跨度——第一版没压准，`with open(` 落到 196，把 `except OSError:` 下面那条注释拆成两行补回 197，`git diff --numstat` 复核 18/18 之后三个锚点逐个复核未动。**刻意没做的一件事**：不加 `target_is_directory=True`——本机 `os.symlink` 无特权直接吃 WinError 1314，没法验证「目录符号链接在 Windows 上能否被 `unlink` 掉」（`DeleteFileW` 对目录符号链接会失败、要 `RemoveDirectory`），引入一个无法验证的失效形状违反「失败要响」。**修完的自愈证据**：那支用例转绿、原来读不了的 LX 链接被 `unlink` 掉（`ls data/versions/current` 报 No such file）、`current.txt` 重写成最新版本 id。

- **新守卫第 4 面首版是恒真的**：替身 `FakeCurrent.symlink_to` 恒抛 ⇒ `.txt` 每次都被重写 ⇒ **改前的老代码也绿**。加 `link_ok` 开关、把断言换成 `assert not txt.exists()`（链接建成了、`.txt` 还留着就是停在旧版本上），注入臂 `old_clear` 红 4 支、`no_txt_clear` 红 2 支、`full_path_target` 红 3 支 ⇒ 四面这才都有牙齿。

- **A315 的代价是量出来的，不是推的**：原判据只盯仓库根 / `data/` / `.backups/` **三处顶层**，理由是「`data/` 更深一层是产品自己的目录结构」——这话对 `data/nested/` 成立，对 `data/versions/` 不成立。**也就是说 A168 入库时（L94）量到的「全套 `RESIDUE_COUNT 0`」是真的，但它量的范围漏掉了产物真正落的那一层**：守卫不红不等于树是干净的。`WATCHED` 三处 → **五处**（加 `data/versions` 与 `checkpoints`），口径表那一行同轮改写。先跑「碰管道/版本」那一族 **47 个文件 / 1 499 条用例**，实测**泄漏 1 条**（`test_partial_branches_l120.py::test_malformed_variants_filtered_out`），就地补 `monkeypatch.chdir(tmp_path)` 收掉；扩范围之后全量 **8 687 条泄漏 0 条** ⇒ 与 L94 那次入库同形、**不需要白名单**。新棘轮 `test_the_runtime_dirs_are_watched_one_level_down`（把 `WATCHED` 改回三处它当场红），注入臂 `watched_three` 实证红 2 支。**断言里那份名单的顺序第一版写反了**：`leaked()` 按 `WATCHED` 元组序返回，不是字母序，`checkpoints/ckpt.json` 要排在 `data/versions/v_…` 之后。

- **门禁跑到第四遍才干净，三遍红全是我自己的改动造成的，逐格按各自 docstring 的处方重钉，没有任何一把尺子被改松**：① **首跑 1 failed** = A314 那支 WinError 1920（产品缺陷，修产品码）；② **第二跑 2 failed** = 我在 `test_repo_residue_l94.py` 第 227 行那处**已登记的子进程调用之前插了 23 行**，而 `test_subprocess_text_decoding.py` 的 `BYTE_MODE_ALLOWLIST` 是按「文件 + 行号」登记的 ⇒ 登记指到空处（`227` 不再是那个调用）、新位置 `250` 未登记，`test_every_byte_mode_site_is_registered` 与 `test_registration_still_points_at_a_byte_mode_call` 各红一条；按处方优先级走 **#1 换点**（新用例挪到文件末尾、`test_deeper_paths_are_not_watched` 的 docstring 压回一行）而不是重钉登记，227 复位；③ **定点复跑仍 1 failed** = 新用例的 docstring 里**写出了那个调用的字面形状**，被 `test_discovery_matches_a_direct_grep_of_the_same_pattern` 那把**纯文本 grep** 当成「AST 漏认的调用点」（它只排除判据自己那一份文件，散文里的同名字样照样命中）——这正是 `guard-assertion-scope-discipline` 说的「守卫被自己写的说明文判假红」，处置是**改自己的散文措辞**（写成「那处子进程调用」）而**不是**放松那把交叉验证的尺子；两格都写进新用例的 docstring 记档，免得下一轮再踩；④ **第四跑干净**。

- **接线守卫（Python 侧）首跑 3 failed、前端 `System.ops.test.tsx` 首跑 4 failed / 35 passed、vitest 首跑 1 failed / 226 passed**，逐格归因：Python 那 3 格全是「按实测回填」类（l224 的消费面数字与名单、l233 的门禁尾巴那一格）；前端那 **4 格全是我自己的判据写错**——表行查询没按卡限定、串到隔壁依赖表；antd 会给两个汉字的按钮文字中间插空格，按整串文本匹配恒红；必填闸挂在 Popconfirm 的 `onConfirm` 上（**这一格不是判据错，是量到了 A317**，就地改成「先确认再点」并把交互顺序记进账）；`pickOption` 按序号点开的是 `AutoComplete` 而不是 `Select`。另有一支 `expect(dropdowns).toBeTruthy()` 是**恒真**（空数组也是 truthy），换成逐项断言；三处注释与卡片文案写着与实测不符的话，全部改成实测口径。**A316 / A318 都是这一步量出来的**（页面文案里那三个阈值没有机器对账、`is_valid` 没有可引权威规则），所以卡片正文刻意**不写成定义**——写的是「合法也不等于没有问题清单」加上出厂那份实测带着 5 条 API_KEY 警告。

- **注入取证十三臂逐臂点名喊红**：接线六臂（`cli_choices` / `page_prose` / `sentinel` / `settings_prose` / `key_numbers` / `clean_identity`）+ 前端三臂（字节级注入，三份文件 sha256 复原后与进场值相等：`api.ts 2a58a060e083`、`Settings.tsx 917dc6af6088`、`System.tsx 32e53ddae8ef`）+ versioning 与残留守卫四臂（`old_clear` 红 4 / `no_txt_clear` 红 2 / `full_path_target` 红 3 / `watched_three` 红 2，**后四臂全走零写入通道**——pytest 插件在导入后的模块对象上打补丁，磁盘一字未动）。两次收尾验盘哈希相同：`cc1a80b8…`（接线批）与 `c17d122404c30688de4f69a767f6be3e84262152`（versioning 批）。

- **三处工具级失误，都记在这里免得下次当成产品问题查**：① `sha1sum` 的输出带 ` *-` 后缀，`[ "$AFTER" = "c17d…" ]` 因此假失败并打出「⚠️ 哈希不同」，加 `cut -d' ' -f1` 之后才读到真值（零残留）；② `grep -rl` 忘了 `--include="*.py"`，把 `tests/**/__pycache__/*.pyc` 也当 pytest 参数 ⇒ `ERROR: not found: …cpython-311-pytest-9.1.1.pyc`、`no tests ran`，加上之后文件数 117 → 47；③ Bash 的 cwd 在长会话里会漂（本轮两次：`grep tests/` 报 No such file、定点复跑报 file not found），处置是每条命令显式 `cd` 到项目根。

- **读数**：门禁 `PYTHONIOENCODING=utf-8 …python.exe -m pytest tests -q -p no:cacheprovider` ⇒ **8687 passed / 0 failed / 11 skipped / 覆盖率 99.83% / 182.41 s**；与 L233 的 8639 相减 `+48` = 接线守卫 36 + l233 新棘轮 1 + versioning 10 + l94 新棘轮 1，11 skipped 与 L233 是同一批。前端 `tsc --noEmit` 与 `eslint . --max-warnings 0` 均 rc=0，`vitest run` **17 文件 / 227 → 18 文件 / 248 支全绿**（`+21` = System.ops 13 + Settings 4 + api.test 4）。census 三数（**421 / 303 / 118**）一格未动——两份新文件里 `parametrize` 出现 **0** 次；l98 哈希槽只读第一本账，本轮动的是第二本 ⇒ 六档现量同样未动。

- **待拍口径**：**A279 剩 3 员**（`health-gate` / `dependencies` / `translate`）。用户上一轮已拍板的两件本轮开始生效：`dependencies` 与 `translate` 两员都接、消费面做到 **75/75**，L226 那条纪律**只在这两员上被撤销**（账上留着原话，别处仍然成立）；`translate` 仍要**先收 A283 的契约缺口**再接。**收口那一轮必须复验一件事**：l224 那把「名单与实测集合相等」的判据在**空集相等**时仍会响——否则「`UNCONSUMED` 归 0」这一步等于把尺子读成恒绿。**A313 / A314 / A315 本轮关闭**；**A316 / A317 / A318 本轮新立待拍**（页面写死三个阈值无对账 / 必填闸在二次确认之后 / `is_valid` 无可引权威）；**A298~A301 / A303 / A304**（L231 那六条备份面缺口）、**A306~A311**（L232 那六条血缘面缺口）、**A283**、**A280**、**A278**、**A276**、**A271** 的 18 只待处置键、**A272**、**A273** 仍待拍。

- **本轮刻意没做的三件事**：① 不给 `symlink_to` 加 `target_is_directory=True`（无法验证的失效形状，见 A314）；② 不把 `export` 做成真的导出（那是 `/api/export/*` 那一族的能力，本轮只把 CLI 那份假清单收掉）；③ 不给 `is_valid` 写一条推出来的规则（只钉观测到的行为，见 A318）。

### L235（2026-10-11）— A279 第十批接线：`health-gate` 进质量中心，消费面 72 → 73/75；同轮量出 A319 / A320 / A321

- **这一轮做两件事**：① 把 `POST /api/quality/health-gate` 接进质量中心新卡「健康门禁」，页面 docstring 那句能力分类跟着改口径（八项 → 九项；前五项的结论并进指标卡，后四项——离群点 / 画像 / 结构校验 / 健康门禁——只回答「这批数据长什么样、能不能放行」）；② 让 `/api/quality/evaluate` 的 `pass_rate` 能**一键喂进门禁**（按钮 `disabled={!evaluation}`，喂的是 `evaluation.pass_rate` 原样）。**同轮把接线途中量出来的三条脾气立成 A319 / A320 / A321**，并把它们写在页面上（那张「读这一页之前要知道的六件事」），不是只留在账本里。本批**后端与 SDK 一行未改**——三条脾气都是「后端一直这样、只是从前没人从页面上读过」；改的只有前端三件（fetcher / 类型 / 页面）、两份新测试与 l224 名册回填，外加 `README.md` 页面表那一行。

- **「一键喂通过率」是本轮唯一有语义风险的接线点**：`pass_rate` 是 Optional，省略 ⇒ 后端把 `pass_rate` 这条规则放进 `skipped_rules`（**不是判负**）；而 `evaluate` 真的会回 `0.0`（一份全不合格的数据集就是 0.0）。所以 fetcher 里那一行**刻意不写 `|| undefined`**——写了就等于把「通过率为 0」这条真判决静默降级成「没填」。守卫 `test_pass_rate_must_not_use_the_or_sentinel` 与渲染面那支「评估回 0.0 也喂得进去（不是被当成『没填』丢掉）」各钉一半，注入第一臂就是把 `||` 加回去，当场点名喊红。

- **另外三处哨兵动机各不相同，且都不写第二套默认**：`text_field` 走 `|| undefined`（**躲判决**——空串会被后端当幽灵字段，`duplicate_rate` 回 1.0，见 A321）；三档阈值与 `block_on_warning` 必须转 `undefined` 而**不能送 `null`**（非 Optional 字段送 `null` 是 422 `float_type` / `bool_type`，而 Optional 的 `weights` / `pass_rate` 送 `null` 是 200 ⇒ `null` 的语义在两侧**不对称**，这是实测不是推理，守卫 `test_null_is_asymmetric_between_optional_and_required_fields` 钉住）；`weights` 不做长度过滤（「必须 4 个元素」「之和必须为 1.0」那两句 400 是**真判决**，前端替后端过滤等于把尺子藏起来）。页面初值与契约默认同值（`instruction` / 0.6 / 0.3 / 0.8 / `false`），所以「不填」与「填默认」在页面上是同一档，没有第二套默认在暗地里生效。

- **A319 / A320 / A321 三条都是仓外探针量出来的**（`l235_probe6.py`，产物只落 Temp：`AUGMENTOR_DATA_ROOTS` 指向 mkdtemp、`model_manager._sentence_model` 打成 `fallback`；探针末尾自带仓库 dirty check，输出为空）：健康分那一族 `[0,0,0,1]` ⇒ 1.0、`[2.0,-0.5,-0.25,-0.25]` ⇒ **1.1697944143478562**（`level` 仍 `healthy`、`verdict` 仍 `failed`、`duplicate_rate` 仍 0.5 ⇒ 门禁三条规则不看权重）；容差那一族 `[0.24,…]` 与 `[0.26,…]` 都 400（浮点差 0.010000000000000009 > 0.01）、`[0.255,…]` 200；`text_field` 那一族 `""` 与 `"no_such_field"` 都回 `duplicate_rate` 1.0 且 `failed_rules == ["duplicate_rate"]`，而 `"output"` 是 0.5；三档阈值那一族 `-1.0` / `5.0` / `-3.0` / `-0.1` **全收**且判决随阈值单调变化。**页面文案在探针之后改过两处**：原先写「`[2.0,-0.5,-0.25,-0.25]` 能压到 0.25 / poor」（实测是 1.1697 / healthy，`health_score` 没有上界钳制）、原先写 failed ＝「至少一条 error 级规则判负」（实测 `block_on_warning=true` 时 `verdict=failed` 而 `failed_rules` **为空**，判负住在 `warned_rules` 里）。

- **守卫首跑红 1 格，是我自己的尺子范围错，不是产品缺陷**：`assert "0.25" not in 整页` 被页面里那句**如实解释后端默认**的文案（「默认权重 [0.25 × 4]」）判红。按「扫描面与禁令面同族」收窄到 `healthGate(file, {…})` 调用点那一段区块、断言区块内一个数字字面都没有；同时留一支反证免得禁令恒真——`test_the_page_admits_the_thresholds_have_no_range_gate` 里既断言三档阈值那一段没有 `min={` / `max={`，也断言 `pass_rate` 那一格**确实**设了 `min={0}` / `max={1}`。

- **渲染面首跑红 3 格，全是判据与取证问题，没有一处是改页面糊过去的**：① `toHaveProperty('value', '0')` 实得 `'0.00'`（`InputNumber step={0.05}` 的显示精度）⇒ 改断言并保留真正要紧的那句（递下去的 `passRate` 是 `0`）；② 那句「健康分本身不参与门禁判决」住在**卡上的 Alert**、不在回执 div 里 ⇒ 改成按 `document.body.textContent` 断言；③ 第三支反复红，根因是**用两份一次性 `Dbg.test.tsx`（建完在同一条命令里删掉）取到的**：任一动作跑过之后 antd 会在按钮里留一只 `ant-btn-loading-icon-motion-leave-start` 图标（自带 `aria-label="loading"`），jsdom 不放动画 ⇒ 可及名**永久**变成「loading健康门禁」，精确名查询恒失败而正则恰好 1 个匹配（按钮此时并没有 disabled，点得动）。仓内 `DataManagement.transform.test.tsx` 早有同一惯例，改按正则取并把理由写进注释。

- **注入取证十臂逐臂点名喊红**：Python 侧八臂（`pass_rate` 加回 `||`、`text_field` 去掉 `|| undefined`、`pass_rate_min` 改送 `null`、fetcher 里写死 `?? 0.3`、页面初值 0.6 → 0.5、`disabled={!evaluation}` → `false`、权重那句脾气被美化、TS 面 `failed_rules` 改成驼峰），前端两臂（摘掉 `disabled={gateWeightsPartial}`、改掉「健康分本身不参与门禁判决」）。锚点全部不含换行 ⇒ 直接在 bytes 上替换、**不碰行尾**；被改的三份文件逐臂字节复原、收尾断言 sha256 与进场值逐份相等（`8b457088…` / `2020a211…` / `6856f28a…`）、基线复跑全绿（33 passed 与 13 passed 各一遍）。**前端那两臂第一版判据是恒真的**：「用例名出现在 vitest 输出里」——vitest 会把全部 13 个用例名连同 ✓/× 一起打出来，所以那句话无论红绿都成立；改成「`FAIL` 行点名 + 失败计数恰好 1」之后第一次仍然匹配不上，原因是**输出带 ANSI 色码**（`cat -A` 看到 `^[[31mFAIL`），剥掉色码才认得。**「判据恒真」这一缺陷类本轮第三次出现**（前两次：L234 的 `expect(dropdowns).toBeTruthy()`、本轮 Python 侧那格整页扫描）。

- **两处工具级失误，记在这里免得下次当产品问题查**：① Bash 的 cwd 漂到 `augmentor` 根之后跑 `npx vitest`，用的是根目录那份配置 ⇒ 报 `window is not defined`（0 test），而且在根上留下一个只含 `.vite` 的空 `node_modules`，已删；② 同一次漂移里 `cd web` 在 cwd 已经是 `web` 时失败，整条 `&&` 链没跑。

- **读数**：门禁 `PYTHONIOENCODING=utf-8 …python.exe -m pytest tests -q -p no:cacheprovider` ⇒ **8721 passed / 0 failed / 10 skipped / 覆盖率 99.83%**（285.65 s）。与 L234 的 8687 / 11 相比 passed `+34`、skipped `-1`，而**收集总数 8698 → 8731 恰好 `+33`** = 新守卫 33 支 ⇒ 那 `-1` 是一格 skip 变成实跑。**已归因**：`test_serve_config_wiring_l97.py:119` 的跳过条件逐字是「本仓未构建前端产物，默认挂载点不存在」，而 `web/dist` 在 22:45 出现了（本会话只跑 typecheck / lint / vitest，三者都不产出 dist ⇒ 是并发会话的构建）⇒ 环境读数，不是产品面变化。前端三门禁：`tsc --noEmit` rc=0、`eslint . --max-warnings 0` rc=0、`vitest run` **18 文件 / 248 → 20 文件 / 276 支全绿**（`+28`：本轮那一支文件 13 支，另 15 支来自并发会话新增的 `api.optional-defaults.test.ts`；该文件此刻仍在被另一会话改写、单独复跑读 14 支，所以那个 15 是**全量跑当时**的读数，不追平）。census 三数（421 / 303 / 118）一格未动（两份新文件零 `parametrize`）。**typecheck 曾在本轮中段红过 1 格**，报错来自那只并发会话的在制文件（`aggregateDatasets(['a.json','b.json'], …)` 与 `api.ts:999` 的 `Record<string, string>` 签名不符），按约束**未 stage、未修改、未删除**，只向用户点明；该文件随后被其作者提交并修好，本轮收尾时 typecheck 已 rc=0。

- **待拍口径**：**A279 剩 2 员**（`dependencies` / `translate`）。用户拍板的两件仍生效：两员都接、消费面做到 **75/75**，`translate` 先收 A283 再接。**收口那一轮必须复验**：l224 那把「名单与实测集合相等」的判据在**空集相等**时仍会响——否则 `UNCONSUMED` 归 0 那一步等于把尺子读成恒绿。本轮已顺手验过它现在仍会响：接线之后、回填之前，`test_unconsumed_matches_the_roster` 首跑就报「已接好或已消失 ['POST /api/quality/health-gate']」（同跑另有 `test_frontend_face_scale` 报「实得 73」与 `test_roster_scale_is_pinned` 报「73」，三格全是按实测回填类）。**A319 / A320 / A321 本轮新立待拍**；**A316 / A317 / A318**（L234 立的三条）、**A298~A301 / A303 / A304**、**A306~A311**、**A283**、**A280**、**A278**、**A276**、**A271** 的 18 只待处置键、**A272**、**A273** 仍待拍。

- **本轮刻意没做的三件事**：① 不给 `weights` 在前端做「非负 / 和为 1」的校验（那两句 400 是后端真判决，前端替它拦等于把 A319 / A320 藏起来）；② 不给三档阈值设 `min` / `max`（后端没有范围闸，`-1.0` / `5.0` / `-3.0` 全收且判决单调变化，前端设闸会造出一条后端不存在的规则）；③ 不把「健康分」并进质量中心上面那四格指标卡（健康分不参与门禁判决，并进去会让「通过 / 过滤 / 通过率」与「能不能放行」两套口径混在一排）。

- **账本批之后复跑，读数有偏差，如实记在这里**：同一条门禁复跑 ⇒ 收集期直接 1 error 中断，报错点是 `tests/unit/test_version_current_link_l234.py` 里多出一行 JS 式的 `//` ⇒ `SyntaxError: invalid syntax`（**行号刻意不钉**：那一行正被对方改着，钉上去就是 A184 要禁的脆引用——实测把它写成带行号的引用，A184 硬 0 档当场红 1 处，理由是 `//` 被归成注释行、不落在代码跨度上；本条按处方第一条「换点」改成只写文件名，红随即收回。这次红是尺子对的、案面错的，留在这里当例子）。该文件是**并发会话的在制改动**（未提交，`git diff --numstat` = 10+/1−，主题是把「本机建不出符号链接」从平台偶然钉成夹具替身，因为 Linux CI 上 `os.symlink` 照常成功、`current` 是真链接、`.txt` 从头到尾不存在），本轮按纪律**不 stage、不修改、不还原**。改跑同一条命令加 `--ignore=tests/unit/test_version_current_link_l234.py` ⇒ **8711 passed / 0 failed / 10 skipped / 覆盖率 99.82%**（276.63 s）：比账本批之前少的 10 支恰好是被排除文件的用例数，覆盖率那 0.01 个百分点的差也来自同一处；本轮自己那一面（0 failed、skip 仍是 10 且三档归因名单逐格相同、覆盖率阈值 80 照过）与账本批之前**读数不变**。欠账写清：等那个文件修好，必须复跑一条**不带 `--ignore`** 的全量，才能拿回与 8721 可比的对账读数。

### L236（2026-10-12）— A279 收口：最后两员接完，消费面 73 → 75/75；同轮收 A283①、新立 A322

- **这一轮做三件事**：① 收 **A283①**——给 `POST /api/dataset/translate` 补落盘口契约（`TranslateRequest.output_file` / `TranslateResponse.output_file`，都可选、键**恒在**），② 把这条端点接进数据管理页新卡「批量翻译」，③ 把 `GET /api/system/dependencies` 接进系统状态页——**A279 十批接线到此收口，消费面 75/75（100%）**，l224 那三个名单（`UNCONSUMED` / `CLI_ENTRY` / `NO_ENTRY`）同时归空集，l233 那格「挂门禁但没接线」也归空（24 / **24**）。**同轮新立 A322**（`text_key` 零判决，后端一行未改）。用户此前拍板的两件在这一轮全部兑现：「两员都接，做到 75/75」与「先补 key 通道（L233）再继续接线」。

- **`dependencies` 这一员的「取法」是论证过的，不是顺手接一格指标**：L226 当年拒接它的理由是「指标涨而能力不涨」，用户显式撤销了这条裁定（**只针对这两员**，账上留着原话）。撤销之后仍要回答「接了到底涨什么」，所以先量两面数据是否相同、再量它们的**失败条件**是否相同：两条端点的诊断逐格相同（同出 `augmentor.diagnostics.check_dependencies`），但 `/api/status` 要真构造管道。把 `api.routes.status.get_pipeline` 换成抛异常的替身之后，**dependencies 读数一格未变**，而 status 退成 `model_default='unknown'` / `model_available=False`（两支都仍 200）⇒ 管道拉不动时这一面没有理由跟着黑掉。页面因此把依赖诊断从「综合状态取到才渲染」那道整体门里摘出来，改成自己取数、卡上自己一只刷新按钮，并留 `deps ?? status?.dependencies ?? null` 这条退路（**退路不是空的**：两面数据相同是实测，所以首载失败时渲染结果与从前逐格相同）。渲染面上还多了一句只在 `!status` 时出现的说明，把「这份读数来自不碰管道的那条端点」讲给用户。

- **A283① 的顺序是量过的，不是写出来的**：落点先 `resolve_data_path(for_write=True)`、再动模型 ⇒ 越界落点 403 出声时**模型调用 0 次**（替身数出来的）。这一格在花钱路径上是要紧的：参数错误必须先出声，不能先花三次 token 再告诉用户落点不合法。响应面刻意让 `output_file` **键恒在**——用「键在不在」区分不了「没落盘」与「落了盘但没回显」，所以两态都要在契约里可读（没给时 `null`，页面回执就照实写「未落盘（只在本次响应里）」）。`docs/API.md` 的 translate 说明行与 README 页面表两行都按这个口径**同行改写**。

- **A322 的第三键是本轮首跑量出来的新事实，不是设计出来的**：新守卫首跑 **2 failed / 16 passed**，两支红的都是 `set(on_disk[0])` 比预期**多一格 `language`**——追到 `augmentor/multilingual.py:190` 那句无条件 `new_item["language"] = target_lang`。探针当时只报了「产物凭空多两键」，所以立账口径当场从「多两键」升级为「**多三键**」，并把「一个字没翻也照盖语言戳」这件事写进四处（守卫 docstring / 页面卡片文案 / 前端用例断言 / `docs/API.md`）。**这才是这一条真正会骗人的地方**：回执四键全零 + 产物上写着 `language: en`，从任何一面都读不出「这次什么都没发生」。

- **注入取证五臂逐臂点名喊红，前三臂走零写入通道**：`atomic_write_json` 打成哑（pytest 插件在导入后的模块对象上打补丁）⇒ **4 支红**（落盘两支 + A322 两支，都是「盘上真有那一份」的判据）；`resolve_data_path` 去掉白名单判决 ⇒ **1 支红**，而且失败信息里赫然是根外那个绝对路径（`…\outside\escaped.json`）与三条译文，等于把缺陷现场打印出来；往 l233 的 `GATED_NOT_YET_CONSUMED` 塞回一条 ⇒ **1 支红**（`test_the_unwired_tail_is_empty`），证明**空集相等那一档仍会响**。后两臂的判据住在前端源码里，必须动文件，所以走字节级注入：`disabled={!trAck}` → `disabled={false}` ⇒ vitest **1 failed / 8 passed**；`{diag && (` → `{status && (` ⇒ vitest **1 failed / 17 passed** 且 Python 静态那一支（`test_the_detail_cards_are_no_longer_behind_the_status_gate`）同时红。两份文件逐臂复原、**复原后重新验盘**并断言 sha256 与进场值相等（`bfa38c4f…` / `27e15fea…`）。

- **一处工具级失误，记在这里免得下次当产品问题查**：第二臂那个注入器首跑在**打印环节**被 gbk 打断（`UnicodeEncodeError: 'gbk' codec can't encode '\u23af'`——vitest 输出里那个「期望/实际」分隔符），异常发生在 `try` 里，`finally` 照常复原并验盘（`System.tsx:432` 仍是 `{diag && (`，`grep` 复核过），但那一臂的读数没取到 ⇒ 单臂重跑一份把打印过 ascii 兜底才拿到。**注入器自己崩在打印上**与 L233 那次「崩在复原上留下残留」是同一族风险的两种结局：这次因为复原在 `finally` 里、且复原后有断言，仓库没被留下任何东西。

- **前端替身的默认值是一处刻意的判据设计**：四份既有 System 测试的替身工厂里，`systemDependencies` 默认就是**裸 `vi.fn()`（不给回执）**⇒ `deps` 落 undefined ⇒ 页面退回综合状态里那一份 ⇒ 那四份文件的渲染结果与 L236 之前**逐格相同**，新增能力的举证集中在新 `describe` 的三支里，不用为了接线去改一堆无关断言。**代价是踩到一次 vitest 陷阱**：新用例给这一面安排了缺失态的 DIAG 回执，而 `vi.clearAllMocks()` **不清实现**，那份回执漏进后面的用例，把向量面那条「必需依赖齐备」判成缺失态 ⇒ **1 支红**（`System 向量库状态 > 向量面失败只降级它自己`）。修法是给新 `describe` 加局部 `afterEach(() => spies.systemDependencies.mockReset())`——因为工厂默认就是裸 `vi.fn()`，`mockReset` 恰好还原成默认，不会顺手清掉别人的默认值。

- **收口那一格按 L235 的待办复验过**：`UNCONSUMED` 归 0 之后，l224 那把「名单与实测集合相等」的尺子仍会响——它带两支牙齿（`test_the_empty_claims_are_not_a_blind_ruler` 与用 monkeypatch 摘掉一处调用形状的 `test_unwiring_one_operation_turns_the_empty_roster_red`），而注入第五臂（把 l233 的空尾巴塞回一条）当场红了 1 支 ⇒ 「空集 == 空集」不是一把恒真的尺子。**这一格是 L235 明确挂给本轮的欠账，不是顺手一提**。

- **文档与行号引用**：README 页面表两行（数据管理页补翻译卡、系统状态页补依赖诊断单独取数）与 HTTP API 段那句消费面口径、`docs/API.md` 的 translate 说明行，全部**同行改写**，`git diff --numstat` 复核 **3/3 与 1/1 净零行** ⇒ 本账里那条 `README.md:65` 引用仍指得准（第 65 行仍是 `/system` 那一行，`sed -n '65p'` 复核过）。**顺手订正一处早已漂移的引用**：A283 行里那句 `api/routes/language.py:88`（指 `items` 那一格）在 L236 之前就已经漂到 `:94`，本轮加了两段 docstring 与一只请求字段之后是 `:105` ⇒ 同轮重钉。**漂移此前没人发现是有原因的，但原因不是「没人扫」**：`test_doc_line_refs_l79` 的 `DOCS` 只有 `docs/ARCHITECTURE.md` 与第一本账，而 A184 那把「引用必须落在代码跨度上」的尺子**是扫第二本账的**——它判**落点形状**（那一行是不是代码），不核**行号是否还指得着当初那一格**，所以 `:88` 从「`items` 那一行」漂进「docstring 里的一行」它照样放过（docstring 行算代码跨度）。**本轮自己就被这把尺子红了一次**：日志里原本写着 `DataManagement.transform.test.tsx` 的那条行号引用，而它指的是一行**注释** ⇒ `dead_line` 硬 0 档当场响，按处方第一条「换点」改成只写文件名、不带行号。**这次红是尺子对的、案面错的**。「行号是否还指着当初那一格」这一维在第二本账上确实没有尺子，扩面要先把全部历史引用逐条验真，是一轮的活，本轮不做。`System.tsx` 那一面本轮所有插入都在第 300 行之后，故 `DataManagement.transform.test.tsx` 的注释里那条 `DataManagement.tsx:96` 未被平移；`System.tsx:3` 那条引用（本账第 339 行）按「只能同行改」的口径处理——第 3 行的 `systemDependencies` 是**同行追加**的。

- **读数**：门禁 `PYTHONIOENCODING=utf-8 …python.exe -m pytest tests -q -p no:cacheprovider` ⇒ **8741 passed / 0 failed / 10 skipped / 覆盖率 lines 99.83%、branches 99%**（363.76 s）。与 L235 的 8721 / 10 相比 passed `+20`，**收集总数 8731 → 8751 恰好 `+20`** = 新守卫 18 支 + 并发会话 `8041df804` 给 `test_version_current_link_l234.py` 补的参数化 2 支（那个文件的 `def test_` 仍是 10 个，多出来的 2 格是参数化）。**这一跑不带 `--ignore`，L235 记下的那笔欠账已还清**：当时因为并发会话把该文件改成 `SyntaxError` 而排除，如今它已被其作者修好并提交（`8041df804`），所以这一份读数与 8721 可比。六把账本与文档守卫**首跑 3 failed / 146 passed**，三格全是本轮写账本时自己写坏的，逐格归因后收口成 **149 passed / 8 skipped**：① `test_bold_markers_paired` 红——A279 行那句 `CLI_ENTRY` 的加粗跨度原本闭在「1 条/1 名」之后那对星号上，我插入「→ **0 条/0 名**」时把那个**收尾标记吃掉了**（`**` 是逐块配对的，吃掉一个整块就变奇数）⇒ 改成把收尾落在 0 条/0 名 之后；② A184 硬 0 档红——日志里那条 `DataManagement.transform.test.tsx` 的行号引用指的是一行**注释**（`dead_line`）⇒ 按处方第一条「换点」改成只写文件名、不带行号；③ 同一处红还**推翻了我自己写进账本的一句判断**（原写「第二本账里的行号引用不在任何尺子的扫描面上」，而 A184 明明扫到了它）⇒ A283 行与日志块两处按实测订正成「A184 **是**扫第二本账的，它判**落点形状**（那一行是不是代码跨度）而不核**行号是否还指着当初那一格**」——错判断不留在账上。**订正过程中同一格缺陷又踩了一次**：为了把①的原文引进账本，我在归因那句里写了一对「半个」加粗标记（只出现一次的两个星号），粗体配对维当场红 1 failed / 148 passed ⇒ 改成用中文说「那对星号」而不是把标记本身抄进来。**教训是具体的**：账本里引用加粗原文时，宁可描述它，也不要抄它。（改文档之后、写账本之前那一遍是 149 passed / 8 skipped，未含本轮账本改动。）前端三门禁：`tsc --noEmit` rc=0、`eslint . --max-warnings 0` rc=0、`vitest run` **21 文件 / 287 支全绿**——`+11` 拆开是本轮 translate 9 支 + System 3 支 − 并发会话在 `api.optional-defaults.test.ts` 删掉的 1 支（15 → 14，`8041df804` 的提交说明写明「移除 healthGate 用例」），276 + 12 − 1 = 287，账对得平。census 三数（421 / 303 / 118）一格未动（两份新文件零 `parametrize`），l98 哈希槽只读第一本账故同样未动。**账本批之后复跑同一条门禁：8741 passed / 0 failed / 10 skipped / 99.83%（339.10 s）⇒ 与账本批之前逐格相同，无偏差可记**（L235 那一轮正是在这一步撞上并发会话的 SyntaxError，本轮没有）。

- **待拍口径**：**A279 已收口**，接线队列清空。本轮新立 **A322**（`text_key` 零判决 + 语言戳）待拍；**A319 / A320 / A321**（L235）、**A316 / A317 / A318**（L234）、**A298~A301 / A303 / A304**、**A306~A311**、**A281**（3 只 TS 类型缺契约字段，含 `StatusResponse` 少 `model_retry_count` / `model_retry_wait_seconds`、`AppConfig` 少 `versioning` / `visualization`）、**A280**、**A278**、**A276**、**A271** 的 18 只待处置键、**A272**、**A273** 仍待拍。**下一轮建议**：A319 / A320 / A321 / A322 是同一族「判决缺失面」（四条里有三条已在页面上写明、后端一行未改），可并表一轮收；A281 是纯前端类型补齐，零风险，适合作为收口后的第一批。

- **本轮刻意没做的三件事**：① 不给 `text_key` 在前端之外补任何后端判决（A322 要拍的是「400 点名」还是「回一个 `text_key_found`」，两者都改对外契约）；② 不把 `language` 那一格改名成 `target_lang`（改名是破坏性变更，且它同时是产物键与落盘键）；③ 不给翻译卡加「预览落盘内容」一类东西（这一支是写盘端点，回执只在点完之后出现——与采样/聚合/RAG 三张卡同一口径，前端用例里有一支专门钉「页面上没有『预览』这两个字」）。
### L237（2026-10-12）— A281 收口：TS 响应类型补齐 12 处 + 两层「TS ↔ OpenAPI 契约」守卫 + 两处渲染补全

- **本轮主叙事是「把尺子做深一层，数字自己变了」**：A281 立账写 3 处，本轮实测 **12 处**。差的那 9 处不是新缺陷，是
  原来那把尺子量不到的面——`ConfigResponse` 的九个节在 OpenAPI 里全是 `{"additionalProperties": true, "type": "object"}`，
  **没有 `properties`** ⇒ 节内键压根不进契约面，静态对账在节名那一层就停了。所以守卫必须两层：第一层静态（fetcher 的
  `Promise<X>` 字段集 ↔ 该操作契约 200 响应的属性集，**按操作配对而不是按同名类型配对**），第二层打只读的
  `GET /api/config` 递归比键树。`AppConfig` 缺的 9 处里 7 处住在节内（`vector.dimension/storage_dir/collection`、
  `rag.chunk_size/chunk_overlap`、`multimodal.image_extensions/audio_extensions`），第一层永远看不见；
  `versioning` / `visualization` 两节是第一层能看见的那一类。补完类型之后同轮渲染，因为「类型上有了、页面上看不见」
  仍然是一处死面（按 A138 那一档口径）。

- **四支仓外探针的读数原样留着**（`Temp/l237_probe1..4.py` 与同名 `.log`）：probe1 走「同名类型」轴（53 只同名 /
  111 interface / 2 别名 / 126 schema，A 段只报出 `StatusResponse`）——**这条轴是错的**，同名类型不代表调用关系；
  probe2 回到 A281 原轴（83 只 `export const` / **75 只带 `Promise<>` 标注且路径可解析** / 改前 70 相等 + 3 不等 +
  0 非具名 + 2 解析不动形状）；probe3 dump 三只契约形状（末尾那句 `from augmentor.config_schema import CONFIG_SCHEMA`
  抛 `ModuleNotFoundError`——真身在 `augmentor/config.py`，探针没修，三段 schema 已经打完够用）；probe4 才是本轮靶子的
  权威来源（活响应递归比对 ⇒ 缺 9 / 缺 2 / 缺 1，反向幽灵 0），它自带尺子自证断言块。

- **两处是尺子错，不是产品缺陷，逐条点名**：⑴ probe4 第一版把跨行内联块 `"".join` 压成一行再用贪婪正则切 ⇒
  只切出第一个字段，制造出 `num_threads` / `threshold` / `formats` / `backend` 四处假「后端在回、TS 没有」；
  修法是 join 用 `\n`、`parse_inline` 改成多行按行切 + 单行按顶层逗号切，并补那条尺子自证（读回 `AppConfig` 八节、
  三处内联字段表、`StatusResponse` 五键）。这条教训同轮写进守卫模块 docstring 的口径 3。⑵ 守卫首跑 **1 failed**：
  `test_layer_two_sees_the_seven_the_contract_cannot` 的期望列表我把 `visualization` 写在 `versioning` 前面，而 `sorted()`
  是 `versioning` 在前（`e` < `i`）⇒ 改断言顺序，**没有任何一把尺子被改松**。

- **前端新用例首跑 4 failed / 18 passed（两文件合跑），四支红全在我自己写的那份文件里**：① 三支栽在
  `getByRole('button', {name: '保存配置'})`——那只按钮带 `SaveOutlined`，可访问名实测是 `save保存配置`，
  既有的 `Settings.test.tsx` 用的是正则 `{name: /保存配置/}`，新文件照它改；② 一支栽在档位取错——
  「success 两边都是 true」那句住在回执卡 `saveResult` **非空**那一档的 Alert `description` 里，而那条用例没点保存，
  页面显示的是「还没保存过」那一档 ⇒ 恒超时，改成先 `save()` 再断言，并把「为什么这句话只在已保存档出现」写进用例注释。
  修后 9 支全绿。**这是判据错，不是产品缺陷**。

- **一次探针事故，记下来免得下次当产品问题查**：第一支 `TestClient` 探针打在**服务自己那份** `config.yaml` 上，
  而 `applied` 非空即 `save_config` 整份重序列化 ⇒ 磁盘从 457 行 / 60 行注释变成 **186 行 / 0 行注释**（`git diff --numstat`
  读 `-334/+63`）。这正是 **A152①** 描述的那一格，探针把它当场实现了出来。已 `git checkout -- augmentor/config.yaml`
  还原（现量 457 行、60 行注释、`git status` 对该文件干净），坏档留 `Temp/l237_config_damage.yaml` 作证据。
  **处方**：打**写盘类**端点的探针必须先把 `AUGMENTOR_CONFIG_PATH` 指到仓外副本——本轮第二支三档探针就是这么跑的，
  才量到 `{}` ⇒ 「本次请求未写入任何配置项」且磁盘 sha256 不动这一档（这一档从配置页不可达，因为 `handleSave`
  恒带 `default_model`）。

- **注入取证 11 臂，两类通道**：守卫内 5 臂走**内存字符串手术**（零写入，改的是 `TsFace` 读到的字符串副本）——
  摘掉 `Promise<StatusResponse>` 标注 ⇒ `unannotated == ["getServiceStatus"]`；删掉 `model_retry_count` 那一行 ⇒
  `mismatched` 点名 `getServiceStatus` 且 `ts_missing == ["model_retry_count"]`；把返回型换成 `NoSuchTypeAtAll` ⇒
  落进 `unresolved` 而不是被跳过；删掉 `versioning` 整节 ⇒ 第二层报缺；单删 `rag.chunk_size` ⇒ 第二层报
  `missing2 == ["rag.chunk_size"]`。渲染面 6 臂**必须动源码**故走字节级注入，每臂锚点唯一性先断言、写完跑 vitest、
  `finally` 复原并比 sha256，跑完再复验基线（`System.test.tsx` 13 支、`Settings.readonly.test.tsx` 9 支全绿）：
  S1 摘掉 `null` 那一档 ⇒ 1 红、S2 改掉「模型重试次数」标题 ⇒ 2 红、T1 摘掉只读回显卡标题 ⇒ 1 红、
  T2 把 `ignored_keys` 的个数写死成 0 ⇒ 1 红、T3 摘掉回执卡标题 ⇒ 4 红、T4 把节内键的值那一格改成只渲键名 ⇒ 2 红。
  **T1 只红 1 支太窄**（四支里只有它按标题查），所以补 T4 证明「含本轮新补的 7 个节内键」那条断言真有牙齿。

- **A323 新立、只立账**：口径按上面那次实测收窄——**不说「页面在什么都没写成时谎称已保存」**，因为空写入那一档
  从这一页不可达；说的是 toast 拼的是**第三处**措辞、不读 `message`，所以后端把忽略清单拼进 `message` 的那一档
  （实测 `qualty` ⇒ 「…（已忽略未写入的配置项：qualty；是否想写 quality？）」）在 toast 里必然丢失，
  而 L237 的回执卡已经把这一格变得可见。同批顺手量到「空串 `default_model` 会真落盘」，它落在
  **B229 / A85 族**既有口径里（校验面对悬空 `models.default` 出 warning），所以**不另立 A 号**。

- **文档三处，两处同行改写、一处末尾追加**：`docs/API.md` 的 status 段补「`model_retry_count` /
  `model_retry_wait_seconds` 管道缺失时为 `null`、前端按两档渲染」（1/1 净零行）；`docs/ARCHITECTURE.md` §3.12.5
  末句追加消费侧现状 + A323 指向（1/1 净零行）；§7.1 追加第三条 bullet 讲两层尺子（+14 行，**刻意落在文末区段**——
  全仓对 `ARCHITECTURE.md` 的活引用只有 `:1232` 一处、对 `API.md` 的是 244 / 282 / 952，两处 1/1 改写不产生位移，
  那 14 行在全部活引用之后）。`git diff --numstat` 复核 API.md `1/1`、ARCHITECTURE.md `16/2`。

- **读数**：全量门禁 `PYTHONIOENCODING=utf-8 …python.exe -m pytest tests -q -p no:cacheprovider` ⇒
  **8762 passed / 0 failed / 10 skipped / 覆盖率 99.83%**（204.28 s）。与 L236 的 8741 / 10 相比 passed **+21**、
  收集总数 8751 → **8772** 恰好 `+21` = 新守卫 21 支，**没有别的增减**（本轮零产品后端改动）。
  六把账本与文档守卫 + 新守卫同跑 ⇒ **170 passed / 8 skipped**（那 8 支 skip 是 `test_doc_line_refs_l79` 的
  作者工作树档，历史既有）。前端三门禁：`tsc --noEmit` rc=0、`eslint . --max-warnings 0` rc=0、
  `vitest run` **22 文件 / 299 支**（287 + 12 = 新 `Settings.readonly.test.tsx` 9 支 + `System.test.tsx` 3 支；
  文件 21 → 22）。census 三数一格未动（新守卫零 `parametrize`）。**本轮没有跑带 `--ignore` 的任何读数。**
  **账本批（`e6718f828`）之后同一条命令复跑** ⇒ **8762 passed / 0 failed / 10 skipped / 覆盖率 99.83%**，与账本批之前的四格逐格相同、
  **无偏差**（`skipped` 的三条理由一字未动：8 支作者工作树档 + `test_config_gates_l82:573` + `test_upload_ceiling_l87:580`；
  `TOTAL` 行 14210 语句 / 8 未覆盖 / 3932 分支 / 22 未覆盖分支、`Required test coverage of 80% reached`）；
  用时 204.28 s → 299.37 s 是同一台机器上的负载差，不是读数差。覆盖产物 `coverage.xml` 每轮重写、不参与对账。

- **待拍口径**：**A281 已收口（L237）**。本轮新立 **A323**（toast 不读后端 `message`）待拍；**A319 / A320 / A321 / A322**
  （同一族「判决缺失面」，**下一轮建议**：L238 按用户已拍的顺序收这一族四条，它们要改对外状态码与响应值域，
  与本轮的「类型 / 渲染」证据链分开走）、**A316 / A317 / A318**、**A298~A301 / A303 / A304**、**A306~A311**、
  **A282**（format 基线红且 CI 无门）、**A280**、**A278**、**A276**、**A271** 的 18 只待处置键、**A272**、
  **A273** 仍待拍。

### L238（2026-10-12）— 判决缺失面四条收口（A319 / A320 / A321 / A322）：同一条禁令的两份措辞合成一份，顺带解释器整体失效重建

- **本轮主叙事是「判据只许住一处」**：四条缺陷的共同根不在各自那一格，而在**同一条判据被抄了两份、两份不
  一起改**——权重原本 `api/routes/quality.py` 里手写一道「个数 = 4 / 之和 ≈ 1.0」，`DatasetHealthScore`
  里另一道（只判长度，不判符号与上界）；字段名原本 impact 有闸、health-gate 与 translate 没有。修法因此
  是把判据搬进 `augmentor/validation.py`（A77）：`require_ratio_list` 让三元与四元共取一个容差与一句
  措辞，新 `require_item_field` 让三面共取一次存在性判据；`_require_any_field` 从「自己判」降级成
  「一层 HTTP 外衣」（净行数压回 0，见下面 A184 那一格）。空串这一档的措辞也随之从「没有任何一条含字段 ''」
  改成「`text_field` 不能是空字符串」——**同一个 400，但后者说的是真凶**。

- **环境在这一轮整体失效，先重建再取数**：旧 venv `D:/WORKSPACE/venvs/augmentor` 的基础解释器随 WorkBuddy
  卸载而断（`.workbuddy/…/3.13.12/python.exe` 成悬空符号链接），`python -m pytest` 直接报
  `did not find executable`；本机唯一可用是 uv 托管 cpython **3.12.14**，于是按纪律在**仓库外**新建
  `D:/WORKSPACE/venvs/augmentor312/Scripts/python.exe`，照旧 site-packages 清单复装（含 openpyxl /
  opentelemetry-api / pandas）。门禁命令逐字：`PYTHONIOENCODING=utf-8 …augmentor312/Scripts/python.exe
  -m pytest tests -q -p no:cacheprovider`（gbk 控制台上不带 `PYTHONIOENCODING` 会在报告中段抛编码错）。
  两条后果如实记：⑴ **`skipped` 10 → 11**，新增那格是 `test_parametrize_blind_face_l98.py:641`——
  解释器名册只按 `[(3, 13), (3, 14)]` 现量过，3.12 这档按「不许猜、要先在该版上重量再钉」的设计跳过，
  不是缺陷也不是回归，**本轮没有重量那份名册**；⑵ 覆盖率 TOTAL 本轮 `14223 语句 / 8 未覆盖 /
  3922 分支 / 22 未覆盖分支`，L237 终态是 `14210 / 8 / 3932 / 22`：语句 +13 是本轮新写的判据与钳制，
  而**分支 −10 是一格未做归因的差值**——旧解释器已不可用，没法在同一份代码上重量对照，所以它既不记成
  改进也不记成回归，只记成「换解释器之后的口径差，待有 3.13/3.14 时复量」。本轮唯一可直接对账的是同机
  同解释器的两次全量（8800 → 8802，差的那 2 支就是补普查读数）。

- **四条的改法各选了哪一半、没选哪一半**（Backlog 行里逐条写了，这里只留最要紧的三条实测）：
  ⑴ 钳制臂**必须先把四项顶满**才越界——`[2.0, -0.5, -0.25, -0.25]` 在 L235 那份 4 条夹具上是
  1.1697944143478562，而本轮按「output 等长 / instruction 唯一 / 词不重复」构造的 `PERFECT_ITEMS`
  四项全 1.0 时，`[0.2525]×4`（和 1.01，容差内合法）才真的把 `raw` 顶到 **1.01** ⇒ 值域唯一剩下的
  越界来源就是那个容差，臂里保留 `raw > 1.0` 的自证，防止钳制判的是一个本来就出不了界的表达式。
  ⑵ 判序是量出来的不是设计的：`weights` 坏 + 文件不存在 ⇒ **400**（参数错在读盘前），
  `text_field` 幽灵 + 文件不存在 ⇒ **404**（没有数据就没有「首条实际字段」可点），空数据集判据先于
  字段判据，权重个数错先于 `text_field` 错——四档各配一支，写进 `TestTheOrderOfVerdicts`。
  ⑶ 两面之差诚实记录：pydantic 的 `Optional[List[float]]` 把 `'abc'` 挡成 **422**、把 `True` 吞成
  **1.0**，所以「布尔逐项拒」与「非数值点名下标」只在 SDK 直构面可测，HTTP 面那条 `[True, 0, 0, 1]`
  是被「和 = 2.0」拦下的——守卫把这一格写成「记录差异」而不是「假装两面同判」。

- **新守卫 `tests/unit/test_decision_gates_l238.py`（30 支 / 5 面）只量行为面测不出的那一维**：
  改前改后那些 `duplicate_rate=1.0` / `health_score=1.17` 的行为断言在 l235 / l236 两份守卫里已经
  有对偶，**抄两份判据在纯行为测试下是不会红的**（两边各自都判对），所以这五面是身份、判序、抄两份：
  `require_ratio_list` 用 `is` 判**同一个函数对象**（API 侧顶层绑定，可判身份）；`_require_any_field`
  的函数体里不许再出现 `in item for item in` 这类自判形状；反手抄臂用模式 `^\s*if\s+abs\(` 扫权重
  求和那一面；字段存在性那一族 SDK 侧是**函数体内 import**（避循环依赖），`is` 判身份不成立，改成
  monkeypatch 迟到绑定哨身、记 `('text_field', …)` / `('text_key', …)` 两格；每格配注入臂，反恒真
  那一支按计数响。

- **注入取证五臂（`Temp/l238_inject.py` → `Temp/l238_inject.log`，仓库外、字节级、finally 复原）**：
  每臂摘掉一处接线后跑定向选择，要求**点名**红，再逐字节复原。A 摘 `quality_gate` 的存在性闸 ⇒
  点名红 **4 条**（四档全响，另 4 条落在 l235 的同族断言上）；B 摘健康分出口钳制（`total = float(raw)`）⇒
  点名红 **1 条**（`test_the_tolerance_leftover_is_what_the_clamp_eats`）；C 把判和之前的量化退回裸
  `abs(total - 1.0) > 0.01` ⇒ 点名红 **2 条**（`[weights0-0.99]` 与 `[weights2-1.01]`，正是 A320 那两档
  该放行的）；D 摘逐项 `[0, 1]` 界 ⇒ 点名红 **2 条**（负权与 >1 各一）；E 摘 `language` 的 `text_key` 闸 ⇒
  点名红 **3 条**（`TestAThreeTwoTwo` 的「0 次模型调用」「点名首条实际字段」那几支）。五臂的
  「字节相同」全 True，进入时快照复验四文件全 True（`health_score.py` 5871 / `language.py` 8416 /
  `quality_gate.py` 11947 / `validation.py` 49814 字节），复原后整跑基线 **106 passed**，脚本自查
  「任何一步不满足 = 无」。**脚本瑕疵一处要记**：首版把 `same` 的比较写在 `write_bytes(base)` **之前**，
  拿的是注入态与基态比，于是每臂都打「字节相同 False」——那是尺子装错位置，不是仓库残留；本轮把比较挪到
  复原之后重跑（上面这份 log 就是重跑的结果），并用 `git status` / `git diff --numstat` 两份快照
  （`Temp/l238_git_before.txt` / `_after.txt`）交叉证明零残留。

- **A184 插行又平移了历史行号，处方仍按纪律走「压回净零 → 重钉 + 写归因」**：`dataset_tools.py` 第一版
  在 `_require_any_field` 的 docstring 里多写了「判据只有一份」那一段，硬 0 档当场红 **6 处** ⇒ 把整段
  挪成函数体内的注释并把文件压回与 HEAD 逐行对齐（`git diff --numstat` = 9/9、总行数 946 == 946、
  line 566 仍是 `    """`），6 → **2** → **1**；剩下的 1 处正是本轮要收口的 A319 行引用
  `health_score.py` 那一格原本钉在第 97 行（旧写法不再照抄进账：A184 把账里每一处 `文件:行号` 都当活引用量一遍）——那句 `"level": "healthy" if total >= 0.7 …` 因为钳制与 docstring
  下移到 **121**，于是随代码批一起重钉成 `augmentor/health_score.py:121`（`29b715600` 那一族先例：行号刷新即本条所在提交）。重钉之后
  六把账本与文档守卫 **149 passed / 8 skipped**。

- **普查读数回填（`Temp/l238_census.py` 现量，不推）**：本轮新写 5 支 parametrize（l238 四支：容差三档
  放行 / 两档拒、`text_field` 四档两面同文、逐项界两档两面同句；`test_health_score.py` 一支：`weights,
  idx` 四档点名下标），值位全是内联字面清单 ⇒ `MEASURED_CALLS` 421 → **426**、`MEASURED_READABLE`
  303 → **308**、`MEASURED_BLIND` 仍 **118**，l98 侧四档和 `58+11+38+11=118` 一字未动。**中间读数不
  美化**：第一次全量因此是 **2 failed** / 8800 passed / 11 skipped（两支都红在 `assert 426 == 421`），
  回填后复跑才到终态。定点复跑 l98 那五支时会拿到 5 个 `RulerNotProven` **error**——那是 `collected`
  夹具的下限设计（单独跑采不到那么多用例，「没有风险对象」在这一跑里不成立），不是本轮引入的缺陷。

- **前端三门禁（本轮 `Quality.tsx` / `DataManagement.tsx` / 两份测试有改动）**：`npm run typecheck` 绿、
  `npm run lint`（`--max-warnings 0`）绿、`npm run test` **22 文件 / 299 支**全绿（与 L237 同数——本轮
  只改既有断言与页面文案，没有新增测试文件）。vitest 首跑 **1 红**：`Quality.healthgate.test.tsx` 还钉着
  旧口径「0.99 与 1.01 都 400」，按 A320 的新读数重钉为「0.99 / 1.005 / 1.01 都放行、0.98 与 1.02 才拒」
  并加两格（`钳进 [0, 1]`、`超过 1`）。`min={0} max={1}` 补在权重输入框上是把「逐项 [0,1]」这件事在
  控件层也说一遍——它不是校验的权威，权威在后端那一份。

- **改后端那天，服务层那两句注释必须一起动**（L236 的 `TestAThreeTwoTwo` 静态臂注释里提前写着这句）：
  代码批 `88837aa2a` 落地后 `web/src/services/api.ts` 里「`text_field` 的 `|| undefined` 是躲判决的」与
  「`text_key` 那一格没有后端判决可暴露」两句立刻变成**假话**，本轮单独补一笔 `049dea678`：两处同行改写、
  净零行（1392 行一字不动），`|| undefined` 本身留着（不填即走默认 `instruction`），但理由从「藏判决」
  改成「少跑一个来回」。补完复跑六把里的 a184 与 l224 / l235 / l236 / l237 / l238 定向面 ⇒ 149 passed。

- **顺带量到一条、按纪律不夹带，立账 A324**：分割比例那一族还是手抄容差 + 不判长度——`0.4 / 0.3 / 0.29`
  这一份完全合法的 40/30/29 切分实测被 `dataset_ops.split` 判「之和必须为 1.0，当前为 0.99」（就是本轮
  A320 刚收掉那个假拒的**另一处**），而 SDK 直构喂 4 个比例时过校验、随后死在
  `ValueError: too many values to unpack (expected 3)`；容差那一半 API / CLI / SDK 三面可达，长度那一半
  只在直构面可达。

- **本轮全量读数**（同一条命令链，取于代码批 `88837aa2a` 落地之后、`049dea678` 那两处注释改写之前——
  注释不进这份数，账本批之后的复跑才是全链终态）：⇒ **8802 passed / 0 failed / 11 skipped，
  覆盖率 99.83%**，`TOTAL 14223 语句 / 8 未覆盖 / 3922 分支 / 22 未覆盖分支`，`Required test coverage of
  80% reached`，用时 210.67 s；六把账本与文档守卫 ⇒ 149 passed / 8 skipped；前端三门禁如上。**门禁运行
  期间没有编辑任何被守卫读取的文件**（账本批的写入全部在两次全量之外）。**本轮没有跑带 `--ignore` 的任何
  读数。** 覆盖产物 `coverage.xml` 每轮重写、不参与对账。
  **账本批（`9eb75f6a8`）之后同一条命令复跑** ⇒ **8802 passed / 0 failed / 11 skipped / 覆盖率 99.83%**，
  与账本批之前的四格逐格相同、**无偏差**（`skipped` 的三条老理由一字未动：8 支作者工作树档 +
  `test_config_gates_l82:573` + `test_upload_ceiling_l87:580`，另加本轮新那一格
  `test_parametrize_blind_face_l98.py:641` 的解释器名册档——它是换代的结果，不是回归）；
  `TOTAL` 行 14223 语句 / 8 未覆盖 / 3922 分支 / 22 未覆盖分支、`Required test coverage of 80% reached`；
  用时 210.67 s → 268.10 s 是同一台机器上的负载差，不是读数差。覆盖产物 `coverage.xml` 每轮重写、
  不参与对账。**这份复跑读数就是 L238 的终态基准，也是下一轮（L239）要比对的那一格。**

- **待拍口径**：**A319 / A320 / A321 / A322 四条已收口（L238）**，各留了一半没做的面写进对应 Backlog 行
  （A319 的 `score_is_unclamped`、A321 的 `text_field_found` / `errored_rules` 恒空、A322 的 `language`
  无条件盖戳与「列在而值全空」）；本轮新立 **A324**（分割比例同族，建议 L239 与 **A323** 一并处置，两条都
  小、都在「同源措辞」这一族上）、**A323**（toast 不读后端 `message`）待拍；**A316 / A317 / A318**、
  **A298~A301 / A303 / A304**、**A306~A311**、**A282**（format 基线红且 CI 无门）、**A280**、**A278**、
  **A276**、**A271** 的 18 只待处置键、**A272**、**A273** 仍待拍。另记一条**环境类待办**（不占 Backlog
  编号）：`test_parametrize_blind_face_l98` 的解释器名册若要在 3.12 上不再跳，得在该版上重量
  `dynamic_risky(...)` 那份名册再钉，换机器不自动恢复环境这条纪律本轮第二次实证。

### L239（2026-10-12）— A286 收口：把「这台机器此刻给不给 CPU」换成「算法跑了几次求和」，顺带 3.12 名册现量钉版与性能热点入账

**本轮定义（动手前拍的）**：零对外行为变化的一轮。三件事——① 3.12 解释器名册现量钉版（消掉一格 skip）；
② A286 收口（把比值型计时守卫换成确定性计数判据）；③ 性能探针读数入账并立性能欠账。A286 **不在**待拍名单内，
属可自行动手的测试面缺陷；A324 / A323 / A316~A318 / A298~A301 / A303 / A304 / A306~A311 / A282 / A280 / A278 /
A276 / A271 / A272 / A273 一律不动（要改对外状态码或响应值域）。

- **① 3.12 名册：先量再写，且把「读数相同」这句推断限定到它成立的那两档**
  uv 托管 **cpython 3.12.14**（本仓唯一可用解释器，venv 在仓库外 `D:/WORKSPACE/venvs/augmentor312`）整跑采集
  **8813 条**，`dynamic_risky` 那份名册现量出**恰好一条**：
  `unit/test_schema_type_spec_l96.py::test_label<-value:typing._UnionGenericAlias:Union`。
  它与 `(3, 13)` 那格**逐字相同**，因为 `Union` 在 3.12/3.13 上都是 `typing._UnionGenericAlias`、
  那个 id 段靠 `__name__` 现造；**劈叉发生在 3.14**（`typing.Union:Union`）。
  ⇒ 文件里原来那句「同一份源码在两版解释器上读数一模一样，就是有把尺子失效了」**是错的推断**——
  它只在 **(3,13) vs (3,14)** 之间成立。据此只做两件事：docstring 从「两版」改成「各版逐版钉死、
  未知版本显式跳过（不许猜）」，并把那条比较**显式限死在 3.13/3.14**，没有改成「表内两两不同」
  （那会把 3.12 与 3.13 的真实相同判成缺陷）。3.12 上那格 skip 消掉 ⇒ 全量 skipped **11 → 10**。
  L238 日志块末尾那条「环境类待办（不占 Backlog 编号）」到此结案。

- **② A286：判据换体，且先证「新判据能红」再删旧判据**
  L227 立案时给的原案是「比值型计时守卫在共享工作机上会假红」，三条处方里 **② 用计数代替墙钟** 才是判据、
  ① 只是让它稳。本轮按 ② 落地：
  · 现量（`Temp/l239_count.py`，同一函数同一口径）：`_calculate_diversity_score()` 整趟 `builtins.sum`
    调用数 n=1000 / 2000 / 10000 / 20000 **四档同为 2**（一次求均值、一次求方差和）；n=0 走空档早退 ⇒ 0。
    ⇒ 常数 `MEASURED_SUM_CALLS = 2`，两档规模（10000 与 20000，后者是前者的**前缀扩展**、同一颗种子）各取一次计数。
  · 尺子自证：计数窗口内跑出的**分数**必须与窗口外逐位相同（`assert counted_score == analyzer._calculate_diversity_score()`）
    ⇒ 替身只多记一次调用，没有改变被测量。
  · 三臂字节级注入取证（`Temp/l239_inject.py`，产品函数里的均值真写回生成器，`analytics.py` 原始字节先进内存、
    跑完逐字节还原并断言与进入时一致）：**三臂全部点名喊红**、进出 sha `13e78b589e8d` 相同。
    关键一条：变异成 O(n²) 之后**返回值一字不变**，只有计数抓得到 ⇒ 新判据测的是旧判据测不到的那一维。
  · 牙齿（复刻退化档，`_quadratic_diversity`）：现量调用数 **n+1**（5000⇒5001、10000⇒10001）。
    这里踩过一格口径坑：第一版复刻把内层写成 `_REAL_SUM(lengths)` 绕过了替身，于是打出 1 次而不是 n+1，
    换成平凡 `sum(...)` 才是真读数 ⇒ **注释里的数字先量出来再写**，这条纪律本轮第三次救场。
  · **撤下不是放宽**：`RATIO_CEILING = 3.0`、`MIN_MEANINGFUL_MS = 10.0` 与那条把阈值冻成断言的
    `test_ratio_ceiling_separates_linear_from_quadratic` 一并退役。它们量的是「这台机器此刻给不给 CPU」
    （L227 全量实测比值 **4.38** > 退化档最小读数 3.78，红那次整轮 257.50 s / 绿那次 188.88 s，高 36%）；
    现在量的是「算法跑了几次求和」，与负载无关。旧表格与旧常数在 docstring 里就地标注退役、
    读数原样留着作历史，没有抹平。
  · 保留两条各测一轴：`ABSOLUTE_BACKSTOP_MS = 2000.0`（现实现约 70 ms，20 倍松，不参与机器抖动，
    只在数量级出事时红）与 `test_the_timer_can_see_scale`（2000→8000 必须 >1.5 倍 ⇒ 计时器仍有分辨力，
    这是本文件「耗时不随规模变的读数一律作废」那根把手）。
  · 泄漏计数替身的复原纪律写进文件 docstring：`builtins.sum` 是**全进程可见**的替换，窗口内抛异常也必须复原，
    由 `test_the_real_builtin_is_restored_after_counting` 专门咬住（`finally` 里还带一条 `is` 断言）。

- **③ 性能热点现量入账（20000 条合成 / fallback 编码器 / best-of）**
  | 入口 | 读数 | 最热内部函数（cumtime） |
  | --- | --- | --- |
  | `LeakageDetector.detect`（10k×10k） | **45 411.2 ms** | `_collections._count_elements` **41.047 s tottime / 9988 次调用**（`leakage.py:233` 摊平倒排） ⇒ **A325** |
  | `Deduplicator.deduplicate`（20k） | 1 065.1 ms | `_inverted_view` 1.242 s、`_ngram_entries` 1.176 s、`_fallback_encode` 0.775 s ⇒ **A326** |
  | `DatasetAnalyzer.analyze`（20k） | 721.1 ms | `_calculate_text_statistics` 0.925 s（3 字段各一趟） |
  | `DatasetIndexer._ensure_default_indexes` | 337.7 ms | `_ensure_ngram_index` 0.923 s（profile 口径，wall 337.7 ms 取 best） |
  | `DatasetStatisticsCalculator.calculate` | 271.4 ms | — |
  | `QualityScorer.batch_score`（n=4000，FakeModel 顶上） | 156.9 ms | `encode` 仅 **3 次 / 8030 行**（无重复编码，L231 那层生效）；`_calculate_diversity` 行范数重算 **28.5 ms** ⇒ **A327** |
  | `ActiveSampler.analyze_coverage` | 113.7 ms | — |
  | `DatasetHealthScore.score` | 44.4 ms | `calculate_coverage` 0.028 s |

  **量化之后不动手的那一条，写清楚为什么不动**：语义相似度逐元素 `np.dot` 循环换 `np.einsum("ij,ij->i")`
  实测快 **3.73×**（14.26 → 3.82 ms），但**产物不位相同**（float32 最大差 2.98e-07、float64 8.88e-16）
  ⇒ 按「性能改动必须产物逐字节相同」的口径否决。这不是保守，是这条口径把「数值可复现」放在常数倍提速之前：
  一旦向量点积的实现换了归约顺序，同一份数据集在两个版本上会给出**不同的**质量分与不同的去重分组，
  而这两样都进产物文件。

  **另两条同样量化后不动手**：A326 的收益只在稠密档（真语料 6902 条判倒排赢 969 ms vs 10484 ms，
  生产主路径不经过它），而 `_batch_encode` 是测试注入向量的接缝、`dedup.py` 挂 6 处历史行号；
  A327 端到端只值 1–2%（真模型下才是可观常数），而 `quality.py` 挂**十余处**历史行号引用。
  ⇒ 三格全部立进 Backlog 而不是顺手做掉，正是「插行会平移历史行号引用」那条纪律在性能轮的具体形态。

- **中间读数（原样不美化）**：定点复跑 `test_analytics_performance.py` **7 passed**；六把账本/文档守卫
  **149 passed / 8 skipped**（与 L238 终态逐格相同）；`git status --porcelain` 在代码 commit 前只有那两个
  测试文件为 M。全量定标门禁（`-p no:cacheprovider`、默认 `--cov=augmentor --cov=api --cov=cli`，逐字照抄
  定标命令）**8806 passed / 0 failed / 10 skipped、覆盖率 99.83%**，与动手前拍的预测逐格相同
  （= L238 的 8802 + 本轮净增 3 支，11 → 10 skipped 正是 3.12 那格 skip 消掉；10 只 skip 的构成按 SKIPPED
  汇总逐只点名为 `test_doc_line_refs_l79` 8 + `test_config_gates_l82` 1 + `test_upload_ceiling_l87` 1，
  **名册那一格已不在名单里**）。
  **A286 换判据在同一次运行里就拿到了正面证据**：整轮 **294.53 s**，比 L238 那次的 188.88 s 高 **56%**
  ——正是 L227 假红那类负载档位（当时 257.50 s），而 `test_analytics_performance` 本轮**照常绿**。
  旧判据在这种整轮慢一半的档位上会因为比值越过 3.0 而假红；新判据数的是求和次数，与墙钟无关。
  覆盖率总量与 L238 逐格相同（TOTAL 14223 语句 / 8 未覆盖 / 3922 分支 / 22 未覆盖分支）——本轮一行产品代码未动，
  这正是「零对外行为变化」的机械版本。
  **账本批（`e3cef17cc`）之后同一条命令复跑** ⇒ **8806 passed / 0 failed / 10 skipped / 覆盖率 99.83%**，
  与账本批之前的四格逐格相同、**无偏差**；`TOTAL` 行 14223 语句 / 8 未覆盖 / 3922 分支 / 22 未覆盖分支、
  `Required test coverage of 80% reached`；用时 **294.53 s → 202.10 s**（相差 46 %）是同一台机器上的
  **负载差不是读数差** —— 而这一维恰好是本轮 A286 换掉的那把尺子所量不到的：两次全量里
  `test_analytics_performance` 七支**都绿**，而 L227 的假红正是在「整轮比基线高 36 %」那一档发生的
  ⇒ 判据换成计数之后，同一种负载不再能把它拨红，也不必靠放宽任何阈值。
  10 只 skip 逐格点名：`test_doc_line_refs_l79.py:647` 8 支（作者工作树档，未入仓取证草稿与
  `data/` 底料）、`test_config_gates_l82.py:573`（export 节没有开关）、`test_upload_ceiling_l87.py:580`
  （无真实数据集即无定标对象）；**`test_parametrize_blind_face_l98` 那一格已不在名单里**，
  即 3.12 名册钉版在 3.12.14 上生效（11 → 10 那一格就是它）。覆盖产物 `coverage.xml` 每轮重写、
  不参与对账。**这份复跑读数就是 L239 的终态基准，也是下一轮（L240）要比对的那一格。**
### L240（2026-10-12）— A325 收口：给泄漏近似计数加一条「按代价选路 + numpy 0/1 矩阵乘法」的稠密档，判决口径一字未改

**本轮定义（动手前拍的）**：只做 A325 那一格（L239 量出的全项目最大热点，`LeakageDetector.detect` 10k×10k
**45.4 s**）。两条硬约束先拍死：① 新路线的**逐条 kind 序列**（exact / fuzzy / None 三类，含 `leaked_examples`
的顺序）必须与现状相同；② 新守卫一律用**计数**（调用数、摊平条目数、字节账）当判据，不用墙钟比值——
A286 上一轮刚把那一族尺子退役。待拍名单（A324 / A323 / A316~A318 / A298~A301 / A303 / A304 / A306~A311 /
A282 / A280 / A278 / A276 / A271 / A272 / A273）与 A326 / A327 一律不动。

- **① 选路判据：两条独立账，全纯整数**（结构照 `dedup.py` 的 `_inverted_blocks_win`，不借它的常数）。
  时间账 `flatten_adds × _DENSE_ADD_COST(=1000) >= block_rows × n_train × vocab`；内存账
  `n_train × vocab × 4 <= _DENSE_MAX_BYTES(=64 MiB)`；块大小 `_dense_block_rows(n) = max(1, min(1024,
  1_000_000 // n))`。任一轴 ≤ 0 直接不换路——稠密在 0 列视图上选不出 argmax，宁可慢也不炸（守卫里
  五档零/负参数化就是钉这句的）。单价 1000 的两侧余量按 `Temp/l240_leakprobe8.py` 六档现量钉：落在
  「判赢最差档比值 71.6」的 14 倍之上、「判负档 84,905」的 85 倍之下，两侧都留整数量级，不是拟合出来的中点。
- **② 「逐条同判」不是近似，可证的链条四步**：`ov` 是 0/1 float32 矩阵乘法的整数值（值 ≤ min(|a|,|b|) < 2^24
  ⇒ float32 精确表示）；判据 `best >= t` 等价于「∃ 候选使 J ≥ t」，而 t > 0 由构造器 (0,1] 保证 ⇒ 剪枝剪不剪
  都不改判决（这正是倒排那条 `need >= int(t·|a|)` 的可采纳性反向用法）；argmax 在 **float64** 下选，两个整数
  在 float64 的商与 Python `int / int` 逐位相同（同为 IEEE 754 正确舍入）；最终 J 由产品权威算式
  `_jaccard_from_counts` 现算 ⇒ 稠密那条**不许**引入第二套相似度算式（这条有专门的计数牙齿，注入臂 A4 就红在它）。
  `union` 恒 ≥ max(|a|,|b|) ≥ 1（签名非空才有三元组进来）⇒ 不写除零分支。
- **③ 一度假设「float32 除法就够」，被自己的探针否证之后才找到真因**：v5 直接量到三条下标在
  float64/float32 与阈值同向不一致 ⇒ 精度假设作废；顺着追下去真因是 **probe3 的下标错位**（exact 即时
  append、非精确项攒在缓冲里 flush 才 append，两条 kind 序列整体错位），不是精度。修法是产品与探针双侧都
  改成 `positions` 索引回填，v6（四套 best 并列）+ v7（邻域打印）复核 ⇒ 六档全 True。**教训**：把「读数不对」
  当成「精度不够」会写出一整套没必要的 machinery；先证伪自己的假设，再决定要不要加那一手 float64。
- **④ 产品面逐字节对账（`Temp/l240_leakprobe9.py` 终态复跑 → `l240_probe9_final.log`）**：六档形状的报告
  `to_dict()`（JSON 序列化后）与强制倒排**逐字节相同 = True**，`exact` / `fuzzy` 计数逐档相等。

| 形状 | 自动路线 | 强制倒排 | 倍数 | 走稠密的块 |
|------|----------|----------|------|------------|
| templated 24 词 10k×10k | **1 742.1 ms** | 48 929.0 ms | **28.1×** | 99/99 |
| templated 24 词 4k×4k | 384.8 ms | 6 845.4 ms | 17.8× | 16/16 |
| templated60 48 词 4k×4k | 576.7 ms | 18 452.2 ms | 32.0× | 16/16 |
| templated 24 词 1k×1k | 67.1 ms | 543.8 ms | 8.1× | 1/1 |
| random5 铺开 4k×4k | 722.3 ms | 666.9 ms | **0.9×** | 0/16 |
| zhshort 塌缩 1.5k×1.5k | 10.0 ms | 9.8 ms | 1.0× | 0（无非精确条目，不进判据） |

  **铺开档判负、而且判负之后还慢 8%** —— 这一笔如实入账不做美化：块化簿记与摊平记账是两条路**共用**的代价，
  倒排赢的时候仍然要付它。同轮第一次 probe9 读数（1 925.3 / 49 585.6 ms ⇒ 25.8×）与终态 28.1× 之间的差是
  同一台机器的负载差，两次「报告逐字节相同」都是 True；收益写进账本的是**同一次运行内**的 auto vs 强制倒排，
  不是跨运行的比较。
- **⑤ A325 原案处方 ①（按长度带剪枝）量化之后否决**：`Temp/l240_leakprobe2.py` 现量带内条目只省一半——
  4000×4000 档平均 posting 摊平 **37,328** → 带内 **17,298** 条目/条测试 ⇒ **省 53.7%**（500×500 / 1k×1k /
  2k×2k 三档分别 53.9 / 53.3 / 53.5%）。常数减半、量级不变，而它要把 `sig_gram_counts` 有序化 + 两次
  `searchsorted` 重写索引形状；同一形状上稠密给的是 **17~32×** ⇒ 按原案处方 ③ 落地。
- **⑥ 内存账先算出来，不等 `tracemalloc` 撞上**：**`tracemalloc` 看得见 numpy 的分配**（1200×300 档强制稠密
  峰值 **16.4 MiB** vs 倒排 **1.23 MiB**），所以稠密侧的峰值不能靠「跑一下量量看」兜底，必须可算。两道账一起
  封顶 ⇒ 视图 ≤ 64 MiB、单块 ≤ 1e6 格（约 24 B/格）⇒ 10k×10k 档的稠密峰值上界由守卫**算**出来（< 95 MiB），
  同时保留 `tracemalloc` 双侧护栏（稠密那一档更贵且 < 24 MiB、倒排 < 2.5 MiB）。
- **⑦ 两支既有 l95 尺子被新判据路由成空转，本轮按「点名被测路线」处置**：
  `test_peak_mib_under_ceiling`（2.5 MiB 上界）与 `test_prune_skips_something_it_should`（`calls < n_pairs`）
  跑在同一份塌缩语料上，现在都被自然判给稠密 ⇒ 前者实测 16.4 MiB **当场红**，后者更阴：注入臂 A1 首跑**整簇全绿**
  （把 `need` 剪枝整段删掉，权威算式调用数从 1 变 1，尺子完全看不见那一维）。两处都改成 `monkeypatch` 强制倒排，
  **上界、阈值、断言一字未动**，并在 docstring 写明「本支钉的是倒排那一条」；稠密那条的牙齿另立新守卫文件。
  这是「加了第二条路线之后，钉第一条路线的尺子必须点名是哪一条」在本仓的第二次复现（第一次是 L239 给
  `test_the_real_builtin_is_restored_after_counting` 补复原纪律）。
- **⑧ 新守卫 `tests/unit/test_leakage_dense_route_l240.py`（29 支 / 5 面，判据全为计数）**：
  判据纯整数（实测赢档逐档放行 + random5 两条账各自独立判负 + 时间账闭区间两侧 + 内存账 `vocab_at_cap` 两侧 +
  五档零/负 + 块大小整数反推含 9791⇒102）；两路线报告 `to_dict()` 逐字节相同（塌缩 / 铺开 / 48 词三份语料，
  加 `(1.0, 3)` / `(0.3, 0)` / `(0.999, 5)` 三档边角，加空签名档断言 `spy.dense_blocks == 0`）；暴力集合交并
  预言机双向对账且 `sum(flags) > 0`（防「两个 0 相等」）；计数牙齿（稠密每行恰一次权威算式、比较次数不随训练
  规模涨、`_char_trigrams` 每签名一次双路参数化、块大小压到 1 的对齐钉子、`_dense_view` 行列与 posting 同源、
  形状→路线点名）；视图只在判赢时建（倒排档 0 次、稠密档恰好 1 次）；内存账既算又量。
  **语料两度返工，都是同一族空转绿**：`templated` 那族 70×70 造不出一条 fuzzy（4000×4000 也才 26 条）⇒ 改
  `near_dup`（三条长中文头部 × 三条尾部 × 五只语气词）。规模钉子又踩两次：第一版追加语料用无关句子 ⇒ 过不了
  `need` 剪枝，读数 **71 对 71** 一格未动；第二版改 `near_dup(450)` ⇒ 该族只有 3×3×5=**45** 只签名，450 条把
  空间填满，测试侧 22 条非精确全转 exact ⇒ 两侧调用数一起变 **0**，又是一格空转绿。终版 `padded_dup`（每条测试
  样本 ×15 只短后缀，`b` 以 `a` 为前缀 ⇒ I=|a| 稳过剪枝而 J=|a|/|b| < 1）⇒ 现量倒排 **71 → 1 466（20.6×）**
  而稠密侧两档都钉在 **22**（= 行数，与 n_train 无关的恒等式）。
- **⑨ 八臂字节级注入取证（`Temp/l240_inject_probe.py`，跑完重新验磁盘）**：`leakage.py` 是**纯 LF**（478 行、
  0 个 CRLF），每臂锚点先断言在原文唯一命中，`finally` 复原后 sha256 与进入时 `df54edca9a61…` 相同（脚本第一行
  就打这条校验，不复原不打印结果表）。

| 臂 | 改写 | 结果 |
|----|------|------|
| A1 | 删 `need` 剪枝（候选全比） | RED 1 failed（l95 剪枝支——强制倒排点名之后才咬得住） |
| A2 | `_dense_route_wins` 恒 False | RED 4 failed |
| A3 | `_dense_route_wins` 恒 True | RED 2 failed |
| A4 | 稠密旁路 `_jaccard_from_counts`（numpy 里直接判） | RED 3 failed |
| A5 | 回填改成按 `flags` 序即时写（错位） | RED 2 failed |
| A6 | argmax 换 float32 | **GREEN** |
| A7 | 块大小不看列数（恒 `_DENSE_MAX_ROWS`） | RED 2 failed |
| A8 | 删内存账 | RED 1 failed |

  **A6 不红是结论，不是漏网**：float64 那一手只在「并列 / 近并列」时决定挑中哪个训练下标，而 kind 只看
  `max J ≥ t`；要在 float32 下翻判决需要 `|J − t|` 落进 float32 eps（≈6e-8）以内 ⇒ 三元组数量级要到 ~1.7e7，
  量程不可达（probe5 / probe6 / probe7 现量）。⇒ 本档如实留作「非恒真校验的阴性臂」，并把原计划的
  「临界档 float32-vs-float64 牙齿」**替换**成真正承重的把手：稠密不许旁路权威算式（A4 那条计数钉子）。
  顺带一句口径：A2 / A3 是**同一句判据**的两条反向退化，各自红的支数不同（4 vs 2）⇒ 说明守卫不是「只要判据动过
  就红」的一锅粥，而是时间账与内存账分别有人看。
- **⑩ 模块 docstring 的设计原则本轮改写两句**：删「纯 Python 实现，无外部依赖」——稠密计数用 numpy，
  而 numpy 是仓内既有硬依赖（`analytics.py` / `dedup.py` 已在用），本轮**没有**新增依赖；删「实测规模翻倍比
  ×2.3~×2.8（改写前 ×4.0…）」那句墙钟比值——那是 A286 之后不该再出现在设计原则里的判据形式，改成
  「规模翻倍时增长是线性的，不再等于全量两两比对」+「posting 塌缩档按 `_dense_route_wins` 换路，逐条判决
  与倒排完全相同」。
- **中间读数（原样不美化）**：定点复跑四文件全簇 **109 passed**（其中新守卫 29 支、l95 36 支）；⑧ 里那两次
  空转绿读数 (22, 22, 71) 对 (22, 22, 71) 与 (0, 0) 对 (0, 0) 原样留着；六把账本/文档守卫 **149 passed /
  8 skipped**（与 L239 终态逐格相同）；普查两把按现量重钉 **CALLS 426 → 429、READABLE 308 → 311、盲面 118
  一格未动**（新三支 parametrize 值位全是内联字面清单；桶归属由仓外探针 `import` 本模块的
  `param_sites(TESTS_ROOT)` 现量逐处打印，303 / 346 / 455 三行全 `plain`，不按「字面量 ⇒ 可读」推）。
  全量定标门禁（逐字 `python -m pytest tests -q -p no:cacheprovider`）**8 835 passed / 0 failed /
  10 skipped、总覆盖 99.84%**，与 L239 终态基准 8 806 恰好差 **+29 = 本轮新立守卫的支数**，l95 与 l97 都只改
  判据归属没有增减支数；10 只 skip 名册逐格相同（`test_doc_line_refs_l79` 8 + `test_config_gates_l82` 1 +
  `test_upload_ceiling_l87` 1）。整轮 **332.81 s** vs 基准 202.10 s（+65 %）：本轮新增守卫在覆盖仪表下的
  **边际成本 ≈1 s**（同一固定开销下 l95 单跑 9.26 s / 36 支 vs 新文件单跑 10.25 s / 29 支），其余那一截正是
  L239 已点名的负载那一维——而那一维恰好是 A286 换掉的尺子量不到的，本轮没有任何一条判据看墙钟。
- **历史行号引用重钉（处方 3，附归因）**：`leakage.py` 本轮净 **+196 行**（+222 / −26，`git diff --numstat`
  复核过），把卷一 L55 日志块那句「`LeakageDetector.fields` 缺省只有 `["instruction"]`（`leakage.py:124`）」
  平移出跨度。先按处方 1（换点：同行追加）与处方 2（压回净零）试过——六档实测表、四步判决链条、两条账的
  定义都要落在模块内，收不回净零 ⇒ 走处方 3 重钉为 `leakage.py:352`（同行**等长**替换，卷一字节数与行数
  一字未动），复跑 `test_doc_line_refs_l79` **33 passed / 3 skipped** 无偏差。卷二 L239 性能表与本条 A325
  里的 `leakage.py:233` / `182/186/233/236` 是**时间戳式读数出处**，按 A127 口径保留、不批量改写；
  `test_comment_refs_a184`（注释里的行号引用）硬 0 档本轮未红 ⇒ 产品码里没有指向 leakage 的活引用，
  A325 原框那句「四处行号还被现存注释点名」实测**不成立**，已在本条收口句里订正。
- **前端三门禁：不适用**（`web/` 零改动；本轮 `git status` 里只有 `augmentor/` 下四个文件，没有漏跑）。
- **对外行为：零变化** —— `LeakageReport.to_dict()` 的字段集、`exact_leaks` / `fuzzy_leaks` /
  `leaked_examples` 的取值与顺序在六档形状上逐字节相同；新增的只有内部选路判据与四个 `_DENSE_*` 模块级常数，
  无新配置键、无新端点、无 CLI 面变化，因此契约数字守卫（`test_doc_contract_numbers_l200`）一格未动。
- **账本批（`633574b31`）之后同一条定标命令复跑** ⇒ **8 835 passed / 0 failed / 10 skipped、
  总覆盖 99.84%**，与账本批之前的那一次**四格逐格相同**（passed / failed / skipped / 99.84 %），
  `TOTAL` 行也逐格相同：**14 291 语句 / 8 未覆盖 / 3 952 分支 / 22 未覆盖分支**、
  `112 files skipped due to complete coverage`（账本批一行产品码未动，这正是「零对外行为变化」的机械版本）。
  用时 **332.81 s → 272.09 s（−18 %）** 仍是同一台机器上的**负载差不是读数差**——本轮 29 支新守卫的判据
  全是计数（调用数 / 摊平条目数 / 字节账），整轮慢一两成拨不动任何一条；这与 L239 那两次
  （294.53 s → 202.10 s，46 %）同族，是 A286 换判据之后第二次拿到正面证据。
  10 只 skip 名册逐格相同（`test_doc_line_refs_l79.py:647` 8 支作者工作树档、
  `test_config_gates_l82.py:573`、`test_upload_ceiling_l87.py:580`）。
  **这份复跑读数就是 L240 的终态基准，也是下一轮（L241）要比对的那一格。**
- **记账批自己抓到的两处，原样入册不覆盖前文**：
  ① 上面批写脚本里那句复验断言（`pairs == newlines`）的量的是「**每一个 `
` 都前接一个 ``**」
  ⇒ 它足以证明「无孤立 LF」，配上 `crlf = b"
" in raw` 为真与 `pairs > 0` 才是「纯 CRLF」。
  第一版把它写成「证明纯 CRLF」，**说的是判据的一半**：单看 `pairs == newlines` 在空文件上恒真。
  本轮两条都实际打了（`lone LF 0`、`pairs 3465`），所以写盘结果没问题，措辞该收准则收准。
  ② 纪律里「`git diff --numstat` 复核净行数」这一步本轮**没跑就进了记账批**（只跑了脚本内的行数自校）
  ⇒ 记账批补跑：`633574b31` 卷二 **+120 / −1**（净 +119）、卷一 **+1 / −1**（同行等长替换，字节数一字未动），
  与账本所述一致。**脚本自校不能替代独立仪器的复核**，这正是那条纪律单列一格的理由。

### L241（2026-10-12）— 把「按代价选路」那两条独立账写进文档，并给它装上机械守卫（§3.34 + 新守卫 24 支）

**本轮定义（动手前拍的）**：**零对外行为变化**——产品码一字不动，只做「文档说明 + 跨家族守卫」这一格，
把 L240 那条纪律（加了第二条路线之后，钉第一条路线的尺子必须点名它量的是哪一条）与 A286 那条纪律（性能判据
一律计数）从散文变成可失败用例。待拍名单（A324 / A323 / A316~A318 / A298~A301 / A303 / A304 / A306~A311 /
A284 / A285 / A287~A289 / A282 / A280 / A278 / A276 / A271 / A272 / A273）与 A326 / A327 一律不动。
本轮**零新立、零收敛**：Backlog 表一格未动，A325 那一行已在 L240 收成「已收口」。

- **① 文档 §3.34（新增一节）**：把两条独立账的**定义**（时间账 = 摊平次数 × 单价 vs 乘加格数；内存账 =
  `n_train × vocab × 4 <= 64 MiB`）、四个单价常数的现量、收益区间 **8.1×~32.0×** 与 10k×10k 那一格
  **28.1×**（1 742.1 ms vs 48 929.0 ms，同一次运行内 auto vs 强制倒排）、铺开档判负之后**仍慢 8%**
  （块化簿记与摊平记账是两条路共用的代价，倒排赢的时候照样要付它）、以及「一条从注入里长出来的纪律」
  那一段（16.4 MiB 峰值臂当场抓红 vs 去剪枝臂整簇全绿的阴性证据）全部写正面。另在 §7.1 补第四颗子弹：
  **性能判据一律计数**——L239 那轮负载差 46%、L240 那轮 −18%，两轮都拨不动任何一条判据，这是 A286 退役
  比值尺子之后连续两次拿到的正面证据；本轮同形（守卫 24 支零 `time` 引用）。
- **② 新守卫 24 支钉五件事**：文档点名与 AST 实推**双向精确相等**；三条独立通道互相咬合；判据本体
  **零浮点、零调用、零属性访问**（放行属性访问等于让它写 `time.monotonic()`，那条纪律当场退化成一句空话）；
  每支预言机在自己的模块被调用、至少一处调用点直接坐在 `if` 的 test 位、承载它的函数同时调用两侧生产者
  （`_batch_encode` 与 `_iter_inverted_similarity_row_blocks`、`_inverted_flags` 与 `_dense_flags`）——
  **判负的一侧不许变装饰品**；每支预言机都要在 tests 树里被写进某处 `setattr` / `patch` 的目标位。
- **③ 为什么方向是「相等」而不是棘轮**：L76 / L77 那套「只降不升」的棘轮恰好把「新增一条路线却不写文档」
  这一半方向放过，而这一半正是本家族的失效形状（一支预言机住进产品码而文档从不提它，全量门禁不会红）。
  代价是双向都变敏感：改名字、加路线、写文档三件事必须同轮发生，这正是想要的摩擦。
- **④ 第三条通道是「先量误伤，再采用」**：`-> bool` 标注 + 比较式有一侧含乘法 + 同模块被 `if` 的 test 位消费。
  加之前先在全包跑一遍命中名册，实测**只命中那两支预言机、零误伤**才写进判据；同判据里配了一支反向臂
  （`test_shape_channel_does_not_fire_on_a_plain_predicate`）证明它不是一把会自己变红的宽尺子。
  残余失明面如实声明：一支三条通道都不占的新预言机对守卫仍然失明，形状与 A129 同类。
- **⑤ 六臂真文件注入取证（全部按预期红，无阴性臂）**：A1 文档写幽灵名 ⇒ `test_doc_roster_equals_ast_roster`；
  A2 `leakage.py` 新加一支 `_probe_route_wins` 而漏文档与用例 ⇒ 连带 6 支红（名单、生产者表、焊死接缝、
  本模块调用、`if`-test 位、三通道咬合）；A3 判据本体长出 `4.0` ⇒ 纯度臂按 id `[leakage]` 点名；
  A4 把调用点从 `if` 的 test 位挪成先存变量 ⇒ `if`-test 臂 + 三通道咬合同红（通道三真的在承重）；
  A5 删掉判负那一侧的生产者调用 ⇒ `test_both_route_producers_live_in_the_calling_function`；
  A6 拆掉 `monkeypatch.setattr` 的焊死字符串 ⇒ `test_every_auto_route_has_a_welded_case`。
  每臂写完立刻读回比哈希，`finally` 逐字节复原并复验；跑完重新验磁盘四份文件（`docs/ARCHITECTURE.md`、
  `augmentor/leakage.py`、两支被借用接缝的既有用例）全部 OK，复原后基线 24 支全绿 ⇒ **无残留**。
- **⑥ 普查按现量重钉**：l97 `MEASURED_CALLS` 429 → **430**、`MEASURED_READABLE` 311 → **312**（本轮一支
  自带字面 ids 的 parametrize，两处同涨）、盲面 **118** 一格未动；l98 `MEASURED_EXEMPT_SITES` 23 → **24**，
  归因是新文件里 `test_no_float_no_call_no_attribute` 整支进豁免名册。豁免是「动态面放过」的名单，
  **涨它不等于放松**：值位那一维仍逐格判，且同轮四档 58/38/11/11、档位和 118、盲面名册 113 全部一字未动。
- **⑦ 中间读数原样入册（不美化）**：门禁 #1 = **1 failed / 8853 passed / 10 skipped**（360.55 s）。
  那一支红是 `test_parametrize_blind_face_l98` 的 `assert 24 == 23`，归因见上第⑥条 ⇒ 按现量重钉之后
  门禁 #2 = **8859 passed / 0 failed / 10 skipped**（403.73 s）。**没有任何一把尺子被改松**：本轮改的两格
  都是「现量涨的那一维」，判据本体一字未动。8835（L240 基准）+ 24（本轮新立）= 8859，逐格对得上。
- **⑧ 全量门禁 #2 覆盖率四格**：**99.84%**，`TOTAL` = **14 291 语句 / 8 未覆盖 / 3 952 分支 / 22 未覆盖分支**、
  `112 files skipped due to complete coverage` ⇒ 与 L240 终态基准**逐格相同**（产品码一字未改，这正是
  「零对外行为变化」的机械版本）。用时 272.09 s → 403.73 s（**+48 %**）仍是同一台机器上的负载那一维，
  本轮 24 支新守卫的判据全是结构与计数，没有一个数字看时钟。10 只 skip 名册逐格相同
  （`test_doc_line_refs_l79.py:647` 8 支作者工作树档 + `test_config_gates_l82.py:573` + `test_upload_ceiling_l87.py:580`）。
- **⑨ 行号引用与行尾**：`docs/ARCHITECTURE.md` 本轮净 **+63 / −1**（`git show --numstat` 复核过），两处插入
  都在卷一 A74 那条 `ARCHITECTURE.md:1232` 引用**之下** ⇒ 1232 那一格未平移，六把账本与文档守卫
  （markdown 结构 / l98 哈希槽 / l203 进度一致性 / a184 注释引用 / l79 文档行号 / l200 契约数字）
  定向读数 **173 passed / 8 skipped**；新文档句子只用名字锚点，未写任何 `x.py:NNN` 与 Temp 反引号路径，
  所以 a184 硬 0 档与 l79 三桶一格未动。卷二本条批为纯 CRLF 字节级写入（3465 → 新行数，逐臂复验
  `crlf_pairs == 行数`），写盘后立刻复跑六把，未混入 LF。
- **⑩ 前端三门禁：不适用**（`web/` 零改动，`git status` 里本轮四份文件全在 `augmentor/` 下）；
  **契约数字守卫：一格未动**（无新端点、无新 tag、无新 router、无新配置键）。
- **⑪ 本轮机制收获**：加一条新通道之前先量它的误伤面（A2 那条阴性臂在 L240 出现过、本轮 A6 是新形状），
  与「先跑探针再写结论」是同一条纪律的两个方向——前者管尺子别太宽，后者管尺子别太窄。

- **⑫ 记账批自己抓到的一处，原样入册**：上面写盘脚本的复验里我多写了一句 `b"\n\r" not in back`，
  它把「纯 CRLF」判成了假——纯 CRLF 文件里的**空行**本来就是 `\r\n\r\n`，天然含子串 `\n\r`。
  写盘结果没有任何问题（`pairs 3525 == lf 3525`、`git diff --numstat` 独立复核 **+60 / −0**），
  红的是那条子判据本身。这与 L240 第①条是同一个形状的再复现（判据只量到一半 / 加了一个不该加的必要条件），
  方向相反但病因相同：**复验断言要独立于「我以为它证明了什么」来写**。该子句已删，只留 `pairs == newlines`。

- **⑬ 账本批（`79fcb31f6`）之后同一条定标命令复跑** ⇒ **8 859 passed / 0 failed / 10 skipped、
  总覆盖 99.84 %**，与账本批之前的那一次**四格逐格相同**（passed / failed / skipped / 99.84 %），
  `TOTAL` 行也逐格相同：**14 291 语句 / 8 未覆盖 / 3 952 分支 / 22 未覆盖分支**、
  `112 files skipped due to complete coverage` ⇒ 账本批一行产品码未动，这正是「零对外行为变化」的机械版本。
  用时 **403.73 s → 404.15 s（+0.42 s）**：两次跑之间机器负载没有台阶，所以这一格拿到的是一个**干净的等号**
  而不是又一轮负载差——它同时反过来支持第⑧格那句「+48 % 是负载不是读数」：同一把尺子在低负载下复现出
  同一秒数，说明多出来的那一截不是守卫自身的成本。10 只 skip 名册逐格相同。
  **这份复跑读数就是 L241 的终态基准，也是下一轮（L242）要比对的那一格。**
  **如实补一句因果顺序**：本格这九行写在复跑**之后** ⇒ 它们不在这份读数里；写完之后单独复跑六把账本与文档
  守卫（markdown 结构 / l98 哈希槽 / l203 进度一致性 / a184 注释引用 / l79 文档行号 / l200 契约数字），
  **149 passed / 8 skipped** 与账本批之前那一次逐格相同，而本轮产品码与 tests 目录零改动 ⇒ 用例数不可能动。

### L242（2026-10-12）— 把账本的第三面接上电：进度声明面说「收了」，Backlog 表那一格必须有标记

**本轮定义（动手前拍的）**：**零对外行为变化**——产品码一字不动，本轮只做「量出双写漏的第三面 + 逐格补标 +
立一把精确相等的对账守卫」。待拍名单（A324 / A323 / A316~A318 / A298~A301 / A303 / A304 / A306~A311 /
A284 / A285 / A287~A289 / A282 / A280 / A278 / A276 / A271 / A272 / A273）与 A326 / A327 一律不动。
另把 L241 遗留在工作树的 `docs/ARCHITECTURE.md` 两句 §3.34 六臂取证随代码提交入册——那两行插在 §7.1
引用行**之下**，不平移任何历史行号引用。
本轮**新立 1 格（A328，定级 M 待拍）、补标 17 格、半程豁免 3 格**——三个数字在清单行与本块同形。

- **① 为什么是「第三面」**：L203 那把尺子（`test_ledger_progress_consistency_l203`）钉的是清单行 ⇄ 日志块
  两面互查，而账本还有第三面——两卷 Backlog 表里每个 `Axxx` / `Bxxx` 行的定级格。一轮收掉一笔账却只写前两遍，
  后来人顺表读到的是「这笔还没做」，而且**没有任何机械通道会出声**。本轮先把全集量出来：开工前账目面
  **247 行**、声明面点到且表里仍有行的 **116** 个 ID，其中带标记 **96** 个、无标记 **20** 个——
  那 20 笔就是欠账（`MEASURED_GAP_BEFORE` = 17 真漏标 + 3 合法半程）。补标之后：账目面 248 行、
  标记面 **115 → 132**、无标记 **20 → 3**（剩下的 3 笔即半程豁免名册）。
- **② 口径是一条一条现量选出来的，不是抄的**：
  ⑴ **ID 宽度三档对照**：只认三位 ⇒ 账目面 190 行、gap **11**；认到两位 ⇒ 234 行、gap **20**；再认到一位 ⇒
  247 行、gap 仍是 **20**。多出的 13 行（A1–A8、B1 / B2 / B5 / B7 / B8）不改变名册，但少认一位就是让这 13 行
  在对账里**永久隐身**，所以取最宽档。尾部加 `(?!\d)` 防贪心（否则 `A1234` 被切成 `A123` 凭空造 ID）。
  ⑵ **声明面只收两个形状**（清单行 `- [x] **L` 与日志标题 `### L`），日志块正文不收：宽口径实测多收 6 个 ID
  而**全都已标**⇒ 窄口径没漏真欠账；正文里满是「未收口」「判负」这类带否定与条件的叙述，收进来只造假欠账。
  这一条由 `test_wide_scope_would_give_the_same_gap` 现场复验，不是写在纸上自证。
  ⑶ **对账只在「表里还有行」的面上做**：声明面点到而表里已裁撤的 ID 现量 **46** 个（`MEASURED_PHANTOM`），
  记成现量而不是假装不存在。
  ⑷ **弱词与划掉的数字都不算标记**：现量反例两条——卷一 A298 行内写「按 id 全清」而那笔账至今待拍、A176 行内
  写着划掉的「6 处」同样待拍。把它们算成标记会让待拍的账永久隐身，所以 `TestWeakTokensAreNotMarks` 把两条都钉住。
  ⑸ **区间展开按自然位数，不补零**：补零会造出 `A08` 这种表里不存在的假 ID，污染 phantom 那一格。
- **③ 判据是「精确相等」而不是棘轮**（承 L241 同一条理由）：`declared ∩ rows − marked` 必须**恰好等于**
  `PARTIAL = {A45, A127, A156}`。写成「≤ 某数」的话，本轮补标的 17 格里任何一格回退成无标记，只要总数不超上限
  就照样绿——那是把尺子调成看不见缺陷。`PARTIAL` 不是白名单：三格各自有点名用例，各自要求「声明带限定词
  （前半 / 一半 / 只关 ①）+ 表行自己没有标记」两条同时成立。
- **④ 新守卫 24 支、零 parametrize**：刻意不参数化，免得动 l97（CALLS 430 / READABLE 312 / BLIND 118）与
  l98（档位 58/38/11/11、EXEMPT_SITES 24）的现量档位。反空转三格（声明面 116 / 标记面 132 / 双卷双贡献）、
  表行唯一性 + 行数 248、结构自检（行首 `| ID | ` 行尾 ` |`，列数交给 markdown 守卫不重复）、
  八臂合成注入（拆标记必进 gap、加标记必进 marked、删声明行 declared 缩小、**加声明句必变红**、
  斜杠/顿号/区间展开、括号断口、行首才算表行）——全部走内存合成，零写入磁盘。
- **⑤ 同轮重钉的尺子只有一把**：`test_doc_line_refs_l79.BACKLOG_A_CLOSED` **107 → 119**。归因：卷一 14 格补标
  划掉定级字母，其中 2 格行内本已含删除线 ⇒ +12；行首结案口径 **87 一格未动**（补标改的是定级格不是行首），
  两口径差值 20 → 32 全部来自这一族。**没有任何一把尺子被改松**：l79 的判据本体、markdown 结构、l98 哈希槽、
  l203 进度一致性、a184 注释引用、l200 契约数字逐格按各自 docstring 的处方重钉或原样通过。
- **⑥ A328（新立，定级 M 待拍）**：本守卫只钉「声明 ⇒ 表行有标记」这一个方向。反方向没钉，现量三条写在表行里。
  **本行自己就撞上过一次失明**：初稿把 7 种标记形状逐字列进格子，那一格立刻被标记判据认成「已关闭」，
  现量从 132 涨到 **133**——这就是「形状没有封闭清单」最直白的证据，也解释了为什么 A328 要求先立封闭清单
  （与 A77 单一权威口径同族）再谈反方向对账。
- **⑦ 中间读数与事故，原样入册不美化**：
  ⑴ 新守卫第一版定向读数 **7 failed / 17 passed**，七支失败全部由「账本被外部打回」导致，不是判据本身错。
  ⑵ ID 宽度改了三遍（三位 ⇒ 两位 ⇒ 一位），`expand` 补零造出假 ID 一版，注入臂打错面一版
  （把「加声明句」写成改表行——表行不在声明面，gap 当然不动，臂空转），`test_every_row_has_four_cells`
  把「竖线数 == 3」当判据一版（实测分布 5/6/7/15，行内码就带竖线），动态取样写死 A31 一版
  （A31 有标记但不含轮号，既不是 gap 也不是干净的正例）。
  ⑶ **工作树事故**：本轮账本改动曾被同一工作树的外部 `git stash` + `git reset` 打回，而那一笔 stash 随后被 drop
  ⇒ `git stash list` 与 stash reflog 双双为空。找回路径：`git fsck --lost-found` 里的不可达对象
  `82f4a4cab3e86d66abc06ac9cb60b7a87532144a`（提交名 `WIP on optimism20261012`，含他会话 40+ 个文件），
  本轮只按显式路径取回自己的三个文件，**没有 pop**、没有触碰 laiyipao / oa4rust / mobile 那一片。
  ⑷ `git checkout <blob> --` 把卷二 L241 日志块里那个历史游离 CR 摘掉了，摘后卷二变成「3543 个 LF + 1 个 CRLF」
  的混用形状，`test_docs_markdown_structure` 的行尾维当场红。本轮顺势把卷二收成**统一 LF**（代价：账本提交里
  多一行**不可见的行尾改动**，那一行文本内容一字未变），同时把 A328 行从「A326 之后」搬到「A327 之后、独立成块」——
  夹在 A326 与空行之间会凑出 2 行连续竖线，`find_tables` 认 ≥2 行即为表并要求第二行是分隔行 ⇒ 结构守卫红。
  ⑸ 前端面（`web/`）**不适用**：本轮零改动，不是漏跑。
