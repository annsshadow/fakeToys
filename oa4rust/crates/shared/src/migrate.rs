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
pub async fn run_migrations(pool: &Pool) -> anyhow::Result<MigrationReport> {
    let dir = migrations_dir();
    anyhow::ensure!(
        dir.exists(),
        "migrations directory not found at {} (override with MIGRATIONS_DIR)",
        dir.display()
    );

    let mut client = pool.get().await?;

    // 1) 确保迁移追踪表存在
    client
        .batch_execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version     TEXT PRIMARY KEY,
                applied_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
                checksum    TEXT NOT NULL,
                execution_ms INTEGER NOT NULL DEFAULT 0
            );",
        )
        .await?;

    // 2) 收集正向迁移（排除 archive/ 子目录与 *_rollback.sql），按文件名排序
    let mut entries: Vec<(String, String)> = Vec::new(); // (filename, sql)
    for entry in std::fs::read_dir(&dir)
        .map_err(|e| anyhow::anyhow!("cannot read {}: {}", dir.display(), e))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue; // archive/ 等子目录整体跳过
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
                "SELECT checksum FROM schema_migrations WHERE version = $1",
                &[&name],
            )
            .await?
            .map(|r| r.get(0));

        if let Some(existing) = already {
            if existing == checksum {
                report.skipped.push(name);
                continue;
            }
            anyhow::bail!(
                "migration {} already applied with a different checksum (changed after deploy?)",
                name
            );
        }

        // 4) 在事务内执行
        let tx = client.transaction().await?;
        let start = std::time::Instant::now();
        tx.batch_execute(&sql)
            .await
            .map_err(|e| anyhow::anyhow!("migration {} failed: {}", name, e))?;
        let ms = start.elapsed().as_millis() as i32;
        tx.execute(
            "INSERT INTO schema_migrations (version, checksum, execution_ms) VALUES ($1, $2, $3)",
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
