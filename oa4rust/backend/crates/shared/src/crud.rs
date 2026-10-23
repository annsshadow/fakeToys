// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 通用参数化 CRUD（针对已建表的家族家族补写端点用）。
//!
//! 安全模型：表名与列名只能取自调用方提供的静态白名单（`&'static str`），
//! 值全部走 `$N` 占位符防注入。表/列不接受任何运行时用户输入，杜绝 SQL 注入。
//! 各 crate 用本模块实现 create/save/delete 薄 handler，无需重复 per-table SQL。

use deadpool_postgres::Pool;
use serde_json::Value;

use crate::error::AppError;

/// 一张表的 CRUD 白名单规格。
#[derive(Debug, Clone, Copy)]
pub struct CrudSpec {
    /// 目标表名（静态常量）。
    pub table: &'static str,
    /// 允许读写的 (json_key, db_column) 映射；create 按 json_key 取 payload 补齐缺键，
    /// save 只写在 payload 里**出现**的键（缺键保持原值，见 `crud_save`）。
    pub columns: &'static [(&'static str, &'static str)],
    /// 是否有软删列 deleted_at（save/delete 追加 AND deleted_at IS NULL / 软删 UPDATE）。
    pub soft_delete: bool,
}

fn str_of(payload: &Value, json_key: &str) -> String {
    payload
        .get(json_key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

/// 参数化 create：INSERT (id, cols...) VALUES ($1, $2...$N)。返回新 id。
pub async fn crud_create(
    pool: &Pool,
    spec: &CrudSpec,
    payload: &Value,
) -> Result<String, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    use deadpool_postgres::tokio_postgres::types::ToSql;

    let id = uuid::Uuid::new_v4().to_string();
    let col_names: Vec<&str> = spec.columns.iter().map(|(_, db)| *db).collect();
    let values: Vec<String> = spec
        .columns
        .iter()
        .map(|(json, _)| str_of(payload, json))
        .collect();

    let mut placeholders: Vec<String> = vec!["$1".to_string()];
    // 占位符数 = 列数 + 1（$1 是 id）；旧实现 1..values.len() 少算末位，
    // INSERT 字段数多于表达式（42601）——所有 ≥1 列 create 全 500。
    for i in 1..=values.len() {
        placeholders.push(format!("${}", i + 1));
    }
    let sql = format!(
        "INSERT INTO {} (id, {}) VALUES ({})",
        spec.table,
        col_names.join(", "),
        placeholders.join(", ")
    );

    let mut params: Vec<String> = vec![id.clone()];
    params.extend(values);
    let refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();

    client
        .execute(&sql, &refs)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(id)
}

/// 参数化 save：只 UPDATE 载荷里**出现**的列（缺键的列保持原值），WHERE id = $N
/// （软删表追加 deleted_at IS NULL）。返回是否命中。
///
/// 缺键不写是判据不是巧合：原实现按 spec 全列写、缺键取 `""`，于是「局部保存」
/// 会把未提交的列（归属 person_id/creator、关联 app_id/portal_id…）抹成空串——
/// x_person_face 实测过归属被抹（face_save / collect_save）。要清空某列就显式给
/// `""`/null；要改归属只能走 create。
pub async fn crud_save(
    pool: &Pool,
    spec: &CrudSpec,
    id: &str,
    payload: &Value,
) -> Result<bool, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    use deadpool_postgres::tokio_postgres::types::ToSql;

    let mut set_parts: Vec<String> = Vec::new();
    let mut params: Vec<String> = Vec::new();
    for (json, db) in spec.columns {
        // 只认载荷里出现的键：`get` 对缺键与载荷不是对象都回 None，两者都不落笔。
        if let Some(value) = payload.get(*json) {
            set_parts.push(format!("{db}=${}", params.len() + 1));
            params.push(value.as_str().unwrap_or("").to_string());
        }
    }

    let deleted = if spec.soft_delete {
        " AND deleted_at IS NULL"
    } else {
        ""
    };

    if set_parts.is_empty() {
        // 载荷里没有可写列：不落笔，只回答「行是否存在」，保住 not-found 语义
        // （UPDATE 空 SET 是语法错，不能拿它当 no-op）。
        let sql = format!("SELECT 1 FROM {} WHERE id = $1{}", spec.table, deleted);
        let owned = id.to_string();
        let refs: Vec<&(dyn ToSql + Sync)> = vec![&owned as &(dyn ToSql + Sync)];
        let hit = client
            .query_opt(&sql, &refs)
            .await
            .map_err(|_| AppError::Internal)?
            .is_some();
        return Ok(hit);
    }

    params.push(id.to_string());
    let id_ph = format!("${}", params.len());
    let sql = format!(
        "UPDATE {} SET {} WHERE id = {}{}",
        spec.table,
        set_parts.join(", "),
        id_ph,
        deleted
    );
    let refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();

    let n = client
        .execute(&sql, &refs)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(n > 0)
}

