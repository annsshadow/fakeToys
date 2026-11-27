# R610a message 已读标记身份盲 IDOR 写修复审计

- 任务：t199 / R610a（implementation）
- 裁决：P1 —— 已读标记越权写入（非越权读、非崩溃、非数据删除）
- 连接信息一律占位：`<TEST_HOST>:<PORT>`，本文不含任何真值

## 1. 根因

crates/message/src/lib.rs 两个写端点 update_single（/api/message/consume/:xid/type/:type）
与 mark_read（/api/message/mark_read/:xid）均调用 require_session(session)? 后丢弃返回的会话值，
UPDATE x_msg_message SET xconsumed = true WHERE xid = $1 仅按裸 xid 定位，未按消息归属人 xconsumer 过滤。
任意登录用户凭他人 xid 即可把他人消息标记为已读；请求体/路径不含 owner 校验，属身份盲 IDOR 写。
兄弟端点 consume_list（:58）与 unread_count（:231）均按 xconsumer = $1 读取，形成读写归属不对称。

## 2. 修复（仅两个端点）

- 两端点 require_session(session)? 改为 let sess = require_session(session)?，保留会话值。
- UPDATE 前先 SELECT xconsumer FROM x_msg_message WHERE xid = $1；行存在时取 owner，
  调用 shared::middleware::require_owner(&pool, &sess, &owner).await?：
  管理员（auth_person LEFT JOIN auth_person_role/auth_role，r.name='admin'）或本人放行，其余 Err(AppError::Forbidden) → 403；无会话 require_session → 401。
- UPDATE 追加 AND xconsumer = $2 兜底（绑定 owner），防止 TOCTOU 或后续 SQL 重构绕过归属门。
- 记录不存在时沿用原 n==0 / rows 空早返回语义（updated=false / markedRead=false），不泄露存在性。
- 守卫零改：require_session 定义、custom_create、consume_list、unread_count、routes.rs、Cargo.toml、migrations、其他 crate 均未修改。

## 3. 验证（定点契约，3 用例不 ignore）

契约：crates/message/tests/r610a_message_consume_owner_contract.rs
经 message::routes::router(pool).layer(Extension(test_session(who)))+tower oneshot 真发 POST，
五路径覆盖：alice 凭 bob 裸 xid → 403 且 bob.xconsumed 保持 false；裸请求 → 401；
alice 凭本人 xid → 200 且 returned flag=true；admin 代操作 bob → 200；不存在 xid → 200 且 flag=false。
source_guard 用例以 include_str! 断言两端点会话保留、xconsumer 归属查询×2、require_owner×2、UPDATE AND xconsumer×2 且旧裸 UPDATE 不残留。

- RED replay：用真父 lib.rs 覆盖源码并 touch 后定点 → EXIT 101，0 passed / 3 failed
  （两端点越权断言 left:200 right:403，source_guard『必须保留会话值』失败）。
- GREEN：恢复修复版 lib.rs 并 touch 后定点 → EXIT 0，3 passed；复核再次 EXIT 0。

## 4. 三 verify（全 EXIT 0）

V1 cd oa4rust/backend && DATABASE_URL=<占位>/oa_r610a_base cargo test -p message --test r610a_message_consume_owner_contract → 3 passed。
V2 cd oa4rust/backend && DATABASE_URL=<占位>/oa_r610a_base cargo test -p message → lib 16 passed + 契约 3 passed + doc-tests 0，0 failed。
V3 cd oa4rust/backend && cargo fmt -p message -- --check → EXIT 0 空输出；cargo clippy -p message -- -D warnings → EXIT 0。

## 5. 据实披露

- 一次性建库迁移测试 tests/r610a_migrate_once.rs 用于创建 oa_r610a_base，验证后已删除。
- cargo fmt -p message 对同包 src/tests.rs 产生真父既有的格式差异（纯空白/换行），非本任务修改，未纳入提交。
- 契约 scratch 库按端点拆为独立库名，消除 V2 并行执行时的 E23505 库名冲突。
- 未跑浏览器 GUI 真页面；以 oneshot 真 handler 请求 + 源码守卫覆盖业务行为。
- 派发父 a8caa2857 之后工作树 HEAD 前移，目标 lib.rs 在真父备份与 HEAD 侧字节一致，RED replay 有效。

## 6. 指纹（分列，不含本文自身 sha）

- runnerSha（sha256(紧凑 JSON meta)）：e1906426492639cec4cae39091017dd8f9a37ceab9f903b2c4400a57b8c9dbe6
- 真父 lib.rs sha256：7e6fe0a71e52a9a3e0d01d84d2fe265ae94e55816b378c70597dfef9d3873e6c
- 修复 lib.rs sha256：ccdeb587213fd12f4d7d9016a1869c7563617ffdf3df6c2e6928991f438a9562
- 契约 sha256：b8be562d512078f4894a99f834af2f170b3d05dffb987efcb8bcd34e5fdc90ce
- RED 日志 sha256：e6ec3ed0e8779d4f673f852954cb21c6a62639144a6c830569ff859750f5a96d
- GREEN 复核日志 sha256：3370f5bea5a9d49d10ee3143ede11a3d0ab21fdf5646d0ffa8450da16709b9a4
