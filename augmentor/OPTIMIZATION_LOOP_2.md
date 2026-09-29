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

## 已完成

- [x] **L101** `015b2ca2e` + （批② docs 即本条所在提交）`test,docs(retry,version_control,models,loop)`：**A201 容错路径族 16 条缺数清零，偏支 85 → 80** —— 三支文件（`augmentor/retry.py`、`augmentor/version_control.py`、`augmentor/models/base.py`）的守卫支第一次被踩，9 例新用例全绿，全量 7290 passed / 3 skipped / exit 0 ⇒ 详见循环日志 L101
- [x] **L102** `c1734be35` + （账本批即本条所在提交）`test(api,config)`：**A202 路由/依赖/配置错误路径族 11 条缺数清零** —— health-gate 的 FileNotFoundError 防御支与 500 收尾、`/api/data/export` 500 收尾、augment/multimodal 的 HTTPException 复位支、`_config_data_roots` 降级不冻缓存、`save_config` 对坏旧文件的「继续保存」两支；8 例新增定向全绿，**全量读数被并行会话撞库污染（9 failed 全部归因对方在途改动），干净全量待补** ⇒ 详见循环日志 L102 与撞库记录
- [x] **L103** `28e3a1264` + （同上）`test(cli,preview)`：**A203/A204——version delete 动作三支（缺参 exit / 删除成功 / 版本不存在 exit）与非原生导出格式守卫支清零**，9 条缺数（version.py 98-106 + preview 177）定向覆盖实证从缺数清单消失 ⇒ 详见循环日志 L103
- [x] **L104–L113** 前端页面级组件测试从零建立（A206）：10 个页面各一份 一份 `Dashboard.test.tsx` 同型文件，逐页提交 —— L104 Dashboard `21c3ae1f5`、L105 Settings `39eaa30b0`、L106 System `51096d9fc`、L107 Augmentation `41024a400`、L108 Multimodal `5559e8768`、L109 Export `6d800cb2f`、L110 Versions `2d71358df`、L111 Analysis `b85aacf32`、L112 Quality `e0de32b94`、L113 Security `7ab30f256`；配套 `284d89e29` 抬高 vitest testTimeout。**前端全量 13 文件 / 112 例全绿，tsc 0 错，各文件 eslint --max-warnings 0 通过** ⇒ 详见循环日志 L104–L113

## Backlog A — 质量缺口（缺数 / 偏支 / 健壮性）

| # | 位置 | 内容 | 量级 |
|---|------|------|------|
| A201 | `augmentor/retry.py`、`augmentor/version_control.py`、`augmentor/models/base.py` | **已关闭（L101）**：16 条缺数 + 3 条隐藏偏支弧全清 | S |
| A202 | `api/routes/quality.py`、`api/routes/export.py`、`api/routes/augment.py`、`api/routes/multimodal.py`、`api/deps.py`、`augmentor/config.py` | **已关闭（L102）**：11 条缺数（路由错误/降级支 + 依赖层降级键 + 配置保存容错）全清 | S |
| A203 | `augmentor/cli/commands/version.py` | **已关闭（L103）**：delete 动作 8 条缺数（98-106）全清 | S |
| A204 | `augmentor/preview.py` | **已关闭（L103）**：非原生格式的 DataValidationError 守卫支（177）清零 | XS |
| A205 | 全仓约 40 支 | **进行中（L114–L115）**：偏支 85 → 77（L101–L103 顺带 −8）；本轮再补 8 条可达假支（export_enhanced 6 + validation 2），立 A205 方法学（可达补测 / 不可达记档），已记档 3 条结构性不可达支 | M |
| A206 | `web/src/pages` | **已关闭（L104–L113）**：10 页各建组件测试，前端全量 13 文件 112 例全绿 | L |
| A207 | `web` | bundle 拆分与构建产物体检（第一本账 P3 段留的口） | M |
| A208 | `docs` | FAQ 与 25 个端点的人读字段说明（第一本账 P3 段留的口） | M |

## Backlog B — 功能增强与体验（价值 ÷ 工作量）

| # | 内容 | 证据 | 量级 |
|---|------|------|------|
| B201 | 待第一轮普查后立项 | —— | — |

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
  里，内层循环必然 break，退出支不可达）。这三条留待后续轮次以「删冗余 `if fix:` 守卫」
  的方式收（改产品码、需回归护栏），本轮只记档不硬凑。
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
