# R601 日历事件列表身份盲 IDOR 审计（验证型甄别）

- 结论：P2。两个日历事件列表端点仅按 calendarId 过滤、不读取会话身份，任何已登录用户可读任意用户的私有日历事件（含 title/content/location）。
- 真父 HEAD：500f51e67efa8dba0cac2132e30789152bfdbb25
- 零源码改；唯一产出：本文档。测试夹具与日志留 gitignored 证据目录，用后即删。

## 立项与范围
- 台账 docs/ops/current-state.md 全文检索「事件列表 IDOR / event_list 身份盲 / R601 事件」零命中，本缺陷从未专项甄别。
- 实例1（legacy Pool 轨）：crates/calendar/src/lib.rs:622-655 event_list(pool, Path(calendar_id))；路由 crates/calendar/src/routes.rs:30 GET /api/calendar/event/list/{calendarId}。
- 实例2（ORM 轨）：crates/calendar_core_entity/src/lib.rs:799-813 event_list_by_calendar(db, Path(calendarId))；路由同文件 :885-888 GET /api/calendar/core/entity/event/list/{calendarId}。
- 两实例属同一缺陷的两种实现，合并为 1 项，不硬凑独立项。

## 机制锚点
- 实例1 SQL（:630-631）：SELECT id,calendar_id,title,content,location,start_time,end_time,all_day,visibility,status,createor FROM CAL_EVENT WHERE calendar_id=$1 AND status=OPEN ORDER BY start_time ASC；仅绑定 calendar_id，无 session、无 createor/is_public 归属谓词。:637-652 将 content/location 一并返回。
- 实例2 ORM（:799-813）：.filter(CalendarId.eq(..).and(Status.eq(OPEN))).limit(100)，同样无身份过滤。
- 全局 auth_middleware 只验登录、向 request 注入 Session，不按资源归属过滤，故泄露面为任意已登录用户而非匿名。
- 兄弟铁证：crates/calendar_assemble_control/src/u2.rs:715-753 event_list_filter 带 session 且 O9 归属收窄（c.is_public = true OR e.createor = $3 OR c.createor = $3，:736，注释 canary 实证）；crates/calendar/src/lib.rs:229-254 calendar_get 私有日历走 require_owner（:251-253）。同一系统其余端点有归属门禁，唯此两列表端点遗漏。
- 前端真实消费：CalendarApp.vue:460（legacy）/:357（ORM）、Dashboard:199。
- 测试覆盖缺口：tests_generated.rs:16 仅 SKIPPED（event_list requires tower），无任何鉴权归属断言。

## 方法与决定性实测（临时 PG + 真迁移）
- 起独立临时 PostgreSQL（主机/端口/账号/口令均占位 <TEST_HOST>:<PORT> / <USER> / <PASS>，真值由环境变量注入、不入库）。
- 用 shared::migrate::run_migrations 建真表；seed 用户 A 的私有日历（is_public=false）与唯一事件，content 内嵌合成标记串 R601_LEGACY_SECRET / R601_ORM_SECRET / R601_GATE_SECRET。
- 直接 oneshot 调真 handler：不带身份调 event_list(Path=A 日历)；ORM 轨用 sea-orm 连接调 event_list_by_calendar；对照轨注入用户 B 的 Session。

| 断言 | 实测 | 结论 |
|---|---|---|
| legacy event_list 读 A 私有事件 | count=1, marker_seen=true | 身份盲坐实 |
| ORM event_list_by_calendar 读 A 事件 | marker_seen=true | 身份盲坐实 |
| calendar_get(B, A 私有日历) | err=true | 兄弟门禁有效 |
| event_list_filter(B) | filter_leak=false | O9 归属收窄有效 |
| 合计 | 4 passed / 0 failed | 全绿 |

- 决定性日志（gitignored）：test-results/r601/r601-test.log，字节 sha256=57f6a371e5c34d0943ee65d5f3309975020a10ae32d17595561316d234db4150（16 行）。
- 夹具（用后即删）：tests/r601_event_list_idor_audit.rs，字节 sha256=cdf3b6ed2617fc582856be0d26f87eedcabbffdbf9420240a8bb3d7057379bc5（118 行）。

## 三分裁决（据实，不夸大不降级）
1. 需知 calendarId：攻击者须先获知目标 calendarId，非无差别遍历全库，故判 P2 而非 P1。
2. 非匿名：全局 auth_middleware 要求登录，泄露面是任意已登录用户，据实披露。
3. 非凭据：读端点泄露的是事件元数据与内容（title/content/location/时间/创建人），不含 token/口令，据实披露。
4. 两实例：legacy 与 ORM 为同一缺陷的两种实现，合并为 1 项，不硬凑独立项。

## Fail Loud
- 首跑 ORM 轨失败：seed 把 timestamp 列传 String 触发 WrongType{postgres:Timestamp}；改 SQL 内嵌字面量后仍因 chrono NaiveDateTime 空格格式 ParseError(Invalid)；最终改为 SQL 内嵌时间字符串字面量、由 Postgres 按列类型转换，4/4 通过。均为夹具问题，非被测逻辑失败。
- 工作区 Cargo.toml:247-248 warnings=deny，未用变量曾致编译失败，已清除。
- 非回环端口/账号/口令均为临时真值，未写入任何入库文件。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=dc4c5830392fa4ac6c154f2283abbd3f6ae9700a8fa3ec262a7a8a83341bf30c
- 证据日志字节 sha256=57f6a371e5c34d0943ee65d5f3309975020a10ae32d17595561316d234db4150
- 夹具字节 sha256=cdf3b6ed2617fc582856be0d26f87eedcabbffdbf9420240a8bb3d7057379bc5
- crates/calendar/src/lib.rs=8ec5ecb23ccea8998717bc4faea3fdb2346157bc11e11de87fca1eb2f04f2e73
- crates/calendar_core_entity/src/lib.rs=877c4cdb2770228d2cbcf3c429de4a68acd013b0b50f32607a8d2d1a610fda1a
- crates/calendar_assemble_control/src/u2.rs=35055626b29b8f4347fd6d78be468e159e5e7f6283097a33dbf73deb10d3b71d
- crates/calendar/src/routes.rs=9d112112d9704a5418ce3994399d9af126f7a2d9a5ee64b55023311de186d220

## 修复方向
- 两列表端点注入 session，按兄弟端点 O9 谓词收窄：仅 is_public=true 或 createor=当前 person 的事件可见；私有日历非拥有者返 403/空。