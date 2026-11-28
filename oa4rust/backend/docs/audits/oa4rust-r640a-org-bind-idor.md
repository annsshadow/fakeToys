# R640a x_org_bind 端点族身份盲 IDOR 甄别（验证型，零源码改）

> 结论：organization_core_entity 的 bind_update / bind_delete 只校验会话存在、丢弃会话人身份，未做任何归属判定，任意登录人可跨人改他人绑定的 role/identity_id 并落库、可跨人软删他人绑定；bind_list handler 连 session 参数都没有，裸读全表。据实判级：写 IDOR = P1，读 IDOR（bind_list）= P2。

## 一、审计范围与判级结论

- 审计对象：oa4rust/backend/crates/organization_core_entity/src/lib.rs（全文 SHA-256 见第五节）。
- 甄别类型：验证型（零源码改、零测试入库），仅新增本审计报告。
- 级别判定（独立据实，不硬凑、不预支）：
    - 写 IDOR（bind_update / bind_delete）= P1。理由：认证后越权写，可跨人改组织绑定角色字段 role（如改为高权限角色）并真正落库、可跨人软删他人绑定，属持久数据篡改；不判 P0，因为需有效会话、需知 bind id、非匿名，不能未认证大规模自动化。
    - 读 IDOR（bind_list）= P2。理由：认证后越权读全表绑定映射（无 session 参数、无身份过滤），危害低于写；生产全局 auth_middleware 对外部匿名返回 401（见第四节披露），故不判更高。
- 反证（同一 crate 的兄弟 handler 有正确归属门）：bind_create 用 CREATOR_OWNER_PREDICATE + ownership_error 校验父资源 x_org_identity.creator，非本人返回 403。详见第三节。

## 二、锚点与根因

文件 oa4rust/backend/crates/organization_core_entity/src/lib.rs（行号为审计时版本）：

    :32-35  fn require_session(session: Option<Extension<Session>>) -> Result<Session, AppError>  仅解包会话，map(|Extension(s)| s).ok_or(Unauthorized)；不返回身份用于比对。
    :40-45  const CREATOR_OWNER_PREDICATE: 归属谓词（id=$1 AND (creator=$2 OR admin EXISTS)），正确归属范本。
    :49-67  async fn ownership_error(db, table, id) -> AppError：写影响 0 行时区分 403/404。
    :1001-1061 pub async fn bind_create(...)：:1006 let actor = require_session(session)?.person_unique；:1015-1031 用 CREATOR_OWNER_PREDICATE 查 x_org_identity，!allowed → ownership_error。正确。
    :1064-1106 pub async fn bind_update(db, session, Path(id), Json(req))：:1070 require_session(session)? 丢弃返回的 Session（未取 person_unique）；:1071 org_bind::Entity::find_by_id(&id).one() 无归属谓词；:1078-1086 按 req 覆盖 identity_id/group_id/role；:1088 active.update(&db.0).await? 直接落库。缺陷根因。
    :1109-1129 pub async fn bind_delete(db, session, Path(id))：:1114 require_session(session)? 同样丢弃；:1115 find_by_id 无归属；:1123 active.deleted_at = Set(Some(now))；:1124 active.update() 软删落库。缺陷根因。
    :409-446 pub async fn bind_list(db: Extension<DatabaseConnection>)：签名无 session 参数；:412-418 find().filter(DeletedAt.is_null()).order_by(CreateTime).limit(20).all()，:420-435 映射 id/identityId/groupId/role 裸读全表。
    :1133-1183 pub fn organization_core_entity_router(db)：:1176 GET /api/organization/bind/list → bind_list；:1177 POST /api/organization/bind → bind_create；:1178-1181 PUT/DELETE /api/organization/bind/{id} → bind_update/bind_delete。路由对外可达。

表结构佐证（迁移脚本）：
    migrations/029_create_missing_core_entity_tables.sql:246-254  x_org_bind(id PK, identity_id, group_id, role, create_time, deleted_at) —— 无 creator 列，故正确归属须经 x_org_identity.creator 关联，而 bind_update/bind_delete 完全没有关联。
    migrations/022_create_org_assemble_tables.sql:45-53  x_org_identity(id PK, name, unit_id, identity_id, creator, create_time, deleted_at) —— creator 即归属列，bind_create 正是据此判定。

