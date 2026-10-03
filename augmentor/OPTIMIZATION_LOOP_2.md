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
| B218 | **已关闭（L148）**：merge_files 的 `removed_duplicates` 统计把「去重删除数 + max_items 截断数」混报成一个值（`total_input - total_output`），用户读到的「去重数」在带截断的配置下虚高（实测 8 条入、去重 2、截 3：旧报 5 新报 2） | 在 merge_files 内部用既有 _deduplicate 复算去重删除数、截断数 = 去重后基数 - 输出数，两数分报（新增键 `truncated_by_max_items`，既有键语义改为只记去重）；实现刻意控制在 242 行参考点以下净增 8 行——历史文档对 dataset_ops.py:242/:243 的行引用平移后仍落在代码行（棘轮 58/58 绿），首版把统计逻辑放进 merge() 净增 15 行、把 :243 翻进空行致 A184 硬 0 红，已回退换点；补 1 条回归（去重+截断双开分报 + 无去重无截断全 0 对照），改前红/改后绿 | M |
| B219 | **已关闭（L149）**：sample() 的比例分支 `elif config.ratio:` 用 falsy 判据——显式 ratio=0 语义是「采 0 条」，但 0 被读成「未设置」直接落 else 采全量（与 L144 max_items、L147 导出 max_items 同族，本处是 ratio 位） | 判据改 `is not None`（1 行替换 + 2 行注释，插入点在被历史行引用的 242/243 之下，棘轮不受扰）；补 1 条回归（ratio=0⇒0 条；对照 None⇒全量），改前红/改后绿 | S |
| B220 | **已关闭（L150）**：_deduplicate 的 threshold 形参从不被读——`MergeConfig.dedup_threshold` 文档承诺「去重阈值」、auto_config 按重复率逐条推荐数值并写理由（「上调/下调去重阈值」），但任何取值结果都相同（死旋钮，全仓 0 消费点） | threshold 分档消费：≥ 0.9 只删 instruction 完全相同（原默认行为，保守档），< 0.9 追加宽松键（大小写折叠 + 全空白删除）把近似重复一并删；字段注释与 docstring 同步改写；宽松键归一化首版用单空格 join（多 token 折叠回一个空格、与无空格形态仍不等，测试当场抓出），改全空白删除。auto_config 的推荐数值（0.85–0.98 区间）自此真有其效：高重复率→0.98 保守档、低重复率→0.85 宽松档，方向与其既有理由文字一致，零改动。补 2 条回归（宽松/保守档对照 + merge 层配置真消费），改前红 2 failed/改后绿 | M |
| B221 | **已关闭（L151，收口第一本账 A125）**：两个组件构造器的阈值判据各缺一半——Deduplicator 的 `threshold < 0 or threshold > 1` 对 NaN 双假放行（去重整条静默零动作）、对 True/False 读成 1.0/0.0（False 实测误删 2 条真数据）、对 None/非数值延后到比较处炸 TypeError；QualityScorer 七档坏值（None/字符串/NaN/bool/越界）全放行、判负迟到 score()。配置层（DedupConfig/QualityConfig）L76 起就拒，组件层 15 轮未收 | 两组件阈值统一接 `require_ratio`（界与配置侧共引 `DEDUP_THRESHOLD_RANGE` / QUALITY_THRESHOLD_RANGE，A77 同式）+ None 显式拒；Deduplicator 仍抛 DedupError 且文案不带键名（消费侧公开契约由 L76 钉子钉住）。L76 两条「洞本体」钉子按 docstring 红字约定翻转（改前红 11 / 改后绿），新增 QualityScorer 阈值守卫 11 例 | M |
| B222 | **已关闭（L152，收口第一本账 A46①）**：`create_stream_processor(processor_func, chunk_size=1000)` 的 `chunk_size` 形参声明了从不被读——工厂闭包逐条处理、无状态，块边界无任何可观测后果，「设了没生效」是静默的，且全仓 0 调用点 | 删除形参（结构性填不了⇒删，不发明语义）；补签名守卫（断言形参不在）；更新钉死该形参的既有调用。A46②（StreamConfig 两字段断线，产品内 0 使用者）留档后续轮 | S |
| B223 | **已关闭（L153，收口第一本账 A124）**：`quality.weights` 的形状判据权威住组件（`QualityScorer` 两行手抄只判长度与和）——配置层只判 null，两面对同一键不同判；三档洞：`'abc'` 长度恰 3 死在 `sum()` 的 TypeError、`[-1.0, 2.0, 0.0]` 静默放行（权重符号反了排序整个反过来不出声）、`[True, True, False]` 被 sum 当 `[1,1,0]` | 判据权威搬进 validation 族（新族员 `require_ratio_list`：逐项 bool/NaN/越界 + 长度 + 和=1±0.01），配置层 / 组件层 / 静态回放三面共引同一份（A77）；`quality.weights` 进静态规格表、L76 的「唯一豁免」钉子按 docstring 红字约定翻转为「规格在场 + 坏值两面同拒 + 合法 6/6 两面放行」；`[]` 由静默回落默认改为即拒。新增 6 条组件层守卫 + 翻转 1 条，改前红/改后绿 | M |
| B224 | **已关闭（L154，收口第一本账 A140）**：A140 余 11 节 29 键（context / versioning / sampler / expander / tracker / visualization / multilingual / evaluation / benchmark / active_learning / frameworks）四面零反馈——运行时零判据（一个节内判据都没有）、静态规格表零行、不可经 API 写入 ⇒ 坏值全静默进加载面；三节有真读者（pipeline 直读 num_turns / storage_dir / auto_snapshot，CLI 基准子命令读 metrics / baseline_file），假零 / 字符串假真 / 标量拆字符等档症状要到消费日才出声 | 11 节整批接节（同式同族一批收掉，A140 原框「单节约等于 S」）：节内判据引 validation 族既有成员（12 布尔键、4 计数键取正、5 字符串键、8 清单键、null 全拒），不新造判据不新造常数；静态面补 11 节 dict 行 + 29 键规格行与运行时同档；加载面构造器自动吃到。新守卫 8 类（普查名单翻转 + 坏值 × 面矩阵 + 合法反向 + 删规格行独立性 + 加载面 + 消费面）；l83 分档 13 键移档、validator 两计数扩员、l73 棘轮第三次翻空 + 1 用例按其 docstring 明文处方退役（假臂无来源 ⇒ B225 留档）；行引用 4 条降名锚 + L79 / A184 / l97 逐格重钉 | M |
| B225 | **已关闭（L155，留防御支 + 钉死其契约）**：`apply_section_update` 的假臂（节无节内判据时「写直通、不复查」那一支）自 L154 全 20 节接上判据起仓内无来源 | 处置二选一拍定为「留」：写入路径拿到的是任意 dataclass 节对象，无判据节（含用户自定义节）是合法入参，删臂 = 对外行为变更；l73 新增 `TestTheUngatedWriteThroughIsAPinnedContract`（2 例）把「写直通、不吞键、不做判决（有判据节会拒的 0 / 负数照落）」钉成契约，谁删臂或顺手接判据就红。测试-only 一轮，产品码零改动、无行引用位移 | S |




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
- **坑记档**：`__init__.py` 是 CRLF/LF 混合文件，行尾探测（sed 显示会吞 ，须用字节级 repr 定行尾）——本轮第一稿锚点因此没中。
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
