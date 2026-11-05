# 类型一致性守卫：迁移列类型 / 可空性 vs Rust 解码（O36-D）

> 结论：**能做**，覆盖可静态判定的子集，已接入 CI 门禁（`column-nullability-scan`）。
> 静态判不了的 6 类不再硬做，逐条写明兜底。

## 1. 它回答什么问题

`backend/scripts/scan_row_get_column_mismatch.py` 从 `backend/migrations/*.sql`
正向累积出每张表的最终列类型与可空性，再与 handler 里
`row.get::<_, T>("col")` / `Value::String(row.get("col"))` 的静态类型比对，输出两档：

| 判定 | 含义 | CI |
|---|---|---|
| `FAIL` | 类型不匹配，tokio-postgres `FromSql` 必然失败 → 请求 500 | 基线外新增即红 |
| `NULLRISK` | 列可空 + Rust 非 `Option` → **该列出现 NULL 行必 500** | 基线外新增即红 |
| `LOOSE` | 同族加宽（INTEGER 读 i64 等），按用户口径不修 | 登记 |
| `SILENT` | `try_get` + 非 Option：失败被吞，字段静默为空 | 登记 |
| `OK` | 匹配（含 `Option<T>` 对可空列） | — |

判定只对**能关联到最近 SELECT 文本**的取值点生效，避免把动态 SQL/辅助函数误报。

## 2. 可空性怎么累积（证据链）

- `CREATE TABLE`：列无 `NOT NULL`、无列级 `PRIMARY KEY`、非 `SERIAL` → 可空；
  同表后续 `CREATE TABLE IF NOT EXISTS` 是 no-op，**不覆盖最早定义**
  （032/034 都重声明过 `x_ai_file`）。
- `ALTER TABLE ADD COLUMN`：无 `NOT NULL` → 可空；带 `IF NOT EXISTS` 且列已存在
  → **整条 no-op**（041 的 `reply_count`、067 的 `locked` 都属此类，真实库仍可空）。
- `ALTER COLUMN ... SET/DROP NOT NULL`、`ALTER COLUMN ... TYPE`：按文件顺序覆盖。
- `ALTER TABLE ... RENAME TO`：011 的 `FILE_FILE → file_file` 等，建别名并复制列元数据。
- `*_rollback.sql` 不参与（044 的回滚会把 `is_public` 改回 TEXT，纳入会得出反结论）。

SQL 侧形态继承：
`col::text` / `to_char(col, fmt)` → `String`，可空性**继承源列**（NULL 仍返回 NULL）；
`COALESCE(col, ...)` → 非空；`COUNT(...)` → `i64` 非空；其他表达式别名 → 跳过。

## 3. 当前存量（基线，2026-11-05）

`docs/audits/artifacts/column_nullability_baseline.tsv`（182 条，已扣除 O36-C1–C3 的 39 条收敛）：

- `FAIL=14`、`NULLRISK=168`、`SILENT=4`、`LOOSE=2`。
- 守卫第一次跑就抓到 O36-C 之外的同族缺陷，例如：
  - `bbs/src/subject.rs` 4 处 `Option<String>` 解裸 TIMESTAMP（`bbs_subject_info.create_time`，007:31）——**每行必 500**，不是 NULL 才 500；
  - `cms_assemble_control` 3 处把 `TEXT[]` 当 `String` 读（`x_cms_surface_appdict.path_levels`）；
  - `console` / `message` / `ai` / `program_center` 若干 `TIMESTAMPTZ` 当 `String` 读。
  
  这些正是 O31/O32/O33/O34/O35 反复出现、靠行为测试才抓到的同一类问题。

## 4. 门禁与基线棘轮

```
python scripts/scan_row_get_column_mismatch.py              # TSV 明细（VERDICT/文件/行/列/Rust/表/SQL）
python scripts/scan_row_get_column_mismatch.py --summary    # 按 crate 汇总
python scripts/scan_row_get_column_mismatch.py --update-baseline
python scripts/scan_row_get_column_mismatch.py --self-test  # 11 条注入用例，CI 每跑必过
```

- 基线键：`verdict\tfile\ttable\tcolumn`（不含行号，避免一改代码就整表漂移）。
- **新增条目 → 退出码 1**；已修条目 → 只打印「收敛」提示，**不阻断**
  （否则并发修复 PR 与基线更新会互相锁死）。
- CI：`.github/workflows/oa4rust-ci.yml` 的 `column-nullability-scan`
  （self-test + `--summary`），与 `fi002-column-scan` 同级成本。

## 5. 做不了的部分（逐条 + 兜底）

| 静态判不了的情形 | 为什么 | 兜底 |
|---|---|---|
| `SELECT *` | 列清单不在 Rust 代码里，运行期才由 DB 决定 | `fi002_column_scan.py` 扫 `SELECT *` 形状；`idor_matrix` 种子扫描对真实行逐字段解码；接口行为测试 |
| `SELECT {var}` / `format!` 插值列 | 列名运行期拼装，静态拿不到 | FI-002 同源提取器把插值 SQL 归入 `Unknown` 桶记账；行为测试覆盖实际模板 |
| CTE / 视图 / 系统表动态列（`filter_attr_options`、`u2_s` 等） | `FROM` 不是基表，迁移里没有对应列定义 | `docs/solutions/` 中 u2/过滤器的行为测试；`idor_matrix` 真实查询解码 |
| 一条 SQL 多表同名列歧义 | 本脚本只取最近一个 `FROM`，join 后会指错表 | 行为测试（种子行两种取值分别断言）；复杂性高的 join 建议改为显式别名/`::text` |
| `try_get` 吞错 | 失败不 panic，只是字段静默为空 | 归 `SILENT` 登记，不列入「NULL 行必 500」；前端联调/契约测试兜底字段非空 |
| SeaORM 实体 | 实体字段类型在 `#[derive]` 宏里，不在 `row.get` 调用点 | 既有 `scripts/scan_entity_column_mismatch.py`（实体 vs 迁移列）+ 实体读写行为测试 |

另外两类**已知不完备**（非「做不了」，是精度上限）：

- 可空性以迁移脚本为准，若线上库被手工 `ALTER` 过，脚本不知道 → 以迁移为单一事实源，
  真的对不上时先补迁移再改代码。
- 同一列在不同 handler 里用不同别名（`AS "createTime"`）时，别名必须与 SELECT 里
  的字面量一致才能命中；不一致就落到「未关联」而不误报。

## 6. 后续建议（不在本轮实施）

- 把基线从 221 条往下降时，优先处理 `FAIL`（每行必 500，比 NULLRISK 更严重）：
  `bbs/src/subject.rs` 4 处、`cms_assemble_control` 的 `TEXT[]` 3 处。
- 新增 handler 时先跑一遍脚本；若 `NULLRISK` 是「业务上该列为空」，解码改 `Option<T>`
  并在响应里给空串默认值，与 O36-C 的修法一致。
