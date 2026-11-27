# R620a 冒名族七端点身份绑定修复审计

## 一、结论
本次针对冒名族七写端点做身份绑定真修复，判定为 P1（越权写入）已修，非 P0，评级 L1-0。
五个组织写端点与两个消息写端点此前仅校验会话存在，丢弃会话身份，转而从请求体自行取 person、unit、fromIdentity、toIdentity、sender，任何已登录用户可伪造他人身份写入，构成跨用户越权写入。
本次将七端点统一绑定到会话人：身份不再由请求体自证；非本人写入需 admin 显式放行，否则 403。门在首个数据库写之前完成。

## 二、锚点与根因
org endpoints_attr.rs 五端点 personattr_set、personattr_append、unitattr_set、unitattr_append、empowerlog_create 均为裸 require_session 丢弃身份；公共写入 attr_write_values 从 body 取 owner。
message lib.rs 两端点 connector_create、ws_create 同为裸 require_session，person 与 sender 从 body 取；connector 写入 x_message_instant.person 与 x_message.sender，ws 写入 x_message_consume.sender。
实体表 x_org_person_attribute、x_org_unit_attribute、x_org_empower_log、x_message_instant、x_message_consume 均无 owner 约束，数据库层不拦越权。
既有测试仅覆盖无会话 401，不覆盖有会话的授权级别，因此缺陷长期未被发现。

## 三、方案与理由
复用共享判定 shared::middleware::rbac::is_admin（deadpool Pool 版，SQL EXISTS 查 admin 角色，fail-closed），不新造判定、不新增依赖。
org 新增 async resolve_write_owner：请求体身份缺省则取会话人；等于会话人则放行；不等于会话人时仅 admin 放行，否则 Err AppError::Forbidden（映射 403）。attr_write_values 签名改为接收已解析 owner，不再自行从 body 取。
empowerlog_create 对 from 身份加同类门：非 admin 且 from 不等于会话人则 403。
message connector_create 与 ws_create 直接将 person 与 sender 取会话人 person_unique，删除请求体自证。
GET、路由、Cargo、migrations、前端均零改；门位于首个数据库写之前。

## 四、行为变化与遗留
行为变化：协议面请求体身份字段不再被信任。以他人身份写入的调用方（如以 system 身份代写的集成场景）此后需持 admin 凭据，否则返回 403。此为预期收紧。
本人写入（body 身份等于会话人）保持 200 落库；admin 可代写他人。
未做 GUI 实测；仅限后端端点契约与单测层验证。

## 五、Fail Loud 验证
RED：真父版 alice 伪造 bob 写 person 属性返回 200 并落 bob 名下，坐实越权；source_guard 因无 admin 门同步失败。
GREEN：修复后 alice 伪造 bob 返回 403 且数据库未改；alice 本人 200 落库；admin 反臂代写 200 落库；裸请求 401。两 crate 契约各 2 用例全通过。
V1 定点契约两 crate 共 4 用例通过。V2 全量单测 org 80、message 117 全部通过（须用已迁移契约库，默认 postgres 库未迁移会 E42P01，与本次改动无关）。V3 cargo fmt --check 与 clippy -D warnings 两 crate 全绿。
首次 fmt 曾越界格式化 message 的 tests.rs 与 tests_generated.rs，已 git checkout 还原，最终 diff 仅含两个写域源码文件加两个新契约。

## 六、sha256 与 runnerSha（分列）
endpoints_attr.rs = 57b8dd8287ed36e1c452a944d6fbc6b2137e7cb948ba6b4d115c97f051ab7d7b
message lib.rs = cde6abe94663d49146be2d814ef74a57a9cef764feb8d847bf9f6663b624ad30
org 契约 = be1616c96eb8e8ca579b3002ea99243650af7e1d659879a801cd4ffeac9d6838
message 契约 = 418ae075a58a3b2bf700daa904d248a3b5372577cc03f69e00f65ee9f8d553fd
runnerSha = dd66bbb9085cbd110e64146ccc98ccc7ba72c4b6e875b9f8feae39d8b75c5725
注：runnerSha 由本轮元数据经 sha256 计算，与上述文件 sha256 分列，互不替代。
