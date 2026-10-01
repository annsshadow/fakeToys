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
| A205 | 全仓约 40 支 | **已关闭（L114–L126）**：偏支 85 → 17，46 条逐支判定（24 条可达补测清零 + L126 删 5 条恒真守卫 + 余 17 条永久记档结构族） | M |
| A206 | `web/src/pages` | **已关闭（L104–L113）**：10 页各建组件测试，前端全量 13 文件 112 例全绿 | L |
| A207 | `web` | **已关闭（L128）**：路由懒加载 + 三 vendor 拆分本体已就位，补 `check-dist.mjs` 六格产物体检 + `npm run build:check` 入口 | M |
| A208 | `docs` | **已关闭（L129）**：API 易混字段词典（9 字段）+ API 面排查 FAQ（6 问） | M |

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
- **遗留（后续轮）**：`merge_files` 的 `removed_duplicates` 统计把「去重删除数 +
  max_items 截断数」混报成一个值，需引入截断前计数变量才能拆分；改动会平移
  `dataset_ops.py` 行号（触及历史文档行引用），留独立轮处理。


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
