# R323 论坛回复时间字段后端补齐（u2 族端点）

## §1 背景与来源

承接 t112 甄别（oa4rust/docs/ops/oa4rust-r323-reply-time-triage.md，commit 088d7b718）：判定 B 真缺陷 P2——PC 论坛详情回复卡片时间恒空（6 条 .reply-time 全空、真实 UI 新建后 7 条仍空），H5 同批回复时间正常（形如 admin · 时间）。根因为后端 u2 族端点漏 SELECT create_time，而 assemble/control 族的 reply_list_sub_id 正常产出。本轮 test-first RED→GREEN 补齐字段，三端点字段名统一 createTime，不改前端 PC/H5。

## §2 实现（bbs_assemble_control/src/u2.rs，4 处）

- ① u2_reply_get（:1162）SQL 补 create_time::text：SELECT id, topic_id, content, creator, accepted, create_time::text FROM x_bbs_reply WHERE id = $1 AND deleted_at IS NULL。
- ② u2_reply_get 映射（:1190 起）accepted 块后加 createTime：按 Option<String> 读 create_time，缺失回退空串（map(Value::String).unwrap_or(Value::String(String::new()))）。
- ③ u2_reply_filter_list 两条 SQL（:1214 带 topic_id 分支、:1233 无过滤分支）SELECT 列表均补 create_time::text。
- ④ u2_reply_filter_list 映射（:1268 起）creator 块后加同款 createTime（Option<String> 缺失回退空串）。
- 字段名统一 camelCase createTime，与前端 BBSForum.vue:565 兼容映射 createTime ?? create_time ?? '' 对齐；lib.rs:772 reply_list_sub_id（H5 消费，产出 snake_case create_time）未改；admin 门、routes.rs、任何无关端点、迁移文件均未改。

## §3 RED→GREEN

- 契约 1（行为，live-gated）：tests_generated.rs 新增 test_reply_time_fields_present——造数据（x_bbs_topic 补 author_id/section_id，x_bbs_reply 补 subject_id，均 NOT NULL）后 PUT reply/filter/list/page/1/count/100 body {topicId} 断言每条 createTime 非空、GET reply/{id} 断言 createTime 非空，末尾清理。修复前 RED：panicked at tests_generated.rs:483「RED: reply/filter/list createTime must be non-empty; got=Object {content, creator, id, topicId}」。
- 契约 2（静态不变量，防回归）：test_reply_time_fields_static_invariants 断言 u2_reply_get SELECT 含 create_time::text 且映射 createTime、u2_reply_filter_list 两分支均含 create_time::text 且映射 createTime+unwrap_or 容错。修复前 RED：panicked at tests_generated.rs:436「u2_reply_get SELECT must include create_time::text」。
- 造数据/编译坑（逐项修）：x_bbs_topic.author_id 与 section_id、x_bbs_reply.subject_id 均 NOT NULL 必须补；同一 &String 绑两个不同列类型触发 PostgreSQL E42P08（varchar vs text inconsistent types deduced），拆成独立 String 变量；Rust assert 消息含 reply/{id} 的花括号被当格式占位符 E0425，改写为文字描述。
- GREEN：touch u2.rs+tests_generated.rs 强制重编后 cargo test -p bbs_assemble_control --lib = 51 passed; 0 failed; 0 ignored（基线 49 + 新增 2），既有 reply/section/forum 用例全部不回退。

## §4 部署与真浏览器复验