## 三、真执行证据（oneshot 直调 pub handler，非静态断言）

方法：自建独占 PostgreSQL 容器（端口避开他组占用段），DATABASE_URL 指向本地回环占位连接串；shared::migrate::run_migrations 建全表；seed 两名占位测试身份与两条 bob 归属的 x_org_bind 行（一条用于改、一条用于删）与两条不同 creator 的 x_org_identity；用 shared::testing::test_session 构造 alice/bob 会话；经 organization_core_entity_router 的 oneshot 注入 Extension(test_session) + Extension(DatabaseConnection) 直调 handler；直接回查 x_org_bind 落库值。判据以测试输出 passed/FAILED 为准。

契约：临时测试 oa4rust/backend/crates/organization_core_entity/tests/r640a_org_bind_idor_contract.rs（runnerSha 见第五节），跑完即删、不入库。

真跑命令（占位连接串）：DATABASE_URL=<占位本地回环连接串> cargo test -p organization_core_entity --test r640a_org_bind_idor_contract -- --nocapture --test-threads=1

实测输出（1 passed; 0 failed）：
    R640A anon update status=401 Unauthorized                       —— (e) 匿名反臂通过
    R640A bind_create non-owner status=403 Forbidden                —— (d) 兄弟 create 拒绝非本人
    R640A bind_list anon status=200 OK contains_bob=true            —— (c) bind_list 无 session 裸读含 bob 绑定
    R640A cross-person update status=200 OK                         —— (a) alice 跨人改 bob 绑定
    R640A cross-person update landed role=superadmin                —— (a) 改 role 已落库
    R640A cross-person delete status=200 OK                         —— (b) alice 跨人软删 bob 绑定
    R640A cross-person delete landed deleted=yes                    —— (b) 软删已落库
    R640A source guard OK: update/delete no ownership, create gated —— 源码守卫：update/delete 无归属谓词、create 有
    R640A ALL ARMS PASS — IDOR present (甄别坐实)
    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

五臂结论：(a) 跨人改 role/identity_id 落库 = 坐实；(b) 跨人软删落库 = 坐实；(c) bind_list 裸读全表 = 坐实；(d) bind_create 拒绝非本人 = 反证成立；(e) 匿名 401 = 反臂成立。

## 四、行为现状与生产鉴权披露（Fail Loud）

- 现状：bind_update / bind_delete 的会话只用于"是否登录"，不用于"是否本人"；任何登录人知 bind id 即可改他人组织绑定的角色字段并落库、可软删他人绑定。bind_list 无身份过滤，返回最近 20 条未删绑定映射。
- 生产鉴权据实披露：本次 oneshot 直调 router，验证的是 handler 自身缺陷（不区分身份）；生产部署若全局挂 auth_middleware，则外部匿名请求在该中间件层被 401 拦截，故本项不是"匿名可读可写"，而是"认证后越权"。判级据此未判 P0。
- 未执行项据实披露：未做 GUI 人工点验；本报告基于 handler 级 oneshot 真测试与源码逐行锚点，不含前端交互验证。
- 下游权限影响：x_org_bind.role 承载组织绑定角色语义，越权改值可致依赖该字段的业务逻辑误判；具体权限后果取决于消费方，本报告不夸大。

## 五、SHA 分列（据实，分列不可互相复算）

- 审计对象 lib.rs 全文 SHA-256：c03c9cf6a0ea359756daea20d72a46cbccaf08113087f695cd06063970d80d2d
- 临时契约 runnerSha（测试脚本全文 SHA-256）：41a1f1ef5ee41501e0021123edfcc3f2d6ad9b245ab1d216b854820e6954e0f6
- 本文档全文 SHA-256：由审计工具落盘后计算，见任务完成报告（docSha）；此值不在文档内自引用。

## 六、零真值声明

- 本报告与随附测试均无真值入库：无真实域名/IP/主机名/SSH 端口、无口令/token/私钥、无个人路径或真实用户目录。连接串一律以本地回环 + 占位符表示，测试身份为占位夹具名（非真实账号）。LEAK=0。
