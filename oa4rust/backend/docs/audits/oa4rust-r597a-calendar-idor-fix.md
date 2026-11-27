# R597a 日历 list/my 身份盲 IDOR 修复审计（t155）

- 规则：R597a（承 R596a P1 甄别）。分支：opt1000/oa4rust-pc-ai-refactor。
- 真父：056cade063d4d954eebb817559388e8e3148e429（他组 R597b appeal 提交令 HEAD 前移，但本任务 3 目标文件 HEAD 版本 sha 与真父备份完全一致，RED replay 有效）。
- 缺陷：两处 calendar_list_my 无会话门、无归属过滤——登录用户不带身份即可拉全表私有日历（SQL 版与 ORM 版各一）。

## 一、修复（最小改动）
- calendar/src/lib.rs calendar_list_my：签名加 Option<Extension<Session>>，require_session 取 person_unique；SQL 加 AND (createor=$1 OR target=$1) 并绑定 actor。对齐兄弟端点 calendar_assemble_control/src/u2.rs:164-178 口径。
- calendar_core_entity/src/lib.rs calendar_list_my：同样加会话门；复用同 crate CALENDAR_READ_PREDICATE（is_public/createor/admin），因谓词含 $2 而本查询只一个绑定，替换为 $1 后走 Statement 原生 SQL（SELECT ... FROM x_cal_calendar WHERE status='OPEN' AND deleted_at IS NULL AND {predicate} ORDER BY create_time DESC LIMIT 50），映射改 try_get。
- calendar/tests/is_public_null_safety.rs：改 seed 三行（alice NULL / bob false / bob true UNIT）+ session_for("probe-alice")，断言仅读到归属自己的一行——保留 is_public IS NULL 解码 false 不 panic 原意图，且不固化身份盲。
- 口径差异据实：SQL 版归属=createor OR target，不含 is_public；ORM 版谓词=is_public OR createor OR admin，不含 target。两版一致的是无会话 401、本人可读、他人私有不可见。

## 二、写域
写域精确 5 文件：M calendar/src/lib.rs、M calendar_core_entity/src/lib.rs、M calendar/tests/is_public_null_safety.rs、M calendar_core_entity/src/tests.rs（获队长授权 amend inScope，仅一行断言适配）、A tests/r597a_calendar_idor_contract.rs；本审计文档为第 6 项（非源码写域）。零改：routes.rs、前端、其他端点、Cargo.toml、DB/SQL 表结构。

## 三、契约与 RED/GREEN
- 契约 tests/r597a_calendar_idor_contract.rs：4 个 tokio test，router + tower oneshot 直调（两 crate handler 签名在真父/修复两版稳定）。4 scratch 库独立名（oa_r597a_sql/orm/null/guard），fresh_scratch 用 DATABASE_URL 连库 + shared::migrate::run_migrations + seed，用完 DROP。Session 用 serde_json 反序列化构造（不新增 chrono 依赖）。
- 两版响应键名不一致据实：SQL 版蛇形 my_calendars/unit_calendars，ORM 版驼峰 myCalendars/unitCalendars；契约遍历 data 下所有数组收集 id，不绑定具体键名。SQL 版 public-cal 不在 my（口径不含 is_public）；ORM 版 public-cal 在 my、alice-target 不在 my（谓词口径）。
- RED replay：cp 三份 parent.bak 覆盖源码 + touch → cargo test --test r597a_calendar_idor_contract EXIT101 3failed（sql/orm/null）1passed（guard 守卫：公开端点与谓词未变）→ 恢复三份 fixed.bak + touch → GREEN EXIT0 4passed。HEAD 未变。
- 踩坑：ops_edit_file 保留 mtime 致 cargo 判 Fresh 链接旧 rlib；每次覆盖源码后必须 touch 强制重编译。

## 四、既有测试断言适配（队长授权，据实披露）
- calendar_core_entity/src/tests.rs:49 test_calendar_list_my_returns_success 原断言 INTERNAL_SERVER_ERROR(500)，固化旧 bug（无 session 落到 DB mock 失败）。本次会话门使无 session 在首个 DB 访问前返回 401（acceptance①），该断言失败 left=401 right=500。
- 同文件 :69-70 test_calendar_get_returns_not_found 已有完全先例（get 端点会话门修复时同步改成 UNAUTHORIZED/401，注释"读门禁：无会话在首个 DB 访问前就返回 401"）。测试名 returns_success 却断言 500 本就畸形。依队长裁决 A 授权，只改这一行 500→401 并加同风格读门禁注释，其他行与其他文件零改。

## 五、三 verify（全 EXIT0）
- V1 literal：cd oa4rust/backend && DATABASE_URL=<占位> cargo test -p calendar -p calendar_core_entity → EXIT0：calendar 8passed / integration 1passed / core_entity --lib 12passed。
- V2 literal：cd oa4rust && pnpm test:contracts → EXIT0：7 files / 22 tests 全 passed。
- V3 literal：cd oa4rust/backend && cargo clippy -p calendar -p calendar_core_entity --all-targets -- -D warnings → EXIT0 无告警。cargo fmt --check（workspace 级）报既有 bbs/src/tests.rs 等格式差异（rustfmt 版本行为，非本任务引入），据实披露；目标文件 rustfmt --check 亦仅报既有 use 排序（同样非本任务引入）。

## 六、SHA256 证据
runnerSha（sha256(JSON.stringify(meta)) 紧凑串，与下行文件 sha256sum 分列）：871388cae5faa48a00db6c2761d0460577b08045ccb089233a3021b71128f666

文件 sha256sum（backend/ 相对路径）：
- crates/calendar/src/lib.rs = 8ec5ecb23ccea8998717bc4faea3fdb2346157bc11e11de87fca1eb2f04f2e73
- crates/calendar_core_entity/src/lib.rs = 877c4cdb2770228d2cbcf3c429de4a68acd013b0b50f32607a8d2d1a610fda1a
- crates/calendar/tests/is_public_null_safety.rs = cf6a36b8fa07f0013872eb0649a61d742a9df5eb4697c58e10d71d80ae89b7a2
- crates/calendar_core_entity/src/tests.rs = 14d280c1c88ca9781c846c3a62baa2c44460904f2366d6da9a8ef3a0be267db6
- tests/r597a_calendar_idor_contract.rs = 6b05447f31501b6a9e9b5cca5d692df7f30431ed396234cd243b7511e3250be4

日志 sha256sum：
- RED /tmp/r597a-red.log = 7aafd12b3193db7eba4166ad538dec2dcfe1086dc8640dd0e05d6fe92484521e
- GREEN /tmp/r597a-green-final.log = adc42ad4ee1ebf34fe458ad35c37d276484917d5fc939240b75e228251edc270
- V1 /tmp/r597a-v1final.log = 46a4e57af404d1d939f041e5d668ac8924e3e36dce0199d409eca8f6cb8ddf12
- V2 /tmp/r597a-v2.log = e33e5b0e948c715a9a15fd2ba54e9ba4dcf30231482b265f9471498f33013dc2
- V3 /tmp/r597a-v3clippy.log = f341e2081d8eae1ef7ad50a02bc801480ec3083b053e71ccd567e3046b8ee2de

真父备份（gitignored）：calendar-src-parent.bak.rs=38287351 / core-src-parent.bak.rs=24ec7ed9 / is_public-parent.bak.rs=d63234be。测试连接走 <TEST_HOST>:<PORT> 占位（实际值由运行环境 DATABASE_URL 注入，不入库）；临时 PG 容器用完即清。
