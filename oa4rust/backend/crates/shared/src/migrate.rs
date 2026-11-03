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

/// 把 `tokio_postgres` 的错误展开成可定位的文本。
///
/// ⚠️ 直接 `{}` 打印 `tokio_postgres::Error` 只会得到 **"db error"** ——
/// 真正的原因（SQLSTATE、服务端 message、DETAIL/HINT）在 `as_db_error()` 里。
/// 迁移运行器是启动路径上唯一会改库结构的组件，它失败时运维最需要的就是
/// 「哪条 SQLSTATE、哪个列/表」；只剩 "db error" 的话，排查只能靠人肉把
/// 迁移文件逐条贴进 psql 复现（实测踩过：019 因为库结构漂移缺列而失败，
/// 日志里只有 "db error"，看不出是 `column "person_unique" does not exist`）。
fn describe_pg_error(e: &deadpool_postgres::tokio_postgres::Error) -> String {
    if let Some(db) = e.as_db_error() {
        let mut s = format!("{} (SQLSTATE {})", db.message(), db.code().code());
        if let Some(detail) = db.detail() {
            s.push_str(&format!("; detail: {detail}"));
        }
        if let Some(hint) = db.hint() {
            s.push_str(&format!("; hint: {hint}"));
        }
        return s;
    }
    // 非服务端错误（IO 断开、协议错误）没有 SQLSTATE，退到 source 链。
    if let Some(src) = std::error::Error::source(e) {
        return format!("{src}");
    }
    e.to_string()
}

/// O1：多实例并发启动时的迁移串行化锁（PG 会话级 advisory lock 的 key）。
///
/// 改前真实后果（O1 探针实测，两个运行者并发跑同一空库）：
/// `CREATE TABLE IF NOT EXISTS schema_migrations` 并发执行撞 pg_type 唯一索引
/// （后到者 SQLSTATE 23505，`pg_type_typname_nsp_index` 冲突），该实例
/// `run_migrations` 返回 Err → 启动失败。滚动重启/多副本部署必踩。
///
/// 取值为 i64 常量（"oa_migra" 的 ASCII），advisory lock 作用域是单库，
/// 与业务侧其它 advisory lock 撞 key 的概率可忽略。
const MIGRATION_ADVISORY_LOCK_KEY: i64 = 0x6F61_5F6D_6967_7261;

/// 应用全部未执行的迁移。幂等、可重复调用。
///
/// 返回 [`MigrationReport`]，便于启动日志与测试断言。
///
/// 当 `DATABASE_DIALECT=mysql`（或 `DB_DIALECT=mysql`）时，
/// 自动将 PostgreSQL 语法的迁移 SQL 重写为 MySQL 兼容语法。
///
/// 并发语义：进入迁移前先取会话级 advisory lock（O1），后到实例阻塞等待
/// 持锁实例跑完全部迁移；拿到锁后逐版本重查 `schema_migrations`，已应用的
/// 自然跳过。锁在函数返回前显式释放（成功/失败路径都释放），防止归还连接
/// 到池后残留会话锁毒害后续使用者；若连接中断，PG 会在断连时自动释放。
/// 锁获取语句不经方言重写：驱动恒为 tokio-postgres，与 DB_DIALECT 无关。
pub async fn run_migrations(pool: &Pool) -> anyhow::Result<MigrationReport> {
    let mut client = pool.get().await?;

    client
        .query_one(
            "SELECT pg_advisory_lock($1)",
            &[&MIGRATION_ADVISORY_LOCK_KEY],
        )
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "acquiring migration advisory lock failed: {}",
                describe_pg_error(&e)
            )
        })?;

    let result = run_migrations_inner(&mut client).await;

    // 无论成败都显式释放锁；释放失败只在迁移本身成功时才上抛
    // （迁移已失败时保留根因，不用锁释放错误覆盖它）。
    let unlock = client
        .query_one(
            "SELECT pg_advisory_unlock($1)",
            &[&MIGRATION_ADVISORY_LOCK_KEY],
        )
        .await;
    match (result, unlock) {
        (Ok(report), Ok(_)) => Ok(report),
        (Err(e), _) => Err(e),
        (Ok(_), Err(e)) => Err(anyhow::anyhow!(
            "releasing migration advisory lock failed: {}",
            describe_pg_error(&e)
        )),
    }
}

