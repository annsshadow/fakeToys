# R319 历史遗留 PC 版块迁移必要性甄别

## §1 来源与边界

本轮为 t100 只读前置甄别，承接 R318/t98 收官如实标注的已知边界：t98 桥接只覆盖【新建】版块（section/create 成功后以同 id 写 x_bbs_forum），历史遗留 PC 版块仍只在 section/list 可见、不在 H5 发帖 picker（forum/view-all）数据源中。按纪律不得臆造「必须迁移」，先只读甄别必要性。
任务性质：只读甄别（不改源码/契约/配置/迁移文件、禁部署、禁执行实际回填），仅产出本文档。
测试形态：本地 Playwright Chromium 无头浏览器真登录 PC 与 H5 双端实测，所有截图均为本地无头浏览器渲染，非物理真机。
测试服：使用现有部署产物（未重新 build/部署），服务可访问；凭据仅运行期经 ssh 从 .local.keys 读取，不回显、不入库。
脱敏：真域名以 <YOUR_TEST_DOMAIN> 占位，0 真值；口令、会话数据、账号唯一标识不入库。
证据目录：oa4rust/test-results/r319-legacy-triage/（gitignored），含 evidence.json 与 2 张 PNG。
证据边界（如实标注）：后端 section/list 接口仅返回 id/name/enabled 三个字段，不返回 creator/create_time；故差集版块的 create_time 无法从接口直接取得，改由版块名称内嵌的毫秒 epoch 时间戳推导（见 §4 判定表，标注为名称推导值），creator 字段对 section 侧不可得（forum 侧有 creator=admin）。此边界不影响测试标记与关联主题两项判据。

## §2 判定结论

三选一判定：A —— 全部为历史 E2E 测试残留，无需迁移；并明确建议【不迁移】（迁移会污染 H5 发帖 picker）。
真浏览器实测数据：PC section/list 共 15 个版块，H5 forum/view-all 共 6 个版块，差集 13 个；13 个差集版块全部带测试标记、关联主题数均为 0、名称内嵌时间戳均落在 R300~R318 历史 E2E 时段；candidateBusiness=0。
结论依据充分：0 API 失败 / 0 pageerror，section/list 与 forum/view-all 均 200，判定非取证受限。

## §3 真浏览器证据四件套

脚本：oa4rust/test-results/r319-legacy-triage.cjs 及 -run.sh（gitignored），真登录 PC 与 H5 双端后逐版块核查。
完成标记：R319_TRIAGE_DONE sections=15 forums=6 diff=13 legacyResidual=13 candidateBusiness=0 verdict=A_ALL_RESIDUAL_NO_MIGRATION apiFailures=0 errors=0。
① 真请求：PC GET /api/bbs/assemble/control/section/list（200）；H5 GET /api/bbs/assemble/control/forum/view/all（200）；对每个差集版块 PC GET /api/bbs/assemble/control/topic/list/forum/{id}（200）。
② 真响应：section/list data 为 15 条 {id,name,enabled}；forum/view-all data 为 6 条 {id,name,creator,create_time,description}；差集按 id 计算（forumIds 集合过滤）得 13 条。
③ 真截图：PC-forum-list.png（PC 论坛版块列表）、H5-picker.png（H5 发帖版块选择器，本地无头浏览器渲染，非物理真机）。picker 文本仅显示 forum/view-all 的 6 个版块（R317H5正式版块、R318甄别/桥接专测等），无任一历史测试残留版块。
④ 源码定位：section/list = backend/crates/bbs_assemble_control/src/lib.rs:209-216（SELECT id,name,enabled FROM x_bbs_assemble_control_section）；forum/view-all = lib.rs:566-588（SELECT ... FROM x_bbs_forum）；topic/list/forum = routes.rs:48 + lib.rs topic 归档按 forum_id；t98 桥接 section_create = lib.rs:263-332（仅新建版块同 id 双写 x_bbs_forum，明确不迁移历史版块，见 oa4rust/docs/ops/oa4rust-r318-bbs-section-bridge-fix.md §2）。

## §4 差集版块逐一判定表（13 个）

