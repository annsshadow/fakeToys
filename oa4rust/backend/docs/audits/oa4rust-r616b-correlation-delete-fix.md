# R616b correlation_core_entity delete_by_id 身份盲软删修复

## 一、结论与定级（三分据实）

- P1 已修：真父 delete_by_id 仅验会话存在，按主键 find_by_id 后无条件置 deleted_at，任意登录人可删他人关联记录，属身份盲越权写（R615b 判 P1）。本轮修复为非 admin 一律 403、admin 200 软删。
- 为何非 P0：调用需有效会话且需知道目标关联 id（非匿名、非枚举全量），故不判 P0；但后端落库破坏数据完整性，仍按 P1 处置。
- 层级 L1/0：根因在 core entity handler 的身份判定缺失，单点修复，不涉及跨服务鉴权链重构。

## 二、锚点与根因

- oa4rust/backend/crates/correlation_core_entity/src/lib.rs:204 require_session 丢弃返回值；:205 find_by_id 仅主键无归属；:210-222 置 deleted_at 软删。
- entities/correlation.rs 表 x_corr_c_correlation 列仅 id/source_type/source_id/target_type/target_id/weight/create_time/deleted_at，无任何 creator/owner 归属列。
- ops_grep is_admin|ownership_error|OWNER_PREDICATE 于本 crate 零命中，无现成归属/管理员 helper。

## 三、方案与理由（据实）

- 不做逐人归属门：source_id 是任意对象 id，不等于 person_unique，强行当 owner 会误伤合法关联；实体亦无 owner 列可判。
- 定稿 admin-only 门：delete_by_id 属全局关联元数据管理面动作；新增纯 SQL is_admin_db 复用 bbs_core_entity:34-50 模式，查 auth_person/auth_person_role/auth_role 的 admin 角色，不引入新依赖。
- 非 admin 返回 AppError::Forbidden（error.rs:202 映射 403 ExceptionAccessDenied），admin 通过后才软删。

## 四、行为变化与已知遗留（据实披露）

- 行为变化：原“本人凭自己 id 删除 200”在新语义下若本人非 admin 变 403。据 R615b 甄别，前端 ProcessWork.vue 消费的是另一 correlation_service 端点，无本端点删除 UI 消费点，此变化不影响现有 UI。
- 已知遗留：create 端点（lib.rs:139-197）同样丢弃会话身份、source/target 全由 payload 控制。本轮不扩大写域，未修改；留待后续独立评估。
- 未定位本端点前端 vue 消费点；未跑浏览器 GUI 真页面，以 oneshot 真 router + 源码真执行替代，据实披露。

## 五、契约与验证（Fail Loud）

- 契约 oa4rust/backend/crates/correlation_core_entity/tests/r616b_correlation_delete_admin_contract.rs，2 用例：跨用户 403 且 DB 记录 deleted_at 仍 NULL、裸 401、非 admin 本人 403（行为变化）、admin 200 且落库软删、admin 删不存在 id 200 error body 且不影响其他记录。
- oneshot 起临时 PG 并 apply 迁移、seed 两非 admin person 与一 admin person 及关联记录，真实 router 执行；连接串走环境变量注入，文档不写真值地址或端口。
- 真父 RED：cargo test 定点 EXIT=101，核心断言“跨用户 delete 必须 403”失败（left 200 / right 403），1 passed 1 failed。
- 修复后 GREEN：定点 2 passed 0 failed EXIT=0。
- V2 cargo test -p correlation_core_entity：unit 11 passed、integration 2 passed、doc 0，EXIT=0。
- V3 cargo fmt --check -p correlation_core_entity 与 cargo clippy -p correlation_core_entity -- -D warnings：均 EXIT=0（fmt 首轮因新契约格式失败，cargo fmt -p 后复跑通过，lib.rs diff 未扩散）。
- 无 SKIP、无冒充 pass；一切失败据实记录。

## 六、溯源码哈希（sha256，非真值）

- 真父 HEAD = 91eaf3d5e81bd24b8a1870c5ff36be5c50916be0
- lib.rs = c4b7d163ffa390bffe942077670d3cccf7320a197e199967333fc19b8fd08d92
- correlation.rs = 08f0f75a57c769eab120daab83a945d6ab77b49a7f827a891100634aa5052c1f
- 契约测试 = ceee710499ce9e471f27e30b31550dbf636db14d65b49a281b850185b318d3cb
- red.log = 72d9a868592cdf903b06a7a72a11ac4a1fb4a591dbac978386d4bce2ee4906eb
- green2.log = cdf882b20376d4d5206e5a399c45089f94b05a2ef0c6ad588abe86d3594ff042
- v2.log = c41b34ddb5b957a1e2ddcb582f11aee05ab1eec07f50f8ea9e6b36f392a31950
- v3-clippy.log = ac8f059feb226fd15aa351da30cb527ca14b4213f059b5d80cf951a6dbac2153
- runnerSha（紧凑 JSON meta）= 0c71f88e440ae623bc8061afc486c1fe51bad0bb5abd31564aaa02c598b2e04d

（文件 sha256 与 runnerSha 分列；文档 sha256 由提交前重算写入，不含自身哈希。）