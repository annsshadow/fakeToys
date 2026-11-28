# R628b ai chat_write_completion_extra 身份盲写 IDOR 修复

## 结论
判级 P2（需有效会话 + 知悉 chat id + 仅覆写 extra 扩展字段非正文）；已真修复，RED→GREEN 坐实。

## 漏洞机理（真父锚点）
- oa4rust/backend/crates/ai_assemble_control/src/lib.rs:2700-2745 chat_write_completion_extra
- :2715 require_session(session)? 仅验会话存在，返回值丢弃。
- :2723 UPDATE x_ai_chat SET extra=$2::text::jsonb WHERE id=$1 AND deleted_at IS NULL 无归属谓词。
- 任意登录人凭他人 chat id 即可覆写其对话扩展数据；网关无 key 时 gateway=simulated 但仍真实落库。
- 路由 oa4rust/backend/crates/ai_assemble_control/src/routes.rs:92-95 POST /api/ai_assemble_control/chat/write/completion/extra。

## 两表关联依据
- 024_create_gap_closure_tables.sql:14-20 x_ai_chat 无 user_id 列（id/conversation_id/role/content/create_time）。
- 025_add_missing_columns.sql:5 补 creator；037 补 deleted_at；080_add_ai_missing_columns.sql:6 补 extra JSONB。
- 026_create_ai_app_conversation.sql:14-19 x_ai_conversation.user_id CHARACTER VARYING(255) NOT NULL。
- 归属须经 x_ai_chat.conversation_id → x_ai_conversation.user_id 关联取得。

## 修复
- 保留 require_session 绑定会话；UPDATE 前先 JOIN 两表取 c.user_id（带双表 deleted_at IS NULL）。
- Some(user_id) → shared::middleware::require_owner(&pool,&session,&user_id)（本人/admin 放行，否则 403）。
- None（无关联/软删/不存在）→ fail-closed 返回 NotFound（404），杜绝盲写。
- 成功响应结构 value/id/gateway 零改；兄弟 chat_delete_clue_id:2639-2660 同 crate 范式。
- 写域外零改：网关转发/gateway 判定/路由/Cargo/migrations/前端。

## RED→GREEN 矩阵
| 场景 | 真父 | 修复 |
|---|---|---|
| alice 凭 bob chat id 覆写 extra | 200（漏洞）| 403 且 DB 未改 |
| bob 本人覆写 | — | 200 落库 |
| admin 跨人覆写 | — | 200 |
| 匿名裸调 | — | 401 |
| 不存在 id | — | 404 |

## verify
- V1 定点：真父 RED EXIT=101 → 修复 GREEN EXIT=0（1 passed）。
- V2 包级：迁移库 cargo test -p ai_assemble_control EXIT=0（33 lib + 1 契约 + 1 迁移 + 0 doc，全绿）。
- V3：cargo fmt --check EXIT=0；cargo clippy -D warnings EXIT=0。
- 临时 PG 容器独占端口（占位 <TEST_PORT>），用后即清。

## 据实披露
- 未跑浏览器 GUI；DB 连接串以 <TEST_HOST>/<TEST_PORT> 占位，真值不入仓。
- 未迁移基础库时 V2 有 4 个环境性失败（关系不存在，写域外测试）；建迁移库后消除，非本修复引入。
- 源守卫断言修复含 require_owner 与两表 JOIN。

## SHA-256（内容摘要，非真值）
- 真父 HEAD: ad4a17f1748309ac54795155d21c0c34dbbf28b1
- 修复前 lib.rs: e2abac22cd6fbaabc702ac2300a7f114e0ef2228f9853e85f33477895e3f4b65
- 修复后 lib.rs: 5ba5e5a7e244f806ed48e93e794aede0ecc23b8a2e8fe84d571d0a6f3dae887f
- 契约测试: b15aa3a973906e80021d97efc8d0418bcbe32ef356e60f9ecf8bb28628fc479f
- red.log: 365894a60c9c9398747a79e1e21442e1a5cb6d5d670aacd73934eeff687bb369
- green.log: 02bb765bcd5afa747eb1864dd1d2bf406e07da4e0ba679f8576d4e2b65cc37eb
- v2-migrated.log: 0cf852597b8c2176441dbee7dc669b94c4b453e96ccd2089a4a3abf1650f22d5
- v3fmt.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
- v3clippy.log: 357fc0cdcfbc909987bc9b22e4b98337d6f1d3ab23230f5bc33715290b991352
- runnerSha: 859d5e6501c9ad0e0f9d0026b4cf39317e240452b3d8f2ee86956c7883d5ed77