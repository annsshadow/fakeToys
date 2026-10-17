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
- [x] **L222** `932c5885a` + `c6209ab67`（归因订正）+（账本批即本条所在提交）`test,docs(augmentor)`：**A275 收口——配置面三侧同源对账守卫落成机器**（`docs/augmentor/README.md` 配置表 / `fields(AppConfig)` / `config.yaml` 三角互核）。README 表格 16 行 34 键 → **22 行 70 键**，权威只有一份：节与键全从 `fields()` 实推，测试侧唯一手写名单 `META_ROWS`（`models` / `app`）逐员机器验真。`config.yaml` 那一侧的判据被文件自己纠正过一次——不是「与 schema 逐节全等」而是**整节写全或整节注释列全**（`logging` 三键刻意不落地，写出来才装配）。用最终版尺子对 HEAD 那份文档量出的真漂移是 **21 红 / 38 绿**：5 个整节缺席、13 节只列半节、恰好列全的只有 `export` 与 `multilingual`。注入七臂各有牙齿（4/2/3/3/5/2/3 红），A5「schema 新增一只 init 键」复现的正是 L220 那一步的形状。顺带量出一格普查盲区：`config.yaml` 的 `app` 三键既无 `AppConfig` 字段、又被校验器 `META_TOP_SECTIONS` 免检 ⇒ 「只覆盖 schema 键」的死键普查**结构上看不见**它（零读者没有读者这件事仓内三处早有记录，历史处置是降为可选；新的是盲区与处置未立账）⇒ 立 **A276**。l97 / l98 两把参数化棘轮按现量回填（CALLS 399→402、盲面 111→114、plain 57→58、derived 36→38、盲面用例 106→109）。门禁 **8298 passed / 11 skipped / 0 failed / 99.80%**，用例增量 `+59` 与新文件支数吻合 ⇒ 详见循环日志 L222
- [x] **L223** `6c53d986c` +（账本批即本条所在提交）`test,docs(augmentor)`：**A277 收口（同轮立同轮关）——`docs/augmentor/API.md` 逐端点「人读字段提示」与 OpenAPI 契约面落成机器对账**（`tests/unit/test_api_doc_field_hints_l223.py`，23 支）。详细表 31 格里提取到 **58** 个标识符形状的字段声称，其中 **40** 个（69%）直接落在该端点的契约面上；其余由四档**有位置的**证据接住（作用域内字面量 / 定义名 / 配置键 / CLI 选项）加一份封闭 JSON 词表。字面量档按「路由文件 + 两跳 import 闭包」限作用域，**两跳这个数字是量的**：一跳会把 `stratify_distribution` 误判成无据，三跳时 convert 的闭包涨到 70 个文件、`output_path` 反而有据 ⇒ 判据失去区分力。尺子首跑报出 3 格无据，**其中 2 格是尺子自己的 bug**（相对导入层级换算写错，`augmentor` 包内部的 `from .x` 一条都没解析进闭包），修完只剩 `output_path` 那一格，也就是本轮要修的真文档缺陷 4 处（总览散文 1 + 详细表 3；字段实为 `output_file`，`split` 是 `output_dir`，`output_path` 只是路由局部变量名）。`docs/augmentor/README.md` 与 `docs/augmentor/API.md` 全文再无该名字。注入九臂各有牙齿（2/2/6/1/7/1/3/1/3 红），其中 A1/A2 成对：同一处文档缺陷，作用域退回全仓后**主判据转绿** ⇒ 那档作用域是承重的。本轮**零 parametrize 新增** ⇒ l97 / l98 两把棘轮读数一格未动，未回填。门禁 **8321 passed / 11 skipped / 0 failed / 99.80%**（195.43 s），用例增量 `+23` 与新文件支数吻合。顺带量到 `additionalProperties: true` 的自由子对象（如 `ImpactResponse.before/after`）在契约面上给不出键名 ⇒ 立 **A278** ⇒ 详见循环日志 L223
- [x] **L224** `08bc4f0c8` +（账本批即本条所在提交）`test,docs(augmentor)`：**A279 立账——后端 75 条操作与前端服务层 43 条调用形状的消费对账落成机器**（`tests/unit/test_api_endpoint_consumption_l224.py`，26 支）。两侧权威各一份：契约面从 `app.openapi()` 实推（**75 条操作 / 71 条路径**），前端面从 `web/src/services/api.ts` 的 `api.<verb>('路径')` 形状实推（**43 条**），路径前缀从同一份源码的 `baseURL` 读，不在测试里写死第二份。**缺口 32 条**落成名单且与实测集合**相等** ⇒ 只会随接线变短，新增一条没接的端点当场红；32 条里 **26 条有 CLI 子命令入口**（映射去重后 **21** 个名字，逐个由 parser 的 `add_parser` 名单验真）、**6 条两边都没有**（`dataset/evaluate`、`impact`、`language-profile`、`semantic-search`、`translate`、`vector-status`）。query 一维量出**两处来源**：写在路径串里的 1 条 + axios `params:` 对象里的 4 条 / 5 个键，只认前者会漏掉 `page_size` 这个「TS 侧改名后的对外名」；归属判据用「最近的前一个调用」，被实测否掉的备选是往后看固定 400 字符（候选从 4 涨到 7，多出三桩冤案）。去歧义同样有实测：43 条形状里恰 1 条（`GET /api/versions/history`）同时落在字面量与参数化两条路由上，绑定只认占位符最少那一档，并拿 `TestClient` 的 200 + 响应体形状证明赢的那条确实是它——依据是运行时行为，不是匹配规则。注入十臂各有牙齿（1/4/6/3/2/2/1/7/2/1 红），其中 A0 只响 1 支：真把两条路由的声明顺序对调，**只有那条运行时判据会响**。本轮**零 parametrize 新增** ⇒ l97 / l98 两把棘轮读数一格未动，未回填。门禁 **8347 passed / 11 skipped / 0 failed / 99.80%**（196.80 s），用例增量 `+26` 与新文件支数吻合。顺手量到 8 个路由文件把下游异常一刀切收敛成 500 ⇒ 立 **A280** ⇒ 详见循环日志 L224
- [x] **L225** `e7dfa43ab` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第一批接线——6 条「两边都没入口」里先接 3 条，消费面 43 → 46**（`language-profile` 与 `evaluate` 进数据分析页、`vector-status` 进系统状态页；服务层三只 fetchers 加三个响应类型，后端字段名与响应形状一字未改，`include_details` 在页面上显式送 `false`——契约默认是 `true`，见 `api/routes/dataset_tools.py:231`）。守卫按实测回填四格：调用形状 **43 → 46**、缺口名单 **32 → 29**（划掉的正是这三条）、已消费 **43 → 46**、`NO_ENTRY` 队列 **6 → 3**（剩 `impact` / `semantic-search` / `translate`）；`CLI_ENTRY` 的 26 条 / 21 个名字一格未动，因为本轮只动前端。A279 的三件收口判据齐：名单与实测相等且变短、`api.wiring.test.ts` 的 `KNOWN_UNWIRED` 仍为空、每员覆盖成功与失败两路（前端 vitest **13 文件 127 支，+8**，`tsc --noEmit` 与 `eslint --max-warnings 0` 干净）。注入五臂各有牙齿（3/1/3/1/1 红）：A0 把服务层路径拼错 ⇒ 消费守卫响 3 支（幽灵方向 + 名单相等 + 名单规模）；A1 只 import 不调用 ⇒ 接线守门响 1 支；A2 删掉整张向量卡 JSX ⇒ 该页响 3 支且原有 4 支不动；A3 把「双侧必填」削弱成单侧 ⇒ 守卫用例响 1 支；A4 摘掉分布表的 `.sort` ⇒ 排序用例响 1 支。**A4 那一支是先量出假绿才补的**：画像 fixture 的插入序原本恰好与条数序同向，摘掉排序照样绿，改成故意反向之后 A4 才响。顺手量两件事，都按「不动，只立账」处置：**A281**（45 只带类型的 fetchers 里 40 只的 TS 响应类型字段集与契约 200 响应的属性集逐字相等，3 只缺字段——`AppConfig` 少 `versioning` / `visualization`、`UpdateConfigResponse` 少 `ignored_keys`、`StatusResponse` 少 `model_retry_count` / `model_retry_wait_seconds`，方向一致都是「后端在回、前端类型上不存在」）、**A282**（`prettier --check src` 的仓内基线本来就是红的：32 个文件不合规，把本轮碰过的 5 个文件从 `HEAD` 取到仓外用同一份配置复查同样全红 ⇒ 本轮没引入漂移，而 CI 里根本没有 format 门）。本轮**零 parametrize 新增** ⇒ l97 / l98 两把棘轮读数一格未动，未回填；后端用例总数与 L224 持平（**8347 passed / 11 skipped / 0 failed / 99.80%**，198.98 s），增量全在前端那一侧 ⇒ 详见循环日志 L225
- [x] **L226** `dfb9ab0ae` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第二批接线——`search` / `validate` / `auto-config` 进页面，消费面 46 → 49**（内容检索进数据分析页、结构校验进质量中心新 tab、按数据推荐配置进配置管理页；三只 fetchers + 三只响应类型 + 三处页面入口 + 三页各补成功/失败两路，后端字段名与响应形状一字未改）。守卫按实测回填五格：调用形状与已消费 **46 → 49**、缺口名单 **29 → 26**、`CLI_ENTRY` **26 条/21 名 → 23 条/18 名**、`NO_ENTRY` 仍 **3**。新立跨语言对账守卫 `test_wired_dropdown_choices_l226.py`（23 支）：两张下拉清单与 `SEARCH_METHODS` / `VALIDATION_PRESETS` / `PRESET_RULES` 逐项相等、CLI 的 `--method` choices 首次入尺、label 写死的数字真读规则表验证、端点面回执 5+3 档全 200 与清单外两档回字符串 detail；五档注入臂红 3/1/1/1/4 格，跑完按字节还原。**明确拒接两员并记账**：`GET /api/system/dependencies`（与 `/api/status` 的 dependencies 同数据两面，接了只涨指标）与 `POST /api/dataset/translate`（整份译文只在响应体、无 `output_file` 可落盘 ⇒ 另立 **A283**）。顺手修掉去重卡片「启用水重」错字。census 两把棘轮按实测重钉（CALLS 402→404、READABLE 288→290、盲面 114 未动）。门禁：后端 **8370 passed / 11 skipped / 0 failed / 99.80%**（189.99 s），前端 **13 文件 / 140 支全绿**（+13）、`tsc` 与 `eslint` 均干净 ⇒ 详见循环日志 L226
- [x] **L227** `835d15d4d` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第三批接线——`impact` / `semantic-search` 进数据分析页，消费面 49 → 51，并把「抄进页面的默认值」钉回请求模型**（`NO_ENTRY` 队列里最后两只「SDK 做完、web 与 CLI 都没人取」的缺口，只动 `web/src`，后端字段名与响应形状一字未改；`translate` 留给 A283）。守卫按实测回填四格：调用形状与已消费 **49 → 51**、缺口名单 **26 → 24**、`NO_ENTRY` **3 → 1**（只剩 `translate`）、`CLI_ENTRY` 的 **23 条 / 18 名一格未动**（本轮只动前端）。新立跨语言对账守卫 `test_impact_semantic_wiring_l227.py`（20 支）：**这两只没有「字段名权威常数」可引**（l226 那种封闭清单在这里不存在，`["instruction", "input", "output"]` 只是 SDK/CLI 里五处手写字面量），权威换成三处来源——请求模型 `model_fields[...].default`、真实回执的 `gains` 键集、`AugmentationMetrics` 的 dataclass 字段；页面四档初值逐个与契约默认同值，服务层不写第二套默认。新暴露两档此前任何产品面都调不到的旋钮（`text_key`、`min_scale_gain`），`前 K 条` 的下界按实测从 1 放宽到 0（`top_k=0` 是合法的「只要回显、不要命中」档，端点 200 且 `results` 为空）。顺手量到 `AugmentationMetrics.unique_instructions` 是误名（`augmentor/impact.py:76` 数的是 `len(set(texts))`，`texts` 取自 `text_field`）⇒ 标签按诚实口径写「唯一值数（按所选字段）」并立 **A284**，那五处重复字面清单立 **A285**。census 按实测重钉（CALLS 404→405、READABLE 290→291、盲面 114 与 l98 档位和未动——本轮唯一新增的 parametrize 值位是内联字符串清单）。注入十二臂各有牙齿（Python 面 2/2/2/3/1/1/3，vitest 面各 1）：**第一批有五臂被取证脚本自己读成「空转」，那是 ANSI 色码截断了汇总行，剥色重跑才是真读数，假读数没有入账**。前端 **13 文件 / 149 支全绿**（+9：Analysis 15 → 24）、`tsc` 与 `eslint` 均干净；门禁：后端 **8390 passed / 11 skipped / 0 failed / 99.80%**（188.88 s，`+20` = 新守卫 20 支）；账本批复跑时 A184 那把注释引用尺子把账本第一版里一个「尚不存在的新模块名」红了两支 ⇒ 改口不指名，阈值一格未动；同轮立 **A286**（比值型计时守卫在负载下会假红：那一次全量 3 failed / 8387 passed、比值 4.38 对阈值 3.0，同条用例隔离复跑四次全绿、整轮 257.50 s 比 188.88 s 高 36%） ⇒ 详见循环日志 L227
- [x] **L228** `ed2409452` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第四批接线——`stats` / `features` / `compare` 三条数据集操作进数据分析页，消费面 51 → 54**（字段级统计、特征普查、双集对比三张新卡 + 三处页面入口，只动 `web/src`，后端字段名与响应形状一字未改）。守卫按实测回填四格：调用形状与已消费 **51 → 54**、缺口名单 **24 → 21**（划掉的正是这三条）、`CLI_ENTRY` **23 条 / 18 名 → 20 条 / 15 名**（三条各有 CLI 子命令，所以这轮动的正是这一半）、`NO_ENTRY` 仍 **1**。新立守卫 `tests/unit/test_stats_features_compare_wiring_l228.py`（30 支），**这批换了一条判据**：l226 量「下拉 vs Python 封闭常数」、l227 退到「页面发得出每档都被端点接受」，这一批连那条都不够用——实测这两档旋钮给错值时后端**不出声**：`fields:["nope"]` ⇒ **200** 且回执里多出一整只十二键全零的**幽灵字段**（`summary` 还说「包含 1 个字段」），`key_fields:["nope"]` ⇒ **200** 且把两份毫不相干的数据回成 `common_items 1 / similarity_score 1.0`，而同一条数据默认键回的是 `0 / 0.0`（真话）⇒ **幽灵匹配键是假阳性，比零更贵**。因此页面这两处不许出现任何手写字段名，候选只能来自上一次 200 回执的键集，由 `TestCandidatesComeFromTheReceipt` 守。**又一条被探针推翻的想象**：初稿断言「换匹配键会让逐条值差异从空变成逐条列出」，读了 `augmentor/comparison.py:217` 与 `augmentor/compare_enhanced.py:289` 才知道 `_compare_fields(items_a, items_b)` 压根不接 `key_fields`，明细恒按 `instruction` 配对、条目键固定 `{instruction, value_a, value_b}`，`key_fields` **只**动 `metrics` ⇒ 订正落在 `types/api.ts` 与页面注释两处，并钉成正向不变量用例。顺手立两笔：**A287**（`similarity_to_a` / `similarity_to_b` 全仓只有 dataclass 默认与 `to_dict()` 两处出现、无赋值点 ⇒ 真数据上恒 `0.0`，契约面死面，页面不渲它）、**A288**（B 侧 0 条时 `size_ratio` 在 SDK 是 `float('inf')`（`augmentor/compare_enhanced.py:190`），经 JSON 到前端是 **`null`** ⇒ TS 类型与页面读数按线上真相写成 `number | null` 并显示「—（B 侧 0 条）」）。census 两把棘轮按现量重钉（CALLS 405→406、READABLE 291→292、盲面 114 与 l98 档位和未动——本轮唯一新增的 parametrize 值位是内联字符串清单）。注入八臂（Python 面 6 臂各点名单支、vitest 面 2 臂各单支）逐臂 rc=1 且输出含 `failed`，三个被改文件逐臂字节复原 + 收尾重新验盘一致。前端三门禁：**`tsc` 干净 / `eslint` 干净 / vitest 13 文件 149 → 163 支全绿**（Analysis 24 → 38）。门禁：**8420 passed / 11 skipped / 0 failed / 99.81%**（238.55 s，`+30` = 新守卫 30 支）。一条「尺子咬到我」的实测：A184 那把注释引用尺子把我写的平局判据行号（259）红了两支——259 是空行，那句判据实落在 **260**（`augmentor/comparison.py:260`）；两处同值就地改口、净行数零（`git diff --numstat` 复核），阈值一格未动 ⇒ 详见循环日志 L228
- [x] **L229** `820821fda` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第五批接线——`convert` / `merge` / `split` 三只写盘操作进数据管理页，消费面 54 → 57**（格式转换、合并、分割三张新卡；**只动 `web/src`**，后端字段名与响应形状一字未改；唯一一处产品面改动是把转换回执补成「响应模型的每一格都读得出来」，同行追加、`git diff --numstat` 复核净行数零）。守卫按实测回填四格：调用形状与已消费 **54 → 57**、缺口名单 **21 → 18**、`CLI_ENTRY` **20 条 / 15 名 → 17 条 / 12 名**（三条各有 CLI 子命令，所以动的又是这一半，与 L226 / L228 同形）、`NO_ENTRY` 仍 **1**（`translate`，归 **A283**）。新立 `tests/unit/test_transform_wiring_l229.py`（**55 支**）与渲染面 `web/src/pages/DataManagement.transform.test.tsx`（**13 支**）。**这批的判据换到「写边/读边两张格式清单各只有一个权威」**：转换目标下拉逐项等于 `OUTPUT_FORMAT_CHOICES`（`augmentor/converter.py:91`），源格式下拉复用上传那份 `UPLOAD_FORMAT_CHOICES`（读边比写边多一个 `xls`，`augmentor/converter.py:56` 的 `EXCEL_TARGET_NAMES` 正是那道差的由来），端点拒 `xls` 时自己枚举的十二档与页面清单逐项相等 ⇒ 页面编不出第十三档。八臂「初值 == 契约默认」逐个对账，`seed` / `source_format` 从空开始（留空即不发那一格），服务层不写第二套默认（全文只有一处 `|| undefined` 哨兵）。**又两条被探针推翻/证实的静默形状立成账**：**A289**（分割两条支路判据不同：容差默认支 0.01、分层支 1e-6，负数只有分层支拦，两句判决文案也不同）、**A290**（合并端点四档旋钮只接一档、SDK 六键只上线五键，而响应模型的 docstring 写着「一一对应」）、**A291**（空 `source_format` 的判决缺主语）。**还有一条被探针推翻的想象**：初稿断言「输出写绝对路径 ⇒ 403」，实测在白名单根内是 **200**，根外才是 403，`..` 则是 400 ⇒ 判据改成三档分开钉。census 两把按现量重钉：CALLS 406 → **411**、READABLE 292 → **293**、盲面 114 → **118**；l98 档位和同步 **118**（opaque 10 → 11、ids-derived 8 → 11，plain / derived 未动）、盲面用例 109 → **113**、豁免用数 17 → **18**。注入八臂各有牙齿（Python 面五臂：回执吞掉 `output_file` / 服务层路径拼错 / 页面往目标清单加 `xls` / 契约默认改一格 / 删掉诚实口径那句；vitest 面三臂：目标清单多一档 / `train` 初值替契约编默认 / 删掉分支判据差那句），逐臂 rc=1 且输出含 `failed`，被改的三个文件逐臂字节复原 + 收尾重新验盘一致；**中间读数不美化**：vitest 探针 v1 用「输出尾部 700 字含 failed」判红，把 vitest 的失败摘要截掉了，三臂全被读成 NOT-RED（其中一臂的输出明明带着红），订正成对全量输出判之后才是真读数。前端三门禁 `tsc` 干净 / `eslint` 干净 / vitest **14 文件 163 → 176 支全绿**。门禁：**8475 passed / 11 skipped / 0 failed / 覆盖率 99.81%**（244.78 s，与 L228 的 8420 相减 `+55` = 新守卫 55 支，接线本身零后端用例增量）；六把账本与文档守卫 **149 passed / 8 skipped** ⇒ 详见循环日志 L229
- [x] **L230** `32028f73b` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第六批接线——`sample` / `aggregate` / `rag` 三只写盘操作进数据管理页，消费面 57 → 60**（采样、多源聚合、转成 RAG 检索格式三张新卡；**只动 `web/src`**，后端字段名与响应形状一字未改）。守卫按实测回填四格：调用形状与已消费 **57 → 60**（75 条的 80%）、缺口名单 **18 → 15**、`CLI_ENTRY` **17 条 / 12 名 → 14 条 / 9 名**（三条各有 CLI 子命令，动的又是这一半，与 L226 / L228 / L229 同形）、`NO_ENTRY` 仍 **1**（`translate`，归 **A283**）——**至此全仓六只会落盘产物的端点（`convert` / `merge` / `split` / `sample` / `aggregate` / `rag`）在页面上都有了入口**。新立 `tests/unit/test_sample_aggregate_rag_wiring_l230.py`（**74 支**）与渲染面 `web/src/pages/DataManagement.sample.test.tsx`（**19 支**）。**这批又换了一档判据**：这三只端点背后的两个后端权威——`DatasetOperations.sample` 的三种 `method`、`DataAggregator.aggregate` 的四种 `strategy`——**在仓内都没有可引常数**（不像 L226 的 `SEARCH_METHODS`、L229 的 `OUTPUT_FORMAT_CHOICES`），只有 if/elif 分发臂与散落的字面量；照抄字面量等于自造第二套权威，所以守卫改成**读 AST 数分发臂**（`dispatch_arms()` 逐条读 `config.method == 「字面量」` / `strategy == 「字面量」` 的比较臂），**一条都数不到就抛 `LookupError` 而不是当成干净**（数不到 ≠ 没有）。八臂「页面初值 == 契约默认」逐个对账，服务层三只 fetchers 全文无第二套默认（只有转换卡那处 `|| undefined` 哨兵仍在，而这一批实测**送空串与省略等价**，所以那一处是形状卫生不是躲判决，与 **A291** 那处动机不同）。**三处被自己的夹具推翻的中间读数**：① 初稿断言「SDK 只报 `conflict_count`、线上只报 `conflicts`」，实测两套键集**各有两格对方没有**（SDK 多 `total_count` / `conflict_count`，线上多 `output_file` / `aggregated_count`）⇒ 改写成「两套手维护的键集已经漂移」并立 **A293**；② 初稿把 RAG 默认档引成 `RagConfig.model_fields`，实测真名是 `RAGConfig` 且是 dataclass（`augmentor/config.py:533`）⇒ 改引 `__dataclass_fields__` 并删掉那条永远走不到的双分支；③ 初稿断言「`target_size` 省略会 400」，实测省略是 **200** 并走 `DEFAULT_TARGET_SIZE = 100`（回落点 `augmentor/aggregator.py:159`），只有 weighted 那一支真的会被它咬 ⇒ 立 **A294**。**顺手订正 `docs/augmentor/API.md` 两句假话**（aggregate 行的「省略会 400」与 rag 行的默认档），两处都是**同行追加、`git diff --numstat` 复核净行数零**，并新立四支「文档行必须说端点真的会做的事」的用例把订正钉住（判据咬到自己那一处：新用例的第一版被我自己写进 API.md 的那句订正说明红了一次）。**六笔新账全部只立不做**：**A292**（`ratio` 无判决，三种方法三种结局）、**A293**（聚合端点构造的是无参 `DataAggregator()`，三只旋钮不可达 + 两套回执键集漂移 + `conflicts` 整份上线）、**A294**（`target_size` 只咬 weighted）、**A295**（默认格式有第三处字面量 `langchain`）、**A296**（分层抽样 `value[:10]` 静默折叠分组）、**A297**（`consistent` 把同源内部分歧算成冲突且那些条目永不落地）。census 两把按现量重钉：CALLS 411 → **419**、READABLE 293 → **301**、盲面 **118 一格未动**（本轮八处新增 parametrize 的值位全是内联字面量清单 ⇒ 全落可读面）；l98 豁免用数 18 → **23**（五支新用例的值位来自 AST 分发臂与请求模型字段集，逐支带理由入册；豁免名单变长不是放松，判据仍逐行判值位形状）、档位和与盲面用例 **118 / 113 未动**。注入九臂各有牙齿（Python 面六臂 2/5/3/1/1/1 红，基线 70 passed；vitest 面三臂 2/1/1 红），逐臂 rc=1 且输出含 `failed`，被改文件逐臂字节复原 + 收尾重新验盘一致（sha256 `8b17a662` 起）；**其中一臂被拦在写盘之前**：W3 的锚点 `完全空转` 在文件里出现两次（模块注释与卡片文案），唯一性断言当场响 ⇒ 磁盘未被触碰，换成更长的锚点后才注入成功——这正是注入器该有的反应。前端三门禁 `tsc` 干净 / `eslint` 干净 / vitest **15 文件 176 → 195 支全绿**。门禁：**8549 passed / 11 skipped / 0 failed / 覆盖率 lines 99.93%、branches 99.39%**（179.78 s，与 L229 的 8475 相减 `+74` = 新守卫 74 支，接线本身零后端用例增量）；六把账本与文档守卫 **149 passed / 8 skipped** ⇒ 详见循环日志 L230
- [x] **L231** `d90148b38` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第七批接线——备份那一族四只端点进系统状态页，消费面 60 → 64**（一张「数据备份」卡把 `GET /api/system/backups`、`POST /api/system/backups`、`POST …/{backup_id}/restore`、`DELETE …/{backup_id}` 四只全接上：源文件候选取自 `/api/data/list` 也允许手填路径，恢复目标手填，删除走 Popconfirm 二次确认，回执把响应模型每一格都读出来，失败一律显示服务端那句判决）。守卫按实测回填四格：调用形状与已消费 **60 → 64**（75 条的 85%）、缺口名单 **15 → 11**、`CLI_ENTRY` **14 条 / 9 名 → 10 条 / 8 名**（`backup` 这一个子命令此前吃掉四条操作，本轮**整名出名单**；一次减 4 条是现量最大的一批）、`NO_ENTRY` 仍 **1**（`translate`，归 **A283**）。新立 `tests/unit/test_backup_wiring_l231.py`（**33 支**）与渲染面 `web/src/pages/System.backup.test.tsx`（**11 支**）。**这一批同时改了一处产品面**：`DatasetBackup.backup()` 的 `name` 直接决定落点，而它来自调用方——仓外探针实测 `name="../../escaped"` 打创建是 **200** 且真把文件写到备份目录之外（白名单只管到 `backup_dir` 那一格），补门之后同一条请求 **400**「备份名称不能指向备份目录之外」、白名单外零文件；`name` 带 `/` 仍在备份目录内建子目录（实测 `sub/deep` ⇒ `.backups/sub/deep.json`），那一档照旧放行。这处 +4 行把 `augmentor/backup.py:188`（L273 后为 `:195`）平移到 **`:192`**（L273 后为 `:199`），两处历史引用（本账 A277 行与 L223 日志块）同轮重钉、`git diff --numstat` 复核净行数 **0**。新立七条 **A298~A304**（同名备份在索引里留两行且删一次全清、校验和从不拦截恢复、两只 SDK 函数没有 HTTP 入口、「默认备份目录」仓内三份且端点那份与前两份不是同一个目录、前端没有送 `X-API-Key` 的通道、`backup()` 里两次 `datetime.now()`、候选清单窄于白名单）。**A277 那道「全文不许出现 `output_path`」的禁令按现量收窄而不是摘掉**：那个名字在 `POST …/restore` 的**响应**里是真的，收窄后 dataset 七条写盘族一行仍不许出现、全文只许 restore 那一行（注入臂 I5 实证原缺陷照旧喊红）。注入六臂逐臂点名喊红（**3 / 1 / 6 / 1 / 2 / 3** 支），被改的六个文件逐臂字节复原、收尾重新验盘并断言哈希与进入时一致。本轮**零 parametrize 新增**（两支新文件里 `parametrize` 出现 0 次）⇒ l97 / l98 两把普查棘轮读数一格未动；哈希槽那把尺子只读**第一本账**（`LEDGER = OPTIMIZATION_LOOP.md`），本轮动的是第二本 ⇒ 六档现量（15/24/15/69/占位 0/R6 白名单四格）同样一格未动。门禁：**8582 passed / 11 skipped / 0 failed / 覆盖率 lines 99.93%、branches 99.39%**（180.57 s，与 L230 的 8549 相减 `+33` = 新守卫 33 支）；前端 `tsc --noEmit` 与 `eslint .` 均 rc=0，`vitest run` **16 文件 / 195 → 206 支全绿** ⇒ 详见循环日志 L231
- [x] **L232** `d17cd5b56` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第八批接线——数据集血缘那一族三条进系统状态页，消费面 64 → 67**（第四张卡「数据集血缘」把 `GET` / `POST /api/system/dependency/datasets` 与 `GET /api/system/dependency/graph` 三条全接上：登记表单（名称 / 源文件候选取自 `/api/data/list` 也允许手填 / 描述 / 标签，中英文逗号都切成数组且空段被丢）、已登记列表按服务端原样渲染（空描述与空标签各有可见占位）、图的三格计数与 `issues` 逐条列出）。守卫按实测回填五格：调用形状与已消费 **64 → 67**（75 条的 89%）、缺口名单 **11 → 8**、`CLI_ENTRY` **10 条 / 8 名 → 7 条 / 7 名**（`dependency` 一个子命令此前吃掉三条操作，本轮**整名出名单**）、`NO_ENTRY` 仍 **1**（`translate`，归 **A283**）、`params` 归属 **8 → 11 条 / 去重后 7 → 8 个键名**（`registry_path` 第一次进这一维）。新立 `tests/unit/test_lineage_wiring_l232.py`（**39 支**）与渲染面 `web/src/pages/System.lineage.test.tsx`（**8 支**）。**这一批同时改掉一处产品缺陷（A305）**：`POST …/dependency/datasets` 的 `description` / `tags` 改前**回显即丢**——端点在 `register_dataset()`（内部已落盘）**之后**才改返回对象，改的是内存，于是响应带着这两格而随后的 `GET` 与磁盘 `datasets.json` 里都是空；改成随登记一起传下去之后三处一致（仓外探针实测响应 / GET / 磁盘都带着 `描述B` 与 `['t1','t2']`）。**只看响应的那把尺子在改前也是绿的**，所以新守卫那一条判据只读磁盘与 GET。`api/routes/system_ops.py` 净 **+1 行**（6/5），而这一处之上的历史行号引用（150/295/397/398/400/415）无一平移，`git diff --numstat` 复核；`docs/augmentor/API.md` 与 `README.md` 各**净 0 行**（同行追加），所以本账第 945 行那条 `docs/augmentor/API.md:1139` 引用仍指得准（940 → **945** 是 L287 记账批按 ⑤ 内容定位器现量重钉：本批在被指向那一行之上落 5 行——清单行 +1 与新立 A390 / A391 两笔连带各自的分隔空行 +4；A389 那一格是同行改写（净 0 行），日志块追加在文件尾 ⇒ 落在被指向那一行之下不参与平移。前置量：批后 import 那把尺子的 `locate_operative_target` 现读 945 且唯一命中；同批 l268 的三档闭合回到 `LIVE 1 / STALE 5`，不是算术推的。936 → **940** 是 L286 记账批按 ⑤ 内容定位器现量重钉：本批在被指向那一行之上落 4 行——清单行与它的分隔空行、新立 A389 那一行与它的分隔空行；A374 那一行与日志块 ⑦ 两处都是同行改写（净零行），日志块追加在文件尾 ⇒ 落在被指向那一行之下不参与平移。前置量：批后 import 那把尺子的 `locate_operative_target` 现读 940 且唯一命中，再把本批那四行在内存里逐行撤掉即复现 936 同样唯一命中；同批 l268 名册那一格与三档闭合由红读数 `assert 936 == 940` 与 `{'LIVE': 0, 'STALE': 6, 'BLIND': 7}` 逼出、重钉后两档回到 `LIVE 1 / STALE 5` ⇒ 不是算术推的；931 → **936** 是 L285 记账批按 ⑤ 内容定位器现量重钉：本批在被指向那一行之上落 5 行——清单行 +1 与新立 A387 / A388 两笔连带各自的分隔空行 +4，日志块追加在文件尾 ⇒ 落在被指向那一行之下不参与平移；这一格的前置量由批写脚本 import 那把尺子的 `locate_operative_target` 现读（批前 931、批后 936，两次各唯一命中），同批 l268 名册那一格由红读数逼出 ⇒ 不是算术推的；926 → **931** 是 L284 记账批按 ⑤ 内容定位器现量重钉：本批在被指向那一行之上落 5 行——清单行 +1 与新立 A385 / A386 两笔连带各自的分隔空行 +4，日志块追加在文件尾 ⇒ 落在被指向那一行之下不参与平移；这一格的前置量由批写脚本 import 那把尺子的 `locate_operative_target` 现读（批前 926、批后 931，两次各唯一命中），同批 l268 的名册与三档闭合由红读数 `assert 926 == 931` 与 `{'LIVE': 0, 'STALE': 6, 'BLIND': 7}` 逼出、重钉后两档回到 `LIVE 1 / STALE 5` ⇒ 不是算术推的；917 → **926** 是 L283 记账批按 ⑤ 内容定位器现量重钉：本批在被指向那一行之上落 9 行——清单行 +1 与新立 A381 / A382 / A383 / A384 四笔连带各自的分隔空行 +8，日志块追加在文件尾 ⇒ 落在被指向那一行之下不参与平移。先红的是三档闭合那一格（逐字 `{'LIVE': 0, 'STALE': 6, 'BLIND': 7} != {'LIVE': 1, 'STALE': 5, 'BLIND': 7}`），再红 operative 那一格（逐字 `assert 917 == 926`），由定位器现量唯一命中重钉之后两档回到 `LIVE 1 / STALE 5`，同批复验 `test_the_operative_number_matches_the_independent_locator` ⇒ 不是算术推的；912 → **917** 是 L282 记账批按 ⑤ 内容定位器现量重钉：本批在被指向那一行之上落 5 行——清单行 +1 与新立 A379 / A380 两笔连带各自的分隔空行 +4；本轮日志块落在被指向那一行之下 ⇒ 不参与平移；改前三档读数逐字 `assert {'LIVE': 0, 'STALE': 6, 'BLIND': 7} == {'LIVE': 1, 'STALE': 5, 'BLIND': 7}` 与 `assert 912 == 917`，重钉后回到 `LIVE 1 / STALE 5`，由 `test_the_operative_number_matches_the_independent_locator` 独立复验 ⇒ 不是算术推的；上一格 907 → **912** 是 L281 记账批按 ⑤ 内容定位器现量重钉：本批在被指向那一行之上落 5 行——清单行 +1 与新立 A377 / A378 两笔连带各自的分隔空行 +4；本轮日志块 11 行落在被指向那一行之下 ⇒ 不参与平移；红读数逐字 `assert {'LIVE': 0, 'STALE': 6, 'BLIND': 7} == {'LIVE': 1, 'STALE': 5, 'BLIND': 7}`，同行改写这一格后两档回到 `LIVE 1 / STALE 5`，由 `test_the_operative_number_matches_the_independent_locator` 独立复验 ⇒ 不是算术推的；上一格 902 → **907** 是 L280 记账批按 ⑤ 内容定位器现量重钉：本轮在被指向那一行之上落 5 行——清单行 +1 与新立 A375 / A376 两笔连带各自的分隔空行 +4；红读数逐字 `assert 902 == 907`，同批 `LIVE 1 → 0` / `STALE 5 → 6` 两档先红、同行改写这一格后回到 `LIVE 1 / STALE 5`，由 `test_the_operative_number_matches_the_independent_locator` 独立复验 ⇒ 不是算术推的；行号最初由 L245 按现量重钉：L244 那轮改成 781，本轮在它之上落了 3 行——清单行、Backlog 新账与它的分隔空行——把它推到 786；**L246 的清单行又把它推到 789 而那一轮只核未重钉**，本轮清单行再推 1 行 ⇒ 按现量重钉为 790，**L248 的清单行再推 1 行 ⇒ 本轮按现量重钉为 791**，**L249 的清单行再推 1 行 ⇒ 本轮按现量重钉为 792**，**L250 的清单行 +1、Backlog 新账那一行连带它与 Backlog B 标题之间的分隔空行 +2 ⇒ 本轮按现量重钉为 795**，**L251 的清单行 +1（本轮不收新账 ⇒ Backlog 区净 0）⇒ 按现量重钉为 796**（**订正：L251 只把这串数字补进链尾，句首 operative 那一格一直写着 795，两者不是同一个数**），**L252 的清单行 +1、Backlog 新账连带它的分隔空行 +2 ⇒ operative 与链尾同批落到 799**；**订正：L253 那一批落了 +3（清单行 +1、A337 连带它的分隔空行 +2）而链上只接了数字 802 没有写这一格归因，本轮补上**、**L254 的清单行 +1、A338 连带它的分隔空行 +2 ⇒ operative 与链尾同批落到 805**、**L255 的清单行 +1 ⇒ 806**（那一轮句首 operative 与链尾同时改过）、**L256 的清单行 +1 且零新 Backlog 行 ⇒ operative 与链尾同批落到 807**、**L257 的清单行 +1 与新立 A339 连带它的分隔空行 +2 ⇒ operative 与链尾同批落到 810**、**L258 的清单行 +1 与新立 A340 连带它的分隔空行 +2 ⇒ operative 与链尾同批落到 813**、**L263 的清单行 +1 而那一轮 ⑬ 声称把 operative 改成 824、实际从未落地（L264 用 `git show 08c0617d4` 复核第 70 行那一格仍是 823 ⇒ A333 的裸行号形状实测到一次）**、**L264 的清单行 +1 ⇒ operative 按现量重钉为 825 并补上 L263 缺失的归因**、**L265 的清单行 +1 而零新 Backlog 行 ⇒ operative 与链尾同批落到 826**、**订正：L265 的记账批在 Backlog 区落了 +2（那一轮新立的那笔账与它的分隔空行）而句首 operative 那一格从未重钉，真值当轮就漂到 828，本轮按现量补上这一格归因**、**L266 的清单行 +1 而零新 Backlog 行 ⇒ operative 按现量重钉为 829**、**L267 的清单行 +1 与 A345 连带它的分隔空行 +2 ⇒ operative 当时重钉为 833，其记账批按现量改成 832（先钉的那一格是错的，L267 块 ⑮ 记着）**、**L268 的清单行 +1 与 A346 连带它的分隔空行 +2 ⇒ operative 与链尾同批落到 835**、**L269 的清单行 +1 与 A347 连带它的分隔空行 +2 ⇒ operative 与链尾同批落到 838**、**L270 的清单行 +1 与新立 A348 / A349 / A350 三笔连带它们的分隔空行 +6 ⇒ operative 与链尾同批落到 845**、**L271 的清单行 +1 与新立 A351 / A352 两笔连带它们的分隔空行 +5、日志块 +15 ⇒ 与链尾同批落到 851**、**L272 的清单行 +1 与新立 A353 连带它的分隔空行 +2（本轮日志块 16 行落在被指向的那一行之下 ⇒ 不参与平移）⇒ operative 与链尾同批落到 854**、**L272 修正账本批的新立 A354 连带它的分隔空行 +2（清单行与要点 ③④ 都是同行改写、要点 ⑭ 落在被指向那一行之下 ⇒ 均不参与平移）⇒ operative 与链尾同批落到 856**、**L273 的清单行 +1 与新立 A355 / A356 / A357 三笔连带其中两笔的分隔空行 +5（本块 17 行落在被指向那一行之下 ⇒ 不参与平移）⇒ operative 与链尾同批落到 862**、**L274 的清单行 +1 与新立 A358 / A359 / A360 / A361 四笔连带它们的分隔空行 +8 ⇒ operative 与链尾同批落到 871**、**L275 的清单行 +1 与新立 A362 / A363 / A364 / A365 四笔连带它们的分隔空行 +9（本轮日志块 23 行（标题 + 空行 + 要点 ①–㉑，本批现量）落在被指向的那一行之下 ⇒ 不参与平移）⇒ operative 与链尾同批落到 880**、**L276 的清单行 +1 与 A362 / A365 两格定级格改写（同行改写、Backlog 区净 0 新行；本轮日志块 12 行（标题 + 空行 + 要点 ①–⑩，本批现量）落在被指向那一行之下 ⇒ 不参与平移）⇒ operative 与链尾同批落到 881**、**L277 的清单行 +1 与新立 A366 / A367 / A368 / A369 四行入表（初稿四行不各带分隔空行 ⇒ Backlog 区净 +4，被表结构守卫判成断表、按既有形状补回四枚分隔空行 ⇒ 同批净 +8；日志块 12 行落在被指向那一行之下 ⇒ 不参与平移）⇒ operative 与链尾同批落到 890**、**L278 的清单行 +1 与新立 A370 / A371 / A372 三笔连带它们的分隔空行 +6（本轮日志块落在被指向那一行之下 ⇒ 不参与平移；两处同行改写净 0 行）⇒ operative 与链尾同批落到 897**；同一处散文行号在本行括号链里至此列出四十个值（781 / 786 / 789 / 790 / 791 / 792 / 795 / 796 / 799 / 802 / 805 / 806 / 807 / 810 / 813 / 825 / 826 / 829 / 832 / 835 / 838 / 845 / 851 / 854 / 856 / 862 / 871 / 880 / 881 / 890 / 897 / 902 / 907 / 912 / 917 / 926 / 931 / 936 / 940 / 945）），而这一形不带文件名、a184 的结构维与内容维都不量 ⇒ 已立 A333；**实测到一次**：补 886 之前名册只有 **29** 个值而散文写着「三十」，是 A333 那一族漂移的本仓现场样本，本轮按现量核对后名册 30 个值才与「三十」对上）。新立七条 **A306~A312**（同名覆盖并刷新 `created_at` 且空值抹掉旧值、两支只读端点会把注册表目录建出来、「默认注册表目录」仓内三份且端点那份与另两份不是同一个地方、登记过的数据集在产品面上删不掉且边没有产品级创建入口、`validate_dependencies` 不查环、422 英文与 4xx 中文双轨错误体、只有 `POST` 挂鉴权而前端没有送头的通道），**A305 本轮已修**故只立不待拍。**census 两把棘轮本轮动了三格**：新守卫两支 `parametrize`（`input_file` 四档坏值 / `registry_path` 两档坏值）值位都是内联字符串字面清单 ⇒ l97 的 `MEASURED_CALLS` **419 → 421**、`MEASURED_READABLE` **301 → 303**、盲面 **118 一格未动**；读数出自仓外探针 `l232_census.py` 直接 `import` 那把尺子逐处打印桶归属，不是按「字面量 ⇒ 可读」推的（L204 那次正是这么想当然写错过一回）；l98 从 l97 `import` 同两个常量故一并生效，其六档分布与豁免 `23` 原地绿。**另外三把按各自 docstring 的处方重钉，没有一把被改松**：`test_api_doc_field_hints_l223.py` 首跑红 1 格——我在 `docs/augmentor/API.md` 那格里反引号引了 `remove_dataset` / `unregister_dataset` 两个**根本不存在**的方法名（它们不存在正是 A309 的内容），而那把尺子要求提示里每个名字都绑得到证据 ⇒ 改自己的散文（去掉反引号与那两个名字），不动尺子；`l224` 首跑红 5 格全是「按实测回填」类；本轮新守卫首跑红 3 格，其中 2 格是**我自己把族过滤写宽了**（`"dependenc" in p` 同时命中另一族的 `GET /api/system/dependencies`，那是 L226 明确拒接的「可选依赖包诊断」）⇒ 收窄成前缀 `/api/system/dependency/`，并留一支注入臂（臂5）证明这次收窄是吃重的。注入五臂逐臂点名喊红（**1 / 4 / 1 / 1 / 1** 支，臂2 同时打红 l224 的三支 query 判据），被改的四个文件逐臂字节复原、收尾重新验盘并断言 sha256 与进场值相等、基线复跑 65 passed。门禁：**8621 passed / 11 skipped / 0 failed / 覆盖率 99.81%**（186.34 s；改前同一条命令是 8618 passed + 3 failed，两边总数一致 ⇒ 那三格全是重钉类而不是用例数变了）；前端 `tsc --noEmit` 与 `eslint . --max-warnings 0` 均 rc=0（首跑 typecheck 红 8 格：两份既有替身文件的 `vi.hoisted` 声明块里没有新键；lint 红 5 格：新用例里 5 处 `any` ⇒ 都是改自己的测试代码收口，没有一处放宽规则），`vitest run` **17 文件 / 206 → 214 支全绿** ⇒ 详见循环日志 L232
- [x] **L233** `5d821c32b` +（账本批即本条所在提交）`feat,docs(augmentor)`：**前端补上 `X-API-Key` 通道——收 A302 / A312（本轮不接新端点，消费面仍 67/75）**。用户拍板走 A302 的修法①：`web/src/services/api.ts` 一处通道（`API_KEY_HEADER` / `API_KEY_STORAGE_KEY` / `getApiKey` / `setApiKey` + 请求拦截器，留空即不送头 ⇒ 零配置照旧可用）、key 的输入界面落在「配置管理 → 写操作鉴权」卡（存 / 清除 / 已存标记，卡上写明这是明文存储、不是登录、也不替代 HTTPS，且刻意**不**报「校验通过」——本层没能力证明那份 key 是对的）、401 的指路加在 `apiErrorDetail` 这个**唯一漏斗**上（一处改动，21 个页面动作同时被指路，并把「存着却不对」与「根本没存」两种成因分开说；服务层不引入 antd，所以不写响应拦截器弹窗）。**规模是被探针更正的，不是推的**：75 条操作里 **24** 条挂 `verify_api_key`，其中 **21** 条正是前端已接线的形状（L231 立账时只数了备份那一族的 3 只，**低估了一个量级**），另 **3** 条挂门禁但还没接线（`translate` / `migrate` / `stream`），门禁里**没有一条 GET**。新立 `tests/unit/test_api_key_channel_l233.py`（**18 支**：规模 / 通道形状 / 口径 / 真链路四类），行为面仍在 `api.test.ts`（+7 支，共 **68**）与 `Settings.test.tsx`（+6 支，共 **14**），两面各钉一半。**l231 那一维的前提被本轮改掉**：「前端零通道」当场成了假话，按新世界重钉为只留后端门禁事实（**不是放宽**，前端通道不在那边重复钉一份）。**注入 14 臂逐臂点名喊红**，收尾逐份 sha256 与进场值相等、基线复跑 18 passed；**其中一臂真的在仓里留了残留**（第一版 harness 在 `finally` 复原时抛 `OSError [Errno 22] EINVAL` 死掉，`api.ts` 少了一行 `if (response?.status !== 401) {`），靠 1 支守卫红 + vitest transform 失败才发现，按原文修回并复验（`tsc` rc=0、101 支绿、numstat 65/10）⇒ 第二版 harness 加三件事：跑前把 7 份原样备份进 Temp、写后读回比 sha256 且 EINVAL 重试三次、逐臂即时打印。**这是 L218 那处「注入残留」同一缺陷类第二次发生**。文档四处（README / API.md / ARCHITECTURE.md / DEPLOYMENT.md）与页面两处口径全部**同行改写、`git diff --numstat` 复核净零行**（2/2、3/3、1/1、1/1、System.tsx 7/7）。门禁：**8639 passed / 11 skipped / 0 failed / 覆盖率 99.81%**（166.24 s，与 L232 的 8621 相减 `+18` = 新守卫 18 支）；前端 `tsc --noEmit` 与 `eslint . --max-warnings 0` 均 rc=0，`vitest run` **17 文件 / 214 → 227 支全绿**；census 两把棘轮一格未动（新守卫零 `parametrize`），l98 哈希槽只读第一本账故同样未动 ⇒ 详见循环日志 L233
- [x] **L234** `aafbcbcc4` + `1ebd2c8ae`（A314 / A315）+（账本批即本条所在提交）`feat,fix,docs(augmentor)`：**A279 第九批接线——系统那一族五条进页面，消费面 67 → 72/75；并把接线途中量出的三处产品缺陷（A313 / A314 / A315）同轮收掉**。`monitor` 与 `auto-test` 进系统状态页第五张卡（都只读、都**不挂**门禁），`migrate` 与 `stream` 进第六张卡（都写盘、都**挂**门禁），`validate-config` 进配置管理页新卡（只读、不挂门禁）——**至此全仓六只落盘产物端点早已在 L230 收齐，本轮收的是「系统运维那一面」**。`UNCONSUMED` 8 → **3**（只剩 `dependencies` / `translate` / `health-gate`）、已消费 67 → **72**（75 条的 **96%**）、`NO_ENTRY` 仍 **1**（`translate`，归 A283 的契约缺口）、`CLI_ENTRY` 7 条/7 名 → **2 条/2 名**（`health-gate` / `doctor`）；l233 立的那格「挂门禁但没接线」3 → **1**。**A313（同轮收）**：`stream` 的 `operation` 在两端不是同一份清单——CLI 写 `quality / dedup / clean / export`，端点认 `quality / dedup / clean / none`；`export` 被 argparse 收下后落进 handler 的 `else`，而那个分支只是 `lambda items: items`（**原样搬运**，名字承诺的导出从来没发生过），连既有测试的 docstring 都逐字写着「export 操作为直通处理」把这句谎话钉成了事实；反过来 HTTP 侧合法的 `none` 在 CLI 里被 argparse 直接拒（退出码 2）。修法是把 CLI 那份字面改成与端点同一份，**没有新插 Python 常数**——把权威搬进 `augmentor/streaming.py` 会在 `WRITER_FORMATS` 之后插行、平移账本 A147 那条活引用，而仓内既有惯例（l226 / l175 / l189）也是「CLI 留字面、守卫对账」。**A314（同轮收）**：全量门禁首跑那**唯一一支红**倒推出来的产品缺陷，不是设计出来的——`OSError: [WinError 1920] 系统无法访问此文件。: 'data\versions\current'`，触发点是 `create_version` 里那句「先看那一格有没有东西」；现场取证量到它是 WSL 在 drvfs 上留下的 **LX 链接**（`is_symlink()` 是 **False**、`os.readlink()` 报 *not a symbolic link*、只有 `os.stat` 抛 1920 ⇒ 「先判 is_symlink 再判 exists」这类重排**治不了它**），且链接目标写的是**整段相对路径** `data/versions/v_…` ⇒ 按自己所在目录解析成 `data/versions/data/versions/v_…` 而**恒断链**（成因是 `storage_dir` 的出厂默认就是相对路径）。**A315（同轮收）**：A168 残留守卫的范围欠账——`data/versions/` 装的不是产品结构而是用例跑出来的运行期产物（现量 **67** 个 `v_*` 目录，那份 LX 链接就住在里面），而原口径「往 `data/` 更深一层写不判」把它整片放过了；补进 `WATCHED` 之后实测代价是**泄漏 1 条用例**（1 499 条里），扩范围后全量 8 687 条**泄漏 0 条** ⇒ 与 L94 那次入库同形、**不需要白名单**。新守卫 `tests/unit/test_system_ops_wiring_l234.py`（**36 支**，五面：清单同源 / 能力真有差 / 哨兵是躲判决的 / 口径如实 / 真链路形状）与 `tests/unit/test_version_current_link_l234.py`（**10 支**，四面：连创两版不炸 / 目标是裸名 / stat 抛 1920 也不许炸 / `.txt` 不会停在旧版本上）。**注入取证十三臂逐臂点名喊红**（接线六臂 + 前端三臂 + versioning 与残留守卫四臂，后四臂走零写入通道），仓库前后哈希相同（`cc1a80b8…` 与 `c17d1224…` 两次各验一遍）。门禁：**8687 passed / 0 failed / 11 skipped / 覆盖率 99.83%**（182.41 s；与 L233 的 8639 相减 `+48` = 接线守卫 36 + l233 新棘轮 1 + versioning 10 + l94 新棘轮 1）；前端 `tsc --noEmit` 与 `eslint . --max-warnings 0` 均 rc=0，`vitest run` **17 文件 / 227 → 18 文件 / 248 支全绿**（`+21` = System.ops 13 + Settings 4 + api.test 4）；census 三数（421 / 303 / 118）一格未动（两份新文件零 `parametrize`）⇒ 详见循环日志 L234
- [x] **L235** `e7c39adc1` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 第十批接线——`health-gate` 进质量中心，消费面 72 → 73/75；同轮量出 A319 / A320 / A321 三条新账**。质量中心从八项能力变成九项：新卡「健康门禁」把 `POST /api/quality/health-gate` 的三面旋钮全接上（文本字段、四个健康分权重、`pass_rate` / `duplicate_rate` / `completeness` 三档阈值、`block_on_warning`），并让 `/api/quality/evaluate` 的 `pass_rate` **一键喂进门禁**——那一格刻意不用 `||` 哨兵，因为 evaluate 真会回 `0.0`，而 `0.0` 是判决不是「没填」（新守卫与渲染面各有一支专门钉这件事）。`UNCONSUMED` 3 → **2**（只剩 `dependencies` / `translate`）、已消费 72 → **73**（75 条的 **97%**）、`CLI_ENTRY` 2 条/2 名 → **1 条/1 名**（只剩 `dependencies` → `doctor`）、`NO_ENTRY` 仍 **1**；这一条**不带 query** ⇒ `params` 那一维一个数没动（仍 11 条 / 8 键）。新守卫 `tests/unit/test_health_gate_wiring_l235.py`（**33 支**，五面：页面初值＝契约默认 / 服务层只有一份权威 / TS 面＝pydantic 面 / 判决是页面说的那样 / 散文如实）与渲染面 `web/src/pages/Quality.healthgate.test.tsx`（**13 支**）。**A319**：四个权重既不校验非负也不校验上界，`health_score` 也不钳 [0,1]——实测 `weights=[2.0,-0.5,-0.25,-0.25]` 把健康分顶到 **1.1698**（`level` 仍 `healthy`），而门禁那三条规则**根本不看权重** ⇒ 同一份回执里「healthy」与「verdict=failed」同时为真。**A320**：权重之和的容差写的是 `abs(sum-1.0) > 0.01`，而 `[0.24,0.25,0.25,0.25]` 的浮点差是 **0.010000000000000009** ⇒ 0.99 与 1.01 两档**都被拒**、`[0.255,…]`（和 1.005）反而放行，「±0.01 以内算 1.0」这句在恰好 0.01 那一档是假话。**A321**：`text_field` 送空串或送一个数据集里不存在的键，后端**零判决**、`duplicate_rate` 一律回 **1.0** ⇒ 与「真的全重复」在响应里同形，而健康分那四项压根不看 `text_field`。三条都写在页面那张「读这一页之前要知道的六件事」里，不是只写在账本里。**守卫首跑红 1 格是我自己的尺子范围错**（`assert "0.25" not in 整页` 被页面里那句如实解释后端默认的文案判红）⇒ 按「扫描面与禁令面同族」收窄到 `healthGate(file, {…})` 调用点区块，并留一支反证（`pass_rate` 那一格确实设了 `min={0}` / `max={1}`）免得禁令恒真。**渲染面首跑红 3 格全是判据与取证问题**：`InputNumber step={0.05}` 把 0 显示成 `'0.00'`；那句「健康分本身不参与门禁判决」住在卡上的 Alert 而不在回执 div 里；第三支反复红的根因用两份一次性 `Dbg.test.tsx`（建完同一条命令里删掉）取到——antd 会在按钮里留一只 `ant-btn-loading-icon-motion-leave-start` 图标（自带 `aria-label="loading"`），jsdom 不放动画 ⇒ 可及名**永久**变成「loading健康门禁」，精确名查询恒失败，改按正则取（仓内 `DataManagement.transform.test.tsx` 早有同一惯例）。**注入取证十臂逐臂点名喊红**（Python 侧八臂 + 前端两臂，都走字节级注入；被改的三份文件逐臂复原并断言 sha256 与进场值逐份相等、基线复跑全绿），其中前端那两臂第一版判据是**恒真的**（「用例名出现在 vitest 输出里」——vitest 会把全部 13 个用例名连同 ✓/× 都打出来），改成「`FAIL` 行点名 + 失败计数恰好 1」，而 `FAIL` 行第一次匹配不上是因为**输出带 ANSI 色码**。**读数**：门禁 `PYTHONIOENCODING=utf-8 …python.exe -m pytest tests -q -p no:cacheprovider` ⇒ **8721 passed / 0 failed / 10 skipped / 覆盖率 99.83%**（285.65 s；与 L234 的 8687 / 11 相比 passed `+34`、skipped `-1`，而**收集总数 8698 → 8731 恰好 `+33`** = 新守卫 33 支 ⇒ 那 `-1` 是一格 skip 变成实跑，已归因到 `test_serve_config_wiring_l97.py:119`：它的跳过条件正是「`web/dist` 不存在」，而 `web/dist` 在 22:45 出现了（本会话只跑 typecheck / lint / vitest，都不产出 dist ⇒ 是并发会话的构建），属环境读数不是产品面变化）。前端三门禁：`tsc --noEmit` 与 `eslint . --max-warnings 0` 均 rc=0，`vitest run` **18 文件 / 248 → 20 文件 / 276 支全绿**（`+28`：本轮那一支文件 13 支，另 15 支来自并发会话新增的 `api.optional-defaults.test.ts`，该文件此刻仍在被另一会话改写、单独复跑读 14 支，所以那个 15 是全量跑当时的读数，不追平）；census 三数（421 / 303 / 118）一格未动（两份新文件零 `parametrize`）。`README.md` 的页面表那一行同行补上健康门禁（`git diff --numstat` 1/1 净零行）⇒ 详见循环日志 L235
- [x] **L236** `890965d32` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A279 收口——最后两员接完，消费面 73 → 75/75（100%）；同轮收 A283①（translate 的落盘口契约）、新立 A322**。① `POST /api/dataset/translate`：先给契约补上 `TranslateRequest.output_file` 与 `TranslateResponse.output_file`（都可选，键**恒在**、没给时为 `null`——用「键在不在」区分不了「没落盘」与「落了盘没回显」，所以两态都要可读），再把「批量翻译」卡接进数据管理页（输出文件名 / 目标语言 / 字段名 / 保留原文四格 + 计费确认闸），失败回执四键（`failed` / `failed_indices` / `errors` / `errors_truncated`）逐格可见。**顺序是量过的：落点先 `resolve_data_path(for_write=True)`、再动模型**，越界落点 403 出声时模型调用 **0** 次（注入臂实证：摘掉白名单判决之后那一份响应体里赫然是根外绝对路径，且三次模型调用都花了）。② `GET /api/system/dependencies`：把依赖诊断从系统状态页那道「综合状态取到才渲染」的整体门里摘出来，改成自己取数、卡上自己一只刷新按钮，并留 `deps ?? status?.dependencies ?? null` 这条退路。**取法是论证过的，不是顺手接一格指标**：两条端点的诊断逐格相同（同出 `augmentor.diagnostics.check_dependencies`），但 `/api/status` 要真构造管道——把 `api.routes.status.get_pipeline` 换成抛异常的替身之后，dependencies 读数一格未变、status 退成 `model_default='unknown'` / `model_available=False`（两支都仍 200）⇒ 管道拉不动时这一面没有理由跟着黑掉，退路也不是空的（两面数据相同是实测）。`UNCONSUMED` 2 → **0**、已消费 73 → **75**（**100%**）、`CLI_ENTRY` 1 条/1 名 → **0 条/0 名**（`doctor` 整名出名单）、`NO_ENTRY` 1 → **0**；l233 那格「挂门禁但没接线」1 → **0**（**24 / 24**），`Settings.tsx` 卡片上那句影响面口径同轮从 24/23 改成 24/24（守卫用 f-string 绑死，改不动就红）。新守卫 `tests/unit/test_last_two_wiring_l236.py`（**18 支**，四面：页面初值＝契约默认 / 落盘口 / A322 / 系统页自取依赖诊断）与渲染面 `web/src/pages/DataManagement.translate.test.tsx`（**9 支**），System 那一份 7 → **10** 支。**L235 挂着的那句「收口轮必须复验空集相等时仍会响」已复验**：l224 那把尺子带两支牙齿（`test_the_empty_claims_are_not_a_blind_ruler`、用 monkeypatch 摘掉一处调用形状的 `test_unwiring_one_operation_turns_the_empty_roster_red`），注入第五臂把 l233 的 `GATED_NOT_YET_CONSUMED` 塞回一条也当场红 ⇒ 收口那一步没有把尺子读成恒绿。**A322 本轮新立、后端一行未改**：`text_key` 那一格零判决——空串或幽灵键都回 **200**、模型调用 **0** 次、`failed=0` / `failed_indices=[]` / `errors=[]`，产物凭空多出**三**键（那一列、它的 `_original`，还有一个 `language` 目标语言戳），而原字段一个字没动。**第三键是本轮首跑量出来的新事实**：守卫首跑 **2 failed**，多出来的那一格是 `language`，源头是 `augmentor/multilingual.py:190` 那句无条件盖戳（探针当时只报了「多两键」）⇒ 「200 且 failed=0 看起来像全成功」这条风险从「回执全零」升级为「回执全零 + 产物带语言戳」，四处同轮写清（守卫 / 页面卡片 / 前端用例 / `docs/augmentor/API.md`）。**注入取证五臂逐臂点名喊红**：前三臂走**零写入通道**（pytest 插件在导入后的模块对象上打补丁：`atomic_write_json` 变哑 ⇒ 4 支红、`resolve_data_path` 去掉白名单判决 ⇒ 越界那一支红、l233 的空尾巴塞回一条 ⇒ 1 支红），后两臂必须动源码故走字节级注入、**复原后重新验盘**（`disabled={!trAck}` → `disabled={false}` ⇒ vitest 1 支红；`{diag && (` → `{status && (` ⇒ vitest 1 支红 + Python 静态那一支红；两份 sha256 与进场值逐份相等）。**中间读数不美化**：第二臂那个注入器首跑在**打印环节**被 gbk 打断（`UnicodeEncodeError: 'gbk' codec can't encode '\u23af'`），finally 已复原并验盘（`System.tsx:505` 仍是 `{diag && (`），单臂重跑才取到读数。文档三处（README 页面表两行 + HTTP API 段一句、`docs/augmentor/API.md` translate 说明行）全部**同行改写**，`git diff --numstat` 复核 **3/3 与 1/1 净零行**，故本账那条 `README.md:65` 引用仍指得准（第 65 行仍是 `/system` 那一行）。门禁：**8741 passed / 0 failed / 10 skipped / 覆盖率 lines 99.83%、branches 99%**（363.76 s；与 L235 的 8721 / 10 相比 passed `+20`，收集总数 8731 → **8751** 恰好 `+20` = 新守卫 18 支 + 并发会话 `8041df804` 给 `test_version_current_link_l234.py` 补的参数化 2 支）——**这一跑不带 `--ignore`，L235 记下的那笔欠账已还清**（那个文件已被其作者修好并提交）。前端三门禁：`tsc --noEmit` rc=0、`eslint . --max-warnings 0` rc=0、`vitest run` **21 文件 / 287 支全绿**（`+11` = 本轮 translate 9 支 + System 3 支 − 并发会话 `8041df804` 在 `api.optional-defaults.test.ts` 删掉的 1 支；276 + 12 − 1 = 287，账对得平）；census 三数（421 / 303 / 118）一格未动（两份新文件零 `parametrize`）⇒ 详见循环日志 L236
- [x] **L237** `043ee69d3` +（账本批即本条所在提交）`feat,docs(augmentor)`：**A281 收口——三只 TS 响应类型补齐 12 处缺口 + 建「TS ↔ OpenAPI 契约」两层跨语言守卫（21 支）+ 补两处渲染**。A281 立账写的是 3 处，本轮把尺子做深一层重量实测 **12 处**（`AppConfig` 缺 9、`StatusResponse` 缺 2、`UpdateConfigResponse` 缺 1，反向幽灵 0）——立账那把尺子没错，只是浅一层：`ConfigResponse` 九个节在契约里全是 `additionalProperties: true` 的自由字典、**没有 `properties`**，节内 7 键根本不进契约 ⇒ 守卫第二层必须打只读的 `GET /api/config` 递归比键树。新守卫 `tests/unit/test_ts_contract_types_l237.py`（**21 支 / 5 面**：尺子先自证、fetcher 按**操作**配对、自由字典面走活响应、改前读数复现、渲染面认领类型新带的那些格），**75 只**带类型 fetchers 第一层全数相等、`unannotated` 与 `unresolved` 皆空集，`UNMEASURABLE` 空集（立账时点名的两只「naive 正则解析不动」`AugmentProgress` 联合别名与 `VersionDetail extends` 在新口径里都解析动了），路径配对按**段级**规则收敛（`/versions/${versionId}` 唯一命中 `{version_id}` 而排除 `/versions/history`）。渲染面同轮补：系统状态页「模型重试次数 / 重试等待」两格（`null` ⇒ 显式「无后端」，不折成 0），配置管理页「其余节的生效值（本页只读）」与「上次保存的回执」两张卡（后者渲染后端真回的 `message` 与逐条 `ignored_keys`）。**本轮零对外契约改动**（只动 TS 与页面）⇒ l200 契约数字守卫原地绿。注入取证 11 臂：守卫内 5 臂走内存字符串手术（零写入），渲染面 6 臂字节级注入、逐臂 sha256 复原（S1=1 / S2=2 / T1=1 / T2=1 / T3=4 / T4=2 支红；T1 只红 1 支太窄所以补 T4）。**新立 A323**（`handleSave` 的 toast 是第三处措辞、不带后端的 `ignored_keys` 判决）只立账不改，可达性已按实测收窄：`{}` 那一档从本页**不可达**（页面恒送 `default_model`）⇒ 详见循环日志 L237
- [x] **L238** `88837aa2a` + `049dea678`（api.ts 两处注释跟上后端新判决）+（账本批即本条所在提交）`fix,test,docs(augmentor)`：**A319 / A320 / A321 / A322 「判决缺失面」四条收口——同一条判据从两份合成一份**。权重族共引 `validation.require_ratio_list`（逐项 [0, 1] 点名 `weights[i]`、出口 `min(1.0, max(0.0, …))` 钳进 [0, 1]、容差抽成 `WEIGHT_SUM_TOLERANCE` 且量化到 9 位使「±0.01 含端点」成真尺子）；字段存在性新建 `validation.require_item_field`，health-gate / impact / translate 三面共引 ⇒ 空串、纯空白、幽灵键一律 400 并带回首条实际字段，`text_key` 那一刀走在模型与写盘之前（0 次调用、0 字节落盘）。新守卫 `test_decision_gates_l238.py`（30 支 / 5 面）专量行为面测不出的那一维：判据身份 `is`、判序（400 先于 404）、两面同文、反手抄与迟到绑定哨身。全量 **8802 passed / 0 failed / 11 skipped / 99.83%**。⇒ 详见循环日志 L238
- [x] **L239** `1c5cdb0b6` +（账本批即本条所在提交）`test,docs(augmentor)`：**A286 收口 + 3.12 解释器名册现量钉版 + 性能热点现量入账（零对外行为变化的一轮）**：① `test_parametrize_blind_face_l98` 的 `MEASURED_HAZARD_ROSTER` 补 `(3, 12)` 档，按 uv 托管 cpython **3.12.14 整跑 8813 条采集现量**钉版——读数与该版上 `dynamic_risky` 的整跑逐字相同，且与 `(3, 13)` 那格**同串**（`typing._UnionGenericAlias:Union`）⇒ 原来那句「同一份源码在两版上读数一样就是有把尺子失效了」只对 **(3,13) vs (3,14)** 成立，劈叉发生在 3.14（PEP 604 之后别名换了类型名），据此把 `test_hazard_roster_is_pinned_per_interpreter` 的 docstring 从「两版」改成「逐版」、并把那条「不许两两相同」的推断显式限死在 3.13/3.14 之间；3.12 上那格 skip 消掉 ⇒ **skipped 11 → 10**。② **A286**：比值型计时守卫按立案时给的处方 ② 收口——「数内层重算次数」取代墙钟比值，`_calculate_diversity_score()` 整趟 `builtins.sum` 调用数在 n=1000/2000/10000/20000 **四档同为 2**（`MEASURED_SUM_CALLS`），判据不再看这台机器此刻给不给 CPU；`RATIO_CEILING`、`MIN_MEANINGFUL_MS` 与那条把阈值冻成断言的用例一并退役（**撤下不是放宽**：统计判据换成精确判据），保留 20 倍松的绝对兜底与「计时把手」自检；牙齿三支：复刻退化档现量 `n+1`（5001/10001）、空档早退 0 次、替身必复原。三臂字节级注入取证全部点名喊红且逐字节复原（`analytics.py` 进出 sha `13e78b589e8d` 一致）。③ 20k 合成条性能现量入账并立 **A325 / A326 / A327**（最大一格是 `LeakageDetector.detect` **45.4 s**，41.0 s 落在 `_collections._count_elements` 的 9988 次调用上），以及 einsum 向量化**量化后否决**（快 3.73× 但产物不位相同）。前端三门禁**不适用**（`web/` 零改动）。⇒ 详见循环日志 L239
- [x] **L240** `807a2beec` +（账本批即本条所在提交）`perf,test,docs(augmentor)`：**A325 收口——泄漏近似计数加一条「按代价选路 + numpy 0/1 矩阵乘法」稠密档，判决逐条不变**：10k×10k 塌缩档从 **48 929.0 ms** 降到 **1 742.1 ms**（同一次运行内 auto vs 强制倒排 **28.1×**），六档形状的报告 `to_dict()` 与倒排**逐字节相同**；选路判据照 `dedup.py` 的「两条独立账」纯整数（时间账 摊平次数 × 单价 1000 vs 乘加格数、内存账 n_train × vocab × 4 ≤ 64 MiB），判决链条靠「`ov` 是 float32 精确整数 + `best >= t` 等价于 ∃候选 J ≥ t ⇒ 剪枝不承重 + argmax 在 float64 选 + 最终 J 仍由 `_jaccard_from_counts` 现算」四步。A325 原案处方 ①（长度带剪枝）实测只省 **53.7%** ⇒ 量化后否决，走处方 ③；铺开档（random5）自然判负给倒排、判负之后仍慢 **8%**（块化簿记是两条路共用的代价，如实入账）。新增守卫 `test_leakage_dense_route_l240.py` 29 支（**全计数判据，不看墙钟**）；两支 l95 尺子（内存上界、剪枝牙齿）被新判据路由成空转，按本轮点名被测路线强制倒排、阈值一字未动。八臂字节级注入取证 **7 臂红**（去剪枝 / 判据恒真 / 恒假 / 旁路权威算式 / 错位回填 / 块大小不看列数 / 删内存账），A6「argmax 换 float32」**不红**——float64 那一手只管并列选择、不承重，如实记为阴性臂并把真正把手换成「稠密不许旁路 `_jaccard_from_counts`」的计数钉子。普查按现量重钉 CALLS 426→429、READABLE 308→311、盲面 118 未动；`leakage.py` 净 +196 行把卷一 L55 的 `leakage.py:124` 平移，按处方 3 重钉为 `leakage.py:352` 并写明归因。全量门禁 **8835 passed / 0 failed / 10 skipped、99.84%**（= L239 基准 + 本轮新立 29 支）。前端三门禁**不适用**（`web/` 零改动）。⇒ 详见循环日志 L240
- [x] **L241** `9d417eda7` +（账本批即本条所在提交）`test,docs(augmentor)`：**代价预言机选路家族入册——文档 §3.34 + 跨家族机械守卫 24 支，零对外行为变化**：`docs/augmentor/ARCHITECTURE.md` 新增 §3.34（两条独立账的定义与四个单价常数、收益 8.1×~32.0× / 10k×10k **28.1×**、铺开档判负之后仍慢 8% 如实入账、「一条从注入里长出来的纪律」）与 §7.1 第四颗子弹（性能判据一律计数，L239 负载差 46% / L240 −18% 两轮正面证据）；新守卫 `tests/unit/test_cost_oracle_routing_l241.py` 24 支钉五件事——文档 §3.34 反引号点名的预言机名与 AST 实推集合**两个方向精确相等**（刻意不用 L76/L77 那套只降不升的棘轮，否则「新增路线却不写文档」那一半被放过）、三条独立通道（名字形状 / 比较式里的 `*_COST` / `-> bool` + 乘法比较 + 同模块 `if`-test 消费）各自实推必须给出同一集合、判据本体零浮点/零调用/零属性访问、每支预言机的调用点坐在 `if` 的 test 位且两侧生产者同住一个函数、每支都在 tests 树里有把它焊死的 `setattr` / `patch` 接缝。第三通道先量过误伤面（全包只命中那两支）才采用。六臂真文件注入取证**全部按预期红**，逐字节复原后复验哈希。普查按现量重钉 l97 CALLS 429→430、READABLE 311→312、盲面 118 未动；l98 豁免用数 23→24（涨它不等于放松：四档 58/38/11/11、盲面 118 与名册 113 一格未动）。产品码一字未改 ⇒ 契约数字守卫一格未动、前端三门禁不适用；全量门禁 **8859 passed / 0 failed / 10 skipped、99.84%**（= L240 基准 8835 + 本轮新立 24 支）。⇒ 详见循环日志 L241
- [x] **L242** `16b5bf4c6` +（账本批即本条所在提交）`test,docs(augmentor)`：**Backlog 表关闭标记与进度声明面对账——量出 20 笔「说收了却没标」、补标 17 格、新守卫 24 支、新立 A328（定级 M 待拍）**：A265 那把尺子管「一轮写两遍自己」，账本的第三面（Backlog 表定级格）此前**没有任何机械通道**——开工前实测欠账 20 笔 = 17 真漏标 + 3 合法半程。口径全部现量选出：ID 认到一位数（三档读数 gap 11 / 20 / 20，账目面 190 / 234 / 247 行）、声明面只收清单行与日志标题行（宽窄口径同 gap）、「全清」与划掉的数字一律不算标记（A298 / A176 现场反例）、判据写成**名册精确相等**而不是棘轮。同轮重钉 `test_doc_line_refs_l79` 的结案口径 **107 → 119**（+12 归因：14 格里 2 格行内本已含删除线；行首口径 87 未动，差值 20 → 32）。本轮零对外行为变化：产品码一字未动，改动面 = 1 支新守卫 + 1 处重钉 + `docs/augmentor/ARCHITECTURE.md` +2 行 + 两卷账本 18 格。
- [x] **L243** `afc702eff` +（账本批即本条所在提交）`feat,test,docs(augmentor)`：**备份完整性那一族接上电——A299 的校验和闸恢复成 409 且零写入、A300 的两只 SDK 函数有了 HTTP 入口与页面按钮**：`DatasetBackup.restore()` 里那句「只 `logger.warning` 然后继续恢复」换成抛 `BackupIntegrityError`，端点显式接住回 **409**，实测输出文件**不存在** ⇒ 坏备份不再灌进目标路径；同轮新增 `GET /api/system/backups/verify`（只读、不挂鉴权、目录缺失时**不建目录**、幽灵 id 回空列表、零字节算丢）与 `POST /api/system/backups/cleanup`（写操作、挂 `verify_api_key`、`max_backups` 下限 1，0 与负数 400 并回 `require_count` 原话、保留数按**索引行数**算），页面「数据备份」卡各配一只按钮（校验「点一次查一次」不随列表自动刷、清理带 Popconfirm 与 `min={1}` 的数字框）。新守卫 `test_backup_integrity_l243.py` **32 支 / 七面**（第六面是判决矩阵，第七面是两只新端点的收尾臂：越界 ⇒ 403 原样重抛两支、未分类异常 ⇒ 500 且响应体不含原文一支——这三支是第一轮门禁量出覆盖率倒退之后补的，见日志块 ⑫⑬）+ 三臂零写入注入取证（摘校验闸 / 摘参数校验 / 摘线程池），每臂指名它要红的那一支。按现量重钉六把尺子：l231 备份名册 4 → 6 员、l224 消费面 75 → **77** 条操作 / 71 → **73** 条路径、l97 普查 CALLS 430 → **431** 与 READABLE 312 → **313**（盲面 118 未动，探针 Counter 为证）、`test_api_security` 的 `PROTECTED` 与 `OPEN_ALLOWLIST` 各进一员、l233 挂门禁规模 24 → **25**（已接线同数，`Settings.tsx` 卡上那句影响面与 `Settings.test.tsx` 同轮跟上）、l162 的读侧哨兵由 `JSONDecodeError` 换成同为 `ValueError` 子类的 `BackupIntegrityError` 并补「输出文件不存在」一维。账本三处历史行号引用按现量重钉（`System.tsx` 的 432 行改成 505 行，两处；旧落点改成 `docs/augmentor/API.md:1139`，三处；本账第 734 行改成第 773 行，两处——L244 复核时那条已漂到第 781 行，散文行号随插行漂、a184 不量这一形，已在原处按现量重钉），过程中查出 a184 的 `dead_line` 档量不到「落点对、内容错」——改前那句「引用仍指得准」是假话而尺子全程绿。新立三条待拍：A329（CLI 只四档）/ A330（契约不声明错误档）/ A331（引用尺子的内容盲区）。全量门禁 **8 922 passed / 0 failed / 10 skipped、99.84 %**，`TOTAL` 行 **14 332 语句 / 8 未覆盖 / 3 954 分支 / 22 未覆盖分支**（用例数闭合 = L242 基准 8 883 + 新守卫 32 + 契约面 +4 + 异常层级 +3）。前端三门禁 typecheck 与 lint 均 rc=**0**、**307 tests passed**。⇒ 详见循环日志 L243
- [x] **L244** `2a04e1ed3` +（账本批即本条所在提交）`feat,test,docs(augmentor)`：**CLI 备份那一族补上 verify / cleanup 两档，A329 收口——三面（SDK / HTTP / CLI）同源钉成双射而不是清单**：`augmentor/cli/parser.py` 的 `backup --action` choices 由四档补到六档，另带 `--max-backups`（默认 **10**，与 HTTP 请求模型、SDK 形参三方同数），`run_backup` 的 `verify` 是校验形命令 ⇒ 查到损坏经 `verdict_exit` 给退出码 **1** 且 stderr **没有**「错误:」（与崩溃那一档分得开），索引空或给幽灵名回「没有找到备份」并退 **0**，`cleanup` 的坏份数由 `require_count` 抛、经 `cli.py` 收尾成「错误: …」+ **1** 且**判参先于建目录**（实测目录不在场），不认识的 `--action` 由 argparse 给 **2**。新守卫 `test_cli_backup_actions_l244.py` **20 支 / 四面**（同源三个方向各自钉死、verify 判决五支、cleanup 档位四支、注入三臂），另以仓外探针做**六臂字节级注入取证**（A–F 逐臂 rc=1、逐字节复原、事后工作树干净）。量到一处真实的三面不对称：CLI 的只读档**会**把不存在的目录建出来并落 `index.json`，HTTP 面同一只 verify 不建 ⇒ 新立 A332（CLI 只读档的落盘面，待拍），不顺手改 SDK 构造。A329 立账那一格写的「当前处置」实测**两格都不成立**（全仓没有守卫钉 CLI choices、页面卡片也没有「CLI 没有这两档」那句），本轮补守卫而不是追认，与 A331 同族入册。按现量重钉三把尺子：l97 普查 CALLS 431 → **432** / READABLE 313 → **314**（盲面 118 未动，仓外探针逐字为证）、l242 表行 251 → **252** / 有标记 134 → **135** / 声明面 116 → **117**、`parser.py` 的两处历史行号引用按**内容**重钉（405 → 406、430 → 431，净 +1 行把它们平移了，而 a184 全程绿 ⇒ A331 那条盲区的第 2 次现量）。页面卡片与 `api.ts` 的「CLI 没有这两档」旧口径按现量改成六档。全量门禁 **8 942 passed / 0 failed / 10 skipped、99.84 %**，`TOTAL` 行 **14 353 语句 / 8 未覆盖 / 3 966 分支 / 22 未覆盖分支**、`112 files skipped due to complete coverage`（用例数闭合 = L243 终态 8 922 + 新守卫 20）。前端三门禁 typecheck 与 lint 均 rc=**0**、**307 tests passed**（本轮前端只改断言与注释，不加支）。⇒ 详见循环日志 L244
- [x] **L245** `34c3e89f9` +（账本批即本条所在提交）`test,docs(augmentor)`：**收 A331——给 a184 的 `dead_line` 档接上内容维，12 条「落点对、内容错」的活引用第一次被尺子点名**：新判据 `anchor_state` 取引用旁「紧跟的后括弧」或「所在的不含嵌套外括弧」，括弧**第一个**反引号跨度必须是裸标识符、必须是目标文件某个 `def` / `class` 的名字，且落点跨度完整落在该符号的某一跨度内，否则开火新档 `dead_anchor`（硬 0，`CEILING` 等值）。判据口径由三档归因实验量出（仓外六支探针直接 import 守卫函数现量读数）：整行归因要背 **3 229** 个反引号标识符、产品注释面「整行名字全命中」只有 **15/48**；句内归因造出 **6 条假红**（点名的多是模块级常量与键名，不是 def/class）；收成括弧首才得到 **24 条主张 / 12 绿 / 12 红** 的干净案面。12 条红逐条按 AST 现量重钉（search_enhanced 308-360 ⇒ **491-553**、dedup 607-653 ⇒ **627-672**、data_ops 40-50 ⇒ **42-55** 与 88 ⇒ **92**、config 807 ⇒ **973** 与 176-196 ⇒ **703-784**、config_validator 449-467 ⇒ **880-900**、logging_setup 132 ⇒ **163** 两处与 95 ⇒ **108**、streaming 274 ⇒ **296** 两处），另 3 处尺子判不到的形状由人眼补（调用点那一格 1005 ⇒ **1163**、ARCHITECTURE 的 ModelConfig 15 ⇒ **176** 与 `AugmentationConfig` 归属 28 ⇒ **249**）；**全部只在行内换数字**，`git diff --numstat` 复核 10/10、2/2、1/1 净 0 行，不触发插行平移族。新判据 5 支：硬 0 一档、`TestContentDimension` 三支（①② 钉「2 主张恰好 1 红」且按**增量**比、③④ 钉「判不到是真判不到」、⑤ 钉空行落点只开火 `dead_line` 不重复开火）、尺子自证一档。新立 **A333**（内容维判不到的三种形状：点号主张、括弧首是散文、不带文件名的裸行号；已知样本是卷一 A95 那一格，本轮人眼核对那三个数字全部陈旧）。按现量重钉 l242 三格（表行 252 → **253** / 有标记 135 → **136** / 声明面 117 → **118**），l97 与 l98 的普查读数**一格未动**（本轮零新 `parametrize` 调用点，432 / 314 / 盲面 118 由全量门禁原地绿证实）。全量门禁 **exit=0 / 8 947 passed / 0 failed / 10 skipped / 6 warnings / 282.59 s**、`TOTAL` **14 353 语句 / 8 未覆盖 / 3 966 分支 / 22 未覆盖分支**、**99.84 %**（用例数闭合 = L244 终态 8 942 + 新判据 5 支）；前端三门禁**不适用**（`web/` 零改动）。⇒ 详见循环日志 L245
- [x] **L246** `669e0132b` +（账本批即本条所在提交）`perf,test,docs(augmentor)`：**收 A327——多样性那一格的行范数改为随参照集编码同批刷新，四档判决产物逐字节相同**：`QualityScorer._calculate_diversity` 把 `np.linalg.norm(_existing_embeddings, axis=1)` 写在逐候选的那条除法里 ⇒ n 条候选就把 sample_size×dim 的平方和付 n 遍；改后行范数只在参照集那一次编码的同一处刷新，**不另设失效判据**（它是 `_existing_embeddings` 的纯函数，寿命同它），`reset_cache()` 同清两格——这一句正是 A327 原格点名「动手留给能把缓存失效边界说清楚的那一轮」要交的账。现量（仓外 FakeModel 顶掉 sentence-transformers，n=4000、`include_semantic=False`；axis=1 的次数是精确读数、wall 是单次）：窗口 30 / 维 64 的参照集稳定档 **4 000 → 1 次**（407.7 → 376.6 ms，−7.6%）、参照集逐条达标档 **4 000 → 1 次**（406.8 → 359.8 ms，−11.6%）、生产第一批形状（参照从 1 涨到窗口 30）**4 000 → 30 次 = 编码次数**（430.4 → 404.8 ms，−6.0%）、真实语义模型量级（窗口 100 / 维 384）**4 000 → 1 次**（459.4 → 396.6 ms，−13.7%）；四档判决产物 4×4 000 格的 `repr`**逐字节相同**（sha256 前后同为 `798206eed79354a5…`），原格里 `np.einsum` 换 `np.dot` 那一刀**维持否决**。**新守卫 8 支**（`tests/unit/test_quality_norm_cache_l246.py`）钉的是**次数**而不是数值——数值一路正确、缓存命中与否是纯内部事实，没有任何尺子会出声；另钉失效边界随参照切片判据走、`reset_cache` 同清两格、逐候选的向量自身范数不许被一起省掉，并把向量矩阵那一格的注入接缝从「填 `_existing_embeddings` 一格」改成「两格同填」点名钉住。四臂字节级注入取证（挪回逐候选 / 缓存不刷新 / 范数 ×2 / 轴向 axis=0）分别咬中**4 / 6 / 2 / 5** 条，探针 finally 复原并比字节（进入与退出同为 `21745677c1d0…`）。`quality.py` 按处方走「同行压缩 + 行内替换」压到净行数 **7/7 = 0** ⇒ 十余处 `quality.py:NNN` 历史引用不平移，七把账本与文档守卫未因此出一格红。顺带新立 **A334**（`_embedding_cache` 实例级无界驻留：4 批 × 1 000 条之后 **8 个条目**、矩阵 `nbytes` 合计**2 048 000**、dim 384 折算 **12 MB**，`reset_cache()` 一格不清）。l242 三格按红读数重钉（表行 253 → **254**、有标记 136 → **137**、声明面 118 → **119**），l97 / l98 的普查读数**一格未动**（本轮零新 `parametrize` 调用点，432 / 314 / 盲面 118 由全量门禁原地绿证实）。全量门禁 **exit=0 / 8 955 passed / 0 failed / 10 skipped / 6 warnings / 281.51 s**、`TOTAL` **14 354 语句 / 8 未覆盖 / 3 966 分支 / 22 未覆盖分支**、**99.84 %**（用例数闭合 = L245 终态 8 947 + 新守卫 8 支；`TOTAL` 语句 +1 就是本轮新增的那一次范数刷新，Miss / Branch / BrPart 三格与 L245 逐格相同）；前端三门禁**不适用**（`web/` 零改动）。⇒ 详见循环日志 L246
- [x] **L247** `4863e1357` +（账本批即本条所在提交）`perf,test,docs(augmentor)`：**收 A334——`_embedding_cache` 收成「只留最近一批」的有界 FIFO，四档判决产物逐字节相同**：`batch_score` 里那格整批向量缓存原本实例级、跨调用只增不减，而生产侧两个真读者（`augmentor/cli/commands/pipeline.py:43`、`augmentor/pipeline.py:557` 的 filter_by_quality）都是「逐 chunk、每批只评一次」的形状 ⇒ **一次都命中不了**，驻留是纯泄漏不是收益；API 侧每请求新建 scorer，本来就无害。现量（仓外 FakeModel、dim 64）：① 流式 4×1000 改前 **8 条目 / 2 048 000 字节 / 8 000 文本引用** → 改后 **2 / 512 000 / 2 000**，编码次数 128 → 128（零代价）；② 同批连评两次（200）**2 / 512 000 不变**、32 + 30 → 32 + 30（命中保住）；③ a_b_a_b 交错（200）4 → **2** 条目、124 → 128（**+4 次整批编码，本轮唯一代价**，产品侧没有这种形状）；④ `augmentor/comparison.py:237-238` 的 a、b 各一次 4 → **2**、64 → 64；① 那档折算真实语义模型 dim 384 是 **12 MB → 3 MB**。四档判决产物 `repr` 落盘**逐字节相同**（sha256 前后同为 `6f405bf0099ef1e9…`，487 191 bytes）⇒ 有界化只改「编码几次」，不改任何一个数。改法取 A334 原格修法 ①（有界化）而不是 ②/③：`while len(...) >= 2` 逐出**最旧**那个键，上限 2 耦合的是「一次 `batch_score` 最多插 orig/gen 两个键」这个事实 ⇒ 将来谁加第三个键，`test_same_batch_scored_twice_skips_both_batch_encodes` 当场红，不必再去静态数调用点；**per-call 作用域（每批清空）由注入臂 ㈢ 现场否决**——它打掉既有 `test_embedding_cache_exists_after_score` 与 l246 守卫 `[30, 4, 4]` 那一格等 **3** 支 ⇒ 跨调用命中是本仓已声明行为，只能有界化不能取消。`reset_cache()` 语义一格未动（A327 那条寿命边界原样保留）。新守卫 `tests/unit/test_embedding_cache_bounded_l247.py` **7 支**，钉**驻留三格读数（条目 / 文本引用 / 向量字节）与编码次数**两条独立判据（驻留是纯内部事实，数值一路正确时没有任何尺子会出声），命中与 miss 各一支点名。四臂字节级注入取证分别咬中 **4 / 5 / 3 / 3** 条，探针 finally 复原并比字节。`quality.py` 走「init 两行收成一行 + 注释块 9→6 行 + 插入 4 行」压到净 **12/12 = 0** ⇒ 十余处 `quality.py:NNN` 历史引用不平移。普查档位由仓外**零写入**插件走 a184 的合成语料通道现量取：新守卫的 **5 条限定仓内相对路径**主张全落 `live`，而同名的**裸名** `pipeline.py:557` 落 `ambiguous`（散名索引歧义）⇒ 主张一律限定路径；另一面要如实记下：a184 的四档精确棘轮由 `require_authoring_tree()` 把关、本机 `Temp/` 底料不在场 ⇒ **本轮不 enforce**，「守卫没红」只证明不平移、证明不了逐条被判 live。l242 三格按红读数重钉（表行 254 未动、有标记 137 → **138**、声明面 119 → **120**），l97 / l98 普查读数**一格未动**（本轮零新 `parametrize` 调用点，432 / 314 / 盲面 118 由全量门禁原地绿证实）。全量门禁 **exit=0 / 8 962 passed / 0 failed / 10 skipped / 6 warnings / 323.87 s**、`TOTAL` **14 355 语句 / 8 未覆盖 / 3 966 分支 / 22 未覆盖分支**、**99.84 %**、`112 files skipped due to complete coverage`（用例数闭合 = L246 终态 8 955 + 新守卫 7 支；`TOTAL` 语句 +1 = 本轮「`hasattr` 两行收成 `getattr` 一行」减 1 条、「`while` + `pop`」加 2 条，Branch / Miss / BrPart 三格与 L246 逐格相同 ⇒ 没有留下死支）；前端三门禁**不适用**（`web/` 零改动）。⇒ 详见循环日志 L247
- [x] **L248** `9ed11b369` +（账本批即本条所在提交）`perf,test,docs(augmentor)`：**收 A326——稠密那条复用选路那一趟建好的列号表，同一批文本少扫一遍字符，四档判决产物 sha256 逐字节相同**：`_similarity_blocks` 判稠密赢时把「逐字符扫一遍全部文本」付了两遍（选路那一趟 `_inverted_view` + 编码那一趟 `_fallback_encode` 的发现遍），本轮拆出 `_ngram_scan`（一趟扫描出「文档号 / 词条号 / 列号表」）与 `_entries_from_scan`（折叠成稀疏三元组），让两条路共用同一趟；`_inverted_view` 加可选 `scan` 入参、`_batch_encode` / `_fallback_encode` 加可选 `vocab` 入参（**不给就照旧自己扫**，语义零变），于是 `_batch_encode` 接缝形状从 `(texts)` 扩成 `(texts, vocab=None)` ⇒ `tests/unit/test_dedup.py` 的 4 处 lambda 注入同步（净 4/4=0 行）。现量（仓外探针 `D:/WORKSPACE/l248_probe.py`，方法级 wall + `tracemalloc` 峰值，min-of-1；**wall 只作背景，判据是 digest 与预言机入参名册**）：D1（20 000 条全重复，occ 24 万）选路+编码两步 910.4 → 463.4 ms（**−49 %**）、峰值 25.2 → **21.8** MiB；D2（20 000×14，「abcd」四字母）1 159.3 → 514.0 ms（**−56 %**）、26.4 → **21.5**；R1（6 000×60）1 242.0 → 794.6 ms（**−36 %**）、46.1 → 46.2（持平）；R2（3 000×420，判**倒排**赢、编码那趟压根不跑）4 884.4 → 3 018.8 ms（**−38 %**，省的全是选路那一趟）、230.5 → **228.7**。四档 dedup 判决产物落盘 sha256 **与基线逐字节相同**（D1 `e9722a89d092ebf6…` 129 001 B / D2 `be6c8ac9275a4737…` 197 103 B / R1 `76aeace0d4096f26…` 384 021 B / R2 `58b568468a3c4978…` 3 792 021 B），预言机入参名册四档逐格不变（(20000,22,1400120000,200) / (20000,16,1043803110,200) / (6000,1296,46610403,666) / (3000,1078389,1454818,1000)，判决三负一正）。**中途一次自我否决**：第一版让逐趟数组与列号表活过倒排那条 ⇒ R2 峰值 230.5 → **258.6 MiB（+12 %）**，改成「判倒排赢立刻两样一起松手、判稠密赢只留列号表」后回到 228.7——列号表正是稠密那条第一遍要建的东西，交过去是净省，数组留着是净亏。两条被测量否决的修法形状：「把倒排的出现数组直接散射进稠密矩阵」nnz 级 int64 索引数组可达矩阵字节的 ~5 倍，与 A19 / L10 记过的 **6.8×** 峰值否决同形；「只把 readings 与 fold 拆成两步」实测只值 ~5 %，落在噪声里。**新守卫 `tests/unit/test_scan_reuse_l248.py` 11 支**钉的是**趟数与身份**而非数值：两条路各自 `_ngram_scan` 恰 **1** 次、`_ngram_entries` **0** 次；`_batch_encode` 收到的表与扫描返回的表 **`is` 同一个对象**（只比内容会放过「当场另建一份」的假复用）；「写就炸 + 读计数」的只读列表明**读次数恰好等于每篇去重后二元组数之和**；`vocab` 复用与自建两条编码 `tobytes()` 逐字节相等；`vocab={}` ⇒ `(n, 0)`；四档预言机入参名册 + 判决形状三负一正各一支；`_ngram_entries` 与「扫描+折叠」逐元素相等；倒排那条在产出第一个块之前 traced current **2.86 MiB**（上限 8 MiB；手工复刻「判完路仍握着表」那一档实测 **21.83 MiB**，142 800 项 dict + 两份 int64 数组）。**五臂字节级注入取证**（`D:/WORKSPACE/l248_inject.py`）分别咬中 **1 / 1 / 1 / 4 / 1** 支：㈠ 发现遍无条件重跑、㈡ 表不交给稠密那条、㈢ 判完路不松手、㈣ 视图仍自己扫第二遍、㈤ 假复用（另建一份同内容新表，数值完全相同）。**㈠ 第一版尺子抓不住**：守卫原本只钉「不许写」，而 `if True:` 那一臂把 `vocab` 重绑成一张**新** dict、写的是自己的新表 ⇒ 恒绿；据此把判据升级成「只读 + 读计数」，㈠ 与 ㈤ 才各点名一支——这是本轮唯一一次「注入把尺子修硬」，记录不美化。探针 finally 复原并比字节（进入与退出同为 `8f6256d1b6e7…`），五臂跑完基线复跑 0 红。**插行平移处置**：`dedup.py` 净 **+29**（54/25），把卷一 A38 那格的 `augmentor/dedup.py:627-672` 平移出 `_find_similar_pairs_chunked` 现量跨度 ⇒ a184 硬 0 档 `dead_anchor` 报 **1** 处，并连带 `TestContentDimension` 的自证臂红（合成注入那一格被真案面挤掉首位）。处方按「换点 → 压回净零 → 才重钉」走到第三档：本轮新增两个方法加一条分支，压不回净零，遂**按内容重钉 627-672 ⇒ 656-701**、归因写进行内、只换数字不改行数（1/1 净 0），并把该文档行随代码批一起提交，好让代码提交自身守卫绿。A326 行内本行原写的 `dedup.py` 跨度 451-481 随收口重钉为现量 473-510。L232 清单行那句**不带文件名**的散文自指（`docs/augmentor/API.md:1139` 所在那一行）按现量 790 → **791** 重钉：同一处行号六轮六个值，而这一形 a184 的结构维与内容维都不量 ⇒ 仍挂在 A333 名下。l242 三格按红读数重钉（表行 254 **未动**、有标记 138 → **139**、声明面 120 → **121**），l97 / l98 普查读数**一格未动**（本轮零新 `parametrize` 调用点）。按现量更新 `docs/augmentor/ARCHITECTURE.md` §3.34 第 4 条（**同行追加、净 0 行**）：把「选路那一趟顺手把列号表交给稠密那条」写进「换路只决定谁来算」那一条。全量门禁 **exit=0 / 8 973 passed / 0 failed / 10 skipped / 6 warnings / 377.38 s**、`TOTAL` **14 363 语句 / 8 未覆盖 / 3 968 分支 / 22 未覆盖分支**、**99.84 %**、`112 files skipped due to complete coverage`（用例数闭合 = L247 终态 8 962 + 新守卫 11 支；`TOTAL` 语句 +8 / 分支 +2 = 本轮两个新方法与那两条可选入参分支，Miss 与 BrPart 两格与 L247 逐格相同 ⇒ 没留死支）；前端三门禁**不适用**（`web/` 零改动）。⇒ 详见循环日志 L248
- [x] **L249** `8ede5113e` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**收 A332 + A307——只读面的磁盘副作用摘在库里，三面（SDK / HTTP / CLI）一次性归零**：改前仓外探针量到 13 条只读通道里**只有 1 条**不建目录（`GET /api/system/backups/verify`），而它干净是因为路由里手写了一道 `if not resolved.exists()` 挡在构造之前——逐面挡板追不上新增的面，所以本轮摘来源：`DatasetBackup.__init__` 不再 `mkdir`、`_load_index` 索引不在场时交回内存空账（不再顺手落一份默认 `index.json`）、`DependencyManager.__init__` 不再 `mkdir`，父目录一律由写侧`atomic_write_json` 自建，故 L162 的原子写判据不松动而写边由两条收成一条。**两刀而不是一刀**的理由在备份那一族：只摘 `mkdir`，A332 的 CLI 现量「目录 + `index.json`」只少一半。同批实测否掉 L243 那格担心（cleanup 无债档仍 **200** `{0, 5}`，不再建目录）与 L44 证人的退役（坏参数不再留下目录 ⇒ 判参先后改由 AST 比 `require_count` 与构造的行号钉）。改后 13 条只读通道**全部**零残留、写面照旧按需建目录（副作用只是延后）。新守卫 `tests/unit/test_readonly_no_disk_writes_l249.py` 22 支十组判据（AST 零写 / 读边 `open` 只许 `'r'` / 只读名单由 `WRITING_MARKERS` 实推而非抄 / 逐面磁盘快照 / 读三遍幂等 / 写面陷阱自证 / HTTP / CLI / 判参先后），六臂注入逐臂点名喊红（b1 11 红、b2 10 红、b3 2 红、r1 9 红、o1 **只有 AST 那一支红**、d1 7 红），逐字节复原并断言哈希相同；同批重钉七处旧判据（`test_dependency.py::test_init`、`test_round68.py` 两支默认落点补一次真写、`test_backup.py::test_init`、l244 的 CLI verify 档、l243 的缺目录档、l232 的血缘档与短语名册、l162 的静态尺子改成走 AST——旧写法 `"atomic_write_json" in body` 会被我新加的注释喂饱）。三面文档同批改写：`docs/augmentor/API.md` 五行、`docs/augmentor/ARCHITECTURE.md` §3.33 那段判据先后、前端 lineage 卡片与服务层注释（`web/` 有改动 ⇒ 三门禁全跑：typecheck 静默通过、lint `--max-warnings 0` 通过、test **22 files / 307 tests**）。**A332 / A307 两行收成已收口，本轮零新立。**全量 **8 995 passed / 0 failed / 10 skipped / 6 warnings / 318.90 s**，`TOTAL` **14 359 / 8 / 3 968 / 22**、**99.84 %**（`lines-valid` 14 363 → 14 359 = 摘掉四句语句，逐格对得上）；七把账本与文档守卫 **178 passed / 8 skipped**；新账 **0** 条、收口 **2** 条（A332 + A307，同根同批）。
- [x] **L250** `e50deb0f5` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**收 A298 + A303——备份索引的同名写点由 `append` 收成「先摘同 id 的旧行再落新行」，同轮 A303 收口（走原格修法①，② 那一半拆给新立 **A335**（回执补 `name` 那一格，键集尺子与 TS 类型同批重钉））**：`DatasetBackup.backup()` 里 `self._index["backups"].append(info.to_dict())` 换成推导式——按 `backup_id` 摘掉旧行再落新行，于是同名第二次备份**更新**那一行而不是在它旁边再加一行；改后索引行数与磁盘 `*.json` 份数**一一对应**（新守卫与 l231 各钉一格）。**为什么走 ①**：第二次 `atomic_write_json` 覆盖的是同一个文件，被留下的旧行里 `checksum` / `item_count` / `file_size` 描述的是**已经不存在的字节**——那是死账而不是历史；读边按 id 去重（② 的一半味道）只是把死账藏起来。**② 未做**（让 `backup_id` 真的唯一）⇒ 同一秒内的两次**匿名**备份仍共享 `backup_YYYYmmdd_HHMMSS`，索引塌成一行、磁盘塌成一份，这一格由 `TestUnnamedBackupsInsideOneSecond`（冻结时钟）**钉成 tripwire 而不是静默消失**；③ 未做（回执加「本 name 已有 N 行」）⇒ ① 之后 N 恒为 1，那一格没有信息量。**读边一字未动**：存量 `index.json` 里 L250 之前写出的遗留多行照原样交回（`TestTheReadEdgeStillsHandsBackWhatIsThere`，样本是立账那两条实测时间戳 `.991804` / `.997820`），所以前端行键 `backup_id#timestamp` **不是遗漏而是兼容**，同批把 `System.tsx` 那处注释的归因换成 L250 前夜。删除的粒度保持原样（按 id 过滤 ⇒ 同 id 的行一次清全部）。A303 那一格：`backup()` 里两次 `datetime.now()` 收成一次，`backup_id` 与回执 `timestamp` 同源，跨秒错开从此不可能；判据两把（AST 数调用次数恰为 **1** + 假钟只许读一次的行为差），前一把会被「把第二次调用挪进注释」喂饱，后一把不会。**新守卫 `tests/unit/test_backup_index_collapse_l250.py` 12 支 / 五类**（写边收敛 4 / 读边原样 3 / 同秒匿名 1 / HTTP 与 CLI 两面一致 2 / AST 钉「写点是推导式而不是 append」2），**零对外行为面之外的改动**：不动契约、不动端点、不动 SDK 形参。**`test_backup_wiring_l231.py` 33 → 34 支**：三支按新世界改名反转（`…leaves_one_index_row_and_one_file`、`…come_from_one_clock_read`、`…still_hedges_for_legacy_duplicate_ids`），新增假钟行为差一支，短语名册里「删一次两条一起消失」换成「一次全清」。**四臂字节级注入取证**分别咬中 **10 / 3 / 1 / 2** 支（a1 写点放回 append、a2 第二次读钟放回、a3 读边改成按 id 去重、a4 摘掉推导式的 `backup_id` 条件），逐臂复原后比磁盘哈希，基线复跑 **0 failed / 46 passed**。**插行平移处置**：`augmentor/backup.py` 净 **6/6 = 0**（A303 那一处从 3 行压回 2 行，把 `backup.py:199` 的活引用留在原位（当轮那一格在 192 行，L273 插行 +7 后重钉））、`docs/augmentor/API.md` 五行原地改写净 **0**（`docs/augmentor/API.md:1139` 一族引用不平移）、`System.tsx` 9/9 净 **0**、`api.ts` 2/2 净 **0**、`types/api.ts` 6/6 净 **0**（顺带订正 `BackupInfo` 注释里那句陈旧的「校验和不匹配照样 200」——A299 早在 L243 就收了）；只有 `System.backup.test.tsx` 净 **+1**（注释多一行）。前端三门禁（`web/` 有改动 ⇒ 适用）typecheck 与 lint（`--max-warnings 0`）均 rc=**0**、test **22 files / 307 tests passed**。全量门禁 **exit=0 / 9 008 passed / 0 failed / 10 skipped / 6 warnings / 328.72 s**、**99.84 %**、`TOTAL` **14 360 / 8 / 3 968 / 22**（用例闭合 = L249 终态 8 995 + 新守卫 12 + l231 拆出 1；语句 +1 就是新增那句 `rows =`，Miss / Branch / BrPart 三格与 L249 逐格相同 ⇒ 没留死支；318.90 s → 328.72 s 按负载差档算噪声不作结论）。l242 三格按红读数重钉（表行 254 → **255**（新立 A335 一行，待拍、不带标记）、声明面 123 → **125**、有标记 141 → **143**），同轮把该守卫「弱词不算标记」那一支的 **live 样本从 A298 迁到 A328**（A298 本轮收了，样本失效是本轮自己造出来的；「全清」那一形由合成断言继续钉，迁移归因写在该支的断言消息里）。**待拍面本轮由代理择定修法**：A298 / A303 立账两格都写着「待拍」，历轮清单行记的是「待拍的一律不动」——本轮按目标线（备份那一族的功能完整性）择定走 ①，改动集中在一只写点，要回退只需把 `backup()` 的推导式换回 `append` 并同批反转 l250 与 l231 两侧判据；其余待拍面（A301 / A304 / A308–A311 / A313–A318 / A323 / A324 / A328 / A330 / A333）的**定级格一律未动**，A306 本轮只按现量改了它括号里那句指涉 A298 的口径（定级格未动），A335 是本轮从 A303 拆出来的新立待拍（剩余笔数以 l242 那把尺子的读数为真相，不手数）。⇒ 详见循环日志 L250
- [x] **L251** `bd8b808b0` +（账本批即本条所在提交）`feat,test,docs(augmentor)`：**收 A304——给 `/api/data/list` 加 `all_files` 那一档，候选口径与白名单同宽（走立账那格的修法①，② 与 ③ 未做）**：新增一只可选布尔 query 键 `all_files`，默认 false ⇒ **默认档一字未动**（不带参数、`false`、`0` 三种写法的**整条目**含 `path` 与 `size` 逐格相同）；宽档只放开**名字**这道闸，扫描范围仍然逐字来自 allowed_data_roots、`glob` 不递归、按 resolve 后的路径去重，所以「列表能给的，路由必须能读」在两档下同成立（宽档四份喂回那只加载端点实测无一 403 也无一 404）。`api/routes/data.py` 净 **7/7 = 0** 行（卷一 A8 那格对着这只文件的历史行号引用一字未平移）、`docs/augmentor/API.md` 净 **4/4 = 0** 行。前端新增 `getBackupSourceCandidates` 一只 fetcher 走宽档，系统状态页那六处候选下拉（备份源 / 数据集源 / 监控数据 / 源数据 / 迁移产物 / 流式产物）共用同一份 state ⇒ **变宽的是这一页的六处，不只是备份那一格**，其余 8 处数据文件选择器继续吃默认档；query 刻意写在路径串里而不是 params 对象里，因为 l224 那把消费对账尺子按（方法, 路径）归属 params 块，同一路径两员调用时它分辨不了。**新守卫 `tests/unit/test_data_list_all_files_l251.py` 19 支 / 七类**（默认档零漂移 / 宽档与白名单**集合相等**而不是包含 / 宽档不越边界（根外文件、子目录、重复根去重、缺失根两档都回空）/ 旋钮非装饰（`all_files=maybe` 由 FastAPI 判 **422**，宽档严格多于默认档）/ OpenAPI 声明逐格（那只端点的 query 面恰好只有它、可选、布尔、默认 false，响应面仍只有 `files` 一键）/ 列出的都读得到 / 前端只有系统页取宽档）。**l231 34 → 35**（新增宽档候选与白名单同宽一支，A304 那格口径改成已落地的形状）、**l232** 只把断言名换成新 fetcher（支数 39 未动）、**l224 三格按红读数重钉**（调用形状 77 → **78**、带 query 的形状 14 → **15**、去重后的键名 9 → **10**）、**l237 三格按红读数重钉**（`export const` 85 → **86**、调用点与对账行 77 → **78**）并新增一支点名「跨语言尺子第一次有两只 fetcher 配到同一只操作」。**同一轮修掉 l224 一处真实的顺序依赖**：旧判据把调用形状塞进 `{（方法, 路径）: 键名}` 一个字典，同一只路径两员调用时留下哪一格取决于集合迭代序——同一份代码在 `PYTHONHASHSEED=0/1` 绿、`2/3/7/11` 红（六档实测），改成 params_bindings 归属 + 「出现集合」逐格断言之后五个种子各 **28 passed**。**四臂字节级注入取证**分别咬中 **5 / 5 / 7 / 4** 支（① 摘掉旋钮让宽档不看 `all_files`、② 默认档翻成 true、③ 扫描范围改成工作目录、④ 前端那一员退回默认档），四个被改文件逐臂复原后重新读盘比字节，基线复跑 **121 passed / 0 failed**。**本轮量到 A280 那一族的又一格现量**（定级格未动，仍待拍）：宽档第一次把非数组那份文件端到候选列表上，而它喂回加载端点拿到的是 **500 + 固定文案**、不是形态那句 400，吞掉它的是 `api/routes/data.py:177` 那句 except Exception（HTTPException 也在它的捕获范围内）；守卫按**观测行为**而不是应有行为钉这一格，将来动 A280 要同批改这一支。门禁：前端三门禁（`web/` 有改动 ⇒ 适用）typecheck 与 eslint（`--max-warnings 0`）均 rc=**0**、vitest **22 files / 307 → 308 支全绿**；账本批之前的全量定标 **exit=0 / 9 029 passed / 0 failed / 10 skipped / 6 warnings / 360.09 s**、**99.84 %**、`TOTAL` **14 360 / 8 / 3 968 / 22**（用例闭合 = L250 终态 9 008 + 新守卫 19 + l231 新增 1 + l237 新增 1；TOTAL 四格与 L250 逐格相同 ⇒ 本轮产品改动净零行、零新语句，宽档靠的是既有那一句条件的布尔取反；328.72 s → 360.09 s 按 L239 / L240 两轮量过的负载差档算噪声，**不作结论**）。l242 三格由该守卫自己的红读数重钉（表行 255 **未动**（本轮不收新账）、声明面 125 → **126**（A304 第一次进声明面：本行开头的「收 A304——」与日志标题各点一次，同一只 ID 只算一笔）、有标记 143 → **144**），phantom 46 与 PARTIAL 三格未动；A304 那一行收成 `~~S~~` 已收口（L251），A280 那一行原地补现量（定级格未动）。其余待拍面（A301 / A306 / A308–A311 / A313–A318 / A323 / A324 / A328 / A330 / A333 / A335）的定级格一律未动，Backlog 剩余笔数以 l242 那把尺子的读数为真相（不手数）。⇒ 详见循环日志 L251
- [x] **L252** `502f83dd7` +（账本批即本条所在提交）`feat,test,docs(augmentor)`：**收 A330——备份一族六员的契约面按实测补上错误档，声明集与真请求同码**：`api/routes/system_ops.py` 给六员各加 `responses=`，**只补 29 个真请求实测到的档**：list `400 / 403`、create `400 / 401 / 403 / 404`、restore 再加 `409`、delete `400 / 401 / 403 / 404`、verify `400 / 403`、cleanup `400 / 401 / 403` ⇒ 六员合计 **20 格**，一格没实测到的码都不写；错误体统一声明成 `{detail: string}`。判据不是「补齐了就行」而是**集合相等**：少声明 = 客户端踩坑，多声明 = 客户端处理永远不会来的分支，两个方向同一把尺子。新守卫 `test_backup_contract_error_codes_l252.py` **31 支 / 六面**（名册与矩阵、逐员臂点名、「只读两员不许虚声明 401」的反面判据、说明面逐档要点名旋钮或状态、错误请求磁盘零留痕、422 自动声明的反向缺口、全仓 census），旋钮清单不手抄而是与三只请求模型的字段集做集合相等。四臂零写入注入取证（在导入后的对象上打补丁，不改仓库文件）：摘 409 ⇒ **4 红**、给只读那员虚声明 401 ⇒ **3 红**、把 409 的说明改成空话 ⇒ **2 红**、从守卫自己的臂表里剪掉 verify 的 403 一臂 ⇒ **3 红**，基线 **31 支全绿**。**同场反转 L243 那把现状事实尺子**：`test_backup_integrity_l243.py` 里钉「契约只声明两只」的那一面改名 `TestTheContractAdmitsTheRuntimeVerdicts`、两只新端点的期望集由 `{200, 422}` 换成实测集，模块 docstring 那三格「现状事实」3 行换 3 行、第 322 行那一处历史引用未平移（`sed` 复核）。**顺手量到的反方向缺口另立 A336**：`422` 由框架见「有参数」就自动挂，六员里带请求体的三员打得出来，不带体的三员（list / verify / delete）构造不出**任何一个** 422 请求 ⇒ 本轮按事实钉成数字，不动公开契约。同场量到两件：同名查询参数重复给值 FastAPI 取**最后一个**（所以 403 那一族每臂只给一个值）、29 个错误请求跑完备份目录 / `index.json` / 数据根三处逐字相同（A299「409 一个字节都不写」从一只端点扩到整个家族）。`docs/augmentor/API.md` **七行同行追加、净 0 行**（错误码表 404 那行讲清 409 为什么不单列 + 六员逐只格各写自己声明了哪几档），`git diff --numstat` 复核 **7/7**。门禁：前端三门禁**不适用**（`web/` 零改动，不是漏跑）；账本批之前的全量定标 **exit=0 / 9 060 passed / 0 failed / 10 skipped / 6 warnings / 331.69 s**、**99.84 %**、`TOTAL` **14 366 / 8 / 3 968 / 22**（用例闭合 = L251 终态 9 029 + 新守卫 31，l243 那一支只改名与换期望值而不增删；`TOTAL` 语句 **+6** = 一份错误体形状常量、`_error_responses` 的 def 与它的 return、三格模块级说明字符串，未覆盖 8 / 分支 3 968 / 未覆盖分支 22 三格与 L251 逐格相同；331.69 s 对 L251 的 360.09 s 按 L239 / L240 量过的负载差档算噪声，**不作结论**）。l242 三格由该守卫自己的红读数重钉（表行 255 → **256**（新立 A336 一行）、声明面 126 → **127**（A330 第一次进声明面：本行开头的「收 A330——」与日志标题各点一次，同一只 ID 只算一笔）、有标记 144 → **145**（A330 收成 `~~M~~` 已收口（L252））），phantom 46 与 PARTIAL 三格未动。**本账自指行号按现量重钉，并订正 L251 的一处半落刀**：L251 日志 ⑯ 声称把 L232 清单行那句「本账第 795 行」推到 796，实测**只落了半刀**——`git show 3a09b3d7a` 只把那一句的**链尾**补成「按现量重钉为 796」，句首 operative 那一格一直写着 795，而 diff 里没有任何「本账第」的增删），而目标那一行本轮已在 **796** 上、经本批 +1 清单行与 +2 Backlog 新账后到 **799** ⇒ operative 与链尾同批落刀，并在链上补记这次订正。其余待拍面（A301 / A306 / A308–A311 / A313–A318 / A323 / A324 / A328 / A333 / A335 / A336）的定级格一律未动，Backlog 剩余笔数以 l242 那把尺子的读数为真相（不手数）。⇒ 详见循环日志 L252
- [x] **L253** `c3a2b9441` +（账本批即本条所在提交）`test,docs(augmentor)`：**收 A328——两卷 Backlog 定级格的标记词汇立成封闭清单，轮号反向可追溯钉成判据**：零产品代码、零 `docs/` 改写，改动面是两卷账本加一把新守卫 `tests/unit/test_backlog_mark_vocab_l253.py`（20 支 / 四面）。**判据面从整行换成定级格**（本轮唯一要紧的取舍）：l242 的标记判据扫整行，而 Backlog 正文里满是引用**别的东西**的闭合语，现量整行 108 行有标记而定级格只有 64 行 ⇒ 把「这行的处置结论」交给散文决定，尺子看着严其实松。四件结构件（字母形 / 闭合词 / 括号头 / 是否粗体）的组合名册 16 种、字母形 7 档、闭合词 5 个一律精确相等 ⇒ 换新写法当场红。轮号反向那一维钉成「每一格闭合语必须从括号里读出轮号，且该轮号能在某一卷的进度声明面查到」：同卷 52、跨卷 12（12 行逐格点名，且**双向**——跨卷不写明红、写明跨卷而本卷也查得到也红）。**收掉一处真缺陷**：卷一 A31 那一格只有划掉的字母而全格没有轮号（已收却不知道哪一轮收的），按声明面补成 `~~S~~` 已收口（L29），此后「有标记而无轮号」为空集。**A328 立账那五条现量按本轮实测作废并改写**（132 → 108 / 64 两面、严格式解析不出 10 → 0、跨卷 3 → 12、形状 7 种 → 组合 16 种，归因逐格写在 A328 那一格与日志 ⑥）。**新立 A337**（定级格停在裸字母的 44 行两格差，连同日志块那 46 行污染表行面的现量），本轮只量不补标：36 行的补标要逐行人核对「正文那句闭合语说的确实是本行整行」。**六臂注入取证写成守卫内的内存改行**（不碰磁盘）：A1 换清单外的新动词 ⇒ 组合名册与词汇表同时变、A2 摘掉轮号 ⇒ 无轮号名册点名该格、A3 谎报轮号 ⇒ 不可追溯名册点名该格、A4 摘掉「第二本账」⇒ 跨卷配对红、A5 给待拍那一格补标 ⇒ 权威面 +1 而宽面不动、A6 只在正文写一句闭合语 ⇒ 宽面 +1 而权威面不动；跑完守卫两卷账本 `git status` 一字未动 ⇒ 零磁盘副作用。l242 三格由该守卫自己的红读数重钉（表行 256 → **257**、声明面 127 → **128**（A328 第一次进声明面：本行开头的「收 A328——」与日志标题各点一次，同一只 ID 只算一笔）、有标记 145 → **146**（A328 那一格此前整行也没有标记）），phantom 46 与 PARTIAL 三格未动。**L232 那句 operative 自指按现量 799 → 802**（本批 +1 清单行、+2 Backlog 新账连带它的分隔空行），链尾补第十个值并以 `sed -n '802p'` 复核落点仍是含 `docs/augmentor/API.md:1139` 的那一句。门禁：前端三门禁**不适用**（`web/` 零改动，不是漏跑）。其余待拍面（A301 / A306 / A308–A311 / A313–A318 / A323 / A324 / A333 / A335 / A336 / A337）的定级格一律未动，Backlog 剩余笔数以 l242 那把尺子的读数为真相（不手数）。⇒ 详见循环日志 L253
- [x] **L254** `e50a3726d` +（账本批即本条所在提交）`test,docs(augmentor)`：**收 A337——l242 的表行面按「## 循环日志」分区，日志块那 46 行名册不再算 Backlog 表行**：零产品代码、零 `docs/` 改写，改动面是两把账本守卫（`tests/unit/test_backlog_closure_l242.py` 与 `tests/unit/test_backlog_mark_vocab_l253.py`）加本卷账。口径 8 新立：`table_rows` 加 `backlog_only`（默认开），只认 Backlog 区的行首 ID。**为什么值得做**：B226–B263 那 38 只从未有过 Backlog 行的 ID，此前在「声明面说收了 ⇒ 表行有标记」里**同时充当被核对的账与自己那条证据**（同一张日志块里的表格两面都是它），过滤之后它们改归 phantom，而该守卫的口径 2 拆成「历史裁撤 46 + 日志名册 38」两格各点名一次。**判据没被改松**：过滤前后 `declared ∩ rows − marked` 都是 PARTIAL 三格，这一格由一支专名用例现场钉住；反空转那一面从「下界 100」改钉成「留在册 91 + 不过滤 129」两个数，MIN_DECLARED 随面从 100 降到 **85** 并把不过滤那一面钉成对照 ⇒ **这一格是唯一被调低的数字，调低的同时多了一支独立断言**，归因写在这里。**两把尺子对上数**：l242 过滤后的有标记名册与 l253 的整行那一面集合相等（**109**）。**l253 同步四格**：表行 257 → **258**（A338 一行）、Backlog 区 211 → **212**、定级格闭合标记 64 → **65**、整行那一面 108 → **109**、同卷可查 52 → **53**，字母形 ~~S~~ 32 → **33**、闭合词「已收口」32 → **33**，裸 S 净 0（A337 离裸字母那一格、A338 入同一格），组合名册里 `(~~S~~ / 已收口 / 括号 L / 不粗体` 那一档 7 → **8**——A337 落进**已有形状**而没有造出第 17 种。**A337 的 ① 与 ② 两半合成新立 A338**（36 行的定级格要逐行人核对，不能机械补标），按口径 5 写成括号形状 ⇒ 不进声明面、`test_gap_equals_partial_roster` 没有多出假欠账。**L232 那句 operative 自指按现量 802 → 805**（本批 +1 清单行、+2 A338 连带分隔空行），并把 L253 那一批欠的归因补进链上（当时只把数字接进链尾），链尾至此列出十一个值；落点以 `sed -n '805p'` 复核仍是含 `docs/augmentor/API.md:1139` 的那一句。门禁：前端三门禁**不适用**（`web/` 零改动，不是漏跑）；其余待拍面（A301 / A306 / A308–A311 / A313–A318 / A323 / A324 / A333 / A335 / A336 / A338）的定级格一律未动，Backlog 剩余笔数以 l242 那把尺子的读数为真相（不手数）。⇒ 详见循环日志 L254
- [x] **L255** `37948d428` +（账本批即本条所在提交）`test(augmentor)`：**收 A316——三处文案里的三个告警阈值与 SDK 当下真的在用的那三个立成同一把尺子**：零产品代码、零 `docs/` 与 `web/` 改写，改动面只有一把新守卫 `tests/unit/test_quality_threshold_copy_l255.py`（10 支 / 四面）。**A316 的账比原格写的还多一面**：原格只点页面卡片，实测这一串数在仓内有三处（卡片正文、`web/src/types/api.ts` 的类型注释、`docs/augmentor/API.md` 的 monitor 那一行），守卫把「三处各恰 1 处、总数精确等于 3」钉成站点名册 ⇒ 删一处或加一处都红。**判据是值而不是字面**（0.70 与 0.7 同一个数），换新写法（拆成 70% 那种）当场抽不到而站点数掉 ⇒ 由专支证明尺子不恒真。**行为维在场**：三项指标全压到阈值之下走告警那一路，要求正文打印的阈值与文案同一个数，这一支把「页面抄的数」与「用户在告警正文里真能读到的数」钉成一份。四臂注入取证（改文案一个数 / 改 SDK 一个档 / 拆掉一处站点 / 给 SDK 加第四个带 alert_below 的指标）全部写成内存里的改写，跑完三处文案字节与进入时相同（守卫自带复验）。**本轮量到的一格尺子代价**：初稿把四臂写成一处 parametrize，定向复跑 l98 那把参数化普查当场两格红（derived 38 → 39、与 l97 的调用点总数不合），改成四条显式用例后 l98 全绿 ⇒ **此后任何一轮新增一处 parametrize，都要同批重钉 l98 的档位计数**；同一次复跑那五支 error 经核与本文件无关（会话级 fixture 拒绝「定向复跑采不满下限」的那一跑形状，全量门禁里一直绿），归因写进守卫口径 7 而不混成本守卫的账。**A316 未做的两格如实留在原行**：修法②（生效阈值进响应面，页面不再抄数）是契约变化、与 A335 同族，仍待拍；告警正文英文单轨那一格归 A311 那一族。**L232 那句 operative 自指按现量 805 → 806**（本批 +1 清单行、零新 Backlog 行），链尾补第十二个值并以 `sed -n '806p'` 复核落点仍是含 `docs/augmentor/API.md:1139` 的那一句。门禁：前端三门禁**不适用**（本轮只读 `web/` 两处文案，一字未改）；l242 两格按红读数重钉（声明面 91 → **92**、有标记 109 → **110**），l253 六格同步（定级格闭合标记 65 → **66**、整行 109 → **110**、同卷可查 53 → **54**、字母形裸 S 93 → **92** 与划掉 ~~S~~ 33 → **34**、闭合词已收口 33 → **34**、组合名册那一档 8 → **9**），表行三面（258 / 212 / 46）与 phantom 84 一格未动。其余待拍面（A301 / A306 / A308–A311 / A313–A315 / A317–A318 / A323 / A324 / A333 / A335 / A336 / A338）的定级格一律未动，Backlog 剩余笔数以 l242 那把尺子的读数为真相（不手数）。⇒ 详见循环日志 L255
- [x] **L256** `80f36337e` +（账本批即本条所在提交）`fix,docs,test(augmentor)`：**收 A324——split 的比例判据两门三容差并成 `validation.require_ratio_list` 一条权威**：产品侧只动两个文件且**净行数 5/5 与 8/8**（`augmentor/dataset_ops.py`、`augmentor/data_splitter.py`），门口那三行手抄 `abs(total_ratio - 1.0) > 0.01` 与分层门那两行`abs(total - 1.0) > 1e-6` 加「只判负数」全部删掉，两处共引家族判据（和 ±0.01 含端点、量化 9 位 + 逐项 bool/NaN/非数值/0..1 + 长度 3）。**为什么这是产品缺陷而不是口径之争**：改前最自然的「留两位凑 1.0」（0.4/0.3/0.29）在门口被判「当前为 0.99」——那是浮点噪声而不是坏输入，A320 在同一族处理过的正是同一个形状；`-0.8/0.1/1.7` 走默认支实测 **200** 且把 6 条全塞进测试段（`0/0/6`），一个负比例悄悄改掉的是产物而不是报错；`ratios` 喂 4 个元素时两条支路都不判长度，SDK 直构过校验后死在 `ValueError: too many values to unpack` ⇒ 500 形状而不是 400。**A57 那一格没有采用**：原记「统一方向要碰默认侧的破坏性变更」，本轮取的是反向——把分层侧的 ±1e-6 抬进家族容差、给默认侧补逐项与长度判据，于是默认侧**没有**新拒任何原本合法的输入（原判「±0.01 不含端点」，新判含端点 ⇒ 可接受域只增不减），分层侧从「一个无关旋钮换一次判决」变成与默认侧同答。**测试按「同答」重写而不是按「放行」重写**：`test_transform_wiring_l229.py` 判红两档（负比例、和超容差 1.2）各要求两支 400 且 `detail` **逐字节相等**（入口那道门先答，两支共引同一条判据 ⇒ 连判决里的参数名都是同一格），判绿两档（和 0.99、和 1.005）各要求两支 200 且三段之和恒等条目数；`test_dataset_ops.py` 把原「分层支更严」那一支换成两支各 raises 同一句；`test_data_splitter.py` 两句 match 改按新文案。**零写入注入取证**：把两处接线逐臂摘成恒不响（`dataset_ops` 臂 **5 红** / `data_splitter` 臂 **2 红** / 双臂 **7 红**，基线 253 全绿），全程只在导入后的模块对象上打补丁、仓库字节未变。**防复发**：`test_decision_gates_l238.py` 的手抄名册由 2 个调用点扩到 4 个（四个门口任一再写 `if abs(` 即红）。**文案与账目同批改**：`docs/augmentor/ARCHITECTURE.md` §3.16 第 3 条（净 4/4 行）、`web/src/services/api.ts` 的 `splitDataset` docstring（净 3/3 行，A289 那两句旧卡片文案随之作废）、A289 行内 ①②③ 就地记为已并入并**删掉三处随代码形状消失的行号引用**（a184 硬 0 档 `dead_line` 首跑红 3 格 ⇒ 全部由改自己的散文收，尺子一格未动）。**L232 那句 operative 自指按现量 806 → 807**（本批 +1 清单行、零新 Backlog 行），链尾补第十三个值并以 `sed -n '807p'` 复核落点仍是含 `docs/augmentor/API.md:1139` 的那一句。门禁：前端三门禁**本轮适用并已跑**（`api.ts` 改了文案）——`tsc --noEmit` 与 `eslint . --max-warnings 0` 均 rc=0、`vitest run` **22 文件 / 308 支全绿**；其余待拍面（A301 / A306 / A308–A311 / A313–A315 / A317–A318 / A323 / A333 / A335 / A336 / A338）的定级格一律未动，Backlog 剩余笔数以 l242 那把尺子的读数为真相（不手数）。⇒ 详见循环日志 L256
- [x] **L257** `33f98dfae` +（账本批即本条所在提交）`fix,test(augmentor)`：**收 A285——「默认看哪三格」的五处手写字面量并成 privacy 模块常数 DEFAULT_TEXT_FIELDS 一份权威**：产品侧动两个文件且**净行数 2/2 与尾部追加 20/1**（augmentor/audit.py、augmentor/privacy.py），augmentor/cli/parser.py **一行未动**——该模块有写进 docstring 的既有口径「不 import 任何 augmentor 业务模块」（EXPORT_FORMATS / INPUT_FORMATS 两处注释即这一口径的现成先例），所以 CLI 那三处保持字面量、由新守卫逐档与权威对账；口径没被破，也没有留下第二种形状。权威取**元组**而不是列表：模块级可变列表被五处共引，任何一处就地 append 会无声改掉其余四处的默认面（注入臂 A3 把这一格当场打成 3 红）。常数住在 privacy.py 的**文件末尾**而不是紧挨 DEFAULT_PATTERNS：audit.py 第 68 行与 privacy.py 第 76 行是账本与 l227 / l228 两把守卫的活引用落点，在它们之上插任何一行都会把落点平移成上一行（正是 A337 / A338 那一族的形状），而取用点是运行时查名，尾部定义两头都保住。新守卫 `tests/unit/test_default_text_fields_l257.py`（**16 支 / 四面**：同一对象面、内容与形状面、SDK 运行时默认面、CLI 逐项对账面）。**六臂注入取证全红、逐臂复原后复验 sha256 与行尾**：A1 audit 抄回字面量 2 红、A2 权威少一格 6 红、A3 权威抄成可变列表 3 红、A4 CLI sanitize 默认漂移成两格 2 红、A6 audit 的 or 被顺手改掉 2 红；**A5 那一臂抓到的是尺子自己的洞**——`test_parser_still_imports_no_augmentor_business_module` 原用行首正则，注入「import argparse; from augmentor.privacy import …」这种分号偷带之后 **0 红**（from 不在行首，正则看不见），改成 AST 枚举真实 import 面之后同一臂 1 红；这条「先写歪的尺子」留在本账而不是悄悄改掉。**顺带核对出一格新的现量并立 A339 待拍**：显式空列表在 PiiSanitizer 侧是合法意图（L188 改的正是这一刀），在 DatasetAuditor 侧仍被 fields-or-默认 读成「没传」而套上三格——本轮只核对未修，改掉会让空值统计面变窄，属行为变更。两把 Backlog 尺子按现量重钉：l242 声明面 93 → **94**、有标记 111 → **112**；l253 权威面 67 → **68**、宽面 111 → **112**（与 l242 同面同数，两把尺子第四次对上）、同卷可查 55 → **56**、字母形裸 S 92 → **91** 与划掉 ~~S~~ 34 → **35**、闭合词已收口 35 → **36**、组合名册 (~~S~~ / 已收口 / 括号 L / 不粗体) 那一档 9 → **10**（落进已有形状，没有造出第 17 种），表行两面 258 → **259** 与 212 → **213**（新立 A339 一行），日志块表行 46 一格未动。前端三门禁**不适用**（本轮零 web/ 改动）。其余待拍面（A301 / A306 / A308–A311 / A313–A315 / A317–A318 / A323 / A333 / A335 / A336 / A338 / A339）的定级格一律未动，Backlog 剩余笔数以 l242 那把尺子的读数为真相（不手数）。⇒ 详见循环日志 L257
- [x] **L258** `db2fd4838` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**「调用方给的名字被当成路径片段用」这一族从未入过账的漏洞第一次有一把 SDK 侧权威——新增 `validation.require_within_dir(label, value, base)`，五个模块十处落点共引，HTTP 门口按类型放行成 400**：改前四条实测全在沙箱里复现过（`VersionManager(storage_dir=<tmp>/versions).delete_version("../victim/important")` 返回 `None`（成功）而版本根**之外**那个整目录被 `shutil.rmtree` 掉、`load_version("../secrets")` 交出目录外的 `data.json`、`CheckpointManager.create_checkpoint("../../escape", 3)` 把断点写到 `checkpoint_dir` 之外、`DiskCache(cache_dir=<数据目录>).clear()` 连目录里**别人的** `*.json` 一起 unlink），HTTP 面对前两条**根本没有闸**——`GET /api/versions/%2e%2e`（无鉴权）与 `/api/versions/%2e%2e/data` 在改前拿的是 **500**，而那不是「拦下了」，是「路径被原样拼出去、恰好那一格没有文件」。**权威只一把**：`augmentor/validation.py` 尾部追加 41 行（新函数起于该文件第 986 行），落点避开 `:217-222 / :437-442 / :639` 三处活引用之下 ⇒ 零平移；空值那一维**不重抄**，交回同族的 `require_string`（自己写 `not isinstance(value, str)` 会把 `5` 判成「不能为空」——那是句假话，注入臂 A8b 专门把「不委托」这一形打成 1 红）；判决文案刻意只含名字与目录标签、**不含解析后的绝对落点**（`INTERNAL_ERROR_DETAIL` 那一族的部署路径外泄面）。它与 `api/deps.py` 的 `resolve_within_roots` 是**同一条口径的两个尺度**：那一把管「HTTP 传入的路径只能落在数据白名单之内」，这一把管「一个名字不许逃出它自己的目录」，SDK 面必须自带，因为 CLI 与直接调 SDK 的调用方**不经过** HTTP 那道闸。**十处调用点 / 五个文件**（AST 现量：`checkpoint.py` 3、`version_control.py` 3、`export.py` 2、`dataset_ops.py` 1、`versioning.py` 1），包含判据的 `.parents` 比较在 SDK 包内**恰好两处实现**（本把权威 + L231 给备份名落下的那条手判），第三份手抄就是 L238 / L256 / L257 收掉的那一族，由普查支当场盯着。**门口按类型放行**：`api/deps.py` 新增 `raise_boundary_error`（净 20/1），`DataValidationError` ⇒ **400 且回 SDK 那句判决**，其余照旧收敛 500——不按「是不是 ValueError」放行，因为 `VersionError` 同为 `ValueError` 子类，一刀切会动对外状态码契约（**A280** 仍待拍），而 `tests/integration/test_api.py:479` 正钉着那一格的 500；`api/routes/version.py` 九处调用点同行改写、净 **9/9 = 0** 行，收尾现量余 **29 处** `raise_internal_error`（audit 1 / augment 3 / config 3 / data 5 / export 4 / leakage 1 / multimodal 2 / privacy 1 / quality 9）。`DiskCache.clear()` 收成只清自己记账的条目（`owned = {md5(key) for key in self._metadata}`），外来件与 `_metadata.json` 都活着。**两处按实测推翻想象**（本轮真实返工，不美化）：`DatasetVersionManager.delete_version("../leak")` 不 raise 而是回 `False`——版本未登记时先短路，那一处权威在那条路径上是**纵深防御而不是当前可达面**；`CheckpointManager` 的 `task_id=".."` 被 `f"{task_id}_checkpoint.json"` 拼成一个**文件名字面量**、落点仍在根内 ⇒ **按实测是接受**，判据单独钉住防后来人改成「凡带点就拦」。**盘符与反斜杠两档按 `os.name` 分臂**——`"C:\abs"` 在 POSIX 是合法文件名、`"..\x"` 在 POSIX 不是分隔符，合并清单会造出跨平台假红；符号链接与 `mklink /J` 那一臂按能力探测，本机实测造得出 ⇒ **未 skip**。**新守卫 `tests/unit/test_name_path_containment_l258.py` 36 支 / 九类**（权威形状与逐档判决 / 调用点名册 / 版本面 / 断点面 / 导出与分割面 / 缓存面 / HTTP 边界 / 尺子有牙齿 / 规范化逃逸臂）。**十二臂注入取证逐臂读数**：A1–A5 红 **3 / 2 / 3 / 1 / 1**、A6 红 **3**、A7 红 **1**、A7b 红 **0**、A8 红 **2**、A8b 红 **1**、A0 与 A9 两个基线臂红 **0**；两处预测被实测推翻，改的是预期而不是尺子——A6 我预测 4 红、实测 3 红，现场理由是固定 500 文案 `INTERNAL_ERROR_DETAIL` 同样不含部署路径，那一支**按设计不响**；A7b 在把空值那一维委托给 `require_string` 之后由 0 红变 1 红 ⇒ 削弱臂不该同时改动空值文案，遂把两个削弱臂都改成先走 `require_string`，另加 A8b 把「委托」本身证成有牙齿。**并行支路吞判决另立 A340（待拍、不带标记）**：`export_batch` / `export_all_formats` 默认并行时任务异常被咽成 `logger.error`，同一输入串行 raise、并行回空 dict（实测 `results == {"../../silent/x": {}}` 且目录外**零产物**，写边拦住了而回执照样 `success: true`，危害面是假回执而不是越界写），三处拒绝臂因此显式 `use_parallel=False`，并单独一支 `test_the_parallel_arm_swallows_the_verdict_and_reports_success` 把这一形**钉成已知限制**而不是假装合法；改它的错误语义是对外契约变更、且修好后该路由的失败会打到 500 那一档，须与 **A280** 同批处置，本轮只量不动；`/api/dataset/split` 同一条判决经 `to_http_error` 已是 400，无需入账。**HTTP 面的前后对比**：`%2e%2e` 那一族由「无闸的 500」变成「带 SDK 判决的 400」，而 `v_ghost` 那一档**照旧 500** 且单独一支——它钉的就是「本轮没有顺手把状态码契约改掉」。**两把普查棘轮按红读数重钉六格**（读数出自仓外探针 `Temp/l258_census.py` 直接 `import` 那把尺子的 `param_sites(TESTS_ROOT)` 现量，不是按「字面量 ⇒ 可读」推的）：l97 三格 CALLS 434 → **436**、READABLE 315 → **316**、BLIND 119 → **120**；l98 三格 plain 59 → **60**、盲面用例名册 114 → **115**、动态面真看到 114 → **115**（exempt 24 / opaque 11 / derived 38 / ids-derived 11 四档一格未动）。**卷一 A144 三处活引用按现量重钉**（`checkpoint.py` 净 +1 行的后果）：`219 → 220`、`263 → 264`、`334 → 335`，1/1 净零行。产品侧净行数逐文件（`git show --numstat db2fd4838`）：`validation.py 41/0`、`api/deps.py 20/1`、`checkpoint.py 4/3`、`version_control.py 4/3`、`export.py 3/2`、`cache.py 3/2`、`dataset_ops.py 2/2`、`versioning.py 2/2`、`api/routes/version.py 9/9`、新守卫 473/0、l97 与 l98 各 3/3。**门禁**：新守卫定向 **36 passed**；账本批之前的全量定标 `python -m pytest tests -q -p no:cacheprovider` 第一轮 **4 failed**（四支全在 l97 / l98 两把普查尺子的聚合断言上，归因是普查面而非产品缺陷），重钉六格后终态 **9150 passed / 0 failed / 10 skipped、99.84%、333.24 s**（用例闭合 = L257 终态 9114 + 新守卫 36，**恰等** ⇒ 没有一支被删或被改名绕开；10 支 skip 全是既有的作者工作树与无实文件定标条件，本轮未新增任何跳过理由）；前端三门禁**不适用**（`web/` 零改动，不是漏跑）。**本账自指行号按现量重钉**：L232 那句 operative 自指 810 → **813**（本批 +1 清单行、+2 A340 连带它的分隔空行），链尾补第十五个值，落点行以 `sed -n '813p'` 复核仍含 `docs/augmentor/API.md:1139` 那一处活引用。其余待拍面（A301 / A306 / A308–A311 / A313–A315 / A317–A318 / A323 / A333 / A335 / A336 / A338 / A339）的定级格一律未动，本轮只新增一行待拍 **A340**（并行支路把判决咽成回执那一格，不带标记），Backlog 剩余笔数以 l242 那把尺子的读数为真相（不手数）。⇒ 详见循环日志 L258
- [x] **L259** `30311ea7d` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**L258 那把 SDK 权威剩下的七处落点一次接干——`require_within_dir` 再落七处（`vector/faiss.py` 的 `persist` 与 `load` 各两处、`tracker._get_experiment_path`、`pipeline.export_dataset`、`visualizer._output_path` 各一处）+ 新守卫 `test_name_path_containment_l259.py` 41 支，把「这一族闭没闭」本身钉成判据**：改前在沙箱里复现过三格（`FAISSDB(collection="../escaped").persist()` 把索引写到 `storage_dir` 之外、`AugmentationTracker` 的实验文件能落到根外、`export_dataset` 的 `formats` 靠 `ExportFormat` 的 13 个成员值不含分隔符才没逃出去——那是枚举在强制，不是拼接那一行自己有闸，本轮补的正是它自己那道）；改后判据一律排在 `mkdir` 之前，坏名字不在盘上留一个空目录。闭族普查现量：权威调用点 **17 处 / 9 个文件**（L258 的 10/5 加本轮 7/4），不走权威的「目录 / 名字」拼接只剩 **4 处**且逐处点名，`.parents` 那一档的包含判据实现数仍是 **2 处**、本轮新增全为共引零手抄。HTTP 门口本轮零新形状：`visualizer._output_path` 只借那句判决、返回值保持原拼接形状，响应面与状态码一字未动。另立 **A341**、**A342** 两行待拍
- [x] **L260** `215b306d5` +（账本批即本条所在提交）`test(augmentor)`：**把「全量门禁的缺数与偏支按墙上时钟抖动」这一格钉成确定性用例**——损坏趋势文件备份防撞那一档 （`augmentor/quality_trend.py:65-67` 的 `while backup.exists():` 循环体）此前只有**同一秒内发生第二次隔离**才走得到，既有 `tests/unit/test_quality_trend.py` 的三支只测首档命名 ⇒ L259 记账批实测同一条定标命令连跑两轮 TOTAL 差 2 格（**14384 10 3968 23** 与 **14384 8 3968 22**），翻动的正是这两行 （`+67/0` 行、零产品码改动）：新两支把 `quality_trend.datetime` 冻成固定一秒，第二次隔离落到 `-<stamp>-1`、第三次落到 `-<stamp>-2`，并断言既有备份逐字节不动；单跑该类即覆盖 65-67，该档从此不再依赖套件里别的用例「恰好同秒」。注入取证两臂 `while`→`if` **1 红**、`while`→`while False` **2 红**，逐字节复原后复验 5 passed。全量 **9193 passed / 10 skipped / 342.05s**，TOTAL **14384 8 3968 22**（99.84%），`quality_trend.py` 由 98%（缺 2、偏支 1）升到 **100%**
- [x] **L261** `82ba55ab2` +（账本批即本条所在提交）`test,docs(augmentor)`：**A338 的 ①+② 同批——「正文写着判决而定级格还是裸字母」那一批 41 行的定级格按各自正文补齐，`test_backlog_closure_l242` 的标记判据同批改读定级格**（逐行核对 44 行两格差的名册：35 行走原格点名的那一批、6 行是 A337 当年误归入「引用别物」档的 `A201` 至 `A206`（它们的正文判决本就是整行，本轮一并订正并补标）、3 行留裸字母并各有专格点名（`A21` 只管 ngram 那一半、`B2` 写着前半、`B266` 写着部分关闭且其余仍在案）。零产品代码、零 Backlog 新行、零裁撤 ⇒ 表行三面 262 / 216 / 46 一格未动。卷一 `A130` 的轮号在卷二，补标按既有跨卷那一批的同族写法带上第二本账限定词 ⇒ 跨卷名册 12 → 13；`PARTIAL` 从三格走到四格（新增 `B2`）。`~~XS~~` 与 `~~L~~ 已关闭` 两形此前不在组合名册里 ⇒ 封闭清单 16 → 18 格，是新形状落进清单并被逐格点名，不是清单被改松。两把尺子在账本批落盘前首跑 16 failed / 39 passed，红读数逐格见循环日志 ⑦，逐格按各自 docstring 的处方重钉，没有任何一把尺子被改松。八把账本与文档守卫 209 passed / 8 skipped，比 L257–L260 那四轮的 205 多 4 支，那 4 支全是 l242 本轮新增的判据支（该文件 `def test_` 31 → 35，l253 与 l79 净 0）；全量定标 9197 passed / 0 failed / 10 skipped（338.71 s），`TOTAL 14384 8 3968 22`、99.84%，四格与 L260 两轮逐字相同 ⇒ 零产品代码，覆盖率分母不动）
- [x] **L262** `1d62df0d9` +（账本批即本条所在提交）`test,docs(augmentor)`：**「调用方显式传进来的空容器被读成『没传』」这一族第一次有全仓普查——新增守卫 `tests/unit/test_empty_container_defaults_l262.py`（28 支），按语法形状量到 38 格、分四族逐格钉名册（A 构造器吞非空默认 **10** / B 方法体 **7** / C 默认本身是空容器 ⇒ 幂等 **10** / D 默认是标量或一次求值 **11**），A 族十格逐格现场实例化证明传 `[]` 与传 `None` 得到**同一个非空默认**，四条 HTTP 可达路（audit / leakage / stats / search）与两处已防住的对照（evaluate 的 `if not metrics:` ⇒ 400、`PiiSanitizer` 在 L188 改过的形状）逐条点名，CLI 无空面与「仓内零调用方喂空容器字面量」各一支现验，并立 A343 待拍（三修法对外语义互不相同 ⇒ 不在代理自开轮里自选）**：本轮**零产品代码**，改动全在尺子面。**第一轮全量 5 红全部落在 parametrize 普查面**（l97 调用数 `assert 438 == 437`、l98 同格、`ids-derived: 12 != 11` 且名册多出新增那一支、盲面用例 `assert 117 == 116`），根因是新守卫那支 parametrize 的 `ids` 由名字现造 ⇒ 被 l98 判进盲面。处置不是放松尺子而是改新支自己的写法：**值位与 `ids=` 两串都写成内联字面量** ⇒ 它既不进盲面也不进 ids-derived 档，l98 的四档（61/38/11/11）、盲面 121、盲面用例 116 一格未动；l97 只按现量重钉 `MEASURED_CALLS 437 → **438**`、`MEASURED_READABLE 316 → **317**`（`MEASURED_BLIND = 121` 未动）。**第二轮全量 1 红**（`assert 25 == 24`，`MEASURED_EXEMPT_SITES`：新支整支进「动态面放过」的豁免名册）⇒ 按红读数重钉 24 → **25**，豁免名单变长不等于放松，与 L230 / L241 同口径。**终态全量定标 9225 passed / 0 failed / 10 skipped（240.41 s）**，覆盖率行逐字 `TOTAL 14384 8 3968 22`、99.84%，四格与 L261 逐字相同 ⇒ 覆盖率分母不动；9225 = 9197 + 新守卫 28 支。代码批两枚哈希 `cd56b4498`（第一版）与 `1d62df0d9`（本批），第一枚落盘时全量还是 5 红。前端三门禁**不适用**（本轮 web 零改动）
- [x] **L263** `be06c5dfe` +（账本批即本条所在提交）`test,docs(augmentor)`：**「轮号写在动词之前」那一形判决语第一次有尺子**——新增守卫 `tests/unit/test_round_first_verdict_l263.py`（**18 支，零 parametrize** ⇒ `test_parametrize_ids_l97` 的 438 / 317 与 `test_parametrize_blind_face_l98` 的盲面 116 / 四档 61-38-11-11 / 豁免 25 一格未动）。判据取两种字面形状（形状① `Lxxx` 紧贴动词、中间不越 12 字符且不断句；形状② 动词在前而**中间没有 `（`**——带 `（L` 的那一形是 `MARK` 的刀口，归 l253 管），与 l253 的宽面**按构造互斥**（候选行先要求整行不带 `MARK`），`test_the_three_faces_partition_the_table` 现场验三面闭合：宽面 **115** + 盲区 **5** + 正文无带轮号判决语 **97** = **217** 条 Backlog 表行。首跑量到 **7 格候选**，逐行读正文后定性：**2 格是整行判决而定级格仍是裸字母**（`A274`「**· L220 已收口**」与 `A71`「**L46 关闭（第三种修法：按依赖等级拆 try）**」——括号里点的是**用哪条修法**而不是**收了哪一半**，正是 A176 那格「欠账看着还在、债其实已经还了」的机械形状，本批把两格各以 1/1 净零行落进定级格）；余 5 格合法留裸字母并逐格点名靠哪个词豁免（半程 3 格 `A45` 只收①、`A127`「前半程关闭，后半仍开放」、`A136`「关闭（假臂那一侧）」；引用别物 2 格 `A121`→`A114`、`A143`→`A133`）。**被点的两只实测都是 phantom**（两卷 `^| A114 ` / `^| A133 ` 均 0 命中，表行早已裁撤而判决进过声明面）⇒ 守卫那一支按实量改成「引用的 ID 不许是开放行」并断言 `targets ⊆ phantoms`。残余名册 **5 格**、整行判决档 **空集**，两支补标回退臂证明那格空集断言变红而不是恒真。棘轮全部由红读数逼出、**没有造出新的标记形状**：l242 `MEASURED_MARKED` 110 → **112**（`DECLARED` 95 与 `PARTIAL` 四格未动）、l253 六格同行重钉（闭合单元 110 → **112**、宽面 113 → **115** ⇒ 两面之差仍是 3、同卷可查 97 → **99**、裸 `S` 70 → 68 与划掉 `~~S~~` 59 → 61、词汇「已收口」38 / 「已关闭」53、组合名册两档 11 → 12 与 24 → 25）、l79 `BACKLOG_A_CLOSED` 122 → **123**（只 +1：那把尺子的 `LEDGER` 只读第一本账）。l242 那条「动词被当散文引用而定级格不动」的 live 样本因 `A274` 本轮真收而失效，同判据换成 `A136`（三处同行改写，判据一字未改）。**新守卫首跑 10 failed / 8 passed，其中 2 处是我自己写错的断言**（详见日志 ⑤⑥，两卷都有盲区那一支是猜的），量出来才是实量。**同批把上一轮 L262 记账批落错块的那条 ⑫ 从 L261 块尾归位进 L262 块内**（本轮零产品码，覆盖率分母不动）。前端三门禁**不适用**（web 零改动）。⇒ 详见循环日志 L263
- [x] **L264** `1a1a203c4` +（账本批即本条所在提交）`test,docs(augmentor)`：**「记账批要点落错块」那一族第一次有尺子**——新增守卫 `tests/unit/test_log_block_bullet_ownership_l264.py`（**24 支，零 parametrize** ⇒ `test_parametrize_ids_l97` 的 438 / 317 与 `test_parametrize_blind_face_l98` 的盲面 116 / 四档 61-38-11-11 / 豁免 25 一格未动）。站点 = 循环日志块行内被反引号包住的 6 到 40 位 hex，每处问一次 git 的主题、按答案分六档：`same_round` / `cross_named` / `cross_unnamed`（缺陷档，现量 **0**）/ `no_round_subject` / `not_a_commit`（21 处全是 sha256 摘要）/ `stash_or_wip`（L242 那枚 `git fsck` 找回的不可达对象）。本轮收掉两格缺陷：`4d177e3f3`（L215 块，补「L212 至 L214 那一批」）与 `29b715600`（L238 块，补「L194」），两处都是**同行改写净 0 行**。**全量门禁第一轮 2 红全部来自新守卫自己**——两处 `text=True` 没传 `encoding=` 被 `test_subprocess_text_decoding` 的规则 A（无豁免档）判红，父进程在中文 Windows 上按 cp936 解 git 的中文主题，本机恰好还能从 mojibake 里吃到轮号所以守卫绿、换机器就是 `UnicodeDecodeError` 炸在采集阶段；按该守卫写的修法补 `encoding="utf-8", errors="replace"` 之后八把 **185 passed / 8 skipped**。另实测 A333 预言的裸行号漂移已经发生一次：L263 的 ⑬ 声称把本条上方的自指改成 824，`git show` 复核那一格从未落地 ⇒ 本批按现量重钉并把归因补进链尾。
- [x] **L265** `58a1f9987` + `ee133fc40` +（账本批 `b2af89396`，原 `1094b0696` 被并行会话两次 amend 改写）+（记账批即本条所在提交）`test,docs(augmentor)`：**「4xx 被路由收尾降级成 500」那一族第一次有全仓 AST 普查尺子；全仓缺数语句由 8 格走到 0（`TOTAL 14384 0 3968 18`、99.90%）**：新守卫 `tests/unit/test_http_exception_passthrough_l265.py`（**746 行 / 30 支 / 零 parametrize** ⇒ l97 的 438 与 317、l98 的盲面 116 / 四档 61-38-11-11 / 豁免 25 一格未动）与 `tests/integration/test_untested_guard_branches_l265.py`（**349 行 / 19 支**）。**这一格为什么是缺陷而不是洁癖**：`api/deps.py` 的闸一律用 `HTTPException` 出声（401 缺 key、403 路径越界、400 非法组件、404 文件不存在、413 上传越界），而路由收尾住在一处（`to_http_error` / `raise_internal_error` / `raise_boundary_error`）——凡不是 `FileNotFoundError` / `JSONDecodeError` / `ValueError` 的异常都收敛成 500，所以「`except HTTPException: raise` 且**排在宽 `except Exception` 之前**」是鉴权与白名单判决能否到达客户端的唯一通道；被降级掉的正是 401 与 403，客户端拿 500 时既不知道被闸拦了、也不知道要带 key，而收尾那句 `logger.exception` 会把一次**正常的**授权判决记成故障。既有覆盖是按端点逐个注入的样本（`tests/integration/test_api_passthrough_branches.py`，L69 那一族），删掉某一路由的那一支、或新写一条漏掉那一支的路由，样本一条都不会红 ⇒ 全仓今天 58 只这样的支、50 个配对点而**零尺子**——「本账第 NNN 行那一形没有尺子」（L242）同款形状的另一个实例。**判据全 AST 推导、不抄清单**（承 L56「判据型脚本一律走 AST」，语料面 = `api/` 全部 24 个 `.py`）：raisers **97 只 = 66 只直接出声 + 31 只按「体内调过已知 raiser」迭代到不动点并进来**；支按体形分三档——`bare` **57** / `conditional` **1**（全仓唯一一只带条件的再抛：`api/routes/data.py` 的 `_upload_landing_path`，A160 与 A163 那一对账的同场形状）/ `swallow`（把异常咽下去）= **缺陷档，现量空集**；「带宽收尾且 try 体内**装载**过 raiser」的配对点 **50 处 / 50 个不同函数**，12 只文件逐档点名（`dataset_tools` 14 与 `system_ops` 14 最大）。**装载维而不是调用维是被实测逼出来的**：只看 `Call` 会让配对点 50 掉到 **37**、少 **13** 只 try，并且让 `audit.py` / `data.py` / `leakage.py` / `privacy.py` **四只文件整块掉出面**，而那 13 只正是 `await run_in_thread(load_items, name)` 这种「把 raiser 当**值**传出去」的点 ⇒ `test_the_load_dimension_is_what_keeps_four_files_in_the_face` 把两面之差（`lid - sid` 恰等于 13 格名册、`sid - lid` 空集）与四只文件一起钉住。**已知盲区三格写明，不装**：① 经非模块对象的属性调用（`p.method()`）可达的 raiser 不进面，依据钉在磁盘上而不是靠叙述（`test_the_sdk_side_cannot_raise_http_exception` 验 `augmentor/` 那一侧根本不 import fastapi）；② 嵌套函数里 raise 的 raiser 只按**名字**参与面、同名并档；③ `try` 体外 raise 的 4xx 不进配对面 ⇒ 余 **7** 支纯防御性冗余（`_upload_landing_path`、`analyze_data`、`delete_data_item`、`update_data_item`、`visualize_data`、`vector_status`、`create_version`）由`test_the_defensive_clauses_are_the_measured_seven` 钉成名册，而不是任其看起来像漏判。**同批把 L260 记账批 ⑪ 记下的那 8 格缺数逐格接上行为差**（vector 收尾支的 403 保码 = 403+原 detail 对 500+`INTERNAL_ERROR_DETAIL` 两臂；`multimodal` 的 `_resolve` 两级皆空交回内置默认 与 `_normalize_extensions` 的 `None`/`[]` 两档；`semantic.embed_texts([])` 的早退及其 `VectorError` 闸序；`versioning` 当前版本指针三分支；`data/image.py` 的 Pillow 优先支），**两处按「本机没有可现量对象」处理而不是造假证据**：本机无特权建符号链接抛 **WinError 1314** ⇒ 指针那一臂替的是指针对象本身（只实现 `is_symlink` / `resolve`，`with_suffix('.txt')` 仍走真路径）而不是造真软链接；Pillow 未装 ⇒ `sys.modules` 内存桩报的 9x8/WEBP 与标准库文件头解析器报的 111x222/PNG **故意不同**，加 `test_pillow_is_still_absent_after_the_stub_tests` 钉住桩不外漏。另把 `semantic.py` 那一格定性写成「等价类快路径」而不是崩溃护栏（实测 `normalize_vectors(np.zeros((0, d)))` 不抛且 `list(...)` 仍是 `[]`）。**门禁**：终态全量定标（逐字 `python -m pytest tests -q -p no:cacheprovider`）**9316 passed / 0 failed / 10 skipped / 6 warnings in 311.60s**，覆盖率行逐字 `TOTAL 14384 0 3968 18`、`Total coverage: 99.90%` ⇒ **缺数语句自本账开轮起第一次为 0**（L258 至 L264 各轮逐字 `14384 8 3968 22`）；9316 = L264 终态 9267 + 两支新守卫 **49**（30+19）**恰等** ⇒ 没有一支被删或被改名绕开。余 **18** 格偏支 / 16 只文件逐格点名进日志块 ⑫ ⇒ L266 的靶子，这一维此前只有聚合数、没有名册。**代码批的哈希落在并行会话的提交里（本轮真实事故，见日志 ⑪）**：两支守卫按显式路径 stage 后 `git commit` 返回「no changes added to commit」，同一共享工作树里并行会话的 `930e31f74 sec(oa4rust/backend): …` 把暂存区那两只文件一并带走（`git show --stat` 复核该提交 6 只文件、含 `+348` 与 `+746`）⇒ 该枚哈希的主题里没有 L265 这个 token；工作树干净、磁盘内容与门禁跑过的那份逐字节相同，**未改写历史**（改写并行会话已落盘的提交，风险大于「主题缺一个轮号」这一格洁癖）。`58a1f9987` 是本轮第二枚：新守卫 docstring 把残差写成「7 条语句」而逐格点数是 8（1+2+1+2+2，与门禁「缺数 8」那一格闭合）——**读自己的账读错了一格**，按现量修正。**L232 那句 operative 自指按现量 825 → 826**（本批 +1 清单行、零新 Backlog 行），链尾补第十七个值，落点以 `sed -n '826p'` 复核仍是含 `docs/augmentor/API.md:1139` 的那一句。其余待拍面的定级格一律未动，本轮**零新 Backlog 行**，Backlog 剩余笔数以 l242 那把尺子的读数为真相（不手数）。前端三门禁**不适用**（`web/` 零改动，不是漏跑）。⇒ 详见循环日志 L265
- [x] **L266** `7e2fe5aa6` +（账本批即本条所在提交）`test,docs(augmentor)`：**「分支维只剩聚合数、没有名册」那一格走到底——L265 ⑫ 那份 18 格 / 16 只文件的偏支名册逐格定性，全仓逐字 `TOTAL 14382 0 3952 0`、`Total coverage: 100.00%`（语句维与分支维同时无残差，本账开轮以来第一次）**：新守卫 `tests/unit/test_partial_branch_residue_l266.py`（**905 行 / 46 支 / 零 parametrize** ⇒ l97 的 438 与 317、l98 的盲面 116 / 四档 61-38-11-11 / 豁免 25 一格未动）把 18 格收成三档闭合分区，名册就是判据（`DEAD_REWRITES` 7 / `CHAIN_FENCES` 5 / `COVERED_ARCS` 6 三张 dict 的键集按现量点名，并有一支把三面并起来验它恰好等于 18 格 / 16 只文件的名册）：**甲 7 格是死支，删除或等价改写**——`cleaner.py` 循环内对存在性的第二次判断（6/6 删除，外层已由封闭名单门把关）、`rag.py` 的空块支与循环自然耗尽支（7/7 改写成「上界 + 一次补尾」的 while）、`report.py` 的指标名 if/elif 链尾（8/9 改成三元组数据表）、`retry.py` 的第二条终止权威（19/19 删掉那记 break，只留「不可重试」那一支，循环改由 range 耗尽退出）、`quality.py` 恒非空建议表上的兜底判断（4/4）、`config_validator.py` 对唯一产地路径的去重（4/4）；**乙 5 格补链尾栅栏**——`ops.py` 的 dependency（+7）、`version.py` 的 version 与 backup 两处（+14）走`stderr` + `sys.exit(1)`，`export.py`（+9/2）与 `sampler.py`（+6）抛域异常，**「argparse 的 choices 与分支表脱节」从此出声，而不是静默少做一件事**；**丙 6 格本来就可达、只是没人测**——`api/main.py` 的未鉴权告警、`cache.py` 淘汰走到最后一条、`model_manager.py` 与 `search_enhanced.py` 的双检锁、`pipeline.py` 并行失败无断点、`visualizer.py` 的第二次加载，六格零产品改动，只补本进程内的取数路径。**等价改写凭什么敢删**：`chunk_text` 的位相同由 **3120 格网格**（尺寸 × 重叠 × 三种长度档）逐格对拍测试内复刻的旧 break 循环，依据钉在构造期不变式 `require_chunk_window`（size≥1、0≤overlap<size ⇒ step = size−overlap ∈ (0, size]）；而 coverage.py 的弧语义下「range 上界 + break」是**双权威**，被 break 跳走的那条出口弧永不被取⇒ 那一格不是没测到，是**量不到**，删支才是唯一能让分母闭合的处置。**两处被尺子逼回来的写法**：`api/main.py:197` 那条假支本来按计划起子进程验，实测否决——覆盖率只量本进程，子进程里跑得再对也不会让那一格闭合 ⇒ 改同进程 `importlib.reload` 双向（设 key 不出声 / 删 key 出声），收尾刻意停在「没设 key」那一档；名册那一支 `assert 7 + 5 + 6 == 18` 被 `test_test_hygiene.py::test_no_vacuous_asserts` 当场判恒真⇒ 改成读三张 dict 的现长度求和，**红的是尺子、错的是案面**。**同批撞到的三格史实**：`test_exceptions.py::test_no_bare_builtin_raises_left_in_package` 在 `export.py` / `sampler.py` 的裸 `raise ValueError` 喊红 ⇒ 换 `UnsupportedFormatError` / `DataValidationError`，并把 import 放在 else 支内（不动活行号）；`ops.py` 净 +7 把 `docs/augmentor/ARCHITECTURE.md` 模块树里那条指向 `cli/commands/ops.py` 的活引用（当时写的是第 159 行）被推出代码跨度 ⇒ a184 硬 0 档与 l79 同时红，按处方 3 同行重钉为 `cli/commands/ops.py:166`（净 0 行，`git diff --numstat` 复核）；`_dedup_by_err_paths` 那把普查判据按「真实源码必须 0 命中」的要求返工 ⇒ 判据**只认裸 `x not in err_paths`**，另以两条 `source.count(...) == 2 / == 1` 钉住改写后的现形，一处也没放宽。**注入取证 9 臂**逐臂内存改写、零落盘，每臂各自打红并按名册点名（含「把判据喂给自己的注入样本」那臂——每条形状判据都必须在真实源码上重新取到规定的命中数）。**共享工作树本轮第三次漂**：开工时并行会话把该工作树切到 `main`（L260–L265 不在那上面⇒ 8 格缺数与 9236 / 9316 的计数差全部归因到分支面，不是我的改动），三批改动按显式路径导出到仓外快照（`l266_code.patch` 14157 B 与新守卫 43690 B）并逐只文件验 `git diff --stat main optimism20261012` 为空⇒ 携带未提交编辑切回 `optimism20261012`；切分支前那两只未跟踪的 `auto-checkin/` 文件先证与分支上的 blob 逐字节相同再挪开，切回后由 git 原样带回。**换分支面这件事按用户拍板执行**（选项：切回 optimism 后收口）。**L232 那句 operative 自指按现量 826 → 829**：L265 的记账批 `19c96b420` 在 Backlog 区落了 +2 行而从未重钉这一格，真值已漂到 828，本轮清单行再 +1 ⇒ 829，落点以 `sed -n '829p'` 复核仍是含 `docs/augmentor/API.md:1139` 的那一句——A333 那句「这一形没有尺子」在本轮第二次拿到实测。**门禁**：代码批落盘后的全量定标（逐字 `python -m pytest tests -q -p no:cacheprovider`）**9362 passed / 0 failed / 10 skipped in 280.57 s**，覆盖率行逐字 `TOTAL 14382 0 3952 0`、`Total coverage: 100.00%`；9362 = L265 终态 9316 + 本轮新守卫 **46** **恰等** ⇒ 没有一支被删或被改名绕开。定点批次 **426 passed / 8 skipped**。wall 与 ⑯ 那把 311.60 s 只差负载噪声，按性能轮口径（位相同 + 计数判据）不作结论。本轮**零新 Backlog 行**、零定级格改动，Backlog 剩余笔数以 l242 那把尺子的读数为真相（不手数）。前端三门禁**不适用**（`web/` 零改动，不是漏跑）。⇒ 详见循环日志 L266
- [x] **L267** `4b3854f47` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**「调用方给的字符串被当成正则模式编译」那一族普查并接线——出厂面（`augmentor/` + `api/` + `cli.py`）里第一实参不是字符串字面量的 `re.<fn>` 调用按 AST 现量恰好 2 格，逐格定性其权威面**：`augmentor/search_enhanced.py:568` 那一格的模式串来自调用方（HTTP 请求体的 `query` 配 `method="regex"`，或 CLI 的 `--query`），改前 `except re.error` 只打一行 warning 就返回空 matches ⇒ 坏模式读成 **200 + `total_matches: 0`**；这与 `normalize_filters` 的 docstring 里已经写明的 A27/A28④ 同一形状，而那一格项目早有判决（形状不对就地报错）⇒ 本轮按既有判决接线，不另立待拍。改后抛 `DataValidationError` ⇒ **400** 并点名模式串与位置（实测 `无效的正则表达式 '[invalid'：unterminated character set（位置 0）`），CLI 面 `augmentor/cli/commands/data_ops.py:168` 同判据出声、经 `main()` 变成 `错误: …` + 退出码 1。**为什么必须走域错误而不是让它传播**：`re.PatternError` 的 MRO 实测 `['PatternError', 'Exception', 'BaseException', 'object']` 不含 `ValueError` ⇒ 按 `api/deps.py:494` 的分派表会被降成 500 固定文案。`augmentor/validation.py:721` 那一格（`forbidden_patterns` 的预编译）模式来自出厂 `PRESET_RULES`，而 `ValidateRequest` 只有 `preset`（新守卫逐字段核过，没有 `rules` / `forbidden_patterns` 两键）⇒ 调用方的串传不进去，坏模式仍原样传播，「构造时不抛、第一条数据才抛」的惰性时机由既有用例钉着。新守卫 `tests/unit/test_user_supplied_pattern_l267.py`（**12 支 / 零 parametrize** ⇒ l97 的 438 / 317 与 l98 的盲面 116 / 四档 61-38-11-11 / 豁免 25 一格未动）三面：名册（`(相对路径, 函数名)` 集合精确相等，另以每格一条源码锚点 `count == 1` 把「格子还在原位」与定性绑在一起）、吞异常形状检测器（`except re.error` / `re.PatternError` 体内无 `raise` 者计数，出厂面实测 **0**）、行为差（空语料也拒 ⇒ 参数校验先于数据面；合法模式 `公.*租房` 仍 2 条、`contains` 把 `[invalid` 当普通字符串仍 0 条 ⇒ 判据没有扩宽）。**注入取证 3 臂**逐臂改写、逐臂 sha256 复原并复验磁盘：臂 1 把 `raise` 换回 `logger.warning` ⇒ **8 支红**（本守卫 6 支 + `tests/unit/test_search_enhanced.py:116` / `:300` 那两支旧契约用例，那两支本轮按新契约**改写**而不是删）；臂 2 在 `api/routes/dataset_tools.py` 尾部追加一个永不调用的 `re.compile(query)` ⇒ **只有名册那支红**（证明普查对新增格有牙齿，且不牵动行为断言）；臂 3 保留 `raise` 但把消息里的模式串去掉 ⇒ **2 支红**（消息面与 HTTP 面）。产品改动**净零行**（`search_enhanced.py` 10/10，`git diff --numstat` 复核）：全仓对该文件的活引用最大 491 与 491-553 都在改动点之上，且改前改后各 10 条语句 ⇒ 语句数与分支数都不变，覆盖率两维残差仍为 0。`docs/augmentor/API.md` 的搜索端点行**同行追加** 400 契约（净 0 行，避开该文件活引用 244 / 282 / 952 / 1139）。前端面零改动（`web/src/services/api.ts` 的响应拦截早已按「4xx 的 detail 是可 actions 的判决」取 `detail` ⇒ 那句 400 原文照到 UI），三门禁**不适用**。**新立一笔 A345**：`method="regex"` 对模式串的长度与复杂度无上限、`run_in_thread` 无超时也无取消，而 haystack 来自调用方可上传的数据文件——同一请求体两个方向都可控；实测 `(a+)+b` 在 `"a"` 重复串上 n=14 / 16 / 18 / 20 / 22 的耗时 0.0008 / 0.00316 / 0.01324 / 0.05305 / 0.20834 s（每 +2 字 ×4），`SearchRequest.query` 实测照收 200,000 字 ⇒ 定级 M，待拍。⇒ 详见循环日志 L267
- [x] **L268** `117517d22` +（账本批即本条所在提交）`test,docs(augmentor)`：**账本自指「本账第 N 行」那一形第一次有尺子——A333 已收口（L268），它的 ①② 两形移交新立 A346（点号主张 / 括弧首是散文）**：新守卫 `tests/unit/test_self_line_claim_l268.py`（**14 支 / 零 parametrize** ⇒ l97 的 438 与 317、l98 的盲面 116 / 四档 61-38-11-11 / 豁免 25 一格未动）。**面怎么量**：站点 = 匹配「本账第 + 数字 + 行」的物理行，一行多处按多处算，两卷现量 **13 处且全部在卷二**；卷一这一形 0 处，它用的是序数「本账第 6 次」那一族（现量 **8** 处），由 `test_the_ordinal_shape_is_not_a_line_claim` 钉成「序数在场而行号档为 0」，证明那个 0 是量出来的、不是短语根本不存在。**三档判据全部可复算**：`LIVE 1 / STALE 5 / BLIND 7`——同物理行带 `文件名:行号` 锚、且主张那一行的内容里确实出现同一个锚才算 LIVE；有锚而读不到是 STALE（历轮当时的现量，按纪律不追改也不美化）；那一行根本没有锚是 BLIND，**这一档 7 格就是 A333 剩下的机械判不到的那一半**，本守卫把它钉成逐格点名而不是留给印象。**站点身份为什么不用物理行号**：每一轮都在清单区与 Backlog 区插行，用「第几行」当名册键会把尺子自己钉成陈旧读数——那正是本尺子要抓的缺陷形 ⇒ 键是 `(卷, 区段, 轮号, 主张的那一行)`，区段取该行自己的轮号（清单行 `^- \[x\] \*\*L(\d+)\*\*` 或所属 `^### L(\d+)` 块标题），它随轮次固定、不随插行平移；代价与 A333 立账时点名的完全一致：**名册随账本变长而每轮都要更新**，本轮的账本批就被自己的尺子红一次。**operative 那一格有独立定位器**：全卷同时含 `DiffResult` 与那条 API 文档引用的物理行现量**唯一** ⇒ 断言写成「散文里的数字 == 按内容重算的数字」，而不是把散文抄一遍，所以定位器本身也有牙齿（找不到或多于一个就 `AssertionError`）。**本轮普查顺手量到的两处历史形状**：L267 的账本批把 operative 先钉成 833、其记账批按现量改回 832（那一轮的 ⑮ 记着），而**同一行的括号链里那一格归因至今仍写着 833、值列表末位写着 832**——同一行两个数不一致，是这一族的第二种形状；本轮把链上那一格按 HEAD 现读补成「当时重钉为 833，记账批改成 832」的两句式（同行改写净 0 行，不抹历史读数）。**五臂自证全部内存改写、零落盘**：数字 off-by-one ⇒ `LIVE` 掉到 0；在它之上插一行（就是「上一轮插行把自指平移」那一形）⇒ `LIVE` 掉到 0；把锚那一格的内容改掉 ⇒ 同一数字不再为真（判据读的是内容不是数字）；新增一条无锚自指 ⇒ 面 +1 且名册对称差恰好点名那一格；喂进「本账第 7 次」那种序数 ⇒ 面一格不动，证明判据不是无差别前缀匹配。**本账批动的三处形状**：A333 的定级格按既有词汇收成划掉字母加闭合词加轮号，L232 清单行那句 operative 与它的括号链由 832 按内容定位器重钉为 835（清单行 +1、A346 连带它的分隔空行 +2 ⇒ 目标被推 3 行），链尾自此列出**二十个值**。**门禁**：新守卫 14 支全绿；全量定标（逐字 `python -m pytest tests -q -p no:cacheprovider`）**9388 passed / 10 skipped / 0 failed / 6 warnings（249.53 s）**，与上一轮的 9374 恰好差本轮新增的 14 支；逐字 `Total coverage: 100.00%` ⇒ 详见循环日志 L268
- [x] **L269** `e3baad45a` +（账本批即本条所在提交）`fix,test,docs(api,augmentor)`：**两处「写坏面」同轮接线——鉴权比较函数的 ASCII 限制与配置文件的整份截断写；同轮的只读安全普查把 A341 那条在册缺口第二次量成现量，并新立 A347（写端点默认匿名可达，待拍）**。① `api/deps.py::verify_api_key` 从前把两个 **str** 直传 `secrets.compare_digest`，而那条实现「对 str 只接受纯 ASCII」：一个含 0x80–0xFF 的 `X-API-Key` 请求头当场抛 `TypeError`，抛点在依赖解析里、路由自己的 `except` 拦不到 ⇒ 已配密钥的部署上**每一个写端点都能被一个请求头稳定打成 500**；更糟的是密钥本身含非 ASCII 时，逐字正确的请求照样 500，鉴权面整体不可用。改成 UTF-8 字节档比较（等值语义一字未动，只是把那条限制绕开），**同行改写、净 0 行**，账本 `api/deps.py:58` 与 `:466` 两处活引用一格未平移（这是 L217 / L219 那条「插行会平移历史行号」纪律在本轮的正面用例）。② `augmentor/config.py::save_config` 从前是 `open(path, 'w')` 整份截断直写：打开那一刻目标就被清空，整个写窗口里盘上是半份 YAML，而它的读侧把坏内容读成**一个错误答案**——`api/deps.py` 装配时对 `OSError / YAMLError` 那一档直接回落成「文件里什么都没有」，下次拿到的是出厂默认而不是用户配置。按 `atomic_write` 模块 docstring 里那条判据口径（只看读侧拿到截断内容时的行为，「变成错误答案」必收）这一边必收；**A182 的逐面收之所以没有它，不是豁免而是从来不在面上**——那把普查尺数的是「`open(...,'w')` 紧跟 `json.dump`」，YAML 边在 `json` 面之外。改走既有 `atomic_write_text`（L163 为 JSONL 导出边加的那把），行尾按 `os.linesep` 显式展开，写盘字节与改前**逐字相同**。③ 新守卫两把：`tests/unit/test_api_key_charset_l269.py` **17 支**（站点 / 判决 / 线路三面闭合，三支零落盘削弱臂；`TestClient` 走 httpx 会把非 ASCII 头在客户端就拒发，实测 `UnicodeEncodeError`，所以线路面下到 **ASGI scope** 用原始字节头取证，否则本轮会写成「只有 SDK 面」的假结论）、`tests/integration/test_config_write_atomic_l269.py` **11 支**（形状 / 字节 / 中断三面，三支削弱臂；零漂移钉的办法是截下同一次调用的 `data` 实参、用改前写法重放一份再逐字节比，行尾那一维按平台分档写明「POSIX 上是空转」而不是硬说两平台都有牙）。④ 只读普查的结论：产品码里没有 `subprocess` / `shell=True` / `eval` / `pickle` / `yaml.load`（零命中，命中全在 `scripts/` 与 `tests/`），`backup_id` 的删除与恢复只作索引键不拼路径 ⇒ 无注入面；**A341 第二次量成现量**（`vector.storage_dir` 只要 `require_string`，`vector` 又在可写七节里，数据根白名单被配置字段整体旁路），本轮**未动它**——改它要把 200 变 400，是对外契约变更；新立 **A347**：`AUGMENTOR_API_KEY` 未设置时写端点整体匿名可达，`api/main.py` 启动只打告警（这是既有契约「零配置即可用」，改它是把默认部署的可用性赌上去），**待用户拍板**。⑤ 门禁：全量定标 **9416 passed / 0 failed / 10 skipped / 6 warnings in 348.56 s**、`TOTAL 14383 语句 / 0 缺数 / 3952 分支 / 0 偏支` 与 `Total coverage: 100.00%`（用例闭合 = L268 终态 9388 + 本轮新守卫 17 + 11，**恰等**）；六把账本与文档守卫 **154 passed / 8 skipped**，与 L268 终态逐格相同；前端三门禁**不适用**（`web/` 零改动）。⇒ 详见循环日志 L269
- [x] **L270** `7c5666a90` +（账本批即本条所在提交）`test,docs(augmentor)`：**「这枚提交在不在分支上」全账无人量那一格第一次有尺子——A344 按它原格写定的修法 ① 结案，剩下的重钉口径与本轮新量出的两形盲区各立一行**：新守卫 `tests/unit/test_ledger_hash_reachability_l270.py`（**380 行 / 19 支 / 零 parametrize** ⇒ l97 的 438 与 317、l98 的盲面 116 / 四档 61-38-11-11 / 豁免 25 一格未动）。**面 = 两卷账本所有被反引号包住的 hex**（形状尺直接 import l264 的那一把，两把尺子不许各写一份），按 `## 循环日志` 边界分清单区与日志区：**现量 634 处 = reachable 459 + dangling 26 + stash 2 + not_a_commit 147 + ambiguous 0**，唯一 hex 379 枚。**判据取 `git rev-list --branches --tags --remotes` 的可达面而不是 HEAD 祖先**：A344 原文写的是 HEAD 祖先，但共享工作树里并行会话会把 HEAD 切走（L265 与 L266 各被咬一次，见 L266 块 ⑮），拿 HEAD 当判据就会把「别人切了分支」误报成「账本写错哈希」；本轮实测两判据在现面上**给同一批 7 枚对象**（对账维钉在 `test_the_two_criteria_agree_on_the_current_face`，一旦分叉就红）。**26 处游离点名逐格名册、全部集中在 L265 那一轮**（清单行 3 处 + 日志块 23 处，成因是并行会话对同一分支的两次 amend 与一次 reset，L265 块 ⑰⑱ 已逐枚写破），**名册外多一处即红**——那一条是整把尺子的牙齿。**与 l98 的分工**：那把哈希槽尺子经 l79 的常量只读卷一 ⇒ 卷二清单区这 120 处此前无人问（`test_l98_reads_only_volume_one` 现场钉）。**与 l264 的分工**：它那 120 处是本尺面的真子集，差集 282 处 = 卷一日志区 277（那卷的日志块标题不是 `### Lnnn` 形状，开不出块）+ 卷二五个块标题行。**四条注入臂全部在内存里改判据或改面、零落盘**：摘掉一枚真祖先 ⇒ 它那一格立刻从 `reachable` 走到 `dangling` 并成为名册外缺陷格；同一枚已知游离对象换个区段身份点名 ⇒ 缺陷格非空（名册不是「游离的一律放过」）；不带反引号的裸哈希双向证明不进面；编造一枚哈希落进 `not_a_commit` 那一档而**不**成为缺陷格 ⇒ 这就是本轮立 A349 的那格盲区。A344 的 ②（那批游离点名重钉成什么、`git gc` 之后怎么办）**不在本轮决定** ⇒ 移交新立 A348；本轮另立 A349（`not_a_commit` 只数总量不逐格点名）与 A350（同一个编号在日志要点里被写成与表行不同的事，全账无对账尺）。⇒ 详见循环日志 L270
- [x] **L271** `c2735dd69` +（账本批即本条所在提交）`fix,test(augmentor)`：**「客户端把名字写坏了」这一族输入此前拿两种码，500 那一支把客户端的错冒充服务端故障——本轮按操作系统自己填的 `errno` 收齐这一档，同时把 L269 ② 记的那组归因量成三条假话**：新守卫 `tests/unit/test_oserror_path_name_l271.py`（**14 支 / 零 parametrize** ⇒ l97 的 438 与 317、l98 的盲面 116 一格未动）。**判据只认 `errno`**：`EINVAL` / `ENAMETOOLONG` 是「这个名字在任何账户下都不可能成立」，与 `ENOSPC` / `EACCES` / `EROFS` 那种服务端状态不同，所以前者进 400、后者照旧 500；注入型 `OSError("磁盘读失败")` 的 `errno` 是 `None` ⇒ 不进这一档（既有 500 档七支套件 337 支全绿是这件事的外部证据，本轮一格服务端状态都没收松）。**文案不许转发 `str(exc)`**：`OSError` 的原文实测带着部署根绝对路径，正是 A155① 防的那格，所以 400 也用固定话术。判据住一处（A77）：`to_http_error` 与 `raise_internal_error` 的 34 处收尾分支共用 `is_rejected_path_name`。**端到端只用超长名**：`a\nb.json` 在 POSIX 上是合法文件名，拿它做断言等于钉一个平台特定的读数。**L269 ② 那组归因被现量推翻三条**：①「空白名 truthy 会直接当 `backup_id` 落盘」——`resolve_within_roots` 第一刀就是 `not name.strip()` ⇒ 空白名当场 400；②「NUL 一路走到 `resolve()` 抛`ValueError` 而 `_safe_data_path` 的 `try` 不覆盖它 ⇒ 500 而不是 400」——实测 `resolve()` 不抛、闸把带 NUL 的名字**放行**（回 `exists=False`），500 也不成立，写侧拿的是 400（`embedded null character`，由操作系统抛 `ValueError` 走原文照回支）、读侧拿的是 404「文件不存在」；③ 超长名与含控制符的名字才是真的那一形（实测两支都折成 `errno 22`，不是 `ENAMETOOLONG`）。**顺带量到 Windows 保留名不成立**：`CON` 在完整路径下就是一个普通目录项，`os.stat` 读得到、`os.remove` 删得掉，L269 ② 担心的那一形在本机不成立。⇒ 详见循环日志 L271
- [x] **L272** `59bb04cac` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**请求侧 float 旋钮碰到非有限浮点时，四只 SDK 咽喉有三种归宿（500 / 200 静默错答 / 无声 `true`）——本轮把这一族共引到同一份权威，并把 14 只旋钮 × 四形的 HTTP 归宿整表钉死**：新守卫 `tests/unit/test_nonfinite_float_knobs_l272.py`（**44 支 / 零 parametrize**）。**判据只加一维，不换文案**：`validation.require_ratio` 此前只判 NaN，无穷是靠闭区间**顺带**挡住的⇒ 一旦出现「下界判死、上界开放」的旋钮（`min_scale_gain` 定义域 `[-1, +∞)`）`float('inf')` 就一路走进比较式；本轮改判 `math.isfinite`，而有限越界那句判决一字未动（既有 `match=` 断言逐支存活）。**四只咽喉**（判在 SDK 而不是判在端点，因为 CLI 与 API 共用这几层）：`DatasetOperations.sample` 的 `ratio`（改前 `int(len × inf)` 抛 `OverflowError`，它**不是** `ValueError` 子类 ⇒ 端点 500；`-0.5` 在 systematic 折成 `items[:-5]`、stratified 折成 0 条，两支都 **200**——有限越界这一维本轮**不修**，它是 **A292** 的三结局形状）、`gate_dataset_health` 的 `pass_rate_min` / `duplicate_rate_max` / `completeness_min`（门口只判了 `pass_rate`，三兄弟此前无人判）、`ImpactEvaluator.is_beneficial` 的 `min_scale_gain`（改前 `scale_gain < NaN` 恒 False ⇒ `beneficial` 无声判成 `true`）、`OutlierDetector` 的 `threshold`（手抄 `threshold <= 0` 换成同族唯一权威）。**现量表把本轮的边界写清楚**：`raise_internal_error` 那一族（**A280**，待拍）的 `dedup` / `benchmark` 四形全部 500，422 渲染崩那一族（**A311** 的新现量）在 `leakage/check` 上连响应体都没有——两族按**观测行为**整格钉死，本轮一格不松。**同族既有口径不擅改**：`require_positive` / `require_seconds` 的 docstring 明文接受正无穷 ⇒ `outliers.threshold=+inf` 仍 200，它要和 A280 同批拍。**探针侧量到一格方法坑**：`client.post(json=...)` 发不出 NaN（httpx 客户端就 `allow_nan=False`），必须按 `requests` 的默认口径发 `json.dumps` 裸体，否则量到的是客户端拒绝而不是服务端归宿；而 `{'x': NaN}` 恰是 Python 标准库认为合法、RFC 8259 认为非法的那一档，真实客户端默认就这么发。**顺带量到写侧没有有限性判据** ⇒ 新立 **A353**（`atomic_write_json` 把 NaN / Infinity 原样落盘，本项目的读侧能读回并继续算门禁，严格 JSON 解析器却当场拒 ⇒ 自家工具链看不见这一格）。零漂移：四只被改模块的既有 228 支全绿，名册内合法值（`ratio=0.0` 仍交 0 条、`0.5` 仍 5 条）逐支复验。**修正批 `34b525ee5`（同一轮的第二支代码批，本轮真实返工）**：接线批顺手把闭区间那一维也判了 ⇒ 改前就合法的有限值（`pass_rate_min=5.0` / `-1.0`、`duplicate_rate_max=99.0`、`completeness_min=-3.0`、`min_scale_gain=-2.0`、`ratio=2.0` 的 systematic / stratified 两支）一并变 400，被两把**既有**守卫（`test_health_gate_wiring_l235.py` 3 支、`test_sample_aggregate_rag_wiring_l230.py` 2 支）当场抓住 ⇒ 四只咽喉的调用点全部改成显式 `minimum=-float("inf"), maximum=float("inf")`，只留有限性判决，范围那一维交回 **A292** 并新立 **A354** 待拍；守卫补三支范围面反向证据（收紧即红）。⇒ 详见循环日志 L272
- [x] **L273** `587022b11` +（账本批即本条所在提交）`fix,test(augmentor)`：**写侧早就原子了，缺的是「读 → 改 → 写」整段的串行化——两次原子不等于一次原子：200 条数据集上 20 支并发 `PUT /api/data/update` 盘上只落 1 处，而那 20 支全部拿到 `success: true`**：新守卫 `tests/integration/test_rmw_serialization_l273.py`（**626 行 / 23 支 / 零 parametrize**）。**普查面 = 全仓「整份读回 → 只改自己那一格 → 整份写回」的函数**，判据不是「有没有原子写」而是「两次原子之间有没有串行化」；现量 7 处写者逐格点名（`api/deps.py::_read_modify_write`、`augmentor/backup.py` 的 `DatasetBackup.backup` / `delete_backup`、`augmentor/version_control.py` 的 `DatasetVersionManager._load_index` / `create_version` / `delete_version` / `set_current_version`），修后名册 **7 处 / 0 处未保护**。**锁的三个选型都是量出来的**：① 键取 `normcase(abspath)` 的**按路径**锁而不是全局锁（不同数据集之间没必要排队）；② `threading.RLock` 而不是 `asyncio.Lock`——同步写者（备份索引、版本账）与异步路由碰的是同一批文件，只有共享的按路径线程键能让他们互斥，而可重入是必需的（外层罩整段与内层自保的读点落在同一个键上）；③ 整段读-改-写**放进一支工作线程**，因为「在 `await` 上持有线程锁」是错的。权威住一处（A77）：`augmentor/atomic_write.py::path_lock` + `api/deps.py::mutate_json_file`。**改前 → 改后（仓外沙箱、单进程单 worker）**：`PUT` 5/20/50 的丢更新 **4/19/49 → 0/0/0**（50 那一档另有 3 支读请求被 `os.replace` 那一瞬折成 500）、`DELETE` 5/20 的丢删除 **4/19 → 0/0**、备份 20 路 `created=20 / index_rows=2 / files_on_disk=20`（`lost_rows=18`：盘上实实在在 20 份而索引只剩 2 行，那 18 份对 `list` / `restore` / `delete` 全部不可见）**→ 四格 `created == index_rows == files_on_disk` 全等**、`version_control` 模块级 20 路 `index_rows=1 / lost_rows=17` **→ 20 / 0**、`versioning.VersionManager` 72 支里的 `PermissionError` **1 → 0** 且 `vanished=0`。**顺带修掉一格 Windows 独有的误拒**：`Path.resolve()` 只在拿到句柄的那一次给路径戴 `\\?\` 扩展长度前缀 ⇒ 备份名落点判据把合法名字判成「指向备份目录之外」，冷启动（目录未预先创建）8 轮 × 20 并发共 **63** 支虚假 `DataValidationError`（逐轮 3/14/8/8/14/5/4/7）**→ 0**；判据收进 `augmentor/validation.py::path_lands_within`（剥前缀后按 `Path` 比较），L258/L259 那把包含判据普查因此从「两处实现」收成 **一处**、不走权威的拼接从 4 处收成 3 处。**候选被现量排除一格**：`POST /api/config` 3 轮 × 12 只不同键 `lost=0` ⇒ 它不是丢更新站点，本轮不接线也不立账，读数留在日志块。**四把既有守卫按红读数跟着代码走，没有一把被改松**（全量首跑 **11 failed / 9505 passed**：接缝位移 2 支、AST 顶层 1 支、包含判据实现计数 1 支、l265 普查常数 7 支）。**重钉历史行号引用时量到一把尺子的盲区**：A184 与 l79 只判「`file.py:NNN` 落在代码跨度内」而不判「那一行有没有被点名的构造」⇒ HEAD 上三处引用早已 off by 3/4/10 行而两把尺全绿，本轮按内容审计重钉时先算错两次（`146 → 153`、`372 → 380`，真值 159 / 390）才发现 ⇒ 新立 **A357**；另把「这把锁只保同进程」与「读侧撞 `os.replace` 那一瞬」各立一行（**A355 / A356**，与 A280 那一族同批待拍）。⇒ 详见循环日志 L273
- [x] **L274** `febbe275b` + `1ca3dd319` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**形状门只判「这一列在不在」，不判「这一列里的值是不是文本」——91 格定向探针实测 **28 格 500**（`TypeError` 20 / `AttributeError` 8），本轮按具名子类把它们全部折成 400，而 A280 那一族一格未动**：`[{"instruction": 1, "output": 2}]` 在 `api/deps.py::assert_dataset_shape`（只判顶层是 list、元素是 dict）与 `require_item_field`（只判这一列存在）两道上全是合法输入，于是它一路走进下游那些把 `item.get(key, "")` 当字符串用的写法——`"为什么" in 1` 抛 `TypeError`、`1.lower()` 与 `1.encode()` 抛 `AttributeError`，而端点收尾是 `raise_internal_error` ⇒ **调用方自己那份数据集让服务端报内部错误**，客户端只拿到 A155① 那句固定文案。**改前逐格读数**（仓外数据根，Temp `probe3.py`，13 臂 × 7 份语料 = 91 格）：500 **28 格**（4 份退化语料 × 7 只端点）、200 56 格、405 7 格（那条路径上压根没有端点，见要点 ⑭）；最深落在本仓内部的栈帧 8 个行号档、去重后**六只咽喉**（`sampler.py:60` 与 `:113`、`api/routes/data.py:182`、`quality.py:26` 与 `:28`、`dedup.py:223` 与 `:225`、`dataset_ops.py:259`）。**修法按值维补一道**：`augmentor/validation.py::require_item_text` 逐条逐字段判「在场且必须是 str」，判决点名第几条、哪个字段、实际类型与截断后的原值（整句 <400 字，防一条大值把响应灌满），并**原样返回入参**⇒ 五处接线全是同行包裹（`sampler.analyze_coverage` / `quality.batch_score` / `dedup.deduplicate` / `dataset_ops._deduplicate` / `routes.data.load_data` 的搜索支——最后一处是延迟 import，落不进同行写法）。**为什么不放宽父类**：门口三只收敛函数对「SDK 已判出来的参数错误」给两种归宿，`raise_internal_error` 那一族的 500 由 **A280**（待拍）在 L272 按现量钉死，按 `DataValidationError` 放行会把那些格子一起翻掉⇒ 新立 `DatasetTextError(DataValidationError)` 具名子类，`api/deps.py` 与 `raise_internal_error` 只认这一个名字；新守卫 `TestFunnelNarrowness` 同时钉「新类型 400 原文照回」与「父类成员仍 500」，缺一格都读不到本轮的边界。**为什么不 `str()` 归一**：归一在只读测量一侧只是含糊读数，在会改写数据的一侧是**静默删行**——`POST /api/dataset/merge` 带 `deduplicate=true` 时 `instruction` 为 `1` 与 `"1"` 的两条被折成同一份文本而互为重复、只留一条，`str(None)` 还造出一个名叫 `None` 的四字符假样本；判据沿用 A321 / L238 那句「静默降级比报错危险」，守卫里那一支把两条一起拒并验产物文件根本没被写。**零漂移**：改前 56 格 200 改后仍 200 且响应体逐字节相同（只有 `auto_test` 那 7 格差 `execution_time_ms` 与 `timestamp` 两个天生不确定的字段），缺列与空数据集仍按改前交 200（把「缺列」一并拒掉会把一批现在就成功的请求改成 400，那是 A292 / A354 那一维要拍的事）。**覆盖面不是只手挑的**：第一版自动普查（Temp `probe5.py`）把 `input_file` 与 `output_file` 填成同一个文件名 ⇒ 前一支把语料吃掉、后一支量到的是「文件格式不对」，三格假 500；`probe7.py` 改成**每格新立语料**并按 `app.openapi()` 枚举全部 POST/GET（146 格），这一族仍 500 的只剩 `POST /api/versions/diff` 一枚——它的 `VersionError` 是 `ValueError` 的子孙但**不是** `DataValidationError`，而 `raise_boundary_error` 的放行名单按后者判 ⇒ 与 A280 同族，本轮只按观测行为钉成现量并验它不回显调用方给的那两个名字。新守卫 `tests/unit/test_dataset_text_field_l274.py`（**624 行 / 41 支 / 零 parametrize** ⇒ l97 与 l98 两把普查棘轮读数一格未动）七面：接线名册（键是 `(文件, 函数)` 而不是行号）+ 出厂面调用点总数恰好 5、尺子自证（`_call_sites` 只认函数体内的调用，函数被改名或删除时**响**而不是静默数 0）、权威面 10 支、咽喉窄性 5 支、HTTP 91 格整表 + 反向证据（`visualize` / `sample` / `stratified` / `auto_test` / `profiling` 五臂钉「本轮一格未动」，谁把它们改成 400 就是越界）、注入臂 8 支全部零写盘、盲区在 CLI 面逐格量出今天仍抛的那一句。**新立四行待拍（本轮不自决，四条都改对外行为或路由装配）**：**A358**~**A361** 分别是「口径相反的一侧仍 `str()` 归一」「CLI 面给裸 SDK 消息」「不存在的版本 id 折成 500（A280 的新成员）」「挂在 `/` 的静态资源把所有非 GET 未知路径折成 405」。**在册活引用零平移**：`api/deps.py:58` 与 `:494` 的插入点在 566 之后、`validation.py:437` / `:721` / `:1021` 的插入点在 1119 之后、其余四只文件净零行（2/2 与 3/3），全部按 `Temp/l273/audit_refs.py` 逐格打印落点内容复验——那是 A357 点名的「尺子只判跨度不判内容」的手工替代。**门禁**：接线批后全量 **9556 passed / 10 skipped**（= L273 终态 9516 + 新守卫 38 + `test_exceptions.py` 的 `BUILTIN_COMPAT` 加一行 × 两支按表展开的用例）、修正批后 **9559 passed / 0 failed / 10 skipped in 331.94 s**（+3 = 残留成员那一类），覆盖率逐字 `TOTAL 14449 0 3966 0` 与 `Total coverage: 100.00%`；六把账本与文档守卫 154 passed / 8 skipped；前端三门禁**不适用**（`web/` 零改动）。⇒ 详见循环日志 L274
- [x] **L275** `b14a118c6` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**「这一笔进度属于哪个任务」住在共享单例的实例属性上**——`api/deps.py` 的 `_pipeline` 是进程内懒加载单例，`POST /api/augment/start` 把 `p.augment_dataset(...)` 丢进 BackgroundTasks，于是两次并发增强共用同一只 `CheckpointManager` 与同一份 `_process_errors`：实测 230 笔 `update_progress` 里 **170 笔落在对方断点上**，B 的 `completed_indices` 因此拿到 **150 个越界下标**（值域 0..199 而 `total_items=50`），A 自己那份盘上停在 `processed_items=0`，另留一枚没人认领的增量文件；errors 那一侧同一形——单跑 120 条时报告里 **139 条 error 全部属于对方** **修法：让状态跟着断点对象走，不跟着「谁最后加载」走**——`_RunState`（`__slots__` 三格）作为**非 dataclass 字段**的私有属性挂在每只 `CheckpointData` 上，`update_progress` / `save_checkpoint` / `get_remaining_indices` / `get_progress` 各加**尾部可选参**（全部既有调用与测试一字不动即可编译），三只 ambient 属性委托给 `_active_state`（无参老写法照旧能用，直接调用方不破）；`augment_dataset` 改用运行局部 `errors` 列表并把它交给 worker。**盘上格式零改动**：`_RunState` 不是字段 ⇒ `asdict()` 键集合一字不变，旧断点文件仍读得回来（两支守卫分别钉这两格）。**普查面按各自 docstring 的处方重钉，没有任何一把尺子被改松**：全仓「构造器之外对 `self.X` 的赋值」现量 **129 处**，其中落在增强跑路径上 **45 处**，另有 **5 处** `self.X.Y=` 的嵌套批评态——本轮只收「断点归属」与「errors 归属」两格，余下四格立账不修（**A362** ambient 进度面读「最后加载的那一份」、**A363** 每次请求改写共享 visualizer 的输出目录、**A364** 残差共享批评态名册、**A365** 并行消费侧按计数配对），四格都写明「本轮只立、行为差未量或未做并发对撞」。**新守卫** `tests/unit/test_run_scoped_checkpoint_state_l275.py`（**325 行 / 14 支 / 零 parametrize**）：AST 名册 `EXPECTED_PRODUCT_CALLS` 逐格钉「增强跑路径上每一只调用点带不带身份实参」，两支合成反证臂证明这把尺子会咬；行为面有栅栏同步的两线程互不越写、一支**ambient 反证臂**（不传 `checkpoint=` 就当场复现污染，证明名册那一格吃重）、增量文件归属、`get_progress(ck)` 与 `get_remaining_indices(ck)` 逐只读、`save_checkpoint(ck_a)` 不许删 B 的增量，以及 errors 运行局部与实例默认两支。**既有四处断言/替身按新口径重钉（是口径变更不是放宽，每处留反证）**：`test_pipeline_error_report_l206.py` 的 `test_internal_list_is_reset_too` 改判「每次跑完实例 sink 归零」、`test_serial_branch_records_under_lock` 换成 `test_serial_branch_writes_the_run_local_list`（串行支只有调用方一个线程，那把锁是死同步），另补 `test_parallel_branch_appends_under_lock` 用缩进嵌套钉「并行支那一笔 append 确实在 `with self._lock` 里」；`test_pipeline_branches.py` 一处 `errors` 计数按运行局部列表重钉；另把 `tests/unit/test_micro_branches_l68.py:200` 那只替身的形参补成与真实签名同 arity——它少一格时并行侧不是报错而是**整批永久挂死**（那一格另立 **A365**，本轮不自决修法）。**仪器自己的一格假绿（本轮最要紧的方法论事故）**：errors 那一档探针的放行条件原本盯 `len(p._process_errors)`，接线后错误写进运行局部列表、实例属性不再增长 ⇒ watcher 永远等不到信号、第二支任务在第一支跑完才被放行，「并发」静默退化成顺序而读数全绿——改成按线程各自数 `update_progress` 并打印重叠窗口才拿到真并发证据。**定位引用**：`checkpoint.py` 头部插入类与三只属性 ⇒ 卷一 :292 那三处历史行号整体下移 **88 / 95 / 95** 行，按 A127 口径重钉为 `augmentor/checkpoint.py:308` / `:359` / `:430`（其内的增量触发见 :467），净 1/1 同行改写；六把账本与文档守卫首跑红 **3 格**（A184 硬 0 档与其注入臂、`test_doc_line_refs_l79`），重钉后 **154 passed / 8 skipped**。**改后逐格复量**：250 笔进度**全部显式带身份**、**0 笔落对方**，A 120/120 与 B 40/40 各归自己、实例残留 0 条，两份产物与单跑逐字节可比（只差 `task_id`，而它本就由输入路径推出）；单跑零漂移——既有断点与流水线测试 **189 支**全绿，改动只有那三处口径重钉。门禁：逐字 `python -m pytest tests -q -p no:cacheprovider` 读出 **9574 passed / 0 failed / 10 skipped in 321.72 s**，覆盖率表逐字 `TOTAL 14491 0 3970 0` 与 `Total coverage: 100.00%`（14449 → 14491、分支 3966 → 3970 是本轮 `checkpoint.py` / `pipeline.py` 净增的行与支）；9574 = L274 终态 9559 + 新守卫 **14** + l206 那一支新增的并行锁判据 **1**（用例数只增不减，没有任何一支被删）。同一条命令在替身补形参之前连续两次停在 62% 零 CPU（见 ⑰）。六把账本与文档守卫 **154 passed / 8 skipped**。；前端三门禁**不适用**（`web/` 零改动，不是漏跑）。⇒ 详见循环日志 L275
- [x] **L276** `518d4d056` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**L275 立账的两格同轮收口——A365（并行增强消费侧按「计数」配对 ⇒ 任一 worker 少投一次整批零 CPU 永挂、`future` 的异常又从未被读而无声吞掉）与 A362（进度端点读「最后加载的那一份」断点）**。A365 按用户拍板的候选①「条目失败继续」修：`augment_dataset` 并行支由「共享一只 `result_queue` + 不带超时的 `get()`」改成「**每条目一只信箱**」（`mailboxes = {idx: Queue()}`），消费侧从 `futures[future]` 拿「这一笔」下标去读那一只，`future.exception()` 出声、具名常数 `RESULT_WAIT_SECONDS` 的有界等待取代无限等待——少投递那一笔记 `ResultNotDelivered`（四键与 worker 自报 `error_info` 同形）并**继续**，整批不再挂死，多投递一条不再记到别人名下，成功支与失败支各投且只投一次；A362 走候选②（只动文档不改响应面）：`docs/augmentor/API.md` 把 `GET /api/augment/progress` 的口径写明是「管理器此刻最后加载的那一份断点」而非「本次请求启动的那只任务」，端点补 `task_id` 那一档（候选①）仍留待拍。新守卫 `tests/unit/test_mailbox_paired_consumption_l276.py`（**433 行 / 19 支 / 零 parametrize**）三面（源码形状 / 行为 / 尺子自证；健康路径零漂移为修法判据）。**门禁**：全量逐字 `python -m pytest tests -q -p no:cacheprovider` 读出 **9593 passed / 0 failed / 10 skipped**（= L275 终态 9574 + 新守卫 **19**，用例数只增不减），覆盖率逐字 `TOTAL 14500 0 3972 0` 与 `Total coverage: 100.00%`；六把账本与文档守卫按红读数逐格重钉后定向全绿（读数见日志块 ⑪）。A365 / A362 两行由「待拍」转「已收口（L276）」；本轮产品文件只改 `augmentor/pipeline.py` 与 `docs/augmentor/API.md`，`web/` 一字未动 ⇒ 前端三门禁不适用。⇒ 详见循环日志 L276
- [x] **L277** `哈希待 L278 回填` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**安全审计轮的九格结案——A292 / A339 / A341 / A343 / A347 五格结案，A366 / A367 / A368 / A369 四行立账同轮收口**：用户 goal 原文「全面找出并修复验证 augmentor 中所有 bug 和漏洞，直到你完全找不出来为止」⇒ 此前标「待拍」的安全族按授权拍定并落地，**逐格都是对外行为变更**（非回环匿名写 403、限流不再信 `X-Forwarded-For`、JSON 请求体纳入体积上限、空作用域清单 200→400、`vector.storage_dir` 出白名单 400/403、`ratio` 越界三支同 400、带分隔符的备份名 200→400、`page` / `page_size` 小于 1 静默错答→400、`/api/data/load` 响应新增 `indices`、`augment_async` 的 `use_checkpoint` 由死参数变真接线、断点增量重放不再累加）；新守卫六只文件（`tests/unit/test_vuln_audit_l277.py` 24 支、`tests/integration/test_data_page_slice_l277.py` 17 支、`tests/unit/test_backup_name_path_segment_l277.py` 16 支、`tests/unit/test_empty_scope_http_l277.py` 11 支、`tests/unit/test_delta_remerge_l277.py` 8 支、`tests/unit/test_config_dir_containment_l277.py` 7 支、`tests/unit/test_async_checkpoint_resume_l277.py` 6 支）+ 前端 `Augmentation` 轮询的停表与重启两支行为守卫；A369 的反证走字节注入三臂（6 failed / 1 failed / 1 failed，finally 复原并断言 sha 与进入时逐位相同）。A354（另外四只旋钮的范围维）**不收**，理由与 L272 修正批的归因同格。在册活引用按现量重钉，两把尺（a184 / l79）回到硬 0，没有任何一把尺子被改松。⇒ 详见循环日志 L277
- [x] **L278** `哈希待 L279 回填` +（账本批即本条所在提交）
- [x] **L279** `哈希待 L280 回填` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**并发共享态罩锁两案与备份名的第八档**——A364 前两格（`quality.py` 参照集分次写、`context.py` 轮数「存-改-复原」）由只立不修推到改前改后成对实测并落罩锁，定级收成 `M 半修（L279）`；A373（`MemoryCache` 全类无锁、`_synchronized` 的豁免面定性是假话）立账同轮收口；A374（`require_within_dir` 族缺冒号档，实测 `C:foo` 被放行且落点名改名、`C:` 吞成基座目录本身）新立待拍。另收三档覆盖率缺口（backup 落点闸补臂、rate_limit 窗口滑动、upload_gate 非数字 Content-Length），把上一跑 99.97% 的回归推回 100.00%；`require_path_segment` 加第八档（冒号），关掉备份名「落盘改名 + 两 id 共用一只文件 + 按一个 id 删除毁掉另一条记录」那一格。两次返工如实入账：`backup.py:120-121` 的「死支应删」被 drive-relative 探针证伪，「后一次创建把索引里前一行整行顶掉」被不删双臂的别名探针证伪（真形是两行并存共用一文件）。详见日志块。`fix,test,docs(augmentor)`：**静态闸自身的六格缺陷与文档表格竖线口径**——`_read_repeat_bound` 三形当场炸 `ValueError` 且域外异常逃过产品入口那道 400 折（实测 `a{1,2,3}` / `a{ }` / `a{²}`：`str.isdigit()` 认 `²` 而 `int()` 不认）、`{0,}` 与 `{,}` 开区间量词整族漏判、字符类内含 `[` 让指数形漏判（POSIX 嵌套类那一支是 `re` 里根本不存在的语法，实测 `findall("[[:alpha:]]", "a]:b[c")` 得 `['a]']`）、`{0,1}` 一族误伤、一枚认不出的 `(?` 扩展就让整条模式不判（`(?i)(a+)+b` 与 `()(?(1)a)(a+)+b` 两族绕过）、`(?aiLmsux)` 纯开关指令未认；`_group_extension_end` 改成「认不出就按普通组继续扫」，并用实测曲线证伪「条件组属不判族」的旧口径；新守卫 `tests/unit/test_pattern_blowup_l278.py` 的 `TestParserBranches`（21 格不下判断 + 8 格现下判断，逐格点名）与危险面 26 / 合法面 20 两张现量。文档族：`docs/augmentor/API.md` 表行竖线按 GFM 转义，列数判据收进 `test_api_doc_endpoint_coverage_l210` 的 `column_count` / `split_cells` 做单一权威，`test_api_doc_field_hints_l223` 共引。A345 的 ① 那一半落地并把定级格收成半修；新立 A370 / A371 / A372 三行。
- [x] **L280** `哈希待 L281 回填` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**`DiskCache` 把「判过存在再动手」的四格与单条超上限时毁掉整本缓存的自逐出**——`delete` / `clear` / `_enforce_size_limit` / `stats` / `_load_metadata` 五处全是「先判存在（或先 glob）再动手」的形状，同一只类里另一侧却已经在用 `missing_ok=True`，形状自相矛盾；确定性替身量到三形必现 `FileNotFoundError`，第四处由新守卫的 AST 名册自己点出来。另收一格更贵的：单条超过 `max_bytes` 的写入会把别人的缓存逐光、再把自己刚写那只文件也删掉，`set` 静默退化成 no-op。A375 立账同轮收口，A376（跨实例共用目录时 `_save_metadata` 全量重序列化 ⇒ last-writer-wins、索引丢键、磁盘留孤儿、上限对孤儿失明）新立待拍。新守卫 `tests/unit/test_disk_cache_missing_file_l280.py`（24 支 / 零 parametrize）三面。
- [x] **L281** `哈希待 L282 回填` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**把 L280 那把「判过存在再动手」的尺子推到全仓产品语料上，量出十格并逐格收口**——只读 AST 普查（仓外探针，133 只产品文件）在 `atomic_write`（2）、`backup`（2）、`checkpoint`（2）、`data/image`（2）、`data/audio`（2）里点出 **10 格 / 5 只文件**：先 `exists()` 再裸 `unlink()` / 裸 `stat()`，窗口里文件被并发删掉就抛穿。最重的一格在承诺面——`delete_backup()` 那一格抛出的 `FileNotFoundError` 会跳过紧随其后的索引重排、被路由折成非 200，而 `DELETE /api/system/backups/{backup_id}` 的 responses 明文写着「索引有条目而备份文件已丢不算这一档，仍回 200」。另立并同轮收口 A377 / A378 两笔（A377 是这十格的一族账，A378 收 `atomic_write` 的 `finally` 裸 `unlink`：清理失败（Windows 上杀软/索引器占句柄）会**顶替**上面那次真正的写失败，与模块 docstring「异常原样上抛」的承诺相反）。新守卫 `tests/unit/test_missing_file_family_l281.py`（34 支 / 六处 parametrize）三面：全仓普查尺零命中 + 十格逐格名册 + 确定性消失路径替身的行为对账（含 HTTP 臂与「改前同形复现」两支）。
- [x] **L282** `哈希待 L283 回填` +（账本批即本条所在提交）`test,docs(augmentor)`：**修 L281 那把全仓普查尺自己的语料面，并用修好的面把 A378 那一族（`finally` 裸清理）从全仓量一遍**——① `product_sources()` 把 `cli` 当**目录**列进语料根，而它在仓库根的真形是 `cli.py` ⇒ 旧实现 `if not base.is_dir(): continue` 静默跳掉那一格，「全仓 133 只产品文件」实际少一只，而缺的那只恰好不在任何子树里、任何「按子树数」的对照都看不见它；修面把语料根拆成 `PRODUCT_DIRS` + `PRODUCT_FILES` 两组声明并**逐根验真、形状对不上就 `AssertionError`**（禁止 `continue`），现量 133 → **134**（红读数逐字 `assert 134 == 133`，由仓外探针 `Temp/probe_l282_face.py` 把常数拨回旧值、在新面上实跑取出，非推断），另补一支**与 l177 那把普查尺按集合相等跨尺对账**的尺——只各钉规模常数挡不住：本尺 133、那把 134 时两边都是自己那份常数上的绿。`cli.py` 进面后普查读数仍是 0 ⇒ L281 那十格结论未被推翻，被推翻的只有「133 就是全仓」。② 新尺 `finally_cleanup_hits` 把「`finally` 里抛出的异常顶替上面那次真正的失败」这一族推到全仓：量到**恰好一格** `augmentor/excel_write.py::export_table_to_excel` 的 `wb.close()`，而它**未坐实成缺陷**——openpyxl 的 `close()` 写作 `if hasattr(self, "_archive"): self._archive.close()`，探针照产品骨架实测「`wb.save()` 因目标是一枚目录而抛 `PermissionError`」那一档时 `close()` **静默**（读数 `ARM_SAVE_FAIL_DIR: close_silent` / `top:PermissionError` / `ctx:None`，真正的写失败没被顶替）⇒ 按「不为不会发生的情形加兜底」不动产品码，把它钉成豁免名册里逐格点名（文件 + 函数 + 调用文本）的唯一一格，配四档自证臂（裸调用点名 / 自罩 `try` 不误杀 / `missing_ok` 不误杀 / 换动词仍认）与一支「豁免格仍量得到」防空集对账。新立并同轮收口 A379 / A380 两笔（A379 是尺子口径族：普查面静默缩水；A380 是这一族在全仓的普查读数与那一格的实测判决）。守卫 `tests/unit/test_missing_file_family_l281.py` 34 → **39 支**（五支新臂零 parametrize ⇒ l97 / l98 那几格一字未动，同一次定点复跑含 a184 与 l97 共 72 passed / 5 skipped）。本轮**未改任何产品文件**（纯尺子面）⇒ 门禁读数与 v7 基线的关系、限流器族复核的负结论、插行平移与逐格重钉明细见循环日志 L282。
- [x] **L283** `哈希待 L284 回填` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**三格产品缺陷坐实并同轮收口，六族普查回到负结论**：①`api/routes/status.py` 的降级支从前**连日志都不留**（本轮按 AST 现量 `api/` 面 74 支宽 `except`，无声格恰好一处就是它）——异常原文被丢进 `unknown/False` 那格默认回执而服务端一字不出，补 `logger.warning` 让原文只落服务端日志、不进响应体，与 `deps` 的 500 支同口径；②`augmentor/atomic_write.py` 的 `_PATH_LOCKS` 按路径**单调增长、无上限、无回收**，而 docstring 写着「有界」——实测（`Temp/probe_l283_locks.py`）20 000 格 **6.25 MB**、每格约 312 B，且把路径对象全丢、文件也不存在之后**一格都不退**，而键来自请求方能点名的数据集文件名 ⇒ 补 `MAX_PATH_LOCKS=10000` + 超限共用一把 `_SPILL_LOCK`（只多串行化、永不放宽，保住「上限之内同一路径 `is` 同一把」与真 `threading.RLock` 返回类型这两条既有钉），**不选 eviction**：回收必须在「取出锁」与「拿到锁」之间开窗，那正是丢更新那一族的形状；③`augmentor/version_control.py::DatasetVersionManager.get_history` 是 A77 真值性判据族的**漏网孪生格**（孪生格 `versioning.py:86` 早有 `require_count` 与「以前是 `if limit else entries`」的注）：`limit=0` 交出**全部**、`limit=-1` 经切片交出「丢掉最新一条」，两种都是形状合法、内容错误的静默答案 ⇒ 共引 `require_count("limit", limit)` 并把 `if limit` 改成 `if limit is not None`。**负结论六族如实记**（路径穿越 10 载荷 × 6 端点形状 = 60 攻：0 回显泄露、根外磁盘 0 变化；越界拒绝回执 8 类文案逐只读：0 处含绝对路径或部署路径；静态「落盘函数无守门」71 只里 64 只 ⇒ 口径作废，改走行为面；读-改-写无罩锁静态候选 33 格逐格读：只有配置那格属待拍；上传面 `max_upload_bytes=256 MiB` 双判 + `index` 边界 + `page/page_size` L277 已判；`cache.py` 的 `md5` 键按 L32 既有口径复核为不动）。**配置并发那一族量到而不自决**：20 支并发 `POST /api/config` ⇒ **16×200 + 4×500**（`os.replace` 的 WinError 5 顶穿 5 次 × 20 ms 退避上限），磁盘终态一组整批丢（`dedup.threshold` 停在原值 **0.9**、`quality.threshold` 落 **0.116**），单发两次都 200 且 `ignored_keys: []` 排除「键本就被拒」，另量到一次成功写入把仓档 `config.yaml` 的 **206 行注释 / 457 行**重写成 **0 / 186**（`sha 79c18e44c614 → 160de1281ef0`）⇒ 属 `mutate_json_file` docstring 明写的「跨进程那一档」同族，新立 **A384 待拍**、本轮只量不动。**新守卫 2 只 9 支**（`test_path_lock_registry_cap_l283.py` 6 / `test_status_degrade_logs_l283.py` 3）+ 常驻族守卫 `test_count_knob_validation.py` **+1 臂 ⇒ 237 passed**。**五臂注入取证全红且逐字节复原**（A1/S1/S2 + V1 `Failed: DID NOT RAISE DataValidationError` / V2 `assert [...] == []`）。**尺子一律按红读数重钉，没有一把被改松**：l177 语料面第七次计数 **398 → 400**（红读数逐字「语料规模现量 (133, 398)，实得 (133, 400)」），卷一两处活引用按处方 3 重钉（`status.py:28-35 → :40-47`、`version_control.py:159 → :160`，各 1/1 净零行、卷一 2891 行未动）。**普查尺三轮返工**（重复踩坑，不美化）：`app.routes` 不展平 `include_router` 连踩两次（常驻守卫早已写明这一格）、`inspect.getsourcelines` 首项是行列表不是文件名、AST 属性名 `ast.Dict.elts` 不存在。**门禁**：v9 **1 failed / 9911 passed / 11 skipped / 344.52 s**（红的正是 l177 那格语料规模）→ 重钉后 v10 终态 **9913 passed / 0 failed / 11 skipped / 359.57 s**，覆盖率 **100.00%**、`TOTAL 14883 stmts / 0 miss / 4132 branch / 0 brpart`（与 v9 的 `14882` 之差恰为 **+1**，即 ③ 里那行新加的 `require_count` ⇒ ①② 的改动是同行改写与既有语句重排，没给分母添负担）；前端三门禁**不适用**（`web/` 零改动，不是漏跑）。**无 commit 授权** ⇒ L277–L283 **七批**全部留在工作树，卷一/卷二哈希槽一格未回填。⇒ 详见循环日志 L283
- [x] **L284** `哈希待 L285 回填` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**错误回执的渲染崩溃两族坐实并同轮接上——非有限浮点把 422 顶成裸 500、孤立代理把回显顶成 `UnicodeEncodeError`；同族另两格量到而不自决 ⇒ 新立 A385 / A386 待拍**：①数值 query 送 `nan` / `inf` / `-inf` ⇒ FastAPI 默认 422 处理器把**原始输入**嵌进 `exc.errors()` 的回显，而 `JSONResponse` 以 `allow_nan=False` 序列化 ⇒ `ValueError: Out of range float values are not JSON compliant` 顶穿**错误处理器本身**，对外只剩一支没有判决正文的裸 500（实测 5 支端点形状：`/api/dataset/search`、`/api/dataset/compare`、`/api/leakage/check`、`/api/dataset/impact`、`/api/dataset/evaluate`）；②请求文本带**孤立代理**（Python 的 json 解得开、UTF-8 编不出）落进任何回显路径 ⇒ `UnicodeEncodeError`，而它是 `ValueError` 的子类 ⇒ 一支兜底盖住两族，这层继承关系由 `test_exception_identity_is_value_error` 钉成断言而不是写在散文里。修法只在**错误面**消毒：`api/main.py` 注册两支 `exception_handler`，先原样委托框架默认处理器（合法回执一字节不改，由 `test_legal_422_payload_unchanged` / `test_legal_http_error_bytes_unchanged` 钉住），仅当抛 `ValueError` 才兜底——非有限浮点换成**同名 token** `NaN` / `Infinity` / `-Infinity`（不折成 `null`，让现场仍读得出），字符串走 `backslashreplace`（可逆、且点名到具体那一枚）而不是 U+FFFD。**成功面刻意不消毒**（不换 `default_response_class`）：否则「服务端自己算出 NaN」那一族缺陷会被静默抹平。**同族另两格按口径只量不接** ⇒ `A385`（search 的**成功路径**回显同一枚孤立代理，崩在 `type_adapter.py:651 dump_json` 的 `PydanticSerializationError` ⇒ 仍是裸 500，接法要改对外语义）、`A386`（`/api/dataset/semantic-search` 把内部异常原文当 400 判决送出，`detail` 逐字带着 `'utf-8' codec can't encode character` ⇒ 既外泄实现文案又撞「4xx 中文 / 422 英文」双轨口径，与 A306~A312 同族）。**新守卫 1 只 41 支**（`test_validation_finite_echo_l284.py`：复现面 8 / 兜底牙齿 3 / 委托证明 2 / 孤立代理面 15 / `_finite_safe` 单元面 13）；牙齿臂走**进程内换处理器**（`_use_default_handler` / `_use_plain_http_handler` 且必须复位 `app.middleware_stack`，首轮踩过「栈在首次请求冻结 ⇒ 换了不生效」）⇒ 零仓库写入，不需要注入器的复原步。**尺子一律按红读数重钉、没有一把被改松**：l177 语料面第八次计数 **400 → 401**，l97 `MEASURED_CALLS` **453 → 458** / `MEASURED_READABLE` **332 → 336** / 盲面 **121 → 122**，l98 `TIER_PLAIN` **61 → 62** / 盲面测试数与已见数 **116 → 117** / 豁免 **33 → 34**（全部由本轮新守卫那 5 支 `parametrize` 造成，逐格读数出自仓外探针 `Temp/probe_l284_census_delta.py` 直接 import 那两把尺子打印桶归属，不按「字面量 ⇒ 可读」推）。**另有两把上一轮就该红而没红的尺子**：l264 与 l270 的量面在**卷二**，而轮次纪律第 2 节那把「六文件」定向名单里没有它们 ⇒ L283 记账批把卷二清单区与日志区各推了站点之后无人复核，本轮按红读数一并重钉（l264 站点 **138 → 139**、`not_a_commit` 档 **21 → 22**；l270 站点 **664 → 666**、档内 **147 → 149**、卷二清单区 **132 → 133** 与日志区 **143 → 144**、唯一 hex **397 → 398**、`MEASURED_L264_SITES` **138 → 139**），并把「l264 / l270 不在六把名单」这一格程序缺口记进日志块 ⑨。**文档面**：`docs/augmentor/API.md` 两处同行改写（422 那一行记孤立代理转义、400 那一行记 `/api/dataset/convert` 的兜底并写明成功路径回显那一格不在覆盖之内、已立 `A385`）⇒ `git diff --numstat` 由 22/9 走到 **23/10**、净 0 行。**门禁**：写账本前全量基线 v1 **9954 passed / 11 skipped / 0 failed / 383.94 s**、覆盖率 **100.00%**（`TOTAL 14926 stmts / 0 miss / 4148 branch / 0 brpart`）；账本批后的定向与全量终态见日志块 ⑬。**无 commit 授权** ⇒ L277–L284 **八批**全部留在工作树，卷一/卷二哈希槽一格未回填。⇒ 详见循环日志 L284
- [x] **L285** `哈希待 L286 回填` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**日志注入族（CWE-117）坐实并同轮收口——请求体里一枚 `\n` 能在审计流伪造第二条日志行，消毒装在渲染的最后一跳；同族另两格要动对外可观测口径 ⇒ 新立 A387 / A388 待拍**：①`POST /api/system/dependency/datasets` 的 `name` 是任意 JSON 字符串，`augmentor/dependency.py` 把它原样拼进 f-string 交给 `logger.info` ⇒ 实测载荷 `evil\n[2026-10-17 02:00:00] INFO ROOT admin login success` 让审计流**凭空多出一行「管理员登录成功」**，时间戳/级别/logger 名全由攻击者写；同一条通道上 `\x1b` 序列原样进流，够驱动终端重绘与擦行；**族范围是量出来的**，三条负数结果把口子钉准：uvicorn access log 打 `get_path_with_query_string(scope)`（源自 raw_path）里换行仍是 `%0A`、经 TestClient 的 `request.url.path` 也未落真换行、HTTP 帧禁止头值含 CR/LF ⇒ `X-Request-ID` 那类透传不构成拆分 —— 远程可写面**只有请求体**。②修法落在**汇**而不是产品面现量 200 条 `logger.*` / `logging.*` 调用点：只覆写 `logging.Formatter.formatMessage`（`logging_setup.ControlSafeFormatter`），因此 `logging.format` 写成什么串都被覆盖，判据与配置面解耦；装配只有两处——唯一的 Formatter 构造点 `build_formatter`（`config.yaml` 与 SDK 直构同过）与 API 入口 `build_handler`（3.12 的 `logging.basicConfig` 实测不收 `formatter=`，逐字 `ValueError: Unrecognised argument(s): formatter`，只能走 `handlers=`）。允许集 `[\x00-\x08\x0a-\x1f\x7f-\x9f]`、**Tab `\x09` 刻意排除**（保留对齐是既有可读性口径），命中写成 `\xNN` 两字符字面量 ⇒ 可逆（`unicode_escape` 反解）、不吞码点；干净行走 `search` 预检后**原对象返回**。对外语义一字未动：登记回来的 `name` 仍是原文，消毒只治日志流（由 `test_the_response_body_still_echoes_the_raw_name` 钉住）。③**代价同口径实测**（都用 `%(message)s`，20000 次 / 轮 × 7 轮取 min，配对 5 轮全同号）：干净 0.471 → 0.724 µs（**+0.236 µs**）、含一枚 `\n` 的载荷 0.483 → 1.704 µs（**+1.202 µs**，只有真载荷多付这一档）；换成生产格式串（含 `%(asctime)s`）时基线涨到约 2.4 µs，同一笔固定开销摊成 **+0.41 µs / +17 %**；`str.translate(dict)` 那版 7.627 µs 直接否掉。④**新守卫 1 只 23 支**（`test_log_forging_l285.py`，刻意**零 `parametrize`** ⇒ l97 / l98 的盲区名册与三档常数一格未动）分六类：单元 / 渲染形状 / 装配 / 族闭包普查（`api/` + `augmentor/` 里裸 `logging.Formatter(` 构造 == 空集，牙齿走**仓外副本**）/ HTTP 端到端 / 非恒真（把消毒器换回裸 `Formatter`，同一支请求当场回到 2 行；`record.getMessage()` 仍含原文，证明改动在 formatter 不在 record）。既有 100 支 logging 装配守卫（`test_logging_wiring_a97.py` + `test_cli_logging_wiring.py`）本轮复跑 **100 passed**，消毒没把 A97 那套接线判据碰红。⑤**尺子一律按红读数重钉、没有一把被改松**：l177 语料面第九次计数 **401 → 402**（红读数逐字「语料规模现量 (133, 401)，实得 (133, 402)」，就是本轮新守卫那一只文件；四处同行改写 ⇒ 净零行，产品面 **133** 第五次确认不动）；l97 / l98 的盲区名册与三档常数**一格未动**——本轮那支新守卫刻意零 `parametrize`（`grep -c parametrize` 整文件现量 0），且这两把尺子本轮**没有**被复跑取读数（子集跑必触 `RulerNotProven` 采集下限闸 ⇒ 断言不执行），所以只按字面证据记，不声称「复跑读数一字未变」；子进程解码普查尺 `tests/unit/test_subprocess_text_decoding.py` 在本轮新守卫那处子进程调用点报「新的未登记子进程调用点（不传 encoding）」⇒ 处置是给自己的用例补 `text=True, encoding="utf-8", errors="replace"` 并给子进程 `PYTHONIOENCODING=utf-8`，**没有**进豁免名单、**没有**放宽那把尺子；本批落账后 l242 / l253 / l263 / l268 的逐格重钉见日志块 ⑫。**文档面**：`docs/augmentor/ARCHITECTURE.md` 新增 §3.35（+41 行，落在 `## 4. 核心数据流` 之前）⇒ 对 `ARCHITECTURE.md:1232` 的活引用在其上、不平移，`DEPLOYMENT.md:226-227` 那格未动。**门禁**：写账本前全量基线 v1 **9977 passed / 11 skipped / 0 failed / 356.56 s**、覆盖率 **100.00%**（`TOTAL 14939 stmts / 0 miss / 4150 branch / 0 brpart`，与 L284 终态的 14926 之差 **+13**，即 `logging_setup` 尾部那三件与 `build_formatter` 那处同行改写）；账本批后的定向与全量终态见日志块 ⑬。**无 commit 授权** ⇒ L277–L285 **九批**全部留在工作树，卷一/卷二哈希槽一格未回填。⇒ 详见循环日志 L285

- [x] **L286** `哈希待 L287 回填` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**「名字被当成路径片段」那一族的最后一格 + 空集静默零写入的委托面孪生格，两格同轮收口；同族另两格量到而不自决 ⇒ 新立 A389 待拍**：①`require_within_dir` 的落点判据只问「在不在基座之内」，而 `path_lands_within` 判的是形状、**相等也算通过** ⇒ 名字 `.`、`./`、空白包裹的 `.`、Windows 盘符相对的 `C:` 一路放行，落点就是基座目录本身；接上 `versioning.delete_version` 那一格 `shutil.rmtree` ⇒ 实测 `delete_version(".")` 一次删掉整个版本根（全部版本目录 + `history.jsonl` + `current.txt`，探针盘上读数 `root alive False / WIPED`），而 HTTP 面 `DELETE /api/versions/%2E` 与双重编码的 `%252E` 都能把 `'.'` 原样送进路径参数 ⇒ 不是「只有 SDK 直接调用才会自伤」那一档。改法是把这一档下推进权威本身：判据仍只有一处，写成正向 + 反向两次 `path_lands_within`（不换成直接比 `==`，那会踩回 L273 现量到的扩展长度前缀形状不一致那一格；也不新增第二个名字出生点，备份名那把强权威的「一处出生」口径一字未动）⇒ 全部调用点一起获得，`path_lands_within` 与 `require_path_segment` 各自一格未改。②`export_enhanced._export_csv` / `_export_tsv` 以「空集直接 return」开头：格式化函数一个字都不写，而 `export()` 照旧返回 `output_path` 与 `item_count: 0` —— 与 `export.py` 原生 CSV 分支刚按 **A343** 立起的那道判据自相矛盾，而 `tsv` 是 `POST /api/data/export` 打得到的格式 ⇒ 现在与原生档同形（产物必建、空集 0 字节、没有真实字段就不谎造表头），`json` / `jsonl` 本来就写产物，本轮一并钉成回归钉。③新守卫两支：`tests/unit/test_dir_self_landing_l286.py`（拒绝档逐点名 + 版本根盘面事实 + HTTP 三档编码 + 「`%252E` 到处理函数手里确实是 `'.'`」这一可达性前提 + 忠实复刻改前判据的 rmtree 牙齿臂 + AST 现量「判据写成两处调用、不许直接比 `==`」）与 `tests/unit/test_export_enhanced_empty_set_l286.py`（逐格式产物 + 非空档回归钉 + 两条 CSV 路的列投影差异按实测量在案 + 摘掉判据产物当场消失）。④同族两格**量到而不自决**：冒号档那三格（A374 的 ①③④）要拍的仍是「这九处的放行算不算对外契约」，并行支路咽判决（A340）要改的是回执形状，本轮一格未动；新立 **A389** 记电子表格公式注入（CWE-1236）那一族四条写边，定级 S 待拍。
- [x] **L287** `哈希待 L288 回填` +（账本批即本条所在提交）`fix,test,docs(augmentor)`：**A389 按用户 2026-10-17 拍板① 收口——四条交付边各按自己格式的原生手段挡住电子表格公式注入（CWE-1236），判据只留一处产地**：CSV / TSV 没有类型位，`validation.sheet_text_cell` 给会被执行的字面串前面加一个前导单引号（Excel / LibreOffice / Numbers 认它是「文本」标记），这一改写**进交付字节**、读回那份 CSV 拿到 `'=1+1`，这条代价由新守卫第 5 组钉在账上而不是藏起来；xlsx 有真正的类型位，`excel_write.text_only_cell` 用 `WriteOnlyCell` 把 `data_type` 显式设成 `s`，**一个字都不必加**、格子里的字面串与进入时相同（唯一例外是格内回车：xlsx 存的就是换行，两条路同形、非本轮引入，第 3 组按这笔记账）。前导字符集 `SHEET_FORMULA_LEADERS` 只有 `augmentor/validation.py` 一处产地，四条边共引，谁都不许手抄字符集。新守卫 `tests/unit/test_sheet_formula_injection_l287.py`（23 支 / 3 处 parametrize）八组面：三条 CSV 边逐边现量、TSV 两条边与 CSV 同判据、非字符串值一字未动、xlsx 产物零公式元素与类型位、表头那一格、json / jsonl 负面对照、读回后果、闭族普查三条（每一处 `writerows` 都过 `sheet_text_row`、每一处 `ws.append` 都过 `text_only_cell`、xlsx 容器的出口只许住 `excel_write.py`）、手抄判据两档普查（四条写边文件里一档字面量都不许有，其余产品树要两档以上才报）、两支牙齿（把权威换成恒等 ⇒ 裸的加一等式当场重现；把判据判成恒假 ⇒ 公式元素当场重现）。同轮另立两格且**都一格未动**：**A390**（xlsx 解压放大：Excel 读边两条入口除压缩后的上传字节上限外没有任何尺寸或比率判据；用户已给方向「按解压比 + 解压后尺寸拒」，阈值要现场量，本轮不落一个未量的读数）与 **A391**（CSV / TSV 的表头那一面：列名同样带前导交付给接收方，而给表头加前缀会改变读回侧用来配行的键，一处转义要两处同步 ⇒ 属对外可观测面，待拍）。尺子按红读数重钉七格（l177 语料规模、l97 三面、l98 三格），全部按各自打出来的红读数重钉、判据本体一格未松。**提交状态本轮改轨**：用户 2026-10-17 把「未授权 commit」改为「每一批都自动分批提交和推送」⇒ L277 起十一批的累积状态按族分四笔补交（产品面 `8c85ab0b3`、守卫面 `4a445da5d`、前端 `16a5dabc9`、两份对外文档 `6c51decca`），分支已推送 origin；那十格逐轮哈希槽无法自指回填（账本批与代码批合并了），占位按原样保留、这笔返工由日志块 ⑮ 统一记。

## Backlog A — 质量缺口（缺数 / 偏支 / 健壮性）

| # | 位置 | 内容 | 量级 |
|---|------|------|------|
| A201 | `augmentor/retry.py`、`augmentor/version_control.py`、`augmentor/models/base.py` | **已关闭（L101）**：16 条缺数 + 3 条隐藏偏支弧全清 | ~~S~~ 已关闭（L101） |
| A202 | `api/routes/quality.py`、`api/routes/export.py`、`api/routes/augment.py`、`api/routes/multimodal.py`、`api/deps.py`、`augmentor/config.py` | **已关闭（L102）**：11 条缺数（路由错误/降级支 + 依赖层降级键 + 配置保存容错）全清 | ~~S~~ 已关闭（L102） |
| A203 | `augmentor/cli/commands/version.py` | **已关闭（L103）**：delete 动作 8 条缺数（98-106）全清 | ~~S~~ 已关闭（L103） |
| A204 | `augmentor/preview.py` | **已关闭（L103）**：非原生格式的 DataValidationError 守卫支（177）清零 | ~~XS~~ 已关闭（L103） |
| A205 | 全仓约 40 支 | **已关闭（L114–L126）**：偏支 85 → 17，46 条逐支判定（24 条可达补测清零 + L126 删 5 条恒真守卫 + 余 17 条永久记档结构族） | ~~M~~ 已关闭（L114–L126） |
| A206 | `web/src/pages` | **已关闭（L104–L113）**：10 页各建组件测试，前端全量 13 文件 112 例全绿 | ~~L~~ 已关闭（L104–L113） |
| A207 | `web` | **已关闭（L128）**：路由懒加载 + 三 vendor 拆分本体已就位，补 `check-dist.mjs` 六格产物体检 + `npm run build:check` 入口 | ~~M~~ 已关闭（L128） |
| A208 | `docs` | **已关闭（L129）**：API 易混字段词典（9 字段）+ API 面排查 FAQ（6 问） | ~~M~~ 已关闭（L129） |
| A265 | `OPTIMIZATION_LOOP_2.md` | **新立（L197）**：账本「已完成」清单与循环日志的进度一致性**没有机器守卫**——L194 与 L50 两次漏写清单行都靠人眼发现。修法方向：从循环日志的 `### L\d+` 标题集合推导应有轮次，与清单行对账（防空转断言同 `test_cli_verdict_wiring.py` 的 S1==S2 型式）。**这是第二次同类，按纪律升级为待办而不是再忍一次** | ~~S~~ **已收口（L203）**：清单行与日志块三条不变量有机器守卫，进度声明面不再靠人眼 |
| A266 | `docs/augmentor/API.md` / `docs/augmentor/ARCHITECTURE.md` / augmentor 顶层 README | **新立（L197）**：契约面数字（端点总数、tag 计数、导出格式清单、前端页面数、事件循环守门条数）全靠人肉对账，L197 一次就抓出六处历史遗留错误。修法方向：从 `app.openapi()` / `ExportFormat` 枚举 / 前端页面目录**实推**后与文档断言相等（同 `TestExportFormatSurface` 先例） | ~~M~~ **已收口（L200）**：端点总数 / tag 计数 / 导出格式清单 / 前端页面数一律从实推对象对账 |
| A267 | `augmentor/visualizer.py`、`augmentor/config.py` 的 `VisualizationConfig.types` | **新立（L215）**：可视化这一族里除 `enabled`（L215 已接）之外还剩一对互证的死面。三条现量：① `visualization.types` 出厂五值 `["wordcloud", "length_distribution", "topic_cluster", "timeline", "quality_distribution"]`，全仓产品读者 **0**（grep 只命中 `config.py` 的定义与 `require_string_list` 校验，配置校验不算读者）；② `DataVisualizer.generate_timeline` 产品读者 **0**，只有 3 支单测调它；③ `generate_all_visualizations` 只出四类图，其中 `quality_distribution` 要 `scores` 形参，而两个产品调用点（`pipeline.visualize_dataset` 与 `cli/commands/analysis.py` 的报告模式）**都不传** ⇒ 出厂配置声称五类、阶段实际最多三类。**为什么 L215 不顺手接**：把 `types` 接成出图清单是一次**行为变更**而非零漂移接线——按出厂值接上，`timeline` 会今天第一次真的生成，产物多出那张图；`quality_distribution` 还要先定「scores 从哪来」。**判据要的是行为差，这里的「行为差」是新增产物而不是消费既有开关**，所以它属功能增强，得单独拍：要么按 `types` 生成（对外产物变化），要么承认清单虚假并从回显/文档降级。定级 M，待用户拍。 | M |
| A268 | `augmentor/semantic.py`、`config.vector.dimension` | **新立（L216）**：嵌入用的是零依赖的确定性字符二元组哈希，所以 `dimension` 就是配置值本身。接**真模型**嵌入（可选依赖 `sentence-transformers`）时要先回答两件事：① 模型输出宽度与本键冲突怎么收（模型给 384、配置写 768 时是拒配置、忽略配置，还是另开一只 `model_name` 键）；② 本机与 CI 都不装这个包，接上去等于写一条**任何没装它的环境都跑不到**的产品路径——正是本仓在收的「只写不读」形状。所以这条的前置条件是「先有一个装了包的定标环境」，不是先写代码。定级 M，待用户拍。 | M |
| A269 | `augmentor/vector/chromadb.py`、`augmentor/semantic.py` 的 `EXPLICIT_PERSISTENCE_BACKENDS` | **新立（L216）**：`persist` / `load` 只由 `FAISSDB` 实现，`ChromaDB` 那侧是 `PersistentClient` 自管（构造阶段就 `mkdir` 并注册集合）。L216 把落盘建库与状态读取定义为「有显式 `persist`/`load` 面」的后端专用，chromadb 在**构造之前**拿 400。要让 `vector.backend: chromadb` 真的可用，得先在装了 `chromadb` 的环境里量两件事：① 该后端的「索引是否已存在、多少条」怎么写成可测判据（今天的 `exists` / `count` 语义靠 `load()` 的布尔返回，那侧没有）；② 「GET 不落盘」这条承诺如何与「构造即落盘」的后端共存（要么给它一条只读的探查路径，要么把 `vector-status` 划成 faiss 专用面并回显清楚）。定级 M，待用户拍。 | M |
| A270 | `augmentor/multilingual.py` 的 `batch_translate` | **新立（L217）**：逐条翻译的异常被 `except Exception as e: logger.error(...)` **吞成「回退原文」**，返回清单与全部成功时**形状完全相同**——没有失败计数、没有逐条状态、没有非零退出信号。L217 接消费层时实测坐实：`model_backend=None` 时端点回 **200 + 整份原文**，调用方读到的是「翻译完成」。本层已在分批之前自保抛 `ModelNotConfiguredError`（后端整体缺失这一档出声了），但**逐条失败**这一档仍然沉默：真后端只对部分条目超时/被限流时，产物里混着原文而无人知道。修法是给 SDK 加**失败回执面**（`failures` 计数 + 索引列表，或逐条 `translation_status`），面一改就要同步 `TranslateResponse`、`test_api_openapi_contract` 与 `test_api_report_keys` 那把键集尺子——所以它是一次产品改动而不是接线。定级 M，待拍。**· 已关闭（L221）**：三面各落一层——`batch_translate` 从「返回清单」改成「返回报告」（`failed` / `failed_indices` / 本批**全量** `errors`；回退原文与长度、顺序对齐原样保留，只是不再无声），`translate_dataset` 把批内下标加 `start` 折成全数据集坐标并**独占**样本上限 20，`TranslateResponse` 声明回执四键。两条口径决策：① **部分失败仍走 200**——整批失败已由 `ModelNotConfiguredError` 落 400，而「25 条里 3 条没翻」既不是客户端错误也不是服务端故障，重发整份只会重复花钱，诚实读数放响应体；② 样本上限**不做成第五只配置键**——它是响应体尺寸的保护栏而不是能力开关，接一只键要规格表 / `config.yaml` / 节注册计数 / 普查尺子四处同步，却没有任何「按部署不同」的需求。本条目原文那句「同步 `test_api_report_keys`」**在仓内没有对应文件**（立账时凭印象写的），真正的键集尺子是 `test_api_openapi_contract.py` 的 `TestResponseModelDoesNotDropFields`，它守两个向（响应实到 + OpenAPI 声明），注入臂 A7 同时把这两支打红。 | ~~M~~ 已关闭（L221） |
| A271 | `augmentor/config.py` 余 10 只 `config.*.enabled` + 10 只非闸门键 | **新立（L217）**：B5 的前沿现在有了机器名单（`tests/unit/test_config_reader_census_l217.py` 的零读者档，20 键逐键带理由）。两族的处置方向不同，**都不许顺手做**：① 10 只 `enabled`（active_learning / benchmark / context / evaluation / expander / frameworks / multimodal / rag / sampler / tracker）出厂全为 `false`，接进阶段入口等于**关掉当前始终在跑的行为**，是对外破坏面（L214 已实测：接成端点拒绝门当场红 13 支），要拍的是「破坏面要不要开」而不是「怎么接」；② 10 只非闸门键**按旋钮到底在不在分三族**（L218 用 `inspect.signature` 与 `api/routes` 的模型字段逐键验真，本条第一版那句「各自对应的模块确有同名构造参数」是错的，七只键里多数根本没有那种参数）：**请求侧已有同名旋钮** 2 只——`evaluation.metrics` / `evaluation.reference_field` 就在 `/api/dataset/evaluate` 的请求模型上，把它们改成「请求 > 配置 > SDK 默认」的缺省来源多半零漂移（出厂清单与 SDK 回落值同为 `bleu / rouge_l / similarity`）。**这一格 L219 已收一半**：`evaluation.metrics` 接进省略支路（顺带把空清单在门口 400，因为 SDK 那行 `metrics or [内置三只]` 会把空值**静默补成三项**），余 `reference_field` 因请求默认是硬值 `"output"` 而必须先改契约，另立 **A274**；**SDK 有同名旋钮但产品侧没有调用者** 4 只——`active_learning.strategy` / `batch_size` 在 `ActiveLearningLoop.__init__`、`max_iterations` 对应 `run(iterations=...)`（默认 `None` ⇒ 1 轮，与出厂 10 不同，接线即行为变更）、`expander.strategies` 在 `DomainExpander.batch_expand`，但 `ActiveLearningLoop` 全树零实例化、`self.expander` 构造之后从未被用（**A273**），所以要先造消费者再谈接线；**根本没有同名旋钮** 4 只——`frameworks.frameworks`（两个集成类的构造器只收 `text_key` / `answer_key`）、`sampler.dimensions`（`analyze_coverage` 只收 `items`，四个分布硬编码）、`tracker.metrics`（只有逐条 `log_metric(metric_name, value)`）与 `visualization.types`（**A267** 在案，接上去是产物新增不属接线）。前两族是接线、第三族是加功能，逐族拍完再按族批量收，别做半族（L214 的「修了 5 处漏 1 处」教训）。定级 M。**· L220 之后第②族只剩「SDK 有旋钮但产品侧无调用者」4 只与「根本没有同名旋钮」4 只**：「请求侧已有同名旋钮」那一格（2 只）已整格收完——`evaluation.metrics` L219、`evaluation.reference_field` L220，并同轮补了原本不存在的 `evaluation.generated_field`（新增键自带读者，未给普查添新账）。现量：schema **70 键**、有读者 **49**、零读者 **21**、归档待处置 **18** = 闸门 `enabled` **10** + 非闸门 **8**（4 只「有旋钮无调用者」+ 4 只「无同名旋钮」）。第①族那 10 只 `enabled` 与 A273 那两个无调用者的属性仍是「要拍」而不是「要写」 | M |
| A272 | `api/routes/config.py` 的可写四节 / `multilingual` 节 | **新立（L217）**：L217 接的四只键里除 `enabled` 之外，**没有任何一只可以经 `POST /api/config` 改**——可写面至今只有 export / rag / vector / multimodal 四节，`multilingual` 要改只能改 `config.yaml` 重启。要么把 `multilingual` 开成第 5 个可写节（连带 `test_the_two_tiers_partition_the_writable_sections` 的键名单扩员、节内判据已有所以不用补），要么在 `docs/augmentor/API.md` 与回显面上写明「这节只读」。这是口径决策不是缺陷，所以先立账：现状下翻译的目标语言与批宽在部署期不可调，而 L217 的两条端点恰恰是运行时才消费它们。定级 S，待拍。 | S |
| A273 | `augmentor/pipeline.py` 的 `self.expander` / `self.tracker` | **新立（L218）**：管道在构造阶段建 `DomainExpander(self.model_backend) if self.model_backend else None`（`pipeline.py:146`）与 `ExperimentTracker()`（`pipeline.py:149`），而**产品树里再没有任何一处引用这两个属性**——grep `self.expander` 与 `self.tracker` 只命中那两行赋值，`api/`、`cli.py`、`augmentor/cli/` 全树对 `expander` / `tracker` 零命中，测试侧只有 `test_pipeline.py:412` 断言「无后端时 `instance.expander is None`」和两处给它塞 `model_backend` 的夹具。于是 `DomainExpander.batch_expand`（收 `strategies`、`topics_per_strategy`、`use_parallel`）与 `ExperimentTracker.log_metric` / `save_experiment` 这些**已写好并单测过的能力今天没有任何产品入口**，也正是 **A271** 里「SDK 有旋钮但没人转」那一族的根因。两条路要拍：① 接消费者（扩展走一条 `/api/dataset/expand` 一类入口、指标随增强结果落到版本面），② 承认死面并把构造删掉——后者会缩 `AugmentorPipeline` 的公开属性面，属对外契约变化，那 3 处测试引用要跟着改。构造本身近乎零成本（`ExperimentTracker` 明写「构造时不建目录，第一次记实验才建」），所以**这是消费条目不是性能条目**，别拿性能理由顺手删。定级 M，待拍。 | M |
| A274 | `api/routes/dataset_tools.py` 的 `EvaluateRequest.reference_field` / `config.evaluation.reference_field` | **新立（L219）**：L219 接 `evaluation.metrics` 时把同节的 `evaluation.reference_field` 留在原地，原因是它**接不动**。请求模型的默认是硬值 `reference_field: str = "output"`，调用方省略与显式传 `"output"` 在 API 面上是同一个形状，`request.reference_field or config....` 的 `or` 永远不会落到配置那一档——写出来就是一只假接线（配置改了读数不变，且没有任何测试能发现，正是本轮普查在收的形状）。真接法只有一条：把字段改成 `Optional[str] = None` 再走「请求 > 配置 > SDK 默认」，而这会同时改掉 OpenAPI 里该字段的**可空性**（定义上就变，不是推测），动到 `test_api_openapi_contract` 那把字段面尺子时要按现量重钉（尚未实测，故本条只立账不预判红几支），也改掉既有客户端「省略即 `output`」的语义。生成侧 `generated_field` 在配置里**没有对应键**，接完会出现「参考侧跟配置、生成侧跟硬编码」的不对称默认，要不要一起补一只 `evaluation.generated_field` 也是同一个口径决策。定级 S，待拍。**· L220 已收口**：两只字段名的请求默认改成 `Optional[str] = None` 哨兵，权威序 请求 > 配置 > 内置默认 才真的可达（省略与显式 `"output"` 从此是两种形状）；对称口径拍的是「补键」而不是留一对不对称默认 ⇒ 新增 `config.evaluation.generated_field`（出厂 `output`，运行时 `require_string` + 规格表 `non_empty` 与兄弟键同规）。零漂移前提不再由散文承担：dataclass 默认、仓库 YAML、规格表形状三臂各自钉住，另加「四列文本两两相似度不同」当地基。坏字段名两处出声（加载期拒空串、门口逐条带下标 400）。OpenAPI 可空性确实改了，但**没有任何一支既有尺子红**（`test_api_openapi_contract` 守的是响应面与端点覆盖，请求字段类型不在它刀口上），所以那把尺子的位置由新章 `TestRequestFieldsAreNowOmissible` 自己钉。⇒ 见循环日志 L220 | ~~S~~ 已收口（L220） |
| A275 | `docs/augmentor/README.md` 的配置段表格 | **新立（L220）**：这张表的「关键项」列与 `AppConfig` 的键清单**没有任何机器对账**——L220 新增 `evaluation.generated_field` 时，`docs/augmentor/README.md` 与 `config.yaml` 两处全靠人肉同步，而 `test_doc_contract_numbers_l200.py` 只核端点数、导出格式数、页面数那一类数字。这正是 **A266** 那句「契约面数字全靠人肉对账」在配置面上的具体形状，且它比端点面更容易烂：本仓 20 节 70 键每轮接线都在动。修法方向（与 A266 同族、可并表执行）：逐节从 `dataclasses.fields()` 实推键清单，与表格该列**逐节比对相等**，并同时核 `config.yaml` 的节/键集合（三侧同源的三角对账）。定级 S。**· 已关闭（L222）**：三侧对账落成 `tests/unit/test_config_three_side_l222.py`（59 支）。权威只有一份——节清单与键清单全从 `dataclasses.fields()` + `getattr` 实推（与 `test_section_registry_l77.py` 同口径），测试侧唯一手写名单是 `META_ROWS = {models, app}`，且每员要**同时**过两条机器验真（确实不是 dataclass 节 + 确在校验器 `META_TOP_SECTIONS` 白名单里），所以「把一只真节塞进例外名单」蒙不过去。`config.yaml` 那侧的判据不是「与 schema 逐节全等」而是**整节写全或整节注释列全**——这条是被文件自己纠正出来的：`logging` 三键刻意留在注释里（整节不存在时日志一字不变），第一版把它当漂移是我读错了产品口径。README 表格随之从 16 行 34 键补全为 22 行 70 键，并把「哪只键有人读」的权威明确留在普查守卫而不是文档里。对 HEAD 那份文档量出的真漂移 **21 红 / 38 绿**（5 整节缺席、13 节半节、恰好列全 2 节）；注入七臂各有牙齿，其中 A5「给 dataclass 加一只 init 键」复现的正是立账本条的那一步（L220 新增 `evaluation.generated_field`）。本轮量到的 `app` 节是另一码事，另立 **A276**。⇒ 见循环日志 L222 | ~~S~~ 已关闭（L222） |
| A276 | `config.yaml` 顶部的 `app` 节（`name` / `version` / `debug`） | **新立（L222）**：**形状本身不是新事实**——`OPTIMIZATION_PLAN.md` §2.4 的表、`README.md` 变更说明第 6 条与 `tests/integration/test_cli_validate_config.py` 的 docstring 三处都记着「`AppConfig` 不建模 `app` 段、没有任何代码读它」，而历史上做过一次处置是把它从必填**降为可选**（因为 `save_config` 写出的配置否则必然校验失败）。**本条立的是另一码事：这只死面的*处置*从未进过账，而且它对现有的死键普查尺子结构上不可见**——`AppConfig` 没有 `app` 字段（`dataclasses.fields()` 实测不含它），`ConfigValidator` 又把它列进 `META_TOP_SECTIONS` 免检，于是 B5 那把「只覆盖 schema 键」的普查尺子永远不会把它报成零读者键，它也不会被加载器装配。当前后果：改这三键不影响任何输出，`GET /api/config` 的响应面里也没有它们（该端点逐字段从 `p.config` 即 `AppConfig` 拼装，见 `api/routes/config.py` 的 `get_config`，没有字段就没有格子）。三条处置方向都动得到对外文件，所以先立账：**① 接成节**（新增 `AppMetaConfig` 三键并接进 `/api/config` 与 `--version` 一类读取面 ⇒ schema 70 → 73 键，l77 节注册、规格表、普查尺子与本把三侧尺子都要按现量重钉）、**② 从出厂文件删掉那一段**（改动用户可见的配置面，且 `ConfigValidator.META_TOP_SECTIONS` 要跟着收窄、`save_config` 的「保留未建模顶层键」那条行为也一并失去对象）、**③ 维持现状并把「产品侧零读者」写明**（本轮 `docs/augmentor/README.md` 表格已按③落了说明，但键本身仍在那儿）。本轮选③而没有顺手删或顺手接：前者是无声地改变出厂形状，后者是一轮做不完的对外契约变化。`META_ROWS` 的验真臂把这个形状钉在明处——白名单成员必须「确实不是节」。定级 S，待拍。 | S |
| A277 | `docs/augmentor/API.md` 逐端点的「人读字段提示」列 | **新立并同轮收口（L223）**：这张四列说明表是 68 条端点里 dataset 与 system 两组共 31 条的**字段级承诺**面（哪个字段省略会落配置、哪个键可能几千个键、哪种坏值 400），但仓内此前没有任何机器读过它的字段名——`test_api_doc_endpoint_coverage_l210.py` 只钉「端点路径集合与说明存在性」，`test_doc_contract_numbers_l200.py` 只核数字。缺这一格的直接后果本轮就撞上了：总览散文与三条说明行把写盘类端点的输出字段写成 `output_path`，而真实字段是 `output_file`（`split` 是 `output_dir`），`output_path` 只是路由里的**局部变量名**——文档在教调用方发一个会被 pydantic 拒掉的键。这是 A266 / A275 那句「契约面数字与字段全靠人肉对账」在逐端点说明面上的最后一格，同一族、同一种修法：**权威只有一份**，字段名从 `app.openapi()` 实推（请求体属性递归过 `$ref` / `allOf` / `additionalProperties` / `items`，再加 `parameters[].name`），契约面接不住的名字再要求落进四档有位置的证据（该端点 import 闭包内的字符串字面量、`def` / `class` / 模块名、`fields(AppConfig)` 的键、CLI parser 的选项）。定级 S。**· 已关闭（L223）**：守卫 `tests/unit/test_api_doc_field_hints_l223.py` 23 支，文档 4 处一并订正（全文再无 `output_path`）。两件事值得留在账上而不是埋在日志里：① 字面量档**必须逐格限作用域**，全仓字面量会把 `output_path` 洗白（它确实是 `augmentor/backup.py:199` 与 `augmentor/export_enhanced.py:143` 的返回键），跳数预算由 `test_hop_budget_is_load_bearing` 钉住；② 尺子首跑的 3 格无据里有 2 格是**尺子自己的 bug**（相对导入层级换算错，`augmentor` 包内 `from .x` 全没解析进闭包），判据修好之后才剩下那 1 格真缺陷。⇒ 见循环日志 L223 **· L231 补记**：本轮把 `POST /api/system/backups/{backup_id}/restore` 的响应落点键 `output_path` 照实写进 `docs/augmentor/API.md`——它是那条端点**真实的响应键**（同族其余五条写盘端点叫 `output_file`，`split` 叫 `output_dir`），与本条订正的「请求侧教人发幽灵键」是两件事。那道「整篇不许出现」的禁令因此按现量**收窄**成「dataset 七条写盘族一行不许出现 + 全文只许 restore 那一行出现，多一行即红」，原缺陷照旧喊红（注入臂 I5：把 convert 那行写回 `output_path` ⇒ 2 支红）。⇒ 见循环日志 L231 | ~~S~~ 已关闭（L223） |
| A278 | OpenAPI 里 `additionalProperties: true` 的自由子对象 | **新立（L223）**：契约面不是全知的。响应模型里凡是 `Dict[str, Any]` 的子对象，在 OpenAPI 上就是 `additionalProperties: true` —— **键名压根不在 schema 里**，机器推不出来。本轮实测的形状：`ImpactResponse.before` 与 `after` 两个子对象（`api/routes/dataset_tools.py:383-384` 声明为 `Dict[str, Any]`，内容是 SDK 那份统计画像的原始 dict），它们内部有哪些键只能由散文说明，新尺子对这类名字只能走「字面量档」，而字面量档是**逐格限作用域**的——一旦某个自由子对象的键来自更深的模块（超过两跳），它就会被报成无据，而那不是文档错。当前后果：客户端只看 OpenAPI 拿不到这些子对象的形状，文档写漏也没有机器兜。两条方向：**① 给自由子对象立最小形状**（哪怕只声明 `count` / `types` 这一类稳定键，其余留在 `additionalProperties` 里），**② 维持自由并由 A277 那把尺子加一档「自由子对象白名单」**，把「这一格只能靠散文」写成可证伪的名单而不是含糊过去。本轮选②的现状（白名单还没立，靠字面量档接住）而没有动形状：改响应模型是对外契约变化。定级 S，待拍。 | S |
| A279 | 前端服务层对后端 15 条操作没有入口（接线队列） | **十一批共接 32 条（L225 三条 + L226 三条 + L227 两条 + L228 三条 + L229 三条 + L230 三条 + L231 四条 + L232 三条 + L234 五条 + L235 一条 + L236 两条）**：`language-profile` 与 `evaluate` 进了数据分析页、`vector-status` 进了系统状态页；L226 把 `search` 进分析页、`validate` 进质量中心、`auto-config` 进配置管理；L227 把 `impact` 与 `semantic-search` 接进数据分析页（四档旋钮全可调：被编码字段、前 K 条、`text_key`、规模增益门槛）；L228 把 `stats`（字段级统计）、`features`（特征普查）、`compare`（双集对比）接进数据分析页；L229 把 `convert`（格式转换）、`merge`（合并）、`split`（分割）接进数据管理页；L230 把 `sample`（采样）、`aggregate`（多源聚合）、`rag`（转成 RAG 检索格式）接进数据管理页——**至此全仓六只会落盘产物的端点全部有页面入口**；L231 把备份那一族四条（列出 / 创建 / 恢复 / 删除）接进系统状态页——**`backup` 这一个子命令名下的四条操作从此在页面上都有入口**，而且是**第一次有一批接线同时改掉一处产品面缺陷**（`name` 越界写盘，见循环日志 L231）；L232 把数据集血缘那一族三条（已登记列表 / 登记 / 依赖图与校验问题）接进系统状态页第四张卡——**这是第二次「接线同轮收掉一处产品缺陷」**（`description` / `tags` 回显即丢，A305 本轮已修，见循环日志 L232）；L234 把系统运维那一面五条接上——`monitor` 与 `auto-test` 进系统状态页第五张卡（都只读、都**不挂**门禁），`migrate` 与 `stream` 进第六张卡（都写盘、都**挂**门禁），`validate-config` 进配置管理页新卡（只读、不挂门禁），这是**第三次「接线同轮收掉产品缺陷」**，而且一次收了三处（A313 / A314 / A315，见循环日志 L234）；L235 把 `health-gate` 接进质量中心新卡「健康门禁」（文本字段 / 四个健康分权重 / 三档门禁阈值 / `block_on_warning` 全可调，并让 `/api/quality/evaluate` 的 `pass_rate` 一键喂进来，**含 `0.0`**），这一批不改产品代码、只量出三条脾气并立成 **A319 / A320 / A321**（权重不校验非负与上界且健康分不钳 [0,1]、和为 1 的容差在恰好 0.01 那一档是假话、`text_field` 空串与幽灵键零判决 ⇒ `duplicate_rate` 1.0 的假判决），三条都同时写在页面正文里，见循环日志 L235。`UNCONSUMED` 32 → 29 → 26 → 24 → 21 → 18 → 15 → 11 → 8 → 3 → 2 → **0**（两员都在 L236 接完）、已消费 43 → 46 → 49 → 51 → 54 → 57 → 60 → 64 → 67 → 72 → 73 → **75**（75 条的 **100%**），`NO_ENTRY` 从 6 一路收到 1、再到 **0**（L236 收掉 A283① 之后 `POST /api/dataset/translate` 有了页面入口）；`CLI_ENTRY` **26 条/21 名 → 23 条/18 名 → 20 条/15 名 → 17 条/12 名 → 14 条/9 名 → 10 条/8 名 → 7 条/7 名 → 2 条/2 名 → 1 条/1 名 → 0 条/0 名**（L236 接掉 `dependencies`，`doctor` 整名出名单）——L225/L227 都只动前端，L226、L228、L229、L230、L231、L232、L234 与 L235 接的条目各有 CLI 子命令，所以动这一半的是那八批；**L234 一批减 5 条 5 名（系统那一族五条各有子命令）是现量最大的单批**，L231 减 4 条（`backup` 整名出名单）次之，L232 减 3 条（`dependency` 整名出名单）第三；剩下那 1 条/1 名就是 `doctor`（L235 把 `health-gate` 接掉之后）。修法照旧按页面分组分批：一批 = 一个页面的 fetchers + 页面入口 + 该页用例。**三件收口判据从第一批起没换过，本轮第十次全部命中**——`test_api_endpoint_consumption_l224.py` 的名单与实测集合相等且变短、`api.wiring.test.ts` 的 `KNOWN_UNWIRED` 仍为空、每员覆盖成功与失败两路（外加一条经验：接线改动本身要让消费守卫先红再回填，红的那几次就是证据）。**L226 立的那条纪律在 L234 被用户显式撤销**（拍板原话「两员都接，做到 75/75」）：`GET /api/system/dependencies` 与 `POST /api/dataset/translate` 都要接。撤销**只针对这两员**——「指标涨而能力不涨的接线一律不做」这句在别处仍然成立，账上留着原话以便日后回看是谁拍的、拍了什么。历史那一段照旧记着：L232 把血缘一族的过滤从 `"dependenc" in p` 收窄成前缀 `/api/system/dependency/`，否则会把 `dependencies` 吞进「血缘一族只有三条」那格判据（首跑实红 2 处，见循环日志 L232）；那次收窄与接不接它无关，接进来之后仍要保持两族分开数。L230 那三员的理由也仍然成立：它们是「SDK 与 CLI 都做完了、页面上连入口都没有」的真缺口，而且是落盘一族。**上一格那条「下一批建议」里点名的系统那一族五条（`monitor` / `stream` / `migrate` / `validate-config` / `auto-test`）L234 全部接完**，而 L234 那一格点名的三员里 `health-gate` 已在 **L235 接完**（质量中心新卡「健康门禁」+ 一键喂 `pass_rate`）。**L236 已收口到 75/75**：① `dependencies` 进系统状态页——不是「多一格指标」，而是把依赖诊断从「综合状态取到才渲染」那道整体门里摘出来（管道拉不动时这一面照旧可读、单独刷新），l224 名册里那条拒接理由同轮改掉；② A283① 的 `output_file` 契约就位后接 `translate`（卡上写明「逐条调模型、按 token 计费」，失败面 `failed` / `failed_indices` / `errors` / `errors_truncated` 四键各有可见读数）。两员收完 `UNCONSUMED` 归 **0**，而 L235 挂着的那句「必须复验空集相等时仍会响」**已复验**：l224 那把尺子带两支牙齿（`test_the_empty_claims_are_not_a_blind_ruler`，以及用 monkeypatch 摘掉一处调用形状的 `test_unwiring_one_operation_turns_the_empty_roster_red`），注入那一臂把 l233 的 `GATED_NOT_YET_CONSUMED` 塞回一条也当场红 ⇒ 收口没有把尺子读成恒绿。**这一维本轮已顺手验过一次**：把 `health-gate` 从名单里删掉之前，`test_unconsumed_matches_the_roster` 首跑就报「已接好或已消失 ['POST /api/quality/health-gate']」，说明它读的是实测集合而不是名单自身；空集那一档仍要在收口轮再验一次。同批可顺带处理 **A281** 里 `AppConfig` 少 `versioning` / `visualization` 两处（配置管理页旁边已有新卡，**仍待拍**）。**· 已收口（L236）**：两员都接完，消费面 **75/75**，l224 那三个名单（`UNCONSUMED` / `CLI_ENTRY` / `NO_ENTRY`）同时归空集。 | ~~M~~ 已收口（L236） |
| A280 | 8 个路由文件把下游异常一刀切收敛成 500，「资源不存在」拿不到 404 | **新立（L224）**：本轮为核对 `GET /api/versions/history` 的运行时归属顺手实测——`GET /api/versions/v_ghost` 返回 **500** + 固定文案（`api/deps.py` 的 `INTERNAL_ERROR_DETAIL`），而 SDK 抛的是 `VersionError`（`augmentor/exceptions.py:133`，`AugmentorError` + `ValueError` 双继承）；同一份路由文件里三种归宿并存：`/api/versions/create` 源文件缺失 → 404（走 `require_file`）、`POST /api/versions/v_ghost/rollback` → **200 且 `success:false`**、`DELETE /api/versions/v_ghost` → **200 且 `success:true`**（`VersionManager` 静默成功）。**这不是无声 bug，是已被钉死的行为**：`tests/integration/test_api.py:479` 两支用例的 docstring 明写「应返回 500」。普查（按调用点数，不含 import 行）：只用 `to_http_error` 分档的 4 个文件 30 处（dataset_tools 14 / system_ops 12 / language 2 / vector 2），`config.py` 与 `data.py` 两把混用（分档 3 处 + 一刀切 8 处），**只用 `raise_internal_error` 一刀切的 8 个文件 29 处**（quality 9 / export 4 / augment 3 / multimodal 2 / audit 1 / leakage 1 / privacy 1 / version 8）。两条方向：**① 把语义明确的「不存在」类异常接进 `to_http_error(not_found_types=...)`**（`BackupError` 已是这一档的先例，见 `api/routes/system_ops.py`），逐文件收窄——改的是对外状态码，上面那两支钉死 500 的用例要同批改；**② 维持现状并把「除 `FileNotFoundError` 外一律 500」写成契约**，`docs/augmentor/API.md` 里相应把 404 档从这些端点去掉。本轮选**不动，只立账**：状态码是对外契约，一刀切里混着的另一半（400 档 `to_http_error` 会把 `str(exc)` 原样回传）正是 A155① 盯住的泄漏面，一次改八个文件不是一轮的量。**L251 量到本族的又一格现量**（定级格未动，仍待拍）：宽档第一次把非数组那份文件端到候选列表上，而 `/api/data/load/configish.json` 实测是 **500 + 固定文案**、不是形态那句 400，吞掉它的是 `api/routes/data.py:177` 那句 except Exception（`HTTPException` 也在它的捕获范围内）；守卫按**观测行为**而不是应有行为钉在 `tests/unit/test_data_list_all_files_l251.py` 的 TestWhatTheListHandsBackIsReachable 那一类，谁动本条要同批改那一支。**L272 量到本族的又一格现量**（定级格未动，仍待拍）：同一族「SDK 有判决、门口把它降成 500」在 float 旋钮上是可复现的：`POST /api/quality/dedup` 与 `POST /api/quality/benchmark` 的 `threshold` 四形（NaN / +∞ / 5.0 / −1.0）**全部**回 **500** + `INTERNAL_ERROR_DETAIL`，而下游抛的是 `DataValidationError`、判决文案里点名 `threshold`——客户端的错照样冒充服务端故障，且这一档与本轮接线的另外四只咽喉只差一个收尾函数。本轮不接它：接它等于替本条拍「一刀切里哪些异常该保码」，而上面提到的那两支钉死 500 的集成用例是同批的对外契约。守卫按**观测行为**钉在 `tests/unit/test_nonfinite_float_knobs_l272.py` 的`TestHttpFace::test_a280_cells_stay_pinned_as_internal_error`（断言 500 且错误体里不出现 `DataValidationError` 字样），谁动本条要同改那一支。定级 S，待拍。 | S |
| A281 | 3 只服务层 fetchers 的 TS 响应类型缺契约里已经存在的字段（后端在回、前端读不到） | **新立（L225）**：接线时顺手量了一遍「`web/src/services/api.ts` 里 `Promise<Xxx>` 的字段集，与该操作契约 200 响应的属性集是否相等」（仓外一次性脚本：正则解析类型 + `app.openapi()` 的 `components.schemas`）。45 只带类型标注的 fetchers 里 **40 只逐字相等**（含本轮新增的三只），**3 只缺字段**，另有 **2 只 naive 正则解析不动**。三处缺口的方向完全一致，都是「契约有、TS 没有」：① `AppConfig` 少 `versioning` 与 `visualization` 两节 ⇒ 配置页拿不到这两节的当前值（与 **A276** / **A271** 相关但不等价——那两条问「这些键怎么处置 / 要不要可写」，这一条问「读侧的类型里根本没有」）；② `UpdateConfigResponse` 只有 `success` 与 `message`，少 `ignored_keys` ⇒ 后端明确回了「哪些键被忽略」，页面不显示，用户会以为自己已经写进去了；③ `StatusResponse` 少 `model_retry_count` 与 `model_retry_wait_seconds` ⇒ 系统状态页四张卡看不到重试的生效值。解析不动的那两只不是缺口，而是**这把尺子进仓前必须先解决的两种形状**：`AugmentProgress` 是联合类型别名（`types/api.ts:135`，由 `NoCheckpointProgress` 与 `AugmentProgressDetail` 二选一），`VersionDetail` 用 `extends VersionInfo` 把字段拆进继承（`types/api.ts:220`）——一行一字段的读法对这两只直接失效。修法两步：先补类型（纯前端，零风险），再决定是否真显示（只显示不渲染仍是死面，那要按 A138 的接线档继续走）。**为什么本轮不补**：本轮已含三张新卡与八支新用例，再掺类型改动会把「接线」这条判据的证据链掺浑；且这条测量目前只是仓外脚本，要做成守卫得先定下别名、继承、嵌套对象三条解析口径，并配 `KNOWN_UNMATCHED` 白名单加规模钉（形状与 `api.wiring.test.ts` 同构）——那是一轮的活。定级 S。**· 已收口（L237）**：立账的 3 处不是错，是浅——把尺子做深一层重量是 **12 处**（`AppConfig` 缺 9 / `StatusResponse` 缺 2 / `UpdateConfigResponse` 缺 1，反向幽灵 0），因为 `ConfigResponse` 九个节在契约里是 `additionalProperties: true` 的自由字典、没有 `properties`，节内 7 键不进契约 ⇒ 守卫分两层（第一层静态比契约属性集、第二层打只读 `GET /api/config` 递归比键树）。12 处全补齐，且**补齐之后同轮渲染**（系统状态页两格重试读数、配置管理页两只新卡），不是「类型上有了、页面上看不见」。守卫 `tests/unit/test_ts_contract_types_l237.py`（21 支 / 5 面）落成：75 只带类型 fetchers 全数相等，`unannotated` / `unresolved` / `UNMEASURABLE` 三格皆空集，每格都配注入臂证明仍会响。同轮量出并新立 **A323**（toast 措辞与后端 `message` 不同源）。 | ~~S~~ **已收口（L237）** |
| A282 | 前端 `npm run format:check` 的仓内基线是红的，而 CI 里没有这道门 | **新立（L225）**：本轮跑格式检查时实测 `prettier --check src` 报 **32 个文件**不合 `.prettierrc.json`（`semi: false` / `singleQuote: true` / `printWidth: 100` / `trailingComma: all` / `arrowParens: always`），而页面代码普遍是省略箭头函数括号的旧写法。**本轮没有引入漂移**，取证方式是把碰过的 5 个文件用 `git show` 取到仓外临时目录、拿同一份配置复查，`HEAD` 版本同样 5 个全红 ⇒ 跟着周围代码写才是对的选择。CI 侧 `.github/workflows/ai-platform-ci.yml` 的前端 job 只跑 `npm run lint`、`npx tsc --noEmit` 与 vitest，**没有 format 门**，所以这道红可以长期存在而不被任何人看见。两条方向：**① 一次性 `npm run format` 全量重排，并把 `format:check` 加进 CI**——此后 diff 与 IDE 一致；代价是那一个提交会淹没同批的真实改动，且重排要在事后复测一遍行号类文档守卫。**② 承认现状，把 `.prettierrc.json` 改成与仓内实际风格一致**（至少 `arrowParens: "avoid"`），让这道门从「恒红」变成可用。本轮选**不动，只立账**：两种改法都是全仓重排或配置改，与接线一批混在一起只会污染 diff。定级 S，待拍。 | S |
| A283 | `POST /api/dataset/translate` 把整份译文只放在响应体里，没有任何落盘口，前端接了也留不住 | **已关闭（L236，①②都做完了）**：①契约——`TranslateRequest` 与 `TranslateResponse` 各加可选 `output_file`（键**恒在**、没给时 `null`），给了就 `resolve_data_path(for_write=True)` **先解析落点**、`atomic_write_json` 原子写进白名单数据根、响应回显**已 resolve 的绝对路径**；越界落点 403 且模型调用 **0** 次。②接线——数据管理页新卡「批量翻译」，带计费确认闸、四格输入与失败回执四键。同轮量出 **A322**（`text_key` 零判决）并立账，后端一行未改。原立账原文照录 —— **新立（L226）**：A279 第二批挑接线对象时实测出来的契约层缺口。① `TranslateResponse`（立账时引 `api/routes/language.py:88`；那一格在 L236 之前就已漂到 `:94`，本轮加了两段 docstring 与一只请求字段之后是 `:105` ⇒ 同轮重钉。**这处漂移此前没人发现是有原因的，但原因不是「没人扫」**：`test_doc_line_refs_l79` 只读 `docs/augmentor/ARCHITECTURE.md` 与第一本账，而 A184 那把尺子**是扫第二本账的**——它判的是**落点形状**（那一行是不是代码跨度），不核**行号是否还指着当初那一格**，所以 `:88` 从 `items` 那一行漂进 docstring 里它照样放过）里 `items: List[Dict[str, Any]]` 就是**整份翻译后的数据集**，请求模型没有 `output_file`，路由也不调 `_dump`——同仓的写盘变换类端点（`convert` / `merge` / `aggregate` 等）都走 `api/routes/dataset_tools.py` 的 `_dump` 落到白名单数据根，这一只没有；② CLI 侧同样没有 `translate` 子命令（`augmentor/cli/parser.py` 的 `add_parser` 名单里查不到），所以它是 A279 的 `NO_ENTRY` 三员之一，「SDK 做完、web 与 CLI 两边都没人取」；③ 花钱面：这条路径逐条调 `model_backend.generate`，挂了 `verify_api_key`（同文件 13 行的鉴权分档说明），`multilingual.enabled` 出厂 `false`。**为什么因此不接**：UI 若只是「点一下把译文显示出来」，用户刷新页面产物就没了，而每一次点击都在按 token 付费；要做成能用的入口，先得让这次花钱留下东西。修法两步：**① 契约**——给 `TranslateRequest` 加可选 `output_file`，非空时走 `_dump` 落白名单根并让响应带一个可核对的落点字段（与 `convert` 同形，顺带把 `failed_indices` 的「指回 `input_file`」这句话变成可操作的：用户看得见原文也看得见译文）；**② 接线**——契约就位后再进多模态/数据管理那一侧的页面，成功与失败两路各自要有可见读数（`failed` / `errors_truncated` 是 L221 刚补的失败面，只显示 `total` 等于把它再吞一次）。**本轮不做 ①**：它改的是对外请求与响应形状，A279 立项时写死「不改后端字段名或响应形状」，混进接线轮会让「接线」这条判据的证据链掺浑。⇒ **①②均在 L236 做完**，见循环日志 L236。 | ~~M~~ 已修（L236） |
| A284 | `AugmentationMetrics.unique_instructions` 的键名与实现不符 | **新立（L227）**：`augmentor/impact.py:76` 那一格赋的是 `len(set(texts))`，而 `texts` 取自 `self.text_field`（默认 `instruction`，见同文件 65 行）。换字段就是在数另一个东西，名字却还写着 instructions；`to_dict()`（同文件 24 行，键在 27 行）把它原样送到 API 响应面上，所以这是**契约面上的谎**而不是局部变量名。L227 只做了前端侧的诚实化：页面标签改成「唯一值数（按所选字段）」，并由 `test_impact_semantic_wiring_l227.py` 钉住「标签必须含『字段』、不得含『指令』」，**键名本身没动**——它同时是 SDK dataclass 属性和响应字段，改名是破坏性变更，不属于「接线」这一档工作量。两条候选处置：① 加 `unique_values` 别名 + 弃用旧名，跨版本删；② 只改 `docs/augmentor/API.md` 的口径说明，把键名当既成契约。判据上的差别是 ①：`_dump` 出来的 JSON 键会变。定级 S，待拍。 | S |
| A285 | `["instruction", "input", "output"]` 这份字段清单在仓内有五处手写字面量，没有权威常数 | **新立（L227）**：`augmentor/audit.py:69`、`augmentor/privacy.py:76`、`augmentor/cli/parser.py:173 / 406 / 431` 各写一遍（后两格是 L244 净 +1 行平移之后的落点，按内容重钉），语义都是「默认看这三格」。直接后果是 L227 那把跨语言尺子**建不出来**：`text_field` / `text_key` 的合法取值来自数据集本身，而这份字面量既不是常数也不能代表合法域，引它当权威等于自造一套——所以守卫只能退到「页面发得出的每一档都被端点接受」这种运行时判据（`TestEndpointAcceptsWhatTheUiSends`）。处置建议：抽一份模块级常数（比如叫 `DEFAULT_TEXT_FIELDS`）并把五处改为引用，归属模块待拍——本轮刻意不指名新模块，因为 **A184 那把注释引用尺子会把「指向仓内并不存在的文件」记成 dead_path**（第一版给的就是一个还没建的新模块名，被它当场红两支，这正是这把尺子该有的反应）。常数就位之后，l226 那把「下拉清单 vs 后端封闭清单」的尺子就能复用到这一族字段上，L227 守卫里那档运行时判据也可以升级成逐项对账。**〔A285 已收口（L257）〕**：权威落在 augmentor/privacy.py 的模块常数 DEFAULT_TEXT_FIELDS（元组），SDK 两处改为引用且**同行改写、净零行**（audit 侧 fields-or-list(权威)、privacy 侧 fields-if-not-None-else-list(权威)），CLI 三处按 parser.py 自己的既有口径「不 import 业务模块」保持字面量、由新守卫 `tests/unit/test_default_text_fields_l257.py` 第 4 面逐档与权威对账——所以本行上面那 三个 CLI 落点现在是 default=[…] 的字面量形状，而 audit 与 privacy 的两个落点已是引用形状，四个行号在**当轮**一格未漂（a184 与 l257 两面同证；**L277 订正**：收 A339 那批改写把 audit 取用点推到第 69 行，本行的活引用已按现量重钉，「一格未漂」说的是 L257 当轮而不是此后恒真——散文里的历史读数不自动升格成现量，这一格正是 A333 点名的形状）。「归属模块待拍」这一格本轮拍定：不住新模块（A184 会把指向不存在文件的引用记成 dead_path，本行原话已记过一次），也不插在 privacy 第 76 行之前（那行与 audit 第 68 行是账本与 l227 / l228 的活引用落点，插行会把落点平移成上一行），于是常数写在文件末尾、取用点运行时查名。「升级成逐项对账」那一半本轮**未做**——它是 l227 那把尺子的下一步，不是本行的欠账，也不构成第二种形状。同场另两条 --fields 默认面不收编：analyze 是两格、check-leakage 是一格，l257 第 4 面把它们逐一点名而不是并进这一族。定级 ~~S~~ 已收口（L257）。 | ~~S~~ 已收口（L257） |
| A286 | `tests/unit/test_analytics_performance.py` 的比值型计时守卫在共享工作机上会假红 | **新立（L227）**：全量门禁复跑实测到一次假红——`stays_linear` 那条比值量到 **4.38**（阈值 3.0，线性档实测 2.0–2.5、退化到 O(n²) 实测 3.1–3.9），而同一条用例本轮第一次全量是绿的、之后隔离复跑四次全绿；红那一次整轮耗时 **257.50 s**，比绿那次的 188.88 s 高 36% ⇒ 是并发负载压在小档计时上，不是算法退化。**为什么这条值得立账**：退化档与噪声档在墙钟上是同一个数（4.38 比 O(n²) 档还大），意味着真来一次退化也可能被当成噪声放过。三条候选：**① 每档取 N 次的中位数或最小值**而不是单次；**② 用计数代替墙钟**（数内层重算次数，确定性、与负载无关）；**③ 把两档规模一起放大**让噪声占比下降（文件里已有这条口径：实现变快了就先放大规模，不许放宽比值阈值）。建议 ①+② 一起做——② 才是判据，① 只是让它稳。本轮**既不动阈值也不改实现**。定级 S，待拍。**L239 收口（按 ②，① 只留作兜底）**：判据换成「整趟 `builtins.sum` 被调几次」——`counted_sum_calls()` 把 `builtins.sum` 临时换成原样转发并计数的替身，`finally` 里复原并断言复原成功；现量 `_calculate_diversity_score()` 在 n=1000/2000/10000/20000 **四档同为 2**（一次求均值、一次求方差和），n=0 走空档早退 ⇒ 0 ⇒ 判据与负载、与机器快慢彻底无关。三条牙齿：复刻退化档（均值写在生成器里）现量调用数 **n+1**（5000→5001、10000→10001）、空档 0 次且分数 0.0、窗口内抛异常也必须复原真函数。`RATIO_CEILING` / `MIN_MEANINGFUL_MS` 与 `test_ratio_ceiling_separates_linear_from_quadratic` 退役；**撤下它不是放宽**——那条用例量的是「这台机器此刻的 CPU 给多少」，现在量的是「算法跑了几次求和」。保留的两条各测一轴：`ABSOLUTE_BACKSTOP_MS`（2000 ms，现实现约 70 ms，20 倍松、只在数量级出事时红）与 `test_the_timer_can_see_scale`（2000→8000 必须明显更慢，证明计时器仍有分辨力、别让任何读数把「耗时不随规模变」当证据）。注入取证：把均值改回生成器里，**返回值一字不变**、只有计数抓得到 ⇒ 这条判据测的正是旧判据测不到的那一维。 | ~~S~~ **已收口（L239）** |
| A287 | `augmentor/comparison.py` 的 `similarity_to_a` / `similarity_to_b`（双集对比质量面的两个键） | **新立（L228）**：接线 `POST /api/dataset/compare` 时顺手普查这两键的写者与读者。机器锁是**双向**的：源码面全仓（不含 import 行）**只有两处出现**——dataclass 默认值 `similarity_to_a: float = 0.0`（声明在同文件 60、61 两行，送出在 87、88 两行的 `to_dict()`），**没有任何赋值点**；行为面同锁——真数据（4 条 vs 2 条、instruction 一句不重）跑一次回执里两键都是 `0.0`。所以这不是「暂时没算」而是**结构上恒零的死面**，而它住在对外契约里：`docs/augmentor/API.md` 全文查不到这两个名字，OpenAPI 的响应面上却有格子。当前后果：客户端读到 `0.0` 会理解成「A 与 B 毫不相似」，真相是「这一档从未被计算」——一个恒零的数比没有这个数更会骗人。两条处置方向：**① 接上计算**，但语义得先拍清楚（`similarity_to_a` 是「B 相对 A 的相似度」还是别的？重叠面已经有 `similarity_score` 一个数，再挂两个近义数是给消费面添混乱，不是补功能）；**② 从响应面删掉这两键**（破坏性变更：同时动 SDK dataclass、`to_dict()` 与契约面，既有用例里按位置构造这两键的点要跟着改）。L228 只做了前端侧的诚实化——页面**不渲染**这两个键，并由 `tests/unit/test_stats_features_compare_wiring_l228.py` 的 `TestDeadAndNullableFaces` 同时钉住「无赋值点」与「页面上没有它们的名字」，键本身一格未动。定级 S，待拍。 | S |
| A288 | `augmentor/compare_enhanced.py:190` 的 `size_ratio`：B 侧 0 条时 SDK 存 `float('inf')`，到线上是 `null` | **新立（L228）**：那一格写的是 `len(items_a) / len(items_b) if items_b else float('inf')`，而同一份 dataclass 把它声明成 `size_ratio: float`（同文件 32 行的声明、45 行的 `to_dict()` 原样送出）。实测的形状是三段式的：SDK 里是 `inf` ⇒ 经过 JSON 到浏览器是 **`null`**（响应体本身仍是合法严格 JSON，探针逐条验过）⇒ 而契约面上它写的是 `number`。也就是说**声明、运行时值、线上形状三者互不相同**，任何一方单独看都会读错：只看 OpenAPI 的客户端当它是数字，`null` 进到 `toFixed` / 算术里直接变 `NaN`；只看 SDK 的人会以为它可能是 `Infinity`（那不是 JSON 的值）。当前后果与处置：本轮把前端改成按**线上真相**说话——`web/src/types/api.ts` 里收成 `size_ratio: number \| null`，页面读到 `null` 时显示「—（B 侧 0 条：SDK 存 `inf`，JSON 面为 `null`）」而不是一个假数；并由新守卫同时钉住三处（类型里有 `\| null`、页面有 `null` 分支、真跑 B 侧空集那一对时回执里确实是 `null`）。两条后续方向：**① 让 SDK 不再发 `inf`**（显式 `None` 或一个可判的上限哨兵），拍的是「B 侧为空时 A ÷ B 该表示什么」这个口径；**② 维持现状并把「`inf` 上线为 `null`」写进 `docs/augmentor/API.md` 的该端点字段说明**——注意现在文档里根本没有 `size_ratio` 这个名字，所以 L223 那把逐端点字段提示尺子对它没有任何覆盖。本轮选**不动后端形状，只把前端与守卫改成诚实的**：改响应值域是对外契约变化。定级 S，待拍。 | S |
| A289 | `POST /api/dataset/split` 的两条支路（`stratify=False` / `True`）用的不是同一套判据，同一份输入在两侧会得到不同的判决甚至不同的产物〔比例那三处已由 **A324 / L256** 并成一条权威，本行现只剩 ④ `shuffle`〕 | **新立（L229）**：接线分割卡时逐档实测出来的三处不对称。① **比例之和的容差不同**：默认支是 `abs(total_ratio - 1.0) > 0.01`（不含端点）且逐项不判，分层支委托 `DataSplitter`，那一支是 `abs(total - 1.0) > 1e-6` ⇒ 实测 0.5/0.25/0.255 这种「差 0.005」的比例默认支**放行**、分层支**拒绝**。② **负数只有一支拦**：`比例不能为负数` 那句判据只在分层支，默认支拿到负比例照切不误（实测 `-0.8/0.1/1.7` 回 200 且把 6 条全塞进测试段成 `0/0/6`）。③ **判决文案是两句不同的话**，客户端按字符串匹配就会在两侧各写一套。**〔①②③ 由 A324 / L256 统一〕**：两处手抄都删掉，门口与分层支共引 `validation.require_ratio_list`（和 ±0.01 含端点、量化 9 位 + 逐项 bool/NaN/非数值/0..1 + 长度 3），同一份 `ratios` 在两条支路同码同答、`detail` 原文逐字节相同；长度与逐项判据同时补上（改前 4 个元素会过校验后死在 unpack）。直构 `DataSplitter` 那份报的名是 `比例` 而不是 `ratios`，是仅存的措辞差。原先点的三处行号随代码形状一起消失，本行不再点行号。④ 还有一处更贵的：**`shuffle` 在两支上语义不同**（20 条语料 0.5/0.25/0.25 实测）——分层支只把每段成员按输入的原始相对顺序回排，成员与条数都不变、`seed` 仍决定谁进哪段；默认支是「先整份打乱再按位置切片」，所以 `shuffle=False` 会**连同成员一起改变**，且三段恒为输入的前 50%/25%/25%，此时 `seed` 完全空转。**当前处置**：页面卡片原先把 ①② 两句写死（「比例之和的容差默认支是 0.01、分层支是 1e-6，负数只有分层支会拦」），L256 起那句改成「两条支路共引 `validation.require_ratio_list`，同一个 `ratios` 两侧同码同答」；④ 记在请求模型的 docstring 里；守卫 `tests/unit/test_transform_wiring_l229.py` 现钉「同一份 ratios 两支同码同答」四档（判红两档：负比例、和超容差；判绿两档：留两位凑 0.99、和 1.005），`tests/unit/test_decision_gates_l238.py` 的名册档盯住「四个调用点都不许再写 `if abs(` 手抄」。**修法方向**：①② 已随 A324 / L256 做完（合并成一处、按逐项 + 长度 + 含端点容差判），③ `shuffle` 那面要么在两支上统一语义，要么在契约面上改名（`shuffle` 在分层支上其实是「保持原始相对顺序」）。定级 M，待拍。 | M |
| A290 | `POST /api/dataset/merge` 只转发 `MergeConfig` 四档旋钮里的一档，回执只上线 SDK 六键里的五键，而响应模型的 docstring 写着「一一对应」 | **新立（L229）**：① **旋钮面**：`MergeConfig` 有四档（`deduplicate` / `dedup_threshold` / `preserve_order` / `max_items`，声明在 `augmentor/dataset_ops.py:27`），而端点构造时只喂了第一档（`api/routes/dataset_tools.py:787` 那一行就是 `MergeConfig(deduplicate=request.deduplicate)`）⇒ `dedup_threshold` 的宽松/保守两档（L150 立的语义：`< 0.9` 启用大小写折叠 + 空白归一并删近似重复）、`preserve_order`、`max_items` 三档在 HTTP 面上**调不到**，CLI 与页面都一样。② **回执面**：`merge_files` 返回六键，其中 `truncated_by_max_items`（`augmentor/dataset_ops.py:220`）被 `MergeResponse`（`api/routes/dataset_tools.py:270`，五个字段）丢掉 ⇒ 而该模型的 docstring 明写「与 `DatasetOperations.merge_files()` 的返回键一一对应」，**这句是谎**。两件事叠起来有一个具体后果：即使哪天把 `max_items` 接上，用户也看不见「被截断了几条」，只会看到 `total_output` 变小。**当前处置**：页面卡片写明「这一档只控『开 / 关』：SDK 里那个宽松 / 保守两档的去重阈值在这只端点上不可调（见账本 A290）」，守卫 `test_the_merge_endpoint_forwards_one_knob_of_four_and_reports_five_of_six` 把两半各钉一格（请求模型字段集 vs `MergeConfig` 字段集、响应模型字段集 vs `merge_files` 返回键集），并另有一支钉住「页面上不许出现假的『预览合并结果』」。**修法方向**：① 最小改动——`MergeResponse` 补 `truncated_by_max_items` 并把 docstring 改成实话；② 接旋钮——请求模型补三档并逐个接线（注意 `max_items` 会改变产物条数，回执不带截断数就是骗人，所以 ① 必须先于 ②）；③ 反向收窄（「把 SDK 那三档删掉，缺口就没了」）**不成立**：`merge_files` 内部三档都有真消费者（`augmentor/dataset_ops.py:157` 的 `preserve_order`、`:163` 的 `dedup_threshold`、`:169` 的 `max_items`），所以缺口只能在入口面上补。定级 M，待拍。 | M |
| A291 | 转换端点在 `source_format` 是空串时回的判决缺主语：「无法从  转换到 JSON」 | **新立（L229）**：请求模型里 `source_format` 的默认是 `None`（＝「按扩展名推断」），但**空串不是 `None`**：实测 `{"source_format": ""}` ⇒ **400**，detail 是 `无法从  转换到 JSON`——那句 f-string 在 `augmentor/converter.py:327`，主语位置插进去的就是那个空串，于是句子里留着两个连续空格且**说不出是什么格式转不了**。缺省不发这一格则是 **200**（走扩展名推断），所以两者不等价：空串不是「未声明」的同义词。**为什么页面碰不到**：服务层全文只有一处 `\|\| undefined` 哨兵，把页面上的空串丢掉再发；能踩到这句判决的是直接调 HTTP 面的人（脚本、第三方客户端、curl）。**当前处置**：守卫钉住三面——请求模型默认是 `None` 而不是 `""`、服务层那一处哨兵存在且只有一处、空串与缺省在线上不等价（一个 400 一个 200）。**修法方向**：① 把空串归一成「未声明」（校验器里 `"" → None`），判决就不再有缺主语那一档；② 保留 400 但把文案改成能读的（「未声明源格式，且扩展名推不出输入格式：`<文件名>`」）；③ 两者都做——① 是行为变化（原本 400 的调用会变成 200），要先拍。定级 S，待拍。 | S |
| A292 | `augmentor/dataset_ops.py` 的 `SampleConfig.ratio`：这一格没有任何判决，同一个越界值在三种方法上给出三种结局 | **新立（L230）**：接线采样卡时逐档实测（20 条语料）。`size` 有域校验 `require_count("size", config.size)`（`augmentor/dataset_ops.py:291`），而 `ratio` 那一支只有 `elif config.ratio is not None:`（`augmentor/dataset_ops.py:299`）后面一句 `int(len(items) * config.ratio)`——**既不判上界也不判负数**。于是：① `ratio=2` ⇒ `random` 支 **400**，detail 是 CPython 自己的英文 `Sample larger than population or is negative`（不是本仓的中文判决），`systematic` 与 `stratified` 两支 **200 且回整份 20 条**；② `ratio=-0.5` ⇒ `random` **400**、`systematic` **200 且回 10 条**（负数被读成切片下标，「去掉末尾 10 条」）、`stratified` **200 且回 0 条**；③ `ratio=0.0001` ⇒ 三支都 **200 且回 0 条**（`int()` 截断）。**当前处置**：页面卡片写明「`ratio` 越界只有 random 支会拦，另外两支要么给整份要么给零份」，守卫把三档 × 三支的行为逐格钉住；`ratio` 与 `size` 在页面上是二选一（给一个就清另一个），所以踩不到「两格都发」那一档。**修法方向**：① 把 ratio 的域校验提到三支之前（`0 < ratio <= 1`），一处判决三支共用；② 或者至少让三支对同一个越界值给同一种结局。注意 ① 是行为变更：今天 `systematic` 上 `ratio=2` 是 200。定级 M，待拍。 | ~~M~~ 已收口（L277） |
| A293 | `api/routes/dataset_tools.py:887` 构造的是无参 `DataAggregator()`，三只旋钮在 HTTP 面调不到；回执是两套手维护的键集且已漂移 | **新立（L230）**：① **旋钮面**：`DataAggregator.__init__` 收 `key_fields` / `value_fields` / `random_state` 三档（`augmentor/aggregator.py:57`），而端点构造的是**无参** `DataAggregator()`（`api/routes/dataset_tools.py:887`）⇒ 三档在 HTTP 面与 CLI 面都调不到。直接后果三条：去重永远按 `instruction`、`consistent` 永远按 `output` 比答案、weighted 那一支的抽样**不可复现**（每次跑可能给不同产物，而写盘端点给不出可复现产物是硬伤）。② **回执面**：`AggregationResult.to_dict()`（`augmentor/aggregator.py:37`）与 `AggregateResponse` 是**两套手维护的键集**，实测已经漂移——SDK 侧多 `total_count` 与 `conflict_count`，线上侧多 `output_file` 与 `aggregated_count`；端点没有走 `to_dict()` 而是手抄了四格（`api/routes/dataset_tools.py:891` 起）⇒ 与 **A290** 同族：「与 SDK 返回键一一对应」这句话在这一只端点上也不成立。③ **`conflicts` 整份上线**：那一格把每条冲突的原文都塞进响应体（实测一组冲突 133 字节起，随语料线性涨），与本仓写盘类端点「只回写到哪、写了多少」的回执口径不一致。**当前处置**：页面把「去重键 / 一致性键不可调」与「冲突明细逐条渲出」两句写死在卡上，守卫同时钉住两套键集的**两个差集**与「端点不走 `to_dict()`」这一格。**修法方向**：① 请求模型补三档旋钮并接进构造（`random_state` 接上才谈得上可复现）；② 回执改成引用 `to_dict()`，或把两套键集合并成一份权威；③ `conflicts` 要么截断（带 `conflicts_truncated` 计数，同 L221 的翻译失败面）、要么落盘只回条数。定级 M，待拍。 | M |
| A294 | 聚合端点的 `target_size` 只咬 weighted 那一支，另外三支给什么都一样 | **新立（L230）**：四种策略里**只有 weighted 真的读 `target_size`**（回落点 `augmentor/aggregator.py:159` 的 `target_size = DEFAULT_TARGET_SIZE`），union / intersection / consistent 三支拿它完全空转。实测（20 条语料）：`target_size=1` ⇒ weighted **1 条**、其余三支都 **20 条**；`target_size=0` ⇒ **200 且 0 条**（0 是合法值，不是「未设置」）；`target_size=-1` ⇒ **400**；**省略 ⇒ 200**，走 `DEFAULT_TARGET_SIZE = 100`——这一格此前 `docs/augmentor/API.md` 写的是「省略会 400」，是假话，本轮已订正并由四支文档用例钉住。**为什么值得立账**：契约面上四支共用一只字段，而它只在一支上有语义 ⇒ 客户端把 `target_size` 当成「聚合结果条数上限」就会读错三份产物。**当前处置**：页面卡片写明「`target_size` 只咬 weighted 那一支」，留空即不发那一格（服务层不写第二套默认，`DEFAULT_TARGET_SIZE = 100` 全仓只有 SDK 一处）。**修法方向**：① 把这一格改成 weighted 专用（请求模型上说明，或拆成 `weighted_target_size`）；② 让四支都尊重它（是行为变更：今天 union 给 20 条，接上会变）；③ 维持现状但把「只对 weighted 生效」写进 `docs/augmentor/API.md` 的字段说明（本轮已按 ③ 落了订正）。定级 S，待拍。 | S |
| A295 | 「转成 RAG 检索格式」的默认档在仓内有三处字面量、两个不同的值，且请求侧多送的分块旋钮被静默忽略 | **新立（L230）**：配置出厂默认是 `"llamaindex"`（`augmentor/config.py:533`）、SDK 的 `RAGFormatter.format` 形参默认也是 `"llamaindex"`（`augmentor/rag.py:187`），而端点那一行是 `fmt = request.format or rag_config.default_format or "langchain"`（`api/routes/dataset_tools.py:927`）⇒ **第三档兜底写的是另一个格式名**。今天它不可达（配置那格永远非空，除非有人把 `rag.default_format` 显式设成空串并绕过校验），但它是一只埋着的谎：真兜到那一档时产物会是 langchain 形状，而文档与配置都说 llamaindex。另两处实测：① **空串 `format` 与省略等价**（两者都回 `llamaindex`），所以服务层那一处 `\|\| undefined` 在这一只端点上是**形状卫生**而不是躲判决——与转换卡那处 `source_format` 的动机不同（**A291**：那一处送空串会 400）；② 请求里多送的 `chunk_size` / `chunk_overlap` **被静默忽略**：端点构造 `RAGFormatter` 时只读配置那两格（`api/routes/dataset_tools.py:929`），调用方以为能按请求调分块。**当前处置**：页面只给三档下拉（含「服务端默认」那一档，留空即不发），守卫钉住「服务层不写第二套默认」与「页面默认档同源于 `RAGConfig` 的 dataclass 默认」；`docs/augmentor/API.md` 的 rag 行已按实测订正（写明出厂默认 `llamaindex` 与那第三处字面量）。**修法方向**：① 删掉 `or "langchain"` 那一档兜底（配置校验已保证非空）；② 或把它改成与出厂默认同源的一个常数；③ 分块两格要么接进请求模型、要么在文档里写明只读配置。定级 S，待拍。 | S |
| A296 | 分层抽样只按字段值的**前 10 个字符**分组，前缀相同的不同类会静默塌进同一组 | **新立（L230）**：`group_key = value[:10] if value else "empty"`（`augmentor/dataset_ops.py:354`，源码注释自己写着「简化：使用前10个字符作为分组键」）。实测（夹具 `AAAAAAAAAA甲` / `AAAAAAAAAA乙` / `BBBBBBBBBB丙`，`size=2`、`seed=7`）⇒ 只回 `['AAAAAAAAAA乙', 'BBBBBBBBBB丙']` 两条，`甲` 那条**永远采不到**，而回执里没有任何一格说明发生过折叠。相邻的一格是**空键出声、幽灵键不出声**：`stratify_key` 给空串是 **400**（`augmentor/dataset_ops.py:350` 的 `分层采样需要非空的 stratify_key`），给一个数据集里**根本不存在的字段名**却是 **200**——所有条目的 `item.get(key, "")` 都是空串，落进同一个 `"empty"` 组，配额等于全量池子的一次抽样（这一格 L229 已在卡片上写明「塌成一个分组」）。**为什么这条比 A289 那类更贵**：折叠是**按数据内容**发生的，同一份代码在短标签数据上完全正常，在长文本字段（比如用 `instruction` 分层）上就会把不同的类当成一类，而产物条数看起来正常。**当前处置**：页面在分层字段那一格写明「只按前 10 个字符分组」，守卫钉住折叠行为与「空键 400 / 幽灵键 200」两格判决。**修法方向**：① 用完整值当分组键（是行为变更：分组数变多、每组配额变小）；② 保留截断但在回执里加一格「发生了折叠的组数」；③ 把截断长度做成可配。定级 M，待拍。 | M |
| A297 | `consistent` 策略把**同一来源内部**的答案分歧也算成冲突，而那些条目永不落地 | **新立（L230）**：这一支的判据是「同一个 key 下的所有 (来源, 答案) 对，答案集合大小为 1 才落地」（`augmentor/aggregator.py:227` 起），**它不区分分歧是跨来源还是同来源内部**。实测两格：① 同一个来源里出现两条同 key 不同答案（回执 `sources` 是 `["甲", "甲"]`）⇒ 被算成**一条冲突**，且这些条目**永不落地**（`aggregated_count` 是 **0**）；② 同 key 同答案但 `input` 不同 ⇒ **正常落地 1 条且不算冲突**（因为只比 `output`，见 **A293** 的「一致性键不可调」）。**为什么值得立账**：docstring 写的是「仅保留对相同 key 答案一致的数据」，用户读成「跨来源交叉验证」，而实现是「同 key 的所有出现都必须一致，包括同一份文件里的重复提问」⇒ 一份内部有重复提问的数据集会让整组产物为空，而回执只说 `aggregated_count: 0` 加一串冲突，不告诉你冲突的两边其实是**同一个来源**。**当前处置**：页面把冲突明细**逐条渲出**（含来源名），用户至少能看见 `["甲", "甲"]` 这种形状；守卫钉住「同源分歧算冲突且条目不落地」与「同答案不同 `input` 不算冲突」两格行为。**修法方向**：① 冲突判据改成按来源去重后再比（同源内部的多答案另立一格回执）；② 或在冲突明细里显式标注「同源内部」，让 0 条产物可解释。定级 S，待拍。 | S |
| A298 | `POST /api/system/backups` 的 `name` 不查重复：同名创建两次 ⇒ 磁盘一份、索引两行，而 `DELETE` 按 id 把两行一起清掉 | **新立（L231）**：接线备份卡时逐档实测。① 同一个 `name`（实测 `mybak`）创建两次 ⇒ `GET /api/system/backups` 回**三条**（时间戳 id 一条 + `mybak` 两条），磁盘上只有**一份** `mybak.json`（第二次 `atomic_write_json` 覆盖第一次），两行的 `timestamp` 不同（实测 `.991804` 与 `.997820`）而 `backup_path` **逐字相同** ⇒ 前端行键不能拿 `backup_id` 当唯一键（本轮页面用 `backup_id#timestamp`）。② `DELETE /api/system/backups/{backup_id}` 按 id 过滤索引 ⇒ **两行一起消失**，磁盘那一份也删掉；此后同 id 再恢复 / 再删都是 **404**「备份不存在」。③ 相邻的一格：`name` 带路径分隔符**放行**——实测 `sub/deep` ⇒ **200**，在备份目录下建 `sub/` 再落 `deep.json`；带 `..` 才 400（L231 补的那道门）。**为什么值得立账**：索引是「备份有哪些」的唯一权威，而它可以合法地含有两行指向同一份文件的记录，删除的粒度（按 id 全清）与创建的粒度（按 name 追加）不是同一把尺子 ⇒ 用户看到的条数与磁盘上的份数可以长期不等。**当前处置**：页面行键拼时间戳、卡片写明「同名再备份会覆盖磁盘那一份、索引里却多一行，删一次两行一起走」，守卫把「两行一文件」与「一次删除清全部」两格行为各自钉住。**修法方向**：① 创建时按 `name` 查重并覆盖索引行（行为变更：列表条数变少）；② 或让 `backup_id` 真的唯一（同名追加时间戳后缀），索引与磁盘一一对应；③ 维持现状但在回执里带一格「本 name 已有 N 行」。**L250 已收口（走立账那一格的修法①，与 A303 同批，原立 L231）**：`DatasetBackup.backup()` 的索引写点由 `self._index["backups"].append(info.to_dict())` 换成推导式——先按 `backup_id` 摘掉同 id 的旧行，再落新行 ⇒ 同名再备份**更新**那一行而不是在旁边再加一行，索引行数与磁盘 `*.json` 份数一一对应。为什么不是「维持两行再去重」：第二次 `atomic_write_json` 覆盖的是同一个文件，被留下的旧行里 `checksum` / `item_count` / `file_size` 描述的是已经不存在的字节——那是**死账**，读边去重只是把死账藏起来。**为什么按 `backup_id` 摘而不是按 `name`**：`backup_id` 本来就是 `name or 时间戳`，按它摘等价于按 name 摘，且不必在索引里引入第二把键（旧行里没有 `name` 那一格可查，见 **A335**）。**② 未做**（让 `backup_id` 真的唯一）⇒ 同一秒内的两次**匿名**备份仍共享 `backup_YYYYmmdd_HHMMSS`、索引塌成一行、磁盘塌成一份，这一格由 `tests/unit/test_backup_index_collapse_l250.py::TestUnnamedBackupsInsideOneSecond`（冻结时钟）钉成 tripwire 而不是静默消失；**③ 未做**（回执加「本 name 已有 N 行」）⇒ ① 之后 N 恒为 1，那一格没有信息量。**读边一字未动**：存量 `index.json` 里本轮之前写出的遗留多行照原样交回（同一支守卫的 `TestTheReadEdgeStillsHandsBackWhatIsThere` 拿立账那两条实测时间戳 `.991804` / `.997820` 手工造盘），所以前端行键 `backup_id#timestamp` 不是遗漏而是兼容；删除的粒度保持原样（按 id 过滤 ⇒ 同 id 的行含遗留多行一次清全部）。判据两面：新守卫 12 支五类（写边 4 / 读边 3 / 同秒匿名 1 / HTTP+CLI 两面一致 2 / AST 钉「写点是推导式而不是 append」2），加上原地反转的 `test_backup_wiring_l231.py::test_same_name_twice_leaves_one_index_row_and_one_file`。定级 ~~M~~ 已收口（L250）。 | ~~M~~ 已收口（L250） |
| A299 | 备份的 `checksum` 算了、存了、比了，但**从不拦截**：改坏的备份照样恢复成 200 | **新立（L231）**：`DatasetBackup.restore()` 会重算校验和并与索引里那一格比对，不匹配时只 `logger.warning("备份文件校验和不匹配，文件可能已损坏")` 然后**继续恢复**。实测把 `.backups/ck.json` 的内容整份换成一条别的记录再打 `POST …/restore` ⇒ **200**，回执 `item_count 1`，落盘的就是被改坏的内容；`GET /api/system/backups` 里那一行的 `checksum` 仍是**创建时**的旧值（不会随文件重算）。**为什么值得立账**：备份的语义是数据保全，而「保全下来的那份已经坏了」这件事只写在服务端日志里——HTTP 面与页面上都读不到，用户拿到 200 就会认为恢复成功。**当前处置**：页面卡片写明「校验和不匹配只留一句 WARNING 日志、不拦恢复」，守卫钉住「改坏的备份仍 200」与「回执不带校验和那一格」两格现量。**修法方向**：① 恢复时不匹配就 409 / 400（是行为变更：今天 200）；② 维持 200 但在回执里补一格 `checksum_ok: false`（响应模型要加字段，`test_api_openapi_contract` 那把键集尺子随之重钉）；③ 给 `verify_backup_integrity` 一只端点，让用户能主动查（与 **A300** 同一件事）。**L243 已收口（用户拍板走 ①+③，原立 L231）**：恢复前把备份文件重算的 sha256 与索引里那一格比，不等就抛 `BackupIntegrityError`，端点显式接住回 **409**，且**一个字节都不写**（库在覆盖输出文件**之前**抛，实测 `out.json` 不存在）；同轮给 `verify_backup_integrity` 加了 `GET /api/system/backups/verify`，逐行读索引回三形状（对得上 / 对不上 / 文件丢了，零字节也算丢），页面「数据备份」卡上有「校验完整性」按钮，点一次查一次、不随列表自动刷。**为什么走 ①+③ 而不走 ②**：② 让回执带 `checksum_ok`，等于把「这份坏了」推给调用方自觉；409 是把闸放在写盘之前，主动查那一档让「还没恢复」也能提前发现。另有两支证明闸不是一刀切：同名再备份一份好文件之后恢复 200（闸认内容不认 id），拿 409 之后索引里的 `checksum` 一字未改（闸不自我修复）。四档与直觉不同的实测（缺目录时 verify 不建目录、同名两行在 verify 结果里塌成一行、坏内容必须是合法 JSON 才走得到闸、保留份数按索引行数算）逐条写在 `test_backup_integrity_l243.py` 的模块 docstring。契约面只声明 {200, 422} 那一格另立 A330 记账。定级 M ⇒ 已收口。 | ~~M~~ 已收口（L243） |
| A300 | `clean_old_backups` / `verify_backup_integrity` 两只 SDK 函数没有任何 HTTP 入口 | **新立（L231）**：两只函数在 `augmentor/backup.py` 里都存在且可调用（实测 `callable` 皆真），而契约面上一个都不认：`app.openapi()["paths"]` 里以 `/api/system/backups` 开头的路径现量只有**三条**（`/api/system/backups`、`/{backup_id}`、`/{backup_id}/restore`），打 `GET /api/system/backups/ck/verify` 实测 **404**。CLI 侧同样没有子命令（`backup` 那个子命令只覆盖四条端点对应的动作）。**为什么值得立账**：这两只正是 **A299**（校验和不拦恢复）与「备份目录无限增长」两件事的现成解法，能力在库里躺着、产品面上够不着。**当前处置**：守卫按 (方法, 路径) 集合精确钉住备份面现量（多一条少一条都红），页面卡片写明这两只函数没有 HTTP 入口。**修法方向**：① 各加一条端点（`POST …/verify`、`POST …/cleanup`，注意 cleanup 是删除类操作要挂 `verify_api_key`）；② 或把 `verify` 的结果并进 `GET /api/system/backups` 的每一行（一次列表就带完整性，代价是每次列表都要重算全部校验和）。**L243 已收口（走修法①，原立 L231）**：`GET /api/system/backups/verify`（只读、不挂鉴权、目录不存在时**不建目录**、幽灵 id 回空列表）与 `POST /api/system/backups/cleanup`（写操作、挂 `verify_api_key`、`max_backups` 下限 1，0 与负数 400 并回 `require_count` 原话）两只端点上线，服务层与页面各接一半——「数据备份」卡上多了「校验完整性」与「清理旧备份」两只按钮，后者带 Popconfirm 与「保留几条」的数字框（页面把下限钉在 1，因为 0 与负数在服务端就是 400）。同轮 `test_backup_wiring_l231.py` 的备份面名册按现量从四员重钉成六员，消费面 75 → **77** 条操作 / 71 → **73** 条路径，`test_api_security` 的 `PROTECTED` 与 `OPEN_ALLOWLIST` 各进一员，`test_api_key_channel_l233` 的挂门禁规模 24 → **25**（已接线同数，未接线那一档仍为空）。两只 SDK 函数的行为面（保留数按**索引行数**算 ⇒ 同名两行会占两个名额却只删一次文件）各由 `test_backup_integrity_l243.py` 点名。CLI 当时没有这两档 ⇒ 另立 A329（L244 已补，`backup --action` 现量六档）；契约面不声明错误档 ⇒ 另立 A330。定级 S ⇒ 已收口。 | ~~S~~ 已收口（L243） |
| A301 | 「默认备份目录」在仓内有三份，而端点那一份与另外两份**不是同一个目录** | **新立（L231）**：三份实测——① CLI `--backup-dir` 的默认是相对路径 `.backups`；② SDK 侧全部形参默认也是 `backup_dir: str = ".backups"`（`augmentor/backup.py` 里现量若干处，守卫按「集合恰为 `{.backups}`」钉）；③ 端点省略 `backup_dir` 时走 `default_backup_dir()`，返回的是**白名单首个根目录下的 `.backups`**（出厂即 `data/.backups`，实测是绝对路径）。⇒ 同一句「默认备份目录」在 CLI/SDK 面上解析成**当前工作目录**下的 `.backups`，在 HTTP 面上解析成 `data/.backups`；`cwd` 恰好是 `data/` 时两者重合，其余时候是两份不同的目录。**为什么值得立账**：这不是「有三个默认值」那么轻——用户用 CLI 建了备份、再到页面上找，看到的是空列表（反之亦然），而两边都各自「按默认」行事，谁都没写错路径。**当前处置**：服务层不抄第二套默认（守卫钉住 `api.ts` 里不出现 `backup_dir` 的字面量默认、页面上也不给这一格的输入），页面卡片与 `docs/augmentor/API.md` 各写明三份默认的关系。**修法方向**：① CLI/SDK 的相对默认改成解析到白名单首根（与端点同源，是行为变更：`cwd` 下不再生成 `.backups`）；② 或把三份默认收敛成一个函数，三面都调它；③ 维持现状但在 CLI 帮助与文档里写明「相对当前工作目录」。定级 S，待拍。 | S |
| A302 | 三只写操作挂 `verify_api_key`，而前端没有送 `X-API-Key` 的通道 ⇒ 设了 key 之后页面上的写操作一律 401 | **已关闭（L233，走修法①；原立 L231）**：实测 `POST /api/system/backups`、`POST …/restore`、`DELETE …/{backup_id}` 三条都挂 `Depends(verify_api_key)`，`GET /api/system/backups` 没挂（守卫按 `route.dependant.dependencies` 逐条读，`gated == seen - {GET}`）。而立账当时 `web/src` 里带引号的 `X-API-Key` 字面量现量**零处**（全文提到这个头名的只有 `System.tsx` 那一处老实文案），axios 实例也没有拦截器注入任何鉴权头。**后果**：`AUGMENTOR_API_KEY` 一旦设上，页面上所有写操作（不止备份这一族：改配置 / 删数据 / 上传 / 导出 / 回滚版本同规）都会 401，而界面只能显示服务端那句判决，用户没有任何地方能填 key。**当时的处置**：页面卡片写明这一条，守卫同时钉住「三条写操作确实挂了闸」与「前端确实没有送头的通道」两格现量。**修法方向**：① 前端加一处 key 输入（localStorage + axios 请求拦截器），并在 401 时引导；② 或改成同源 Cookie / 会话（部署面改动更大）；③ 或明确记档「设了 key 就只走 API 客户端，页面只读」并把页面上的写操作在设 key 时禁用掉。**注意 ①③ 都是产品决策**，不是接线。**L233 收口（用户拍板走 ①）**：仓外探针把整张契约面重量了一遍，规模比立账时以为的大一个量级——75 条操作里 **24** 条挂 `verify_api_key`，其中 **21** 条正是前端已接线的形状（立账时只数了备份那一族的 3 只），另 **3** 条挂门禁但还没接线（`translate` / `migrate` / `stream`），且门禁里**没有一条 GET**。通道落在 `web/src/services/api.ts` 一处：`API_KEY_HEADER` / `API_KEY_STORAGE_KEY` / `getApiKey` / `setApiKey` + 请求拦截器（留空即不送头 ⇒ 零配置照旧可用），401 的指路加在 `apiErrorDetail` 这个唯一漏斗上（一处改动，21 个动作同时被指路），key 的输入界面在「配置管理 → 写操作鉴权」卡上。l231 那支守卫的「前端零通道」一维**前提已变**，按新世界重钉为只留后端门禁事实（不是放宽），前端通道改由 `tests/unit/test_api_key_channel_l233.py`（18 支）与 `api.test.ts` / `Settings.test.tsx` 两面钉。注入 14 臂逐臂点名喊红。定级 M ⇒ **关闭**。 | ~~M~~ 已关闭（L233） |
| A303 | `backup()` 里两次 `datetime.now()`：秒级 `backup_id` 与微秒级 `timestamp` 跨秒时对不上，且 `name` 不在回执里 | **新立（L231）**：`backup_id` 由 `datetime.now().strftime("%Y%m%d_%H%M%S")` 生成（秒级），回执里的 `timestamp` 由**另一次** `datetime.now().isoformat()` 生成（微秒级）——实测两次调用之间隔着读文件、写盘、算 sha256，跨秒时会出现「id 写着 175833、timestamp 写着 …175834」。守卫按 AST 数 `backup()` 里的 `datetime.now()` 调用次数（现量 **2**）。相邻的一格：回执七键里**没有 `name`**（`backup_id` 就是 `name or 时间戳`），所以调用方给过 `name` 之后无法从回执里区分「这是我起的名字」还是「服务端生成的 id」。**为什么值得立账**：id 与 timestamp 在同一条记录里语义上应该是同一时刻，而它们不是；排序、去重、按时间清理（**A300** 的 `clean_old_backups`）都读这两格。**修法方向**：① 取一次 `now()` 复用（一行改动，行为上更自洽）；② 回执补 `name` 那一格（响应模型加字段，键集尺子随之重钉）。**L250 已收口（走修法①，与 A298 同批，原立 L231）**：`backup()` 里那两次 `datetime.now()` 收成一次——`now = datetime.now()` 之后，`backup_id` 取 `now.strftime('%Y%m%d_%H%M%S')`、回执 `timestamp` 取 `now.isoformat()`，同一刻的两格自此不可能跨秒。**判据两把（一把静态一把行为）**：AST 数 `backup()` 体内 `datetime.now()` 的调用次数恰为 **1**（`test_backup_wiring_l231.py::test_the_id_and_the_receipt_timestamp_come_from_one_clock_read`），以及假钟只许读一次、第二次读当场响的行为差（同文件 `test_a_second_clock_read_would_change_the_answer`，钉 `backup_id[-6:] == timestamp[11:19]` 去冒号那一格）——前一把会被「把第二次调用挪进注释或别处」喂饱，后一把不会；这正是 L243 记过的 A331 那一族盲区（只判落点不判内容的尺子）在本轮的处置姿势。**`name` 不在回执里那一格本轮没做**：它是「响应模型加字段」的活，要同时动 `test_api_openapi_contract` 的键集尺子与 `web/src/types/api.ts` 的 `BackupInfo`（L237 建的跨语言两层对账），和「一次读钟」不是同一件事，捆在一起收口会让人把「收了①」读成「整条清了」⇒ 已拆成独立的一行继续待拍，见 **A335**。定级 ~~S~~ 已收口（L250）。 | ~~S~~ 已收口（L250） |
| A304 | `/api/data/list` 只列 `train_data*.json` ⇒ 备份源的候选口径窄于白名单 | **新立（L231）**：页面「备份源文件」那一格的候选取自 `GET /api/data/list`，而它的过滤口径实测只认 `train_data*.json` 形状的名字——白名单根里放 `p.json` 与 `train_data1.json`、`train_dataX.json` 三份时，列表只回后两份（守卫用同一组夹具钉住这一格）。⇒ 用户想备份一份不叫这个形状的文件（比如 `convert` / `merge` / `sample` 的产物，它们的名字由调用方给）时，下拉里没有它。**当前处置**：那一格做成 `AutoComplete`（既能选也能手填任意路径），卡片写明「候选只列 `train_data*.json`，别的名字要手填」，手填的路径仍受白名单管（实测白名单外绝对路径 403、`..` 400、空串 400）。**修法方向**：① 给 `/api/data/list` 加一个「不按名字过滤」的档（query 参数或新端点），候选与白名单同宽；② 或让写盘端点的产物统一落一个可列举的目录；③ 维持现状（页面已给手填通道）。**L251 走 ① 落地**：宽档做成可选布尔 query 键 `all_files`（默认 false ⇒ 默认档一字未动，`""` / `false` / `0` 三种写法的**整条目**逐格相同），它只放开**名字**这道闸——扫描范围仍然逐字来自 allowed_data_roots、`glob` 不递归、按 resolve 后的路径去重，所以宽出来的那几份照样读得到（两档喂回 `/api/data/load/{filename}` 都不拿 403 也不拿 404）。前端只让系统状态页取宽档，而那一页六处候选下拉共用同一份 state ⇒ 变宽的是这一页的六处，其余 8 处数据文件选择器继续吃默认档。② 未做（没有新增可列举的落盘目录），③ 也没当成终态（手填通道仍在）。判据 **19 支 / 七类**在 `tests/unit/test_data_list_all_files_l251.py`。定级 ~~S~~ 已收口（L251）。 | ~~S~~ 已收口（L251） |
| A305 | `POST /api/system/dependency/datasets` 的 `description` / `tags` **回显即丢**：响应带着这两格，随后的 `GET` 与磁盘 `datasets.json` 里都是空 | **新立并同轮收口（L232）**：接线血缘卡时仓外探针逐档实测。请求体 `{name, input_file, description: "描述B", tags: ["t1","t2"]}` ⇒ **200**，回执九键里 `description` 与 `tags` **都在**；紧接着 `GET …/datasets` 里同一条的这两格是 `""` 与 `[]`，磁盘 `datasets.json` 同样是空。根因在路由里读得出来：`register_dataset()` 内部就 `atomic_write_json` 落了盘，端点却在这之后才 `info.description = …` / `info.tags = …`，改的是**内存里那个返回对象**。⇒ 契约声明得进、存不下来，而**只看响应的那把尺子在改前也是绿的**（回执确实回显了），这就是本条值得单独记账的理由：「响应形状对」不等于「写进去了」。**修法（本轮已落）**：把两格随登记一起传下去 `manager.register_dataset(name, path, item_count, description=…, tags=list(… or []))`（SDK 形参本来就有这两格，`augmentor/dependency.py:142-144`），删掉事后赋值那四行；`api/routes/system_ops.py` 净 **+1 行**。**判据**：`tests/unit/test_lineage_wiring_l232.py::test_description_and_tags_reach_the_disk_not_just_the_receipt` 只读磁盘与 `GET`、**不读响应**，注入臂1（把修法改回改前形状）实证它点名喊红。 | ~~S~~ 已修（L232） |
| A306 | 同名再登记是**覆盖**：`created_at` 被刷新、第二次送空值会把旧值抹成空，且没有 409 也没有任何提示 | **新立（L232）**：`dataset_id = md5(name)[:8]`（实测 `dsA` → `8c7f0201`、`dsB` → `0d2cace8`），注册表顶层是 `dict: id → 条目`，所以同名同文件再 `POST` 一次 ⇒ **200**、索引与磁盘各 **1** 条（备份那一族自 L250 起也是「一行对一份文件」，与这里**同形**，A298 那一格已按现量改写成一行一份），但 `created_at` 被**刷新**成第二次的时间，`updated_at` 同步；第二次送空 `description` / 空 `tags` 会把第一次写进去的值**抹成空**。**为什么值得立账**：`created_at` 在语义上是「这条登记什么时候建的」，而它现在等于「最后一次覆盖的时间」，且调用方拿不到任何「你刚才覆盖了一条已有登记」的信号 ⇒ 前端上「登记」按钮的行为是幂等的假象，误点两次就静默改掉一条既有登记。**当前处置**：卡片写明「id 只由名字算（md5 前 8 位）、同名再登记是覆盖、`created_at` 会被刷新」，守卫钉住「同名两次 ⇒ 索引仍 1 条」这一格现量。**修法方向**：① 已存在同 id 时回 **409**（行为变更：今天 200）；② 或 200 但回执里补一格 `overwrote: true`（响应模型加字段，键集尺子随之重钉）；③ 或把 `created_at` 真的保住、只动 `updated_at`（最贴合语义，是行为变更）。定级 M，待拍。 | M |
| A307 | 两支**只读**端点会把注册表目录建出来（读面写盘） | **新立（L232）**：`GET …/dependency/datasets` 与 `GET …/dependency/graph` 在注册表不存在时都回 **200**（`{"datasets": []}` / `{"nodes": [], "edges": [], "issues": []}`，不是 404 也不是 null），而探针量到**请求之前目录不存在、请求之后存在**（空目录，此时还没有 `datasets.json`）——`DependencyManager.__init__` 里 `mkdir(parents=True)`。**为什么值得立账**：`GET` 在只读白名单语义下不该产生文件系统副作用；这一格的实际后果是「探活一次就多一个目录」，而在只读挂载 / 配额受限的部署面上它会变成 500（`to_http_error` 兜底），页面上只会看到一句服务端判决。**当前处置**：新守卫 `test_the_two_read_endpoints_create_the_registry_dir` 把这一格现量钉住（断言「请求前不存在、请求后是目录、且此时没有 `datasets.json`」），卡片与 `docs/augmentor/API.md` 各写明「只读但会建目录」。**修法方向**：① 构造管理器时不 `mkdir`，改成写之前才建（读面就真的只读了，是行为变更）；② 或给 `DependencyManager` 一个 `readonly` 档，两条 `GET` 走它；③ 维持现状但把这一格写进契约说明（本轮已做）。**L249 已收口（走修法①，与 A332 同根同批）**：`DependencyManager.__init__` 那行 `mkdir(parents=True)` 摘掉，构造改成零磁盘副作用；父目录由写侧自建（`atomic_write_json` 内部就建 parent），所以 `register_dataset` / `add_dependency` 的落点一字未变（实测 `wr/datasets.json`）。同批把备份那一族（A332）按同一修法收掉，于是这一族在 SDK / HTTP / CLI 三面一次性归零：改前探针量到 13 条只读通道只有 1 条干净且干净靠路由挡板，改后 13 条全部零残留。**判据**：`tests/unit/test_readonly_no_disk_writes_l249.py`（22 支十组判据，注入臂 d1 把 `mkdir` 放回去 ⇒ 7 支点名喊红），加上原地反转的旧判据——`test_lineage_wiring_l232.py` 那一格由「请求后是目录」改成「请求后目录不在场」（改名 `test_the_two_read_endpoints_leave_no_registry_dir`），`tests/unit/test_dependency.py::test_init` 与 `test_round68.py` 的两支默认落点同批重钉（后者补一次真写，好把「延后建」与「不建」分开，否则重钉会把功能删成恒真）。文档面 `docs/augmentor/API.md` 两行、前端血缘卡片与 `api.ts` 注释同步改成「只读且不写盘」。定级 ~~S~~ 已收口（L249）。 | ~~S~~ 已收口（L249） |
| A308 | 「默认注册表目录」在仓内有**三份**，而端点那一份与另外两份不是同一个地方 | **新立（L232）**：与 A301（备份目录）同型、同根。三份实测——① CLI `--registry-path` 默认 `.dependency_registry`（`augmentor/cli/parser.py:472`，相对**工作目录**）；② SDK `DependencyManager.__init__` 形参默认也是 `.dependency_registry`（`augmentor/dependency.py`，相对**工作目录**）；③ 端点省略 `registry_path` 时走 `default_registry_dir()`＝**白名单首个根目录下**的 `.dependency_registry`（出厂 `data/.dependency_registry`，实测是绝对路径）。⇒ 用户用 CLI 登记了数据集、再到页面上找，看到的是空列表（反之亦然），而两边都各自「按默认」行事，谁都没写错路径。**当前处置**：服务层不抄第二套默认（守卫钉住 `api.ts` **代码**里不出现 `.dependency_registry` 字面量、页面上也不给这一格的输入；页面正文里出现它是**诚实口径**，判据只看 `strip_comments` 之后的代码面），`docs/augmentor/API.md` 白名单那一节与卡片各写明三份的关系。**修法方向**：① CLI/SDK 的相对默认改成解析到白名单首根（与端点同源，是行为变更：`cwd` 下不再生成 `.dependency_registry`）；② 或三份默认收敛成一个函数、三面都调它；③ 维持现状但在 CLI 帮助与文档里写明「相对当前工作目录」。**与 A301 合并处置更省一次破坏性变更评估**（同一族、同一个修法、同一批用户可见后果）。定级 S，待拍。 | S |
| A309 | 登记过的数据集在产品面上**删不掉**，而依赖边没有任何产品级创建入口 | **新立（L232）**：HTTP 面上 dependency 路由现量**只有三条**（两支 `GET` + 一支 `POST`，守卫按 (方法, 路径) 集合精确钉住，多一条少一条都红）：无 `DELETE`、无注销、无建边入口。SDK 侧 `DependencyManager` 的公开方法有八个，而 `add_dependency` / `remove_dependency` / `get_dependencies` / `validate_dependencies` **四个一个都没有 HTTP 入口**，且 SDK 连注销数据集的方法都没有 ⇒ 登记了只能手改 `datasets.json`。相邻的一格：`edges` 只能由 SDK 的 `add_dependency` 建立，所以**纯前端用户的 `edges` 恒为空**，页面上「图中边」那一格恒为 0 而「依赖图」这个标题会让人以为坏了。**为什么值得立账**：这是「能力在库里躺着、产品面上够不着」的又一例（与 A300 同型），而且它是**成对**的——登记有入口、注销没有；图有入口、建边没有 ⇒ 用户能把注册表写脏却清不干净。**当前处置**：卡片写明「边只能由 SDK `add_dependency` 建立、纯前端 `edges` 恒为 0、登记过的数据集在产品面上删不掉（SDK 也没有 `remove_dataset`）」，守卫钉住「四条 SDK 方法都没有 HTTP 入口」与「三条端点里没有 DELETE」两格现量（前者按 openapi 的 `operationId` 查，因为按路径串查 `add_dependency` 是恒真的——那是一把不会响的尺子）。**修法方向**：① 补 `DELETE …/dependency/datasets/{dataset_id}` 与 `POST …/dependency/edges`（两只都要挂 `verify_api_key`，与 A312 一起拍）；② 或先在 SDK 补 `remove_dataset`，再各加一条端点；③ 维持现状但把「边」那一格从页面上撤掉（少一格恒 0 的死面）。定级 M，待拍。 | M |
| A310 | `validate_dependencies()` **不查环**：A→B 与 B→A 成环时 `issues` 里零报告 | **新立（L232）**：探针用 SDK 直写三条边（`dsA → ghost12345` derived、`dsA → dsB` merged、`dsB → dsA` filtered）后打 `GET …/graph` ⇒ **200**，`edges` 三条都在，而 `issues` 只有**悬空那 1 条**（`{"type": "missing_target", …, "message": "目标数据集不存在: ghost12345"}`）；环那一格 `issues` 里查不到任何 `cycle`/环相关的 `type`（实测 `type` 集合就是 `["missing_target"]`）。读实现也一致：`validate_dependencies` 只发 `missing_source` / `missing_target` 两种。**为什么值得立账**：页面上「校验问题」那一格是 **0 就绿**的口径，而 0 只证明「没有悬空端点」，不证明「依赖关系没问题」——一个环会让下游任何按依赖顺序重建的流程死循环或取到过期数据，而这件事在产品面上完全静默。**当前处置**：卡片写明「`issues` 只报悬空端点、**不查环**，0 不能读成依赖关系没问题」，守卫钉住「成环时 `issues` 仍只有悬空那 1 条」这一格现量。**修法方向**：① 给 `validate_dependencies` 加环检测（DFS 染色），`issues` 多一种 `type: "cycle"`（响应模型不变，但 TS 侧 `DependencyGraphIssue.type` 的取值域要跟着写进文档与用例）；② 或只在文档与页面上写清边界（本轮已做）。定级 M，待拍。 | M |
| A311 | 这一族的错误体是**双轨**的：缺必填字段是 FastAPI 英文 **422**，其余坏参数一律中文 4xx | **新立（L232）**：实测缺 `input_file` ⇒ **422**，`detail` 是 FastAPI 那份英文结构（`Field required`）；而 `input_file` 的其余四档坏值全是中文：不存在 **404**「文件不存在」、白名单外绝对路径 **403**「路径超出允许的数据目录范围」、带 `..` **400**「路径包含非法组件」、空串 **400**「文件路径 不能为空」（label 与「不能为空」之间那个空格是 f-string 原样），外加一档这一族独有的：顶层不是 JSON 数组 **400**「数据文件的顶层必须是 JSON 数组，当前是对象」；`registry_path` 那四档同样是中文。**为什么值得立账**：前端只有一处 `apiErrorDetail` 取错误文案，而 422 的 `detail` 是**数组**、4xx 的是**字符串** ⇒ 「填漏了必填项」这一种最常见的用户错误，界面上拿到的是一段英文结构体而不是那句话。**当前处置**：卡片写明「两种错误体都要认」，守卫把 422 那一档与五档中文判决逐格钉住（参数化四臂 + 顶层非数组一臂）。**修法方向**：① 全局装一个 `RequestValidationError` 处理器，把 422 也翻成中文同一形状（是行为变更：状态码可保持 422，`detail` 的形状要变，`apiErrorDetail` 与所有认 422 的用例随之重钉）；② 或只在前端把 422 的 `detail` 数组渲染成中文一句；③ 与 **A280**（错误分档一刀切）合并拍——两者是同一件事的两面。**L272 量到本族更重的一格**（定级格未动，仍待拍）：422 那一轨遇到**非有限浮点**时不是「英文文案」而是**整个响应没了**：`POST /api/leakage/check` 传 `fuzzy_threshold=NaN`，starlette 的 `JSONResponse.render` 用 `allow_nan=False` 序列化错误体，而回显的 `input` 里就带着那个 NaN ⇒ 渲染阶段抛 `ValueError: Out of range float values are not JSON compliant`，TestClient 把它升成异常原样上抛，生产部署的对应物是「连 500 都没有」。同一路径的有限越界（`fuzzy_threshold=5.0`）仍正常回 422 ⇒ 炸的只是非有限那一形，不是整条端点。这一格把本条从「错误体文案双轨」抬成「**响应可得性**双轨」：修法 ①（全局 `RequestValidationError` 处理器）顺带就把回显里的非有限值消掉，② 纯前端渲染救不了它。守卫按观测行为钉在 `tests/unit/test_nonfinite_float_knobs_l272.py` 的`TestHttpFace::test_a311_new_cell_render_crash_stays_pinned`。定级 S，待拍。 | S |
| A312 | 三条里只有 `POST` 挂 `verify_api_key`，而前端没有送 `X-API-Key` 的通道 ⇒ 设了 key 之后页面上的登记一律 401 | **已关闭（L233，随 A302 一起走修法①；原立 L232）**：静态读 + 行为双证——`route.dependant.dependencies` 逐条读出来是 `POST …/dependency/datasets` 挂着 `verify_api_key`、两支 `GET` 都是空；设上 `AUGMENTOR_API_KEY` 后无头 `POST` ⇒ **401**「缺少或无效的 X-API-Key」、带对头 ⇒ 200，两支 `GET` 无头照样 200。而立账当时 `web/src` 里带引号的 `X-API-Key` 字面量现量**零处**（全文提到这个头名的只有 `pages/System.tsx` 一处老实文案，那是 L231 立的判据），axios 实例也没有拦截器注入任何鉴权头。**与 A302 是同一个缺口的第二例**（备份族三只写操作同规），差别只在这一族**只有一支写操作**、两支读操作不设防 ⇒ 设了 key 之后页面变成「看得见、动不了」。**当时的处置**：卡片写明这一条（头名逐字只写在卡片正文里，服务层刻意不复述——「提到这个头名的文件只有一处」是 L231 那条判据的量法，多一处回声就会把它判红），守卫同时钉住「只有 POST 挂闸」与「前端确实没有送头的通道」两格现量。**修法方向**：与 **A302** 一起拍（① 前端加一处 key 输入 + axios 请求拦截器 + 401 引导；② 或改同源 Cookie / 会话；③ 或明确记档「设了 key 就只走 API 客户端，页面只读」并把页面上的写操作在设 key 时禁用）——**三个都是产品决策**，不是接线。**L233 收口**：走 ①。这一族那支 `POST` 与另外 20 条已接线的写操作共用同一条通道（`api.ts` 的请求拦截器），页面上不必逐族再补什么；「带引号字面量只有一处」这条量法**保留并搬家**——现在那一处是 `api.ts` 的 `API_KEY_HEADER`（L231 当时是零处），`System.tsx` 卡片正文里那句仍是**不带引号**的老实文案，所以不算第二处通道；血缘卡与备份卡各指一次「配置管理 → 写操作鉴权」，由新守卫按 `count == 2` 钉住。定级 M ⇒ **关闭**。 | ~~M~~ 已关闭（L233） |
| A313 | `POST /api/system/stream` 的 `operation` 清单在 CLI 与端点两端不是同一份，而 CLI 独有的那一档 `export` 是**原样搬运** | **已关闭（L234）**：接线流式处理卡时两端对读量出来。CLI 那份 `--operation` 的 `choices` 写 `quality / dedup / clean / export`，端点认的是 `quality / dedup / clean / none` ⇒ 两头各错一半：① `export` 被 argparse 收下之后落进 handler 的 `else` 臂，而那一句只是 `lambda items: items`——**名字承诺的「导出」从来没发生过**，产物就是输入原样；② 反过来 HTTP 侧合法的 `none` 在 CLI 里被 argparse 直接拒（退出码 2）。**最贵的一格是它已经被钉成了事实**：既有测试的 docstring 逐字写着「export 操作为直通处理」，所以那把尺子改前是绿的、且它在为谎话作证。**修法**：把 CLI 那份字面改成与端点同一份（`export` → `none`），**没有新插 Python 常数**——把权威搬进 `augmentor/streaming.py` 会在 `WRITER_FORMATS` 之后插行、平移账本 A147 那条活引用（`test_doc_line_refs_l79` 与 `test_comment_refs_a184` 同时红），而仓内既有惯例（l226 / l175 / l189）也是「CLI 留字面、守卫对账」；同源由 `tests/unit/test_system_ops_wiring_l234.py` 的清单同源那一面钉住，注入臂 `cli_choices` 实证它点名喊红。**残留一格**：`export` 这个动作在产品面上仍然不存在（CLI 上曾经「看起来存在」），要真做导出得走 `POST /api/export/*` 那一族。⇒ 见循环日志 L234 | ~~S~~ 已修（L234） |
| A314 | `data/versions/current` 那一格：断链上 `Path.exists()` 抛 WinError 1920，把 `create_version` 与 `rollback` 整个炸掉；而链接目标写的是整段相对路径 ⇒ 恒断链 | **新立并同轮收口（L234）**：**这一条不是设计出来的，是全量门禁首跑那唯一一支红倒推出来的**——`OSError: [WinError 1920] 系统无法访问此文件。: 'data\versions\current'`，触发点是 `create_version` 里那句「先看那一格有没有东西」。现场取证量到三件：① 那份 `current` 是 **WSL 在 drvfs 上留下的 LX 链接**，`is_symlink()` 是 **False**、`os.readlink()` 报 *not a symbolic link*、只有 `os.stat` 抛 1920 ⇒ **「先判 is_symlink 再判 exists」这类重排治不了它**（我最初的修法正是这一版，被探针当场推翻）；② 链接目标是**整段相对路径** `data/versions/v_…`，而 `Path.symlink_to` 的相对目标按**链接自己所在目录**解析 ⇒ 实际指向 `data/versions/data/versions/v_…`，**恒断链**（成因是 `storage_dir` 的出厂默认就是相对路径）；③ 那个目录里当时堆着 **67** 个用例跑出来的 `v_*` 快照（⇒ A315）。**修法两处**：把「判存在 + 清除」搬进新方法 `_clear_current_link()`（先 `lstat` 再 `stat`，**stat 抛出来就当作「那里有东西」照清**；清除按「先当文件删、删不掉再当目录删」两步走，两步都失败就放过——这一格是便利索引不是数据，建不出新的也不该让 create/rollback 失败），链接目标改成**只写版本目录名**。**两处都刻意做成净零行**：账本里有三处活引用指着这只文件（`:205` / `:289` / `:390`），插行会把它们平移出代码跨度。**刻意没做的一件事**：不加 `target_is_directory=True`——本机 `os.symlink` 无特权直接吃 WinError 1314，没法验证「目录符号链接在 Windows 上能否被 `unlink` 掉」（`DeleteFileW` 对目录符号链接会失败、要 `RemoveDirectory`），引入一个无法验证的失效形状违反「失败要响」。**修完的现场证据**：那支用例转绿、原来读不了的 LX 链接被 `unlink` 自愈（`ls data/versions/current` 报 No such file）、`current.txt` 重写成最新版本 id。判据四面由 `tests/unit/test_version_current_link_l234.py`（10 支）钉住，注入臂 `old_clear` 红 4 支、`no_txt_clear` 红 2 支、`full_path_target` 红 3 支。⇒ 见循环日志 L234 | ~~M~~ 已修（L234） |
| A315 | A168 仓库残留守卫的范围欠账：`data/versions/` 与 `checkpoints/` 里堆的是用例跑出来的运行期产物，而原口径「往 `data/` 更深一层写不判」把它们整片放过 | **已关闭（L234）**：A314 那支红暴露出来的第二件事。原判据只盯仓库根 / `data/` / `.backups/` **三处顶层**，理由是「`data/` 更深一层是产品自己的目录结构」——这话对 `data/nested/` 成立，对 `data/versions/` 不成立：那里现量 **67** 个 `v_*` 快照全是测试跑出来的，A314 那份读不了的 LX 链接就住在里面。**也就是说 A168 那把尺子入库时（L94）量到的「全套跑下来 `RESIDUE_COUNT 0`」是真的，但它量的范围漏掉了产物真正落的那一层**——守卫不红不等于树是干净的。**修法**：`WATCHED` 三处 → **五处**（加 `data/versions` 与 `checkpoints`），口径表那一行同轮改成「写进 `data/` 更深一层（`data/versions/` 与 `checkpoints/` 除外）或 `tests/` 里 ⇒ 不判」。**代价是量出来的不是推的**：先跑「碰管道/版本」那一族 47 个文件 / 1 499 条用例，实测**泄漏 1 条**（`test_partial_branches_l120.py::test_malformed_variants_filtered_out`），就地补 `monkeypatch.chdir(tmp_path)` 收掉；扩范围之后全量 **8 687 条泄漏 0 条** ⇒ 与 L94 那次入库同形、**不需要白名单**。**新棘轮**：`test_the_runtime_dirs_are_watched_one_level_down`（把 `WATCHED` 改回三处它当场红），注入臂 `watched_three` 实证红 2 支。**这一支为什么住在文件末尾**：`test_repo_residue_l94.py` 第 227 行那处子进程调用是 `test_subprocess_text_decoding.py` 的 `BYTE_MODE_ALLOWLIST` 按「文件 + 行号」登记的调用点，在它前面插行会把登记指到空处、两条规则同时判红（实测插 23 行 ⇒ `227 → 250`，全量门禁 2 failed）⇒ 按处方优先级走「换点」而不是「重钉登记」。⇒ 见循环日志 L234 | ~~S~~ 已修（L234） |
| A316 | 页面把三个告警阈值写死在文案里（`completeness 0.7 / diversity 0.4 / consistency 0.7`），与 SDK 的 `_setup_default_thresholds()` 之间没有任何机器对账 | **新立（L234）**：接线自动质检卡时实测出来的。`web/src/pages/System.tsx` 的卡片正文写着「『触发告警』为 0 只说明这一份没触三个阈值（实测 completeness 0.7 / diversity 0.4 / consistency 0.7）」，那三个数来自 `augmentor/quality_monitor.py` 的 `_setup_default_thresholds()`（`alert_below` 逐档），**读数当时是用探针量出来的、写进文案之后就再没有任何尺子回头看它**。这与 A266 / A275 同族（契约面数字全靠人肉对账），但比那两条更容易烂：阈值在 SDK 里是一串构造实参、不是可引常数，改一个数不会让任何测试红，页面上那句话就静默变成假话。相邻两格也是同一次实测到的：① 告警文案是**英文单轨**（形如 `completeness below threshold: 0.50 < 0.70`），与本仓「4xx 中文判决」的口径不一致（⇒ 与 A311 同族）；② 这一支只比这三个数，**字段缺失、重复、长度异常都不在判据里**，所以「触发告警 0」不能读成「质量合格」——这句已写进卡片正文，但它同样是散文承诺。**修法方向**：① 阈值抽成 SDK 侧可引常数（`DEFAULT_QUALITY_THRESHOLDS` 一类），守卫逐项对账页面文案里的三个数；② 或页面不写死数字，改成从回执/端点读当前生效阈值（要求后端把阈值放进响应面，是契约变化）；③ 维持现状但把「这三个数是 L234 那次实测的快照」写进文案（最省，也最不抗漂移）。**L255 已收口（三处文案与 SDK 现用值立成同一把尺子）**：新守卫 `tests/unit/test_quality_threshold_copy_l255.py`（10 支 / 四面）。① **站点名册比原格多一面**：原格只点页面卡片，实测这一串数在仓内有**三处**——「系统状态 → 质量监控与自测」卡正文、`web/src/types/api.ts` 的类型注释、`docs/augmentor/API.md` 的 monitor 那一行；守卫把「三处各恰 1 处、总数精确等于 3」钉成名册，删一处或加一处都红。② **比的是值而不是字面**：0.70 与 0.7 是同一个数的两种排版，红的时候报「哪一档从几到几」；真正会红的是换写法（把三个数拆成「70%」那一类），那一形抽不到 ⇒ 站点数掉 ⇒ 由「换写法」那一支证明尺子不是恒真。③ **行为维在场**：把三项指标全压到阈值之下走告警那一路，要求正文里打印的阈值（0.70 / 0.40 / 0.70）与文案是同一个数——这一支把「页面抄的数」与「用户在告警正文里真能读到的数」钉成一份，SDK 调档或把格式从两位小数改掉都红在这里，而不只是红在文本对读那一支。④ **四臂注入取证**（B1 改文案一个数、B2 改 SDK 一个档、B3 拆掉一处站点、B4 给 SDK 加第四个带 alert_below 的指标）全部在内存里做，跑完三处文案的字节与进入时相同。**本轮量到的第二条是尺子的代价**：初稿把四臂写成一处 parametrize，定向复跑 l98 那把参数化普查两格红（derived 38 到 39、且与 l97 的调用点总数不合）⇒ 判据与参数化无关，按现量改成四条显式用例；同一次复跑另有五支 error，经核与本文件无关（会话级 fixture 拒绝「采不满声明下限」的那一跑形状，全量门禁里它们一直绿），归因如实写进守卫口径 7，不混成「新守卫让尺子红了」。**未做的两格如实留在本行**：修法②（把生效阈值放进 monitor 的响应面，让页面不再抄数）是契约变化、与 A335 同族，仍待拍；告警正文是英文单轨（形如 diversity below threshold 0.00 小于 0.40），与本仓4xx 中文判决的口径不一致，那一格归 A311 那一族，本轮只对账数字没有动语种。定级 ~~S~~ 已收口（L255）。 | ~~S~~ 已收口（L255） |
| A317 | 迁移卡的必填闸挂在 Popconfirm 的 `onConfirm` 上 ⇒ 用户先看见「照改」二次确认、点完才得到一句 warning | **新立（L234）**：`web/src/pages/System.tsx` 的 `handleMigrate` 开头两格判空（源文件、产物名）都在 `onConfirm` 指向的那个函数里，而 Popconfirm 包在按钮外面 ⇒ 交互顺序是「点『迁移结构』→ 弹出『迁移会改字段名，产物已存在则整份覆盖』→ 点『照改』→ 才发现自己没填东西」。**那句二次确认的措辞是有分量的**（它在为一次破坏性写盘要授权），而它在信息不全时照样弹出来，用户读完一段警告、确认了一次破坏性操作，得到的是「先填要迁移的那份服务端数据文件」。**为什么值得立账**：这不是样式问题，是**授权语义被空表单消耗掉了一次**——真正要拦的那次点击（填全了、确认覆盖）与这次白确认在界面上长得一模一样。同批接的其余四条里没有这一格（`auto-test` / `monitor` / `validate-config` 只读，`stream` 没有二次确认）。**修法方向**：① 把两格判空提到 Popconfirm 之前（按钮 `disabled` 或把确认框做成条件渲染），确认只在真要写盘时出现；② 或在确认框文案里带上当前两格的值，让用户在确认时就看见自己在迁移什么；③ 维持现状但把 warning 换成确认框内的一句提示。①最贴合语义、也是最小改动。定级 S，待拍。 | S |
| A318 | `validate-config` 的 `is_valid` 没有可引权威规则，而它与「问题清单条数」不是互补关系 | **新立（L234）**：接线配置校验卡时逐档实测。这一支校验不通过也回 **HTTP 200**，判决只在 body 的 `is_valid` 上 ⇒ 页面红绿不能看状态码（这一点已写进卡片正文）。真正没底的一格是 **`is_valid` 的确切规则在仓内没有可引权威**：本轮只观测到「`errors` 为 0 ⇒ `true`、`errors` ≥ 1 ⇒ `false`」这一族读数，而**出厂那份配置实测 `is_valid` 为 `true` 却带着 5 条「环境变量未设置: …_API_KEY」的警告** ⇒ `true` 不等于「没有问题清单」，`errors` / `warnings` / `info` 三档里只有第一档参与判决。**当前处置**：页面文案刻意**不写成定义**——它写的是「合法也不等于没有问题清单」加上那 5 条实测警告，而不是「`is_valid` 等于 errors 为 0」；守卫只钉观测到的行为（三档都渲出、判决来自 body 而不是状态码），**不钉一条推出来的规则**。**修法方向**：① 给 `is_valid` 写一句权威定义（在响应模型的 docstring 与 `docs/augmentor/API.md` 里各一份，并由守卫对账）；② 或把三档计数一起上线一个 `blocking` 标记，让「哪几条会翻判决」变成可读的；③ 维持现状但把「warnings 不参与判决」写进文档（本轮已在页面上写了，文档面还没有）。定级 S，待拍。 | S |
| A319 | 四个健康分权重既不校验非负也不校验上界，`health_score` 也不钳 [0,1]，而门禁三条规则根本不看权重 ⇒ 同一份回执里「healthy」与「verdict=failed」同时为真 | **新立（L235）**：接线健康门禁卡时逐档实测。① `POST /api/quality/health-gate` 的 `weights` 只校验两件事——「必须 4 个元素」与「之和 ≈ 1.0」（那两句 400 的原文是「权重必须包含 4 个元素」/「权重之和必须为 1.0」），**负数与大于 1 的值全收**；② `DatasetHealthScore` 加权求和之后**不做 [0,1] 钳制**，实测 `weights=[2.0,-0.5,-0.25,-0.25]`（和恰为 1.0）把健康分顶到 **1.1697944143478562**，`weights=[0,0,0,1]` 顶到 **1.0**（同一份夹具、默认权重下是 **0.8927055856521438**）；③ `level` 的三档阈值（healthy ≥0.7 / moderate ≥0.4 / 否则 poor，`augmentor/health_score.py:121`）只跟这个**未钳制**的分数比 ⇒ 1.1698 读成 `healthy`；④ 而门禁那三条规则（`pass_rate` / `duplicate_rate` / `completeness`）**一个都不看权重**，所以同一份回执里 `level=healthy` 与 `verdict=failed` 同时成立（本轮夹具上 `duplicate_rate` 恒 0.5）。**为什么值得立账**：页面上「健康分 1.17 / healthy」与「判定 failed」并排显示，用户读到的是一句自相矛盾的话，而两侧都没说谎——真正缺的是**一句权威说明「健康分不参与门禁判决」**（本轮已写进卡片正文与回执 Alert，但那是散文承诺，没有机器对账，与 A316 同族）。**修法方向**：① `health_score` 出口钳到 [0,1] 并把钳制写进 docstring（破坏性：会让既有读数变小）；② 权重加「非负」校验（同样破坏性，且会挡住「刻意用负权重压低某一维」这类用法——本轮实测这种用法确实能用）；③ 维持现状但在响应里加一个显式标记（如 `score_is_unclamped`），让前端不必靠散文解释。**L238 收口（选 ①+② 两半，③ 刻意没做）**：判据搬进 `validation.require_ratio_list`，健康分权重与 `quality.weights` 共引同一份（三元与四元同一个容差、同一句措辞），逐项 [0, 1] 并点到 `weights[i]` 的下标，`score()` 出口钳 `min(1.0, max(0.0, float(raw)))` ⇒ 实测那一档 **1.1697944143478562** 今天到不了；破坏性已承担并写进文档：L235 量出的「刻意用负权重压低某一维」这一用法自本轮起 400。**没做的那一半要说清**：钳制只消灭了「分数越界」，`level=healthy` 与 `verdict=failed` 并排仍然可能出现（门禁三条规则一条都不看权重，本轮夹具上 `duplicate_rate` 恒 0.5），页面文案因此从「自相矛盾」改成「两套判据各说各话」，`score_is_unclamped` 那一格没加 ⇒ 「健康分不参与门禁判决」仍是散文承诺，与 **A316** 同族、仍待拍。守卫 `test_decision_gates_l238.py::TestTheClampBites`（4 支，含「四项必须全 1.0 才顶得出越界」的自证臂 `raw > 1.0`，防恒真）+ `test_health_score.py::test_each_weight_must_be_a_ratio`（4 档点名下标）。 | ~~S~~ **已收口（L238）** |
| A320 | 权重之和的容差写成 `abs(sum-1.0) > 0.01`，而「±0.01 以内算 1.0」这句在恰好 0.01 那一档是假话 | **新立（L235）**：同批实测。`[0.24,0.25,0.25,0.25]` 与 `[0.26,0.25,0.25,0.25]` **都被拒**（400「权重之和必须为 1.0」），因为它们的浮点差是 **0.010000000000000009** > 0.01；而 `[0.255,0.25,0.25,0.25]`（和 1.005）**放行**。⇒ 实际边界不是文档与直觉里的「0.99 与 1.01 都算 1.0」，而是「比 0.01 严格更小」，且这条边界**由浮点表示决定、不由十进制决定**（0.99 那档刚好越界，1.005 那档刚好在内）。**为什么值得立账**：这是一条**用户会照着写、然后被拒**的规则——「四个权重之和留两位小数凑 1.0」是最自然的用法，而 0.24/0.26 正是两位小数里最常见的一档；错误信息只说「必须为 1.0」，不说是容差判的、也不说容差多大。本轮页面文案刻意写成实测口径（「0.99 与 1.01 都 400、1.005 才 200」），并由守卫 `test_a320_the_sum_tolerance_is_not_what_the_docstring_says` 钉住。**修法方向**：① 把容差抽成可引常数（`WEIGHT_SUM_TOLERANCE` 一类）并在 400 的 `detail` 里带上「实测和 = X，容差 = Y」；② 或改用 `math.isclose` 一类的显式口径并把边界说成「四舍五入到 N 位」；③ 维持现状但把「恰好 0.01 会被拒」写进 `docs/augmentor/API.md` 与响应模型 docstring（最省，也最不抗漂移）。**L238 收口（选 ①，并把 ② 的口径做实）**：容差抽成 `validation.WEIGHT_SUM_TOLERANCE = 0.01`，判和之前先 `round(abs(total - 1.0), WEIGHT_SUM_QUANTIZE_DIGITS)`（9 位）把浮点噪声消掉 ⇒ 「±0.01 **含端点**」第一次成为真尺子：实测 0.99 / 1.005 / 1.01 三档放行、0.98 与 1.02 拒，400 的 detail 带回实测和（`… 之和必须是 1.0（容差 0.01，含端点），当前 0.98`，守卫逐档 `endswith` 钉死）。`0.24,0.25,0.25,0.25` 的十进制差是 0.01 而 IEEE-754 求和是 **0.010000000000000009**——量化到 9 位正是为它，那条「留两位小数凑 1.0」最自然的用法不再被假拒。页面与 `docs/augmentor/API.md` 同轮换口径，L235 钉的旧断言「0.99 与 1.01 都 400」按新读数重钉。守卫 `TestTheToleranceIsRuled`（5 支，含 `DEFAULT_HEALTH_WEIGHTS` 不被调用方就地改写那一支）。 | ~~S~~ **已收口（L238）** |
| A321 | `text_field` 送空串或送一个数据集里不存在的键，后端零判决 ⇒ `duplicate_rate` 回 1.0，与「真的全重复」在响应里同形 | **新立（L235）**：同批实测，也是本轮前端**唯一一处刻意用 `\|\|` 躲判决**的地方。① `text_field=""` 与 `text_field="no_such_field"` 都**不报错**（200），`gate.metrics.duplicate_rate` 一律 **1.0**、`failed_rules == ["duplicate_rate"]`；② 同一份数据用真实存在的 `text_field="output"` 时 `duplicate_rate` 是 **0.5** ⇒ 那 1.0 不是数据性质，是**取不到字段**的产物；③ 健康分那四项（`completeness` / `diversity` / `quality_balance` / `coverage`）**压根不看 `text_field`**，实测 `completeness` 仍 1.0 ⇒ 回执里没有任何一格能告诉用户「你字段名写错了」。**为什么值得立账**：这是一条**假判决**——它长得与真判决（数据真的全重复）逐字节相同，而门禁的用途正是「按判决放行/阻断」，所以写错一个字段名的后果是**整份数据被拦下**，且拦下的理由看起来完全合法。与 **A285**（五处手写字段清单没有权威）同根：字段名既没有可引权威、也没有存在性校验。**当前处置**：前端 fetcher 把空串转成 `undefined`（不发这一格 ⇒ 走后端默认），页面文案写「`duplicate_rate` 变成 1.0 时先怀疑字段名」，守卫 `test_a321_a_ghost_text_field_produces_a_fake_duplicate_verdict` 钉住行为。**修法方向**：① 端点侧校验 `text_field` 存在于首条记录，不存在就 400 并点名（最贴合「假判决」这个缺陷类）；② 或在响应里回一个 `text_field_found: false`、或把该规则挪进 `errored_rules`（那一格实测**恒空**，正好是它该住的地方）；③ 维持现状但把这句写进 `docs/augmentor/API.md`。**L238 收口（选 ①，② 没做）**：新建 `validation.require_item_field(name, items, field, label)`——两刀各有分工：空串 / 纯空白 / 非字符串由 `require_string` 点名**参数名**，名字成立但这里没有这一列由存在性判据点名**首条实际字段**（拼错的人真正需要的是「那一份里到底有什么」）；存在性取「**至少一条**含该字段」而不是每条，因为数据集异构是常态（增强与合并的产物天然多列）。health-gate、`/api/dataset/impact` 的 `_require_any_field`（降级成一层 HTTP 外衣）、批量翻译**三面共引**，四档（空串 / 纯空白 / `no_such_field` / `Instruction`）SDK 与 HTTP 两面逐字节同文。判序也钉死：**参数错在读盘之前**（坏权重 + 文件不存在 ⇒ 400，不是 404），**字段错在读盘之后**（幽灵字段 + 文件不存在 ⇒ 404，没有数据就没有「首条实际字段」可点）。**② 没做**：既没有 `text_field_found`，`errored_rules` 也仍恒空 ⇒ 这一族是被 400 **拦住**的，不是被**如实标出**的。守卫 `TestTheTwoFacesSayTheSameSentence` + `TestTheOrderOfVerdicts`（含迟到绑定哨身：SDK 侧是函数体内 import，`is` 判身份对它不成立，只能按「迟到绑定」验）。 | ~~M~~ **已收口（L238）** |
| A322 | `POST /api/dataset/translate` 的 `text_key`：空串与幽灵键都零判决，产物凭空多三键而原字段一字未动 | **新立（L236）**：接翻译卡时逐档实测（3 条中文语料、`multilingual.enabled=true`、管道用替身好数模型调用）。① `text_key=""` 与 `text_key="no_such_field"` **都回 200**，且 `failed=0` / `failed_indices=[]` / `errors=[]` / `errors_truncated=false` —— **回执四键全是「全成功」的形状**；② 模型调用 **0** 次（替身数出来的）⇒ 这一次不花钱，但也不产出任何译文；③ 产物里每条凭空多出**三**个键：`<text_key>` 那一列（值为空串）、它的 `_original`（同样空串），以及一个 `language` 目标语言戳（实测 `en`）——第三键来自 `augmentor/multilingual.py:190` 那句无条件 `new_item["language"] = target_lang`，**一个字没翻也照盖**；④ 原字段（`instruction`）一个字没动，落盘那一份与响应体里的 `items` 逐格相同（实测键集 `{instruction, output, "", _original, language}` 与 `{instruction, output, no_such_field, no_such_field_original, language}`）。**为什么值得立账**：与 **A321**（`text_field` 幽灵键 ⇒ `duplicate_rate` 假判决）同一缺陷类，但这一条更会骗人——A321 至少在响应里留了一个可疑的 `1.0`，而这一条的**回执与真成功逐字节同形**，唯一的线索在产物里那一列空值上；又因为 0 次模型调用，它连「花钱买到了什么」这条通常的追查路径都不会触发。**当前处置**（后端一行未改）：页面自己拦下空值（`if (!trTextKey.trim())` 那道必填闸 + 一句点名 A322 的提示），卡上写明「**别把「200 且 failed=0」读成翻成了**——先核对产物里那一列」，守卫 `TestAThreeTwoTwo` 三支钉住行为与文案（其中静态那一支的注释写明「改了后端那天这两句要一起动」），`docs/augmentor/API.md` 的 translate 说明行同轮写清。**修法方向**：① 端点侧校验 `text_key` 非空且存在于首条记录，不满足就 400 并点名（最贴合缺陷类，与 A321 可并表执行）；② 或在响应里回一个 `text_key_found: false`、或把这一档挪进 `errors`（那一格实测恒空，正好是它该住的地方）；③ `language` 那一格要么只在真翻了的时候盖，要么改名成 `target_lang`——今天这个名字读起来像「检测到的语言」，而它是**目标**语言且无条件写入（改名是对外契约变化，故只立账）。**L238 收口（选 ①，③ 没做）**：`translate_dataset` 在分批之前共引 `require_item_field`（`name` 送 `text_key`），与 `backend is None` 那道闸同一条纪律——花钱路径上的参数错误不该先花一笔才知道。实测四档坏 `text_key` 一律 **400**、模型调用 **0** 次、**一个字节都不落盘**（写盘类探针先把 `AUGMENTOR_CONFIG_PATH` 指向仓外副本）；改前那份「200 / `failed=0` / 四键全零」的同形回执从源头消失。**③ 没做**：`language` 仍是 `multilingual.py:190` 无条件盖上的**目标**语言戳，改名是对外契约变化、不在本轮；而且存在性判的是「有这一列」，所以**列存在而整列为空串**时仍会走 SDK 的「回退原文并计入成功」⇒ 页面那句「别把『200 且 failed=0』读成翻成了」留着，它不是历史包袱。守卫 `test_last_two_wiring_l236.py::TestAThreeTwoTwo`（本轮把 L236 刻意留给「A322 自己那一轮」的三格判据接成真判据）+ `test_decision_gates_l238.py` 的同文面（CLI `run_health_gate` 与 impact 的「基线数据集」label 也在这条链上）。 | ~~M~~ **已收口（L238）** |
| A323 | `Settings.handleSave` 的 toast 是自己拼的第三处措辞，不读后端 `message` ⇒ 后端把 `ignored_keys` 判决拼进 `message` 那一档，页面上看不见 | **新立（L237）**：补 `UpdateConfigResponse.ignored_keys` 时逐档实测。后端两档措辞是「配置已保存，部分配置需要重启服务生效」与被忽略时的「配置已保存，部分配置需要重启服务生效（已忽略未写入的配置项：qualty；是否想写 quality？）」，另有空写入档「本次请求未写入任何配置项，config.yaml 保持不变」；页面 toast 写死的却是**第三处**「配置已保存到 config.yaml，部分配置需要重启服务生效」——它不读响应，所以被忽略的那串判决在 toast 里必然丢失。**可达性按实测收窄，不夸大**：空写入那一档从本页不可达——`handleSave` 恒带 `default_model`，而实测只发 `default_model` 那一格就 `applied` 非空、回「配置已保存」并真落盘（三档探针都跑在仓外副本上：`{}` ⇒ 「未写入任何配置项」且磁盘 sha256 不动；只有 `default_model` ⇒ 「已保存」且磁盘变）。**当前处置**：L237 新立的「上次保存的回执」卡把后端真回的 `message` 与逐条 `ignored_keys` 渲染出来，并写明 `success` 两边都是 true、它只表示「请求被正常处理」⇒ 判决在页面上已经可见，只有 toast 那一句还在说固定措辞。**为什么本轮不改 toast**：`Settings.test.tsx:142` 断言的正是那句硬编码文案，改措辞要连判据一起重钉，而本轮的证据链是「类型 ↔ 契约」，掺一条改判据的前端行为差会把两件事的证据混着算。**修法方向**：① toast 直接显示 `result.message`（一行改动，`Settings.test.tsx` 那句断言随之重钉）；② 或把「已保存」那句改成条件句（`ignored_keys` 非空时改口为「部分未写入」）；③ 或删掉 toast 只留回执卡。定级 S，待拍。 | S |
| A324 | `dataset_ops.split` 的三个分割比例仍是**手抄容差 + 不判长度**：完全合法的 40/30/29 切分被拒，SDK 直构喂 4 个元素则过校验后死在 unpack | **新立（L238）**：给权重族做「一条权威只住一处」时顺手量了同族的另一处——`split` 那三个比例**没有**走 `require_ratio_list`。① `train_ratio=0.4, val_ratio=0.3, test_ratio=0.29`（十进制和 0.99，浮点差 `0.010000000000000009`）实测被判「分割比例之和必须为 1.0，当前为 0.99」⇒ **A320 刚收掉的那个假拒在这一处还没收**，而「留两位小数凑 1.0」正是最自然的用法；② 同一处不判长度，SDK 直构把 `ratios` 喂成 4 个元素时**通过校验**，随后死在 `ValueError: too many values to unpack (expected 3)` ⇒ 一个 500 形状而不是 400；③ 三面可达性不同：API 与 CLI 由三个标量拼 tuple，所以长度那一半**只在直构面可达**，容差那一半**三面都可达**，这与 A320「同一份 YAML 在一面能加载、另一面被判拒」是同一个缺陷类的两种形状。**修法方向**：两刀都搬进 `require_ratio_list`（长度 3、逐项 [0, 1]、和为 1 ± 容差含端点）并三面共引，`split` 的比例形参改成显式具名校验。**为什么本轮不夹带**：A320 的四档实测、页面文案与 `docs/augmentor/API.md` 是同一条证据链，改 `split` 要牵动 `dataset_ops` 的既有读数与 CLI 帮助文本，另起一轮才守得住「数字必须来自同一条命令链」——那一轮要量的数在 L256 补齐。**〔A324 已收口（L256）〕**：门口那三行手抄 `abs(total_ratio - 1.0) > 0.01`（不含端点、不判长度与逐项）与分层门那两行 `abs(total - 1.0) > 1e-6` 加「只判负数」都删掉，`DatasetOperations.split` 与 `DataSplitter.__init__` 共引 `validation.require_ratio_list`（和 ±0.01 含端点、量化 9 位 + 逐项 bool/NaN/非数值/0..1 + 长度 3）。① 的 0.4/0.3/0.29 与 0.8/0.1/0.105 两支同放行（三段之和恒等条目数，配额由 `largest_remainder` 按 share/weight_sum 归一化）；② 的 4 元素在直构面从「过校验后死在 unpack」改成同一句 400 形判决；③ 的三面同答由 `tests/unit/test_transform_wiring_l229.py`（判红两档 + 判绿两档，两支 `detail` 逐字节相等）与 `tests/unit/test_dataset_ops.py` 钉住，防复发由 `tests/unit/test_decision_gates_l238.py` 的手抄名册（调用点 2 → 4）盯。A57 那一格（碰默认侧的破坏性变更）没有采用：默认侧原判「±0.01 不含端点」，新判含端点且不新增任何拒绝 ⇒ 可接受域只增不减。定级 ~~M~~ 已收口（L256）。 | ~~M~~ 已收口（L256） |

| A325 | `LeakageDetector.detect` 的近似匹配把**整份倒排按每条测试样本摊平重数一遍**：10k×10k 实测 **45.4 s**，是全项目当前最大的一格热点 | **新立（L239）**：性能现量探针（`Temp/l239_perf.py`，20000 条合成数据、uv 托管 cpython 3.12.14、sentence-transformers 未安装⇒走 fallback）。按 10k/10k 切给 `detect()` 的 wall time best **45411.2 ms**，cProfile 里 **41.047 s 的 tottime 全在 `_collections._count_elements`**、调用次数 **9988**（⇒ 每次约 4.1 ms），出处是 `augmentor/leakage.py:233` 那句 `Counter(chain.from_iterable(gram_index.get(gram, ()) for gram in test_grams))`。形状解释：Jaccard 只用到「共享三元组数」，所以每条测试样本都要把自己全部三元组的 posting set **串起来数一遍出现次数**；训练侧 posting 越宽（真实指令文本里常见子串能进几千个 sid），单条摊平就越接近「重跑一遍全索引」。对比同批读数：`Deduplicator.deduplicate` 20k 才 1065 ms ⇒ leakage 这一格比它大 **42 倍**。**为什么值得立账**：泄漏检测是「增强产物还能不能进评估」的守门面，API 面可达，而 45 s 量级会让一次页面点击看起来像卡死；且它随 posting 宽度而不是随数据条数恶化，**合成随机数据已经这么慢，真实中文语料只会更慢**（子串共享率远高于随机五字母词）。**修法方向**：① 按长度带剪枝——`J >= t` 蕴含候选 sid 的三元组数 `|b| ∈ [t·|a|, |a|/t]`（t=0.8⇒`[0.8|a|, 1.25|a|]`），把 posting 按 `sig_gram_counts` 有序化后用两次 `searchsorted` 夹出带内段（`dedup.py` 的倒排块就是这套 CSR 做法，仓内已有先例）；② 或给「出现次数低于 `need`」的 gram 先做 posting 长度预筛；③ 或按 dedup 同款代价预言机在「摊平计数」与「逐候选求交」之间选路。**为什么 L239 不动手**：本轮定义是「零对外行为变化」，而 ① 要重写 `leakage.py` 的索引形状，判据「产物逐字节相同」需要新的对照守卫与注入取证，`leakage.py:182/186/233/236` 四处行号还被现存注释点名，另起一轮才守得住口径。定级 **M**（性能面，本仓当前最大可量化收益），L240 主项。**· 已收口（L240）**：落地走的是原案处方 ③（dedup 同款代价预言机在「摊平计数」与「稠密矩阵乘法」之间选路），处方 ①（长度带剪枝）量化后否决——`Temp/l240_leakprobe2.py` 现量 4000×4000 档 posting 摊平 37,328 → 带内 17,298 条目/条测试 ⇒ 只省 **53.7%**，常数减半而量级不变。本行原框那句「`leakage.py:182/186/233/236` 四处行号还被现存注释点名」是**改写前的现场读数**，按 A127「账本允许时间戳式引用」保留不批量改写；实测那四处号里没有一条被产品注释点名（`test_comment_refs_a184` 硬 0 档本轮未红），真正需要重钉的是卷一 L55 那一处 `leakage.py:124`，本轮净 +196 行把它平移出跨度 ⇒ 已按处方 3 重钉 `leakage.py:352`。10k×10k 收益 **48 929.0 → 1 742.1 ms（28.1×）**、六档报告逐字节相同、八臂注入 7 臂红；新守卫 29 支全部计数判据。⇒ 详见 L240 日志块。 | ~~M~~ **已收口（L240）** |

| A326 | `Deduplicator` 在 fallback 编码器且代价预言机判稠密赢时，**同一批文本被逐字符扫两遍**（`_inverted_view` 选路 + `_batch_encode` 正式编码） | **新立（L239）**：同批探针 profile `deduplicate` 20k 的 cumtime：`_ngram_entries` **1.176 s** / `_inverted_view` **1.242 s** / `_fallback_encode` **0.775 s**（内部 2 511 040 次 `dict.get` + 2 499 756 次 `list.append`）。`_similarity_blocks`（`dedup.py:473-510`，L239 记时为 451-481）在 `self._model == "fallback"` 时**先建一份倒排视图喂 `_inverted_blocks_win` 选路**，判负才把 `inverted = None` 松手、走 `_batch_encode` 的稠密路 ⇒ 稠密那一条把 n×vocab 的字符扫描做了两遍。实测真语料 6902 条 / vocab 23033 判**倒排赢**（969 ms vs 10484 ms），所以这条重复扫描在生产主路径上**不成立**，只在「短文档 / 高重复 / 小词表」这类稠密档里出现。**为什么值得立账**：它是「为了选路而付出的成本在选路失败时被完整丢弃」的形状——预言机的读数越保守，被浪费的那一份越大。**修法方向**：① 让稠密路复用 `_ngram_entries` 已经产出的词条/文档流（同一趟扫描同时喂两边）；② 或把预言机改成不建实体索引、只用 n 与词表规模的估计量（现在它真读了 `doc_count`/`vocab_size`/`pair_adds`）。**为什么 L239 不动手**：`_batch_encode` 是**测试注入向量的接缝**（`dedup.py` 的注释与多支守卫按它下钉子），合并成单趟这条路 A19/L2/L10 已记录被显存护栏 `test_fallback_encode_peak_holds_only_one_matrix` 否决过一次；且 `dedup.py` 身上挂着 **6 处** `dedup.py:NNN` 历史行号引用（104/132/193/242/607/830），插行必平移。定级 **L**，待拍。**L248 已收口（走修法 ① 的收窄版：只交列号表，不交出现数组）**：`_similarity_blocks` 把选路那一趟建好的列号表直接交给稠密那条（`_batch_encode` / `_fallback_encode` 各加一格可选 `vocab`），稠密档总共只扫两遍字符；判倒排赢时列号表连逐趟数组一起在产出第一个块之前松手。四档语料的 dedup 判决产物 sha256 **逐字节不变**、预言机入参一格未动；两步 wall 之和 D1 −49 %、D2 −56 %、R1 −36 %、R2 −38 %（R2 是纯选路那一趟的节省），峰值 25.2→21.8 / 26.4→21.5 / 46.1→46.2 / 230.5→228.7 MiB。原格修法 ① 的完整版「把 `_ngram_entries` 的出现数组直接散射进稠密矩阵」按实测否决——nnz 级 int64 索引数组可达矩阵字节的 ~5 倍，与 A19 / L10 记过的 6.8× 峰值否决同形；「只把 readings 与 fold 拆开」实测 ~5 % 落在噪声里，未采纳。守卫 11 支 + 五臂注入取证见循环日志 L248。本行原写的 `dedup.py` 跨度 451-481 现量 473-510（L248 在其上方净插 29 行所致，所指未变）。另记一行 A326 原格「为什么不动手」的账：**接缝可以扩，只要既有注入点全部同步**——本轮给 `_batch_encode` 加的是**可选**形参，`tests/unit/test_dedup.py` 4 处 lambda 同签名即可，没有一条既有用例的可观测语义被动过。定级 ~~L~~ 已收口。 | ~~L~~ 已收口（L248） |

| A327 | `QualityScorer._calculate_diversity` 每选一个候选都重算一遍已有向量集合的**逐行范数**：FakeModel 顶上 n=4000 实测 28.5 ms ≈ 整个向量侧的 18% | **新立（L239）**：同轮 quality 探针（`Temp/l239_quality.py`，FakeModel 用 `zlib.crc32` 顶掉 sentence-transformers，每轮重置 `_embedding_cache` 以免 `hasattr` 短路让第二次跑白捡缓存）。`batch_score` n=4000 best **156.9 ms**，其中 `encode` 只被调 **3 次 / 8030 行**⇒ 惰性批量编码那层（L231 收的重复编码）确实生效、没有重复编码；剩余可定位成本是 `_calculate_diversity` 里对 `_existing_embeddings` 每次候选重算 `np.linalg.norm(..., axis=1)`，单点插桩 **28.5 ms**。顺手量了同族的另一刀并**量化后否决**：语义相似度那个逐元素 `np.dot(a[i], b[i])` 循环换成 `np.einsum("ij,ij->i")` 快 **3.73×**（14.26 ms → 3.82 ms），但**产物不位相同**——float32 最大差 **2.98e-07**、float64 最大差 **8.88e-16** ⇒ 按本仓「性能改动必须产物逐字节相同」的口径不予采纳。**为什么值得立账**：1–2% 的端到端收益在真实语义模型（n×dim 更大）下会放大成可观常数，而行范数缓存是纯内部改动、可零对外行为变化。**为什么 L239 不动手**：收益量级（真模型下端到端 1–2%）撑不起一次改动——`quality.py` 身上挂着**十余处** `quality.py:NNN` 历史行号引用（417-433、312、133/153、85-89、81、80 等），任一处插行都要重钉一串历史引用；本轮先把读数钉进账，动手留给能同时把「缓存失效边界」（`_embedding_cache` 与 `_existing_embeddings` 的寿命）说清楚的那一轮。**L246 已收口**：原格点名「动手留给能把缓存失效边界说清楚的那一轮」，本轮就是那一轮——行范数改为在参照集那一次编码的同一处刷新，**不另设失效判据**（它是 `_existing_embeddings` 的纯函数，寿命同它），`reset_cache()` 同清两格。四档现量（n=4 000、`include_semantic=False`、FakeModel 顶掉 sentence-transformers；axis=1 的次数为精确读数、wall 为单次）：窗口 30 / 维 64 的「参照集稳定」档 **4 000 → 1 次**（407.7 → 376.6 ms，−7.6%）、「参照集逐条达标」档 **4 000 → 1 次**（406.8 → 359.8 ms，−11.6%）、生产第一批形状（参照从 1 涨到窗口 30）**4 000 → 30 次 = 参照集编码次数**（430.4 → 404.8 ms，−6.0%）、真实语义模型量级（窗口 100 / 维 384）**4 000 → 1 次**（459.4 → 396.6 ms，−13.7%）。四档判决产物 4×4 000 格（每格的三个分与两个布尔量的 `repr`）**逐字节相同**，sha256 前后同为 `798206eed79354a5…`；本格那条「`np.einsum` 快 3.73× 但产物不位相同」的否决**维持否决**，本轮没有顺手做。**新守卫 8 支**（`tests/unit/test_quality_norm_cache_l246.py`）钉的是**次数**而不是数值，四臂字节级注入取证（挪回逐候选 / 缓存不刷新 / 范数 ×2 / 轴向 axis=0）分别咬中 4 / 6 / 2 / 5 条。顺带把向量矩阵那一格的注入接缝从「填一格」改成「两格同填」并点名钉住：只喂 `_existing_embeddings` 不喂 `_existing_norms` 时当场 `TypeError`，不静默拿一份不同源的范数算出一个数。**为什么 `quality.py` 一处历史行号都没平移**：全部改动走行内替换与同行压缩（encode 调用的三行参数压成一行，给刷新那一行腾位），净行数 7/7 = 0。定级 ~~L~~ 已收口（四档读数与 `einsum` 否决见本格）。 | ~~L~~ 已收口（L246） |

| A328 | 两卷 Backlog 表的关闭标记词汇与轮号可追溯性 | **新立（L242）**：本轮那把守卫只钉「进度声明面说收了 ⇒ 表行有标记」这一个方向。反方向（表行有标记 ⇒ 标记里的轮号能在**同一卷**的进度声明面查到）**本轮没钉**，现量三条：①补标之后 132 行有标记，按严格式「左括号 + L + 数字」解析轮号，有 **10 行**解析不出（限定词插在括号与轮号之间，例如卷一 A122 写作「已关闭 + 第二本账 + 轮号」的形状），另有 **3 行**（A46 / A130 / A138）解析得出但那一轮不在本卷——跨卷语义本身是对的，只是同一卷查不到；②放宽到行内任意轮号则 132 行全部「可查」，但那量的已经不是标记里的轮号而是行内任一提及，不足以当判据；③标记词汇形状现量 **7 种**且**没有封闭清单**（五个动词「已关闭 / 已收口 / 关闭 / 收口 / 已修」各配一个括号加轮号，再加两种划掉的定级字母 S 与 M），换一种新写法（例如未来的「已了结 + 轮号」）对守卫结构性失明。**本行自己就撞上过一次**：初稿把七种形状逐字写进格子里，于是那一格被标记判据当成「已关闭」，现量从 132 涨到 133——这就是形状失明方向的证据。修法方向：先给标记词汇立封闭清单（与 A77 单一权威口径同族），再把轮号查找做成「括号后允许限定词」的可解析式并跨卷对账。**L253 已收口（原格两件事都落成判据，并把标记判据的面从整行收到定级格）**：新守卫 `tests/unit/test_backlog_mark_vocab_l253.py`（20 支 / 四面）。① **词汇封闭清单**：定级格拆成四件结构件——字母形、闭合词、括号头、是否粗体——组合名册现量 **16 种**逐格点名（`MEASURED_COMBOS` 精确相等），字母形 **7 档**（裸 S 93 / M 56 / L 2 / XS 1，划掉 ~~S~~ 32 / ~~M~~ 25 / ~~L~~ 2），闭合词 **5 个**（已收口 32 / 已修 18 / 已关闭 13 / 关闭 1 / 半修 1）。`半修` 进词汇表而不进收口计数，那一格是 A156，与 l242 的 `PARTIAL` 名册对上——两把尺子在同一只 ID 上互相印证。换新写法（「已了结 + 括号 + 轮号」）当场红，臂 A1 用改写现量的一格证明清单不是恒真。② **轮号反向可追溯**：64 格闭合标记逐格都要从括号里读出轮号（`ROUNDS` 允许「括号与轮号之间有限定词」这一形），且读出的轮号必须能在某一卷的进度声明面查到——现量同卷可查 **52**、跨卷 **12**，12 行逐格点名（`CROSS_ROSTER`），两个方向同一把尺子：跨卷而不写明红（臂 A4 把「第二本账」摘掉 ⇒ 当场红），写明跨卷而那一轮本卷也查得到也红（`test_no_cell_says_the_other_volume_when_it_is_this_ones_own`）。**立账时那五条现量按本轮实测作废并改写**：有标记的行不是 132 而是「整行 108 / 定级格 64」两面（差 **44** 行，另立 **A337**）；严格式解析不出不是 10 而是 **0**——那 10 行的限定词形状由 `ROUNDS` 的「括号内任意位置」读法收进来了，代价是判据从「紧跟括号的轮号」松成「括号内第一个轮号」，所以同批补了反面判据（括号里没有轮号 = 不可追溯，臂 A2 点名）；本卷查不到而需跨卷的是 **12** 行不是 3 行（立账那轮的 A46 / A130 / A138 只数了当时看得见的三行，A130 那一行其实连轮号都写在正文里而定级格是裸字母，正是 A337 的那一族）；形状不是 7 种而是四件结构件的 16 种组合。**另有一处真缺陷本轮收掉**：卷一 A31 那一格写作划掉字母而整格没有任何轮号，反向不可追溯在它身上是现成的盲区（划掉说明「已收」，却查不到哪一轮收的）——按声明面现量补成划掉字母加 L29 那一轮（`- [x] **L29**` 那一条写着 A31 的回显口径补齐），此后「有标记而无轮号」的名册是空集，A31 由臂 A2 那一支的对照组变成可读的现量。定级 ~~M~~ 已收口（L253）。 | ~~M~~ 已收口（L253） |

| A329 | CLI 的 `backup --action` 只认 4 档，而 HTTP 面这一族已有六只端点 | **新立（L243）**：实测 `augmentor/cli/parser.py:394` 的 `choices` 现量是 `["create", "restore", "list", "delete"]`，而 L243 给契约面新加的两只（`GET /api/system/backups/verify`、`POST /api/system/backups/cleanup`）在 CLI 上够不着 ⇒ 三面口径再次分叉（SDK 有函数、HTTP 有端点、CLI 没有档）。**为什么值得立账**：「清理旧备份」正是防目录无限增长的日常操作，而日常自动化（脚本、cron、任务计划）走的通常是 CLI 而不是页面；只有页面能点，等于把这一族里最该无人值守的那一半留给了手工。**当前处置**：守卫按现量钉住 CLI 的 choices 恰为那 4 档（多一档少一档都红），页面卡片与 `docs/augmentor/API.md` 各写明「CLI 没有这两档」。**修法方向**：① 给 CLI 补 `verify` / `cleanup` 两档并带上 `--max-backups`；② 或明确记档 CLI 只覆盖四档、清理一律走 SDK。**L244 已收口（走修法①，原立 L243）**：`augmentor/cli/parser.py` 的 choices 现量六档，另带 `--max-backups`（默认 **10**，与 HTTP 请求模型 `BackupCleanupRequest`、SDK 形参 `clean_old_backups` 三方同数，守卫逐值比）。档位口径与端点同源：`verify` 是校验形命令 ⇒ 查到损坏经 `verdict_exit` 给退出码 **1**，且 stderr **没有**「错误:」（判决与崩溃必须分得开，`cli.py` 把 handler 异常翻译成「错误: {e}」+1），完好 ⇒ 0、空索引或幽灵名 ⇒ 「没有找到备份」+ **0**（与 HTTP 面「幽灵 id 回空列表」同档，既不判坏也不报错）、零字节 ⇒ 单列「文件丢失或为空」；`cleanup` 的 0 与负数由 `require_count` 抛、经 `cli.py` 收尾成「错误: max_backups 必须是不小于 1 的整数，当前是 0」+ **1**，且**判参排在构造之前**（实测目录不在场，与 HTTP 面「0/-1 ⇒ 400」同档不同传输）；不认识的 `--action` 由 argparse 给 **2**。新守卫 `test_cli_backup_actions_l244.py` 20 支把**三个方向**各自钉死：choices ↔ `run_backup` 的分支集合（AST + 文本双查）、choices ↔ `ACTION_TO_ENDPOINT` ↔ OpenAPI 里备份族全部操作集（**双射**，契约面多一条或少一格都红）、每档与对应端点调用**同名**的 SDK 入口（两侧都读 import 名单）；另钉 `--help` 里那句退出码宣称（`verdict.py` 口径：接了却不宣称 = 文档撒谎）。**三面同源判据为什么不写成清单**：清单（「CLI 有这六档」）只防得住「CLI 少一档」，防不住「CLI 多一档而没人接」与「HTTP 加了第 7 员而 CLI 不知情」——后两条正是本条的成因；注入取证把这一条量成了现量：摘 choices 里的 `verify`（臂 A）红的是名册两支，而「分支 ↔ choices」那一支要摘 handler 分支名（臂 B）才咬得到，两个方向的牙齿不一样。**订正立账时那格「当前处置」**：它写着「守卫按现量钉住 CLI 的 choices 恰为那 4 档」与「页面卡片与 `docs/augmentor/API.md` 各写明『CLI 没有这两档』」，本轮开工前 grep 现量**两格都不成立**——全仓没有任何守卫钉 CLI 的 choices（四档那阵子也一样），页面卡片也没有那句话（只有本账自己这么写着）。这与 A331 同族：**账上写的处置与量到的处置不是一件事**，落笔时把「打算做的守卫」当成了「已就位的守卫」。本轮补的是真守卫并把它钉成六档，不追认旧话。同轮量到一处真实的三面不对称（CLI 的只读档会把不存在的目录建出来并落 `index.json`，HTTP 面同一只 verify 不建）⇒ 另立 **A332**。定级 S ⇒ 已收口。 | ~~S~~ 已收口（L244） |

| A330 | 备份一族六员的契约面只声明 `200` 与 `422`，409 / 401 / 403 / 404 全在 OpenAPI 之外 | **新立（L243）**：实测 `app.openapi()["paths"]` 里 `/api/system/backups/verify` 与 `/api/system/backups/cleanup` 的 responses 键集现量都是 `{200, 422}`，同族的创建 / 恢复 / 列表 / 删除四员也一样 ⇒ L243 新立的那一档 **409**（坏备份被拦下、一个字节都不写）只在代码与文档里，契约上不承认。**为什么值得立账**：只吃契约的客户端（代码生成器、契约测试、第三方集成）无从知道会拿 409，也就不会为「拒绝且零写入」这一档写处理分支；401（缺 key）、403（路径出白名单）、404（幽灵 id）同病，且这三档早在 L231 / L233 就实测过。**当前处置**：`test_backup_integrity_l243.py::TestTheContractDeclaresOnlyTwoCodes` 把「契约只声明两只」钉成**现状事实**而不是承诺，档位口径写在 `docs/augmentor/API.md` 的逐只提示格里。**修法方向**：① 这六只补 `responses={401, 403, 404, 409}` 的说明；② 或在全仓范围给所有写端点统一补错误档，代价是 `test_api_openapi_contract` 那把键集尺子要同轮重量一遍（面大，先拍范围）。**L252 已收口（走修法①，原立 L243）**：六员各补 `responses=`，只写 29 个真请求实测到的档——list 与 verify 各 `400 / 403`，create 与 delete `400 / 401 / 403 / 404`，restore 另加 `409`（全家族独此一档），cleanup `400 / 401 / 403` ⇒ 六员合计 **20 格**，没实测到的码一格不写；错误体统一声明成 `{detail: string}`，因为 `HTTPException` 出去的永远是那一句。判据写成**集合相等**（声明的错误档 == 实测的错误档）而不是「补齐了就行」：少一格是客户端踩坑，多一格是客户端去处理永远不会来的分支，两个方向同一把尺子；「实测」那一侧不是手抄清单，而是把 29 个请求真打出去、每臂必须命中它挂在的那一档。新守卫 `test_backup_contract_error_codes_l252.py` **31 支**，四臂零写入注入取证分别咬住「摘掉 409 / 给只读那员虚声明 401 / 把说明写成空话 / 剪掉一条实测臂」，基线 31 支全绿。**订正立账时那格「当前处置」**：它说 `test_backup_integrity_l243.py::TestTheContractDeclaresOnlyTwoCodes` 把「契约只声明两只」钉成现状事实——那只类名本轮按新事实改名 `TestTheContractAdmitsTheRuntimeVerdicts`、期望集换成实测集（这一格量的是**当时**的现状，不是承诺，改名而不是删掉正是为了留下这条边）；而「档位口径写在 `docs/augmentor/API.md` 的逐只提示格里」那半句在 L252 之前**只覆盖新建的两员**，其余四员的格里没有错误档口径，本轮补齐七行（六员 + 错误码表 404 那行）。同场量到一处**反方向**缺口 ⇒ 另立 **A336**（不带请求体的三员，`422` 由框架自动声明却构造不出任何 422 请求）。修法② 那半（全仓统一补错误档）仍未办：全仓现量 **77 员 = 6 + 58 + 13**，其余 58 员仍只有 `{200, 422}`。定级 ~~M~~ 已收口（L252）。 | ~~M~~ 已收口（L252） |

| A331 | 行号引用尺子的 `dead_line` 档只判「落点在不在代码跨度里」，量不到「落点在代码块内而内容与引用所要说明的东西无关」 | **新立（L243）**：L243 开工前账本 L232 那一格写着 `docs/augmentor/API.md:952` 并称「引用仍指得准」，而 952 行在 HEAD 上落进 quality 的一段 JSON 示例里，本轮要指的 diff 请求示例其实在 1139 行附近 ⇒ 尺子全程绿、那句话是假话。本轮把这一处连同另两处（`System.tsx` 的两格）一起按现量重钉，重钉后 a184 与 `test_doc_line_refs_l79` 两把都过。**为什么值得立账**：`file:NNN` 是账本「可复核」的唯一机械通道，落点对、内容错比落点错更危险——落点错会响，内容错只会让后来人以为已经复核过了。**当前处置**：三处引用已按现量钉到真实落点（逐字读过落点那一行），本条记的是尺子的深度缺陷，不是这三处引用本身。**修法方向**：① 给 `dead_line` 加内容维，要求引用旁点到的符号 / 键名出现在落点 ±N 行的窗口内；② 或把「引用内容对不对」做成逐格点名的复核清单（人力维，会随账本变长而失效）。**L245 已收口（走修法①）**：`test_comment_refs_a184.py` 的 `dead_line` 之后接了内容维 `anchor_state`——落点是代码 ⇒ 取引用旁紧跟的后括弧或所在的不含嵌套外括弧，括弧**第一个**反引号跨度必须是裸标识符、必须是目标文件某个 `def` / `class` 的名字，且落点跨度完整落在该符号某一跨度内，否则开火新档 `dead_anchor`（硬 0）。**为什么不做整行或句内归因**：三档现量比对（整行要背 3 229 个标识符、产品注释面「全部命中」只有 15/48；句内造出 6 条假红）⇒ 窄到「引用自己声明的符号」才咬得住又不撒谎。立档时案面现量 **12 条红**，逐条按 AST 现量重钉、行内换数字净 0 行；本轮另人眼补 3 处尺子判不到的假句。**判不到的三种形状**（点号主张、括弧首是散文、不带文件名的裸行号）不硬判，另立 **A333**。定级 ~~M~~ 已收口。 | ~~M~~ 已收口（L245） |

| A332 | CLI 的 `backup --action verify` 会把不存在的备份目录**建出来**，HTTP 面同一只 verify 不会 | **新立（L244）**：实测 CLI 侧 `--backup-dir` 指向不存在的目录 ⇒ 退出码 0、打印「没有找到备份」，磁盘上却多了一个目录和一份 `index.json`；HTTP 侧同一件事（`GET /api/system/backups/verify` 给白名单内但不存在的目录）回 200 空结果且**不建目录**——路由里有一道 `if not resolved.exists()` 挡在构造之前，CLI 没有那道挡，而 `DatasetBackup` 的构造无条件 `mkdir`。既有的 `--action list` 一直就是这个形状（同一只构造），所以本轮不是新引入的缺陷，而是**三面同源补齐之后才量到的不对称**。**为什么值得立账**：只读操作留下磁盘副作用，这一族已经伤过两次（A307 的依赖列表与依赖图），而且同一句「查一下有没有坏的」在 CLI 与 HTTP 两面留下**不同的磁盘状态**——脚本里跑一次、页面上跑一次，目录是否存在这件事的答案就取决于先跑了哪面。**当前处置**：`test_cli_backup_actions_l244.py::TestCliVerifyVerdict::test_a_missing_dir_is_created_and_reports_no_backup` 把**现量**钉成「建目录 + 目录里只有 `index.json`」，哪天有人把两面统一掉，这一支要响并同步改本条（不是让它静默通过）。**修法方向**：① 给 SDK 加一只不 mkdir 的只读构造，三面同源（代价：`DatasetBackup` 的构造语义要分两种，A307 那一族同批受益）；② 或在 CLI 的 `verify` / `list` 之前加与 HTTP 同形的 exists 挡板（最小面，但写档与读档的落盘面仍不一致）；③ 或维持现状，把「只读也建目录」写进 `docs/augmentor/API.md` 与 CLI 帮助，让脚本作者知道它会留东西。**L249 已收口（走修法①，与 A307 同根同批，且多补一刀）**：`DatasetBackup.__init__` 的 `mkdir` 摘掉，`_load_index` 索引不在场时交回内存里的 `{"backups": []}`，不再顺手落一份默认 `index.json`。**为什么两刀而不是一刀**：只摘 `mkdir`，本条的现量「多一个目录**和一份 `index.json`**」只少一半——loader 自己会写默认账，目录照样被 `atomic_write_json` 的父目录创建建出来。首份索引延后到 `_save_index`，那一只仍走 `atomic_write_json`（父目录由它自建）⇒ L162 的原子写判据不松动，写边由两条收成一条而读边现在是零写。三面现量（改后同探针）：CLI `--action list` / `verify`、HTTP 四只 `GET` 与默认档、SDK 的 `list_backups` / `verify_backup_integrity` / 无债档 `clean_old_backups` 全部零残留；L243 担心的「写端点不建目录 ⇒ 由 200 变 500」实测否掉（cleanup 无债仍回 **200** `{deleted_count: 0, max_backups: 5}`）。**判据**：同 A307 的新守卫（注入臂 b1 11 红、b2 10 红），加上原地反转的 `test_cli_backup_actions_l244.py`（改名 `test_a_missing_dir_stays_absent_and_reports_no_backup`，断言换成「目录不在场且 `list(tmp_path.iterdir()) == []`」）、`test_backup_integrity_l243.py` 的缺目录档、`test_backup.py::TestDatasetBackup::test_init`，以及 `test_backup_index_atomic_l162.py` 那一支静态尺子——旧写法比的是文本包含，删掉真调用它照样绿（注释里提一句名字就能喂饱），本轮改成走 AST 只数调用点。L44「判参先于副作用」的证人是「坏参数会留下目录」，本轮证人退役 ⇒ 口径改由 AST 比 `require_count` 与 `DatasetBackup` 的行号先后钉住（注入臂 o1 只有这一支红，行为那一支变绿正是证人退役的现量）。定级 ~~S~~ 已收口（L249）。 | ~~S~~ 已收口（L249） |

| A333 | a184 的内容维只认「括弧首是裸标识符」这一形，点号主张、括弧首是散文、不带文件名的裸行号三种形状结构性判不到 | **新立（L245）**：L245 给 `dead_line` 加内容维时把判据收成括弧首（理由与三档现量见 A331 那一格），代价是三种形状落在尺子外：① 点号主张，即写成「文件:名字」而不是「文件:行号」的那种；② 括弧里第一个反引号跨度是散文而不是符号名；③ 不带文件名的裸行号。**为什么值得立账**：这三形不是稀有形状——卷一 A95 那一格同时用了 ① 与 ③，本轮人眼核对现量**三个数字全部陈旧**：`_resolve_env` 的 def 现在 `augmentor/config.py:864`、未设置时回落空串那一行在 886、而当年的三处调用点已并成 `for` 循环里的**一处** 949，账上写的还是 345 / 356 / 434-436，尺子一声不响。A331 修好的是「落点对、内容错」里能被符号名点到的那一半，剩下这一半没有任何机械通道。**当前处置**：`TestContentDimension.test_the_two_excluded_shapes_are_not_claims_at_all` 把「判不到」钉成现量（单独喂那两支 ⇒ `anchor_claims` 增量 0，证明不是碰巧也绿）；A95 那三个数字本轮**只核对未重钉**——改它要连整句散文一起重写，且改完仍没有尺子能验。**修法方向**：① 认点号主张（把「文件:名字」钉成该名字必须是符号且行号落在其跨度内）；② 裸行号绑到同一行最近的文件名；③ 或把这三种形状做成逐格点名的复核清单（承 A331 修法② 的同一代价：名册随账本变长而失效）。**当前处置（L268）**：③ 那一形由新守卫 `tests/unit/test_self_line_claim_l268.py` 落成逐格点名名册——两卷现量 13 处「本账第 + 数字 + 行」站点全部在卷二，按同物理行有没有 `文件名:行号` 锚、且主张那一行读不读得到那个锚，分 `LIVE`（恰好 1 格，就是 L232 清单行那句 operative）/ `STALE`（5 格，历轮当时的现量，按纪律不追改）/ `BLIND`（7 格，那一行根本没有锚）三档闭合；operative 另有一支**按内容独立定位**（全卷同时含 `DiffResult` 与那条 API 文档引用的物理行现量唯一）复算它应当写的行号，所以插行没同步重钉它就当场红。名册键用 `(卷, 区段, 轮号, 主张的那一行)` 而不是物理行号，代价与立账时写的一致：随账本变长每轮要更新。**① 点号主张与 ② 括弧首是散文两形本轮未动、仍在案** ⇒ 移交新立 A346。 | ~~M~~ 已收口（L268） |

| A334 | `QualityScorer._embedding_cache` 是实例级、跨 `batch_score` 终身累积的整批向量缓存，`reset_cache()` 一格不清 ⇒ 长流水线里每批的矩阵连同它的全文键元组都被扣住 | **新立（L246）**：收 A327 时顺带量的现量（4 批 × 1 000 条、`include_semantic=True`、dim 64）：跑完之后 `_embedding_cache` **8 个条目**（每批 gen + orig 各一）、矩阵 `nbytes` 合计 **2 048 000**，键元组另持 **8 000** 个文本引用；同一形状折算真实语义模型 dim 384 是 **12 MB**，而 `augmentor/pipeline.py:110` 与 `augmentor/comparison.py:109` 把 scorer 挂在**实例**上 ⇒ 一次流水线内逐批累积、永不释放。本轮 `test_reset_cache_clears_norms_along_with_embeddings` 的读数就是这条边界的现量：`reset_cache()` 之后第二趟只重编码参照集（编码次数 `[30, 4, 4]`），整批生成向量仍然命中这份缓存。**为什么值得立账**：`reset_cache()` 的名字是「重置缓存」，读名字的人有理由以为它清得干净，而它清的是参照集那一族四格；A327 原格点名的「`_embedding_cache` 与 `_existing_embeddings` 的寿命」这条边界本轮在测试里说清楚了，但**寿命的另一半——有没有上限——仍然没人管**。**修法方向**：① 有界化（只留最近一批，或 LRU 上限），代价是命中率下降 ⇒ 编码次数上升，属「数值不变而次数变」那一族，要独立的行为差取证；② 或把 `reset_cache()` 定义成清全部缓存面，并加守卫点名「清了哪几格」；③ 或在 pipeline / comparison 每批之后重建 scorer（改调用方，产品代码最省，但把义务外推）。**为什么 L246 不动手**：本轮的收口口径是「产物逐字节相同」，而有界化必然改变编码次数与调用形状；且 `quality.py` 的目标是净零行，插不进一条淘汰策略。**L247 已收口（走修法 ①「有界化」，代价按原格预告独立取证）**：`cached_encode` 前加一条**上限 2 的 FIFO**（`while len(self._embedding_cache) >= 2` 逐出**最旧**那个键），上限 2 与「一次 `batch_score` 最多插 orig/gen 两个键」是同一件事的两面 ⇒ 语义即「只留最近一批」；`reset_cache()` 的语义一格未动（A327 那条「`_embedding_cache` 跨调用、`_existing_embeddings` 随参照切片」的寿命边界原样保留），调用方（pipeline / comparison / benchmark / API）也一字未改。**为什么不走 ②/③**：③ 由四臂注入的 ㈢ 现场否决——把作用域收成 per-call 会打掉既有 `test_embedding_cache_exists_after_score`、l246 守卫 `[30, 4, 4]` 那一格与本轮的同批命中支共 **3** 支，跨调用命中是本仓**已声明**的行为，不是可静默撤销的实现细节；② 会把「清不清整批缓存」变成调用方的义务而产品侧一个也没有。四档现量（① 8 条目 / 2 048 000 → **2 / 512 000**、编码次数 128 不变；② 同批连评两次形状**驻留与编码次数全不变**；③ a_b_a_b 交错 124 → 128 是本轮唯一代价，产品侧无此形状；④ comparison 形状 64 不变）、位相同口径（四档判决产物 `repr` 逐字节相同，sha256 前后同为 `6f405bf0099ef1e9…`）与 7 支新守卫（驻留三格读数 + 编码次数，两条独立判据）见循环日志 L247。定级 ~~M~~ 已收口。 | ~~M~~ 已收口（L247） |

| A335 | `POST /api/system/backups` 的回执七键里没有 `name`，调用方分不清「自己起的名字」与「服务端生成的时间戳 id」 | **新立（L250）**：从 A303 拆出来的一笔（原格修法②，本轮未做）。现量：`BackupInfo.to_dict()` 是七键 `{backup_id, timestamp, source_path, backup_path, item_count, file_size, checksum}`，而 `backup_id = name or f"backup_{时间戳}"` ⇒ 给过 `name` 之后回执与匿名备份的 id 只在**形状**上可辨（有无 `backup_` 前缀），语义上不可判，调用方要靠猜。**为什么单独立账而不与 A303 同格**：这一笔要动的是响应模型（加一格），会同时牵动 `test_api_openapi_contract` 那把键集尺子与 `web/src/types/api.ts` 的 `BackupInfo`（L237 建的 TS ↔ 契约两层跨语言对账），而 A303 的主张（一次读钟）一项都不碰——捆在同一格里收口，会让「收了 ①」被读成「整条清了」。**相邻的一格**：A298 本轮按 `backup_id` 摘旧行而不按 `name`，正是因为索引行里没有 `name` 可查；补上这一格之后那处写点才有第二种键可用。**当前处置**：守卫与文档只写「回执七键、`name` 不在其中」，页面「数据备份」卡的名称列继续显示 `backup_id`（同名备份现在恒为一行，见 A298 于 L250 的改写）。**修法方向**：① 回执补 `name` 那一格（键集尺子与 TS 类型同批重钉，`test_ts_contract_types_l237.py` 一族随之对账）；② 或把 `backup_id` 与 `name` 拆成两格并让 id 真唯一（与 A298 未做的 ② 同族，是行为变更）；③ 维持现状，靠 `backup_` 前缀猜。定级 S，待拍。 | S |

| A336 | 不带请求体的三只备份端点仍被框架自动声明 `422`，而这一档在它们身上构造不出任何请求 | **新立（L252）**：收 A330 时同场量到的**反方向**缺口。现量：`422` 不是本轮补的，是 FastAPI 见「操作有参数」就自动挂上去的，六员全都有；但 list、`GET …/verify`、`DELETE …/{backup_id}` 三只的参数只有字符串查询参数 `backup_dir` 与路径参数，**任何值都合法** ⇒ 探针的六档尝试（空串、白名单外绝对路径、重复给值两种顺序、多余键、幽灵 id）实测 `[400, 200, 403, 200, 404, 404]`，**无一档落到 422**。带请求体的另三员（create / restore / cleanup）打得出来，且 `detail` 是框架的英文字段错误列表——与本轮声明的错误体的 `detail` 字符串**两种形状**，文档里都要认。**为什么值得立账**：A330 收的是「有判决而契约不写」，这一条是反面——契约写了一档而它永远不来；只吃契约的客户端会为它写一条死分支，代码生成器会真生出那只永不命中的 handler。**当前处置**：`tests/unit/test_backup_contract_error_codes_l252.py::TestTheAutoFortyTwoTwoIsHonestOnlyOnThreeEntries` 把两面各自钉死（带体的三员逐只验 422 打得出来且 `detail` 是列表；不带体的三员按六档实测码逐格点名并断言 `422 not in codes`），**没有**为了让尺子好看而把 422 从契约里抹掉。**修法方向**：① 给这三员显式覆盖 `422` 那一档的说明（只改说明不改可达性，代价最小，且不改公开契约的键集）；② 或先量清 FastAPI 能否整档摘掉自动 422 再摘（要钉版本行为，与 A330 修法② 的全仓统一是同一次活）；③ 维持现状，把这格不对称写进 `docs/augmentor/API.md` 的 422 那一行。定级 S，待拍。 | S |

| A337 | Backlog 表有两处「哪一格才算这行的处置结论」没有对账：定级格停在裸字母，而带轮号的闭合语写在正文里；以及表行面把日志块里的同名表格也算成欠账 | **新立（L253）**：收 A328 时把标记判据从整行收到定级格，同场量到两面之差——Backlog 区 211 行里整行带闭合标记的 **108** 行、定级格带闭合标记的只有 **64** 行，差 **44** 行（名册见 `tests/unit/test_backlog_mark_vocab_l253.py` 的 `TWO_FACE_ROSTER`）。这 44 行按声明面分两档：**36 行**的那只 ID 进过进度声明面（A130 / A270 / A275 / A277 / A302 / A312 与 B201–B225 那一族：正文第一句就是带轮号的闭合语，而定级格还写着裸字母 ⇒ 账上「已收」与格子「待拍」同时成立）；**8 行**没进过（A21 / A201–A206 / B266），它们的正文闭合语要么只覆盖本行的一半（A21 那句说的是 ngram 那一半，contains / regex 两格本行自己写着仍待办），要么属于别的 id（B266 那句只点了 StreamWriter 一处）。同场量到第三件事：l242 认表行只看行首，于是**日志块里的表格也算 Backlog 行**——现量 **46** 行（A1–A8 是注入臂名册，B226–B263 只在日志块里出现过），两卷合计 257 行表行里有 46 行不在 Backlog 区。**为什么值得立账**：l242 那条「声明面说收了 ⇒ 表行有标记」走的是宽面，而宽面会被正文喂饱 ⇒ 上面那 36 行**永远不会红**，尺子看着严其实松；反过来，日志块那 46 行让「表行数」这一格读数的含义不唯一。**当前处置**：新守卫把定级格当权威面，两面之差逐格点名（44 行名册精确相等 ⇒ 谁补一格这一格要缩），区段边界按 `## 循环日志` 钉住（211 / 46 两格现量）。本轮**不批量补标**——36 行的补标要逐行核对「正文那句闭合语说的确实是本行整行」，那是人眼核对而不是机械活，且每行都要为它挑一个轮号。**修法方向**：① 逐行核对后把 36 行的定级格收成划掉字母加轮号（名册随之缩到 8，剩下 8 行留作「正文引用别物或只覆盖一半」的现量档）；② 或把 l242 的标记判据也从整行收到定级格（一改就把那 36 行现成打成欠账，当场红出 30+ 支，必须与 ① 同批做）；③ 给 l242 的认表行加区段过滤（日志块那 46 行不算表行），并把它的三格读数按现量重钉。**L254 已收口（走原格修法③，① 与 ② 两半原样拆给新立 A338）**：l242 的认表行加了区段过滤——`## 循环日志` 之下的行首 ID 不再算 Backlog 表行，被请出去的正是那 46 行（A1–A8 注入臂名册 + B226–B263 某一轮收口清单），于是「表行数」这一格读数从此含义唯一（212）。**这一改顺带现形出一族此前根本看不见的账**：B226–B263 那 38 只从未有过 Backlog 行的 ID，此前在「进度声明面说收了 ⇒ 表行有标记」这一判据里**同时充当被核对的账与自己那条证据**——同一张日志块里的表格既是声明又是账，判据在它身上是自我循环的；过滤之后它们改归 phantom 那一族，而该守卫把口径 2 那一格从 46 拆成「历史裁撤 46 + 日志名册 38」两格、各有一支逐格点名。**两把尺子在同一只面上对上数**：l242 过滤后的「有标记」名册与 l253 的「整行带闭合标记的 Backlog 行」是同一集合（本轮 109），l253 那把尺子独立把 46 量成日志块表行数 ⇒ 同面同数由 l242 的 `test_the_other_ruler_counts_the_same_wide_face` 钉成判据，而「过滤前后 gap 都是 PARTIAL 三格」由 `test_the_gap_is_the_same_on_both_faces` 钉住——这次改的是**面的定义**，不是判据的松紧。**未做的两半**：① 那 36 行的补标要逐行人核对「正文那句闭合语说的确实是本行整行」（A21 那句只管 ngram 一半、B2 那句写着「前半关闭」、另 8 行引用的是别只 ID 的账）；② 把 l242 的标记判据也从整行收到定级格——一改就当场红出三十多支，必须与 ① 同批做。两半合成新行 **A338**。定级 ~~S~~ 已收口（L254）。 | ~~S~~ 已收口（L254） |

| A338 | A337 剩下的两半：36 行「正文里写着带轮号的闭合语而定级格还是裸字母」要逐行人核对才敢补标，而 l242 的标记判据本身仍按整行（宽面）而不是定级格 | **新立（L254）**：收 A337 走的是原格修法③（区段过滤），① 与 ② 原样带到这一行。现量在 l253 那把尺子里：Backlog 区 212 行中整行带闭合标记 **109** 行、定级格带的只有 **65** 行，差 **44** 行的名册钉在 `TWO_FACE_ROSTER`（集合精确相等 ⇒ 谁补一格这一格就要缩）。其中 **36 行**的那只 ID 进过进度声明面（A130 / A270 / A275 / A277 / A302 / A312 与 B201–B225 那一族：正文第一句就是带轮号的闭合语，而定级格还写着裸字母 ⇒ 账上「已收」与格子「待拍」同时成立，l242 走宽面所以这一族**永远不会红**）；另 **8 行**没进过声明面（A21 / A201–A206 / B266），它们的正文闭合语要么只覆盖本行的一半，要么属于别的 ID。**为什么本轮不做**：① 是人眼核对而不是机械活——A21 那句说的是 ngram 那一半、B2 那句写着「前半关闭」，把 36 行一律补成已收会把没还的债划掉；② 单独做会让 l242 当场红出三十多支欠账，而那些欠账里有真的没收的，必须与 ① 同批。**修法方向**：① 逐行读那 36 行的正文，确认本行整行确实收了才补定级格，其余留裸字母并把理由写进正文；② 随后把 l242 的标记判据收到定级格，并与 l253 的44 行名册对齐；③ 与本题同族但不是一件事——「本账第 NNN 行」那一形没有尺子量（**A333**）。 **L261 走原格 ①+② 同批**：逐行核对 44 行两格差之后的处置是 **41 行补标定级格、3 行留裸字母**。补标那 41 行里 35 行是本格点名的那一批，另外 6 行（`A201` 至 `A206`）本行原写「8 行没进过声明面（A21 / A201–A206 / B266）」并把它们一并归因成「只覆盖本行的一半或属于别的 id」——本轮逐行读那六行正文，第一句就是本行整行的判决（`A201`「16 条缺数 + 3 条隐藏偏支弧全清」、`A206`「10 页各建组件测试，前端全量 13 文件 112 例全绿」），归因错了，故连那六行一起补标，订正见循环日志 ③。留裸字母的三行各有专格点名：`A21` 那句只管 ngram 那一半（本行自己写着 contains / regex 两格待办）、`B2` 那句写着前半且剩下的在 **A132**、`B266` 那句写着部分关闭且其余仍在案。判据侧同批：l242 的 `marked()` 改读定级格，`PARTIAL` 名册从三格走到四格（新增 `B2`），`TWO_FACE_ROSTER` 从 44 格缩到 3 格。定级 ~~S~~ 已收口（L261）。 | ~~S~~ 已收口（L261） |

| A339 | `DatasetAuditor` 的 `fields` 仍走 fields-or-默认 那一形，显式空列表被读成「没传」而套上三格——同一族 falsy-zero 陷阱在 privacy 侧已于 L188 改掉，audit 侧漏了 | **新立（L257）**：L257 收 A285 把两处默认面并成一份权威时，同场量到两侧对**空容器**的判据不同：PiiSanitizer 用「fields is not None 才用传入值」（L188 的原话是显式空容器是零元素的合法意图，不是没传），DatasetAuditor 用 or。实测 `PiiSanitizer(fields=[]).fields == []`，而 `DatasetAuditor(fields=[]).fields` 等于那三格 ⇒ 想「一格都不看」的调用方在审计面被换成「三格全看」。**为什么值得立账**：这一格决定的是 PII 命中与空值统计的**范围**，放大范围不是无害默认——报告会据三格给出「这几处有空值」的 finding，而调用方本意是谁都不看；且它是 l257 那把新守卫里唯一「钉着现状不放手」的一格（`test_the_empty_list_is_honest_on_one_face_and_swallowed_on_the_other`），有人顺手改掉就会撞上那条断言，红得有名字，但账不在本行。**当前处置**：l257 把两侧行为逐面钉成现量，本轮**未动产品行为**，audit.py 那一行只把字面量换成 list(权威)、or 形原样保留。**修法方向**：① audit 侧收成 is-not-None 形（与 L188 同源，代价是空值统计面变窄、AuditReport 的字段计数随之变小，须与 l228 的 stats 面同批核对）；② 维持现状，把「空列表按默认三格处理」写进 docstring 并让页面把这句话说出来；③ 两侧同批改走 require_string_list 那一族的显式判决。定级 S，待拍。 | ~~S~~ 已收口（L277） |

| A340 | `Exporter.export_batch` / `export_all_formats` 的默认并行支路把任务异常咽成 `logger.error`：同一份输入串行走是 `DataValidationError`，并行走是一片空 dict，而 `POST /api/export/batch` 走的正是并行档 ⇒ 被拦下的越界名称照样回 `success: true` | **新立（L258）**：L258 给「名字被当成路径片段用」那一族装权威时量到的第二格。写边（`export.py` 的两处 `require_within_dir`）拦住了，回执却看不见那次拦截——并行分支的 `except Exception: logger.error(...)` 把子任务异常咽掉，返回 `{name: {}}`，而路由把「有返回」当成「有结果」。实测 `Exporter().export_batch({"../../silent/x": items}, output_dir=<out>)` 并行档得到 `{"../../silent/x": {}}`，且目录外**零产物** ⇒ 危害面是**假回执**而不是越界写。**为什么值得立账**：`/api/dataset/split` 上同一条判决经 `to_http_error` 已经是 400，所以这一族里只有导出面在骗客户端；「失败冒充成功」的账不能只记在产品侧，调用方据此重试与不重试的决策全部反过来。**为什么本轮不动**：改它要动 `export_batch` / `export_all_formats` 的**返回与抛出语义**（现况连「部分失败」也是 `success: true`），那是对外契约变更；且修好之后该路由的失败会打到 `raise_internal_error` 那一档 ⇒ 状态码是 500，与 **A280**（一刀切收敛状态码，仍待拍）必须同批处置，否则本轮就等于顺手改掉一格没人拍过的契约。**当前处置**：守卫把这一形按实测**钉成已知限制**（`test_the_parallel_arm_swallows_the_verdict_and_reports_success`），三处拒绝臂显式带 `use_parallel=False`——钉住现状而不是假装它合法。**修法方向**：① 并行支路把每个任务的异常收进返回值（`{name: {"error": ...}}`）或整批 raise，路由随之回 400；② 与 A280 同批把门口判决与状态码一次性立清；③ 最小一步：`export_batch` 在有任何子任务失败时回执带 `failed` 名单，页面把这格说出来。定级 S，待拍。 | S |

| A341 | `POST /api/config` 能把 `vector.storage_dir` 这一格目录键写成数据根白名单之外的路径而照单回 200——同一份配置里数据文件那一族走 `api/deps.resolve_within_roots`，两条入口对「路径」的口径不一致 | **新立（L259）**：L259 给「名字拼进路径」那一族接权威时，同场量到**根**这一格不在那一族的射程内——`vector.storage_dir` 是 `WRITABLE_SECTIONS`（augmentation / quality / dedup / export / vector / rag / multimodal 七节）里唯一带目录的键，配置侧只有 `augmentor/config.py:617` 的 `require_string("vector.storage_dir", ...)`，判的是「非空字符串」，不是「落在哪」。**实测**（沙箱副本，仓库 `config.yaml` 的 sha256 事后复核未变）：`POST /api/config` 带 `{"vector": {"storage_dir": "../../outside_vectors"}}` 回 **200**、`ignored_keys: []`，沙箱副本里存进去的就是那串相对路径；读侧 `index_status` / `persist_index` 都按 `Path(storage_dir)` 直接用，相对路径按**服务进程 cwd** 解析，与 `AUGMENTOR_DATA_ROOTS` 无干涉。写边也真通：`augment_dataset` 在 `if self.config.vector.enabled` 那一支调 `build_vector_index` ⇒ 接一次 `POST /api/augment` 就能把索引建到仓库与数据根之外。**放大器**：出厂默认不设 `AUGMENTOR_API_KEY`，`api/main` 启动时把「写操作当前无鉴权」写成告警而不拒绝 ⇒ 这两步都可匿名完成。**为什么值得立账**：白名单那一族的威胁模型是「客户端给的路径不能逃出授权目录」，配置面是同一批客户端可写的另一个入口，两条口径不一致时白名单只是把绕行挪到更隐蔽的一格。**为什么本轮不动**：改它要动 `POST /api/config` 的回执语义（此刻这七节的目录键原样写入），200 变 400 是对外契约变更，且与 **A280**（状态码一刀切收敛，仍待拍）同族 ⇒ 等拍板。**当前处置**：守卫把这一形按实测钉成**已知限制**（`TestTheConfigSurfaceStaysOpen` 三支：配置面此刻不走白名单这一事实，不是它合法）。**修法方向**：① 把 `vector.storage_dir` 接进 `resolve_within_roots` 同口径，越界回 400；② 维持可指任意目录，但在 `GET /api/config` 回执与页面上把这句话说出来（配置即权限，需明文承认）；③ 把带目录的键请出 `WRITABLE_SECTIONS`，改它需重启。定级 M，待拍。 | ~~M~~ 已收口（L277） |

| A342 | `visualizer._output_path` 交回的是**未规范化的拼接**（`self.output_dir / output_file`），而同轮的 `pipeline.export_dataset` 与 `version_control` 交回的是 `require_within_dir` 解析之后的落点——同族两个返回值形状不一致 | **新立（L259）**：L259 接 `visualizer.py` 时面前有一个二选一：判决用权威，返回值改不改？本轮选了**只借那句判决、返回原形状**，理由是响应面零漂移优先——`_output_path` 的结果经 `generate_all_visualizations` 进 `visualize_dataset` 的返回值，而 `GET /api/visualize/{filename}` 把它直接回给客户端；`Path("visualizations") / "plot.png"` 与解析之后的绝对落点是两种文案，换形状要连页面一起改。代价就是这一格：包含判据本身判的是解析之后的落点（拦截是对的，`test_a_naive_containment_check_would_have_called_that_escape_inside` 量到「未规范化的浅判据会把根外形状放行」），但交回调用方那串还没规范化，调用方拿它再拼一层时看到的仍是相对片段。**当前处置**：`TestTheVisualizerFace` 把「判决生效」与「返回原形状」两格分开钉住，改任何一侧都会撞上点名断言。**修法方向**：① 返回值改用 `require_within_dir` 的返回，与另两面齐平，代价是回执文案变绝对落点、须与页面同批；② 维持现状，把「这一面回拼接、那一面回落点」写进两处 docstring 并在文档表里说明；③ 三面统一走一个 helper，按调用方意图开两个出口。定级 S，待拍。 | S |

| A343 | `x = x or DEFAULT` 把调用方**显式传入的空容器**（`[]` / `{}`）读成「没传」而套上非空默认——同族判决在 `augmentor/privacy.py` 的 `PiiSanitizer` 那一格 L188 已经写死过（换成 `x if x is not None else DEFAULT`，注释承认「显式空容器是『零元素』的合法意图，不是『没传』；旧写法把空容器读成没传而悄悄套上默认」），其余格数此前无人量过 | **新立（L262）**：L262 按语法形状全仓普查（`*.py`，排除 `tests/` 与 `test_*.py`），现量 **38** 格、分四族——A 构造器吞非空默认 **10** 格、B 方法体吞非空默认 **7** 格、C 默认本身是空容器的幂等档 **10** 格（`[] or []` 仍是 `[]`，不构成吞，由 `test_class_C_is_provably_idempotent` 逐格验默认字面量）、D 默认是标量 / 对象 / 一次求值 **11** 格。**可达面才是分级依据**：四条 HTTP 路把外部空清单直送进 A / B / D 三族且门口无人拦——`POST /api/audit`（`AuditRequest.fields` 不设 `min_length`）⇒ `DatasetAuditor.fields`、`POST /api/leakage/check` ⇒ `LeakageDetector.fields`、空清单经位置参数落到统计面 ⇒ **「一列都不统计」翻成「全部列都统计」**、`POST /api/dataset/search` ⇒ `EnhancedSearcher.search` 的 `fields`。**两处已防住的对照格证明这一族早被承认**：`POST /api/dataset/evaluate` 在路由层加了 `if not metrics:` ⇒ 400，注释原话是「空列表是假值，传下去会被静默补成三只」；`PiiSanitizer` 在构造器层改了形状。**CLI 无空面**（`--fields` / `--rules` 一律 `nargs="+"`，argparse 保证要么 `None` 要么至少一值），**仓内零调用方传空**（AST 扫全产品码，没有一处把空容器字面量喂给这些参数）⇒ 吞空只在外部输入那一面成立。**为什么本轮不动**：三种修法的对外语义互不相同——① 逐格改 `is not None`（把 L188 那一格的判决推广到 A + B 共 17 格，四条 HTTP 路的响应内容会变）；② 只在门口加封闭校验（`min_length=1`，空清单判 422，产品内部形状不动）；③ 维持现状并把「空 ⇒ 全量」写成明示契约。① 与 ② 都动对外响应面 ⇒ 属契约变更，不在代理自开轮里自选，与 **A280** 那一族的状态码口径须同批拍。**当前处置**：守卫 `tests/unit/test_empty_container_defaults_l262.py` 把 38 格四族名册、A 族十格的现场实例化（传 `[]` 与传 `None` 得到同一个非空默认）、四条可达路与两处对照逐格钉住，另有五支注入臂证明这一形尺子有牙齿。定级 M，待拍。 | ~~M~~ 已收口（L277） |

| A344 | 日志块点名的提交哈希只要 `git rev-parse --verify` 解析得过就算归档 ⇒ 「这枚提交在不在分支上」全账无人量；共享工作树里并行会话的 reset/rebase 会把被改写过的哈希变成游离对象（L265 记账批实测：面上 60 格里 **6 格不在分支上**，全在 L265 块，其中主守卫补交那枚落盘后分钟内就被重写成另一枚），而 `git gc` 之后那六格会整批落进 `not_a_commit` 而红。**当前处置**：本轮只把现量钉进 ⑱，尺子的祖先校验维与游离格的重钉口径一格未动。定级 M，待拍。**当前处置（L270）**：① 那一维由新守卫 `tests/unit/test_ledger_hash_reachability_l270.py` 落成逐格点名名册——两卷现量 634 处反引号 hex，按可达性分五档（reachable 459 / dangling 26 / stash 2 / not_a_commit 147 / ambiguous 0），26 处游离点名逐格名册、全部在 L265 那一轮，名册外多一处即红；判据取全 ref 可达面而不是 HEAD 祖先，两判据的对账维一并钉住（现量给同一批 7 枚对象）。**② 那批游离点名的重钉口径本轮未拍、仍在案** ⇒ 移交新立 A348；新尺自身的 `not_a_commit` 盲区另立 A349，而本编号在历轮日志要点里的三种互斥说法另立 A350。这三笔都是待拍项，本轮一律不拍、也不动。 | ~~M~~ 已收口（L270） |

| A345 | `augmentor/search_enhanced.py` 的 `method="regex"` 与 `POST /api/dataset/search` | **新立（L267）**：坏模式的报错形状本轮已接（编译不过 ⇒ 400 点名模式串与位置），但**模式串本身无上限**——`SearchRequest.query` 是无约束 `str`（实测 200,000 字照收、模型层不拦），`_search_regex` 对每条候选字段跑 `pattern.search`，`run_in_thread` 既无超时也无取消，而 haystack 来自调用方可上传的数据文件：同一请求体两个方向都可控。实测 `(a+)+b` 在 `"a"` 重复串上 n=14 / 16 / 18 / 20 / 22 耗时 0.0008 / 0.00316 / 0.01324 / 0.05305 / 0.20834 s（每 +2 字 ×4 ⇒ n=40 量级就是小时）。候选口径三条都要动对外契约或依赖面（模式长度 + 嵌套量词黑名单 / 请求级超时 / 换 `re2` 类线性引擎），且换了引擎会改语义（现用 `re.IGNORECASE`）⇒ 定级 M，待拍。**L278 落了 ① 那一半**（模式形状闸 + 长度上限，判在语料之前，域外异常一并折成 400），② 请求级超时与 ③ 线性引擎仍在那张拍面上（两半都要动对外语义，见 A371） | M 半修（L278） |

| A346 | a184 内容维剩下的两形：① 点号主张（写成「文件:名字」而不是「文件:行号」）、② 括弧里第一个反引号跨度是散文而不是符号名 | **新立（L268）**：A333 立账点名的三种形状里，L268 只把 ③（不带文件名的裸行号，且限定在「本账第 + 数字 + 行」这一短语）做成了逐格点名名册，①② 两形**依旧没有任何机械通道**——现量证据：卷二 Backlog 与日志块里「`api/deps.py:to_http_error`」这一类点号主张、以及括弧首是散文的引用形状，a184 的结构维与内容维都判成「这不是主张」，于是它们既不会因为内容变陈旧而红、也不会因为落点漂移而红。**为什么单立而不是继续挂在 A333**：A333 的 ③ 已交尺，剩下的两形要动的是 a184 自己的判据面——认点号主张就得把「该名字必须是符号、且行号落在其跨度内」写进内容维；括弧首是散文就要区分「这一串反引号到底是不是一次引用」——两条都比 L268 那把名册尺子改动面大，且都要重钉 a184 的三档现量与硬 0 档。候选口径两条：① 点号主张按符号跨度判定（承 A331 内容维同一实现路径，代价是符号重命名时引用静默失效）；② 把括弧首散文形显式登记为「非主张」并逐格点名（代价同样是名册随账本变长）。定级 M，待拍。 | M |

| A347 | 写端点在未设置 `AUGMENTOR_API_KEY` 时**整体匿名可达**：改配置、删数据、回滚版本、清备份、发起增强都不需要任何凭据，`api/main.py` 启动时只把这一格写成一条告警 | **新立（L269）**：本轮做只读安全普查时量到 `api/deps.py::verify_api_key` 的形状——`expected` 为空（未设置或纯空白）即 `return`，放行方向与「密钥不匹配 ⇒ 401」那条契约同源但语义相反；docstring 明写「未设置时不做校验——保持零配置即可用的既有行为」，所以这**不是写错，而是一条被明文承认的默认策略**。它同时是 **A341** 的放大器：那两步（把 `vector.storage_dir` 指到数据根之外、再触发一次增强把索引写到那里之外）在默认部署上都可匿名完成。**为什么立账而不是直接改**：改成 fail-closed 会让所有「装上就用」的既有部署当场失去写面，是可用性赌注而不是缺陷修复，且与 A341 / A280 同一族对外契约。**候选口径**：① 未设置即拒绝全部写端点（401/403 由 A280 那一族定），配一条显式的「我知道我在干什么」开关才放开；② 保持放行但把告警升成启动失败除非显式确认；③ 维持现状，但在 `GET /api/system/monitor` 与页面上把「当前写面无凭据」显式回执。定级 M，待拍。 | ~~M~~ 已收口（L277） |

| A348 | A344 剩下的那一半：账本里那批**解析得过但任何 ref 都到不了**的点名要怎么重钉——清单行的槽位半新半旧，而日志块 ⑰⑱ 是历史叙述，不能追改 | **移交自 A344 的修法 ②（L270 立）**：L270 的可达性尺子把两卷 634 处点名分了档，量到 7 枚不可达对象、26 处点名全部集中在 L265 那一轮，成因（并行会话对同一分支的两次 amend 与一次 reset）已由 L265 块 ⑰⑱ 逐枚写破，所以这些格**是记录而不是错账**。**为什么本轮不重钉**：① 把清单行槽位换成在分支上的真身，等于改写那一轮「当时提交了什么」的证据，而那些真身只在本机 git 里成立；② `git gc` 一旦跑过，这 26 处会从 `dangling` 整批掉进 `not_a_commit`，那时无处可换，只能按守卫红读数重钉名册——那是纪律而不是决策。**候选口径**：① 清单行保留原哈希并同行追加「已被改写出分支，真身见 ⑰⑱」；② 清单行换成真身、原哈希只留在日志块；③ 维持现状，靠 L270 的名册把 26 处钉死，gc 之后按读数重钉名册。定级 M，待拍。 | M |

| A349 | L270 那把可达性尺子的 `not_a_commit` 档**只数总量（现量 147 处）不逐格点名** ⇒ 一枚写错一位的 commit 哈希会安静落进这一档而永远不会成为缺陷格 | **新立（L270）**：立尺当场就量到的自身盲区，用 `test_a_fabricated_hash_lands_in_the_count_tier` 把它做成案面而不是辩解（编造一枚哈希 ⇒ 落 `not_a_commit`、缺陷格仍空）。对照 l264：它对 21 处逐格点名，本尺面大一圈（147 处、跨两卷），没做同一件事。**为什么本轮不顺手做掉**：逐格点名就是把名册从 11 格扩到 147 格，每一轮落账都要重钉，正是 A333 点名的「名册随账本变长」那一笔代价，要不要付不该由代理定。**候选口径**：① 照 l264 把 147 处逐格点名；② 只点名长度落在批次短哈希档且同行写着「批 / 提交」的子集，其余按区数钉；③ 维持现状并在守卫 docstring 显式承认盲区（本轮已是这一档）。定级 M，待拍。 | M |

| A350 | 日志要点给 Backlog 编号配的那句括号说明与表行内容之间没有任何对账尺 ⇒ 同一个编号可以被写成三件不同的事而全账无一处红 | **新立（L270）**：本轮读 A344 的历轮引用时撞出来的现量——它的表行写的是「点名的提交哈希没人量可达性」，而 L266 块 ⑮ 把它写成「共享工作树吞提交 / 换机器部署」、L268 块 ⑫ 写成「`api/routes/*` 的写盘端点失败形状」、L269 块 ②⑫ 沿用后一种说法并据此决定「与它同批处置更省一次对外契约评估」。**后果不是笔癖**：「写盘端点失败形状」那一格在全账找不到对应表行（本轮逐条比对 Backlog A 与 B 的全部表行，讲写盘端点的两格是 A340 的批量导出咽异常与 A347 的写面匿名可达，都不是失败形状那一维）⇒ 后两轮的「待拍清单」按一个不存在的账做了归并。**为什么没有尺子**：l253 的「同卷可查」判的是编号在不在，不判内容；语义对账要读散文，确定性做不到。**候选口径**：① 把这三形按同行改写订正掉（净 0 行）并立一条「编号 → 允许的说法」逐格名册把订正钉住；② 只订正不立尺；③ 维持现状，把这三形留在本行当读数。定级 M，待拍。 | M |

| A351 | 路径名那一维收齐 500→400 之后还剩三形无人拍：**读侧坏名字回 404「文件不存在」**（名字根本不可能成立，却说「不在」）、**同一档 400 的文案两形**（NUL 走操作系统英文原文 `embedded null character`，L271 新档走中文固定话术）、**POSIX 与 Windows 对控制符名的合法性相反**（`a\nb.json` 在 POSIX 合法、在 Windows 是 `errno 22`） | **新立（L271）**：本轮修的是「写侧被 OS 判死却报 500」那一半，剩下的三半都要用户拍。① 读侧要不要把坏名字判成 400：现量 `POST /api/dataset/stats` 对含 NUL 与超长的名字一律 404，因为闸只判 `..` 与空白、其余交给 `exists()`，而「不存在」与「不可能存在」在客户端看到的文案是同一句；改它就是把 404 换成 400，属响应面判决而不是笔癖。② 文案要不要统一：本轮刻意不转发原文（防部署根路径外泄），而 ValueError 那一支按既有口径**就是**转发原文——NUL 的原文恰好不含路径，所以今天不泄漏，这是巧合而不是设计，要不要收成同一把刀没拍。③ 跨平台口径：要不要在闸里按 `os.name` 显式拒绝 Windows 禁止名（含保留名与控制符），代价是同一份请求在 Linux 能成、在 Windows 不能成，即把「平台中立」这一维变成显式政策。**候选口径**：① 只做 404→400 的读侧对齐（文案不变）；② 三形一起做，闸内按平台判字符集；③ 维持现状，把这三形留在本行当读数。定级 M，待拍。 | M |

| A352 | 十三支文档面守卫把文档路径**硬编码**成 `AI_DIR/docs/...`，仓库级文档搬迁（`augmentor/docs/*` → `docs/augmentor/*`、`docs/oa/*` → `docs/oa4rust/*` 等）之后一度 48 支红 + 2 支采集期 error | **新立（L271）并于同轮由搬迁方按候选 ③ 落地**：搬迁发生在本会话之外（由用户执行），本轮不追改别人的在制品；立账之后搬迁方把守卫面一并补齐并提交（`e4ce1ae36` 把测试的文档路径解析改指仓库根、`94f7f05cc` 重写项目内 390 处文档引用并把 a184 的普查面改指仓库根）⇒ **本轮一行守卫路径都没改，也没选①或②**。**立账时的量法**（现已被上面的提交取代，留着不美化）：`grep` 静态点出把 `docs` 拼在 `AI_DIR` 下的测试文件 **12 支**（`test_api_doc_endpoint_coverage_l210` / `test_api_docs` / `test_api_key_channel_l233` / `test_comment_refs_a184` / `test_config_three_side_l222` / `test_cost_oracle_routing_l241` / `test_doc_contract_numbers_l200` / `test_doc_line_refs_l79` / `test_docs_markdown_structure` / `test_python_version_floor_l198` / `test_quality_threshold_copy_l255` / `test_sample_aggregate_rag_wiring_l230`），另加 `test_api_doc_field_hints_l223` 按另一形拼同一目录 ⇒ 13 支。**产品码零依赖**（`augmentor/` 与 `api/` 里读 `docs/` 的路径一处没有），所以这是**尺子面的路径假设**，不是产品缺陷。**收口证据（本轮全量门禁现量）**：采集期零 error、9429 passed / 10 skipped，文档路径面零红，剩下的 20 红全是本批自己的账本常数与两处账本散文 ⇒ 「跟着搬迁改路径」这一格已由搬迁方做完，不是本账做完，这一笔归因写进定级格之外的那一句。**本轮实测的另一半**：六把账本与文档守卫（markdown 结构 / l98 / l203 / a184 / l79 / l200）在搬迁后**定向跑 168 passed 全绿**，因为它们走的是多根回落而不是单一硬路径 ⇒ 红的集中在按字面路径读文件的每一支。**候选口径**：① 把这 13 支的路径改成「仓库根 `docs/augmentor` 优先、旧 `augmentor/docs` 回落」，让尺子跟着搬迁走；② 在 `tests/` 里立一份单一权威的路径常量，13 支共引，下次搬迁只改一处；③ 搬迁方自己补齐守卫改动再提交，本轮不动。**落地的是 ③**。 | ~~M~~ 已收口（L271） |

| A353 | 写侧没有有限性判据：非有限浮点由本项目自己写进数据集文件，而严格 JSON 解析器读不了那份文件 | **新立（L272）**：`atomic_write_json` 走 `json.dump` 的默认 `allow_nan=True` ⇒ 实测 `{"v":NaN}` / `{"v":Infinity}` / `{"v":-Infinity}` 三形**原样落盘**；本项目的读侧（数据集加载那一族的裸 `json.load`，同样默认接受非有限字面量）把它读回 `float('nan')` 并继续参与统计与门禁 ⇒ **往返闭合**，所以自家工具链看不见这一格，而按 RFC 8259 判的解析器（严格档 `parse_constant`、几乎所有非 Python 消费者）当场 `ValueError`，即本仓库产出的数据集文件不是合法 JSON。本轮普查同时量到入口：读侧裸 `json.load` 让一份含 `NaN` 字面量的输入文件合法进门，写侧再原样送出，而请求侧那一族本轮已在门口挡住非有限值——两侧判据不一致。**候选口径**：① `atomic_write_json` 传 `allow_nan=False`，序列化在临时件上就抛 `TypeError`（与本模块既有「失败时目标文件保持原样」同一失败语义），代价是影响面 16 处以上写点，且**已带 NaN 的存量数据集会从「能写」变成「写不出去」**；② 读侧对输入文件用 `parse_constant` 拒绝，把 NaN 挡在进入点，代价是存量文件当场读不了；③ 维持现状并写成契约（明示本仓库允许产出非严格 JSON 的数据集）。要拍的是「存量数据能不能读」这一维，与 **A311**（422 回显里的 NaN 让响应消失）是同族两侧——一处渲染失败、一处落盘不合规。本轮不拍：它改的是对外产出文件的格式契约，且 ① 与 ② 都把既有读数换成新异常。定级 M，待拍。 | M |

| A354 | `gate_dataset_health` 的三只阈值与 `ImpactEvaluator.is_beneficial` 的 `min_scale_gain` 只有有限性判据、没有范围判据：有限越界照旧进门并出一份判决 | **新立（L272 修正批）**：改前实测 `pass_rate_min=5.0` 与 `-1.0`、`duplicate_rate_max=99.0`、`completeness_min=-3.0` 与 `5.0` 全部 200 并各自给出判决，`min_scale_gain=-2.0`（低于 `scale_gain` 值域下界 `-1`）也 200——门槛设在值域之下，语义上等于「永远放行」，那是一种用法而不是笔误。L272 的接线批一度把 `require_ratio` 的闭区间一并接上，于是这些**改前就合法**的有限值全变 400，被 `tests/unit/test_health_gate_wiring_l235.py`（3 支：判决单调性要靠 `99.0` / `5.0` 才取得到 passed 与 failed 两极）与 `tests/unit/test_sample_aggregate_rag_wiring_l230.py`（2 支：`ratio=2` / `ratio=-0.5` 的三方法结局矩阵）当场抓住 ⇒ 处置是把产品改回去，不是重钉那两把尺。**候选口径**：① 按语义各定上下界（三只阈值收 `[0,1]`、`min_scale_gain` 收 `[-1,+∞)`）；② 只在表单侧拦而后端不动；③ 维持现状并写成契约，在文档里明示这一族不拦范围。三条都要拍「已部署的客户端会不会因此从 200 改收 400」这一维。它与 **A292** 是同一维度的两侧——那一格管 `sample.ratio` 与 `split` 的三只 ratio，这一格管另外四只旋钮。定级 M，待拍。 | M |

| A355 | `path_lock` 是**进程内**的线程锁：同一份文件的读-改-写只在同进程互斥，`uvicorn --workers N` / 多实例部署下丢更新照旧发生 | **新立（L273）**：本轮把 7 处写者串到一把按路径的 `threading.RLock` 上，取证面是**单进程单 worker**（仓外沙箱 + `TestClient` / 直接调 SDK），所以「lost=0」这一读数只覆盖同进程那一维；跨进程时两台 worker 各持一把自己的锁，磁盘上仍然是「各读旧快照、各写整份」，回执也仍然全是 `success: true`。三条候选：① 落盘段改走进程间文件锁（Windows `msvcrt.locking` / POSIX `fcntl.flock`，锁一个与目标同目录的 `.lock` 件），代价是新增一类操作系统特定的失败档；② 单写者队列（所有 JSON 落盘投递给一支 worker 进程串行执行），代价是写侧延迟与一处新的生命周期；③ 乐观并发（写前比 `sha256` 或版本号，冲突回 409 而不是无声覆盖），代价是改响应面、与 **A280** 的状态码契约同批。要拍的是**部署形态**：本仓今天按单进程跑还是允许多 worker，决定了这一格是缺陷还是纸面风险。本轮不拍：三条候选都改对外行为或部署契约。定级 M，待拍。 | M |

| A356 | HTTP **读**侧撞上写侧 `os.replace` 那一瞬时拿 `PermissionError [Errno 13]` 并折成 500，本轮只在改前读数里观测到、没做定向探针 | **新立（L273）**：改前 50 支并发 `PUT /api/data/update` 的 3 支 500 全部落在 `api/deps.py::read_items` 的 `open()` 上，调用栈是「另一支正在 `os.replace` 同一份文件」——Windows 在那一瞬拒绝打开，POSIX 不会，所以这是一形**平台特定的瞬时失败**而不是逻辑缺陷。本轮串行化把 RMW 整段罩进同一把键 ⇒ 该形状在**写侧互斥面内**现量为 0；但 `GET` 那一族的裸读（不经 `path_lock` 的读点）与写侧之间的窗口**没有**被本轮关掉，只是没有为它写探针。三条候选：① 补一支定向探针把「读点与 `os.replace` 相撞」钉成已知形状（不改产品，只补尺子）；② 读侧对 `PermissionError` 做有限次重试（改的是延迟与吞吐）；③ 与 **A280** 同批拍状态码（瞬时 OS 失败算 500 还是 503 / 429）。本轮不拍：② 与 ③ 都改对外响应面，① 单独做也只是给一形尚未被用户拍定的读数加尺子。定级 M，待拍。 | M |

| A357 | 定位引用两把尺（`test_comment_refs_a184` / `test_doc_line_refs_l79`）只判「`file.py:NNN` 落在代码跨度内」，不判「那一行是否含有被点名的构造」 | **新立（L273）**：本轮重钉历史行号时按内容审计（`Temp/l273/audit_refs.py` 打印每处被引行的当前内容）量到三处在 HEAD 上**早已平移出被点名的构造而两把尺全绿**——`augmentor/backup.py:90`、`augmentor/version_control.py:146`、`augmentor/versioning.py:372`（真值分别是 `:96` / `:159` / `:390`，off by 3〜10 行）。尺子为什么看不见：它按 `dead_line` 判「行号还在代码跨度里」，行号落在同一只文件的另一段代码上时判据成立、语义却已经错了；**同族的另一半本轮也量到**——先按统一位移算术重钉会算错（本轮把 146 钉成 153、372 钉成 380 都是错的，因为插行不止一处且各处不同向），只有逐格读内容才能钉对。**候选口径**：① 逐格比对被引行的**内容**（要求点名的符号或构造出现在那一行），代价是历史引用会成批变红、要一轮专门重钉；② 历史行号降为散文（**A141** 的候选之一，与 **A148** 同族）；③ 维持现状并把「内容审计」定成每轮固定动作。要拍的是「一次改严、成批重钉的成本谁付」，与 A141 / A148 是同一条线上的三格。定级 M，待拍。 | M |

| A358 | `api/routes/dataset_tools.py` 的 `_as_column`（评估侧）与 `augmentor/converter.py` 的 `read_conversation_turns`（读入侧）把在场非字符串 `str()` 归一，与本轮「在场且非字符串即拒」**口径相反** | **新立（L274）**：同一份 `[{"instruction": 1, "output": 2}]` 打在 `POST /api/dataset/merge` 上是本轮的 400 判决，打在 `POST /api/dataset/impact` 上实测仍 **200** 并出一份判决（评估两侧按索引配对，归一后照常算分）；读入侧的归一是「把 JSON 里的数字转成文本再落库」，属于产出面的口径。三条候选：① 统一成拒绝——改的是现在成功的评估与读入请求，状态码语义与输入接纳面同时变；② 统一成归一——本轮那一族的 400 要退回，而 merge 带 `deduplicate=true` 时 `1` 与 `"1"` 会折成同一份文本而互为重复、`str(None)` 造出一个名叫 `None` 的假样本；③ 维持两侧并存并把差异写进文档与守卫名册（本轮走的就是这一档，守卫 `BLIND_ROSTER` 逐格钉住，谁接线那一格当场红）。要拍的是「同一份文件在两只端点上一只被拒一只被算」这条对外口径，本轮不自决。定级 M，待拍。 | M |

| A359 | CLI 面同一形给的是裸 SDK 消息：`augmentor/cli/commands/analysis.py` 与 `quality.py` 那三员（`analytics.analyze` / `quality_report.generate_report` / `visualizer.generate_statistics`）没有本轮那道值维门 | **新立（L274）**：本机实测 `analyze --input <退化.json>` 与 `quality-report --input <同一份>` 都是 **rc=1** 且 stderr 一行 `错误: object of type 'int' has no len()`——不点名第几条、哪个字段；`cli.py:53` 的 `except Exception` 把域判决与崩溃折成同一句式同一退出码，而 `augmentor/cli/verdict.py` 的 docstring 自己写明「同样是 1，判决与崩溃必须由 stderr 有无「错误:」来分」，这一格两者都有。两条候选：① 把 `require_item_text` 接到那三员（与 HTTP 面同一权威、改动小，但今天的 `错误: <裸消息>` 会换成 `错误: <判决>`，CLI 的 stderr 文案是要拍的对外面）；② 在 CLI 层单独判（同一判据两份实现，违 A77 那条「判据住一处」）。另有一格实测留档：`visualize --input` 同形 **rc=0** 只打降级告警，因为本机未装 matplotlib / wordcloud，图表族整族被跳过、根本没走到 `generate_statistics` ⇒ 这条读数**不能**读成「visualize 在这一形下静默出错答」。要拍的是 CLI 的 stderr 文案面，本轮不自决。定级 S，待拍。 | S |

| A360 | `POST /api/versions/diff` 拿不存在的版本 id ⇒ **500**：本轮自动普查 146 格里这一族仍 500 的唯一一格，它同时把 A280 要拍的判据维第一次量准 | **新立（L274）**：`api/routes/version.py::diff_versions` 收尾用 `raise_boundary_error`，而那只函数的放行名单判的是 `DataValidationError`；实测 `VersionError` 的 MRO 里**有** `ValueError`、**没有** `DataValidationError` ⇒ 它被交给 `raise_internal_error` 折成 500 固定文案，两份语料形状同码（与数据集内容无关，纯 ID 形状）。三条候选：① 放行名单改判 `ValueError`——那一批域错误会一并进 400，正是 A280 那一格要拍的事；② 给「ID 不存在」单独出 404——新增一类状态码语义；③ 维持 500（本轮走这一档，只在守卫里按观测行为钉住并验不回显调用方给的那两个名字）。要拍的是放行维：按「是不是 `ValueError`」还是按「是不是某个具名域错误」。定级 M，待拍。 | M |

| A361 | `api/main.py:216` 把 `StaticFiles(html=True)` 挂在 `/` ⇒ 所有**非 GET 的未知路径**拿 405「Method Not Allowed」而不是 404，端点名拼错的客户端收到误导回执 | **新立（L274）**：实测（`web/dist` 在场）`POST /api/dataset/filter`、`POST /api/dataset/nonexistent-xyz`、`POST /api/healthzzz`、`POST /` 四格全部 **405**，而 GET 同路径全部 **404**；同一批请求在未构建前端的机器上变成 404，即这一读数**随部署产物漂移**——本轮守卫里 `dataset_filter` 那一臂本来把 405 抄成常数，修正批 `1ca3dd319` 改按 `app.routes` 现算并补一支「phantom 与任意不存在路径同码」的牙齿。两条候选：① 静态挂载只接非 `/api` 前缀（把 API 面的 404 找回来，动的是路由装配与前端兜底）；② 维持现状并写进文档（本轮走这一档）。要拍的是路由装配面；它与 A352 不是同一件事——那一格量的是文档路径搬迁，这一格量的是挂载顺序。定级 S，待拍。 | S |

| A362 | `api/routes/augment.py:114` 的 `GET /api/augment/progress` 调的是**不带参数**的 `get_progress()` ⇒ 它报的是「manager 此刻最后加载的那一份断点」，并发增强时这一面向客户端说不清是哪只任务 | **新立（L275）**：接线批把「这一笔进度属于谁」交给了调用方手里的对象，但那只端点手里没有对象——它只有一个进程内单例 manager。现量（仓外沙箱，A=60 条与 B=20 条并发跑完后读无参 `get_progress()` 与 `list_checkpoints()` 对照）：并发跑完那一刻无参读到的是一只（`total_items=60` / `processed_items=60`），把盘上两份各 `load_checkpoint` 一遍再读同一只无参函数就换成另一只（`total_items=20` / `processed_items=20`），而盘上两份同时都在 ⇒ 同一时刻两种答案都「对」，取决于谁最后加载；两份报告自己的 `progress` 格各自正确（60/60 与 20/20）。 两条候选：① 端点补 `task_id` 查询参数并让启动回执带回它（要动请求模型与响应模型）；② 文档写明「本端点只报最近加载的那一份」。① 属改响应面一族，本轮不自决。定级 M，待拍。 | ~~M~~ 已收口（L276） |

| A363 | `augmentor/pipeline.py:718` 每次 `visualize_dataset` 都写 `self.visualizer.output_dir` ⇒ 共享单例上那只 visualizer 的落盘目录被最后一个请求搬走 | **新立（L275）**：L275 普查（构造器之外对 `self.X` 的赋值 129 处、其中增强跑路径 45 处）里同一族的第三格。本轮**只立不修、行为差未量**——要真跑出两份并发可视化才有可观察的差，而它和 A362 一样要先拍「每次调用带目录参数」还是「visualizer 也做成运行局部」，属改对外面一族。定级 S，待拍。 | S |

| A364 | 残差「共享批评态」名册（L275 普查的余量）：`augmentor/quality.py:226` 那一份参照判据与其后的三份分次写、`augmentor/context.py:114` 的轮数读取、`augmentor/tracker.py:86` 的 `_current_experiment` | **新立（L275）→ 半修（L279）**：第一格最尖——参照切片判与 embeddings、行范数、参照文本三份写点分居四处，并发时另一次运行可以读到「新的 embeddings 配旧的参照文本」这一对撕裂组合。L279 按两线程对撞量出改前行为差（仓外探针 + 守卫内的 no-op 假锁臂，Temp/l277_probes）：n-gram 支**静默污染**，A 的候选配 B 的参照集，多样性由真值 `0.5833…` 变 `1.0`——本该被过滤的重复样本被放行且没有异常出声；向量支两臂当场崩，`ValueError: operands could not be broadcast together with shapes (3,) (2,)`（写完矩阵停在范数之前）与 `(2,) (3,)`（读完矩阵停在除法之前），前者还把缓存毒成「2 条文本 / 3 行矩阵 / 2 个范数」，判据与内容不再同源、此后每次调用都崩；`cached_encode` 的有界 FIFO 里两只线程各看过 `len()>=2` 又取到同一枚最老键，后落盘那一只 `pop` 当场 `KeyError`。`augmentor/context.py:114` 那一格改前是「存-改-复原」，交错两轮**双方都错**（要 2 轮的拿到 5、要 5 轮的拿到 3），实例默认值还被永久留在最后一次复原的那一格（3→2）。修法：`quality.py` 用同一把 `self._cache_lock` 罩住全部「判据 → 刷新 → 读取」（现量四处临界区：n-gram 支、向量支、`reset_cache`、`cached_encode` 的淘汰与插入同一临界区），轮数改沿调用链带参数（`augmentor/context.py:203`），顺带把 `_embedding_cache` 的初始化从 `batch_score` 里的 `getattr` 赋回收进构造器（并发首调两份空字典互相顶掉）。新守卫 `tests/unit/test_shared_cache_lock_l279.py`（19 支 / 零 parametrize）三面：AST 名册 + 两线程行为对账（A 停在临界区内时 B 进不去，这就是互斥的直接证据）+ 反证与尺子自证（假锁臂必须重现 `1.0` 污染、两形 `ValueError`、毒态 `(2, 3, 2)` 与 `KeyError`；「存-改-复原」替身臂必须重现 `{A: 5, B: 3}` 与默认值 3→2；HEAD 版裸写形状的合成语料判 11 处裸点，换一把不相干的锁同样判红）。**仍在案两格**：`augmentor/tracker.py:86` 的 `_current_experiment`（要给 `log_metric` 加把手参，属对外签名面）与「run-scoped 缓存 vs 并发互逐 thrash」的取舍，均待拍。定级由 S 收成 M 半修，逐格点名以上。 | M 半修（L279） |

| A365 | 并行增强消费侧按**计数**配对：`for future in as_completed(futures)` 每只 future 迭代一次，结果却不看 future，而是从不带超时的 `result_queue.get()` 取 ⇒ 任一 worker 少投递一次，整批增强就**永久挂死且零 CPU**，而那笔异常因为 `future.result()` 从未被读而**无声吞掉** | **新立（L275）**：接线批落盘后全量定标连续两次停在 62%，加 `-o faulthandler_timeout=120` 打出栈：主线程停在 `augmentor/pipeline.py:398` 的 `Queue.get`，两支 worker 线程 idle 在池里等活，CPU 两次采样逐字相同（208.3125 → 208.3125，即完全没在跑）；触发者是 `tests/unit/test_micro_branches_l68.py:200` 那只替身的形参比真实签名少一格（本轮给 `_process_single_item` 追加了尾部可选参），`executor.submit` 传第 5 个实参时 worker 里抛 `TypeError`，future 带异常完成、队列里少一条，主线程于是永等。同一形状在产品里也走得到：`except` 支内部（建 `error_info` / append / `logger.error`）任何一格抛出，那一笔 put 就没了。更要紧的是 `futures[future] = idx` 建了却从不读——配对纯靠顺序，所以 worker **多**投一条时 `idx` 与 future 会错配，进度与变体记到别的条目名下。本轮只把替身补成同 arity（否则门禁无法继续）并立账；修法要先拍「少投递那一笔算条目失败继续，还是整批出声」，后者改的是 `augment_dataset` 的对外归宿，本轮不自决。定级 S。 | ~~S~~ 已收口（L276） |

| A366 | `augmentor/backup.py:116` 的备份名此前只过「落在根内」那道门：`name="sub/deep"` 建得出 `sub/deep` 这一份，而 `GET /api/backups` 交回的名字带斜杠、`DELETE /api/backups/{name}` 与 `restore` 把它当两个 URL 段 ⇒ 「建得出来却寻不了址」的失踪态：盘上有那一份，索引里那一行却永远够不着 | **新立（L277）并同轮收口**：新公共判据 `augmentor/validation.py:1191 require_path_segment`——分隔符整族（`/` 与反斜杠）、`..` 组件、NUL 与空串一律 `DataValidationError` ⇒ HTTP 400；出生点只接一处（`= require_path_segment(… or …)` 的同行写法，守卫钉「全仓只有一处调用」，接两处就是两套口径）。`web/src/services/api.ts` 的备份名单一路由改走 `encodeURIComponent`：存量名字里可能有 `#`（改前建得出来），裸插值会被浏览器折成 fragment ⇒ 那些历史名字今天仍然删得掉。行为变更：带分隔符的名字改前 **200** 并建子目录、改后 **400** 且不落盘。新守卫 `tests/unit/test_backup_name_path_segment_l277.py`（16 支）；`test_backup_wiring_l231.py` 与 `test_backup_index_collapse_l250.py` 里各一档语料按新口径改判（口径变更不是放宽，各留反证）。定级 M，本轮按用户 goal 授权直接收口。 | ~~M~~ 已收口（L277） |

| A367 | `/api/data/load` 的 `page` / `page_size` 一路落进 `items[start:end]`：`page=0` 与 `page=1` 同答、`page=-1` 折成尾部切片、`page_size=0` 交一份空页还自称分页成功、`page_size=-2` 悄悄删掉末尾两条；更要紧的是**页内下标与盘上下标不同形**——前端拿页内下标发 `PUT /api/data/update`，第二页的第一条会改到第一条 | **新立（L277）并同轮收口**：门口 `api/routes/data.py:172`~`:173` 用 SDK 唯一权威 `require_count("page" / "page_size", minimum=1)` 判掉，判在 `_safe_data_path` 与 `try` **之外**（写在 `try` 里就被 `deps.raise_internal_error` 收敛成 500——A280 那一族本轮不动）；响应新增 `indices`（与返回行同序的盘上 0 基下标），写端点坐标交回唯一权威，前端 `web/src/pages/DataManagement.tsx:110`（`setRowIndices(result.indices)`）改读它——初稿指的 `web/src/services/api.ts` 那一行是注释而非代码，a184 当场判 dead_line。行为变更：`page<1` / `page_size<1` 改前静默错答、改后 **400**；`/api/data/load` 响应体多一格 `indices`（OpenAPI 契约与 TS 类型面同步，见日志块 ⑪）。新守卫 `tests/integration/test_data_page_slice_l277.py`（17 支）。定级 M，本轮按用户 goal 授权直接收口。 | ~~M~~ 已收口（L277） |

| A368 | `AugmentorPipeline.augment_async` 的 `use_checkpoint` 是**死参数**：`docs/augmentor/API_CORE.md` 把它写在公开签名上、默认 `True`，函数体一字没读它 ⇒ 异步入口既不建断点也不落盘，`list_checkpoints()` 恒空，中断之后无从续传，而回执里的 `async_mode: true` 让人以为同一套断点机制在跑 | **新立（L277）并同轮收口**（`augmentor/pipeline.py:721` 起）：与同步入口 `augment_dataset` 逐字同式——任务 ID 由输入路径派生、`get_remaining_indices(ck)` 取未做下标、只投余下的条目、每条 `update_progress(index, success, checkpoint=ck)` **带身份**（L275 的运行局部状态那一格在这里必须传，否则又回到「A 的进度落进 B 的断点」）、收尾 `save_checkpoint(ck)`。报告键**不加** `progress`：文档未承诺，加了就是另一族契约变更。新守卫 `tests/unit/test_async_checkpoint_resume_l277.py`（6 支）含反证臂：把 `get_remaining_indices` 改回「返回全部下标」，产物第一行立刻变成变体 ⇒ 续传那一格吃重。`test_run_scoped_checkpoint_state_l275.py` 的 AST 名册因此从 4/1/1 走到 6/2/2。定级 M，本轮按用户 goal 授权直接收口。 | ~~M~~ 已收口（L277） |

| A369 | `load_checkpoint` 的增量并回是纯 `extend`：崩溃窗（主文件写成之后、`_remove_delta` 之前被杀）让同一批增量下次恢复再并一次，实测 6 条已完成读成 `processed_items=12`、`progress_percent=200.0%`，且这份错读数随合并**落盘固化**；另一形是 `_delta_files` 把 legacy（`_delta.json`）与 JSONL 一并读回，同一下标在两份里各出现一次时照样记两笔 | **新立（L277）并同轮收口**：合并两侧都改判「这一格记过没有」——`already_completed` / `already_failed` 先取主文件名册，再对 `dict.fromkeys(delta[...])` 过滤；写成**净零行**的原地改写，所以 `_save_delta` / `_read_delta` / `update_progress` 三档现量不平移。`test_checkpoint_recovery_l100.py` 钉的是写**失败**那一支，这两序此前都无守卫。新守卫 `tests/unit/test_delta_remerge_l277.py`（8 支）＋三支字节注入臂（摘主文件对照 ⇒ 6 failed、摘增量自身去重 ⇒ 1 failed、摘 failed 那一侧 ⇒ 1 failed，finally 复原并断言 sha 与进入时逐位相同）证明两维都吃重。行为变更：重放同批不再累加（改前 12 / 200.0%，改后 6 / 100.0%）。定级 M，本轮按用户 goal 授权直接收口。 | ~~M~~ 已收口（L277） |

| A370 | 静态闸判据与 `re` 权威之间剩下的三处口径差 | **新立（L278）**：本轮把五格缺陷接进判据之后，仓外探针（Temp `l277_probes`）按 CPython 3.12 的 `re` 逐形对账，量到三类差异仍留在两侧之间——① `a{+1}` 实测是量词而闸按字面量放行（那一形自身不含可变长组，无安全后果，但判据与权威不同字）；② 已知误伤两格由真值表的 `KNOWN_FALSE_POSITIVES` 逐格点名，判据比 `re` 严；③ 括号不配对、类未闭合、组名没写完、结尾裸反斜杠这四种形状闸仍不下判断，而它们同时是 `re.compile` 语法错⇒ 由编译那一步折成 400，用户看到的回执形状与闸拦下时不同（闸给「模式含指数风险」，编译给「语法错位置」）。要不要把判据与权威逐字对齐，先得拍「误伤算不算对外契约」⇒ 定级 S，待拍。 | S |

| A371 | A345 剩下的 ② ③ 那一半——请求级超时与线性引擎（`re2` 类） | **新立（L278）**：形状闸与长度上限这半边已在 L278 落地，运行时那一半仍空：`augmentor/search_enhanced.py` 的 `_search_regex` 跑在 `run_in_thread` 上，既无请求级超时也无取消，而闸只拦「可变长组被重复到 ≥2 次」那一族指数形，合法但极贵的单发长回溯形状仍能占住一只工作线程到分钟级；换 `re2` 类线性引擎会改语义（现用 `re.IGNORECASE`，`\d` 与 `(?i)` 的口径在两库里不完全一致），装超时改的是搜索端点的对外回执形状⇒ 两半都要先拍，本轮不自决。定级 M，待拍。 | M |

| A372 | 两卷账本表格的 GFM 竖线口径余面——尺子按裸 `split("\|")` 数格，渲染按未转义竖线数列 | **新立（L278）**：本轮把 `docs/augmentor/API.md` 的表行竖线改按 GFM 转义、并把列数判据收进 l210 的单一权威之后，同一把尺普查两卷账本（仓外探针 Temp `l277_probes/_gfm_residual_census.py`），量到三处余面——① 卷二 :191（A325）整行 15 枚竖线、`column_count` 读成 15 列而表头只有 4 列 ⇒ GitHub 渲染时那一行与表头错位（其中 6 枚在行内码里，其余在正文）；② 卷二 :229（A344）整行只有 4 枚 ⇒ 少一级分隔，定级格落在「面」列的位置上；③ `tests/unit/test_backlog_mark_vocab_l253.py` 的「末格定位不靠格数」那一格按裸竖线数格（现量 9 行格数不等于 6，逐格点名 A70 / B2 / A288 / A291 / A295 / A321 / A325 / A344 / **A372 自己**——本行就是第九格，它引用的那条判据把它也算进去了），其中 7 行带的全是已转义竖线（A70 1 枚、B2 3 枚、A288 / A291 / A295 / A321 各 2 枚、A372 自己 1 枚）⇒ 它们在尺子里仍算异形行；真正与表头错位的只有 A325（15 枚未转义）与 A344（少一级分隔）两行。修法要同时动两卷正文与那把尺的判据（9 按 L278 修正批红读数重钉并逐格点名），且改的是历史日志档的排版 ⇒ 非本轮自决。定级 S，待拍。 | S |

| A373 | `augmentor/cache.py::MemoryCache` 的七个公开入口此前全在锁外，而同文件的 `_synchronized` docstring 把它写成豁免面、理由逐字是「后者只碰进程内的 dict/计数器，竞态最坏结果是少统计一次」；`models/base.py:183` 又把**一只**实例交给整池工作线程共用 ⇒ 一笔已成功的生成会被记成失败条目 | **新立（L279）并同轮收口**：确定性钩子（把 `self._cache` 换成在指定那一步停住的 dict 替身）量到两形**必现异常**，不是少统计——`get` 的 `if key in self._cache: entry = self._cache[key]` 停在 `__contains__` 时另一线程删同一枚键，A 复跑第二次读当场 `KeyError: 'a'`；`_evict` 先取 `sorted(self._cache.keys(), key=lambda k: self._cache[k].hit_count)` 的键名单再逐个回读，名单里任何一键在这两步之间被删 ⇒ `KeyError: 'b'`。概率锤（8 线程 × 4000 轮）0.02 秒内一格都打不出（窗口只有几条字节码宽）⇒ 循环撞不可作证据。修法：实例级 `threading.RLock()`，七个公开入口全装 `_synchronized`（整块装饰器定义前移到 `class MemoryCache` 之前，否则 import 期找不到），`_evict` 的口径改写走同行净零并写明「调用方持锁」；docstring 那句假话改判成「L279 把这句话证伪了」。新守卫 `tests/unit/test_memory_cache_lock_l279.py`（13 支 / 零 parametrize）三面：AST 名册（七格逐个带、`__init__` 与 `_evict` 不带、装饰器定义先于类、`DiskCache` 五格未退化、可达性前提三格现量）＋ 真锁行为对账（A 停在临界区内时 B 等 0.25 秒进不去，放行后两臂零异常）＋ 反证与尺子自证（no-op 假锁臂原样重现 `KeyError('a')` / `KeyError('b')`；摘掉其中两格装饰器的合成语料被逐格点名 `{'get', 'clear'}`；换一把不相干的锁同样整族判红）。落点代价：`cache.py` 净 +17 行全部落在 `_get_cache_path` 之前，把 l259 名册键 `cache.py:245` 平移成 `cache.py:262`（同一处 md5 拼接，处置理由一字未动）。定级 M，本轮按用户 goal 授权直接收口。 | ~~M~~ 已收口（L279） |

| A374 | `augmentor/validation.py:991 require_within_dir` 那一族缺第八档（冒号）：九个模块的请求 id 仍以 `C:` 开头走进 `root / candidate`，pathlib 把盘符前缀吞掉 ⇒ 落点名与回执 id 不同形 | **新立（L279）**：L279 给备份名落下第八档之后，按同一判据把权威调用面过了一遍（只读探针，仓外 `Temp/l277_probes/colon_within_dir.json`，基座取绝对路径 `C:\base\backups`）——① `C:foo` **被放行**，返回落点 `C:\base\backups\foo`：`path_lands_within` 判不出逃逸（同盘），但落点名已经不是 `C:foo`；② `C:` **被放行**，落点直接变成基座目录本身 `C:\base\backups`——对「拿返回值当文件写」的调用点就是把目录当文件用；③ `a:b` 与 `D:evil` 判 400，文案是「名称 不能指向 backups 目录之外」，而 `a:b` 的真实形状是基座被整个丢掉、`resolve()` 之后才落到根外 ⇒ 回执可读性误导；④ 基座是相对路径时同一枚 `C:foo` 依 cwd 解析成 `C:\backups\foo`，`a:b` 解析成 `a:\backups\b`，跨盘且cwd 相关。权威调用点 17 处 / 9 个文件（`checkpoint.py` 3、`dataset_ops.py` 1、`export.py` 2、`pipeline.py` 1、`tracker.py` 1、`vector/faiss.py` 4、`version_control.py` 3、`versioning.py` 1、`visualizer.py` 1，格数沿用 l259 名册）。两条候选：① 把冒号档下推进 `require_within_dir`，九处一起获得，代价是**对外回执变更**——这些位置今天传 `C:foo` 是放行（本轮只量判据本身，没逐处打 HTTP 量端到端回执），改后是 400；② 只在备份名一处保留第八档，其余记为已知残留，并把 ③ 那句文案改诚实。要拍的是「这九处的放行算不算对外契约」与 400 文案口径 ⇒ 定级 M，待拍。**· L286 摘掉② 那一格（① ③ ④ 原样待拍）**：L286 没有下推冒号档，而是给权威补了「落点不许是基座目录本身」那一档（正向 + 反向两次 `path_lands_within`），于是本轮实测的 ② 里 `C:` 与 `.` / `./` / 空白包裹的 `.` 同判 400；② 那句「对拿返回值当文件写的调用点就是把目录当文件用」在版本面还有一形更狠的——`delete_version` 拿的是**目录**，实测 `delete_version(".")` 直接把整个基座 `rmtree` 掉（含 `history.jsonl` 与全部版本），这一形由同一档一起堵住，且它的对外回执只是把一枚「今天 200 且毁盘」的请求改成 400，与 L258 立尺时把 `../victim` 那一形从 200 改成 400 是同一条口径，不算新的对外承诺。① ③ ④ 三格（`C:foo` 放行且落点名变形、`a:b` 的文案误导、基座相对时依 cwd 解析）一格未动，判据、档名与「九处的放行算不算对外契约」这一问原样留着。 | M |

| A375 | `augmentor/cache.py::DiskCache` 的「判过存在再动手」一族（`delete`、`clear`、`_enforce_size_limit`、`stats`、`_load_metadata` 五处）与 `set` 在单条超上限时的自逐出 | **新立（L280）并同轮收口**：确定性替身（路径对象先按真实结果返回、再在调用方动手那一步之前把文件删掉）在仓外 `Temp/l277_probes/l280_cache_toctou.json` 量到 `delete` / `clear` / `stat` 三形**必现** `FileNotFoundError`；第四处 `stats` 不是照着清单补的，是新守卫的 AST 名册对现版源码跑出来的第一条红读数（`stat_unprotected` 点名那一格）。可达性按调用点量：`models/base.py` 每只模型各造一只 `DiskCache`，HTTP 生成路径读缓存那一行**没有** try 罩（同一函数里只有写那一侧罩了 OSError，那句注释是「磁盘写入失败不应让已经成功的生成变成失败」），而 `get` 的过期分支正是 `delete` 的产品调用点 ⇒ 一次外部删除（第二个实例、运维手删、临时清理任务）能把本该算未命中的生成整个判失败。自逐出那一格更贵：`max_bytes=400` 先存三条小条目再写一条 3000 字节，改前 `_metadata` 与目录里的 `*.json` **同时清零**、`get` 那条大的读回 `None`（`Temp/l277_probes/l280_cache_selfevict.json`，两轮复现），`set` 静默变成 no-op 且每写一次超大响应就毁一次缓存。修法：`delete` / `clear` 走 `unlink(missing_ok=True)`，`_enforce_size_limit` 与 `stats` 把 `exists()`/裸 stat 换成 `try/except FileNotFoundError`（消失的文件按「已经不在目录里」计），`_load_metadata` 直接开文件、缺失即空账，`_enforce_size_limit` 新增 `protect` 形参、由 `set` 把刚写入那一条交进去——它只贡献体积、不进被淘汰的名单，单条本身就超线时其余条目一律不牵连并出 warning。行为变更：这三形从 500/异常变成正常未命中或正常统计；超大条目的写入从「毁掉整本缓存」变成「自己留着、别人也留着」。新守卫 `tests/unit/test_disk_cache_missing_file_l280.py`（24 支 / 零 parametrize）三面：AST 名册（`unlink` 恰好 3 处且逐处声明 `missing_ok=True`、`.exists()` 只剩 `get` 一处、每一枚 `.stat()` 都落在 `except FileNotFoundError` 之下、`set` 确实把 `protect=key` 交下去、`get` 的 `open` 落在 IOError 之下——那处豁免是**被证明**的，不是又一句「最坏只是少统计一次」）＋ 真临时目录的行为对账 ＋ 反证与尺子自证（同一替身喂改前的裸写形状必抛、`missing_ok=True` 换成 `False` 必被名册点名、`_enforce_size_limit()` 摘掉 `protect` 必重现「四条同时清零」、HEAD 形状合成语料逐格点名）。落点代价：本轮产品改动净 +30 行全部落在 `_get_cache_path` **之前**，把 l259 名册键由 262 顶成 268，并把卷二历史日志里那格指向 md5 拼接的活引用顶出代码跨度 ⇒ a184 硬 0 档当场判 `dead_line`，两处都按守卫现量重钉（不是按「应该移了几行」推的）。定级 M，本轮按用户 goal 授权直接收口。 | ~~M~~ 已收口（L280） |

| A376 | `augmentor/cache.py::DiskCache._save_metadata` 是**整本元数据的全量重序列化**：两只实例共用同一枚 `cache_dir` 时后落盘那一份把对方新增的键从索引里抹掉 | **新立（L280）**：仓外探针 `Temp/l277_probes/l280_cache_toctou.json` 量到的形状是——A 与 B 各持一只实例、各写一键，B 的 `_save_metadata` 落盘后索引只剩 B 那一格（A 的键从索引消失），而磁盘上两份 `.json` 都还在；两侧实例锁 `locks_are_same_object` 现量 `false` ⇒ 进程内两道锁互不相干，罩锁罩不到跨实例这一族。更糟的后果是容量：`_enforce_size_limit` 只遍历 `self._metadata`，被抹掉的键从此没人再记账 ⇒ **孤儿文件永不被淘汰，`max_bytes` 对它们失明**，而 `delete` 那一侧仍回执 `True`（改前在这一格还会抛 `FileNotFoundError`，见 A375）。对照实验：单实例多线程跑同一批交错，`errors` 现量 `[]` ⇒ 这一族的成因是「两只实例两份账」，不是「同一把锁没罩住」。可达性：`models/base.py` 每只模型各造一只 `DiskCache`，工厂允许两只模型指同一枚 `response_cache_dir`。三条候选各有代价：① 落盘加文件锁（要么引依赖，要么自写 `O_EXCL` 自旋，且读-改-写窗口仍要在锁内重读索引）；② 改成增量写单键（要换索引格式，存量 `_metadata.json` 的可读性属数据迁移面）；③ 按 `cache_dir` 在进程内共用一只实例（改的是构造器的身份语义 ⇒ 对外可观察：一个实例的 `clear()` 会顶掉另一个的账）。三条都动对外语义或存量数据 ⇒ 定级 M，待拍。 | M |

| A377 | `augmentor/atomic_write.py`（两处 `finally` 的临时件清理）× `augmentor/backup.py`（`delete_backup`、`verify_backup_integrity`）× `augmentor/checkpoint.py`（`_remove_delta`、`delete_checkpoint`）× `augmentor/data/image.py`、`augmentor/data/audio.py`（各一处取尺寸） | **新立并同轮收口（L281）**：A375 那一族在 `DiskCache` 之外还剩十格，由本轮的只读 AST 普查在 133 只产品文件里点名（探针写在仓库外的临时目录，不入仓故此处不写成定位引用）。形状同一：**先判存在再动手**，`exists()` 与随后的 `unlink()` / `stat()` 之间文件被并发删掉就抛穿。可达性逐格坐实——`POST /api/multimodal/scan` 是 `iterdir()` 排好队再逐只处理，队列里任何一只在轮到它之前消失，那一次裸 `stat()` 就把整次扫描变成 500；`delete_backup()` 那一格抛出的 `FileNotFoundError` 会跳过紧随其后的索引重排并被路由折成非 200，而该端点的 responses **明文**写着「索引有条目而备份文件已丢不算这一档，仍回 200」⇒ 代码与自己写定的对外契约相反。修法沿用 A375 口径：删除类走 `missing_ok=True` 并摘掉预检，读取类把 `stat` 与紧随其后的整份读一起罩进 `except FileNotFoundError`，缺失按该函数**自己的**诚实读数计（`exists: False` / `valid=False, error="文件不存在"`）。守卫与三面见 `tests/unit/test_missing_file_family_l281.py`。 | ~~M~~ **已收口（L281）** |

| A378 | `augmentor/atomic_write.py` 的 `finally` 清理步 | **新立并同轮收口（L281）**：`finally` 里那句裸 `tmp.unlink()` 管的是「临时件还在」，但它自己也会失败——Windows 上杀软或索引器暂占句柄时 `unlink` 抛 `PermissionError`，而 **`finally` 里抛出的异常会顶替上面那次真正的写失败**，于是模块 docstring 承诺的「异常原样上抛」被换成一句不相干的清理报错，调用方按「清理失败」判决，真正的「写失败」凭空消失。修法：`_discard_tmp()` 尽力清理，`missing_ok=True` 管「已经不在了」，`except OSError` 管「删它这件事失败」并 `logger.warning` 出声不顶替；留在盘上的临时件不匹配任何目录扫描（前导点 + `.tmp-` 中段），代价是垃圾而不是坏答案。行为面由「临时件位置被一只目录占住」这一必现臂钉住（写失败与清理失败同时发生 ⇒ 报出去的必须是写失败、`__context__` 为空），另附一支改前同形复现证明顶替真的会发生。 | ~~M~~ **已收口（L281）** |

| A379 | `tests/unit/test_missing_file_family_l281.py::product_sources`（L281 那把全仓普查尺的语料根声明） | **新立并同轮收口（L282）**：尺子口径缺陷，不是产品缺陷。L281 把语料根写成 `("augmentor", "api", "cli")` 三棵目录，而 `cli` 在仓库根的真形是单文件 `cli.py` ⇒ 旧实现 `if not base.is_dir(): continue` 让那一格静默跳过，「全仓 133 只产品文件」从此少一只，而①面那条零命中是在**一块缩了水的面**上量出来的——缺的那一只恰好不在任何子树里，任何「按子树数」的对照都看不见它。危害形状与 l177 那把尺的 docstring 警告的同一族：扫描面缩水会把「量不到的 0」重新变成假绿。三条修：① 语料根拆成 `PRODUCT_DIRS`（目录）与 `PRODUCT_FILES`（单文件）两组声明；② 每一根逐根验真，形状对不上就 `AssertionError`，禁止 `continue`；③ 补一支**跨尺集合相等**对账（与 l177 的 `_PRODUCTS` 比路径集合而不是比规模数）。修面现量 133 → **134**，`cli.py` 在该尺上 0 命中 ⇒ L281 的十格结论未被推翻。取证全在仓外：`Temp/probe_l282_face.py` 把常数拨回 133 在新面实跑得红读数 `assert 134 == 133`，同一份探针量出 `files=()` 复现旧面 133、两把尺集合相等 `True`。 | ~~S~~ **已收口（L282）** |

| A380 | `augmentor/excel_write.py::export_table_to_excel` 的 `finally: wb.close()`（全仓 `finally` 裸清理普查量到的唯一一格） | **新立并同轮收口（L282）**：A378 那一族（`finally` 里抛出的异常顶替上面那次真正的失败）此前只按名册管着 `atomic_write` 那两处，全仓没量过。L282 用修好语料面之后的那把尺普查 134 只产品文件（新尺 `finally_cleanup_hits`，只认 `unlink/remove/rmtree/close/rename/replace` 这一族动词，`missing_ok=True` 与自罩 `try` 两道豁免），读数**恰好一格**就是本行这一格。**但这一格未坐实成缺陷**：openpyxl 的 `Workbook.close()` 写成 `if hasattr(self, "_archive"): self._archive.close()`，仓外探针 `Temp/probe_finally_close.py` 照产品骨架（`try: … wb.save() … finally: wb.close()`）实测——`wb.save()` 因目标是一枚目录而抛 `PermissionError` 时 `close()` **静默**（读数 `ARM_SAVE_FAIL_DIR: close_silent` / `top:PermissionError` / `ctx:None`），save 成功后的 close 与重复 close 也静默 ⇒ 没有「真正的失败被顶替」这一档。按「不为不会发生的情形加兜底」**不动产品码**，改把这一格钉成守卫里的豁免名册（逐格点名：文件 + 函数 + 调用文本），配四档自证臂与一支「豁免格仍量得到」防空集对账；产品码一改就红，别处新长出一格同样红。 | ~~S~~ **已收口（L282）** |

| A381 | `api/routes/status.py::get_status` 的降级分支（本轮 L283 给它补了日志，**没动状态码**） | **新立（L283）**：后端不可用时那一格照旧回 **200**，`status` 仍是 `"ok"`，只在体内把 `model_default` 写成 `"unknown"`、`model_available` 写成 `False`。本轮坐实的是「不出声」那一半（原文现在进服务端日志，`tests/unit/test_status_degrade_logs_l283.py` 三面钉住：恰好 1 条 WARNING / 原文不进响应体 / 健康路径零 WARNING），**状态码语义一半未动**——它是对外契约：`HealthResponse` 与 `StatusResponse` 的键集已由 `test_doc_contract_numbers_l200` 与卷一 A81 钉着，探活脚本与前端都按「200 即可用」判。**修法方向**：① 降级时回 **503**（最贴语义，但改对外契约，且与 `docs/augmentor/API.md` 的回执样例同批动）；② 或 200 不变、体内补一格 `degraded: true`（响应模型加字段 ⇒ 键集尺子随之重钉）。与 **A280**（`VersionError` → 400 还是 500）同属「状态码语义」一族，宜同批处置。定级 M，待拍。 | M |

| A382 | `augmentor/atomic_write.py::_PATH_LOCKS`（锁表本身的增长维） | **新立并同轮收口（L283）**：L273 为「丢更新」那一族立了按路径的锁表，但表本身**单调增长、无上限、无回收**，而 `path_lock` 的 docstring 写着「进程碰过的文件数量有界」——这句话在这仓里没有 enforce 它的那段代码。键来自请求方能点名的数据集文件名 ⇒ 上界必须由代码给。实测（`Temp/probe_l283_locks.py`）：20 000 个不同路径登记 **6.25 MB**、每格约 **312 B**，把路径对象全部丢弃、文件也不存在之后**一格都不退**。修法：`MAX_PATH_LOCKS = 10000` + 超限之后所有新路径共用一把 `_SPILL_LOCK`——与 `api/middleware/rate_limit.py` 的 `DEFAULT_MAX_KEYS` 同一取向（**封顶而不是回收**）。为什么不回收：eviction 必须在「取出锁」与「拿到锁」之间开窗，A 取到锁对象还没进 `with`、B 删掉这一格、C 给同一路径建出第二把 ⇒ 两边并发写同一份文件而两边都回 `success: true`，正是本族要治的形状；共用 spill 锁只会**多**串行化、永不放宽，且保住 `test_rmw_serialization_l273` 钉住的两条（上限之内同一路径 `is` 同一把、返回类型是真 `threading.RLock`）。新守卫 `tests/unit/test_path_lock_registry_cap_l283.py` 6 支：常数钉 / 真上限跑到 cap+500 断表长恰等 cap 且溢出格不进表 / 相对-绝对-str-Path 四形同锁 / monkeypatch cap=3 验共用与可重入 / 溢出路径仍串行化（主线程按住 spill 锁，另一支 0.15 s 内进不去）/ 注入臂把 cap 顶成 `sys.maxsize` ⇒ 60 格全登记、各得新锁。autouse fixture 快照并恢复 `_PATH_LOCKS`，防跨用例污染。定级 M，本轮按用户 goal 授权直接收口。 | ~~M~~ 已收口（L283） |

| A383 | `augmentor/version_control.py::DatasetVersionManager.get_history` 的 `limit` 判据 | **新立并同轮收口（L283）**：A77 那条口径（「`x if limit else …` 把『要 0 条』读成『没传参数』；负数走切片换语义」）在 `versioning.py:86` 早已收口并留了注，`version_control.py` 这一格是**同族的漏网孪生**——该类是 `augmentor/__init__.py:65` 的公开导出、CLI `version history` 的后端（`augmentor/cli/commands/version.py:57`）。改前实测（3 条 history 夹具）：`limit=0` 交出**全部 3 条**、`limit=-1` 交出 `[{'version_id': '0'}]` 即「丢掉最新一条」，两种都是形状合法、内容错误的静默答案，调用方无从分辨。修法按既有权威共引，不另立一套：`require_count("limit", limit)`（负数当场 `DataValidationError`）+ 回落条件从 `if limit` 改成 `if limit is not None`。**落点代价**：净 +1 行把同文件第 159 行下移一格 ⇒ 卷一那一处活引用 `version_control.py:159 → :160` 按处方 3 同行重钉（净零行）。配套臂加在常驻族守卫 `tests/unit/test_count_knob_validation.py` 的 `TestHistoryLimit` 里（`0` 与 `-1` 两个错答同一格钉，`limit=2` 的顺序与 `limit=0` 的空页逐条对照），该文件 **237 passed**。注入取证两臂：**V1** 摘掉判据 ⇒ `Failed: DID NOT RAISE DataValidationError`（1 failed / 236 passed）；**V2** 把回落条件恢复成真值性判据 ⇒ `assert [{'action': '…version_id': '0'}] == []` 在 `:466` 红。两臂复原后 sha `b268e921bc94b8f0` 与进入时逐字节相等。定级 M，本轮按用户 goal 授权直接收口。 | ~~M~~ 已收口（L283） |

| A384 | `api/routes/config.py::update_config` → `augmentor/config.py::save_config` 的并发与注释代价 | **新立（L283）**：本轮按 L273 同法把并发打到配置面，读数（仓外沙箱 `Temp/l283_conc/`，20 支并发各持一只 `TestClient` ⇒ 各一支事件循环）：**16×200 + 4×500**，500 是 `atomic_write._replace_with_retry` 的 `os.replace` 撞 Windows `WinError 5`（拒绝访问）**顶穿了 5 次 × 20 ms 的退避上限**；磁盘终态是一组整批丢——`quality.threshold` 落成 0.116 而 `dedup.threshold` **停在原值 0.9**（该组 10 支的写入全被抹掉），单发两次都 200 且 `ignored_keys: []` ⇒ 排除「那个键本就被拒」。第二次读数是代价面：一次**成功**的配置写入把仓档 `config.yaml` 的 **206 行注释 / 共 457 行**重写成 **0 行注释 / 186 行**（`sha 79c18e44c614 → 160de1281ef0`），而 A152① 只挡「空写入不碰文件」，`yaml.safe_dump` 不回写注释这一半从未变过——那些注释里住的是判据本身。**为什么本轮不自决**：`update_config` 的「改内存 → 落盘」同步区内**没有 `await`**，单事件循环部署下两支请求结构上无法交错，这组读数出自线程并发（每线程一循环）与多进程部署，正是 `mutate_json_file` docstring 明写的「只管同进程：多 worker 那一档记在账本里待拍」同一格，也与 **A156 / A158** 的跨进程口径重叠。修法三选一：① 给 `save_config` 罩 `path_lock(config_file_path())` 并把整段改-写放进同一把锁（治线程面，治不到进程面）；② 跨进程文件锁（新增依赖或自造 `msvcrt.locking`，Windows/POSIX 两形要分别实测）；③ 把配置写入收敛成「只允许单写者」并在启动时按 `workers>1` 出声拒绝。注释那一半另需 `ruamel.yaml` 类的 round-trip 保留，是独立一笔开销。**都不属「修了不会改对外语义」那一类**，故只立账。定级 M，待拍。 | M |

| A385 | 成功路径的回显仍会被孤立代理打穿：`api/routes/dataset.py::search` 把请求里的 `query` 原样嵌进响应文本 | **新立（L284）**：本轮的错误面兜底只管 422 与 4xx 两支处理器，而**成功 200 的回显不走它们**。实测（仓外沙箱 + `AUGMENTOR_API_KEY`）`POST /api/dataset/search` 的 `query` 送一枚 U+D800 ⇒ 校验过、路由跑完、在**序列化响应**这一步崩：`pydantic_core._pydantic_core.PydanticSerializationError`，栈顶 `type_adapter.py:651 dump_json` ⇒ 对外一支裸 500，且 ⑤ 那两层兜底结构上接不到它。**为什么不本轮自决**：三条接法都改对外语义——① 换 `default_response_class` 让成功面也消毒，代价是「服务端自己算出 NaN / 非法字符」这一族缺陷从此静默（与 A77 真值性判据族相反的方向），② 在端点里先清洗入参，等于把回显承诺改掉（文档明写回显查询词），③ 只把响应体那一格从 `str` 换成受检类型，改动面跨 `dataset.py` 与模型定义。定级 M，待拍。 | M |

| A386 | `api/routes/vector.py::semantic_search` 把内部异常原文当 400 判决送给客户端 | **新立（L284）**：同一枚 U+D800 打到语义检索 ⇒ 400，`detail` 逐字是 `'utf-8' codec can't encode character '\ud800' in position 0: surrogates not allowed`。这是**框架/编码层的内部消息**被当业务判决外发：① 对客户端泄露实现细节（编码层、字符位置），② 与账本既有的「4xx 中文判决 / 422 英文判决」双轨口径（A306~A312 族）再开一个第三种形状，③ 判决文案随 Python 版本与 locale 变，不可钉死。**为什么不本轮自决**：接法是「在那条 `except` 里换成中文判决并把原文只落日志」，属对外文案变更且有既有断言按当前 `detail` 形状钉着，需拍「是否保留英文轨」这一维；与 L283 那格出声（A381）同族但更轻。定级 S，待拍。 | S |

| A387 | 纯 CLI 进程的默认日志出口未经消毒：配置不写 `logging` 节时 root 上没有任何 handler，走 stdlib 的 `logging.lastResort` | **新立（L285）**：L285 的消毒装在唯一的 Formatter 构造点上，因此 **API 面与装了 `logging` 节的 CLI 面**都被覆盖。两档对照实测（仓外探针 `Temp/l285_probe8_cli.py`，同一支子进程脚本只换配置文件，`--config` 指向仓外副本跑 `validate-config`，那份最小配置判负 ⇒ RC 1，但配置确实经 `load_config` 装过）：**无 `logging` 节**那档 root 上 `HANDLERS []`，载荷 `evil\n[2026] INFO ROOT admin login` 落成**两支物理行**，第二行逐字是 `[2026] INFO ROOT admin login`（`lastResort` 的裸 `%(message)s`）；**写了 `logging` 节**那档 root 上是 `[(StreamHandler, ControlSafeFormatter)]`，同一载荷落成一行 `ERROR evil\x0a[2026] INFO ROOT admin login` ⇒ 消毒确实按构造点生效，缺的只是「CLI 默认档根本没有 handler」这一格。**为什么不本轮自决**：补法要么在 CLI 入口无条件装一次 handler（**直接动 §3.32 与 `tests/integration/test_cli_logging_wiring.py` 那 17 支钉着的「14 条命令出厂默认档一字未改」主张**，默认档的级别与格式串都得重定标），要么给 `lastResort` 打替身（monkeypatch 运行期，判据反而变脆，且 `Logger.callHandlers` 是每次现场取属性、替身站不住）。载荷面是本地数据文件与本地文案，远程不可写，比 L285 那一格低一个数量级。定级 M，待拍。 | M |

| A388 | traceback 体内的控制字符仍原样多行：`Formatter.format()` 在 `formatMessage` **之后**才追加栈 | **新立（L285）**：消毒只覆写 `formatMessage`，而 `logging.Formatter.format` 的形状是 `MESSAGEFORMAT → formatMessage() → 若 exc_info 则 exc_text = formatException() 追加`。因此 `logger.exception` 打出的栈文本里若含 `\n`（本来就有）与控制字符（来自被记录的对象 repr、用户数据），仍是原样多行、原样 ESC。由 `test_traceback_stays_multiline` 把「栈仍多行」钉成**断言而不是遗漏**，防止下一个人把它当没消毒的 bug 顺手压平。**为什么不本轮自决**：把栈压成一行要覆写 `format()` 或在 `formatException` 后消毒，两条都改「Python 日志的标准可读形」这一对外可观测口径，而多行栈本身不构成注入——伪造一行需要「行首可控」，栈的行首恒为 `  File "..."` / `Traceback`，攻击者写不进一个 `INFO ROOT ...` 形状的顶层行。定级 S，待拍。 | S |

| A389 | 电子表格公式注入（CWE-1236）：导出与写入的四条边把前导 `=` 的字符串原样存成公式，接收方一打开就执行 | **新立（L286）**：L285 把 CWE-117 日志注入那一族收口之后，按同一条「不可信字符串跨过一个解释器边界」的线索把**交付面**过了一遍（只读普查，四形量到）——①`augmentor/export.py` 原生 CSV 支路（`csv.DictWriter` 原样写值）；②`augmentor/export_enhanced.py` 的 `_export_csv` / `_export_tsv`（同一形状，L286 只补了空集那一格、值面一字未动）；③`augmentor/converter.py` 的表格转换；④`augmentor/excel_write.py` 的单元格写入。实测读数：CSV 那一面 `=1+1` 与 `=SUM(...)` 形状的载荷**原样成文**（字节里就是那行文本，Excel / LibreOffice 打开即当公式解释）；XLSX 那一面读回的是 `<f>` 公式元素而不是字面串 ⇒ 两条路都会执行。`excel_write.py` 的文件头注释此前已经把「前导 `=` 的字符串两条路都被 openpyxl 当公式存（golden 读回 `None`）」写在案，并明说本函数**不**悄悄改写它、理由是「那是另一个契约」——这一格与那句注释是同一件事的两面：产品**已声明**行为，动它要拍的是对外可观测口径（导出物字节形状三选一：加前导单引号 / 强制文本类型 / 直接 400 拒收，三种都会改变交付字节，且第三种还会新增一档拒绝语义）。另有一条已知约束留在案上而不是埋在散文里：载荷面是数据集里的 `instruction` / `output` 字段，任何能写数据集的人都能种，因此它不是「只有联网才够得着」的远程面，也不是「只有恶意管理员才做得到」的本地面 ⇒ 判级 S 但**待拍**，四边一处未动，本轮只把面与读数留在账上。 | ~~S~~ **已收口（L287）** |

| A390 | 电子表格解压放大（zip bomb，CWE-409）：读 Excel 的两条入口只受「压缩后字节上限」管辖，解压后的尺寸与比率无人读 | **新立（L287）· 方向已由用户 2026-10-17 给定，实现未动**：只读普查量到 `augmentor/converter.py::_read_excel_source` 是 Excel 读边在本仓的唯一落点，两条入口（按路径的 `_read_excel` 与按内存字节的 `read_records`）都直通 `pandas.read_excel`；上游唯一存在的闸是 `api/deps.py` 里那道读 `web.max_upload_bytes` 的判据（默认 256 MiB，量的是**压缩后**的上传体），所以一份高比率的 zip 工作簿能让服务端在内存里展开成远超那个上限的工作表。**为什么本轮不动**：用户给的方向是「按解压比 + 解压后尺寸拒」，但两个阈值住哪里（配置键还是常量）、拒时报哪一档错（沿用 `DataFormatError` 还是新立一档）要连着**现场量到的比率读数**一起定，而本轮没有做真实炸弹的现量 ⇒ 刻意不落一个未量的读数，量比率与实现留给下一轮（同族先例是 L286 那一格：只把面与读数留在账上）。定级 M，待实现。 | M |

| A391 | CSV / TSV 的**表头**那一面未转义：列名同样会以会被执行的前导交付给接收方，而给它加前缀会改变读回侧用来配行的键 | **新立（L287）· 刻意不做，待拍**：A389 按拍板① 落地时判据只用在**值**位上，这是分工而不是漏——`validation.sheet_text_row` 的 docstring 明写「键一字不动」，原因是 `csv.DictWriter` 的表头与读回侧的键是同一把尺子，给表头加前导单引号会让写出去的文件在自己的读边上配不上行（形状：写出一个以引号开头的列名，读回来的记录键就是那个带引号的名字）。xlsx 那一侧相反，表头格子已经被 `text_only_cell` 强制成文本、一个新字符都不加（新守卫第 3 组第二条现量），所以这一格只关 CSV / TSV 两条文本容器。**候选口径**：① 表头与值一起加前缀、并在读边把前缀剥回来（两处同步，改既有读形状）；② 拒收「列名里出现会被执行的前导」（新增一档拒绝语义，改对外面）；③ 维持现状，把「列名不可信」写进文档与页面提示。三档都动对外可观测面 ⇒ 待拍，本轮一格未动。定级 S。 | S |

## Backlog B — 功能增强与体验（价值 ÷ 工作量）

| # | 内容 | 证据 | 量级 |
|---|------|------|------|
| B201 | **已关闭（L131）**：`DatasetAnalyzer.get_duplicate_candidates`（SDK 公共方法）O(n²) 里逐对重建字符集是真热点 | 实测 6000 条 **71.8 s**；预计算后 2500 条 A/B new/old ×0.36 / ×0.34（双序），等价性 5 阈值逐元素一致 | ~~S~~ 已关闭（L131） |
| B202 | **已关闭（L132）**：`ActiveLearningLoop._diversity_scores` 逐对调用 `compute_similarity`，每对把两侧重新 tokenize，n 条共约 2n² 次切词 | 预计算词元集合后 n=800 A/B new/old ×0.088（≈11×，双序），等价性 4 数据集（含重复/空串/单条/多词）逐元素一致 | ~~S~~ 已关闭（L132） |
| B203 | **已关闭（L133）**：`DataCleaner._normalize_punctuation`（`normalize_punctuation` 规则）此前无任何直达行为用例；候选「14 道 replace → str.translate」优化经 A/B **证否**（长文本稀疏匹配 ×8.7 变慢）故不改产品，改补首个行为守卫 | A/B：重匹配 ×0.87、但真实场景（长文本少量全角）translate 逐字符查表 ×8.7 慢于 replace 的无命中短路；新增映射表 14 项 + 顺序 replace 参照逐字符等价的 3 用例 | ~~S~~ 已关闭（L133） |
| B204 | **已关闭（L134）**：`ContextAugmentor.generate_multi_turn` 在「每轮 × 每条历史」里各自重建 user 问题列表再 `in` 线性查，且重复判定散在循环内不易推理；循环不变量 `existing_histories` 被反复重算 | 改为循环外一次性摊平成单一 `existing_questions` 集合（并集语义），内层 O(1) 命中；新增 2 条跨多条历史的去重用例钉死并集语义 | ~~S~~ 已关闭（L134） |
| B205 | **已关闭（L135）**：`DataSanitizer.remove_duplicates(keep="last")` 每次遇到重复都在**全 result 上**逐条 `r.get(key)` 线性找槽位，O(n×distinct)；重复多的数据上显著热点 | N=40000/D=400 实测 new/old **×0.0235**（≈42×，双序 min-of-2）；seen 改存 result 下标、O(1) 覆盖；等价性 300 dup 密集集 + 50 keep=first 集逐元素一致；1 条新槽位不变量用例 | ~~S~~ 已关闭（L135） |
| B206 | **已关闭（L136）**：`EnhancedComparator._compare_fields` 的 instruction 匹配是 O(F×A×B) 逐条扫 B、每字段再各扫一遍 A/B 算 count；且 `type_mismatches`/`value_differences`/`diff_datasets` 成员语义无直达行为用例 | 双层索引 `instruction→{field:首个含该field的item_b}` + 一次 O(A+B) 字段计数，匹配变 O(A)；N=1500/F=12 A/B new/old ×0.097；等价性 200 随机数据集逐字段逐差异一致；补 3 条行为用例 | ~~M~~ 已关闭（L136） |
| B207 | **已关闭（L137）**：`AugmentorPipeline._process_single_item` 的 except 分支里 `_process_errors` 惰性「没有就建 + append」非原子——并行路径（ThreadPoolExecutor 多工作线程）并发触发时，两线程都读到 `hasattr` 为 False 各自建 `[]`，后建者吞掉先 append 的那条；而 `__init__` 里的 `self._lock` 建了却全程无人用（dead lock） | 持 `self._lock` 包住 check-then-act；补 `TestProcessErrorsThreadSafety` 2 条（并发 N 条错误一条不丢 + 源码级守卫钉死锁确实被用上，防未来把锁拿掉却因 GIL 窗口窄测不出） | ~~S~~ 已关闭（L137） |
| B208 | **已关闭（L138）**：`ActiveSampler.generate_report` 里 `recommended_seed_indices` 用 `items.index(seed)`（== 相等）取「seed 的下标」，内容相等的兄弟 item 会误报第一个的下标；候选「改 id(seed) 索引」优化经 A/B **证否**（真实 k 很小，O(n) 建表 > per-seed 2×O(n) 扫描，反而更慢）故不改产品，改补下标精确性守卫 | A/B：n=3000/50 seeds 下 new(id-map)/old ×3.56（变慢）；新增 1 条钉死「下标＝被选中对象本身的下标，不是内容相等里的第一个」的守卫 | ~~S~~ 已关闭（L138） |
| B209 | **已关闭（L139）**：`EnhancedComparator._generate_recommendations` 的 4 条字符串分支（size_diff>±100 / 相似度<0.5 / 类型不匹配 / 值差异）此前**无任何直达断言**——既有用例只 `assert "recommendations" in d`（key 存在），全分支实际盲跑 | 补 3 条字符串守卫：size_diff=150⇒「数据集A比B多 150 条数据」；similarity=0.1⇒「相似度较低」；type_mismatches=2/value_diffs=1⇒各一条「字段 X 存在 N 个…」（含「高度相似」顺带路径） | ~~S~~ 已关闭（L139） |
| B210 | **已关闭（L140）**：`turns_to_canonical`（chatml/vicuna/sharegpt 逆运算）末尾两轮角色检查用裸 `["role"]`，缺 role 键时抛**裸 `KeyError`** 而非契约承诺的 `DataFormatError`（函数 docstring Raises 段明写抛 DataFormatError）；且该逆运算此前**无直达单测**（只在集成 CLI 用例里被间接触发） | `["role"]` 改 `.get("role")`（缺键 → 比对 `None` ≠ "user"/"assistant" → 走既有 DataFormatError 分支，顺带把 f-string 里的裸引用一起改）；补 `TestTurnsToCanonicalContract` 2 条（缺 role 键 / 角色顺序错），实证改前 KeyError 红、改后 DataFormatError 绿 | ~~S~~ 已关闭（L140） |
| B211 | **已关闭（L141）**：`VersionManager.diff()` 用 `(instruction, output)` 组合键做身份 ⇒ 「原地改某条 output」被拆成「删旧+增新」两条，`modified` 分支**恒为空**（`modified_count` 永远 0），而 `DiffResult` 字段注释与 docs/augmentor/API.md 示例都承诺 `modified_count` 非零；既有唯一守卫还是空转的 `assert modified_count >= 0` | 身份键改成 `instruction` 单键，同 instruction 两版都在 ⇒ 整条 dict 不等即计入 `modified`；原地修改 now 报 modified=1/added=0/removed=0。既有 4 条 diff 用例（added/removed/identical/empty）全绿无回归；强化 `test_diff_modified_items` 断言 + 新增 1 条原地修改守卫 | ~~M~~ 已关闭（L141） |
| B212 | **已关闭（L142）**：`QualityTrendTracker.compare_trends()` 的 `comparison` 用「A > B else B_higher」二态判据——**平局**与「某侧根本没记过该指标」都误报 `dataset_b_higher`（缺指标被静默当 0 比，语义错）；既有唯一用例只钉了 A>B 一条 | 改成三态：双侧都有最新值且 A>B ⇒ `dataset_a_higher`，B>A ⇒ `dataset_b_higher`，其余（含平局、任一侧无记录）⇒ `tie`；补 3 条用例（平局 / 双侧缺指标 / 单侧缺指标）钉死 `tie` | ~~S~~ 已关闭（L142） |
| B213 | **已关闭（L143）**：`ActiveSampler.identify_underrepresented` 把 `length_distribution` 的全部键都当占比桶遍历（`.items()`），但 `_analyze_length_distribution` 会额外塞一个 `avg_length` **均值**键（绝对值，非占比）；当 `avg_length < threshold` 时会误产出伪桶 `length:avg_length`，污染 `recommend_seeds` 的 underrepresented 列表并白占一个 top_k 名额（其匹配循环因没有该 bucket 分支而永远打不中） | 长度桶改为显式只遍历 `("short","medium","long")`；`avg_length` 不再是候选。等价性 200 随机数据集 new⊆old 且排除 avg_length 全过；新增 1 条守卫（空指令 avg_length=0 < 0.1 时不产出 `length:avg_length`），改前红/改后绿 | ~~S~~ 已关闭（L143） |
| B214 | **已关闭（L144）**：`DatasetOperations.merge()` 的 `max_items` 截断用 falsy 判据 `if config.max_items and ...`——`max_items=0` 语义是「一条不留」，但 0 被读成「不限」而返回全量（`merge_files` 委托 `self.merge()`，同一截断点一并覆盖） | 判据改 `is not None` 并加注释钉住 falsy 回归防护；补 `test_merge_max_items_zero_keeps_none`（改前 0 返回全量，改后 0→0 条；对照 None→全量、3→3 条防修过头） | ~~S~~ 已关闭（L144） |
| B215 | **已关闭（L145）**：A184 引用普查的索引跳过集只有 .git/__pycache__/.pytest_cache/node_modules——测试套件自写的运行时快照目录 tests/.backups/（gitignored，门禁的 fixture 运行期写入 index.json、snap.json、opt100_progress.md）进索引：跑过一轮全量门禁后，index.json、snap.json 这类文件名有多候选，下一次门禁普查 9 条引用被翻 live→ambiguous（336 vs 钉值 327），棘轮测试红——门禁状态依赖、自伤 | 快照目录加入索引跳过集（语料面跳过集一直有、索引面漏配；L79 的 build_index 只收 .py 不受影响）；补 1 条守卫（改前红：3 条泄漏候选；改后绿）；按「动案面要重跑普查再改这里」条款逐格重钉：scratch_missing 289→302（排除快照目录候选后 13 条翻进本档，逐格归因），ambiguous 327 与其余各档原地未动；幂等性实证：普查含/不含快照目录逐格同读 | ~~M~~ 已关闭（L145） |
| B216 | **已关闭（L146）**：QualityTrendTracker 的趋势历史文件加载失败（JSON 损坏、`trends` 非列表）时**静默置空**历史并只告警一条；随后记录指标触发的保存会用 `open` 写模式**截断覆盖原文件**成「仅含新条目」——原始数据永久丢失、零备份；且原实现不校验 `trends` 类型，它是字符串时下一步追加直接崩 | 畸形文件先备份为带时间戳的损坏副本再置空（同秒重名自动加序号；Windows 句柄被占用挡掉 rename 时自动退化成复制式备份）；两条备份路径全断才置禁写标记、保存跳过（原文件逐字保留、内存历史不丢）；`trends` 非列表改抛领域异常 DataFormatError（对齐裸内置异常 raise 守卫）；新增损坏防护测试类 3 条，改前红 3 failed（旧版文件沙盒实证）/ 改后绿 | ~~M~~ 已关闭（L146） |
| B217 | **已关闭（L147）**：增强导出器的条数限制分支写成 falsy 判据——max_items=0 语义是「一条不导」，但 0 被读成「不限」而导全量（与 L144 的 merge() 缺陷同族，全仓漏网的最后一处） | 判据改 `is not None` 加 `>= 0`（负数仍读「不限」维持旧行为，防修过头）；补 1 条回归（0→0 条；对照 None→全量、负数→全量），改前红（实测导了 3 条）/ 改后绿 | ~~S~~ 已关闭（L147） |
| B218 | **已关闭（L148）**：merge_files 的 `removed_duplicates` 统计把「去重删除数 + max_items 截断数」混报成一个值（`total_input - total_output`），用户读到的「去重数」在带截断的配置下虚高（实测 8 条入、去重 2、截 3：旧报 5 新报 2） | 在 merge_files 内部用既有 _deduplicate 复算去重删除数、截断数 = 去重后基数 - 输出数，两数分报（新增键 `truncated_by_max_items`，既有键语义改为只记去重）；实现刻意控制在 242 行参考点以下净增 8 行——历史文档对 dataset_ops.py:242/:243 的行引用平移后仍落在代码行（棘轮 58/58 绿），首版把统计逻辑放进 merge() 净增 15 行、把 :243 翻进空行致 A184 硬 0 红，已回退换点；补 1 条回归（去重+截断双开分报 + 无去重无截断全 0 对照），改前红/改后绿 | ~~M~~ 已关闭（L148） |
| B219 | **已关闭（L149）**：sample() 的比例分支 `elif config.ratio:` 用 falsy 判据——显式 ratio=0 语义是「采 0 条」，但 0 被读成「未设置」直接落 else 采全量（与 L144 max_items、L147 导出 max_items 同族，本处是 ratio 位） | 判据改 `is not None`（1 行替换 + 2 行注释，插入点在被历史行引用的 242/243 之下，棘轮不受扰）；补 1 条回归（ratio=0⇒0 条；对照 None⇒全量），改前红/改后绿 | ~~S~~ 已关闭（L149） |
| B220 | **已关闭（L150）**：_deduplicate 的 threshold 形参从不被读——`MergeConfig.dedup_threshold` 文档承诺「去重阈值」、auto_config 按重复率逐条推荐数值并写理由（「上调/下调去重阈值」），但任何取值结果都相同（死旋钮，全仓 0 消费点） | threshold 分档消费：≥ 0.9 只删 instruction 完全相同（原默认行为，保守档），< 0.9 追加宽松键（大小写折叠 + 全空白删除）把近似重复一并删；字段注释与 docstring 同步改写；宽松键归一化首版用单空格 join（多 token 折叠回一个空格、与无空格形态仍不等，测试当场抓出），改全空白删除。auto_config 的推荐数值（0.85–0.98 区间）自此真有其效：高重复率→0.98 保守档、低重复率→0.85 宽松档，方向与其既有理由文字一致，零改动。补 2 条回归（宽松/保守档对照 + merge 层配置真消费），改前红 2 failed/改后绿 | ~~M~~ 已关闭（L150） |
| B221 | **已关闭（L151，收口第一本账 A125）**：两个组件构造器的阈值判据各缺一半——Deduplicator 的 `threshold < 0 or threshold > 1` 对 NaN 双假放行（去重整条静默零动作）、对 True/False 读成 1.0/0.0（False 实测误删 2 条真数据）、对 None/非数值延后到比较处炸 TypeError；QualityScorer 七档坏值（None/字符串/NaN/bool/越界）全放行、判负迟到 score()。配置层（DedupConfig/QualityConfig）L76 起就拒，组件层 15 轮未收 | 两组件阈值统一接 `require_ratio`（界与配置侧共引 `DEDUP_THRESHOLD_RANGE` / QUALITY_THRESHOLD_RANGE，A77 同式）+ None 显式拒；Deduplicator 仍抛 DedupError 且文案不带键名（消费侧公开契约由 L76 钉子钉住）。L76 两条「洞本体」钉子按 docstring 红字约定翻转（改前红 11 / 改后绿），新增 QualityScorer 阈值守卫 11 例 | ~~M~~ 已关闭（L151） |
| B222 | **已关闭（L152，收口第一本账 A46①）**：`create_stream_processor(processor_func, chunk_size=1000)` 的 `chunk_size` 形参声明了从不被读——工厂闭包逐条处理、无状态，块边界无任何可观测后果，「设了没生效」是静默的，且全仓 0 调用点 | 删除形参（结构性填不了⇒删，不发明语义）；补签名守卫（断言形参不在）；更新钉死该形参的既有调用。A46②（StreamConfig 两字段断线，产品内 0 使用者）留档后续轮 | ~~S~~ 已关闭（L152） |
| B223 | **已关闭（L153，收口第一本账 A124）**：`quality.weights` 的形状判据权威住组件（`QualityScorer` 两行手抄只判长度与和）——配置层只判 null，两面对同一键不同判；三档洞：`'abc'` 长度恰 3 死在 `sum()` 的 TypeError、`[-1.0, 2.0, 0.0]` 静默放行（权重符号反了排序整个反过来不出声）、`[True, True, False]` 被 sum 当 `[1,1,0]` | 判据权威搬进 validation 族（新族员 `require_ratio_list`：逐项 bool/NaN/越界 + 长度 + 和=1±0.01），配置层 / 组件层 / 静态回放三面共引同一份（A77）；`quality.weights` 进静态规格表、L76 的「唯一豁免」钉子按 docstring 红字约定翻转为「规格在场 + 坏值两面同拒 + 合法 6/6 两面放行」；`[]` 由静默回落默认改为即拒。新增 6 条组件层守卫 + 翻转 1 条，改前红/改后绿 | ~~M~~ 已关闭（L153） |
| B224 | **已关闭（L154，收口第一本账 A140）**：A140 余 11 节 29 键（context / versioning / sampler / expander / tracker / visualization / multilingual / evaluation / benchmark / active_learning / frameworks）四面零反馈——运行时零判据（一个节内判据都没有）、静态规格表零行、不可经 API 写入 ⇒ 坏值全静默进加载面；三节有真读者（pipeline 直读 num_turns / storage_dir / auto_snapshot，CLI 基准子命令读 metrics / baseline_file），假零 / 字符串假真 / 标量拆字符等档症状要到消费日才出声 | 11 节整批接节（同式同族一批收掉，A140 原框「单节约等于 S」）：节内判据引 validation 族既有成员（12 布尔键、4 计数键取正、5 字符串键、8 清单键、null 全拒），不新造判据不新造常数；静态面补 11 节 dict 行 + 29 键规格行与运行时同档；加载面构造器自动吃到。新守卫 8 类（普查名单翻转 + 坏值 × 面矩阵 + 合法反向 + 删规格行独立性 + 加载面 + 消费面）；l83 分档 13 键移档、validator 两计数扩员、l73 棘轮第三次翻空 + 1 用例按其 docstring 明文处方退役（假臂无来源 ⇒ B225 留档）；行引用 4 条降名锚 + L79 / A184 / l97 逐格重钉 | ~~M~~ 已关闭（L154） |
| B225 | **已关闭（L155，留防御支 + 钉死其契约）**：`apply_section_update` 的假臂（节无节内判据时「写直通、不复查」那一支）自 L154 全 20 节接上判据起仓内无来源 | 处置二选一拍定为「留」：写入路径拿到的是任意 dataclass 节对象，无判据节（含用户自定义节）是合法入参，删臂 = 对外行为变更；l73 新增 `TestTheUngatedWriteThroughIsAPinnedContract`（2 例）把「写直通、不吞键、不做判决（有判据节会拒的 0 / 负数照落）」钉成契约，谁删臂或顺手接判据就红。测试-only 一轮，产品码零改动、无行引用位移 | ~~S~~ 已关闭（L155） |
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

- **A208 = 「FAQ + 端点人读字段说明」**。既有 `docs/augmentor/API.md` 第 1~7 节已逐端点给出
  响应示例，缺的是两块：跨端点复现字段的**人读语义**、以及 REST 面特有的**排查**。
- 新增 `docs/augmentor/API.md` **易混字段词典**（9 行表，插在「响应契约」后）：只收字面直觉
  与实际语义不符的字段，逐条标注陷阱，字段名以 `api/routes/*.py` 的 `*Response`
  schema 为准（读源核过、未手抄）——`pass_rate` vs `avg_score`（门禁读前者）、
  `effective_weights` 重归一化、`semantic_evaluated` API 面恒 false、`duplicate_groups`
  是组数非明细、`removed_count`/`dropped_count` 不同名是既有契约、`is_valid` 空集
  恒真（A92）、`overall_passed` 默认不判负（A91）、`metrics`/`all_metrics` 之别。
- 新增 `docs/augmentor/API.md` **§11 排查（API 面）**（6 问）：路径白名单 400、写入不自动建
  目录、空集 is_valid=true、报告端点如何让 CI 变红、并发 progress 串号、
  X-Process-Time 单位；只收 REST 面特有坑，通用依赖问题指回 README §9。
- **A184 漂移连带修**：易混字段词典插在 API.md 中部（+约 18 行），把账本 L49 三处
  历史行号引用（API.md 加冒号 849 那种写法）顶到空行（dead_line 0→3）。按 A153/A127
  纪律«历史行号不是对今天文件的断言» 降成裸文件锚点 `docs/augmentor/API.md`（file_tokens 侧仍
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
  缺陷被 100% 盖住。`DiffResult` 字段注释（`modified_count  # 修改数量`）与 docs/augmentor/API.md:1139
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

| B254 | **已关闭（L184，A138 收口：可写四节 14 键「读者分档」记档 + 回显面机器钉死）**：export/vector/rag/multimodal 四节 14 键 L82 起全接判据（写坏即 400），但「写得进」不等于「有人读」——按现量分三档：① 有行为读者 3 键（export.default_format→augmentor/pipeline.py；rag.chunk_size 与 rag.chunk_overlap→augmentor/cli/commands/quality.py）、② 仅回显 6 键（GET /api/config 原样回显、不改行为）、③ 连回显都没有 5 键（vector.dimension/storage_dir/collection、multimodal.image/audio_extensions，无处消费）；契约面承诺能力、实现面不消费，用户配 vector.backend: faiss 会以为向量库在用 faiss（实际后端入口是调用参数） | 候选①（接消费入口=功能增强，要先拍「配置键 vs 请求参数谁是权威」）与②（降级=对外破坏面）均不在本轮；取**记档 + 机器钉面**：docs/augmentor/API.md 新增「可写四节的读者分档」小节（三档表 + 权威口径）；tests/integration/test_api_config_extended.py 新增守卫 2 例钉回显面——③ 档 5 键不得出现在 GET /api/config、①② 档 7 键是既定回显面不得被顺手摘掉（谁接了读者/回显都要先翻这一格）。A184 新增引用全用全路径 live 形、①②③ 键为点号键名（无文件扩展名、不进 REF）⇒ A184/L79 各棘轮原地绿；全量 7806 passed / 3 skipped / exit 0 | M |

| B255 | **已关闭（L185，A95/A122 合轮收口：模型凭证 ${ENV} 占位符展开语义钉成契约）**：模型条目三凭证键（api_key/secret_key/base_url）经 _resolve_env 解析 ${ENV} 占位符，**未设置的环境变量静默回落 ''**（与 None 等价，都是「这条没配凭据」的合法状态，由后端 ModelNotConfiguredError 在建后端时判）；A122 原框把「${ENV} 展开失败静默」与「三键零判据」留待拍，与 A95（展开出声口径）是同一件事的两半 | 拍定取**非破坏面收口（记档 + 机器钉死，不改加载期行为）**：加载期「环境变量未设置」条数随本机 shell 而变、接进加载声会让同一份配置不同机器行数不同（docs/augmentor/API.md 3.x 既定口径），故静默 '' 是**刻意契约**、出声归 validate-config 的 warning 通道；本轮把该语义钉死——augmentor/config.py 的 _resolve_env docstring 写明四档（已设置/未设置静默''/非 ${...} 完整形状原样/非串原样）；tests/unit/test_model_entry_defaults_l75.py 新增守卫 13 例（已设置取值 + 未设置静默'' + 坏形状 5 档原样 + 非串 4 档原样 + 合法占位未设置'' + MODEL_CREDENTIAL_KEYS 权威清单 == 三键）。l97 普查 CALLS 382→384 / READABLE 281→283 逐格重钉（两支 parametrize 字面清单进可读桶、盲面 101 不动、l98 随之回稳）；A184/L79 各棘轮原地绿（全路径 live 形引用）；全量 7819 passed / 3 skipped / exit 0 | M |

| B256 | **已关闭（L186，cache TTL falsy 假零收口）**：augmentor/cache.py 的 MemoryCache.set（:121）与 DiskCache.set（:268）用 `ttl or self._default_ttl` 把显式 0.0 读成「没传」而落到默认 TTL——但 0.0 本是合法「立即过期」值（CacheEntry.is_expired 对 ttl=0 恒真）；同族第三处在 DiskCache.get（:235）`if ttl and ...` 把读侧 ttl=0.0 读成「不过期」。三处 falsy 假零让「立即过期」这个档位静默失效 | 三处改 `is not None` 判型（set×2 + get×1，零值保留为「立即过期」、None 才回落默认/不过期）；tests/unit/test_cache.py 新增守卫 2 例（MemoryCache 与 DiskCache 各钉「ttl=0.0 立即过期 + None 回落默认」两侧对照）。CRLF 文件全程二进制写保真；A184/L79/l97 各棘轮原地绿（无散名引用、非参数化位、cache.py 插行使 0 条活行引用顶歪）；全量 7821 passed / 3 skipped / exit 0 | S |

| B257 | **已关闭（L187，benchmark delta_ratio 除零档自相矛盾收口）**：augmentor/benchmark.py 的 compare_with_baseline（:240）对基准读数为 0.0 的指标（pass_rate / diversity 合法读成 0）用 `if reference else 0.0` 记 delta_ratio=0.0——把「相对变化未定义（除零）」读成「无相对变化」，与同条目 improved/regressed 状态自相矛盾（baseline=0、current=0.5 判 improved，delta_ratio 却 0.0） | 改 `if reference != 0.0 else None`：reference==0.0 时记 None（与 :217 no_baseline 哨兵同口径），非 0 照旧相除；下游 Markdown 报告只读 current/baseline/delta/status（delta_ratio 不进表格）、无测试面读 delta_ratio ⇒ 零回归。tests/unit/test_benchmark.py 新增守卫 3 例（零基准 improved→None、零基准零当前 unchanged→None、非零基准照旧 0.8 防修过头）；A184/L79/l97 各棘轮原地绿；全量 7824 passed / 3 skipped / exit 0 | S |

| B258 | **已关闭（L188，PiiSanitizer 构造器 falsy 假零收口：fields/patterns/placeholders 三处）**：augmentor/privacy.py 的 PiiSanitizer.__init__ 三处 `x or default`——显式空容器（fields=[] / patterns={} / placeholders={}）本是「零元素」的合法意图，却被 falsy 读成「没传」而悄悄套上默认（脱敏范围 / 字段面 / 占位符表被无声放大）；最典型 patterns={}（「我只要这几条自定义规则、其余不脱」）被读成「用全量 DEFAULT_PATTERNS」 | 三处改 `x if x is not None else default`（空容器保留为「零元素」、None 才回落默认）；tests/unit/test_privacy.py 新增守卫 4 例（patterns={} 零模式不脱敏 + None 回落默认全量防修过头 + fields=[] 零字段 + placeholders={} 保留空表）。CRLF/LF 保真；A184/L79/l97 各棘轮原地绿（无散名引用、非参数化位）；全量 7828 passed / 3 skipped / exit 0 | S |

| B259 | **已关闭（L189，验证预设封闭清单下沉 SDK 直构面 + `rules` falsy 假零）**：augmentor/validation.py 的 DatasetValidator.__init__ 同格两缺陷：① `if rules:` 把显式 `rules={}`（「零规则」的合法意图）读成「没传」，静默落 preset 规则面——strict 档传 {} 拿到 basic 面（falsy 假零族，L144/L147/L149/L186/L188 先例）；② 未知 preset 静默回落 basic——拼错 strict/chat 被读成 basic，规则面整档放宽不出声（L175/L176 封闭清单族同型：API 面 400、CLI 面 argparse choices 早有收口，SDK 直构面独缺这一格） | ① 改 `is not None` 判型（`rules={}` 保留零规则、None 才走 preset）；② 回落支改抛 DataValidationError 并列出 PRESET_RULES 全部键（清单权威住 SDK 层，A77）；API 面 VALIDATION_PRESETS 由手抄字面改共引 `tuple(DatasetValidator.PRESET_RULES)`（L175 的 SEARCH_METHODS 共引先例，400 判据与文案不变）；CLI 面 parser.py 维持「不 import 业务模块、字面清单」既有惯例，同一性由新守卫钉；test_validation.py 旧「未知回落 basic」例红字重述为拒收例；tests/unit/test_validation_preset_closed_list_l189.py 新增守卫 10 例（rules={} 零规则判别探针 + None 回落 preset 浅拷贝防修过头 + 坏预设 5 档 parametrize 字面清单 + None 预设拒 + API 共引派生钉（源码级读法防回流手抄）+ CLI choices 与 PRESET_RULES 同一集合钉）；l97 重钉 CALLS 384→385、READABLE 283→284（盲面 101 不动）；A184 账本既有活行引用 validation.py:639 因 __init__ 插入 4 行注释档平移至 :697（修引用，非新增散名）；L79 原地绿。全量门禁：7838 passed / 3 skipped / exit 0（L188 的 7828 + 10 新守卫，无回归） | S |

| B260 | **已关闭（L190，leakage min_examples 计数旋钮收口）**：augmentor/leakage.py 的 LeakageDetector.__init__ 对 min_examples 零判据（`self.min_examples = min_examples` 原样入切片下标）：负数让 `[:n]` 切片换语义（`[:-2]` = 丢掉最后 2 条，「要 2 条」变「要 N-2 条」，silent 错答族；与 L144 的「取前 N 条」计数旋钮同式，此前漏在这一格），None / 非整数 / bool 原样进切片下标。0 是合法「报告不带示例」意图（与「没传参数」区分开，require_count minimum=0 口径） | 构造器加 `require_count("min_examples", min_examples, minimum=0)` + 显式 None 专判（L181 的 fuzzy_threshold None 专判先例）；tests/unit/test_leakage.py 新增守卫 13 例（合法 4 档 parametrize + 坏形状 8 档 parametrize + 0 档语义钉死：精确泄漏也零示例但统计照出）；l97 重钉 CALLS 385→387、READABLE 284→286（盲面 101 不动）；A184/L79/markdown-structure 各棘轮原地绿（B 表新行须与相邻行留空行，GFM 表格判据）；全量门禁：7851 passed / 3 skipped / exit 0（L189 的 7838 + 13 新守卫，无回归） | S |

| B261 | **已关闭（L191，sampler recommend_seeds 问题类型面 O(k×n) 重复分析→预计算）**：augmentor/sampler.py 的 recommend_seeds 在「每个覆盖不足类型 × 每条数据」的内层循环里对同一条 item 反复调 _analyze_question_type（纯函数、同输入同输出），k 个 question_type 覆盖不足档 = k 倍重复调用（98 条语料实测：top_k=1 付 2n+91 次、top_k=20 付 2n+186 次，差 95 次；修后两档同收在 3n=294 次） | 循环前预计算一次成索引数组 qtypes（O(n)），内层循环改 `enumerate` + `qtypes[i]` 查表（O(1)），语义不变（首匹配 seed 仍是数据里第一条匹配条目，守卫钉死 items[90]）；tests/unit/test_sampler.py 新增守卫 2 例（形状棘轮：同数据 top_k=1 与 top_k=20 两档调用数必须相同且 ≤3n——旧 k×n 形状当场分叉变红；+ 首匹配语义钉）；l121 docstring 的分支弧 `sampler.py 265->260` 因插入 5 行平移 `270->265`（红字重述）；变异自证：换回旧形 1 红（调用数分叉）、sha256 逐字节还原；l97/A184/L79/markdown 各棘轮原地绿（无新参数化位、无新增散名引用）；全量门禁：7853 passed / 3 skipped / exit 0（L190 的 7851 + 2 新守卫，无回归） | S |

| B262 | **已关闭（L192，cleaner 规则名封闭清单下沉 SDK 直构面）**：augmentor/cleaner.py 的 DatasetCleaner.clean 对未知规则名静默跳过（`for rule_name in rules: if rule_name in self._default_rules:`），拼错 normalize_whitespace = 该规则无声不生效、清洗报告照常出（checkpoint 症状族语义漂移档；L175/L176/L189 封闭清单族同式）。清单权威住 `_default_rules` 键（A77），CLI 面 parser.CLEAN_RULES 字面清单早有 argparse choices 收口，SDK 直构面独缺这一格 | clean() 入口补封闭清单判据（unknown = 非 str 或不在 `_default_rules`，一并拒；列出全部 11 个合法规则名），clean_dataset / clean_batch_optimized 经委托自动吃档；test_cleaner.py 新增守卫 5 例（未知名拒+全量文案 + 坏形状 4 档 + 合法名照旧生效防修过头 + 两条入口委托继承 + CLI 字面清单与 SDK 键同一集合钉）；test_partial_branches_l118 的旧「未知名跳过不报错」例红字重述为拒收例、docstring 勘误注 L192 假支不可达；l97/A184/L79/markdown 各棘轮原地绿（守卫无 parametrize、无新增散名行引用）；全量门禁：7858 passed / 3 skipped / exit 0（L191 的 7853 + 5 新守卫，无回归） | S |

| B263 | **已关闭（L193，迁移规则 id 封闭清单下沉 SDK 直构面）**：augmentor/migration.py 的 DatasetMigrator.migrate 对未知规则 id 静默丢弃（`r.rule_id in rules` 过滤，全拼错时迁移照跑、rules_applied 空数组，「迁移跑了个寂寞」不出声；L175/L176/L189/L192 封闭清单族同式）。API 面 system_ops.MIGRATION_RULES 早有 400 判据，SDK 直构面（migrate / migrate_dataset / migrate_file / CLI migrate）独缺这一格 | migration.py 加内置清单常量 `BUILTIN_MIGRATION_RULE_IDS`（A77 权威，守卫钉与 `_create_builtin_rules` 逐一对上）+ `migrate()` 入口拒未知/坏形状 id（列全部合法 id；自定义规则经 add_rule 注册后仍是合法 id，不修过头）；API 面 MIGRATION_RULES 由手抄字面改共引同一常量（is 钉，400 判据不变）；test_migration.py 新增守卫 5 例（常量追平内置规则 + 未知 id 拒 + 坏形状 3 档 + 自定义 id 放行 + API 共引 is 钉），旧「未知跳过不崩溃」例红字重述为拒收例；L193 顶部插 1 行共引 import 使 system_ops.py 全部行号 +1：OPTIMIZATION_LOOP.md 既有 6 处 `system_ops.py:147/:292×3/:397×2` 活行引用平移 :148/:293×3/:398×2（修引用，非新增散名）、docs/augmentor/ARCHITECTURE.md 2 处同移（:147→:148、:292→:293，L79 散文数字重钉随修引用回稳而非升档）；l97 原地绿（守卫无 parametrize）；全量门禁：7863 passed / 3 skipped / exit 0（L192 的 7858 + 5 新守卫，无回归） | S |

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
- **落点**：docs/augmentor/API.md 新增「可写四节的读者分档（A138）」小节（三档表 + 权威口径 + 非破坏面说明）；tests/integration/test_api_config_extended.py 新增 TestConfigEchoReaderSplitL184 守卫 2 例（③ 档 5 键不回显 + ①② 档 7 键既定回显面不得被摘）。
- **棘轮**：A184 新增引用全用全路径 live 形（augmentor/pipeline.py、augmentor/cli/commands/quality.py、tests/integration/test_api_config_extended.py）、①②③ 键为点号键名（无文件扩展名、不进 REF）⇒ A184/L79 各棘轮原地绿（全量门禁证实）；API.md 不在 L79 两文档扫描面、只进 A184 语料面。
- 全量门禁：7806 passed / 3 skipped / exit 0（L183 的 7804 + 2 新守卫，无回归）。**B254 关闭，A138 收口**。


### L185（2026-10-04）— B255 立项 + 关闭：A95/A122 合轮收口（模型凭证 ${ENV} 占位符展开语义钉成契约）

- **真缺口（A122 原框，与 A95 是同一件事的两半）**：模型条目三凭证键经 _resolve_env 解析 ${ENV} 占位符，未设置的环境变量**静默回落 ''**（与 None 等价，都是「没配凭据」的合法状态，由后端 ModelNotConfiguredError 在建后端时判）。A122 原框把「${ENV} 展开失败静默」与「三键两侧零形状判据」留待拍；A95 是「展开失败的出声口径」，两格合并处置。
- **定档（非破坏面收口）**：加载期「环境变量未设置」的条数随本机 shell 而变，接进加载声会让同一份配置在不同机器上行数不同（docs/augmentor/API.md 3.x 既定口径），故**静默 '' 是刻意契约**、出声归 validate-config 的 warning 通道（A76/A84）。本轮不改加载期行为，把该语义**钉死**：_resolve_env docstring 写明四档；新增守卫 13 例钉「已设置取值 / 未设置静默'' / 坏形状原样 / 非串原样 / 合法占位未设置'' / 权威清单 == 三键」——谁把静默改成加载期 fail-loud（或把 '' 换成报错）都要先翻这一格并重新对账对外契约。
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
- **棘轮（行号位移房）**：system_ops.py 顶部插 1 行共引 import ⇒ 全文件行号 +1，OPTIMIZATION_LOOP.md 既有 6 处 `system_ops.py:` 活行引用（:147/:292×3/:397×2）与 docs/augmentor/ARCHITECTURE.md 2 处（:147/:292）同步平移，L79 `code_but_no_name_match` 两格（账本 146、架构 34）随修引用**回稳而非升档**；l97 原地绿（守卫不参化）。
- 全量门禁：7863 passed / 3 skipped / exit 0（L192 的 7858 + 5 新守卫，无回归）。**B263 关闭**。

### L194（2026-10-08，L197 补记）— B264 立项 + 关闭：套件名封闭清单下沉 SDK 直构面

- **真缺陷（静默回落）**：`augmentor/auto_test.py` 的 `DatasetTestRunner.run_tests()` 对**显式未知套件名**静默回落到内置 default 套件 —— `_test_suites` 初始为空且没有注册入口，`if suite_name and suite_name in self._test_suites:` 不中就往 `create_test_suite("default", …)` 落。拼错的套件名拿到 default 套件的判决**不出声**（checkpoint 症状族语义漂移档；L175 / L176 / L189 / L192 / L193 封闭清单族同式）。API 面 `AUTO_TEST_SUITES` 早有 400 白名单，SDK 直构面（`run_tests` / `run_dataset_tests` / CLI `auto-test`）独缺这一格。
- **修法**：① `auto_test.py` 立模块级 `DEFAULT_SUITE_NAME = "default"`（A77 单一权威，字面 "default" 全仓不再有两份）；② `run_tests()` 入口按 `set(self._test_suites) | {DEFAULT_SUITE_NAME}` 判封闭清单，未知名 / 非 str 抛 `DataValidationError` 并列出全部合法名；`None` = 「未指定」仍走 default、显式 `"default"` 在注册前也合法、`create_test_suite` 注册过的自定义套件名照旧放行（三条防修过头对照同轮钉死）；③ API 面 `AUTO_TEST_SUITES` 由手抄 `("default",)` 改共引 SDK 常量（`is` 钉，400 判据与文案不变）。
- **守卫**：`tests/unit/test_auto_test.py` 新增 `TestSuiteNameClosedListL194` **6 例**；旧 `test_to_dict` 里靠静默回落才没红的套件名 `"test"` 改为 `"default"`。
- **棘轮（行号位移房）**：顶部插 1 行共引 import ⇒ `system_ops.py` 全文件行号 +1，`OPTIMIZATION_LOOP.md` 既有活行引用与 `docs/augmentor/ARCHITECTURE.md` 2 处（`:147→:148`、`:292→:293`）随移（`29b715600`，纯引用漂移）；L79 `code_but_no_name_match` 漂移桶按「数字搬进常量」条款重钉 **146 → 149**（`2b2d8fb03`）；`line_refs 270 / file_tokens 1661` 未动；l97 原地绿。
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
- **改动的六处数字**：README 端点 68 → **71**（两处）、README `dataset` 分组 12 → **14**、`docs/augmentor/API.md` 端点总数 70 → **71**（两处）、`docs/augmentor/API.md` `### quality（8）` → **（9）**（该节表格本来就有 9 行，是标题写错）、`docs/augmentor/API.md` 导出格式示例 5 → **13**、`docs/augmentor/ARCHITECTURE.md` router 11 → **13** 并补 `dataset_tools.py` / `system_ops.py` 两行、页面 9 → **11**（两处）、事件循环守门 12 → **13** 条且 quality 8 → **9** 条。
- **会漂移的数字改成「以枚举为准」的写法**（承第一本账 F-06 纪律）：README 的导出格式行与 API.md 的 `GET /api/export/formats` 示例都加了「清单由 `ExportFormat` 驱动、精确值以枚举/实时接口为准、不要从文档手抄」；并**顺手堵掉一个会让人改错的相邻陷阱** —— `GET /api/config` 回显里的 `export.formats` 是**五值出厂默认**，与这里的「全部支持格式」不是同一件事（探针实测两者分别为 5 与 13，第二本账 L184 也把前者归入「仅回显」档）。
- **账本自洽三件**：① 补 L194 的「已完成」行与循环日志段（L194 当时只写日志块、漏写清单行，同型事故 L50 也发生过一次 ⇒ 判据本身没有守卫，这是**第二次**同类，按纪律升级为一条待办而不是再忍一次）；② 在「已完成」段首加指针，说明清单只到 L113、L114–L193 以循环日志为权威，不再回填 80 行（两处数字长期不一致的风险高于补行的边际价值）；③ 全量读数从 7869 更新到 7897。
- **留作维护者决策、本轮只记不改的一处**：Python 版本口径三处不一致 —— README `3.8+`、`docs/augmentor/README.md` `3.10+（开发验证环境为 3.13）`、`docs/augmentor/DEPLOYMENT.md` 镜像与服务器段 `3.11`。实测代码侧无 3.10 独有语法（无 `match`、无内置泛型运行期注解），依赖上限也按 3.8 钉（`numpy<3.0.0` / `fastapi<1.0.0`），但两个验证解释器是 3.13.14 与 3.14.4、部署镜像是 3.11-slim ⇒ 「最低支持哪个版本」是产品决策，按「先记后问」处理，不擅自拍数。
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
  全量里红。已重排该格（代码跨度不跨行）并在原格留一句指向本块。这不是本轮新引入的语义，是 `4d177e3f3`（L212 至 L214 那一批账本补写）
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
  **14**（export 2 / rag 4 / vector 5 / multimodal 3）；② `docs/augmentor/API.md` 引用了一支不存在的测试名
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
  判据改成批宽 3 才量得出「峰值 = 批宽」，这条也写进了 `docs/augmentor/API.md`。③ 原本断言 `backend=None` 即抛 ⇒
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
- **本轮被自己的守卫抓住的第二次机会**：`docs/augmentor/ARCHITECTURE.md` 的模块树里插两行新模块，把历史账本引用的
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
- **新立 A275**：`docs/augmentor/README.md` 的配置段表格「关键项」列与 `AppConfig` 的键清单**没有机器对账**，本轮两处文档
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
  `docs/augmentor/ARCHITECTURE.md` 引用的 `multilingual.py:43` 之下**，`test_comment_refs_a184.py` 与
  `test_doc_line_refs_l79.py` 当场绿；这也是本轮刻意不新增模块级常量（`sample_limit` 写为函数内局部量）的原因。
- **配置普查本轮零变动**：没有新增、删除或接线任何配置键，现量仍是 schema **70 键** / 有读者 **49** /
  零读者 **21** / 归档待处置 **18**。`multilingual` 四键的读者面仍是 L217 那一条消费层。
- **文档同步**：`docs/augmentor/API.md` 翻译行补上失败回执四键的读法，并把「为什么部分失败仍 200」与「只读 `items`
  读不出失败」写进同一格。`docs/augmentor/README.md` 的 `multilingual` 行讲的是四只键的落点，本轮未涉配置面，故不改。
- **仍在账上的邻居**：**A272**（`multilingual` 要不要开成第 5 个可写配置节——本轮明确回答了「样本上限」那半边：
  它不该成为键，但 `default_target_lang` 等四键是否运行时可改仍是待拍项）；以及 L220 之后量到的
  「**32 个端点没有前端调用者**」这一消费面缺口，`/api/dataset/translate` 本身就在其中。

### L222（2026-10-11）— 配置面三侧同源对账：README 表格、`AppConfig`、`config.yaml` 三角互核，权威只有一份

- **补的是 A275 那一格**，而且是它自己立的账被自己的尺子量了一遍：L220 新增
  `evaluation.generated_field` 时 `docs/augmentor/README.md` 与 `config.yaml` 全靠人肉同步，当时我在日志里
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
- **文档同步**：`docs/augmentor/README.md` §7 的配置表从 16 行重排为 22 行，列名「关键项」改成「键清单」——
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
- **插行与引用**：`docs/augmentor/README.md` §7 表格净增 12 行（+29 / −17），而 `test_doc_line_refs_l79.py`
  的扫描面是 `docs/augmentor/ARCHITECTURE.md` 与本账本这两份活文档，README 不在刀口上；本轮代码文件一行未动，
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
  `docs/augmentor/API.md` 的四列说明表里那些反引号字段名**没有任何机器读过**（l210 只核路径集合与说明
  存在性，l200 只核数字），确认之后才动手，不是先动手再找理由。
- **权威只有一份**：字段名从 `app.openapi()` 实推，契约面 = 请求体属性（递归过 `$ref` / `allOf` /
  `additionalProperties` / `items`）+ `parameters[].name` + 响应体属性。现量：详细表 **31** 格
  （dataset 18 + system 13）、提取到 **58** 个标识符形状的声称、其中 **40** 个（**69%**）直接落在
  所属端点的契约面上。剩下 18 个不许靠「散文嘛」过关，各自要落进一档**有位置**的证据。
- **字面量档的作用域是量出来的，不是拍出来的**：`output_path` 确实存在于产品代码，但只是
  `augmentor/backup.py:199` 与 `augmentor/export_enhanced.py:143` 的返回键加一堆局部变量名；
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
  整篇文档级判据，防止「表格修干净、注里还写着旧名」。改后 `docs/augmentor/API.md` 与 `docs/augmentor/README.md`
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
  另有两只类型是 naive 正则读不动的形状（联合别名 `types/api.ts:135`、`extends` 继承 `types/api.ts:220`）——
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
  的合法取值来自数据集本身，而 SDK 里那份 `["instruction", "input", "output"]` 只是 `augmentor/audit.py:69`、
  `augmentor/privacy.py:76` 与 `augmentor/cli/parser.py:173 / 406 / 431` 的**五处手写字面量**（后两格
  由 L244 按内容重钉：那一轮给 `parser.py` 净加了一行，把这两处平移走了，而 a184 全程绿 ⇒ A331 的盲区），
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

- **`docs/augmentor/API.md` 两句订正（都是同行追加，`git diff --numstat` 复核 2/2 与 1/1 净行数零）**：① aggregate 行原本写着「`target_size` 省略会 400」——假话，实测 200；订正后写明**省略不是 400**、走 `DEFAULT_TARGET_SIZE`、且这一格只咬 weighted（指向 **A294**），并把回执偏离 SDK 键集这件事写进行里（指向 **A293** 与 **A297**）；② rag 行原本没写默认档是哪个，订正后写明出厂默认 **`llamaindex`**（`augmentor/config.py:533`）与端点里那第三处字面量 `langchain`（**A295**），并写明多送的 `chunk_size` / `chunk_overlap` 会被静默忽略。**订正之后新立四支文档用例**把这两行钉住：文档行必须说端点**真的会做的事**，并且与一次真跑的 200 回执交叉核对。**判据咬到自己那一处**：新用例的第一版断言「aggregate 行不得含『`target_size` 省略会 400』」，而我自己刚写进去的订正说明里**引用了这句被推翻的原文**，于是当场红 ⇒ 改成断言整句原文的形状（订正说明可以引，独立声称不行）。

- **六笔新账（全部只立不做，逐条都有实测读数）**：**A292**（`ratio` 无判决，三种方法三种结局：`ratio=2` 在 random 是 400 且 detail 是 CPython 的英文原文、在 systematic/stratified 是 200 且回整份；`ratio=-0.5` 在 systematic 是 200 且回 10 条）、**A293**（端点构造无参 `DataAggregator()`，`key_fields` / `value_fields` / `random_state` 三档不可达 ⇒ 去重恒按 `instruction`、一致性恒按 `output`、weighted 不可复现；外加两套回执键集漂移与 `conflicts` 整份上线）、**A294**（`target_size` 只咬 weighted，0 合法、-1 是 400、省略走 100）、**A295**（默认格式的第三处字面量写的是 `langchain`；空串与省略等价；分块两格静默忽略）、**A296**（分层分组键只取前 10 字，前缀相同的不同类静默折叠，且折叠在回执上无痕迹）、**A297**（`consistent` 把同源内部分歧算成冲突，那些条目永不落地 ⇒ 一份内部有重复提问的数据集会让整组产物为空）。**六条一条都没动产品代码**：改判决、改回执键集、改分组键都是对外契约变化，按纪律待拍。

- **八臂「页面初值 == 契约默认」逐个对账**：`method="random"`、`size` / `ratio` / `seed` / `stratify_key` 四格从空开始（留空即不发那一格）、`strategy="union"`、`target_size` 留空、`format` 留空（＝「服务端默认」那一档）。**`DEFAULT_TARGET_SIZE = 100` 全仓只有 SDK 一处**，页面**不抄那个数当默认**——由一支用例同时钉住「常数确实是 100」与「页面 state 初值是 `null`」。服务层三只 fetchers 全文**没有第二套默认**（既不写 `= '…'` 也不写 `??`），唯一一处默认表达式是转换卡那处 `format: opts.format || undefined` 形状卫生哨兵；这一批实测送空串与省略等价，所以那一处在**这一只端点上不是为了躲判决**（与 A291 那处不同），动机差别写进了用例 docstring。

- **census 两把按现量重钉（不许照抄上一轮）**：l97 CALLS 411 → **419**、READABLE 293 → **301**、盲面 **118 一格未动**——本轮八处新增 `@pytest.mark.parametrize` 的值位**全是内联字面量清单**，所以八格全落可读面，两边独立闭合（+8 / +8 / +0）。l98 档位和 **118** 与盲面用例名册 **113** 未动，豁免用数 18 → **23**：五支新用例的值位来自 AST 分发臂与请求模型字段集（不是手写清单），逐支带理由入册。**豁免名单变长不是把尺子放松**——l98 判的仍是**逐行**的值位形状，整支豁免只适用于「值位由机器实推」那一种理由。**一处读数口径的坑**：l98 的动态面依赖 `collected`（下限 3000 条），单跑子集会抛 `RulerNotProven` 报成 **5 errors**，所以那两格只能在全量里量；第一次全量读到的是 `assert 23 == 18` **1 failed**，按实测把五支新豁免点数清楚之后重钉成 23，不是先改数再看。

- **注入九臂，逐臂红格按实测记**：Python 面六臂（① 摘掉 `dispatch_arms()` 的 `LookupError` 那一档 ⇒ 响 **2** 支；② 页面 `method` 下拉多编一档 ⇒ 响 **5** 支；③ 把「两套键集漂移」的两个差集断言削弱成一个 ⇒ 响 **3** 支；④ 服务层给 `target_size` 抄一份默认 100 ⇒ 响 **1** 支；⑤ 删掉卡片上「只咬 weighted」那句 ⇒ 响 **1** 支；⑥ 把文档用例的交叉核对摘掉 ⇒ 响 **1** 支），基线 **70 passed**；vitest 面三臂（W1 采样卡的目标清单多一档 ⇒ **2** 支、W2 把 `seed` 初值替契约编一个默认 ⇒ **1** 支、W3 删掉「只按前 10 个字符分组」那句 ⇒ **1** 支）。九臂 rc 全 1 且输出含 `failed`，被改文件逐臂字节复原、收尾重新验盘并断言哈希与进入时一致（sha256 `8b17a662` 起），复原后基线复跑 **2 文件 / 32 支**全绿。**一臂被拦在写盘之前**：W3 的锚点 `完全空转` 在文件里出现**两次**（模块注释一处、卡片文案一处），注入器的唯一性断言当场响 ⇒ **磁盘未被触碰**，换成更长的锚点（`配其余三支**完全空转**`）并按 argv 过滤到单臂之后才注入成功。这正是「锚点必须唯一」这条该有的反应：少写一步就会改错地方并把红归因到错的臂上。

- **前端面的三件收口判据齐**（A279 从第一批起没换过，本轮第六次命中）：消费守卫的名单与实测集合**相等且变短**、`api.wiring.test.ts` 的 `KNOWN_UNWIRED` **仍为空**、三员各覆盖**成功与失败两路**。三门禁 `npx tsc --noEmit` / `npx eslint . --ext .ts,.tsx` / `npx vitest run`：前两者 rc=0，vitest **15 文件 176 → 195 支全绿**（新增那一支文件 19 支）。**渲染面用例第一版撞的三处地雷，全部按实测改判据而不是改页面**：① antd 的 loading 图标会并进按钮的可达名 ⇒ 断言改成子串正则；② 按钮文案在成功/失败两路上不同 ⇒ 用 role + 名字前缀定位而不是整串相等；③ `InputNumber` 的 spinbuttons 读出来带 step 补零，且那一页的 spinbutton 数量比 L229 多 ⇒ 按 `slice(4)` 取本卡那几格并写明为什么是这个偏移。

- **门禁读数**：全量 `python -m pytest tests -q -p no:cacheprovider` ⇒ **8549 passed / 11 skipped / 0 failed**，覆盖率 **lines 14186/14196 = 99.93%、branches 3912/3936 = 99.39%**（179.78 s；与 L229 的 8475 相减 `+74` = 新守卫 74 支，接线本身零后端用例增量）。这一跑同时是 l98 动态面的实测点。**本轮全量跑了三次，中间两次读数留着**：第一次 **8544 passed / 1 failed**（169.66 s，红的是 l98 豁免用数 `assert 23 == 18`）；第二次是修完 A184 那两支之后的 **8545 passed / 0 failed**（112.82 s，带 `--no-cov`，只为确认收集面干净，不作为定标读数）；第三次才是上面那份定标读数。六把账本与文档守卫 **149 passed / 8 skipped**（8 支 skip 是 `test_doc_line_refs_l79.py` 要作者工作树那一档，与本轮无关）。**账本批之后复跑同一份全量，读数一字未动**（8549 passed / 11 skipped / 0 failed，163.18 s，覆盖率仍是 lines 14186/14196 = 99.93%、branches 3912/3936 = 99.39%；两次都是 0 failed，差值只在墙钟上：179.78 s → 163.18 s）。

- **仍在账上的邻居**：**A279**（剩 15 条，其中 `NO_ENTRY` 1 条 = `translate`，归 **A283**）、**A292**~**A297**（本轮新立）、**A289** / **A290** / **A291**（L229 立的三条同族契约面缺口，其中 A290 与本轮 A293 是同一形状：回执手抄而不是引用 SDK）、**A287** 与 **A288**（契约面死面与值域）、**A284**（`unique_instructions` 误名）、**A285**（五处手写字段清单没有权威——本轮这两个后端权威同样没有可引常数，只是这两个能用 AST 数出来，A285 那五处不能）、**A286**（比值型计时守卫会假红）、**A281**（3 处响应类型缺契约已有字段）、**A282**（format 基线红且 CI 无门）、**A280**（错误分档一刀切）、**A278**（自由子对象的形状 vs 可证伪白名单）、**A276**（`app` 三键的处置）、**A271** 的 18 只待处置键、**A272**（`multilingual` 要不要可写）、**A273**（`self.expander` / `self.tracker` 的消费者还是删除）。

### L231（2026-10-11）— A279 第七批：备份那一族四条进系统状态页，并补掉 `name` 越界写盘那一处产品缺陷

- **这一轮做三件事**：① `GET` / `POST /api/system/backups`、`POST …/{backup_id}/restore`、`DELETE …/{backup_id}` 四条进系统状态页（一张「数据备份」卡：列表 + 创建 + 恢复 + 删除，源文件候选取自 `/api/data/list` 也允许手填，删除走 Popconfirm 二次确认）；② 新立 `tests/unit/test_backup_wiring_l231.py`（**33 支**）与渲染面 `web/src/pages/System.backup.test.tsx`（**11 支**）；③ **改一处产品面**——`DatasetBackup.backup()` 的 `name` 越界写盘。前六批接线都只动 `web/src`，这一批动不了：接线之前先量后端，量出来一处真缺陷。

- **那处产品缺陷的取证与修法**：`backup_path = self._backup_dir / f"{backup_id}.json"`，而 `backup_id = name or 时间戳`，`name` 直接来自请求体 ⇒ 仓外探针实测 `name="../../escaped"` 打 `POST /api/system/backups` 是 **200**，且临时目录**顶层**真的多出 `escaped.json`（白名单只管到 `backup_dir` 那一格，管不到 `name` 这一段）。修法是四行：算出 `backup_path` 之后断言 `self._backup_dir.resolve()` 在它的 `parents` 里，不在就抛 `DataValidationError`（经 `to_http_error` 落 **400**）。改后同一条请求 **400**「备份名称不能指向备份目录之外: ../../escaped」、白名单外零文件；`name` 带 `/` 那一档**照旧放行**（实测 `sub/deep` ⇒ 200，在备份目录下建 `sub/` 再落 `deep.json`）——它没有逃出备份目录，拦它就是砍能力。**这四处 +4 行把 `augmentor/backup.py:188`（L273 后为 `:195`）平移到 `:192`**（L273 后为 `:199`），两处历史行号引用（本账 A277 行、L223 日志块）同轮重钉，另加 `tests/unit/test_api_doc_field_hints_l223.py` 的 docstring 一处；`git diff --numstat` 复核三处净行数 **0**（`2/2`、`1/1`）。**插行安全不可外推**这条纪律本轮又验了一次：先 grep 过全仓，指向 `System.tsx` 的历史引用只有 `System.tsx:3` 一格，所以页面新增的 import 一律放在第 4 行及以后，第 3 行仍是原来那句服务层 import。

- **消费面按实测回填四格**：调用形状与已消费 **60 → 64**（75 条的 **85%**）、缺口名单 **15 → 11**、`CLI_ENTRY` **14 条 / 9 名 → 10 条 / 8 名**、`NO_ENTRY` 仍 **1**（`translate`，归 A283）。**这一批的名字侧动的是「一个子命令整名出名单」**：`backup` 此前吃掉四条操作（`GET` / `POST` / `DELETE` / `restore`），本轮四条一起接走 ⇒ 条目减 4、名字减 1。**我第一版把这件事写成「第一次出现一个子命令吃掉多条操作」，那是假话**——`dependency` 早就映射 3 条，只是没被接走；按实测改成「一次减 4 条是现量最大的一批」。判据 4 那一维同时涨：带 query 的调用 **5 → 9** 条、去重后键名 **6 → 7** 个（多的一个是 `backup_dir`）、`params: {…}` 归属 **4 → 8** 条、最大间隔 **14 → 41** 字符（`PARAMS_MAX_GAP = 60` 一格未动）。

- **那把窗口尺子本轮真的咬人了**：`createBackup` 的第一版把请求体写在 `api.post(…)` 的实参位上，`params` 块离所属调用超过 60 字符 ⇒ `backup_dir` **绑不上**，query 那一维对这一条失明（实测：8 条里只有它没绑上）。修法是**改产品侧而不是放宽尺子**——把请求体提成一行 `const body = {…}`，间隔回到 14 字符，`backup_dir` 当场绑上。这一格的教训与 L224 立尺子时那句「上限守的是尚未出现的形状」正好接上：形状真的出现了，而它出现在**我新写的代码**里。

- **新守卫首跑 3 failed / 30 passed，三格全是尺子自己的 bug**：① 路径字面量对账没剥 `/api` 前缀（契约面是 `/api/system/backups`，`api.ts` 里写的是 `/system/backups`，`baseURL` 补前缀）⇒ 补 `.removeprefix("/api")`；② 「服务层不写第二套默认」那一支被**页面上的老实文案**判红（`.backups` 出现在 JSX 正文里，那句话正是在向用户解释三份默认的关系）⇒ 收窄成「只扫 `api.ts` + 页面里不许出现 `backup_dir` / `backupDir` 的字面量赋值」；③ `verify_api_key` 门禁普查拿 `app.routes` 当语料得到**空名单**——这版 Starlette 不展平 included router（l224 里为路由顺序那一格已经量过一次同一个坑）⇒ 换成 `api.routes.system_ops.router.routes` + `route.dependant.dependencies`，并补一格 `seen` 断言证明四条备份路由确实都在语料里（否则「零条未挂闸」与「零条被读到」长得一样）。

- **复跑又 2 failed / 31 passed，还是尺子的范围写错**：④ A300 那一支我写成「全契约里不许出现 `verify` 或 `clean` 字样」，被 `/api/quality/clean` 判红——那是一条与备份毫不相干的端点 ⇒ 收窄到备份面这一族的 (方法, 路径) 集合**精确相等**（四条，多一条少一条都红）；⑤ A302 那一支写成「`web/src` 全文零处 `X-API-Key`」，被 `System.tsx` 里那句「前端目前没有送 X-API-Key 的通道（A302）」判红 ⇒ 只认**带引号的字面量**（真有通道时那个头名必然写在 `headers: { 'X-API-Key': … }` 的字符串位上），同时把「提到这个头名的文件恰好只有 `System.tsx` 一处」钉成现量。**两次都是同一个错误形状**：把「这一族里没有」写成「全仓没有」，语料一宽就撞上无关住户；五格全在尺子这一侧，产品面一处未改。

- **渲染面第一版撞的两处 antd 形状，都按实测改判据而不是改页面**：① `AutoComplete` 把 `aria-label` 同时挂在**外层 div 与内层 input** 上 ⇒ `getByLabelText('备份源文件')` 命中 2 个直接抛，加一个只取 `tagName === 'INPUT'` 的 helper 才定位到；② antd 报 `[antd: Table] index parameter of rowKey function is deprecated` ⇒ 行键从 `backup_id#index` 改成 `backup_id#timestamp`（这也更老实：索引里两条同 id 的记录本来就是靠时间戳区分的，见 **A298**），并因此要求夹具里那两条同 id 记录带上**实测的两个不同时间戳**（`.991804` / `.997820`）——否则 React 键会撞，夹具就不老实了。

- **注入六臂，逐臂红格按实测记**：① 摘掉 `backup.py` 那道越界门 ⇒ **3** 支红（HTTP 面 400 那一支、异常类型那一支、SDK 直构面那一支）；② TS 侧把恢复回执的落点键改名 `output_path` → `output_file` ⇒ **1** 支红（两侧键集对账）；③ 服务层把恢复路径拼错一个字母 `restore` → `recover` ⇒ **6** 支红（l231 的 fetcher 路径那一支 + l224 的幽灵调用、query 声明、params 归属、名单相等、规模档五支）；④ 页面摘掉删除前的 Popconfirm ⇒ vitest **1** 支红；⑤ 文档把 convert 那行的输出字段写回 `output_path` ⇒ **2** 支红（收窄后的禁令 + 无据 token 那一支）；⑥ 往 l224 的 `UNCONSUMED` 里塞回一条已接好的操作 ⇒ **3** 支红（名单相等、规模档、两分档）。六臂 rc 全 1，被改的六个文件（`augmentor/backup.py`、`web/src/types/api.ts`、`web/src/services/api.ts`、`web/src/pages/System.tsx`、`docs/augmentor/API.md`、`tests/unit/test_api_endpoint_consumption_l224.py`）逐臂字节复原、收尾重新验盘并断言 sha256 与进入时一致，复原后基线复跑 **82 支**（33 + 26 + 23）全绿 + vitest rc=0。**一臂被拦在写盘之前**：⑥ 的第一版锚点是「把名单里那条 `DELETE /api/system/backups/{backup_id}` 删掉」，而那条本轮已经不在名单里了 ⇒ 注入器的唯一性断言命中 0 次当场抛，**磁盘未被触碰**；换成「塞回一条已接好的 `GET /api/system/backups`」才注入成功。

- **A277 的全文禁令与本轮的真话冲突，处置是收窄而不是摘掉**：L223 立的判据是「整篇 `docs/augmentor/API.md` 不许出现 `output_path`」，起因是那个名字当时只是路由里的局部变量名、文档在教调用方发一个会被 pydantic 拒掉的键。本轮量出它在**另一族**里是真的：`POST /api/system/backups/{backup_id}/restore` 的响应落点键就叫 `output_path`（同族其余五条写盘端点叫 `output_file`，`split` 叫 `output_dir`）。照实写进文档就撞上禁令 ⇒ 判据改成「dataset 七条写盘族一行不许出现 + 全文只许 restore 那一行出现，多一行即红」，并把那七条路径提成 `WRITE_SIDE_PATHS` 供两支判据共用（原缺陷那一侧**没有变松**：注入臂 ⑤ 实证把 convert 那行写回幽灵名仍 2 支红）。

- **census 两把棘轮本轮一格未动，且这不是「没量」**：两支新文件里 `parametrize` 出现 **0** 次（`grep -c` 实测），l97 的 `MEASURED_CALLS = 419` / `READABLE = 301` / 盲面 `118` 与 l98 的四档分布、豁免 `23` 全部原地绿；l98 的动态面只在整跑里可量（子集跑会以 `RulerNotProven` 报 5 个 error，那是判据在自证而不是坏了），本轮全量跑过 ⇒ 那一面也是实测绿。**这一族里还有一格假读数，是我自己写出来又划掉的**：清单行第一版写「哈希槽 prog 侧 15 → 16、唯一哈希 69 → 70」，理由是「本轮往清单里加了一行带哈希的条目」——六把守卫跑完全绿才发现那把尺子的语料是 `LEDGER = OPTIMIZATION_LOOP.md`（**第一本账**，`tests/unit/test_ledger_hash_slots_l98.py:53`），而 L231 写在第二本 ⇒ 六档现量（15/24/15/69/占位 0/R6 白名单四格）一格未动，守卫 **149 passed / 8 skipped** 原地绿。已按实测改回。教训与 L224 那句「尺子的语料面要先读判据再报读数」是同一件事：**加一行清单条目并不必然动哈希槽，动的是它读的那一本账**。

- **门禁读数**：全量 `python -m pytest tests -q -p no:cacheprovider` ⇒ **8582 passed / 11 skipped / 0 failed**（180.57 s，覆盖率 **lines 99.93% / branches 99.39%**，TOTAL 行 99%）；与 L230 的 8549 相减 `+33` 恰等于新守卫支数（接线本身零后端用例增量，产品面那四行的行为由新守卫的 3 支覆盖）。前端三门禁：`npx tsc --noEmit` rc=0、`npx eslint . --ext .ts,.tsx` 零输出、`npx vitest run` **16 文件 / 206 支全绿**（基线 15 文件 / 195 支，`+11` = 新那支文件的 11 支）。六把账本与文档守卫在账本批改完后复跑 ⇒ **149 passed / 8 skipped**（与 L230 同一读数；8 个 skip 全是 l79 那支「需要作者工作树」的既有豁免）。

- **下一批的口径**：**A279**（剩 11 员：`CLI_ENTRY` 10 员 + `NO_ENTRY` 1 员 = `translate`；`GET /api/system/dependencies` 仍按 L226 的纪律拒接）、**A298~A304**（本轮新立的七条备份面缺口，其中 A299 与 A300 是同一件事的两半）、**A283**（`translate` 的契约缺口）、**A280**（错误分档一刀切）、**A278**（自由子对象的形状 vs 可证伪白名单）、**A276**（`app` 三键的处置）、**A271** 的 18 只待处置键、**A272**（`multilingual` 要不要可写）、**A273**（`self.expander` / `self.tracker` 的消费者还是删除）。

### L232（2026-10-11）— A279 第八批：数据集血缘那一族三条进系统状态页，并收掉 `description`/`tags` 回显即丢那一处产品缺陷

- **这一轮做三件事**：① `GET` / `POST /api/system/dependency/datasets` 与 `GET /api/system/dependency/graph` 三条进系统状态页（第四张卡「数据集血缘」：登记表单 + 已登记列表 + 图的三格计数与 `issues` 逐条列出）；② 把接线时量出来的一处产品缺陷（**A305**：`description` / `tags` 回显即丢）当场修掉；③ 新立 `tests/unit/test_lineage_wiring_l232.py`（39 支）与渲染面 `web/src/pages/System.lineage.test.tsx`（8 支），五臂注入取证。

- **卡片为什么叫「数据集血缘」而不是「依赖」**：页面上已经有一张「依赖明细」卡，那是 `GET /api/system/dependencies`（**可选依赖包诊断**：已装/缺失两栏 + 降级功能清单），与这一族（数据集之间的血缘/派生关系）**同前缀不同族**。两个标题同时在场由判据钉住（`test_the_card_is_named_after_lineage_not_after_packages`），模块 docstring 里也写清了区别。**这个「同前缀不同族」本轮咬了我一口**，见下面第 6 条。

- **A305 的取证与修法**：请求体带上 `description: "描述B"` / `tags: ["t1","t2"]` ⇒ **200**，回执九键里两格都在；紧接着 `GET …/datasets` 里同一条是 `""` 与 `[]`，磁盘 `datasets.json` 同样为空。根因在路由里读得出来：`register_dataset()` 内部就 `atomic_write_json` 落了盘，端点却在这之后才 `info.description = …` / `info.tags = …` —— 改的是内存里那个返回对象。修法是随登记一起传下去（SDK 形参本来就有这两格），删掉事后赋值那四行，`api/routes/system_ops.py` 净 **+1 行**（6/5）。**这一条最值得留在账上的是「哪把尺子在改前也是绿的」**：只看响应形状的判据（回执确实回显了两格）在改前照样绿 ⇒ 新守卫那一条判据**只读磁盘与随后的 GET、不读响应**；注入臂1（把修法改回改前形状）实证它点名喊红。

- **插行有没有平移历史行号**：`system_ops.py` 那处改动**之上**的历史引用现量在 150/295/397/398/400/415，净 +1 行落在它们全部之下 ⇒ 无一平移；`docs/augmentor/API.md` 与 `README.md` 都是**同行追加、净 0 行**（4/4 与 1/1，`git diff --numstat` 复核），所以本账第 773 行那条 `docs/augmentor/API.md:1139` 引用仍指得准——L231 那次是 +4 行真的平移了两处历史引用、当轮重钉，这一次是**先躲开**。

- **census 两把棘轮动了三格，读数是量出来的不是推的**：新守卫两支 `parametrize`（`input_file` 四档坏值 / `registry_path` 两档坏值）。仓外探针 `l232_census.py` 直接 `import` l97 那把尺子、对 `tests/` 现量并逐处打印桶归属 ⇒ 两处都落 `plain`（可读档），`MEASURED_CALLS` **419 → 421**、`MEASURED_READABLE` **301 → 303**、盲面 `no-literal-list` **118 一格未动**；l98 从 l97 `import` 同两个常量故一并生效，其六档分布与豁免 `23` 原地绿。**没有按「值位是字面量 ⇒ 可读」想当然写**——L204 那次正是这么推的，实测当场红（那两支的值位是模块级名字引用）。l98 的动态面只在整跑里可量（子集跑以 `RulerNotProven` 报 5 个 error，那是判据在自证而不是坏了；本轮隔离跑实测 54 passed / 5 errors，全量跑里 5 支都绿）。

- **三把尺子首跑红，逐格按各自 docstring 的处方重钉，没有一把被改松**：① `test_api_doc_field_hints_l223.py` 红 1 格 —— 我在 `docs/augmentor/API.md` 那格里用反引号引了 `remove_dataset` / `unregister_dataset` 两个**根本不存在**的方法名（「它们不存在」正是 A309 的内容），而那把尺子要求提示里每个名字都绑得到证据（契约面 / 字面量 / 符号 / 配置键 / CLI / JSON）。处方是改自己的散文：去掉反引号与那两个名字，写成「SDK 也没有注销数据集的方法」；**尺子没动**，而且页面上那句原文（`System.tsx` 里带 `remove_dataset`）不在它的语料面内、照旧留着。② `l224` 红 5 格，全是「按实测回填」类（形状 67、query 12 条 / 8 键、`params` 归属 11、缺口名单 8、名册规模），逐格用 `l232_measure.py` 的读数重钉。③ **本轮新守卫自己首跑红 3 格**：1 格是诚实口径那句短语我写成了服务端判决原文（卡上其实写的是「白名单外是 403」），另 **2 格是我自己把族过滤写宽了** —— `"dependenc" in p` 同时命中另一族的 `GET /api/system/dependencies`，于是「血缘一族只有三条」那两格判据多出一员。收窄成前缀 `/api/system/dependency/` 并把理由写进常量注释；**这与 L231 那次「把『不在这一族』写成『不在仓里』」是同一个缺陷形状**：判据的作用域写宽，就会把邻族当成本族。顺手把同一支里另一把**恒真的尺子**换成会响的：原本断言 `add_dependency` 等三个方法名不出现在 openapi 的**路径串**里（路径从来不长这样，永远绿），改成查 `operationId`（FastAPI 的默认 operationId 同时含函数名与路径，新增一条以这三个方法命名的路由会红）。另外我自己写出过 5 处「永远为真」的断言（`… or True`、`… if … else True`、`assert not x.is_absolute() is False` 一类），全部删掉或换成真的 `in` / `==`。

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

- **与 A162 那条架构决定不冲突，且已写进 ARCHITECTURE.md**：账上有一条「取文案选 helper，不选 axios 拦截器」（A162）。本轮挂的是**请求**拦截器，只管把头挂上去；**文案仍然全部住在 `apiErrorDetail` 这个纯函数里**，那一拍没被推翻。这层关系同行写进 `docs/augmentor/ARCHITECTURE.md` 密钥那一条（净零行），免得后来读的人以为两处互相打脸。

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

- **A314 是全量门禁首跑那唯一一支红倒推出来的，不是设计出来的**：`OSError: [WinError 1920] 系统无法访问此文件。: 'data\versions\current'`，触发点是 `create_version` 里那句「先看那一格有没有东西」。现场取证量到三件：① 那份 `current` 是 **WSL 在 drvfs 上留下的 LX 链接**——`is_symlink()` 是 **False**、`os.readlink()` 报 *not a symbolic link*、只有 `os.stat` 抛 1920；② 链接目标写的是**整段相对路径** `data/versions/v_…`，而 `Path.symlink_to` 的相对目标按**链接自己所在目录**解析 ⇒ 实际指向 `data/versions/data/versions/v_…`，**恒断链**（成因是 `storage_dir` 的出厂默认就是相对路径）；③ 那个目录里当时堆着 **67** 个用例跑出来的 `v_*` 快照（⇒ A315）。**我最初的修法被探针当场推翻**：原打算「先判 `is_symlink()` 再判 `exists()`」，而 ① 量到的正是 `is_symlink()` 也认不出来 ⇒ **重排治不了它**。改成把「判存在 + 清除」搬进 `_clear_current_link()`，**stat 抛出来就当作「那里有东西」照清**；清除按「先当文件删、删不掉再当目录删」两步走，两步都失败就放过（这一格是便利索引不是数据，建不出新的也不该让 create/rollback 失败）。**两处改动都刻意做成净零行**：账本里有三处活引用指着这只文件（`:205` / `:289` / `:390`），插行会把它们平移出代码跨度——第一版没压准，`with open(` 落到 196，把 `except OSError:` 下面那条注释拆成两行补回 197，`git diff --numstat` 复核 18/18 之后三个锚点逐个复核未动。**刻意没做的一件事**：不加 `target_is_directory=True`——本机 `os.symlink` 无特权直接吃 WinError 1314，没法验证「目录符号链接在 Windows 上能否被 `unlink` 掉」（`DeleteFileW` 对目录符号链接会失败、要 `RemoveDirectory`），引入一个无法验证的失效形状违反「失败要响」。**修完的自愈证据**：那支用例转绿、原来读不了的 LX 链接被 `unlink` 掉（`ls data/versions/current` 报 No such file）、`current.txt` 重写成最新版本 id。

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

- **A283① 的顺序是量过的，不是写出来的**：落点先 `resolve_data_path(for_write=True)`、再动模型 ⇒ 越界落点 403 出声时**模型调用 0 次**（替身数出来的）。这一格在花钱路径上是要紧的：参数错误必须先出声，不能先花三次 token 再告诉用户落点不合法。响应面刻意让 `output_file` **键恒在**——用「键在不在」区分不了「没落盘」与「落了盘但没回显」，所以两态都要在契约里可读（没给时 `null`，页面回执就照实写「未落盘（只在本次响应里）」）。`docs/augmentor/API.md` 的 translate 说明行与 README 页面表两行都按这个口径**同行改写**。

- **A322 的第三键是本轮首跑量出来的新事实，不是设计出来的**：新守卫首跑 **2 failed / 16 passed**，两支红的都是 `set(on_disk[0])` 比预期**多一格 `language`**——追到 `augmentor/multilingual.py:190` 那句无条件 `new_item["language"] = target_lang`。探针当时只报了「产物凭空多两键」，所以立账口径当场从「多两键」升级为「**多三键**」，并把「一个字没翻也照盖语言戳」这件事写进四处（守卫 docstring / 页面卡片文案 / 前端用例断言 / `docs/augmentor/API.md`）。**这才是这一条真正会骗人的地方**：回执四键全零 + 产物上写着 `language: en`，从任何一面都读不出「这次什么都没发生」。

- **注入取证五臂逐臂点名喊红，前三臂走零写入通道**：`atomic_write_json` 打成哑（pytest 插件在导入后的模块对象上打补丁）⇒ **4 支红**（落盘两支 + A322 两支，都是「盘上真有那一份」的判据）；`resolve_data_path` 去掉白名单判决 ⇒ **1 支红**，而且失败信息里赫然是根外那个绝对路径（`…\outside\escaped.json`）与三条译文，等于把缺陷现场打印出来；往 l233 的 `GATED_NOT_YET_CONSUMED` 塞回一条 ⇒ **1 支红**（`test_the_unwired_tail_is_empty`），证明**空集相等那一档仍会响**。后两臂的判据住在前端源码里，必须动文件，所以走字节级注入：`disabled={!trAck}` → `disabled={false}` ⇒ vitest **1 failed / 8 passed**；`{diag && (` → `{status && (` ⇒ vitest **1 failed / 17 passed** 且 Python 静态那一支（`test_the_detail_cards_are_no_longer_behind_the_status_gate`）同时红。两份文件逐臂复原、**复原后重新验盘**并断言 sha256 与进场值相等（`bfa38c4f…` / `27e15fea…`）。

- **一处工具级失误，记在这里免得下次当产品问题查**：第二臂那个注入器首跑在**打印环节**被 gbk 打断（`UnicodeEncodeError: 'gbk' codec can't encode '\u23af'`——vitest 输出里那个「期望/实际」分隔符），异常发生在 `try` 里，`finally` 照常复原并验盘（`System.tsx:505` 仍是 `{diag && (`，`grep` 复核过），但那一臂的读数没取到 ⇒ 单臂重跑一份把打印过 ascii 兜底才拿到。**注入器自己崩在打印上**与 L233 那次「崩在复原上留下残留」是同一族风险的两种结局：这次因为复原在 `finally` 里、且复原后有断言，仓库没被留下任何东西。

- **前端替身的默认值是一处刻意的判据设计**：四份既有 System 测试的替身工厂里，`systemDependencies` 默认就是**裸 `vi.fn()`（不给回执）**⇒ `deps` 落 undefined ⇒ 页面退回综合状态里那一份 ⇒ 那四份文件的渲染结果与 L236 之前**逐格相同**，新增能力的举证集中在新 `describe` 的三支里，不用为了接线去改一堆无关断言。**代价是踩到一次 vitest 陷阱**：新用例给这一面安排了缺失态的 DIAG 回执，而 `vi.clearAllMocks()` **不清实现**，那份回执漏进后面的用例，把向量面那条「必需依赖齐备」判成缺失态 ⇒ **1 支红**（`System 向量库状态 > 向量面失败只降级它自己`）。修法是给新 `describe` 加局部 `afterEach(() => spies.systemDependencies.mockReset())`——因为工厂默认就是裸 `vi.fn()`，`mockReset` 恰好还原成默认，不会顺手清掉别人的默认值。

- **收口那一格按 L235 的待办复验过**：`UNCONSUMED` 归 0 之后，l224 那把「名单与实测集合相等」的尺子仍会响——它带两支牙齿（`test_the_empty_claims_are_not_a_blind_ruler` 与用 monkeypatch 摘掉一处调用形状的 `test_unwiring_one_operation_turns_the_empty_roster_red`），而注入第五臂（把 l233 的空尾巴塞回一条）当场红了 1 支 ⇒ 「空集 == 空集」不是一把恒真的尺子。**这一格是 L235 明确挂给本轮的欠账，不是顺手一提**。

- **文档与行号引用**：README 页面表两行（数据管理页补翻译卡、系统状态页补依赖诊断单独取数）与 HTTP API 段那句消费面口径、`docs/augmentor/API.md` 的 translate 说明行，全部**同行改写**，`git diff --numstat` 复核 **3/3 与 1/1 净零行** ⇒ 本账里那条 `README.md:65` 引用仍指得准（第 65 行仍是 `/system` 那一行，`sed -n '65p'` 复核过）。**顺手订正一处早已漂移的引用**：A283 行里那句 `api/routes/language.py:88`（指 `items` 那一格）在 L236 之前就已经漂到 `:94`，本轮加了两段 docstring 与一只请求字段之后是 `:105` ⇒ 同轮重钉。**漂移此前没人发现是有原因的，但原因不是「没人扫」**：`test_doc_line_refs_l79` 的 `DOCS` 只有 `docs/augmentor/ARCHITECTURE.md` 与第一本账，而 A184 那把「引用必须落在代码跨度上」的尺子**是扫第二本账的**——它判**落点形状**（那一行是不是代码），不核**行号是否还指得着当初那一格**，所以 `:88` 从「`items` 那一行」漂进「docstring 里的一行」它照样放过（docstring 行算代码跨度）。**本轮自己就被这把尺子红了一次**：日志里原本写着 `DataManagement.transform.test.tsx` 的那条行号引用，而它指的是一行**注释** ⇒ `dead_line` 硬 0 档当场响，按处方第一条「换点」改成只写文件名、不带行号。**这次红是尺子对的、案面错的**。「行号是否还指着当初那一格」这一维在第二本账上确实没有尺子，扩面要先把全部历史引用逐条验真，是一轮的活，本轮不做。`System.tsx` 那一面本轮所有插入都在第 300 行之后，故 `DataManagement.transform.test.tsx` 的注释里那条 `DataManagement.tsx:96` 未被平移；`System.tsx:3` 那条引用（本账第 339 行）按「只能同行改」的口径处理——第 3 行的 `systemDependencies` 是**同行追加**的。

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

- **文档三处，两处同行改写、一处末尾追加**：`docs/augmentor/API.md` 的 status 段补「`model_retry_count` /
  `model_retry_wait_seconds` 管道缺失时为 `null`、前端按两档渲染」（1/1 净零行）；`docs/augmentor/ARCHITECTURE.md` §3.12.5
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
  下移到 **121**，于是随代码批一起重钉成 `augmentor/health_score.py:121`（`29b715600`，L194 那一族先例：行号刷新即本条所在提交）。重钉之后
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
  每臂写完立刻读回比哈希，`finally` 逐字节复原并复验；跑完重新验磁盘四份文件（`docs/augmentor/ARCHITECTURE.md`、
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
- **⑨ 行号引用与行尾**：`docs/augmentor/ARCHITECTURE.md` 本轮净 **+63 / −1**（`git show --numstat` 复核过），两处插入
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
另把 L241 遗留在工作树的 `docs/augmentor/ARCHITECTURE.md` 两句 §3.34 六臂取证随代码提交入册——那两行插在 §7.1
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
- **⑭ 账本批（`6c141d7d5`）之后同一条定标命令复跑** ⇒ **8 883 passed / 0 failed / 10 skipped / 6 warnings**、
  `exit 0`、用时 **486.31 s**、**总覆盖 99.84 %**、`112 files skipped due to complete coverage`。
  用例数自己闭合：**L241 终态基准 8 859 + 本轮新立 24 = 8 883**，逐格对得上 ⇒ 涨的 24 支正是新守卫，
  没有任何一支既有的用例被挤掉或跳过。10 只 skip 名册逐格相同（`test_doc_line_refs_l79.py:647` 8 支
  作者工作树档 + `test_config_gates_l82.py:573` + `test_upload_ceiling_l87.py:580`）。
  用时 404.15 s → 486.31 s（**+20 %**）仍是负载那一维：本轮 24 支判据全是计数与结构，没有一条看时钟。
- **⑮ 复跑自己抓到的一处读数事故，原样入册**：这一次用 `... | tail -25` 收读数，**把控制台的 `TOTAL`
  那一行截在了窗口外面**，只留下 `112 files skipped` / `Total coverage: 99.84%` 两行。于是「覆盖率四格
  逐格相同」这句话本轮**拿不到同一次运行的控制台证据**，改从同一次运行落盘的 `coverage.xml` 复算：
  `line-rate 0.9994` / `branch-rate 0.9944` / `branches-valid 3 952`、`<line>` 元素 **14 290** 条、
  `hits=0` 的 **8** 条 ⇒ 未覆盖语句 8、分支总数 3 952、未覆盖分支按比率反算 **22**，这三格与 L241/L240
  那两格逐格相同；**只有语句总数那一格差 1**（账本钉的是 14 291，XML 侧复算 14 290）。
  `git diff --stat 78dd6602a..HEAD -- augmentor api cli` **为空** ⇒ 自 L241 终态基准以来产品面一条语句
  未动，所以这一格之差不可能是代码变化，只能是**两份读数的口径不同**（控制台的 Stmts 列 vs XML 的
  `<line>` 元素数）。本轮不把它写成「已解释」，写成一条**未对账的欠账**：**L243 的全量门禁必须把控制台
  `TOTAL` 整行留在读数窗口里**，用同一次运行的两份读数把这一格的口径钉掉；在此之前，L241 那一格的
  14 291 仍是唯一被控制台证实过的数。

### L243（2026-10-12）— 备份完整性那一族接上电：坏备份拿 409 且一个字节都不写，两只 SDK 函数有了 HTTP 入口与页面按钮

**本轮定义（动手前拍的）**：把「备份完整性」这一族一次补齐——**A299**（校验和算了、存了、比了，却从不拦截）
与 **A300**（`clean_old_backups` / `verify_backup_integrity` 两只 SDK 函数没有任何 HTTP 入口）同轮收口，两员
接进前端系统页「数据备份」卡。这是一轮**对外行为变化**的轮子（恢复的判决从 200 变 409），不是记账轮。
待拍的 A298 / A301 / A303 / A304 与性能档 A326 / A327 一律不动；本轮新立三条（A329 / A330 / A331）。

- **① 产品面三处**：㈠ `augmentor/backup.py` 的 `restore()` 里，校验和不匹配那一支从「`logger.warning`
  然后继续恢复」换成 `raise BackupIntegrityError`，detail 逐字「备份文件校验和不匹配，文件可能已损坏，
  未执行恢复: b」，抛点坐在覆盖输出文件**之前** ⇒ 零写入。㈡ `api/routes/system_ops.py` 新增四只模型
  （`BackupVerifyEntryResponse` / `BackupVerifyResponse` / `BackupCleanupRequest` / `BackupCleanupResponse`）
  与两只端点：`GET /api/system/backups/verify`（只读、不挂鉴权）与 `POST /api/system/backups/cleanup`
  （写操作、挂 `verify_api_key`）。㈢ 前端 `web/src/services/api.ts` 补 `verifyBackups` / `cleanupBackups`
  两员，`web/src/pages/System.tsx` 的「数据备份」卡补两只按钮。
- **② 四档与直觉不同的实测**（每档都有各自的点名用例，不是推的）：⑴ 目录缺失时 verify **不建目录**（回空
  结果），而 cleanup **会建**目录并只落 `index.json` ⇒ 两只端点的落盘面**相反**，各钉一支，防止日后被谁
  「统一」掉。⑵ 索引里同名两行在 verify 的结果里**塌成一行**（结果以备份 id 为键，后一条覆盖前一条），
  而列表那张表能出两行——这是 A298 的另一面。⑶ 零字节文件算「文件丢了」（`exists` 判的是存在**且非空**），
  此时 `checksum_match: false` 无从比 ⇒ 页面单列成「没法比」而不是同一种红。⑷ `clean_old_backups` 的保留数
  按**索引行数**算：那组夹具索引 4 行、磁盘 3 份，`max_backups=1` 交出 `deleted_count == 2` 而不是 3。
- **③ 闸不是一刀切的两支证明**：同名再备份一份好文件之后恢复 **200**（闸认内容不认 id，没有把 id 拉黑）；
  拿 409 之后索引里那一格的 `checksum` **一字未改**（闸不自我修复，也不改写账目）。
- **④ 缺陷注入三臂**（全走零写入通道：`monkeypatch` 打在导入后的模块对象上，仓库字节一字不动）：臂 1 摘掉
  `DatasetBackup._calculate_checksum` ⇒ 坏备份恢复回 **200** 且坏内容真的落盘 ⇒ 证明 409 由校验和承重；
  臂 2 把 `augmentor.backup.require_count` 换成 no-op ⇒ `max_backups=-1` 从 400 变 **200**、`deleted_count`
  由 2 变 **3**、索引被清空 ⇒ 证明参数闸与「按索引行数」那一格各自承重；臂 3 把
  `api.routes.system_ops.run_in_thread` 换成 async 空返回 ⇒ verify 回 `{"results": []}` ⇒ 证明线程池那一跳
  真在搬数据。每臂指名它要红的那一支判据，不写「大概会红」。
- **⑤ 判序与档位**：`BackupIntegrityError` 同时是 `AugmentorError` 与 `ValueError` 的子类，而 `api.deps`
  的 `to_http_error` 把「其余 ValueError」兜底映成 **400 原文**（直调实测 `(400, "boom")`，**不是** 500、
  也不是 404）⇒ 端点里那句显式 `except BackupIntegrityError → 409` 是吃重的，摘掉它就退成 400。
  `BackupIntegrityError` 与 `BackupError` 是**平级兄弟**（`isinstance` 互不为真，守卫单独钉），所以
  「幽灵 id ⇒ 404」与「坏备份 ⇒ 409」两档互不吞。其余实测档位：verify 缺目录 200 空结果、幽灵 id 200 空
  列表、cleanup 无 key 401「缺少或无效的 X-API-Key」、`max_backups` 的 0 与 -1 一律 400 且回
  `require_count` 原话、恢复源路径出白名单 403。
- **⑥ 契约与 CLI 的两格另立**：六员的 `responses` 键集现量都是 `{200, 422}`，409 / 401 / 403 / 404 全在
  OpenAPI 之外 ⇒ 立 **A330**（本轮把这一格钉成**现状事实**而不是承诺）；CLI 的 `backup --action` choices
  现量仍四档（`augmentor/cli/parser.py:394`）⇒ 立 **A329**（该格 L244 已收口成六档；那一行的行号未漂，本轮在 `parser.py` 加的那一行落在它**之下**）。
- **⑦ 前端面**：`System.backup.test.tsx` 的口径从「只写文案」翻正成「文案 + 行为」两面（头 docstring 改成
  八件事），新立「校验完整性」5 支（挂载不发 / 三形状分开 / 全好走成功档 / 失败只报这一句 / 列表刷新清读数）
  与「清理旧备份」3 支（Popconfirm 才发且送 `{ maxBackups: 10 }` / 份数改写 3 之后确认框文案与出参 / 401
  原话）。两处踩过的坑如实入册：㈠ 按钮的 accessible name 取自 `aria-label`（「校验备份完整性」）而不是可见
  文字（「校验完整性」），`getByRole` 第一跑四支红在这一格；㈡ antd `message` 的回执是异步渲染，同步读
  `document.body.textContent` 拿不到，那一支改 `waitFor`（实测之后才写的机制，不是先猜）。
- **⑧ 按现量重钉的六把尺子**（逐格按实测回填，**没有任何一把被改松**）：
  ㈠ `test_backup_wiring_l231.py` 的备份一族名册 4 → **6** 员，`test_two_sdk_functions_now_have_http_entries`
  明确把「判决与形状」交给新守卫，l231 只钉接线。㈡ `test_api_endpoint_consumption_l224.py` 消费面
  75 → **77** 条操作 / 71 → **73** 条路径，缺口仍 ∅。㈢ `test_parametrize_ids_l97.py` 的 CALLS 430 → **431**、
  READABLE 312 → **313**、盲面 **118** 未动，桶归属出自探针 Counter（`plain 312 / no-literal-list 118 /
  type-valued 1`），新那支落 `plain`（`tests/unit/test_backup_integrity_l243.py:322`）。㈣ `test_api_security.py`
  的 `PROTECTED` 进 `("POST", "/api/system/backups/cleanup")`、`OPEN_ALLOWLIST` 进
  `("GET", "/api/system/backups/verify")`——归类判据是「会不会落盘 / 改状态」，不是 HTTP 动词。㈤
  `test_api_key_channel_l233.py` 的 `GATED_TOTAL` 与 `GATED_CONSUMED` 24 → **25**、`CONSUMED_AND_GATED`
  +cleanup、两支函数名与 docstring 同数，`Settings.tsx` 卡上那句影响面（75 → 77 条操作、24 → 25、
  「备份那三只」→「备份那四只」）与 `Settings.test.tsx` 的三处断言同轮跟上——守卫用 f-string 把两处绑死，
  不同轮改必红。㈥ `test_backup_index_atomic_l162.py` 的读侧哨兵由 `json.JSONDecodeError` 换成
  `BackupIntegrityError` 并补「输出文件不存在」那一维。**这不是放宽**：那条判据的原话是「读侧响亮失败、
  不吞成空数据」，两只同为 `ValueError` 的子类，而新哨兵排在解析与写盘**之前**，多出来的一维比改前更严。
- **⑨ 账本三处历史行号引用重钉**（插行平移那一族，本轮第 3 次踩）：`System.tsx` 那一处旧引用（432 行档）改成
  505 行（两处）、`docs/augmentor/API.md` 的旧落点改成 `docs/augmentor/API.md:1139`（三处）、本账「第 734 行那条」改成
  「第 773 行那条」（两处）。这里有一条**尺子反过来咬记账文笔迹**的现量：a184 会把账本每一行里「文件名字面量
  后面紧跟冒号和行号」的形状都当活引用量一遍，所以「旧引用是 432 行」这句说明只要写成那个形状，当场就成了
  一个新的 `dead_line`（本轮第一跑就红在这一格，连改一次都还没改完又红第二次），只能写成「432 行」这种
  不带文件名的读法。处方按优先级走的是**改引用而不是挪代码**：本轮 `System.tsx` 只在 830-833 那一段同行追加
  一句、`docs/augmentor/API.md` 同行追加两格，两处的净行数都是 **0**（`git diff --numstat` 复核过）。
- **⑩ 过程中查出 a184 的一处深度缺陷，另立 A331**：它的 `dead_line` 档只判「引用落点在不在代码跨度里」。
  实测改前那一格在 HEAD 上落进 quality 的一段 JSON 示例，而 L232 那格的正文写着「引用仍指得准」⇒
  **尺子全程绿、话是假的**。落点对、内容错比落点错更危险：落点错会响，内容错只会让后来人以为已经复核过。
  本轮把它按现量钉到真实落点并逐字读过那一行。
- **⑪ l242 那把尺子的名册重钉**：表行 248 → **251**（新三行）、有标记 132 → **134**（A299 与 A300 各带一格
  `~~X~~ 已收口（L243）`）、声明面 116 → 现量（本轮实测，见下面的复跑格）。`PARTIAL` 三格（A127 / A156 / A45）
  一字未动，判据仍写成**名册精确相等**。另有一条写作时自己踩到的形状：**清单行与日志标题行是声明面**，
  所以这两行里写「A330 … 收口」这类形状会把一笔待拍的账拉进声明面、当场造出假欠账——本轮三条新账因此在
  清单行里一律写成「Axxx（限定词）」的括号形状（括号是 AFTER 式的断口），日志块正文不进声明面所以能正常
  叙述。这与 L241 那条「钉尺子必须点名它量的是哪一条路」同族：尺子的口径会反过来约束记账的文笔。
- **⑫ 全量门禁 #1（补收尾臂之前）与覆盖率倒退的归因**：定标命令整跑 `exit=0`，`TOTAL` 行
  **14 332 语句 / 13 未覆盖 / 3 954 分支 / 22 未覆盖分支**、`Required test coverage of 80% reached.
  Total coverage: 99.81%`、**8 919 passed / 0 failed / 10 skipped / 6 warnings in 277.20s**、
  `111 files skipped due to complete coverage`。与 L242 终态的 99.84 % 比退了一档，多出的 **5** 条
  未覆盖语句被 coverage 表点名在一处：`api\routes\system_ops.py` 那一行的 `753-756, 784`——正是本轮
  两只新端点的收尾臂（verify 的 `except HTTPException: raise` 与 `except Exception → to_http_error`、
  cleanup 的 403 重抛臂），cleanup 的兜底臂并不缺，因为「0 / -1 ⇒ 400」那两支走的正是它。**没有把这
  五臂记成一格「边角料不计较」**：其中两条是安全语义的落点（越界必须仍是 403 原文、未分类异常不许被
  `str(e)` 带出部署路径，A155①），同族既有四员的这两臂各有用例，唯独新两员没有就是半成品。补的是
  点名用例，不是豁免。
- **⑬ 全量门禁 #2（补三支之后）**：同一条定标命令 `exit=0`，`TOTAL` 行 **14 332 / 8 / 3 954 / 22**、
  `Total coverage: 99.84%`、**8 922 passed / 10 skipped / 6 warnings in 278.74s**、
  `112 files skipped due to complete coverage`。语句总数与分支总数一字未动（本轮那一步只加测试、
  没动产品码），未覆盖语句 13 → **8** 与 L242 终态逐格相同 ⇒ 倒退消除。`111 → 112` 那一格是同一件
  事的另一把尺子：`api/routes/system_ops.py` 从「有余量」挪进了「全覆盖」名册，两把尺子互相咬合。
  三支新用例（越界 403 原样重抛 × 2、未分类异常 ⇒ 500 且响应体不含原文 × 1）与既有 29 支同文件单跑
  **32 passed**。
- **⑭ 用例数增量闭合**（不采信「大约」）：以 HEAD 的 detached worktree 同命令逐文件采集对照，base
  **8 893 collected / 8 883 passed / 10 skipped** ⇒ new **8 932 collected**（`--collect-only` 现量）
  **/ 8 922 passed / 10 skipped**，差 **39** = 新守卫 **32** + `test_api_openapi_contract.py` **+4**
  （两员进契约面）+ `test_exceptions.py` **+3**（`BackupIntegrityError` 层级那一族）。对照用的临时
  worktree 用完即 `git worktree remove`，`git worktree list` 复核过已干净、未碰他人那两个工作树。
- **⑮ 其余两面读数**：前端三门禁 `npm run typecheck` rc=**0**、`npm run lint` rc=**0**、
  `npm run test` **Test Files 22 passed (22) / Tests 307 passed (307)**（Duration 97.40s）；七把账本与
  文档守卫（六把常规 + l242）本轮改文前后各跑一次都是 **173 passed / 8 skipped**，8 支 skip 全是 l79
  的 `authoring_tree_present` 那一族（干净克隆里作者工作树不在场，与本轮无关），没有任何一把尺子被
  改松。
- **⑯ 账本批（`a76ae612b`）之后同一条定标命令复跑**：`exit=0`，**8 922 passed / 0 failed /
  10 skipped / 6 warnings in 281.55s**，`TOTAL` 行 **14 332 语句 / 8 未覆盖 / 3 954 分支 /
  22 未覆盖分支**、`Total coverage: 99.84%`、`112 files skipped due to complete coverage` ⇒
  四格与账本批之前（⑬）逐格相同、**无偏差**，本轮终态即 ⑬ 那一格。278.74 s → 281.55 s 差
  +2.81 s，同机同码，按 L239 / L240 两轮量过的负载差档（±46 % / −18 %）算噪声，不作任何结论。

### L244（2026-10-12）— CLI 备份补上 verify / cleanup 两档：三面同源钉成双射，A329 收口并订正它立账时的「当前处置」

**本轮定义（动手前拍的）**：收 **A329**——`backup --action` 的 choices 由四档补到六档，与 L243 那两只
新端点共用同一份 SDK 权威，重钉 CLI↔HTTP 覆盖面守卫与文档口径。待拍的 A298 / A301 / A303 / A304 /
A330 / A331 与性能档 A326 / A327 一律不动；本轮新立一条 A332（CLI 只读档的落盘面）。

- **① 产品面三处**：㈠ `augmentor/cli/parser.py`——choices 补 `"verify"` / `"cleanup"`，`--action` 那一行
  仍在第 **394** 行（本轮新增的 `--max-backups` 落在它**之下**的第 399 行，净 **+1** 行），子命令 help 改成
  「数据备份（verify 查到损坏时退出码 1）」。㈡ `augmentor/cli/commands/version.py`——`run_backup` 的
  import 名单从四只补到六只，新增 `verify` 与 `cleanup` 两分支，`verify` 末尾走 `verdict_exit(not broken)`，
  模块级 `from ..verdict import verdict_exit`（净 +35 行）。㈢ 前端 `web/src/pages/System.tsx` 的「数据备份」
  卡与 `web/src/services/api.ts` 的注释把「CLI 没有这两档」按现量改成六档，两处**净 0 行**（同行改写）。
- **② 五档退出码实测**（`verdict.py` 的三种形状，每档各有指名用例）：完好 ⇒ 无 SystemExit（0）且两行形状
  `  - a: 完好（1 条，…）` + `1 个备份全部完好`；校验和不匹配 ⇒ **1** 且 stderr **没有**「错误:」；零字节
  ⇒ **1** 且单列「文件丢失或为空」（不是「校验和不匹配」）；幽灵名 ⇒ 0 且 stdout 逐字「没有找到备份（索引里
  没有 ghost）」；`cleanup --max-backups 0 / -1` ⇒ **1** 且 stderr 逐字「错误: max_backups 必须是不小于 1
  的整数，当前是 0」、目录**不在场**（判参先于构造）；`--action nope` ⇒ argparse **2**。
- **③ 三面同源判据写成双射的三个方向**：㈠ choices ↔ `run_backup` 源码里的 `args.action == "…"` 分支集合
  （正则派生，两个方向的差集都印进断言消息）；㈡ choices ↔ `ACTION_TO_ENDPOINT` 键集 ↔ OpenAPI 里以
  `/api/system/backups` 开头的**全部**操作集，并额外钉「表里两条档不许指同一只端点」（双射的另一半）；
  ㈢ 每档调用的 SDK 入口与对应端点路由调用的那只**同名**——两侧都从 AST 读 `from augmentor.backup import …`
  的名单，不抄第二份清单。另钉 `--max-backups` 的默认值在 CLI / HTTP 请求模型 / SDK 形参三处同为 **10**，
  与 `--help` 里那句「退出码」宣称（`test_cli_verdict_wiring.py` 同族口径：接了却不宣称 = 文档撒谎）。
- **④ 注入取证两轮，各有牙齿**：轮一是本文件内的**三臂零写入**（`monkeypatch` 打在导入后的模块对象上，
  仓库字节一字不动）——摘 `verify_backup_integrity` ⇒ verify 只剩「没有找到备份」且退 0；摘 `verdict_exit`
  ⇒ 坏备份从 1 变 0 而**文案一字未变**（正面证据：那个 1 确实由判决产生，不是崩溃）；摘 `clean_old_backups`
  ⇒ 明明两份却说「删除 0 个」、索引 `[a, b]` 原样。轮二是仓外探针的**六臂字节级**改写（`/d/WORKSPACE/l244_inject.py`，
  每臂写完立即读回比对哈希再复原）：**A 摘 choices 里的 verify / B 把 handler 的分支名改成 `verifyx` /
  C 摘 SDK 清理入口的 import / D 摘 help 里的退出码宣称 / E 把默认值 10 换成 5 / F 名册里删掉 cleanup 那一格**
  ⇒ 六臂逐臂 `rc=1`、逐臂读数 `1 failed, 19 deselected`，复原后 `git status` 对本轮八个文件逐格干净、
  无任何注入残留。**A 与 B 红的是不同的两支**——这正是「清单式判据防不住多一档不接线」的现量证据。
- **⑤ 量到一处真实的三面不对称，立 A332**：CLI 的只读档（`verify`）在目录不存在时**会**把目录建出来并落
  一份 `index.json`，HTTP 面同一只 verify **不建**（路由里那道 `if not resolved.exists()` 挡在构造之前）。
  处方按「不擅自扩大改动面」走：既有的 `--action list` 一直就是这个形状（同一只 `DatasetBackup` 构造），
  本轮不是新引入的缺陷，所以把现量钉成用例、给不对称立账待拍，而不是顺手改 SDK 的构造语义。
- **⑥ A329 立账时那格「当前处置」两格都不成立，本轮订正**：它写着「守卫按现量钉住 CLI 的 choices 恰为那
  4 档」与「页面卡片与 `docs/augmentor/API.md` 各写明『CLI 没有这两档』」。开工前 grep 现量：全仓**没有任何**守卫钉
  CLI 的 choices，页面卡片也**没有**那句话（只有本账自己写着）。落笔时把「打算做的守卫」写成了「已就位的
  守卫」——与 **A331** 同族（写的与量的不是一件事），差别是这条没有尺子兜底，纯靠人眼。本轮补真守卫并把它
  钉成六档，A329 行内按现量写明订正，不追认旧话。
- **⑦ 按现量重钉的尺子**（**没有任何一把被改松**）：㈠ `test_parametrize_ids_l97.py` 的 CALLS 431 → **432**、
  READABLE 313 → **314**、盲面 **118** 一格未动。读数出自仓外探针 `l244_census.py`（直接 import 那把尺子自己的
  `param_sites(TESTS_ROOT)` 逐处打印桶归属）：新那处 `tests/unit/test_cli_backup_actions_l244.py:338`
  `bucket=plain ids=False`，桶分布 `plain 313 / no-literal-list 118 / type-valued 1`；l98 侧 `blind_rows` 仍
  **118**、档位分布 `plain 291 / ids-literal 25 / ids-derived 11 / derived 94 / opaque 11`，`MEASURED_TIERS`
  四格一字未动。㈡ `test_backlog_closure_l242.py` 三格重钉（表行 251 → **252**、有标记 134 → **135**、
  声明面 116 → **117**；`PARTIAL` 三格未动）——新账 A332 在清单行里写成括号形状避开口径 5 的 AFTER 式。㈢
  A285 行与 L227 日志块里那两处 `parser.py` 的历史行号引用按**内容**重钉：405 → **406**、430 → **431**
  （`--max-backups` 那一个净 +1 行把它们平移走了；173 那一格在插入点之上，未动）。**a184 与 l79 两把尺子
  全程绿** ⇒ 这是 A331 那条盲区（只判落点在不在代码跨度、不判落点内容）的**第 2 次现量**，第 1 次是 L243 的
  `docs/augmentor/API.md` 那一格。㈣ 同一条盲区还咬到一处**不带文件名的散文行号**：L232 清单行写着「本账第 773 行那条
  `docs/augmentor/API.md:1139` 引用仍指得准」，本轮插行之后那条实际在第 **781** 行 ⇒ 一并按现量重钉，并在 L243 ⑨ 那格
  的历史陈述后面补一句现量（不改写它当时改成过什么）。
  **L245 复核**：那一条今天在第 **786** 行（L245 在它之上落了清单行、Backlog 新账与它的分隔空行，共 3 格），散文行号仍无尺子量得到 ⇒ 见 A333。
  **L246 复核**：那一条今天在第 **789** 行（L246 在它之上落了清单行与 A334 那笔新账及其分隔空行，共 3 格），散文行号仍无尺子量得到 ⇒ 见 A333。
- **⑧ 文档面**：`README.md` 的 CLI 索引 help 逐字改成「数据备份（verify 查到损坏时退出码 1）」，示例区在
  `--action delete` 之后补 `verify` 与 `cleanup --max-backups 10` 两行。这一格的第一版**漏改了 help**，被
  `test_readme_cli_index_l204.py` 当场红（**1 failed / 92 passed**）——文档与 parser 的漂移有尺子兜着，
  这条不是靠人眼发现的。`web/src/pages/System.backup.test.tsx` 的旧断言（页面必须说 CLI 只四档）翻面成
  「必须说六档 + 退出码 1」，并各补一条 `not.toContain` 钉旧话已下线。
- **⑨ 中间读数原样入册，不美化**：改完 CLI 之后第一次跑普查守卫是**红的**——`test_parametrize_ids_l97`
  与 `test_parametrize_blind_face_l98` 各报一支 `assert 432 == 431`（两把尺子在同一格互相咬合，任一边漏改
  都红），同一次运行另有 **5 个 error**：那是 l98 的动态面护栏 `MIN_COLLECTED_ITEMS = 3000`——我只跑两支
  文件时它只采到 **36** 条，于是抛「数不到东西」而不是让「没有风险对象」冒充通过。这一族不是失败，是护栏
  该有的反应，逐格全量复跑后消除（见 ⑩）。
- **⑩ 全量门禁 #1（账本批之前）**：定标命令 `python -m pytest tests -q -p no:cacheprovider` ⇒
  **8 942 passed / 0 failed / 10 skipped / 6 warnings in 275.33s**、`Total coverage: 99.84 %`、`TOTAL` 行
  **14 353 语句 / 8 未覆盖 / 3 966 分支 / 22 未覆盖分支**、`112 files skipped due to complete coverage`。
  用例数自己闭合：**L243 终态 8 922 + 本轮新守卫 20 = 8 942**，没有一支既有用例被挤掉；10 只 skip 名册与
  L243 逐格相同（l79 的作者工作树 8 支 + `test_config_gates_l82.py:573` + `test_upload_ceiling_l87.py:580`）。
  产品面新增 **21** 条语句（14 332 → 14 353）而**未覆盖仍是 8**，与 L242 / L243 终态逐格相同 ⇒ 本轮新增代码
  100 % 被覆盖；coverage 表里 `augmentor\cli\commands\version.py` 那一行现量 **119 语句 / 0 缺数 / 66 分支 /
  2 偏支**（两条偏支是 `99->exit`、`183->exit`，与本轮无关的既有形状）。分支总数 3 954 → 3 966（**+12**）
  正是两分支新带的判断。定向面另跑：本轮相关的七支文件（新守卫 + l243 + l231 + l204 + l97 + CLI 接线两支）
  **157 passed**。
- **⑪ 前端面与账本守卫**：`npm run typecheck` rc=**0**、`npm run lint` rc=**0**、`npm run test`
  **Test Files 22 passed (22) / Tests 307 passed (307)**（Duration 122.54 s）——用例数与 L243 同数，本轮前端
  只改断言与注释、不加支，所以这一格「同数」是**预期**而不是漏跑。七把账本与文档守卫（六把常规 + l242）在
  账本改文之前跑的是 **173 passed / 8 skipped**；改文之后的复跑见下一格。
- **⑫ 账本改文之后的复跑**：七把账本与文档守卫同一条命令再跑 ⇒ **173 passed / 8 skipped**，与改文前**逐格
  同数**（8 支 skip 全是 l79 的 `authoring_tree_present` 那一族，干净克隆里作者工作树不在场，与本轮无关）。
  卷二的行尾仍是**纯 LF**（本轮所有插行与同行追加都在 LF 面上做，`test_docs_markdown_structure` 的行尾维绿），
  `git diff --numstat` 复核（对代码批 `2a04e1ed3` 跑 `HEAD~1 HEAD`，不是对索引）：`System.tsx` **2/2**、
  `api.ts` **1/1**（两处都净 0 行，同行改写）、`parser.py` **3/2**（净 +1，落在被引用行之下）、
  `README.md` **3/1**、`version.py` **36/1**。中间那一次 l242 的三格红读数（`252==251` / `117==116` / `135==134`）
  按 ⑨ 的同一口径原样留在账上，重钉后的数出自那一次运行本身。
- **⑬ 账本批（`1a431da73`）之后同一条定标命令复跑**：`exit=0`（这次把 rc 显式接进读数链，不留「大概为 0」），
  **8 942 passed / 0 failed / 10 skipped / 6 warnings in 287.92s**，`TOTAL` 行 **14 353 语句 / 8 未覆盖 /
  3 966 分支 / 22 未覆盖分支**、`Total coverage: 99.84%`、`112 files skipped due to complete coverage`
  ⇒ 四格与账本批之前（⑩）**逐格相同、无偏差**，本轮终态即 ⑩ 那一格。275.33 s → 287.92 s（+12.59 s），
  同机同码，仍按 L239 / L240 两轮量过的负载差档算噪声，不作任何结论。
### L245（2026-10-12）— 给 a184 的 dead_line 档接上内容维：12 条「落点对、内容错」的活引用第一次被尺子点名

**本轮定义（动手前拍的）**：收 **A331**——`tests/unit/test_comment_refs_a184.py` 的 `dead_line` 档只判
「引用落点在不在代码跨度里」，量不到「落点是代码，但不是它所点名的那个符号的家里」。待拍的 A327 / A326（性能档）
与 A298 / A301 / A303–A305 / A328 / A330 / A332 一律不动。

- **① 为什么这一格值得做**：这一族已经撞过两次，两次都是人眼撞的。L243 那次是账本 L232 那一格写着
  `docs/augmentor/API.md:1139` 并称「引用仍指得准」，落点确实是代码而内容不是那件事；L244 那次是 `parser.py` 净加一行
  把两处历史行号平移走，落点仍在代码跨度里 ⇒ 两把尺子全程绿。`file:NNN` 是账本「可复核」的唯一机械通道，
  落点对而内容错的引用比落点错的更危险：后者会响，前者只会让后来人以为已经复核过了。

- **② 判据口径是三档归因实验量出来的，不是拍脑袋选的**（仓外六支探针全部 `import` 守卫自己的函数现量读数，
  不猜桶归属）：
  - **整行归因**（把引用所在整行里所有反引号标识符都当主张）：案面要背 **3 229** 个标识符，产品注释面
    「该行所有名字都命中」只有 **15/48** ⇒ 假红成灾，弃。
  - **句内归因**（只取引用所在子句）：仍造出 **6 条假红**——点名的多是模块级常量、配置键名，不是 `def` / `class`
    ⇒ 弃。
  - **括弧首归因**（引用旁紧跟的后括弧，或所在的不含嵌套外括弧，取括弧**第一个**反引号跨度）：必须是裸标识符、
    必须是目标文件某个 `def` / `class` 的名字 ⇒ 案面收敛到 **24 条主张**（卷一 22 / `docs` 1 / 测试 1），
    其中 12 绿、**12 红**。这一形只问「引用自己声明它指哪个符号」，所以判据窄而咬得住。

- **③ 立档时案面 12 条红，逐条按 AST 现量重钉**（新档 `dead_anchor`，`CEILING` 等值 0）：卷一 A28 的
  `_search_fuzzy` 308-360 ⇒ 491-553、A38 的 `_find_similar_pairs_chunked` 607-653 ⇒ 627-672、A56 的 `run_split`
  40-50 ⇒ 42-55、A76 的 `_load_section` 807 ⇒ 973、A80 的 `WebConfig` 176-196 ⇒ 703-784、A89 的
  `run_validate_config` 88 ⇒ 92、A95 的 `_validate_env_refs` 449-467 ⇒ 880-900、A102 与 A106 的
  `apply_logging_config` 132 ⇒ 163（两处）、A147 的 `read_chunks` 274 ⇒ 296；`docs/augmentor/ARCHITECTURE.md` 的
  `assert_format_renderable` 95 ⇒ 108；`tests/unit/test_system_ops_wiring_l234.py` 的 `read_chunks` 274 ⇒ 296。
  **12 条全部是「落点是代码，但不是所点名的 def/class 的家里」**，没有一条是路径坏或行号越界——这正是 `dead_line`
  档的盲区，也是本轮要量的那一格。

- **④ 另 3 处由人眼补的假句**（尺子判不到的形状，见 A333）：卷一 A76 那一格里「调用点」那格裸行号 1005 ⇒ **1163**
  （`_load_section` 在 config 里的唯一调用点，本轮按 grep 现量核对）；ARCHITECTURE 第 1163 行说 `ModelConfig`
  在 config 的第 15 行 ⇒ **176**，又说两个重试旋钮属于第 28 行那个类 ⇒ 按现量改成 **249** 的 `AugmentationConfig`。
  这三处的共同点是**没有括弧首的裸标识符可取**，新判据对它们结构性失明。

- **⑤ 核对而未重钉的一格（写进 A333 当已知样本）**：卷一 A95 那一格同时用了点号主张与三个裸行号，本轮核对现量
  三个数字**全部陈旧**——`_resolve_env` 的 def 现在 `augmentor/config.py:864`、未设置回落空串那一行在 886、
  当年「三处调用点」已并成 `for` 循环里的一处 949。不顺手改的理由与 A331 修法② 同一条：改它要连整句散文一起重写，
  而改完仍然没有任何尺子能验，写进账比写进散文更能让下一轮看见。

- **⑥ 净行数复核**：三处落点全部**只在行内换数字**，`git diff --numstat` 交出 `OPTIMIZATION_LOOP.md` 10/10、
  `docs/augmentor/ARCHITECTURE.md` 2/2、`tests/unit/test_system_ops_wiring_l234.py` 1/1 ⇒ 净 0 行，插行平移族（L217 / L219
  两次实测的那条缺陷类）本轮没有对象。批写脚本每处断言锚点在该行恰好命中一次，写完读回逐处验新串在场、旧串消失、
  行数不变、无 CRLF。
  另：卷二这一侧本轮在它之上落了 3 行（清单行、Backlog 新账、以及新账前那个分隔空行——这张表近期每行之间
  空一行是既有惯例，省掉它 `find_tables` 会把相邻两行读成一张没有分隔行的表，markdown 结构守卫当场红），于是
  L232 清单行里那句「本账第 781 行」被推到 **786** ——同一个散文行号三轮三个值（773 ⇒ 781 ⇒ 786），而这一形不带文件名，
  a184 的结构维与内容维都判不到 ⇒ 本轮按现量重钉，并把成因写进 A333；L244 ⑦ 那格后面只补一句复核现量，
  不改写它当时的陈述。

- **⑦ 中间读数原样记账（不美化）**：守卫改完首跑 `2 failed, 20 passed, 5 skipped`，硬 0 那一支的失败信息逐条点名
  12 处并给出真量跨度（`assert 12 == 0`）；第二支红是我自己写的注入判据 bug——名册解包时写成 `_real_rows` 又在下一行
 引用 `real_rows`（`NameError`）。更早一处自抓：注入臂起初按**绝对计数**写 `len(anchor_rows) == 1`，在案面尚有 12 格
  真红时必然假红 ⇒ 改成增量比 `len(anchor_rows) - real_anchor == 1`，并留注释「别轮若留下真红，这一格仍要精确交出
  合成语料只造了 1 格」。还有一处方向性错误：四支新判据起初都挂了 `require_authoring_tree()`，而本机 `Temp/` 不在场
  ⇒ 它们会自我跳过、注入取证白做；判据量的是增量对账、不吃作者底料，所以把那 4 行删掉并在类 docstring 写明理由。

- **⑧ 代码面**：`test_comment_refs_a184.py` 新增 `def_spans`（AST + `lru_cache`，只解析 `.py`，回
  「符号名 → 跨度名册」）、`claim_paren`（后括弧优先，其次不含嵌套的外括弧）、`anchor_state`（回
  主张与否 / 判决 / 名字 / 现量跨度），`audit()` 在 `dead_line` 判过之后接线内容维；`BUCKETS` 加 `dead_anchor`，
  `FLOOR` 加 `anchor_claims: 20` 作反空转下界（名册不钉死，理由见 A331 那一格）。docstring 的「可证失效三档」改四档，
  口径 5 记录判据与三条判不到的边界。

- **⑨ 尺子与普查按现量重钉，没有一把被改松**：l242 三格随本轮 A331 收口与 A333 新立重钉（表行 252 → **253**、
  有标记 135 → **136**、声明面 117 → **118**，三格都由红读数当场逼出）；l97 普查 CALLS **432** / READABLE **314** / 盲面 **118** 一格未动（本轮零新 `parametrize` 调用点，
  只加函数与类），由全量门禁原地绿证实；a184 自身 census 终态 `anchor_claims` **24**、`dead_path` / `dead_line` /
  `dead_anchor` / `symbol_dead` 四档全 **0**、`refs_total` 4 976 / `files_seen` 573。

- **⑩ 门禁**：提交前七把账本与文档守卫 `178 passed, 8 skipped`（skip 全在 l79 家族，作者底料 `Temp/` 不在场，
  与 L243 / L244 同档）；全量定标 `python -m pytest tests -q -p no:cacheprovider` **exit=0**、
  **8 947 passed / 0 failed / 10 skipped / 6 warnings in 282.59 s**、`TOTAL` 行 **14 353 语句 / 8 未覆盖 /
  3 966 分支 / 22 未覆盖分支**、`Total coverage: 99.84%`、`112 files skipped due to complete coverage`。
  用例数闭合 = L244 终态 8 942 + 新判据 5 支（`TestHardZero` 1 + `TestContentDimension` 3 + `TestRulerProvesItself` 1）。
  `TOTAL` 四格与 L244 逐格相同 ⇒ 本轮零产品代码改动，覆盖率面未变，没有倒退要补测。

- **⑪ 前端三门禁不适用**：`web/` 本轮零改动（工作树里 augmentor 只有三份 `.md` 与两份 `.py`，无 `.ts` / `.tsx`），
  按纪律记「不适用」而不是漏跑。
- **⑫ 自抓一次 markdown 结构红（中间读数不美化）**：卷二 Backlog A 表近期的惯例是每行之间空一行，A332 原本是
  表尾单独一行、下面接空行 ⇒ 单行不构成表；第一版把 A333 直接贴在 A332 之后，`find_tables` 就把这两行读成一张
  没有分隔行的表，`test_tables_have_header_separator[OPTIMIZATION_LOOP_2.md]` 当场红（那一次七把是
  **4 failed / 174 passed**，另三格正是 l242 等着重钉的计数）。处置是 A333 前面补一个空行，并由此把散文自指
  行号再多推一格（见 ⑥）。**这是插行平移族的又一次现量，只不过这一次插行发生在账本自己身上**——而 markdown
  那把尺子兜住了它，说明「账本插行也要跑结构守卫」不是可选项。
- **⑬ 账本批（`8e3356d89`）之后同一条定标命令复跑**：**exit=0**、**8 947 passed / 0 failed / 10 skipped /
  6 warnings in 283.19 s**，`TOTAL` 行 **14 353 语句 / 8 未覆盖 / 3 966 分支 / 22 未覆盖分支**、
  `Total coverage: 99.84%`、`112 files skipped due to complete coverage` ⇒ 四格与账本批之前（⑩）**逐格相同、
  无偏差**，本轮终态即 ⑩ 那一格。282.59 s → 283.19 s（+0.60 s），同机同码，仍按 L239 / L240 两轮量过的
  负载差档算噪声，不作任何结论。

### L246（2026-10-12）— 收 A327：多样性那一格的行范数不再逐候选重算，四档判决产物逐字节相同

**本轮定义（动手前拍的）**：收 **A327**——`QualityScorer._calculate_diversity` 的向量分支把参照集矩阵的行范数写在「每候选一次」的那条除法里，并把 A327 原格点名要的
「缓存失效边界」这句话一起交掉。待拍的 A326（稠密路两遍扫描）与 A298 / A301 / A303–A305 / A328 / A330 / A332 / A333 一律不动。

- **① 为什么这一格值得做**：这是纯内部改动里最典型的「尺子全绿而成本每候选付一遍」的形状——数值一路正确，缓存命中与否没有任何机械通道出声，所以 L239 只能把读数钉进账、
  由本轮把它变成**计数判据**。真实语义模型下（窗口 100 / 维 384）这一格占单次 `batch_score` 全程的 **13.7%**，比 FakeModel 小窗口那一档（7.6%）大出一倍，这正是 A327 原格
  说「收益会随 n×dim 放大」的那件事，本轮把它量成了现量而不是推测。

- **② 改前现量按四档形状各量，没有把一档结论外推到另一档**（仓外探针 `D:\WORKSPACE\l246_probe.py`，FakeModel 用 `zlib.crc32` 顶掉 sentence-transformers，`include_semantic=False`
  让语义维度不掺进向量侧，`_cross_encoder = "fallback"`）：
  - **参照集稳定**（`threshold=0.999`，窗口 30 已满）：axis=1 **4 000 次** → 改后 **1 次**，wall 407.7 → 376.6 ms。
  - **参照集逐条达标**（`threshold=0.0`，窗口 30 已满）：axis=1 **4 000 次** → **1 次**，wall 406.8 → 359.8 ms。这一档证明 L239 那条「`encode` 只被调 3 次 / 8 030 行」的读数
    不是本轮的对照——`diversity_sample_size` 把参照切片钉死在窗口长度，`existing.append` 在窗口已满时不改变切片 ⇒ 门槛放行与否**不引入重编码**，两档的编码次数逐格相同。
  - **生产第一批**（`refs_n=1`、窗口 30、`threshold=0.0`）：编码 **31 次**（整批 4 000 行 + 参照切片 2…30 共 30 次），axis=1 **4 000 次** → **30 次**（= 参照切片编码次数），
    wall 430.4 → 404.8 ms。这一档才是 `existing_generated` 为空或极短时的真实形状，范数收益落在「编码次数」这一侧而不是候选数那一侧。
  - **真实语义模型量级**（窗口 100 / 维 384）：axis=1 **4 000 次** → **1 次**，wall 459.4 → 396.6 ms。
  - 插桩本身按 axis 分桶计数（`axis=1` 与其余），所以「改后 axis=1 = 1」与「逐候选那个向量自身范数仍然 4 000 次」是**两个独立读数**，不是同一件事的正反面。

- **③ 改法与「位相同」的口径**：四处改动全在既有行上——`__init__` 与 `reset_cache` 把 `_existing_embeddings` 那行改成元组赋值，`_calculate_diversity` 里在编码之后加一行
  `self._existing_norms = np.linalg.norm(self._existing_embeddings, axis=1)`，用法那一行把 `np.linalg.norm(self._existing_embeddings, axis=1)` 换成 `self._existing_norms`。
  位相同的论证不是「应该一样」而是「同一条 numpy 调用、同一份输入、乘法次序未变」，取证走两条腿：仓外四档 4×4 000 格 `repr` 落盘逐字节比（sha256 前后同为 `798206eed79354a5…`），
  仓内 `test_cached_path_reproduces_the_inline_formula_bit_for_bit` 照**改前那条式子**现算一遍与走缓存的产物逐位比，另加一支 `test_warm_cache_matches_a_cold_scorer_per_candidate`
  把「整批走缓存」与「每候选换新 scorer」（= 改前每候选必然重算的形状）逐条比。**A327 原格里那条 `np.einsum` 否决维持不变**：它量到的 3.73× 会带来 float32 最大差 2.98e-07，
  按本仓「性能改动必须产物逐字节相同」的口径不予采纳，本轮也没有顺手做。

- **④ 净零行的处置（处方优先级走第 2 条）**：`quality.py` 身上挂着十余处 `quality.py:NNN` 历史行号引用（A327 原格自己列的 417-433、312、133/153、85-89、81、80 等），
  第一版在 `__init__` 多插了两行注释 ⇒ 立刻收回，改走「同行压缩 + 行内替换」：把 `_model.encode(` 的三行参数压成一行，给新增的范数刷新那一行腾位，注释压进被替换行的行尾。
  `git diff --numstat` 复核 **7/7 = 净 0 行** ⇒ 本轮零插行，七把账本与文档守卫（含 a184 硬 0 档与 `test_doc_line_refs_l79`）未因平移出一格红。**注意这条「守卫没红」证明的是
  不平移，不是判据完备**：`quality.py:225` 那句「入口 `:181`」与「偏支 222->230」是**不带文件名的裸行号**，正是 A333 记的第三种失明形状，尺子从一开始就量不到它们。

- **⑤ 新守卫钉次数，不钉数值**：`tests/unit/test_quality_norm_cache_l246.py` **8 支**，四族各有名——
  `TestRowNormIsComputedOnce`（窗口满档 `axis1 == 1` 且 `other == 50`、编码次数逐格点名 `[50, 8]`；涨窗档 `axis1 == 参照切片编码次数 == 5` 且 `< 候选数`）、
  `TestBitIdentity`（③ 那两条腿）、`TestCacheLifetime`（`reset_cache` 同清两格并**如实记录两格寿命不同轴**：第二趟只重编码参照集 `[30, 4, 4]`，整批向量走 `_embedding_cache`
  那一格、`reset_cache` 不清它 ⇒ 见 ⑦）、`TestInjectionSeamNowNeedsBothCells`（接缝形状）。计数器不是恒真的那格保险是 `test_the_counter_is_not_vacuous_when_the_window_churns`：
  逐候选把 `_existing_texts` 换成不相等的切片 ⇒ axis=1 必须回到 **20 = 候选数**，即改前的形状。
- **⑥ 四臂注入取证**（`D:\WORKSPACE\l246_inject.py`，字节级：读 bytes、断言锚点在原文恰好命中一次、写完读回比哈希、finally 复原并断言与进入时字节一致）：
  ㈠ 把范数挪回逐候选那条除法（= 改前形状）咬中 **4** 支；㈡ 缓存不随编码刷新（留空）咬中 **6**；㈢ 刷新了但值摆错（`* 2`）咬中 **2**（两支数值判据，计数判据按口径**不**红——
  次数对、值错，正是「两维分开钉」要的结果）；㈣ 轴向写反（`axis=0`）咬中 **5**。进入与退出哈希同为 `21745677c1d0…`，复原后复跑 **8 passed**。
- **⑦ 顺带量出的另一笔账 → 新立 A334**：`_embedding_cache`（键 `(batch_key, tuple(文本))`、实例级、跨 `batch_score` 终身累积）`reset_cache()` 一格不清；4 批 × 1 000 条
  `include_semantic=True` 之后 **8 个条目 / 2 048 000 bytes / 8 000 个文本引用**，dim 384 折算 **12 MB**，持有者是 `pipeline.py:110`、`comparison.py:109` 这类实例级 scorer。
  本轮只立账不动手：有界化改变的是「编码次数」，与本轮「逐字节相同 + 净零行」两条口径都不合。
- **⑧ 缓存失效边界这句话按两格分开记**（A327 点名要的就是它）：`_embedding_cache` 的寿命 = **实例寿命**，判据是 `(batch_key, 全文元组)`，`reset_cache()` 不清；
  `_existing_embeddings` / `_existing_norms` / `_existing_texts` 的寿命 = **参照集那一族**，判据是参照切片本身，`reset_cache()` 同清。行范数**不另设判据**，它是矩阵的纯函数，
  只在那一次编码的同一处刷新。两格寿命不同轴这件事，本轮由 `test_reset_cache_clears_norms_along_with_embeddings` 的 `[30, 4, 4]` 读数钉在仓内。

- **⑨ 尺子与普查按现量重钉，没有一把被改松**：l242 三格随本轮 A327 收口与 A334 新立重钉（表行 253 → **254**、有标记 136 → **137**、声明面 118 → **119**，三格都由红读数当场逼出，
  `PARTIAL` 三格与 `MEASURED_PHANTOM` 46 未动）；l97 / l98 普查 **432 / 314 / 盲面 118 一格未动**（本轮零新 `parametrize` 调用点，只加函数与类），由全量门禁原地绿证实；
  a184 走 FLOOR / CEILING 两型（`files >= 400` / `refs >= 3 000` / `symbol_live >= 300` / `anchor_claims >= 20` 与四档硬 0），本轮新守卫不含任何 `file:NNN` 主张 ⇒ 无需抬版。
- **⑩ 门禁**：提交前七把账本与文档守卫（`test_docs_markdown_structure` / `test_ledger_hash_slots_l98` / `test_ledger_progress_consistency_l203` / `test_comment_refs_a184` /
  `test_doc_line_refs_l79` / `test_doc_contract_numbers_l200` / `test_backlog_closure_l242`）**178 passed, 8 skipped**（skip 全在 l79 家族，作者底料 `Temp/` 不在场，与 L243–L245 同档）；
  全量定标 `python -m pytest tests -q -p no:cacheprovider` **exit=0**、**8 955 passed / 0 failed / 10 skipped / 6 warnings in 281.51 s**、`TOTAL` **14 354 语句 / 8 未覆盖 /
  3 966 分支 / 22 未覆盖分支**、`Total coverage: 99.84%`、`112 files skipped due to complete coverage`。用例数闭合 = L245 终态 8 947 + 新守卫 8 支 = 8 955（逐格对得上，没有一支是意外多出来的）。
  `TOTAL` 语句 **14 353 → 14 354（+1）** = 本轮新增的那一次范数刷新语句；Miss 8 / Branch 3 966 / BrPart 22 三格与 L245 逐格相同 ⇒ 本轮产品改动只加了一条被覆盖的语句，没有留下死支。
- **⑪ 前端三门禁不适用**：`web/` 本轮零改动（工作树里 augmentor 只有 `quality.py` 与新守卫两份 `.py`），按纪律记「不适用」而不是漏跑。
- **⑫ 中间读数不美化**：探针自身先坏过两处——第一版把 `fm.encode` 包成计数函数写成自递归（`RecursionError`），改成 FakeModel 类级计数；digest 取字段用了不存在的 `s.total`
  （实为 `s.total_score`）。第一版基线还只量了两档形状，`refs_growing` 与 `refs_stable` 读数**几乎相同**这件事本身暴露了「窗口已满 ⇒ 门槛放行不引入重编码」这条未写下来的性质，
  于是补了 `refs_churn`（参照从 1 涨到窗口）与 `wide_window`（100 / 384）两档，才把「生产第一批」与「真实模型量级」各自量到。账本批的 `__init__` 注释第一版多插两行，按处方收回压成
  净 0 行（见 ④）。l242 三格第一版按预测值 **254 / 137 / 119** 写，红读数若与预测不符一律以红读数为准。
- **⑬ 账本批（`90aa931b8`）之后同一条定标命令复跑**：**exit=0**、**8 955 passed / 0 failed / 10 skipped /
  6 warnings in 297.16 s**，`TOTAL` **14 354 语句 / 8 未覆盖 / 3 966 分支 / 22 未覆盖分支**（按 `coverage.xml`
  的 `lines-valid` / `lines-covered` / `branches-valid` / `branches-covered` 逐格现量：14 354 / 14 346 /
  3 966 / 3 944 ⇒ 未覆盖 8 与 22）、`Total coverage: 99.84%`、`112 files skipped due to complete coverage`
  ⇒ **四格与账本批之前（⑩）逐格相同、无偏差**，本轮终态即 ⑩ 那一格。281.51 s → 297.16 s（+15.65 s），
  同机同码（账本批只动 `.md` 与 l242 的常量注释行，产品语句数未变），仍按 L239 / L240 两轮量过的负载差档
  算噪声，不作任何结论。**散文自指那一格复核**：本轮追加只落在文件尾部，第 **789** 行那一条未再平移。

### L247（2026-10-12）— 收 A334：`_embedding_cache` 收成「只留最近一批」的有界 FIFO，四档判决产物逐字节相同

**本轮定义（动手前拍的）**：收 **A334**——`QualityScorer._embedding_cache` 是实例级、跨 `batch_score` 终身累积的整批向量缓存，`reset_cache()` 一格不清。按任务点名的顺序，**动手前先把「仓内有哪些测试钉着跨调用复用同一份 `_embedding_cache` 的形状」量清楚**，再决定改法；待拍的 A326（稠密路两遍扫描）与 A298 / A301 / A303–A305 / A328 / A330 / A332 / A333 一律不动。

- **① 动手前的既有形状普查（三处，全是「跨调用命中」的活证据，不是本轮新造的）**：
  - `tests/unit/test_quality_score_opt.py:31` —— 同一批连评两次，第二次**必须白捡**（整批编码不再发生），这一支是 A334 修法 ③「每批重建 scorer / 每批清空」的头号反例。
  - `tests/unit/test_quality_norm_cache_l246.py` 的编码次数读数 `[30, 4, 4]` —— 上一轮 L246 亲手钉的：`reset_cache()` 之后第二趟**只重编码参照集**，整批生成向量仍然命中 `reset_cache()` 没清掉的那份缓存。这一格把「`_embedding_cache` 跨调用、`reset_cache()` 不清它」写成了**声明行为**。
  - `augmentor/comparison.py:237-238` 与 `augmentor/benchmark.py:124` —— 产品侧同一 scorer 上的两处复用（a、b 各一次 / 每次 `run_benchmark` 一批），改前后编码次数都不变，属于「命中形状在场但本轮拿不到收益」那一族。
  - 结论：跨调用命中**有既有测试背书**，所以三档改法里只有 ①（有界化）能既止住驻留又不撤销已声明的行为。

- **② 改前 / 改后现量按四档形状各量**（仓外探针 `D:\WORKSPACE\l247_probe.py`，FakeModel 用 `zlib.crc32` 顶掉 sentence-transformers、dim=64、`_cross_encoder = "fallback"`；驻留读一次三格：条目数 / 键里的文本引用数 / 矩阵 `nbytes` 合计）：
  - **① 逐 chunk 流式**（4 个互不相同的批次、每批 1000 条各评一次；这是 `augmentor/cli/commands/pipeline.py:43` 与 `augmentor/pipeline.py:557` 的 filter_by_quality 形状）：改前 **8 条目 / 8 000 文本引用 / 2 048 000 字节**，改后 **2 / 2 000 / 512 000**。编码次数 **128 → 128**（**零代价**：命中本来就恒 0，纯泄漏被收掉）。同形状折算真实语义模型 dim 384 是 **12 MB → 3 MB**。
  - **② 同一批连评两次**（200 条）：驻留 **2 / 512 000 → 2 / 512 000 不变**，编码 **32 + 30 → 32 + 30** —— 上限 2 恰好等于一批的两个键，同批命中原样保住。
  - **③ a_b_a_b 交错**（两批各评两次）：驻留 **4 → 2**，编码 **124 → 128**（**+4 次整批编码 ⇒ 本轮唯一付出的代价**）。产品侧没有这种形状：`augmentor/comparison.py:237-238` 是 a、b **各一次**（=④），`augmentor/benchmark.py:124` 每实例一批，两边改前后编码次数都不变。这一档单独量出来是为了把「有界化的代价落在哪」写成读数而不是写成猜测。
  - **④ a、b 各一次**（comparison 形状）：驻留 **4 → 2**，编码 **64 → 64**。
  - wall 不是本轮判据（`cached_encode` 那一路的成本被 `show_progress_bar=False, batch_size=32` 钉住，四档的差异只在「少挂几 MB」），所以本轮所有结论都走**驻留三格读数 + 编码次数**这两条精确尺子。

- **③ 改法与位相同口径**：`quality.py` 三处改动 = init 的 `if not hasattr(...)` 两行收成 `getattr(self, '_embedding_cache', {})` 一行（顺带把「驻留上限见下面」写进同行注释）；`cached_encode` 里 9 行注释压成 6 行；插入 4 行（2 行说明 + `while len(...) >= 2` + `pop(next(iter(...)))`）。**逐出方向是最旧**，留下的必须恰好是最近一批的两个键。**数值一字未改**：四档判决产物（每条样本的三个分与两个布尔量）`repr` 落盘逐字节相同，`l247_digest_before.json` 与 `_after.json` 同为 487 191 bytes、sha256 前后同为 `6f405bf0099ef1e9…` —— 本轮属「数值不变而次数变」那一族，所以按 A334 原格预告**另配独立行为差取证**（见 ⑤⑦）。

- **④ 三档改法的取舍（A334 原格给的 ①/②/③）**：走 **①有界化**。③「调用方每批重建 scorer」**由四臂注入的 ㈢ 现场否决**——把缓存作用域收成 per-call 会打掉 **3** 支既有/本轮用例（`test_embedding_cache_exists_after_score`、l246 的 `test_reset_cache_clears_norms_along_with_embeddings` 即 `[30, 4, 4]` 那一格、本轮 `test_same_batch_scored_twice_skips_both_batch_encodes`），跨调用命中是本仓已声明行为，不是可静默撤销的实现细节；②「把 `reset_cache()` 定义成清全部」会把「清不清整批缓存」变成**调用方的义务**，而产品侧一个调用方都没有清过，等于用一条新语义换掉一处泄漏。`reset_cache()` 的语义本轮一格未动，A327 那条寿命边界原样保留。

- **⑤ 上限 2 的耦合怎么钉**：上限 2 与「一次 `batch_score` 最多插 orig/gen 两个键」是同一件事的两面（`cached_encode` 全仓只有 `cached_encode(generateds, "gen")` 与 `cached_encode(originals, "orig")` 两个调用点）。**不写「静态数源码里的调用点」那种用例**——它会随重构失效；改由 ② 那一档的行为差承担：将来谁加第三个键，同批连评两次就退化成全 miss，`test_same_batch_scored_twice_skips_both_batch_encodes`（`sizes == [50, 50]` 然后 `[]`）当场红。逐出方向另开一支 `test_the_surviving_pair_is_exactly_the_most_recent_batch`。

- **⑥ 新守卫 7 支**（`tests/unit/test_embedding_cache_bounded_l247.py`）钉两条**互相独立**的判据：驻留是纯内部事实（数值一路正确、命中与否都不改变判决产物），所以「条目数 / 文本引用数 / 向量字节」三格读数与「编码次数」分开钉，且 ① 那一档另用 `test_residency_does_not_grow_from_the_first_chunk_to_the_last` 钉成 `[2]*5`、`[80]*5`、`[2*40*DIM*4]*5` 三列**逐 chunk 不增长**。命中与 miss 各一支点名：`test_interleaved_batches_lose_the_cross_batch_hit` 把 ③ 的代价（`sizes == [20]*8`）**主动钉进测试**，免得将来有人把它当 bug 修掉而没人记账。`TestNumbersAreUntouchedByEviction` 两支钉「被逐出的批次是重编码，不是从陈旧键里拿」——`test_evicted_batch_is_reencoded_not_reused_from_a_stale_key`（`sizes == [25, 25]`）与「三个外来批次之后的分数 == 全新 scorer」。

- **⑦ 四臂字节级注入取证**（仓外探针 `D:\WORKSPACE\l247_inject.py`，以那 FIFO 四行为锚点、断言锚点在原文唯一命中、每臂写完读回比哈希、finally 复原并断言与进入时字节一致）：**㈠ 去掉淘汰循环**（等价于回到无界）咬中 **4** 支；**㈡ 逐出方向改成最新**（`pop` 最后一个键）咬中 **5** 支；**㈢ 作用域收成 per-call** 咬中 **3** 支（见 ④，这一臂是本轮的**否决读数**）；**㈣ 上限改成 1**（连同批第二个键都留不住）咬中 **3** 支。四臂全部按预期红、无恒真臂，`-rf` 打印的红名册逐臂点名，探针退出后重新验磁盘并与进入时同哈希。

- **⑧ 净零行处置**：`quality.py` 挂十余处历史 `quality.py:NNN` 引用（L246 同族），本轮改动按处方走「同行改写 + 插入 4 行、init 两行收成一行」，`git diff --numstat` 复核 **12/12 ⇒ 净 0** ⇒ 不平移；`quality.py:417-433` 那一格的落点仍在原位。**但这一条不外推**：见 ⑨ 的 skip 归因——本机 a184 的四档精确棘轮不 enforce，所以「七把没红」只证明引用不平移，**不证明**新写的 5 条主张逐条被判 live。

- **⑨ 主张档位与 skip 归因（本轮把两件事都按现量核对，不靠推断）**：
  - 档位读数由仓外**零写入** pytest 插件（`D:\WORKSPACE\l247_askplugin.py`）走 a184 自己的合成语料通道 `probe()` 取：新守卫的 **5 条限定仓内相对路径**主张（`augmentor/cli/commands/pipeline.py:43`、`augmentor/pipeline.py:557`、`augmentor/comparison.py:237-238`、`augmentor/benchmark.py:124`、`api/routes/quality.py:150`）在真身普查里**逐条落 `live`**；同一批主张改成**裸名**时 `pipeline.py:557` 落 **`ambiguous`**（索引按文件名建键，散名可指向多处）⇒ 本轮把守卫里的路径**全部限定成仓内相对路径**，其余四条裸名仍 live 但同样限定，不留「靠运气落 live」的主张。
  - **中途一次无效读数如实记账**：先试过把裸串喂给 `reference_state` / `ask_state`，那一通道绕过了 `REF` 的反引号抽取口径，实测连 `augmentor/pipeline.py:110` 都答 `dead_path`，而普查硬 0 档为 0 ⇒ 判为无效、不作结论；改用 `probe()` 后第一版又误读 `rows`（`rows` 只收 dead 档，`live` 不入 rows）为「普查没看见」，最终收成**逐条比 bucket delta**。
  - **skip 归因精确化（订正 L246 那一格的粗话）**：七把运行时的 8 格 skip 不是「全在 l79 家族」，按**报告位置**看确实都显示为 `test_doc_line_refs_l79.py:647`，实际构成是 **a184 的 5 格 + l79 的 3 格**，因为 `require_authoring_tree()` 是从 `test_doc_line_refs_l79.py:647` 导入的。含义：作者底料 `Temp/` 不在场 ⇒ a184 的 `TestRatchetsAreExact` 四档精确欠账棘轮（ambiguous 300 / runtime_ns 0 / scratch 768 / scratch_missing 340）**本机不 enforce**，只有 FLOOR / CEILING 两型（`files >= 400` / `refs >= 3 000` / `symbol_live >= 300` / `anchor_claims >= 20` 与四档硬 0）在本机生效。

- **⑩ 尺子按现量重钉，没有一把被改松**：l242 三格随本轮 A334 收口重钉（表行 **254 未动**、有标记 137 → **138**、声明面 119 → **120**，两格都由红读数当场逼出；`PARTIAL` 三格与 `MEASURED_PHANTOM` 46 未动）；l97 / l98 普查 **432 / 314 / 盲面 118 一格未动**（本轮零新 `parametrize` 调用点，只加函数与类），由全量门禁原地绿证实；卷二第 **790** 行那条散文自指行号按现量重钉（786 → 790，中间 L246 那一轮的 789 是「只核未重钉」，本轮补上并把五个值串写全；重钉落在 L232 清单行的行内，净 0 行）。

- **⑪ 门禁**：提交前七把账本与文档守卫（`test_docs_markdown_structure` / `test_ledger_hash_slots_l98` / `test_ledger_progress_consistency_l203` / `test_comment_refs_a184` / `test_doc_line_refs_l79` / `test_doc_contract_numbers_l200` / `test_backlog_closure_l242`）加本轮新守卫共 **185 passed / 8 skipped**；逐把单跑分解（订正用）为 markdown 58 / l98 19 / l203 7 / a184 22+5skip / l79 33+3skip / l200 15 / l242 24 ⇒ 七把本体 178 passed + 8 skipped，+ 新守卫 7。全量定标 `python -m pytest tests -q -p no:cacheprovider` **exit=0**、**8 962 passed / 0 failed / 10 skipped / 6 warnings in 323.87 s**、`TOTAL` **14 355 语句 / 8 未覆盖 / 3 966 分支 / 22 未覆盖分支**、`Total coverage: 99.84%`、`112 files skipped due to complete coverage`。用例数闭合 = L246 终态 8 955 + 新守卫 7 支 = 8 962（逐格对得上，没有一支是意外多出来的）。`TOTAL` 语句 **14 354 → 14 355（+1）** = 本轮「−1 语句（`hasattr` 两行收成 `getattr` 一行）+ 2 语句（`while` + `pop`）」；Miss 8 / Branch 3 966 / BrPart 22 三格与 L246 逐格相同 ⇒ 新加的两条语句都被覆盖、没留死支。

- **⑫ 前端三门禁不适用**：`web/` 本轮零改动（工作树里 augmentor 只有 `quality.py` 与新守卫两份 `.py`），按纪律记「不适用」而不是漏跑。

- **⑬ 中间读数不美化**：① 第一次 Edit 失败是因为把 `quality.py` 原有那 **9** 行注释抄成 8 行（并掉一行、丢一个顿号）⇒ 拆成三处小 Edit 才落对。② 新守卫首版两处期望写错：`nbytes` 那一格漏乘条目数（应为 `2 * 40 * DIM * 4`）、一处反斜杠续行改写成局部变量。③ 探针首版把 `env` 写成含 `if False else` 的畸形 dict 且缺 `import os` ⇒ 修成 `dict(os.environ)` 并补 `-rf` 打印红名册。④ 普查通道的两次无效读数见 ⑨。⑤ 观察跑 a184 单文件时漏加 `--no-cov`，触发一次 `ERROR: Coverage failure: total of 23`（只污染那次读数，非产品缺陷）。⑥ l242 三格第一版按预测值 **254 / 138 / 120** 写，红读数与预测不符一律以红读数为准。

- **⑭ 账本批（`e563850d3`）之后同一条定标命令复跑**：**exit=0**、**8 962 passed / 0 failed / 10 skipped /
  6 warnings in 452.13 s**，`TOTAL` **14 355 语句 / 8 未覆盖 / 3 966 分支 / 22 未覆盖分支**（按 `coverage.xml`
  的 `lines-valid` / `lines-covered` / `branches-valid` / `branches-covered` 逐格现量：14 355 / 14 347 /
  3 966 / 3 944 ⇒ 未覆盖 8 与 22）、`Total coverage: 99.84%` ⇒ **四格与账本批之前（⑪）逐格相同、无偏差**，
  本轮终态即 ⑪ 那一格。323.87 s → 452.13 s（+128.26 s）：同机同产品码（账本批只动 `.md` 与 l242 的注释加
  两个整数常量，产品语句数未变，`14 355` 逐格相同就是这条的闭合证据），仍按 L239 / L240 两轮量过的负载差档
  算噪声，不作任何结论。**另记一次读数取舍**：代码批之后、账本批之前另有一次同码整跑，给出
  **8 962 passed / 297.32 s** ⇒ 用例数与 `TOTAL` 四格三者完全一致、只有 wall 不同（297.32 / 323.87 / 452.13
  三个值），本轮一律以「计数」为判据、wall 只作背景，这正是 L239 撤掉比值型尺子之后立住的口径。
  **散文自指那一格复核**：本轮追加只落在文件尾部，第 **790** 行那一条实测仍在原位（`sed -n '790p'` 打出的
  正是那句 `docs/augmentor/API.md:1139`），L232 清单行里的 786 → 790 重钉没有被这次追加再平移。

### L248（2026-10-12）— 收 A326：稠密那条复用选路扫描的列号表，四档判决产物 sha256 逐字节相同

① **动手前先普查「哪些尺子已经坐在改动面上」**：`tests/unit/test_dedup.py` 的 L10 峰值护栏
`test_fallback_encode_peak_holds_only_one_matrix`（`peak < 1.5×` 一个矩阵）、
`TestOnlyOneProducerRuns`（四条接缝的调用次数）、`TestInvertedBlocksMatchDenseBlocks`（两条路逐元素相等）、
`tests/unit/test_cost_oracle_routing_l241.py` 的 `MEASURED_PRODUCERS` 名册。这四族把「谁算」「算得对不对」
都焊住了，**唯独没量「同一批字符扫了几遍」**——这正是 A326 的形状：复用失败时数值一路正确、峰值不涨、
选路不变，没有任何既有尺子会出声。新守卫因此只补这一维，不重复别人已经钉的。

② **基线先落盘再改码**：仓外探针 `D:/WORKSPACE/l248_probe.py` 在方法级包 wall + `tracemalloc`，
并把四档 dedup 判决产物写成 `l248_digest_before.json` 的 sha256。基线读数（`_inverted_view` 里含那一趟
`_ngram_entries`，wall 单位 ms）：D1 236.4 / 674.0、D2 338.9 / 820.4、R1 427.9 / 814.1、R2 4 884.4 /
（编码那趟不跑）。四档 digest：`e9722a89d092ebf6…` / `be6c8ac9275a4737…` / `76aeace0d4096f26…` /
`58b568468a3c4978…`。预言机入参名册同时先钉下来：(20000, 22, 1400120000, 200)、(20000, 16, 1043803110, 200)、
(6000, 1296, 46610403, 666)、(3000, 1078389, 1454818, 1000)，判决 `[False, False, False, True]`。

③ **改法形状与「位相同」的论证链**：`_ngram_scan` 只改「谁扫」，不改「扫出什么」——列号按**首次出现顺序**编，
而复用的那一趟与原 `_fallback_encode` 发现遍吃的是同一批 `texts`、同一套 `text[j:j+2]`、同一个遍历顺序
⇒ 两张表**逐键相同**；另一侧「编号本身不参与语义」是相似度是点积、对维度任意排列不变这条老口径。
`vocab=None` 时行为与原实现完全一致（老调用方与语义模型那条都不受影响），所以接缝扩展是纯加法。

④ **三条候选修法的取舍全部按实测**：㈢「只把 readings 与 fold 拆成两步」基线里那一段只值 ~5 %，
落在 min-of-1 的噪声里，不配当一轮；㈡「稠密那条直接散射倒排已算好的出现数组」被峰值否决——nnz 级
int64 索引数组（文档号 + 词条号两份）在 R2 那一档可达矩阵字节的 ~5 倍，与 A19 / L10 记录过的
「单趟合并实测 6.8× 一个矩阵、撞 L10 护栏」同形，本轮不去撞第二遍；㈠「只交列号表」是**内存中性偏省**的那条：
表本来就是稠密那条第一遍要建的，交过去少扫一遍，峰值仍是「一个矩阵 + 一份词表」，与 L10 护栏同形。

⑤ **一次自我否决，由测量抓出来**：第一版把逐趟数组（`doc_arr` / `term_arr`）和列号表一起留到了倒排那条
⇒ R2 峰值 230.5 → **258.6 MiB（+12 %）**，等于「省时间、赔内存」。改成 fold 之后立刻 `doc_arr = term_arr = None`、
判倒排赢时连 `vocab = None` 一起松手，R2 回到 **228.7**（低于基线），D1 / D2 的峰值也从 25.2 / 26.4 掉到
**21.8 / 21.5**。这一条是本轮唯一「改动本身先写坏再改好」的记录，不删。

⑥ **新守卫 11 支的判据清单**（`tests/unit/test_scan_reuse_l248.py`）：稠密那条 `{_ngram_scan:1,
_ngram_entries:0, _fallback_encode:1, _batch_encode:1}`、倒排那条 `{_ngram_scan:1, _ngram_entries:0,
_fallback_encode:0, _batch_encode:0}`（计数键名册先声明，免得「一次没跑」被写成「缺键」）、
`_batch_encode` 收到的表 `is` 扫描返回的表、只读且计数的表**读次数 == 每篇去重后二元组数之和**、
`tobytes()` 逐字节相等、`vocab={}` ⇒ `(n, 0)`、`_ngram_entries` == 「扫描 + 折叠」逐元素、
四档预言机入参名册 + 判决形状三负一正、倒排那条第一个块产出前 traced current < 8 MiB。
**语料形状全部由下标算出，不用随机数**：L248 写 R 档时先用纯线性同余 `31i + 17j`，二元组完全由起点决定 ⇒
词表塌成 4 000、df 涨到几十、`pair_adds` 262 万，倒排反而判负（读数 `(4000, 2623572, False)`），
加二次项 `29j²` 让相邻差值随 `j` 变才拿到真实语料「词表大、df≈1」那一侧。

⑦ **五臂字节级注入取证**（`D:/WORKSPACE/l248_inject.py`，读 bytes / 断言纯 LF / 每锚点断言唯一命中 /
每臂写完跑守卫 / 立刻逐字节复原并断言与进入时相等 / 收尾再读盘复验一次）：
㈠ 发现遍无条件重跑（`if vocab is None:` ⇒ `if True:`）→ **1** 支红；㈡ 表不交给稠密那条
（`self._batch_encode(texts)`）→ **1** 支红（身份那一支）；㈢ 判完路不松手（删 `vocab = None` 那一行）→
**1** 支红（scratch 那一支）；㈣ 视图仍自己扫第二遍（`_inverted_view(texts)`）→ **4** 支红；
㈤ 假复用（`vocab = dict(vocab)`，内容相同、数值全同）→ **1** 支红（读计数那一支）。
**㈠ 的第一版尺子是恒绿的**：守卫当时只钉「收到现成表不许往里写」，而 `if True:` 那一臂把 `vocab` 重绑成
一张全新的 dict、写的是它自己的新表 ⇒ 11 支全绿。据此把 `_FrozenTable` 升级成「写就炸 **且读会计数**」的
`_ReadOnlyTable`，㈠ 与 ㈤ 才各点名一支。这轮记的账是：**「不许写」只否证「还在原地扫」，
「读够数」才否证「另建一份」**——两半合起来才是「复用」的完整机器定义。

⑧ **净行数与插行平移**：`dedup.py` 54/25 ⇒ 净 **+29**，`tests/unit/test_dedup.py` 4/4 ⇒ 净 **0**，
`OPTIMIZATION_LOOP.md` A38 那一行 1/1 ⇒ 净 **0**。处方按「换点 → 压回净零 → 才重钉」逐级试过：
换点无处可换（新增的是两个方法），压回净零要拆掉本轮自己的内容，遂走第三档——A38 的
`dedup.py:627-672` 按 AST 现量重钉为 **656-701**、归因写进行内。**归因必须如实**：a184 当时红的是
`TestHardZero::test_no_line_reference_claims_a_symbol_it_does_not_land_in` 与
`TestContentDimension::test_only_the_two_landing_lines_are_claims_and_exactly_one_is_red` **两支**——
第二支不是「另一处真缺陷」，而是它自己的合成注入格被真案面挤掉了首位，修好第一支它跟着绿。
另记一次「插行安全不可外推」：本轮同批在 `docs/augmentor/ARCHITECTURE.md` §3.34 只做**同行追加**（净 0 行），
所以不平移任何引用；而 `dedup.py` 的 +29 平移了 A38——同一轮里两种处置各按各自的净行数走。

⑨ **普查档位与本机失明（承 L247 的如实归因）**：新守卫的文档主张一律写成限定仓内相对路径
（`augmentor/dedup.py:486-510`、`augmentor/dedup.py:182-191`），落点分别完整落在
`_similarity_blocks`（473-510）与 `_fallback_encode`（154-209）跨度内。a184 硬 0 档本轮从 1 红回到 0；
但四档精确棘轮（`ambiguous` / `runtime_ns` / `scratch` / `scratch_missing`）由
`require_authoring_tree()` 把关、本机 `Temp/` 底料不在场 ⇒ **本轮同样不 enforce**。
「七把守卫 178 passed / 8 skipped」只证明不平移、不证明逐条被判 live，这句话不许写成「普查全绿」。

⑩ **尺子重钉清单（没有任何一把被改松）**：l242 三格按红读数重钉（表行 254 未动 / 有标记 138 → 139 /
声明面 120 → 121，两支红读数 `assert 121 == 120` 与 `assert 139 == 138` 当场逼出）、卷一 A38 的行内跨度
627-672 ⇒ 656-701、A326 行内 `dedup.py` 跨度 451-481 ⇒ 473-510、L232 的散文自指 790 ⇒ 791。
l97 / l98 的普查读数一格未动（本轮零新 `parametrize` 调用点，432 / 314 / 盲面 118 由全量门禁原地绿证实）。

⑪ **门禁读数**：定点面 `test_dedup.py` + `test_cost_oracle_routing_l241.py` + 新守卫
**152 passed**（141 + 11）；七把账本与文档守卫 **178 passed / 0 failed / 8 skipped**；
全量定标 **exit=0 / 8 973 passed / 0 failed / 10 skipped / 6 warnings / 377.38 s**，
`TOTAL` **14 363 / 8 / 3 968 / 22**、覆盖率 **99.84 %**。闭合核对：用例数 = L247 终态 8 962 + 新守卫 11 支；
`TOTAL` 语句 14 355 → 14 363（+8 = 两个新方法的语句）、分支 3 966 → 3 968（+2 = 两条新增的可选入参分支），
**Miss 8 与 BrPart 22 两格与 L247 逐格相同** ⇒ 新分支两侧都有真读者，没留下死支。

⑫ **前端三门禁不适用**：本轮 `web/` 零改动，`npm run typecheck / lint / test` 无对象可跑，按纪律记「不适用」而非漏跑。

⑬ **中间读数不美化**：探针第一版用 `{"text": t}` 喂 `deduplicate`，而它的 `text_key` 默认是
`"instruction"` ⇒ 全部文本读成空串、词表 0、每组只有一行、`groups == n`，是一份什么都没量的假读数；
补上路由与判决读数后才发现，遂改 `{"instruction": t}` 重跑。探针另有一次把 `\n` 写进替换串，
把格式串拆成 unterminated string literal 的 SyntaxError。R 档形状两次踩坑（见 ⑥）。
㈠ 臂恒绿是本轮最大的一次「尺子虚报」，处置见 ⑦。第一版的峰值回退见 ⑤。

⑭ **账本批之后复跑记账**（代码批 `9ed11b369` + 账本批之后的整跑，命令逐字照抄定标门禁）：
**exit=0 / 8 973 passed / 0 failed / 10 skipped / 6 warnings / 398.15 s**、**99.84 %**、
`112 files skipped due to complete coverage`。`coverage.xml` 根节点逐格读数为
`lines-covered 14 355 / lines-valid 14 363`（⇒ Miss **8**）与
`branches-covered 3 946 / branches-valid 3 968`（⇒ BrPart **22**），与 ⑪ 那一格
（`TOTAL` **14 363 / 8 / 3 968 / 22**）**四格全同**——本轮终态即 ⑪ 那一格。
377.38 s → 398.15 s（+20.77 s）：同机同产品码（账本批只动 `.md` 与 l242 的注释加两个整数常量，
产品语句数未变，`14 363` 逐格相同就是这条的闭合证据），按 L239 / L240 两轮量过的负载差档算噪声，
不作任何结论；判据一律是计数，wall 只作背景。**散文自指那一格复核**：本轮日志块的追加与 ⑩ 那次
换行全部落在文件尾部，第 **791** 行实测仍在原位（`sed -n '791p'` 打出的正是那句 `docs/augmentor/API.md:1139`），
L232 清单行里 790 → 791 的重钉没有被这次追加再平移。**账本批净行数逐处核对**（按 `git show --numstat` 真值）：
`OPTIMIZATION_LOOP_2.md` 93/2 ⇒ 净 **+91**（清单行 +1、日志块 +89、⑩ 那一处把一行说明拆成两行 +1），
`docs/augmentor/ARCHITECTURE.md` 1/1 ⇒ 净 **0**（§3.34 第 4 条同行追加），
`tests/unit/test_backlog_closure_l242.py` 3/3 ⇒ 净 **0**（三行常量原地换值 + 行尾注释），
`OPTIMIZATION_LOOP.md` 1/1 ⇒ 净 **0**（A38 那格行内换数字，已随代码批提交）。
七把账本与文档守卫在账本批之后复跑 **178 passed / 0 failed / 8 skipped**（本机 `Temp/` 底料不在场那 8 格）。
### L249（2026-10-12）— 收 A332 + A307：只读面的磁盘副作用摘在库里，三面一次性归零

**本轮定义（动手前拍的）**：收 **A332 + A307**——同一根因（构造函数 `mkdir`）的两条只读面写盘账，一条在备份
那一族、一条在依赖注册表那一族。不收其它账、不立新账。

- **① 测量先行**：仓外探针 `D:\WORKSPACE\l249_probe.py` 把「谁在写盘」量成现量而不是印象——**13 条只读通道
  （SDK / HTTP / CLI 三面）里只有 1 条不建目录**，即 `GET /api/system/backups/verify`。
- **② 那 1 条干净的原因不是运气**：路由里手写了 `if not resolved.exists()` 挡在构造之前。挡板是**逐只**打的，
  SDK 面与 CLI 面一条都没有 ⇒ 改前「只读面不写盘」这句口径只在那一格侥幸成立。
- **③ 所以本轮动的是库而不是路由**（A332 的修法② 就是逐面挡板那一派，按现量否掉）：摘掉副作用的来源，六只
  `GET`、六只 SDK 读入口、五只 CLI 读档一起变干净——三面同源，与 L244 把 verify / cleanup 接进 CLI 是同一套路。
- **④ 备份那一族两刀**（`augmentor/backup.py` 净 **+2**，两处都在被引用的 `:90` / `:192` 之下，跨度未动）：
  `__init__` 的 `mkdir` 换成一行注释（净 0）；`_load_index` 交回内存空账、不落默认 `index.json`（净 +2）。
  **两刀的理由**：只摘 `mkdir` 的话本族现量「目录 + 一份 `index.json`」只少一半，loader 会自己写默认账。
  首份索引延后到 `_save_index`，那一只仍走 `atomic_write_json`（父目录由它自建）⇒ L162 的原子写判据不松动。
- **⑤ 注册表那一族一刀**（`augmentor/dependency.py` 净 **0**）：`mkdir` 原地换成一行注释。净 0 是被迫的——
  A305 那一格引用 `augmentor/dependency.py:142-144`（`register_dataset` 的跨度），第一版写成两行注释把它
  平移了，按「压回净零」那一档收回一行（处方优先级来自循环纪律第 5 节）。
- **⑥ 改后同一支探针复跑**：13 条只读通道**全部**零残留（跑前跑后目录树名单逐字相同）；写面照旧按需建目录
  （`wb/index.json` + `wb/w1.json`、`wr/datasets.json`），换新实例读得回那一行且 `checksum_match` 仍 `True`
  ⇒ 副作用是**延后**而不是取消，这一格由守卫第 7 组判据钉住而不是靠散文说。
- **⑦ L243 那格担心被实测否掉**：当时记的是「写端点若不建目录，`POST …/cleanup` 可能从 200 变 500」。改后
  无债档仍回 **200** `{deleted_count: 0, max_backups: 5}`——库里根本没有「目录必须在场」这一步，`_load_index`
  读不到就交回空账。**这是一次「担心有理由但结论错」**：理由（构造建目录）与被担心的对象（500）之间还隔着一层
  「谁去读目录」，只有实测能分开。
- **⑧ L44「判参先于副作用」的证人退役**：那条口径原本靠「坏参数会留下目录」当磁盘证人。证人本轮被摘掉 ⇒
  改由 AST 比 `require_count` 与 `DatasetBackup` 在 `clean_old_backups` 里的行号先后钉住。注入臂 o1 的读数
  就是这条改动的理由：把 `require_count` 挪到构造之后，**行为那一支仍然绿**，只有 AST 那一支红。
- **⑨ 同批重钉七处旧判据**（它们钉的是缺陷形状本身，不重钉等于把 A307 写进断言当真相）：
  `test_dependency.py::test_init`、`test_round68.py` 的两支默认落点（**补一次真写**来验「延后建」而不是
  「不建」）、`test_backup.py::TestDatasetBackup::test_init`、`test_cli_backup_actions_l244.py` 的 CLI verify
  档（改名 + 断言反转）、`test_backup_integrity_l243.py` 的缺目录档（含 cleanup 那一档）、
  `test_lineage_wiring_l232.py` 的血缘档与短语名册、`test_backup_index_atomic_l162.py` 的静态尺子。
  最后一处是本轮抓到的**尺子虚报**：旧判据比 `"atomic_write_json" in body`，而我替换掉的调用恰好被新注释
  提了一句名字——删掉真调用它照样绿。改成走 AST 只数调用点之后 b2 / b3 两臂才点得到名。
- **⑩ 新守卫 `tests/unit/test_readonly_no_disk_writes_l249.py`（22 支，十组判据，全部计数或磁盘事实，
  不量墙钟）**：AST 零写调用 / 读边 `open` 只许 `'r'` / 只读名单由 `WRITING_MARKERS` **实推**而非抄 /
  逐面磁盘快照 / 读三遍幂等 / **陷阱自证**（写面必须让快照变化，否则「没变化」恒真）/ HTTP 四只 `GET` 与
  默认档 / CLI 五只读档 / 判参先后。实推那一组本轮修过一次分类错：最初按「体内有没有调 `_save_index`」二分，
  把 `restore` 判成了读方法（它落输出文件而不动索引），读数 **1 failed / 142 passed**；改成「落索引 / 落文件 /
  删文件」三个标记后集合才与名单精确相等。另修过两处自己的错：快照右端把根下相对路径写成带目录前缀（假红）、
  一支用例曾拿 `__file__` 同目录当「不存在的目录」（回归会在版本树里留目录，改成 `tmp_path`）。
- **⑪ 注入取证六臂**（`D:\WORKSPACE\l249_inject.py`，字节级：读 bytes、归一 LF 做锚点、按原行尾回译、
  逐臂断言磁盘哈希与进入时相同）：b1 `DatasetBackup.__init__` 放回 `mkdir` ⇒ **11 红**；b2 `_load_index`
  放回默认落盘 ⇒ **10 红**；b3 索引读边 `open` 改 `r+` ⇒ **2 红**（模式维 + l162 的 AST 维）；r1 把读方法
  `list_backups` 改成会写 ⇒ **9 红**，其中名单实推那支单独点名「读侧不等」；o1 判参挪到构造后 ⇒ **1 红**
  （见 ⑧）；d1 `DependencyManager.__init__` 放回 `mkdir` ⇒ **7 红**。六臂全部点名，复原后基线复跑
  **26 passed / 0 failed**，两支产品文件按 `git diff --numstat` 回到 **+2 / 0**。
- **⑫ 前端三门禁（`web/` 有改动 ⇒ 适用）**：`npm run typecheck` 静默通过、`npm run lint`（`--max-warnings 0`）
  通过、`npm run test` **22 files / 307 tests passed**。改动三只文件净 0 行（`System.tsx` 2/2、`api.ts` 1/1、
  `System.lineage.test.tsx` 2/2）；短语名册与卡片文案在四处（l232 名册、lineage 测试、`api.ts`、`System.tsx`）
  同批换成「构造不再碰磁盘」，因为那三支尺子判的是同一句话在场。
- **⑬ 门禁读数**：全量 **8 995 passed / 0 failed / 10 skipped / 6 warnings / 318.90 s**、**99.84 %**、
  `TOTAL` **14 359 / 8 / 3 968 / 22**。`lines-valid` 14 363 → **14 359**（−4）与本轮摘掉的语句逐格对得上：
  两处 `mkdir`、loader 的默认落盘调用、它后面那句 `return`；`Miss 8` 与 `BrPart 22` 两格未动。
  318.90 s 与上一轮 398.15 s 之间的差按 L239 / L240 量过的负载差档算噪声，**不作任何结论**（判据一律计数）。
  七把账本与文档守卫在账本批前后各跑一次，读数见下面 ⑭。零 `parametrize` 新增（l97 / l98 普查读数不动）。
- **⑭ 账本批净行数与遗留**：清单行 **+1**、A307 / A332 两行原地改写（2/2，净 **0**）、日志块追加 **65** 行
  ⇒ `git diff --numstat` 真值 **68/2、净 +66**（这处数字第一次写成 67/2，是追加完再补一句造成的旧读数，当场按真值改回——中间读数不美化），
  `docs/augmentor/ARCHITECTURE.md` 净 **0**（那句改写走「换点」——第一版多插一行把 §3.33 的散文拆成两行，同轮收回
  同行追加，`git diff --numstat` 复核 1/1），`tests/unit/test_backlog_closure_l242.py` 三行常量原地换值净 **0**。
  清单区插一行会把下方历史行号引用平移 **+1**——本轮实测七把账本与文档守卫在账本批后复跑 **178 passed /
  0 failed / 8 skipped**（那 8 格是 `Temp/` 底料不在场的本机档），未出现平移红，故不重钉任何 `file:NNN`。
  Backlog 剩余以 l242 那把尺子的读数为真相（本轮 `MEASURED_MARKED` 与 `MEASURED_DECLARED` 两格按红读数
  当场重钉），A298 / A301 / A303 / A304 / A306 / A308–A311 / A328 / A330 / A333 那一族一律未动。
- **⑮ 账本批之后复跑记账**（代码批 `8ede5113e` + 账本批 `72b63c432` 之后的整跑，命令逐字照抄定标门禁）：
  **exit=0 / 8 995 passed / 0 failed / 10 skipped / 6 warnings / 364.67 s**、**99.84 %**，`coverage.xml`
  根节点逐格读数为 `lines-covered 14 351 / lines-valid 14 359`（⇒ Miss **8**）与 `branches-covered
  3 946 / branches-valid 3 968`（⇒ BrPart **22**），与 ⑬ 那一格（`TOTAL` **14 359 / 8 / 3 968 / 22**）
  **四格全同**——账本批只动 `.md` 与 l242 的三行常量（注释加两个整数），产品语句数未变，`14 359`
  逐格相同就是这条的闭合证据。318.90 s → 364.67 s 按 L239 / L240 两轮量过的负载差档算噪声，
  **不作任何结论**（判据一律计数）。**散文自指那一格按现量重钉**：清单行落在第 87 行，把 L232 清单行里
  那句「本账第 **791** 行那条 `docs/augmentor/API.md:1139` 引用仍指得准」推到 **792** ⇒ 已按现量改成 792 并在同一处
  记下「L249 的清单行再推 1 行」（这是同一行号第七轮第七个值，而这一形不带文件名、a184 的结构维与内容维
  都不量，A333 记的正是它）；`sed -n '792p'` 实测打出的就是那一句。L247 / L248 两块里「第 790 / 791 行
  实测仍在原位」是各自轮次的快照，**不改写**。七把账本与文档守卫在账本批之后复跑 **178 passed /
  0 failed / 8 skipped**（那 8 格是 `Temp/` 底料不在场的本机档）。

### L250（2026-10-13）— 收 A298 + A303：备份索引的同名写点从 append 收成「摘旧行再落新行」，id 与时间戳同源一次读钟

- **① 开工前的待拍冲突，先摆出来再说**：A298 与 A303 两格都写着「定级 M / S，待拍」，而 L246 到 L249 的清单行连着写的是「待拍的那一族一律未动」——**动手的是代理而不是用户**。本轮按目标线（备份那一族的功能完整性与文档一致性）择定 A298 走立账原格的修法①、A303 走修法①，理由见 ②③；**可回退面**只有一处写点（`augmentor/backup.py` 的 `backup()`）加两侧判据，回退成本已经写在 A298 那一格里。L243 开过这个先例（A299 由用户拍板走 ①+③），本轮没有等到拍板，所以把「谁拍的」记在账上而不是含糊成「收了」。
- **② 改前两行一文件的现量（立账那一批量的，本轮复核）**：同一个 `name`（实测 `mybak`）创建两次 ⇒ 磁盘只有**一份** `mybak.json`（第二次 `atomic_write_json` 覆盖第一次），索引里却是**两行**，`timestamp` 不同（实测 `.991804` 与 `.997820`）而 `backup_path` **逐字相同**。第二行的 `checksum` / `item_count` / `file_size` 描述的是已经不存在的字节 ⇒ **死账**，不是历史。相邻的不对称是尺子该响的地方：删除按 id 过滤（一次清全部），创建按 name 追加（一行一份）——两把尺子不同规。
- **③ 修法为什么是 ①**：① 把「一份文件对一行索引」钉在**写点**上，读边与前端一行都不必动；② 让 `backup_id` 真唯一（同名追加后缀）要动 id 的语义、`verify` 与 `cleanup` 的行数口径、前端行键与 `docs/augmentor/API.md` 的示例，改动面比缺陷大；③ 回执加「本 name 已有 N 行」在 ① 之后恒等于 1，没有信息量。**读边去重**（把两行塌成一行返回）本轮明确否掉：它只是把死账藏起来，`GET` 与磁盘两份数还会长期不等，且 `clean_old_backups` 的保留数按索引行数算，藏行等于少删。
- **④ 改动落点（`augmentor/backup.py` 三处，净 6/6 = 0 行）**：`now = datetime.now()` 只读一次钟；`backup_id = name or f"backup_{now.strftime('%Y%m%d_%H%M%S')}"` 与 `timestamp=now.isoformat()` 同源于那一格；索引写点由 `self._index["backups"].append(info.to_dict())` 换成推导式 `rows = [b for b in self._index["backups"] if b.get("backup_id") != backup_id]` 再 `rows + [info.to_dict()]` 落回。两处注释同行追加，不带新行号主张。
- **⑤ 为什么按 `backup_id` 摘而不是按 `name`**：`backup_id` 本来就是 `name or 时间戳`，按它摘在语义上等价于按 name 摘，且不必在索引行里引入第二把键——`BackupInfo.to_dict()` 是七键 `{backup_id, timestamp, source_path, backup_path, item_count, file_size, checksum}`，**没有 `name`**（那一格现在是 A335，继续待拍）。将来补了 `name`，这处写点才有第二种键可用，这一点写在 A335 行里互相指认。
- **⑥ 未做 ② 的残余形状，钉成 tripwire 而不是静默消失**：同一秒内的两次**匿名**备份仍共享 `backup_YYYYmmdd_HHMMSS` ⇒ 索引塌成一行、磁盘塌成一份。`TestUnnamedBackupsInsideOneSecond::test_two_unnamed_backups_in_one_second_share_one_id_and_one_file` 用冻结时钟把这一格现量钉住（它是本轮唯一一支「钉的是没修的那一半」的判据，docstring 里写明它存在的意思是**别把这条当已清**）。
- **⑦ 读边一字未动，前端行键因此不是遗漏而是兼容**：存量 `index.json` 里本轮之前写出的遗留多行照原样交回（`TestTheReadEdgeStillsHandsBackWhatIsThere` 三支：两行照样回两行、下一次备份把遗留行一起摘掉、一次删除连文件清全部）。`System.tsx` 的 `rowKey={r => \`${r.backup_id}#${r.timestamp}\`}` **保持**，注释的归因由「A298 的两行」换成「L250 之前的存量索引」；`api.ts` 的 `deleteBackup` 与 `types/api.ts` 的 `BackupInfo` 注释同批改写（后者顺带订正一句陈旧话——它写着「校验和不匹配照样 200」，而 A299 早在 L243 就收成 409 了）。
- **⑧ A303：两把判据一把静态一把行为**：AST 数 `backup()` 体内 `datetime.now()` 的调用次数恰为 **1**（`test_the_id_and_the_receipt_timestamp_come_from_one_clock_read`），加上假钟只许读一次、第二次读当场 assertion 失败的行为差（`test_a_second_clock_read_would_change_the_answer`，另钉 `backup_id[-6:] == timestamp[11:19]` 去冒号那一格）。两把都要：前一把会被「把第二次调用挪进注释或别的函数」喂饱，这正是 L245 记过的引用尺子内容盲区那一族在测试面的同型。
- **⑨ 新守卫 `tests/unit/test_backup_index_collapse_l250.py`（12 支 / 五类，全部计数或磁盘事实，不量墙钟）**：写边收敛 4 支（三次同名 ⇒ 1 行且等于最新那份的 `to_dict()`、不同名各留一行、**行数 == 磁盘 `*.json` 份数**跨 create / 子目录名 `sub/deep` / delete 三个动作、新实例读得回收敛后的行）；读边原样 3 支（见 ⑦）；同秒匿名 1 支（见 ⑥）；两面一致 2 支（HTTP 连 POST 两次同名字 ⇒ `GET` 回 1 行且那一行 == 第二次的回执；CLI `backup --action create` ×2 后 `--action list` 打印「找到 1 个备份」）；AST 2 支（全类零处对 `_index[...]` 的 `append`；`backup()` 内必须有一条以 `backup_id` 为条件的推导式——条件取自 `generator.ifs` 而不是 `node.test`，写第一版时在这里踩过一次 `AttributeError`）。**边界照旧写进 docstring**：不重量 L243 的校验和闸、L231 的四条调用形状与穿越闸、L249 的只读零副作用面。
- **⑩ `test_backup_wiring_l231.py` 33 → 34 支**：三支按新世界改名并反转判据（`test_same_name_twice_leaves_one_index_row_and_one_file`：第二次用一份 10 条的 `big.json` 建，钉 `len(named) == 1`、那一行逐格等于第二次的回执、`item_count` 从 5 变 10、磁盘只剩 `index.json` + `mybak.json`；`test_the_id_and_the_receipt_timestamp_come_from_one_clock_read`；`test_the_row_key_still_hedges_for_legacy_duplicate_ids`——**断言一字未动，改的是理由**：前端行键继续拼时间戳现在是为了存量索引而不是为了新写的两行），新增假钟行为差一支；页面短语名册里「删一次两条一起消失」换成「一次全清」，`System.backup.test.tsx` 的三处短语断言同批跟上（该文件净 **+1** 行，是注释多一行）。
- **⑪ 四臂字节级注入取证**（`D:\WORKSPACE\l250_inject.py`：读 bytes、归一 LF 做锚点、断言锚点在原文唯一、逐臂写完读回比哈希、`finally` 复原并断言与进入时字节相同）：a1 写点放回 `append` ⇒ **10 红**；a2 把第二次读钟放回 ⇒ **3 红**；a3 读边改成按 id 去重（证明尺子分得清「摘写点」与「藏读边」）⇒ **1 红**；a4 摘掉推导式的 `backup_id` 条件 ⇒ **2 红**。基线 **0 failed / 46 passed**，四臂跑完逐臂断言磁盘哈希与进入时相同（复原由探针自证，事后 `git diff --numstat` 复核 `backup.py` 仍是 **6/6**）。**中途两次自己的错都记下来**：驱动的第一版把「基线全绿」判成了成立（`N failed…N passed` 那条正则在 0 红时匹配不到任何东西 ⇒ 读出 `0 failed / 0 passed` 也过断言），补上 `(\d+) passed` 与 `total > 0` 才真堵住；第一次跑驱动时管道被 `head -8` 截断，先复核磁盘（无注入残留、推导式恰一处、`timestamp=now.isoformat()` 仍在 136 行）再重跑到底。
- **⑫ 文档三面**：`docs/augmentor/API.md` 五行原地改写净 **0**（列表行的两行同 id 改成历史形状、创建行写「id 与 timestamp 自 L250 起同一次读钟」并点名同秒匿名那一格由 l250 守卫钉住、删除行写「同 id 全部行一起消失」、verify 行写存量多行只出一行、cleanup 行把旧算术改述成「只有存量索引才走的路径」，历史现量「4 行索引 / 3 文件 / 保留 1 ⇒ 回 2」留着不抹）；`System.tsx` 五处净 **0**；`api.ts` 2/2、`types/api.ts` 6/6 净 **0**。`docs/augmentor/API.md:1139` 那一族的引用没被平移（净 0 行的全部意义就在这里）。
- **⑬ 前端三门禁（`web/` 有改动 ⇒ 适用）**：`npm run typecheck` 静默通过、`npm run lint`（`--max-warnings 0`）通过、`npm run test` **22 files / 307 tests passed**。
- **⑭ 门禁读数**：全量 **exit=0 / 9 008 passed / 0 failed / 10 skipped / 6 warnings / 328.72 s**、**99.84 %**、`TOTAL` **14 360 / 8 / 3 968 / 22**。用例闭合 = L249 终态 8 995 + 新守卫 12 + l231 拆出的那 1 支；`lines-valid` 14 359 → **14 360**（+1 就是新增那句 `rows =`），`Miss 8` / `Branch 3 968` / `BrPart 22` 三格与 L249 逐格相同 ⇒ 没留死支。318.90 s → 328.72 s 按 L239 / L240 两轮量过的负载差档算噪声，**不作任何结论**（判据一律计数）。零 `parametrize` 新增（l97 / l98 普查读数不动）。
- **⑮ 账本批净行数与遗留**：清单行 **+1**、A298 / A303 / A306 三行原地改写（净 **0**）、Backlog 新账 A335 **+2**（一行本体 + 它与 Backlog B 标题之间的分隔空行）、日志块追加在文件尾部（标题行 + 15 支 bullet 跨 16 个物理行 + 前面的分隔空行 = **+18**）、`tests/unit/test_backlog_closure_l242.py` 实测 **11/10、净 +1**（三行常量原地换值净 0，弱词样本那一支 5 行原地换形净 0，多出的那一行在模块 docstring 口径 7 那一格——样本迁移的归因要写在那里），于是 `OPTIMIZATION_LOOP_2.md` 的真值是 **25/4、净 +21** = 清单行 **+1**、Backlog 新账 **+2**、日志块 **+18**。清单区插行会把下方历史散文行号平移——本账第 **792** 行那条 `docs/augmentor/API.md:1139` 引用实测被推到第 **795** 行（+3 的构成见上），已按现量重钉并 `sed -n '795p'` 复核；L247 / L248 / L249 三块里「第 790 / 791 / 792 行实测仍在原位」是各自轮次的快照，**不改写**。l242 三格由该守卫的红读数当场逼出（预测 255 / 125 / 143，逐格以红读数为准），Backlog 剩余笔数以它的读数为真相（不手数）；A301 / A304 / A308–A311 / A313–A318 / A323 / A324 / A328 / A330 / A333 那一族的定级格一律未动，A306 本轮只改了它引用 A298 的那半句（定级格未动、净 0 行），A335 是新立待拍而不是未动。
- **⑯ 账本批之后复跑记账**（代码批 `e50deb0f5` + 账本批 `9ca6f7849` 之后的整跑，命令逐字照抄定标门禁）：
  **exit=0 / 9 008 passed / 0 failed / 10 skipped / 6 warnings / 335.77 s**、**99.84 %**，`coverage.xml`
  根节点逐格读数 `lines-covered 14 352 / lines-valid 14 360`（⇒ Miss **8**）与 `branches-covered 3 946 /
  branches-valid 3 968`（⇒ BrPart **22**），与 ⑭ 那一格（`TOTAL` **14 360 / 8 / 3 968 / 22**）**四格全同**
  ——账本批只动 `.md` 与 l242 的常量、docstring 与那一支弱词用例（净 +1 行落在测试文件里，不进产品覆盖率
  分母），产品语句数未变，`14 360` 逐格相同就是这条的闭合证据；328.72 s → 335.77 s 按 L239 / L240 两轮
  量过的负载差档算噪声，**不作任何结论**。**清单行落点按现量核对**：L250 那一行实测在第 **88** 行，L232
  清单行里那句散文自指随之从 792 推到 **795**（+3 的构成见 ⑮），`sed -n '795p'` 实测打出的正是含
  `docs/augmentor/API.md:1139` 的那一句；L247 / L248 / L249 三块里的 790 / 791 / 792 是各自轮次快照，**不改写**。
  七把账本与文档守卫在账本批之后复跑 **178 passed / 0 failed / 8 skipped**（那 8 格是 `Temp/` 底料不在场
  的本机档），未出现平移红。本支自身追加的行数记在**它所在提交的 message 里**而不是回填进 ⑮——那是
  L247 到 L249 一路沿用的处置（自指的行数永远差一格）。**本轮两处自己写错又改回的账，留着不美化**：
  ① 第一版清单行把复跑读数写成「8 008 → 9 008」，那是把 L249 的 8 995 记串了，按真值改成 9 008 一格；
  ② 第一版把 A306 与 A335 一起写进「一律未动」的名册——A306 本轮被改过半个口径句、A335 是新立，
  两者都不属于「未动」，已按现量分开表述（就在本支上面那一格的末尾）。

### L251（2026-10-13）— 收 A304：`/api/data/list` 加 `all_files` 那一档，宽的是名字闸而不是路径范围

- **① 开工前的待拍冲突，先摆出来再说**：A304 那一格写的是「定级 S，待拍」，历轮清单行记的是
  「待拍的一律不动」，而本轮目标线（消费能力与功能完整性）要的正是这一格——它卡着「用户想备份
  一份 `convert` / `merge` / `sample` 的产物，下拉里没有它」。本轮按代理择定走**立账那格写好的
  修法①**，理由是这一条**纯加法**：默认档一字未动、契约只多一只可选键、响应面一字未动，回退
  只需把 `scan_files` 里那句条件还原并把两侧判据同批反转。**它与 A280 的关系记在这里免得下一轮
  误读**：宽档第一次把「非数组」那份文件端到列表上，于是把 A280 那一族的 500 吞 400 暴露到产品面
  （见 ⑪），本轮**只量不动**它，A280 的定级格未动。
- **② 零漂移的先拍片**：动手之前先把默认档的形状钉死（不带参数、`?all_files=false`、
  `?all_files=0` 三种写法的**整条目**相同，含 `path` 与 `size`，不是只比名字）。只比名字的那把尺子
  抓不到「加档时顺手把 `path` 改成相对路径」这一类，所以判据一开始就按条目比。三档比在同一只测试
  里用循环而不是 `parametrize`：l97 / l98 那两把普查尺子数的是全仓 parametrize 档位，新增一档就得
  连带重钉它们的读数（L249 立的「零 parametrize 新增」纪律）。
- **③ 宽档宽的到底是什么**：只有**名字**这道闸。扫描范围仍然逐字来自白名单那一只权威、
  `glob` 不递归、去重仍按 resolve 后的路径，于是四格安全判据成立且各有注入臂：根外那份
  `secret.json` 两档都不列（③ 臂把它喂进来就红）、子目录那份两档都不列、白名单里写 `ds` 与
  `ds/.` 两个根时宽档仍四份不翻倍、根不存在时两档都是 200 + 空列表而不是抛错。
- **④ 「列表能给的，路由必须能读」在宽档下续量**：这一句是 `test_api_security.py` 里
  `TestDataListScansWhitelist` 立的承诺（当年量到的是「列表端出工作目录里的文件、用户一点就 403」）。
  本轮把同一句在**两档**下各量一遍：宽档列出的每一份喂回那只加载端点都**不拿 403 也不拿 404**，
  这一句管的是路径归属，不是内容形态。
- **⑤ 契约面**：那只端点的 query 面恰好只有 `all_files` 一只（`len(params) == 1` 而不是「包含」），
  `in=query`、`required is False`、`type == boolean`、`default is False` 逐格钉；响应的顶层键集仍是
  单一 `files`，加档没有顺手加键。这一格不只是文档口径——l224 的 query 对账与 l237 的两层尺子都
  读它。
- **⑥ 前端接线的范围：诚实版**：A304 立账那格讲的是「备份源文件」那一格，实际读
  `dataFiles` 的消费者才发现系统状态页**六处**候选下拉共用同一份 state（备份源 / 数据集源 /
  监控数据 / 源数据 / 迁移产物 / 流式产物）⇒ 走宽档之后变宽的是这一页的六处，不是只有备份那张卡。
  本轮**没有**为了「只宽一格」去拆 state（多一次取数、多一份缓存，而六处本来就是同一族服务端路径），
  而是把这件事写进卡片文案与守卫判据（`options={dataFiles.map` 现量恰 6 处）。其余页面那 8 处数据
  文件选择器继续走默认档，清单没有在全站范围变宽，这一格由「默认档那只 fetcher 的页面消费者数 == 8」
  钉住。
- **⑦ 一处实现选择是被尺子逼出来的**：query 写在路径串里（`/data/list?all_files=true`）而不是
  `params` 对象里。原因是 l224 那把消费对账尺子把 params 块按（方法, 路径）归属，同一路径两员调用
  时它分辨不了——写成 `params` 会让宽档那一员白得默认档的归属。这不是偏好，是判据机制决定的写法，
  所以 `api.ts` 那段注释与守卫 docstring 都写了归因。
- **⑧ l224 三格按红读数重钉 + 一处真实缺陷**：调用形状 77 → **78**（这张表第一次出现「一只操作、
  两种吃法」，形状数比操作数多 1 不是缺口）、带 query 的形状 14 → **15**、去重后键名 9 → **10**
  （新增的就是 `all_files`）。重钉之后旧判据**首跑绿、复跑红**，追下去是集合迭代序依赖：
  `{（方法, 路径）: 键名}` 那种字典推导在同一只路径有两员调用时留下哪一格取决于集合迭代顺序，
  同一份代码在 `PYTHONHASHSEED=0/1` 绿、`2/3/7/11` 红（六档实测）。改成先由 params_bindings 判归属、
  再对「出现集合」逐格断言之后，五个种子（0 / 2 / 7 / 11 / 42）各 **28 passed**。**这一格属于尺子
  自己的缺陷，不是被本轮的接线逼松的**：判据从「一个字典的一格」变成「一个集合的两格」，覆盖面只增。
- **⑨ l237 三格按红读数重钉 + 一支新判据**：`export const` 85 → **86**、调用点与对账行 77 → **78**。
  这把跨语言尺子从此也有「两只 fetcher 配同一只操作」，所以新增一支 `test_the_two_fetchers_on_one_operation_are_not_a_scan_artifact`
  点名差额**恰好**是这一对、且两只共用同一份响应模型（`DataFileListResponse`），并把「同操作多
  fetcher 的差额 == 1」钉成算术式——否则将来谁再往同一只路径塞第三员，本尺子的行数与契约操作数
  会静默错位。两支新判据各红过一次（第一次是我把 `sorted()` 的名册顺序写反了，`getBackup...` 在
  `getData...` 之前），按现量改对后 22 passed。
- **⑩ l231 / l232 的连带**：l231 34 → **35**（新增 `test_the_wide_candidate_list_is_as_wide_as_the_whitelist`，
  宽档把 `p.json` 端出来而根外的 `secret.json` 进不来；A304 那格口径改成已落地；「候选来自那只
  端点」的断言换成新 fetcher 名），l232 支数 **39 未动**（只把同名断言换成新 fetcher，那一族血缘卡
  是六处共用 state 中的一处）。
- **⑪ A280 的现量新增（本轮量到，未动）**：宽档列出的非数组那份文件喂回加载端点，实测是
  **500 + 固定文案**、不是 `assert_dataset_shape` 那句 400，因为 `api/routes/data.py:177` 那句
  except Exception 把 HTTPException 一并吞进一刀切。守卫按**观测行为**钉（状态码恰 500、文案恰等于
  那份固定文案、400 的原话不出现在响应里），不按应有行为钉——L234 立过同型先例（「不给 `is_valid`
  写一条推出来的规则，只钉观测到的行为」）。为什么值得单独记：默认档根本不列这类文件，宽档第一次
  把它端到列表上，于是这一格从「SDK 里才碰得到」变成「产品面上点得到」。
- **⑫ 文档两面**：`docs/augmentor/API.md` 两处**原地改写**净 **4/4 = 0** 行（那只端点的小节写明两档口径与
  「宽的是名字不是范围」；顶部那句「扫描范围与白名单同源，所以它列出的文件一定都读得到」在两档下
  仍然成立），于是 `docs/augmentor/API.md:1139` 一族历史引用不平移。系统状态页卡片文案同步换成「六处共用一
  只 fetcher、取自宽档、默认档仍只列 `train_data*.json`、这一格仍是可输入下拉」。
- **⑬ 四臂字节级注入取证**：① 摘掉旋钮（宽档不看 `all_files`）咬中 **5** 支、② 默认档翻成 true
  咬中 **5** 支、③ 扫描范围改成工作目录咬中 **7** 支、④ 前端那一员退回默认档咬中 **4** 支。四个
  被改文件逐臂字节复原（读 bytes、纯 LF 锚点唯一命中、写完重新读盘比哈希），基线复跑
  **121 passed / 0 failed**（l251 + l231 + l232 + l224 合跑）。② 与 ④ 的目标面里都带了 l224 或 l237，
  于是「接线退回默认档」这一臂同时验到消费对账那把尺子也会响，不只是本轮新守卫自说自话。
- **⑭ 门禁读数**：前端三门禁（`web/` 有改动 ⇒ 适用）`tsc --noEmit` 与 `eslint . --max-warnings 0`
  均 rc=**0**、vitest **22 files / 307 → 308 支全绿**（新增那支宽档 fetcher 的调用形状判据）。
  账本批之前的全量定标 **exit=0 / 9 029 passed / 0 failed / 10 skipped / 6 warnings / 360.09 s**、
  **99.84 %**、`TOTAL` **14 360 / 8 / 3 968 / 22**。用例闭合：L250 终态 9 008 + 新守卫 19 + l231 新增
  1 + l237 新增 1 = 9 029，与实跑逐格相同。TOTAL 四格与 L250 完全相同 ⇒ 本轮产品改动净零行、
  零新语句（宽档靠的是既有那一句条件的布尔取反），Miss / Branch / BrPart 三格未涨就是说没留死支。
  328.72 s → 360.09 s 按 L239 / L240 两轮量过的负载差档算噪声，**不作结论**。
- **⑮ 账本批净行数与平移处置**：清单行 **+1**（第 89 行）、A304 与 A280 两行**原地改写**净 **0**
  （本轮不收新账 ⇒ Backlog 区行数未动）、日志块追加在文件尾部。于是本账那句活散文自指
  「`docs/augmentor/API.md:1139` 引用仍指得准」从 **795** 推到 **796**，已在 L232 清单行那一处按现量重钉并
  `sed -n '796p'` 复核（打出的正是那一句）；更早轮次写在各自日志块里的 790 / 791 / 792 / 795 是
  轮次快照，**不改写**。测试文件 `test_ts_contract_types_l237.py` 净 **+18** 行（新增一支 + docstring
  解释），全仓实测**没有任何一处引用它的行号**，所以不平移两把行号守卫；`api/routes/data.py` 净
  **7/7 = 0** 是本轮唯一被历史引用指着的产品文件，A8 那一格与卷一各行号引用一字未动。l242 三格由
  该守卫的红读数当场逼出（预测 255 / 126 / 144，逐格以红读数为准），Backlog 剩余笔数以它的读数为
  真相（不手数）。
- **⑯ 本轮自己写错又改回的账，留着不美化**：① 新守卫第一版按「应有行为」假定非数组那份拿到 400，
  实跑 **2 failed** 才量到 500 + 固定文案，于是那一类整体改成按观测行为钉并把它归到 A280 名下；
  ② 卡片文案第一版写「只有备份那一格变宽」，读 `dataFiles` 的消费者才发现是六处，文案、守卫
  docstring 与判据三处同批改；③ `docs/augmentor/API.md` 第一次改写把段落弄成 4 行（+1，会平移
  `docs/augmentor/API.md:1139` 一族），第二次把第 50 行拆成两行（又 +1），两次都收回净零；④ 新守卫第一版
  用了 `@pytest.mark.parametrize`，为不动 l97 / l98 棘轮改成测试内循环；⑤ 同一支新守卫的 SPDX 头
  第一版写成 `AGPL-3.0-only`，按仓内既有档改成 `AGPL-3.0-or-later`；⑥ l224 那条重钉后的判据首跑
  绿、换种子红，追出的是**尺子自己的**顺序依赖（见 ⑧），不是接线逼出来的宽容。
- **⑰ 下一轮口径**：本轮收掉 **A304**，待拍名册（A301 / A306 / A308–A311 / A313–A318 / A323 / A324 /
  A328 / A330 / A333 / A335，外加本轮给 A280 添的那格现量）的定级格一律未动。**A280 值得单开一轮**：
  它一次牵动八个文件的错误分档、两支钉死 500 的既有用例、以及 400 档 `to_http_error` 原样回传
  `str(exc)` 的泄漏面（A155①），不是一轮的量；谁收它要把 `TestWhatTheListHandsBackIsReachable`
  那一类同批改。纯加法面还剩两只可径自动手：**A330**（契约补错误档）与 **A328**（标记形状与轮号
  解析式的封闭清单）。
- **⑱ 账本批之后的复跑记账**：全量门禁在账本批之后重跑 **exit=0 / 9 029 passed / 0 failed /
  10 skipped / 6 warnings / 405.14 s**，`TOTAL` **14 360 / 8 / 3 968 / 22** 与覆盖率 **99.84 %**
  **四格全同**——账本批只动 `.md` 与 l242 的三行常量和注释（`git diff --numstat` 实测该守卫
  **3/3 = 净 0**，不进产品覆盖率分母），于是语句数与偏支数一格未动就是这条的闭合证据。
  360.09 s → 405.14 s 仍按 L239 / L240 量过的负载差档算噪声，**不作结论**（判据一律计数）。
  **清单行落点按现量核对**：L251 那一行实测在第 **89** 行 ⇒ L232 清单行那句散文自指由 795 推到
  **796**，`sed -n '796p'` 实测打出的正是含 `docs/augmentor/API.md:1139` 的那一句；790 / 791 / 792 / 795 是
  各自轮次快照，**不改写**。七把账本与文档守卫在账本批之后合跑 **178 passed / 0 failed /
  8 skipped**（那 8 格是 `Temp/` 底料不在场的本机档），未出现平移红。本支自身追加的行数记在
  **它所在提交的 message 里**而不是回填进 ⑮——沿用 L247 到 L250 的处置（自指的行数永远差一格）。

### L252（2026-10-13）— 收 A330：备份一族六员的契约面按实测补错误档，声明集 == 实测集

- **① 这一格当初为什么立、本轮为什么走得动**：A330 是 L243 立的一条「判决全在运行时，契约上看不见」，
  当年它同时留了两格不实之词（「当前处置」里的类名与文档覆盖面），本轮一并订正。走的是原格修法①
  而不是②，理由很硬：①只动六员、不动任何响应体与状态码语义，是**纯加法**；② 要在全仓 77 员上统一
  补档，`test_api_openapi_contract` 那把键集尺子得同轮重量，面大且没有本轮的证据支撑——那 58 员至今
  没有任何一支实测过错误档，给它们补声明就等于把 A330 的病反过来再得一次（声明没有实证的档）。
- **② 判据选型：集合相等而不是「补齐了就行」**：`got == want`（`got` = 契约里去掉框架自动挂的
  `{200, 422}`，`want` = 那一员全部臂真打出来的码集）。两个方向的牙齿不同：少档 ⇒ 客户端踩坑、
  多档 ⇒ 客户端写死分支，而「补齐」式断言（`want <= got`）只防得住前者。注入臂「给只读那员虚声明
  401」正是为了证明后半个方向有牙——那一臂实测红 3 支。
- **③ 29 个真请求的来历**：矩阵不靠推理，靠仓外探针实测。三只探针（`l252_probe.py` /
  `l252_probe2.py` / `l252_probe3.py`，跑在仓库外、用完即删）逐档打出状态码与响应体，
  守卫里的 `ARMS` 就是这张表的代码化：外层键是状态码，内层是 `(人读标签, 打请求的函数)`，
  同一档多臂是因为同一状态码有好几种触发条件（create 的 400 就有「源为空串 / 源含 `..` /
  `name` 指向备份目录之外 / 目录传空串」四臂）。
- **④ 说明面也要有牙**：每一档的 `description` 必须点名是哪颗旋钮或哪一种状态——400 必须点出旋钮名、
  403 必须说「白名单」、401 必须说 `X-API-Key`、409 必须带上「校验和」与「未被改写」（A299 承诺的内容
  本体）。旋钮清单不手抄：`TestTheKnobListIsDerived` 把 `KNOBS` 与三只请求模型的字段集并 `backup_dir`
  做集合相等，加字段不更新说明、或说明里点一颗不存在的旋钮，都会在这一格响。错误体统一声明成
  `{detail: string}`，因为 `HTTPException` 出去的永远是那一句。
- **⑤ 反转 L243 那把尺子的过程**：`test_backup_integrity_l243.py` 的 `TestTheContractDeclaresOnlyTwoCodes`
  当年钉的是**现状事实**，本轮事实变了 ⇒ 改名 `TestTheContractAdmitsTheRuntimeVerdicts`、两只新端点的
  期望集换成实测集（list `200 / 400 / 403 / 422`、cleanup 那只 `200 / 400 / 401 / 403 / 422`），
  模块 docstring 里那三格「契约面看不见判决」3 行换 3 行。**行号约束**：该文件第 322 行被本账与
  `test_parametrize_ids_l97.py` 两处引用，故 1 到 321 行必须净 0——docstring 同长改写、类的改动全部
  压在 322 行以下，`sed -n '320,324p'` 复核未平移（该文件 `git diff --numstat` 实测 13/10、净 +3）。
- **⑥ 反方向缺口另立 A336，不顺手改**：`422` 是框架见「有参数」就挂的，六员里不带请求体的三员
  （list / verify / delete）**构造不出任何一个 422 请求**：`backup_dir` 是字符串查询参数，任何值都合法。
  六档探针实测 `[400, 200, 403, 200, 404, 404]`，`422 not in codes`。本轮只把事实钉成数字，
  摘与不摘都属公开契约的变更，留给拍板。同场量到：同名查询参数重复给值时 FastAPI 取**最后一个**
  （`?backup_dir=<根外>&backup_dir=x` ⇒ 200，反序 ⇒ 403），所以 403 那一族的臂一律只给一个值；
  而 `x` / `y` 这类相对名走的是「首个根解释」那条兜底（A163 的历史），所以 delete 那一档是 404
  「那个目录里没有 a」而不是 403——写探针时我先按直觉猜成 403，被实测否掉，期望码按读数改过。
- **⑦ 磁盘零留痕那一面**：29 个错误请求跑完，备份目录文件名、`index.json` 的逐条形状、数据根顶层
  三处逐字相同。这一格是 A299「409 一个字节都不写」的**续量**——把同一句话从一只端点扩到整个家族，
  并且不是靠注释声明，而是在同一个 fixture 里前后各取一次磁盘读数。restore 那一员另有一支：五档
  全被拒之后数据根仍然只有 `p.json`。
- **⑧ 全仓 census（新增端点会动这三个数，按现量重钉）**：**77 员 = 6 + 58 + 13**。6 是本轮补的；
  58 员仍只有 `{200, 422}`；另 **13 员连 422 都没有**，逐只点名全是**无任何参数**的 GET
  （`/api/health`、`/api/config`、`/api/models`、`/api/status`、`/api/versions`、`/api/demo/data` 那一类），
  框架连 422 都不挂 ⇒ 它们不在 A336 的面上。六员声明过的错误档恰好五码 `{400, 401, 403, 404, 409}`，
  出现第六码即说明有人补了没实证的档。
- **⑨ 四臂注入取证，与第一次尝试怎么失败的**：默认走零写入通道。第一版插件在 `pytest_configure`
  里遍历 `app.routes` 找 `APIRoute` 改它的 `responses`，结果三臂**当场 INTERNALERROR**——这版 FastAPI
  把 `include_router()` 的结果包成 `_IncludedRouter`，顶层既没有 `path` 也没有 `methods`（`integration`
  那把契约尺子当年就为同一件事改过写法），报错是「没找到 POST …/restore」而不是「测试红了」，
  这种红是假的。改成在**导入后的 `app.openapi_schema` 快照**上动手（守卫读的就是 `app.openapi()`），
  第四条臂直接在守卫模块对象上剪 `ARMS` 的一臂，全程不改仓库文件、进程退出即复原。读数：基线
  **31 支全绿**；摘 409 ⇒ **4 红**；虚声明 401 ⇒ **3 红**；409 说明改成空话 ⇒ **2 红**；剪掉 verify
  的 403 臂 ⇒ **3 红**。每臂都点名它要咬的那一支，跑完 `git status` 复核工作树与跑前逐字相同。
- **⑩ 守卫初稿自己的四处手滑（留着不美化）**：㈠ census 把 list 写成四档（实际两档）；㈡ `measure()`
  的断言消息写成了 `assert X if False else Y` 形状的残句，等于永远不报错；㈢ 臂数统计把「档数」当
  「臂数」（`sum(len(entry.values()))` 数的是状态码个数）；㈣ census 那一支第一版返回的是操作对象，
  于是 `set(v) - AUTO` 比的是 OpenAPI 的键名集合，census 两支恒真、外加一支凭直觉猜的 403。
  全部按实测改对之后，剩下那**一支有意的红**是 `test_backup_integrity_l243.py` 的现状事实尺子——
  正是本轮要反转的那一面。**没有任何一把尺子被改松**：改的是判据的形状与期望读数。
- **⑪ 文档面：409 为什么不单起一行**：`docs/augmentor/API.md` 的错误码表加一整行会把下方被账本引用的
  `docs/augmentor/API.md:1139` 一族平移（本账有十余处散文引用那一格，L232 起就吃过两次同一缺陷），
  所以七行全部**同行追加、净 0 行**（`git diff --numstat` = **7/7**）：404 那一行说明「409 不在表里
  是『内容坏了』而不是『资源不在』」以及「文件不存在」在备份一族里既可能指源数据文件不在场、
  也可能指 `backup_id` 不在索引里；六员逐只格各写自己声明了哪几档，verify 那格明写「只读面不声明
  401，也不给幽灵 id 或坏备份声明 404 / 409」。
- **⑫ 门禁读数**：定向簇（l252 + l243 + l231 + l250 四把备份守卫合跑）**110 passed / 0 failed**；
  七把账本与文档守卫在代码批后 **178 passed / 0 failed / 8 skipped**（那 8 格是 `Temp/` 底料不在场的
  本机档）；全量定标 **exit=0 / 9 060 passed / 0 failed / 10 skipped / 6 warnings / 331.69 s**、
  **99.84 %**、`TOTAL` **14 366 / 8 / 3 968 / 22**——语句 +6 全部来自本轮新增的产品代码（一份形状常量、
  一只函数的 def 与其 return、三格说明串），未覆盖与分支三格与 L251 逐格相同。前端三门禁
  **不适用**：`web/` 本轮零改动。
- **⑬ 查出并订正一处半落刀的历史重钉**：L251 日志 ⑯ 写着「L232 清单行那句『本账第 795 行』已按现量推到
  **796**」，本轮复核发现**只落了半刀**——清单行的链尾确实补上了「按现量重钉为 796」，而同一句开头的 operative 那一格一直写着 795；`git show 3a09b3d7a` 的 diff 里
  没有任何「本账第 NNN 行」的增删。⇒ **链上记的数与正文用的数不是同一个数**（目标那一行真值当时确是 796，所以链对、正文错），与 A329 那格
  「账上写的处置与量到的处置不是一件事」、A331 的「落点对、内容错」同族。本轮把 operative 与链尾**同批**落到 **799**
  （796 + 1 清单行 + 2 Backlog 新账），并在链上补记这次订正。**尺子量不到它是既知事实**：
  `test_comment_refs_a184` 与 `test_doc_line_refs_l79` 都判带文件名的 `file:NNN`，不带文件名的散文
  行号正是 A333 名册里的第三种形状（③），本条不因此免于记账。
- **⑭ l242 三格按红读数重钉**：表行 255 → **256**、声明面 126 → **127**、有标记 144 → **145**，
  phantom 46 与 `PARTIAL` 三格未动。A336 按口径 5 写成「新立 **A336**（……）」的括号形状，
  不进声明面 ⇒ `test_gap_equals_partial_roster` 没有多出假欠账。
- **⑮ 遗留**：A330 的修法②（全仓 58 员统一补错误档）仍在待拍面上；A336 待拍；A328 仍是纯加法候选。
  本轮账本批净行数：清单行 **+1**、A330 行与自指那一格原地 **净 0**、Backlog 新账 **+2**、
  日志块追加在文件尾部（不平移任何历史行号）。追加那一格的实际行数记在**它所在提交的 message 里**
  而不是回填进本支——沿用 L247 到 L251 的处置（自指的行数永远差一格）。
- **⑯ 账本批之后的复跑记账**：全量门禁在账本批之后重跑 **exit=0 / 9 060 passed / 0 failed /
  10 skipped / 6 warnings / 350.54 s**，`coverage.xml` 的 `lines-valid` **14 366** / 未覆盖 **8** /
  `branches-valid` **3 968** / 未覆盖 **22**、覆盖率 **99.84 %** ⇒ **四格与账本批前逐格相同**：账本批只动
  `.md` 与 l242 的三行常量（`git diff --numstat` 实测该守卫 **3/3 = 净 0**，测试文件不进产品覆盖率分母），
  于是语句数与偏支数一格未动就是这条的闭合证据；331.69 s → 350.54 s 仍按 L239 / L240 量过的负载差档算
  噪声，**不作结论**（判据一律计数）。**清单行与自指落点按现量核对**：L252 那一行实测在第 **90** 行、
  A330 那一格现在第 **175** 行、新立的 A336 在第 **187** 行，L232 那句 operative 自指钉的 **799** 用
  `sed -n '799p'` 复核打出的正是含 `docs/augmentor/API.md:1139` 的那一句；本支追加在 799 **之下**，所以钉值不因
  本次追加而漂——这是 L251 那处「半落刀」订正之后，链尾与 operative **第一次同数**。七把账本与文档守卫
  在账本批之后合跑 **178 passed / 0 failed / 8 skipped**（那 8 格是 `Temp/` 底料不在场的本机档），
  其间 a184 的硬 0 档**红过一次**：日志块 ⑩ ㈣ 写的是一个草稿期用过、最终守卫里并不存在的名字，
  尺子按「符号主张必须指得到」当场开火 ⇒ 按处方第一条「换点」在同一物理行内改成不点名（净 0 行），
  **这次红是尺子对的、案面错的**。本支自身追加的行数记在**它所在提交的 message 里**而不是回填进 ⑮
  ——沿用 L247 到 L251 的处置（自指的行数永远差一格）。

### L253（2026-10-13）— 收 A328：两卷 Backlog 定级格的标记词汇立成封闭清单，轮号反向可追溯钉成判据

本轮零产品代码、零 `docs/` 改写，改动面只有两卷账本与一把新守卫。收的是 A328 那一格（L242 立账：关闭标记词汇没有封闭清单、轮号只查得到「声明面 ⇒ 表行有标记」这一个方向）。

- ① **判据面从整行换成定级格**（本轮唯一要紧的取舍）。l242 的标记判据扫的是整行，而 Backlog 那一格的正文里满是引用**别的东西**的闭合语——「A95 已关闭（L185）」「ngram 已修（L24）」这一类。两面同场读数：Backlog 区 211 行里整行带闭合标记 **108** 行，定级格带闭合标记 **64** 行，差 **44** 行。定级格是唯一一格「这行的处置结论」由表结构本身承担的地方，所以本轮把它当权威面，正文里的散文爱怎么写怎么写，不该决定一行算不算收口。
- ② 定级格拆成四件结构件：字母形（裸 S / M / L / XS 与划掉 ~~S~~ / ~~M~~ / ~~L~~）、闭合词（紧贴「（ + 轮号」的那 2–3 个汉字）、括号头（`（L` 还是 `（第二本账 L`）、是否粗体。组合名册现量 **16 种**、字母形 **7 档**、闭合词 **5 个**，三张表一律精确相等。词汇表由现量导出而不是先立后用 ⇒ 换新写法（「已了结 + 括号 + 轮号」）当场红，而不是静默地不算标记。
- ③ `半修` 进词汇表而不进收口计数：A156 那一格写作 `M 半修（L88）`，裸字母 + 半程词，既不是已收也不是待拍的常规形状。它与 l242 的 `PARTIAL` 名册对上（A127 / A156 / A45），两把尺子在同一只 ID 上互相印证——这一格是本轮唯一「词汇在场而判据不认账」的现量，由 `test_the_uncounted_word_is_named` 点名。
- ④ **轮号反向可追溯**：每一格闭合语必须从括号里读出至少一个轮号（括号与轮号之间允许限定词，例如「第二本账」「L184，B254」），且读出的轮号必须能在某一卷的进度声明面（`- [x] **LNNN**` 清单行或 `### LNNN` 日志标题）查到。现量同卷可查 **52**、跨卷 **12**、两卷都查不到 **0**、读不出轮号 **0**（补 A31 之前是 1）。
- ⑤ **跨卷那一族钉成双向**：12 行逐格点名（全部在卷一、全部指向卷二、全部在括号头自我声明「第二本账」）。摘掉限定词 ⇒ 配对红（臂 A4），反过来写明「第二本账」而那一轮本卷也查得到 ⇒ 也说谎、同样红（`test_no_cell_says_the_other_volume_when_it_is_this_ones_own` 现量空集）。A328 立账时只数到 3 行，是因为当时只看了 A46 / A130 / A138 三格；本轮按声明面逐卷核对成 12 行。
- ⑥ **立账那五条现量按本轮实测作废并改写**：有标记的行不是 132 而是 108 / 64 两面（差 44 行 ⇒ 新立 A337）；严格式「紧跟括号的轮号」解析不出的是 10 行，换成「括号内第一个轮号」之后 **0** 行——代价是判据松了一格，所以同批补上反面判据（括号里没有轮号 = 不可追溯）；本卷查不到而需跨卷的是 12 行不是 3 行；形状不是 7 种而是四件结构件的 16 种组合。原格还留着一句「放宽到行内任意轮号则 132 行全部可查，但那已经不是标记里的轮号」——本轮读的正是定级格里的轮号，所以这条反对意见没有作废，它变成了「为什么不能拿整行当判据」的同一条理由。
- ⑦ **A31 那一格是真缺陷，本轮收掉**：卷一 A31 写作划掉字母而整格没有任何轮号——划掉承诺「已收」却查不到哪一轮收的，反向可追溯在它身上是现成的盲区。按声明面现量补成划掉字母加 L29 那一轮（`- [x] **L29**` 那一条写着 A31 的回显口径补齐、`SearchResult` 新增那一格），A328 原格把它记在「另 3 行跨卷」那一档，读数错了：A31 那一格根本没有轮号可查。
- ⑧ **新立 A337**（两格差 44 行 + 日志块 46 行污染表行面）：本轮只量不补标。36 行的补标要逐行核对「正文那句闭合语说的确实是本行整行」——A130 / A270 / A275 / A277 / A302 / A312 与 B201–B225 那一族看着都能补，但 B2 那句写的是「前半关闭」、A21 那句说的是 ngram 那一半，机械补标会把没收的写成收了。日志块那 46 行（A1–A8 是注入臂名册、B226–B263 只在日志块里出现）让 l242 的「表行数」读数含义不唯一。
- ⑨ 六臂注入取证写成守卫内的内存改行，磁盘零副作用（跑完守卫两卷账本 `git status` 一字未动）：A1 换清单外的新动词 ⇒ 组合名册与词汇表同时变（证明封闭清单不是恒真）；A2 摘掉轮号 ⇒ 无轮号名册点名那一格；A3 谎报一个不存在的轮号 ⇒ 不可追溯名册点名那一格；A4 摘掉「第二本账」⇒ 跨卷配对红；A5 给待拍的 A335 补一格 ⇒ 权威面 +1 而宽面不动；A6 只在正文写一句闭合语 ⇒ 宽面 +1 而权威面不动。A5 与 A6 是同一条主张的两个方向：宽面与权威面各自只对一种改写敏感，这正是 l242 那把尺子松在哪里。
- ⑩ l242 三格由该守卫自己的红读数重钉（表行 256 → **257**、声明面 127 → **128**、有标记 145 → **146**），phantom 46 与 PARTIAL 三格未动。三格在本批落刀前的红读数分别是 `assert 257 == 256` / `assert 128 == 127` / `assert 146 == 145`，其中声明面那一笔来自 A328 第一次进声明面（清单行开头与日志标题各点一次，同一只 ID 只算一笔），A337 按口径 5 写成括号形状 ⇒ 不进声明面、不造出假欠账，`test_gap_equals_partial_roster` 那一支本轮实测未红。
- ⑪ 本轮新守卫在账本批**之前**跑出的红读数逐格留底（不美化）：**9 支红 / 11 支绿**。红的六格现量是表行 `assert 257 == 256`、宽面 `108 == 107`、权威面 `64 == 63`、同卷 `52 == 50`、字母形三档差（`{M: 57, S: 92, ~~S~~: 32, ~~M~~: 24}`）与组合名册差出一格；另三支是 A31 未补标那一族的连带红（`['A31'] == []`、摘轮号那一臂报 `['A31', 'A330']`）与表行那一格所在的支。**同一支只报第一处 assert**：表行那格先响，所以 Backlog 区 **211** 那一格的红读数被它遮住、本批落刀前没有单独现出过——它是本轮唯一不是由红读数逼出来的现量，由账本批之后的 20 支全绿补证，如实记下不假称。草稿期还红过两支：「半修」的名册我先写成 A157、实测 **A156** ⇒ 按现量重钉后转绿。**账本批之后同守卫 20 支全绿**，六臂各自点名喊红。
- ⑫ **L232 那句 operative 自指按现量 799 → 802**（本批 +1 清单行、+2 Backlog 新账连带它的分隔空行），链尾补第十个值（781 / 786 / 789 / 790 / 791 / 792 / 795 / 796 / 799 / **802**），并用 `sed -n '802p'` 复核落点打出的正是含 `docs/augmentor/API.md:1139` 的那一句——这一形不带文件名，a184 的结构维与内容维都不量它（A333 的第三种形状），所以只能靠人眼加复核命令。
- ⑬ 门禁：七把账本与文档守卫合跑读数见本支所在提交的记账；前端三门禁**不适用**（`web/` 零改动）。本轮没有产品代码与对外契约的改动，因此不需要重钉 l97 / l98 / l200 的普查读数（一轮零新 `parametrize` 调用点，端点数与 tag 数一字未动）。
- ⑭ 待拍面本轮减一（A328 收了）加一（新立 A337）：A301 / A306 / A308–A311 / A313–A318 / A323 / A324 / A333 / A335 / A336 / A337，其余定级格一律未动。

- **⑯ 账本批之后的复跑记账**：全量门禁在账本批之后重跑 **exit=0 / 9 080 passed / 0 failed / 10 skipped / 6 warnings / 341.12 s**，`coverage.xml` 的 `lines-valid` **14 366** / 未覆盖 **8** / `branches-valid` **3 968** / 未覆盖 **22**、覆盖率 **99.84 %** ⇒ **四格与 L252 终态逐格相同**：账本批只动两份 `.md` 与 l242 的常量加 docstring（`git diff --numstat` 实测该守卫 **9/6 = 净 +3**，测试文件不进产品覆盖率分母），于是语句数与偏支数一格未动就是这条的闭合证据；用例闭合 = L252 终态 9 060 + 本轮新守卫 20 = **9 080**；341.12 s 与 L252 的 350.54 s 仍按 L239 / L240 量过的负载差档算噪声，**不作结论**（判据一律计数）。八把（七把账本与文档守卫 + 本轮新守卫）合跑 **198 passed / 0 failed / 8 skipped**（那 8 格是 `Temp/` 底料不在场的本机档），闭合 = L252 的 178 + 新守卫 20。
- **⑰ 清单行与自指落点按现量核对**：L253 那一行实测在第 **91** 行、A328 那一格现在第 **172** 行、新立的 A337 在第 **190** 行、本轮日志块标题在第 **4489** 行；L232 那句 operative 自指钉的 **802** 用 `sed -n '802p'` 复核打出的正是含 `docs/augmentor/API.md:1139` 的那一句。本支追加在 802 **之下**，所以钉值不因本次追加而漂——链尾与 operative 自 L252 第一次同数之后，本轮仍是同数。
- **⑱ 本轮批写撞到的一条事实，订正历轮自述**：卷二**不是**「纯 LF」——正文里有一处**字面 CR 字节**（那一格在讲行尾守卫，写的是「每一个 `\n` 都前接一个 `\r`」），现量 LF **4 484** / CR **1** / CRLF **0**。批写脚本原本断言「输出里不许出现 CR」，于是它在落盘前当场把这一批拒了（先写的卷一已落、卷二未落，按 `git restore` 把卷那一格恢复后重跑）——**那次红是脚本对的、案面错的**：我此前历轮账本里写的「两卷纯 LF」是不精确的自述，精确说法是「无 CRLF 行尾」。断式据此收窄成「不许出现 CRLF 行尾」，那个字面 CR 是内容而不是行尾，须原样留着。l242 与本轮新守卫的读取路径本来就把 CRLF 归一成 LF 之后再按换行切行，所以这一字节在场时两把尺子合跑 **44 passed** 逐格绿（判据不受影响，已由该读数证明）。
- **⑲ 哈希槽**：本支之后全账只允许 L254 那一格留占位，l98 的 R 档现场绿（占位数量棘轮未升）。本支自身追加的行数记在**它所在提交的 message 里**而不是回填进 ⑯——沿用 L247 到 L252 的处置（自指的行数永远差一格）。


### L254（2026-10-13）— 收 A337：l242 的表行面按「## 循环日志」分区，日志块那 46 行名册不再算 Backlog 表行

本轮零产品代码、零 `docs/` 改写，改动面只有两把账本守卫与本卷账。收的是 A337 那一格的修法③（L253 立账：表行面把日志块里的同名表格也算成 Backlog 行，「表行数」含义不唯一），① 与 ② 两半拆成新行 A338 继续待拍。

- ① **改的是面的定义而不是判据的松紧**：`table_rows` 加 `backlog_only`（默认开），边界只认 `## 循环日志` 这一形。**为什么不用 `### L` 当边界**：卷一的日志块标题不是那一形（历轮量过），拿它当边界会让卷一整个日志区留在 Backlog 里——那正是「含义不唯一」的另一半。边界唯一性由 `test_the_boundary_is_unique_per_volume` 现场验，两卷各恰 1 处。
- ② **被请出去的 46 行逐格点名**：A1–A8（注入臂名册）与 B226–B263（某一轮收口清单），`test_the_filtered_out_rows_are_the_named_forty_six` 除了点名名册还逐格断言行号在边界之下——过滤是按区段做的，不是按 ID 形状挑的。
- ③ **现形出一族此前根本看不见的账**：B226–B263 那 38 只从未有过 Backlog 行的 ID，此前在「进度声明面说收了 ⇒ 表行有标记」这一判据里**同时充当被核对的账与自己那条证据**（同一张日志块里的表格，一面是声明一面是账）。过滤之后它们改归 phantom，而 l242 把口径 2 那一格从 46 拆成「历史裁撤 **46** + 日志名册 **38**」，两族各有一支专名用例点名。
- ④ **核心判据一格未动**：过滤前后 `declared ∩ rows − marked` 都是 `PARTIAL` 三格（A127 / A156 / A45），由 `test_the_gap_is_the_same_on_both_faces` 两面同钉。这一条是本轮唯一能证明「区段过滤不是把尺子调松」的判据——如果只重钉读数而不钉这一条，改面与改松就分不出来。
- ⑤ **唯一被调低的数字**：`MIN_DECLARED` 100 → **85**。留在册的声明面从 128 缩到 91，下界必须跟着面走，否则口径变更当场变成假阳性红。放松的担心由另一面挡着：不过滤那一面同批钉成 **129**（`test_the_unfiltered_face_still_pairs_with_the_filtered_one`），解析器一旦退化到看不见东西，两面里至少有一面先响。归因写在常量注释里，不留「下界自己就变小了」这种说法。
- ⑥ **两把尺子在同一只面上对上数**：l242 过滤后的「有标记」名册（109）与 l253 的「整行带闭合标记的 Backlog 行」是同一集合，由 l242 的 `test_the_other_ruler_counts_the_same_wide_face` 断言集合相等；l253 的定级格那一面（65）严格小于它——44 行那格差是 A338 的账。**这次对数是本轮最硬的证据**：两把尺子各自独立定义边界、独立扫面，读数相同才说明 109 不是某一把尺子的口径产物。
- ⑦ **l253 同步五格**：表行 257 → 258、Backlog 区 211 → 212、定级格闭合标记 64 → 65、整行那一面 108 → 109、同卷可查 52 → 53；闭合词 `已收口` 32 → 33、字母形 ~~S~~ 32 → 33，裸 S **净 0**（A337 离格、A338 入格），组合名册 `~~S~~ / 已收口 / 括号 L / 不粗体` 那一档 7 → 8——**A337 落进已有形状，没有造出第 17 种形状**，这一点由组合名册的精确相等当场证明。
- ⑧ **账本批之前的红读数逐格留底**：两把尺子合跑 **12 支红 / 39 支绿**，红的是 l242 五格（`assert 211 == 212` / `assert 257 == (212 + 46)` / `assert 128 == (91 + 38)` / `assert 90 == 91` / `assert 108 == 109`）与 l253 七格（`assert 257 == 258`、宽面 108 == 109、字母形差 `~~S~~ 32 vs 33`、闭合词差 `已收口 32 vs 33`、组合名册差出 `((~~S~~ / 已收口 / L / False), 7 vs 8)`、定级格面 64 == 65、同卷 52 == 53）。**账本批之后两把尺子全绿**，新七支各自点名。
- ⑨ **L232 那句 operative 自指 802 → 805**（本批 +1 清单行、+2 A338 连带它的分隔空行），同时补一处账本卫生：L253 那一批把数字接进链尾却没写归因，本轮按现量补成「L253 落 +3 ⇒ 802」，链尾至此列出十一个值（781 / 786 / 789 / 790 / 791 / 792 / 795 / 796 / 799 / 802 / 805），并用 `sed -n '805p'` 复核落点打出的正是含 `docs/augmentor/API.md:1139` 的那一句。这一形不带文件名，a184 的结构维与内容维都不量（A333 的第三种形状），所以只能靠人眼加复核命令。
- ⑩ **新立 A338**（① 36 行的定级格逐行人核对 + ② 把 l242 的标记判据也从整行收到定级格）：本轮不做①——那是人眼活，A21 那句只管 ngram 一半、B2 那句写着「前半关闭」、另 8 行引用的是别只 ID 的账，机械补标会把没还的债划掉；②单独做会当场红出三十多支，其中混着真没收的，必须与 ① 同批。A338 按口径 5 写成括号形状 ⇒ 不进声明面，`test_gap_equals_partial_roster` 没有多出假欠账。
- ⑪ 门禁：八把账本与文档守卫（七把原有 + 本轮 l253）与全量门禁的读数记在账本批之后的记账支；前端三门禁**不适用**（`web/` 零改动）。本轮没有产品代码与对外契约改动，因此不需要重钉 l97 / l98 / l200 的普查读数（零新 `parametrize` 调用点，端点数与 tag 数一字未动）。
- **⑪′ 本轮自己造出来又被尺子当场抓住的假声明（如实记下）**：账本批落盘后 l242 出 4 支红——`test_declaration_face_is_populated` 报声明面 92 而不是钉的 91，`test_gap_equals_partial_roster` 与两条双面支的名册多出 A338 那一格。归因不是读数错而是**散文粘出来的**：清单行与本日志 ⑦ 各写了一句「…（A337 离格、A338 入格），闭合词 `已收口` 32 → 33…」，名字在前、判决动词在后、中间只隔 10 个字符 ⇒ 按口径 5 的 AFTER 形状被粘成一句判决，而那一格按表结构仍是待拍。**改法走处方第一条「换点」**：清单行把判决动词挪到名字之前、日志 ⑦ 把两半顺序对调，两处各自净 0 行；没有重钉 91 → 92，那会把一处散文缺陷记成读数。**这一支是口径 5 那条断口的现行复刻**（L253 立 A337 时用的就是同一条规避：括号头先断），而且它分两次才被抓全——第一处由窄面（清单行）当场红，把清单行改完之后剩下的那一支红来自宽面支 `test_wide_scope_would_give_the_same_gap`，它扫的是日志块正文，正好说明「正文不进声明面」不等于「正文不会骗人」。
- ⑫ 待拍面本轮减一（A337 收了）加一（A338 立账）：A301 / A306 / A308–A311 / A313–A318 / A323 / A324 / A333 / A335 / A336 / A338，其余定级格一律未动。
- **⑬ 账本批之后的复跑记账**：全量门禁 `python -m pytest tests -q -p no:cacheprovider` = **9 087 passed / 0 failed / 10 skipped**，TOTAL **14 366 / 8 / 3 968 / 22**，覆盖率 **99.84 %**——与 L253 终态（9 080 / 0 / 10、14 366 / 8 / 3 968 / 22、99.84 %）逐格相比**只多 7 支用例**，少的地方一格没有：那 7 支正是本轮新增的 `TestTheSectionBoundary`（区段边界那一族），产品语料面（语句 / 未盖 / 分支 / 未盖分支）四格一字未动 ⇒ 本轮确实是一轮「只改面的定义」的账本轮。八把账本与文档守卫合跑 **205 passed / 0 failed / 8 skipped**（L253 终态 198，+7 同一族），两把 Backlog 尺子单跑 51 passed（24 + 7 边界支 + l253 的 20）。8 skip 全是 l79 那条「需要作者工作树」的既有条件跳过，本轮没有新增跳过。
- **⑭ 自指落点与本批自述**：L232 那句 operative 自指仍钉 **805**，落点行复核含 `docs/augmentor/API.md:1139`（本批两支追加在文件末尾的日志块里，在 805 之后 ⇒ 行号不漂，这一形依旧没有尺子量，正是 A338 立账的理由）；L254 清单行实测仍在第 **92** 行，行首哈希槽已是 e50a3726d 真值，全账「哈希待 Lxxx 回填」占位仍只有历史那 14 条（l79 的棘轮未升）。两把尺子跑完两卷账本 `git status` 一字未动 ⇒ 零磁盘副作用。本批自身净行数记在它所在的提交 message 里，不写在案面上（写了就要重量）。



### L255（2026-10-13）— 收 A316：三处文案里的三个告警阈值与 SDK 当下真的在用的那三个立成同一把尺子

本轮零产品代码、零 `docs/` 与 `web/` 改写，改动面只有本卷账与一把新守卫。收的是 A316 那一格（L234 立账：页面把三个告警阈值写死在文案里，与 SDK 的默认阈值之间没有任何机器对账）。

- ① **账比原格多一面**：立账时只写了页面卡片，本轮按现量普查到同一串数在仓内**三处**——「系统状态 → 质量监控与自测」卡正文、`web/src/types/api.ts` 里那段类型注释、`docs/augmentor/API.md` 的 monitor 那一行。守卫判的不是「每处若存在则相等」（那会在有人删掉一处之后静默变绿），而是「三处各恰 1 处、总数精确等于 3」。
- ② **判据比的是值**：`0.70` 与 `0.7` 是同一个数的两种排版，按 float 判相等，读数里两边的字面都留（红的时候报「diversity 从 0.45 到 0.4」）。会红的不是排版而是**换写法**：把三个数拆成「70%」那种本守卫抽不到的形状，站点数当场从 3 掉到 2——这一支同时是尺子的反恒真证明。
- ③ **行为维**：光比「文案 vs SDK 属性」仍是两份静态文本对读。把三项指标全压到阈值之下走告警那一路，要求每条告警正文里印出来的阈值与文案是同一个数（0.70 / 0.40 / 0.70）。这一支管住的是「用户真正在告警里读到的那个数」，SDK 调档、或把打印格式从两位小数改掉，都红在这里而不是只红在文本那一支。
- ④ 四臂注入取证（全在内存里，磁盘零副作用，跑完自验三处文案的字节与进入时相同）：B1 把卡片正文的 diversity 改成 0.45 ⇒ 名册点名 diversity 那一档；B2 把 SDK 的 completeness 调成 0.75 而三处文案不动 ⇒ 同一把尺子反向点名；B3 把 `docs/augmentor/API.md` 那一处的连写拆掉 ⇒ 站点总数少一格；B4 给 SDK 多加第四个带 alert_below 的指标 ⇒ 指标集合当场变，而文案的覆盖不再算齐。
- ⑤ **四臂为什么写成四条显式用例而不是 parametrize**：初稿是 `ARMS` 表加一处 parametrize，落盘后定向复跑 l98 那把参数化普查报两格红——档位计数 derived 38 → 39，且它与 l97 静态尺子的调用点总数同时不合。判据与参数化无关（四臂各有独立期望值与独立报错信息），所以按现量改形不改判据。**这一格的结论要留给后面的轮次**：新增一处 parametrize 就要同批重钉 l98 的档位计数，那不是可选的同步。
- ⑥ **同一次复跑里的五支 error 不是本守卫的账**：它们走会话级的采集 fixture，而该 fixture 拒绝「本次采到的用例少于声明下限」的那种跑法形状——定向复跑本来就采不满，全量门禁里这五支一直是绿的。本轮把它们与两格真红分开归因并写进守卫口径 7：把「尺子的跑法形状」记成「新守卫让尺子红了」是假归因，而假归因比红更贵。
- ⑦ **A316 未做的两格留在原行而不是另立新账**：① 修法②（把生效阈值放进 monitor 的响应面，让页面不再抄数）是契约变化，牵动键集尺子与 TS 类型两侧，与 A335 同族，仍待拍；② 告警正文是英文单轨，与本仓 4xx 中文判决的口径不一致，那一格本来就是 A311 那一族的账。本轮只把**数字**对账，没有动语种，也没有动响应面。
- ⑧ 两把账本尺子按本轮红读数重钉：l242 声明面 91 → **92**（A316 第一次进声明面：清单行开头的「收 A316——」与日志标题各点一次，同一只 ID 只算一笔）、有标记 109 → **110**；l253 定级格闭合标记 65 → **66**、整行那一面 109 → **110**、同卷可查 53 → **54**、字母形裸 S 93 → **92** 与划掉 ~~S~~ 33 → **34**、闭合词已收口 33 → **34**、组合名册 `(~~S~~ / 已收口 / 括号 L / 不粗体` 那一档 8 → **9**（A316 落进已有形状，没有造出第 17 种）。表行三面（258 / 212 / 46）与 phantom 84 一格未动 ⇒ 本轮没有新立 Backlog 行，也没有裁撤任何行。
- ⑨ L232 那句 operative 自指按现量 805 → **806**（本批只有 +1 清单行），链尾补第十二个值，落点以 `sed -n '806p'` 复核仍是含 `docs/augmentor/API.md:1139` 的那一句；这一形依旧不带文件名、a184 两维都不量，仍是 A333 / A338 那一族的现量证据。
- ⑩ 待拍面本轮减一（A316 收了，它原在 A313–A318 那一段里，所以名单从 A313–A318 拆成 A313–A315 与 A317–A318 两段）：A301 / A306 / A308–A311 / A313–A315 / A317–A318 / A323 / A324 / A333 / A335 / A336 / A338，其余定级格一律未动。
- **⑪ 账本批之后的复跑记账**：全量门禁 `python -m pytest tests -q -p no:cacheprovider` 回 **9097 passed / 0 failed / 10 skipped（323.69 s）**，覆盖率 **99.84 %**，`TOTAL 14366 / 8 / 3968 / 22`（语句 / 缺失 / 分支 / 部分分支）。与 L254 终态的 9087 之间**只差本轮新增的那 10 支**（`tests/unit/test_quality_threshold_copy_l255.py`），产品语料四格逐格相同——本轮零产品代码，这正是预期而不是巧合。10 格 skip 的真实构成：8 格是 `test_doc_line_refs_l79.py:647` 的作者工作树条件（`Temp/` 取证草稿与 `data/` 数据集底料未入仓），另两格各是 `test_config_gates_l82.py:573`（export 节没有开关）与 `test_upload_ceiling_l87.py:580`（干净克隆里没有真实数据集可定标），**都不是本轮跳过的**。
- **⑫ 落盘形状与自指复核（含本轮自己造出来又被当场抓住的第二个缺陷）**：八把账本与文档守卫 **205 passed / 8 skipped**，与 L254 同数（本轮没有新增账本守卫文件，205 那一格因此未动）；两把 Backlog 尺子加本轮新守卫 **61 passed**。卷二本批净 **+17 行**（清单行 1 + 日志块 16），四处 hunk 与四处预定改动逐一对应（L232 行、清单插入、A316 行、尾部日志块）。**但记账批的第一步就撞上上一批的落盘缺陷**：脚本 `assert v2[-1] == ""` 只验进入时以换行收尾，`extend` 之后 `"\n".join` 产出的末元素是 ⑩ 那一支，**写出去的文件因此没有末尾换行**（本批 `lines[j+1]` 直接 IndexError 现形）。这一形六把守卫都不量（它们按文本读，md 结构守卫因此一路绿），所以它靠的是「脚本自己读回自己的行」而不是靠尺子——如实记下：**写大文件批处理要在写出后单独验一次 `text.endswith("\n")`**，本批已补回该形状并与进入时一致。行尾口径不变：**`b"\r\n"` 零命中、内容 CR 恰 1 处**（那一处 CR 是正文里的字面字符，不是行尾）。L232 那句 operative 自指钉 **806** 且本批不再平移——追加的物理位置在文件末尾、806 之后，落点复验仍含 `docs/augmentor/API.md:1139`；这一形依旧没有尺子量（A333 / A338 那一族的现量证据，本批照旧未动）。

### L256（2026-10-13）— 收 A324：split 的比例判据两门三容差并成一条权威

- ① **缺陷事实（三处实测，全部在改前取）**：`0.4/0.3/0.29` 十进制和 0.99、浮点差 `0.010000000000000009`，被门口那行「不含端点」的 `> 0.01` 判成 `400 分割比例之和必须为1.0，当前为 0.99`——最自然的「留两位凑 1.0」被误拒；`-0.8/0.1/1.7` 走默认支 **200** 且把 6 条全塞进 test 段（`0/0/6`），分层支同一份输入 **400**；`ratios` 喂 4 个元素时两条支路都不判长度，SDK 直构过校验后死在 `ValueError: too many values to unpack (expected 3)`。
- ② **改法：两处手抄都删，判据只住一处**。`dataset_ops.split` 门口读 `require_ratio_list("ratios", config.ratios)`，`DataSplitter.__init__` 读 `require_ratio_list("比例", (train_ratio, val_ratio, test_ratio))`，同一份判据（长度 3 + 逐项 bool/NaN/非数值/0..1 + 和 ±0.01 含端点、量化 9 位）。这一族判据自 A124 / L153 起住在 `validation`，本轮是把权重族旁边的第二处同族手抄接进去，不是新造判据。
- ③ **容差放宽为什么安全**：`largest_remainder` 按 `share / weight_sum` 归一化摊派，配额之和恒等于条目数，所以和偏离 1.0 在容差内不会丢条目、也不会多出条目——这一条由判绿两档的「三支段之和 == `len(CORPUS)`」与 API 面的 `sum(counts) == 6` 各钉一次。
- ④ **A57 的取舍写清**：A57 记的是「统一方向要碰默认侧的破坏性变更」，本轮**没有**采用那一支，取的是抬分层侧、并给默认侧补它原本缺的长度与逐项判据。默认侧的可接受域因此**只增不减**（不含端点 → 含端点，且新增的两道判据只拒绝本来就该拒绝的形状），分层侧的可接受域缩小到与默认侧同一条。留在账上的仍是 ④ 那一格：`shuffle` 在两支上语义不同（A60 / A289）。
- ⑤ **测试按「同答」重写**：`tests/unit/test_transform_wiring_l229.py` 把原「+0.005 两支一过一拒、负数两支一过一拒」四格换成「同一份 ratios 两支同码同答」四档（判红：负比例 / 和 1.2；判绿：和 0.99 / 和 1.005），并加一条跨面等式 `strict.detail == loose.detail`——入口那道门先答，两支共引同一条判据，所以连判决里的参数名都是同一格（直构 `DataSplitter` 才会读到 `比例` 那一份）。`tests/unit/test_dataset_ops.py` 的 `test_strict_ratio_rules_apply_on_the_stratified_path` 改名成 `test_both_branches_reject_the_same_illegal_ratios`，同一处 parametrize 站点只换档位与 ids（**站点数未增** ⇒ l98 的档位计数一格未动，已复核）。
- ⑥ **零写入注入取证（三臂 + 基线）**：`dataset_ops` 臂摘掉门口判据 ⇒ **5 红**（`test_split_invalid_ratios`、同答两支的两档、API 两面）；`data_splitter` 臂摘掉分层门 ⇒ **2 红**（`TestInit::test_rejects_ratio_sum_not_one` / `test_rejects_negative_ratio`）；双臂 ⇒ **7 红**。基线同一次跑 **253 passed**。补丁只打在导入后的模块对象上，仓库字节全程未变，收尾以 `git status --short -- augmentor` 复核。
- ⑦ **防复发**：`tests/unit/test_decision_gates_l238.py::test_no_hand_copied_sum_rule_survives_in_the_four_callers`（原名 `..._two_callers`）把「不许再手抄 `if abs(`」的名册从 2 个调用点扩到 4 个，四格同时要求该文件里读得到 `require_ratio_list(`。判的是代码行而不是散文，两份 docstring 里「改前是 `abs(...)`」的历史叙述不进这一档。
- ⑧ **a184 首跑红 3 格与归因**：账本 A289 行点的三处坐标（dataset_ops 门口 427 行、data_splitter 的 63 与 67 行）随代码形状变成注释行 ⇒ `dead_line` 从 0 跳到 3。处置是**改自己的散文**：三处行号不再点，本条刻意写成不带反引号的散文形——带反引号才是 a184 的扫描面，把同一串数原样抄进日志块会自己再造三格死引用；①②③ 就地记为已并入，④ 留在原格待拍。尺子一格未松，`CEILING` 里那一档仍是 0。
- ⑨ **净行数复核**（`git diff --numstat`，四处逐一对账）：`dataset_ops.py` 5/5、`data_splitter.py` 8/8、`api/routes/dataset_tools.py` 3/3、`docs/augmentor/ARCHITECTURE.md` 4/4、`web/src/services/api.ts` 3/3、A289 行所在账本 1/1 ⇒ 活引用零平移，`test_doc_line_refs_l79` 与 `test_comment_refs_a184` 的落点全部原地复核。
- ⑩ **前端三门禁本轮适用**（`api.ts` 有文案改动，不是不适用）：`tsc --noEmit` rc=0、`eslint . --max-warnings 0` rc=0、`vitest run` **22 文件 / 308 支全绿**（120 s）。改的是 docstring 散文，没有动类型面与调用面。
- ⑪ 待拍面减一（A324 出名单）：A301 / A306 / A308–A311 / A313–A315 / A317–A318 / A323 / A333 / A335 / A336 / A338，其余定级格一律未动；A289 的定级格也**未动**（仍是 M，本行只剩 ④ 那一格待拍）。
- ⑫ **L232 那句 operative 自指按现量 806 → 807**：本批 +1 清单行、零新 Backlog 行 ⇒ 只平移一行；链尾补第十三个值，落点以 `sed -n '807p'` 复核仍是含 `docs/augmentor/API.md:1139` 的那一句。这一形依旧不带文件名、a184 两维都不量（A333 / A338 那一族的现量证据）。
- ⒔ **账本批之后复跑八把账本与文档守卫**：205 passed / 8 skipped，与批前基线逐字同读数（8 支 skip 全为 `test_doc_line_refs_l79.py:647` 的作者工作树条件）。两把 Backlog 尺子按现量重钉：l242 声明面 92 → **93**、有标记 110 → **111**；l253 六格 66 → **67**、110 → **111**、54 → **55**、裸 M 56 → **55** 与划掉 `~~M~~` 25 → **26**、闭合词已收口 34 → **35**、组合名册 `(~~M~~ / 已收口 / L / 不粗体)` 7 → **8**——落进已有形状，名册精确相等当场证明没有第 17 种形；表行三面 258 / 212 / 46 一格未动。**没有任何一把尺子被改松**，六格全部由账本批自身形状推出并按守卫打印的实测量钉死。
- ⒕ **全量定标门禁**：`python -m pytest tests -q -p no:cacheprovider` ⇒ **9098 passed / 0 failed / 10 skipped，309.30 s**，TOTAL `14360 8 3962 22`、**99.84%**。与 L255 终态（9097 passed、14366 语句、3968 分支）三处差各有账：用例 **+1**（`test_dataset_ops.py` 新增的那一支「三档两位凑 1.0 全放行」）；语句 **−6**、分支 **−6**——本轮删掉的正是三处手抄判据的三个 `if` 与它们的三个 `raise` 体（`if abs(total - 1.0) > 1e-6` / `if any(r < 0 for r in ...)` / `if abs(total_ratio - 1.0) > 0.01`），3 个 `if` 带 6 条分支、3 条 `raise` 带 3 个语句，合起来 −6/−6，与覆盖率分母的读数逐位对上；10 支 skip 全是既有环境条件（l79 作者树 8 + l82 export 节 1 + l87 无真实数据集 1），本轮未新增任何跳过理由。落盘形状复核：`crlf 0 / cr 1 / 以换行结尾`，第 **807** 行仍是含 `docs/augmentor/API.md:1139` 的那一句（本批两行追加在文件末尾，在其后 ⇒ 零平移）。前端三门禁不适用（本批只动账本）。

### L257（2026-10-13）— 收 A285：默认文本面五处手写字面量并成 privacy.DEFAULT_TEXT_FIELDS 一份权威

- ① **本轮的问句**：A285 从 L227 挂着的原因是「一份字面量抄五遍，谁是权威没有答案」，而它同时挡住 l227 那把跨语言尺子的升级——没有常数，页面档位与后端合法域就只能用运行时判据对账。收法按原格建议走：抽模块级常数，五处按各自面的既有口径就位。
- ② **权威的形状**：`DEFAULT_TEXT_FIELDS = ("instruction", "input", "output")`，**元组**。取元组不是审美：模块级可变列表被五处共引，任何一处就地 append 会无声改掉其余四处的默认面。注入臂 A3（把定义换成列表字面量）在「内容与形状面」与「共引者不能互相污染」两条上同时红，证明这一格不是装饰。
- ③ **归属拍定与它的两层理由**：住 augmentor/privacy.py。不建新模块——A184 那把注释引用尺子会把「指向仓内并不存在的文件」记成 dead_path，A285 原格就记过一次（第一版给的正是还没建的新模块名，当场红两支）。不插在 privacy 第 76 行之前——那一行与 audit 第 68 行是账本第 128 行、l227 与 l228 的模块 docstring 共四处活引用的落点，在其上插行会把落点平移成上一行（A337 / A338 那一族的形状），而 a184 的结构维只问「落点还是不是代码」，平移后大概率**不红**——这正是本轮不肯留的债。于是常数写在文件末尾、取用点在运行时查名，引用与权威同时成立。
- ④ **SDK 两面真引用**：privacy 侧走 is-not-None 形、audit 侧保持 or 形，两边都取 list(权威)，都是**同行改写**（audit.py 净 2/2、privacy.py 除尾部追加外同一行），返回类型仍是列表，显式传入（fields=["output"]）的行为不变。守卫第 1 面验「两个模块绑定是同一个对象」，抄第二份的写法在这一面必红（注入臂 A1 实测 2 红）。
- ⑤ **CLI 那一面为什么不引用**：`augmentor/cli/parser.py` 的模块 docstring 在 EXPORT_FORMATS 与 INPUT_FORMATS 两处写明本模块**不 import 任何 augmentor 业务模块**，一致性由守卫比两份常量。AGENTS 第 7 条要求把冲突摊开而不是各取一半：这一轮选既有口径（l189 与 l226 两把尺子的叙述把 CLI 那份手写字面清单当作第三漂移面在管），因此 parser.py 一行未动，三档 --fields 默认由第 4 面逐档真解析 argv 之后与权威逐项相等（注入臂 A4 漂一档 ⇒ 2 红）。同场 analyze（两格）与 check-leakage（一格）不属于这一族，第 4 面把它们逐一点名而不是收编——「凡是叫 fields 的都并进来」是假收口。
- ⑥ **A5 探针抓到尺子自己的洞**：第 4 面那条「parser 没有 import 权威」原本写成行首正则。注入「import argparse; from augmentor.privacy import DEFAULT_TEXT_FIELDS」这种分号偷带之后 **0 红**——from 不在行首，正则看不见，而这条断言的全部意义就是拦这种 import。改成 AST 枚举真实 import 面（收 Import 与 ImportFrom 的模块名）之后同一臂 1 红，散文里提到 augmentor 也不再可能误红（禁令管的是导入，不是措辞）。这条「先写歪的尺子」留在本账，不改写成看起来一开始就对了。
- ⑦ **A339 是本轮量到的，不是本轮顺手改的**：两侧对空容器的判据不同——privacy 侧 L188 已把 or 改成 is-not-None，audit 侧仍是 or。实测 PiiSanitizer 传空列表得到空列表，DatasetAuditor 传空列表得到默认三格。统一两侧会让审计的**空值统计面变窄**（字段计数随之变小），是行为变更，留 A339 待拍；本轮只把现状钉成断言（注入臂 A6 改掉 or ⇒ 2 红，其中一支正是那一格）。
- ⑧ **A285 那一行的五个落点为什么仍写得住**：收口之后落点内容变了（两处变引用、三处仍是字面量），行号一格未漂，所以那一行仍按原号引用而不重钉。这也是本轮坚持「同行改写 + 尾部追加」的直接理由：净行数零 ⇒ 不需要为四格行号写归因。
- ⑨ **净行数与落盘形状**：`git diff --numstat` —— audit.py 2/2、privacy.py 20/1（1 行同行改写加 19 行尾部追加）、parser.py 不出现在 diff 里。账本本批净 +18（清单行 +1、新立 A339 连带它的分隔空行 +2、日志块 +15），其中只有前两处落在第 810 行之前，故 L232 那句 operative 自指按现量 807 → 810、链尾补第十四个值，落点行复核仍含 API.md 的那一条引用。写前读数 crlf 0 / 裸 CR 1 / 以换行结尾，写后三格不变。
- ⑩ **两把 Backlog 尺子的八格同步**：l242 声明面 93 → 94、有标记 111 → 112；l253 权威面 67 → 68、宽面 111 → 112、同卷可查 55 → 56、字母形裸 S 92 → 91 与划掉 ~~S~~ 34 → 35、闭合词已收口 35 → 36、组合名册 (~~S~~ / 已收口 / 括号 L / 不粗体) 那一档 9 → 10（落进已有形状，没有造出第 17 种）、表行两面 258 → 259 与 212 → 213，日志块表行 46 一格未动。**没有任何一把尺子被改松**，八格全部按守卫打印的实测量钉死；宽面与 l242 的「有标记」第四次同面同数。
- ⑪ 门禁：新守卫 16 支与受影响的 privacy / audit / l227 / l228 / CLI 面连同 a184 一起跑 158 passed / 5 skipped；八把账本与文档守卫连新守卫 221 passed / 8 skipped（8 支 skip 仍全为 l79 的作者工作树条件）。全量定标门禁与账本批之后的复跑读数记在记账提交里。
- ⑫ 待拍面**净增一**（A285 出名单、A339 入名单）：A301 / A306 / A308–A311 / A313–A315 / A317–A318 / A323 / A333 / A335 / A336 / A338 / A339。A285 的定级格收成划掉字母加轮号，A339 留裸字母待拍。
⒔ **八把账本与文档守卫在账本批前后同读数**：批前 205 passed / 8 skipped，批后复跑仍 205 passed / 8 skipped（8 支 skip 全部为 `test_doc_line_refs_l79.py:647` 的作者工作树条件，与历史七轮同一归因）。两把 Backlog 尺子逐格按**守卫打印的实测量**重钉，没有任何一把尺子被改松：l242 三格 212 → **213**、93 → **94**、111 → **112**（不过滤那一面由 `MEASURED_DECLARED + MEASURED_PHANTOM_LOG_ROSTER` 的等式自己从 131 走到 132，没有另钉）；l253 六格 67 → **68**、111 → **112**、55 → **56**、裸 S **净 0**（A285 离格、A339 入格，与 L254 那一档同一形状）、划掉 ~~S~~ 34 → **35**、闭合词已收口 35 → **36**、组合名册 `(~~S~~ / 已收口 / 括号 L / 不粗体` 那一档 9 → **10**（落进已有形状，没有造出第 17 种，由组合名册的精确相等当场证明）。表行三面 259 / 213 / 46，日志块那一面一格未动。宽面与 l242 的「有标记」**第四次**同面同数。新守卫 `test_default_text_fields_l257.py` 单跑 16 passed。
⒕ **全量定标门禁（逐字 `python -m pytest tests -q -p no:cacheprovider`）终态读数**：9114 passed, 10 skipped, 6 warnings in 294.69s (0:04:54)。覆盖率行逐字 `TOTAL                               14361      8   3962     22    99%`。**第一轮全量是 4 红 / 9110 绿 / 10 skip（366.12s），如实记下不美化**：四支全部是 parametrize 普查尺子（`test_parametrize_blind_face_l98.py` 的 `test_census_agrees_with_the_l97_ruler`、`test_tier_counts_are_pinned`、`test_the_dynamic_face_sees_every_blind_site_by_name` 与 `test_parametrize_ids_l97.py` 的 `test_census_totals_match_this_rounds_measurement`），红在**新守卫自己带进去的两支 parametrize**——一支值位引用模块级常数 `CLI_TRIO_SUBCOMMANDS`（落盲面 plain 档），一支是内联字面清单（落可读档）。归因是普查面而非产品缺陷：产品代码一处未坏，覆盖率 TOTAL 语句 14360 → **14361**（唯一净新语句是 `privacy.DEFAULT_TEXT_FIELDS` 那一行模块级赋值，import 即覆盖）、未盖 8、分支 3962、半覆盖 22 三格一字未动，两处默认面改写都是**同行改写**所以语句与分支数不变。处置按普查尺子的既有口径逐格重钉现量（l97 三格 CALLS 432 → **434**、READABLE 314 → **315**、BLIND 118 → **119**；l98 三格 plain 58 → **59**、盲面用例名册 113 → **114**、动态面真看到 113 → **114**），两把尺子之间独立咬合，没有把任何一把改松。**第二轮仍差一支**（1 failed / 9113 passed / 10 skip，294.71s）：红的正是「动态面真看到的盲面数」那一格（`assert 114 == 113`）——这一格的绿必须是「真采到」而不是「没采到」，所以按实测量补钉而不是把判据放宽。补钉后第三轮才是下面这份终态。落盘形状：本账追加这两条后共 4594 行（尾部追加、零位移），纯 LF（crlf 0 / 裸 CR 1 / 以换行结尾），本账第 810 行仍写着 `docs/augmentor/API.md:1139` 那一处活引用、未随本轮平移。前端三门禁**不适用**（本轮零 web/ 改动）。

### L258（2026-10-13）— 名字被当成路径片段用那一族装上一把 SDK 权威：require_within_dir 十处共引 + 门口按类型放行 400

- ① **本轮的问句**：目标线问的是「还有哪些漏洞」，而这一族的四条现量此前**一条都没入过账**——不在 Backlog、不在清单行、不在任何一轮日志里。它们共同的形状是一句话：调用方给的**名字**（版本 ID、任务 ID、数据集名、分片前缀）被当成**路径片段**拼进 `base / f"{name}..."`，而 `pathlib` 对 `..` 与绝对路径是照字面拼的，不报错也不解释。
- ② **改前四条实测**（仓外探针 `Temp/l258_probe.py` 与 `l258_probe2.py`，沙箱里跑，最重的三条复现到磁盘）：`VersionManager(storage_dir=<tmp>/versions).delete_version("../victim/important")` 返回 `None`（成功）而版本根**之外**那个整目录被 `shutil.rmtree` 掉了；同一只管理器的 `load_version("../secrets")` 交出目录外的 `data.json` 内容；`CheckpointManager.create_checkpoint("../../escape", 3)` 把断点文件写到 `checkpoint_dir` 之外（`outside_exists=True`、目录内为空）；`DiskCache(cache_dir=<数据目录>).clear()` 把目录里**别人的** `*.json` 一起 unlink（用户数据集 `my_dataset.json` 在 clear 之后不存在）。同一条形状另两处只在 SDK/CLI 面：`Exporter.export_batch` 的数据集名称与 `DatasetOperations.split_file` 的分片前缀（`{"../../evilsite/x": items}` 实测把产物写到输出目录之外，返回路径里赫然带着两点斜杠）。
- ③ **HTTP 面改前没有闸**：`GET /api/versions/%2e%2e`（无鉴权）与 `/api/versions/%2e%2e/data` 在改前拿到的是 **500**。这一格要说准：500 **不是**「拦下了」，而是「路径被原样拼出去、恰好那一格没有文件」——写侧（`POST`、删除）同一条路径就是②里那两条磁盘事实。本轮之后同一族越界请求拿 **400**、正文是 SDK 那句判决，而 `v_ghost` 那一档**照旧 500** 并单独一支钉住：它钉的是「本轮没有顺手把状态码契约改掉」，不是「这一格对了」。
- ④ **权威只一把，住 `augmentor/validation.py`**：`require_within_dir(label, value, base)` 是该文件**尾部追加 41 行**、新函数起于第 986 行。不建新模块（A184 那把注释引用尺子会把「指向仓内并不存在的文件」记成 dead_path），也不插在 `validate_dataset` 之前的任何活引用之上——`:217-222 / :437-442 / :639` 三处是本账与守卫的落点，在它们之上插行会把落点平移（A337 / A338 那一族的形状），而 a184 的结构维只问「落点还是不是代码」，平移后大概率**不红**，这正是本轮不肯留的债。
- ⑤ **空值那一维不重抄**：`value = require_string(label, value)`。自己写 `not isinstance(value, str)` 会把整数 `5` 判成「不能为空」——那是句假话，而假判决会顺着门口那支 400 原样回给客户端。注入臂 **A8b**（不委托、自己写「不能为空」那一形）实测 **1 红**，说明「委托同族」这件事本身在这把尺子下有牙齿；两个削弱臂（`_weaken_keep_resolve` / `_weaken_only_empty`）也改成先走 `require_string`，因为一条削弱臂不该同时改动两维判决——那是把两次变更混成一个读数。
- ⑥ **判决文案不含解析后的绝对落点**：`{label} 不能指向 {base.name} 目录之外` 只带名字与目录标签，`test_the_verdict_does_not_leak_the_resolved_path` 另加一条正则 `[A-Za-z]:[\\/]` 当场盯着盘符形状。这一格与 `INTERNAL_ERROR_DETAIL`（A155①）同族：400 档回原文是**参数判决**该回原文，500 档回固定文案是**部署细节**不该回，两条边界共用同一个「不把解析结果塞进响应」的判据。
- ⑦ **同一条口径的两个尺度**：`api/deps.py` 的 `resolve_within_roots` 管「HTTP 传入的路径只能落在数据白名单之内」，这一把管「一个名字不许逃出它自己的目录」。SDK 面必须自带，因为 CLI 与直接调 SDK 的调用方**不经过** HTTP 那道闸——这正是 L231 以来反复撞到的「闸装在门口、门里没人管」。
- ⑧ **调用点名册与「恰好两处实现」**：AST 现量十处 / 五个文件（`checkpoint.py` 3、`version_control.py` 3、`export.py` 2、`dataset_ops.py` 1、`versioning.py` 1），守卫写成「文件 → 次数」的字典相等而不是总数，这样把一个调用点从 A 文件挪到 B 文件也会红。包含判据的 `.parents` 比较在 SDK 包内普查**恰好两处实现**（本把权威 + L231 给备份名落下的那条手判）——第三份手抄就是 L238 / L256 / L257 收掉的那一族，所以这一格写成名册精确相等而不是「不超过两处」。
- ⑨ **门口按类型放行而不是按父类放行**：新增 `api/deps.py` 的 `raise_boundary_error`（净 20/1 行）——`DataValidationError` ⇒ 400 且回 SDK 原话，其余交回 `raise_internal_error`。不按「是不是 ValueError」放行，因为 `VersionError` 同为 `ValueError` 子类，一刀切会把它们一起改成 400 ⇒ 动对外状态码契约（**A280** 仍待拍），而 `tests/integration/test_api.py:479` 正钉着那一格的 500。`api/routes/version.py` 九处调用点同行改写、净 **9/9 = 0** 行；改后全仓收尾现量余 **29 处** `raise_internal_error`（audit 1 / augment 3 / config 3 / data 5 / export 4 / leakage 1 / multimodal 2 / privacy 1 / quality 9）——这一格记在账上是为了让 A280 那一轮拿到基数，不是为了本轮扩大处置面。
- ⑩ **缓存面是同一族的另一格**：`DiskCache.clear()` 改前把 `cache_dir` 里所有 `*.json` 一律 unlink，包括**不是它记账的**文件（用户数据集就死在这里）。改后按 `owned = {md5(key) for key in self._metadata}` 只删自己的条目，守卫同时要求清完之后 `stats` 的 `entries` 与 `total_size_bytes` 归零（不是「没删所以没少」）、外来件与 `_metadata.json` 都活着。
- ⑪ **两处按实测推翻想象，如实记下**：`DatasetVersionManager.delete_version("../leak")` 不 raise 而是回 `False`——版本未登记时它先短路返回，那一处 `require_within_dir` 在这条路径上是**纵深防御**而不是当前可达面，所以判据按 `is False` 写而不是 `pytest.raises`；`CheckpointManager` 的 `task_id=".."` 被 `f"{task_id}_checkpoint.json"` 拼成一个**文件名字面量**（`.._checkpoint.json`），落点仍在根内 ⇒ **接受**，并单独一支 `test_a_leading_double_dot_as_a_name_prefix_is_inert_and_stays_inside` 钉住它，防后来人改成「凡带点就拦」。这两条都是新守卫首跑 2 红逼出来的——**红的是我的想象，不是尺子也不是产品**。
- ⑫ **跨平台与能力探测**：拒绝清单按 `os.name` 分臂——`"C:\abs"` 在 POSIX 只是一个合法文件名、`"..\x"` 在 POSIX 不是分隔符，把两族并进同一张清单会在别的平台上造出假红（本轮撤掉了反斜杠形状那一臂）。符号链接那一臂先探测能力（POSIX `symlink` / Windows `mklink /J`），造得出才判；本机实测造得出 ⇒ **未 skip**，这一格不是靠 skip 蒙过去的绿。
- ⑬ **并行支路吞判决 ⇒ 新立 A340（只立账）**：`export_batch` 默认并行，其 `except Exception: logger.error(...)` 咽下 `DataValidationError`，同一输入串行 raise、并行回空 dict（实测 `results == {"../../silent/x": {}}`、目录外零产物），而 `POST /api/export/batch` 走的正是并行档 ⇒ 被拦下的越界名称照样回 `success: true`。三处拒绝臂因此显式 `use_parallel=False`，另加一支把并行这一形**钉成已知限制**。本轮不改：改它要同时改 `export_batch` / `export_all_formats` 的返回与抛出语义（对外契约），且修好后该路由失败会打到 500 那一档，必须与 **A280** 一并拍。`/api/dataset/split` 同一条判决经 `to_http_error` 已是 400，无需入账。
- ⑭ **十二臂注入取证逐臂读数**（零写入通道：在导入后的模块对象上打补丁）：A1 摘掉权威的一处调用点 **3 红**、A2 把门口收成裸拼接 **2 红**、A3 把版本面退回手抄 **3 红**、A4 缓存面退回「全清」 **1 红**、A5 断点面摘掉一处 **1 红**、A6 门口把 400 收回 500 **3 红**、A7 拒绝清单收成只判 `..` 头 **1 红**、A7b 同臂的空值维同步削弱 **0 红**、A8 判决里带上绝对落点 **2 红**、A8b 空值维不委托 `require_string` **1 红**、A0 与 A9 两个基线臂 **0 红**。两处预期被实测推翻并改的是预期：A6 我预测 4 红、实测 3 红，现场理由是固定 500 文案 `INTERNAL_ERROR_DETAIL` 同样不含部署路径，那一支**按设计不响**；A7b 在委托之后由 0 红变 1 红，说明削弱臂原本混入了两维改动。全程跑完复验磁盘：被触达的文件逐臂与进入时字节相等，收尾重新 `git status` 验盘。
- ⑮ **两把普查棘轮按红读数重钉六格**：l97 三格 CALLS 434 → **436**、READABLE 315 → **316**、BLIND 119 → **120**；l98 三格 plain 59 → **60**、盲面用例名册 114 → **115**、动态面真看到 114 → **115**（exempt 24 / opaque 11 / derived 38 / ids-derived 11 四档一格未动）。推动这一格的是新守卫自己带进的两支 parametrize：值位引用模块级常数字典 `sorted(MEASURED_CALL_SITES)` 的那支落**盲面 plain 档**，`("method,url", [...])` 内联字面清单那支落**可读档**。读数出自仓外探针 `Temp/l258_census.py` 直接 `import` 本模块 `param_sites(TESTS_ROOT)` 现量，**不是按「字面量 ⇒ 可读」推的**（L204 与 L257 各这么想过当然一回）。定向复跑这两把尺子会抛 `RulerNotProven`（采集下限的结构性 error，不是失败），所以读数必须来自全量跑。
- ⑯ **卷一 A144 三处活引用按现量重钉**：`checkpoint.py` 本轮净 +1 行（4/3），于是卷一第 292 行那三处 `checkpoint.py:NNN` 各漂一格 ⇒ `219 → 220`、`263 → 264`、`334 → 335`，同行改写、`git diff --numstat` 读 **1/1 净零**。这是 skill 第 5 节那条「插行会平移历史行号引用」的又一次实证，处置走处方第三条（重钉）是因为前两条在本轮都不通：追加点在文件尾部而在账本引用之下、而 +1 行是产品代码的必要行数，压不回净零。
- ⑰ **净行数与落盘形状**：产品侧 `git show --numstat db2fd4838` —— `validation.py 41/0`、`api/deps.py 20/1`、`api/routes/version.py 9/9`、`cache.py 3/2`、`checkpoint.py 4/3`、`dataset_ops.py 2/2`、`export.py 3/2`、`version_control.py 4/3`、`versioning.py 2/2`、新守卫 `test_name_path_containment_l258.py 473/0`、l97 与 l98 各 3/3、卷一账 1/1。本账这一批净 **+26**（清单行 +1、新立 A340 连带它的分隔空行 +2、日志块 +23 = 前置空行 + 标题 + 标题后空行 + 20 支 bullet，其中 ⑳ 是账本批落盘之后补记的首跑读数），其中只有前两处落在第 813 行之前 ⇒ L232 那句 operative 自指按现量 810 → **813**、链尾补第十五个值，落点行以 `sed -n '813p'` 复核仍含 `docs/augmentor/API.md:1139` 那一处活引用。写前读数 crlf **0** / 裸 CR **1** / 以换行结尾，写后三格不变（卷二不是纯 LF 的那一个 CR 是散文内容，批写的写回断言只能是「不含 CRLF」，判 `count(b"\r") == 0` 会把自己判死）。
- ⑱ **门禁读数**：新守卫定向 **36 passed / 0 failed**（九类全跑）。账本批之前的全量定标逐字 `python -m pytest tests -q -p no:cacheprovider`：第一轮 **4 failed**（四支全在 l97 / l98 两把普查尺子的聚合断言上，归因是普查面而非产品缺陷，产品逻辑一处未坏），按 ⑮ 重钉六格后终态 **9150 passed / 0 failed / 10 skipped、99.84 %、333.24 s**。用例闭合 = L257 终态 9114 + 新守卫 36 = **9150**，**恰等** ⇒ 没有一支被删或被改名绕开；10 支 skip 仍是既有那十条环境条件（l79 作者工作树 8 + l82 export 节 1 + l87 无真实数据集 1），本轮未新增任何跳过理由。覆盖率 TOTAL 四格与账本批之后的复跑读数记在记账批里，本块不预写。前端三门禁**不适用**（本轮零 `web/` 改动，不是漏跑）。
- ⑲ **两把 Backlog 尺子的账面预测与处置**：本轮**零收口**（改的是从未入账的一族）且**新立一行**（A340，定级格是裸字母、整行不带闭合词）⇒ 预期推动的只有表行那一面：l242 `MEASURED_ROWS` 213 → **214**、l253 `MEASURED_ALL_ROWS` 259 → **260** 与 `MEASURED_BACKLOG_ROWS` 213 → **214**、字母形裸 S 92 → **93**；声明面 94、有标记 112、定级格闭合标记 68、宽面 112、同卷可查 56、闭合词五档、组合名册 16 档、phantom 84 与日志名册 38 与日志块表行 46 全部应当一格未动。**A340 在清单行与日志里一律写成加粗或括号形状**（口径 5），绝不让它带着闭合词进声明面——否则 `test_gap_equals_partial_roster` 会当场多出一格假欠账。逐格以守卫打印的实测量为准，预测被推翻就改预测、不改尺子；A338 那 44 行两格差名册也不该被推动（A340 两面都不带标记）。
- ⑳ **账本批首跑 8 红，其中四红是本案自己造的假声明**（中间读数留着不美化）：定向跑两把 Backlog 尺子得 **8 failed / 43 passed**。四红是本轮预期的账面移动——l242 表行 `assert 214 == 213`、l253 全表行 `assert 260 == 259`、l253 的 Backlog 区同面同数一格、字母形裸 S 的 `{'S': 93} != {'S': 92}`——四格按红读数原样重钉，**没有一把尺子被改松**。**另四红同一根因**：清单行末句我写成 `本轮零收口`，紧跟在整串待拍名册之后，而 `AFTER` 那一支的窗口（名字之后 ≤20 个字符内出现闭合动词即算这一格被声明收掉）里**能打断的只有 `（` `(` `|` `*` 反引号与 ASCII 字母数字，`）` 与任何汉字都打断不了** ⇒ 名册右括号后面那一路全是合法填充，17 只 ID（区间按自然位数展开）一次进声明面：声明面 94 → **108**、gap 从 `PARTIAL` 三格涨到 **十七格**，连带 `test_declaration_face_is_populated`、`test_the_gap_is_the_same_on_both_faces`、`test_the_unfiltered_face_still_pairs_with_the_filtered_one` 与 `test_gap_equals_partial_roster` 四支齐响。**处置是同行改写那句散文**（净 0 行，把那个带动词的短语换成不带闭合动词的说法），94 与 112 两格一字未动 ⇒ **红的是案面而不是尺子**，把 94 重钉成 108 才是本项目历史上真实犯过的那类错。这是「口径 5 粘字」那一格在本账的第三次撞到（L254 与 L255 各一次，两次都由同一支 `test_gap_equals_partial_roster` 抓出），本轮把归因写进 l242 的 `MEASURED_ROWS` 注释里，让后来人改清单行末句时先查到这一条。改写之后再用守卫自己的 `BEFORE` 与 `AFTER` 两支直接扫本轮落进声明面的那两行（L232 自指行与本批清单行），命中 **1** 支、且那一支是 L232 行里既有的 A305 历史声明（「本轮已修」的形状），与本轮无关。
- ㉑ **账本批（86225b5b8）落盘之后的复跑记账**：八把账本与文档守卫（l242 / l253 / markdown 结构 / 哈希槽与 git 真值对账 / 进度一致性 / a184 硬 0 档 / l79 文档行号 / l200 契约数字）复跑 **205 passed / 8 skipped**，与 L257 批前批后两轮**逐字同读数**（8 支 skip 仍是 `test_doc_line_refs_l79.py:647` 的作者工作树条件，本轮未新增跳过理由）。全量定标门禁逐字 `python -m pytest tests -q -p no:cacheprovider` ⇒ **9150 passed / 0 failed / 10 skipped / 6 warnings / 352.77 s**，覆盖率行逐字 `TOTAL 14379 8 3968 22 99%`、Total coverage **99.84%**。四格与 L257 终态 `14361 8 3962 22` 的差各有账：语句 **+18** 与分支 **+6** 全部落在本轮三处净新代码上（`validation.require_within_dir` 的两次 resolve 加三条判决、`deps.raise_boundary_error` 的两支、`cache.DiskCache.clear` 的 owned 集合与那圈循环），未覆盖 **8** 与半覆盖 **22** 两格**一字未动** ⇒ 新代码没留死支，也没有把旧死支藏起来。用例数 **9150 与账本批之前那一轮一字不差**：本批只动两份常量与账本自身，未增删任何用例，也没有改名绕过。352.77 s 对账本批之前同一条命令的 333.24 s 按 L239 / L240 量过的负载差档算噪声，**不作结论**。前端三门禁**不适用**（本批只动账本与两份测试常量，`web/` 零改动，不是漏跑）。落盘形状：本账至此共 **4620** 行（本支追加在文件末尾、零位移），CRLF **0** / 裸 CR **1** / 以换行结尾，第 **813** 行仍写着 `docs/augmentor/API.md:1139` 那一处活引用。

### L259（2026-10-13）— 「名字拼进路径」一族剩下的七处落点接进 L258 那把权威，闭族本身立成判据

- ① **本轮动的是哪七处**：`augmentor/vector/faiss.py` 的 `persist()` 两处（`{collection}_vectors.npy`、`{collection}_meta.json`）与 `load()` 两处、`augmentor/tracker.py::_get_experiment_path`、`augmentor/pipeline.py::export_dataset`、`augmentor/visualizer.py::_output_path`。权威函数一字未动——L258 立的 `validation.require_within_dir(label, value, base)` 是本案唯一的判决来源，本轮七处全部共引，零手抄。
- ② **改前三格实测复现（沙箱，仓外 cwd）**：`FAISSDB(dimension=4, collection="../escaped", storage_dir=<db>)` 加向量后 `persist()` 在 `storage_dir` 之外落盘；`AugmentationTracker` 的实验文件名同样能逃出 `storage_dir`；`Pipeline.export_dataset(..., ["../../evilfmt"])` 这一格最有意思——改前挡住它的不是拼接那一行，而是 `ExportFormat(name)` 的枚举强制（13 个成员值无一含 `/`、`\`、`..`），**枚举在强制冒充闸**，所以「这一处本来安全」的说法不成立：新增一只带分隔符的成员就当场破。守卫里 `test_the_enum_values_carry_no_separator` 把那句「本来安全」钉成会过期的断言，改形当场红。
- ③ **判据一律排在 `mkdir` 之前**：`persist()` 与 `_output_path()` 两处把 `require_within_dir` 写在建目录之前，坏名字不该在盘上留下一个空目录——与 L45 那一条纪律同形，由 `test_a_bad_collection_creates_no_directory` 与 `test_a_bad_file_name_is_rejected_and_builds_nothing` 分别点名。
- ④ **`visualizer.py` 走函数内延迟导入**：卷一 A137 按 `augmentor/visualizer.py:27` 点名，顶层加一行就把它平移出跨度 ⇒ 那 9 行新增全落在函数体内（76-83 之间），27 行原地不动。这是 skill 第 5 节「插行平移历史引用」的第三次撞上，本轮以量过再动手避开。
- ⑤ **`pipeline.py` 净零行（2/2）**：该文件有四处活引用（`:115`、`:125`、`:158-193`、`:557`），所以走「第 22 行 import 同行扩写 + `export_dataset` 内单行赋值」，`git show --numstat` 复核净行数 0。
- ⑥ **新守卫 41 支、八组**：`TestTheFamilyCensusIsClosed`（4 支 + 按 9 个模块参数化那一支 = 13）、`TestTheVectorFace` 5、`TestTheTrackerFace` 3、`TestTheVisualizerFace` 4、`TestThePipelineExportFace` 4、`TestTheConfigSurfaceStaysOpen` 3、`TestTheHttpBoundary` 4、`TestTheRulerHasTeeth` 5。定向 `--no-cov` 复跑 **41 passed**；L258 那把守卫同批复跑 **36 passed**（它的名册按文件 curated，不是 SDK 全量 ⇒ 本轮共引不推它）；相关既有测试 **294 passed**。
- ⑦ **闭族普查三格现量**：权威调用点 17 处 / 9 个文件（`checkpoint.py` 3、`dataset_ops.py` 1、`export.py` 2、`pipeline.py` 1、`tracker.py` 1、`vector/faiss.py` 4、`version_control.py` 3、`versioning.py` 1、`visualizer.py` 1）；不走权威的「目录 / 名字」拼接只剩 4 处并逐处给出处置理由（`backup.py:117` 是 L231 的手判、与权威同口径，**L273 起改为共引 `validation.path_lands_within` ⇒ 本行读数是 L259 当轮形状，现量已由 4 处收成 3 处、包含判据实现数由 2 收成 1，见 L273 块 ⑥**；`cache.py:268` 的键是 md5 十六进制摘要、形状上不含分隔符 ⇒ 惰性（L280 重量：原写 244，`_load_metadata` 由 `exists()` 后 `open()` 换成直接开文件 + `except FileNotFoundError` 净 +6 行，全落在拼接之前；a184 硬 0 档当场把它点名为 `dead_line`，按现量重钉，判据一格未松）；`validation.py:1021` 是权威自己判完才拼那一行；`visualizer.py:83` 是上一行刚判过的同一个 `output_file`）；`.parents` 那一档的包含判据实现数仍是 2 处（`backup.py`、`validation.py`），本轮新增七处全走共引。
- ⑧ **浅判据那一格机械事实**：未规范化的 `output_dir / "../../evil/plot.png"` 对 `output_dir` 的 `is_relative_to` 判 **True**，而真实落点在根外 ⇒ 包含判据必须在解析之后做。这一格既是权威 docstring 的理由，也是 **A342** 的根。
- ⑨ **HTTP 门口本轮零新形状**：`_output_path` 只借那句判决、返回值保持 `output_dir / 名字` 原形状，`GET /api/visualize/{filename}` 的回执一字未动；`vector-status` / `semantic-search` 那两只 400 早在 L258 已在册。守卫里 `test_the_export_route_still_answers_500_for_a_boundary_verdict` 与 `test_the_export_route_legitimate_arm_still_writes_inside` 把「本轮没改响应面」钉成对照两臂。
- ⑩ **五臂注入取证（零写入通道：在导入后的模块对象上打补丁）**：摘掉 `persist` 的闸 ⇒ 写手收到根外落点、**红**；摘掉 `load` 的闸 ⇒ 读到根外文件、**红**；摘掉 tracker 的闸 ⇒ 实验文件落根外、**红**；摘掉 visualizer 的闸 ⇒ 交回裸拼接、**红**；摘掉 pipeline 的闸 ⇒ 把根外路径交给写手、**红**。判据形状统一为「闸在位时写手收不到路径 / 摘闸后收得到」，不靠标题不靠散文。
- ⑪ **A341 的取证按 skill 第 9 节四格纪律走沙箱副本**：`AUGMENTOR_CONFIG_PATH` 与 `AUGMENTOR_DATA_ROOTS` 都指到仓外临时副本，进沙箱前先记仓库 `config.yaml` 的 sha256、探针后复核**未变**（沙箱副本被整份重序列化成 186 行，正是 A152① 描述那一格，所以绝不打在服务自己那份配置上）。读数：`status: 200`、`ignored_keys: []`、存进去的 `storage_dir` 就是 `"../../outside_vectors"`。可写名单里的目录键现量只有这一格——`versioning.storage_dir` 与 `web.static_dir` 都不在 `WRITABLE_SECTIONS`。
- ⑫ **中间读数留着不美化（新守卫首稿的三处自造缺陷）**：首稿里一个 `stub_pipeline` fixture 写了非法语法（`from api.deps import _reset_pipeline_for_tests if False else None`），一个注入臂绕远且逻辑不成立（伪造枚举类型），当场重写为静态 helper 与「摘闸前后写手收什么」的判据；docstring 先写「判据九组」而实际八组，SPDX 先写 `AGPL-3.0-only` 而房规是 `AGPL-3.0-or-later`，两处按行尾安全的字节级脚本改回、行数不变。探针首跑另有两次接口形状错：`FAISSDB.add_vectors` 的真签名是 `(vectors, metadata, ids=None)` 而不是 `metadatas=`；`WRITABLE_SECTIONS` 不住在 `augmentor.config` 而是 `api/routes/config.py:101`。
- ⑬ **两把普查棘轮按红读数重钉六格**：l97 `MEASURED_CALLS` 436 → **437**、`MEASURED_BLIND` 120 → **121**、`MEASURED_READABLE` 316 一字未动（`316 + 121 == 437` 自闭合）；l98 plain 档 60 → **61**、盲面用例名册 115 → **116**、动态面真采到 115 → **116**。涨一格的原因是本轮那支按 9 个模块参数化的用例值位引用模块级常数字典、无 `ids` ⇒ 进 CALLS 与 BLIND 两面。**没有任何一把尺子被改松**。
- ⑭ **门禁读数链**：账本批之前的全量定标逐字 `python -m pytest tests -q -p no:cacheprovider` 第一轮 **4 failed / 9187 passed / 10 skipped / 351.01s**，四红全部落在 ⑬ 那六格；`--no-cov` 全量复跑取同一批红读数（232.54s，wall 差按负载噪声不作结论）。重钉之后八把账本与文档守卫复跑 **205 passed / 8 skipped**，与 L257 / L258 逐字同读数（8 支 skip 仍是 `test_doc_line_refs_l79.py:647` 那条作者工作树条件）。覆盖率 TOTAL **14384 8 3968 22**、99.84%，对 L258 终态 14379 是语句 +5、未覆盖 8 与半覆盖 22 两格未动 ⇒ 本轮新增行 100% 被覆盖。
- ⑮ **定向复跑 l98 会造出 5 支 `RulerNotProven` error（下限 3000 条用例）**：已知工件，动态面要全量采集才有数，不据定向读数改尺子——本案的红读数与绿读数都取自全量那一跑。
- ⑯ **账本批落盘形状**：本账 4620 → **4644** 行（+24：清单行 +1、Backlog 两行连带分隔空行 +4、日志块自身 +19；前那 5 行会把下方历史行号平移，日志块在尾部零位移），纯 LF 保持（crlf 0 / 裸 CR 1 / 以换行结尾）。清单区与 Backlog 区的插行会把下方历史散文行号平移——本账第 813 行那条 `docs/augmentor/API.md:1139` 引用按现量推到第 **818** 行，L232 清单行那句自指同行改写。**前端三门禁不适用**（本轮 web 零改动）。
- ⑰ **账本批（9bc9145cf）落盘之后的复跑记账，并量到一格定标读数抖动**：两把 Backlog 尺子首跑 **4 failed / 47 passed**，红读数分别是 `assert 216 == 214`、`assert 262 == (214 + 46)`、`assert 262 == 260`、`MEASURED_LETTERS` 的 `{'M': 56} != {'M': 55}` 与 `{'S': 94} != {'S': 93}`；五格按红读数重钉（**没有任何一把尺子被改松**），两把复跑 **51 passed**，八把账本与文档守卫 **205 passed / 8 skipped** 与 L257 / L258 逐字同读数。全量定标逐字同一条命令连跑两轮：第一轮 **9191 passed / 10 skipped / 313.33s** 而 TOTAL **14384 10 3968 23**（99.82%），第二轮 **9191 passed / 10 skipped / 291.48s** 而 TOTAL 回到 **14384 8 3968 22**（99.84%，与账本批之前那一跑逐格相同）⇒ 上一记 ⑭ 的四红转绿已证。而那 **+2 格缺数 / +1 偏支**不是本轮改出来的：两轮 `coverage.xml` 逐档对账，翻动的只有 `augmentor/quality_trend.py:66-67`——那是 `_quarantine_corrupt_file` 里 `while backup.exists():` 的循环体，只有**同一秒内发生第二次隔离**才走得到（备份名带 `%Y%m%d%H%M%S` 时间戳），既有 `tests/unit/test_quality_trend.py` 的三支只测首档命名。⇒ 全量门禁的「缺数」与「偏支」两格按墙上时钟抖动，两轮之间不可直接比数，这一形此前从未入账。**处置**：L260 补一支确定性用例把这一档钉住（冻结 `quality_trend.datetime` 的 `now()` 并预置同名 `.corrupt-<stamp>` 文件，断言第二次隔离落到 `-1` 后缀那一格），纯测试面、零产品码改动。

### L260（2026-10-13）— 定标读数抖动那一格补上确定性用例：损坏备份防撞的同秒第二次与第三次隔离从此必到

- ① **选题来历是上一轮的记账，不是新拍的**：L259 记账批 ⑰ 记录「同一条定标命令连跑两轮 TOTAL 不一致」，本轮先把那两格的成因查清再动手——读数不可比会让后面每一轮的棘轮与覆盖率对账都失去基准。
- ② **成因（机械事实）**：`_quarantine_corrupt_file` 的备份名是 `trend.json.corrupt-%Y%m%d%H%M%S`（秒粒度），撞名时才走 `while backup.exists():` 的循环体加 `-1`、`-2` 后缀。既有三支用例里两支背靠背跑，是否落在同一秒由调度决定 ⇒ 改前全量读数：一轮 `quality_trend.py 98 2 28 1`（缺 66-67）、另一轮 `98 0 28 0`。这两行不是本轮新写的产品码，L146 / B216 那两轮就存在，只是从来没有一支用例**保证**走到它。
- ③ **修法**：两支新用例把 `augmentor.quality_trend.datetime` 冻成固定一秒（模块是 `from datetime import datetime` 的顶层名字，patch 模块属性即可，**零产品码改动、零文件系统技巧**），预置同名当秒备份后触发一次隔离 ⇒ 断言落到 `-<stamp>-1` 且第一份取证逐字未动；第二支预置 `-<stamp>` 与 `-<stamp>-1` 两份 ⇒ 断言第三份只能落到 `-2`。
- ④ **为什么要两支**：只走一圈时 `while` 与 `if` 同形（都只让一格），单靠第一支没有牙；第二支预置两份取证才把「一直让到位」与「让一格」区分开——`if` 形会撞上已有 `-1`，第二支当场红。
- ⑤ **注入取证两臂（字节级改写、finally 复原、复原后逐字节比 sha）**：M1 `while backup.exists():` → `if …` ⇒ **1 failed / 4 passed**；M2 → `while False:` ⇒ **2 failed / 3 passed**（两支新用例全响，符合「M2 连第一档防撞都摘掉」的预期）。基线与复原后各 **5 passed**，复原后文件 sha `51ca546db…`，`git status` 复验该文件已回到进入前状态。
- ⑥ **隔离证明（这一格从此不靠邻居）**：单跑 `TestCorruptHistoryQuarantine`（5 passed）后读 `coverage.xml`，`quality_trend.py` 的缺数名单里已无 65-67 ⇒ 该档由本类自己保证，与套件里其他用例是否同秒、以什么顺序跑无关。
- ⑦ **全量定标读数**：逐字 `python -m pytest tests -q -p no:cacheprovider` ⇒ **9193 passed / 10 skipped / 342.05s**，TOTAL **14384 8 3968 22**、99.84%；`augmentor/quality_trend.py` 由 98 语句缺 2、28 偏支 1（98%）升到 **缺 0、偏支 0（100%）**。10 支 skip 与前几轮逐字同名单（8 支作者工作树 + `test_config_gates_l82.py:573` + `test_upload_ceiling_l87.py:580`）。
- ⑧ **账面零推动**：本轮不收 Backlog 也不新立（问题在 L259 记账里查清、在本轮直接补上用例），所以 l242 `MEASURED_ROWS` 216、l253 `MEASURED_BACKLOG_ROWS` 216 / `MEASURED_ALL_ROWS` 262 / `MEASURED_LETTERS` S 94、M 56 六格一格未动；新两支不带 `parametrize` ⇒ l97 / l98 的普查棘轮也不动。八把账本与文档守卫的读数见本轮记账支（⑪）。等拍板的面（A280、A341、A342、A340 一族）本轮一个都没碰。
- ⑨ **前端三门禁不适用**（`web/` 零改动）。
- ⑩ **账本批落盘形状**：本账 4645 → **4659** 行（清单区 +1、日志块 +13 = 前置空行 1 + 标题 1 + 空行 1 + 要点 10），L232 清单行那句「本账第 818 行」按现量同行改写到第 **819** 行；纯 LF 保持（crlf 0 / 裸 CR 1 / 以换行结尾）。注意本账有一处内容级裸 CR，用 `read_text()` 读会被通用换行翻译成多出一行——历次行数断言一律走 `read_bytes().decode()`。
- ⑪ **记账批（本条所在提交）读数**：账本批（`4be6d91ba`）落盘之后八把账本与文档守卫 **205 passed / 8 skipped**，与 L257 / L258 / L259 逐字同读数。全量定标在同一条命令上连跑两轮（账本批前后各一轮）读数逐格相同：**9193 passed / 10 skipped**（342.05s 与 334.96s，wall 差按负载噪声不作结论）、TOTAL **14384 8 3968 22**、`augmentor/quality_trend.py` 两轮均 **98 0 28 0（100%）** ⇒ L259 记账批 ⑰ 记下的那格抖动在本轮消掉，「缺数」与「偏支」两格重新可比；这两格是全账棘轮与覆盖率对账的基准，基准不可比时任何「+5 语句 / 未动」的说法都不成立，所以本轮把它当缺陷处理而不是当噪声记录。**未修的那一格**：本轮只补了防撞循环体那两行的确定性，`quality_trend.py` 之外仍有 8 格缺数与 22 格偏支在册（`semantic.py:94`、`versioning.py:283-284`、`data/image.py:135-136`、`routes/multimodal.py:102/113`、`routes/vector.py:150`）——这一批是 L257 / L258 / L259 三轮逐字相同的既有读数，本轮不动，也没有替它编造归因。

### L261（2026-10-13）— 收 A338：41 行的定级格按各自正文判决补齐，l242 的标记判据同批改读定级格

- ① **选题与射程**：A338 的 ①+② 同批——原格写死「② 单独做会让 l242 当场红出三十多支欠账，而那些欠账里有真的没收的，必须与 ① 同批」。零产品代码、零新守卫文件、零 Backlog 新行。
- ② **逐行核对的口径**：仓外探针三支（Temp `a338_probe.py` 出名册 44 行、`a338_dump36.py` 逐行导出正文首 420 字、`a338_quals.py` 扫限定词 `前半|后半|部分关闭|半修|一半|待办|仍在|未收|留档|后续轮|待拍|另立|余面|其余` 并给命中上下文）。判据是「正文那句闭合语说的确实是本行整行」，不是「这一行看起来收了」。
- ③ **读数订正（本格的核心）**：A337 与 A338 都把两格差写成「36 行进过声明面 + 8 行没进过（`A21` / `A201`–`A206` / `B266`）」，并把那 8 行一并归因成「只覆盖本行的一半或属于别的 id」。本轮逐行读 `A201` 至 `A206`：那六句的第一格就是本行整行的判决（`A201` 16 条缺数 + 3 条隐藏偏支弧全清、`A204` 非原生格式的守卫支（177）清零、`A205` 偏支 85 → 17 且 46 条逐支判定、`A206` 10 页各建组件测试 112 例全绿），**归因错了** ⇒ 六行本轮一并补标。真正该留裸字母的是三行而不是八行。补标 **41 行** / 留裸字母 **3 行**（`A21` 只管 ngram 那一半且本行自己写着 contains / regex 两格待办、`B2` 写着前半而剩下的一半在 A132、`B266` 写着部分关闭且其余仍在案）。
- ④ **补标的写法**：定级格 = 划掉字母 + 正文那句判决的闭合词 + 轮号（照抄正文的轮号，不写成本轮轮号 L261），不加粗体、不改列数、不动正文一个字。例：`B201` 收成 `~~S~~ 已关闭（L131）`，`A205` 保留区间形 `~~M~~ 已关闭（L114–L126）`（轮号解析式只取括号里第一个 `L\d+` ⇒ 114 在卷二声明面查得到）。
- ⑤ **跨卷那一格**：卷一 `A130` 的判决轮号 L183 在卷二，若照直写 `已关闭（L183）` 就会撞上「跨卷而不写明」那一支 ⇒ 按既有 12 行的同族写法带上限定词，写成 `~~S~~ 已关闭（第二本账 L183）`，跨卷名册 12 → **13**（`CROSS_ROSTER` 新增一格）。
- ⑥ **封闭清单长出新形状**：`A204` 的 XS 档此前从未被划掉过、`A206` 的 L 档此前只有「已收口」一种搭配 ⇒ 组合名册 16 → **18** 格（新增 `~~XS~~ / 已关闭 / 括号 L / 不粗体` 与 `~~L~~ / 已关闭 / 括号 L / 不粗体`）。这是新形状落进清单并被逐格点名，清单本身一格未松——精确相等当场证明。
- ⑦ **两把尺子的红读数**：判据批改完、账本批未落盘时首跑 **16 failed / 39 passed**，逐格为 `assert 94 == 95`（声明面，A338 第一次进）、`assert 68 == 110`（定级格那一面，同一只数字在 l242 两支与 l253 两支里各响一次）、`assert 112 == 113`（l253 的宽面，只 +1 因为那 41 行本就在宽面里）、`assert 56 == 97`（同卷可查）、`assert 132 == (95 + 38)`（未过滤面配对）；集合类断言（`PARTIAL` 三格 → 四格、`TWO_FACE_ROSTER` 44 格 → 3 格、字母形 / 闭合词 / 组合名册三张表）的红读数打印成两个集或两张 dict 的差，不是单个数字。账本批落盘之后两把尺子 **55 passed**，上面每一格的左侧读数与仓外探针 `measure()` 的最终态预测逐格相同（`assert 68 == 110` 的右侧 110 = 探针 narrow 110，`assert 94 == 95` 的右侧 95 = 探针 declared_backlog 95）。处置全部是按现量重钉常数与名册，**没有任何一把尺子的判据被改松**；`marked()` 从整行收到定级格是把尺子**收紧**，并且由注入臂「正文写一句闭合语而定级格不动 ⇒ 宽面 +1 而权威面不动」继续钉住两面之差。
- ⑧ **插行的前置量**：本轮只在清单区插 1 行（`- [x] **L261**` 落在 L260 之后），41 处定级格与 A338 行、L232 自指行号全是同行改写（净 0 行）⇒ 历史 `file:NNN` 活引用零平移。日志块 14 行落在文件末尾，高于全部活引用，`git diff --numstat` 复核两卷净增删为 卷一 `+0 / -0`、卷二 `+15 / -0`（清单行 +1 与日志块 +14，其中前置空行 1 由原文件末的空行位置承担）。
- ⑨ **门禁读数**：八把账本与文档守卫 **209 passed / 8 skipped**（与 L257 / L258 / L259 / L260 逐字同读数）；全量定标 **9193 passed / 10 skipped**，`TOTAL 14384 8 3968 22`（99.84%），与 L260 那两轮逐格相同——本轮零产品代码，覆盖率分母不动。
- ⑩ **账本批落盘形状**：本账 4660 → **4675** 行（清单区 +1、日志块 +14 = 前置空行 1 + 标题 1 + 段内空行 1 + 要点 11，前置空行由原文件末的空行承担），L232 清单行那句「本账第 819 行」按现量同行改写到第 **820** 行（本轮只有清单区那一行落在它之上）。卷一 3 处定级格同行改写，行数一格未动。
- ⑪ **记账批（本条所在提交）读数**：账本批（`40d2270b8`）落盘之后八把账本与文档守卫复跑 **209 passed / 8 skipped**，与账本批之前**逐字同读数**（8 支 skip 仍是 `test_doc_line_refs_l79.py:647` 的作者工作树条件，本轮未新增跳过理由）。全量定标 **9197 passed / 0 failed / 10 skipped / 6 warnings（338.71 s）**、`TOTAL 14384 8 3968 22`、99.84%，那一轮跑在代码批（`82ba55ab2`）与 l242 三处 docstring 措辞修订**之前**；三处只改注释文字，净 0 行 0 用例，改后两把 Backlog 尺子复跑仍 **55 passed**、八把仍 209 / 8，所以读数照用不作外推。**一处按实测如实记下**：账本批的提交主题末段被我写成 `323(echo 82ba55ab2)`——命令里用了 `$$(echo …)`，shell 把 `$$` 展开成进程号、把括号留成字面量。哈希 `40d2270b8` 与两处 diff 内容一字未错，受影响的只有主题文字；**不走 amend**，理由是共享工作树里 HEAD 随时可能被别的会话推进，改写 HEAD 有替别人重写历史的风险，所以留原样并把这一格记进账，后续提交主题一律 heredoc 或写死字符串。

### L262（2026-10-13）— 普查「显式空容器被当成没传」那一族：38 格四族名册入尺，四条 HTTP 可达路点名，立 A343 待拍

- ① **普查判据是语法形状不是文件名**：`SITE = ^\s*(?:self\.)?([A-Za-z_]\w*)\s*=\s*\1\s+or\s+(.+)$`，语料面 `*.py` 排除 `tests/`、任何 `test_*.py`、`.venv/`、`web/`、`__pycache__`。现量 **38** 格，四族分档：A 构造器属性面吞非空默认 **10**、B 方法体吞非空默认 **7**、C 默认本身是空容器的幂等档 **10**、D 默认是标量 / 对象 / 一次求值 **11**。分族的理由是「判据是形状，越界的归因会造出假缺陷」：C 族 `[] or []` 仍是 `[]`，`test_class_C_is_provably_idempotent` 逐格验默认字面量确实是空容器而不是靠人判断；D 族里三格语义上与 A / B 同族（空 ⇒ 换成非空默认），本轮**不混成一谈**。
- ② **A 族十格逐格现场实例化**：`TestEmptyIsIndistinguishableFromUnset` 对每格断言 `ctor(**{attr: []}).attr == ctor(**{attr: None}).attr` 且 `len(...) > 0`——前者证「空与未传同值」这一现状，后者证这一档确实是**吞**而不是幂等。同批一支反向对照钉住 `PiiSanitizer(fields=[]) == []`：L188 那一格改了形状之后，空清单在那里**有**独立语义。守卫钉的是现状，不是合法性。
- ③ **可达面逐条对账（全用文本 / 形状，不猜）**：HTTP 直送且无人拦四路——`/api/audit` 请求模型 `fields: Optional[List[str]] = None` 且全文无 `min_length`；`/api/leakage/check` 同形；`/api/dataset/stats` 与 `/api/dataset/search` 把空清单当位置参数喂进 D / B 族。已拦住的对照两格——`/api/dataset/evaluate` 的路由层 `if not metrics:` ⇒ 400（注释自陈「空列表是假值」），`PiiSanitizer` 的构造器层形状。CLI 一支现验：`--fields` / `--rules` 一律 `nargs="+"`。最后一支 AST 臂扫全产品码，证明**仓内没有任何调用方把空容器字面量喂给这些参数** ⇒ 缺陷面在外部输入。
- ④ **只立账不修**：三修法对外语义不同（① 内部形状改 `is not None`，四条路的响应内容变；② 门口加 `min_length=1`，空清单 422；③ 维持现状并把「空 ⇒ 全量」写成契约），① 与 ② 都是响应面 / 状态码语义变更 ⇒ 按代理自开轮的界线留给拍板，A343 定级 M 待拍。
- ⑤ **第一轮全量 5 红，全部来自 parametrize 普查面，逐格如实记下不美化**：`assert 438 == 437`（l97 调用数）与 l98 同格、`ids-derived: 12 != 11`、`MEASURED_IDS_DERIVED_SITES` 多出 `unit/test_empty_container_defaults_l262.py::test_empty_list_is_swallowed_into_the_default#0`、`assert 117 == 116`（盲面用例名册）。根因是新守卫那支 parametrize 的 `ids` 由名字现造 ⇒ 被 l98 判进盲面。**同一批定点复跑另有 5 个 `RulerNotProven` error**（定向子集只采到 64 条 < 下限 3000），那是动态面自证「这一跑没采到东西」，不是回归。
- ⑥ **处置是改新支的写法，不是放松尺子**：先试过按盲面重钉 l98（`ids-derived` 11 → 12、名册 +1 行），被我自己的净零行前置检查拦住——`MEASURED_IDS_DERIVED_SITES` 那一格是 +1 行，而卷二对 `test_parametrize_blind_face_l98.py` 靠后那支动态臂有活行号引用，插一行就平移它。于是改走更正确的一格：新守卫的 parametrize 把**值位与 `ids=` 两串都写成内联字面量** ⇒ 它既不进盲面也不进 ids-derived 档，l98 的四档（61/38/11/11）、盲面 121、盲面用例 116 三格一格未动。l97 侧只按现量重钉 `MEASURED_CALLS 437 → **438**`、`MEASURED_READABLE 316 → **317**`（`MEASURED_BLIND = 121` 未动），两处都写明 **L262 重量** 的归因。
- ⑦ **第二轮全量 1 红 → 按红读数重钉**：`assert 25 == 24`（`MEASURED_EXEMPT_SITES`）——新支每支 parametrize 都带字面 ids ⇒ 整支进「动态面放过」的豁免名册。豁免名单变长不等于放松（值位那一维仍逐格判），与 L230 / L241 那两格同口径，按红读数重钉 24 → **25**。
- ⑧ **终态全量门禁（逐字 `python -m pytest tests -q -p no:cacheprovider`）**：**9225 passed / 0 failed / 10 skipped / 6 warnings in 240.41s**，覆盖率行逐字 `TOTAL 14384 8 3968 22`、`Total coverage: 99.84%`——四格与 L261 两轮**逐字相同** ⇒ 本轮零产品代码、覆盖率分母不动。9225 = L261 的 9197 + 新守卫 28 支（`def test_` 数与采集数两侧独立闭合）。
- ⑨ **普查面的自证（五支注入臂，`TestTheRulerBites`）**：新造一个 A 形属性面站点 ⇒ 进 A 族名册；`xs or [] or [1]` ⇒ 落 B 档而不是 A；写成 `if … is not None else` ⇒ 尺子**看不见**（改过的形状不再被数，这一臂同时证明「修好了就不红」不是恒真）；`or []` ⇒ 落 C 幂等档；把 `privacy.py` 那一格退回旧写法 ⇒ A 族名册立刻多出 `augmentor/privacy.py::PiiSanitizer.__init__::fields`。全部走内存改写，零落盘。
- ⑩ **账本批落盘形状**：本账 4675 → **4692** 行（净 +17 = 清单区 +1、A343 行 +1 与它同 A342 之间的分隔空行 +1、日志块自身 +14 = 前置空行 1 + 标题 1 + 段内空行 1 + 要点 11）。**第一次批写有两处形状错，由 `test_tables_have_header_separator[OPTIMIZATION_LOOP_2.md]` 当场判红**：清单行插完多带一格空行（`## Backlog A` 之前变三连空行），A343 那一行紧贴 A342 没留分隔空行 ⇒ 两行连排被读成「一张没有表头分隔行的表」。处置是同一批内补正（撤那一格空行、把分隔空行落到 A342 与 A343 之间），净行数不变仍是 +17，自指那一格按现量复核后仍落在第 **824** 行，L232 清单行那句「本账第 820 行」按现量同行改写到第 **823 行**（本轮只有清单区 1 行与 Backlog 区 2 行落在它之上，日志块在尾部零位移）。纯 LF 保持（crlf 0 / 内容级裸 CR 1 / 以换行结尾），批写走 `read_bytes().decode()` 归一，不经通用换行翻译。**卷一一格未动** ⇒ l98 那六个哈希槽数字不动。前端三门禁**不适用**（本轮 web 零改动）。
- ⑪ **代码批两枚哈希**：`cd56b4498`（新守卫第一版，28 支）与 `1d62df0d9`（本批续：内联字面量改写 + l97 / l98 按现量重钉）。第一枚落盘时全量是 5 红，第二枚之后才是 ⑧ 那格读数——链上不留「一次提交就绿」的假形状。
- ⑫ **记账批（本条所在提交）读数**：账本批（`486c4f6ec`）落盘后八把账本与文档守卫复跑 **209 passed / 8 skipped**，与账本批之前**逐字同读数**（8 支 skip 仍是 `test_doc_line_refs_l79.py:647` 那一族的作者工作树条件）⇒ 本批的三处重钉没有放松任何一把尺子：l242 `MEASURED_ROWS` 与 l253 `MEASURED_ALL_ROWS` / `MEASURED_BACKLOG_ROWS` / `MEASURED_LETTERS["M"]` 四格全部由各自的红读数当场逼出（`assert 217 == 216`、`assert 263 == 262`、`assert 41 == 40`），不是推的。全量定标取自账本批之前的同一次落盘状态：**9225 passed / 0 failed / 10 skipped / 6 warnings in 233.92s**，`TOTAL 14384 8 3968 22`、99.84%；记账批本身只在日志块**尾部**追加要点，不动收集面也不动第 823 行之上的任何一行 ⇒ 不重跑全量，这一句按 L261 的同一口径如实标注而不是补一次读数。净行数复核：账本批 `git diff --numstat` 读 **18/1、1/1、3/3**（本账净 +17，两把尺子各自净 0）。**同场量到但不进本轮**：A274 那一行正文写着「L220 已收口」而定级格仍是裸 `S`，两把尺子的标记判据都读不到那句（`已收口` 后面接的是轮号在前而不是括号在后 ⇒ 不落 `MARK` 的形状），所以它既没进 `TWO_FACE_ROSTER` 也没进 `PARTIAL`。这一格属于 A338 那一族的残余，本轮只做 A343 立账，它留给下一轮按普查处理。

### L263（2026-10-13）— 收「轮号写在动词之前」那一形判决语的盲区：7 格候选逐格定性，2 格整行判决落进定级格，新守卫 18 支把三面钉成闭合分区

- ① **判据是字面形状，不是读后印象**：`VERB = (?<![未不没])(?:已关闭|已收口|已结案|结案|已修|清零|关闭|收口)`、`PRE = L(\d{1,3})([^。|\n]{0,12}?)(VERB)`、`POST = (VERB)([^。|（(\n]{0,12}?)L(\d{1,3})`、`REF_TAIL` = 动词之后 8 字符内紧跟另一个 ID、`SCOPE` = 半程限定词封闭表（前半 / 后半 / 一半 / 部分 / 那一半 / 那一侧 / 仍开放 / 仍在案 / 留一格 / 未关闭 / 带圈序号）。负向前瞻 `(?<![未不没])` 是必需的：账本里「未关闭」是常态描述，把它读成判决会让普查面凭空胖一圈假候选，`test_arm_a_negative_prose_shape_is_not_a_verdict` 单独钉这一维。
- ② **为什么两面都读不到**：l242 与 l253 共用同一支 `MARK`，它只认**闭合词紧贴 `（L`** 或划掉字母。`A274` 那句是「`已收口`」前面挂着轮号（`L220 已收口`）——动词后面接的是散文不是括号，不落 `MARK`；`A71` 那句是「`L46 关闭（第三种修法…）`」——动词后面确实紧贴全角括号，但括号头是「第」而不是 `L`，同样不落。两形此前**任何一把尺子都不量**，所以 L262 记账批只能把它写成「同场量到但不进本轮」的散文（见本块 ⑫）。
- ③ **三面闭合分区**：宽面（`MARK` 读得到的 l253 那面）**115** + 盲区（本守卫那面）**5** + 正文里根本没有带轮号的判决语 **97** = **217** 条 Backlog 表行。`test_the_three_faces_partition_the_table` 现场把三档加回总数，不是把三个常数抄在一起——候选行先要求整行不带 `MARK` ⇒ 与宽面**按构造互斥**。注意 97 与 l253 的 `MEASURED_SAME_VOLUME`（本轮 97 → 99）是两个不同族的数字，同名同数纯属巧合，两边各有自己的判据与红读数。
- ④ **首跑 7 格候选逐格定性**：**2 格整行判决** ⇒ 落进定级格；**3 格半程**（`A45`「L36 更新（①已修…」只收①、`A127`「L79 前半程关闭，后半仍开放」、`A136`「L81 关闭（假臂那一侧）」且同行明写留一格开放）；**2 格引用别物**（`A121`「L75 新记（关闭 `A114` 时……）」、`A143`「L84 新记（关闭 `A133` 时……）」，本行自己写着「要连着端点表一起拍」「两格都要先拍」）。半程与引用两档的豁免词逐格钉进 `MEASURED_HITS`（**换词就红**），`test_arm_dropping_a_scope_token_promotes_the_row_to_a_verdict` 再证明那个词真的承重——封闭限定词表有风险这一维我在 docstring 里写明并配了臂，而不是靠「表够长」安慰自己。
- ⑤ **两处补标各有正文证据，且都是 1/1 净零行**：`A274`（卷二第 123 行）定级格 ` S ` → ` ~~S~~ 已收口（L220） `，正文那句「**· L220 已收口**：两只字段名的请求默认改成 `Optional[str] = None` 哨兵……⇒ 见循环日志 L220」，而 L220 的日志标题在卷二第 2291 行 ⇒ **同卷可查**；`A71`（卷一第 220 行）定级格 ` S ` → ` ~~S~~ 已关闭（L46） `，正文「**L46 关闭（第三种修法：按依赖等级拆 try）**：`import requests` 挪到 `try` 外先做……冗余的第二次 `import requests` 删除」——整行判决，且那句里没有任何半程限定词。轮号取正文那句判决自己的轮号（L220 / L46），**不是本轮轮号**：本轮只是把别人的判决落格。
- ⑥ **被引用的两只 ID 实测是 phantom——这一支首跑红，是尺子对的、案面错的**：我最初写的是`test_the_reference_rows_point_at_rows_that_are_closed`（假设 `A114` / `A133` 是已标收口的表行），实测两卷 `grep '^| A114 '` 与 `'^| A133 '` **均 0 命中** ⇒ 它们早已随表行裁撤退出，只在声明面留过判决。改成 `test_the_reference_rows_point_at_ids_that_are_not_open`，断言 `targets ⊆ phantoms` 且 `targets & narrow == set()`。
- ⑦ **另一支红同样是我猜的**：`test_both_volumes_contribute_to_the_blind_face` 按「盲区该两卷都有」写，实量残余 5 格**全在卷一**、卷二 0 格 ⇒ 按实量改写成 `test_the_blind_face_sits_in_volume_one_only`，用逐卷集合钉死（`OPTIMIZATION_LOOP.md` 五只 / `OPTIMIZATION_LOOP_2.md` 空集），并留 `A274` 回退臂证明卷二那一档**可达**而不是「没写」。同理 `sorted()` 那一格踩过字典序陷阱（`A121 < A143 < A45`），断言改用集合而不是有序名单。
- ⑧ **形状② 现量 0 格是量出来的空集**：`FULL_VERDICT` 与 `post` 档都写成**精确相等**而不是上界（写成上界就允许一格回退）。注入臂 `test_arm_verb_then_round_without_brackets_is_caught` 造一句 `已收口，见 L263` ⇒ 立刻被数到；`test_the_pre_shape_is_the_whole_measured_face` 钉 `(pre, post) == (5, 0)`。
- ⑨ **棘轮全部由各自红读数当场逼出，没有任何一把尺子被改松**：l242 `MEASURED_MARKED` 110 → **112**（`DECLARED` 95 未动 ⇒ `gap = declared ∩ rows − marked` 仍精确等于 `PARTIAL` 四格）、l253 六格（闭合单元 110 → **112**、宽面 113 → **115**、两面之差仍是 **3**、同卷可查 97 → **99**、裸 `S` 70 → 68 与划掉 `~~S~~` 59 → 61、词汇「已收口」37 → 38 与「已关闭」52 → 53、组合名册两档 11 → 12 与 24 → 25）、l79 `BACKLOG_A_CLOSED` 122 → **123**（只 +1，因 `LEDGER` 只读第一本账；`BACKLOG_A_CLOSED_AT_HEAD` 87 未动）。**没有造出新的标记形状** ⇒ `TWO_FACE_ROSTER` 三格、`CROSS_ROSTER`、`PARTIAL` 四格、`MEASURED_ALL_ROWS` 263 一格未动。三把老尺子那次读数 **12 failed** 给出以上每一格，不是推的。
- ⑩ **l242 的弱标记活样本换行**：`test_mark_verb_quoted_as_prose_does_not_mark_a_row` 原本钉 `A274` 那句「关闭 `A274`」形状的散文，而本轮 `A274` 那行真收了、判决落进定级格 ⇒ 判据一字未改，活样本换成 `A136` 的「关闭（假臂那一侧）」（动词紧贴全角括号而括号头不是 `L`），三处同行改写、净 0 行。
- ⑪ **六臂自证（`TestRulerBites`）全部内存改写、零落盘**：`A274` 定级格收回裸 `S` ⇒ 重新进盲区并被分类成整行判决；`A71` 同形回退臂；摘掉 `A136` 的「那一侧」⇒ 该格从半程**提升**为整行判决；注入 `已收口，见 L263` ⇒ 形状② 那档从 0 变 1；注入「未关闭」散文 ⇒ **不**被数成判决；给 `A121` 补标 ⇒ 盲区 5 → 4。新守卫首跑 **10 failed / 8 passed**，其中 8 处是账本还没补标的预期红（盲区 7≠5、`FULL_VERDICT` 非空、两格未落格、`A121` 臂读数差 2），2 处就是 ⑥⑦ 那两支我自己写错的断言；两格落格后仍 1 红（⑥ 那一支），改完 **18 支全绿**。
- ⑫ **把上一轮落错块的那条 ⑫ 归位**：L262 的记账批把「记账批读数」那条要点插到了 **L261 块尾**（`### L262` 标题在它下面），而它讲的是 `486c4f6ec`（L262 账本批）、`A343` 立账与 823 行自指——本批把整条原文搬到 L262 块 ⑪ 之后，**一字未改**（含它里面那句「第 823 行」的历史现量，历史读数不美化也不追改）。**同场量到但没有尺子**：「日志块要点落错块」这一形 l98 只管清单区的哈希槽，扫不到日志块内部 ⇒ 若要立账需按「要点里点名的 hash 必须出现在同块所属轮号的清单行」做机械判据，本轮只记不做。
- ⑬ **净行数与落盘形状**：本账 4693 → **4711** 行（净 +18 = 清单区 +1、L232 自指 823 → 824 同行改写净 0、⑫ 归位一删一加净 0、日志块自身 +17 = 前置空行 1 + 标题 1 + 段内空行 1 + 要点 14）。清单区那一行落在第 823 行**之上** ⇒ L232 那句 operative 自指按现量同行改写到第 **824** 行，落点行以 `sed -n '824p'` 复核仍含 `docs/augmentor/API.md:1139` 那一处活引用；链尾不补第十六个值，与 L261 / L262 同口径——这一形不带文件名、a184 的结构维与内容维都不量，缺口早已立在 A333。写前读数 crlf **0** / 内容级裸 CR **1** / 以换行结尾，写后三格不变；批写走 `read_bytes().decode()` 归一，不经通用换行翻译。**卷一除 `A71` 那一格定级格（1/1 净零行）外一格未动** ⇒ l98 那六个哈希槽数字不动。
- ⑭ **门禁读数**：全量定标（逐字 `python -m pytest tests -q -p no:cacheprovider`）终态 **9243 passed / 0 failed / 10 skipped / 6 warnings in 280.51s (0:04:40)**，覆盖率行逐字 `TOTAL 14384 8 3968 22`、`Total coverage: 99.84%`——四格与 L260 / L261 / L262 各轮**逐字相同** ⇒ 本轮零产品代码、覆盖率分母不动。9243 = L262 的 9225 + 新守卫 **18** 支（`def test_` 数与采集数两侧独立闭合）。账本批之前的九把账本与文档守卫定向读数 **227 passed / 8 skipped**（八把 209 + 新守卫 18），8 支 skip 仍是 `test_doc_line_refs_l79.py:647` 那一族的作者工作树条件。前端三门禁**不适用**（`web/` 零改动，不是漏跑）。
- ⑮ **记账批（本条所在提交）读数**：账本批（`08c0617d4`）落盘后九把账本与文档守卫复跑 **227 passed / 8 skipped**，与账本批之前**逐字同读数**（8 支 skip 仍是 `test_doc_line_refs_l79.py:647` 那一族的作者工作树条件） ⇒ 本批四处改动（清单行 +1、L232 自指 823 → 824 同行改写、⑫ 归位一删一加、日志块 +17）没有平移任何活引用，也没有放松任何一把尺子。全量定标取自账本批之前的同一次落盘状态：**9243 passed / 0 failed / 10 skipped / 6 warnings in 280.51s**、`TOTAL 14384 8 3968 22`、99.84%；记账批本身只在日志块**尾部**追加这一条，不动收集面也不动第 824 行之上的任何一行 ⇒ 不重跑全量，这一句按 L261 / L262 的同一口径如实标注而不是补一次读数。净行数复核：代码批 `git diff --numstat` 读 **1/1、1/1、4/4、8/8、1/1**（五把改动全部净零行；新守卫 344 行是全新文件）⇒ 这正是 `test_comment_refs_a184` 硬 0 档要的形状，插行救火那一步本轮没有发生。**同场量到但不进本轮**：「日志块要点落错块」这一形**没有尺子**——`test_ledger_hash_slots_l98` 判的是清单区那一格的哈希槽与 git 真值对账，扫不到日志块内部的要点归属，所以 L262 那条 ⑫ 落进 L261 块尾之后一路全绿而被本轮人眼发现；若要立账需按「要点里点名的 hash 必须出现在同块所属轮号的清单行」做机械判据，本块 ⑫ 已把这一格记在案上留给下一轮。

### L264（2026-10-13）— 日志块要点里点名的哈希第一次能归档回本块轮号：76 处六档入尺，两格 cross_unnamed 落定，自己的门禁顺手逮出新守卫两处 cp936 解码

- ① **判据是字面形状 + 一次 git 问话**：站点 = `## 循环日志` 之后、`^### L` 开块行内被反引号包住的 6 到 40 位 hex（允许尾随一个省略号，账本里 sha256 缩写就写成那一形），每处问 `git rev-parse --verify <h>^{commit}` 与 `git show -s --format=%s`，按答案分**六档**。分档判据全部是复算出来的字面形状，不是读后印象。
- ② **起因是 L262 的真实事故，而九把账本与文档守卫当时全绿**：L262 的记账批把「记账批读数」那条 ⑫ 插到了 **L261 块尾**（`### L262` 标题在它下面）。`test_ledger_hash_slots_l98` 判的是**清单区**那一格的哈希槽与 git 真值对账，日志块内部的要点归属**没有任何一把尺子**。本轮臂 `test_arm_moving_a_bullet_into_the_wrong_block_breaks_the_same_round_count` 把那一形打成 `same_round` 减 1 且缺陷档 1 格 ⇒ 落错块从此必红。
- ③ **与 l98 的分工现场验，不靠叙述**：`test_the_whole_face_sits_below_the_log_section` 断言每只站点的行号都严格在 `## 循环日志` 边界之下 ⇒ 两把尺子按构造互斥，不会重复判同一格；`test_volume_one_contributes_no_site_at_all` 量出卷一的日志块里一处反引号 hex 都没有（**0 是量出来的**，不是没写）。
- ④ **六档而不是四档的理由**：`not_a_commit` 那批全是 **sha256 摘要**（注入器逐臂验盘用的前缀或全文），它们不是提交，本来就不该拿轮号去问；把它们混进缺陷档会造出 21 格假红。
- ⑤ **`stash_or_wip` 一档的来历与它的到期日**：账本唯一一次引用 stash 是 L242 块记录共享工作树事故时点名的 `git fsck --lost-found` 不可达对象 `82f4a4cab3e86d66abc06ac9cb60b7a87532144a`，它主题里的轮号是从被 stash 的 HEAD **继承**来的 ⇒ 算成缺陷是假缺陷。`test_the_stash_tier_is_what_keeps_that_site_out_of_the_defect_tier` 现场证明这一档承重（内存里清空 `WIP_HEADS` ⇒ 该档归 0、缺陷档立刻 1）。**已知到期日**：将来 GC 真把对象收走，它从这一档掉进 `not_a_commit`、名册当场红 ⇒ 届期按实量重钉，不偷偷改档。
- ⑥ **本轮收掉的两格 `cross_unnamed` 都是同行改写、各 1/1 净零行**：L215 块的 `4d177e3f3` 补「（L212 至 L214 那一批账本补写）」、L238 块的 `29b715600` 补「，L194」。改前读者拿到一枚哈希却不知道它属于哪一轮、更不知道它为什么出现在这一块里；改后 `test_the_two_rows_name_the_round_they_point_at` 逐格验点名维确实吃到这两格。
- ⑦ **缺陷档写成精确相等而不是上界**：`names(sites(), UNNAMED) == set()` 与 `MEASURED_COUNTS[UNNAMED] == 0` 两格同断言（写成上界就允许一格回退）；反向臂 `test_arm_dropping_the_named_round_falls_into_the_unnamed_defect` 与 `test_arm_pointing_at_another_rounds_commit_without_naming_it` 各自删掉一处点名 ⇒ 缺陷档立刻 1 格，那个 0 不是恒真。
- ⑧ **已知盲区三格写明，不装**：一是不带反引号的裸 hex 不进面（臂 `test_arm_a_negative_prose_shape_is_not_a_site` 钉住这一维确实是形状判据而不是「反正都会红」）；二是 ambiguous 短哈希被 `--verify` 拒 ⇒ 落 `not_a_commit`，名册是精确集合相等，撞车即红；三是 sha256 前缀若与某提交前缀撞车会被当成提交，概率极低而一旦撞车必落进非本块档而红。
- ⑨ **六臂自证（`TestRulerBites`）全部内存改写、零落盘**：搬错块 / 删点名 / 造一枚 git 里没有的哈希 / 删标签里的轮号 / 把一处本块引用换成那枚 stash / 把反引号去掉。六臂各自红的档位与常数一一对应，没有一支是靠查标题。
- ⑩ **全量门禁第一轮 2 红，红的是新守卫自己的缺陷**：`test_subprocess_text_decoding` 规则 A（文本模式必须显式给编码，**无豁免档**）判 `tests/unit/test_log_block_bullet_ownership_l264.py:79` 与 `tests/unit/test_log_block_bullet_ownership_l264.py:308` 两处 `text=True` 没传 `encoding=` ⇒ 父进程按 `locale.getpreferredencoding()` 解码，中文 Windows 是 **cp936**。这不是形式问题：本仓提交主题带中文，cp936 解 UTF-8 字节在本机是 mojibake 而 `L(\d{1,3})` 恰好还能从里面吃到轮号（所以新守卫自己 24 支全绿），换一台机器或换一个字节序列就是 `UnicodeDecodeError` 炸在采集阶段。按该守卫自己写的修法补 `encoding="utf-8", errors="replace"` 两处；第二支的改写让该文件 331 → 332 行，而两处站点行号未平移。
- ⑪ **为什么走规则 A 补编码，而不是登记进 `BYTE_MODE_ALLOWLIST`**：规则 B 的登记要求「父进程不解码文本 + 用具名编解码器 + 子进程钉 `PYTHONIOENCODING`」三格同时成立，而本守卫解的**正是文本**（git 主题）⇒ 登记等于承认要解码却不给编码。规则 A 无豁免是 L195 立这把尺子的原意，不改尺子。
- ⑫ **A333 预言的裸行号漂移本轮实测到一次**：L263 的 ⑬ 写着「L232 自指 823 → 824 同行改写净 0」，而 `git show 08c0617d4` 复核第 70 行那一格**仍是 823**——那句改写从未落地。本批实测 `docs/augmentor/API.md:1139` 那条引用在**第 825 行**（L263 的清单行 +1 推的，本批清单行再 +1）⇒ operative 按现量重钉，并在链尾补上 L263 那一格缺失的归因。其余 13 处「本账第 NNN 行」写在日志块里，是各自写下当时的读数、属历史陈述，**批量改写等于伪造历史** ⇒ 口径留给 A333 的尺子，本轮不动。
- ⑬ **净行数与落盘形状**：本账 4712 → **4730** 行（净 +18 = 清单区 +1、L232 自指与链尾同行改写净 0、日志块自身 +17 = 前置空行 1 + 标题 1 + 段内空行 1 + 要点 14）。批写走 `read_bytes().decode()` 归一、不经通用换行翻译；落盘后 crlf **0** / 内容级裸 CR **1** / 以换行结尾三格不变（由 `test_backlog_closure_l242` 的 `test_line_endings_are_the_documented_shape` 现场验）。**卷一一格未动** ⇒ l98 那六个哈希槽数字不动。前端三门禁**不适用**（本轮 web 零改动）。
- ⑭ **门禁读数**：全量定标（逐字 `python -m pytest tests -q -p no:cacheprovider`）第一轮 **2 failed / 9265 passed / 10 skipped / 6 warnings in 391.48s**，覆盖率行逐字 `TOTAL 14384 8 3968 22`、`Total coverage: 99.84%`——两红都在 `test_subprocess_text_decoding`（见 ⑩），无一处产品代码缺陷；修复后定点八把（新守卫 24 支 + l195 那把 + 六把账本与文档守卫）**185 passed / 8 skipped**。本批落盘会让日志块自身进入本守卫的站点面 ⇒ 现量常数按红读数重钉，终态全量读数由记账批补。
- ⑮ **记账批（本条所在提交）读数**：账本批（`2788ef66b`）落盘后十把账本与文档守卫（新守卫 24 支 + l195 那把 + 六把账本与文档守卫 + l242 / l253 / l263 三把 Backlog 尺子 + l97 / l98 两把普查尺子）复跑 **282 passed / 8 skipped**，l98 那 5 个 error 是定向子集的结构性自证（子集只采到几十条 < 下限 3000 的 `RulerNotProven`），与全量门禁无关。**终态全量定标**（逐字 `python -m pytest tests -q -p no:cacheprovider`）**9267 passed / 0 failed / 10 skipped / 6 warnings in 307.27s**，覆盖率行逐字 `TOTAL 14384 8 3968 22`、`Total coverage: 99.84%`——四格与 L260 / L261 / L262 / L263 各轮**逐字相同** ⇒ 本轮零产品代码、覆盖率分母不动。9267 = L263 的 9243 + 新守卫 24 支（`def test_` 计数与采集数两侧独立闭合）。第一轮全量的 2 红（⑩）修复后不再复现。**本条把账本批那枚哈希写进了面** ⇒ 站点现量 79 → 80、`same_round` 41 → 42（主题点着 L264，落进本块），常数按红读数重钉在同批。A333 那一格本轮实测到一次真漂移（⑫），但**尺子仍未立**：「本账第 NNN 行」这一形今天没有判据，L265 之前它还会继续漂 ⇒ 记账批不动其他 13 处历史读数，只把证据留在案上。

### L265（2026-10-13）— 4xx 保码面第一次有全仓 AST 普查尺子（58 支 / 50 配对点 / 97 只 raiser 闭包），全仓缺数语句由 8 格走到 0：99.90%

- ① **收的是哪一格**：`api/deps.py` 的五只闸用 `HTTPException` 出声（401 / 403 / 400 / 404 / 413），路由收尾只住一处（`to_http_error` / `raise_internal_error` / `raise_boundary_error`），而这三兄弟**都不认识** `HTTPException`——docstring 明写「它应当由调用方原样抛出，否则会被降级成 500」。于是 `except HTTPException: raise`（且必须在宽 `except Exception` **之前**）是判决能否到达客户端的唯一通道；通道断掉时对外只剩固定文案 `INTERNAL_ERROR_DETAIL`，运维日志再把一次正常授权判决记成故障。这一格与 A155①（500 档不回显原文）是同一条口径的两半。
- ② **为什么样本测试不算尺子**：`tests/integration/test_api_passthrough_branches.py`（L69）按端点逐个注入，删掉某一路由的那一支、或新写一条漏掉那一支的路由，它一条都不会红。全仓现量 58 只支、50 个配对点，改造前对这一族的读数能力是零 ⇒ 本轮是「同轮立尺」而不是「补用例」。
- ③ **raisers 靠不动点闭包而不是名册**：种子 = 体里直接 `raise HTTPException` 的 66 只，扩张 = 「体内调过已知 raiser」迭代到不再新增的 31 只，合计 **97**。`test_the_closure_seeds_every_direct_producer` 写成集合相等（`prod - known == set()`）而不是「≤」，因为种子漏一只直接产 4xx 的函数，就等于整只文件不在面上。
- ④ **三档体形，缺陷档写成精确空集**：`bare`（整支就一个裸 `raise`）**57** / `conditional`（带条件再抛）**1** / `swallow`（咽下去）**0**。`conditional` 全仓唯一一只是 `api/routes/data.py` 的 `_upload_landing_path`（A160 与 A163：先解释一次、越界才救）；`swallow` 若出现就是「4xx 被静默吃掉」，所以断言是 `== set()` 而不是「≤1」。
- ⑤ **配对面按装载（`ast.Name` 的 `ctx is Load`）计量，而不是按调用**：装载维 **50** 只 try、调用维只有 **37**，差 **13** 只；少掉的 13 只里 raiser 是当**参数**传出去的（`await run_in_thread(load_items, request.input_file)`，`api/routes/audit.py` 第 31 行那一形）。按调用维写判据的后果不是「少几格」而是 `audit.py` / `data.py` / `leakage.py` / `privacy.py` **四只文件的配对面整块不存在**——连「这一格有没有支」这个问题都问不出来，而 `upload_data` 恰恰是真会 403 的路由。
- ⑥ **尺子有牙齿：六臂全部内存改写、零落盘**（承 L218 之后的「会改仓库文件的探针必须自带复验」，本轮走不到落盘那一步）。搬掉一支保码支 ⇒ 配对档当场点名；把支写在宽支**之后** ⇒ 顺序判据红；造一只吞异常的支 ⇒ `swallow` 档 1 格；在真实模块尾部追加一只全新 raiser + 一条无守卫路由 ⇒ 种子含它、missing 恰 1 且点名 `gate_l265`；摘掉 `audit.py` 那一支 ⇒ 装载维红而调用维**看不见这个缺陷**（盲区实证，`TestRulerBites` 的臂 D）。
- ⑦ **已知盲区三格写明**：① 属性调用可达面（`p.method()`）不进闭包，依据钉在磁盘上（`test_the_sdk_side_cannot_raise_http_exception` 现验 `augmentor/` 一侧零 fastapi import），不是「应该没有」；② 嵌套函数按名字并档，同名会合格；③ `try` 体外 raise 的 4xx 不进配对面，因此 7 支是纯防御性冗余——由名册支钉成**已知名册**，而不是让它看起来像漏判。
- ⑧ **第二只文件接的是 L260 记账批 ⑪ 留下的那 8 格缺数**（`semantic.py` 一处、`versioning.py` 两行、`data/image.py` 两行、`api/routes/multimodal.py` 两处、`api/routes/vector.py` 一处）：每一格都配**反向臂**，否则断言只是在复述代码——vector 那一臂把闸抛的 403 与「没有支时的 500+固定文案」放在一起比，`multimodal` 把「两级皆空」与「配置级填一格」放在一起比，指针那一臂把「软链接命中」与「`.txt` 命中」放在一起比。
- ⑨ **两处诚实边界**：真符号链接形状本机不可测（`os.symlink` 抛 **WinError 1314**），所以替的是指针对象本身而非宣称「真实软链接也这样」；真 Pillow 形状本机不可测（未装包），所以桩报的尺寸与标准库头部解析器**故意不同**（9x8/WEBP 对 111x222/PNG），量到的是「Pillow 优先」这条声明而不是「能跑通」，并补一支复验桩没有漏进后续用例。`semantic.py` 的早退按实测定性成**等价类快路径**（`normalize_vectors(np.zeros((0, d)))` 不抛且 `list(...)` 仍是 `[]` ⇒ 早退与不早退同值），它唯一有牙齿的那一格是「早退不越过维度闸」。
- ⑩ **自己的账读错了一格**：新守卫 docstring 第一版写「7 条语句」，而逐格点数 1+2+1+2+2 = **8**（与门禁里「缺数 8」那一格闭合）。这不是笔误级别的小事——本轮的全部卖点就是「残差有名册」，名册条数读错等于把尺子的证据自己钉歪，所以单独一枚提交 `58a1f9987` 修它，而不是留到记账批顺手带上。
- ⑪ **代码批哈希被并行会话的提交带走**：两支守卫按显式路径 stage 后 `git commit` 返回「no changes added to commit」；`git show --stat 930e31f74` 复核该提交 6 只文件，其中含我的 `tests/integration/test_untested_guard_branches_l265.py`（+348）与 `tests/unit/test_http_exception_passthrough_l265.py`（+746），主题是 `sec(oa4rust/backend): general/org 管理写族越权门禁…` ⇒ **主题里没有 L265 token**，本轮第一轮没有点着轮号的代码提交。验盘：`git status --porcelain` 干净、两只文件已 tracked、磁盘字节与跑过全量门禁的那一份相同。**未做历史改写**——共享工作树里改写别的会话已落盘的提交，风险远大于这一格洁癖；这是「共享工作树会吞暂存区」的第二种形状（第一种是 stash 事故）。落到尺子上的后果由记账批按红读数重钉：本块写进面之后 `test_log_block_bullet_ownership_l264` 的站点数 +1，而 `930e31f74` 的主题问不出轮号 ⇒ 从 `same_round` 之外落进 `no_round_subject` 档（现量 6 格），常数随记账批重钉。
- ⑫ **覆盖率闭合读数与剩下的面**：终态全量 `9316 passed / 0 failed / 10 skipped / 6 warnings in 311.60s`，`TOTAL 14384 0 3968 18`、99.90% ⇒ 语句维**缺数 0**（本账开轮以来第一次）；分支维仍余 **18 格偏支 / 16 只文件**，逐格名册（记号「甲→乙」= 从甲行跳到乙行，负数按 coverage 的出口约定写）：`api/main.py` 197→211、`augmentor/cache.py` 300→306、`augmentor/cleaner.py` 165→164、`augmentor/cli/commands/ops.py` 125→出口、`cli/commands/quality.py` 86→91、`cli/commands/version.py` 99→出口 与 183→出口、`augmentor/config_validator.py` 837→840、`augmentor/export.py` 270→279、`augmentor/model_manager.py` 23→26、`augmentor/pipeline.py` 399→382、`augmentor/rag.py` 61→68 与 63→65、`augmentor/report.py` 145→138、`augmentor/retry.py` 197→237、`augmentor/sampler.py` 281→276、`augmentor/search_enhanced.py` 227→出口、`augmentor/visualizer.py` 46→出口。这一维此前只有聚合数、没有名册 ⇒ L266 的靶子。
- ⑬ **为什么零 parametrize**：`test_parametrize_ids_l97`（438 / 317）与 `test_parametrize_blind_face_l98`（盲面 116 / 四档 61-38-11-11 / 豁免 25）把全仓 parametrize 三面冻成现量数，本批两支守卫一律走名册字典循环 ⇒ 六格读数一格未动（由终态全量绿反证，不靠推理）。
- ⑭ **自指落点**：L232 那句 operative 自指按现量 **825 → 826**（本批清单行 +1、零新 Backlog 行），链尾补第十七个值（781 / 786 / 789 / 790 / 791 / 792 / 795 / 796 / 799 / 802 / 805 / 806 / 807 / 810 / 813 / 825 / 826）；A333 的尺子仍未立，这一形不带文件名，a184 的结构维与内容维都不量它。
- ⑮ **净行数与落盘形状**：本账 4731 → **4750** 行（净 +19 = 清单区 +1、L232 自指两处同行改写净 0、日志块自身 +18 = 前置空行 1 + 标题 1 + 段内空行 1 + 要点 15）。批写走 `read_bytes().decode()` 归一、不经通用换行翻译；落盘后 crlf **0** / 内容级裸 CR **1** / 以换行结尾三格不变（由 `test_backlog_closure_l242` 的 `test_line_endings_are_the_documented_shape` 现场验）。**卷一一格未动** ⇒ l98 那六个哈希槽数字不动，而 R6 的白名单也仍是你四格：`LEDGER` 只指第一本账，所以 ⑪ 那一格不会让 R6 红——这一点由跑守卫实证，不靠读代码推理。前端三门禁**不适用**（`web/` 零改动）。
- ⑯ **记账批（本条所在提交）读数**：账本批（`1094b0696`）落盘后六把账本与文档守卫 **154 passed / 8 skipped**（含 a184 硬 0 档与 l79：L232 那句自指改到第 826 行之后，落点复验仍是含 `docs/augmentor/API.md:1139` 的那一句；本条只追加到文末 ⇒ 不动任何历史行号）。l242 / l253 / l263 与 l264 四把合跑首跑 **9 failed / 88 passed**，九格全落在 l264 的现量常数上，逐档归因：站点 80 → **86**、`same_round` 42 → **44**（本块写进面的哈希里点着 L265 的那几枚）、`no_round_subject` 6 → **10**（⑪ 那枚被并行会话吞走的代码批 `930e31f74`，加上本条为「HEAD 被谁推进」留证而点名的 `b7b7f0b94` 与 `adc0df075`，三枚主题都不带轮号 token），`cross_named` 9 / `cross_unnamed` 0 / `not_a_commit` 21 / `stash_or_wip` 2 四档一格未动，`MEASURED_NOROUND` 唯一名册 4 → **7**；**没有任何一把尺子被改松**，全部按红读数重钉。**第九红不是账本的错而是尺子取错了观测量**：`test_the_git_side_is_alive` 原写作「HEAD 的主题必须点着轮号」，而共享工作树里 HEAD 由并行会话推进（本轮实测两次），于是「git 侧活着」被当成「最近一枚提交属于本仓」——并行会话一动就凭空红。按该守卫自己声明的意图改成「HEAD 解析得到主题 + 近 40 枚提交主题里读得到 ≥200 的轮号」，两处都必须从 git 真取到东西，git 坏掉时两格同时空 ⇒ 牙齿没被削；增行全部落在该文件第 311 行之下（332 → **341** 行）⇒ 账本里既有的两处活引用（该文件第 79 与 308 行）零平移；常数那两处改写按净零行写（`git diff --numstat` 复核 `7 7` ⇒ 净 0），因为第 308 行就住在它们下面。**记账批自身给本账 +1 行**（4750 → 4751，只在文末追加一条要点，清单区与 L232 自指均未再动）。**九格里有两格重钉的不是常数，而是判据本身**：`test_the_repeat_is_a_single_line_triple_mention` 原写作「同一行有 3 处 `no_round_subject` ⇒ 那三处必须是同一枚哈希」，而这一形在 L264 之前确实是全账唯一的重复形状（L236 块那一枚单行三名）；本条在同一行点名三枚**互不相同**的跨项目提交（就是上一格列出的那三枚）⇒ 判据按新形状红 `assert 6 == 3`。改法不是删判据，而是**把「行内三名同一枚」升成「按哈希逐只点名重复」**（一枚三笔、一枚两笔，重复名册写成字典精确相等），原先那一格的牙齿（同一枚的三笔必须落在同一行）由单独一条断言保留。本段刻意不再写入新的反引号 hex：⑯ 已经点名的三枚把面推到位之后，重复点名只会把「同一枚被点几次」这一维搅浑，而那正是本守卫要量的东西。**终态全量定标**（逐字 `python -m pytest tests -q -p no:cacheprovider`）**9316 passed / 0 failed / 10 skipped / 6 warnings in 330.95s，exit=0**、`TOTAL` **14 384 语句 / 0 缺数 / 3 968 分支 / 18 偏支**、聚合 **99.90 %**、`117 files skipped due to complete coverage`。用例数与账本批那一跑（同口径 311.60s）逐格相同 ⇒ 记账批对 l264 的常数重钉与判据改写没有让任何一把尺子失去或获得用例（本轮零新用例）。
- ⑰ **记账批复验 git 侧时发现「代码批其实掉出过分支」——本轮最重的一条**：`git status` 把 `tests/unit/test_http_exception_passthrough_l265.py` 报成**未跟踪**文件，`git ls-files` 与 `git log -- <该路径>` 在 HEAD 历史上**零命中** ⇒ 746 行、30 支用例的主守卫从来不在分支上。链路逐格按 `git reflog` 现读，不凭记忆：并行会话把暂存面吞进它自己的提交（那果确实带走我的两文件 +348/+746），随后 `reset: moving to HEAD~1` 撤销该提交并重做为 `d9578f87b`（同标题、摘掉我的文件）；我这侧当时只补交了集成守卫 `58a1f9987`（那果的 stat 是 349/0 纯新增 ⇒ 当时那个文件同样不在 HEAD 上），主守卫因此一直躺在工作树。**全量门禁跑的是工作树** ⇒ 9316 的读数没造假，⑫ 写的「被并行会话吞走」也是事实；造假的是「本轮代码批已在分支上」这个隐含假设——它在 ⑫ 落笔之后被那次 reset 推翻，而我没有复验。**处置**：先补交代码批 `ee133fc40`（746/0，与被吞走那份 blob `cmp` 逐字节相同 ⇒ 工作树这份自被吞走起未再改动），再把清单行的哈希槽按分支真值改写（摘掉游离的那果，换成 `58a1f9987` + `ee133fc40`，账本批写成 `b2af89396`），最后才是本条。**同一批还复现：账本批自身也被改写过两次**——我提交的 `1094b0696` 在 reflog 里先被 `commit (amend)` 成 `b9d3ad7b1`（挂着他们前端那条标题），再被 amend 回我的标题 ⇒ 分支上真实的那果是 `b2af89396`（与 `1094b0696` 同为本账 20/1）。⑯ 里「账本批（`1094b0696`）」因此指向一果**游离**对象：它现在仍能解析，所以 l264 不红，但 `git gc` 之后就会消失 ⇒ 那一格连同本条一起按分支真值看，不美化。**教训成一句**：共享工作树里「我提交完了」不是终态，记账批必须重新 `git ls-files` 逐只点名的文件验一验在不在分支上——此前各轮只验哈希可解析，从不验 blob 归属。
- ⑱ **记账批落盘之后 git 侧又漂了一次，本条按 HEAD 现读写**：⑰ 里那格「分支上真实的那枚是 `b2af89396`」在 `fb0ba6e91` 落盘后几分钟内就被并行会话的下一次改写推翻。同一时刻 `merge-base --is-ancestor` 现读：L265 三批在分支上的真身是 `58a1f9987`（集成守卫）、`fdc8a5e4a`（主守卫补交，原 `ee133fc40`）、`0b32da27f`（账本批，原 `1094b0696` → 中间态 `b9d3ad7b1` → `b2af89396`），记账批自身是 `fb0ba6e91`。**面上 60 格点名的提交里 6 格已不在分支上**（`1094b0696`、`930e31f74`、`adc0df075`、`b2af89396`、`b9d3ad7b1`、`ee133fc40`，全部集中在 265 块），其余五十五格在。**为什么 l264 不红**：那把尺子验的是「git 解析得到 + 主题说什么」，而游离对象在 `git gc` 之前照样解析得过 ⇒ 「代码批在不在分支上」这一维全账无人量，本轮已经被它咬两口（一次文件整只被吞走、一次哈希指到游离态）。**⇒ 立 A344**（M，待拍）：给 l264 加一维「点名的提交必须是 HEAD 的祖先」，并定下 `git gc` 之后那六格重钉成什么的口径；本轮不动它，只把现量与归因钉住。
### L266（2026-10-14）— 分支维残差 18 格逐格定性并清零：7 格死支删除或等价改写 + 5 格链尾栅栏 + 6 格可达补测（全仓逐字 `TOTAL 14382 0 3952 0`、100.00%）

- ① **收的是哪一格**：L265 ⑫ 记下「余 18 格偏支 / 16 只文件逐格点名进日志块」并把它们交回本账当靶子。本轮收的就是那 18 格：语句维自 L265 起已经是 0，分支维此前只有聚合数（`3968 22` 一路读到 `3968 18`），本轮走成 `3952 0`。
- ② **三档是怎么分的（判据是「删掉那一支会不会改变行为」，不是「看着顺不顺眼」）**：**甲 7 格死支**——那一支在当前不变式下永远取不到，留着只会让分母虚高；**乙 5 格链尾**——链的兜底档可达（choices 与分支表脱节时必然走到），但原来什么都没说，本轮给它加出声；**丙 6 格可达且行为正确**——只是没有任何一条用例把那条弧在本进程里走过，补测即闭合，产品一行未改。三档由新守卫的三张名册 dict 承载，键集与格数都是判据（`len(DEAD_REWRITES) == 7`、`len(CHAIN_FENCES) == 5`、`len(COVERED_ARCS) == 6`，另有一支把三面并集与 16 只文件的名册做精确相等）。
- ③ **推翻了 L124 那条「记档不测」的旧判法**：本轮之前，「链尾那支不可达 ⇒ 只能记档、不能测」是账上的口径。实测把它推翻了两半——甲档那 7 支确实不可达，正解是**把支删掉或把双权威收成单权威**（分母闭合，无需造假证据）；乙档那 5 支不是不可达，而是**没有栅栏**，加 else 出声之后它变成可达且可测（`argparse` 那三格另有一支验它仍然exit 2，栅栏没有把参数错误吞掉）。留一支脱节注入臂：往 `choices` 里塞一个没有分支的动作，栅栏当场喊红。
- ④ **rag 那一格凭什么敢删两支**：`chunk_text` 旧形是「步长推进 + `if chunk:` 兜空 + 到尾 break」。构造期不变式 `require_chunk_window`（size≥1、0≤overlap<size）把 step 钉在 (0, size]，于是空块支与「耗尽而未 break」那两支都取不到。**改写不是凭这套推理落盘**的：测试里复刻了旧循环，与改写后的实现在 **3120 格**（尺寸档 × 重叠档 × 三种文本长度）网格上逐格对拍位相同，其中 `["abcd","de"]` 那一格是重叠为 0 时补尾支的形状——网格跑不满就不敢叫位相同。
- ⑤ **coverage.py 的弧语义是本轮唯一的「尺子决定修法」的一格**：一条循环行的出口弧只在**自然耗尽**时记一次，而 body 里的 break 记在跳走的那只 `if` 上。所以「range 上界 + break」这种双权威写法会让自然耗尽那条弧**永远不会被取到**⇒ 它不是覆盖缺口，是不可测量的死弧。`retry.py` 19/19 的改法就是把第二条终止权威删掉（只留「不可重试」那一支 break），循环改由 range 耗尽退出；配套判据钉的是**行为不变量**而不是行数：attempts 计数与 sleeps 计数各自等于名册（最后一支不 sleep）。
- ⑥ **report 的 8/9**：指标名 if/elif 链改三元组数据表（`("semantic_similarity", "语义相似度", "收敛生成约束或优化prompt")` 那一形）。链尾那一支原本在指标名不在名单时静默什么都不做 ⇒ 表化之后没有链，也没有静默。对拍写成「旧链参考实现 vs 新表输出」在同两组输入（4 / 10 条指标）上逐格相同，另补一支点名建议文案本身。
- ⑦ **乙档 5 格栅栏的形状按各自族的权威选**：CLI 那三格（dependency / version / backup）打 `stderr` 一行写明「choices 与分支表脱节」再 `sys.exit(1)`；库内两格抛域异常——`export.py` 抛 `UnsupportedFormatError`（那句消息是「在 `NATIVE_FORMATS` 里，却没有原生导出分支」），`sampler.py` 抛 `DataValidationError`（「的类别 X 没有对应的种子分支」）。**为什么库内不 print**：库函数把判决写进 stdout 就等于把它丢掉，异常才有出口。
- ⑧ **裸 raise 被自家普查尺逮回来**：两格栅栏第一版写 `raise ValueError(...)`，`test_exceptions.py::test_no_bare_builtin_raises_left_in_package` 立刻点名 ⇒ 换成域异常。import 放在 else 支**内部**（局部 import）而不是文件头：文件头加一行会把该文件全部活行号往下推一格，而 a184 的量面是 `file.py:NNN` 引用落点。
- ⑨ **插行平移历史行号第三次撞上同一缺陷类**：`ops.py` 净 +7 ⇒ `docs/augmentor/ARCHITECTURE.md` 模块树里那条指向 `cli/commands/ops.py` 的活引用（当时写的是第 159 行）被推出代码跨度，a184 硬 0 档与 l79 **同时红**（与 skill 第 5 节记的 L217 / L219 同一形状）。处置走处方 3 **同行重钉**为 `cli/commands/ops.py:166`（净 0 行），`git diff --numstat` 复核该处 1/1。
- ⑩ **丙 6 格为什么零产品改动**：六格的共同形状是「可达、正确、只是本进程没人走过」。`cache.py` 淘汰走到最后一条、`model_manager.py` 与 `search_enhanced.py` 的双检锁第二重、`pipeline.py` 并行失败而不带断点、`visualizer.py` 的第二次加载 ⇒ 各补一条取数路径即可，断言写「那一格的行为差」而不是「跑通了」。
- ⑪ **`api/main.py:197` 那条假支：子进程读数对本格无效**：原先准备起一个真进程去验「没设 key 会告警」，本轮按覆盖率口径否决——coverage 只统计本进程，子进程里跑得再对也不会让 `197→211` 那条弧闭合。改法是同进程 `importlib.reload` 走两个方向（设 `AUGMENTOR_API_KEY` ⇒ 不告警 / 删掉 ⇒ 告警），断言写成「文本里出现『无鉴权』当且仅当 expect_warning」，收尾刻意停在「没设 key」那一档，免得把后续用例的读数污染成「没告警」。
- ⑫ **恒真断言被卫生尺当场判死**：名册那一支第一版写 `assert 7 + 5 + 6 == 18`，`test_test_hygiene.py::test_no_vacuous_asserts`（判据在 `scripts/triage_tests.py`）判它字面对字面恒真 ⇒ 改成读三张 dict 的现长度求和。**这一格值得记的理由**：三档格数是人算出来的，写成字面量就是把结论当判据；改成现读之后，谁把某档改成 6 而总数不动，尺子立刻红。
- ⑬ **普查判据「在真实源码上必须 0 命中」那一格逼出一次收窄**：`_dedup_by_err_paths` 原本认「任何含 `err_paths` 的 `not in`」，而 `config_validator.py` 改写后仍留两处合法的 `path in err_paths` 读形状 ⇒ 判据收窄成**只认裸 `x not in err_paths`**，另用两条 `count(...) == 2` / `== 1` 钉住现形；`test_the_shape_checkers_read_the_real_sources_not_a_fixture` 逐条在真实源码上取规定命中数，防「判据只对自己的注入样本有效」。
- ⑭ **注入取证 9 臂**：新守卫 `TestRosterAndRulersAreNotVacuous` 九臂全部走内存改写、零落盘（承 L218 之后「能不改仓库文件就别改」），逐臂各自打红并按名册点名；另有一臂证明三面并集这个分区不是恒真（把某档挪一格就撞车）。
- ⑮ **共享工作树这一轮又漂了一次**：开工时 HEAD 在 `main`（并行会话切的），L260–L265 的 8 格接线不在那上面 ⇒ 首跑读数与 L265 的账对不上（缺数 8、用例 9236）。处置顺序是先归因再动手：`git diff --stat main optimism20261012 -- <本轮 11 只文件>` 逐只取空 ⇒ 证明我的改动面在两分支等价；三批改动按显式路径导出到仓外快照（patch 14157 B / 守卫 43690 B）留证；那两只未跟踪的 `auto-checkin/` 文件先证与 optimism 上的 blob 逐字节相同才挪开，切回分支后由 git 原样带回。**换分支是并行会话的状态，不是我的** ⇒ 这一步先向用户拍板（答复：切回 optimism 后收口）。A344 那一格（共享工作树吞提交 / 换机器部署）本轮仍未拍，它不是本轮能自行处置的面。
- ⑯ **自指落点**：L232 那句 operative 按现量 **826 → 829**。归因两刀：L265 的记账批 `19c96b420` 在 Backlog 区落了 +2 行（那笔新账与它的分隔空行），而那一轮只往括号链尾补了数字、没有重钉句首那一格，真值当轮就漂到 828（本轮 `sed -n '828p'` 现读到含 `docs/augmentor/API.md:1139` 的那一句）；本轮清单行再 +1 ⇒ 829。**这是 A333 那句「本账第 NNN 行那一形没有尺子」的第二次实测**（第一次是 L265 的 ⑫，`git show` 复核 L263 声称的 823→824 从未落地）。本轮不新立账，证据继续记在 A333 名下。
- ⑰ **净行数与落盘形状**：本账 4755 → **4777** 行（净 +22 = 清单区 +1、L232 自指两处同行改写净 0、日志块自身 +21 = 标题 1 + 段内空行 1 + 要点 19），本条只追加到文末 ⇒ 不动任何历史行号引用，a184 与 l79 的落点在写回后复读复核（含本轮重钉的 `cli/commands/ops.py:166` 那一格）。卷二的行尾形状按字节现读仍是「裸 LF 4755 / 游离 CR 1 / CRLF 0」，那枚 CR 是散文内容而不是行尾⇒ 批写的写回断言只能是 `b"\r\n" not in out`，写成 `count(b"\r") == 0` 会把自己判死；流程沿用「读 bytes ⇒ 归一成 LF 做锚点 ⇒ 每处改动断言唯一命中 ⇒ 按原行尾回译 ⇒ 二进制写回 ⇒ 复读比字节」。
- ⑱ **门禁读数**：代码批（`7e2fe5aa6`）落盘后的全量定标（逐字命令）`9362 passed / 0 failed / 10 skipped in 280.57 s`，覆盖率逐字 `TOTAL 14382 语句 / 0 缺数 / 3952 分支 / 0 偏支` 与 `Total coverage: 100.00%`。9362 = L265 的 9316 + 本轮 46 **恰等**；定点批次 426 passed / 8 skipped。wall（280.57 s）对 L265 ⑯ 的 311.60 s 只是负载噪声，按性能轮口径（位相同 + 计数判据，不看 wall）不作结论。前端三门禁**不适用**（`web/` 零改动，不是漏跑）。
- ⑲ **为什么零 parametrize**：`test_parametrize_ids_l97`（438 / 317）与 `test_parametrize_blind_face_l98`（盲面 116 / 四档 61-38-11-11 / 豁免 25）把全仓 parametrize 三面冻成现量数，本守卫 46 支全走名册字典循环与显式 monkeypatch ⇒ 那两把普查尺子一格未动（终态由全量门禁复验）。
- ⑳ **记账批（本条所在提交）读数**：账本批（`6bf1f34ca`）落盘后六把账本与文档守卫 **154 passed / 8 skipped**（含 a184 硬 0 档与 l79：本轮两处同行改写与文末追加都不动活引用落点）。l242 / l253 / l263 / l264 与 l97 / l98 / 卫生尺七把合跑首跑 **6 failed / 138 passed / 5 errors**，六格失败全落在 l264 的现量常数上，逐档归因：站点 113 → **116**、`same_round` 65 → **67**（本块新写进面的两枚都点着 L266）、`cross_named` 9 → **10**（⑯ 那枚 L265 记账批的哈希在同一物理行上写着「L265」⇒ 走点名档而不是缺陷档），其余四档（`cross_unnamed` 恒 0、`no_round_subject` 16、`not_a_commit` 21、`stash_or_wip` 2）一格未动。四把按各自 docstring 的处方重钉，**没有任何一把被改松**；重钉后十把账本与文档守卫合跑 **251 passed / 8 skipped**（那 8 格 skip 全是 l79 需要作者工作树的定向档）。l242 / l253 / l263 与 l97 那把静态普查在首跑里就绿——本轮零新 Backlog 行、零定级格改动、新守卫零 parametrize，那三面本就应当未动。上面那 5 格 error 全在 l98 的会话级动态档，它是「会话级 fixture 在全量 items 低于声明下限时抛 `RulerNotProven`」的**跑法形状**，与案面无关（全量门禁里恒绿），把它记成本轮缺陷是假归因。另记一格本轮新撞的形状：**账本自己写的历史行号也在 a184 的量面里**——我在清单行与 ⑨ 里落了「文件名 + 冒号 + 改前那一格行号」的形状（真值已经是 166）⇒ a184 硬 0 档当场红 2 处，按换点处置把散文改写成「指向 `cli/commands/ops.py` 的活引用（当时写的是第 159 行）」、在新落点只写 `cli/commands/ops.py:166`，不动尺子。⑳ 前几段文字落盘后、本批最后一组同句改写之前，同一条逐字命令在全量门禁上复跑一次：**9362 passed / 10 skipped / 0 failed in 376.41 s**，覆盖率行逐字 `TOTAL 14382 0 3952 0` 与 `Total coverage: 100.00%` ⇒ 与 ⑱ 那格只差 wall（280.57 → 376.41 s），**用例数与覆盖两维一位未动**——这正是性能轮口径里「wall 是负载噪声、不作结论」的那一格的现场样本。**本批文字全部落盘之后的终态复跑不在本条里**——那一格的读数以复跑为准，记进下一轮进场（本轮不把自己没量过的数写成量过）。落盘形状：账本批 4755 → **4777** 行，本批再 +1（⑳ 自身）⇒ **4778** 行，行尾仍逐字「裸 LF 计数与行数相等 / 游离 CR 恰 1 / CRLF 0」⇒ 写回断言只用 `b"\r\n" not in out`。

### L267（2026-10-14）— 坏正则模式不再读成「零命中」：出厂面普查两格逐格定性、调用方那一格改抛域错误（HTTP 400 / CLI 出声），新守卫 12 支零 parametrize

- ① **选题依据（先把安全形状的名单扫干净）**：本轮先按「不安全执行 / 反序列化」那一族普查产品码——`shell=True`、`eval(`、`exec(`、`pickle`、`os.system`、`yaml.load` 全部 0 命中，密钥比较已走 `secrets.compare_digest`（`api/deps.py:58`）。落到「调用方给的字符串被编译成正则模式」这一族才量到活的一格，而且是 HTTP 可达的 ⇒ 不是猜题材。
- ② **普查名册（AST，两格闭合分区）**：出厂面 = `augmentor/` + `api/` + `cli.py`，判据是第一实参**不是字符串字面量**的 `re.<fn>` 调用（`compile` / `search` / `match` / `fullmatch` / `sub` / `subn` / `findall` / `finditer` / `split`）。实测 2 格：`augmentor/search_enhanced.py:568`（模式来自调用方 ⇒ 本轮改的就是它）与 `augmentor/validation.py:721`（模式来自出厂 `PRESET_RULES` ⇒ 定性后不动）。`archive/enhanceTXT.py` 里的 `re.sub(pattern, ...)` 不在射程：它的 `pattern` 来自同文件的源码字面量表 `synonym_rules`，不在任何产品入口的调用图上，也不进覆盖率面；`scripts/` 是开发期工具，同理排除。
- ③ **缺陷形状与读数**：`_search_regex` 把 `re.compile(query, re.IGNORECASE)` 包在遍历所有字段的 `try` 里，`except re.error` 只写一行 `logger.warning` 就返回 `matches`（空）。改前实测：`POST /api/dataset/search` 配 `{"query": "[invalid", "method": "regex"}` 回 **200 + `total_matches: 0`**，SDK 层 `search()` 同样返回零匹配；改后回 **400**，`detail` 逐字 `无效的正则表达式 '[invalid'：unterminated character set（位置 0）`。同一文件的 `normalize_filters` 早就把这一形判成「形状不对就地报错」，本轮是补齐那一族没扫到的第二维——**值本身没形状错，但它被当成模式用时才是参数**。
- ④ **为什么走 `DataValidationError` 而不是让它传播**：`re.PatternError` 的 MRO 实测 `['PatternError', 'Exception', 'BaseException', 'object']`，不含 `ValueError`；`api/deps.py:494` 的 `to_http_error` 只对 `ValueError` 回原文 400，其余一律 500 固定文案 ⇒ 直接传播会把「调用方写错了」报成服务端故障，还丢掉那句能 actions 的中文判决。`DataValidationError` 同继承 `AugmentorError` 与 `ValueError`（`exceptions.py` 里既有那一格），所以 SDK 层是域错误、HTTP 层是 400、CLI 层经 `main()` 的 `except Exception` 变成 `错误: …` + 退出码 1，三面同源、不写第二套判据。
- ⑤ **净零行与活引用前置量法**：动手前 `grep` 全仓对 `search_enhanced.py` 的活引用，最大落点 491 与 491-553 都在改动点（565 起）之上 ⇒ 插行会平移它们，所以本次同行改写并把改动压成 **10/10 净零行**；副作用是改前改后各 10 条语句、分支数不变 ⇒ `TOTAL 14382 0 3952 0` 那六个数一格未动，覆盖率两维残差仍为 0（延续 L266 第一次「两维同时无残差」的状态）。`docs/augmentor/API.md` 同样同行追加（净 0 行），因为该文件的活引用是 244 / 282 / 952 / 1139，搜索端点那一行在 599——插在它上面会把 952 与 1139 推走。
- ⑥ **新守卫三面与零 parametrize**：`tests/unit/test_user_supplied_pattern_l267.py` 共 12 支。名册面写成集合精确相等（`(路径, 函数名)`）而不是「不许出现 X」的全仓扫描，配一支把每格源码锚点`count == 1` 的断言——名册绿不代表那一格还钉在它被定性的位置。行为面钉住四处：坏模式抛域错误且消息同时含模式串 / `re` 的原话 / 位置；空语料也拒（参数校验先于数据面）；`search_dataset` 入口同样出声；合法模式与 `contains` 的坏串语义各自不变。
- ⑦ **吞异常检测器的零写盘自证**：尺子读的是「`except re.error` / `re.PatternError` 的处理器体内有没有 `raise`」，出厂面实测 0 处。恒真风险由一支内存注入臂否决——把改前那 4 行在内存里喂给检测器拿到 `[(3, 1)]`（第 3 行的处理器、体内 1 条语句），再把同一形状改成体内 `raise` 必须拿到 `[]`。这一支不落盘、不碰仓库文件，因此不会在门禁运行期间污染被守卫读取的面。
- ⑧ **注入取证 3 臂**（逐臂改写 → 定向复跑 → 逐字节复原并断言 sha256 与进场值相等）：臂 1 把 `raise DataValidationError(...)` 换回 `logger.warning(exc)` ⇒ **8 支红**（本守卫的吞异常尺子 1 支 + 行为面 5 支 + `tests/unit/test_search_enhanced.py:116` / `:300` 两支旧契约用例）；臂 2 往 `api/routes/dataset_tools.py` 尾部追加一个永不调用的模块级 `re.compile(query)` ⇒ **1 支红**（只有名册那一支，行为面全绿 ⇒ 证明普查面与行为面各管各的）；臂 3 保留 `raise` 但把消息里的模式串删成固定短句 ⇒ **2 支红**（消息面与 HTTP 面）。三臂复跑读数 8 / 1 / 2 与首次逐字节一致。
- ⑨ **`validation.py` 那一格凭什么不修**：它的模式来自 `DatasetValidator.PRESET_RULES`，HTTP 与 CLI 两侧都只传 `preset=`（路由 `api/routes/dataset_tools.py:452` 同组的校验端点请求体只有 `preset`，CLI 走 `--preset`），`rules=` 是进程内形参、没有调用方通道；新守卫逐字段核过 `ValidateRequest.model_fields` 里既无 `rules` 也无 `forbidden_patterns`。它的非法模式**本来就出声**——`test_validation.py` 钉着「构造时不抛、第一条数据才抛」的惰性时机，本轮不改它，也不把那支既有断言改松。
- ⑩ **前端面零改动的依据**：`web/src/services/api.ts` 的响应拦截按「4xx 的 `detail` 是可 actions 的判决」取字符串 detail 原样上抛，所以那句 400 中文判决不需要新增翻译层；页面本来就把错误显示成错误，而不是显示「0 条结果」⇒ 本轮 `web/` 零改动，前端三门禁（typecheck / lint / test）**不适用**，不是漏跑。
- ⑪ **新立 A345（待拍）**：本轮只把「坏模式」这一格接上判决，没有动「模式复杂度」这一格——那是三个候选口径的取舍（模式长度 + 嵌套量词黑名单 / 请求级超时 / 换线性引擎），都会改对外契约或依赖面，按循环纪律立账而不是顺手挑一条实现。实测证据进表行：`(a+)+b` 在 `"a"` 重复串上 n=14 / 16 / 18 / 20 / 22 的耗时 0.0008 / 0.00316 / 0.01324 / 0.05305 / 0.20834 s（每 +2 字 ×4），`SearchRequest.query` 收 200,000 字不设限。
- ⑫ **自指落点**：L232 那句 operative 按现量 **829 → 832**。归因：本轮清单行 +1、A345 连带它的分隔空行 +2 ⇒ +3；链尾数字与句首那一格同批改写，落点以写回复盘的行号为准，不是算出来的。A333 那句「这一形没有尺子」至此第三次拿到实测实例（仍待拍）。
- ⑬ **净行数与落盘形状**：本账 4778 → **4798** 行（净 +20 = 清单区 +1、Backlog 区 +2（表行与它的分隔空行）、L232 自指两处同行改写净 0、日志块自身 +17 = 前置空行 1 + 标题 1 + 段内空行 1 + 要点 14），日志块只追加到文末 ⇒ 不动任何历史行号引用。本批第一版有两处形状缺陷——清单行插成了「行 + 空行」、块内要点之间留出空行——由修复脚本按现量收回；**而那个修复脚本自己犯了同一形**：它的锚点按「全文第一个命中」定位，落到 L263 块那条同前缀要点上（现读第 4718 行），把上一轮的三段数字改写成本轮的，处置是按 HEAD 现读把那一行逐字节还原。卷二按字节现读 `CR 1 / LF 4798 / CRLF 0`（那枚 CR 是散文里的内容字节，不是行尾）⇒ 写回断言只能用 `b"\r\n" not in out`。
- ⑭ **门禁读数**：代码批（`4b3854f47`）落盘后的全量定标（逐字 `python -m pytest tests -q -p no:cacheprovider`）**9374 passed / 0 failed / 10 skipped in 363.61 s**，覆盖率逐字 `TOTAL 14382 语句 / 0 缺数 / 3952 分支 / 0 偏支` 与 `Total coverage: 100.00%`；9374 = L266 终态 9362 + 本轮新守卫 **12** **恰等** ⇒ 没有一支被删或被改名绕开。六把账本与文档守卫在代码批落盘后 **154 passed / 8 skipped**（l79 那 8 格是作者工作树专属跳过，历史同形）。
- ⑮ **记账批（本条所在提交）读数**：账本批（`4d29342cf`）落盘后四把账本文本尺合跑首跑 **11 failed / 86 passed**，十一格失败逐格归因：l242 两格（表行面 `MEASURED_ROWS` 218 → **219**、区段对照那一格跟着逼出同一张数 265 == 219 + 46）、l253 两格（行面 `MEASURED_ALL_ROWS` 264 → **265**、字母形 `M` 42 → **43**，新账那一格是裸字母待拍、整行不带闭合词 ⇒ `MEASURED_CLOSURE_CELLS` / `MEASURED_WIDE_ROWS` / 名册与组合三面一格未动）、l263 一格（三面闭合 ⇒ `MEASURED_NO_VERDICT_ROWS` 98 → **99**，那一格既无标记也无带轮号的判决语，落「无判决语」档）、l264 六格（站点 116 → **117**、`same_round` 67 → **68**，其余四档 `cross_named` 10、`cross_unnamed` 恒 0、`no_round_subject` 16、`not_a_commit` 21、`stash_or_wip` 2 一格未动 ⇒ 本批新增的那一处站点没有一格落进缺陷档，那格的恒 0 仍是量出来的而不是推的）。**本条自己也在面里**：它点名账本批那枚哈希，而块号与提交主题同号 ⇒ 站点再 +1、`same_round` 再 +1，终态 118 / **69**，重钉按这一格现量。六格重钉全部走各自 docstring 的处方，**没有任何一把尺子被改松**，也没有一格是把红读数当噪声绕开；重钉后四把合跑 **97 passed**，六把账本与文档守卫复跑 **154 passed / 8 skipped**（与代码批落盘后**逐字同读数** ⇒ 本轮账本改动不进 l98 那六个数）。另记两格本轮自撞：账本批第一版的修复脚本把 L263 块那条 ⑬（现读第 4718 行）按「全文第一个命中」当成 L267 的那条改写，处置是按 HEAD 现读逐字节还原，见 ⑬ 末句；同批 L232 的 operative 先钉成 833 而复盘行号是 832 ⇒ 按现量同行改回，净 0 行。重钉后十把账本与文档守卫合跑 **251 passed / 8 skipped**（那 8 格 skip 全是 l79 需要作者工作树的定向档，历史同形）。**全量定标**（逐字 `python -m pytest tests -q -p no:cacheprovider`）在本批全部改动落盘后复跑：**9374 passed / 0 failed / 10 skipped in 395.01 s**、覆盖率逐字 `TOTAL 14382 语句 / 0 缺数 / 3952 分支 / 0 偏支` 与 `Total coverage: 100.00%` ⇒ 与代码批那一次（⑭ 记的 363.61 s）只差 wall，**用例数与覆盖两维一位未动**——本批只改账本文本与四把尺子的现量常数，收集面本就应当未动。补进这一读数与 251 那一格之后又复跑十把账本与文档守卫，读数**逐字同上** ⇒ 末句的同句改写（净 0 行）没动任何一面。

### L268（2026-10-14）— 账本自指那一形「本账第 N 行」第一次有尺子：收 A333 的 ③ 那一形，①② 两形移交新立 A346

- ① **这一轮收的是哪一格**：A333 立账时点名 a184 内容维判不到的三种形状，本轮落它自己给的修法 ③——「把这一形做成逐格点名的复核清单」。被抓的现场是账本自己的散文：L232 清单行里那句自指——形状是「本账第 + 数字 + 行」，后面点名一条 `docs/augmentor/API.md` 的行号引用——它主张的是**本卷自己的行号**，而 a184 只量带文件名的 `file.py:NNN` 落点，裸行号它结构性看不见，于是历轮每次在清单区或 Backlog 区插行都得靠人眼把它平移一次。L267 的账本批就当场错过一回：先钉成 833，复盘按现量改回 832，同一行括号链里那一格归因至今还写着 833 而值列表末位写着 832。
- ② **面怎么定义**：站点 = 匹配 `本账第 (\d+) 行` 的物理行，同一行多处按多处算（`\d+` 而不是 `[0-9]{1,4}`：位数上限是一种「以后不用再改」的假设，而这一格要的恰恰是每次都能被量到）。两卷现量 **13 处**，全部落在卷二；卷一 0 处，但卷一有 **8** 处序数形状「本账第 6 次 / 本账第 **5** 次 / 本账第三次」，由 `test_the_ordinal_shape_is_not_a_line_claim` 钉成「序数在场而行号档为 0」——那一条是防「面 0 其实是短语根本不存在」的假证据。序数正则的窗口 `[^行、\n]{0,12}?` 是量出来的：把上界收成 6 会把「本账第 **5** 次」那一格漏掉（现量从 8 掉到 7，本轮首跑就是这么红的，按现量放宽到 12 才对上）。
- ③ **三档判据与闭合**：`LIVE 1 / STALE 5 / BLIND 7 = 13`，判据全部是可复算的字面形状——同物理行找得到 `文件名:行号` 锚、且锚确实出现在主张的那一行内容里 ⇒ LIVE；有锚但那一行读不到 ⇒ STALE（历轮当时的现量，按纪律不追改）；根本没有锚 ⇒ BLIND。**BLIND 这 7 格是 A333 剩下的机械判不到的那一半**，本守卫不把它交回印象，而是逐格点名（`test_the_blind_tier_is_derived_not_asserted` 反向验：BLIND 的每一格 `refs` 必为空、非 BLIND 的每一格 `refs` 必非空）。`test_every_claim_targets_a_real_line` 另钉一格：所有主张的行号都落在卷内，「写飞」与「陈旧」是两种病。
- ④ **站点身份为什么不用行号当键**：用「第几行」当名册键，会被下一轮的插行钉成陈旧读数——那正是本尺子要抓的缺陷形，等于把尺子自己放进被测面。所以键是 `(卷, 区段, 轮号, 主张的那一行)`：区段取站点自己所属的轮号（清单行按 `^- \[[ x]\] \*\*L(\d+)\*\*` 取自己的轮号，日志块按所属 `^### L(\d+)` 标题取），它随轮次固定、不随物理行号平移。代价 A333 当初就写明并本轮兑现：名册随账本变长而必须逐轮更新，本轮账本批落下 3 行 ⇒ 名册里 operative 那一格从 832 走到 835，`test_the_roster_is_exact` 第一次就红。
- ⑤ **operative 用内容定位而不是抄散文**：两枚标记是 `DiffResult` 与 `docs/augmentor/API.md` 的第 1139 行那条引用，**全卷同时命中它们的物理行现量唯一**（写本批时按现读复核仍是唯一，命中行就是那条讲 `modified_count` 空转断言的日志行）。`locate_operative_target` 找不到或多于一个都直接 `AssertionError`——定位器自己也要有牙齿，否则「独立复算」只是一句自述。`test_the_operative_number_matches_the_independent_locator` 因此是「散文写的数 == 按内容重算的数」，不是把散文抄第二遍。
- ⑥ **五臂自证（全部内存改写、零落盘）**：off-by-one（把 832 改成 833）⇒ `LIVE` 掉到 0、`STALE` 走到 6；在目标之上插一行 ⇒ `LIVE` 掉到 0，这一臂就是「上一轮插行把自指平移而散文没改」那一形的机械版本；把锚那一格内容改成别的文字 ⇒ 同一个数字不再为真，证明判据读内容不读数字；新增一条无锚自指 ⇒ 面从 13 走到 14 且名册对称差恰好点名新增那一格；新增一条序数 ⇒ 面一格不动，证明判据不是「凡 `本账第` 都算」的五面通吃。五臂都走 `patched()` 在 `load_volumes()` 的返回值上改写，仓库文件一格未动，臂与臂之间互不共享行表。
- ⑦ **为什么零 parametrize**：`test_parametrize_ids_l97`（438 / 317）与 `test_parametrize_blind_face_l98`（盲面 116 / 四档 61-38-11-11 / 豁免 25）把全仓 parametrize 三面冻成现量数。本守卫的逐格断言走名册字典循环，一支 parametrize 都不需要，那两把普查尺子一格未动。
- ⑧ **收 A333 的口径与移交**：A333 的定级格按账本既有词汇收成「划掉字母 + 已收口（轮号）」，正文同步补「③ 已落尺、①② 仍在案」的处置段；剩下两形**不是**留在 A333 里等下一轮——它整行已经带闭合标记，再挂半程就是 l263 描述的「正文说了而定级格没说」那一族的反向，所以按 l242/l263 已有的「移交新立」惯例落成 **A346**（`| A346 |` 一行入 Backlog A 表，定级格是裸字母、待拍）。清单行与日志标题里 A346 一律写成括号形状，避开 l242 口径 5 的 ≤20 字符粘字窗口，**不进声明面**：声明面本轮只多 A333 一格（95 → 以守卫红读数为准）。
- ⑨ **写本批前先做的两次现读**：一是锚点形状——A333 / A345 / Backlog B 标题 / L267 清单行各自按前缀 grep 出唯一命中，再断言 `count(old) == 1`，不凭记忆写锚点（L267 那轮修复脚本就是凭「全文第一个命中」把 L263 块的 ⑬ 改成本轮数字的，本轮按前缀定位而不是按散文前缀定位）；二是行尾——卷二按字节现读 `CR 1 / LF 4799 / CRLF 0`，那一枚 CR 是散文里的内容字节而不是行尾，所以写回断言只能是 `b"\r\n" not in out`，且**所有行号算术一律走 `read_bytes().decode("utf-8").split("\n")`**：文本模式读会把那枚内容 CR 翻译成一个额外换行，行号当场错一格。
- ⑩ **净行数与落盘形状**：本账 4799 → **4817** 行（净 +18 = 清单区 +1、Backlog 区 +2（A346 表行与它的分隔空行）、L232 那一格 operative 与链尾两处同行改写净 0、日志块自身 ++15 = 前置空行 1 + 标题 1 + 段内空行 1 + 要点 12），日志块只追加到文末 ⇒ 不动任何历史行号引用。对活引用的处置：L232 那句 operative 自指是本卷唯一「现在必须为真」的行号主张，它之上落了 3 行 ⇒ 按 ⑤ 的内容定位器现读重钉，不用算术推。
- ⑪ **门禁读数**：代码批（`117517d22`）落盘后的全量定标（逐字 `python -m pytest tests -q -p no:cacheprovider`）**9388 passed / 10 skipped / 0 failed / 6 warnings in 249.53 s**，覆盖率逐字 `TOTAL 14382 语句 / 0 缺数 / 3952 分支 / 0 偏支` 与 `Total coverage: 100.00%`（守卫文件不在覆盖率测量面内，故语句数不动）；9374 + 14 = 9388 与上一轮总数逐格衔接，没有任何一把尺子被改松。前端三门禁**不适用**（本轮 `web/` 零改动）。
- ⑫ **待拍与下一步**：本轮不拍的产品缺口，全部留在册待用户拍板——**A344**（`api/routes/*` 的写盘端点失败形状）、**A343**（or-默认吞空容器一族 10 格）、**A345**（正则模式串无上限与嵌套量词，L267 立账）、**A346**（本轮新立，a184 内容维的 ①② 两形）。另有 A333 移交出来的两形要不要动 a184 判据面，与「名册型守卫随账本变长」这一族的整体代价，都需要用户口径而不是代理择定 ⇒ 详见循环日志 L268。
- ⑬ **记账批（本条所在提交）读数**：账本批（`fbc455bf4`）落盘后，五把账本文本尺合跑首跑 **25 failed / 86 passed**，本条要点入面后再跑一次仍是 25 格（l264 那两格随站点面一起走），逐把按守卫打出来的红读数重钉、没有一把尺子被改松：**l242** 表行 219 → **220**、声明面 95 → **96**（A333 第一次进声明面：清单行与本轮日志标题各点一次，同一只 ID 只算一笔；A346 按口径 5 写成括号形状 ⇒ 不进声明面，`gap` 一格未动）、有标记 112 → **113**；**l253** 表行三面 265 → **266** 与 219 → **220**、定级格闭合面 112 → **113**、宽面 115 → **116**（两面之差仍是 3、`TWO_FACE_ROSTER` 一字未动 ⇒ 两把尺子第六次对上）、字母形 `~~M~~` 42 → **43** 而裸 `M` 仍是 43（A346 带进一格、A333 交出一格，两数恰好同抵——这一格没动本身就是「只收一格又只立一格」的证据）、闭合词「已收口」38 → **39**（本轮散文刻意不复用闭合词，日志要点里那次是名册形状字面、不进这一面，所以恰好 +1）、组合名册 `('~~M~~', '已收口', 'L', False)` 8 → **9**（**没有造出第 20 种形状**）、同卷可查 99 → **100**；**l263** 只有表行一格动（219 → **220**），三档闭合仍按 116 + 5 + 99 = 220 成立——A333 带着标记走进宽面、A346 带着裸字母走进「无判决语」，两档各 +1 与 −1 在同一格抵平 ⇒ 盲区 5 格与 `MEASURED_HITS` 名册一字未动；**l264** 站点 118 → 119 → **120**、`same_round` 69 → 70 → **71**（⑪ 点名代码批那一枚、本条点名账本批那一枚，两枚主题与块号同号，其余四档一格未动 ⇒ `cross_unnamed` 的恒 0 仍是量出来的）。**本守卫第一次咬到自己，而且咬了两处不同的**：`test_the_roster_is_exact` 打出 `('...', 'checklist', 232, 835)` 对 `(..., 832)` 的对称差——那一格**按纪律手工重钉为 835**，因为「名册随账本变长而每轮要更新」是 A333 修法③ 自己点名的代价，派生掉它就等于把「散文没重钉」这一格从名册面上抹掉；另外三支臂里，**两支是锚写死导致的假形状**（off-by-one 臂的锚文字与 scrub 臂的物理下标仍指旧行号，臂变成空转），改为按 `locate_operative_target` 现读——两臂的断言逐字不变（`LIVE` 掉到 0、`STALE` 走到 6），牙齿没动；新增无锚自指臂与序数负向臂只是被旧名册牵连，名册重钉后自然复绿。重钉后五把合跑 **111 passed**；六把账本与文档守卫 **154 passed / 8 skipped**（账本批落盘后、记账批改写后再跑，两次逐字相同）；全量定标 **9388 passed / 0 failed / 10 skipped / 6 warnings in 399.18 s**（用例数与代码批那一次逐字相同：9374 + 新守卫 14 = 9388 ⇒ 记账批一支用例未增），覆盖率逐字 `TOTAL 14382 语句 / 0 缺数 / 3952 分支 / 0 偏支` 与 `Total coverage: 100.00%`。**本条第一版把两处反引号跨度写空了**：填读数用的是 bash 内联 `python -c`，反引号被 shell 当命令替换吃掉（`TOTAL` 与 `Total coverage` 各报一次 `command not found`），脚本自己的 `residual=False` 只查了占位符没查内容 ⇒ 落盘后按现读发现、由脚本文件重写这一行修回（同行改写净 0 行）。这与 L267 那一批里「内联写反斜杠被吞成双份」的缺陷同一族，本轮起的口径：**凡带反引号或反斜杠的账本散文一律走脚本文件，不走 bash 内联 `python -c`**。本轮不拍的产品缺口继续在册待用户拍板：**A344**、**A343**、**A345**、**A346**（本轮新立，a184 内容维的 ①② 两形）。

### L269（2026-10-14）— 两处「写坏面」同轮接线：鉴权比较的 ASCII 限制、配置的整份截断写；只读安全普查把 A341 复现并新立 A347

- ① **选题依据**：本轮先把产品码的**安全形状**扫干净（只读普查，未动任何产品码之前先量），再把其中两处「代码把坏事写进磁盘 / 把判决写成崩溃」的边接线。普查分五族走：路径与数据根白名单、鉴权、写盘原子性、外部命令执行、标识符清洗。
- ② **五族读数**：外部命令执行族**零命中**（产品码里 `subprocess` / `os.system` / `shell=True` / `os.popen` 全仓无一处，命中只在 `scripts/inject_defect_check.py`、`scripts/mutation/run.py` 与 `tests/`；亦无 `eval` / `exec` / `pickle` / `yaml.load`，配置读侧用 `yaml.safe_load`）；标识符清洗族在 L258 / L259 已接权威（`require_within_dir` 与备份名的 `.parents` 判据），本轮量到的余面是**字符集那一维**（空白名 `"  "` 是 truthy 会直接当 `backup_id` 落盘、NUL 一路走到 `resolve()` 抛 `ValueError` 而 `_safe_data_path` 的 `try` 不覆盖它 ⇒ 500 而不是 400、超长名与 Windows 保留名无上限），这一维**本轮只量未动**，与 A344 的「失败形状」同批处置更省一次对外契约评估；路径族与鉴权族各产出一处本轮接线与一条在册/新立（下面 ③④⑤）。
- ③ **鉴权那处的形状**：`secrets.compare_digest` 对 **str** 只接受纯 ASCII，非 ASCII 当场 `TypeError`（本机复算：`compare_digest('ÿ', 'k')` 抛 `TypeError: comparing strings with non-ASCII characters is not supported`）。HTTP 头按 latin-1 解码，所以线路上任意 0x80–0xFF 字节都会成一个合法字符；抛点在 FastAPI 的依赖解析里，早于路由函数体，路由自己的 `except` 结构拦不到，异常一路冒到框架。两档后果：**客户端**用一个非 ASCII 头能把已配密钥部署的每个写端点稳定打成 500；**运维**把密钥本身设成非 ASCII 时连逐字正确的请求也 500，鉴权面整体不可用。改法是字节档比较——「两个字符串相等」与「两者的 UTF-8 字节相等」是同一件事，等值语义一字未动。
- ④ **配置那处的形状**：`save_config` 是 `open(config_path, "w")` + `yaml.safe_dump(data, f)`。截断发生在 `open` 那一刻，于是整个序列化窗口里盘上是半份文件；读侧 `api/deps.py` 对 `OSError / YAMLError` 的那一档回落成 `{}`，即「坏内容 ⇒ 出厂默认」= 判据口径里的**错误答案档**。它与 A182 的关系要说清楚：**不是豁免，是从来不在面上**——A182 的普查尺数「`open(...,'w')` 紧跟 `json.dump`」，YAML 写边不在其内；本轮把这一边接进同一把工具（`atomic_write_text`），不扩任何既有判据、不改任何回执形状。
- ⑤ **零漂移怎么钉的**：不去复刻合并逻辑（`save_config` 里有「读现有文件 → 合并未识别键 → 密钥占位符回填 → `logging` 节 pop」四段，手抄一份期望值只会造出第二份会过期的账），而是把 `yaml.safe_dump` 的**第一个实参**在调用现场截下来，用改前写法（text 模式直写）把同一份 `data` 重放到第二个路径，再逐字节比。实测两侧字节相同（本机 Windows：186 行 `\r\n`，3 470 B）。**行尾这一维单独一支并如实分档**：改前走 text 模式由平台翻译行尾，`atomic_write_text` 用 `newline=""` 不翻译，所以代码里显式做了 `replace("\n", os.linesep)`；Windows 上摘掉它就写出不同的字节，POSIX 上 `os.linesep == "\n"` 该步为空转——不把「它有时空转」写进判据，后来人会把跨平台相等误读成尺子失灵。
- ⑥ **两面尺子为什么都下到 ASGI**：`TestClient` 走 httpx，它先把请求头按 ASCII 编码，非 ASCII 在**客户端**就抛 `UnicodeEncodeError`（本机实测两档：`'ÿx'` 与 `'é'` 都发不出去）。如果守卫只用 TestClient，本轮会写出一条假结论——「这一形 HTTP 面打不出来，只是 SDK 缺陷」。真实服务器（h11）按 latin-1 解头，所以守卫构造原始 ASGI scope、把 `b"\xff"` 直接喂进 `headers`，取 `http.response.start` 的状态码：改后 401，削弱臂（依赖换成旧写法复刻）异常冒泡且**一格响应都没写出**。
- ⑦ **削弱臂的教训一支（本轮真实返工，不美化）**：第一版把依赖换成旧写法时写成 `lambda x_api_key=None: ...`，FastAPI 按签名把 `x_api_key` 当**查询参数**绑，头根本没进比较，臂「不红」的原因与被测缺陷毫无关系。补上参数标记 `Header(None, alias="X-API-Key")` 之后臂才真的以 `TypeError` 失败。**记法**：往框架里注入替身函数时，替身的**签名形状**也是被测面的一部分。
- ⑧ **守卫的两处口径修正（同样是本轮真实返工）**：站点普查第一版按「属性名是 `dump` / `safe_dump`」扫，把全仓 `json.dump` 一起算进 YAML 面（现量 3 处而不是 1 处），改成「属性名 + 表达式根必须是名字 `yaml`」才对；形状面第一版断言「`save_config` 体内一次 `open` 都没有」，实测它是 2 处——读现有配置做合并那一条本来就用 `open(path, 'r')`，于是判据换成**按打开模式计数**（写模式 0、读模式 1，后者现量钉住，防后来人把合并读边一起「收干净」）。
- ⑨ **插行纪律的用例**：`api/deps.py` 的改动刻意压成**同行改写、净 0 行**（`git diff --numstat` 读数 2/2），因为账本对该文件有两处活引用 `:57` 与 `:466`；第一版写了 5 行注释把它抬到 +5，a184 硬 0 档当场报 3 处 `dead_line`（其中两处就是这两条引用），压回净零并**把 WHY 挪进函数 docstring 的同一行**之后 a184 与 l79 全绿。`augmentor/config.py` 净 +10 行：先 grep 全仓对该文件的活引用，读数最大的是 `:973`，全部落在插入点之下 ⇒ 不平移，实测六把守卫零红点。
- ⑩ **本账自指行号按现量重钉**：清单区 +1 行、Backlog 区 +2 行都住在 835 之上，L232 那一格 operative 由 835 改钉为 **838**，括号链尾补第二十一个值（改完全量现读链内 21 个值、尾值 838），同一格的**逐轮归因链**补上本轮那一笔「L269 的清单行 +1 与 A347 连带它的分隔空行 +2 ⇒ 落到 838」，三处都是同行改写、净 0 行；新写的清单行与日志块里**不出现**「本账第 + 数字 + 行」那一形，也不在同一行同时写下两个定位标记（`DiffResult` 与那条 `docs/augmentor/API.md` 活引用），否则 l268 的内容定位器不再唯一。
- ⑪ **门禁读数**：新守卫定向 17 + 11 全绿；六把账本与文档守卫 **154 passed / 8 skipped**（与 L268 终态逐格相同）；全量定标 **9416 passed / 0 failed / 10 skipped / 6 warnings in 348.56 s**，`TOTAL 14383 语句 / 0 缺数 / 3952 分支 / 0 偏支`、`Total coverage: 100.00%`。用例闭合 = 9388 + 28，**恰等** ⇒ 没有一支被删或被改名绕开。全部写盘用例都在 pytest 的 tmp 目录里，仓库那份 `config.yaml` 从头到尾没被打开过（探针前先记 sha256 的做法本轮不需要，因为没有`TestClient` 打写盘端点的臂）。
- ⑫ **待拍与下一步**：本轮不动的三格留在册——**A341**（配置字段旁路数据根白名单，L259 立、本轮第二次量成现量）、**A347**（本轮新立，写端点默认匿名可达，是既有明文策略而不是写错）、标识符清洗的**字符集那一维**（空白名 / NUL / 超长名 / 保留名，本轮只在 ② 记读数，它与 A344 的失败形状同批处置更省一次对外契约评估）。已经拍过才动的两格是：鉴权比较的 ASCII 限制（docstring 自己写着 401 那一档，代码写成会崩的形状）与配置整份写（`atomic_write` 的判据口径自己点名的错误答案档）——两格都不是新决策。
- ⑬ **记账批读数（五把账本尺首跑红 8 格、分四把，逐格按各自红读数重钉，没有任何一把被改松）**：l242 两格红同源于一把常数（`MEASURED_ROWS` 220 → **221**，红读数 `assert 221 == 220`，同批由区段对照那一格逼出 `assert 267 == (221 + 46)`）；l253 两格红 + 一处级联（`MEASURED_ALL_ROWS` 266 → **267**，它之后的 `MEASURED_BACKLOG_ROWS` 220 → **221** 是同一次红里没走到的下一步，`MEASURED_LETTERS["M"]` 43 → **44**）；l263 一格红逼出两格重钉（`MEASURED_BACKLOG_ROWS` 同上到 **221**，`MEASURED_NO_VERDICT_ROWS` 99 → **100**，闭合式 `116 + 5 + 100 == 221`）；l268 三格红同源于名册那一格（`ROSTER` 里 operative 由 835 改钉 **838**，`test_the_roster_is_exact` 与两支注入臂一起红）；l264 全绿。**三面读数一致对外**：A347 那一格定级格是裸 `M`、整行不带 `MARK` 读得到的闭合语、也没有「轮号在动词之前」的盲区形状 ⇒ 权威面 113、宽面 116、盲区 5 三格一格未动，只走「无判决语」与「裸 M」两档，两把尺子的面差仍是 **3**。**同批的第二处陈旧**：l268 那条用例的 docstring 写着「（L268 账本批之后现量 835）」，散文跟着名册一起改成 838 ⇒ 尺子自己的说明与案面同读法，留旧数就是给自己留下一格没人量的陈旧。重钉后五把 **111 passed / 0 failed**（9.86 s），六把账本与文档守卫复跑 **154 passed / 8 skipped**（9.30 s），两批读数与代码批逐格相同 ⇒ 记账批没有把任何一把尺子改松，也没有制造新的陈旧。

### L270（2026-10-14）— 全账第一次问「这枚提交还在不在分支上」：A344 的 ① 立尺结案，重钉口径与新量出的两形盲区拆 A348 / A349 / A350 继续待拍

- ① **选题依据（代理自开轮的边界）**：本轮零产品代码，只做一件事——把 A344 原格里**已经写定**的修法 ①（「给尺子加一维『点名的提交必须是祖先』」）落成守卫。凡是改响应面 / 改状态码语义 / 加字段的一律等用户拍板，本轮一处都没碰；A344 的 ②（重钉口径）也不替用户拍，拆成 A348 继续在册。
- ② **面的定义**：两卷账本里所有被反引号包住的 `[0-9a-f]{6,40}`（允许一枚尾随省略号），形状尺直接 import l264 的 `HEX`——两把尺子各写一份就会在下一轮漂出两种面。按 `## 循环日志` 边界分两区：**现量 634 处** = 卷一清单区 112 + 卷一日志区 277 + 卷二清单区 120 + 卷二日志区 125；唯一 hex 379 枚（处数与枚数是两个维，同一枚在一行里被点名三次是账本常态）。
- ③ **五档与闭合等式**：reachable 459 + dangling 26 + stash 2 + not_a_commit 147 + ambiguous 0 = 634，分档之和等于总数由 `test_the_five_tiers_partition_the_face` 逐轮复核（漏一档就加不回去）。`ambiguous` 现量**空集**：一枚前缀同时匹配两枚提交对象就红，不豁免——库里 3057 枚 commit 对象，六到九位前缀的撞车概率不是零，这一格必须是有牙齿的而不是没人看过。
- ④ **判据为什么换成全 ref 可达**：A344 原文写「必须是 HEAD 的祖先」。本机共享工作树里并行会话会把 HEAD 切走（L265 开工时在 `main`、L266 块 ⑮ 记着那一次 8 格接线不在 HEAD 上），拿 HEAD 当判据就会把「别人切了分支」报成「账本写错哈希」，一次切分支顶红四百多格。本轮实测两判据在现面上**给同一批 7 枚对象**，所以换判据没有放松任何一格，而 `test_the_two_criteria_agree_on_the_current_face` 把这句话钉成对账维：两判据一旦分叉（例如某轮把只存在于别的分支的哈希写进账本）当场红，逼下一轮重新看这一格。
- ⑤ **7 枚不可达对象的逐枚归因**：账本在 L265 那两轮里点名过 `1094b0696`（账本批第一版）、`b9d3ad7b1`（被并行会话的提交吞走之后 amend 出的中间态）、`b2af89396`（账本批第二版）、`ee133fc40`（补交的主守卫，被 reset 重做出分支）、`930e31f74` 与 `adc0df075`（oa4rust 那两枚把本仓暂存区一并带走的提交），外加 L242 记录的共享工作树 stash 事故那枚 `82f4a4cab3e86d66abc06ac9cb60b7a87532144a`（主题以 `WIP on ` 开头 ⇒ 单独一档 `stash`，与 l264 同一口径）。本守卫的名册键是 `(卷, 区段, 轮号, hash)`，**本块重提这七枚就 ⇒ 名册随本轮长出 7 格**，这正是 A333 点名的「名册型守卫随账本变长」那一笔代价，也是 A348 那一格要不要把历轮重提压回单点的理由。**这一档带着显式的不确定性**：gc 跑过之后那枚 stash 对象会消失并掉进 `not_a_commit`，名册当场红，届时按守卫实读重钉而不是偷偷改档。
- ⑥ **与 l98 的分工（A344 那句「全账无人量」的前提）**：`test_ledger_hash_slots_l98` 的账本路径来自 l79 的常量，而那个常量指向**卷一**——它的 R1 确实要求「槽内每笔哈希都能解析成全祖先链里的提交」，但卷二清单区这 120 处从来不在它的面里。`test_l98_reads_only_volume_one` 现场把这两格钉住（路径名 == 卷一，且卷二清单区现量 120 处），所以「补这一维」不是重复劳动。
- ⑦ **与 l264 的分工**：它那 120 处站点是本尺面日志区的**真子集**（差集 282 处 = 卷一 277 + 卷二五个块标题行）。原因是它只认 `### Lnnn` 开块、且见到标题就 `continue`，而卷一的日志块标题不是那一形，一格都开不出来。对账按**处**算而不是按去重后的格算——同一行重提同一枚哈希在两把尺子里各算两处，第一版按 set 去重量出 278 处，是错的读数（⑨ 记着）。
- ⑧ **四条臂的读数（全部在内存里改判据或改面，零落盘）**：臂1 从可达面摘掉 L269 代码批那一枚 ⇒ 它那 1 处站点立刻从 `reachable` 走到 `dangling`、`dangling` 计数 +1 而 `reachable` −1、缺陷格恰好是它那一格，复原后 `sites()` 与基线逐格相等；臂2 把一枚**已在名册里**的游离对象换到名册没有的区段身份上 ⇒ 缺陷格非空，证明名册不是「游离的一律放过」；臂3 双向证明不带反引号的裸哈希不进面（同一枚加了反引号就多一处）；臂4 编造一枚哈希 ⇒ 落 `not_a_commit` 且缺陷格仍空，这一条不是洗地而是 A349 的量法。
- ⑨ **本轮真实返工两处，不美化**：① 区段身份第一版照 l264 只由 `### Lnnn` 开块，实测与 l264 的差集是 278 处而不是 282 处——差在五个块标题行本身（标题行被 `continue` 掉）与「按 set 去重吞掉同行重提」两处，改成 `round` 从清单行与块标题两者继承、另存 `block` / `on_head` 两格专供对账，并把对账改成按处算；② 第一版把卷一日志区写成 278 处，现读 277 处，多出来的那一处是我自己算错。两处都以守卫实读为准，没有任何一把尺子被改松。
- ⑩ **为什么零 parametrize**：l97 的 438 / 317 与 l98 的盲面 116 / 四档 61-38-11-11 / 豁免 25 把全仓 parametrize 三面冻成现量数，本守卫走名册字典循环，那两把普查尺子一格未动（定向合跑实测 315 passed / 8 skipped，另有 l98 那 5 支 error 是**单文件复跑的跑法形状**——它的会话级 fixture 在采集数低于 3000 时抛 `RulerNotProven`，与本轮无关，全量门禁里恒绿）。
- ⑪ **立账三行**：A348 = A344 剩下的 ②（那批游离点名的重钉口径，含「gc 之后整批掉档怎么办」）；A349 = 本尺 `not_a_commit` 档只数总量不逐格点名 ⇒ 写错一位的哈希永不被抓；A350 = 同一个编号在日志要点里被写成与表行不同的事（A344 实测三种互斥说法，其中「写盘端点失败形状」在全账没有对应表行），l253 的「同卷可查」只判在不在、不判内容。**三行都是待拍，本轮一律不动**。
- ⑫ **待拍与下一步**：本轮不拍的面全部留在册并点名给用户——**A348**（游离点名重钉口径）、**A349**（可达性尺的 count 档盲区）、**A350**（编号说法无对账尺）、**A341**（配置字段旁路数据根白名单，L259 立、L269 第二次量成现量）、**A347**（写端点默认匿名可达，L269 立）、**A343**（or-默认吞空容器一族）、**A345**（正则模式串无上限）、**A346**（a184 内容维两形）、以及标识符清洗的字符集那一维（空白名 / NUL / 超长名 / 保留名，L269 ② 只量未动）。前端三门禁本轮不适用（`web/` 一字未动）。
- ⑬ **记账批读数（账本批 `fe5836f8f` 落盘之后按实量重钉六把尺）**：本块与清单行自己就是新站点——清单行点名代码批那一枚 ⇒ 卷二清单区 120 → 121，本块 ⑤ 逐枚点名七枚不可达对象 ⇒ 卷二日志区 125 → 132 且名册随本轮长出 7 格（6 格游离 + 1 格 stash），记账批这一条再点名账本批那一枚（主题与块号同轮 ⇒ 只进 `same_round`）⇒ **l270 尺面 634 → 643 = reachable 461 + dangling 32 + stash 3 + not_a_commit 147 + ambiguous 0**，唯一 hex 379 → 381 枚，与 l264 的差集仍是 282 处（卷一日志区 277 + 卷二五个块标题行）。**l264 面 120 → 128**（`same_round` 71 → 72、`cross_named` 10 → 13、`no_round_subject` 16 → 19、`not_a_commit` 21 一格未动、`stash_or_wip` 2 → 3，缺陷档 `cross_unnamed` 仍 0 ⇒ 那格的恒 0 是量出来的而不是推的）。**四把普查尺同一批**：l242 表行 221 → 224、声明面 96 → 97（A344 第一次进声明面）、定级格面 113 → 114，两个配式 `224 + 46 = 270` 与 `97 + 38 = 135` 同时闭合；l253 全面 267 → 270、Backlog 区 224、权威面 114、宽面 116 → 117（两面之差仍是 3）、裸 `M` 44 → 46 与 `~~M~~` 43 → 44（那一格交回划掉字母、新立三笔各拿一格）、`已收口` 39 → 40、同卷可查 100 → 101；l263 表行 224 与闭合式 `117 + 5 + 102 == 224`（无判决语档 100 → 102：新立三笔 +3，A344 那一格带上判决语就离开这一档 −1）；l268 operative 自指按 ⑤ 的内容定位器从 838 重钉到 845（+7 行 = 清单行 +1 与新立三笔连带分隔空行 +6），散文写的数与独立定位的数由该守卫自己验等。**逐格按各自 docstring 的处方重钉，没有任何一把尺子被改松；⑨ 那两处真实返工留着不美化，本条之外没有第二处中间读数被抹平。**

### L271（2026-10-14）— 路径名那一维：操作系统判死的名字不该冒充服务端故障，而 L269 ② 记的三条归因本轮量成假话

- ① **选题依据与边界**：本轮动的这一格不改「哪些请求能成功」，只改「已经必然失败的请求报什么码」，且方向是既有 docstring 自己声明的那一档（`to_http_error` 的口径写明 400 用于参数或数据不合法、500 只兜未分类）。同族的其余三形（读侧 404、文案两形、跨平台字符集）不在本轮拍 ⇒ 立 A351 继续在册。
- ② **现量矩阵（写侧 = `POST /api/dataset/convert`，读侧 = `POST /api/dataset/stats`，数据根指向仓外副本）**：含 NUL 的名字——写侧 400 `embedded null character`、读侧 404；超长名（5000 字符）——**修前写侧 500**、读侧 404；含换页符的名字 `a\nb.json`——**修前写侧 500**、读侧 404；`CON`——写侧 200 且落的是普通目录项；空白名与纯空白——400「文件路径 不能为空」（闸的第一刀）；`..` 与 `a/../b.json`——400「路径包含非法组件」；`a:b.json` 与 UNC 前缀——403（越界那一档，与非法组件可区分）。
- ③ **异常类型才是分岔点**：同一族「名字坏了」在 `open()` 层分成两类——NUL 由 CPython 抛 `ValueError("embedded null character")`，走 `to_http_error` 第 3 支拿 400；超长与控制符由操作系统抛 `OSError(errno=22)`（本机实测超长名也折成 22 而不是 `ENAMETOOLONG`），落到「其它 → 500」那一支。**修法不是猜字符集**，而是直接读操作系统自己填的 `errno`：`EINVAL` / `ENAMETOOLONG` 进 400，`ENOSPC` / `EACCES` / `EROFS` 这类服务端状态照旧 500，`errno` 为 `None` 的注入型 `OSError` 也照旧 500。
- ④ **文案这一维不能照抄 ValueError 支**：`str(OSError(22, "Invalid argument", "D:/deploy/root/..."))` 的原文带着部署根绝对路径，而 A155① 建立的规则正是「500 不转发原文，因为原文可能带部署根与文件系统状态」。新档进的是 400，如果按第 3 支的「原文照回」写法就会**把 500 档防的东西从 400 档放出去**，所以用固定话术 `INVALID_PATH_NAME_DETAIL`。守卫里 `test_the_detail_never_carries_the_deploy_path` 钉这件事，第三臂把常量换成路径原文当场看到标记 ⇒ 这条断言不是恒真。
- ⑤ **判据住一处**：`api/deps.py` 里 500 的收敛有两个入口——`to_http_error`（6 个路由文件 45 处收尾）与 `raise_internal_error`（9 个路由文件 34 处收尾）。只改前者会留下「同一个坏名字在 augment / quality / export 那一族仍回 500」的第二形，所以两个入口共用 `is_rejected_path_name`（A77「一条泄漏判据住一处」同口径）。
- ⑥ **L269 ② 被量成三条假话**（逐条按现量，不美化）：①「空白名 `"  "` 是 truthy 会直接当 `backup_id` 落盘」——在路径闸这一径不成立，`resolve_within_roots` 第一刀就是 `not name.strip()`；②「NUL 一路走到 `resolve()` 抛 `ValueError` 而 `_safe_data_path` 的 `try` 不覆盖它 ⇒ 500 而不是 400」——`_safe_data_path` 是一行委托、根本没有 `try`，`resolve()` 对 NUL 不抛，写侧拿到的是 400，那句 500 从未发生；③「超长名与 Windows 保留名无上限」——超长名成立（就是本轮修的那一形），保留名不成立（`CON` 实测可 `os.stat` 可 `os.remove`，是普通目录项）。**L269 块的那段文字按纪律不追改**（A348 正是这件事的在册版），本轮只在 L271 块里留下现量与差异。
- ⑦ **四格活引用按处方 3 重钉**：`api/deps.py` 顶部新增 `import errno`（+1）与 `to_http_error` 之前的新常量与判据函数（+27）、docstring 补第 4 支（+4）⇒ `api/deps.py` 的 57 那一格现量 58、466 那一格现量 494（此处刻意不写成「文件:行号」反引号形：a184 把那一形当活主张，而这两个数讲的是重钉之前的旧落点，按主张量会双双落在非代码跨度上）；账本四格（清单行 105 / 107 与日志块 4798 / 4801）随行改动、**净 0 行**。这是 L217 / L219 / L270 之后的第四次同一缺陷类，而且这次是我自己往被引用的产品文件里插行——前置量法这次做了（动手前 grep 出全部`deps.py:NNN` 活引用），处置走的是处方 3 而不是换点，因为新增的是一个函数而不是几句话。**日志块里那句「两处活引用一格未平移」是 L269 当时的真话**，本轮把数字换成 58 / 494 只表示「现在指得准」，不是改写归因。
- ⑧ **守卫 14 支的形状**：`TestDispatchTable` 6 支（两枚 errno 进 400 / 文案不带路径 / 三枚服务端 errno 照旧 500 / 无 errno 的注入 OSError 照旧 500 / `FileNotFoundError` 虽是 `OSError` 子类但 404 未被翻掉 / 既有四支一字未动）、`TestPredicate` 1 支、`TestRaiseInternalError` 2 支、`TestEndToEnd` 2 支（走 TestClient，数据根在 tmp、残留由 conftest 逐用例盯）、`TestRulerBites` 3 臂（判据集合清空 ⇒ 回到 500；把 `EACCES` 塞进集合 ⇒ 服务端状态被吞成 400；文案换成原文 ⇒ 部署根出现在响应里）。**全部在内存里改，零落盘。**
- ⑨ **本轮真实返工三处**：① 新守卫第一版把 `exc.errno` 断在 `HTTPException` 上（`AttributeError`，HTTPException 没有 errno 这一维），改成对**输入**断言 `is_rejected_path_name` 才是要钉的那件事；② 探针脚本先用 heredoc 写，反斜杠被折半成 `\server\share` 之类导致 SyntaxError，改用文件写入工具落盘后才跑；③ 账本批第一次写完，A350 与 A351 两行之间漏了这张表约定的分隔空行 ⇒ markdown 结构守卫把 A350 读成一张新表的第一行（GFM 会丢掉表头）当场红。补回那一格让本批净行数 +20 → +21，连带 operative 自指从 850 移到 851，清单行 L232 的括号链尾与「本账第 N 行」那一格同批重钉。三处都是读数形状问题，没有尺子被改松。
- ⑩ **A352 这一格在本轮之内被搬迁方自己补完（候选 ③）**：本轮写作期间仓库根发生了一次跨项目文档搬迁（`augmentor/docs/*` → `docs/augmentor/*`、`docs/oa/*` → `docs/oa4rust/*`），`augmentor/docs/` 目录不再存在。我先按「改名还在共享索引里、没提交」把它立成待拍，当时的现量是**采集期就断**（`test_api_doc_field_hints_l223` 与 `test_config_three_side_l222` 在 import 期按旧路径读文档抛 `FileNotFoundError`），绕开那两支之后的诊断读数 **48 failed / 9289 passed / 10 skipped**，红的全部是按字面路径读文档的守卫与文档契约普查，产品码零依赖。**随后搬迁方把守卫面一并补齐并提交**（`e4ce1ae36` 测试的文档路径解析改指仓库根、`94f7f05cc` 重写项目内 390 处文档引用 + a184 普查面改指仓库根），本轮一行守卫路径都没改。收口证据取本轮那次全量门禁：采集期零 error、**9429 passed / 10 skipped / 20 failed**，20 红全是本批自己的账本常数与两处账本散文（表格分隔空行、旧行号被写成活主张形），文档路径面零红；先前那 48 红读数已被这次全量跑取代，留着不美化。**落刀前的定向基线**：六把账本与文档守卫加新守卫 = 168 passed / 8 skipped。
- ⑪ **立账两行**：A351 = 路径名那一维剩下的三形（读侧 404 / 文案两形 / 平台字符集），三形都要拍；A352 = 13 支文档面守卫的硬编码路径假设，**立账同轮由搬迁方按候选 ③ 补齐并提交**，本轮按全量门禁现量收成划掉字母（落地者不是本账，归因写在正文里）。
- ⑫ **待拍与下一步**：本轮不拍的面点名给用户——**A351**（路径名余下三形）、**A348**（游离点名重钉口径）、**A349**（可达性尺的 count 档盲区）、**A350**（编号说法无对账尺）、**A341**（配置字段旁路数据根白名单）、**A347**（写端点默认匿名可达）、**A343**（or-默认吞空容器一族）、**A345**（正则模式串无上限）、**A346**（a184 内容维两形）。A352 **本轮出名单**（立账同轮由搬迁方按候选 ③ 落地，见 ⑩）。前端三门禁本轮不适用（`web/` 一字未动）。
- ⑬ **记账批读数（账本批 `cd47d9845` 落盘之后按红读数逐格重钉六把尺，没有任何一把被改松）**：本块与清单行自己就是新站点——清单行点名代码批那一枚、A352 表行与本块 ⑩ 各点名搬迁方那两枚⇒ **l270 尺面 643 → 649 = reachable 467 + dangling 32 + stash 3 + not_a_commit 147 + ambiguous 0**，唯一 hex 381 → 385 枚，卷二清单区 121 → 124、日志区 133 → 136（本条自己就是新站点，与 L270 块 ⑬ 同形），与 l264 的差集仍是 282 处（卷一日志区 277 + 卷二五个块标题行）；`test_l98_reads_only_volume_one` 里那句硬编码 121 跟着改钉 124（尺子自己的说明与案面同读法，留旧数就是给自己留下一格没人量的陈旧）。**l264 面 128 → 131**：`no_round_subject` 19 → 21、名册长出 (271, e4ce1ae36) / (271, 94f7f05cc) 两只、唯一对 12 → 14，`same_round` 72 → 73（本条主题与块号同号），其余三档一字未动——那两枚的主题是跨项目文档搬迁而不是本账的轮，落 `no_round_subject` 而不是 `same_round` 是量出来的而不是推的。**四把普查尺同一批**：l242 表行 224 → 226（A351 / A352 两行入表）、定级格面 114 → 115（A352 那一格收成划掉字母，A351 留裸字母）；l253 全面 270 → 272、Backlog 区 224 → 226、权威面 114 → 115、宽面 117 → 118（两面之差仍是 3）、裸 M 46 → 47 与划掉 M 44 → 45（A352 那一格交回划掉字母、新立两笔各拿一格）、闭合词已收口 40 → 41、组合 (划掉 M / 已收口 / 括号 L / 不粗体) 那一档 10 → 11（落进已有形状，没有造出第 19 种）、同卷可查 101 → 102；l263 表行 226 与闭合式 `118 + 5 + 103 == 226`（无判决语档 102 → 103）；l268 operative 自指按 ⑤ 的内容定位器从 845 重钉到 **851**（+21 行 = 清单行 +1、Backlog 两笔连带分隔空行 +5、日志块 +15），散文写的数与独立定位的数由该守卫自己验等。**l242 的声明面 97 一格未动**：本轮刻意让 A352 的散文用「落地 / 补齐」而不是闭合词，闭合只写在定级格那一格 ⇒ 声明面与定级格面第一次出现「只长后者、不长前者」的形状，这也是 ⑨ 那一格返工之外本轮唯一不需要重钉的普查档。**逐格按各自 docstring 的处方重钉，没有任何一把尺子被改松**；⑨ 那三处真实返工留着不美化。重钉后六把账本与文档守卫加四把普查尺与新守卫合计 **298 passed / 8 skipped**（8 支 skip 仍全为 l79 的作者工作树条件）。**终态全量定标**（逐字 `python -m pytest tests -q -p no:cacheprovider`，本批全部改动落盘之后复跑）**9449 passed / 0 failed / 10 skipped / 6 warnings in 409.96 s**，覆盖率逐字 TOTAL 14 392 语句 / 0 缺数 / 3 956 分支 / 0 偏支与 Total coverage: 100.00%，另有 133 只文件因完整覆盖而不进逐行表。用例数与 ⑩ 那次诊断读数逐格衔接：**9 429 passed + 本批自己的 20 红 = 9 449**，即那 20 红全部由本批常数重钉与两处账本散文返工消掉，没有一支用例被删或被改名绕开。语句数 14 383 → 14 392 是 `api/deps.py` 代码批那 +44 行走进测量面，守卫文件本就不在覆盖率面上；分支数 3 952 → 3 956 同源。前端三门禁**不适用**（`web/` 一字未动，不是漏跑）。

### L272（2026-10-14）— 非有限浮点在请求侧 float 旋钮族有三种归宿：四只咽喉共引同一份权威，14 只 × 四形整表钉死，待拍两族按观测行为入尺

- ① **选题依据与普查面**：本轮的靶子不是候选清单里挑的，是量出来的。全仓 float 型请求旋钮做一次「非有限浮点能不能进门」的普查，得到 14 只旋钮 × 四形（NaN / +∞ / 5.0 / −1.0）共 56 格，四种归宿并存：**400 点名旋钮**、**400 但只说族名**、**500 冒充服务端故障**、**200 静默错答**，外加一格**连响应都没有**。两条候选被现量排除：`compare_enhanced.py` 的 `size_ratio=inf` 那一格非 HTTP 可达（只走 CLI 的对比报告，量到了但不构成本轮的对外判决面）；写侧的有限性判据本轮只量不修，立成 **A353**。
- ② **探针方法的一格坑（如实记，因为它差点把读数换成假的）**：`TestClient` 的 `client.post(json=...)` 在**客户端**就 `allow_nan=False`，NaN 那一形连请求都发不出去 ⇒ 量到的是 httpx 的拒绝而不是服务端的归宿。改成按 `requests` 的默认口径发 `json.dumps(payload)` 裸字节才量到真形状。这一改不是取巧：`{"x": NaN}` 恰是 Python 标准库认为合法、RFC 8259 认为非法的那一档输入，真实客户端默认就这么发——所以这一族不是「理论输入」，是「客户端真能发出来的输入」。
- ③ **现量表（改后，逐格按 HEAD 现读）**：`sample.ratio` 400×4（表用默认 `method=random`，另两支的有限越界见本块 ④（乙））、`split` 三只 ratio 400×4（判决点名族名 `ratios`）、`search.fuzzy_threshold` 400×4、`impact.min_scale_gain` NaN/∞ 400 与 5.0/−1.0 走判据后仍按语义回 200、`health-gate` 的 `pass_rate` 400×4，另三只阈值 NaN/∞ 400 而 `5.0` / `−1.0` 仍按既有语义回 200（范围那一维新立 **A354**，本轮不拍）、`outliers.threshold` NaN/−1.0 走 400 而 +∞/5.0 按同族既有口径放行回 200、`dedup.threshold` 与 `benchmark.threshold` 四形全 500（**A280**）、`leakage.check.fuzzy_threshold` NaN 让渲染崩、有限越界正常 422（**A311** 新格）。整表由 `TestHttpFace::test_census_table_is_exact` 判**字典相等**而不是逐格 `in`，多一格少一格换一格都红。
- ④ **改前三格逐个定性（两格本轮修掉，第三格按既有账交回待拍）**：**（甲）** `ratio=+inf` 走 `int(len(items) × inf)` 抛 `OverflowError`，而它**不是** `ValueError` 子类 ⇒ `to_http_error` 的 `except ValueError` 接不住，落 500——客户端参数错误冒充服务端故障的又一形，与 **A280** 不同源（那一族是 `raise_internal_error` 主动一刀切，这一形是异常类型不在分档表里）。**（乙）** `ratio=-0.5` 在 systematic 支折成 `items[:-5]`（「丢最后 5 条」，实测 10 条里回 5 条）、stratified 支折成 0 条，两支都回 **200**：越界的比例参数不是被拒绝而是被**重新解释**成了另一种语义。**修正批订正（不美化）**：这一格本轮**没有**修——接线批曾把它一并改成 400，而「同一个越界比例三种结局」要变一种结局正是 **A292** 的修法 ①，属对外行为变更 ⇒ 已交回待拍。改后 10 条语料实测：`-0.5` 在 systematic 仍交出**前 5 条**、在 stratified 仍交 0 条；`5.0` 在 random 是 400（CPython 英文 `ValueError`，不是 `DataValidationError`），在另两支仍 200 交整份 10 条。本轮真正修掉的是甲与丙两格，加上 `outliers.threshold=NaN` 那一形。**（丙）** `min_scale_gain=NaN` 让 `scale_gain < NaN` 恒 False ⇒ `is_beneficial` 无声返回 `true`，「有没有收益」这一判决在最坏的情况下永远是好消息。
- ⑤ **判据住一处（A77）**：四只咽喉全部共引 `augmentor/validation.py`，端点里不写第二把刀。理由与 `gate_dataset_health` 自己 docstring 里那句「判在 SDK 而不是判在路由，因为 CLI 与 API 共用本函数」同一条——判在路由等于只有一面有声，CLI 用户仍然拿到 ④ 那三种归宿。`outlier.py` 那支把原先手抄的 `threshold <= 0` 换成 `require_positive`，是因为手抄判据对 NaN 恒不成立（`NaN <= 0` 为假 ⇒ 构造成功，检测在 NaN 阈值上无声跑完）。
- ⑥ **`require_ratio` 为什么要动权威而不是在调用点补判**：旧写法是「非数值 / NaN / 不在闭区间内」三段，无穷在闭区间那一档被**顺带**挡住，所以看代码会以为「无穷已管」。开上界的旋钮（`[-1, +∞)`）一出现，`maximum=float('inf')` 就让那一档恒真 ⇒ inf 直穿。本轮因此把判据改成 `math.isfinite`，并**只加这一维**：有限越界那句判决文案逐字未动（`test_finite_out_of_range_keeps_the_bound_wording` 钉），NaN 那句换了措辞但旋钮名与 `NaN` 字样都保留，既有 `match=` 断言逐支存活。
- ⑦ **同族既有口径不擅改（一格按观测行为钉住而不是按应有行为钉）**：`require_seconds` / `require_positive` 的 docstring 明文写了正无穷可接受 ⇒ `OutlierDetector(threshold=+inf)` 构造成功、端点回 200。本轮不把它改成拒绝：这一维要和 **A280**（状态码对外契约）同批拍，单独收紧等于替用户拍了一半。读数由 `test_positive_infinity_still_accepted_by_family_convention` 钉住，将来拍完要同批改那一支。
- ⑧ **待拍两族本轮只量不动**：**A280**——`dedup` / `benchmark` 的 `threshold` 四形全部 500 + 固定文案，而下游判决文案里点名 `threshold`，接它要同批改钉死 500 的集成用例；**A311**——422 的 `detail` 回显里带非有限值时，`JSONResponse.render` 的 `allow_nan=False` 让服务端在渲染阶段自抛 ⇒ 「英文文案双轨」在这一格升级为「**响应可得性**双轨」（生产对应物是连接层失败，不是 500）。两族各自有守卫支（`test_a280_cells_stay_pinned_as_internal_error` / `test_a311_new_cell_render_crash_stays_pinned`），谁拍谁改。
- ⑨ **新立 A353 的现量（写侧）**：`atomic_write_json` 用 `json.dump` 默认档 ⇒ 实测 `{"v":NaN}` / `{"v":Infinity}` / `{"v":-Infinity}` 原样落盘，读侧裸 `json.load` 读回 `nan` / `inf` 并继续参与统计，往返闭合所以自家工具链看不见；严格 JSON 解析器（`parse_constant` 严格档）同一份文件当场 `ValueError`。三条候选（写侧 `allow_nan=False` / 读侧 `parse_constant` 拒 / 维持现状写成契约）都要拍「存量数据能不能读」这一维，与 ⑧ 的 A311 是同族两侧。
- ⑩ **行号引用前置量（本轮没有靠事后救火）**：动手前 grep 全仓活引用 ⇒ `dataset_ops.py` 有十处 `file:NNN` 在册（测试、两卷账本、`web/src/types/api.ts`），所以那一处改动写成**同行改写**（把 `require_ratio` 折进既有表达式，`git diff --numstat` 2/2 净 0 行）；`impact.py` 的在册引用只到 :76，插入点在其后；`quality_gate.py` 与 `outlier.py` 全仓零活引用，插入安全不外推（判的是引用落点，不是逐符号比对）。改后六把账本与文档守卫定向读数见本块末格 ⑬。
- ⑪ **本轮真实返工三处（不美化）**：守卫第一版 **（甲）** 按记忆把 `AugmentationImpact` 写成 `before=5, after=10, metrics_before=None, metrics_after=None`，实际是 `before: Dict / after: Dict / gains: Dict` 三字段（`evaluate()` 里存的是两侧 `to_dict()`），读码后改成 `AugmentationImpact(before={}, after={}, gains={...})`；**（乙）** 门禁判决词表按记忆写了 `("passed", "blocked", "warned")`，真表是 `GateVerdict.PASSED / WARNED / FAILED`，且 `pass_rate_min=1.0` 配 `pass_rate=0.9` 的正确期望是 **failed** 而不是 passed——这一格改完反而更强（它证明闭区间上端 1.0 不是被当成「没传」而回落默认值，正是 L149 / B219 那一族的形状）；**（丙）** 「本轮改前静默、改后必须 400」名册里写了 `shape="2.0"`，而名册的四形键是 `nan / inf / 5.0 / -1.0` ⇒ `KeyError` 当场抓住（`2.0` 对 random 支本来就是 `ValueError` → 400，不属于「改前静默」那一档，换成 `inf` 才是本轮真修的那格）。三处都在任何断言之前被自己的判据抓住。
- ⑫ **注入臂五支全部零落盘**（`TestRulerBites`）：削 `dataset_ops` 那处 ⇒ 退回 `OverflowError` 与 `ratio=True` 交出全量两种改前形状；削权威 ⇒ 门禁不再拒绝 NaN 阈值而是拿它算出判决（`warned_rules` 里出现 `completeness`）；削 `is_beneficial` ⇒ NaN 无声 `true` 复现；把权威退回「只判 NaN」那三行 ⇒ 开上界的旋钮立刻放行 inf（这一支是本轮改权威的直接证据，不是改文案的证据）；削 `require_positive` ⇒ `OutlierDetector(threshold=NaN)` 构造成功。五支之后另有一支复验还原态，证明削弱全靠 `monkeypatch` 自动回落、磁盘上没留残留。
- ⑬ **记账批读数（账本批 `6f62ece26` 落盘之后按红读数逐格重钉常数尺，没有任何一把尺子被改松）**：本块与清单行自己就是新站点——本轮只新立一行 Backlog（A353）与一枚可达哈希，所以红的全是**常数面**而不是判据面。l242 的 `MEASURED_ROWS` **226 → 227**（红读数 `assert 227 == 226`，同批由区段对照那一格逼出 `assert 273 == (226 + 46)` 同一张数；被过滤的 46 行一格未动 ⇒ 这就是「A353 落进 Backlog 区而不是日志区」的证据），l253 的 `MEASURED_ALL_ROWS` **272 → 273** 与 `MEASURED_BACKLOG_ROWS` **226 → 227**、字母形裸 `M` **47 → 48**（红读数 `{'M': 48} != {'M': 47}`），而宽面 **118** 与两面之差 **3** 逐格未动 ⇒ A353 整行不含闭合语，与 A343 / A347 那一族同形；l263 的 `MEASURED_BACKLOG_ROWS` **226 → 227** 连带「无判决语」档 **103 → 104**，闭合式 `118 + 5 + 104 == 227` 与 l253 的两面同批对上；l268 的 operative 自指由独立定位器逼出 **851 → 854**（本轮在它之上落 3 行：清单行 +1、A353 连带它的分隔空行 +2；日志块那 16 行落在被指向的那一行之下，不参与平移），同批按「同一格三处」的口径把句首数字、括号链链尾值与计数散文「至此列出二十三个值 → 二十四个」、逐轮归因那一笔一起改——后两处不带文件名，没有任何守卫量得到，漏了只会在下一轮复盘时现形；l270 的面与档全部由红读数逼出（本块点名账本批哈希自身又添一处站点，所以这一格要连本块一起重量）。**六把账本与文档守卫（markdown 结构 / l98 / l203 / a184 / l79 / l200）在账本段落盘后定向全绿**，唯一的红是 A353 紧贴 A352 那一格：本卷 Backlog A 表的既有风格是每行之间空一行，缺那一行空行就把两行读成一张没有分隔行的表 ⇒ 按既有风格补空行（净 +1 行），不是把结构尺改松。
- ⑭ **修正批 `34b525ee5`：接线批顺手多判的那一维交回待拍（本轮真实返工，不美化）**：**它是被两把既有守卫抓住的，不是被我发现的**——全量门禁里 `tests/unit/test_health_gate_wiring_l235.py` 红 3 支、`tests/unit/test_sample_aggregate_rag_wiring_l230.py` 红 2 支，五支都是既往轮次按现量钉死的既有契约。**咬到的形状**：`require_ratio` 的默认参数是 `minimum=0.0, maximum=1.0`，接线时只写了 `require_ratio(name, value)` ⇒ 「有限性」那一维（本轮的靶）与「闭区间」那一维（**A292** / 新立 **A354** 的靶）被一把接上，于是 `pass_rate_min=5.0` / `-1.0`、`duplicate_rate_max=99.0`、`completeness_min=-3.0`、`min_scale_gain=-2.0`、`ratio=2.0`（systematic / stratified）这些改前就合法的有限值全从 200 变 400。**那两把尺为什么有牙齿而不是陈旧**：l235 的判据是「把 `duplicate_rate_max` 放到 `99.0` 同一份数据就 passed、把 `pass_rate_min` 放到 `5.0` 就 failed」——判决单调性要靠越界阈值才取得到两极，收紧范围等于把这条证据链掐掉；l230 那一格更贵，它钉的正是 **A292** 的三结局矩阵，一把闭区间判下去就是**替用户拍了修法 ①**，而本轮的授权范围里没有这一格。**处置不是重钉那两把尺子而是把产品改回去**：四只咽喉的调用点全部显式传 `minimum=-float("inf"), maximum=float("inf")`，`require_ratio` 内部只新增 `math.isfinite` 那一判 ⇒ 与 L256「可接受域只增不减」同一条口径。**新守卫随之改成反向证据**（`tests/unit/test_nonfinite_float_knobs_l272.py` 43 → **44 支**）：CENSUS 表把 `health-gate` 三只阈值由 400×4 订正为 `400 / 400 / 200 / 200`，`SILENT_CELLS_FIXED` 名册补一句「只有非有限那一形在这一列里」，并新增三支范围面判据（SDK 层 **A292** 九格复现、HTTP 层三方法 × `5.0` / `-1.0` 矩阵、`min_scale_gain` 低于值域下界仍放行）⇒ 哪天有人单方面收紧范围，这三步当场红；本轮把它钉成「不许擅动」而不是「已经修好」。**产品面净行数纪律**：`augmentor/dataset_ops.py` 1/1（同行改写，全仓十处对该文件的活引用一格未平移）、`augmentor/impact.py` 3/2、`augmentor/quality_gate.py` 7/3。**落盘前读数**：被改面定向复跑（新守卫 + l235 + l230 + `test_impact*` / `test_outlier*` / `test_quality_gate*` / `test_validation*`，共十一支文件）**322 passed**，本块 ⑬ 的 12 把账本与普查尺复跑 **284 passed / 8 skipped**（本批落盘后由 ⑮ 重量）；`web/` 一字未动 ⇒ 前端三门禁**不适用**。**A292 维持原样不碰**，范围那一维新立 **A354** 承接。
- ⑮ **记账批读数（修正账本批 `e2665d612` 落盘之后按红读数逐格重钉常数尺，没有任何一把尺子被改松）**：本批的站点全部来自「新立一行 Backlog」与「文件尾追加的两行要点」，判据面一格未动 ⇒ 红的仍是常数面。**A354 那一行**（定级格裸 `M`、整行不带闭合语，散文刻意用「交回 / 落地」而不是闭合词）一次逼出四把尺：l242 的 `MEASURED_ROWS` **227 → 228**（红读数 `assert 228 == 227`，同批由区段对照那一格逼出 `assert 274 == (228 + 46)` 同一张数，被过滤的 46 行一字未动 ⇒ 这就是「A354 落进 Backlog 区而不是日志区」的证据）；l253 的全面 **273 → 274**、Backlog 面 **227 → 228** 与字母形裸 `M` **48 → 49**（红读数 `{'M': 49, ...} != {'M': 48, ...}`），而宽面 **118** 与两面之差 **3** 逐格未动 ⇒ 与 A353 / A351 / A347 那一族同形；l263 的 Backlog 面 **227 → 228** 连带「无判决语」档 **104 → 105**，闭合式 `118 + 5 + 105 == 228` 与 l253 的两面同批对上。**文件尾追加的两行要点**（⑭ 点名修正批那一枚、⑮ 本行点名修正账本批那一枚，两枚的主题与块号都同号）各处长一次：l264 的面 **132 → 133 → 134** 与 `same_round` **74 → 75 → 76**（其余四档与缺陷档 `cross_unnamed` 的恒 0 一字未动），两格都留着是因为 ⑭ 与 ⑮ 分两次落盘；l270 的面 **651 → 653 → 654**、`reachable` **469 → 472**（五档闭合 `472 + 32 + 3 + 147 + 0 == 654`，其余四档一格未动 ⇒ 新增三处站点全是可达对象）、卷二清单区 **125 → 126**（`test_l98_reads_only_volume_one` 里那句硬编码跟着改钉）与卷二日志区 **137 → 139**、唯一 hex **387 → 389**（新点名两只，其中修正批那一枚被清单行与要点 ⑭ 各点一次 ⇒ 站点 +2 而唯一数 +1）、与 l264 对账的 `MEASURED_L264_SITES` **132 → 134** 同批跟上。**l268 的 operative 自指**由独立定位器逼出 **854 → 856**（修正账本批在它所指那一行之上落 3 行：新立 A354 连带它的分隔空行 +2 与要点 ⑭ +1；清单行与要点 ③④ 是同行改写 ⇒ 净 0，要点 ⑮ 落在被指向那一行之下 ⇒ 不参与平移），同批按「同一格三处」的口径把句首数字、括号链链尾值与计数散文「至此列出二十四个值 → 二十五个」一起改——后两处不带文件名，没有任何守卫量得到，漏了只会在下一轮复盘时现形。**两把既有守卫一字未动**：l235 那 3 支与 l230 那 2 支在修正批落盘后各自回到原有形状全绿，本轮改的是产品而不是尺子。**六把账本与文档守卫**（markdown 结构 / l98 / l203 / a184 / l79 / l200）在 A354、要点 ⑭ 与 operative 重钉之后定向全绿；12 把账本与普查尺重钉后 **284 passed / 8 skipped**（8 支 skip 仍是 l79 的作者工作树条件）。**零 parametrize 新增**（本轮新守卫的 44 支一律手写循环）⇒ l97 / l98 的普查棘轮读数一格未动。**终态全量定标**（逐字 `python -m pytest tests -q -p no:cacheprovider`，本批全部改动落盘之后复跑）**9493 passed / 0 failed / 10 skipped / 6 warnings in 354.49 s**，覆盖率逐字 TOTAL 14 397 语句 / 0 缺数 / 3 954 分支 / 0 偏支与 `Total coverage: 100.00%`，另有 133 只文件因完整覆盖而不进逐行表。用例数与 L271 的终态读数逐格衔接：**9 449 passed + 本轮新守卫自己的 44 支 = 9 493**，即本轮没有任何一支用例被删或被改名绕开，而 l235 那 3 支与 l230 那 2 支是被产品改回去才变绿的，不是被改松。语句数 14 392 → 14 397 与分支数 3 956 → 3 954 只能来自本轮改过的五只产品文件（`dataset_ops.py` / `impact.py` / `quality_gate.py` / `outlier.py` / `validation.py`，逐文件差额本轮没有单独量）：判据行走进测量面，而多数新增行是注释与文档串、注释不计语句。前端三门禁**不适用**（`web/` 一字未动，不是漏跑）。

### L273（2026-10-14）— 两次原子不等于一次原子：按路径的 RLock 把同步写者与异步路由串起来，顺带修掉 Windows 冷启动的 63 支误拒

- ① **选题依据与普查面**：本轮的靶子是一条**回执与磁盘不一致**的形状，不是「会不会崩」。判据写成一句话：**写侧原子（临时件 + `os.replace`）保证的是读者看不见半份文件，它保证不了写者不互相覆盖**。普查面因此取「整份读回 → 只改自己那一格 → 把整份写回」这个函数形状（AST 判读点与写点是否同函数、同目标、中间无串行化），得到 7 处写者：HTTP 面的 `api/deps.py::_read_modify_write`（`PUT /api/data/update` / `DELETE /api/data/delete` / 索引写入的共同落点）与 SDK 面的备份索引两处、版本账四处。两条被排除的形状按实测记下：只写不回读的不算（`atomic_write_json` 的裸写点），读写不同目标的不算（注入臂 S4 钉住这一点）。
- ② **缺陷形状（为什么这一族比 500 更贵）**：两个请求各自读回同一份旧内容、各自只改自己那一格、各自把**整份**写回 ⇒ 后写的那一份带着旧快照，前一次改动凭空消失，而**两边都拿到 `{"success": true}`**。备份索引那一格还要更暗：`created=20 / files_on_disk=20` 而 `index_rows=2`，盘上数据一份没坏，**账本丢了 18 行** ⇒ 那 18 份备份对 `list` / `restore` / `delete` / `clean_old_backups` 全部不可见，用户视角是「备份消失了」，而每一次创建调用都回成功。这一格与 A152①（写盘端点的注释被抹）同族：静默、且回执是正的。
- ③ **改前逐格读数（仓外沙箱、单进程单 worker，探针 `probe_race.py` / `probe3_versions.py`，输出 `race_out.txt`）**：200 条数据集上 `PUT /api/data/update?index=N` n=5/20/50 ⇒ `landed=1`、`lost` = **4 / 19 / 49**，n=50 那一档 `all_200=False` （3 支 500，见 ⑥ 与 A356）；`DELETE` n=5/20 ⇒ `before=200 after=199`、`lost_deletions` = **4 / 19**；`DatasetBackup.backup`（备份目录预先创建）n=5/20 ⇒ `index_rows` = **1 / 2**、`lost_rows` = **4 / 18**；`version_control` 模块级 `create_version` n=20 三轮 ⇒ `index_rows=1 / version_dirs=1 / lost_rows` = **17 / 17 / 19**，另有 2 / 2 / 0 支 `PermissionError` 落在 `index.json`；`versioning.VersionManager` n=24 三轮 ⇒ 72 支里 **1** 支 `PermissionError`（`current.txt`，「先判存在再清」与另一支的写相撞）并留下孤儿版本目录；冷启动（备份目录**未**预先创建）8 轮 × 20 并发 ⇒ **63** 支虚假 `DataValidationError`，逐轮 3/14/8/8/14/5/4/7。
- ④ **修法与三个选型（都不是拍的）**：一把**按路径**的可重入锁 + 一份共引的权威。**键**取 `os.path.normcase(os.path.abspath(str(path)))`，注册表由 `_PATH_LOCKS_REGISTRY` 保护——按路径而不是全局，因为不同数据集之间没有必要排队，而全局锁会把并发度换成常数。**类型**取 `threading.RLock` 而不是 `asyncio.Lock`，因为同步写者（`augmentor/backup.py` 的索引、`version_control.py` / `versioning.py` 的版本账）与异步路由面碰的是同一批文件，两把不同世界的锁互不排斥；**可重入**是必需的：`version_control._load_index` 被 `create_version` 在锁内再次调用时按同一键嵌套。**放置**：HTTP 面把整段读-改-写交给一支工作线程（`api/deps.py::mutate_json_file` 内的 `_read_modify_write` 经 executor 跑），因为「在 `await` 上持有线程锁」是错的——这条纪律是本轮把 `read_json_file` 那类裸读点从路由里搬进锁段时量出来的。
- ⑤ **名册与统一判据**：`MEASURED_RMW_SITES` 现量 7 处、每处写点数恰好 1，`test_no_rmw_write_lands_outside_a_path_lock` 判「**0 处未保护**」。串行化之后 `version_control` 那四处全部长成同一种形状（整段 `with path_lock(...)`），所以普查尺只需要一条规则就能判定，不必为「有的函数自己上锁、有的靠调用方上锁」写两套分支——这是本轮把 `_load_index` 也纳入名册的理由。
- ⑥ **顺带修掉的 Windows 落点误判（63 → 0）**：`Path.resolve()` 只有在**拿到句柄**的那一次会给路径戴上 `\\?\` 扩展长度前缀，目录尚未创建时的那一次不戴 ⇒ 同一对路径在「目录已创建」与「目录未创建」两种冷启动下给出不同判决，备份名落点判据因此把 `b002` 这种合法名字判成「指向备份目录之外」。探针的现场读数是决定性的：`in_parents=True exists=True short=…` 三个都成立而判决仍然拒绝。处置是把包含判据收进 `augmentor/validation.py::path_lands_within`（剥前缀后按 `Path` 相等 / `.parents` 比较），**并把它同时用于备份名与 HTTP 面**，所以 L258 / L259 那把「`.parents` 包含判据实现数」普查由「两处实现」收成 **一处**、「不走权威的名字拼接」由 4 处收成 3 处（两把尺的名册与 module docstring 同批改，见 ⑩）。合法名字判据零漂移：`test_escape_backup_name_is_still_rejected` 钉住 `../../x` 照旧被拒。
- ⑦ **候选被现量排除一格（记读数而不立账）**：`POST /api/config` 曾被列为 RMW 候选——它「读配置 → 改一段 → 整份重序列化」。三轮 × 12 只**不同键**并发写实测 `all_200=True landed=12 lost=0`（`p4b_out.txt`）⇒ 它写的不是「回读的那份旧快照」，本轮不接线、不立账，只把这一格排除记在这里。同一支探针顺手量到 `disk_comment_lines=0`，即 A152① 那一族（注释被重序列化抹光）在仓外副本上照旧成立——那一格不在本轮处置面，本轮刻意把写盘探针指向仓外副本。
- ⑧ **改后读数（与 ③ 同面同参，输出 `race_final.txt` / `p3_final2.txt`）**：`PUT` 5/20/50 全部 `lost=0` 且 `all_200=True`（那 3 支 500 一并消失，因为读点进了同一把锁）、`DELETE` 5/20 `lost_deletions=0`、备份四格（目录已创建/未创建 × 5/20）全部 `created == index_rows == files_on_disk`、`version_control` 三轮 `created=20 / index_rows=20 / version_dirs=20 / lost_rows=0 / errors=0`、`versioning` 三轮 72 支 `errors=0 distinct_ids=72 dirs=72 vanished=0`、冷启动误拒 0 支。**一次探针自身的方法坑如实记**：`race_after.txt` 那一轮备份两格报 `NameError: name 'os' is not defined`，是探针脚本自己漏了 import（不是产品缺陷），补完重跑得 `race_after2.txt` 与 `race_final.txt` 两次同读数才落账。
- ⑨ **新守卫的四面结构**（`tests/integration/test_rmw_serialization_l273.py`，**626 行 / 23 支 / 零 parametrize**）：一、**锁的身份** 3 支（同一文件不同拼法给同一把锁、不同文件不给同一把、嵌套可重入）；二、**改后并发读数** 6 支（PUT / DELETE / 备份 / versioning / version_control / 冷启动 20 路）；三、**语义支** 2 支（transform 抛异常 ⇒ 磁盘一字不动；越界 `index` 的判决按**加锁后那份快照**给）；四、**结构普查尺 + 注入臂** 4+2 支（名册集合相等、0 处未保护，S1 未包装的副本被点名 / S2 包了但没调用 / S3 裸写不算 RMW / S4 读写不同目标不算）；五、**落点判据没被写松** 4 支（混合 `resolve` 形状、兄弟前缀目录不算在内、旧写法会对同一对路径误判、合法逃逸名照旧拒）；外加一支探针副作用支（临时根必须在仓外）。**牙齿臂** `test_arms_have_teeth_when_the_lock_is_faked_away`：把四只消费者（`api.deps` / `augmentor.backup` / `augmentor.version_control` / `augmentor.versioning`）的 `path_lock` 换成「每次调用新建一把 RLock」，`lost > 0 or bad > 0` 当场成立 ⇒ 名册与并发读数都不是恒真。
- ⑩ **四把既有守卫按红读数跟着代码走（全量首跑 11 failed / 9505 passed，没有一把尺被改松）**：**（甲）** `tests/integration/test_api_data_500_branches.py` 红 2 支——它注入的接缝 `read_json_file` 本轮被搬进锁段，改法是把 monkeypatch 目标换成 `deps.read_items`（同族既有口径），`TestLoad500` 那一支仍打 `read_json_file` 且照旧绿，两支之后一起复验 500 与 `INTERNAL_ERROR_DETAIL` 一字未动。**（乙）** `test_backup_index_collapse_l250.py` 红 1 支——它按 `body.body` 只取函数体**顶层**语句，而写点本轮罩进了 `with path_lock(...)` ⇒ 数到 0；处置是把定位改成 `ast.walk` 取该函数内全部赋值节点，**判据「恰好一处」原文一字未动**。**（丙）** `test_name_path_containment_l258.py` 红 1 支——`.parents` 包含判据的实现数由 2 收成 1（⑥），名册与 module docstring 同批订正。**（丁）** `test_http_exception_passthrough_l265.py` 红 7 支——它按 AST 现量 raiser / producer / paired 三面，本轮把 RMW 整段搬进工作线程改变了 `try` 体的形状，常数由 `measure265.py` 重钉为 raisers 101 = 68 直呼 + 33 间接、paired 52 / 52 所有者、调用维 39、`api/routes/data.py` 3 处、判据文件与防御档名册逐格跟上（`who265.py` 做 HEAD↔现在的名字差，逐格给归因）。四把的处置共同点：**改的是定位与常数，不改判据强度**。
- ⑪ **历史行号引用重钉 12 行，以及本轮自己的两次算错（内容审计抓住）**：产品面插行把在册引用平移——`versioning.py` 的 `:197 / :281 / :372` → `:205 / :289 / :390`、`:275-276` → `:283-284`、`backup.py` 的 `:90 → :96`、`:114 → :117`、`:192 → :199`（含 ⑥ 的注记）、`version_control.py` 的 `:146 → :159`。**先错在按统一位移算术重钉**：把 `:146` 钉成 `:153`、`:372` 钉成 `:380`——插行不止一处且各处不同向，算术外推不成立；改法是用 `Temp/l273/audit_refs.py` 逐格打印被引行的**当前内容**并按点名的构造定位，两格当场纠正。12 行改动全部走**同行改写**（`git diff --numstat` 3/3 + 9/9 ⇒ 净行数 0），所以账本面自身没有新增漂移。**这一格顺手量出一把尺子的盲区** ⇒ 新立 **A357**（三处引用在 HEAD 上就已 off by 3/4/10 行而 A184 / l79 全绿）。
- ⑫ **新立三行待拍（本轮不自决）**：**A355** 这把锁只保同进程，多 worker 部署照旧丢更新——要拍的是部署形态；**A356** 读侧撞 `os.replace` 那一瞬的 `PermissionError` 折成 500（③ 里那 3 支），本轮只在改前读数里观测到、没为它写定向探针——要拍的是「读要不要上锁 / 瞬时 OS 失败算哪一档码」，与 **A280** 同族；**A357** 定位引用尺子只验落点不验该行内容——要拍的是「一次改严、成批重钉的成本谁付」，与 **A141 / A148** 同一条线。本轮不碰的既有待拍面一字未动（**A354 / A353 / A351 / A350 / A349 / A348 / A347 / A346 / A345 / A343 / A341 / A292 / A280 / A311**）。
- ⑬ **门禁读数**：全量首跑（本轮改动落盘后）`11 failed / 9505 passed / 10 skipped in 459.91s`，11 红逐把归因见 ⑩ 的四组（2 / 1 / 1 / 7）；处置后终态复跑 **9516 passed / 0 failed / 10 skipped in 355.80s**，覆盖率逐字 `TOTAL 14435 0 3956 0` 与 `Total coverage: 100.00%`。用例数与 L272 终态逐格衔接：**9493 + 本轮新守卫的 23 支 = 9516**，即本轮没有删或改名绕开任何一支用例。语句数 14 397 → **14 435（+38）**、分支数 3 954 → **3 956（+2）** 只来自本轮改过的五只产品文件（`api/deps.py`、`augmentor/atomic_write.py`、`augmentor/backup.py`、`augmentor/version_control.py`、`augmentor/versioning.py` + `validation.py`、`api/routes/data.py`）：判据与锁段走进测量面，而新增行里注释与文档串不计语句。六把账本与文档守卫（markdown 结构 / l98 / l203 / a184 / l79 / l200）在代码批落盘后定向 **154 passed / 8 skipped**（8 支 skip 是 l79 的作者工作树条件）。`web/` 一字未动 ⇒ 前端三门禁**不适用**，不是漏跑。
- ⑭ **两格方法坑（如实记，它们都会把读数换成假的）**：**（甲）** 后台命令 `pytest … > out.txt` 之后串了 `echo` / `tail`，于是「exit code 0」量的是最后那条 `tail` 而不是 pytest ⇒ 11 支红被报成通过；处置是**只读 summary 行**，不看 exit code 作结论。**（乙）** `git status --porcelain -- augmentor` 在 cwd 为 `augmentor/` 时只圈到内层包 `augmentor/augmentor/`，把 `api/` 与 `tests/` 的改动藏成假干净 ⇒ 改用 `-- .` 或仓库根路径。另记一笔时间事实：本轮代码批 `587022b11` 落在 10-14 23:53，账本批落笔已过午夜（10-15 00:0x），块标题按开轮日期记 2026-10-14。
- ⑮ **记账批读数（账本批 `1a82af467` 落盘之后按红读数逐格重钉常数尺，没有任何一把尺子被改松）**：本批的站点全部来自「新立三行 Backlog」与「⑮ 本块自身」（清单行与块 ⑭ 那一档是账本批落盘时的增量），判据面一格未动 ⇒ 红的仍是常数面。**三行 Backlog（A355 / A356 / A357，定级格全部裸 `M`、整行不带闭合语）** 一次逼出四把尺：l242 的 `MEASURED_ROWS` **228 → 231**（红读数 `assert 231 == 228`，同批由区段对照那一格逼出 `assert 277 == (231 + 46)` 同一张数，被过滤的 46 行一字未动 ⇒ 这就是「三行落进 Backlog 区而不是日志区」的证据）；l253 的全面 **274 → 277**、Backlog 面 **228 → 231** 与字母形裸 `M` **49 → 52**（红读数 `{'M': 52, ...} != {'M': 49, ...}`），而宽面 **118**、窄面 **115** 与两面之差 **3** 逐格未动 ⇒ 三行都不含闭合语，与 A353 / A354 那一族同形；l263 的 Backlog 面 **228 → 231** 连带「无判决语」档 **105 → 108**，闭合式 `118 + 5 + 108 == 231` 与 l253 的两面同批对上，盲区 **5** 与 `MEASURED_HITS` 名册一字未动。**清单行与本块自己**各处长一次：l264 的面 **134 → 135 → 136**、`same_round` **76 → 77 → 78**（其余四档与 `cross_unnamed` 的恒 0 一字未动）；l270 的面 **654 → 656 → 657**、`reachable` **472 → 474 → 475**（五档闭合，其余三档一格未动 ⇒ 新增站点全是可达对象）、卷二清单区 **126 → 127**（`test_l98_reads_only_volume_one` 里那句硬编码跟着改钉）与卷二日志区 **139 → 140 → 141**、唯一 hex **389 → 390 → 391**（「站点比唯一数多长一次」发生在**账本批**那一档：同一枚代码批哈希被清单行与本块 ⑭ 各点一次，站点 +2 而唯一 +1；账本批自己那一枚在 ⑮ 之前无处可点——它就是把账本批落盘的那次提交 ⇒ 本批这一处让面与唯一数**各长 1**，两把尺的差集由 `test_l264s_face_is_a_strict_subset_of_mine` 逐格对上）、与 l264 对账的 `MEASURED_L264_SITES` **134 → 135 → 136** 同批跟上。**l268 的 operative 自指**由独立定位器逼出 **856 → 862**（账本批在它所指那一行之上落 6 行：清单行 +1 与新立三行 Backlog 连带它们的分隔空行 +5；本块的 17 行落在被指向那一行之下 ⇒ 不参与平移），同批按「同一格三处」的口径把句首数字、括号链链尾值与计数散文「至此列出二十五个值 → 二十六个值」一起改——后两处不带文件名，没有任何守卫量得到，漏了只会在下一轮复盘时现形。**本轮真实返工两处（不美化）**：**其一**，⑮ 首稿写着「同一枚账本批哈希被清单行与本要点各点一次」，把 HEAD 版与现工作树按站点做逐格差集后证明这是假的——被两处点名的是**代码批**那一枚（清单行与本块 ⑭ 各一次，所以站点 +2 而唯一 +1），账本批那一枚在 ⑮ 之前整个账本里无处可点、只在 ⑮ 出现一次 ⇒ 面与唯一数**各长 1**；上面填的终值按订正后的形状写。**其二**，l268 与 l270 各有一行链式注释的正文中间粘着「#: 」（L272 记账批写盘时带进续行的前缀残留，注释不进任何判据 ⇒ 没有任何一把尺子量得到它，本轮按内容清掉），并顺手把 l270 文件头那行「站点总数」的分解式与常数同批跟上——分解式与常数分两处写就会分两次腐。**六把账本与文档守卫**（markdown 结构 / l98 / l203 / a184 / l79 / l200）在账本段落盘时定向 **154 passed / 8 skipped**（8 支 skip 仍是 l79 的作者工作树条件），本批重钉后复跑见终态读数。**零 parametrize 新增**（本轮新守卫的 23 支一律手写）⇒ l97 / l98 的普查棘轮读数一格未动。**终态全量定标**（逐字 `python -m pytest tests -q -p no:cacheprovider`，本批全部改动落盘之后复跑）**9516 passed / 0 failed / 10 skipped**、覆盖率逐字 TOTAL **14435 0 3956 0** 与 `Total coverage: 100.00%`；用例数与 L272 的终态读数逐格衔接 **9493 + 23 = 9516**，即本轮没有删或改名绕开任何一支用例。前端三门禁**不适用**（`web/` 一字未动，不是漏跑）。

### L274（2026-10-15）— 值维缺口：形状门与存在门都不判「这一列里的值是不是文本」——28 格 500 按具名子类折成 400，A280 一格未动

- ① **选题与两道既有门的判据维**：本轮的靶子是一格「调用方自己那份文件让服务端报内部错误」。既有两道门量的都不是这一维——`api/deps.py::assert_dataset_shape` 判「顶层是 list、元素是 dict」，`augmentor/validation.py::require_item_field` 判「这一列存在吗」，所以 `[({"instruction": 1})]` 一路畅通。落点是把 `item.get(key, "")` 直接当字符串用的写法：成员运算 `"为什么" in 1` 抛 `TypeError`、`1.lower()` 与 `1.encode()` 抛 `AttributeError`。端点收尾一律 `raise_internal_error` ⇒ 客户端拿到 A155① 那句固定文案，可行动信息全埋在服务端日志里。
- ② **改前逐格读数**（仓外数据根、Temp `probe3.py`，13 臂 × 7 份语料 = 91 格）：**500 28 格**（4 份退化语料 × 7 只端点）、200 56 格、405 7 格；异常类型 `TypeError 20 / AttributeError 8`；退化语料四形 = 数字 / `null` + 嵌套字典 / 列表 / 布尔。
- ③ **八枚栈帧、六只咽喉**：最深落在本仓内部的帧按行号档是 8 个（`sampler.py:60`、`sampler.py:113`、`api/routes/data.py:182`、`quality.py:26`、`quality.py:28`、`dedup.py:223`、`dedup.py:225`、`dataset_ops.py:259`），按 `(文件, 函数)` 去重是 6 只；本轮接线接的是这 6 只而不是这 8 档——行号会被插行平移，而「这一层有没有判」不该跟着物理行号走。
- ④ **为什么不放宽父类（与 A280 的边界）**：门口三只收敛函数对「SDK 已经判出来的参数错误」给两种归宿——`to_http_error` 与 `raise_boundary_error` 落 400，`raise_internal_error` 将其余一律折成 500。后者那一族在 L272 由 **A280** 按现量钉死（`dedup` / `benchmark` 的非有限浮点四形全部 500，待用户拍板），按父类 `DataValidationError` 放行会把那些格子一起翻掉 ⇒ 新立具名子类 `DatasetTextError`，`api/deps.py` 的 import 与 `raise_internal_error` 只认这一个名字。先例是同仓的 `BackupIntegrityError` 走兄弟而不走子类的选择，那一格的理由在这里反过来用。
- ⑤ **为什么不 `str()` 归一**：归一在只读测量一侧顶多是含糊读数，在会改写数据的一侧是**静默删行**——`POST /api/dataset/merge` 带 `deduplicate=true` 时 `instruction` 为 `1` 与 `"1"` 的两条会折成同一份文本而互为重复、只留一条，`str(None)` 还会造出一个名叫 `None` 的四字符假样本。判据沿用 A321 / L238 那句「静默降级比报错危险」。守卫里 `test_merge_with_deduplicate_does_not_silently_drop_a_record` 把两条一起拒掉并验产物文件根本没被写。
- ⑥ **修法形状**：`require_item_text(items, *fields, label=...)` 逐条逐字段判「在场且必须是 str」，判决点名第几条、哪个字段、实际类型与截断后的原值，整句压在 400 字以内（一条大值不许把响应灌满）；合法时**原样返回入参**，所以五处接线全是同行包裹。缺列与空数据集都是合法——本轮只拒「在场且非字符串」。
- ⑦ **接线与净行数**：四只 SDK 咽喉各 2/2 **净零行**（`sampler.analyze_coverage` / `quality.batch_score` / `dedup.deduplicate` / `dataset_ops._deduplicate`），路由搜索支 3/3 净零行（那一处是延迟 import，同行包裹落不进，所以 `TestRulerBites` 削的是模块属性本身），`api/deps.py` 10/1、`augmentor/validation.py` 65/1（新员追加在文件尾）、`augmentor/exceptions.py` 18/0、`augmentor/__init__.py` 2/0（导出 + `__all__`）。**在册活引用零平移**：按 `Temp/l273/audit_refs.py` 打印每一格落点内容复验——`api/deps.py:58` 仍是 `verify_api_key` 的比较行、`:494` 仍是 `def to_http_error(`（本轮插入点在 566 之后），`validation.py:437` / `:721` / `:1021` 都还在被点名的构造上（插入点在 1119 之后），其余四只文件净零行 ⇒ 无平移。**这一格是 A357 的手工替代**：两把定位尺只判「引用落在代码跨度内」，不判「那一行有没有被点名的构造」，全绿不等于逐格对得上。
- ⑧ **零漂移取证**（Temp `before.json` / `after2.json` 同一批语料）：28 格 500→400，改前 56 格 200 改后仍 200 且响应体**逐字节相同**；只有 `auto_test` 那 7 格不同，差量仅 `execution_time_ms` 与 `timestamp`。缺列（`no_instruction`）与空数据集（`empty`）仍按改前交 200：把「缺列」一并拒掉会把一批现在就成功的请求改成 400，那是 A292 / A354 那一维要拍的事。
- ⑨ **覆盖面第二次量（本轮自己的探针事故一次）**：第一版自动普查 `probe5.py` 用 `input_file` / `output_file` 两张键名相同的槽位，填成同一个文件名 ⇒ 前一支把语料吃掉、后一支量到「文件格式不对」，凭空报出 `merge` / `sample` / `split` 三格假 500（与同一轮 `compare.py` 的键冲突是同一形状）。`probe7.py` 改成**每格新立一份语料文件**并按 `app.openapi()` 枚举全部 POST/GET，146 格里这一族仍 500 的只剩一枚。
- ⑩ **残留那一格的定性**：`POST /api/versions/diff` 拿不存在的 id ⇒ `VersionError`（MRO 实测**含** `ValueError`、**不含** `DataValidationError`）⇒ `raise_boundary_error` 把它交给 `raise_internal_error` ⇒ 500 固定文案，两份语料形状同码（与数据集内容无关，纯 ID 形状）。它与本轮那一形不同族，动它等于替 A280 拍放行维 ⇒ 本轮只按观测行为钉住（`TestResidualMembersOfTheSameFunnel` 验它仍 500、文案固定、不回显调用方给的那两个名字），并立 **A360**。
- ⑪ **新守卫**：`tests/unit/test_dataset_text_field_l274.py`（624 行 / 41 支 / 零 parametrize）。七面：接线名册`WIRING_ROSTER` 五格 + 出厂面调用点总数恰好 5（写成总数是因为新增接线必然让这一格与名册同时红，不会出现「名册说一处、实际两处」）、`TestRulerItself` 用合成语料钉 `_call_sites` 只认函数体内的调用并要求「函数找不到时响而不是静默数 0」（否则盲区名册会自己变绿）、权威面 10 支、咽喉窄性 5 支、HTTP 91 格整表 + 反向证据、`TestRulerBites` 8 支、`TestBlindSpotsAreRealOnTheCliFace` + `TestResidualMembersOfTheSameFunnel`。
- ⑫ **注入臂 8 支全部零写盘**（在导入后的模块对象上打补丁）：五处削接线各自回到 500、按父类放行 ⇒ 本轮 400 仍绿而 A280 那一格当场红（这一支专门证明「父类仍 500」那支钉子不是恒真）、削掉放行支 ⇒ 六只端点回 500、改判决文案 ⇒ 点名丢失。路由那一处的臂削的是 `augmentor.validation` 的模块属性，因为调用点是延迟 import。
- ⑬ **盲区名册 5 格（写明，不装）**：`analytics.analyze` / `quality_report.generate_report` / `visualizer.generate_statistics` 三只量到 HTTP 面走不到（`GET /api/visualize/{filename}` 走 `pipeline.visualize_dataset`），CLI 面实测 `错误: object of type 'int' has no len()` + rc=1；`_as_column` 与 `read_conversation_turns` 是口径相反的一侧（`str()` 归一），实测 `/api/dataset/impact` 同形仍 200。每一格都要求 `require_item_text` **没有**出现在那个函数里——谁去接线，那一格当场红，必须把该条挪进 `WIRING_ROSTER` 并同步账本。立 **A358** / **A359**。
- ⑭ **两格方法坑（都会把读数换成假的）**：**（甲）** `POST /api/dataset/filter` 那一臂的 405 不是判决而是 `api/main.py:216` 的静态挂载决定（`web/dist` 在 ⇒ `StaticFiles` 部分匹配 405，不在 ⇒ 404），把 405 抄成常数等于把「作者机构建过前端」写进判据，干净克隆会自己变红 ⇒ 修正批改按 `app.routes` 现算并补一支「phantom 与任意不存在路径同码」的牙齿。**（乙）** `split` 的 `output_dir` 是必填键，缺它时 422 先于本轮判据 ⇒ 同一形在 `split` 上要补齐必填键才量得到。另立 **A361** 记录（甲）那一格的产品面形状。
- ⑮ **新立四行待拍（本轮不自决）**：**A358** 口径相反的一侧仍 `str()` 归一（统一任一边都改对外行为）；**A359** CLI 面给的是裸 SDK 消息且判决与崩溃同句式同码（要拍 stderr 文案面）；**A360** 不存在的版本 id 折成 500（放行维是 A280 的事）；**A361** 挂在 `/` 的静态资源把所有非 GET 未知路径折成 405（要拍路由装配）。四行的候选与「为什么不拍」逐行写进表内，不在本轮挑任何一条。
- ⑯ **门禁读数**：接线批落盘后全量定标 **9556 passed / 0 failed / 10 skipped in 348.89 s**，9556 = L273 终态 9516 + 新守卫 **38** + `test_exceptions.py` 的 `BUILTIN_COMPAT` 加一行 × 两支按表展开的用例；修正批落盘后复跑 **9559 passed / 0 failed / 10 skipped in 331.94 s**（+3 = 残留成员那一类）。覆盖率两批逐字相同：`TOTAL 14449 0 3966 0` 与 `Total coverage: 100.00%`（语句维与分支维同时零残差）。提交前六把 **154 passed / 8 skipped**（与 L273 终态逐格相同）；定点批次 `test_dataset_text_field_l274.py` + `test_exceptions.py` **100 passed**。前端三门禁**不适用**（`web/` 零改动，不是漏跑）。
- ⑰ **记账批读数（账本批 `147fbb613` 落盘之后按红读数逐格重钉常数尺，没有任何一把尺子被改松）**：**账本批自己的四处改动**（清单行 +1、A358 / A359 / A360 / A361 四行连带分隔空行 +8、日志块 +19 行落在 871 之下 ⇒ 不参与 operative 平移，operative 由内容定位器现读重钉 **862 → 871**；定位器本身要逐字符唯一——裸 `docs/augmentor/API.md:1139` 在卷二有 **47 处**命中，按「最后一处」取会指到 4807 那一行、与真落点差出 3936 行，这就是假账，本批唯一命中的那一串带前导反引号与中文括号）一次逼出五把尺：l242 的 `MEASURED_ROWS` **231 → 235**（红读数 `assert 235 == 231`，同批由区段对照那一格逼出 `assert 281 == (231 + 46)` ⇒ 被过滤的 46 行一字未动，这就是「四行落进 Backlog 区而不是日志区」的证据）；l253 的全面 **277 → 281**、Backlog 面 **231 → 235** 与字母形裸 `M` **52 → 54**、裸 `S` **68 → 70**（红读数 `{'M': 54, 'S': 70, ...} != {'M': 52, 'S': 68, ...}`），而宽面 **118**、权威面 **115**、同卷 **102**、跨卷 **13** 四格逐格未动 ⇒ 四行全是裸字母待拍，与 A353 / A354 / A355 那一族同形；l263 的 Backlog 面 **231 → 235** 连带「无判决语」档 **108 → 112**，闭合式 `118 + 5 + 112 == 235` 与 l253 的两面同批对上；l268 的 operative 自指由独立定位器逼出 **862 → 871**（同一处散文行号的括号链链尾值与「至此列出二十六个值 → 二十七个」按「同一格三处」的口径同批改，后两处不带文件名、没有任何守卫量得到）；l270 的面 **657 → 661**、`reachable` **475 → 479**（五档闭合 `479 + 32 + 3 + 147 + 0 == 661`，其余四档一格未动 ⇒ 新增四处站点全是可达对象）、卷二清单区 **127 → 130**（清单行两只 + A361 那一行重点修正批那一枚）、卷二日志区 **141 → 142**（本行点名账本批那一枚）、唯一 hex **391 → 394**（新点名三只，其中修正批那一枚被清单行与 A361 各点一次 ⇒ 站点比唯一数多一）、与 l264 对账的 `MEASURED_L264_SITES` **136 → 137** 同批跟上，l264 自己的面与 `same_round` 各 **+1**（本行的账本批点名与块号同号 ⇒ 只进 `same_round`，缺陷档 `cross_unnamed` 仍恒 0）。**本行的落点复核**：追加在文件尾 ⇒ 定位器仍指 871，净行数 +1；**六把账本与文档守卫**（markdown 结构 / l98 / l203 / a184 / l79 / l200）在账本批落盘后定向全绿 **154 passed / 8 skipped**（与 L273 终态逐格相同；8 支 skip 仍是 l79 的作者工作树条件，只证明引用不平移，不证明逐条主张被判 live）；**终态全量定标**在本批全部改动落盘后复跑，读数另起一行 ⑱ 记。
- ⑱ **终态门禁读数（本批全部改动落盘后复跑）**：全量定标逐字 `python -m pytest tests -q -p no:cacheprovider` 读出 **9559 passed / 0 failed / 10 skipped in 329.30 s**，覆盖率表逐字 `TOTAL 14449 0 3966 0` 与 `Total coverage: 100.00%`（133 只文件因完整覆盖而不进逐行表）⇒ 与修正批落盘后的读数**逐格相同**，即记账批只动账本与常数尺、没动任何判据面。用例数与 L273 的终态逐格衔接：**9516 + 新守卫 38 + `test_exceptions.py` 的 `BUILTIN_COMPAT` 加一行 × 两支按表展开的用例 2 + 残留成员那一类 3 = 9559**，本轮没有删或改名绕开任何一支用例。**十二把账本与普查尺**（六把账本与文档守卫 + l242 / l253 / l263 / l264 / l268 / l270）在常数重钉之后定向 **284 passed / 8 skipped**（8 支 skip 仍是 l79 的作者工作树条件 ⇒ 只证明引用不平移，不证明逐条主张被判 live）。**零 parametrize 新增**（本轮新守卫 41 支一律手写）⇒ l97 / l98 的普查棘轮读数一格未动。**在册活引用零平移**（本批只改文档与尺子常量，产品文件一字未动）。前端三门禁**不适用**（`web/` 一字未动，不是漏跑）。

### L275（2026-10-15）— 运行身份竞态：并发增强共用一只 CheckpointManager，230 笔进度 170 笔落对方——把状态跟着断点对象走，盘上格式一字不变

- ① **选题与缺陷形状**：`api/deps.py::_pipeline` 是进程内懒加载单例，`POST /api/augment/start` 把 `p.augment_dataset(...)` 丢进 `BackgroundTasks` ⇒ 两只任务共用同一只 `CheckpointManager` 与同一份 `AugmentorPipeline._process_errors`。L273 的 `path_lock` + `self._lock` 治的是「同一份文件别写坏」，治不到「这次更新属于哪个任务」：`update_progress(index, success)` 不带任务身份，它写的永远是「此刻 `_current_checkpoint` 指向的那一份」，而 `_current_checkpoint` 由**另一次调用**的 `load_checkpoint` 移动。判据是归属，不是原子性。
- ② **改前逐格读数**（仓外沙箱、Temp `probe2.py` / `probe3.py`，A=120 条与 B=50/40 条真并发、限速器在导入后关掉）：进度侧 **230 笔里 170 笔落在对方断点上** ⇒ B 的 `completed_indices` 拿到 **150 个越界下标**（值域 0..199 而 `total_items=50`），A 自己那份盘上停在 `processed_items=0`，另留一枚没人认领的增量文件；errors 侧单跑 A=120 与单跑 B=40 各得 0 格越界，并发时第一份报告里的 **139 条 error 全部属于对方**，实例属性上另残留一坨没人清的计数。
- ③ **修法（身份跟着对象走）**：`_RunState`（`__slots__` = 待完成集 / 待失败集 / 上次自动保存计数）作为**非 dataclass 字段**的私有属性 `_run_state` 挂在每只 `CheckpointData` 上；`_state_of(ck)` 负责惰性建，`_orphan_state` 兜住「还没有任何断点」这段窗口。`update_progress` / `_save_delta` / `save_checkpoint` / `get_remaining_indices` / `get_progress` / `_calculate_avg_quality_score` 各加**尾部可选参**，缺席时回落到 `_active_state` ⇒ 全部既有调用点与既有测试一字不动即可编译，直接调用方不破。
- ④ **为什么不能写成 dataclass 字段**：那等于改盘上格式——`asdict()` 会把它序列化出去，旧断点文件立刻读不回来。两支守卫钉住这一格：`test_asdict_key_set_does_not_grow` 逐键比键集合、`test_pre_existing_checkpoint_file_still_loads` 用手写的旧格式文件走一遍加载。
- ⑤ **errors 那一侧**：`augment_dataset` 里改成运行局部 `errors: List[Dict]`，worker 线程拿到的是这个列表本身（`executor.submit(..., errors)`），`_process_single_item(..., errors=None)` 的 `None` 保持「写实例属性」这个既有默认；报告读 `list(errors)`，实例 sink 每次起跑清空。守卫两支分别钉「直接调用方仍用实例 sink」与「给了运行局部列表时它赢」。
- ⑥ **普查面**：全仓「构造器之外对 `self.X` 的赋值」现量 **129 处**，其中落在增强跑路径上 **45 处**，另有 **5 处** `self.X.Y=` 的嵌套批评态。本轮收的是「断点归属」与「errors 归属」两格；余下按名册立账 **A362 / A363 / A364**，三格都写明「本轮只立不修」，其中 A363 / A364 的**行为差未量**（未做并发对撞）——不拿「读到了就说是缺陷」冒充实测。
- ⑦ **仪器自己的一格假绿（本轮最要紧的一课）**：errors 那一档探针的放行条件原本盯 `len(p._process_errors) >= 10`，接线之后错误落在运行局部列表、实例属性不再增长 ⇒ watcher 永远等不到信号，第二支任务在第一支**跑完之后**才被放行，「并发」静默退化成顺序，而读数照样全绿。改成按线程各自数 `update_progress` 来放行并打印重叠窗口，才拿到「确实重叠」的证据。**修完之后仍在工作的仪器才算仪器**；凡是「靠被修的那个缺陷才会触发」的探针信号，接线后必须换仪器并重跑。
- ⑧ **新守卫**：`tests/unit/test_run_scoped_checkpoint_state_l275.py`（**325 行 / 14 支 / 零 parametrize**）。四面：AST 名册面（`EXPECTED_PRODUCT_CALLS` 逐格钉「增强跑路径上每只调用点带不带身份实参」，含 `api/routes/augment.py` 那格 `(1, 0)`——它是 A362 的具名 ambient 位，两支合成反证臂证明这把尺子会咬、也证明它认位置实参）；行为面（栅栏同步的两线程互不越写、一支 **ambient 反证臂**：同一份代码不传 `checkpoint=` 就当场复现 170/230 那一形，增量文件归属、`get_progress(ck)` 逐只读、`get_remaining_indices(ck)`、`save_checkpoint(ck_a)` 不许删掉 B 的增量）；errors 归属面两支；盘上格式面两支。
- ⑨ **守卫自己的第一版假绿一次**：`test_save_checkpoint_finishes_the_named_one` 起初给 B 只写 5 笔，而 `auto_save_interval=10` ⇒ B 根本没有增量文件，那句「A 收口不许删掉 B 的增量」是空跑。改成 B 写满 10 笔让增量真落盘，判据才有牙齿。
- ⑩ **既有四处断言/替身按新口径重钉（口径变更，不是放宽）**：`test_pipeline_error_report_l206.py` 的 `test_internal_list_is_reset_too` 改判「跑完实例 sink 归零」；`test_serial_branch_records_under_lock` → `test_serial_branch_writes_the_run_local_list`（串行支的调用方只有一个线程，那把锁在这一支是死同步，留着等于「为守卫而同步」）；补 `test_parallel_branch_appends_under_lock`——并行支那一笔仍在锁内，判据用缩进嵌套而不是字符窗口，因为第一版用 200 字符窗口量「最近的 `with self._lock`」会被我自己写的注释推出去（那一版当场红，是尺子写错不是产品写错）。`test_pipeline_branches.py` 一处的 `errors` 计数改按运行局部列表重钉；第四格是 `tests/unit/test_micro_branches_l68.py` 的替身补形参（见 ⑰）。四处各留反证，改前改后各自喊红过。
- ⑪ **定位引用重钉**：`augmentor/checkpoint.py` 头部插入类与三只属性 ⇒ 卷一 :292 那三处 `file.py:NNN` 整体下移 **88 / 95 / 95** 行，按 A127 口径重钉为 308 / 359 / 430 三档，同行内散文补「其内的增量触发见 :467」；L277 收 A366 时又在本文件 `load_checkpoint` 的增量合并段插了 12 行 ⇒ 这三档的现量落点是 320 / 371 / 442，活引用只留卷一 :292 那一处；`git diff --numstat` 复核净 **1/1**（单行改写，不平移任何活引用）。六把账本与文档守卫首跑红 3 格（A184 硬 0 档 dead_line 3≠0、它的注入臂、`test_doc_line_refs_l79`），重钉后 154 passed / 8 skipped。
- ⑫ **单跑零漂移**：接线批落盘后既有断点与流水线测试 **189 支**全绿，唯一改动是三处口径重钉（替身那一格是这次门禁才暴露的，见 ⑰）⇒ 「修并发」没有改坏任何一次单跑的行为面。产物逐字节可比对（同一份语料单跑 vs 并发跑，只差 `task_id`，而它本就由输入路径哈希推出）。
- ⑬ **改后复量**：进度侧 **250 笔全部显式带身份、0 笔落对方**；errors 侧 A 120/120 归自己、B 40/40 归自己、属于对方 0 条、实例残留 0 条，且打印出的重叠窗口非空（证明这一趟真是并发）。归属档对照：单跑 A、单跑 B、并发三档同尺。
- ⑭ **ambient 那一格现量（A362 的凭据）**：Temp `probe5_ambient.py`，A=60 与 B=20 并发跑完后读不带参数的 `get_progress()` 与 `list_checkpoints()` 对照——并发跑完那一刻读无参 `get_progress()` 报的是 **A 那一份**（`augment_4f385f14d1d0`，`total_items=60`、`processed_items=60`）；把盘上两份各 `load_checkpoint` 一遍之后再读**同一只无参函数**，报的换成了 **B**（`augment_7e41d7d047de`，`total_items=20`、`processed_items=20`），而盘上两份同时都在（60/60 与 20/20）⇒ 端点对同一份磁盘状态给两种答案，取决于「谁最后加载」，与请求者想问哪只任务无关。两份运行报告自己的 `progress` 格则各自正确（A 60/60、B 20/20），因为接线批之后 `augment_dataset` 传的是手里那一份。
- ⑮ **为什么不顺手把 A362 接掉**：那条端点的响应模型里没有任何任务身份，加参数或加字段都是改对外面；按纪律「凡是改响应面 / 改状态码语义 / 加字段的一律等拍板」，本轮只把 ambient 调用点在守卫名册里具名钉成 `(1, 0)` 并立账，不动端点。
- ⑯ **门禁读数**：逐字 `python -m pytest tests -q -p no:cacheprovider` 读出 **9574 passed / 0 failed / 10 skipped in 321.72 s**，覆盖率表逐字 `TOTAL 14491 0 3970 0` 与 `Total coverage: 100.00%`（14449 → 14491、分支 3966 → 3970 是本轮 `checkpoint.py` / `pipeline.py` 净增的行与支）；9574 = L274 终态 9559 + 新守卫 **14** + l206 那一支新增的并行锁判据 **1**（用例数只增不减，没有任何一支被删）。同一条命令在替身补形参之前连续两次停在 62% 零 CPU（见 ⑰）。六把账本与文档守卫 **154 passed / 8 skipped**。
- ⑰ **门禁连挂两次，量到并行消费侧的另一格缺陷（另立 A365）**：接线批落盘后，逐字全量定标连续两次停在 62%——第一次 24 分钟、第二次 4 分钟，`Get-Process` 采样 CPU **两次逐字相同（208.3125 → 208.3125）** ⇒ 不是慢，是零 CPU 的永久等待。加 `-o faulthandler_timeout=120` 才打出栈：主线程停在 `augmentor/pipeline.py:398` 的 `Queue.get`，两支 worker 线程 idle 在池里等活，挂住那一支测试是 `tests/unit/test_micro_branches_l68.py::test_parallel_window_trims_existing`。根因两层——**其一**，替身 `big_single(idx, item, use_quality_check, q)` 比真实签名少一格形参（本轮给 `_process_single_item` 追加了尾部可选参），`executor.submit` 传第 5 个实参 ⇒ worker 里抛 `TypeError`；**其二**，消费侧写的是「每只 future 迭代一次、每迭代从不带超时的队列取一条」，而 `future.result()` **一次也不读**（`futures[future] = idx` 建了也从没读过）⇒ 少投一条就是永久挂死，异常还无声吞掉，日志里连那笔 `TypeError` 都没有。本轮只补替身 arity 让门禁能走，产品那一格立账 **A365**：修法要先拍「少投递那一笔算条目失败继续，还是整批出声」，后者改 `augment_dataset` 的对外归宿，本轮不自决。**顺带一条现场教训**：产品加了尾部可选参之后，窄形参的测试替身不会立刻红，它会把整批挂死。
- ⑱ **前端三门禁不适用**：`web/` 一字未动（不是漏跑）。**零 parametrize 新增**（本轮新守卫 14 支一律手写），接线落盘后普查棘轮读数待记账批按红读数逐格重钉。**在册活引用零平移**：本批产品文件只改 `checkpoint.py` 与 `pipeline.py`，卷一 :292 已按新行号重钉，其余 `file.py:NNN` 引用逐把守卫复核过。
- ⑲ **账本批自己红的一格（读数不美化）**：卷二清单行落盘后六把首跑 **1 failed / 153 passed / 8 skipped**，唯一红的是 `test_docs_markdown_structure.py::test_bold_markers_paired[augmentor/OPTIMIZATION_LOOP_2.md]`，报错元组第二格的 2523 是**块内粗体标记计数而不是行号**——整个清单区被空行劈成一块，计数为奇数即红。归因：本轮那一行里「这一笔进度属于哪个任务」住在共享单例的实例属性上那一段只写了开标记，闭标记漏在破折号之前。修法为同行改写补一枚闭标记（净 0 行），该行标记数 73 → **74**，块级奇偶判据返回空表，六把复跑 **154 passed / 8 skipped**。**顺带量到那把尺子的两条口径**：`blank_code` 先剥行内码再数标记 ⇒ 行内码里的星号不进计数；判据是**块级**奇偶 ⇒ 按行去数会看见上百处「奇数」却几乎全是跨行配对，真要定位得把加粗跨度逐对打印出来才看得见漏在哪一对。
- ⑳ **记账批读数（账本批 `a18c8567c` 落盘之后按红读数逐格重钉常数尺，没有任何一把尺子被改松）**：账本批自己的三处改动（清单行 +1 且只点名代码批那一枚、A362 / A363 / A364 / A365 四行连带分隔空行 +9、日志块整块（标题 + 空行 + 要点 ①–㉑）落在被 operative 指向的那一行之下 ⇒ 不参与平移）一次逼出六把尺：l242 的 `MEASURED_ROWS` **235 → 239**（红读数 `assert 239 == 235`，同批由区段对照那一格逼出 `assert 285 == (235 + 46)` ⇒ 被过滤的 46 行一字未动，这就是「四行落进 Backlog 区而不是日志区」的证据）；l253 的全面 **281 → 285**、Backlog 面 **235 → 239**，字母形裸 M **54 → 55** 与裸 S **70 → 73**（红读数 `{'M': 55, 'S': 73, ...} != {'M': 54, 'S': 70, ...}`，四行定级按 A362 M / A363 S / A364 S / A365 S 入面），而宽面 **118**、权威面 **115**、同卷 **102**、跨卷 **13**、闭合词五档逐格未动 ⇒ 四行全是裸字母待拍，与 A355~A357 那一族同形；l263 的 Backlog 面 **235 → 239** 连带「无判决语」档 **112 → 116**，闭合式 `118 + 5 + 116 == 239` 与 l253 的两面同批对上；l268 的 operative 自指 **871 → 880**（名册那一格换值，同一格散文里的括号链链尾值与「至此列出二十九个值」按「同一格三处」口径同批改，后两处没有任何守卫量得到）；l270 的面 **661 → 663**、reachable **479 → 481**（五档闭合 `481 + 32 + 3 + 147 + 0 == 663`，其余四档一格未动 ⇒ 新增站点全是可达对象）、卷二清单区 **130 → 131**（清单行点名代码批那一枚）、卷二日志区 **142 → 143**（本行点名账本批那一枚）、唯一 hex **394 → 396**；与 l264 对账的 `MEASURED_L264_SITES` **137 → 138** 同批跟上，l264 自己的面 **137 → 138** 与 `same_round` **79 → 80**（本行的账本批点名与块号同号 ⇒ 只进 `same_round`，缺陷档 `cross_unnamed` 仍恒 0）。**⑲ 那一格是账本批自己的缺陷**：块级粗体奇偶判据被本轮清单行漏写的闭标记逼红一次，且报错元组第二格（2523）是**块内标记计数而不是行号**——同行改写补形后该行标记数 74、六把全绿。**本行的落点复核**：追加在文件尾 ⇒ operative 定位器仍指 880，净行数 +1；本行只点名一枚新哈希（账本批那一枚），不再重提代码批那一枚 ⇒ 站点与唯一名册各 +1。终态门禁读数见 ㉑。
- ㉑ **终态门禁读数（本批全部改动落盘后复跑）**：全量定标逐字 `python -m pytest tests -q -p no:cacheprovider` 读出 **9574 passed / 0 failed / 10 skipped in 330.93 s**，覆盖率表逐字 `TOTAL 14491 0 3970 0` 与 `Total coverage: 100.00%` ⇒ 与要点 ⑯ 那一次（账本批落盘前）按**计数判据**逐格相同——用例数、failed、skipped 与覆盖率两维四个数一字不差；用时不列进对账（同一份语料在不同跑次的 wall 会因线程调度漂移，L268 起定的就是这条口径）。用例数 **9574 = L274 终态 9559 + 新守卫 14 + l206 那一支并行锁判据 1**，本轮没有删或改名绕开任何一支用例。**十二把账本与普查尺**（六把账本与文档守卫 + l242 / l253 / l263 / l264 / l268 / l270）在常数按现量重钉之后定向 **284 passed / 8 skipped**（这条命令只列那十二个文件，收集 292 支）；把 l97 那把零 parametrize 棘轮一并列进同一条命令（第十三个文件）读数是 **295 passed / 8 skipped**。两个数按**文件清单**区分而不是同一读数的两种写法——本行最初填的是 295 而标签写着「十二把」，是**读数与尺面清单不同名**的一格自证缺陷，现按两套清单各钉各的数（重跑两次分别复现 284 与 295）（8 支 skip 仍是 l79 的作者工作树条件 ⇒ 只证明引用不平移，不证明逐条主张被判 live）；重钉逐格按各自 docstring 的处方走，**没有任何一把尺子被改松**——五档闭合 `481 + 32 + 3 + 147 + 0 == 663`、三面闭合式 `118 + 5 + 116 == 239`、l264 与 l270 在同一枚账本批哈希上同批对上，这三处闭合关系本身就是复验。**零 parametrize 新增**（本轮新守卫 14 支一律手写）⇒ l97 的普查棘轮读数一格未动。**在册活引用零平移**：本批产品文件一字未动，改的是账本散文与六把尺的常数、注释与一行归因；operative 自指仍指 880（本行与 ⑳ 都追加在文件尾，净行数 +2 全在被指向那一行之下）。前端三门禁**不适用**（`web/` 一字未动，不是漏跑）。

### L276（2026-10-15）— A365 并行消费侧按「这一笔」配对收口 + A362 进度端点口径文档

- ① **选题（L275 立账两格同轮收口）**：**A365**（并行增强消费侧按「计数」配对 ⇒ 任一 worker 少投一次整批零 CPU 永挂、`future` 的异常又从未被读而无声吞掉）与 **A362**（`GET /api/augment/progress` 读「最后加载的那一份」断点）。A365 按用户拍板的候选①「条目失败继续」修，A362 走候选②「只动文档」，两格都不动 `augment_dataset` 之外的对外响应面。
- ② **修法（A365 信箱配对）**：`augment_dataset` 并行支由「共享一只 `result_queue` + 不带超时的 `get()`」改成「**每条目一只信箱**」（`mailboxes = {idx: Queue()}`）。消费侧从 `futures[future]` 拿到「这一笔」下标去读那一只：`future.exception()` 出声（逃出的异常不再被吞）、具名常数 `RESULT_WAIT_SECONDS` 的有界等待取代无限等待——少投递那一笔记 `ResultNotDelivered`（四键与 worker 自报 `error_info` 同形：`index` / `error_type` / `message` / `item_preview`）并**继续**，整批不再挂死；多投递一条不再记到别人名下；成功支与失败支各投且只投一次。
- ③ **为什么不改成整批出声**：那会改 `augment_dataset` 的对外归宿（少投一次 ⇒ 整批报错而非继续），属改对外契约一族，与 A280 / A360 同族；按纪律「改响应面 / 状态码语义一律等拍板」，本轮只走拍定后的「条目失败继续」。
- ④ **A362（文档，候选②）**：`docs/augmentor/API.md` 把 `GET /api/augment/progress` 的口径写明是「管理器此刻最后加载的那一份断点」而非「本次请求启动的那只任务」，并给出两条精确读法替代（`GET /api/augment/checkpoints` 拿任务 ID 按断点文件对账 / 串行跑）。端点补 `task_id`（候选①）仍留待拍，故 A362 本轮以文档口径收口、不改端点。
- ⑤ **新守卫** `tests/unit/test_mailbox_paired_consumption_l276.py`（**433 行 / 19 支 / 零 parametrize**）三面：源码形状面（AST 名册钉「按笔配对」而非「按计数配对」：`futures[future]` 恰好读一次、`mailboxes[idx].get(timeout=RESULT_WAIT_SECONDS)` 恰好一次且超时必须是具名常数、`future.exception()` 读一次、不带信箱的 `result_queue.get()` 不许回来、每条目各建一只信箱、worker 拿到的是 `mailboxes[idx]` 而非共享队列、worker 两条支路各投且只投一次）、行为面（少投 ⇒ 该条目失败并继续、逃出异常出声、多投不进账、逐只归属、有界等待不挂死、健康路径零漂移、失败明细四键同形）、尺子自证面（把改前那段消费侧喂给同一只分析器它必须拒，反证臂让「形状名册不是恒真」）。
- ⑥ **健康跑次零漂移（修法的判据）**：全部 worker 正常投递时，并行与串行的产物条数、错误清单、逐条内容（按 `sort_keys` 归一）完全一致——「修法①只在坏档上出声」，改前同批语料产物逐字可比。
- ⑦ **门禁读数（账本批落盘后复跑）**：全量逐字 `python -m pytest tests -q -p no:cacheprovider` 读出 **9593 passed / 0 failed / 10 skipped**（= L275 终态 9574 + 新守卫 **19**，用例数只增不减、没有任何一支被删或改名绕开）；覆盖率逐字 `TOTAL 14500 0 3972 0` 与 `Total coverage: 100.00%`（语句 14491 → 14500、分支 3970 → 3972 是本轮 `pipeline.py` 净增的行与支）。10 支 skip 逐格同 L275 终态：l79 的作者工作树条件 8 支 + `test_config_gates_l82` 1 支 + `test_upload_ceiling_l87` 1 支（干净 CI 克隆口径，非本轮新增）。
- ⑧ **六把账本与文档守卫**：清单行 + A365 / A362 两格定级改写 + 日志块落盘后，六把首跑按红读数逐格重钉，没有任何一把尺子被改松（逐格读数见 ⑪）；重钉后定向全绿（见 ⑫）。
- ⑨ **在册活引用零平移**：本轮产品文件只改 `augmentor/pipeline.py`（新增 `RESULT_WAIT_SECONDS` 与信箱消费侧）与 `docs/augmentor/API.md`（进度端点口径，净同行改写），`web/` 一字未动 ⇒ 前端三门禁**不适用**（不是漏跑）。
- ⑩ **A365 / A362 两行由「待拍」转「已收口（L276）」**：Backlog A 区两格**定级格**改写（`~~S~~ 已收口（L276）` / `~~M~~ 已收口（L276）`），表行正文一字不动（历史叙述不追改）；l242 的表行数、l253 的字母面与闭合词面、l263 的判决面按各自 docstring 处方随定级格改写而重钉（见 ⑪）。

### L277（2026-10-16）— 安全审计轮的九格结案：A292 / A339 / A341 / A343 / A347 五格结案 + A366 / A367 / A368 / A369 四行同轮收口

- ① **选题与授权**：用户 goal 原文是「全面找出并修复验证 augmentor 中所有 bug 和漏洞，直到你完全找不出来为止」⇒ 本轮把账上挂着「待拍」的安全族一次拍掉（A341 / A347 两格自 L269 起待拍、A343 自 L262、A339 自 L257、A292 自 L230），并把本轮新查出的四格（A366~A369）立账同轮收口。**每一格都改对外行为**，所以逐格在下面点名归宿；可回退路径 = 逐格 revert 那一支产品文件的 hunk（本轮没有跨格耦合的改动）。
- ② **A347（写面匿名可达，收「未设密钥就全开」那一格）**：新层 `api/middleware/write_auth_gate.py::WriteTransportGate`，装在 `api/main.py:153`（`FastAPI(middleware=[...])` 之内 ⇒ 在 CORS **之内**，与限流同族口径）——未设 `AUGMENTOR_API_KEY` 时，连接本地侧地址非回环的 POST / PUT / PATCH / DELETE 一律 **403**，文案给两条出路（设密钥，或把服务绑回 `127.0.0.1`）；读操作与 `/api/health` 一字不动。`is_trusted_transport` 明确**不信** `X-Forwarded-For`（那是调用方可伪造的头），判据只取 `scope["server"]`。同一族里把 `RateLimiter` 的 XFF 归属一并改掉，并把 JSON 请求体纳入 `UploadBodyGate` 的体积上限（改前只有表单受上限约束）。守卫在 `tests/unit/test_vuln_audit_l277.py` 的第 1 族（纯 ASGI 消息驱动，`TestClient` 给不出任意 `scope["server"]`）。
- ③ **A339 + A343（空作用域族，修法①）**：`augmentor/audit.py:69` 与 `augmentor/privacy.py:76` 收成同一句 `fields if fields is not None else list(DEFAULT_TEXT_FIELDS)`。**分界线是「交回判决还是交回计数」**：`DatasetAuditor(fields=[])` 与 `LeakageDetector(fields=[])` 当场抛 `DataValidationError` ⇒ 端点 **400**（它们交回的是「有没有命中」这个判决，空作用域下的「零命中」是假零）；`PiiSanitizer(fields=[])` 仍合法返回 `[]`（它交回的是计数，且「不脱任何字段」是一种用法）。同族另五处一起改判 `is not None`（`augmentor/aggregator.py:70`、`augmentor/cleaner.py:156`~`:157`、`augmentor/export.py:364`、`augmentor/pipeline.py:653`；初稿用裸名加改前坐标，`aggregator.py` 那一格落在注释行上，由 a184 抓出）；`formats=[]` 从此是「一种都不导」而不是「没表态」，空数据集的 CSV 导出与 jsonl 档同形（**落零字节文件**，改前静默零文件 + 成功回执）。HTTP 侧读数由 `tests/unit/test_empty_scope_http_l277.py`（11 支）钉，SDK 侧由 `test_empty_container_defaults_l262.py` 按修法①重钉。
- ④ **A341（配置即权限）**：`api/routes/config.py:111 _reject_escaping_dir_value` 把目录类配置键交给数据文件那一族的同一把闸 `resolve_within_roots`，判在 `apply_section_update` **之前**（坏值不该先在内存对象上留一秒，也不该被 `save_config` 写进磁盘）；名单 `_DIR_VALUED_KEYS`（`api/routes/config.py:108`）由守卫机器核对「恰好覆盖 7 个可写节里所有目录语义的字符串键」，多写或少写都红。归宿：含 `..` 或空值 **400**、规范化后落在白名单之外 **403**。**为什么不装进 `VectorConfig.__post_init__`**：配置层没有 HTTP 白名单的概念，装进去会把 CLI/SDK 的合法绝对路径一起拦掉。守卫 `tests/unit/test_config_dir_containment_l277.py`（7 支）。
- ⑤ **A292（`sample.ratio` 的范围维，修法①）**：`augmentor/dataset_ops.py:300` 现在是 `int(len(items) * require_ratio("ratio", config.ratio))` ⇒ 同一越界值在 random / systematic / stratified 三支给**同一种结局**（400），改前是「一支 400、一支整份、一支零份」的三结局矩阵；split 那三只 ratio 早就走 `require_ratio_list`（A324 / L256），两半至此都有判决。文档与页面卡片文案从「只有 random 支会拦」改成「一律 400，三种方法一种结局」。**A354 本轮不收**：它管的是另外四只旋钮（`pass_rate_min` / `duplicate_rate_max` / `completeness_min` / `min_scale_gain`），L272 修正批已经量到「把它们一并收紧会把改前合法的有限值全变 400」并被两把既有守卫当场抓住，那一格仍待拍。
- ⑥ **A366**、⑦ **A367**、⑧ **A368**、⑨ **A369**：四格的表行各带完整读数（本轮新立、同轮收口），此处只记跨格的三件事——⑥ 的判据住 `augmentor/validation.py:1191` 且全仓只有一个出生点；⑦ 的判据必须写在 `try` 之外，否则被 `raise_internal_error` 折成 500（A280 那一族本轮不动，只在守卫里钉「仍 500」作反向证据）；⑨ 的改写刻意做成**净零行**，因为 `load_checkpoint` 之上每加一行都要重钉 `_save_delta` / `_read_delta` / `update_progress` 三档现量。
- ⑩ **前端**：`web/src/pages/Augmentation.tsx` 的轮询补「终态停表 + 重启再走表」两支行为守卫。两条实测口径写进测试注释——(a) `vi.clearAllMocks()` 只清调用记录、不清 `mockResolvedValue` / `mockRejectedValue` ⇒ 上一支设的 reject 会漏进下一支，本 describe 的 `beforeEach` 显式重设；(b) `message.success(...)` 那枚 toast 在 `vi.useFakeTimers()` 下渲染不出（sinon 连 `queueMicrotask` 一起假掉，切回真实定时器时已排队的回调丢失）⇒ 终态断言判 React state 渲出的「增强进度100%」而不是 toast 文案。另有 `createVersion` 的 `filename` 由路径串改走 query、备份名单一走 `encodeURIComponent`，归属检查面现量 13 → 14。
- ⑪ **在册活引用与常数按红读数重钉（尺子一格未松）**：卷一 :292 那三档 checkpoint 落点重钉 `augmentor/checkpoint.py:321`（`_save_delta`）× `augmentor/checkpoint.py:372`（`_read_delta`）× `augmentor/checkpoint.py:443`（`update_progress`，其内的增量触发见 :480）——本轮两批都在 `load_checkpoint` 的合并段落笔，合计净 **+13** 行（收 A366 那批 +12、收 A369 的去重重写再 +1：「净零行」的打算**没做到**，按现量认账）；本轮在册活引用按机器提取的 diff 对账（改前坐标一律不再以 `文件:行号` 形状留在册，a184 实测把三处这样的旧数读成 dead_line）：checkpoint 三档 308/359/430 → 见上一格、`api/routes/config.py` 的 100 → **`api/routes/config.py:101`**、130~132 那段裸 `setattr` 旧坐标 → **`api/routes/config.py:245`**、`api/routes/data.py` 的 163 → **`api/routes/data.py:172`**、`augmentor/dataset_ops.py` 的 296 → **`augmentor/dataset_ops.py:299`**、347 → **`augmentor/dataset_ops.py:350`**、`augmentor/logging_setup.py` 的 163 → **`augmentor/logging_setup.py:166`**、`augmentor/pipeline.py` 的 678 → **`augmentor/pipeline.py:718`**、`web/src/types/api.ts` 的 218 → **`web/src/types/api.ts:220`**（同句联合别名那一格 133 → 135）；本轮新落册的落点另含 `augmentor/validation.py:1191`、`augmentor/backup.py:116`、`api/main.py:153`、`augmentor/audit.py:69`、`augmentor/privacy.py:76`；`test_api_deps.py` 的 `raise_internal_error` 名册加 `"api/routes/data.py": 1`（400 档，不是 500 档）并按现量重钉 10 处；`test_export_census_sentinel_l177.py` 的语料规模 (133, 390)→(133, 392)，按 mtime 逐格核对后写明「392 = 386 已跟踪 + 6 只本轮新增未跟踪」。l177 的语料规模按红读数二次重钉 (133, 392) → **(133, 393)**（⑨ 的新守卫 `tests/unit/test_delta_remerge_l277.py` 是第 7 只本轮新增未跟踪文件）；**同场返工一格**：A366 / A367 / A368 / A369 四行初稿不带分隔空行入表，表结构守卫把 A365 那一块判成断表（案面点名 A365 行首）⇒ 按 Backlog 区既有形状补回四枚分隔空行（净 +4，且落在被指向那一行之上 ⇒ operative 由初钉的 886 推到 890，链尾名册尾值与句首那一格同批改写，本行净 0）；l242 / l253 / l263 / l177 四把的常数全部按守卫打出的实际读数重钉，没有一把改松（逐格读数见本轮记账批）。
- ⑫ **门禁读数（由 L278 记账批按实测填入，本批落盘之后复跑）**：全量定标 **9785 passed / 11 skipped / 0 failed / 474.28s**（收集面 9796）；覆盖率 **TOTAL 14842 语句 / 缺 4 / 4140 分支 / 半分支 2 = 99.97%**，131 个文件满覆盖，本轮改过的 `augmentor/validation.py` 满覆盖（不在缺数名单）；缺数逐格点名 `api/middleware/rate_limit.py:92`（`max_keys` 压回那一支）、`api/middleware/upload_gate.py:74-75`（非十进制 `Content-Length`）、`augmentor/backup.py:121`；11 支 skip 构成 **8 + 1 + 1 + 1** 逐档点名（`test_doc_line_refs_l79.py:647` 作者工作树档 8 支、`test_config_gates_l82.py:573` export 节无开关、`test_serve_config_wiring_l97.py:119` 未构建前端产物、`test_upload_ceiling_l87.py:580` 无真实数据集）；账本与文档守卫集（15 把）**331 passed / 8 skipped / 0 failed / 25.69s**；前端三门禁本轮复跑（虽然零 `web/` 改动，仍按实测记）：`tsc --noEmit` 零输出、`eslint . --max-warnings 0` 零告警退出 0、`vitest run` **22 个测试文件全绿 / 0 失败 / 146.88s**（逐档用例数留在仓外 `front_three_gates_l278.txt`）。

### L278（2026-10-16）— 静态闸自己被量出六格缺陷：域外异常逃逸、两族漏判、一族误伤、一枚扩展记号废掉整把闸，加文档表格竖线的 GFM 口径

- ① **选题与授权**：goal 原文里「修复验证所有 bug 和漏洞」这一句在 L277 之后指向了尺子自己——上一轮给 `require_search_pattern` 装的那把静态闸（`augmentor/validation.py` 的 `find_repeated_ambiguous_group` 一族）是安全判据，而判据本身是一段手写解析器。本轮按三族取证：与 `re` 权威逐形对账（`_re_semantics_probe*.py`）、指数形状实测（`_static_gate_prefix_evidence*.py` 与 `_static_gate_postfix_readings.py`）、把每一形钉进真值表。可回退路径 = revert 本轮 `augmentor/validation.py` 与 `tests/unit/test_pattern_blowup_l278.py` 两文件的 hunk。
- ② **六格缺陷与改前读数**（每形都有改前 / 改后实测）：(a) `_read_repeat_bound` 用 `int()` 直接吃`{}` 里的串，`a{1,2,3}` 与 `a{ }` 当场抛 `ValueError`，而 `_search_regex` 只 `except re.error` ⇒ **域外异常逃出产品入口**（该 400 的走了 500 那条路）；(b) `str.isdigit()` 认 `²` 而 `int("²")` 炸，同一形另一条逃逸路径；(c) `{0,}` / `{,}` 是开区间量词却整族不判，实测 `((a+)){0,}b` 在 n=14 / 16 / 18 / 20 / 22 上 0.00074 / 0.00242 / 0.00864 / 0.03659 / 0.14244 秒（每 +2 字 ×4，与基准 `(a+)+b` 同曲线）；(d) `_skip_char_class` 认 POSIX 嵌套类那一支——那是 `re` 里不存在的语法（实测 `findall("[[:alpha:]]", "a]:b[c") == ['a]']` ⇒ `[[:alpha:]]` 是「`[`、`:`、`a`… 的类」），留着它让「类内含 `[`」的指数形 `[ab[]+)+b` 整族漏判；(e) `(a{0,1})+b` 一族误伤，实测与 `(a?)+b` 在 22 字上都是 0.00000 秒 ⇒ 判据把「长度可变」与「可重复」两维混在一格，改后 `variable = (hi is None or hi > lo) and (lo, hi) != (0, 1)`；(f) **一般性绕过**：任何认不出的 `(?` 扩展让整条模式返回「不判」，一枚前缀就废掉整把闸（改前 `(?i)(a+)+b`、`(?s)(a*)*c`、`()(?(1)a)(a+)+b` 三形读数全是 `None`）。
- ③ **修法**：`_read_repeat_bound` 只吃 ASCII 数字、至多一枚逗号、跨度 ≤ 21、`hi < lo` 视为语法错（与 `re` 同形：`{3,2}` 直接报错）；`_skip_char_class` 删掉 POSIX 那一支，只认「首位置 `]` 算字面量 + 第一枚未转义 `]` 闭合」；新增 `_flag_directive_end` 认纯开关指令；`_group_extension_end` 认不出时**不再回 `None`**，改成把 `(?` 当普通组开头继续扫（栈仍平衡），只有 `(?P<` / `(?<x` 这类「名字没写完就到底」仍回 `None`，而那些形状同时是 `re.compile` 语法错。剩余 `None` 面在新守卫里逐格点名。
- ④ **被本轮证伪的一条旧口径**：条件组 `(?(1)...)` 原先归在「本闸不判的族」。实测 `()(?(1)a+|b)+c` 与基准 `(a+)+c` 同曲线（不是线性），而 `()(?(1)a)(a+)+b` 在改后被判中 ⇒ 旧口径是错的分类，`UNJUDGED_FAMILIES` 从 7 格缩到 6 格、危险面 26 格、合法面 20 格，三张面各由一支现量守卫钉住。本轮**没有把判据放宽一格**：缩的是「不判族」名册，判中面只增不减。
- ⑤ **回归自己踩到的一格（如实记）**：`_read_repeat_bound` 第一稿把 `{4}` 读成「上界缺失」⇒ 既有真值表当场红（`assert ['(?:\\d{2}){4}'] == []`），修法是 `low, comma, high = body.partition(",")` 后 `hi = int(high) if high else (None if comma else lo)`。**教训**：动解析器必须先让既有真值表跑，否则自己的回归会溜进「新守卫全绿」的读数里。
- ⑥ **文档表格竖线的 GFM 口径**：`docs/augmentor/API.md` 里内联码带裸竖线的表行在 GitHub 被切成额外列（渲染与表头错位），本轮把 8 处内联码竖线改成反斜杠转义，并把「数格子」这件事收进 `test_api_doc_endpoint_coverage_l210.py` 的 `column_count` / `split_cells` 两处单一权威，`test_api_doc_field_hints_l223.py` 改为共引（改前两把尺各自按裸 `line.count("|")` 判列数 ⇒ 同一个问题两处两套口径），新臂 `test_the_column_split_follows_gfm_escaping` 钉转义形；`test_docs_markdown_structure.py` 把「内联码竖线不是分隔符」那一格改成双向尺（转义要认、未转义要判）并新增「围栏里的示例表对尺子不可见」一格。两卷账本自己的余面立成 A372（含 A325 与 A344 两行的实测读数），不在本轮自决改动历史档。
- ⑦ **A345 收成半修**：定级格写作 `M 半修（L278）`，走 A156 那一格已有的形状（裸字母 + 半程词，`半修` 进 `MEASURED_WORDS` 而不进权威面的计数），① 那一半（形状闸 + 长度上限，判在语料之前，域外异常也折成 400）本轮落地，② 请求级超时与 ③ 线性引擎立成 A371。
- ⑧ **接线面复核（只有一个用户正则汇聚点）**：全仓 `re.compile` 的用户可控模式串只有 `augmentor/search_enhanced.py:567` 一处，`validation.py` 里编译 `forbidden_patterns` 那一处只吃构造器传入的预设（`ValidateRequest` 不带该字段，由 `test_user_supplied_pattern_l267.py` 钉住）⇒ 闸没有第二个出生点，也没有旁路。
- ⑨ **尺子自证**：新增三支注入臂（摘掉 `_search_regex` 的接线、把静态闸打成恒真、把长度上限打到无限）各自让对应那格红；`TestParserBranches` 里 `test_no_foreign_exception_escapes_the_parser` 对七形断言「只许判中或放行，抛任何异常就红」，`test_library_entry_returns_a_result_not_a_foreign_exception` 与 `test_http_face_no_longer_leaks_a_500_on_a_literal_brace` 把「域外异常逃逸」钉在库面与 HTTP 面两处。
- ⑩ **待复跑与未归因项（不美化）**：全量收集面本轮实测 9796 格（9785 passed + 11 skipped），与上一轮台账数逐档对账仍有 **1 格差未归因**；已把逐文件 collect 基线存到仓外（Temp `l277_probes/collect_baseline_l278.txt`），L279 起以逐文件计数对账钉死，不再用总数推断。缺数三档（`api/middleware/rate_limit.py:92`、`api/middleware/upload_gate.py:74-75`、`augmentor/backup.py:121`）本轮未动，立为 L279 候选。
- ⑪ **修正批（同轮）——12 支账本守卫红的逐格重钉**：账本批落盘后按纪律复跑，12 格红全部是「按现量重钉」类，没有一把尺被改松。l242：`MEASURED_ROWS` 243 → **246**，区段对照那一格由 `292 == 246 + 46` 自行闭合，`test_backlog_ids_are_unique` 的第一断言（无重复 ID）本来就是绿的，红在表行面计数上。l253：`MEASURED_ALL_ROWS` 289 → **292**、`MEASURED_BACKLOG_ROWS` 243 → **246**、`MEASURED_LETTERS` 的 S 71 → **73**（A370 / A372 两格裸 S）与 M 50 → **51**（A371 一格裸 M）、`MEASURED_WORDS` 的「半修」1 → **2**（A345 那一格），未计词名册 `['A156']` → `['A156', 'A345']`；「末格定位不靠格数」那一格从裸 `len(odd) == 8` 收成 **9 且加一条集合相等逐格点名**（第九格是 A372 自己）——这是把尺子收紧，不是放松。宽面 129 与权威面 126 两格**一格未动**：A345 收成 `M 半修（L278）` 之后 `MARK` 认的是「已修（L」而不是「半修（L」⇒ 那一格既不进宽面也不进权威面，半程不算收，与 `A156` 同档。l263：`MEASURED_BACKLOG_ROWS` 243 → **246**、`MEASURED_NO_VERDICT_ROWS` 109 → **112**（三行新账整行没有带轮号的判决语 ⇒ 全走「无判决语」档），盲区 5 与 `MEASURED_HITS` 名册一字未动，`FULL_VERDICT` 仍是量出来的空集。**l263 注入臂换主体的归因（本批新学的一格，双面）**：`test_arm_verb_then_round_without_brackets_is_caught` 的宿主从 `A345` 换到 `A371`，红读数 `assert '半程' == '整行判决'` 点名原因——`SCOPE` 取的窗口是「命中串起点 → 动词后 24 字」，而 `A345` 的正文在本批落了「**L278 落了 ① 那一半**」，那枚 ① 落在窗口里把注入句豁免成半程。一方面臂必须换个窗口干净的宿主才测得到形状②（`A371` 的锚落在定级格之前，窗口里只有 ` | M |`）；另一方面这一格就是「限定词窗口会吞掉真判决」的现场证据，而它的承重不是散文而是 `MEASURED_BLIND` 与 `MEASURED_HITS` 的精确相等——任何一行真掉进半程档都会把盲区推出 5 并当场红，逼人来读那一句。反向那支（「未关闭」不进盲区）与窗口无关，仍留 `A345`。l268：账本 :70 的operative 主张按红读数 `assert 890 == 897` 重钉，`ROSTER` 与散文链尾同批改，三档回到 `{LIVE: 1, STALE: 5, BLIND: 7}`。另记一笔机械教训：**A372 那一行自己的两处账本自引用（`:190` / `:228`）被本批的+1 清单行平移成 `:191` / `:229`**，而 a184 那把可达性尺看不见不带文件名前缀的裸行号引用（A333 / A346 正是这一族），只能人工核对——本批核完并写在这里。
### L279（2026-10-16）— 并发共享态罩锁两案 + 备份名第八档：A364 前两格落罩锁（半修）、A373 立账同轮收口、A374 新立待拍

- ① **本轮动的是哪几格**：A364 名册四格里的第一格（`augmentor/quality.py` 参照集分次写）与第二格（`augmentor/context.py` 的「存-改-复原」），定级由 `S` 收成 `M 半修（L279）`，逐格点名：quality 与 context 两格已收，tracker 与「run-scoped 缓存」两格仍在案。A373（`MemoryCache` 全类无锁 + `_synchronized` 的豁免面定性是假话）新立并同轮收口。A374（`require_within_dir` 族缺冒号档）新立待拍。备份名那一族补第八档（冒号），关掉 A366 收口后仍留在案的一格真实损毁。
- ② **改前行为差（仓外探针 + 守卫内的 no-op 假锁臂，Temp/l277_probes 的 `a364_ngram_*.json` / `a364_vector_*.json` / `a364_context_old.json` / `a364_ambient_nooplock.json`）**：n-gram 支静默污染——A 的候选配 B 的参照集，多样性由真值 `0.5833…` 变 `1.0`，**无异常出声**，等于放行本该被过滤的重复样本；向量支两臂当场崩，`ValueError: operands could not be broadcast together with shapes (3,) (2,)`（写完矩阵停在范数之前）与 `(2,) (3,)`（读完矩阵停在除法之前），前者还把缓存毒成 `(2 条文本, 3 行矩阵, 2 个范数)`——判据与内容不再同源，此后每次调用都崩；context 那一格交错两轮**双方都错**（要 2 轮的拿 5、要 5 轮的拿 3）且实例默认值被永久留在 3→2。
- ③ **修法**：`quality.py` 用同一把 `self._cache_lock` 罩住 n-gram 支、向量支、`reset_cache` 与 `cached_encode` 的「淘汰 + 插入」（四处临界区），顺带把 `_embedding_cache` 的初始化从 `batch_score` 里的 `self._embedding_cache = getattr(self, "_embedding_cache", {})` 收进构造器（并发首调各造一枚空字典再赋回，后赋那一只把前一只已插入的批次整份顶掉）；`context.py` 给 `generate_multi_turn` / `batch_generate` 加 `num_turns` 形参，轮数沿调用链带下去，删掉 10 行存-改-复原。`git diff --numstat` 复核 `context.py 15/15`（净零行）、`quality.py` 净 +7 行——**这 7 行把 A364 行里原先钉的 `quality.py` 第 227 行与 `context.py` 第 196 行两处活引用平移出了代码跨度**，a184 硬 0 档当场 2 处红（`dead_line` 2 == 上限 0），按处方 3 重钉成 `:226` / `:114` / `:203` 并在同一格写明归因，没有任何一把尺被改松。
- ④ **新守卫（罩锁两案共两支）**：`tests/unit/test_shared_cache_lock_l279.py`（19 支 / 零 parametrize）三面——AST 名册（锁区恰好 4 处、参照属性在构造器之外 17 点全在锁内、`_embedding_cache` 锁外只剩两枚 dict 原子读、构造器外对 `self.num_turns` 的赋值 0 处、淘汰与插入同一 `with` 体）＋ 两线程行为对账 ＋ 反证与尺子自证（假锁臂重现 `1.0` 污染 / 两形 `ValueError` / 毒态 `(2, 3, 2)` / `pop` 的 `KeyError`；「存-改-复原」替身臂重现 `{A: 5, B: 3}` 与默认值 3→2；HEAD 版裸写形状的合成语料判 11 处裸点，换一把不相干的锁同样判红）。`tests/unit/test_memory_cache_lock_l279.py`（13 支 / 零 parametrize）见 ⑪。定点复跑：前者 19 passed / 7.3 s（两次无抖动），后者 13 passed / 1.55 s。
- ⑤ **仪器假绿一次（如实记，不美化）**：context 的第一版驱动只在 `batch_generate` 外面挂一只钩子，old/new 两份读数**完全相同**——并发静默退化成顺序，与 L275「watcher 盯错信号」同一类缺陷。改成三段事件（A 停在已写未读、B 停在已写未读、A 跑完并复原后才放行 B）才逼出 old 的 `{A: 5, B: 3, 实例 3→2}`。这条已写进守卫的 helper docstring：调度钩子必须双向对齐。
- ⑥ **环境事实与一格差的归因**：本机 venv 里 `matplotlib` / `wordcloud` / `sentence_transformers` 全部 MISSING、numpy 在 ⇒ `QualityScorer._model == "fallback"`，向量支只能由守卫内的 FakeModel 替身走到（生产装齐依赖时同一条路）。另把上一轮遗留的「1 格差」归因收口：`web/dist` 缺失（且 git 不跟踪）⇒ `tests/unit/test_serve_config_wiring_l97.py:119` 由 passed 翻成 skipped（passed −1 / skipped +1，collect 面不变），复跑读数 `6 passed, 1 skipped`。**曾用 `git archive` 取 L276 历史树做 collect 基线，被 2 个 collection error（`test_api_doc_field_hints_l223.py` 于模块级 import 另一只守卫的 `schema_keys`）污染成 9491 的不可用读数 ⇒ 该考古路线弃用**，改走「按文本读台账 + 现场复跑单文件」归因。
- ⑦ **判定不改的三格（负结论同样入账）**：`augmentor/models/base.py::_cache_put` 的 `getattr` 惰性造缓存——最坏后果是少命中若干条、窗口一次性，不改（罩锁之后同一格的两形竞态已由 `MemoryCache` 内部那把锁接住，惰性首用只剩「多造一只又被顶掉」的形状）；`augmentor/indexer.py::search_ngram` 在 `n<=0` 时把整表回成「全部匹配」（实测 `n=0` 与 `n=-2` 都返回 `[0, 1]`），但产品面 `search()` 恒用 `n=2`、HTTP/CLI 的可达路径 `search_enhanced` 已在 `:374` 判掉 `ngram_n >= 1`，故只是 SDK 裸调用面的形状，不加投机判据；`analytics.py` / `statistics.py` / `indexer.py` 的 `load()` 是「共享实例改 `_items`」形状，但逐个构造点查下来全是每次新建（`statistics.py:377` / `:391`、`analytics.py:502`、`indexer.py:564`），HTTP 面不共享，不成立。另验 `api/middleware/write_auth_gate.py` 的 `API_PREFIX="/api/"` 不漏面：按 `app.openapi()` 枚举 53 只写操作，`/api/` 之外 **0 只**（非 GET 的 `/docs` 一族只读）。
- ⑧ **A364 剩下的两格（不在本轮自决）**：`augmentor/tracker.py:86` 的 `_current_experiment`——收口要给 `log_metric` 加把手参，属对外签名面；「run-scoped 缓存」替代现在这把锁要动公开签名与 pipeline 循环、砸掉 `test_quality_norm_cache_l246.py` 的注入接缝，并会失去逐条摊销，「并发两批互相逐出」的 thrash 也仍在案。两格连同上面 ⑦ 的形状一起留在 A364 那格里待拍。
- ⑨ **覆盖率三档缺口全部改判并补口（上一跑 99.97% 的回归已推回 100.00%）**：`backup.py:120-121` 的落点闸原判「死支、应删除」被 drive-relative 探针**证伪**——`\\?\C:\evil`、`D:evil`、`\\\\nas\\share\\x`、`\\absolute`、`../victim` 五形都能走到那一行，故保留旧闸并补名册臂（identity monkeypatch 逼出「指向备份目录之外」文案，并验零落盘）；`rate_limit.py:92` 的窗口滑动（`while … popleft()`）由 `max_requests=1` 那只恒打不到（整键被 `_sweep_locked` 删掉），补一支三只命中跨窗口的行为测试，钉住「摘错格子（该摘最旧那只）」与 `Retry-After` 按最旧那只算；`upload_gate.py:74-75` 非数字 `Content-Length` 的兜底此前只有 multipart 那把尺覆盖，补 `TestDeclaredContentLengthReading` 七法（垃圾尺寸放行、`-1` 判不可见、`b"1_024"` **不是**垃圾——`int()` 认下划线，第一次就断在这格，改用 `b"1.5"` / `b"7,000"`）。三档补完后全量 `TOTAL 14841 0 4140 0 / Total coverage: 100.00%`，本项目自定的 100% 不变量恢复。
- ⑩ **第八档（冒号）与其间的两次返工**：`require_path_segment` 加 `":" in value ⇒ DataValidationError`，关掉 A366 收口后仍留在案的真实损毁。两次返工都记下来：其一，`backup.py` 那格原判「永不执行的死支」被证伪（见 ⑨）；其二，第一版别名探针的 `arm()` 在比对之前先 DELETE，据此写下的「后一次创建把索引里前一行整行顶掉」是**探针收尾动作造出的假结论**——用不删、两份不同源的第二版探针重测，真形是两份名字（`C:foo` 与 `foo`）**并存两行、共用一只文件**，`DELETE /C:foo` **200** 之后 `foo` 行成幽灵、`POST /foo/restore` **404**、verify 读 `exists:false`。守卫与源码注释两侧都按真形改过（`validation.py` 冒号档注释第二句现在是「按其中一个 id 删除就毁掉另一条的载荷」）。同族还量到 `C:` 落到 `.json`（文件名被吞光）与 POSIX 上 `C:foo` 合法 ⇒ 同一枚 id 两平台两个落点。`tests/unit/test_backup_name_path_segment_l277.py` 因此从 12 臂走到 16 臂（名册三格同步：`colonish == ["C:foo", "C:", "a:b", "D:evil"]`、判据档数 `== 8`），另加 `TestTheColonIsSwallowedByPathlib` 四法与 `TestTheLandingGateIsStillLive`。
- ⑪ **A373（`MemoryCache` 无锁）与「豁免面」那句假话**：详见 A373 那行。取证口径值得单独留一句：**概率锤不算证据**——8 线程 × 4000 轮在 0.02 秒内 0 错误，窗口只有几条字节码宽，任何「跑了 N 次没出事」的读数在这一族都不成立；能作证据的是把 `self._cache` 换成停在指定那一步的 dict 替身（`__contains__` 停在取值**之后**、`__getitem__` 停在取值**之前**，各忠实复刻对应形状的真实交错）。同文件 `_synchronized` 的 docstring 原判「`MemoryCache` 只碰进程内的 dict/计数器，竞态最坏结果是少统计一次」是错的，已改写成「L279 把这句话证伪了」并留下被证伪的原话。守卫三面的反证臂读数：假锁下 `KeyError('a')` / `KeyError('b')` 原样回来、真锁下 B 等 0.25 秒进不去；合成语料摘两格装饰器判红且逐格点名 `{'get', 'clear'}`。
- ⑫ **A374（`require_within_dir` 族缺冒号档）新立待拍**：四格实测量在仓外 `Temp/l277_probes/colon_within_dir.json`（基座 `C:\base\backups`）——`C:foo` **放行**、落点 `C:\base\backups\foo`（落点名 ≠ 回执 id，`path_lands_within` 判不出）；`C:` **放行**、落点直接是基座目录本身；`a:b` 与 `D:evil` 判 400 但文案是「不能指向 backups 目录之外」，对 `a:b` 属误导（真实形状是基座被整个丢掉）；基座为相对路径时 `C:foo` 依 cwd 解析成 `C:\backups\foo`、`a:b` 解析成 `a:\backups\b`。要不要把这档下推进权威，改的是九模块 id 面的对外回执（今天放行、改后 400）与 400 文案口径 ⇒ 不自决。
- ⑬ **普查与账本尺的按现量重钉（逐格点名，没有任何一把尺被改松）**：l253 `MEASURED_LETTERS` `S 73 → 72` / `M 51 → 52`（A364 那一格从裸 S 换成裸 M 加「半修」用词）、`MEASURED_WORDS` `半修 2 → 3`、半程名册 `['A156', 'A345'] → ['A156', 'A345', 'A364']`；l177 语料规模 `(133, 394) → (133, 395) → (133, 396)`（两次都是新增守卫文件本身，docstring 里逐只点名）；l97 `MEASURED_CALLS 445 → 447`、`MEASURED_READABLE 324 → 326`（`test_upload_gate_l90.py` 补口那两支内联字面清单 parametrize，盲面 121 一格未动，`326 + 121 == 447` 闭合），l98 从 l97 共引同一格自动跟上；l259 名册键 `cache.py:245 → cache.py:262`（本轮 `cache.py` 净 +17 行全部落在 `_get_cache_path` 之前，四处拼接与处置理由一字未动）。本批账本落地后按同法复量并以红读数重钉，逐格：l242 `MEASURED_ROWS 246 → 248`（红读数 `assert 248 == 246`，区段对照那一格同时逼出 `assert 294 == (248 + 46)` ⇒ 被过滤的 46 行一字未动，两行新账确实落进 Backlog 区）、`MEASURED_DECLARED 108 → 109`（本轮清单行与日志标题各点一次 A373，同一只 ID 只算一笔；不过滤那一面由配对格从 146 走到 `assert 147 == (109 + 38)` 自行闭合）、`MEASURED_MARKED 126 → 127`（A373 那一格走清单里已有的 `划掉 M + 已收口 + 括号 L` 组合，没有造出第 20 种形状）；l253 `MEASURED_ALL_ROWS 292 → 294`、`MEASURED_BACKLOG_ROWS 246 → 248`、`MEASURED_CLOSURE_CELLS 126 → 127`、`MEASURED_WIDE_ROWS 129 → 130`、`MEASURED_SAME_VOLUME 113 → 114`、`MEASURED_LETTERS` 裸 `M 52 → 53`（A374 留裸 M 待拍）与 `~~M~~ 54 → 55`（A373）、`MEASURED_WORDS` 已收口 `52 → 53`、`MEASURED_COMBOS` 的 `('~~M~~', 已收口, L, 不粗体)` `20 → 21`；l263 `MEASURED_BACKLOG_ROWS 246 → 248` 与 `MEASURED_NO_VERDICT_ROWS 112 → 113`（闭合式 `130 + 5 + 113 == 248` 与 l253 的宽面、本守卫的盲区两格同批对上；盲区 5 与 `MEASURED_HITS` 名册一格未动）。**另记一处 :117 清单行插行的平移代价**：本轮清单行落在 117，加上 Backlog 两行与它们各自的分隔空行，117 之下共 +5 行 ⇒ 卷二唯一那格 operative 自指那一格（原写 897）被推到 **902**（`test_self_line_claim_l268` 的独立定位器现量 902，`LIVE 1 → 0` / `STALE 5 → 6` 两档同时红），按处方 2 在 L232 那一行**同行改写** 897 → 902（净零行）后 `LIVE` 回到恰好一格；仓外另一处活引用（`tests/unit/test_backlog_closure_l242.py` 第 151 行注释里指向 A362 那一格的那个行号，原写 265）同批被推成 266，a184 的硬 0 档当场以 `dead_line` 点名它（265 那一行现在是分隔空行），一并按现量重钉——两侧行号都用文件现量对过，不是按「应该移了几行」推的。
- ⑭ **门禁**：受影响既有测试定点 `tests/unit/test_context.py`、`test_quality.py`、`test_quality_norm_cache_l246.py`、`test_embedding_cache_bounded_l247.py`、`test_quality_dedup_gates_l76.py` 共 **283 passed**；罩锁改动邻面 `-k "cache or model or generation or base"` **1083 passed**（无回归、无死锁）；`-k "backup or path_segment or l277"` **354 passed**；六把账本与文档守卫按红读数逐格重钉后 **155 passed / 8 skipped**（8 支 skip 全是 `test_doc_line_refs_l79.py:647` 的 authoring-tree 档：`Temp/` 取证据与 `data/` 数据集未跟踪，与本轮无关）；十二把普查与账本尺加本轮两支守卫合跑 **292 passed / 5 errors**，那 5 支 error 逐字点名是 `test_parametrize_blind_face_l98.py` 的动态面档（`RulerNotProven`：本次只采到远少于 3000 条用例时按设计拒收），在下面的全量里为 0 failed；全量逐字 `python -m pytest tests -q -p no:cacheprovider` 读出 **9840 passed, 11 skipped, 0 failed（449.79s）**，覆盖率 **100.00%（`TOTAL 14849 0 4140 0`）**——⑨ 里那格 `14841` 是补覆盖率之前的中间读数，`+8` 语句正是本轮 `MemoryCache` 的七格装饰器与构造器里那把锁，保留不美化。**落账复验（v4）**：清单行、A373 / A374 两行、17 行日志块与五面常数重钉全部落盘之后，逐字同一条命令复跑读出 `9840 passed, 11 skipped, 0 failed（356.96s）`、覆盖率仍是 `100.00%`、`TOTAL 14849 0 4140 0` 一格未动；六把复跑仍是 `155 passed / 8 skipped`、十二把加本轮两支仍是 `292 passed / 5 errors`（就是那 5 支动态面档的 `RulerNotProven`）⇒ 这一批写作面对全量计数零影响，是复跑读数而不是推定。前端三门禁**不适用**（本轮 `web/` 零改动——注：L277 之前的批次动过 `web/`，那一轮的读数记在 L277 块里）。
- ⑮ **无 commit 授权**：本轮全部改动（产品码 `quality.py` / `context.py` / `cache.py` / `validation.py` / `backup.py` 侧判据 + 两支新守卫 + 账本）仍未提交，L277 / L278 / L279 三行的 hash 槽各留 `哈希待 L<n> 回填` 占位，占位格数按守卫现量记账，不美化。

### L280（2026-10-16）— DiskCache 的「判过存在再动手」五格 + 单条超上限毁掉整本缓存：A375 立账同轮收口、A376 新立待拍

- ① **本轮动的是哪几格**：A375 一整格（`augmentor/cache.py::DiskCache` 的缺文件语义五处 + `set` 的自逐出），立账同轮收口。A376（跨实例共用目录时索引被全量重序列化顶掉）新立待拍。L279 那三格（A364 剩下两格、A373 已收、A374 待拍）本轮一格未动。
- ② **改前行为差（仓外探针，`Temp/l277_probes/l280_cache_toctou.json`）**：`delete` 与 `clear` 与 `_enforce_size_limit` 三形在「判过存在之后文件消失」时**必现** `FileNotFoundError`；第四处 `stats` 是守卫尺子自己点出来的（首轮红读数逐字是 `stat_unprotected: ['stats:426']`），不是照着前三个凑的。单实例多线程对照 `errors: []` ⇒ 这一族不是锁的问题，是 check-then-act 与跨实例两份账的问题。
- ③ **自逐出那一格（`Temp/l277_probes/l280_cache_selfevict.json`）**：`max_bytes=400`，先存三条 20 字节小条目，再写一条 3000 字节 ⇒ 改前 `_metadata` 与目录里的 `*.json` 同时变成空集、`get` 那条大的读回 `None`，第二轮（`small_a`/`small_b` 在册）复现同形。归因是淘汰名单里没排除刚写入那一条：`entries.sort` 按 `created_at` 从旧到新，逐到 `total <= max_bytes` 为止，而单条超线时永远达不到，于是把整本逐空后连自己那条一起删。改后 `protect=key` 只贡献体积、不进名单，单条超线时其余条目一律不牵连并出 warning（代价是目录可以比上限多一条）。
- ④ **修法与可达性**：`delete` / `clear` 走 `unlink(missing_ok=True)`；`_enforce_size_limit` 与 `stats` 把 `exists()` + 裸 `stat()` 换成 `try/except FileNotFoundError`（消失的文件按「已经不在目录里」计）；`_load_metadata` 直接开文件、缺失即空账；`_enforce_size_limit(protect=None)` 默认值保持旧调用形状，`tests/unit/test_partial_branch_residue_l266.py` 那两档（`max_bytes=1` 全逐、`keep = 新条 + 1` 只逐最旧）一字未动仍然通过。可达性按调用点量：HTTP 生成路径读缓存那一行不带 try，同一函数里只有写那一侧罩了 `OSError`；`get` 的过期分支正是 `delete` 的产品调用点 ⇒ 一次外部删除就能把本该算未命中的生成判成失败。
- ⑤ **新守卫**：`tests/unit/test_disk_cache_missing_file_l280.py`（24 支 / 零 parametrize）三面——AST 名册（`unlink` 恰好 3 处且逐处声明 `missing_ok=True`、`.exists()` 只剩 `get` 那一处、每一枚 `.stat()` 都在 `except FileNotFoundError` 之下、`set` 把 `protect=key` 交下去、`get` 的 `open` 落在 IOError 之下）＋ 真临时目录行为对账 ＋ 反证与尺子自证。定点复跑 **24 passed / 1.0 s**。
- ⑥ **仪器假绿与尺子缺陷（如实记，不美化）**：**（甲）** 探针第一版把 drop 写在 `exists()` 判断**之前**、且每次调用新造一只替身（`fired` 被重置），两臂同时假绿，`get_expired_branch` 那格的 `ok` 读数是替身形状失真造成的，不是产品安全 ⇒ 改成「先按真实结果返回、再在动手那一步之前删」并按 key 缓存替身。**（乙）** 尺子首轮 7 支红，其中三处是尺子自己的缺陷：`open(...)` 是 **Name 调用**、`_attr_calls(node, "open")` 恒空 ⇒ 那处豁免被误判 False；`missing_ok=False` 曾被当成「已声明」⇒ 换修法那一臂的 `bare_unlink_sites` 读成空集；`self._enforce_size_limit(...)` 是**属性调用**、按 Name 找 ⇒ `set_passes_protect` 恒 False。**（丙）** `clear` 那一臂最初断言「glob 出的每只替身都开火」，但 `_metadata.json` 也在 glob 结果里且被 owned 名单跳过 ⇒ 恒不 fired，判据改成「owned 两只必开火、索引那枚必不开火」，两半都有牙齿。**（丁）** 本轮把 `_enforce_size_limit` 的第二档写成 `if/elif` 之前，正臂量到「超大条目留着但三条小条目仍被逐空」——那条 `if total > max` 在超线条目下永远达不到，等于换了一种方式毁缓存；改为 `elif` 后正臂才真的成立。
- ⑦ **判定不改的格（负结论同样入账）**：`DiskCache.get` 里 `if not cache_path.exists(): return None` 之后那次 `open` 仍是 check-then-act，但读侧整体罩在 `except (json.JSONDecodeError, IOError)` 之下、`FileNotFoundError` 是 `OSError` 的子类 ⇒ 消失的文件已被折成未命中，不加投机判据；`set` 写值那一次 `open(cache_path, "w")` 没有 try，但它的失败由 `models/base.py` 写那一侧的 `except OSError` 接住，且新目录不可用时构造期就先炸（`mkdir` 在 `__init__`）；`api` / `cli` 面对 `DiskCache` 的 `delete` / `clear` / `stats` 现量 0 个调用点（grep 全仓只命中 SDK 与 `cached` 装饰器的 `wrapper.clear_cache`）⇒ 这三格是 SDK/运维面，不据此抬级别。
- ⑧ **A376（跨实例两份账）新立待拍**：`_save_metadata` 是整本 `_metadata` 的全量重序列化 ⇒ 后落盘那一份把对方新增的键从索引里抹掉，磁盘文件留下、`_enforce_size_limit` 只遍历 `self._metadata` ⇒ **上限对孤儿失明**，`delete` 那一侧还回执 `True`。两侧实例锁 `locks_are_same_object` 现量 `false`。三条候选（文件锁 / 增量写换索引格式 / 按目录共用实例）分别要动依赖、存量数据可读性、构造器身份语义 ⇒ 不自决，与 A364 剩下两格和 A374 同批待拍。
- ⑨ **落点代价与按现量重钉（没有任何一把尺被改松）**：本轮产品改动净 +30 行全部落在 `_get_cache_path` 之前 ⇒ l259 名册键由 262 顶成 **268**（红读数逐字是 `At index 1 diff: 'augmentor/cache.py:268' != 'augmentor/cache.py:262'`，同一处 md5 拼接、处置理由一字未动）；卷二 L259 日志块里那格指向 md5 拼接的活引用由 244 被顶出代码跨度 ⇒ `test_comment_refs_a184` 硬 0 档当场点名 `dead_line`（红读数把它与注入探针那一格一起列出：`assert 2 == 1`），按处方 3 重钉成 268 并写明归因。A375 / A376 两行 + 本轮清单行的插入对 l242 / l253 / l263 / l268 / l177 的计数代价按红读数逐格重钉，见本块 ⑫。
- ⑩ **门禁**：新守卫定点 24 passed / 1.0 s；cache 与 model 邻面 `-k "cache or model or generation or base"` **917 passed**（无回归、无死锁）；cache 面既有测试加 l259 加两支罩锁守卫合跑 **252 passed / 1 failed**，那 1 支正是 ⑨ 的名册键平移，重钉后转绿；六把账本与文档守卫本轮首跑 **155 passed / 8 skipped**（8 支 skip 全是 authoring-tree 档，与本轮无关）。全量逐字 `python -m pytest tests -q -p no:cacheprovider` 复跑得 **9864 passed / 11 skipped / 0 failed（369.24s）**，覆盖率 `TOTAL 14863 0 4140 0 = 100.00%`（行与分支双 0 缺，134 files skipped due to complete coverage，阈值 80% 达成）——较 v4 的 `9840 passed / 14849 stmts` 恰好多 24 支测试与 14 条语句，正是本轮新守卫那 24 支与 `cache.py` 的 +14 行有效语句，**同一张账的两个面各归各位**。重钉后六把复跑仍是 **155 passed / 8 skipped**、十三把普查与名册尺子集 **303 passed / 5 errors**（5 支 error 全部是 `test_parametrize_blind_face_l98` 的 `MIN_COLLECTED_ITEMS=3000` 自拒档：子集只 308 项 ⇒ 静态尺不放行，与本批无关、全量门禁里它们正常通过）。前端三门禁**不适用**（本轮 `web/` 零改动）。
- ⑪ **无 commit 授权**：本轮全部改动（产品码 `cache.py` 五格 + 新守卫 + 账本）与 L277 / L278 / L279 三批同样仍未提交，占位格因此按守卫现量记账，不美化。
- ⑫ **记账批落盘后的六面尺子重钉（逐格按红读数，没有任何一把尺被改松）**：l242 表行 `248 → 250`、声明面 `109 → 110`、定级格面 `127 → 128`（红读数逐字 `assert 250 == 248` / `assert 110 == 109` / `assert 128 == 127`，另由区段格逼出 `assert 296 == (248 + 46)`、由配对格逼出 `assert 148 == (109 + 38)` ⇒ 被过滤的 46 行与日志名册那 38 只一字未动，这就是「两行落进 Backlog 区而不是日志区」的证据；`gap` 仍是 `PARTIAL` 四格——A376 是待拍而不是半程，它不进声明面也不造假欠账）。l253 全面 `294 → 296`、Backlog `248 → 250`、闭合格 `127 → 128`、宽面 `130 → 131`、本卷档 `114 → 115`、字母面裸 M `53 → 54` 与划掉 M `55 → 56`（裸 S 72、划掉 S 63、L 1、划掉 L 3、划掉 XS 1 五档一格未动）、词汇面「已收口」`53 → 54`（已关闭 53 / 已修 20 / 关闭 1 / 半修 3 四档未动 ⇒ 没有新造第 6 个闭合词）、组合面「划掉 M 加已收口加括号轮号不加粗」那一档 `21 → 22`（19 格组合名册一格未增 ⇒ 没有造出第 20 种形状）。l263 表行 `248 → 250`、无判决语档 `113 → 114`，三面闭合式走到 `131 + 5 + 114 == 250` 与 l253 的两面同批对上（盲区 5 与命中名册一格未动）。l268 把 operative 名册键与链尾归因改成 `902 → 907`，账本清单区那一格的自指主张同行改准 ⇒ 三档先红在 `LIVE 1 → 0 / STALE 5 → 6`、改准后回到 `LIVE 1 / STALE 5 / BLIND 7`，站点总数 13 一格未动。l177 语料规模 `(133, 396) → (133, 397)`——第 11 只未跟踪测试文件就是本批新守卫，docstring、断言与 f-string 三处同数。a184 那把尺本轮还点名两格旧值：l242 注释里对 A362 那一行的引用按 grep 现量改准 266 → 267（硬 0 档 `dead_line` 当场逼出），同一行注释里对 A365 那一行的旧值按现量补两格到 273——后者不带卷名、a184 扫不到，旧值本来就少一格，本轮照实改、照实记，不美化。

### L281（2026-10-16）— 把「判过存在再动手」那把尺子推到全仓：普查出十格、逐格收口，并抓出代码与自己写定的 200 承诺相反的那一格

- ① **选题来自一次只读普查，不是印象**：上一轮（L280）在 `DiskCache` 里收了这一族的第一批，本轮把同一条判据写成仓外只读 AST 探针，扫 `augmentor/` + `api/` + `cli/` 全部 **133 只产品文件**，判据是「同一只路径先过 `exists/is_file/is_dir`，再在能兜住缺文件的 handler 之外对它动手（`unlink/stat/rename/replace/mkdir/open/copy/move/…`）」。改前读数 **10 格 / 5 只文件**，逐格点名：`atomic_write` 的 `atomic_write_json` 与 `atomic_write_text` 各一只 `finally` 清理（各自配在自己那一行上面的 `exists`）、`backup` 的 `delete_backup`（unlink）与 `verify_backup_integrity`（stat）、`checkpoint` 的 `_remove_delta` 与 `delete_checkpoint`（各一只 unlink）、`data/image` 与 `data/audio` 的两只 `process_*`（各两格 stat 配同一只 `exists`）。改后同一把探针复跑：**133 只扫到、0 格命中**。
- ② **可达性逐格坐实才动手**（这条决定级别，不是修辞）：`POST /api/multimodal/scan` 走 `process_directory` ⇒ `iterdir()` 先把整目录排成队列，再逐只 `fuse_modalities` → `process_image` / `process_audio`，所以**队列里任何一只文件在轮到它之前消失，那一次裸 `stat()` 抛的 `FileNotFoundError` 落在 try 之外，会让整次扫描 500**——一份本来就该报成「文件不存在」的文件把别人的结果一并带走。备份那一族经 `run_in_thread` 直接坐在 HTTP 面上，且 `DELETE /api/system/backups/{backup_id}` 的 responses 明文写着「索引有条目而备份文件已丢不算这一档，仍回 200」⇒ 那一格不是健壮性建议，是**契约违背**。`checkpoint` 的两格有产品内调用点（增量并入主文件之后调 `_remove_delta`，断点删除走 `delete_checkpoint`），`api/deps.py` 侧的异步恢复面 L277 已接通过。
- ③ **修法按 A375 已定的口径逐格走，不新造第二套**：删除类摘掉 `exists()` 预检、改 `unlink(missing_ok=True)`（共五格：`delete_backup` × `_remove_delta` × `delete_checkpoint` × `atomic_write` 的两只清理）；读取类把「判过存在」换成「动手处自己兜」——`verify_backup_integrity` 把 `stat` **与紧随其后的那次整份读**（`_calculate_checksum` 直接 `open`，同一只文件、同一个窗口）一起罩进 `except FileNotFoundError`，缺档按该函数自己的诚实读数计（`exists: False`、`checksum_match: False`），而不是让一次完整性巡检因为一份被并发删掉的备份而整份失败；两只 `process_*` 把取尺寸那一步提到 `is_supported` 之前并罩住，缺失就交回与早退那一格逐字相同的 `valid=False, error="文件不存在"`（unsupported 分支复用同一个 `size_bytes`，读数与改前对在册文件完全相同）。
- ④ **A378 是这一轮唯一的「异常顶替」格**：`atomic_write` 的 `finally` 里那句裸 `tmp.unlink()` 失败时（Windows 上杀软/索引器暂占句柄 ⇒ `PermissionError`），**`finally` 抛出的异常会顶替上面那次真正的写失败**，于是模块 docstring 承诺的「异常原样上抛、本模块不吞异常」被换成一句不相干的清理报错，调用方按「清理失败」判决，「写失败」这一事实凭空消失。修法把清理收成一只 `_discard_tmp()`：`missing_ok=True` 管「已经不在了」，`except OSError` 管「删它这件事失败」并 `logger.warning` 出声而不顶替。为什么不干脆吞掉：留在盘上的临时件带前导点 + `.tmp-` 中段、不匹配任何目录扫描，代价是垃圾而不是坏答案——所以这里出声即可，不升级成失败。为此新增模块级 `logging`（此前这只文件没有 logger）。
- ⑤ **新守卫三面**（`tests/unit/test_missing_file_family_l281.py`，34 支 / 六处 parametrize，六格值位全是内联字面清单且都写字面 `ids=`）：①面是全仓普查尺（语料规模另钉一格，防「换目录/加过滤」把量不到的 0 变成假绿，并有一支断言这五只文件真在扫描面上）；②面是十格逐格名册——**名册管的是①看不见的那一半**：预检一摘掉，「裸 `unlink()`」在语料尺上就没有配对锚、结构性隐形，所以名册逐格判 `missing_ok` 取值、判 `stat` 是否落在缺文件 handler 之下、判 `finally` 体恰好是一只尽力清理的调用；③面是确定性消失路径替身的行为对账，含 HTTP 臂（替身挂在 `Path` 工厂上，只把名单里那一只换成替身，其余原样交回真实现，避免索引/锁/校验和连坐）。
- ⑥ **插行前置量**（纪律第 5 节，动手前先 grep 全仓活引用再决定落点）：`checkpoint.py` 的活引用最大落到 `update_progress` 那一行（443）⇒ 它的**下面**那一格 `delete_checkpoint` 可自由增减行，而**上面**的 `_remove_delta` 必须净零行——本轮用「一行注释 + 一行 `unlink(missing_ok=True)`」换掉「`if exists:` + 裸 `unlink`」两行，实测 `def update_progress` 仍在第 443 行；`backup.py` 的活引用最大 199、`data/image.py` 最大 135-136（都在这两格之上）⇒ 自由；`data/audio.py` 与 `atomic_write.py` 全仓零处活引用 ⇒ 自由。改完六把账本与文档守卫在**写账本之前**先跑一次：a184 与 doc_line_refs 一字未红，实证这五只文件的插行落点没有平移任何活引用。
- ⑦ **负结论与刻意不动的格（同样入账）**：`backup()` 里那句取 `file_size` 的 `stat` 紧跟自己刚完成的原子落盘、且它没有配对的 `exists()` 预检，普查尺不报它，本轮也不动——「刚由本函数写出来的文件」与「索引里记着的一只可能已被删掉的档」不是同一件事，把它一并罩住只会把真正的写失败也吞掉。`DiskCache.get` 的 open-after-exists 那一格在 L280 ⑦ 已判「按未命中重算是可恢复代价」，本轮不翻案。普查尺的配对判据有它的天花板（只看同函数内的同名预检），这一格天花板由名册面补，不假装①面面俱到。本轮**只动 Python 产品面 + 测试 + 账本，`web/` 零改动** ⇒ 前端三门禁不适用。
- ⑧ **本轮过程缺陷如实记**：新守卫的第一版把三处「反证臂」写成靠全文正则扫 `missing_ok`，实测对 HEAD 形状**恒绿**（预检一摘掉就没有配对锚，①面瞎）——这不是推测，是第一版跑出来 34 支全绿之后逐臂改名册尺才暴露的：名册面若不独立存在，六支「翻 `missing_ok` / 换 handler」的注入臂全部没有牙齿。第二版把尺子拆成「吃源码文本、吐读数」的四只函数（`bare_unlink_lines` / `unguarded_stat_lines` / `finally_call_names` / `check_then_act_hits`），注入臂改成喂合成语料、零落盘，并把「①面在这一格是瞎的」这一事实写进名册面的 docstring 而不是藏起来。
- ⑨ **尺子重钉明细（逐格按红读数，没有任何一把尺被改松）**：**写账本之前**先跑六把账本与文档守卫 ⇒ 全绿，实证五只产品文件的插行落点没平移任何活引用（`checkpoint.py` 的 `def update_progress` 实测仍在第 443 行、`_remove_delta` 那一处走同行改写净 0）。**写账本之后**a184 的硬 0 档红一处 `dead_line`，点名的正是 l242 常数区那句带卷名的活引用：`OPTIMIZATION_LOOP_2.md` 的 A362 表行被本批清单行下移一格 ⇒ 按 `grep -n -a "^| A362 |"` 现量重钉 267 → **268**，同一格里 A365 那串不带卷名（a184 扫不到）也照现量改准 273 → **274**，两处都在原注释里续写平移链而不抹旧值。**l242**：表行 250 → **252**、声明面 110 → **112**（清单行写下「另立并同轮收口 A377 / A378」，`BEFORE` 档摊成两只 ID 各算一笔；日志标题只用「逐格收口」不点名 ⇒ 不额外进账）、定级格面 128 → **130**；三条闭合式自行对上，红读数逐字 `assert 252 == 250` / `assert 112 == 110` / `assert 130 == 128`，配对那格逼出 `assert 150 == (112 + 38)` 与 `assert 130 == (130 + 0)` ⇒ 被过滤的 46 行与日志块标记面各一格未动。**l253**：全面 296 → **298**、Backlog 面 250 → **252**、宽面 131 → **133**、权威面 128 → **130**、本卷轮号可追溯 115 → **117**、字母形 `~~M~~` 56 → **58**、闭合词 `已收口` 54 → **56**、组合名册 `划掉 M / 已收口 / 括号 L / 粗体` 5 → **7**（落进已有形状，名册没有多出一格，两面之差仍是 `TWO_FACE_ROSTER` 那三格）。**l263**：表行面 250 → **252**，闭合式 `133 + 5 + 114 == 252`，盲区 5 与 `MEASURED_HITS` 名册一格未动。**l268**：operative 那一格按 ⑤ 内容定位器现量唯一命中重钉 907 → **912**（本批在被指向那一行之上落 5 行 = 清单行 +1 与 A377 / A378 连带各自分隔空行 +4；日志块那 11 行落在它之下 ⇒ 不参与平移），账本第 70 行那句自指同行改写、链尾补值；红读数先是 `assert {'LIVE': 0, 'STALE': 6, 'BLIND': 7} == {'LIVE': 1, 'STALE': 5, 'BLIND': 7}` 与名册那一格不同，改写后两档回到 `1 / 5` 并由 `test_the_operative_number_matches_the_independent_locator` 独立复验。**普查尺**：l177 语料面 397 → **398**（产品面 133 一格未动）、l97 的 `MEASURED_CALLS` 447 → **453** 与 `MEASURED_READABLE` 326 → **332**（闭式 `332 + 121 == 453`，盲面四档未动）、l98 的 `MEASURED_EXEMPT_SITES` 27 → **33**（六支新用例逐格点名；豁免名单变长只是动态面放过的那一格多了新成员，值位那一维仍逐格判）。
- ⑩ **门禁读数**：全量定标逐字 `python -m pytest tests -q -p no:cacheprovider` ⇒ **9898 passed / 0 failed / 11 skipped，423.86 s**，覆盖率 `TOTAL 14875 语句 0 未盖 4130 分支 0 半盖 ⇒ 100.00%`（语句数随本轮产品改动增加，阈值 80% 远超）；skip 的真实构成 8 + 1 + 1 + 1 与既有归因同。写账本之后的账本与文档全套（六把加 l242 / l253 / l263 / l268 / l264）**266 passed / 8 skipped**。定点三面尺复跑 **75 passed + 5 error**，那五支是 l98 在子集跑时的 `MIN_COLLECTED_ITEMS = 3000` 自拒档，全量跑里不红（本轮 v6 那一次就是全量，五支未出现在 skip/failed 名单）。新守卫定点 **34 passed**、邻面 **1022 passed**。前端三门禁不适用：`web/` 零改动。
- ⑪ **无 commit 授权 ⇒ 本轮改动全部未提交**：本会话从未获授权执行 `git commit`，L277 / L278 / L279 / L280 / L281 五批产品码 + 测试 + 账本改动都还在工作树里，清单区这五行的哈希槽只能各留 `哈希待 L<n+1> 回填` 占位（卷二 8 格、卷一 29 格为历史既有形状，`test_ledger_hash_slots_l98` 本轮实读未红）。授权后的顺序按纪律走：先按显式路径 stage 只属于本项目的文件（不用 `git add -A`）⇒ 产品 commit ⇒ 账本 commit ⇒ 把上一轮哈希回填进它所属轮号那一行的槽，回填后重跑六把。

### L282（2026-10-16）— 修 L281 那把全仓普查尺自己的语料面（它静默少了一只产品文件），并用修好的面把 `finally` 裸清理那一族从全仓量一遍

- ① **缺陷形状（这次在尺子上，不在产品码上）**：`tests/unit/test_missing_file_family_l281.py` 的 `product_sources()` 把语料根声明成 `("augmentor", "api", "cli")` 三棵**目录**，而 `cli` 在仓库根的真形是单文件 `cli.py` ⇒ 旧实现里那一格走 `if not base.is_dir(): continue`，静默跳掉。后果不是「少一只文件没被扫」这么简单：①面那条「全仓零命中」于是是在**一块缩了水的面**上量出来的，而缺的那一只恰好不在任何子树里——任何「按子树数文件」的对照都看不见它。这与 l177 那把尺的 docstring 警告的同一族：**扫描面缩水会把「量不到的 0」重新变成假绿**。
- ② **修面**：语料根拆成 `PRODUCT_DIRS = ("augmentor", "api")` 与 `PRODUCT_FILES = ("cli.py",)` 两组声明，`product_sources()` 改成**逐根验真**——目录根不是目录就 `AssertionError("语料根 X 不是目录（真形可能是仓库根的单个模块文件）")`，文件根不在磁盘上就 `AssertionError("语料文件 X 不在磁盘上")`，`continue` 那一格从此不存在。现量 133 → **134**。红读数不是推断：仓外探针 `Temp/probe_l282_face.py` 只做两件事——在**新面**上把 `MEASURED_PRODUCT_FILES` 拨回旧值跑那一支（吐 `assert 134 == 133`，与守卫消息逐字一致）与把三档「列了但形状不对」的声明喂进 `product_sources()`（`dirs=(..., "cli")` / `files=("no_such.py",)` / `dirs=("augmentor_old", ...)` 三档全部 `LOUD`，无一静默返回读数）。
- ③ **跨尺对账钉成集合相等**：新臂 `test_the_corpus_is_the_same_face_as_l177` 读 l177 那把尺**自己的** `_PRODUCTS` 定义（`importlib` 按路径加载，不在这里抄清单），与本尺的面比**路径集合**。为什么不能各钉一个规模数：本尺 133、那把 134 的当下，两把尺在自己那份常数上都是绿的，只有集合相等这一形能指出分叉与分叉的那一只。同轮另一臂 `test_the_census_face_covers_the_cli_entry_module` 钉「`cli.py` 在面上」，并以 `files=()` 复现旧面读数（133 = `MEASURED_PRODUCT_FILES - 1`）证明这一格有牙齿而不是恒真。
- ④ **面改宽之后 L281 那十格的结论未被推翻**：`cli.py`（59 行，CLI 入口）经同一把「判过存在再动手」的尺读数是 **0 命中**，全仓普查那条零命中断言在 134 只文件上重跑仍绿 ⇒ 被推翻的只有「133 就是全仓」这句话，不是任何一格产品结论。
- ⑤ **新尺 `finally_cleanup_hits`：把 A378 那一族推到全仓**。判的是族而不是全文：只认 `unlink / remove / rmtree / close / rename / replace` 这六只动词，且只认落在 `finally` 体里、没被内层 `try` 罩住、又不带 `missing_ok=True` 的调用（两道豁免各对应一条已收口口径——`missing_ok` 是 A377 的正解，自罩 `try` 是 `atomic_write._discard_tmp` 那形尽力清理；只调用一只尽力清理 helper 的格子天然落在名单之外）。 enclosing 函数用 parent 映射上溯取，避免嵌套函数被外层 `walk` 双计。
- ⑥ **量到一格，实测未坐实成缺陷 ⇒ 按原话不动产品码**：读数是 `augmentor/excel_write.py::export_table_to_excel` 的 `wb.close()`。openpyxl 的 `Workbook.close()` 写作 `if hasattr(self, "_archive"): self._archive.close()`，仓外探针 `Temp/probe_finally_close.py` 照产品骨架（`try: …wb.save()… finally: wb.close()`）实测三档——`wb.save()` 因目标是一枚目录而抛 `PermissionError` 时 `close()` **静默**（逐字读数 `ARM_SAVE_FAIL_DIR: close_silent` / `top:PermissionError` / `ctx:None`，真正的写失败没有被顶替）、save 成功后的 `close()` 静默、重复 `close()` 静默。于是按「不为不会发生的情形加兜底」这条约束**不给它包 `try/except`**，改成把这一格钉进 `MEASURED_FINALLY_EXEMPT`（逐格点名：文件 + 函数 + 调用文本），配四档自证臂（裸调用点名 / 自罩 `try` 不误杀 / `missing_ok` 不误杀 / 同一族换动词仍认）与一支「豁免那一格在尺子上仍量得到」的防空集臂。产品码把那一句改形 ⇒ 名册红；别处新长出一格 ⇒ 同样红。
- ⑦ **负结论也入账——限流器那一族本轮复核为无缺陷**：`api/middleware/rate_limit.py` 四格口径逐条读源码坐实——客户端键只取 `request.client.host`、**不读** `X-Forwarded-For`（docstring 写着伪造头的实测：`max_requests=2` 时同一台机器换头发 50 个请求全 200），拿不到对端地址折成 `"unknown"` 共享桶而不是按头猜；键空间封顶 `DEFAULT_MAX_KEYS = 10000`、`_sweep_locked` 每窗口至多一次并按最近命中丢最旧、新建键顶过上限时当场带 `keep` 清扫（`keep` 那一格不参与，防「自己刚建的桶被自己顶掉 ⇒ 对该键永久放行」）；`_hits` 全部读写在 `threading.Lock` 内，`retry_after` 用 `.get()` 而非默认工厂（注释写明「只查不买」的读侧不许长大键空间）；429 响应带 `Retry-After` 头。⇒ 本轮 HTTP 对撞期间那次 `POST /api/system/backups/cleanup` 的 429 是**正常判决**，不是缺陷。
- ⑧ **守卫计数与邻面**：`tests/unit/test_missing_file_family_l281.py` 34 → **39 支**（语料面三支 + `finally` 面两支），五支新臂**全部零 parametrize** ⇒ l97 的 `MEASURED_CALLS 453 / MEASURED_READABLE 332 / MEASURED_BLIND 121` 与 l98 的 `MEASURED_EXEMPT_SITES 33` 一字未动，同一次定点复跑（l281 + a184 + l97，l97 会连带 l79）读数 **72 passed / 5 skipped / 0 failed**（那 5 支 skip 是 `test_doc_line_refs_l79` 的 authoring-tree 档，本机缺 `Temp/` 取证数据，历轮同形）。
- ⑨ **尺子重钉明细（逐格按红读数，没有任何一把尺被改松）**：本批四处账面改动（清单行 +1、新立 A379 / A380 连带各自分隔空行 +4、第 70 行同行改写、日志块追加在文件尾）先由 **a184 硬 0 档**红一处 `dead_line` 点名：l242 常数区那句带卷名的活引用被平移 ⇒ 按 `grep -n -a "^| A362 |"` / `"^| A365 |"` **现量**重钉 `OPTIMIZATION_LOOP_2.md:268 → 269` 与 `:274 → 275`，两处都在原注释里续写平移链、不抹旧值。**l242**：表行 252 → **254**、声明面 112 → **114**（清单行写下「另立并同轮收口 A379 / A380 两笔」，`declared()` 的 `BEFORE` 档把它摊成两只 ID 各算一笔；日志标题不点名 ⇒ 不额外进账）、定级格面 130 → **132**；红读数逐字 `assert 254 == 252` / `assert 114 == 112` / `assert 132 == 130`，同批由区段对照与配对那两格逼出 `assert 300 == (252 + 46)`（⇒ 被过滤的 46 行一字未动，这就是「两行落进 Backlog 区而不是日志区」的证据）与 `assert 152 == (112 + 38)`、`assert 132 == (132 + 0)`（日志块标记面一格未动）。**l253**：全面 298 → **300**、Backlog 面 252 → **254**、宽面 133 → **135**、权威面 130 → **132**、本卷轮号可追溯 117 → **119**、字母形 `~~S~~` 63 → **65**、闭合词 `已收口` 56 → **58**、组合名册 `划掉 S / 已收口 / 括号 L / 粗体` 5 → **7**——两格走的是清单里**已有**的形状（与 L281 那两格同族，只是字母换成 S），名册没有多出第 20 格，两面之差仍是 `TWO_FACE_ROSTER` 那三格 ⇒ 两把尺子第十二次同面同数对上。**l263**：表行面 252 → **254**，闭合式 `135 + 5 + 114 == 254`，盲区 5 与 `MEASURED_HITS` 五格名册一格未动（A379 / A380 的定级格带轮号闭合语 ⇒ 走宽面那档，不入「无判决语」档）。**l268**：operative 那一格按 ⑤ 的内容定位器现量唯一命中重钉 912 → **917**（本批在被指向那一行之上落 5 行 = 清单行 +1 与 A379 / A380 连带分隔空行 +4；日志块 12 行落在它之下 ⇒ 不参与平移），账本第 70 行那句自指主张同行改写、链尾补值；红读数逐字 `assert {'LIVE': 0, 'STALE': 6, 'BLIND': 7} == {'LIVE': 1, 'STALE': 5, 'BLIND': 7}` 与 `assert 912 == 917`，改写后两档回到 `1 / 5` 并由独立定位器复验。四把名册尺合跑 **87 passed / 0 failed**，六把账本与文档守卫合跑 **155 passed / 8 skipped**（8 支同一归因：authoring-tree 档）。
- ⑩ **终态门禁读数**：全量定标 `python -m pytest tests -q -p no:cacheprovider` ⇒ **9903 passed / 0 failed / 11 skipped / 290.25 s**，覆盖率 **100.00%**、`TOTAL 14875 stmts / 0 miss / 4130 branch / 0 brpart`。与 v7 基线 **9901** 之差恰为 **+2**——即 ⑥ 里那两支 `finally` 面新臂（`test_bare_finally_cleanups_are_exactly_the_named_cell` + `test_the_finally_census_names_a_bare_cleanup`），语料面三支已计在 v7 内。更要紧的是 **`14875 / 4130` 两格与 v7 逐字相同**：本轮零产品文件改动 ⇒ 覆盖率分母一格未动，这是「纯尺子面」的可验证证据而不是自述。11 支 skip 的构成与历轮同形（8 支 authoring-tree + 1 支 export 无开关 + 1 支前端产物不在 + 1 支 `data/` 无真实数据集）。
- ⑪ **提交状态如实记**：用户至今未明确要求 commit ⇒ L277 / L278 / L279 / L280 / L281 / L282 **六批**改动全部留在工作树未提交，卷一与卷二的 `哈希待 Lxxx 回填` 槽因此一格都未回填（占位数量仍在棘轮封顶之内：卷一 `PLACEHOLDER_CEILING=14` 那把尺只扫卷一、本轮读数未动）。先产品码 commit、再账本 commit 并回填上一轮 hash 的顺序**没有执行**，等用户授权；本轮不声称任何已提交状态。

### L283（2026-10-16）— 三格产品缺陷坐实并同轮收口（HTTP 面的静默落空 / 锁表自己无上限 / 计数旋钮族最后一格漏网），六族普查回负结论，配置并发那一族量到但按口径不自决

- ① **缺陷一（HTTP 面的静默落空）**：`api/routes/status.py` 的宽 `except` 分支把异常吞成 `model_default="unknown"` / `model_available=False` 而**服务端一个字都不留**——运维只看得到一支 200 的 `/api/status`，事后无从知道后端为什么起不来。本轮按 AST 现量（只认 `Exception/BaseException` 那一档，`api/` 面 74 支）逐格读「体里有没有出声」，无声格**恰好一处**就是它；改后同一把尺复跑 **0 格**。修法按既有口径同行改写：`logger.warning(f"读取管道状态失败，按模型后端不可用回执: {e}")`，原文只进日志、不进响应体（与 `deps.py:556-578 raise_internal_error` 同一处置），`import logging` + `logger` 两行落在 `router = APIRouter(...)` 之前。**这一格只补出声，不动状态码**——降级仍回 200 + `status="ok"` 那一半属对外契约，新立 **A381 待拍**。
- ② **缺陷二（锁表自己无上限）**：`_PATH_LOCKS` 单调增长、无回收，而 docstring 写着「有界」⇒ 那句话在本仓没有 enforce 它的代码。实测 `Temp/probe_l283_locks.py`：20 000 格 **6.25 MB / 每格约 312 B**，丢弃路径对象、文件也不存在之后**一格不退**；键来自请求方能点名的数据集文件名 ⇒ 上界必须由代码给。处置走**封顶而不是回收**（`MAX_PATH_LOCKS = 10000` + 超限共用 `_SPILL_LOCK`），与 `api/middleware/rate_limit.py` 的 `DEFAULT_MAX_KEYS` 同一取向。**回收为什么不行要写清楚**：eviction 必须在「取出锁」与「拿到锁」之间开窗——A 取到锁对象还没进 `with`、B 删掉这一格、C 给同一路径建出第二把 ⇒ 两边并发写同一份文件而两边都回 `success: true`，正是 L273 整族要治的形状；共用 spill 锁只会**多**串行化、永不放宽，且 `test_rmw_serialization_l273` 钉的两条（`is` 同一把、返回真 `threading.RLock`）都保住。净 **43/6** 行，其中 docstring 那句假话改判成「单调增长、**不回收**，因此有一条硬上限」。
- ③ **缺陷三（A77 计数旋钮族最后一格漏网）**：`version_control.DatasetVersionManager.get_history` 与已收口的孪生格 `versioning.py:86` 同形而从未被推到——`limit=0` 交出全部 3 条、`limit=-1` 交出 `[{'version_id': '0'}]`（「丢掉最新一条」）。可达面不是 SDK 边角：该类是 `augmentor/__init__.py:65` 的公开导出，也是 CLI `version history`（`cli/commands/version.py:57`）的后端。修法共引权威不另立：`require_count("limit", limit)` + `if limit is not None`。净 **4/3** 行（+1 语句，正是 v10 覆盖率 `14883` 与 v9 `14882` 的那一格差）。
- ④ **新守卫三面与注入取证**：`tests/unit/test_path_lock_registry_cap_l283.py` 6 支（常数钉 / 真上限跑到 cap+500 断表长恰等 cap 且溢出格不进表 / 相对-绝对-str-Path 四形同锁 / cap=3 的共用与可重入 / 溢出路径仍串行化 / 注入臂 `cap=sys.maxsize` ⇒ 60 格全登记），`tests/unit/test_status_degrade_logs_l283.py` 3 支（恰好 1 条 WARNING 且原文不进 `response.text` / 降级契约四格未动 / 健康管道零 WARNING），`test_count_knob_validation.py` +1 臂 ⇒ **237 passed**。注入取证：**A1/S1/S2** 三臂（本轮前两格）与 **V1/V2**（第三格）逐臂红后逐字节复原，V1 读数 `Failed: DID NOT RAISE DataValidationError`、V2 读数 `assert [{'action': '…version_id': '0'}] == []`（`:466`），复原后 sha `b268e921bc94b8f0` 与进入时相等。两支守卫合跑 **251 passed / 8 skipped**，与六把账本守卫同跑 **165 passed / 8 skipped**。
- ⑤ **守卫设计本身的两次返工（过程缺陷，如实记）**：`test_overflow_paths_still_serialize_writes` 第一版用 `threading.Barrier(2)`，而串行化本身就让第二支线程永远进不到 barrier ⇒ **测试自己造死锁**，改成主线程持锁 + `Event.wait(timeout=0.15)` 的确定性形状；`test_healthy_pipeline_leaves_no_warning` 第一版构造真 `AugmentorPipeline`（会落盘、会读仓内 `config.yaml`）⇒ 换成 `types.SimpleNamespace` 替身，探针不碰仓库。
- ⑥ **负结论一：路径穿越族攻不动**。10 种载荷（`../`、`..\`、`%2e%2e%2f`、`....//`、`%00` 截断、绝对盘符路径、`/etc/passwd`、`C:/Windows/win.ini`）× 6 种端点形状 = **60 攻**：`CANARY-OUTSIDE-ROOT` 回显 **0 次**、`outside_root` 与 `data_root` 磁盘快照（size + sha256 前 16 位）**0 格变化**。拒绝码分布 `{404:48, 400:6, 403:6, 422:4}`——同一越界按形状不同回不同码，但每一支都是拒绝，且分布本身不构成新缺陷（那族语义差记在 **A280** 的既有待拍里，本轮不重复立项）。
- ⑦ **负结论二：越界回执不外泄部署路径**。8 类越界/畸形请求的响应体逐只读：判决文案是 `路径包含非法组件` / `路径超出允许的数据目录范围` 这类只含名字与目录标签的话，**0 处**出现绝对路径、盘符或 `D:\WORKSPACE\...` 形状的部署位置；500 那几支走 `INTERNAL_ERROR_DETAIL` 固定文案。与 L258 那条「判决文案刻意不含解析后的绝对落点」的口径一致。
- ⑧ **负结论三：静态「落盘无守门」与「读-改-写无罩锁」两把普查都判为口径不合格**。前者读数 71 只落盘函数里 64 只无守门 ⇒ **静态口径作废**（绝大多数路径来自配置或调用方内部，不是请求方点名），改走行为面（⑥⑦）；后者 33 格候选逐格读源码，`api/routes/language.py::translate_endpoint` 的读与写落在**两个不同文件**（`input_file` 读、`output_file` 写，非丢更新形），其余是「读 A 写 B」，唯一同文件那一格是 `save_config` ⇒ 见 ⑨。**这两把尺都没写成守卫**：判据不能把「形状像」当成「可达」。
- ⑨ **配置并发那一族：量到、按口径不自决（新立 A384）**。20 支并发 `POST /api/config`（两半各改不同节）⇒ **16×200 + 4×500**，500 的现场是 `os.replace` 撞 `WinError 5` 顶穿 5 次 × 20 ms 退避上限；磁盘终态一组整批丢（`quality.threshold=0.116` 落盘、`dedup.threshold` **停在原值 0.9**）。单发两次都 200、`ignored_keys: []` 两键都落 ⇒ 排除「键本就被拒」这一种假读数。**为什么这不是本轮的自决面**：`update_config` 的改-写同步区内没有 `await`，单事件循环部署下两支请求结构上无法交错，读数出自「每线程一只 TestClient ⇒ 各一循环」与多进程部署，正是 `mutate_json_file` docstring 明写的「只管同进程，多 worker 那一档待拍」与 **A156 / A158** 的跨进程口径同族；三种修法（罩 `path_lock` / 跨进程文件锁 / 启动期按 `workers>1` 出声拒绝）都要么新增依赖要么改对外语义。**附带量到**：一次**成功**写入把仓档 `config.yaml` 的 **206 行注释 / 457 行**重写成 **0 / 186**（`sha 79c18e44c614 → 160de1281ef0`）——A152① 只挡「空写入不碰文件」，`yaml.safe_dump` 不回写注释这一半从未变过，本轮按原话只记不动。
- ⑩ **负结论四～六**：上传面有闸（`web.max_upload_bytes: 268435456`，`_read_upload_body` 先按 declared 长度判、读满后再按实读字节判第二道；落点经 `_upload_landing_path` → `resolve_within_roots` 白名单，`..` 400 / 越界绝对路径 403）；`PUT|DELETE /api/data/*` 的 `index` 有 `0 <= index < len(items)` 判（负数不会走 `pop(-1)` 那种子语义）；`cache.py` 的 `md5(key)` 本地寻址按 L32 既有口径复核为**不动**——L32 已把两处不忠实 md5 键换成「失效判据 == 被缓存的那份输入」，剩下的摘要只作文件名，改它要同时顶穿 L258 名册键 `cache.py:268` 与 `test_disk_cache_missing_file_l280` 的清理口径，而威胁模型内攻击者本就能提交任意数据并读回结果，投毒自己那份缓存不构成越权。
- ⑪ **普查尺三轮返工（重复踩坑，写清楚以免第三次）**：`app.routes` 在本版 starlette/fastapi **不展平 `include_router`** ⇒ v1 数到 9 条，v2 手写 walker 找 `.routes`/`.router` 得 1，真解是 `shell.original_router.routes` + `shell.include_context.prefix` ⇒ 展平 **76**，与 `app.openapi()['paths']` 的 77 对账只差 `/api/health`。这一格常驻守卫早已写明，本轮仍连踩两次，属**读数前的现量纪律**没执行到位。另两处工具口径错：`inspect.getsourcelines(fn)` 首项是**行列表**不是文件名（`Path(list)` 抛 `TypeError`，改用 `inspect.getsourcefile` + `[1]`，并把 AST 绑定放宽成 `abs(node.lineno - start) <= 5` 以容装饰器）；`ast.Dict` 没有 `.elts`（用 `.values`）。写盘函数名单 `export_data/append/dump` 过宽（把路由函数名与 `list.append` 误判成落盘证据）⇒ 该静态口径整体作废。
- ⑫ **尺子按红读数重钉，没有任何一把被改松**：l177 语料面第七次计数 **398 → 400**，红读数逐字「语料规模现量 (133, 398)，实得 (133, 400)」，归因是本轮新增两只测试文件（`test_path_lock_registry_cap_l283.py` / `test_status_degrade_logs_l283.py`），产品面 **133 一格未动**（那把尺数文件不只行数）。卷一两处活引用按处方 3 同行重钉：`status.py:28-35 → :40-47`（本轮 ① 的两行 logger 加上 L68 那两个重试键共同下移；这一格在 HEAD 时就已平移进 `StatusResponse` 体内而无人出声，因为 `test_doc_line_refs_l79` 判的是「落点是不是代码跨度」不比对符号名——**这是尺子的既有口径，本轮按现量重钉而不是改松**）、`version_control.py:159 → :160`（③ 的 +1 行）。两处各 1/1 净零行，卷一 **2891 行未动**（bytes 1726625）。
- ⑬ **账面改动与位移预告**：本批卷二四处落点——清单行 +1（第 120 行之下）、新立 A381 / A382 / A383 / A384 四笔连带各自分隔空行 +8（第 305 行 A380 之下）、日志块追加在文件尾。**`test_backlog_closure_l242.py:150` 那处活引用（`OPTIMIZATION_LOOP_2.md:114`）在本批落点之上 ⇒ 不平移；`:151` 那处写账本时读到的旧值 269 在清单行之下 ⇒ 被顶成 270，现钉 `OPTIMIZATION_LOOP_2.md:275`（**L285 记账批按现量重钉**：271 → 272，本批清单行 +1 落在被指向那一行之上；**L287 记账批按现量续钉**：272 → 275——本批清单行 +1 只解释其中一格，另两格是更早批次的漏账（上一批把这一格记成「现量」而实际按的是算术推：grep -n -a "^| A362 |" 在本批落盘前现量 274）。a184 一路绿是因为旧值 272 那一行是 A361 的表行 —— 落点是代码跨度、内容却是另一格⇒ 这正是 `dead_line` 量不到的「落点对、内容错」那一族；本轮按现量改准并照实记，不美化。）**，这一格是**写账本时的预判失误**（原话「那两格不平移」只对了一半，漏量了清单行 +1 也会顶下面的引用），由 a184 硬 0 档的红读数 `assert 1 == 0`（`dead_line` 档，点名 `tests/unit/test_backlog_closure_l242.py:151`）当场逼出、按现量重钉为 271（现量落点 `| A362 | api/routes/augment.py:114`；**L284 记账批的清单行 +1 把这一格再顶一行，270 → 271 随同批重钉，归因见 L284 块 ⑫**），同批 `TestRedCapability` 那一格由「注入后 dead_line 恰一处」变两处而连带红、重钉后复绿；**全部落在 `test_self_line_claim_l268` 的 operative 自指目标（现值 917）之上** ⇒ 预告位移 **+9 ⇒ 926**，按红读数与 ⑤ 的内容定位器现量重钉，不按算术推。卷一只动第 230 / 1713 两行（同行改写，净零行）。
- ⑭ **门禁读数**：定点复跑先于全量——六把账本与文档守卫 + l177 ⇒ **165 passed / 8 skipped**；族守卫 `test_count_knob_validation.py` ⇒ **237 passed**；本轮三把新守卫合跑 ⇒ **251 passed / 8 skipped**。全量定标逐字 `python -m pytest tests -q -p no:cacheprovider` 两趟：**v9 = 1 failed / 9911 passed / 11 skipped / 344.52 s**（唯一红是 l177 的语料规模，见 ⑫），**v10 终态 = 9913 passed / 0 failed / 11 skipped / 359.57 s**、覆盖率 **100.00%**、`TOTAL 14883 stmts / 0 miss / 4132 branch / 0 brpart`。与 v9 用例数之差 **+2** = 那一格由红转绿 + ③ 的新臂；与 v9 语句数之差 **+1** = ③ 那行 `require_count`。11 支 skip 的构成与历轮同形（8 支 authoring-tree + 1 支 export 无开关 + 1 支前端产物不在 + 1 支 `data/` 无真实数据集），本轮**未新增任何跳过理由**。前端三门禁**不适用**（`web/` 零改动，不是漏跑）。
- ⑮ **提交状态如实记**：用户至今未明确要求 commit ⇒ L277 / L278 / L279 / L280 / L281 / L282 / L283 **七批**改动全部留在工作树未提交，卷一与卷二的 `哈希待 Lxxx 回填` 槽因此**一格都未回填**（卷一 `PLACEHOLDER_CEILING=14` 那把尺只扫卷一，本轮卷一占位数未动）。先产品码 commit、再账本 commit 并回填上一轮 hash 的顺序**没有执行**，等用户授权；本轮不声称任何已提交状态。
- ⑯ **记账批第二处过程缺陷：卷二走了 `read_text` 往返，把内容里的裸 CR 折成 LF（已按字节补回）**。第二轮重钉脚本用 `Path.read_text(encoding="utf-8")` 读写卷二，通用换行把**文档内容里那一枚孤立回车**（它夹在两枚反引号中间，作者用它直接表示「前接一个回车」这个字符本身）折叠成了一个 LF ⇒ 落盘读数 `lf 5156 crlf 0 bare_cr 0 sha b02c0f8d92db`，而进入时是 `bare_cr 1`、内容行 5155。归因不靠印象：`git show HEAD:augmentor/OPTIMIZATION_LOOP_2.md` 现量该卷在 HEAD 就是 `lf 5006 crlf 0 cr 1`，那一枚 CR 的唯一字节偏移 929186。第一次按记忆写的锚点 `count(old)==0`——真形是 CR 落在两枚反引号之间，不是反引号之前；按纪律加长锚点而不是放宽断言，改成「都前接一个 + 反引号 + LF + 反引号 + 两枚星号 + 直角引号」那一串字节才 `count==1`。修法走字节面：`read_bytes()` → 一次定点把那个 LF 换回 CR → 落盘前断言 `cr==1` 且 `crlf==0` 且 LF 比批前少 1，终态 `lf 5155 crlf 0 cr 1 sha a0bee94d3b9b`；批前快照留在仓外 `Temp\l283_cr_fix_pre.bin`。**钉住的口径**：卷二此后任何批一律 `read_bytes` 与写 bytes + 显式 LF 回译，禁止 `read_text` / `write_text` 往返——这一枚 CR 是内容不是行尾，`crlf_batch_edit.py` 会因为「有裸 CR」拒写卷二，而 `read_text` 会把裸 CR 静悄悄改掉，两头的坑在同一格上。
### L284（2026-10-17）— 错误回执的渲染崩溃两族坐实并同轮接上（422 回显里的非有限浮点 / 回显与判决文案里的孤立代理），成功面与内部文案两格按口径只量不接（新立 A385 / A386 待拍）

- ① **缺陷一（非有限浮点把 422 顶成裸 500）**：数值型 query 送 `nan` / `inf` / `-inf` 时校验失败本是 422，但 FastAPI 默认处理器把**调用方原样送进来的值**嵌进 `exc.errors()` 的 `ctx` / `input`，`JSONResponse` 用 `allow_nan=False` 序列化 ⇒ `ValueError: Out of range float values are not JSON compliant` 从错误处理器里抛出，客户端看到一支没有判决正文的 500。实测面 5 支端点形状（`/api/dataset/search`、`/api/dataset/compare`、`/api/leakage/check`、`/api/dataset/impact`、`/api/dataset/evaluate`），逐支按「送 NaN ⇒ 期望 422 且判决正文读得到」钉。
- ② **缺陷二（孤立代理把回显顶成 `UnicodeEncodeError`）**：U+D800 这一枚**Python 的 json 解得开**（`json.loads` 默认不校验代理对）、UTF-8 **编不出**，于是任何「把请求文本嵌进响应」的路径都在渲染期炸：`/api/dataset/convert` 的 400 判决文案（「`src.csv` 声明为 …，但它既不是合法 JSON…」）与 `/api/dataset/search` 的 422 回显（`query` / `pattern` / `regex_pattern` 三格）各成一族现场。
- ③ **两族共用一支兜底的依据是继承关系而不是巧合**：`UnicodeEncodeError` 是 `ValueError` 的子类 ⇒ 422 那支 `except ValueError` 天然盖住代理面。这一层写成断言（`test_exception_identity_is_value_error`），否则下一个读账本的人会把兜底范围猜成「只管浮点」。
- ④ **消毒形状的两处选择**：非有限浮点换成**同名 token** `NaN` / `Infinity` / `-Infinity`，不折成 `null` 也不折成 `0`——错误回执的使命是让现场读得出，折叠会把「谁送了 NaN」这一格抹掉；字符串走 `backslashreplace` 而非 U+FFFD——前者可逆且点名到具体那一枚字符（`\ud800`），后者把缺陷伪装成「一个普通替换字符」。递归覆盖 dict / list / tuple 三形（`_finite_safe` 与 `_encodable` 各 13 与 7 支单元面钉住形状，含「不改动入参」那一支：`test_does_not_mutate_input`）。
- ⑤ **成功面刻意不消毒**：不换 `default_response_class`、不给路由加清洗。理由是这一族缺陷（服务端自己算出非有限浮点、把非法字符回显出去）恰恰**应当**以 500 的形式暴露；把它消毒掉等于给 A77 真值性判据族添一个新的静默面。代价记成 `A385`：同一枚代理走成功路径仍是裸 500，只是那一次崩在 pydantic 序列化（`type_adapter.py:651 dump_json` 的 `PydanticSerializationError`），两支异常处理器结构上接不到。
- ⑥ **新守卫 41 支与零写入牙齿臂**：`tests/unit/test_validation_finite_echo_l284.py` 分五面——复现面 8（5 端点参数化 + Inf/-Inf 各点名 + 嵌套容器递归 + 合法 422 一字节不改）、兜底牙齿 3（默认处理器独跑仍 500 / 继承关系 / 已注册到 app）、委托证明 2（`parametrize` 两臂，断言消毒支**先原样委托**默认处理器一次，证明本轮没把 422 的判决体换成自造形状）、孤立代理面 15（convert 400 文案转义后仍读得出台词 / 合法 404 字节不变 / **摘掉守卫回到裸 500** / 已注册 / 422 三格参数化 / `_encodable` 逐形 7 / 合法代理对不动）、`_finite_safe` 单元面 13。牙齿臂全部走**进程内换处理器**（`_use_default_handler` / `_use_plain_http_handler` + `_restore_*`），因此没有仓库写入、也就没有注入器的逐臂复原与验盘步。
- ⑦ **本轮第一次踩到的两处（过程缺陷如实记）**：① 换处理器后仍绿——因为 starlette 的中间件栈在**首次请求**时冻结，只改 `app.exception_handlers` 不置 `app.middleware_stack = None` 就不生效，红的是测试自己造出来的假绿；② 探针脚本里写了带真实孤立代理的字符串字面量，打印环节撞 `UnicodeEncodeError: ... surrogates not allowed`（写盘在编码之后 ⇒ 文档文件未被动过），改用 `chr(92)` 拼字面量后通。两条都不是产品缺陷，但都会让人把「绿」误读成「已验」。
- ⑧ **同族另两格量到而不自决（新立 A385 / A386）**：读数逐字留在上面两行 Backlog 正文里。判据与 L283 那格（A384 跨进程那一档）同一口径：**改法要动对外语义的，本轮只量不动**；A385 三条接法（换响应类 / 端点清洗入参 / 改响应字段类型）分别抹平缺陷面、改掉回显承诺、跨文件动模型，A386 的接法是换判决文案且有既有断言按当前英文形状钉着。
- ⑨ **尺子一律按红读数重钉，没有任何一把被改松**：l177 语料面第八次计数 **400 → 401**（红读数逐字「语料规模现量 (133, 400)，实得 (133, 401)」，就是本轮那一只新测试文件）；l97 `MEASURED_CALLS` **453 → 458**、`MEASURED_READABLE` **332 → 336**、盲面 **121 → 122**；l98 `TIER_PLAIN` **61 → 62**、盲面测试数与已见数各 **116 → 117**、豁免 **33 → 34**（新豁免那格是 `test_default_handler_called_once`——值清单与 `ids=` 都是内联字面 ⇒ 整格豁免）。逐格归属出自仓外探针 `Temp/probe_l284_census_delta.py` 直接 import 两把尺子打印每处桶归属，不按「字面量 ⇒ 可读」推（L204 那次正是这么想当然写错过一回）。
- ⑩ **程序缺口：l264 / l270 不在轮次纪律第 2 节那把「六文件」定向名单里**，而它们量的正是**卷二**的账本内容面 ⇒ L283 记账批落完账本后没被定向复核，漂移一直躺到本轮全量跑才现形（本轮首跑 19 支红里那 4 支属它俩）。已按红读数重钉：l264 站点 **138 → 139**、`not_a_commit` 档 **21 → 22**（名册那一格保持**同一行**追加 ⇒ 净零行）；l270 站点 **664 → 666**、档内 **147 → 149**、卷二清单区 **132 → 133** 与日志区 **143 → 144**、唯一 hex **397 → 398**、`MEASURED_L264_SITES` **138 → 139**，含 `test_l98_reads_only_volume_one` 里那格硬编码 132 → 133。**缺口本身不写进本轮的自决面**：把这两把补进定向名单属**流程文档**改动，与账本内容无关，留作下一轮的候选。
- ⑪ **文档面同行改写**：`docs/augmentor/API.md` 两处——422 那一行补「孤立代理转义成 `\ud800` 同名 token 形状」，400 那一行补「`/api/dataset/convert` 走 `HTTPException` 兜底」并**明写成功路径回显那一格不在覆盖之内**。`git diff --numstat` 由 22/9 走到 **23/10** ⇒ 净 0 行，所以全账对 `docs/augmentor/API.md:1139` 的活引用不平移（位移只来自卷二自身行数，见 ⑫）。
- ⑫ **账面改动与位移**：本批卷二三处落点——清单行 +1（第 121 行 L283 之下）、新立 A385 / A386 两笔连带各自的分隔空行 +4（第 314 行 A384 之下）、日志块追加在文件尾。由这两处推出的连带重钉有两格：① `tests/unit/test_backlog_closure_l242.py` 那处指向卷二 Backlog 区旧值 270 的活引用被清单行顶成 `OPTIMIZATION_LOOP_2.md:275`（**L285 记账批按现量重钉**：271 → 272，本批清单行 +1 又顶一格；**L287 记账批按现量续钉**：272 → 275——本批清单行 +1 只解释其中一格，另两格是更早批次的漏账（上一批把这一格记成「现量」而实际按的是算术推：grep -n -a "^| A362 |" 在本批落盘前现量 274）。a184 一路绿是因为旧值 272 那一行是 A361 的表行 —— 落点是代码跨度、内容却是另一格⇒ 这正是 `dead_line` 量不到的「落点对、内容错」那一族；本轮按现量改准并照实记，不美化。同族另一格在本批落点之上 ⇒ 不动；本块刻意只留一个带文件名的行号，写两个就等于给 a184 造一处指向空行的死引用），② 本账第 **926** 行的 operative 自指应走 **931**——这一格**不按算术放行**：批写脚本的前置量直接 import l268 的 `locate_operative_target`，批前批后各现量一次（它认「两件结构件同在一行」才算命中，哪两件写在那把尺子的注释里，**本块刻意不复述**——复述就会让卷二的命中数从 1 变 2、把定位器自己的唯一性断言当场打死，这一形正是 A333 / A346 点名的「没人量的散文」）。读数 926 → 931 且批后仍唯一命中；同批 l268 的名册与三档闭合按红读数一并重钉。日志块落在被指向那一行之下 ⇒ 不参与平移。
- ⑬ **门禁读数**：账本批**之前**的全量基线 v1 ⇒ **9954 passed / 11 skipped / 0 failed / 383.94 s**，覆盖率 **100.00%**、`TOTAL 14926 stmts / 0 miss / 4148 branch / 0 brpart`（与 L283 终态的 14883 之差 **+43**，是本轮 `api/main.py` 两支处理器与两个消毒函数的语句）。账本批之后的定向读数（六把 + l242 / l253 / l268 / l264 / l270 / l97 / l98 / l177）：**367 passed / 8 skipped / 5 errors**——那 5 支 error 全是 `test_parametrize_blind_face_l98` 的 `RulerNotProven` 采集下限闸（子集跑只采到三百余条，低于 3000 它拒不判，不是缺陷）；定向名单在 v2 红之后补进 l263 与 l284 两支，即「六把 + l242 / l253 / l268 / l264 / l270 / l97 / l98 / l177 / l263 / l284」；全量终态 v2：首跑 **1 failed / 9953 passed / 11 skipped / 435.74 s**（覆盖率 **100.00%**、`TOTAL 14926 stmts / 0 miss / 4148 branch / 0 brpart`）。那一红是 `test_round_first_verdict_l263::test_the_three_faces_partition_the_table`，逐字红读数 `assert 260 == 258` 与闭合式 `assert ((137 + 5) + 116) == 260`：本批新立的 A385 / A386 两行定级格是裸字母、整行没有带轮号的判决语 ⇒ 走 l263 的「无判决语」档，而 l263 同样不在第 2 节六把名单里（**同一处流程缺口第三次坐实**，前两次是 l264 / l270）。按红读数逐格重钉 `MEASURED_BACKLOG_ROWS 258 → 260`、`MEASURED_NO_VERDICT_ROWS 116 → 118`（尺子一格未松，只是把现量钉准；闭合式 `137 + 5 + 118 == 260` 与 l253 的宽面同批对上），l263 在那次 367 的定向跑里 18 支全绿 ⇒ **重钉后全量 v3 终态 9954 passed / 11 skipped / 0 failed / 355.08 s**，覆盖率与语句数读数与 v1 逐字相同。
- ⑭ **提交状态如实记**：用户至今未明确要求 commit ⇒ L277 / L278 / L279 / L280 / L281 / L282 / L283 / L284 **八批**改动全部留在工作树未提交，卷一与卷二的 `哈希待 Lxxx 回填` 槽因此**一格都未回填**；卷二这一面本轮新增第 13 个 `哈希待 L285 回填` 占位（l98 的哈希槽与占位棘轮经 `test_doc_line_refs_l79` 的 LEDGER 常量**只读卷一** ⇒ 卷二的占位数不进那把尺子，这一格按现量记，不当成已通过）。
### L285（2026-10-17）— 日志注入族（CWE-117）坐实并同轮收口：请求体里的换行能在审计流伪造第二条日志行，消毒装在渲染的最后一跳；同族另两格要动对外可观测口径 ⇒ 新立 A387 / A388 待拍

- ① **缺陷形状（逐字载荷）**：`POST /api/system/dependency/datasets` 的 `name` 是任意 JSON 字符串，`augmentor/dependency.py::register_dataset` 用 `f"数据集注册成功: {name} ({dataset_id})"` 交给 `logger.info`。载荷 `evil\n[2026-10-17 02:00:00] INFO ROOT admin login success` 在裸 `Formatter` 下落成**两支物理行**，第二行逐字是 `[2026-10-17 02:00:00] INFO ROOT admin login success (7333cc18)`——一次谁也没做过的「管理员登录成功」就这样进了审计流，时间戳、级别、logger 名全部由请求方指定。同一条通道上 `\x1b` 序列原样进流（终端重绘 / 擦行）。这一族没有任何现成转义可依赖：`%`-style 只认 `%`，`logging` 从不看消息内容，而 `logging.config.dictConfig` 那条标准解法（`disable_existing_loggers` / 自定义 formatter）在本仓不存在——装配是自己写的。
- ② **族范围靠三条负数结果钉准（仓外探针 `Temp/l285_probe*.py`）**：① uvicorn 的 access log 打的是 `get_path_with_query_string(scope)`，源自 raw_path，URL 里的换行到那一跳仍是 `%0A` 百分号编码 ⇒ 拆不出行；② 经 TestClient 的 `request.url.path` 同样未落真换行；③ HTTP 帧本身禁止头值含 CR/LF ⇒ `X-Request-ID` 这类透传字段不构成拆分。**结论：远程可写面只有请求体**，所以消毒按「渲染出口 + 请求体族」收口，不写「全链路都转义」那种证伪不了的主张（这一格与 A333 / A346 点名的「没人量的散文」是同一条纪律的反面教材）。
- ③ **修法选在汇，不选在 200 处调用点**：产品面 `logger.*` / `logging.*` 语句现量 **200 条**（`grep -rn`，注释也计入，因此是上界）。逐点 `escape()` 有两个必坏的性质：新加一处调用就漏一处（普查尺只能事后追），且把「消毒」变成写日志的人的义务。改成只覆写 `logging.Formatter.formatMessage`（`augmentor/logging_setup.py:ControlSafeFormatter`）⇒ **`logging.format` 写成什么格式串都被覆盖**，判据与配置面解耦，换格式串关不掉消毒；`Formatter.format()` 在 `formatMessage` 之后才追加 traceback，所以栈刻意保持多行（⇒ A388）。
- ④ **两处装配**：唯一的 Formatter 构造点 `build_formatter`（`config.yaml` 的 `logging.format` 与 SDK 直构 `AppConfig()` 都从这一跳过，`apply_logging_config` 装出来的 handler 因此全是消毒器），加 API 入口一次。入口那次是**实测逼出来的**：3.12 的 `logging.basicConfig()` 不收 `formatter=`，逐字 `ValueError: Unrecognised argument(s): formatter`（探针在干净子进程里量，pytest 进程 root 已有 handler ⇒ `basicConfig` 空转，这条负数结果本身也是「入口装配必须开子进程量」的理由），所以 `api/main.py` 改成 `handlers=[logging_setup.build_handler(...)]`，格式串与改前逐字相同。
- ⑤ **消毒语义三格**：允许集 `[\x00-\x08\x0a-\x1f\x7f-\x9f]`，**Tab `\x09` 刻意排除**——日志里保留对齐是既有可读性口径，把它转义成 `\x09` 会让每一行缩进文案变形；命中写成 `\xNN` 两字符字面量，**可逆**（`unicode_escape` 反解回原串，由 `test_escape_is_reversible` 钉）、**不吞码点**（逐一体 11 个码点 + 孤立代理位不算控制字符那一支）、长度不炸；干净行走 `search` 预检后**原对象返回**，不产生新串（这一条由 `is` 断言钉，不是「大概相等」）。
- ⑥ **代价同口径实测**（都用 `%(message)s`，20000 次 / 轮 × 7 轮取 min，配对 5 轮全同号，探针 `Temp/l285_probe7_cost_pair.py`）：干净载荷 0.471 → 0.724 µs（**+0.236 µs**），含一枚 `\n` 的载荷 0.483 → 1.704 µs（**+1.202 µs**，只有真载荷才多付这一档，`sub` 要重建字符串）。换成生产格式串（含 `%(asctime)s`，它自己就贵）时基线涨到约 2.4 µs，同一笔固定开销摊薄成 **+0.41 µs / +17 %**。`str.translate(dict)` 那版实测 7.627 µs ⇒ 否掉。一条日志几微秒，不进任何端到端预算；**主张只取「同号」那部分**，不报绝对收益。
- ⑦ **新守卫 1 只 23 支，零 `parametrize`**：`tests/unit/test_log_forging_l285.py` 分六类——单元（允许集逐码点、Tab 例外、可逆、中文 / `%` / 引号 / 反斜杠不动、孤立代理位）、渲染形状（伪造载荷只落一行、`record` 不被改写、args 插值后消毒、`%(name)s` 字段同样消毒、栈仍 >2 行）、装配（`build_formatter` 返回的类且 `_fmt` 未被换掉、`build_handler` 的 stream 是 `sys.stderr`、`apply_logging_config` 装的全是消毒器、`RotatingFileHandler` 落盘恰好一行）、族闭包普查（`api/` + `augmentor/` 里裸 `logging.Formatter(` 构造 == 空集）、HTTP 端到端（伪造换行只落一行、ANSI 不再原样进流、**响应体仍回显原文 `name`**）、非恒真（裸 `Formatter` 下同一支请求当场回到 2 行）。刻意**不开 `parametrize`**：一开就要动 l97 / l98 的盲区名册与三档常数，而本轮的改动面已经够宽；普查尺的牙齿臂走**仓外副本**（往 `tmp_path` 的副本注入一处真构造必须量到、同一串写成正注释不算），因此零仓库写入。既有 logging 装配守卫复跑 **100 passed**（`test_logging_wiring_a97.py` + `test_cli_logging_wiring.py`）⇒ 消毒没有把 A97 那套「写了才动手」判据碰红。
- ⑧ **本轮自抓的三处过程缺陷（如实记，不美化）**：① 我自己新写的产品代码与测试各撞一次收集期 `(unicode error) 'unicodeescape' codec can't decode bytes in position 132-133: truncated \\xXX escape`——docstring 里的 `` `\xNN` `` 被当转义序列，改 raw 前缀（`r"""`）才过；这是「在文档串里写反斜杠字面量」这一族的第二次踩坑，L284 的 ⑦② 记过打印环节的同类。② 普查尺的牙齿臂**一开始写成会改仓库文件**（往 `augmentor/dependency.py` 追加一处构造再复原），判为「中断即留残留」的形状后删掉，改成仓外副本喂尺子。③ 端到端那支非恒真用例把 `dataset_id` 的尾巴写死成字面量，实测 ID 挂在**被伪造出来的那一行**末尾（f-string 里 `{dataset_id}` 在 `{name}` 之后）且是按内容算的哈希 ⇒ 改成 `re.fullmatch` 钉形状，并把「为什么不能钉字面」写进注释。三处都在测试面或我自己的探针里，没有一处是产品逻辑。**第四处（落账后自查抓到，本批返工）**：第一版批写把 A388 那一格里的 `` `\n` `` 写成**单反斜杠** ⇒ Python 在拼字符串时就把它解成真换行，落盘后那一行被劈成两物理行、定级格掉出末列（l242 报「表行形状不合」、l253 的字母形多出 `无字母` 1 格），而 operative 位移也因此多数一格（首轮现量 937 ⇒ 修好后现量 936）；脚本原有的自检只断言「`| A388 |` 恰占一行行首」，行首匹配看得见「这一格从行首开始」、看不见「这一格自己又被劈开」⇒ 补上「末列必须逐字等于定级格」这一断言，同批清单行 ⑤ 里那枚孤零 `**`（`同行改写）**；`，块粗体配对守卫当场红 1 处）一并改掉 ⇒ 本批卷二净行数由首轮 +22 变 **+21**、Backlog 区由 +5 变 **+4**。
- ⑨ **两格要拍板的残留，本轮不单方面改口径（新立 A387 / A388）**：A387 —— 纯 CLI 没有 `logging` 节时 root 上**一个 handler 都没有**，走 `logging.lastResort`，那条内置 handler 的 Formatter 不经构造点 ⇒ 同一载荷在 CLI 默认档仍落两行。两档对照实测（`Temp/l285_probe8_cli.py`，只换配置文件）：无节那档 `HANDLERS []` 且第二行逐字 `[2026] INFO ROOT admin login`，有节那档 `HANDLERS [('StreamHandler', 'ControlSafeFormatter')]` 且同一载荷落成一行 `ERROR evil\x0a[2026] INFO ROOT admin login` ⇒ 缺的不是消毒，而是「默认档压根没有装配」。补它要动 §3.32 与 17 支 CLI 装配守卫钉着的「出厂默认档一字未改」主张；载荷是本地数据文件，比远程面低一个数量级。A388 —— `formatMessage` 之后才追加的 traceback 里控制字符仍原样多行，压平它属改「Python 日志的标准可读形」这一对外可观测口径，而多行栈本身伪造不出顶层行（栈的行首恒为 `  File "..."` / `Traceback`，攻击者写不进一个 `INFO ROOT ...` 形状的顶层行）；「栈仍多行」写成断言钉住，防止下一个人把它当漏消毒顺手压平。判据与 L283（A384）/ L284（A385 / A386）同一口径：**改法要动对外语义的，本轮只量不动**。
- ⑩ **文档面**：`docs/augmentor/ARCHITECTURE.md` 新增 **§3.35**（+41 行），落点在 `## 4. 核心数据流` 之前；先量过活引用——全仓对 ARCHITECTURE.md 的行号引用只有 `ARCHITECTURE.md:1232`（卷一 A74 行 + 卷二 2282 / 3679 两处散文），它在落点**之上** ⇒ 不平移；对 DEPLOYMENT.md 的活引用 `:226-227`（`config.py:43` 的注释，A184 硬 0 档的量面）本轮未动那个文件；对 API.md 的活引用 `:1139` 未动那个文件。⇒ 三处引用逐格现量，不靠「文档插行通常安全」外推。
- ⑪ **账面改动与位移**：本批卷二两处落点——清单行 +1（第 122 行 L284 之下）、新立 A387 / A388 两笔连带各自的分隔空行 +5（批前第 320 行 A386 之下，脚本现量）、日志块追加在文件尾。连带重钉按 L284 ⑫ 同法走**前置量**：批写脚本直接 import l268 的 `locate_operative_target`，批前批后各现量一次（批前 931、批后 936，两次各唯一命中；位移 +5 = 清单行 +1 与 Backlog 两笔连带分隔空行 +4，逐处由脚本现量而不是算术推），**不按算术放行**；`tests/unit/test_backlog_closure_l242.py` 那处指向卷二的活引用由清单行顶下一格（现量：该文件对卷二 Backlog 区那一格的活引用被本批清单行顶下一格，a184 硬 0 档当场红 4 处 `dead_line`（逐字 `assert 4 == 0`，点名卷二三条历史账本提及与 l242 的注释），四处一并重钉为 `OPTIMIZATION_LOOP_2.md:275`，落点现量 `| A362 | api/routes/augment.py:114` ⇒ 由红读数逼出而不是算术推；**L287 记账批按现量续钉**：272 → 275——本批清单行 +1 只解释其中一格，另两格是更早批次的漏账（上一批把这一格记成「现量」而实际按的是算术推：grep -n -a "^| A362 |" 在本批落盘前现量 274）。a184 一路绿是因为旧值 272 那一行是 A361 的表行 —— 落点是代码跨度、内容却是另一格⇒ 这正是 `dead_line` 量不到的「落点对、内容错」那一族；本轮按现量改准并照实记，不美化。）。日志块落在被指向那一行之下 ⇒ 不参与平移。
- ⑫ **尺子按红读数重钉，没有任何一把被改松**（账本批首跑 13 支红 ⇒ 逐格按守卫打出来的实际值重钉，中间读数留着不美化）：l242 的 `MEASURED_ROWS` **260 → 262**（逐字 `assert 262 == 260`），同批由区段对照那一格逼出 `assert 308 == (262 + 46)` ⇒ 被过滤的 46 行一字未动，这就是「两行落进 Backlog 区而不是日志区」的证据；l253 动三格——`MEASURED_ALL_ROWS` **306 → 308**（逐字 `assert 308 == 306`）、`MEASURED_BACKLOG_ROWS` **260 → 262**（逐字 `assert 262 == 260`）、字母形两档 S **73 → 74** 与 M **57 → 58**（diff 逐字点名 `{'S': 74} != {'S': 73}` 与 `{'M': 58} != {'M': 57}`），宽面、权威面与 `TWO_FACE_ROSTER` 三格一格未动；l263 动两格并把闭合式一起推——`MEASURED_BACKLOG_ROWS` **260 → 262**、`MEASURED_NO_VERDICT_ROWS` **118 → 120**（A387 裸 M / A388 裸 S 两格整行没有带轮号的判决语 ⇒ 入「无判决语」档），闭合式由 `137 + 5 + 118 == 260` 走到 **`137 + 5 + 120 == 262`**，宽面 137 与盲区 5 未动；l268 名册那一格 **931 → 936**（三支用例同因，逐字 `('OPTIMIZATION_LOOP_2.md', 'checklist', 232, 936) != (..., 931)`），`MEASURED_SITES` 13 与三档 `{LIVE 1, STALE 5, BLIND 7}` 一格未动 ⇒ 本轮新增的账本散文刻意**不写**「本账第 N 行」那一形（⑪ 只用「operative 自指」措辞），写进去就是为了一句话给自指面 +1；**a184 硬 0 档首跑 `assert 4 == 0`**（`dead_line` 档）点名四处同一个串——卷二 L283 块 ⑫ 与 L284 块 ⑫ 两处历史提及、本轮 ⑪、以及 l242 那把尺子注释里的引用 ⇒ 四处一并重钉为 **:272**，落点现量是 Backlog 区里 augment 进度端点那一格；同批注入臂那一格由 `assert 5 == 1` 一并回绿 ⇒ 「案面多一处死引用就把注入臂顶成两处」这个签名第二次实测到（与 ⑤ 那格同一形状），尺子判据未动；markdown 结构守卫首跑红 1 处（清单行 ⑤ 里一枚孤零的粗体闭标记）⇒ 改自己的散文，判据未动；l177 的语料面在账本批**之前**就已按红读数 **401 → 402** 重钉（四处同行改写、净零行）。**一格未动**：l97 / l98 的 parametrize 名册与三档常数（本轮新守卫零 `parametrize`，且这两把尺子的子集跑必触 `RulerNotProven` 采集下限闸 ⇒ 不声称复跑读数）、六把里其余四把（l98 哈希槽六数、l203 进度一致性、l79 文档行号、l200 契约数字）。**返工一处**：见 ⑧ 末的第四处——A388 那一格被自己写的单反斜杠劈成两物理行、清单行 ⑤ 多一枚孤零粗体标记 ⇒ 卷二净行数由首轮的 +22 修正为 **+21**、operative 位移由 937 修正为现量 **936**、Backlog 区由 +5 修正为 **+4**；首轮那一次落盘用批前快照逐字节还原（sha 前缀 `3bf7100a34ec`，还原后复验 sha 与批前读数逐位相等）才重落，账本里没留半成品。**第二小批那一跑的红（留着，不并入上面首跑那 13 支）**：把 ⑫ 与 ⑬ 的占位填成现量散文之后，13 文件定向门禁由 295 / 8 / 0 变 **14 failed / 281 passed**，14 支全部来自 l270（9 支）与 l264（5 支）这两把「哈希可达面」尺子——我在 ⑫ 的「返工一处」那句里抄了一枚批前快照的十二位十六进制前缀，那枚前缀按这两把尺子的口径是**一处新站点**（git 解析得过、但不是任何一轮的批次哈希 ⇒ 只进 `not_a_commit` 那一档）。红读数逐字：l270 四格 `assert 667 == 666`、`{'not_a_commit': 150} != {'not_a_commit': 149}`、`('OPTIMIZATION_LOOP_2.md', 'log'): 145 != 144`、`assert 399 == 398`；l264 两格 `assert 140 == 139` 与 `{'not_a_commit': 23, ...} != {'not_a_commit': 22, ...}`，而多出那一格被逐字点名成 `(285, 那一枚前缀)`；两把尺子各自的注入臂也被这枚多余站点顶红（`assert 24 == (22 + 1)` 与末格的 `assert 667 == 666`）——**同一枚签名第三次实测到**（⑤ 那格 a184 是第一次，L284 块 ⑫ 是第二次）。处置按 L283 块要点 ④ 的先例走**按红读数重钉**（那一轮同样把一枚快照 sha256 前缀记成站点并重钉，而不是把散文改写掉）：l270 动六格——`MEASURED_SITES` **666 → 667**、`MEASURED_TIERS` 的 `not_a_commit` **149 → 150** 并连带它的五档闭合式走到 `482 + 32 + 3 + 150 + 0 = 667`、`MEASURED_REGIONS` 卷二日志区 **144 → 145**、`MEASURED_UNIQUE_HEX` **398 → 399**、`MEASURED_NOTCOMMIT_REGIONS` 卷二日志区 **22 → 23**、`MEASURED_L264_SITES` **139 → 140**；l264 动三格——`MEASURED_SITES` **139 → 140**、`MEASURED_COUNTS` 的 `not_a_commit` **22 → 23**、`MEASURED_NOTCOMMIT` 名册添 `(285, 那一枚前缀)` 那一格。**判据、档名与闭合式形状一格未松**；`MEASURED_OUTSIDE_L264` 282 与 `MEASURED_DEFECTS` 的空集一格未动 ⇒ 那一枚落在 l264 的面内而不是面外，「面外那 282 处」这一维没被我这次改动污染。**同跑另有 a184 硬 0 档二次红**（`assert 1 == 0` 加注入臂 `assert 2 == 1`）：成因是我给 l264 的名册**加了两行**，把卷二日志块里那条指向该守卫某行的活引用平移出代码跨度——轮次纪律第 5 节那条「插行会平移历史行号引用」第一次打在**测试文件**方向上（此前两次于文档与 docstring）。处置走处方 2 **压回净零行**（新格写在同一行末、说明一并挪进去），`git diff --numstat` 复核两把尺子各自净零，复跑 13 文件回到 **295 passed / 8 skipped / 0 failed**。
- ⑬ **门禁读数**：账本批**之前**的全量基线 v1 ⇒ **9977 passed / 11 skipped / 0 failed / 356.56 s**，覆盖率 **100.00%**、`TOTAL 14939 stmts / 0 miss / 4150 branch / 0 brpart`（+13 那格同上）。账本批之后的定向读数（六把 + l242 / l253 / l263 / l268 / l264 / l270 / l97 / l98 / l177）：**295 passed / 8 skipped / 0 failed / 27.39 s**（13 文件同跑，这一读数是常数重钉与名册那格压回净零行之后复跑的那一次；填完 ⑫⑬ 的第一跑是 **14 failed / 281 passed**，14 支逐格读数见 ⑫ 末「第二小批」那一段；8 支 skip 全部来自 `test_doc_line_refs_l79.py:647` 的作者工作树门，逐字理由「需要作者工作树（Temp/ 取证草稿 + data/ 数据集底料）」，非本轮新增）；全量终态 v2：**9977 passed / 11 skipped / 0 failed / 361.76 s**，覆盖率 100.00%，`TOTAL 14939 stmts / 0 miss / 4150 branch / 0 brpart`；与 v1 逐格相同 ⇒ 清单行与 Backlog 两行没打红任何尺子（那一跑里 ⑫⑬ 还是占位、哈希可达面两把尺子还没重钉），被覆盖语句面一格未动；11 支 skip 的构成与 v1 相同（作者工作树门 8 支 + l82 / l97 / l87 各 1 支），本轮没新增 skip。**全量终态 v3**（⑫⑬ 填完、l264 / l270 按红读数重钉之后）：**9977 passed / 11 skipped / 0 failed / 373.34 s**，覆盖率 100.00%，`TOTAL 14939 stmts / 0 miss / 4150 branch / 0 brpart`——与 v1、v2 三格逐字相同 （9977 / 11 / 0 与同一张 TOTAL）⇒ 这一批从头到尾只动了账本散文与 l264 / l270 两把哈希尺子里的常数，被覆盖的语句面与分支面一格未变。11 支 skip 的构成也与两版基线相同（作者工作树门 8 支 + l82 / l97 / l87 各 1 支）。**第一次那一跑被我自己作废，不记账**：我在上一跑（压缩前发起、标题同为 v3 的那一次）还在跑的时候又发起了一次全量，两次并发 ⇒ 后起那次的收尾读到 **9977 passed / 11 skipped / 1 error**，唯一的 error 是逐用例残留守卫（`tests/conftest.py` 里那把 `fail_on_repo_residue`，A168 那一把）在被盯目录顶层点出一件并发写盘留下的 coverage 数据件（形如 .coverage.DragonPro.pid11788.XltpZ1Mx，跑完即被 combine 吃掉，事后 `git status` 查无残留）。缺陷不在产品也不在尺子——它正好实现了这条守卫想抓的形状：**门禁运行期间不得再起一次门禁**（轮次纪律第 7 节的反向形状：那一节写的是「别在跑的时候改文件」，这次踩的是「别在跑的时候再跑一遍」，同一条判据的漏网孪生格，本轮记在这里，不改尺子）。作废那次的读数留在 Temp 的日志里不美化，记账用的是随后**隔离单跑**的那一次——v3 跑完只做了「把这一格填成 v3 读数」那一次同行改写，净零行、不触任何常数，填完复跑同一把 13 文件定向门禁 ⇒ **295 passed / 8 skipped / 0 failed / 27.75 s**，与填之前那一次（27.39 s 那一跑）**逐格相同** ⇒ 把 ⑬ 填成读数这一次同行改写没给哈希可达面添任何站点：填充文本里刻意**不写**反引号包裹的十六进制串，也不写「某文件冒号某行」那一形（那两形各自会顶起 l264 / l270 与 a184 / l79 的常数，本轮已各自重钉过一次，不该为记账散文再钉第二轮）。8 支 skip 仍是 `test_doc_line_refs_l79` 的作者工作树门，非本轮新增。
- ⑭ **提交状态如实记**：用户至今未明确要求 commit ⇒ L277 / L278 / L279 / L280 / L281 / L282 / L283 / L284 / L285 **九批**改动全部留在工作树未提交，卷一与卷二的 `哈希待 Lxxx 回填` 槽因此**一格都未回填**；卷二这一面本轮新增第 15 个 `哈希待 L286 回填` 占位（l98 的哈希槽与占位棘轮经 `test_doc_line_refs_l79` 的 LEDGER 常量**只读卷一** ⇒ 卷二的占位数不进那把尺子，这一格按现量记，不当成已通过）。

### L286（2026-10-17）— 「名字被当成路径片段」那一族的最后一格：落点是基座本身时版本根被整座 rmtree；同轮收空集静默零写入的委托面孪生格

- ① **缺陷与实测（格一）**：`require_within_dir` 的两道判据是「字面形状」（绝对路径 / 含 `..` 组件）与「规范化之后是否仍在基座之内」，而 `path_lands_within` 判的是**形状**且明写「相等也算落在之内」。于是名字 `.`、`./`、`  .  `、Windows 的 `C:` 两道全过，返回值就是基座目录本身。接上 `versioning.delete_version` 那格 `shutil.rmtree(version_dir)` ⇒ 仓外探针实测：造一个含两只版本 + `history.jsonl` + `current.txt` 的版本根，`delete_version(".")` 之后 `root alive False / WIPED`，整座基座连同账本性文件一起消失。这一格不在 A374 的冒号档里（它不需要盘符，`.` 在任何平台都成立），也不在 A340 的并行咽判决里（它连异常都不抛，走的是「成功」那条路）⇒ 是本族**真正没人收过的最后一格**。
- ② **可达性（不是 SDK 自伤那一档）**：同一条探针起一只最小 FastAPI 把路径参数原样回显，再打真路由：`DELETE /api/versions/%2E`、`%2e`、`%252E` 三档到处理函数手里都是 `'.'`（`%252E` 被二次解码这一事实单独由一支守卫钉住——它是上面那三档判据的**前提**，前提哪天变了要出声，而不是让读数静默变成「测了个不相干的字符串」）；对照组是 `notes%2Fdeep` 那一形拿到 404 ⇒ 斜线编码那一族确实只到 SDK/CLI 为止，两族不要混着讲。改后同一批编码全部 400 且版本根一格未损。
- ③ **修法与三条否决**：这一档下推进**权威本身**，判据仍只有一处、全部调用点一起获得。否决一：不在版本面另写「落点等于基座就 raise」的手抄比较——L273 那一次已经把「两处手抄同一判据」的代价量在案，l258 也为此而钉「包含判据恰好一处实现」。否决二：不写成直接比两条已解析路径——`resolve()` 的扩展长度前缀形状在两次调用之间可以不一致（L273 现量的那一格），直接比的失效方向是**漏判**（`.` 又被放行）而不是误判，因而按形状走权威自己的归一：正向问「落点在不在基座之内」+ 反向问「基座在不在落点之内」，两真即同一条路径。否决三：不新增第二个名字出生点——备份名那一处有守卫钉「名字只有一个出生点，接两处就是两套口径」，本轮那把强权威与 `path_lands_within` 各自一格未改。
- ④ **宽度没有被误伤（接受清单）**：`a/.` 的落点是 `a` 而不是基座 ⇒ 照旧放行；`.._checkpoint.json` 那一形（检查点族把任务 id 拼在前后缀里）照旧放行；`v_20261013_120000` / `sub/dir_ok` / `plain.json` 三格与 L258 的接受清单逐字相同；合法版本删除仍然真删（一支用例专钉判据**宽度**，防后来人把这一档收成「凡带点就拦」）。另一把 `DatasetVersionManager` 的 `delete_version(".")` 实测在未登记时短路回 `False`、压根走不到权威（与 L258 给那一处记的同一句归因：那是纵深防御而不是当前可达面）⇒ 判据按实测定，只有读面当场 raise；顺带记一句它的读面在到权威之前会先落一只 `index.json`，那是这一族的既有副作用，本轮没有动它。
- ⑤ **格二（委托面孪生）**：`export_enhanced` 的 CSV 与 TSV 两支都以「空集直接 return」开头，而 `export()` 一律返回带 `output_path` 与 `item_count: 0` 的成功回执 ⇒ 那条路径指向一个不存在的文件。同一份空集在 `json` / `jsonl` 两档是写产物的（`[]` 与 0 字节），在原生 CSV 支路也已于 A343 那一轮修成「产物存在、读回 0 条」⇒ 委托面这两档是那一族的漏网孪生，判据现成、不必新立。修法与之同形：文件必建、空集 0 字节、没有真实字段可抄就不谎造表头。`POST /api/data/export` 打的正是这两档（`tsv` 无原生实现，必走委托边）。
- ⑥ **量到而不抹平的一格**：本轮取证顺手量到两条 CSV 路的**列投影本来就不同**——原生支路的表头是 `instruction,input,output`（它走字段归一），委托面的表头是观测到的键 `instruction,output`。这是 `export.py` 文件头注释里写明的分工，不是缺陷 ⇒ 按实测量进守卫当断言（两侧各自钉自己的表头），而不是拿「同形」去把一个既有差异压平。
- ⑦ **同族两格不自决**：A374 的 ①③④（冒号档要不要下推九处、`a:b` 的文案误导、相对基座依 cwd 解析）与 A340（并行支路把判决咽成空 dict 而 HTTP 照回 `success: true`）都要求动**对外可观测口径**（放行变异、回执形状），按待拍纪律一格未动；本轮对 A374 只摘掉它自己记下的 ② 那一格（落点变成基座本身），改后 `C:` 与 `.` 同判，A374 行内用「摘掉 ② 那一格」的措辞把这一格与其余三格分开，档位与判据名未动。新立 **A389**（CWE-1236 电子表格公式注入，四条写边，实测 CSV 原样成文、XLSX 读回公式元素）记 S 待拍，理由写在行内：`excel_write.py` 的文件头注释已经把「不悄悄改写」声明成契约。
- ⑧ **新守卫两支的四面与牙齿**：`tests/unit/test_dir_self_landing_l286.py` 四面——拒绝档逐点名（含平台档单独标注，且验文案只含名字与目录标签、不含解析后的绝对落点）、版本根盘面事实（含「合法删除仍真删」那一支宽度钉）、HTTP 三档编码 + 二次解码前提 + `.%2E` 走「之外」那一档的旧文案不被混写、牙齿臂把权威换成**改前的忠实复刻**后必须当场重现 rmtree（复刻体自己得先拦住 `..` 与绝对路径，否则那一支是在打稻草人，另开一支专门验这件事），另有一支 AST 现量钉「判据写成两处包含调用、不许换成直接比两条已解析路径」——这一支刻意读 AST 打出的代码而不是源码文本，因为权威 docstring 里就写着那句反面说明，按文本比会被自己的说明文判红。`tests/unit/test_export_enhanced_empty_set_l286.py` 四面——逐格式产物、非空回归钉、两条 CSV 路各自的表头、摘掉判据产物当场消失。
- ⑨ **尺子按红读数重钉，没有任何一把被改松**：本批账面改动逼出九格常数，逐格按各自红读数重钉，没有任何一把尺被改松：l242 表行面 262 → **263**（同批由区段对照那一格逼出 309 = 263 + 46，被过滤的 46 行一字未动）；l253 全面 308 → **309** 与 Backlog 区 262 → **263**（新立 A389 那一行同时进两面）、字母档裸 S 74 → **75**（M 58、划掉四档、宽面 137、权威面 134 一格未动 ⇒ 只动这一档）；l263 表行面 262 → **263** 与「无判决语」档 120 → **121**（A389 那一格定级格裸字母、整行没有带轮号的判决语 ⇒ 入本档；三档闭合现量 137 + 5 + 121 == 263，与 l253 的宽面同批对上）；l268 名册 operative 那一格 936 → **940**（前置量：批后由 `locate_operative_target` 现读 940 且唯一命中，再在内存里把本批那四行逐行撤掉即复现 936、同样唯一命中 ⇒ 不是算术推的；同批三档闭合的红读数是 `LIVE 1 → 0` / `STALE 5 → 6` 与 `assert 936 == 940`，重钉后两档回到 1 / 5、BLIND 7 一格未动）；普查三面按现量重钉：l97 调用面 458 → **463**、可读面 336 → **339**、盲面 122 → **124**（本轮新增五处 parametrize 逐格点名在那把尺子的注释里，闭合 339 + 124 == 463）；l98 四档里 plain 62 → **63**、derived 38 → **39**，ids-derived 11 与 opaque 11 一格未动，档位和 122 → 124 与 l97 侧独立咬合；l177 语料规模 (133, 402) → (133, 404)（133 那一格第六次确认不动，404 = 402 加本轮两只新守卫文件）。**另记两处「改写散文而不是重钉数字」**：初稿把 A374 那一行的同行改写写成「L286 部分收口」，于是 l242 的二十字窗口在日志块正文里把它读成一句没有标记的整行判决——宽面比窄口径多认一只 ID，那一格宽窄对照当场红；l263 的十二字窗口又把同一句读成带限定词的半程判决——盲区 5 → 6、逐格名册多出一格、卷一只放五格那一格也报出卷二的新面孔。两格名册不可能同时成立：补进半程名册会把「声明却没标」那张对照判红，给 A374 的定级格加标记等于谎称整行已收，所以处置是把账本里那两处措辞改成不带判决动词族的「摘掉 ② 那一格」；改后盲区回到 5、半程名册与逐格命中清单逐字未动，A374 仍留在「无判决语」档，A389 那一格也仍留在同一档。改写完的复跑读数：十二只文件一起跑——六把账本与文档守卫（markdown 结构、哈希槽、进度一致性、注释行号引用、文档行号引用、文档契约数字）加 l242 / l253 / l263 / l264 / l268 / l270——共 **285 passed / 8 skipped / 0 failed**（八只 skipped 仍是文档行号引用那把尺的作者树一面）；其中注释行号引用与文档行号引用两把尺一起绿，才是「本批四处落点没有把任何活引用平移出代码跨度」的证据，operative 自指面那一格也在这一批里同批改写，账本散文、l268 名册与定位器三处一致。
- ⑩ **定向门禁读数**：定向门禁跑十四只文件——本轮两支新守卫、落点族既有守卫四只（l258 / l259 两只名字路径包含面、l277 两只备份名与目录包含面）、同面既有产品测试六只（版本控制、版本面两族、导出三族含 CSV/Excel 读回）、普查尺两只（l97 调用面与 l177 语料规模）——读数 **422 passed / 0 failed / 16.25 秒**。l98 不在这一组里跑：它自带一道「本次采集用例数不得低于 3000」的尺子自证闸门，局部跑必出 5 只 `RulerNotProven` error（那是闸门在出声，不是产品面红），它自己那四档的静态判据留到全量终态那一格一起验。
- ⑪ **账面改动与位移**：本批卷二四处落点——清单行 +1（L285 那一行之下）、新立 A389 连带其分隔空行（A388 那一行之下）、A374 那一行同行改写（净零行）、日志块追加在文件尾。operative 自指面按 L285 ⑪ 同法走**前置量**：批写脚本直接 import l268 的 `locate_operative_target`，批前批后各现量一次、逐次验唯一命中，**不按算术放行**；本批散文刻意不写「某文件冒号某行」那一形，也不写反引号包裹的十六进制串（那两把哈希可达面尺子的口径就吃这两形）。
- ⑫ **全量终态**：**第一次全量红三只**（返工前，日志在仓外 `Temp/l286_full.log`）——`test_micro_branches_l65.py` 的 `test_empty_items_csv_early_return` 与 `::test_empty_items_tsv_early_return` 各断言「空集不落盘」，钉的正是本轮摘掉的那一档旧行为；第三只是 `test_parametrize_blind_face_l98.py::TestDynamicIdMakerFace::test_the_dynamic_face_sees_every_blind_site_by_name`，红读数 `assert 119 == 117`。同一次全量 0 只 error ⇒ ⑩ 那句「l98 局部跑必出尺子自证闸门的五只 `RulerNotProven`」由这次全量反证为真：那五只在整跑里确实不出现。三只红同源同批——L286 定向那十四只文件不含这两支，而 l98 的档位两格（plain 62 → 63、derived 38 → 39）在定向之前就按现量钉过，漏的是名册那一格与 BLIND_SEEN 那一格。**处置与牙齿**：① l65 那两支的用例名与断言改成新判据（`..._writes_an_empty_artifact`，断言产物存在且 0 字节），文件头注释那一行同行改字，两处改动净零行；② l98 两格按红读数 117 → **119** 重钉，另用仓外整树 `--collect-only` 现量补证动态面全数采到（采集 10022 条、名册减动态面差集为空、豁免名册仍是 34 一格未动 ⇒ 这两格进的是「被看到」那一维，不是「被放过」那一维，两把尺子都没松）；定向复跑三支被改文件 41 passed。**全量终态**：10011 passed / 0 failed / 11 skipped，349.44 秒，覆盖率 100.00%（阈值 80 那格照过）。十一只 skip 逐只点名而不是含糊成一句：八只来自 `test_doc_line_refs_l79.py` 的作者工作树面，另三只各自是 `test_config_gates_l82.py`（export 节没有开关，只有格式两键）、`test_serve_config_wiring_l97.py`（本仓未构建前端产物，默认挂载点不存在）、`test_upload_ceiling_l87.py`（工作目录根 / data/ 里没有真实数据集，护栏无对象可证）。10011 + 11 == 10022 与采集数相抵 ⇒ 读数里没有藏 xfail 面。本轮十批改动**仍未提交**（用户未授权提交），全账 `哈希待回填` 那些槽照原样留着，⑫ 落字后卷二再无成对尖括号形状的占位。
- ⑬ **提交状态如实记**：用户至今未明确要求 commit ⇒ L277 / L278 / L279 / L280 / L281 / L282 / L283 / L284 / L285 / L286 **十批**改动全部留在工作树未提交，卷一与卷二的 `哈希待 Lxxx 回填` 槽因此**一格都未回填**；卷二这一面本轮新增第 16 个 `哈希待 L287 回填` 占位（l98 的哈希槽与占位棘轮只读卷一 ⇒ 卷二的占位数不进那把尺子，这一格按现量记而不成已通过）。
### L287（2026-10-17）— A389 按拍板① 收口：四条交付边各按格式原生手段挡住公式注入，另立 A390 / A391

- ① **这一轮收的是哪一格、按谁的什么决定**：A389（电子表格公式注入，CWE-1236）由用户 2026-10-17 拍板走「各按格式原生转义」那一档，本轮按该档落四条边；这不是代理替用户拍板——立账那一格写的是三种对外可观测形状三选一（加前导单引号 / 强制文本类型 / 直接拒收），用户选的是「按格式各自的原生手段」，落到两种容器上是两种不同手段。同批用户还拍了三格：A347 维持现状（写面无凭据那条默认策略不改，账上早已是 `~~M~~ 已收口（L277）`，本轮无需再动）、A384 并发配置写同批加写锁（下一轮）、xlsx 解压放大按「解压比 + 解压后尺寸」拒（本轮立成 A390，实现留下一轮）。
- ② **判据产地只有一处**：`augmentor/validation.py` 尾部新增四件——`SHEET_FORMULA_LEADERS`（六档：等号、加号、减号、at 符号、制表、回车）与 `is_sheet_formula_text` / `sheet_text_cell` / `sheet_text_row`。文件 1618 → 1676 行，新增全部落在 EOF 之后 ⇒ 上游活引用零位移（批前 grep 现量：卷二对 `validation.py` 的活引用落点最低在 217 行那一格，远高于插入点）。为什么住 `validation` 而不是新开一只模块：四条边里三条已经 import 它，且「判一格会不会被执行」与`require_within_dir` 那一族同性质（交付前的闸门，不是业务转换）。
- ③ **CSV / TSV 三条边的改法与它的确切代价**：`export.py` 原生支、`export_enhanced.py` 的 csv 与 tsv 两支、`converter.py` 的 csv 与 tsv 两支，各把 `writerows(...)` 的实参换成「每条记录过一遍 `sheet_text_row`」；`export.py` 那处走模块级共引（同一行加一个名字，净零行），另两只文件按本仓既有口径在函数内导入。**代价是进交付字节的**：纯文本容器没有类型维度，客户端只认前导单引号这一条既有约定，所以那份 CSV 再读回来时值位是`'=1+1`——新守卫第 5 组把这条后果单独钉一支，配文写明「下一轮若有人把它当回归改回去，这一支与第 1 组会同时红，改动必须在账本里重拍一次」。
- ④ **xlsx 那条边的改法**：`excel_write.py` 里新增 `text_only_cell(ws, value)`，判据命中就用 `WriteOnlyCell` 把 `data_type` 显式设成 `s`，两处 `ws.append(...)`（表头那行与逐行）都过它。与 CSV 那侧相反，**交付字节里不加任何字符**，产物 XML 里 `<f>` 归零、读回的字面串与进入时相同。改前实测（L286 立账那轮的读数，本轮在仓外复量）：openpyxl 按值自己定类型，前导等号那格存成公式元素、读回是那个表达式而不是数据本身，pandas 侧读回是空值 ⇒ 那一格连数据都丢了。
- ⑤ **格内回车那一笔不是本轮引入的**：两条路（裸 `ws.append` 与强制文本类型）写同一份以回车开头的串，读回来都是同一个 LF 形状、都是 `s`（仓外探针 `Temp/l287q/crprobe.py` 现量）——xlsx 的换行表示本来就把回车吃掉。守卫第 3 组因此按「逐字相同，除格内回车按 xlsx 自己的换行记」这条写，而不是假装逐字相同。
- ⑥ **族闭合三条（新守卫第 6 组）**：⑴产品树每一处 `writerows` 的实参写法里都要出现 `sheet_text_row`——现量 5 处（`export.py:280`、`export_enhanced.py:208` 与 `:218`、`converter.py:999` 与 `:1005`）；⑵每一处 `ws.append` 都要过 `text_only_cell`——现量 2 处（`excel_write.py:166` 与 `:170`），同族另外两处 `rows.append(...)` 按名字排除；⑶**xlsx 容器的出口只许住一只文件**：`to_excel` / `ExcelWriter` / `Workbook()` 三形在产品树现量只有 `excel_write.py` 那一处 `Workbook(write_only=True)` ⇒ 一条新写的 `frame.to_excel(path)` 绕开前两条普查也会被第三条点名。三条各有牙齿臂（合成未过权威的 `writerows`、合成 `frame.to_excel(...)` 都被同一个扫描器读出）。
- ⑦ **判据只一处的两档普查（第 7 组）**：四条写边文件（`export.py` / `export_enhanced.py` / `converter.py` / `excel_write.py`）里连一档前导字面量都不许出现（它们只许共引权威）；其余产品树只禁「同一个字面量集合里抄了两档以上」。窄的那一档不是放松，是**对齐禁令面**：制表与回车在解析类代码里是空白档常客，宽松版现量把 `json_extract.py` 的`in ' \t\r\n'` 与 `validation.py` 的 glob 字符组都报成缺陷（第一版实测 4 处假红 ⇒ 按现量分成两档，两档各补牙齿臂）。
- ⑧ **负面对照与刻意不做面**：`json` / `jsonl` 两档字节一字未动（新守卫第 4 组，判据不许扩到没有公式解释器的容器之外）；表头那一面不做（立成 A391 待拍）；解压放大那一面不做（立成 A390，实现留下一轮）；标识符清洗的字符集那一维仍按 L269 的决定并入 A344 那一族，不在本轮。
- ⑨ **新守卫的八组面**：`tests/unit/test_sheet_formula_injection_l287.py`，23 支、3 处 parametrize、零写入面（全部落 `tmp_path`）。第 1 组三条 CSV 边逐边六档前导全过、第 2 组 TSV 两条边同判据（原生 `export.py` 根本没有 tsv 分支，`NATIVE_FORMATS` 里就没这一档 ⇒ 写成三条会红在「格式不存在」上，那是 A343 族的另一格账）、第 3 组 xlsx 两格、第 4 组负面对照、第 5 组读回后果、第 6 组族闭合三条、第 7 组判据住一处两档、第 8 组两支牙齿。
- ⑩ **第一次全量红六支（返工前，日志在仓外 `Temp/l287q/full1.log`）**：`test_export_census_sentinel_l177.py::test_scanned_corpus_scale`（语料 404 → 405）、`test_parametrize_blind_face_l98.py` 四支（`MEASURED_CALLS` 466 对 463、档位表 ids-derived 12 对 11、名册多出一格、盲面用例 120 对 119）、`test_parametrize_ids_l97.py::test_census_totals_match_this_rounds_measurement`。六支全部是「新增一只带 parametrize 的守卫文件」移动普查面，产品逻辑与本轮四条写边的行为**一处未坏**；同跑 10028 passed / 11 skipped，覆盖率 TOTAL 14953 语句 0 漏（100%）。
- ⑪ **运行期冻结被我自己破坏一次，如实记**：03:47 启动的那次全量跑到 58% 时，我还在往新守卫里加第 6 组的第三条普查（03:53 与 03:54 两次编辑同一只测试文件）⇒ 那次读数不可信（l98 的动态面可能在不同名册上各采一次），按循环纪律第 7 节**停掉重跑**，03:55 起的那次才是可记账的读数。教训写死在这里：**加测试要在启动门禁之前写完**，而不是「反正只是加一支测试」。
- ⑫ **尺子按红读数重钉七格，没有任何一把被改松**：l177 语料规模 (133, 404) → **(133, 405)**（产品面 133 第七次确认不动，本轮没新增产品文件）；l97 `MEASURED_CALLS` 463 → **466**、`MEASURED_READABLE` 339 → **341**、`MEASURED_BLIND` 124 → **125**（闭合式 `341 + 125 == 466` 由那把尺子自己验）；l98 `TIER_IDS_DERIVED` 11 → **12**、`MEASURED_IDS_DERIVED_SITES` 补进`test_every_leader_gets_the_apostrophe_on_every_edge` 那一格并逐格点名、`MEASURED_BLIND_TESTS` 与 `MEASURED_BLIND_SEEN` 119 → **120**。**为什么三格 parametrize 只让盲面涨一格**：另两格把值位写成内联字面清单 ⇒ 走可读面，这是 l97 / l98 两把尺子各自的判据差别，不是某一把漏量。
- ⑬ **账面改动与位移（初稿把这一格判成「零位移」，被 a184 硬 0 档当场推翻，按现量重写）**：本批卷二四处落点——清单行 +1（L286 那一行之下）、A389 那一行同行改写（定级格收成 `~~S~~ 已收口（L287）`，净零行）、新立 A390 / A391 两行连带各自的分隔空行 +4（落在 Backlog B 标题之前，也就是在 A362 那一格之下）、日志块追加在文件尾。全仓对卷二的活引用现量 6 处，位移账逐格：目标 114 那一处在清单行插入点之上 ⇒ 不动；目标 268 那一处写作「268 → 269」的链条形状、反引号没有紧贴数字 ⇒ REF 扫不到，不进 a184 的面（照实记，不当免死金牌）；目标 272 那四处（卷二 L283 / L284 / L285 三块的历史提及加 l242 注释里那处活引用）全在本批清单行之下 ⇒ 一并被顶下一格，a184 逐字 `assert 4 == 0` 点名四处、注入臂同批 `assert 5 == 1`，按 `grep -n -a` 那条现量（A362 那一格在 275）一并续钉为 275。**比位移更难看的一笔**：旧值 272 从 L286 那批起落的就是 A361 的表行——落点是代码跨度 ⇒ 尺子一路绿，内容却指错一格，这正是 `dead_line` 量不到的「落点对、内容错」；A331 那把内容维只覆盖 .py 的 def / class 主张，管不到 markdown 表格行，所以卷二**内部指针**这一族在它眼里是彻底盲区，本轮按现量改准并照实记，不美化。
- ⑯ **同批另修两格账本自指的旧账，都不是本轮新造的**：⑴卷二第 70 行那条 operative 自指由「第 940 行」那一形按现量改准为 **945**（l268 的 `locate_operative_target` 现读且唯一命中；940 → 945 全由本批在被指向那一行之上落 5 行解释：清单行 +1 与 A390 / A391 连带空行 +4，A389 同行改写净零、日志块在文件尾 ⇒ 不参与）；⑵同一行那条名册**早就漏量**——它自称「至此列出三十一个值」，而 L279 起历批各往那一面加值却没同步名册，本轮按现量重数到 **四十个值**（902 / 907 / 912 / 917 / 926 / 931 / 936 / 940 / 945 这一串里补回缺失的八格）。⑵不是本轮改动造成的漂移，是「只改数字不改名册」那一族的老形状，被 l268 的 `test_the_roster_is_exact` 当场点名成 945 对 940 才现形。
- ⑭ **定向门禁读数**：新守卫单跑 23 passed；四条边相邻的老守卫（`test_excel_write_native_l85.py` / `test_excel_write_edge_l84.py` / `test_csv_excel_export.py` / `test_export_enhanced_empty_set_l286.py` / `test_micro_branches_l65.py`）同跑 99 passed 零红。**其中一笔值得记下**：L287 动手前预测 `test_excel_write_native_l85.py::TestCellsMatchPandas::test_single_value_lands_the_same[formula] 会红（那条用例把原生路与 pandas 路逐格对照），实测**没有红**——归因是那把尺子比的是读回来的**值**，而 openpyxl 读一个公式格子给的也是那串表达式文本，两路在「值」这一维本来就同，分岔发生在类型位与产物 XML 里的公式元素，恰是新守卫第 3 组补上的那一维。等价面没有因此变松：它照旧逐档比，只是它量不到这一维。
- ⑮ **提交状态如实记（用户 2026-10-17 改了口径：每一批都要分批提交并推送）**：本批之前的十一轮改动已按族分四枚 commit 落到 `augmentor-vuln-audit-20261016` 并推送——`8c85ab0b3` 是 L277–L287 的产品面（44 只，+1549/-231，含新的 `api/middleware/write_auth_gate.py`），`4a445da5d` 是同批守卫与尺子（56 只，+8173/-492），`16a5dabc9` 是 web 面（7 只，+178/-44），`6c51decca` 是 docs 面（API.md +23/-10、ARCHITECTURE.md +41/-0）。**为什么是四枚而不是十一枚**：L277 起每一轮的改动在 `validation.py` / `export.py` / `export_enhanced.py` / `excel_write.py` 里逐 hunk 互相穿插，按轮切分做不到（切了就是假账），所以按族合并、主题里写清轮号区间；**从本轮起每轮单独提交**。由此带来一笔必须承认的账本缺口：卷一与卷二 L277–L286 那十格的 `哈希待 Lxxx 回填` 槽**永远回填不了**——代码与账本合在同一批里，一枚提交无法在自己的正文里引用自己的哈希；这些槽按原样留作占位，占位棘轮（只读卷一）未被推动，本轮的 L287 那一格同样按 `哈希待 L288 回填` 留占位。
- ⑰ **账本批尺子按红读数重钉（本批第二段，落在四枚哈希之后）**：写进账本的那三处落点会把六把尺子的面顶起来，逐格按守卫当场打出的读数重钉，判据一格未松。l242 三面：`MEASURED_ROWS` 263 → **265**（逐字 `assert 265 == 263`，同批由区段对照那一格逼出 `assert 311 == (263 + 46)` ⇒ 重钉后 311 = 265 + 46，被过滤的 46 行一字未动）、`MEASURED_DECLARED` 114 → **115**（逐字 `assert 115 == 114`，配对那一格逼出 `assert 153 == (114 + 38)` ⇒ 153 = 115 + 38 自行闭合）、`MEASURED_MARKED` 134 → **135**（两处同数逐字 `assert 135 == 134`）。l253 那把独立尺子本轮复跑**全绿、一格未动**。l263：`MEASURED_BACKLOG_ROWS` 263 → **265** 之后，三档闭合那一支当场给出宽面现量 `138` ⇒ `MEASURED_NO_VERDICT_ROWS` 121 → **122**（一减一加：A389 那一格把带轮号的判决语落进定级格就离开「无判决语」档，而 A390 裸 M 与 A391 裸 S 两行整行没有带轮号的判决语 ⇒ 进档；盲区 5 格未动；闭合式 138 + 5 + 122 = 265 与 l242 的 265 同批对上）。l264：站点面 140 → **144**，六档 `same_round` 80 → 82、`cross_named` 13 → 15（其余四档未动、缺陷档仍恒 0），跨轮名册补两格（就是 ⑯ 那两枚主题不带本轮轮号的哈希 ⇒ 与 l270 的 `MEASURED_L264_SITES` 同批对上）。l268：名册里 operative 那一格 940 → **945**，与 ⑯ 同一笔、由该守卫名册那一支当场点名后按现量钉。l270：面 667 → **675**，`reachable` 482 → 490（八处新站点全在可达档，其余四档逐格未动），卷二清单区 133 → 137、日志区 145 → 149，唯一哈希 399 → 403（站点 +8 而唯一 +4 = 同一枚在清单行与日志块各点一次），与 l264 对账那一格 140 → 144。**账本批后的定向门禁**（第 2 节六把 + l242 / l253 / l263 / l264 / l268 / l270 / l97 两支 / l177 / l284，共 16 文件）：**353 passed / 9 skipped / 0 failed / 34.94 s**；把本条 ⑰ 落盘后按同名单复跑 **353 passed / 9 skipped / 0 failed / 32.29 s** ⇒ 两跑逐格同数，这一行追加本身不进任何一把尺子的面。9 支 skip 的构成是 l79 的 8 支 authoring-tree 与 l97 的 1 支前端产物不在位，与历轮同形，本轮未新增任何跳过理由。**全量定标终态**（`python -m pytest tests -q -p no:cacheprovider`，账本批全部落盘后单跑一次、跑期间对被守卫读取的文件零编辑）：**10034 passed / 11 skipped / 0 failed / 408.15 s**，覆盖率 **100.00%**、`TOTAL 14953 stmts / 0 miss / 4152 branch / 0 brpart`；11 支 skip 的构成照旧（8 支 authoring-tree + 1 支 export 无开关 + 1 支前端产物不在位 + 1 支 `data/` 无真实数据集），全量面同样未新增跳过理由。前端三门禁对**本批（账本批）不适用**：`web/` 这一批一字未改，不是漏跑。
