# R595 forwarddays days 参数 TEXT 字典序上界失效修复(t149)

## 缺陷
- 真父 HEAD=b0bef0e2b7d9b1bde6b50c3ab6b60718cee7b0aa(含 R594 台账),分支 opt1000/oa4rust-pc-ai-refactor。
- worktime_forwarddays_start_start_days_days 原用 format!("{} + {} days", start, days) 作 TEXT 列 date 上界。
- '+' (0x2B) 小于数字字符,上界字典序小于任何 YYYY-MM-DD,导致 date <= 上界只命中 start 当天行,R593 真测 rows_in_range=3/returned=1。

## 修复(方案 B,写域精确 3 文件)
- 新增纯整数 helper worktime_forwarddays_end_date(start, days):解析 start(YYYY-MM-DD),经 Howard Hinnant days_from_civil/civil_from_days 加 days 后格式化回 YYYY-MM-DD 上界。
- days=0 上界即 start 当天;非法 start(格式不符/数字解析失败)降级原样返回,与 legacy 不 panic 行为一致。
- 无新依赖,Cargo.toml 零改;SQL / 其他端点 / routes 零改。
- 硬守卫:R594 Sakamoto helper worktime_fallback_weekday 定义恰 1、四端点调用恰 4、total_days=0、周末 w>=5 语义 4、indefined 语义 4 全部零回归。

## 契约 RED -> GREEN
- 契约 tests/r595_forwarddays_days_contract.rs(合法 Rust 标识符文件名禁点号),3 用例:主上界(01-01 起 days=30 应返 3 条,旧返 1 条)+ days=0 边界与 R594 守卫 + 跨月(01-20+15=02-04 返 2 条)。
- oneshot 经 general_assemble_control_router 对临时 scratch 库造数;每例独立 scratch 库,跑完 DROP。
- 真父源码覆盖 + touch -> RED EXIT101:2 failed(主上界 line119 返 1 条非 3 条;跨月 line204 返 1 条非 2 条)、1 passed(守卫)。
- 恢复修复版 + touch -> GREEN EXIT0:3 passed,0 failed;HEAD 未变。RED 由 cp 真父备份实现,据实披露。
- 踩坑:ops_edit_file 保留原 mtime 致 cargo 判 Fresh 用旧 rlib,须 touch 强制重编译。

## 三 verify(全 EXIT0)
- V1 cargo test -p general_assemble_control --test r595_forwarddays_days_contract:3 passed。
- V2 cargo test -p general_assemble_control:15+2+3+1+0 passed,35 ignored。基础库须先跑迁移(一次性测试用完即删),否则既有 --lib 用例报 relation 不存在。
- V3 cargo fmt --check -p general_assemble_control:clean(首轮契约 2 处格式差异,精确修正后复跑 EXIT0)。

## 脱敏与 sha256 证据
- 环境:临时 PG 仅用占位 <TEST_HOST>:<PORT>;连接 trust 无口令真值;测试库/容器用完即删;他组容器未触碰。文档不含自身 sha,避免自引用悖论。

### runnerSha = sha256(JSON.stringify(meta)) 紧凑串
- caa4ac2de90edae44a8ffefdca9de31472733598ffa48eadbb5b5d6dfbfe2124

### 文件 sha256sum(逐项核对)
- lib.rs(修复版)= b0906c521afc1882ff68572ceb92d2e5f360c0e477370423744b3953f7647464
- r595_forwarddays_days_contract.rs= e75d7ddccdf6de1ccc8ea2ab1648023e4b668b57bb301a365035fcabff287f91
- lib-parent.bak.rs(真父,gitignored)= 7df51518c4c8007813417ad6607581699d82fb32eccaacf453b547949aa17254
- lib-fixed.bak.rs(修复版,gitignored)= b0906c521afc1882ff68572ceb92d2e5f360c0e477370423744b3953f7647464
- RED 日志= 61dc445fc528d73c0631c6d7831dc5f0af2d8aaedf4791476e29e57c59587f24
- GREEN 日志= 52c7ca69ddfe7da6d6b451c3e308f4fe92e1dca853c07d6e8cc99ef0b5e6f609
- V1 日志= c7fd2914a9750962bae2eae1c80a8bb4cb6d33e264cffcb6a457ff33f5a46fb6
- V2 日志= e9200ed880e68685c79233bc03dee98bc39fdf115e778d3c0fafcdc2c6ce99e5
- V3 日志(空)= e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## 结论
- 候选 B 上界字典序失效已闭合;R594 成果零回归;三 verify 全 0;精确 git add 三文件单提交已 push。