/// 软删（有 deleted_at 列）或硬删。返回是否命中。
pub async fn crud_delete(pool: &Pool, spec: &CrudSpec, id: &str) -> Result<bool, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    use deadpool_postgres::tokio_postgres::types::ToSql;

    let owned = id.to_string();
    let refs: Vec<&(dyn ToSql + Sync)> = vec![&owned as &(dyn ToSql + Sync)];
    let sql = if spec.soft_delete {
        format!(
            "UPDATE {table} SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            table = spec.table
        )
    } else {
        format!("DELETE FROM {table} WHERE id = $1", table = spec.table)
    };
    let n = client
        .execute(&sql, &refs)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(n > 0)
}

// ──────────────────────────────────────────────────────────────────────────────
// 归属版（IDOR 门禁）——A3-N9：通用 crud_save/crud_delete 只按 id 落笔、无 owner 谓词，
// 任意登录用户可改/删他人行（含 x_person_face 生物特征）。归属版先按 owner_column 取属主、
// require_owner（admin 放行 / 本人放行，归一化），行不存在回 false（保 not-found 语义）。
// owner_column 由调用方以静态字面量传入（与 table/columns 同口径，不接受运行时输入）。
// ──────────────────────────────────────────────────────────────────────────────

/// 归属校验：行不存在回 Ok(false)；命中则 require_owner（非属主且非 admin → Err(Forbidden)）。
async fn crud_owner_ok(
    pool: &Pool,
    spec: &CrudSpec,
    id: &str,
    owner_column: &str,
    session: &crate::session::Session,
) -> Result<bool, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let deleted = if spec.soft_delete {
        " AND deleted_at IS NULL"
    } else {
        ""
    };
    let sql = format!(
        "SELECT {owner_column} AS owner FROM {} WHERE id = $1{deleted}",
        spec.table
    );
    let owner: Option<String> = client
        .query_opt(&sql, &[&id])
        .await
        .map_err(|_| AppError::Internal)?
        .map(|r| r.get::<_, Option<String>>("owner").unwrap_or_default());
    match owner {
        None => Ok(false),
        Some(o) => {
            crate::middleware::require_owner(pool, session, &o).await?;
            Ok(true)
        }
    }
}

/// 归属版 save：先判归属再 UPDATE（IDOR 门禁）。
pub async fn crud_save_owned(
    pool: &Pool,
    spec: &CrudSpec,
    id: &str,
    payload: &Value,
    owner_column: &str,
    session: &crate::session::Session,
) -> Result<bool, AppError> {
    if !crud_owner_ok(pool, spec, id, owner_column, session).await? {
        return Ok(false);
    }
    crud_save(pool, spec, id, payload).await
}

/// 归属版 delete：先判归属再软删/硬删（IDOR 门禁）。
pub async fn crud_delete_owned(
    pool: &Pool,
    spec: &CrudSpec,
    id: &str,
    owner_column: &str,
    session: &crate::session::Session,
) -> Result<bool, AppError> {
    if !crud_owner_ok(pool, spec, id, owner_column, session).await? {
        return Ok(false);
    }
    crud_delete(pool, spec, id).await
}

// ──────────────────────────────────────────────────────────────────────────────
// 批量插入——crud_create 的多行版
// ──────────────────────────────────────────────────────────────────────────────

/// PostgreSQL 单语句绑定参数上限（协议硬限 65535）。多行 INSERT 按
/// 「列数 + 1（id）」分片，保证每片参数数不越界（越界报 54000）。
pub const PG_MAX_PARAMS: usize = 65535;

/// 单个语句最多容纳的行数（至少 1，防极端宽表除零/空片）。
fn max_rows_per_statement(spec: &CrudSpec) -> usize {
    (PG_MAX_PARAMS / (spec.columns.len() + 1)).max(1)
}

/// 多行 INSERT 的 SQL 构造（纯函数，无 DB 依赖）：
/// `INSERT INTO <table> (id, c1, c2) VALUES ($1, $2, $3), ($4, $5, $6), ...`
/// 占位符全局连续编号；rows=0 返回空串（调用方应跳过）。
fn build_batch_insert_sql(spec: &CrudSpec, rows: usize) -> String {
    if rows == 0 {
        return String::new();
    }
    let col_names: Vec<&str> = spec.columns.iter().map(|(_, db)| *db).collect();
    let per_row = spec.columns.len() + 1; // + id
    let mut tuples: Vec<String> = Vec::with_capacity(rows);
    let mut n = 0usize;
    for _ in 0..rows {
        let placeholders: Vec<String> = (0..per_row)
            .map(|_| {
                n += 1;
                format!("${n}")
            })
            .collect();
        tuples.push(format!("({})", placeholders.join(", ")));
    }
    format!(
        "INSERT INTO {} (id, {}) VALUES {}",
        spec.table,
        col_names.join(", "),
        tuples.join(", ")
    )
}

