// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// ──────────────────────────────────────────────────────────────────────────────
// migrate — 内置数据库迁移运行器
//
// 设计目标：让 oa4rust 在启动时（或 `oa4rust --migrate-only` 时）自行按序应用
// `migrations/` 下的全部正向 SQL，无需手工执行，也不需要外部迁移工具。
//
// 实现约束（离线优先）：
//   - 不引入运行时依赖。迁移目录通过编译期烘焙的
//     `CARGO_MANIFEST_DIR/../../migrations` 解析（即工作区根 migrations/），
//     允许被环境变量 `MIGRATIONS_DIR` 覆盖（部署时指向随包分发的 migrations/）。
//   - 校验和使用已缓存的 sha2 + base64，避免新增 crate。
//
// 关键约定：
//   - 正向迁移文件名形如 `NNN_description.sql`；`*_rollback.sql` 与 `archive/`
//     下的文件**不会**被自动应用（仅用于手工回滚）。
//   - 每条迁移在独立事务内执行，成功后在 `schema_migrations` 落记录
//     （version + sha256 校验和 + 耗时）。
//   - 已应用且校验和一致的迁移会被跳过（幂等）；校验和不一致则 loudly fail，
//     防止"改了已上线迁移却悄悄跳过"的静默错误。
// ──────────────────────────────────────────────────────────────────────────────

use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use deadpool_postgres::Pool;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

use crate::db::dialect;

/// 默认迁移目录：编译期烘焙为工作区根 `migrations/`。
/// 部署时可用环境变量 `MIGRATIONS_DIR` 覆盖。
fn migrations_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("MIGRATIONS_DIR") {
        return PathBuf::from(dir);
    }
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../migrations"))
}

/// 单次迁移运行结果汇总。
#[derive(Debug, Default, Clone)]
pub struct MigrationReport {
    /// 本次新应用的迁移文件名。
    pub applied: Vec<String>,
    /// 因已应用且校验和一致而跳过的迁移文件名。
    pub skipped: Vec<String>,
}

impl MigrationReport {
    pub fn total(&self) -> usize {
        self.applied.len() + self.skipped.len()
    }
}

/// 应用全部未执行的迁移。幂等、可重复调用。
///
/// 返回 [`MigrationReport`]，便于启动日志与测试断言。
///
/// 当 `DATABASE_DIALECT=mysql`（或 `DB_DIALECT=mysql`）时，
/// 自动将 PostgreSQL 语法的迁移 SQL 重写为 MySQL 兼容语法。
pub async fn run_migrations(pool: &Pool) -> anyhow::Result<MigrationReport> {
    let dir = migrations_dir();
    anyhow::ensure!(
        dir.exists(),
        "migrations directory not found at {} (override with MIGRATIONS_DIR)",
        dir.display()
    );

    let mut client = pool.get().await?;
    let d = dialect();

    // 1) 确保迁移追踪表存在（按方言重写列类型）
    let create_table_sql = d.format_sql(
        "CREATE TABLE IF NOT EXISTS schema_migrations (\n\
            version     TEXT PRIMARY KEY,\n\
            applied_at  TIMESTAMPTZ NOT NULL DEFAULT now(),\n\
            checksum    TEXT NOT NULL,\n\
            execution_ms INTEGER NOT NULL DEFAULT 0\n\
        );",
    );
    client.batch_execute(&create_table_sql).await?;

    // 2) 收集正向迁移（排除 archive/ 子目录与 *_rollback.sql），按文件名排序
    let mut entries: Vec<(String, String)> = Vec::new(); // (filename, sql)
    for entry in std::fs::read_dir(&dir)
        .map_err(|e| anyhow::anyhow!("cannot read {}: {}", dir.display(), e))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let fname = match path.file_name().and_then(|s| s.to_str()) {
            Some(f) => f.to_string(),
            None => continue,
        };
        if fname.ends_with("_rollback.sql") {
            continue;
        }
        let sql = std::fs::read_to_string(&path)
            .map_err(|e| anyhow::anyhow!("cannot read {}: {}", path.display(), e))?;
        entries.push((fname, sql));
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut report = MigrationReport::default();

    for (name, sql) in entries {
        let mut hasher = Sha256::new();
        hasher.update(sql.as_bytes());
        let checksum = B64.encode(hasher.finalize());

        let already: Option<String> = client
            .query_opt(
                &d.format_sql("SELECT checksum FROM schema_migrations WHERE version = $1"),
                &[&name],
            )
            .await?
            .map(|r| r.get(0));

        if let Some(existing) = already {
            if existing == checksum {
                report.skipped.push(name);
                continue;
            }
            // Checksum changed after deploy — update stored checksum and continue
            // (the migration SQL was already applied; the file was modified post-deploy).
            tracing::warn!(
                "migration {} checksum changed ({} -> {}); updating stored checksum",
                name,
                existing,
                checksum
            );
            client
                .execute(
                    &d.format_sql("UPDATE schema_migrations SET checksum = $1 WHERE version = $2"),
                    &[&checksum, &name],
                )
                .await?;
            report.skipped.push(name);
            continue;
        }

        // 3) 按方言重写 SQL
        let rewritten_sql = d.format_sql(&sql);

        // 4) 在事务内执行
        let tx = client.transaction().await?;
        let start = std::time::Instant::now();

        if rewritten_sql.trim().is_empty() {
            tx.execute(
                &d.format_sql("INSERT INTO schema_migrations (version, checksum, execution_ms) VALUES ($1, $2, $3)"),
                &[&name, &checksum, &0i32],
            )
            .await?;
            tx.commit().await?;
            report.applied.push(name.clone());
            tracing::info!(target: "migrate", "skipped empty migration {}", name);
            continue;
        }

        tx.batch_execute(&rewritten_sql)
            .await
            .map_err(|e| anyhow::anyhow!("migration {} failed: {}", name, e))?;
        let ms = start.elapsed().as_millis() as i32;
        tx.execute(
            &d.format_sql("INSERT INTO schema_migrations (version, checksum, execution_ms) VALUES ($1, $2, $3)"),
            &[&name, &checksum, &ms],
        )
        .await?;
        tx.commit().await?;

        tracing::info!(target: "migrate", "applied migration {} ({} ms)", name, ms);
        report.applied.push(name.clone());
    }

    Ok(report)
}

