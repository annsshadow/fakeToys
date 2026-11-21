# R320 section 删除与桥接 forum 一致性甄别

## §1 来源与边界

本轮为 t101 只读前置甄别，承接 §282 队长纯只读侦察登记的候选（非已确认缺陷，禁臆造）。
静态事实：t98 桥接 section_create 成功后以同 id 写 x_bbs_forum；但 section_delete（bbs_assemble_control/src/lib.rs:359-375 shared::crud_delete，section_create_spec soft_delete=false）只删 section 不删桥接 forum；migrations/037_add_deleted_at.sql 已给多表加 deleted_at 列而该实现仍硬删（注释/DDL 漂移）。
任务性质：只读甄别（不改源码/契约/配置/迁移文件、禁部署、禁实际修复），仅创建并删除自身专用测试数据以完成闭环验证。
测试形态：本地 Playwright Chromium 无头浏览器真登录 PC 与 H5 双端实测，所有截图均为本地无头浏览器渲染，非物理真机。
测试服：使用现有部署产物（含 t98 桥接修复），未重新 build/部署；凭据仅运行期经 ssh 从 .local.keys 读取，不回显、不入库。
脱敏：真域名以 <YOUR_TEST_DOMAIN> 占位，0 真值；口令、会话数据、账号唯一标识不入库；版块/主题 id 仅以首 8 位展示，完整 id 见 gitignored 证据。
证据目录：oa4rust/test-results/r320-delete-orphan/（gitignored），含 evidence.json 与 4 张 PNG。

## §2 判定结论

三选一判定：B —— 存在孤儿 forum 在 H5 picker 可见（真缺陷，P2）。
判定理由：真浏览器坐实 PC 真 UI 删除版块后，section/list 已不再含该版块（section 硬删成功），但 t98 桥接写入的同 id forum 仍留在 x_bbs_forum，H5 forum/view-all 仍可见、发帖 picker 仍显示该版块名；该 forum 下的关联主题（forum_id=section id）删除后仍可见，形成孤儿主题。
严重度 P2：影响版块管理数据一致性（删除操作不彻底），用户可在 H5 继续选中已删除版块发帖；不阻断核心写入，故非 P0/P1。
按甄别纪律：本轮仅给最小修复建议，交队长另派实施，未执行任何修复。

## §3 真浏览器证据四件套

脚本：oa4rust/test-results/r320-delete-orphan.cjs 及 -run.sh（gitignored），真登录 PC 与 H5 双端，删除走 PC 真 UI 确认弹窗（非直接调 API）。
完成标记：R320_DELETE_DONE verdict=B_ORPHAN_FORUM_VISIBLE sectionId=true forumBefore=true delBtn=1 sectionAfterPresent=false forumAfter=true topicBefore=true topicAfter=true orphanTopic=true pickerHasName=true apiFailures=0 errors=0。
① 真请求（31 次 /api/ 调用，0 失败）：
  - PC 建版块：经真 UI 点击 bbs-section-add → 输入名称 → bbs-section-save（触发 section/create，t98 桥接写 x_bbs_forum）；
  - PC/H5 GET section/list、forum/view-all；H5 POST topic/create；H5 GET topic/list/forum/{id}；
  - PC 真 UI 删除：定位 section 项内删除按钮（aria-label=删除版块，hover 后点击）→ #oa4-confirm-overlay 确认 → POST section/delete/{id}。
② 真响应（节选，完整见证据 JSON）：
  - 建版块名 R320孤儿甄别_1795266184009，section id 首8位 afc32cde（200，section/list count=17）；
  - 删除前 forum/view-all visible=true（count=8，同 id 同名匹配成功）；
  - 建主题 POST topic/create 200（topic id 首8位 b4a47934），topic/list/forum found=true；
  - 删除 toast「版块已删除」；删除后 section/list present=false（section 已消失）；
  - 删除后 forum/view-all visible=true（同 id 同名仍在，孤儿 forum 坐实）；H5 picker 仍含该版块名（pickerHasName=true）；
  - 删除后 topic/list/forum found=true（孤儿主题坐实）。
