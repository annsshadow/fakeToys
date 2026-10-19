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