/// 批量 INSERT 的扁平参数表（纯函数，无 DB 依赖）：按行交错
/// `[id, col1, col2, ..., id, col1, col2, ...]`，与 `build_batch_insert_sql`
/// 的全局连续编号一一对应；缺键口径与 `crud_create` 一致（`str_of` 补 ""）。
fn batch_params(spec: &CrudSpec, payloads: &[Value], ids: &[String]) -> Vec<String> {
    let mut params: Vec<String> = Vec::with_capacity(payloads.len() * (spec.columns.len() + 1));
    for (payload, id) in payloads.iter().zip(ids) {
        params.push(id.clone());
        for (json, _) in spec.columns {
            params.push(str_of(payload, json));
        }
    }
    params
}

/// 批量参数化 create：一次事务内多行 INSERT（`crud_create` 的多行版）。
/// 每行生成独立 uuid；列取值口径与 `crud_create` 一致（`str_of` 缺键补 ""）。
/// 超过 PostgreSQL 参数上限时自动分片，整批仍保持原子性（单事务）。
/// 返回新 id 列表（与 `payloads` 顺序一致）。
pub async fn crud_create_many(
    pool: &Pool,
    spec: &CrudSpec,
    payloads: &[Value],
) -> Result<Vec<String>, AppError> {
    if payloads.is_empty() {
        return Ok(Vec::new());
    }

    let ids: Vec<String> = (0..payloads.len())
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect();

    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    use deadpool_postgres::tokio_postgres::types::ToSql;

    let max_rows = max_rows_per_statement(spec);
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    for (chunk_payloads, chunk_ids) in payloads.chunks(max_rows).zip(ids.chunks(max_rows)) {
        let sql = build_batch_insert_sql(spec, chunk_payloads.len());
        let params = batch_params(spec, chunk_payloads, chunk_ids);
        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
        tx.execute(&sql, &refs)
            .await
            .map_err(|_| AppError::Internal)?;
    }
    tx.commit().await.map_err(|_| AppError::Internal)?;

    Ok(ids)
}

#[cfg(test)]
mod batch_insert_tests {
    use super::*;
    use serde_json::json;

    fn probe_spec() -> CrudSpec {
        CrudSpec {
            table: "x_probe",
            columns: &[("a", "col_a"), ("b", "col_b")],
            soft_delete: false,
        }
    }

    #[test]
    fn batch_sql_numbering_is_sequential_and_covers_all_rows() {
        // 占位符必须全局连续（$1..$9），不能按行重置——否则参数错位。
        let sql = build_batch_insert_sql(&probe_spec(), 3);
        assert_eq!(
            sql,
            "INSERT INTO x_probe (id, col_a, col_b) VALUES ($1, $2, $3), ($4, $5, $6), ($7, $8, $9)"
        );
    }

    #[test]
    fn batch_sql_single_row_matches_create_shape() {
        // 单行形状必须与 crud_create 一致（id + 列数占位符）。
        let sql = build_batch_insert_sql(&probe_spec(), 1);
        assert_eq!(
            sql,
            "INSERT INTO x_probe (id, col_a, col_b) VALUES ($1, $2, $3)"
        );
    }

    #[test]
    fn zero_rows_yields_empty_sql() {
        assert_eq!(build_batch_insert_sql(&probe_spec(), 0), "");
    }

    #[test]
    fn batch_params_interleave_ids_and_values_with_missing_key_fallback() {
        // 参数顺序必须与 SQL 占位符逐位对应：每行先 id 后各列；
        // 缺键补 ""（与 crud_create 的 str_of 口径一致）。
        let spec = probe_spec();
        let payloads = [
            json!({ "a": "1", "b": "2" }),
            json!({ "a": "3" }), // 缺 "b" → ""
        ];
        let ids = ["id-1".to_string(), "id-2".to_string()];
        let params = batch_params(&spec, &payloads, &ids);
        assert_eq!(params, vec!["id-1", "1", "2", "id-2", "3", ""]);
    }

    #[test]
    fn chunk_size_respects_pg_param_limit() {
        // 2 列 + id = 3 参数/行 → 65535/3 = 21845 行/片。
        assert_eq!(max_rows_per_statement(&probe_spec()), 21845);
        // 极端宽表（70000 列）也不能得到 0 行/片（除零保护）。
        let wide = CrudSpec {
            table: "x_wide",
            columns: &[("c", "c"); 70000],
            soft_delete: false,
        };
        assert_eq!(max_rows_per_statement(&wide), 1);
    }

    #[tokio::test]
    async fn create_many_empty_payload_returns_empty_without_db() {
        // 空批必须在触库前短路返回 Ok(空)。deadpool 池懒连接，
        // 构建不连 DB，无 PG 环境也可验证该短路路径。
        use deadpool_postgres::tokio_postgres::{Config, NoTls};
        let mut cfg = Config::new();
        cfg.host("localhost").port(5432).user("u").dbname("d");
        let pool = deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(cfg, NoTls))
            .build()
            .unwrap();
        let ids = crud_create_many(&pool, &probe_spec(), &[]).await.unwrap();
        assert!(ids.is_empty());
    }
}
