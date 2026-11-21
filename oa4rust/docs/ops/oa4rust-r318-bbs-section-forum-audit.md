# R318 BBS 版块跨端表分叉甄别审计

## §1 来源与边界

本轮为 t96 前置甄别，承接 t90[R317] H5 深度审计的跨表分叉观察与 t91 静态审计的 BBS 裁决。
甄别对象：PC 端版块创建/消费与 H5 端发帖 picker 版块数据源是否跨端同源。
任务性质：只读甄别（不改源码/契约/不部署），仅产出本文档；判定 B 时只给修复建议，交队长另派实施。
测试形态：本地 Playwright Chromium 无头浏览器真页面流程实测，所有截图均为本地无头浏览器渲染，非物理真机。
测试服：使用现有部署产物（未重新 build/部署），服务 active；入口构建哈希保持本轮起始态。
脱敏：真域名以 <YOUR_TEST_DOMAIN> 占位，0 真值；口令仅运行期读 .local.keys/环境变量，不回显、不入库；会话数据与账号唯一标识不入库。
证据目录：oa4rust/test-results/r318-bbs-audit/（gitignored），含 evidence 系列 JSON 与若干 PNG。

## §2 判定结论

三选一判定：B —— 真缺陷（跨端版块概念的数据源不同源），严重度 P2。
判定理由：用户视角中版块是同一业务概念，但 PC 新建的版块在 H5 发帖 picker 完全不可见；三张表并存且现存桥接端点读的是无关的第三张表，无任何 section→forum 同步或映射。
影响面：功能可用但数据割裂，不阻断写入（故非 P0/P1）；本任务为前置甄别，按纪律仅给建议，不改源码。
与 t91/t90 的关系：t91 仅裁 PC section 族 create/list 同表同源=是；t90 把跨表分叉标为观察未判缺陷；本轮首次真浏览器坐实跨端不可见，补足判定证据。

## §3 源码定位四件套（写读链）

① PC 版块写入链：frontend/apps/desktop/src/views/BBSForum.vue:34-35 新建版块按钮（aria-label=新建版块 / data-testid=bbs-section-add，文本为 +）；点击后调用 POST /api/bbs/assemble/control/section/create。后端 backend/crates/bbs_assemble_control/src/lib.rs:247-298 section_create，spec 表 x_bbs_assemble_control_section（:249，注释 :245 无 deleted_at 为硬删，:268-275 admin 门）。
② PC 版块读取链：BBSForum.vue:315 api.get(section/list)；lib.rs:216 SELECT id,name,enabled FROM x_bbs_assemble_control_section；与写入同表同源。
③ H5 picker 数据源：mobile/src/pages/bbs/new.vue:12 forums ref，:22-30 loadForums→bbsApi.forumList→resp.data；picker :range=forums.map(f=>f.name)。后端 GET /api/bbs/forum/view/all → lib.rs:566-588 forum_view_all，SELECT id,name,description,sort,creator FROM x_bbs_forum WHERE deleted_at IS NULL。
④ 补写 x_bbs_forum 的真实端点：POST /api/bbs/assemble/control/user/forum → u2.rs:1406 INSERT INTO x_bbs_forum（section/create 与 forum_create 写入的是别的表，无法供 H5 picker 消费）。
⑤ 伪桥接（读第三张表）：section_viewforum_forumId（lib.rs:834-869 SELECT ... FROM x_bbs_section WHERE forum_id=$1）；u2_user_section_forum（u2.rs:1513-1527 query_sections forum_id=$1）；u2_user_section_save（u2.rs:1537-1574 POST user/section admin 门，:1560 INSERT INTO x_bbs_section）。三处既非 x_bbs_assemble_control_section 亦非 x_bbs_forum，不构成有效桥接。
⑥ 主题表双列：topic_create（lib.rs:451-474 INSERT INTO x_bbs_topic(id,forum_id,title,content,creator,author_id,section_id,create_time)，section_id 缺省回退 forum_id :463-467）；topic/list/forum/{id}（lib.rs:491-497 FROM x_bbs_topic WHERE forum_id=$1，消费点为 BBSForum.vue:951）。发帖按 forum_id 归档，进一步证明 forum 才是主题归属主键。

## §4 真浏览器证据四件套（真请求+真响应+真截图+源码定位）

