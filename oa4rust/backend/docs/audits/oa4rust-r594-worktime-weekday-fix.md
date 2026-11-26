# R594 worktime 伪星期回退算法修复（Sakamoto helper）

## 结论

修复 R593 甄别确认的 P2：后端 worktime 四个公开 GET 端点在无日期配置行时走错误伪星期回退。改为 Sakamoto 纯整数真日历算法，DB 命中分支与 SQL、候选 B、其他文件、Cargo.toml 均零改。

## 立项与范围

- 实现型修复，写域精确 3 文件：lib.rs（helper+4 处替换）、契约测试、本文档。证据日志在 gitignored oa4rust/test-results/r594/。
- 连接参数经环境变量 DATABASE_URL 注入；本报告以 <TEST_HOST>:<PORT> 与占位口令表达，0 真值。临时 PG 容器与 scratch 库用完即删。
- 真父 HEAD = ee580d49cbec93b2cc6eb90c8a1047a10bb1797e；分支 opt1000/oa4rust-pc-ai-refactor。

## 修改内容

- lib.rs:2924 新增 worktime_fallback_weekday(date: &str) -> Option<u8>：Sakamoto 表 T=[0,3,2,5,0,3,5,1,4,6,2,4]，y-=m<3，raw=(y+y/4-y/100+y/400+T[m-1]+d)%7（原始 0=Sun..6=Sat），+6 偏移转 0=Mon..6=Sun 以适配下游 w>=5 判周末。日期格式非 YYYY-MM-DD 返回 None（保持 indefined 语义）。
- lib.rs:2969/3010/3051/3091：四端点 None 回退分支统一替换为 let weekday = worktime_fallback_weekday(&date);，原伪算法 (year-2000)*365+(month-1)*30+day 共 4 处全部移除（total_days 残留 0）。
- 下游 4 处 let is_weekend = weekday.map(|w| w >= 5).unwrap_or(false) 及 indefined=weekday.is_none() 语义零改；DB 命中 Some(row) 分支、SQL、候选 B（forwarddays days）、其他文件、Cargo.toml 零改，无新依赖。
- 据实披露：队长给的裸公式标 0=Mon..6=Sun 有歧义；实测原始 Sakamoto raw 是 0=Sun..6=Sat，故采用 +6 偏移，经 node 真日历逐例校验（2026-01-03 raw6→mon5 周六；2026-01-04 raw0→mon6 周日）。

## 契约测试（oneshot 真处理器）

- 文件：crates/general_assemble_control/tests/r594_worktime_weekday_contract.rs（合法 Rust 标识符文件名；初版带点号因 invalid character in crate name 更名）。2 个 #[tokio::test] 不 ignore。
- 测试 1：四端点 × 2026-01-03 周六/01-04 周日/2024-02-29 周四/2000-01-01 周六/2025-06-01 周日/2026-01-05 周一反例/date=0 indefined 不进伪星期。周六 indefined/workday 断言 isWorkday=false（非工作日，据实修正初版方向错误）。
- 测试 2：DB 配置行（is_holiday=false, 2026-01-03）优先于回退；源码守卫 helper 定义恰 1、调用恰 4、total_days=0、w>=5 四处、indefined 四处。
- 自建 scratch 库（DATABASE_URL 基础连接 + cfg.dbname），跑完 DROP；两测试用独立 scratch 库名规避并发建库竞态。

## RED → GREEN 与决定性踩坑

- RED replay：git show HEAD 真父版 lib.rs 覆盖工作区 + touch → EXIT101 2failed（核心行为断言 line71 isHoliday + 源码守卫 line121 helper 定义 left0 right1）；恢复修复版 → GREEN EXIT0。HEAD 全程未变。
- 决定性踩坑：ops_edit_file 保留 lib.rs 原 mtime，比编译产物 rlib 旧，cargo 判 Fresh 不重编译，测试二进制仍链接旧算法导致 probe 输出 isHoliday=false。修复 = touch lib.rs 强制重编译。
- V2 全量首轮 EXIT101：既有 --lib 测试 6 个报 relation "auth_role" does not exist，根因 DATABASE_URL 基础库无迁移表（与 R594 零关联）。建库跑一次性 shared::migrate::run_migrations 初始化后删除该临时测试，重跑 EXIT0。
- V3 首轮 EXIT1 格式差异，cargo fmt -p general_assemble_control 后重跑 EXIT0；复核 git diff 仅 2 个目标文件。

## 三 verify（cd oa4rust/backend，字面命令）

- V1: DATABASE_URL=<占位> cargo test -p general_assemble_control --test r594_worktime_weekday_contract → EXIT0 2passed。
- V2: DATABASE_URL=<占位> cargo test -p general_assemble_control → EXIT0（15+2+1+0 passed; 35 ignored）。
- V3: cargo fmt --check -p general_assemble_control → EXIT0（空输出）。

## 证据 sha256（runner 紧凑 JSON 与文件字节分列）

- runnerSha（sha256(JSON.stringify(meta))）= 79a01f3329ba38826ced95eb7dec98bb07a0cf3780f49f615da7bc45cd717f52。
- lib.rs（修复版文件字节）= 7df51518c4c8007813417ad6607581699d82fb32eccaacf453b547949aa17254。
- 契约测试文件字节 = 1651ca4b36190a5d2b85185035c126e561b729386a91eae2b460b4fdf6e03f6d。
- 真父备份 lib-parent.bak.rs = f28941dbbc52170f00072b016a81179ffa6f4f97dbe17a6ddc1fcf7be0e36679；修复版备份 lib-fixed.bak.rs = 2222087f49bb8e96aaab7647c40c626620ccc58f0ea319c5922879feb2851965。
- RED 日志 = c86c0cc81ab91e073889e66b5646de987e2b72f3b24be585cedf1b0c9cb6bd36；GREEN 日志 = 10a40cbfdf9552e9e95ca5022b8c5a8a423f29ebffb88180d1633487d53a0aed。
- V1 日志 = 6a0d2459c973728364a114595665c9a05e816f4c2160d9469b4bbfbe5f12e897；V2 日志 = a1b28974ba23494ce57208484fc188409b51cffd5137c89b29f7e32433f263ca；V3 日志 = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855。
- 本文档不含自身 sha，避免自引用悖论。
