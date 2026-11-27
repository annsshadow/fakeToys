# R602 日历事件列表身份盲 IDOR 修复（R601 判 P2 的修复闭环）

- 真父 HEAD：993b8cf3b0bcc0ee99a14c9b5515d5977bf1f9bb
- 修复目标：R601 审计判定的两裸读端点（legacy SQL + core entity ORM）身份盲 IDOR P2。
- 写域：M crates/calendar/src/lib.rs、M crates/calendar_core_entity/src/lib.rs、A 本文档；零改其他端点/SQL 业务语义/前端/迁移/Cargo。

## 修复机制
- SQL 版 crates/calendar/src/lib.rs:622 event_list 增 session:Option<Extension<Session>>（:624）；查 CAL_CALENDAR is_public/createor（:632-638），私有日历（false 或 NULL）走 require_session + require_owner（:639-646），照同文件 calendar_get 现成口径；公开日历与不存在日历保持可读。事件查询 FROM CAL_EVENT ... status=OPEN 零改（:648-653）。
- ORM 版 crates/calendar_core_entity/src/lib.rs:799 event_list_by_calendar 增 session:Option<Extension<Session>>（:801）；查 x_cal_calendar createor/is_public（:807-816），私有走 require_session(...).person_unique + require_owner(&db.0,&owner,&actor)（:817-824），公开日历保持可读。ORM 事件查询 cal_event::Column::Status.eq(OPEN) 零改（:826-836）。
- 无会话读私有日历：require_session 返 401（Option 为 None 时不 panic）；非属主读私有：require_owner 返 403；本人与 admin 放行。

## RED → GREEN（真父源码覆盖定点）
- 记录修复态两文件 sha → git stash 两文件回真父 → cargo clean → 契约 RED EXIT=101：3 failed（SQL 版无会话实得 200 OK、ORM 版无会话实得 200 OK、source_guard ORM 版归属门缺失）。
- git stash pop 恢复（sha 一致）→ cargo clean → 契约 GREEN EXIT=0：3 passed。定点 RED 有效，队长可独立 replay。
- RED 日志（gitignored）test-results/r602/r602-red.log 39 行，字节 sha256=a889636990b0f00e83c8473e9602111032bd005e0ac7dc275ec8d82224666647。
- GREEN 日志（gitignored）test-results/r602/r602-green.log 14 行，字节 sha256=08d52bbe51a7813590116d850cae87b90dab1f63ca3ada4d168920df1fc0f978。

## 三 verify（全据实）
- (a) cd oa4rust/backend && cargo test -p calendar -p calendar_core_entity：21 passed / 0 failed（calendar 8/8 + is_public_null_safety 1/1 + core entity 12/12）。前置：给临时 PG postgres 库跑 shared::migrate::run_migrations 灌真表（非本次引入的环境依赖）。
- (b) cd oa4rust/backend && cargo test --test r602_event_list_idor_contract：3 passed / 0 failed。
- (c) cargo clippy -p calendar -p calendar_core_entity -- -D warnings：EXIT=0；cargo fmt --check 全 workspace EXIT=1 为 baseline 既有（bbs/src/tests.rs 等大量既有格式差异），rustfmt --edition 2021 --check 单文件 calendar/src/lib.rs EXIT=0，core 文件 EXIT=1 但 diff 在 :221 既有代码（HEAD 版同格式），本次 hunk 仅 +624/+629/+801/+804 不覆盖；契约文件不在写域不修改。
- parity 冒烟：cargo test -p parity generated_tests::parity__calendar__event_list：1 passed（无 Session oneshot 经 router 非 404、不 panic；签名漂移守卫通过）。

## 核心底线实测（契约 3 用例）
- SQL 版：无会话读 A 私有事件 401；bob 读 A 私有 403；alice 本人可读 CANARY；admin 可读；bob 读公开日历事件 OK。
- ORM 版：无会话 401；bob 403；alice/admin 可读；公开可读；源码守卫确认两变体均含归属门。

## Fail Loud
- alice CANARY 首次未命中根因=ops_edit_file 保留 mtime 致 cargo 复用陈旧二进制（非产品逻辑、非测试断言缺陷）；cargo clean -p 两包后单线程与并行均 3 passed，据实披露。
- verify(a) 初跑两失败均为既有测试环境依赖（空库缺表 / 无 DATABASE_URL 连接拒绝），非本次引入；给临时 PG 灌真迁移后全绿。
- 临时 PG 主机/端口/账号/口令均占位，由环境变量注入、不入库；harness 与临时日志用后即删（gitignored）。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=5cba7e86b3d7c00672ca53617f3d7b56182fb712f506c0876388cede50f65786
- crates/calendar/src/lib.rs=d476511db17d5908ce71e3c1afdc5ad707060bce28be186aa69bd294fc1029fd
- crates/calendar_core_entity/src/lib.rs=4d8a148cc77d81144624fd7641ba466b6181dde77f1be9f764b4cda3f89cfc1f
- tests/r602_event_list_idor_contract.rs（不在写域，临时契约）=878629f639f9b482cb7ae19a8ea1b81362d2ed17a02dd3b1ac615088d7c66624

## 结论
- 两变体归属门禁正确、核心底线全满足；RED→GREEN 定点有效；三 verify 据实全绿（fmt baseline 差异披露）；精确单 commit push。