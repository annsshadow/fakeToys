# R623b 考勤统计导出端点身份盲读甄别（只读）

## 结论 / 真父 / 零源码改
- 结论：判 P2（可达真缺陷 2 项）——两统计端点签名无 Session、body 直取 person，任意已登录用户可查他人状态计数，前端恒传空 person 导出全员统计；有认证门、仅聚合计数非逐条 PII，故 P2 而非 P1。
- 真父：1c87a418e50ffd913c887133fa825a30e6793e93（台账 §623 立项）；当前 HEAD == 真父。
- 本次零源码/配置改动；oneshot 夹具用后即从源码树删除，唯一入库物为本报告。

## 立项与范围
- 目标：甄别 v2 统计/导出端点能否按 body.person 身份盲读他人考勤状态计数；以同 crate group/rebuild 的 require_admin 作语义分叉铁证。
- 不计入：R623a 逐条明细越权族（逐条 PII），本项数据形状仅为 status+count 聚合；台账对 statistic/export/filter 零命中。

## 机制锚点
- attendance_assemble_control/src/lib.rs:8081 detail_statistic_aggregate，SQL :8091-8095 `WHERE ($1='' OR person_id=$1) AND ($2='' OR date>=$2) AND ($3='' OR date<=$3) GROUP BY COALESCE(status,'')`——person 空即聚合全员。
- :8119 v2_detail_statistic_filter、:8138 v2_detail_statistic_export_filter 均 `(pool: Extension<Pool>, body: Option<Json<Value>>)`，无 Session；:8161-8170 extract_stat_filter 从 body 直取 person/startDate/endDate（trim，缺省 ''）。
- 兄弟正面：:8176 v2_group_rebuild_detail_group_date 带 session + require_admin（:4617），管理类写操作有 admin 门。
- 路由真实消费：src/routes.rs:291 statistic/filter、:292 statistic/export/filter；前端 AttendanceApp.vue:488/:496 exportData 传 `person:''` 导出全员 CSV。

## 方法与决定性实测（独立 PG + 真迁移 + oneshot 直调）
- 独立临时 PostgreSQL（主机/端口/账号/口令均占位 <TEST_HOST>:<PORT> / <USER> / <PASS>，真值由环境变量 DATABASE_URL 注入、不入库），shared::migrate::run_migrations 建真表；seed 三人 7 条明细（self 3、other 2、third 2）。
- 真 handler 直调 + 裸会话经 auth_middleware 的 oneshot；日志见 gitignored 证据目录。

| 断言 | 实测 | 结论 |
|---|---|---|
| 空 person → export | success；status1=4、status2=3（全员 7 条） | 全员聚合暴露 |
| 指定他人 person → filter | success；status1=1、status2=1（他人 2 条） | 任意 person 可查 |
| 指定第三人 → export | success；status1=2 | 导出同样盲读 |
| 裸会话 → middleware | 401 | 认证门有效，非匿名 |
| 源码签名 | 两 handler 无 Session；body 直取 person | 与静态一致 |
| 合计 | 2 passed / 0 failed | 身份盲读坐实 |

## 三分裁决（据实，不夸大不降级）
1. 可达真缺陷 2 项（P2）：filter/export 两 handler 均缺身份绑定；任意已登录用户可传他人 person 读其状态计数，前端恒传空 person 导出全员计数。需有效会话（裸 401）、数据为聚合计数非逐条 PII（逐条由 R623a 覆盖），故 P2。
2. 不可达 L1：0 项——两 handler 真实可达且实测坐实。
3. 正常项：裸会话 401 门有效；路由 :291/:292 与前端 :496 真实消费；兄弟 group/rebuild 的 require_admin 未受影响；extract_stat_filter 对三字段做 trim 与缺省归一。
4. 预判（空 person 聚合全员 + 指定他人可查）已由真库 oneshot 坐实，非静态猜测。

## Fail Loud
- 夹具迭代首次编译失败：ActionResult.data 为 Option<Value>，三处 .get 未解包（E0599/E0308），改为 .as_ref().expect("data") 后全绿，不掩盖。
- 临时端口/账号/口令为真值，仅经环境变量注入，未写入任何入库文件；夹具用后即从源码树删除，git status 零残留。
- 当前 HEAD == 真父；若后续有人改动上述锚点源码需重跑。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=c20bb8300c8961657198493eb93b9cbd6ebf236194de7873500d2d9fa4ea179b
- oa4rust/backend/crates/attendance_assemble_control/src/lib.rs=2fa0d41af986e80322debdb30ec15219696910a78132aa5ebb21831132ccf047
- oa4rust/backend/crates/attendance_assemble_control/src/routes.rs=d829b2f6e01ffac6b900cb59882d0d995104cb799f191cb9623ebd1741e1c0b9
- oa4rust/frontend/apps/desktop/src/views/AttendanceApp.vue=938c0839b705345ed41039f9794bfbd833b868320b735176e9bb452e5d017d21
- oa4rust/backend/migrations/032_create_assemble_control_tables.sql=722c2899500c201245ffc3b612ae00fd8c05c21aa8512bab2453b57266428d9e
- 夹具字节 sha256=fdcb7b15ce380462cc4c84232c639b5ced0609cc36262be64427a6b5fd7483de（gitignored，用后即删）
- 决定性日志字节 sha256=60add22dd0fb896bf027496801253a90915fa7309fdf98411045b7b17e7074e3（gitignored）
- meta.json 字节 sha256=c20bb8300c8961657198493eb93b9cbd6ebf236194de7873500d2d9fa4ea179b（gitignored）

## 修复方向
- 给两统计端点注入 Session：普通用户仅允许 person='' 且 person 强制取会话人；管理员可传任意 person 或空聚合全员；参照兄弟 require_admin 分流。前端导出按需传当前会话人或由后端按会话裁剪。