async fn run_migrations_inner(
    client: &mut deadpool_postgres::Object,
) -> anyhow::Result<MigrationReport> {
    let dir = migrations_dir();
    anyhow::ensure!(
        dir.exists(),
        "migrations directory not found at {} (override with MIGRATIONS_DIR)",
        dir.display()
    );
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
            .map_err(|e| anyhow::anyhow!("migration {} failed: {}", name, describe_pg_error(&e)))?;
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

        // ⑦ read_dir 失败分支（第十六轮补）：MIGRATIONS_DIR 指向一个**文件**而
        // 非目录时，`std::fs::read_dir` 必须报错（"cannot read …"），不能把
        // 文件路径当成空目录静默返回 0 条迁移——部署漏拷 migrations/ 时若
        // 恰好有同名文件占位，静默成功会拖到启动后第一次查询才炸。
        let file_path = std::env::temp_dir().join(format!(
            "oa4rust_migrations_file_{}.sql",
            std::process::id()
        ));
        std::fs::write(&file_path, "not a directory").expect("写临时文件");
        let saved3 = std::env::var(MIGRATIONS_DIR_KEY).ok();
        std::env::set_var(MIGRATIONS_DIR_KEY, file_path.to_str().expect("UTF-8 路径"));
        let err5 = run_migrations(&pool)
            .await
            .expect_err("迁移目录是文件时必须报错");
        let msg5 = format!("{err5}");
        assert!(
            msg5.contains("cannot read"),
            "应点明无法读取迁移目录，实际: {msg5}"
        );
        match saved3 {
            Some(v) => std::env::set_var(MIGRATIONS_DIR_KEY, v),
            None => std::env::remove_var(MIGRATIONS_DIR_KEY),
        }
        std::fs::remove_file(&file_path).ok();

        // ⑧ 坏迁移 SQL 必须 loud 失败（第二十轮补）：迁移文件内容非法时，
        // 事务内执行必须报错并点名文件——不能静默跳过（否则坏迁移在部署里
        // 被当作已应用，启动照常、首次查询才炸）。
        //
        // ⚠️ R17：错误里必须带**真实的库错误**（SQLSTATE + message），
        // 不能只是 `tokio_postgres::Error` 的 Display 输出 "db error" ——
        // 后者让运维无法判断是语法错、缺列还是权限问题（实测库结构漂移时
        // 019 只报 "db error"，得手工把 SQL 贴进 psql 才看出缺 `person_unique`）。
        let bad_dir =
            std::env::temp_dir().join(format!("oa4rust_bad_migration_{}", std::process::id()));
        std::fs::create_dir_all(&bad_dir).expect("创建临时迁移目录");
        std::fs::write(bad_dir.join("001_bad.sql"), "THIS IS NOT SQL;").expect("写坏迁移");
        let saved5 = std::env::var(MIGRATIONS_DIR_KEY).ok();
        std::env::set_var(MIGRATIONS_DIR_KEY, bad_dir.to_str().expect("UTF-8 路径"));
        let err7 = run_migrations(&pool).await.expect_err("坏迁移必须失败");
        let msg7 = format!("{err7}");
        assert!(
            msg7.contains("001_bad.sql"),
            "错误信息应点名坏迁移文件，实际: {msg7}"
        );
        assert!(
            msg7.contains("42601"),
            "错误信息应带 SQLSTATE（语法错 42601），实际: {msg7}"
        );
        assert!(
            msg7.contains("syntax error"),
            "错误信息应带服务端 message，实际: {msg7}"
        );
        assert!(
            !msg7.contains("failed: db error"),
            "不得退化成不透明的 \"db error\"，实际: {msg7}"
        );
        match saved5 {
            Some(v) => std::env::set_var(MIGRATIONS_DIR_KEY, v),
            None => std::env::remove_var(MIGRATIONS_DIR_KEY),
        }
        std::fs::remove_dir_all(&bad_dir).ok();
    }

    // ──────────────────────────────────────────────────────────────────────
    // O1 探针/回归：多实例并发迁移必须串行。
    //
    // 场景：滚动重启/多副本部署时两个实例同时启动，都会执行 run_migrations。
    // DDL 与种子都写成幂等（IF NOT EXISTS / ON CONFLICT DO NOTHING），但每条
    // 迁移收尾的 `INSERT INTO schema_migrations` 不幂等：两个运行者看到同一条
    // 版本未应用，先后 INSERT 同一 version → 后到者主键冲突（SQLSTATE 23505），
    // 整个迁移事务回滚，该实例启动失败。
    // ──────────────────────────────────────────────────────────────────────

    /// 用 DATABASE_URL 的凭据连到指定库名（scratch 库专用）。
    fn pool_to_db(dbname: &str) -> Pool {
        let raw = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust".to_string());
        let url = url::Url::parse(&raw).expect("invalid DATABASE_URL");
        let mut cfg = deadpool_postgres::tokio_postgres::Config::new();
        cfg.host(url.host_str().unwrap_or("localhost"))
            .port(url.port().unwrap_or(5432))
            .user(url.username())
            .password(url.password().unwrap_or(""))
            .dbname(dbname);
        let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
        Pool::builder(mgr).max_size(2).build().unwrap()
    }

    /// 重建 scratch 库（存在则强删重建），返回库名。
    async fn fresh_scratch_db(dbname: &str) -> String {
        let admin = pool_to_db("postgres");
        let client = admin.get().await.expect("admin 连接");
        client
            .execute(
                &format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"),
                &[],
            )
            .await
            .expect("drop scratch db");
        client
            .execute(&format!("CREATE DATABASE {dbname}"), &[])
            .await
            .expect("create scratch db");
        dbname.to_string()
    }

    async fn drop_scratch_db(dbname: &str) {
        let admin = pool_to_db("postgres");
        if let Ok(client) = admin.get().await {
            client
                .execute(
                    &format!("DROP DATABASE IF EXISTS {dbname} WITH (FORCE)"),
                    &[],
                )
                .await
                .ok();
        }
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)] // MIGRATIONS_DIR 是进程级变量，须串行化
    async fn concurrent_migration_runs_are_serialized() {
        if !crate::testing::is_db_available().await {
            eprintln!("SKIP: no database available");
            return;
        }
        let _env_guard = crate::testing::ENV_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if dialect().name() != "postgres" {
            eprintln!("skip: 当前方言非 postgres");
            return;
        }

        let dbname = fresh_scratch_db("oa_par_mig_o1").await;

        // 临时迁移目录：pg_sleep 拉宽竞态窗口，保证两个运行者必然重叠。
        let dir = std::env::temp_dir().join(format!("oa_par_mig_o1_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("创建临时迁移目录");
        std::fs::write(dir.join("001_sleep.sql"), "SELECT pg_sleep(0.5);").unwrap();
        std::fs::write(
            dir.join("002_table.sql"),
            "CREATE TABLE o1_probe_t (id int);",
        )
        .unwrap();
        std::fs::write(
            dir.join("003_seed.sql"),
            "INSERT INTO o1_probe_t VALUES (1);",
        )
        .unwrap();

        let saved = std::env::var(MIGRATIONS_DIR_KEY).ok();
        std::env::set_var(MIGRATIONS_DIR_KEY, dir.to_str().expect("UTF-8 路径"));

        let pool_a = pool_to_db(&dbname);
        let pool_b = pool_to_db(&dbname);
        let (ra, rb) = tokio::join!(run_migrations(&pool_a), run_migrations(&pool_b));

        match saved {
            Some(v) => std::env::set_var(MIGRATIONS_DIR_KEY, v),
            None => std::env::remove_var(MIGRATIONS_DIR_KEY),
        }

        let ok_a = ra
            .as_ref()
            .map(|r| r.applied.len())
            .map_err(|e| e.to_string());
        let ok_b = rb
            .as_ref()
            .map(|r| r.applied.len())
            .map_err(|e| e.to_string());
        eprintln!("runner A: {ok_a:?}; runner B: {ok_b:?}");

        // 两个实例都必须启动成功；全部迁移恰好被应用一次。
        assert!(ra.is_ok(), "实例 A 迁移失败: {:?}", ra.err());
        assert!(rb.is_ok(), "实例 B 迁移失败: {:?}", rb.err());
        let total_applied = ra.unwrap().applied.len() + rb.unwrap().applied.len();
        assert_eq!(total_applied, 3, "3 条迁移必须恰好被应用一次（两实例合计）");

        // 库内状态自证：3 条版本记录、表存在且只有 1 行种子。
        let client = pool_a.get().await.unwrap();
        let versions: i64 = client
            .query_one("SELECT count(*) FROM schema_migrations", &[])
            .await
            .unwrap()
            .get(0);
        assert_eq!(versions, 3);
        let rows: i64 = client
            .query_one("SELECT count(*) FROM o1_probe_t", &[])
            .await
            .unwrap()
            .get(0);
        assert_eq!(rows, 1, "种子迁移必须恰好执行一次");

        std::fs::remove_dir_all(&dir).ok();
        drop_scratch_db(&dbname).await;
    }
}