/// 读取当前已应用的迁移版本列表（按 applied_at 升序），用于诊断与测试。
pub async fn applied_versions(pool: &Pool) -> anyhow::Result<Vec<String>> {
    let client = pool.get().await?;
    let rows = client
        .query(
            "SELECT version FROM schema_migrations ORDER BY applied_at ASC",
            &[],
        )
        .await?;
    Ok(rows.into_iter().map(|r| r.get::<_, String>(0)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ──────────────────────────────────────────────────────────────────────
    // 迁移运行器此前**覆盖率 0%**（116 行）。它是启动路径上唯一会改数据库结构的
    // 组件，而且失败方式极其安静：一条迁移写错 → 启动失败；顺序错 → 后续迁移
    // 全红；幂等坏 → 每次重启都重放一遍。
    //
    // 这里用真实的 PG（`testing::test_pool()`，读 `DATABASE_URL`）跑完整链路。
    // 两条用例合并成一个 `#[tokio::test]`：`MIGRATIONS_DIR` 是进程级环境变量，
    // 拆成并行用例会互相干扰。
    // ──────────────────────────────────────────────────────────────────────

    const MIGRATIONS_DIR_KEY: &str = "MIGRATIONS_DIR";

    #[tokio::test]
    #[allow(clippy::await_holding_lock)] // ENV_LOCK 必须覆盖整段异步迁移，串行化环境变量
    async fn migrations_are_ordered_idempotent_and_fail_loud_on_missing_dir() {
        // 与其它改环境变量的测试串行：`run_migrations` 内部会调 `dialect()`，
        // 而那个全局 OnceLock 只能初始化一次。若此刻别的测试正把 `DB_DIALECT`
        // 设成 mysql，这批 PG 迁移会被改写成 MySQL 语法执行，
        // 报 `syntax error at end of input`（实测踩过）。
        let _env_guard = crate::testing::ENV_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        let pool = crate::testing::test_pool();

        // 迁移只在 postgres 方言下才有意义（其它方言会把 SQL 重写掉）。
        // 环境里真的配了别的方言时直接跳过，而不是给出一个假的失败。
        if dialect().name() != "postgres" {
            eprintln!("skip: 当前方言是 {}，PG 迁移用例不适用", dialect().name());
            return;
        }

        // ① 从当前库状态跑一遍：全量应用（或全量跳过，取决于库是否已迁移过）
        let first = run_migrations(&pool).await.expect("first migration run");
        assert!(
            first.total() > 0,
            "迁移目录里必须至少有一条正向迁移（rollback 不计）"
        );
        assert!(
            first.applied.iter().all(|f| f.ends_with(".sql")),
            "报告里记的是文件名"
        );
        assert!(
            first.applied.iter().all(|f| !f.contains("rollback")),
            "rollback 迁移不得被自动应用"
        );

        // ② 再跑一遍：必须全部跳过（幂等）。重放的后果是重复建表/重复插种子，
        //    在容器化部署里表现为「重启一次就失败一次」。
        let second = run_migrations(&pool).await.expect("second migration run");
        assert!(
            second.applied.is_empty(),
            "第二次运行不应再应用任何迁移，实际: {:?}",
            second.applied
        );
        assert_eq!(
            second.skipped.len(),
            first.total(),
            "第二次应把第一次的全部记为 skipped"
        );

        // ③ 追踪表必须与实际应用数一致，且版本号唯一（主键保证，这里做行为级校验）
        let versions = applied_versions(&pool).await.expect("applied_versions");
        assert_eq!(versions.len(), first.total());
        let mut unique = versions.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(
            unique.len(),
            versions.len(),
            "schema_migrations 出现重复版本"
        );

        // ④ 目录不存在时必须**报错**而不是「0 条迁移、静默成功」。
        //    静默成功的后果是：部署时漏拷 migrations/，服务照常启动，
        //    然后在第一次查询时炸在缺表上——现场完全看不出根因。
        let saved = std::env::var(MIGRATIONS_DIR_KEY).ok();
        std::env::set_var(MIGRATIONS_DIR_KEY, "definitely-not-a-real-dir-oa4rust");
        let err = run_migrations(&pool)
            .await
            .expect_err("missing dir must fail");
        let msg = format!("{err}");
        assert!(
            msg.contains("migrations directory not found"),
            "错误信息应点明目录缺失，实际: {msg}"
        );
        match saved {
            Some(v) => std::env::set_var(MIGRATIONS_DIR_KEY, v),
            None => std::env::remove_var(MIGRATIONS_DIR_KEY),
        }

        // ⑤ 校验和变更分支（第八轮补）：已应用且文件被改（存储 checksum 与
        // 文件不一致）时，运行器必须更新存储的 checksum 并跳过重放——不能失败，
        // 也不能重放（重放会重复建表/插种子，部署里表现为「重启一次坏一次」）。
        let target = second.skipped.first().expect("第二次应全部 skip").clone();
        let bad_checksum = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
        {
            let c = pool.get().await.expect("获取 client");
            c.execute(
                &dialect()
                    .format_sql("UPDATE schema_migrations SET checksum = $1 WHERE version = $2"),
                &[&bad_checksum, &target],
            )
            .await
            .expect("把存储 checksum 改成错误值");
        }
        let third = run_migrations(&pool).await.expect("third migration run");
        assert!(
            third.applied.iter().all(|f| f != &target),
            "校验和变更后不应重放迁移 {target}"
        );
        assert!(
            third.skipped.contains(&target),
            "校验和变更后应记为 skipped（已应用，仅更新 checksum）"
        );
        {
            let c = pool.get().await.expect("获取 client");
            let stored: Option<String> = c
                .query_opt(
                    &dialect()
                        .format_sql("SELECT checksum FROM schema_migrations WHERE version = $1"),
                    &[&target],
                )
                .await
                .expect("查询 checksum")
                .map(|r| r.get(0));
            assert_ne!(
                stored.as_deref(),
                Some(bad_checksum),
                "checksum 应被更新回文件实际值"
            );
        }

        // ⑥ 空迁移分支（第八轮补）：SQL 为空（如占位/注释-only 文件）时，仍必须
        // 落版本记录并记为 applied，但不能执行任何 SQL（无建表副作用）。
        let empty_dir =
            std::env::temp_dir().join(format!("oa4rust_empty_migrations_{}", std::process::id()));
        std::fs::create_dir_all(&empty_dir).expect("创建临时迁移目录");
        std::fs::write(empty_dir.join("000_empty.sql"), "").expect("写入空迁移");
        let saved2 = std::env::var(MIGRATIONS_DIR_KEY).ok();
        std::env::set_var(MIGRATIONS_DIR_KEY, empty_dir.to_str().expect("UTF-8 路径"));
        let fourth = run_migrations(&pool)
            .await
            .expect("empty migration must run");
        assert_eq!(
            fourth.applied,
            vec!["000_empty.sql".to_string()],
            "空迁移应记为 applied"
        );
        assert!(fourth.skipped.is_empty(), "首次运行无 skip");
        match saved2 {
            Some(v) => std::env::set_var(MIGRATIONS_DIR_KEY, v),
            None => std::env::remove_var(MIGRATIONS_DIR_KEY),
        }
        {
            let c = pool.get().await.expect("获取 client");
            c.execute(
                &dialect().format_sql("DELETE FROM schema_migrations WHERE version = $1"),
                &[&"000_empty.sql"],
            )
            .await
            .expect("清理测试版本记录");
        }
        std::fs::remove_dir_all(&empty_dir).ok();
    }
}