判据：名称含实测/专测/甄别/回显/筛选/测试/Rxxx 等测试标记 + 关联主题数=0 + 名称内嵌时间戳落在历史 E2E 时段 → 判测试残留。下表 id8 为版块 id 前 8 位（完整 id 见证据 JSON）。
1. R300 B4 实测版块（id8=2a7eb2d6）：测试标记=是；关联主题=0；名称无内嵌时间戳（早期 E2E）→ 测试残留。
2. R300 B4 实测版块（id8=24a3cb89）：测试标记=是；关联主题=0；名称无内嵌时间戳（早期 E2E）→ 测试残留。
3. R302 发帖回显实测版块 1795209213286（id8=5235a08d）：测试标记=是；关联主题=0；名称 epoch 推导 2026-11-20T21:13:33Z（历史 E2E）→ 测试残留。
4. R302 发帖回显实测版块 1795209310097（id8=f01e974a）：测试标记=是；关联主题=0；epoch 推导 2026-11-20T21:15:10Z → 测试残留。
5. R302 发帖回显实测版块 1795209365916（id8=2c90821d）：测试标记=是；关联主题=0；epoch 推导 2026-11-20T21:16:05Z → 测试残留。
6. R302 发帖回显实测版块 1795209486101（id8=8ce80126）：测试标记=是；关联主题=0；epoch 推导 2026-11-20T21:18:06Z → 测试残留。
7. R303 版块筛选实测版块 1795211085758（id8=92090f0c）：测试标记=是；关联主题=0；epoch 推导 2026-11-20T21:44:45Z → 测试残留。
8. R303 版块筛选实测版块 1795211148748（id8=452caef1）：测试标记=是；关联主题=0；epoch 推导 2026-11-20T21:45:48Z → 测试残留。
9. R303 版块筛选实测版块 1795211213781（id8=edeb31b0）：测试标记=是；关联主题=0；epoch 推导 2026-11-20T21:46:53Z → 测试残留。
10. R303 版块筛选实测版块 1795212191775（id8=bdccc22e）：测试标记=是；关联主题=0；epoch 推导 2026-11-20T22:03:11Z → 测试残留。
11. R309版块_1795227175842（id8=0c563d18）：测试标记=是；关联主题=0；epoch 推导 2026-11-21T02:12:55Z → 测试残留。
12. R317H5版块_1795242333258（id8=967d038a）：测试标记=是；关联主题=0；epoch 推导 2026-11-21T06:25:33Z → 测试残留。
13. R318专测版块_1795262355835（id8=5eff83bf）：测试标记=是；关联主题=0；epoch 推导 2026-11-21T11:59:15Z → 测试残留。
汇总：13/13 判测试残留；候选业务版块 0 个。注：epoch 为毫秒时间戳，toISOString 显示为 UTC，与北京时间相差 8 小时，不影响「落在历史 E2E 时段」的判定。

## §5 A/B/C 三选一排除依据

A 成立：差集 13 个版块全部带测试标记、关联主题=0、时段落在历史 E2E，无一为真实业务版块；且 forum/view-all 现有 6 个均为经 user/forum/桥接补入的专测版块，H5 picker 已被测试数据填充。
B 排除（无真实业务版块需回填）：candidateBusiness=0，无任何差集版块满足「名为真实业务版块且有真实主题归属」。按纪律不臆造回填需求，本轮不执行、不建议回填。
C 排除（非取证受限）：section/list 与 forum/view-all 均 200，13 个差集版块逐一完成 topic/list/forum 核查，0 API 失败 / 0 pageerror，证据充分，不属证据不足。

## §6 迁移（不执行）风险评估与建议

本轮结论为不迁移。理由：13 个均为历史 E2E 测试残留，迁移到 x_bbs_forum 会将其注入 H5 发帖 picker，污染真实用户选择列表（当前 picker 已含 6 个专测版块，再迁移 13 个测试残留将显著恶化）。
即使未来判 B 需回填，id 一致性风险须评估：主题表 x_bbs_topic 以 forum_id 为主归档（topic_create lib.rs:505 写 forum_id，section_id 缺省回退 forum_id:497-501；topic/list/forum 按 forum_id 过滤）；回填 forum 的 id 必须与原 section id 严格一致，否则历史主题按 forum_id 查询时无法对应到回填版块，造成主题与版块脱节。t98 桥接已采用「forum id 复用 section id」的正确口径（fixdoc §2），可作为未来回填 reference。
以上仅为风险说明与建议，本轮未执行任何回填、未改任何源码/契约/配置/迁移文件。

## §7 证据索引

- oa4rust/test-results/r319-legacy-triage/evidence.json（差集明细、逐版块 topic 核查、判定汇总，gitignored）
- PC-forum-list.png / H5-picker.png（本地无头浏览器渲染，非物理真机，gitignored）
- 对照：oa4rust/test-results/r318-bbs-audit/evidence-final.json（R318 跨表分叉坐实）
- 源码：lib.rs:209-216/263-332/566-588、routes.rs:48/139、t98 桥接修复文档

## §8 结论

R319 历史遗留 PC 版块迁移必要性甄别完成：判定 A —— 13 个差集版块全部为历史 E2E 测试残留（带测试标记、关联主题=0、时段落在 R300~R318），无真实业务版块，无需迁移且明确建议不迁移（避免污染 H5 picker）。真浏览器双端实测 0 API 失败 / 0 pageerror，证据充分非取证受限。未改任何源码/契约/配置，未执行回填。项目持续迭代，非终态。