③ 真截图（4 张，本地无头浏览器渲染，非物理真机）：PC-section-created.png（建版块后列表）、PC-section-deleted.png（删除 toast 与列表）、H5-picker-after-delete.png（删除后 picker 仍显示该版块）、evidence.json。
④ 源码定位：
  - section_create 桥接：bbs_assemble_control/src/lib.rs:291-325（:310 INSERT INTO x_bbs_forum，forum id 复用 section id）；
  - section_delete：lib.rs:359-375，仅 shared::crud_delete(&pool, &section_create_spec(), &id)（:368），无 forum 清理；
  - section_create_spec：lib.rs:247-253，soft_delete=false；
  - crud_delete 硬删分支：shared/src/crud.rs:141-159（soft_delete=false → DELETE FROM {table} WHERE id=$1，无级联）；
  - forum_view_all 数据源：lib.rs:566-588，SELECT ... FROM x_bbs_forum WHERE deleted_at IS NULL；
  - topic 归档：lib.rs:505 INSERT x_bbs_topic(forum_id,...)，topic/list/forum 按 forum_id 过滤；
  - 端点：routes.rs:42-43 section/delete/{id}。

## §4 A/B/C 三选一排除依据

B 成立：删除后 section 消失但桥接 forum 仍在 H5 picker 可见，关联主题仍可见，真浏览器证据充分，判定为真缺陷。
A 排除：section_delete 无级联/软删 forum，产品语义亦不允许「已删除版块仍在 H5 可发帖」（确认弹窗文案明示删除该版块），非预期独立存在。
C 排除：31 次 API 调用 0 失败 / 0 pageerror，建→桥接→删→复查闭环全部走通，证据充分非取证受限。

## §5 最小修复建议（仅建议，交队长另派，本轮不执行）

方向一（对称桥接删除）：在 section_delete 成功删除 section 后，按同 id 同步删除 x_bbs_forum 中桥接行（与 section_create 的 forum id=section id 口径对称），消除孤儿 forum。
方向二（软删对齐 DDL）：若按 037 迁移启用 deleted_at 语义，则 section 与 forum 均改软删并在 forum/view-all 的 WHERE deleted_at IS NULL 过滤下自然隐藏；须同步 section_create_spec.soft_delete 与 forum 视图口径，避免注释/DDL/实现三者漂移。
方向三（关联主题处理）：明确删除版块后其下 topic 的归属策略——随版块软删/归档，或保留但标记「原版块已删除」并在 PC/H5 列表过滤，避免孤儿主题仍以已删版块 forum_id 可见。
方向四（护栏）：新增端到端契约/E2E 覆盖「建版块→H5 可见→删版块→H5 不可见→主题处理」，做 RED→GREEN，防止桥接只覆盖创建不覆盖删除的半闭环回归。
以上仅为建议，本轮未改任何源码/契约/配置/迁移文件，未执行实际修复。

## §6 取证边界与清理说明

边界：本轮为坐实孤儿链路，在本人 admin 会话下创建了 1 个专用测试版块（R320孤儿甄别_1795266184009）及其下 1 条测试主题，用于验证删除前后可见性差异；删除经真 UI 完成，残留的孤儿 forum 与孤儿主题是本缺陷的直接产物（非人为清理遗漏）。
首轮 E2E 因删除按钮需 hover 才可见导致点击超时（脚本层问题），已改 hover+force click 重跑成功；首轮曾遗留一个测试版块（R320孤儿甄别_1795266103880），属脚本失败残留测试数据，非本轮判定依据。
测试服数据清理不在本任务范围（只读甄别禁改数据外的运维操作），如需可由队长另派清理。

## §7 证据索引

- oa4rust/test-results/r320-delete-orphan/evidence.json（建/桥接/删/复查全流程响应与判定，gitignored）
- PC-section-created.png / PC-section-deleted.png / H5-picker-after-delete.png（本地无头浏览器渲染，非物理真机，gitignored）
- 源码：bbs_assemble_control/src/lib.rs:247-253/291-325/359-375/505/566-588、shared/src/crud.rs:141-159、routes.rs:42-43
- 对照：oa4rust/docs/ops/oa4rust-r318-bbs-section-bridge-fix.md（t98 桥接只覆盖创建）

## §8 结论

R320 section 删除与桥接 forum 一致性甄别完成：判定 B（真缺陷 P2）——PC 真 UI 删除版块后，t98 桥接写入的 x_bbs_forum 行未同步清理，H5 forum/view-all 与发帖 picker 仍可见该孤儿 forum，关联主题亦成孤儿仍可见。真浏览器双端实测 0 API 失败 / 0 pageerror，证据充分。仅给最小修复建议，未执行修复、未改源码/契约/配置。项目持续迭代，非终态。