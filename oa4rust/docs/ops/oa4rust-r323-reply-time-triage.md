# OA4Rust R323 论坛详情回复时间字段缺失甄别

- 轮次：R323（审计前置甄别，只读，不改源码/契约/配置、禁部署禁修复）
- 分支：opt1000/oa4rust-pc-ai-refactor（非 main）
- 相关任务：t108/t110（R322 双层信封修复）诚实更正登记的 §298 独立候选 → t112 本文甄别
- 测试服构建：PC 入口 index-rf91JkBU.js，H5 入口 index-BUQSTyeU.js（构建哈希，非地址真值）

## 1. 来源与边界

- 真浏览器：Playwright Chromium headless，PC 1440x900 + H5 390x844 isMobile/hasTouch DPR2；本地无头浏览器渲染，非物理真机。
- 站点地址、账号口令、会话数据一律不入库；本文统一用 <YOUR_TEST_DOMAIN> 占位，口令仅运行期读取不回显。
- 证据截图与 JSON 落 gitignored 目录 oa4rust/test-results/r323-reply-time/，均标注非物理真机。
- 只读甄别：判 B 只给最小修复方向交队长另派，本任务不执行任何修复。

## 2. 源码级三端点口径确认

路由绑定（oa4rust/backend/crates/bbs_assemble_control/src/lib.rs:173-178）实测三端点分属两族，口径不一致：

- GET reply/{id} → u2_reply_get（u2.rs:1158）。SQL u2.rs:1162 为 SELECT id, topic_id, content, creator, accepted FROM x_bbs_reply，无 create_time；映射 u2.rs:1169-1189 只产出 id/topicId/content/creator/accepted，无时间字段。
- PUT reply/filter/list/page/{page}/count/{count} → u2_reply_filter_list（u2.rs:1197）。两条 SQL u2.rs:1208（带 topic_id 过滤）与 :1227（全量）均为 SELECT id, topic_id, content, creator FROM x_bbs_reply，无 create_time；映射 u2.rs:1243-1263 只产出 id/topicId/content/creator，无时间字段。
- GET reply/list/sub/{id} → reply_list_sub_id（lib.rs:764）。SQL lib.rs:772 为 SELECT id, topic_id, content, creator, create_time::text FROM x_bbs_reply，含 create_time；经 row_to_reply（lib.rs:139-150）产出 create_time 字段。此项属正常的 assemble/control 族。

前端消费：PC 端 oa4rust/frontend/apps/desktop/src/views/BBSForum.vue:171 回复卡模板为 fmtTime(reply.createTime)，列表 queryFn :555 读 PUT reply/filter/list，详情 extras 读 GET reply/{id}；fmtTime 定义 :820。t110 已做兼容映射（createTime = r.createTime ?? r.create_time ?? ''，缺字段不臆造），故时间显示缺失的责任在后端字段供给，不在前端。H5 端 oa4rust/mobile/src/pages/bbs/topic.vue:30-33 的 timeOf 读 create_time/createTime，replyList 服务（mobile/src/services/index.ts:533）走 reply/list/sub，故 H5 不受影响。

## 3. 真浏览器证据（真请求 / 真响应 / DOM 三件套）

同一主题（含 6 条真实回复）在 PC 与 H5 双端对照，pageerror 全程 0：

- PC（我的 → 我的主题 → 打开有回复主题）：6 条回复卡片正文/作者正常，DOM .reply-time 文本为 ["","","","","",""] 全空。抓 PUT /api/bbs/assemble/control/reply/filter/list/page/1/count/50 响应：data.data[] 每项仅 content/creator/id/topicId，hasCreateTime=false；抓 GET reply/{id} 同样无时间。补做真实 UI 提交新回复：卡片 6→7、新回复正文可见，但 7 条 .reply-time 仍全空，refetch 的 filter/list 仍 hasCreateTime=false。
- H5（论坛列表 → 主题详情）：6 条 .reply-meta 文本为 admin · 2026-11-21 22:09 等，时间正常渲染。抓 GET /api/bbs/assemble/control/reply/list/sub/{id} 响应：6 条均含 create_time（如 2026-11-21 22:09:45.823553），hasCreateTime=true。
- 同一批回复（同为 admin 创建、同 topicId），PC 端经 u2 族端点无时间、H5 端经 assemble/control 族端点有时间，确证端点口径不一致导致 PC 时间缺失。

截图证据（gitignored，本地无头浏览器非物理真机）：oa4rust/test-results/r323-reply-time/PC-topic-detail-v2.png、PC-after-create.png、H5-topic-detail.png、PC-forum-list.png。结构化证据：同目录 evidence.json、pc-supplement2.json、pc-create-verify.json。

## 4. 判定

结论 B（真缺陷，P2）：后端 u2 族端点 u2_reply_get 与 u2_reply_filter_list 的 SQL 漏 SELECT create_time 且映射不产出时间字段，致 PC 端论坛详情回复卡片时间恒空；新建回复后复验仍空，排除前端兼容映射缺位或数据本身无时间的可能。非 A（产品口径无需时间）：模板明确有 .reply-time 与 fmtTime 渲染位、H5 同数据能显示时间，证明时间是预期展示字段。非 C（取证受限）：PC/H5 两端真浏览器均完整走通，证据齐全。

附带观察（非独立缺陷）：reply/{id}、reply/filter/list 与 reply/list/sub 三个端点返回字段口径不一（前两者无 create_time，后者有），属同一根因的另一面，建议在修复时一并对齐。

## 5. 影响面与严重度

- 影响面：仅 PC 端 BBSForum.vue 详情回复卡片的时间文本显示；H5 端不受影响。不涉及回复读写主流程、不涉及时序功能——SQL 的 ORDER BY create_time 本就在使用该列，仅是查询未把它 SELECT 出来返回前端。
- 严重度 P2：信息展示缺失（用户看不到回复时间），不影响发帖/回帖/作者与正文等核心功能，无数据损坏，无权限或安全风险。

## 6. 最小修复方向（仅建议，本任务不执行）

1. u2.rs u2_reply_filter_list：两条 SQL（:1208、:1227）SELECT 列表补 create_time::text；映射（:1243-1263）增加 createTime 字段，按 Option<String> 读 create_time，缺失回退空串。
2. u2.rs u2_reply_get：SQL（:1162）SELECT 列表补 create_time::text；映射（:1169-1189）增加 createTime 字段。
3. 对齐三端点字段名：统一返回 createTime（或 create_time），避免 PC/H5 口径分叉；前端 t110 兼容映射已就位，无需改动。
4. 一致性护栏：契约测试固定三端点响应含时间字段，确保后续不回退。

## 7. 验证方式

- test-first RED→GREEN：新增契约断言 GET reply/{id}、PUT reply/filter/list 响应项含 createTime；修复前 RED，修复后 GREEN。
- E2E 复验：PC 真浏览器打开有回复主题，断言 .reply-time 非空且与 H5 同主题时间一致；真实 UI 新建回复后新卡片时间非空；三端点响应体字段一致。

## 8. 取证限制

- 本轮未做全量回归；修复由队长另派实施并跑全量测试。本甄别结论仅针对时间字段显示问题。
- 站点真值、口令、会话数据不在本文出现；截图为本地无头浏览器渲染，非物理真机。

## 9. 结论

R323 甄别判定为真实缺陷 P2（后端 u2 族回复端点漏 SELECT create_time），影响 PC 端回复卡片时间显示，H5 端正常；根因、三端点口径差异、真浏览器证据与最小修复方向均已定位。交付队长另派修复，本轮不改任何源码/契约。项目持续迭代（≥1000 轮），非终态。