# R629c organization_assemble_express 组织属性/授权裸读族身份盲读甄别（只读）

## 结论 / 真父 / 零源码改
- 结论：可达真缺陷 1 项 P2——四个组织属性/授权「读」端点只接 pool+Json<Value>、无 Session 参数，SQL 按 body 指定的 person/unit/identity 查询且无归属谓词，任何已认证用户可读他人属性值与授权映射；需有效会话 + 需知目标 flag，无写操作故不判 P1；attribute_value 是否含高敏 PII 未逐字段确认，不预支 P1。
- 真父：26a2c002b2ac824ec773658d6d5c6321f4141e7a（台账 R629c 立项）；观察 HEAD=7d08e399fcbad595bead334e9323a1a5ae1b4860（队友并行提交），锚点源码真父→HEAD 零 diff。
- 本次零源码/配置改动；oneshot 夹具用后即从源码树删除，唯一入库物为本报告。

## 立项与范围
- 目标：甄别 R619b 覆盖的「写」族之外，「读」侧四端点是否同样与会话身份解绑（身份盲读 IDOR）。
- 不计入：R619b 五写 + 消息两 handler（已判 P2）、round4.md:875 只读挂写 false-positive；台账 grep 零身份盲读专项命中。

## 机制锚点（organization_assemble_express/src/endpoints_attr.rs）
- personattr_list_attribute_person_name:34-71、personattr_list_person_object:75-115、unitattr_list_unit_object:379-419、empower_list_identity_object:461-500：签名均为 (pool: Extension<Pool>, Json(body): Json<Value>)，无 Session。
- :49-52 SQL `... WHERE a.deleted_at IS NULL AND (p.id=$1 OR p.name=$1) AND a.attribute_key=$2` 按 body 的 person/name 查，无 owner 谓词；:84-88 person_id=ANY($1) OR name=ANY($1)；:388-392 unit_id=ANY($1) OR name=ANY($1)；:470-473 `FROM x_empower WHERE ... (from_identity=ANY($1) OR to_identity=ANY($1))`。
- 回传：:58-67 属性 value 数组；:94-114 {person,name,attributeList}；:398-418 {unit,name,attributeList}；:479-499 {id,from_person,to_person,from_identity,to_identity,role_id,enabled}。
- 兄弟铁证（同文件）：resolve_write_owner:216-235 取 actor=sess.person_unique，string_field(body) 非本人需 is_admin 否则 Forbidden；personattr_set/unitattr_set/empowerlog_create:504+ 均带 session:Option<Extension<Session>>。写侧有归属门，读侧四端点无 Session。
- 路由 routes.rs:481-488/506-509/519-522 注册四 POST；:525 .layer(Extension(pool))；crate 内无 auth_middleware，全局 backend/src/lib.rs:227-230 只验登录注入 session、不验归属。
- 前端 OrgViewer.vue:390-406 loadOrgObjectReads 消费四 API；flags=["0"] 为存在性探针（据实披露：前端未直接展示他人数据，但后端对任意已认证客户端开放构造 body 读取）。

## 方法与决定性实测（独立 PG + 真迁移 + oneshot 直调）
- 独占 PG（<TEST_HOST>:<PORT>，postgres:16，连接串由环境变量 DATABASE_URL 注入不入库），shared::migrate::run_migrations 建真表；seed alice/bob、unit-a/unit-b、bob 人员属性 mobile=测试串、unit-b 属性 cost=测试串、bob→admin 授权映射。
- 命令：`cargo test -p organization_assemble_express --test r629c_org_attr_read_idor_audit -- --nocapture` → 2 passed / 0 failed。

| 臂 | 结果 | 判定 |
|---|---|---|
| alice 读 bob mobile 属性 | 200，attributeList 含 bob 测试值 | 跨人身份盲读坐实 |
| router 不带 session 读 bob | 200（handler 无 Session 参数） | 无归属门 |
| alice 读 bob person object | 200，含 bob 属性 | 盲读坐实 |
| alice 读 unit-b object | 200，含 cost 测试值 | 跨单元盲读 |
| alice 读 bob 授权映射 | 200，from_person=bob | 授权信息盲读 |
| alice 读自己 | 200 | 正常 |
| 源码守卫 4 端点签名 | 4/4 仅 pool+Json 无 Session | 根因坐实 |
| app 层裸请求 4 路径 | 4/4 → 401 | 认证门正常 |

## 三分裁决
- 可达真缺陷 1（P2）：四裸读端点身份盲读；已认证用户构造 body 即可读他人属性/授权；无写、需会话+知 flag，不判 P1。
- 不可达 L1：0。
- 正常项 3：写侧归属门（resolve_write_owner）+ app 层裸 401 + 本人读正常路径。

## Fail Loud / 据实披露
- 前端 flags=["0"] 为存在性探针、未直接呈现他人数据；但缺陷在后端 API 层对任意已认证客户端成立，不因前端用法而消失。
- attribute_value 是否含高敏 PII 未逐字段确认，本判级按普通属性计，不预支 P1。
- 观察 HEAD 相对真父前移（队友并行提交），锚点源码零 diff，据实声明。
- 证据日志、夹具副本落 gitignored oa4rust/test-results/r629c-org-attr/；源码树夹具测后即删。

## SHA 清单（runnerSha 与文件字节 sha 分列）
- runnerSha=4ab52d9f07801756e151b8a184b65a7e18f8986e001f7a50102c00f30995af8a（sha256(JSON.stringify(meta))）
- 源码树夹具 sha256=b311e8185e1141d920eb4dd0f88dc7c93227b3d2f4f5a40a2a1583e68fafda1a
- 证据日志 sha256=b065c3cb89cf4435566732535a364f47a45d9bb2020a427467c242a5f93747b2
- 证据副本夹具 sha256=5ed029c372aed03b37b0fa20385df66f4d3371718d3436a19d4b44d3ce4922f4（LF 归一化留档，源码树 CRLF 为准）
- 均为占位 <TEST_HOST>:<PORT>，数据库 URL/端口由环境变量注入，0 真值入库。

## 修复方向
- 与写侧对称：四读端点加 session:Option<Extension<Session>>，按 body 指定的 person/unit/identity 逐一走 require_owner(pool,&session,&owner)（非本人且非 admin → Forbidden），或对高敏属性键加字段级白名单；unit/empower 同理解析归属人后校验。
