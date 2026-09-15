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
    /// 允许读写的 (json_key, db_column) 映射；create 按 json_key 取 payload，写按 db_column。
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
    let refs: Vec<&(dyn ToSql + Sync)> =
        params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();

    client
        .execute(&sql, &refs)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(id)
}

/// 参数化 save：UPDATE 指定列 WHERE id = $N（软删表追加 deleted_at IS NULL）。返回是否命中。
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
        set_parts.push(format!("{db}=${}", params.len() + 1));
        params.push(str_of(payload, json));
    }
    params.push(id.to_string());
    let id_ph = format!("${}", params.len());
    let deleted = if spec.soft_delete {
        " AND deleted_at IS NULL"
    } else {
        ""
    };
    let sql = format!(
        "UPDATE {} SET {} WHERE id = {}{}",
        spec.table,
        set_parts.join(", "),
        id_ph,
        deleted
    );
    let refs: Vec<&(dyn ToSql + Sync)> =
        params.iter().map(|v| v as &(dyn ToSql + Sync)).collect();

    let n = client
        .execute(&sql, &refs)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(n > 0)
}

/// 软删（有 deleted_at 列）或硬删。返回是否命中。
pub async fn crud_delete(
    pool: &Pool,
    spec: &CrudSpec,
    id: &str,
) -> Result<bool, AppError> {
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
        format!(
            "DELETE FROM {table} WHERE id = $1",
            table = spec.table
        )
    };
    let n = client
        .execute(&sql, &refs)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(n > 0)
}