脚本：oa4rust/test-results/r318-bbs-audit-final.cjs 及同名 -run.sh（gitignored），真登录 PC 与 H5 双端，凭据运行期读取不回显。
流程与结果（R318_FINAL_DONE，verdict=CROSS_TABLE_DIVERGENCE_CONFIRMED，0 API 失败 / 0 pageerror）：
步骤1 PC 真登录：afterLogin=/app/dashboard；真实点击左侧论坛菜单进入 forumUrl=/app/bbs（history 路由，非 hash）。
步骤2 PC 真建版块：addBtn=1，点击新建版块按钮（data-testid=bbs-section-add），输入名称（data-testid=bbs-section-name）并保存（data-testid=bbs-section-save）；POST section/create 200；pcSectionList.found=true（该版块出现在 section/list）。
步骤3 H5 可见性核查：刚建的 PC 版块在 forum/view/all 响应中 pcNameInH5=false（完全不可见）；pickerText 仅显示历史经 user/forum 补入的版块。
步骤4 H5 真发帖：topic/create 200；pcTopicList.found=true（H5 发的帖在 PC topic/list 可见）——说明主题本身跨端可见，唯独版块维度不互通。
四件套对应：真请求=section/create、section/list、forum/view/all、topic/create、topic/list/forum；真响应=上述 200 与 found 布尔；真截图=PC-login.png / PC-forum-page.png / PC-section-created-final.png / H5-picker-final.png / PC-topic-visible.png（本地无头浏览器渲染，非物理真机）；源码定位=§3 六条。

## §5 甄别三选一依据（A/B/C 排除）

A 产品语义差异（排除）：若 PC 版块与 H5 论坛是刻意不同的业务对象，应有文档/命名/数据模型区分与桥接。实测二者在 UI 均称版块或论坛且主题按同一 forum_id 归档，topic 表 forum_id/section_id 双列并存且 forum_id 为主，缺乏双轨独立概念的设计证据。
C 取证受限（排除）：本轮已用真实账号经真 UI 完成建版块与可见性核查，接口 200、截图齐备、0 pageerror，不存在无法取证的入口或权限阻断；结论为确定性的不可见，而非证据不足。
B 真缺陷（成立）：同一版块概念跨端数据源不同源，且无同步或映射桥接，导致 PC 建版块对 H5 发帖不可见；数据割裂可由用户直接感知，判定 P2。

## §6 复现路径

1. 以管理员账号登录 PC 端，进入 /app/bbs（论坛）。
2. 点击新建版块按钮（+ / data-testid=bbs-section-add），填写任意名称并保存，确认出现在左侧 section 列表。
3. 以同一账号在 H5 端进入发帖页 /pages/bbs/new，展开版块 picker。
4. 观察：步骤 2 新建的版块不在 picker 选项中（forum/view/all 无该记录）。
5. 对照：经 POST /api/bbs/assemble/control/user/forum 补入 x_bbs_forum 后，该版块才出现在 picker，证明数据源为 x_bbs_forum 而非 x_bbs_assemble_control_section。

## §7 修复建议方向（仅建议，交队长另派实施）

方向一（统一权威表）：令 PC section/create 与 H5 forum/view_all 落到同一张权威表，或让 section/list 与 forum/view/all 读同一数据源，消除双写分叉。
方向二（补桥接）：在 section/create 成功后同步写入 x_bbs_forum（u2_user_forum_save 口径），或提供 section→forum 映射关系供 picker 消费；注意 topic 归档以 forum_id 为主键，桥接须保证 id 一致。
方向三（补测试护栏）：新增端到端契约或 E2E 覆盖 PC 建版块→H5 picker 可见→H5 发帖→PC 可见的跨端闭环，防止再次回归。
本轮不修改任何源码/契约/后端/配置，以上仅为建议。

## §8 验证方式（供修复后复验）

真浏览器重跑 r318-bbs-audit-final.cjs 的判定断言：pcSectionList.found=true 且 pcNameInH5 由 false 转 true；补契约测试固定跨端可见性信封，做 RED→GREEN。复验仍须标注本地无头浏览器非物理真机，凭据运行期读取。

## §9 证据索引

- oa4rust/test-results/r318-bbs-audit/evidence 系列 JSON（v1~v3 + final，gitignored）
- 截图：PC-login.png / PC-forum-page.png / PC-section-created-final.png / H5-picker-final.png / PC-topic-visible.png（本地无头浏览器渲染，非物理真机）
- 源码定位：见 §3 六条（BBSForum.vue / bbs new.vue / bbs_assemble_control/src/lib.rs / src/u2.rs）

## §10 结论

甄别结论为 B（真缺陷，P2）：PC 与 H5 的版块数据源分属 x_bbs_assemble_control_section 与 x_bbs_forum，无有效桥接，真浏览器坐实 PC 新建版块对 H5 发帖不可见。本任务为前置甄别，未改任何源码/契约/部署，修复建议见 §7，交队长另派实施。项目持续迭代，非终态。