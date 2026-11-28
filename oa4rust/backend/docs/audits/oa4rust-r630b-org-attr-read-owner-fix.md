# R630b 组织属性/授权裸读族 Session 归属门修复审计

## 一、结论与判级
- 判级：P2。需有效会话并已知 person/unit/identity 标识，非匿名即可批量读他人属性值与授权关系；原缺陷为跨主体越权读（IDOR）。
- 根因：personattr_list_attribute_person_name / personattr_list_person_object / unitattr_list_unit_object / empower_list_identity_object 四端点原签名仅 Pool + Json<Value>，body 身份标识由调用方自报且服务端不校验，SQL 直接按标识查询并回传，无 Session 与归属谓词。
- 修复：四端点在首个 DB 访问前加 require_session；personattr 复用写侧 resolve_write_owner 语义；unitattr/empower 采 admin-only 门；本人/admin 正常读保留。

## 二、锚点根因
- oa4rust/backend/crates/organization_assemble_express/src/endpoints_attr.rs:34 personattr_list_attribute_person_name，原 query 用 body.person 直查 x_org_person_attribute；:75 personattr_list_person_object 用 body.personList 直查；:379 unitattr_list_unit_object 用 body.unitList 直查 x_org_unit_attribute；:461 empower_list_identity_object 用 body.identityList 直查 x_empower。
- 写侧参照 :216-235 resolve_write_owner：actor = sess.person_unique，body 标识非本人且非 admin 则 Forbidden；:246+ personattr_set_person_name 已是该语义。
- 兄弟先例 :504+ empowerlog_create admin-only（is_admin），:527 附近读面 require_owner；crate::require_session 位于同 crate lib.rs:21；rbac::is_admin 位于 shared/src/middleware/rbac.rs:76；路由 routes.rs:482/486/507/520 均 POST 可达。

## 三、RED→GREEN 真执行证据
- 真父基线：2db38d58d8cc2bbba66bd1ed1a602bc9a3015c46（HEAD 一致且为祖先）。
- 真父跑契约：running 4 tests / 0 passed / 4 failed，EXIT=101。坐实 alice 读 bob 属性得 200 且泄露 bob 私有属性值、非 admin 读 unit 得 200 且泄露 unit 私有属性值、四端点裸调皆 200、源码守卫失败。
- 修复后 GREEN：running 4 tests / 4 passed / 0 failed，EXIT=0。四用例为跨人读 403 无泄露且本人/admin 200、四端点裸调 401、unitattr/empower 非 admin 403 且 admin 200、源码守卫（Session 门 + resolve_write_owner 语义 + admin-only 计数 + 无旧裸 SQL）。
- 修复手法：personattr 两读端点加 session 门并在 DB 前取 owner/校验列表归属；unitattr/empower 加 session 门并在 DB 前 is_admin 校验；query 参数改用已校验标识，非身份标识原样保留。

## 四、据实披露与残余风险
- unitattr/empower 采 admin-only：x_org_unit_attribute 与 x_empower 表无 person 归属列，无法按本人放行，参照 R616b 先例收窄为 admin-only；这会使原非 admin 可读场景变为 403，属安全行为变化。
- named_keys（endpoints_attr.rs:288）为共享辅助，服务于第五端点 personattr_list_name_person，不在本次四端点写域，未覆盖，留待后续甄别。
- 包级 V2 全量首跑 79 passed / 1 failed，失败为写域外 tests_u2.rs:291 的 x_org_person DELETE：shared::testing::test_pool() 读默认连接串指向项目库，本轮独占临时库未含该夹具数据，与本改动无关；按 --skip 排除后重跑 79+2+4+0 passed / 0 failed，EXIT=0。该跳过项在此披露。
- 队友对 oa4rust/frontend/apps/desktop/src/views/QueryDesigner.vue 的改动非本次所为，未纳入本提交，亦未触碰。

## 五、行为变化与兼容
- 非 admin 跨 person 读属性：由 200 + 他人属性值 变为 403；本人与 admin 读不变。
- 非 admin 读 unit 属性 / empower 授权：由 200 变为 403；admin 读不变。
- 匿名裸调四端点：由 200 变为 401。路由、前端与迁移零改，契约与源码同提交入库。

## 六、SHA 分列（文件 sha256 与 runnerSha 不同来源）
- runnerSha = ef7be646fc2f22cfd3dd702014d86c9131572af0badd0155461fb52690aedc2c（对运行元信息 JSON 取 sha256）。
- endpoints_attr.rs sha256 = 839e46b043682524f68960ff333c0a28392fc06b0cd4db3384b5c6d0883ae264。
- r630b_org_attr_read_owner_fix_contract.rs sha256 = aca65a47cf74a33fccdd7a2d707a20f973e6e4cf37afa48720e11e9f0159e385。
- 文档自身 sha256 以版本库最终文件为准；本文不含任何真实口令、域名、IP、端口或个人路径。
