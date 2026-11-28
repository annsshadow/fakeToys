# R641a CMS file_id_upload 跨人覆盖身份盲写甄别审计

## 结论
- 真父：25eb3d69c7f2c49c393d3d098c13906095b18bc3（观察 HEAD 同值；锚点源码真父..HEAD 零 diff）。
- 纯甄别：零源码改动；临时 oneshot 夹具测后即删，仓库无残留（tests 目录无 r641a 文件）。
- 三分判定：可达真缺陷 1 项（P1，认证后凭任意文件 id 覆写他人文件正文与元数据，真实落库）；不可达 L1 0 项；正常项 3 类。
- 判级说明：有会话门（匿名 401），不判 P0；但写的是文件核心正文 content_base64 与元数据，跨人覆盖属完整性破坏，与认证后身份盲写先例（凭 id 跨人写、无归属门、库真改）同档 P1；区别于仅写 extra/邀请记录的 P2 项，不夸大亦不降级。

## 立项范围与机制锚点
- 目标：oa4rust/backend/crates/cms_assemble_control/src/lib.rs:4229-4257 file_id_upload（POST /api/file/{id}/upload）。
- :4229 签名 pool:Extension<Pool> + session:Option<Extension<Session>> + Path(id) + Json(body)。
- :4235-4236 源码注释明言「x_cms_file 的 creator 列本 upsert 不写，行属主不可判，暂按仅会话存在档收口」。
- :4237 require_session(session)? 仅验会话存在、丢弃身份，无属主/admin 校验。
- :4249-4253 SQL：INSERT INTO x_cms_file (id,name,content_base64,content_type,size) ... ON CONFLICT (id) DO UPDATE SET name=$2,content_base64=$3,content_type=$4,size=$5 RETURNING *——按 path id upsert，无 creator 谓词。
- :4256 返回 ActionResult::success(row_to_json(&row))。
- schema migrations/023_create_cms_assemble_tables.sql:57-67 x_cms_file 有 creator 列（:64），可归属但本端点不写不校验。
- 兄弟门（同文件，形成对照）：:4032 u2_check_owner(&pool,"x_cms_file","creator",...)（file_flag_mockdeletetoget）；:4043 cms_gate_owned_read(&pool,"x_cms_file","creator",...)→U2Gate::Forbidden→Err（file_id）。
- 门函数：:8548 enum U2Gate、:8580-8592 u2_check_owner、:8688-8701 cms_gate_owned_read；is_admin 于 shared/src/middleware/rbac.rs:76-98 fail-closed。
- 路由 cms_assemble_control/src/routes.rs:661 post(file_id_upload)；lib.rs:232 router()；主路由 backend/src/lib.rs:142 .merge 真实挂载；全局认证中间件只验登录不验归属。
- 前端 FileManager.vue:1267-1285 uploadToExistingFile()：:1269 prompt 取 fid、:1280 api.upload('/api/file/{fid}/upload',form)、toast.success('文件内容已追加上传')——真实 UI 消费。
- 台账避重：backend/docs 中 file_id_upload 相关仅 2026-10-14-security-audit-round4.md:143 宽泛风险行 A2-F7（标「未复核」、非专项甄别）、:983 路径笔误设计面行、:991 不同 crate 的 not-accessible 行；本 handler 无 oneshot 专项甄别。与 R412 前端假 id 审计维度不同（本条为后端 handler 真实写库）。

## 决定性 oneshot 真测（独占临时 PG，DATABASE_URL 由环境变量注入）
- 夹具 crates/cms_assemble_control/tests/r641a_cms_file_upload_idor_audit.rs；cargo test -p cms_assemble_control --test r641a_cms_file_upload_idor_audit -- --nocapture：exit 0，2 passed / 0 failed。
- 臂1 alice 凭 bob 文件 id 上传：success（type=success）；DB 回读 creator 仍为 r641a-bob，但 name=hacked-name、content_base64=ALICE_HACKED_B64、content_type=application/x-hacked、size=999 全部被 alice 覆写——跨人正文覆盖真落库坐实。
- 臂2 同一 alice 经兄弟读门 file_id 读 bob 文件：Err(Forbidden)——写端点 success 而读端点 403，门不一致坐实。
- 臂3 本人（bob）上传：success；臂4 admin 跨人上传：success（正常臂未破坏）。
- 臂5 无会话直调：Err(Unauthorized)；反臂 自建 SecurityState + router.layer(auth_middleware) + oneshot 裸 POST → 401。
- 源码守卫：写端点含会话存在门 + upsert 主体、无 u2_check_owner / cms_gate_owned_read / creator 归属谓词；兄弟读写门均在（upload_no_owner_gate=1 sibling_gates_present=1）。

## 三分裁决
- 可达真缺陷 1（P1）：file_id_upload 仅验会话存在，无文件属主/admin 门，任何认证用户凭已知文件 id 即可覆写他人文件正文与元数据并真实落库；与兄弟归属门矛盾，前端真实可达。
- 不可达 L1 0：无死代码误报，锚点经真测可达。
- 正常 3 类：匿名 401 门、本人/admin 上传正常、兄弟 file_id / file_flag_mockdeletetoget 归属门未被改动。

## Fail Loud / 脱敏 / SHA
- 未跳过任何步骤；夹具首轮源码守卫切片边界（end<start）panic 已修正重跑，2 passed；非业务失败。
- 0 真值：连接信息用 <TEST_HOST>:<PORT> 占位，无密钥/真实域名/IP/个人路径。
- runnerSha=408b7db24e0fd9f2eeefec37338b682b03713903c012664c25d87ebfafbb2195（sha256(JSON.stringify(meta))）。
- 文件字节 sha256：夹具=cd2948179dd9c63ea66f0def6055f2b26eff3507d05551d6f64b5a4d51c0a68b、oneshot.log=b9f46c0a77bbd11a15c90605afd463142e1eb3489af76940848bcc9fbb4fb141。
- 证据目录 oa4rust/test-results/r641a-cms/（.gitignore:28 已忽略）。

## 修复方向（不在本次范围）
- file_id_upload 在首个 DB 写前查 SELECT creator FROM x_cms_file WHERE id=$1，行不存在→NotFound，存在→require_owner(pool,&session,&creator)（admin/本人放行，他人 403）；或复用 u2_check_owner(&pool,"x_cms_file","creator",...)。upsert 的 ON CONFLICT DO UPDATE 不得绕过该门。