- Linux ELF 构建：gitignored oa4rust/test-results/build-linux-r323reply.sh（docker rust:1.98-bullseye 内层 bash -lc 单引号、touch u2.rs+tests_generated.rs 强制重编、cargo +1.98.0 build --release -p oa4rust --offline --target-dir /workspace/target/linux），后台 13m05s 完成 R323REPLY_BUILD_DONE，新 ELF 118344040 字节（旧 118342728），时间戳 2026-11-21 23:10:07，file=ELF executable（Windows PE32+ 在测试服 203/EXEC，禁用）。
- 原子部署：oa4rust/test-results/deploy-backend-r323reply.sh（scp→备份 bak-r323reply→install→mv 原子替换→healthcheck 失败回滚，远端命令单引号避免本地展开）输出 SERVICE_ACTIVE=active / BACKEND_DEPLOY_OK，exit 0。部署前已确认工作树仅本任务两文件、无并行后端改动。
- 真浏览器：oa4rust/test-results/r323-reply-verify.cjs + r323-reply-verify-run.sh（Playwright Chromium headless、1440x900 PC + 390x844 H5、凭据 ssh 运行期读取不回显、base 从既有脚本提取），证据 oa4rust/test-results/r323-reply-verify-fix/（4 PNG + evidence.json，gitignored；source=本地无头 Chromium 渲染，非物理真机）。
- 首轮 NOT_VERIFIED 根因（非修复失败）：复验脚本停在论坛「全部」tab，该 tab PUT subject/index/list/page/1/count/20 返回空；经页面内 fetch 诊断，主题实际在「我的」tab（POST subject/filter/listsubjectinfo/page/1/count/20）。修脚本切「我的」tab + 无回复主题点 [aria-label=关闭主题详情] 关闭再试下一张。
- 复验判定：R323_REPLY_VERIFY_DONE verdict=REPLY_TIME_VERIFIED，found=0、pcTimeNonEmpty=true、filterCT=true、newReplyOK=true、h5TimeNonEmpty=true、apiFailures=0、errors=0。主题 8eb22714-4499-41dc-8553-5207c379152e（7 条回复）：PC .reply-time 7 条全非空（28分钟前/42分钟前/45分钟前/50分钟前/1小时前×3），PC PUT reply/filter/list 响应含 createTime（如 2026-11-21 22:47:43.782568）；真实 UI 新建回复后 7→8 条、新卡片首条 刚刚；H5 同主题 .reply-meta 8 条 admin·时间全非空（2026-11-21 22:09 … 23:15）；PC/H5 loggedIn 均 true。

## §5 脱敏

真域名/测试服地址一律以 <YOUR_TEST_DOMAIN> 占位，0 真值；口令仅运行期经 ssh 从 .local.keys 读取、不回显不入库；主题 id 为业务数据非凭据。本文档不含任何真域名、IP、口令、私钥或个人路径。

## §6 已知边界

- 仅补齐 u2 族三端点（reply/filter/list 两分支 + reply/{id}）的 create_time 输出；不改 assemble/control 族 reply_list_sub_id（本就正常）。
- 历史回复 create_time 为 NULL 时按 Option<String> 回退空串，不伪造时间；本轮只修字段供给，不回填历史数据。
- PC/H5 前端未改；时间格式化仍由前端 fmtTime 负责（空白值返回空串）。

## §7 证据索引

- 契约：oa4rust/backend/crates/bbs_assemble_control/src/tests_generated.rs（test_reply_time_fields_present / test_reply_time_fields_static_invariants）。
- 源码：bbs_assemble_control/src/u2.rs:1162/:1190/:1214/:1233/:1268；对照 lib.rs:772 reply_list_sub_id（未改）；BBSForum.vue:166/:171/:565/:820（未改）。
- 甄别前置：oa4rust/docs/ops/oa4rust-r323-reply-time-triage.md（t112 判 B P2）。
- 真浏览器证据目录：oa4rust/test-results/r323-reply-verify-fix/（gitignored，本地无头 Chromium 非物理真机）。

## §8 结论

R323 论坛回复时间字段缺失缺陷闭环：后端三端点响应项 createTime 非空且与 H5 同主题一致、真实 UI 新建回复后时间非空、0 pageerror/0 API失败。test-first RED→GREEN 有据，Linux ELF 原子部署成功，脱敏与精确单提交符合铁律。项目 ≥1000 轮持续迭代，本轮非终态。