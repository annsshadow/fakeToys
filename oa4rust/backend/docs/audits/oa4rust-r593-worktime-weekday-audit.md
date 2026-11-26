# R593 worktime 伪星期算法验证型甄别报告

## 结论

判 P2（可达真缺陷，附限制）：后端 worktime 四个处理器在无该日期配置行时走错误伪星期回退，公开 GET 请求即可稳定触发错判；候选 B（forwarddays days 参数失效）亦 oneshot 真测坐实。两项均属 API 层缺陷，据实披露限制、不夸大为 UI 可见。

## 立项性质与范围

- 验证型甄别（真测试非静态预判）；零源码改，唯一交付本文档。证据日志置于 gitignored oa4rust/test-results/r593/（.gitignore:28 覆盖）。
- 连接参数由环境变量 DATABASE_URL 注入，本报告一律以 <TEST_HOST>:<PORT> 与占位口令表达，0 真值。
- 临时 PG 容器与建表仅供甄别，跑完即清理；集成测试夹具用后即删。

## 精确源码锚点

- crates/general_assemble_control/src/lib.rs:2956/3008/3060/3111：四处理器（indefinedholiday/indefinedworkday/isholiday/isworkday）共用 let w=(((year-2000)*365+(month-1)*30+day)%7) as u8，w>=5 判周末→节假日；固定 365 天/年、30 天/月，忽略闰年与每月实际天数。
- lib.rs:2800：let end_date = format!(拼接 start 加 days 天); :2804 WHERE date >= $1 AND date <= $2（TEXT 列字典序上界），致 days 参数失效（候选 B）。
- crates/general_assemble_control/src/routes.rs:103-113：forwarddays/start/{start}/days/{days}、indefinedholiday/{date}、indefinedworkday/{date}、isholiday/{date}、isworkday/{date} 均公开 GET，仅 .layer(Extension(pool))，无 Session 提取器/无鉴权，参数用户可控。
- 建表依据：migrations/032_create_assemble_control_tables.sql 的 x_general_assemble_worktime(date TEXT, is_holiday BOOL, is_workday BOOL, ...)。
- 前端：oa4rust/frontend/apps/desktop/src/views/OrgViewer.vue:546/547 调用 indefinedholiday/0 与 indefinedworkday/0（传字面量 0）；派发写 548/551，实测为 546/547，据实更正。

## 方法与决定性命令

- 起临时 PG（postgres:16-alpine，仅测试占位口令），容器内建 x_general_assemble_worktime（count=0，无配置行）。
- 写真集成测试 crates/general_assemble_control/tests/r593_worktime_weekday_audit.rs（2 个 #[tokio::test]，经 general_assemble_control::router(test_pool()) oneshot 真处理器，跑完即删）。
- 命令：DATABASE_URL=<占位> cargo test -p general_assemble_control --test r593_worktime_weekday_audit -- --nocapture → test result: ok. 2 passed; 0 failed。

## 决定性实测输出（节选，完整见证据日志）

- R593_SAMPLE 2026-01-03 real=Sat isHoliday=false isWorkday=true
- R593_SAMPLE 2026-01-04 real=Sun isHoliday=false
- R593_SAMPLE 2024-02-29 real=Thu isHoliday=true
- R593_SAMPLE 2000-01-01 real=Sat isHoliday=false
- R593_SAMPLE 2025-06-01 real=Sun isHoliday=false
- R593_INDEFINED date=0 isHoliday=false indefined=true isWorkday=true indefined=true
- R593_DEFECT_CONFIRMED samples=5 endpoints=isholiday/isworkday/indefinedholiday/indefinedworkday
- R593_CAND_B rows_in_range=3 returned=1（01-01/01-05/01-20 三行，days=30 应返 3 条，实返 1 条仅 01-01）

## 错误样例对照表

| 日期 | 真实星期 | 正确 isHoliday | 实测 isHoliday | 判定 |
|---|---|---|---|---|
| 2026-01-03 | 周六 | true | false（且 isWorkday=true） | RED |
| 2026-01-04 | 周日 | true | false | RED |
| 2024-02-29 | 周四 | false | true | RED |
| 2000-01-01 | 周六 | true | false | RED |
| 2025-06-01 | 周日 | true | false | RED |

队长确定性脚本对 2000–2027 全量比对：4170/10227 天判错（约 40.8%），随日期漂移非固定偏移。次生问题：year<2000 时 (year-2000) 为负，%7 as u8 截断，据实记录不在主裁决内。

## 三分裁决

- 可达 P2（采纳）：正常公开请求、DB 无该日期配置行走伪星期回退即错判，覆盖多端点多日期、可稳定复现，属客观 API 缺陷。候选 B 同为 API 层可达缺陷（days 参数失效），并入 P2。
- 不可达 L1（排除）：无需故障注入、无需鉴权绕过、无需深层造数，公开 GET 即可。
- 正常 0（排除）：五个错误样例 + 候选 B 均实测坐实，非语义正确。

## 限制披露（据实，不夸大）

- (a) 仅回退分支：处理器先 query_opt 查 x_general_assemble_worktime 该日期行，有权威配置行则走 DB 真值分支规避错判；缺陷仅在无配置行时暴露。生产若有完整配置可缓解，但回退路径本身错误仍成立。
- (b) 当前无 UI 可见影响：OrgViewer.vue:546/547 仅传字面量 0（len()!=10 → indefined 分支，indefined=true，测试实测确认不进伪星期）；前端无消费者以真实日期调用 isholiday/isworkday/forwarddays。故本轮判 P2 仅指 API 层缺陷，不宣称 UI 可见。
- 候选 B 前端无消费点（forwarddays 仅在通用端点探测中传 0/0），同样为 API 层缺陷。

## Fail Loud

- 临时 PG + 测试夹具为本地可复现环境；测试连接真值与口令均未入库，仅占位符表达。
- 集成测试用后即删，工作树零源码改。前端行号派发写 548/551、实测 546/547，已据实更正。
- 全量 4170/10227 判错统计为队长确定性脚本结果，本轮引用未独立重跑全量，据实标注来源。

## sha256 证据表

runner 紧凑 JSON（sha256(JSON.stringify(meta))）与文件字节哈希（sha256sum）分列，不可混用：

- runnerSha(R593 meta) = 1824a15a4439d9942dd22818552e5e56e3c23449f33a44273f5676bdb41b11bb
- 证据日志 oa4rust/test-results/r593/r593-test.log（字节）= 90289fa85856d944aa2f516a6a383c941556f17c60e5d3ad7b8c359d31129f89（11 行）
- 测试夹具 r593_worktime_weekday_audit.rs（字节，已删）= 07d1828502ae684d24274856bbe6f5feb0ef0708ad76662da5aae110b3268ec5
- 源文件 crates/general_assemble_control/src/lib.rs = f28941dbbc52170f00072b016a81179ffa6f4f97dbe17a6ddc1fcf7be0e36679
- 源文件 crates/general_assemble_control/src/routes.rs = a1123e1b03c222d4f3f205fc23a7db37af3a4a4e1a9988f9559fc20d580dc8c7
