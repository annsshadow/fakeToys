# oa4rust 1000 轮实质优化 · Track-B 台账

> 本文件是 oa4rust 子项目「千轮实质优化」Track-B 线的唯一进度台账。
> Track-A 线（laiyipao + oa4rust 主线）台账在 `docs/plans/optimization-1000-oa4rust-laiyipao.md`（另一会话所有，本线不写）。
>
> 隔离约定：本线在独立 worktree `D:\WORKSPACE\fakeToys-oa4rust-oa4rust`（分支 `opt1000/oa4rust`）工作，
> 只改 `oa4rust/` 下内容；构建产物落 `CARGO_TARGET_DIR=E:\cargo-target\oa4rust-par`（仅命令环境变量，不入库）；
> 临时库一律 `oa_par_` 前缀，用完 DROP。
>
> 轮次编号 `O1、O2…` 与 Track-A 的 `R` 编号区分。
> Track-A 已完成的 oa4rust 轮次（不重做）：R4 优雅停机、R5 全局超时、R6 池超时 503、R7 结构化访问日志、
> R11 panic 收敛、R12 池取连失败 503、R16 /ready 真查库、R17 迁移错误可定位、R24 body 上限、
> R25 DATABASE_URL 配置错误可定位、R26 statement_timeout、R31 /mcp 请求层防护。

## 轮次标准

- 一轮 = 实质性功能或性能优化；编译/lint/纯重构/纯测试/纯文档均不算轮。
- 每轮先探针证明真实后果（改前证据），修复后证明「改后好 + 原功能没坏」（三段证据）。
- 探针打不出真实后果 → 判定「不开轮」，记录结论换题。

## 逐轮台账

| 轮次 | 主题 | 状态 | 缺口/改法/证据摘要 | commit |
|------|------|------|--------------------|--------|
| O1 | 迁移并发安全：pg advisory lock 串行化多实例迁移 | done | 缺口：多实例并发启动跑迁移无串行化；改前探针实测两个运行者并发跑同一空库，`CREATE TABLE IF NOT EXISTS schema_migrations` 撞 `pg_type_typname_nsp_index`（SQLSTATE 23505），后到实例启动失败。改法：`run_migrations` 前取会话级 advisory lock（固定 key），逐版本重查自然跳过，成功/失败路径都显式 unlock（防连接回池残留锁），断连由 PG 自动释放。证据：改前探针 runner A Err(23505)/B Ok；改后双方 Ok、3 条迁移恰好应用一次、种子恰好一行；回归 shared 363/363（`oa4rust_test` 库）。scratch `oa_par_mig_o1` 已 DROP | 068371db6 |
| O2 | 取连失败语义统一：`shared::db::acquire` 全仓收口（R12 分类覆盖全部请求路径） | done | 缺口：R12 的池取连失败 503 分类只挂在 `ControlPool` trait 上，全仓约 2930 处 handler 仍手写 `pool.get().await.map_err(|_| AppError::Internal)` —— DB 停机/池满/重启窗口一律 500，客户端不敢重试、监控误报代码缺陷（改前证据：ai_assemble_control 等 crate 死池测试断言 500，实测连接被拒返回 `internal server error`）。改法：`shared::db::acquire(&Pool)` 公开入口复用 `classify_acquire_error`（Timeout/Backend/Closed→503，配置类→500），全仓机械替换 2930 处 + 116 文件。证据：死池请求由 500 → 503（`service unavailable: ...`）；`Extension<Session>` 缺失的 500 语义不变。测试同步：~250 处死池断言由 500 翻 503；混合路由循环（同表内既有必需会话扩展的 500 又有池失败的 503）改为「500|503 均证明路由可达」家族断言（保持≠404 的可达性语义）；`crates/shared/crud.rs` fail-loud 契约改锚 Unavailable。验证：全 workspace `cargo test --no-fail-fast` 除 4 个 main 上既有红守卫（quoted_key_guard/quoted_relation_guard/test_module_registration_guard/arity_probe，已在 origin/main 原态复现确认与本改动无关）外全绿；`cargo clippy --workspace --all-targets --keep-going` 零告警 | 待提交 |

## 不开轮记录（探针全绿 / 无真实后果）

| 日期 | 探针主题 | 结论 |
|------|----------|------|
| - | - | - |

## 候选方向池（按价值排序，执行时取舍）

1. 迁移并发安全：多实例同时启动跑迁移无串行化（Track-A 在 laiyipao 已踩过同类问题）。
2. sea-orm/sqlx 错误分类：`sqlx::Error` 全部 → 500，PoolTimedOut/Io 断连应与 R6 同口径 → 503。
3. 限流器 key 空间洪泛（唯一 key 1h 窗口内无总量上界）。
4. IDOR / 越权：资源归属校验抽查→穷举。
5. 管理端路由鉴权穷举扫描。
6. 输入健壮性：超长字段/深嵌套/畸形 JSON 的上界缺失。
7. 性能基线：真实数据量下关键路径 P95/P99。
8. 数据完整性：业务写与审计写同事务、孤儿行、级联语义。
