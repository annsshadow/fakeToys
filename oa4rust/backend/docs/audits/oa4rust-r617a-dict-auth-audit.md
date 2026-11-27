# R617a general_core_entity dict/dict_item 写端点授权级别甄别

## 一、结论与定级（三分据实）

- P2（有据）：真父六个 dict/dict_item 写端点只验会话存在（require_session），任意登录人可 200 改/删全局字典与字典项，属授权级别缺失的越权写；不硬凑 P1，不掩盖为 0。
- 非 P0：调用需有效会话（裸请求 401 已坐实），非匿名；故不判 P0。
- 需知 id 否：id 可由公开 GET /api/general/dict/list 与 item/list 枚举（list 无会话 200 且返回 id，已坐实），不满足“需知 id”缓解，维持 P2。
- 层级 L1/0：根因在 core entity handler 只挂会话档、未挂授权级别档，单点甄别，不涉及跨服务鉴权链重构。

## 二、锚点与根因

- oa4rust/backend/crates/general_core_entity/src/lib.rs:26-30 require_session 只验会话存在即返回 Session，无角色/归属判断。
- dict_create :215/:220、dict_update :280/:286、dict_delete :323/:328、dict_item_create :355/:360、dict_item_update :425/:431、dict_item_delete :475/:480 均为 require_session 直落 find_by_id + update/delete_by_id，无 admin 门。
- 同文件 :46-62 已有 is_admin_db（纯 SQL 查 admin 角色、失败 fail-closed）、:66-88 resolve_creator 已用 AppError::Forbidden，但六个 dict 写端点均未调用。
- 实体 general_application_dict.rs / general_application_dict_item.rs 表 x_general_application_dict / x_general_application_dict_item 无 creator/owner 列，字典为全局配置性质。

## 三、兄弟对照与测试缺口

- 对照：同 crate file/invoice 写端点在 SQL WHERE 内用 CREATOR_OWNER_PREDICATE（:36-41）做“本人或 admin”收口；dict 类因无归属列仅留会话档，注释 :23-25 亦明示“dict 类全局配置仍为会话档”。
- security_gate_tests_b65.rs:44-87 只测 12 个写端点“无会话 401”，不测有会话时授权级别；security_gate_tests_idor_r6w.rs 只覆盖 file/invoice 归属，dict 授权级别无回归。

## 四、oneshot 真测试证据（Fail Loud）

- 起独占临时 PostgreSQL（端口由环境变量注入，文档不写真值地址/端口），apply shared::migrate 全量迁移，seed 两 person（alice 非 admin / admin 角色）与 4 条 dict + 4 条 dict_item，走真 general_core_entity_router 的 axum oneshot。
- 修前真父实测（1 passed / 0 failed / EXIT=0，全部断言坐实“缺陷存在”）：
  - GET /api/general/dict/list 无会话 200 且 body 含 d1 → 公开枚举 id 成立。
  - alice（非 admin）POST dict/update/d1 → 200，DB name 由 dict-one 变 hacked。
  - alice POST dict/delete/d2 → 200，DB 记录数变 0。
  - alice POST dict/item/update/di1 → 200，DB value 由 v1 变 hv。
  - alice POST dict/item/delete/di2 → 200，DB 记录数变 0。
  - admin POST dict/update/d3 → 200，DB name 变 admin-edit；admin POST dict/delete/d4 → 200，DB 记录数变 0（反臂通过）。
  - 裸会话 POST dict/update/d1、dict/delete/d1、dict/item/update/di1、dict/item/delete/di1 → 均 401。
- 说明：本轮为验证型甄别，零源码改；契约断言的是“修前缺陷态”，真父上应全 pass（缺陷被证实），未做 RED→GREEN 修复循环。
- 未跑浏览器 GUI 真页面；前端 grep（oa4rust 下 *.vue/*.ts/*.tsx/*.js 搜 api/general/dict/(update|delete|item/update|item/delete)）零命中，未定位本端点前端消费点，以上据实披露。

## 五、溯源码哈希（sha256，非真值）

- 真父 HEAD = 3b4d35d4edf918bdfd4817a7dbafa9866e41df5f
- lib.rs = 9acd1117288a4665298b55ff001c56bdb35294abc1f18ce895d4e171645cd286
- general_application_dict.rs = 6b08c2a3826755ce6914cad955e055645443ca90839b1675a204283c84ad04bc
- general_application_dict_item.rs = fc3cca225cfb6b70fb89de5d45d990312532ea91ff6bcbe556d99ba549a232b0
- 临时契约（验证后删除） = 81707e91e57c8b35ce7d79330e8d8b738ae3037ac8bea60a899f5b7a9824e12c
- oneshot.log = 830af4c21b1d057d190393699320ee9f7d974a1ef8efa24f6e76305b197f6f1b
- runnerSha（meta 紧凑 JSON）= 38794c1831010746dead0c319c807c01dbfe3b57d6818286b4393c1cf9bda8b0
- runnerSha meta 字段（按序）：task,round,parent,verdict,oneshot,lib,dict_entity,dict_item_entity,contract,oneshot_log；值即本节各 sha 与真父 HEAD。
- 文件 sha256 与 runnerSha 分列；文档 sha256 提交前重算写入，不含自身哈希。

（0 真值：无密钥/口令/真域名/IP/私路径；连接串端口由环境变量注入，仅占位。）