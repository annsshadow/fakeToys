# R631b meeting add_participant 身份盲邀请甄别审计

## 结论
- 真父：c1879f8868a0374d4444f5b0367a8fece20a0ebf
- 纯甄别：零源码改动；临时 oneshot 夹具测后即删，仓库无残留。
- 三分判定：可达真缺陷 1 项（P2，认证后身份盲写 + 缺前置资源校验）；不可达 L1 0 项；正常项 3 类。
- 判级说明：需有效会话 + 知目标 meeting_id，写对象为邀请记录（非权限/核心敏感数据/资金），与历史 R619b/R627b/R621b 认证后 IDOR 写同档 P2；不预设 P1、不夸大、不降 L1。加重因素（幽灵记录可批量制造）据实披露供复核。

## 立项范围与机制锚点
- 目标：oa4rust/backend/crates/meeting/src/lib.rs:483 add_participant（POST /api/meeting/{meetingId}/participant/add）。
- :489 require_session(session)? 仅验会话存在，丢弃身份；:491-494 invitee 直取 body；:497-503 INSERT x_meeting_invite 无会议属主/admin 门、不校验 meeting 存在；:505-512 仍返回 added:true。
- 路由 meeting/src/routes.rs:15/:28-29/:35；app 全局认证中间件只验登录不验归属。
- 兄弟正确范本 meeting_assemble_control/src/lib.rs:2687/:2693 meeting_id_add_invite 首行 u2_meeting_owner_gate（:2487-2508 查 creator + require_owner，行不存在放行保 not-found 语义）。
- 前端 MeetingApp.vue:318-328 inviteParticipant 真实消费该 URL。
- 表：migrations/006_meeting_tables.sql:25-34 x_meeting、:36-42 x_meeting_invite。

## 决定性 oneshot 真测（独占临时 PG，DATABASE_URL 由环境变量注入）
- 夹具 crates/meeting/tests/r631b_participant_idor_audit.rs；cargo test -p meeting --test r631b_participant_idor_audit -- --nocapture：exit 0，2 passed / 0 failed。
- 臂1 跨属主写：alice(非属主) 对 bob 会议注入 invitee → success、added:true、x_meeting_invite 落库 1 行（COUNT 断言）。
- 臂2 幽灵写：对不存在 meeting_id 注入 → success、added:true、幽灵记录落库 1 行（无前置资源校验）。
- 臂3 属主本人写：owner 邀请 → success、落库 1 行（正常臂未被破坏）。
- 臂4 无会话直调 → Unauthorized（会话存在门仍在）。
- 反臂 自建 SecurityState + meeting_router.layer(auth_middleware) + oneshot 裸 POST → 401。
- 源码守卫：meeting add_participant 无 u2_meeting_owner_gate；兄弟 meeting_id_add_invite 含 owner gate（含串断言）。
- 与 R359/R362/R363/R364 区分：那些是 assemble_control 状态翻转端点 bool:false 误报成功；本条为 meeting crate 独立写端点，从无属主门，实测真落库。

## 三分裁决
- 可达真缺陷 1（P2）：身份盲邀请——认证用户可对他人会议注入任意 invitee 并真实落库，且可为不存在会议制造幽灵邀请记录；缺属主门与前置资源校验双重缺陷。
- 不可达 L1 0：无死代码误报，全部锚点经真测可达。
- 正常 3 类：会话存在门（无会话401）、属主本人邀请正常、兄弟端点 owner gate 与源码守卫一致。

## Fail Loud / 脱敏 / SHA
- 未跳过任何步骤；夹具首轮编译失败（unused Router import + Json 未 deref）已修正重跑，非业务失败。
- 0 真值：连接信息用 <TEST_HOST>:<PORT> 占位，无密钥/真实域名/IP/个人路径。
- runnerSha=1412c5dc13cb4ce6f345a0bc081353331ceaa203236a54b77b1b0e3ff58cb94c（sha256(JSON.stringify(meta))）。
- 文件字节 sha256：夹具=0eb254e4cce9cbcfff7404d2e5fba7e7836b62c7c52fb9fceb373d2646f9500a、oneshot.log=55a9ac0204e6baaad359614d9a518292d6f0eaa143515c978bb79c50231924ec、meta.json=1412c5dc13cb4ce6f345a0bc081353331ceaa203236a54b77b1b0e3ff58cb94c。
- 证据目录 oa4rust/test-results/r631b-meeting/（.gitignore:28 已忽略）。

## 修复方向（不在本次范围）
- add_participant 在 require_session 后、写之前查 SELECT creator FROM x_meeting WHERE id=$1，不存在返回 NotFound，存在则 require_owner(pool,&session,&creator)，非属主返回 Forbidden；补前置资源校验杜绝幽灵记录。对齐兄弟 u2_meeting_owner_gate。
