# oa4rust FAQ（骨架）

本文件只覆盖**已有台账可指回**的三个高频问题；全量 FAQ 是独立轮次（oa4rust 是 96 个 crate 的
系统，宁少勿错）。每条事实句后面都能顺着 §号或代码锚点回查，指不回的不写。

约定：`§x.y` 指 docs/plans 下对应计划文档的小节；`文件:行号` 指当前代码落点
（行号会搬家，改代码时一并改引用）。

---

## 1. 为什么刚建好的库会一片 500？现在靠什么防？

**根因不是「没数据」，而是 handler 的 SQL 引用了 migrations 里压根不存在的列 / 表 / 视图**
（PG 侧报 `undefined_column` / `undefined_table`，HTTP 层就是 500）。2026-10-26 的只读勘察
点出 F-1~F-4，随后的 CI PG 日志又暴露 F-5~F-8 与 FX-1（10-26 fresh-db-500 计划 §2.2、§2.1 末）；
这些缺陷此前全部被既有测试漏过——因为当时的测试要么不连库，要么**把 500 当成期望值钉住了**（F-4）。

现在有四道防线：

1. **静态列扫描棘轮**（oa4rust/backend/scripts/fi002_column_scan.py）：解析 migrations 得到
   `(table, column)` 宇宙，再扫 `crates/*/src` 与 `src/` 里 Rust 字符串字面量的表 / 列引用，
   **新增**未知对即退出 1。基线是 `docs/audits/artifacts/fi002_unknown_columns.tsv`，
   只允许人工逐条复核后 `--update-baseline` 收敛——棘轮不许被自动重写。
   2026-10-26 的 P1-1 补了 WHERE / FILTER / ORDER BY / GROUP BY 裸列抽取，
   F-1（`status`）、F-2（`category`）、F-5（`sort_order`）这类原本从盲区溜过去的缺陷才被看见
   （10-26 计划 §2.3、§3 P1-1）。
2. **迁移一次补齐**：migration 112 按 111 的范式（幂等 + `TO_REGCLASS` 守卫 + 出处行号注释 +
   rollback）把缺失的列 / 视图 / 四张遗留表补上；112 的 rollback 与重放都实测过
   （10-26 计划 §3 P0、§4）。
3. **测试改成行为断言**：F-4 那条「钉 500」的假测试改成 `is_db_available()` 门 +
   「200 且四键齐全」的行为断言，同时保留一条无库也能跑的 router 构建测试
   （10-26 计划 §3 P0-5）。判据从「错误长这样」变成「正确能走通」。
4. **真库回归**：SQL 改动的验证不看编译过没过，而在真实 PG 上直跑（本仓用 Docker 起
   postgres，跑 001→112 全量迁移后逐条验；2026-10-26 的 V 批次就是这么做的，
   含负对照——把视图 DROP 掉确认查询真的会报 `relation does not exist`）。

> **别和挂载守卫混为一谈**：`oa4rust/backend/tests/mount_coverage_guard.rs` 防的是另一种
> 「测试全绿但生产不可用」——crate 的 `router()` 已实现却没 merge 进 `create_app`，生产全 404
> （2026-10-24 一次抓到 4 个 crate）。它守的是**挂载**，不是列缺失的 500；
> 挂载点在 `oa4rust/backend/src/lib.rs:96` 起那一段。

---

## 2. 那 73 条「契约路径别名」是干什么的？中间件四表镜像又是什么不变式？

**背景**：前端按 Java 时代的契约基路径调用（`/api/organization/assemble/personal/*`），
而 Rust 侧 personal / empower 等 crate 的 handler 早就实现了，只是挂在 `/api/person/*`。
routing-completion-plan 里「NEEDS 73 ROUTES」的旧结论容易被读成「73 条没实现」，
实际缺的只是**契约基路径的接线**（10-24 综合优化计划 §14 R1）。

- **落地**：三个 crate 的 router 里新增「契约路径别名」段，共 **73 条**（personal 57 /
  empower 14 / personal_extend 2），原样委托同一 handler；旧路径行为零变化。
- **3 条有意不映射**：两条「口令进 URL」的 GET（Rust 侧已改成 POST body，别名回 GET 是安全
  回退）、以及 `custom/{name}/mockputtopost`（与既有路由同形，双注册会让 axum panic，
  该契约路径已由既有路由服务）。
- **不变式（四表镜像）**：别名「能用」不能以「少一道闸」为代价，所以别名与主路径必须在四张
  表里逐项相等——`constants.rs` 的豁免清单、限流表、`ADMIN_WRITE_PREFIXES`、`rbac.rs`
  的权限注册表。任何一项只加一边，就等于给别名开了一条绕过鉴权 / 限流 / admin 门禁的侧门。
  尤其 `rbac.rs` 的**兜底条不能省**：`check_permission` 对注册表未命中的路径会早退
  Authenticated 且不再 consult `requires_admin`，少了它，别名下的写操作会绕过 admin 门禁
  （10-24 计划 §14 R1 的「中间件 parity」段）。
- **守卫两道，表同源**：`shared` crate 的单测 `contract_alias_parity_tests` 逐对比较
  豁免 / 限流 / Public / **有效 admin**（复刻真实控制流，不是裸比 `requires_admin`），
  外加「豁免 ⟹ Public」的单向不变式（防 401 死态）；集成测试
  `oa4rust/tests/contract_alias_parity.rs` 用全栈 oneshot 实测 73 对状态码逐对相等，
  并断言别名非 404。

---

## 3. 本机没有 Rust 编译器，改了 Rust 代码怎么验证？

**先承认事实**：本机当前的 GNU 工具链缺 gcc / dlltool（`ring`、`parking_lot_core` 这类带 C
依赖的 crate 建不了），也没有 MSVC 工具链，所以 `cargo build` / `cargo test` 在本机**跑不了**。
10-11/12 能编是因为当时装了 MinGW，现在不在（10-26 计划 §2.4）。

**口径（不是「大概看一下」）**：

| 本机能做 | 交给 CI |
|---|---|
| `cargo fmt`（CI 同版本 1.98.1，能查语法 + 格式） | `cargo check` / `cargo clippy` / `cargo test` 全量 |
| 纯算法用 Python 端口复跑（对照同一组输入输出） | 真库（fresh PG + 001→112 迁移）上的集成 / 行为测试 |
| 人工类型与 borrow 复审 | 契约门禁（openapi / 路由 fixture / FI-002 棘轮） |

**代价要如实记**：2026-10-26 那轮就发生了两轮 CI 反馈循环——① 3 处 `&c` 撞
`needless_borrow`（形参本就是 `&Client`）；② 把 `Path::file_name()` 的 `Option<&OsStr>`
当成 `DirEntry::file_name()` 的 owned 返回值用（E0599）。每轮约 10 分钟 CI，终态全绿
（10-26 计划 §2.4 末行）。

**因此**：不要写「本机验证通过」这种话——要么写清「本机做了 rustfmt + 算法实跑，编译与测试
待 CI」，要么等 CI 结果再下结论。改了 SQL 的，另加一栏「真库直跑结果」，那才是行为证据。
