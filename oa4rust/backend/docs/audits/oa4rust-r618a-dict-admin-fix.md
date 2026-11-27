# R618a general_core_entity dict 六写端点 admin 门真修复审计 (t233)

## 结论
- 判级 P1（已修复）：六处字典写端点仅校验会话存在即放行，任何登录用户可创建/更新/删除全局字典及条目；修复后非 admin 统一 403。

## 缺陷与根因
- oa4rust/backend/crates/general_core_entity/src/lib.rs:26-30 require_session 只验会话存在、丢弃 person_unique。
- 六写端点均 require_session(session)? 后直写 DB，无角色/归属判定：
  - dict_create:215 / dict_update:280 / dict_delete:323
  - dict_item_create:355 / dict_item_update:425 / dict_item_delete:475
- 全局字典 id 可由公开 GET list 枚举，威胁面为任意登录用户。

## 修复（仅六写端点）
- 每端点：let actor = require_session(session)?; if !is_admin_db(&db.0, &actor.person_unique).await { return Err(AppError::Forbidden); }
- is_admin_db(:46-62) 为现成 SQL admin 门（auth_person 关联 auth_person_role/auth_role，name=admin 且未删未禁；查询失败 false fail-closed），直接复用未改。
- 门置于首个 DB 写之前；Json 提取器保持末位（axum 要求）；成功返回结构零改。
- 读端点（dict_get/dict_get_item/dict_list/dict_item_list）与 file/invoice 端点零改。

## 可达性与消费点
- 路由 lib.rs:1091-1099 六写端点均 POST，可达；backend app .merge(general_core_entity::router(...))。
- 前端 OrgViewer.vue:451/452/579-582 冒烟探针调用六写端点（据实：非真实管理 UI，但确有调用点，区别于无消费点场景）。

## 验证（oneshot 集成契约）
- 契约 tests/r618a_dict_admin_contract.rs：fresh scratch + 迁移 + seed alice/admin，六端点非 admin 403 且 DB 未改、裸 401、admin 200 落库、source_guard。
- tests/security_gate_tests_b65.rs（任务声明写域新建集成测试）：六写端点非 admin → Err(Forbidden)。src/security_gate_tests_b65.rs 零改。
- 真父覆盖 lib.rs → RED EXIT101（dict_writes left 200 / right 403 @ dict/create；source_guard 失败）；恢复修复版 → GREEN EXIT0（2 passed）。
- V1 定点：cargo test -p general_core_entity --test r618a_dict_admin_contract → EXIT0 2 passed。
- V2 包级：cargo test -p general_core_entity → EXIT0（lib 37 + contract 2 + b65 1 + doc 0；一次性迁移测试用后即删）。
- V3：cargo fmt -p general_core_entity --check EXIT0；cargo clippy -p general_core_entity -- -D warnings EXIT0。
- 临时 PG 地址 <TEST_HOST>:<PORT>；契约日志 <PROBE_LOG_PATH>；一次性迁移测试与临时库用后即删。

## 据实披露
- 行为变化：普通登录用户原可 200 增改删全局字典，修复后 403——属安全修复，非功能回归。
- 判级 P1 而非 P0/L1/0：需登录会话且需知字典 id，非未认证、非数据外泄、不写库破坏、不崩溃；修复前影响为全局字典被篡改。
- 未跑浏览器 GUI；断言基于 oneshot 路由与源码守卫。

## 哈希
- runnerSha（meta 紧凑 JSON 的 sha256）：f0873ad2bdcbbd9ea51f3b1798dd5216436b58a1b1b522990191033ced43ba04
- 源码 src/lib.rs sha256：6734cdaf84f67132c76812eed88058468651f04b37edacbf46325bc6724a0c14
- 契约 sha256：61cfd47ec3814686c21eb351e234c2913331ff56341a540ff3fee5d9ea0a184b
- 本文档文件 sha256 见任务回执（文档不含自身哈希）。
