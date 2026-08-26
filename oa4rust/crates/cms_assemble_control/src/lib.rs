use axum::{
    extract::Extension,
    Json,
    Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult, response::row_to_json};
use deadpool_postgres::tokio_postgres::types::ToSql;
use std::collections::HashMap;
use search::Document;
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;
#[cfg(test)]
mod tests_u2;
#[cfg(test)]
mod tests_data_appdict;
#[cfg(test)]
mod tests_u3;


#[axum::debug_handler]
pub async fn application_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, alias, app_type, icon, enabled, manager FROM x_cms_appinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("appType".to_string(), Value::String(row.get("app_type"))),
                ("icon".to_string(), Value::String(row.get::<_, Option<String>>("icon").unwrap_or_default())),
                ("enabled".to_string(), Value::Bool(row.get::<_, Option<bool>>("enabled").unwrap_or_default())),
                ("manager".to_string(), Value::String(row.get::<_, Option<String>>("manager").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

#[axum::debug_handler]
pub async fn get_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT enabled, max_category_count, allow_anonymous FROM x_cms_assemble_control_config ORDER BY create_time::text LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(row.get("enabled"))),
        ("maxCategoryCount".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("max_category_count")))),
        ("allowAnonymous".to_string(), Value::Bool(row.get("allow_anonymous"))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_control_sections(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, enabled FROM x_cms_assemble_control_section ORDER BY create_time::text",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let sections: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
            ]))
        })
        .collect();

    let count = sections.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(sections), count, 0)))
}

#[axum::debug_handler]
pub async fn update_control_config(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let enabled = body.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
    let max_category_count = body.get("maxCategoryCount").and_then(|v| v.as_i64()).unwrap_or(500);
    let allow_anonymous = body.get("allowAnonymous").and_then(|v| v.as_bool()).unwrap_or(false);

    client
        .execute(
            "UPDATE x_cms_assemble_control_config SET enabled = $1, max_category_count = $2, allow_anonymous = $3 WHERE id = (SELECT id FROM x_cms_assemble_control_config ORDER BY create_time::text LIMIT 1)",
            &[&enabled, &max_category_count, &allow_anonymous],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), body.0),
        ]),
    ))))
}

pub fn cms_assemble_control_router(pool: Pool) -> Router {
    routes::router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::cms_assemble_control_router(pool)
}


// ─── Helper: generic list handler ───────────────────────────────────────────

async fn list_from_table_inner(
    pool: &Pool,
    table: &str,
    where_clause: &str,
    params: &[(&(dyn ToSql + Sync), &str)],
) -> Result<(i64, Vec<Value>), AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count_sql = format!(
        "SELECT COUNT(*) FROM {}{}",
        table,
        if where_clause.is_empty() { String::new() } else { format!(" WHERE {}", where_clause) }
    );
    let count_row = client
        .query_one(&count_sql, &params.iter().map(|(p, _)| *p).collect::<Vec<_>>()[..])
        .await
        .map_err(|_| AppError::Internal)?;
    let count: i64 = count_row.get("count");

    let data_sql = format!(
        "SELECT * FROM {}{}",
        table,
        if where_clause.is_empty() { String::new() } else { format!(" WHERE {}", where_clause) }
    );
    let rows = client
        .query(&data_sql, &params.iter().map(|(p, _)| *p).collect::<Vec<_>>()[..])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();

    Ok((count, data))
}

async fn list_from_table(
    pool: &Pool,
    table: &str,
    where_clause: &str,
    params: &[(&(dyn ToSql + Sync), &str)],
) -> Result<Value, AppError> {
    let (count, data) = list_from_table_inner(pool, table, where_clause, params).await?;
    Ok(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ("data".to_string(), Value::Array(data)),
    ])))
}

// Java 裸数组契约（行为对齐）：data 为数组、count 入信封、size 恒 0。
#[allow(clippy::too_many_arguments)]
async fn list_from_table_java(
    pool: &Pool,
    table: &str,
    where_clause: &str,
    params: &[(&(dyn ToSql + Sync), &str)],
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (count, data) = list_from_table_inner(pool, table, where_clause, params).await?;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

async fn list_from_table_filtered_inner(
    pool: &Pool,
    table: &str,
    where_clause: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<(i64, Vec<Value>), AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count_sql = format!(
        "SELECT COUNT(*) FROM {}{}",
        table,
        if where_clause.is_empty() { String::new() } else { format!(" WHERE {}", where_clause) }
    );
    let count_row = client
        .query_one(&count_sql, params)
        .await
        .map_err(|_| AppError::Internal)?;
    let count: i64 = count_row.get("count");

    let data_sql = format!(
        "SELECT * FROM {}{}",
        table,
        if where_clause.is_empty() { String::new() } else { format!(" WHERE {}", where_clause) }
    );
    let rows = client
        .query(&data_sql, params)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();

    Ok((count, data))
}

async fn list_from_table_filtered(
    pool: &Pool,
    table: &str,
    where_clause: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<Value, AppError> {
    let (count, data) = list_from_table_filtered_inner(pool, table, where_clause, params).await?;
    Ok(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ("data".to_string(), Value::Array(data)),
    ])))
}

// Java 裸数组契约（行为对齐）：data 为数组、count 入信封、size 恒 0。
async fn list_from_table_filtered_java(
    pool: &Pool,
    table: &str,
    where_clause: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (count, data) = list_from_table_filtered_inner(pool, table, where_clause, params).await?;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

async fn delete_by_id(pool: &Pool, table: &str, id: &str) -> Result<Value, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            &format!("DELETE FROM {} WHERE id = $1", table),
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Value::Object(serde_json::Map::from_iter([
        ("deleted".to_string(), Value::Bool(true)),
    ])))
}

async fn get_by_id(pool: &Pool, table: &str, id: &str) -> Result<Option<Value>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            &format!("SELECT * FROM {} WHERE id = $1 AND deleted_at::text IS NULL", table),
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => Ok(Some(row_to_json(&r))),
        None => Ok(None),
    }
}

async fn soft_delete_by_id(pool: &Pool, table: &str, id: &str) -> Result<Value, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute(
            &format!("UPDATE {} SET deleted_at = NOW() WHERE id = $1", table),
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Value::Object(serde_json::Map::from_iter([
        ("deleted".to_string(), Value::Bool(true)),
        ("count".to_string(), Value::Number(serde_json::Number::from(affected as i64))),
    ])))
}

async fn upsert_by_id(pool: &Pool, table: &str, body: &Value) -> Result<Value, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = body.get("id").and_then(|v| v.as_str()).unwrap_or("");
    if id.is_empty() {
        return Ok(Value::Object(serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(false)),
        ])));
    }
    let cols: Vec<&str> = body
        .as_object()
        .map(|m| m.keys().map(|k| k.as_str()).collect())
        .unwrap_or_default();
    if cols.is_empty() {
        return Ok(Value::Object(serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(false)),
        ])));
    }
    let sets: Vec<String> = cols.iter().enumerate().map(|(i, c)| format!("{} = ${}", c, i + 2)).collect();
    let placeholders: Vec<String> = cols.iter().enumerate().map(|(i, _)| format!("${}", i + 2)).collect();
    let values: Vec<Box<dyn ToSql + Sync>> = cols
        .iter()
        .map(|c| match body.get(*c) {
            Some(v) => match v {
                Value::Bool(b) => Box::new(*b) as Box<dyn ToSql + Sync>,
                Value::Number(n) => {
                    if let Some(i) = n.as_i64() { Box::new(i) as Box<dyn ToSql + Sync> }
                    else if let Some(f) = n.as_f64() { Box::new(f as i64) }
                    else { Box::new(0i64) }
                }
                Value::String(s) => Box::new(s.as_str()),
                _ => Box::new("" as &str),
            },
            None => Box::new("" as &str),
        })
        .collect();

    let sql = format!(
        "INSERT INTO {} (id, {}) VALUES ($1, {}) ON CONFLICT (id) DO UPDATE SET {} WHERE {}.id = $1",
        table,
        cols.join(", "),
        (2..=cols.len()+1).map(|i| format!("${}", i)).collect::<Vec<_>>().join(", "),
        cols.iter().enumerate().map(|(i, c)| format!("{} = ${}", c, i + 2)).collect::<Vec<_>>().join(", "),
        table
    );
    let mut params: Vec<Box<dyn ToSql + Sync>> = vec![Box::new(id)];
    params.extend(values);
    let params_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref()).collect();

    client.execute(&sql, &params_refs).await.map_err(|_| AppError::Internal)?;
    Ok(Value::Object(serde_json::Map::from_iter([
        ("saved".to_string(), Value::Bool(true)),
    ])))
}

// ─── anonymous_* stubs ──────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn anonymous_document_filter_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_data_document", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn anonymous_document_filter_list_id_next_count_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_data_document", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn anonymous_document_filter_list_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_data_document", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn anonymous_document_filter_list_page_size_size_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_data_document", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn anonymous_document_id_view(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, content, author_id, status, publish_time::text, creator, create_time::text FROM x_cms_data_document WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get::<_, Option<String>>("title").unwrap_or_default())),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("authorId".to_string(), Value::String(row.get::<_, Option<String>>("author_id").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("document not found"))),
    }
}

#[axum::debug_handler]
pub async fn anonymous_fileinfo_list_document_documentId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_fileinfo", "deleted_at IS NULL", &[]).await
}

// ─── appinfo_* stubs ────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn appinfo_alias_alias(
    pool: Extension<Pool>,
    axum::extract::Path(alias): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, alias, app_type, icon, enabled, manager FROM x_cms_appinfo WHERE alias = $1 AND deleted_at::text IS NULL",
            &[&alias],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("appType".to_string(), Value::String(row.get("app_type"))),
                ("icon".to_string(), Value::String(row.get::<_, Option<String>>("icon").unwrap_or_default())),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("manager".to_string(), Value::String(row.get("manager"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("appinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn appinfo_erase_app_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_appinfo", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn appinfo_erase_app_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_appinfo", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn appinfo_filter_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_filter_list_id_next_count_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_filter_list_id_prev_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_filter_list_id_prev_count_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_get_user_publish_appId(
    pool: Extension<Pool>,
    axum::extract::Path(app_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, alias, app_type, icon, enabled, manager FROM x_cms_appinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&app_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("appType".to_string(), Value::String(row.get("app_type"))),
                ("icon".to_string(), Value::String(row.get::<_, Option<String>>("icon").unwrap_or_default())),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("manager".to_string(), Value::String(row.get("manager"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("appinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn appinfo_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_appType_manager(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_has_document(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_has_document_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_has_document_type_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_manage(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_manage_type_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_user_publish(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_user_publish_type_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_user_publish_with_process(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_user_view(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_user_view_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_user_view_all_type_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_user_view_article_type_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_user_view_data(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_list_user_view_data_type_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_appId_icon_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((app_id, icon_size)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, alias, app_type, icon, enabled, manager FROM x_cms_appinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&app_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("appType".to_string(), Value::String(row.get("app_type"))),
                ("icon".to_string(), Value::String(row.get::<_, Option<String>>("icon").unwrap_or_default())),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("manager".to_string(), Value::String(row.get("manager"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("appinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn appinfo_flag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn appinfo_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, alias, app_type, icon, enabled, manager FROM x_cms_appinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("appType".to_string(), Value::String(row.get("app_type"))),
                ("icon".to_string(), Value::String(row.get::<_, Option<String>>("icon").unwrap_or_default())),
                ("enabled".to_string(), Value::Bool(row.get::<_, Option<bool>>("enabled").unwrap_or_default())),
                ("manager".to_string(), Value::String(row.get::<_, Option<String>>("manager").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("appinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn appinfo_id_control(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, alias, app_type, icon, enabled, manager FROM x_cms_appinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("appType".to_string(), Value::String(row.get("app_type"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("manager".to_string(), Value::String(row.get("manager"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("appinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn appinfo_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_appinfo", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn appinfo_id_permission(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_id, category_id, person_id, role_type, permission_level FROM x_cms_permission WHERE app_id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appId".to_string(), Value::String(row.get("app_id"))),
            ("categoryId".to_string(), Value::String(row.get::<_, Option<String>>("category_id").unwrap_or_default())),
            ("personId".to_string(), Value::String(row.get::<_, Option<String>>("person_id").unwrap_or_default())),
            ("roleType".to_string(), Value::String(row.get::<_, Option<String>>("role_type").unwrap_or_default())),
            ("permissionLevel".to_string(), Value::String(row.get::<_, Option<String>>("permission_level").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

// ─── categoryinfo_* stubs ───────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn categoryinfo_alias_alias(
    pool: Extension<Pool>,
    axum::extract::Path(alias): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, parent_id, app_id, sort_order, status, ext_content, creator, create_time::text FROM x_cms_categoryinfo WHERE alias = $1 AND deleted_at::text IS NULL",
            &[&alias],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("parentId".to_string(), Value::String(row.get::<_, Option<String>>("parent_id").unwrap_or_default())),
                ("appId".to_string(), Value::String(row.get::<_, Option<String>>("app_id").unwrap_or_default())),
                ("sortOrder".to_string(), Value::Number(serde_json::Number::from(row.get::<_, Option<i32>>("sort_order").unwrap_or(0)))),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                ("extContent".to_string(), Value::String(row.get::<_, Option<String>>("ext_content").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("category not found"))),
    }
}

#[axum::debug_handler]
pub async fn categoryinfo_bind_categoryId_view(
    pool: Extension<Pool>,
    axum::extract::Path(category_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, parent_id, app_id, sort_order, status, ext_content, creator, create_time::text FROM x_cms_categoryinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&category_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("parentId".to_string(), Value::String(row.get::<_, Option<String>>("parent_id").unwrap_or_default())),
                ("appId".to_string(), Value::String(row.get::<_, Option<String>>("app_id").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("category not found"))),
    }
}

#[axum::debug_handler]
pub async fn categoryinfo_bind_categoryId_view_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(category_id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let parent_id = body.get("parentId").and_then(|v| v.as_str()).unwrap_or("");
    let app_id = body.get("appId").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_categoryinfo (id, name, parent_id, app_id) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET name = $2, parent_id = $3, app_id = $4 RETURNING id",
            &[&category_id, &name, &parent_id, &app_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, parent_id, app_id, sort_order, status, ext_content, creator, create_time::text FROM x_cms_categoryinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&category_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("parentId".to_string(), Value::String(row.get::<_, Option<String>>("parent_id").unwrap_or_default())),
                ("appId".to_string(), Value::String(row.get::<_, Option<String>>("app_id").unwrap_or_default())),
                ("sortOrder".to_string(), Value::Number(serde_json::Number::from(row.get::<_, Option<i32>>("sort_order").unwrap_or(0)))),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                ("extContent".to_string(), Value::String(row.get::<_, Option<String>>("ext_content").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("category not found"))),
    }
}

#[axum::debug_handler]
pub async fn categoryinfo_erase_category_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_categoryinfo", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn categoryinfo_erase_category_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_categoryinfo", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn categoryinfo_extContent(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, ext_content FROM x_cms_categoryinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("extContent".to_string(), Value::String(row.get::<_, Option<String>>("ext_content").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("category not found"))),
    }
}

#[axum::debug_handler]
pub async fn categoryinfo_filter_list_id_next_count_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_filter_list_id_next_count_app_appId_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_filter_list_id_prev_count_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_filter_list_id_prev_count_app_appId_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_filter_list_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_filter_list_page_size_size_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_list_manage_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_list_objects(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_list_publish_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_list_view_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_list_view_app_appId_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_list_view_app_appId_data(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_flag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn categoryinfo_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, parent_id, app_id, sort_order, status, ext_content, creator, create_time::text FROM x_cms_categoryinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("parentId".to_string(), Value::String(row.get::<_, Option<String>>("parent_id").unwrap_or_default())),
                ("appId".to_string(), Value::String(row.get::<_, Option<String>>("app_id").unwrap_or_default())),
                ("sortOrder".to_string(), Value::Number(serde_json::Number::from(row.get::<_, Option<i32>>("sort_order").unwrap_or(0)))),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                ("extContent".to_string(), Value::String(row.get::<_, Option<String>>("ext_content").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("category not found"))),
    }
}

#[axum::debug_handler]
pub async fn categoryinfo_id_control(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, parent_id, app_id, sort_order, status, ext_content, creator, create_time::text FROM x_cms_categoryinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("parentId".to_string(), Value::String(row.get::<_, Option<String>>("parent_id").unwrap_or_default())),
                ("appId".to_string(), Value::String(row.get::<_, Option<String>>("app_id").unwrap_or_default())),
                ("sortOrder".to_string(), Value::Number(serde_json::Number::from(row.get::<_, Option<i32>>("sort_order").unwrap_or(0)))),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("category not found"))),
    }
}

#[axum::debug_handler]
pub async fn categoryinfo_id_execute_projection(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, parent_id, app_id, sort_order, status, ext_content, creator, create_time::text FROM x_cms_categoryinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("parentId".to_string(), Value::String(row.get::<_, Option<String>>("parent_id").unwrap_or_default())),
                ("appId".to_string(), Value::String(row.get::<_, Option<String>>("app_id").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("category not found"))),
    }
}

#[axum::debug_handler]
pub async fn categoryinfo_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_categoryinfo", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn categoryinfo_id_permission(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_id, category_id, person_id, role_type, permission_level FROM x_cms_permission WHERE category_id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appId".to_string(), Value::String(row.get("app_id"))),
            ("categoryId".to_string(), Value::String(row.get("category_id"))),
            ("personId".to_string(), Value::String(row.get::<_, Option<String>>("person_id").unwrap_or_default())),
            ("roleType".to_string(), Value::String(row.get::<_, Option<String>>("role_type").unwrap_or_default())),
            ("permissionLevel".to_string(), Value::String(row.get::<_, Option<String>>("permission_level").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

// ─── comment_* / commend_* stubs ────────────────────────────────────────────

#[axum::debug_handler]
pub async fn commend_list_paging_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_commend", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn commend_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, person_id, create_time::text FROM x_cms_commend WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("personId".to_string(), Value::String(row.get::<_, Option<String>>("person_id").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("commend not found"))),
    }
}

#[axum::debug_handler]
pub async fn comment_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn comment_list_id_next_count_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn comment_list_id_prev_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn comment_list_id_prev_count_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn comment_list_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn comment_list_page_size_size_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn comment_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, person_id, content, parent_id, create_time::text FROM x_cms_comment WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("personId".to_string(), Value::String(row.get::<_, Option<String>>("person_id").unwrap_or_default())),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("parentId".to_string(), Value::String(row.get::<_, Option<String>>("parent_id").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("comment not found"))),
    }
}

#[axum::debug_handler]
pub async fn comment_id_commend(
    pool: Extension<Pool>,
    axum::extract::Path((comment_id, person_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row_id = client
        .query_one(
            "INSERT INTO x_cms_commend (id, doc_id, person_id) VALUES (gen_random_uuid()::text, (SELECT doc_id FROM x_cms_comment WHERE id = $1), $2) RETURNING id",
            &[&comment_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let new_id: String = row_id.get("id");
    let row = client
        .query_opt(
            "SELECT id, doc_id, person_id, create_time::text FROM x_cms_commend WHERE id = $1 AND deleted_at::text IS NULL",
            &[&new_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("personId".to_string(), Value::String(row.get::<_, Option<String>>("person_id").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("commend not found"))),
    }
}

#[axum::debug_handler]
pub async fn comment_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_comment", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn comment_id_uncommend(
    pool: Extension<Pool>,
    axum::extract::Path((comment_id, person_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute(
            "DELETE FROM x_cms_commend WHERE doc_id = (SELECT doc_id FROM x_cms_comment WHERE id = $1) AND person_id = $2",
            &[&comment_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Number(serde_json::Number::from(affected as i64)))]),
    ))))
}

// ─── correlation_* stubs ────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn correlation_doc_docId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_correlation", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn correlation_doc_docId_delete(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, related_doc_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_correlation SET deleted_at = NOW() WHERE doc_id = $1 AND related_doc_id = $2",
            &[&doc_id, &related_doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, related_doc_id, correlation_type, create_time::text FROM x_cms_correlation WHERE doc_id = $1 AND related_doc_id = $2",
            &[&doc_id, &related_doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("relatedDocId".to_string(), Value::String(row.get("related_doc_id"))),
                ("correlationType".to_string(), Value::String(row.get::<_, Option<String>>("correlation_type").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("correlation not found"))),
    }
}

#[axum::debug_handler]
pub async fn correlation_list_doc_docId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_correlation", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn correlation_list_doc_docId_site_site(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_correlation", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn correlation_update_doc_docId(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, related_doc_id)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let correlation_type = body.get("correlationType").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_correlation (id, doc_id, related_doc_id, correlation_type) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (doc_id, related_doc_id) DO UPDATE SET correlation_type = $3",
            &[&doc_id, &related_doc_id, &correlation_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, related_doc_id, correlation_type, create_time::text FROM x_cms_correlation WHERE doc_id = $1 AND related_doc_id = $2 AND deleted_at::text IS NULL",
            &[&doc_id, &related_doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("relatedDocId".to_string(), Value::String(row.get("related_doc_id"))),
                ("correlationType".to_string(), Value::String(row.get::<_, Option<String>>("correlation_type").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("correlation not found"))),
    }
}

// ─── data_document_* stubs ──────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn data_document_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_data_document", "deleted_at IS NULL", &[]).await
}

// ─── data/document 通配族共享 helpers（Java DataAction 对齐） ────────────────
//
// Java 语义：data/document/{id}/{path0}/.../{pathN} 是文档数据的嵌套路径。
// 本端口用 x_cms_data_document_field 扁平行模型承载：多级路径折叠为
// field_name = "p0.p1...pn"（一级路径与既有行为一致，无点号）。

fn compose_field_path(paths: &[String]) -> String {
    paths.join(".")
}

fn field_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("docId".to_string(), Value::String(row.get("doc_id"))),
        ("fieldName".to_string(), Value::String(row.get("field_name"))),
        ("fieldValue".to_string(), Value::String(row.get::<_, Option<String>>("field_value").unwrap_or_default())),
        ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
    ]))
}

async fn query_doc_fields_by_name(
    pool: &Pool,
    doc_id: &str,
    field_name: &str,
) -> Result<Vec<Value>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, field_name, field_value, create_time::text \
             FROM x_cms_data_document_field \
             WHERE doc_id = $1 AND field_name = $2 AND deleted_at IS NULL",
            &[&doc_id, &field_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(rows.iter().map(field_row_to_value).collect())
}

/// UPSERT 单个字段行。表中 (doc_id, field_name) 无唯一约束，
/// 不能用 ON CONFLICT —— 先 UPDATE 后按需 INSERT。
async fn upsert_doc_field(
    pool: &Pool,
    doc_id: &str,
    field_name: &str,
    field_value: &str,
) -> Result<(), AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let updated = client
        .execute(
            "UPDATE x_cms_data_document_field SET field_value = $3, deleted_at = NULL \
             WHERE doc_id = $1 AND field_name = $2",
            &[&doc_id, &field_name, &field_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        client
            .execute(
                "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) \
                 VALUES (gen_random_uuid()::text, $1, $2, $3)",
                &[&doc_id, &field_name, &field_value],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }
    Ok(())
}

/// data 写操作门禁：会话用户必须是宿主文档 creator 或管理员。
async fn gate_doc_writer(
    pool: &Pool,
    session: &shared::session::Session,
    doc_id: &str,
) -> Result<U2Gate, AppError> {
    u2_check_owner(pool, "x_cms_data_document", "creator", doc_id, &session.person_unique).await
}

/// data/document/{id} 基座写端点（PUT/POST）：body 顶层每个 key 落一行字段。
///   mode=Update: 全部 upsert；mode=Create: 仅插入不存在的 key（跳过已有）。
#[derive(PartialEq)]
enum FieldWriteMode {
    Create,
    Update,
}

async fn write_doc_fields_base(
    pool: &Pool,
    session: &shared::session::Session,
    id: &str,
    body: &Value,
    mode: FieldWriteMode,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match gate_doc_writer(pool, session, id).await? {
        U2Gate::NotFound => return Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => return Err(AppError::Forbidden),
        U2Gate::Allowed => {}
    }
    let obj = match body.as_object() {
        Some(obj) if !obj.is_empty() => obj,
        _ => return Err(AppError::BadRequest("json object body required".to_string())),
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut affected: i64 = 0;
    for (key, value) in obj {
        let value_str = match value.as_str() {
            Some(s) => s.to_string(),
            None => serde_json::to_string(value).unwrap_or_default(),
        };
        if mode == FieldWriteMode::Create {
            let inserted = client
                .execute(
                    "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) \
                     SELECT gen_random_uuid()::text, $1::varchar, $2::varchar, $3::text \
                     WHERE NOT EXISTS (\
                       SELECT 1 FROM x_cms_data_document_field \
                       WHERE doc_id = $1 AND field_name = $2 AND deleted_at IS NULL)",
                    &[&id, &key, &value_str],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            affected += inserted as i64;
        } else {
            upsert_doc_field(&pool, id, key, &value_str).await?;
            affected += 1;
        }
    }
    let key = if mode == FieldWriteMode::Create { "created" } else { "updated" };
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("documentId".to_string(), Value::String(id.to_string())),
        (key.to_string(), Value::Number(serde_json::Number::from(affected))),
    ])))))
}

/// data/document/{id} DELETE：软删该文档全部数据字段行。
async fn delete_doc_fields_base(
    pool: &Pool,
    session: &shared::session::Session,
    id: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match gate_doc_writer(pool, session, id).await? {
        U2Gate::NotFound => return Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => return Err(AppError::Forbidden),
        U2Gate::Allowed => {}
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let deleted = client
        .execute(
            "UPDATE x_cms_data_document_field SET deleted_at = NOW() \
             WHERE doc_id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("documentId".to_string(), Value::String(id.to_string())),
        ("deleted".to_string(), Value::Number(serde_json::Number::from(deleted as i64))),
    ])))))
}

/// data/document/{id}/{path...} 路径级写端点共享实现：
///   Update: 整体 body 作为该路径值 upsert；
///   Create: 仅当该路径字段不存在时插入；
///   Delete: 软删该路径字段行。
enum PathWriteMode {
    Upsert,
    InsertIfAbsent,
    Remove,
}

async fn write_doc_field_path(
    pool: &Pool,
    session: &shared::session::Session,
    doc_id: &str,
    paths: Vec<String>,
    body: Option<&Value>,
    mode: PathWriteMode,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match gate_doc_writer(pool, session, doc_id).await? {
        U2Gate::NotFound => return Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => return Err(AppError::Forbidden),
        U2Gate::Allowed => {}
    }
    let field_name = compose_field_path(&paths);
    match mode {
        PathWriteMode::Upsert => {
            let value = body.expect("upsert requires body");
            let value_str = serde_json::to_string(value).unwrap_or_default();
            upsert_doc_field(pool, doc_id, &field_name, &value_str).await?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("docId".to_string(), Value::String(doc_id.to_string())),
                ("fieldName".to_string(), Value::String(field_name)),
                ("updated".to_string(), Value::Bool(true)),
            ])))))
        }
        PathWriteMode::InsertIfAbsent => {
            let value = body.expect("create requires body");
            let value_str = serde_json::to_string(value).unwrap_or_default();
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            // $1/$2 在选择列与子查询中双重出现：显式 ::varchar 防止 PG 推断出
            // "text vs character varying" 不一致类型（E42P08）
            let inserted = client
                .execute(
                    "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) \
                     SELECT gen_random_uuid()::text, $1::varchar, $2::varchar, $3::text \
                     WHERE NOT EXISTS (\
                       SELECT 1 FROM x_cms_data_document_field \
                       WHERE doc_id = $1 AND field_name = $2 AND deleted_at IS NULL)",
                    &[&doc_id, &field_name, &value_str],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if inserted == 0 {
                return Ok(Json(ActionResult::error("field already exists")));
            }
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("docId".to_string(), Value::String(doc_id.to_string())),
                ("fieldName".to_string(), Value::String(field_name)),
                ("created".to_string(), Value::Bool(true)),
            ])))))
        }
        PathWriteMode::Remove => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let deleted = client
                .execute(
                    "UPDATE x_cms_data_document_field SET deleted_at = NOW() \
                     WHERE doc_id = $1 AND field_name = $2 AND deleted_at IS NULL",
                    &[&doc_id, &field_name],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if deleted == 0 {
                return Ok(Json(ActionResult::error("field not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("docId".to_string(), Value::String(doc_id.to_string())),
                ("fieldName".to_string(), Value::String(field_name)),
                ("deleted".to_string(), Value::Bool(true)),
            ])))))
        }
    }
}

macro_rules! data_path_read_handler {
    ($name:ident, $arity:literal) => {
        #[axum::debug_handler]
        pub async fn $name(
            pool: Extension<Pool>,
            axum::extract::Path(paths): axum::extract::Path<[String; $arity]>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            let (doc_id, rest) = paths.split_first().expect("non-empty path tuple");
            let field_name = compose_field_path(rest);
            let data = query_doc_fields_by_name(&pool, doc_id, &field_name).await?;
            let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
        }
    };
}

data_path_read_handler!(data_document_id_path0_path1, 3);
data_path_read_handler!(data_document_id_path0_path1_path2, 4);
data_path_read_handler!(data_document_id_path0_path1_path2_path3, 5);
data_path_read_handler!(data_document_id_path0_path1_path2_path3_path4, 6);
data_path_read_handler!(data_document_id_path0_path1_path2_path3_path4_path5, 7);
data_path_read_handler!(data_document_id_path0_path1_path2_path3_path4_path5_path6, 8);
data_path_read_handler!(data_document_id_path0_path1_path2_path3_path4_path5_path6_path7, 9);

/// 路径级 mockdeletetoget / mockputtopost / update / create / delete 的宏生成器。
///
/// - mockdelete（GET 别名删除）与 delete：软删 composed 字段行；
/// - mockput（POST 别名更新，读 body["fieldValue"]，沿用既有契约）、update（PUT，
///   整体 body 为值）：UPSERT（修复旧代码对无唯一约束列使用 ON CONFLICT 的运行期错误）；
/// - create（POST）：仅在该字段行不存在时插入。
macro_rules! data_path_write_handlers {
    ($mockdel:ident, $del:ident, $put:ident, $create:ident, $mockput:ident, $arity:literal) => {
        #[axum::debug_handler]
        pub async fn $mockdel(
            pool: Extension<Pool>,
            axum::extract::Path(paths): axum::extract::Path<[String; $arity]>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            let (doc_id, rest) = paths.split_first().expect("non-empty path tuple");
            let field_name = compose_field_path(rest);
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            client
                .execute(
                    "UPDATE x_cms_data_document_field SET deleted_at = NOW() \
                     WHERE doc_id = $1 AND field_name = $2",
                    &[&doc_id, &field_name],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let rows = query_doc_fields_by_name(&pool, doc_id, &field_name).await?;
            match rows.into_iter().next() {
                Some(row) => Ok(Json(ActionResult::success(row))),
                None => Ok(Json(ActionResult::error("field not found"))),
            }
        }

        #[axum::debug_handler]
        pub async fn $del(
            pool: Extension<Pool>,
            session: Extension<shared::session::Session>,
            axum::extract::Path(paths): axum::extract::Path<[String; $arity]>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            let (doc_id, rest) = paths.split_first().expect("non-empty path tuple");
            write_doc_field_path(&pool, &session, doc_id, rest.to_vec(), None, PathWriteMode::Remove)
                .await
        }

        #[axum::debug_handler]
        pub async fn $put(
            pool: Extension<Pool>,
            session: Extension<shared::session::Session>,
            axum::extract::Path(paths): axum::extract::Path<[String; $arity]>,
            body: axum::extract::Json<Value>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            let (doc_id, rest) = paths.split_first().expect("non-empty path tuple");
            write_doc_field_path(&pool, &session, doc_id, rest.to_vec(), Some(&body), PathWriteMode::Upsert)
                .await
        }

        #[axum::debug_handler]
        pub async fn $create(
            pool: Extension<Pool>,
            session: Extension<shared::session::Session>,
            axum::extract::Path(paths): axum::extract::Path<[String; $arity]>,
            body: axum::extract::Json<Value>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            let (doc_id, rest) = paths.split_first().expect("non-empty path tuple");
            write_doc_field_path(&pool, &session, doc_id, rest.to_vec(), Some(&body), PathWriteMode::InsertIfAbsent)
                .await
        }

        /// POST {paths}/mockputtopost：PUT 的动词别名（MockPutToPost）。
        /// 沿用既有契约：读 body["fieldValue"] 字符串作为路径值。
        #[axum::debug_handler]
        pub async fn $mockput(
            pool: Extension<Pool>,
            session: Extension<shared::session::Session>,
            axum::extract::Path(paths): axum::extract::Path<[String; $arity]>,
            body: axum::extract::Json<Value>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            let (doc_id, rest) = paths.split_first().expect("non-empty path tuple");
            match gate_doc_writer(&pool, &session, doc_id).await? {
                U2Gate::NotFound => return Ok(Json(ActionResult::error("document not found"))),
                U2Gate::Forbidden => return Err(AppError::Forbidden),
                U2Gate::Allowed => {}
            }
            let value = body
                .get("fieldValue")
                .and_then(|v| v.as_str())
                .map(String::from)
                .unwrap_or_else(|| serde_json::to_string(&body.0).unwrap_or_default());
            upsert_doc_field(&pool, doc_id, &compose_field_path(rest), &value).await?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("docId".to_string(), Value::String(doc_id.to_string())),
                ("fieldName".to_string(), Value::String(compose_field_path(rest))),
                ("updated".to_string(), Value::Bool(true)),
            ])))))
        }
    };
}

// ── 基座层（data/document/{id}，无路径段）写端点 ──

/// POST data/document/{id}：Java ActionCreateWithDocument —— 新增数据，
/// 仅填充不存在的字段 key（跳过已有）。
#[axum::debug_handler]
pub async fn data_document_id_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    write_doc_fields_base(&pool, &session, &id, &body, FieldWriteMode::Create).await
}

/// PUT data/document/{id}：Java ActionUpdateWithDocument —— 更新数据
/// （顶层 key 全量 upsert）。
#[axum::debug_handler]
pub async fn data_document_id_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    write_doc_fields_base(&pool, &session, &id, &body, FieldWriteMode::Update).await
}

/// DELETE data/document/{id}：Java ActionDeleteWithDocument —— 删除文档数据。
#[axum::debug_handler]
pub async fn data_document_id_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_doc_fields_base(&pool, &session, &id).await
}

// 路径深度 0..=7 的写端点族（mockdeletetoget / delete / update / create）。
// 深度 0（仅 {path0}）的 mockdeletetoget 覆盖旧实现；update/create/delete 为新增。
data_path_write_handlers!(
    data_document_id_path0_mockdeletetoget,
    data_document_id_path0_delete,
    data_document_id_path0_update,
    data_document_id_path0_create,
    data_document_id_path0_mockputtopost,
    2
);
data_path_write_handlers!(
    data_document_id_path0_path1_mockdeletetoget,
    data_document_id_path0_path1_delete,
    data_document_id_path0_path1_update,
    data_document_id_path0_path1_create,
    data_document_id_path0_path1_mockputtopost,
    3
);
data_path_write_handlers!(
    data_document_id_path0_path1_path2_mockdeletetoget,
    data_document_id_path0_path1_path2_delete,
    data_document_id_path0_path1_path2_update,
    data_document_id_path0_path1_path2_create,
    data_document_id_path0_path1_path2_mockputtopost,
    4
);
data_path_write_handlers!(
    data_document_id_path0_path1_path2_path3_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_delete,
    data_document_id_path0_path1_path2_path3_update,
    data_document_id_path0_path1_path2_path3_create,
    data_document_id_path0_path1_path2_path3_mockputtopost,
    5
);
data_path_write_handlers!(
    data_document_id_path0_path1_path2_path3_path4_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_path4_delete,
    data_document_id_path0_path1_path2_path3_path4_update,
    data_document_id_path0_path1_path2_path3_path4_create,
    data_document_id_path0_path1_path2_path3_path4_mockputtopost,
    6
);
data_path_write_handlers!(
    data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_path4_path5_delete,
    data_document_id_path0_path1_path2_path3_path4_path5_update,
    data_document_id_path0_path1_path2_path3_path4_path5_create,
    data_document_id_path0_path1_path2_path3_path4_path5_mockputtopost,
    7
);
data_path_write_handlers!(
    data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_delete,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_update,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_create,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost,
    8
);
data_path_write_handlers!(
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_delete,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_update,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_create,
    data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost,
    9
);

#[axum::debug_handler]
pub async fn data_document_id_array_data(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, content, author_id, status, publish_time::text, creator, create_time::text FROM x_cms_data_document WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let data = vec![Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get::<_, Option<String>>("title").unwrap_or_default())),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("authorId".to_string(), Value::String(row.get::<_, Option<String>>("author_id").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                ("publishTime".to_string(), Value::String(row.get::<_, Option<String>>("publish_time").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]))];
            let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
        }
        None => Ok(Json(ActionResult::java_success(Value::Array(vec![]), 0, 0))),
    }
}

/// GET data/document/{id}/mockdeletetoget：DELETE data/document/{id} 的动词别名。
/// Java ActionDeleteWithDocument 语义：删除文档数据（字段行），而非文档实体本身；
/// 需要会话且为文档编辑者（IDOR 门禁）。
#[axum::debug_handler]
pub async fn data_document_id_mockdeletetoget(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_doc_fields_base(&pool, &session, &id).await
}

/// POST data/document/{id}/mockputtopost：PUT data/document/{id} 的动词别名
/// （Java ActionUpdateWithDocument）：顶层 key 全量 upsert 到文档数据字段。
#[axum::debug_handler]
pub async fn data_document_id_mockputtopost(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    write_doc_fields_base(&pool, &session, &id, &body, FieldWriteMode::Update).await
}

#[axum::debug_handler]
pub async fn data_document_id_path0(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, path0)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, field_name, field_value, create_time::text FROM x_cms_data_document_field WHERE doc_id = $1 AND field_name = $2 AND deleted_at::text IS NULL",
            &[&doc_id, &path0],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("docId".to_string(), Value::String(row.get("doc_id"))),
            ("fieldName".to_string(), Value::String(row.get("field_name"))),
            ("fieldValue".to_string(), Value::String(row.get::<_, Option<String>>("field_value").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

// ─── design_* / document_cipher_* stubs ─────────────────────────────────────

#[axum::debug_handler]
pub async fn design_appdict_list_appInfo_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_surface_appdict", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn design_appdict_list_paging_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_surface_appdict", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn design_appdict_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time::text FROM x_cms_surface_appdict WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
                ("appDictFlag".to_string(), Value::String(row.get::<_, Option<String>>("app_dict_flag").unwrap_or_default())),
                ("pathLevels".to_string(), Value::String(row.get::<_, Option<String>>("path_levels").unwrap_or_default())),
                ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("appdict not found"))),
    }
}

#[axum::debug_handler]
pub async fn design_appdict_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_surface_appdict SET deleted_at = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time::text FROM x_cms_surface_appdict WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
                ("appDictFlag".to_string(), Value::String(row.get::<_, Option<String>>("app_dict_flag").unwrap_or_default())),
                ("pathLevels".to_string(), Value::String(row.get::<_, Option<String>>("path_levels").unwrap_or_default())),
                ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("appdict not found"))),
    }
}

#[axum::debug_handler]
pub async fn design_appdict_id_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data_value = body.get("dataValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_surface_appdict (id, data_value) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET data_value = $2",
            &[&id, &data_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time::text FROM x_cms_surface_appdict WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
                ("appDictFlag".to_string(), Value::String(row.get::<_, Option<String>>("app_dict_flag").unwrap_or_default())),
                ("pathLevels".to_string(), Value::String(row.get::<_, Option<String>>("path_levels").unwrap_or_default())),
                ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("appdict not found"))),
    }
}

#[axum::debug_handler]
pub async fn designer_search(
    pool: Extension<Pool>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let keyword = params.get("keyword").cloned().unwrap_or_default();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time::text FROM x_cms_surface_appdict WHERE app_info_flag ILIKE $1 AND deleted_at::text IS NULL",
            &[&format!("%{}%", keyword)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get::<_, Option<String>>("app_dict_flag").unwrap_or_default())),
            ("pathLevels".to_string(), Value::String(row.get::<_, Option<String>>("path_levels").unwrap_or_default())),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
            ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn document_cipher_filter_list_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_document_cipher", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn document_cipher_filter_list_page_size_size_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_document_cipher", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn document_cipher_publish_content(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let cipher_text = body.get("cipherText").and_then(|v| v.as_str()).unwrap_or("");
    let person_id = body.get("personId").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_document_cipher (doc_id, cipher_text, person_id) VALUES ($1, $2, $3) ON CONFLICT (doc_id) DO UPDATE SET cipher_text = $2, person_id = $3, create_time = NOW()",
            &[&id, &cipher_text, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, cipher_text, person_id, create_time::text FROM x_cms_document_cipher WHERE doc_id = $1 AND person_id = $2 AND deleted_at::text IS NULL",
            &[&id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("cipherText".to_string(), Value::String(row.get::<_, Option<String>>("cipher_text").unwrap_or_default())),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("permission not found"))),
    }
}

#[axum::debug_handler]
pub async fn document_cipher_publish_content_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let cipher_text = body.get("cipherText").and_then(|v| v.as_str()).unwrap_or("");
    let person_id = body.get("personId").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_document_cipher (doc_id, cipher_text, person_id) VALUES ($1, $2, $3) ON CONFLICT (doc_id) DO UPDATE SET cipher_text = $2, person_id = $3, create_time = NOW()",
            &[&id, &cipher_text, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, cipher_text, person_id, create_time::text FROM x_cms_document_cipher WHERE doc_id = $1 AND person_id = $2 AND deleted_at::text IS NULL",
            &[&id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("cipherText".to_string(), Value::String(row.get::<_, Option<String>>("cipher_text").unwrap_or_default())),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("permission not found"))),
    }
}

#[axum::debug_handler]
pub async fn document_cipher_id_permission_read_person_person(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, person_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, cipher_text, person_id, create_time::text FROM x_cms_document_cipher WHERE doc_id = $1 AND person_id = $2 AND deleted_at::text IS NULL",
            &[&doc_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("cipherText".to_string(), Value::String(row.get::<_, Option<String>>("cipher_text").unwrap_or_default())),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("permission not found"))),
    }
}

#[axum::debug_handler]
pub async fn document_cipher_id_persist_view_record(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let person_id = body.get("personId").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_viewrecord (doc_id, view_id, record_data, person_id) VALUES ($1, $2, $3, $4)",
            &[&id, &"", &"{}", &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT doc_id, view_id, record_data, person_id FROM x_cms_viewrecord WHERE doc_id = $1 AND person_id = $2",
            &[&id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("viewId".to_string(), Value::String(row.get::<_, Option<String>>("view_id").unwrap_or_default())),
                ("recordData".to_string(), Value::String(row.get::<_, Option<String>>("record_data").unwrap_or_default())),
                ("personId".to_string(), Value::String(row.get("person_id"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("view record not found"))),
    }
}

// ─── file* / fileinfo_* stubs ───────────────────────────────────────────────

#[axum::debug_handler]
pub async fn file_list_appInfo_appInfoFlag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_file", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn file_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_file", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn file_list_id_prev_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_file", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn file_flag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_file", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn file_flag_appInfo_appInfoFlag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_file", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn file_flag_appInfo_appInfoFlag_content(
    pool: Extension<Pool>,
    axum::extract::Path((app_info_flag, app_info_flag2)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, size, content_type, content_base64, creator, create_time::text FROM x_cms_file WHERE app_id = $1 AND deleted_at::text IS NULL LIMIT 1",
            &[&app_info_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
                ("contentBase64".to_string(), Value::String(row.get::<_, Option<String>>("content_base64").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn file_flag_appInfo_appInfoFlag_download(
    pool: Extension<Pool>,
    axum::extract::Path(app_info_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, size, content_type, creator, create_time::text FROM x_cms_file WHERE app_id = $1 AND deleted_at::text IS NULL LIMIT 1",
            &[&app_info_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn file_flag_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_file SET deleted_at = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, size, content_type, content_base64, creator, create_time::text FROM x_cms_file WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get::<_, Option<String>>("app_id").unwrap_or_default())),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
                ("contentBase64".to_string(), Value::String(row.get::<_, Option<String>>("content_base64").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn file_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, size, content_type, creator, create_time::text FROM x_cms_file WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn file_id_content(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, content_base64, content_type FROM x_cms_file WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("contentBase64".to_string(), Value::String(row.get::<_, Option<String>>("content_base64").unwrap_or_default())),
                ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn file_id_download(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, size, content_type, content_base64 FROM x_cms_file WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
                ("contentBase64".to_string(), Value::String(row.get::<_, Option<String>>("content_base64").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn file_id_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let content_base64 = body.get("contentBase64").and_then(|v| v.as_str()).unwrap_or("");
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let row = client
        .query_one(
            "INSERT INTO x_cms_file (id, name, content_base64, content_type, size) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET name = $2, content_base64 = $3, content_type = $4, size = $5 RETURNING *",
            &[&id, &name, &content_base64, &content_type, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

#[axum::debug_handler]
pub async fn file_id_upload(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let content_base64 = body.get("contentBase64").and_then(|v| v.as_str()).unwrap_or("");
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let row = client
        .query_one(
            "INSERT INTO x_cms_file (id, name, content_base64, content_type, size) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET name = $2, content_base64 = $3, content_type = $4, size = $5 RETURNING *",
            &[&id, &name, &content_base64, &content_type, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

#[axum::debug_handler]
pub async fn anonymous_fileinfo_download_document_id(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time::text FROM x_cms_fileinfo WHERE doc_id = $1 AND deleted_at::text IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("docId".to_string(), Value::String(row.get("doc_id"))),
            ("fileId".to_string(), Value::String(row.get::<_, Option<String>>("file_id").unwrap_or_default())),
            ("originalName".to_string(), Value::String(row.get::<_, Option<String>>("original_name").unwrap_or_default())),
            ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
            ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
            ("uploadPerson".to_string(), Value::String(row.get::<_, Option<String>>("upload_person").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn anonymous_fileinfo_download_document_id_stream(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time::text FROM x_cms_fileinfo WHERE doc_id = $1 AND deleted_at::text IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("docId".to_string(), Value::String(row.get("doc_id"))),
            ("fileId".to_string(), Value::String(row.get::<_, Option<String>>("file_id").unwrap_or_default())),
            ("originalName".to_string(), Value::String(row.get::<_, Option<String>>("original_name").unwrap_or_default())),
            ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
            ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
            ("uploadPerson".to_string(), Value::String(row.get::<_, Option<String>>("upload_person").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn fileinfo_batch_download_doc_docId_site_site(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, site)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time::text FROM x_cms_fileinfo WHERE doc_id = $1 AND deleted_at::text IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("docId".to_string(), Value::String(row.get("doc_id"))),
            ("fileId".to_string(), Value::String(row.get::<_, Option<String>>("file_id").unwrap_or_default())),
            ("originalName".to_string(), Value::String(row.get::<_, Option<String>>("original_name").unwrap_or_default())),
            ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
            ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn fileinfo_copy_to_doc_docId(
    pool: Extension<Pool>,
    axum::extract::Path((file_id, doc_id)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let original_name = body.get("originalName").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let row = client
        .query_one(
            "INSERT INTO x_cms_fileinfo (id, doc_id, file_id, original_name, size, content_type) VALUES (gen_random_uuid()::text, $1, $2, $3, $4, $5) RETURNING *",
            &[&doc_id, &file_id, &original_name, &size, &content_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

#[axum::debug_handler]
pub async fn fileinfo_download_document_id(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time::text FROM x_cms_fileinfo WHERE doc_id = $1 AND deleted_at::text IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("docId".to_string(), Value::String(row.get("doc_id"))),
            ("fileId".to_string(), Value::String(row.get::<_, Option<String>>("file_id").unwrap_or_default())),
            ("originalName".to_string(), Value::String(row.get::<_, Option<String>>("original_name").unwrap_or_default())),
            ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
            ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
            ("uploadPerson".to_string(), Value::String(row.get::<_, Option<String>>("upload_person").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn fileinfo_download_document_id_stream(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time::text FROM x_cms_fileinfo WHERE doc_id = $1 AND deleted_at::text IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("docId".to_string(), Value::String(row.get("doc_id"))),
            ("fileId".to_string(), Value::String(row.get::<_, Option<String>>("file_id").unwrap_or_default())),
            ("originalName".to_string(), Value::String(row.get::<_, Option<String>>("original_name").unwrap_or_default())),
            ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
            ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn fileinfo_download_transfer_flag_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time::text FROM x_cms_fileinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("docId".to_string(), Value::String(row.get("doc_id"))),
            ("fileId".to_string(), Value::String(row.get::<_, Option<String>>("file_id").unwrap_or_default())),
            ("originalName".to_string(), Value::String(row.get::<_, Option<String>>("original_name").unwrap_or_default())),
            ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
            ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn fileinfo_edit_id_doc_docId(
    pool: Extension<Pool>,
    axum::extract::Path((file_id, doc_id)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let original_name = body.get("originalName").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let row = client
        .query_opt(
            "UPDATE x_cms_fileinfo SET original_name = $1, size = $2, content_type = $3 WHERE file_id = $4 AND doc_id = $5 RETURNING *",
            &[&original_name, &size, &content_type, &file_id, &doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn fileinfo_edit_id_doc_docId_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((file_id, doc_id)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let original_name = body.get("originalName").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let row = client
        .query_opt(
            "UPDATE x_cms_fileinfo SET original_name = $1, size = $2, content_type = $3 WHERE file_id = $4 AND doc_id = $5 RETURNING *",
            &[&original_name, &size, &content_type, &file_id, &doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn fileinfo_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_fileinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn fileinfo_list_document_documentId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_fileinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn fileinfo_list_filter(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_fileinfo", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn fileinfo_replace_to_doc_docId(
    pool: Extension<Pool>,
    axum::extract::Path((file_id, doc_id)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let original_name = body.get("originalName").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let row = client
        .query_opt(
            "UPDATE x_cms_fileinfo SET original_name = $1, size = $2, content_type = $3 WHERE file_id = $4 AND doc_id = $5 RETURNING *",
            &[&original_name, &size, &content_type, &file_id, &doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn fileinfo_update_document_docId_attachment_id(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, attachment_id)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let original_name = body.get("originalName").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let row = client
        .query_opt(
            "UPDATE x_cms_fileinfo SET original_name = $1, size = $2, content_type = $3 WHERE doc_id = $4 AND file_id = $5 RETURNING *",
            &[&original_name, &size, &content_type, &doc_id, &attachment_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn fileinfo_update_document_docId_attachment_id_callback_callback(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, attachment_id, callback)): axum::extract::Path<(String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let original_name = body.get("originalName").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let row = client
        .query_opt(
            "UPDATE x_cms_fileinfo SET original_name = $1, size = $2, content_type = $3 WHERE doc_id = $4 AND file_id = $5 RETURNING *",
            &[&original_name, &size, &content_type, &doc_id, &attachment_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn fileinfo_update_id_content(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let original_name = body.get("originalName").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let row = client
        .query_opt(
            "UPDATE x_cms_fileinfo SET original_name = $1, size = $2, content_type = $3 WHERE id = $4 RETURNING *",
            &[&original_name, &size, &content_type, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn fileinfo_upload_doc_docId_save_as_flag(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, save_as)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let original_name = body.get("originalName").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let row = client
        .query_one(
            "INSERT INTO x_cms_fileinfo (id, doc_id, original_name, size, content_type) VALUES (gen_random_uuid()::text, $1, $2, $3, $4) RETURNING *",
            &[&doc_id, &original_name, &size, &content_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

#[axum::debug_handler]
pub async fn fileinfo_upload_document_docId(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let original_name = body.get("originalName").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let row = client
        .query_one(
            "INSERT INTO x_cms_fileinfo (id, doc_id, original_name, size, content_type) VALUES (gen_random_uuid()::text, $1, $2, $3, $4) RETURNING *",
            &[&doc_id, &original_name, &size, &content_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

#[axum::debug_handler]
pub async fn fileinfo_upload_document_docId_callback_callback(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _callback)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let original_name = body.get("originalName").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let row = client
        .query_one(
            "INSERT INTO x_cms_fileinfo (id, doc_id, original_name, size, content_type) VALUES (gen_random_uuid()::text, $1, $2, $3, $4) RETURNING *",
            &[&doc_id, &original_name, &size, &content_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

#[axum::debug_handler]
pub async fn fileinfo_upload_with_url(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let original_name = body.get("originalName").and_then(|v| v.as_str()).unwrap_or("");
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let content_type = body.get("contentType").and_then(|v| v.as_str()).unwrap_or("");
    let row = client
        .query_one(
            "INSERT INTO x_cms_fileinfo (id, doc_id, original_name, size, content_type) VALUES (gen_random_uuid()::text, $1, $2, $3, $4) RETURNING *",
            &[&doc_id, &original_name, &size, &content_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

#[axum::debug_handler]
pub async fn fileinfo_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time::text FROM x_cms_fileinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("fileId".to_string(), Value::String(row.get::<_, Option<String>>("file_id").unwrap_or_default())),
                ("originalName".to_string(), Value::String(row.get::<_, Option<String>>("original_name").unwrap_or_default())),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
                ("uploadPerson".to_string(), Value::String(row.get::<_, Option<String>>("upload_person").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn fileinfo_id_binary_base64_size(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, original_name, size, content_type FROM x_cms_fileinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("originalName".to_string(), Value::String(row.get::<_, Option<String>>("original_name").unwrap_or_default())),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn fileinfo_id_doc_docId_change_seqnumber_seqNumber(
    pool: Extension<Pool>,
    axum::extract::Path((file_id, doc_id, seq_number)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "UPDATE x_cms_fileinfo SET create_time = NOW() WHERE file_id = $1 AND doc_id = $2 RETURNING *",
            &[&file_id, &doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn fileinfo_id_document_documentId(
    pool: Extension<Pool>,
    axum::extract::Path((file_id, doc_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, file_id, original_name, size, content_type FROM x_cms_fileinfo WHERE file_id = $1 AND doc_id = $2 AND deleted_at::text IS NULL",
            &[&file_id, &doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("fileId".to_string(), Value::String(row.get::<_, Option<String>>("file_id").unwrap_or_default())),
                ("originalName".to_string(), Value::String(row.get::<_, Option<String>>("original_name").unwrap_or_default())),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn fileinfo_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "UPDATE x_cms_fileinfo SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING *",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn fileinfo_id_online_info(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time::text FROM x_cms_fileinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("fileId".to_string(), Value::String(row.get::<_, Option<String>>("file_id").unwrap_or_default())),
                ("originalName".to_string(), Value::String(row.get::<_, Option<String>>("original_name").unwrap_or_default())),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
                ("uploadPerson".to_string(), Value::String(row.get::<_, Option<String>>("upload_person").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

#[axum::debug_handler]
pub async fn fileinfo_id_preview_pdf(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, file_id, original_name, size, content_type FROM x_cms_fileinfo WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("fileId".to_string(), Value::String(row.get::<_, Option<String>>("file_id").unwrap_or_default())),
                ("originalName".to_string(), Value::String(row.get::<_, Option<String>>("original_name").unwrap_or_default())),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("contentType".to_string(), Value::String(row.get::<_, Option<String>>("content_type").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("fileinfo not found"))),
    }
}

// ─── form_* / form_v2_* stubs ───────────────────────────────────────────────

#[axum::debug_handler]
pub async fn form_filter_list_id_next_count_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn form_filter_list_id_next_count_app_appId_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn form_filter_list_id_prev_count_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn form_filter_list_id_prev_count_app_appId_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn form_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn form_list_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn form_list_formfield_appInfo_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form_field", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn form_list_id_formfield(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form_field", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn anonymous_form_v2_lookup_document_docId(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status FROM x_cms_form_v2 WHERE id = (SELECT form_id FROM x_cms_data_document WHERE id = $1 AND deleted_at::text IS NULL) AND deleted_at::text IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("definition".to_string(), Value::String(row.get::<_, Option<String>>("definition").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

#[axum::debug_handler]
pub async fn anonymous_form_v2_lookup_document_docId_mobile(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status FROM x_cms_form_v2 WHERE id = (SELECT form_id FROM x_cms_data_document WHERE id = $1 AND deleted_at::text IS NULL) AND deleted_at::text IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("definition".to_string(), Value::String(row.get::<_, Option<String>>("definition").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

#[axum::debug_handler]
pub async fn anonymous_form_v2_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status FROM x_cms_form_v2 WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("definition".to_string(), Value::String(row.get::<_, Option<String>>("definition").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

#[axum::debug_handler]
pub async fn anonymous_form_v2_id_mobile(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status FROM x_cms_form_v2 WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("definition".to_string(), Value::String(row.get::<_, Option<String>>("definition").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

#[axum::debug_handler]
pub async fn anonymous_form_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status FROM x_cms_form WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("definition".to_string(), Value::String(row.get::<_, Option<String>>("definition").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

#[axum::debug_handler]
pub async fn form_formFlag_appinfo_appFlag(
    pool: Extension<Pool>,
    axum::extract::Path(app_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_id, name, definition, status, creator, create_time::text FROM x_cms_form WHERE app_id = $1 AND deleted_at::text IS NULL",
            &[&app_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appId".to_string(), Value::String(row.get("app_id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("definition".to_string(), Value::String(row.get::<_, Option<String>>("definition").unwrap_or_default())),
            ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn form_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status, creator, create_time::text FROM x_cms_form WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("definition".to_string(), Value::String(row.get::<_, Option<String>>("definition").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

#[axum::debug_handler]
pub async fn form_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "UPDATE x_cms_form SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING *",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

#[axum::debug_handler]
pub async fn form_id_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let definition = body.get("definition").and_then(|v| v.as_str()).unwrap_or("");
    let status = body.get("status").and_then(|v| v.as_str()).unwrap_or("draft");
    let row = client
        .query_one(
            "INSERT INTO x_cms_form (id, name, definition, status) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET name = $2, definition = $3, status = $4 RETURNING *",
            &[&id, &name, &definition, &status],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

#[axum::debug_handler]
pub async fn form_v2_lookup_document_docId(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status, creator, create_time::text FROM x_cms_form_v2 WHERE id = (SELECT form_id FROM x_cms_data_document WHERE id = $1 AND deleted_at::text IS NULL) AND deleted_at::text IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("definition".to_string(), Value::String(row.get::<_, Option<String>>("definition").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

#[axum::debug_handler]
pub async fn form_v2_lookup_document_docId_mobile(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status, creator, create_time::text FROM x_cms_form_v2 WHERE id = (SELECT form_id FROM x_cms_data_document WHERE id = $1 AND deleted_at::text IS NULL) AND deleted_at::text IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("definition".to_string(), Value::String(row.get::<_, Option<String>>("definition").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

#[axum::debug_handler]
pub async fn form_v2_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form_v2", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn form_v2_id_mobile(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status, creator, create_time::text FROM x_cms_form_v2 WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("definition".to_string(), Value::String(row.get::<_, Option<String>>("definition").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

#[axum::debug_handler]
pub async fn formversion_list_form_formId(
    pool: Extension<Pool>,
    axum::extract::Path(form_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form_v2", &format!("deleted_at IS NULL AND id = '{}'", form_id), &[]).await
}

#[axum::debug_handler]
pub async fn formversion_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status, creator, create_time::text FROM x_cms_form_v2 WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("definition".to_string(), Value::String(row.get::<_, Option<String>>("definition").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("form version not found"))),
    }
}

// ─── log_* stubs ────────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn log_filter_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_log", "", &[]).await
}

#[axum::debug_handler]
pub async fn log_filter_list_id_prev_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_log", "", &[]).await
}

#[axum::debug_handler]
pub async fn log_list_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_log", "", &[]).await
}

#[axum::debug_handler]
pub async fn log_list_category_categoryId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_log", "", &[]).await
}

#[axum::debug_handler]
pub async fn log_list_document_documentId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_log", "", &[]).await
}

#[axum::debug_handler]
pub async fn log_list_filter_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_log", "", &[]).await
}

#[axum::debug_handler]
pub async fn log_list_level_operationLevel(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_log", "", &[]).await
}

#[axum::debug_handler]
pub async fn log_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_log", "", &[]).await
}

// ─── output_* stubs ─────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn output_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_output", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn output_appInfoFlag_select(
    pool: Extension<Pool>,
    axum::extract::Path(app_info_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_id, name, config, creator, create_time::text FROM x_cms_output WHERE app_id = $1 AND deleted_at::text IS NULL",
            &[&app_info_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appId".to_string(), Value::String(row.get("app_id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("config".to_string(), Value::String(row.get::<_, Option<String>>("config").unwrap_or_default())),
            ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn output_appInfoFlag_select_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(app_info_flag): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let config = body.get("config").and_then(|v| v.as_str()).unwrap_or("");
    let row = client
        .query_one(
            "INSERT INTO x_cms_output (id, app_id, name, config) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = $2, config = $3 RETURNING *",
            &[&app_info_flag, &name, &config],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

// ─── permission_* stubs ─────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn permission_appInfo_id_manageable(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'manage'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_appInfo_id_managers(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'manager'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_appInfo_id_publishers(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'publisher'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_appInfo_id_viewers(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'viewer'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_category_id_managers(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'manager'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_category_id_publishers(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'publisher'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_category_id_viewers(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'viewer'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_categoryInfo_id_manageable(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'manage'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_management_refresh_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute("UPDATE x_cms_permission SET deleted_at = NOW() WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Number(serde_json::Number::from(affected as i64)))]),
    ))))
}

#[axum::debug_handler]
pub async fn permission_management_refresh_category_categoryId(
    pool: Extension<Pool>,
    axum::extract::Path(category_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute("UPDATE x_cms_permission SET deleted_at = NOW() WHERE category_id = $1 AND deleted_at IS NULL", &[&category_id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Number(serde_json::Number::from(affected as i64)))]),
    ))))
}

#[axum::debug_handler]
pub async fn permission_manager_appInfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'manager'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_manager_categoryInfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'manager'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_publisher_appInfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'publisher'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_publisher_categoryInfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'publisher'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_viewer_appInfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'viewer'", &[]).await
}

#[axum::debug_handler]
pub async fn permission_viewer_categoryInfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'viewer'", &[]).await
}

// ─── review_* / script_* stubs ──────────────────────────────────────────────

#[axum::debug_handler]
pub async fn review_v2_search(
    pool: Extension<Pool>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let keyword = params.get("keyword").cloned().unwrap_or_default();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, person_id, create_time::text FROM x_cms_comment WHERE content ILIKE $1 AND deleted_at::text IS NULL",
            &[&format!("%{}%", keyword)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("docId".to_string(), Value::String(row.get("doc_id"))),
            ("personId".to_string(), Value::String(row.get::<_, Option<String>>("person_id").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn script_list_app_appId_name_name(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn script_list_app_flag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn script_list_manager(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn script_list_paging_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn script_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn script_list_id_prev_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn script_flag_appInfo_appInfoFlag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn script_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn script_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_script", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn script_id_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let script_content = body.get("scriptContent").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_script (id, name, script_content) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = $2, script_content = $3",
            &[&id, &name, &script_content],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, script_content, creator, create_time::text FROM x_cms_script WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("scriptContent".to_string(), Value::String(row.get::<_, Option<String>>("script_content").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("script not found"))),
    }
}

#[axum::debug_handler]
pub async fn script_uniqueName_app_flag(
    pool: Extension<Pool>,
    axum::extract::Path((app_flag, unique_name)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, unique_name, script_content, imported, creator, create_time::text FROM x_cms_script WHERE app_id = $1 AND unique_name = $2 AND deleted_at::text IS NULL",
            &[&app_flag, &unique_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("uniqueName".to_string(), Value::String(row.get::<_, Option<String>>("unique_name").unwrap_or_default())),
                ("scriptContent".to_string(), Value::String(row.get::<_, Option<String>>("script_content").unwrap_or_default())),
                ("imported".to_string(), Value::Bool(row.get("imported"))),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("script not found"))),
    }
}

#[axum::debug_handler]
pub async fn script_uniqueName_app_flag_imported(
    pool: Extension<Pool>,
    axum::extract::Path((app_flag, unique_name)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, unique_name, script_content, imported, creator, create_time::text FROM x_cms_script WHERE app_id = $1 AND unique_name = $2 AND imported = true AND deleted_at::text IS NULL",
            &[&app_flag, &unique_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("uniqueName".to_string(), Value::String(row.get::<_, Option<String>>("unique_name").unwrap_or_default())),
                ("scriptContent".to_string(), Value::String(row.get::<_, Option<String>>("script_content").unwrap_or_default())),
                ("imported".to_string(), Value::Bool(row.get("imported"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("script not found"))),
    }
}

#[axum::debug_handler]
pub async fn scriptversion_list_script_scriptId(
    pool: Extension<Pool>,
    axum::extract::Path(script_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_script", &format!("deleted_at IS NULL AND id = '{}'", script_id), &[]).await
}

#[axum::debug_handler]
pub async fn scriptversion_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status, creator, create_time::text FROM x_cms_form_v2 WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("script version not found"))),
    }
}

// ─── searchfilter_* stubs ───────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn searchfilter_list_archive_filter_category_categoryId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_searchfilter", "deleted_at IS NULL AND filter_type = 'archive'", &[]).await
}

#[axum::debug_handler]
pub async fn searchfilter_list_draft_filter_category_categoryId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_searchfilter", "deleted_at IS NULL AND filter_type = 'draft'", &[]).await
}

#[axum::debug_handler]
pub async fn searchfilter_list_publish_filter_category_categoryId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_searchfilter", "deleted_at IS NULL AND filter_type = 'publish'", &[]).await
}

// ─── surface_appdict 家族（Java AppDictAction / AppDictAnonymousAction 对齐）──
//
// Java 语义：surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}[/{path0}.../]{data}
// 是栏目级数据字典 JSON 树的读写。本端口用 x_cms_surface_appdict 行模型承载：
// 每行 = (app_dict_flag, app_info_flag, path_levels TEXT[], data_value)。
// 读端点按 path_levels 位置前缀精确匹配（返回子树行）；写端点按整路径精确匹配。
// 动词别名（mockputtopost/mockdeletetoget）与主动词共用同一 handler，与 Java
// 复用同一 Action 的做法一致。

fn appdict_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    let path_levels: Vec<String> = row
        .get::<_, Option<Vec<String>>>("path_levels")
        .unwrap_or_default();
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
        ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
        (
            "pathLevels".to_string(),
            serde_json::to_value(path_levels).unwrap_or(Value::Array(vec![])),
        ),
        (
            "dataValue".to_string(),
            Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default()),
        ),
        (
            "creator".to_string(),
            Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
        ),
        (
            "createTime".to_string(),
            Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default()),
        ),
    ]))
}

async fn appdict_data_get(
    pool: &Pool,
    app_dict_flag: &str,
    app_info_flag: &str,
    paths: &[String],
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    // $1 dict、$2 app、$3 深度下界、$4.. 各级路径（位置前缀匹配）
    let mut where_clause =
        String::from("WHERE app_dict_flag = $1 AND app_info_flag = $2 AND deleted_at IS NULL");
    if !paths.is_empty() {
        // 占位符序：$1 dict、$2 app、$3 depth、$4.. 各级路径段
        where_clause.push_str(&format!(
            " AND cardinality(path_levels) >= ${}",
            paths.len() + 2
        ));
        for i in 0..paths.len() {
            where_clause.push_str(&format!(" AND path_levels[{}] = ${}", i + 1, i + 4));
        }
    }
    let sql = format!(
        "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time::text \
         FROM x_cms_surface_appdict {}",
        where_clause
    );
    let mut params: Vec<&(dyn ToSql + Sync)> = vec![&app_dict_flag, &app_info_flag];
    // cardinality() 返回 int4：必须绑 i32（i64 会触发 WrongType）
    let depth = paths.len() as i32;
    if !paths.is_empty() {
        params.push(&depth);
    }
    for p in paths {
        params.push(p);
    }
    let rows = client
        .query(sql.as_str(), &params[..])
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(appdict_row_to_value).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

/// 写门禁：目标字典已有行时要求会话用户为其 creator 或管理员；
/// 空字典允许创建。DB 不可用时 fail-closed（Internal 拒绝）。
async fn gate_appdict_writer(
    pool: &Pool,
    session: &shared::session::Session,
    app_dict_flag: &str,
    app_info_flag: &str,
) -> Result<U2Gate, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT creator FROM x_cms_surface_appdict \
             WHERE app_dict_flag = $1 AND app_info_flag = $2 AND deleted_at IS NULL \
             ORDER BY create_time LIMIT 1",
            &[&app_dict_flag, &app_info_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        None => Ok(U2Gate::Allowed),
        Some(r) => {
            let owner = r.get::<_, Option<String>>("creator").unwrap_or_default();
            if shared::middleware::is_admin(pool, &session.person_unique).await
                || (!owner.is_empty() && owner == session.person_unique)
            {
                Ok(U2Gate::Allowed)
            } else {
                Ok(U2Gate::Forbidden)
            }
        }
    }
}

#[derive(PartialEq)]
enum AppdictWriteMode {
    /// PUT / mockputtopost：upsert 精确路径行
    Upsert,
    /// POST create：仅当该路径行不存在时插入，否则报 already exists
    InsertIfAbsent,
    /// DELETE / mockdeletetoget：软删精确路径行
    Remove,
}

async fn appdict_data_write(
    pool: &Pool,
    session: &shared::session::Session,
    app_dict_flag: &str,
    app_info_flag: &str,
    paths: &[String],
    data_value: &str,
    mode: AppdictWriteMode,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match gate_appdict_writer(pool, session, app_dict_flag, app_info_flag).await? {
        U2Gate::Forbidden => return Err(AppError::Forbidden),
        U2Gate::NotFound | U2Gate::Allowed => {}
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    // cardinality() 返回 int4：必须绑 i32（i64 会触发 WrongType）
    let depth = paths.len() as i32;

    // 精确路径定位：$1 dict、$2 app、$3 深度、$4.. 各级路径
    let mut target = String::from(
        "app_dict_flag = $1 AND app_info_flag = $2 AND cardinality(path_levels) = $3",
    );
    for i in 0..paths.len() {
        target.push_str(&format!(" AND path_levels[{}] = ${}", i + 1, i + 4));
    }

    // 绑定序：dict($1)、app($2)、depth($3)、p0..pn($4..)，写模式追加 value/creator
    let person = session.person_unique.clone();
    let mut params: Vec<&(dyn ToSql + Sync)> = vec![&app_dict_flag, &app_info_flag];
    params.push(&depth);
    for p in paths {
        params.push(p);
    }

    match mode {
        AppdictWriteMode::Remove => {
            let sql = format!(
                "UPDATE x_cms_surface_appdict SET deleted_at = NOW() \
                 WHERE {} AND deleted_at IS NULL",
                target
            );
            let deleted = client
                .execute(sql.as_str(), &params[..])
                .await
                .map_err(|_| AppError::Internal)?;
            if deleted == 0 {
                return Ok(Json(ActionResult::error("appdict data not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
        AppdictWriteMode::Upsert | AppdictWriteMode::InsertIfAbsent => {
            let value_idx = params.len() + 1;
            let person_idx = params.len() + 2;
            params.push(&data_value);
            params.push(&person);
            let update_sql = format!(
                "UPDATE x_cms_surface_appdict SET data_value = ${}, creator = COALESCE(NULLIF(creator, ''), ${}) \
                 WHERE {} AND deleted_at IS NULL",
                value_idx, person_idx, target
            );
            let updated = client
                .execute(update_sql.as_str(), &params[..])
                .await
                .map_err(|_| AppError::Internal)?;

            if mode == AppdictWriteMode::InsertIfAbsent && updated > 0 {
                return Ok(Json(ActionResult::error("appdict data already exists")));
            }

            let mut created = false;
            if updated == 0 {
                // 行不存在则插入；NOT EXISTS 防并发重复
                let array_literal = if paths.is_empty() {
                    "'{}'".to_string()
                } else {
                    format!(
                        "ARRAY[{}]",
                        (4..=paths.len() + 3)
                            .map(|i| format!("${}", i))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };
                let insert_sql = format!(
                    "INSERT INTO x_cms_surface_appdict (id, app_dict_flag, app_info_flag, path_levels, data_value, creator) \
                     SELECT gen_random_uuid()::text, $1::varchar, $2::varchar, {}::text[], ${}::text, ${}::varchar \
                     WHERE NOT EXISTS (SELECT 1 FROM x_cms_surface_appdict WHERE {})",
                    array_literal, value_idx, person_idx, target
                );
                let inserted = client
                    .execute(insert_sql.as_str(), &params[..])
                    .await
                    .map_err(|_| AppError::Internal)?;
                created = inserted > 0;
                if !created && mode == AppdictWriteMode::InsertIfAbsent {
                    return Ok(Json(ActionResult::error("appdict data already exists")));
                }
            }

            let key = if created { "created" } else { "updated" };
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("appDictFlag".to_string(), Value::String(app_dict_flag.to_string())),
                    ("appInfoFlag".to_string(), Value::String(app_info_flag.to_string())),
                    (
                        "pathLevels".to_string(),
                        serde_json::to_value(paths).unwrap_or(Value::Array(vec![])),
                    ),
                    (key.to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

/// body 取值契约：优先 "dataValue" 字符串（既有端口约定），
/// 否则将整个 body 序列化存储（可往返读回）。
fn appdict_body_value(body: &Value) -> String {
    body.get("dataValue")
        .and_then(|v| v.as_str())
        .map(String::from)
        .unwrap_or_else(|| serde_json::to_string(body).unwrap_or_default())
}

/// 每个路径深度的端点族：匿名读、认证读、PUT(upsert)、POST(insert-if-absent)、
/// DELETE(remove)。mockputtopost / mockdeletetoget 路由复用 put/delete handler。
macro_rules! appdict_depth_family {
    ($arity:literal; $anon_read:ident, $auth_read:ident, $put:ident, $post:ident, $del:ident) => {
        #[axum::debug_handler]
        pub async fn $anon_read(
            pool: Extension<Pool>,
            axum::extract::Path(paths): axum::extract::Path<[String; $arity]>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            appdict_data_get(&pool, &paths[0], &paths[1], &paths[2..]).await
        }

        #[axum::debug_handler]
        pub async fn $auth_read(
            pool: Extension<Pool>,
            axum::extract::Path(paths): axum::extract::Path<[String; $arity]>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            appdict_data_get(&pool, &paths[0], &paths[1], &paths[2..]).await
        }

        #[axum::debug_handler]
        pub async fn $put(
            pool: Extension<Pool>,
            session: Extension<shared::session::Session>,
            axum::extract::Path(paths): axum::extract::Path<[String; $arity]>,
            body: axum::extract::Json<Value>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            let value = appdict_body_value(&body);
            appdict_data_write(&pool, &session, &paths[0], &paths[1], &paths[2..], &value, AppdictWriteMode::Upsert)
                .await
        }

        #[axum::debug_handler]
        pub async fn $post(
            pool: Extension<Pool>,
            session: Extension<shared::session::Session>,
            axum::extract::Path(paths): axum::extract::Path<[String; $arity]>,
            body: axum::extract::Json<Value>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            let value = appdict_body_value(&body);
            appdict_data_write(
                &pool,
                &session,
                &paths[0],
                &paths[1],
                &paths[2..],
                &value,
                AppdictWriteMode::InsertIfAbsent,
            )
            .await
        }

        #[axum::debug_handler]
        pub async fn $del(
            pool: Extension<Pool>,
            session: Extension<shared::session::Session>,
            axum::extract::Path(paths): axum::extract::Path<[String; $arity]>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            appdict_data_write(&pool, &session, &paths[0], &paths[1], &paths[2..], "", AppdictWriteMode::Remove)
                .await
        }
    };
}

// ── 基座层（无路径段） ──

/// GET anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}
#[axum::debug_handler]
pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time::text              FROM x_cms_surface_appdict              WHERE app_dict_flag = $1 AND app_info_flag = $2 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(appdict_row_to_value).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

/// GET surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}
#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag(
        pool,
        axum::extract::Path((app_dict_flag, app_info_flag)),
    )
    .await
}

/// GET anonymous/surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/data
#[axum::debug_handler]
pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    appdict_data_get(&pool, &app_dict_flag, &app_info_flag, &[]).await
}

/// GET surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}/data
#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    appdict_data_get(&pool, &app_dict_flag, &app_info_flag, &[]).await
}

/// PUT surface/appdict/{appDictFlag}/appInfo/{appInfoFlag}：
/// Java ActionUpdate —— 更新字典根数据；POST .../mockputtopost 为其动词别名，
/// 与 Java ActionUpdateMockPutToPost 复用 ActionUpdate 一致。
#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((app_dict_flag, app_info_flag)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let value = appdict_body_value(&body);
    appdict_data_write(
        &pool,
        &session,
        &app_dict_flag,
        &app_info_flag,
        &[],
        &value,
        AppdictWriteMode::Upsert,
    )
    .await
}

/// GET anonymous/surface/appdict/list/appInfo/{appInfoFlag}：
/// Java ActionListWithAppInfo —— 按栏目过滤字典列表。
#[axum::debug_handler]
pub async fn anonymous_surface_appdict_list_appInfo_appInfoFlag(
    pool: Extension<Pool>,
    axum::extract::Path(app_info_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time::text              FROM x_cms_surface_appdict WHERE app_info_flag = $1 AND deleted_at IS NULL",
            &[&app_info_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(appdict_row_to_value).collect();
    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

/// GET surface/appdict/list/appInfo/{appInfoFlag}
#[axum::debug_handler]
pub async fn surface_appdict_list_appInfo_appInfoFlag(
    pool: Extension<Pool>,
    axum::extract::Path(app_info_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    anonymous_surface_appdict_list_appInfo_appInfoFlag(pool, axum::extract::Path(app_info_flag)).await
}

// ── 路径深度 0..=7 端点族 ──

appdict_depth_family!(3;
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_delete
);
appdict_depth_family!(4;
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_delete
);
appdict_depth_family!(5;
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_delete
);
appdict_depth_family!(6;
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_delete
);
appdict_depth_family!(7;
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_delete
);
appdict_depth_family!(8;
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_delete
);
appdict_depth_family!(9;
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_delete
);
appdict_depth_family!(10;
    anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_put,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_post,
    surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_delete
);


// ─── templateform_* stubs ───────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn templateform_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form_v2", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn templateform_list_category(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form_v2", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn templateform_list_category_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_form_v2", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn templateform_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status, creator, create_time::text FROM x_cms_form_v2 WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("definition".to_string(), Value::String(row.get::<_, Option<String>>("definition").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("template form not found"))),
    }
}

#[axum::debug_handler]
pub async fn templateform_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_form_v2", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

// ─── uuid_random ────────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn uuid_random(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let uuid = uuid::Uuid::new_v4().to_string();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("uuid".to_string(), Value::String(uuid))]),
    ))))
}

// ─── view* / viewcategory* / viewfieldconfig* / viewrecord_* stubs ──────────

#[axum::debug_handler]
pub async fn view_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_view", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn view_list_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_view", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn view_list_category_categoryId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_view", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn view_list_form_formId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_view", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn view_viewdata_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_viewrecord", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn view_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_view", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn view_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_view", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn view_id_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let view_config = body.get("viewConfig").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_view (id, name, view_config) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = $2, view_config = $3",
            &[&id, &name, &view_config],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, view_config, creator, create_time::text FROM x_cms_view WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("viewConfig".to_string(), Value::String(row.get::<_, Option<String>>("view_config").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

#[axum::debug_handler]
pub async fn viewcategory_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_viewcategory", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn viewcategory_list_category_categoryId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_viewcategory", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn viewcategory_list_view_viewId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_viewcategory", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn viewcategory_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_viewcategory", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn viewcategory_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_viewcategory", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn viewfieldconfig_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_viewfieldconfig", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn viewfieldconfig_list_view_viewId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_viewfieldconfig", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn viewfieldconfig_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_viewfieldconfig", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn viewfieldconfig_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_viewfieldconfig", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn viewfieldconfig_id_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let field_name = body.get("fieldName").and_then(|v| v.as_str()).unwrap_or("");
    let field_config = body.get("fieldConfig").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_viewfieldconfig (id, field_name, field_config) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET field_name = $2, field_config = $3",
            &[&id, &field_name, &field_config],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, field_name, field_config, creator, create_time::text FROM x_cms_viewfieldconfig WHERE id = $1 AND deleted_at::text IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("fieldName".to_string(), Value::String(row.get("field_name"))),
                ("fieldConfig".to_string(), Value::String(row.get::<_, Option<String>>("field_config").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("view field config not found"))),
    }
}

#[axum::debug_handler]
pub async fn viewrecord_document_docId_filter_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_viewrecord", "deleted_at IS NULL", &[]).await
}

#[axum::debug_handler]
pub async fn viewrecord_document_docId_has_view(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT COUNT(*)::bigint AS cnt FROM x_cms_viewrecord WHERE doc_id = $1 AND deleted_at::text IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let count: i64 = row.map(|r| r.get("cnt")).unwrap_or(0);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("hasView".to_string(), Value::Bool(count > 0))]),
    ))))
}

#[axum::debug_handler]
pub async fn viewrecord_list_install_log_paging_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_viewrecord", "deleted_at IS NULL", &[]).await
}

// ─── image / input helpers (STUB: no corresponding DB table, business logic unclear) ──

// STUB: image_encode_base64 - image processing utility, no DB table mapping

#[axum::debug_handler]
pub async fn image_encode_base64(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let data_url = body.get("dataUrl").and_then(|v| v.as_str()).unwrap_or_default();
    let b64_str: String = data_url.replace("data:image/", "").split(',').last().unwrap_or("").to_string();
    let decoded = BASE64.decode(&b64_str).ok();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(decoded.is_some())),
            ("message".to_string(), Value::String(if decoded.is_some() { "Image encoded to base64".to_string() } else { "Invalid base64 image".to_string() })),
        ]),
    ))))
}

// STUB: image_encode_base64_size_size - image processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn image_encode_base64_size_size(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let data_url = body.get("dataUrl").and_then(|v| v.as_str()).unwrap_or_default();
    let b64_str: String = data_url.replace("data:image/", "").split(',').last().unwrap_or("").to_string();
    let decoded = BASE64.decode(&b64_str).ok();
    let size = decoded.as_ref().map(|d| d.len()).unwrap_or(0);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(decoded.is_some())),
            ("message".to_string(), Value::String("Image encoded to base64 with size".to_string())),
            ("size".to_string(), Value::Number(serde_json::Number::from(size))),
        ]),
    ))))
}

// STUB: image_resize_id_id_width_width_height_height - image processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn image_resize_id_id_width_width_height_height(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let width = body.get("width").and_then(|v| v.as_i64()).unwrap_or(0);
    let height = body.get("height").and_then(|v| v.as_i64()).unwrap_or(0);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(width > 0 && height > 0)),
            ("message".to_string(), Value::String("Image resized".to_string())),
            ("width".to_string(), Value::Number(serde_json::Number::from(width))),
            ("height".to_string(), Value::Number(serde_json::Number::from(height))),
        ]),
    ))))
}

// STUB: export_app_info_app_info_flag - U2 收尾对齐：Java GET /export/appInfo/{appInfoFlag}
#[axum::debug_handler]
pub async fn export_app_info_app_info_flag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

// STUB: import_app_info_app_info_flag - U2 收尾对齐：Java GET /import/appInfo/{appInfoFlag}
#[axum::debug_handler]
pub async fn import_app_info_app_info_flag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await
}

// STUB: input_compare - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_compare(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let a = body.get("a").and_then(|v| v.as_str()).unwrap_or_default();
    let b = body.get("b").and_then(|v| v.as_str()).unwrap_or_default();
    let equal = a == b;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(equal)),
            ("message".to_string(), Value::String("Input compared".to_string())),
            ("equal".to_string(), Value::Bool(equal)),
        ]),
    ))))
}

// STUB: input_compare_mockputtopost - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_compare_mockputtopost(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let a = body.get("a").and_then(|v| v.as_str()).unwrap_or_default();
    let b = body.get("b").and_then(|v| v.as_str()).unwrap_or_default();
    let equal = a == b;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(equal)),
            ("message".to_string(), Value::String("Input compared and saved".to_string())),
            ("equal".to_string(), Value::Bool(equal)),
        ]),
    ))))
}

// STUB: input_cover - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_cover(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let value = body.get("value").and_then(|v| v.as_str()).unwrap_or_default();
    let covered = !value.is_empty();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(covered)),
            ("message".to_string(), Value::String("Input covered".to_string())),
            ("covered".to_string(), Value::Bool(covered)),
        ]),
    ))))
}

// STUB: input_cover_mockputtopost - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_cover_mockputtopost(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let value = body.get("value").and_then(|v| v.as_str()).unwrap_or_default();
    let covered = !value.is_empty();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(covered)),
            ("message".to_string(), Value::String("Input covered and saved".to_string())),
            ("covered".to_string(), Value::Bool(covered)),
        ]),
    ))))
}

// STUB: input_create - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_create(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let value = body.get("value").and_then(|v| v.as_str()).unwrap_or_default();
    let saved = !value.is_empty();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(saved)),
            ("message".to_string(), Value::String("Input created".to_string())),
        ]),
    ))))
}

// STUB: input_create_mockputtopost - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_create_mockputtopost(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let value = body.get("value").and_then(|v| v.as_str()).unwrap_or_default();
    let saved = !value.is_empty();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(saved)),
            ("message".to_string(), Value::String("Input created and saved".to_string())),
        ]),
    ))))
}

// STUB: input_prepare_cover - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_prepare_cover(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let value = body.get("value").and_then(|v| v.as_str()).unwrap_or_default();
    let prepared = !value.is_empty();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(prepared)),
            ("message".to_string(), Value::String("Cover prepared".to_string())),
        ]),
    ))))
}

// STUB: input_prepare_cover_mockputtopost - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_prepare_cover_mockputtopost(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let value = body.get("value").and_then(|v| v.as_str()).unwrap_or_default();
    let prepared = !value.is_empty();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(prepared)),
            ("message".to_string(), Value::String("Cover prepared and saved".to_string())),
        ]),
    ))))
}

// STUB: input_prepare_create - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_prepare_create(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let value = body.get("value").and_then(|v| v.as_str()).unwrap_or_default();
    let prepared = !value.is_empty();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(prepared)),
            ("message".to_string(), Value::String("Create prepared".to_string())),
        ]),
    ))))
}

// STUB: input_prepare_create_mockputtopost - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_prepare_create_mockputtopost(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    let value = body.get("value").and_then(|v| v.as_str()).unwrap_or_default();
    let saved = !value.is_empty();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(saved)),
            ("message".to_string(), Value::String("Create prepared and saved".to_string())),
        ]),
    ))))
}

// ─── document_id_view_count ─────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn document_id_view_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "UPDATE x_cms_document SET view_count = view_count + 1 WHERE id = $1 RETURNING view_count AS new_count",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(r) => {
            let new_count: i64 = r.get("new_count");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("viewCount".to_string(), Value::Number(serde_json::Number::from(new_count))),
                ]),
            ))))
        }
        None => Err(AppError::NotFound),
    }
}

// ─── commend_list_paging ────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn commend_list_paging(
    pool: Extension<Pool>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let doc_id = params
        .get("doc_id")
        .ok_or_else(|| AppError::BadRequest("doc_id is required".to_string()))?
        .clone();

    let page: i64 = params
        .get("page")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let size: i64 = params
        .get("size")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20);

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count_row = client
        .query_one(
            "SELECT COUNT(*) FROM x_cms_commend WHERE doc_id = $1 AND deleted_at::text IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let count: i64 = count_row.get("count");

    let rows = client
        .query(
            "SELECT id, doc_id, person_id, create_time::text FROM x_cms_commend WHERE doc_id = $1 AND deleted_at::text IS NULL ORDER BY create_time::text DESC LIMIT $2::bigint OFFSET $3::bigint",
            &[&doc_id, &size, &((page - 1) * size)],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ─── document_search ─────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn document_search(
    pool: Extension<Pool>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let query = params
        .get("q")
        .ok_or_else(|| AppError::BadRequest("q is required".to_string()))?
        .clone();

    let limit: i32 = params
        .get("limit")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20);

    let results = search::search_documents_smart(&pool, &query, limit).await;

    let data: Vec<Value> = results
        .iter()
        .map(|doc| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(doc.id.clone())),
                ("title".to_string(), Value::String(doc.title.clone().unwrap_or_default())),
                ("content".to_string(), Value::String(doc.content.clone().unwrap_or_default())),
                ("rank".to_string(), Value::Number(serde_json::Number::from_f64(doc.rank.unwrap_or(0.0)).unwrap())),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}


#[axum::debug_handler]
pub async fn queryview_flag_definition(
    pool: Extension<Pool>,
    axum::extract::Path((view_flag, query_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, view_flag, query_flag, content FROM x_query_view WHERE view_flag = $1 AND query_flag = $2 LIMIT 1",
            &[&view_flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(r) => {
            let content: Option<String> = r.get("content");
            let content_str = content.clone().unwrap_or_default();
            if let Some(c) = &content {
                if let Ok(json) = serde_json::from_str::<Value>(c) {
                    if let Some(fields) = json.get("fields").and_then(|v| v.as_array()) {
                        return Ok(Json(ActionResult::success(Value::Object(
                            serde_json::Map::from_iter([
                                ("fields".to_string(), Value::Array(fields.clone())),
                            ]),
                        ))));
                    }
                }
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("viewFlag".to_string(), Value::String(view_flag)),
                    ("queryFlag".to_string(), Value::String(query_flag)),
                    ("content".to_string(), Value::String(content_str)),
                ]),
            ))))
        }
        None => Err(AppError::NotFound),
    }
}

// ════════════════════════════════════════════════════════════════════
// plan002 U2 — Java 对齐缺口端点（对照 jaxrs 静态提取全集补齐）
//
// 表全部沿用既有 CMS 表；migration 064 幂等补两列：
//   x_cms_data_document.is_top / x_cms_appinfo.config
//
// 写操作按 IDOR 门禁：
//   - 个人资源（document/comment/form/script/view/file/...）改删前
//     校验所有者（u2_check_owner，admin 放行）；
//   - 管理资源（appinfo 创建、permission 授予、批量 category change、
//     script manager 列表）一律 require_admin；
//   - 派生资源（viewfieldconfig/categoryinfo/correlation）经父资源或
//     所属文档所有者校验。
// ════════════════════════════════════════════════════════════════════

/// 写操作门禁结果：区分“不存在”（404 语义）与“非所有者”（403）。
enum U2Gate {
    NotFound,
    Forbidden,
    Allowed,
}

async fn u2_gate_by_sql(
    pool: &Pool,
    sql: &str,
    id: &str,
    person_unique: &str,
) -> Result<U2Gate, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(sql, &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        None => Ok(U2Gate::NotFound),
        Some(r) => {
            let owner = r.get::<_, Option<String>>("owner").unwrap_or_default();
            if shared::middleware::is_admin(pool, person_unique).await
                || (!owner.is_empty() && owner == person_unique)
            {
                Ok(U2Gate::Allowed)
            } else {
                Ok(U2Gate::Forbidden)
            }
        }
    }
}

async fn u2_check_owner(
    pool: &Pool,
    table: &str,
    owner_col: &str,
    id: &str,
    person_unique: &str,
) -> Result<U2Gate, AppError> {
    let sql = format!(
        "SELECT {} AS owner FROM {} WHERE id = $1 AND deleted_at IS NULL",
        owner_col, table
    );
    u2_gate_by_sql(pool, &sql, id, person_unique).await
}

async fn u2_require_admin(
    pool: &Pool,
    session: &shared::session::Session,
) -> Result<(), AppError> {
    if shared::middleware::is_admin(pool, &session.person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

fn u2_body_str(body: &Value, key: &str) -> Option<String> {
    body.get(key).and_then(|v| v.as_str()).map(String::from)
}

fn u2_body_i64(body: &Value, key: &str) -> Option<i64> {
    body.get(key).and_then(|v| v.as_i64())
}

fn u2_body_strs(body: &Value, key: &str) -> Vec<String> {
    body.get(key)
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

// ─── document 域 ────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn document_u2_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match get_by_id(&pool, "x_cms_data_document", &id).await? {
        Some(doc) => Ok(Json(ActionResult::success(doc))),
        None => Ok(Json(ActionResult::error("document not found"))),
    }
}

#[axum::debug_handler]
pub async fn document_u2_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_data_document", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            soft_delete_by_id(&pool, "x_cms_data_document", &id).await?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn document_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let title = u2_body_str(&body, "title").unwrap_or_default();
    let content = u2_body_str(&body, "content").unwrap_or_default();
    let app_id = u2_body_str(&body, "appId").unwrap_or_default();
    let category_id = u2_body_str(&body, "categoryId").unwrap_or_default();

    client
        .execute(
            "INSERT INTO x_cms_data_document (id, app_id, category_id, title, content, author_id, status, creator) \
             VALUES ($1, $2, $3, $4, $5, $6, 'draft', $6)",
            &[&id, &app_id, &category_id, &title, &content, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("status".to_string(), Value::String("draft".to_string())),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_u2_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_data_document", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let title = u2_body_str(&body, "title");
            let content = u2_body_str(&body, "content");
            let category_id = u2_body_str(&body, "categoryId");
            let affected = client
                .execute(
                    "UPDATE x_cms_data_document SET \
                     title = COALESCE($2, title), \
                     content = COALESCE($3, content), \
                     category_id = COALESCE($4, category_id) \
                     WHERE id = $1 AND deleted_at IS NULL",
                    &[&id, &title, &content, &category_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if affected == 0 {
                return Ok(Json(ActionResult::error("document not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("updated".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

async fn document_u2_set_status(
    pool: &Pool,
    id: &str,
    status: &str,
) -> Result<bool, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sql: &str = if status == "published" {
        "UPDATE x_cms_data_document SET status = 'published', publish_time = NOW() \
         WHERE id = $1 AND deleted_at IS NULL"
    } else {
        "UPDATE x_cms_data_document SET status = 'draft', publish_time = NULL \
         WHERE id = $1 AND deleted_at IS NULL"
    };
    let affected = client
        .execute(sql, &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(affected > 0)
}

async fn document_u2_status_response(
    result: bool,
    id: String,
    ok_key: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if !result {
        return Ok(Json(ActionResult::error("document not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            (ok_key.to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_u2_publish(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_data_document", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let published = document_u2_set_status(&pool, &id, "published").await?;
            document_u2_status_response(published, id, "published").await
        }
    }
}

#[axum::debug_handler]
pub async fn document_u2_publish_cancel(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_data_document", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let cancelled = document_u2_set_status(&pool, &id, "draft").await?;
            document_u2_status_response(cancelled, id, "cancelled").await
        }
    }
}

#[axum::debug_handler]
pub async fn document_u2_commend(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let doc = client
        .query_opt(
            "SELECT 1 FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if doc.is_none() {
        return Ok(Json(ActionResult::error("document not found")));
    }
    client
        .execute(
            "INSERT INTO x_cms_commend (id, doc_id, person_id) \
             SELECT gen_random_uuid()::text, $1, $2 \
             WHERE NOT EXISTS (\
               SELECT 1 FROM x_cms_commend WHERE doc_id = $1 AND person_id = $2 AND deleted_at IS NULL)",
            &[&id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("docId".to_string(), Value::String(id)),
            ("commended".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_u2_uncommend(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let removed = client
        .execute(
            "DELETE FROM x_cms_commend WHERE doc_id = $1 AND person_id = $2",
            &[&id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("docId".to_string(), Value::String(id)),
            ("removed".to_string(), Value::Number(serde_json::Number::from(removed as i64))),
        ]),
    ))))
}

async fn document_u2_set_top(
    pool: &Pool,
    session: &shared::session::Session,
    id: &str,
    top: bool,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(pool, "x_cms_data_document", "creator", id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let affected = client
                .execute(
                    "UPDATE x_cms_data_document SET is_top = $2 WHERE id = $1 AND deleted_at IS NULL",
                    &[&id, &top],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if affected == 0 {
                return Ok(Json(ActionResult::error("document not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id.to_string())),
                    ("isTop".to_string(), Value::Bool(top)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn document_u2_top(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    document_u2_set_top(&pool, &session, &id, true).await
}

#[axum::debug_handler]
pub async fn document_u2_un_top(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    document_u2_set_top(&pool, &session, &id, false).await
}

#[axum::debug_handler]
pub async fn document_u2_category_change(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let category_id = match u2_body_str(&body, "categoryId") {
        Some(c) if !c.is_empty() => c,
        _ => return Err(AppError::BadRequest("categoryId required".to_string())),
    };
    let doc_ids = u2_body_strs(&body, "docIds");
    if doc_ids.is_empty() {
        return Err(AppError::BadRequest("docIds required".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let updated = client
        .execute(
            "UPDATE x_cms_data_document SET category_id = $1 \
             WHERE id = ANY($2) AND deleted_at IS NULL",
            &[&category_id, &doc_ids],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("categoryId".to_string(), Value::String(category_id)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(updated as i64))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_u2_document_data(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT field_name, field_value FROM x_cms_data_document_field \
             WHERE doc_id = $1 AND deleted_at IS NULL ORDER BY field_name",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let mut fields = serde_json::Map::new();
    for row in rows.iter() {
        let name: String = row.get("field_name");
        let value: Option<String> = row.get("field_value");
        fields.insert(name, Value::String(value.unwrap_or_default()));
    }
    fields.insert("docId".to_string(), Value::String(id));
    Ok(Json(ActionResult::success(Value::Object(fields))))
}

#[axum::debug_handler]
pub async fn document_u2_list_document(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids = u2_body_strs(&body, "ids");
    if ids.is_empty() {
        return Err(AppError::BadRequest("ids required".to_string()));
    }
    list_from_table_filtered_java(
        &pool,
        "x_cms_data_document",
        "deleted_at IS NULL AND id = ANY($1)",
        &[&ids],
    )
    .await
}

#[axum::debug_handler]
pub async fn document_u2_fields(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT DISTINCT field_name AS name FROM x_cms_data_document_field \
             WHERE deleted_at IS NULL ORDER BY name LIMIT 200",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let names: Vec<Value> = rows
        .iter()
        .map(|r| Value::String(r.get::<_, String>("name")))
        .collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(names.len() as i64))),
            ("data".to_string(), Value::Array(names)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_u2_filter_count(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let app_id = u2_body_str(&body, "appId").unwrap_or_default();
    let category_id = u2_body_str(&body, "categoryId").unwrap_or_default();
    let status = u2_body_str(&body, "status").unwrap_or_default();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT COUNT(*) AS total FROM x_cms_data_document \
             WHERE deleted_at IS NULL \
             AND ($1 = '' OR app_id = $1) \
             AND ($2 = '' OR category_id = $2) \
             AND ($3 = '' OR status = $3)",
            &[&app_id, &category_id, &status],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = row.get("total");
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("total".to_string(), Value::Number(serde_json::Number::from(total)))]),
    ))))
}

// ─── comment 域 ─────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn comment_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let doc_id = match u2_body_str(&body, "docId") {
        Some(d) if !d.is_empty() => d,
        _ => return Err(AppError::BadRequest("docId required".to_string())),
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let doc = client
        .query_opt(
            "SELECT 1 FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if doc.is_none() {
        return Ok(Json(ActionResult::error("document not found")));
    }
    let id = uuid::Uuid::new_v4().to_string();
    let content = u2_body_str(&body, "content").unwrap_or_default();
    let parent_id = u2_body_str(&body, "parentCommentId");
    client
        .execute(
            "INSERT INTO x_cms_comment (id, doc_id, person_id, content, parent_id) \
             VALUES ($1, $2, $3, $4, $5)",
            &[&id, &doc_id, &session.person_unique, &content, &parent_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("docId".to_string(), Value::String(doc_id)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn comment_u2_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_comment", "person_id", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("comment not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            soft_delete_by_id(&pool, "x_cms_comment", &id).await?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn comment_u2_list_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let page = page.max(1);
    let size = size.clamp(1, 200);
    let offset = (page - 1) * size;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count_row = client
        .query_one(
            "SELECT COUNT(*) AS total FROM x_cms_comment WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = count_row.get("total");
    let rows = client
        .query(
            "SELECT * FROM x_cms_comment WHERE deleted_at IS NULL \
             ORDER BY create_time DESC LIMIT $1 OFFSET $2",
            &[&size, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(total))),
            ("page".to_string(), Value::Number(serde_json::Number::from(page))),
            ("size".to_string(), Value::Number(serde_json::Number::from(size))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ─── correlation 域 ─────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn correlation_u2_doc_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let gate_sql = "SELECT creator AS owner FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL";
    match u2_gate_by_sql(&pool, gate_sql, &doc_id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let removed = client
                .execute(
                    "UPDATE x_cms_correlation SET deleted_at = NOW() WHERE doc_id = $1",
                    &[&doc_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("docId".to_string(), Value::String(doc_id)),
                    ("deleted".to_string(), Value::Number(serde_json::Number::from(removed as i64))),
                ]),
            ))))
        }
    }
}

// ─── file / fileinfo 域 ─────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn file_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = u2_body_str(&body, "name").unwrap_or_default();
    let app_id = u2_body_str(&body, "appId").unwrap_or_default();
    let content_type = u2_body_str(&body, "contentType").unwrap_or_default();
    let content_base64 = u2_body_str(&body, "contentBase64").unwrap_or_default();
    let size = u2_body_i64(&body, "size").unwrap_or(0);

    client
        .execute(
            "INSERT INTO x_cms_file (id, app_id, name, size, content_type, content_base64, creator) \
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[&id, &app_id, &name, &size, &content_type, &content_base64, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_u2_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_file", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("file not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let name = u2_body_str(&body, "name");
            let content_type = u2_body_str(&body, "contentType");
            let content_base64 = u2_body_str(&body, "contentBase64");
            let affected = client
                .execute(
                    "UPDATE x_cms_file SET \
                     name = COALESCE($2, name), \
                     content_type = COALESCE($3, content_type), \
                     content_base64 = COALESCE($4, content_base64) \
                     WHERE id = $1 AND deleted_at IS NULL",
                    &[&id, &name, &content_type, &content_base64],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if affected == 0 {
                return Ok(Json(ActionResult::error("file not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("updated".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn fileinfo_u2_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_fileinfo", "upload_person", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("fileinfo not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            soft_delete_by_id(&pool, "x_cms_fileinfo", &id).await?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn fileinfo_u2_filter(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let doc_id = u2_body_str(&body, "docId").unwrap_or_default();
    let original_name = u2_body_str(&body, "originalName").unwrap_or_default();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count_row = client
        .query_one(
            "SELECT COUNT(*) AS total FROM x_cms_fileinfo \
             WHERE deleted_at IS NULL \
             AND ($1 = '' OR doc_id = $1) \
             AND ($2 = '' OR original_name ILIKE '%' || $2 || '%')",
            &[&doc_id, &original_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = count_row.get("total");
    let rows = client
        .query(
            "SELECT * FROM x_cms_fileinfo \
             WHERE deleted_at IS NULL \
             AND ($1 = '' OR doc_id = $1) \
             AND ($2 = '' OR original_name ILIKE '%' || $2 || '%') \
             ORDER BY create_time DESC LIMIT 100",
            &[&doc_id, &original_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        total,
        0,
    )))
}

async fn fileinfo_u2_doc_gate(
    pool: &Pool,
    session: &shared::session::Session,
    doc_id: &str,
) -> Result<U2Gate, AppError> {
    let gate_sql =
        "SELECT creator AS owner FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL";
    u2_gate_by_sql(pool, gate_sql, doc_id, &session.person_unique).await
}

#[axum::debug_handler]
pub async fn fileinfo_u2_copy_to_doc(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match fileinfo_u2_doc_gate(&pool, &session, &doc_id).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let attachment_ids = u2_body_strs(&body, "attachmentIds");
            if attachment_ids.is_empty() {
                return Err(AppError::BadRequest("attachmentIds required".to_string()));
            }
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let copied = client
                .execute(
                    "INSERT INTO x_cms_fileinfo (id, doc_id, file_id, original_name, size, content_type, upload_person) \
                     SELECT gen_random_uuid()::text, $1, file_id, original_name, size, content_type, upload_person \
                     FROM x_cms_fileinfo WHERE id = ANY($2) AND deleted_at IS NULL",
                    &[&doc_id, &attachment_ids],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("docId".to_string(), Value::String(doc_id)),
                    ("copied".to_string(), Value::Number(serde_json::Number::from(copied as i64))),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn fileinfo_u2_replace_to_doc(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match fileinfo_u2_doc_gate(&pool, &session, &doc_id).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let attachment_ids = u2_body_strs(&body, "attachmentIds");
            if attachment_ids.is_empty() {
                return Err(AppError::BadRequest("attachmentIds required".to_string()));
            }
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let moved = client
                .execute(
                    "UPDATE x_cms_fileinfo SET doc_id = $1 \
                     WHERE id = ANY($2) AND deleted_at IS NULL",
                    &[&doc_id, &attachment_ids],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("docId".to_string(), Value::String(doc_id)),
                    ("moved".to_string(), Value::Number(serde_json::Number::from(moved as i64))),
                ]),
            ))))
        }
    }
}

// ─── form 域 ────────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn form_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let app_id = match u2_body_str(&body, "appId") {
        Some(a) if !a.is_empty() => a,
        _ => return Err(AppError::BadRequest("appId required".to_string())),
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let app = client
        .query_opt(
            "SELECT 1 FROM x_cms_appinfo WHERE id = $1 AND deleted_at IS NULL",
            &[&app_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if app.is_none() {
        return Ok(Json(ActionResult::error("application not found")));
    }
    let id = uuid::Uuid::new_v4().to_string();
    let name = u2_body_str(&body, "name").unwrap_or_default();
    let definition = u2_body_str(&body, "definition").unwrap_or_default();
    client
        .execute(
            "INSERT INTO x_cms_form (id, app_id, name, definition, status, creator) \
             VALUES ($1, $2, $3, $4, 'draft', $5)",
            &[&id, &app_id, &name, &definition, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("appId".to_string(), Value::String(app_id)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn form_u2_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_form", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("form not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let name = u2_body_str(&body, "name");
            let definition = u2_body_str(&body, "definition");
            let status = u2_body_str(&body, "status");
            let affected = client
                .execute(
                    "UPDATE x_cms_form SET \
                     name = COALESCE($2, name), \
                     definition = COALESCE($3, definition), \
                     status = COALESCE($4, status) \
                     WHERE id = $1 AND deleted_at IS NULL",
                    &[&id, &name, &definition, &status],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if affected == 0 {
                return Ok(Json(ActionResult::error("form not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("updated".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn form_u2_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_form", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("form not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            soft_delete_by_id(&pool, "x_cms_form", &id).await?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
    }
}

// ─── script 域 ──────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn script_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let app_id = match u2_body_str(&body, "appId") {
        Some(a) if !a.is_empty() => a,
        _ => return Err(AppError::BadRequest("appId required".to_string())),
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let app = client
        .query_opt(
            "SELECT 1 FROM x_cms_appinfo WHERE id = $1 AND deleted_at IS NULL",
            &[&app_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if app.is_none() {
        return Ok(Json(ActionResult::error("application not found")));
    }
    let id = uuid::Uuid::new_v4().to_string();
    let name = u2_body_str(&body, "name").unwrap_or_default();
    let unique_name = u2_body_str(&body, "uniqueName");
    let content = u2_body_str(&body, "scriptContent").unwrap_or_default();
    client
        .execute(
            "INSERT INTO x_cms_script (id, app_id, name, unique_name, script_content, imported, creator) \
             VALUES ($1, $2, $3, $4, $5, false, $6)",
            &[&id, &app_id, &name, &unique_name, &content, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("appId".to_string(), Value::String(app_id)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn script_u2_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_script", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("script not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let name = u2_body_str(&body, "name");
            let unique_name = u2_body_str(&body, "uniqueName");
            let content = u2_body_str(&body, "scriptContent");
            let imported = body.get("imported").and_then(|v| v.as_bool());
            let affected = client
                .execute(
                    "UPDATE x_cms_script SET \
                     name = COALESCE($2, name), \
                     unique_name = COALESCE($3, unique_name), \
                     script_content = COALESCE($4, script_content), \
                     imported = COALESCE($5, imported) \
                     WHERE id = $1 AND deleted_at IS NULL",
                    &[&id, &name, &unique_name, &content, &imported],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if affected == 0 {
                return Ok(Json(ActionResult::error("script not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("updated".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn script_u2_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_script", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("script not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            soft_delete_by_id(&pool, "x_cms_script", &id).await?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn script_u2_list_manager(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    list_from_table_filtered_java(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await
}

// ─── templateform / view / viewcategory / viewfieldconfig 域 ────────────

#[axum::debug_handler]
pub async fn templateform_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = u2_body_str(&body, "name").unwrap_or_default();
    let category = u2_body_str(&body, "category").unwrap_or_default();
    client
        .execute(
            "INSERT INTO x_cms_templateform (id, xname, xcategory, creator_person) \
             VALUES ($1, $2, $3, $4)",
            &[&id, &name, &category, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

#[axum::debug_handler]
pub async fn templateform_u2_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_templateform", "creator_person", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("templateform not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            soft_delete_by_id(&pool, "x_cms_templateform", &id).await?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn view_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = match u2_body_str(&body, "name") {
        Some(n) if !n.is_empty() => n,
        _ => return Err(AppError::BadRequest("name required".to_string())),
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let app_id = u2_body_str(&body, "appId").unwrap_or_default();
    let category_id = u2_body_str(&body, "categoryId").unwrap_or_default();
    let view_config = u2_body_str(&body, "viewConfig").unwrap_or_default();
    client
        .execute(
            "INSERT INTO x_cms_view (id, app_id, category_id, name, view_config, creator) \
             VALUES ($1, $2, $3, $4, $5, $6)",
            &[&id, &app_id, &category_id, &name, &view_config, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

#[axum::debug_handler]
pub async fn view_u2_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_view", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("view not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let name = u2_body_str(&body, "name");
            let view_config = u2_body_str(&body, "viewConfig");
            let affected = client
                .execute(
                    "UPDATE x_cms_view SET \
                     name = COALESCE($2, name), \
                     view_config = COALESCE($3, view_config) \
                     WHERE id = $1 AND deleted_at IS NULL",
                    &[&id, &name, &view_config],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if affected == 0 {
                return Ok(Json(ActionResult::error("view not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("updated".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn view_u2_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_view", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("view not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            soft_delete_by_id(&pool, "x_cms_view", &id).await?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn viewcategory_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = match u2_body_str(&body, "name") {
        Some(n) if !n.is_empty() => n,
        _ => return Err(AppError::BadRequest("name required".to_string())),
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let parent_id = u2_body_str(&body, "parentCategoryId").unwrap_or_default();
    client
        .execute(
            "INSERT INTO x_cms_viewcategory (id, name, parent_id, creator) \
             VALUES ($1, $2, $3, $4)",
            &[&id, &name, &parent_id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

#[axum::debug_handler]
pub async fn viewcategory_u2_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_viewcategory", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("viewcategory not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            soft_delete_by_id(&pool, "x_cms_viewcategory", &id).await?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
    }
}

const U2_VIEW_FIELD_OWNER_SQL: &str =
    "SELECT v.creator AS owner FROM x_cms_viewfieldconfig vfc \
     JOIN x_cms_view v ON v.id = vfc.view_id WHERE vfc.id = $1 AND vfc.deleted_at IS NULL";

#[axum::debug_handler]
pub async fn viewfieldconfig_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let view_id = match u2_body_str(&body, "viewId") {
        Some(v) if !v.is_empty() => v,
        _ => return Err(AppError::BadRequest("viewId required".to_string())),
    };
    let gate_sql = "SELECT creator AS owner FROM x_cms_view WHERE id = $1 AND deleted_at IS NULL";
    match u2_gate_by_sql(&pool, gate_sql, &view_id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("view not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let id = uuid::Uuid::new_v4().to_string();
            let field_name = match u2_body_str(&body, "fieldName") {
                Some(f) if !f.is_empty() => f,
                _ => return Err(AppError::BadRequest("fieldName required".to_string())),
            };
            let field_config = u2_body_str(&body, "fieldConfig").unwrap_or_default();
            let sort_order = u2_body_i64(&body, "sortOrder").unwrap_or(0) as i32;
            client
                .execute(
                    "INSERT INTO x_cms_viewfieldconfig (id, view_id, field_name, field_config, sort_order) \
                     VALUES ($1, $2, $3, $4, $5)",
                    &[&id, &view_id, &field_name, &field_config, &sort_order],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("viewId".to_string(), Value::String(view_id)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn viewfieldconfig_u2_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_gate_by_sql(&pool, U2_VIEW_FIELD_OWNER_SQL, &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("viewfieldconfig not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let field_config = u2_body_str(&body, "fieldConfig");
            let sort_order = u2_body_i64(&body, "sortOrder").map(|v| v as i32);
            let affected = client
                .execute(
                    "UPDATE x_cms_viewfieldconfig SET \
                     field_config = COALESCE($2, field_config), \
                     sort_order = COALESCE($3, sort_order) \
                     WHERE id = $1 AND deleted_at IS NULL",
                    &[&id, &field_config, &sort_order],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if affected == 0 {
                return Ok(Json(ActionResult::error("viewfieldconfig not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("updated".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn viewfieldconfig_u2_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_gate_by_sql(&pool, U2_VIEW_FIELD_OWNER_SQL, &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("viewfieldconfig not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            soft_delete_by_id(&pool, "x_cms_viewfieldconfig", &id).await?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
    }
}

// ─── appinfo / categoryinfo / permission / appconfig / designer 域 ──────

#[axum::debug_handler]
pub async fn appinfo_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = match u2_body_str(&body, "id") {
        Some(i) if !i.is_empty() => i,
        _ => uuid::Uuid::new_v4().to_string(),
    };
    let alias = u2_body_str(&body, "alias").unwrap_or_default();
    let app_type = u2_body_str(&body, "appType").unwrap_or("cms".to_string());
    let icon = u2_body_str(&body, "icon").unwrap_or_default();
    let manager = u2_body_str(&body, "manager").unwrap_or_else(|| session.person_unique.clone());
    client
        .execute(
            "INSERT INTO x_cms_appinfo (id, alias, app_type, icon, enabled, manager, creator) \
             VALUES ($1, $2, $3, $4, true, $5, $6)",
            &[&id, &alias, &app_type, &icon, &manager, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("manager".to_string(), Value::String(manager)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn appinfo_u2_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_appinfo", "manager", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("application not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            soft_delete_by_id(&pool, "x_cms_appinfo", &id).await?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn categoryinfo_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let app_id = match u2_body_str(&body, "appId") {
        Some(a) if !a.is_empty() => a,
        _ => return Err(AppError::BadRequest("appId required".to_string())),
    };
    let gate_sql = "SELECT manager AS owner FROM x_cms_appinfo WHERE id = $1 AND deleted_at IS NULL";
    match u2_gate_by_sql(&pool, gate_sql, &app_id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("application not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let id = uuid::Uuid::new_v4().to_string();
            let name = u2_body_str(&body, "name").unwrap_or_default();
            let parent_id = u2_body_str(&body, "parentCategoryId").unwrap_or_default();
            client
                .execute(
                    "INSERT INTO x_cms_categoryinfo (id, name, parent_id, app_id, creator) \
                     VALUES ($1, $2, $3, $4, $5)",
                    &[&id, &name, &parent_id, &app_id, &session.person_unique],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("appId".to_string(), Value::String(app_id)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn categoryinfo_u2_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let gate_sql = "SELECT (SELECT manager FROM x_cms_appinfo WHERE id = ci.app_id AND manager IS NOT NULL) AS owner \
                    FROM x_cms_categoryinfo ci WHERE ci.id = $1 AND ci.deleted_at IS NULL";
    match u2_gate_by_sql(&pool, gate_sql, &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("category not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            soft_delete_by_id(&pool, "x_cms_categoryinfo", &id).await?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
    }
}

async fn u2_write_permissions(
    pool: &Pool,
    scope_col: &str,
    scope_id: &str,
    body: &Value,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let role_type = match u2_body_str(body, "roleType") {
        Some(r) if ["manager", "publisher", "viewer"].contains(&r.as_str()) => r,
        _ => return Err(AppError::BadRequest("roleType must be manager/publisher/viewer".to_string())),
    };
    let person_ids = u2_body_strs(body, "personIds");
    if person_ids.is_empty() {
        return Err(AppError::BadRequest("personIds required".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            &format!(
                "DELETE FROM x_cms_permission WHERE {} = $1 AND role_type = $2",
                scope_col
            ),
            &[&scope_id, &role_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let mut granted: u64 = 0;
    for pid in &person_ids {
        granted += client
            .execute(
                "INSERT INTO x_cms_permission (id, role_type, permission_level, person_id) \
                 VALUES (gen_random_uuid()::text, $1, 'write', $2)",
                &[&role_type, &pid],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }
    let _ = pool;
    let _ = scope_col;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("scopeId".to_string(), Value::String(scope_id.to_string())),
            ("roleType".to_string(), Value::String(role_type)),
            ("granted".to_string(), Value::Number(serde_json::Number::from(granted as i64))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn permission_u2_app_info(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let app = client
        .query_opt(
            "SELECT 1 FROM x_cms_appinfo WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if app.is_none() {
        return Ok(Json(ActionResult::error("application not found")));
    }
    u2_write_permissions(&pool, "app_id", &id, &body).await
}

#[axum::debug_handler]
pub async fn permission_u2_category_info(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let cat = client
        .query_opt(
            "SELECT 1 FROM x_cms_categoryinfo WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if cat.is_none() {
        return Ok(Json(ActionResult::error("category not found")));
    }
    u2_write_permissions(&pool, "category_id", &id, &body).await
}

#[axum::debug_handler]
pub async fn appconfig_u2_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(app_id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_appinfo", "manager", &app_id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("application not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let config_text = serde_json::to_string(&body.0).unwrap_or_else(|_| "{}".to_string());
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let affected = client
                .execute(
                    "UPDATE x_cms_appinfo SET config = $1 WHERE id = $2 AND deleted_at IS NULL",
                    &[&config_text, &app_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if affected == 0 {
                return Ok(Json(ActionResult::error("application not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("appId".to_string(), Value::String(app_id)),
                    ("saved".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn appconfig_u2_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT config FROM x_cms_appinfo WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        None => Ok(Json(ActionResult::error("application not found"))),
        Some(r) => {
            let raw: Option<String> = r.get("config");
            let config = match raw {
                Some(text) => serde_json::from_str::<Value>(&text).unwrap_or(Value::Null),
                None => Value::Object(serde_json::Map::new()),
            };
            Ok(Json(ActionResult::success(config)))
        }
    }
}

#[axum::debug_handler]
pub async fn designer_u2_search(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let keyword = u2_body_str(&body, "keyword").unwrap_or_default();
    if keyword.trim().is_empty() {
        return Err(AppError::BadRequest("keyword required".to_string()));
    }
    let pattern = format!("%{}%", keyword);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT kind, id, label FROM (\
               (SELECT 'appInfo' AS kind, id, COALESCE(alias, '') AS label FROM x_cms_appinfo \
                 WHERE deleted_at IS NULL AND alias ILIKE $1 LIMIT 20)\
               UNION ALL \
               (SELECT 'category', id, name FROM x_cms_categoryinfo \
                 WHERE deleted_at IS NULL AND name ILIKE $1 LIMIT 20)\
               UNION ALL \
               (SELECT 'form', id, name FROM x_cms_form \
                 WHERE deleted_at IS NULL AND name ILIKE $1 LIMIT 20)\
               UNION ALL \
               (SELECT 'view', id, name FROM x_cms_view \
                 WHERE deleted_at IS NULL AND name ILIKE $1 LIMIT 20)\
               UNION ALL \
               (SELECT 'script', id, name FROM x_cms_script \
                 WHERE deleted_at IS NULL AND name ILIKE $1 LIMIT 20)\
             ) s ORDER BY kind, label LIMIT 100",
            &[&pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let results: Vec<Value> = rows
        .iter()
        .map(|r| {
            Value::Object(serde_json::Map::from_iter([
                ("kind".to_string(), Value::String(r.get("kind"))),
                ("id".to_string(), Value::String(r.get("id"))),
                ("label".to_string(), Value::String(r.get("label"))),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("keyword".to_string(), Value::String(keyword)),
            ("count".to_string(), Value::Number(serde_json::Number::from(results.len() as i64))),
            ("data".to_string(), Value::Array(results)),
        ]),
    ))))
}

// ═══ plan002 U2 冲刺收尾（重试批次，U3）：Java 对齐缺口端点 ═════════════════
// 覆盖剩余缺口：document 管理面（batch/filter/draft/cipher/control/publish）、
// file/fileinfo 规范路径、design/appdict CRUD、permission save 家族、review 搜索、
// viewrecord unread、comment commend、correlation create/update、categoryinfo
// extContent/objects 等。写路径统一 IDOR 门禁 + ActionResult 契约。

async fn u3_gate_document(
    pool: &Pool,
    session: &shared::session::Session,
    id: &str,
) -> Result<U2Gate, AppError> {
    u2_check_owner(pool, "x_cms_data_document", "creator", id, &session.person_unique).await
}

async fn u3_field_upsert(
    client: &deadpool_postgres::Client,
    doc_id: &str,
    field_name: &str,
    field_value: &str,
) -> Result<u64, AppError> {
    client
        .execute(
            "DELETE FROM x_cms_data_document_field WHERE doc_id = $1 AND field_name = $2",
            &[&doc_id, &field_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) \
             VALUES (gen_random_uuid()::text, $1, $2, $3)",
            &[&doc_id, &field_name, &field_value],
        )
        .await
        .map_err(|_| AppError::Internal)
}

async fn u3_bulk_publish_status(
    pool: &Pool,
    ids: &[String],
    status: &str,
) -> Result<u64, AppError> {
    if ids.is_empty() {
        return Err(AppError::BadRequest("docIds required".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = if status == "published" {
        client
            .execute(
                "UPDATE x_cms_data_document SET status = 'published', publish_time = NOW() \
                 WHERE id = ANY($1) AND deleted_at IS NULL",
                &[&ids.to_vec()],
            )
            .await
            .map_err(|_| AppError::Internal)?
        } else {
            client
                .execute(
                    "UPDATE x_cms_data_document SET status = 'draft', publish_time = NULL \
                     WHERE id = ANY($1) AND deleted_at IS NULL",
                    &[&ids.to_vec()],
                )
                .await
                .map_err(|_| AppError::Internal)?
        };
    Ok(affected)
}

async fn u3_cipher_upsert(
    pool: &Pool,
    doc_ids: &[String],
    cipher_text: &str,
    person_id: &str,
) -> Result<u64, AppError> {
    if doc_ids.is_empty() {
        return Err(AppError::BadRequest("docIds required".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut written = 0u64;
    for doc_id in doc_ids {
        written += client
            .execute(
                "INSERT INTO x_cms_document_cipher (id, doc_id, cipher_text, person_id) \
                 VALUES (gen_random_uuid()::text, $1, $2, $3) \
                 ON CONFLICT (doc_id) DO UPDATE SET cipher_text = $2, person_id = $3, create_time = NOW()",
                &[&doc_id, &cipher_text, &person_id],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }
    Ok(written)
}

async fn u3_can_manage_scope(
    pool: &Pool,
    scope: &str,
    id: &str,
    person_unique: &str,
) -> Result<U2Gate, AppError> {
    let gate_sql = match scope {
        "appInfo" => "SELECT manager AS owner FROM x_cms_appinfo WHERE id = $1 AND deleted_at IS NULL"
            .to_string(),
        _ => {
            "SELECT (SELECT manager FROM x_cms_appinfo WHERE id = ci.app_id AND manager IS NOT NULL) AS owner \
             FROM x_cms_categoryinfo ci WHERE ci.id = $1 AND ci.deleted_at IS NULL"
                .to_string()
        }
    };
    u2_gate_by_sql(pool, &gate_sql, id, person_unique).await
}

async fn u3_save_scope_permissions(
    pool: &Pool,
    session: &shared::session::Session,
    scope: &str,
    id: &str,
    role_type: &str,
    body: &Value,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u3_can_manage_scope(pool, scope, id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("scope not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let person_ids = u2_body_strs(body, "personIds");
            if person_ids.is_empty() {
                return Err(AppError::BadRequest("personIds required".to_string()));
            }
            let scope_col = if scope == "appInfo" { "app_id" } else { "category_id" };
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            client
                .execute(
                    &format!(
                        "DELETE FROM x_cms_permission WHERE {} = $1 AND role_type = $2 AND deleted_at IS NULL",
                        scope_col
                    ),
                    &[&id, &role_type],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let mut granted = 0u64;
            for pid in &person_ids {
                granted += client
                    .execute(
                        &format!(
                            "INSERT INTO x_cms_permission (id, {}, role_type, permission_level, person_id) \
                             VALUES (gen_random_uuid()::text, $1, $2, 'write', $3)",
                            scope_col
                        ),
                        &[&id, &role_type, &pid],
                    )
                    .await
                    .map_err(|_| AppError::Internal)?;
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("scope".to_string(), Value::String(scope.to_string())),
                    ("scopeId".to_string(), Value::String(id.to_string())),
                    ("roleType".to_string(), Value::String(role_type.to_string())),
                    ("granted".to_string(), Value::Number(serde_json::Number::from(granted as i64))),
                ]),
            ))))
        }
    }
}

// ── categoryinfo 管理面 ─────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn categoryinfo_ext_content_save_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let category_id = match u2_body_str(&body, "categoryId").or_else(|| u2_body_str(&body, "id")) {
        Some(c) if !c.is_empty() => c,
        _ => return Err(AppError::BadRequest("categoryId required".to_string())),
    };
    let ext_content = u2_body_str(&body, "extContent").unwrap_or_default();
    match u3_can_manage_scope(&pool, "categoryInfo", &category_id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("category not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let affected = client
                .execute(
                    "UPDATE x_cms_categoryinfo SET ext_content = $2 WHERE id = $1 AND deleted_at IS NULL",
                    &[&category_id, &ext_content],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if affected == 0 {
                return Ok(Json(ActionResult::error("category not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(category_id)),
                    ("saved".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn categoryinfo_list_objects_u3(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids = u2_body_strs(&body, "ids");
    if ids.is_empty() {
        return Err(AppError::BadRequest("ids required".to_string()));
    }
    list_from_table_filtered_java(
        &pool,
        "x_cms_categoryinfo",
        "deleted_at IS NULL AND id = ANY($1)",
        &[&ids],
    )
    .await
}

#[axum::debug_handler]
pub async fn comment_commend_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let exists = client
        .query_opt(
            "SELECT doc_id FROM x_cms_comment WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let doc_id: String = match exists {
        Some(r) => r.get::<_, Option<String>>("doc_id").unwrap_or_default(),
        None => return Ok(Json(ActionResult::error("comment not found"))),
    };
    client
        .execute(
            "INSERT INTO x_cms_commend (id, doc_id, person_id) VALUES (gen_random_uuid()::text, $1, $2)",
            &[&doc_id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("commended".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn comment_uncommend_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let doc_id = client
        .query_opt(
            "SELECT doc_id FROM x_cms_comment WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let doc_id: String = match doc_id {
        Some(r) => r.get::<_, Option<String>>("doc_id").unwrap_or_default(),
        None => return Ok(Json(ActionResult::error("comment not found"))),
    };
    let affected = client
        .execute(
            "DELETE FROM x_cms_commend WHERE doc_id = $1 AND person_id = $2",
            &[&doc_id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("uncommended".to_string(), Value::Bool(true)),
            ("count".to_string(), Value::Number(serde_json::Number::from(affected as i64))),
        ]),
    ))))
}

// ── correlation create / update（canonical）────────────────────────────────

#[axum::debug_handler]
pub async fn correlation_create_u3(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut related = u2_body_strs(&body, "relatedDocIds");
    if related.is_empty() {
        if let Some(one) = u2_body_str(&body, "relatedDocId") {
            related.push(one);
        }
    }
    if related.is_empty() {
        return Err(AppError::BadRequest("relatedDocId(s) required".to_string()));
    }
    let correlation_type = u2_body_str(&body, "correlationType").unwrap_or_default();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut created = 0u64;
    for target in &related {
        client
            .execute(
                "DELETE FROM x_cms_correlation WHERE doc_id = $1 AND related_doc_id = $2",
                &[&doc_id, &target],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        created += client
            .execute(
                "INSERT INTO x_cms_correlation (id, doc_id, related_doc_id, correlation_type) \
                 VALUES (gen_random_uuid()::text, $1, $2, $3)",
                &[&doc_id, &target, &correlation_type],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("docId".to_string(), Value::String(doc_id)),
            ("created".to_string(), Value::Number(serde_json::Number::from(created as i64))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn correlation_update_u3(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut related = u2_body_strs(&body, "relatedDocIds");
    if related.is_empty() {
        if let Some(one) = u2_body_str(&body, "relatedDocId") {
            related.push(one);
        }
    }
    if related.is_empty() {
        return Err(AppError::BadRequest("relatedDocId(s) required".to_string()));
    }
    let correlation_type = u2_body_str(&body, "correlationType").unwrap_or_default();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut updated = 0u64;
    for target in &related {
        client
            .execute(
                "DELETE FROM x_cms_correlation WHERE doc_id = $1 AND related_doc_id = $2",
                &[&doc_id, &target],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        updated += client
            .execute(
                "INSERT INTO x_cms_correlation (id, doc_id, related_doc_id, correlation_type) \
                 VALUES (gen_random_uuid()::text, $1, $2, $3)",
                &[&doc_id, &target, &correlation_type],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("docId".to_string(), Value::String(doc_id)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(updated as i64))),
        ]),
    ))))
}

// ── design/appdict CRUD（canonical，含归一化查重）──────────────────────────

fn u3_normalize_path_levels(raw_levels: &[String]) -> String {
    let mut parts: Vec<String> = raw_levels.iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    parts.sort();
    parts.join("/")
}

#[axum::debug_handler]
pub async fn design_appdict_create_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let app_info_flag = u2_body_str(&body, "appInfoFlag")
        .or_else(|| u2_body_str(&body, "appInfo"))
        .unwrap_or_default();
    let app_dict_flag = u2_body_str(&body, "appDictFlag")
        .or_else(|| u2_body_str(&body, "name"))
        .unwrap_or_default();
    if app_info_flag.is_empty() || app_dict_flag.is_empty() {
        return Err(AppError::BadRequest("appInfoFlag/appDictFlag required".to_string()));
    }
    let path_levels = u3_normalize_path_levels(&u2_body_strs(&body, "pathLevels"));
    let data_value = match body.get("dataValue") {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => "{}".to_string(),
    };
    // 归一化查重：同一 (app, dict, 归一化 pathLevels) 仅允许一行活动数据。
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let dup = client
        .query_opt(
            "SELECT id FROM x_cms_surface_appdict \
             WHERE app_info_flag = $1 AND app_dict_flag = $2 AND path_levels = $3 AND deleted_at IS NULL",
            &[&app_info_flag, &app_dict_flag, &path_levels],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if dup.is_some() {
        return Ok(Json(ActionResult::error("appdict data already exists")));
    }
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_cms_surface_appdict (id, app_info_flag, app_dict_flag, path_levels, data_value, creator) \
             VALUES ($1, $2, $3, $4, $5, $6)",
            &[&id, &app_info_flag, &app_dict_flag, &path_levels, &data_value, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

#[axum::debug_handler]
pub async fn design_appdict_update_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let data_value = match body.get("dataValue") {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => "{}".to_string(),
    };
    let path_levels = u2_body_strs(&body, "pathLevels");
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = if path_levels.is_empty() {
        client
            .execute(
                "UPDATE x_cms_surface_appdict SET data_value = $2 WHERE id = $1 AND deleted_at IS NULL",
                &[&id, &data_value],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        let normalized = u3_normalize_path_levels(&path_levels);
        client
            .execute(
                "UPDATE x_cms_surface_appdict SET data_value = $2, path_levels = $3 WHERE id = $1 AND deleted_at IS NULL",
                &[&id, &data_value, &normalized],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };
    if affected == 0 {
        return Ok(Json(ActionResult::error("appdict not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn design_appdict_delete_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    soft_delete_by_id(&pool, "x_cms_surface_appdict", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

// ── review 搜索 ─────────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn review_v2_search_u3(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let keyword = u2_body_str(&body, "keyword").unwrap_or_default();
    let size = u2_body_i64(&body, "size").unwrap_or(20).clamp(1, 100);
    let pattern = format!("%{}%", keyword);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, app_id, category_id, creator, publish_time::text \
             FROM x_cms_data_document \
             WHERE status = 'published' AND deleted_at IS NULL \
               AND ($1 = '%%' OR title ILIKE $1 OR content ILIKE $1) \
             ORDER BY publish_time DESC NULLS LAST LIMIT $2",
            &[&pattern, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(row_to_json).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

// ── document 管理面 ─────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn document_achive_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u3_gate_document(&pool, &session, &id).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let affected = client
                .execute(
                    "UPDATE x_cms_data_document SET status = 'archived' WHERE id = $1 AND deleted_at IS NULL",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if affected == 0 {
                return Ok(Json(ActionResult::error("document not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("archived".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

async fn u3_batch_modify_impl(
    pool: &Pool,
    session: &shared::session::Session,
    body: &Value,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(pool, session).await?;
    let doc_ids = u2_body_strs(body, "docIds");
    let data = body.get("data").cloned().unwrap_or(Value::Null);
    let fields = match data.as_object() {
        Some(m) if !m.is_empty() => m.clone(),
        _ => return Err(AppError::BadRequest("data required".to_string())),
    };
    if doc_ids.is_empty() {
        return Err(AppError::BadRequest("docIds required".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut written = 0u64;
    for doc_id in &doc_ids {
        for (name, value) in &fields {
            let text = match value {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            written += u3_field_upsert(&client, doc_id, name, &text).await?;
        }
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modified".to_string(), Value::Number(serde_json::Number::from(written as i64))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_batch_modify_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u3_batch_modify_impl(&pool, &session, &body.0).await
}

#[axum::debug_handler]
pub async fn document_batch_modify_mock_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u3_batch_modify_impl(&pool, &session, &body.0).await
}

#[axum::debug_handler]
pub async fn document_batch_status_u3(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT batch_name, COUNT(*)::bigint AS cnt FROM x_cms_data_document \
             WHERE batch_name IS NOT NULL AND batch_name <> '' AND deleted_at IS NULL \
             GROUP BY batch_name ORDER BY batch_name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|r| {
            Value::Object(serde_json::Map::from_iter([
                ("batchName".to_string(), Value::String(r.get("batch_name"))),
                (
                    "count".to_string(),
                    Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt"))),
                ),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("data".to_string(), Value::Array(data))]),
    ))))
}

#[axum::debug_handler]
pub async fn document_batch_name_status_u3(
    pool: Extension<Pool>,
    axum::extract::Path(batch_name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT COUNT(*)::bigint AS cnt FROM x_cms_data_document \
             WHERE batch_name = $1 AND deleted_at IS NULL",
            &[&batch_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("batchName".to_string(), Value::String(batch_name)),
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(row.get::<_, i64>("cnt"))),
            ),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_batch_delete_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(batch_name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute(
            "UPDATE x_cms_data_document SET deleted_at = NOW() \
             WHERE batch_name = $1 AND deleted_at IS NULL",
            &[&batch_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("batchName".to_string(), Value::String(batch_name)),
            ("deleted".to_string(), Value::Number(serde_json::Number::from(affected as i64))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_batch_delete_mock_u3(
    pool: Extension<Pool>,
    axum::extract::Path(batch_name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT COUNT(*)::bigint AS cnt FROM x_cms_data_document \
             WHERE batch_name = $1 AND deleted_at IS NULL",
            &[&batch_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("batchName".to_string(), Value::String(batch_name)),
            (
                "wouldDelete".to_string(),
                Value::Number(serde_json::Number::from(row.get::<_, i64>("cnt"))),
            ),
        ]),
    ))))
}

async fn u3_publish_content_impl(
    pool: &Pool,
    session: &shared::session::Session,
    body: &Value,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(pool, session).await?;
    let mut ids = u2_body_strs(body, "docIds");
    if ids.is_empty() {
        ids = u2_body_strs(body, "documentIds");
    }
    let affected = u3_bulk_publish_status(pool, &ids, "published").await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("published".to_string(), Value::Number(serde_json::Number::from(affected as i64))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_publish_content_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u3_publish_content_impl(&pool, &session, &body.0).await
}

#[axum::debug_handler]
pub async fn document_publish_content_mock_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u3_publish_content_impl(&pool, &session, &body.0).await
}

async fn u3_cipher_publish_workflow_impl(
    pool: &Pool,
    body: &Value,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut ids = u2_body_strs(body, "docIds");
    if ids.is_empty() {
        if let Some(one) = u2_body_str(body, "docId") {
            ids.push(one);
        }
    }
    let cipher_text = u2_body_str(body, "cipherText").unwrap_or_default();
    let person_id = u2_body_str(body, "personId").unwrap_or_default();
    let written = u3_cipher_upsert(pool, &ids, &cipher_text, &person_id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("ciphered".to_string(), Value::Number(serde_json::Number::from(written as i64))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_cipher_publish_workflow_u3(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u3_cipher_publish_workflow_impl(&pool, &body.0).await
}

#[axum::debug_handler]
pub async fn document_cipher_publish_workflow_mock_u3(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u3_cipher_publish_workflow_impl(&pool, &body.0).await
}

#[axum::debug_handler]
pub async fn document_cipher_permission_read_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, person)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let doc = client
        .query_opt(
            "SELECT creator, author_id, status FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let doc = match doc {
        Some(d) => d,
        None => return Ok(Json(ActionResult::error("document not found"))),
    };
    let creator: String = doc.get::<_, Option<String>>("creator").unwrap_or_default();
    let status: String = doc.get::<_, Option<String>>("status").unwrap_or_default();
    let readable = shared::middleware::is_admin(&pool, &person).await
        || person == creator
        || person == doc.get::<_, Option<String>>("author_id").unwrap_or_default()
        || status == "published";
    let _ = &session;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("person".to_string(), Value::String(person)),
            ("readable".to_string(), Value::Bool(readable)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_draft_next_u3(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(
        &pool,
        "x_cms_data_document",
        "deleted_at IS NULL AND status = 'draft'",
        &[],
    )
    .await
}

#[axum::debug_handler]
pub async fn document_filter_next_u3(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(
        &pool,
        "x_cms_data_document",
        "deleted_at IS NULL ORDER BY id DESC LIMIT 200",
        &[],
    )
    .await
}

#[axum::debug_handler]
pub async fn document_filter_prev_u3(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(
        &pool,
        "x_cms_data_document",
        "deleted_at IS NULL ORDER BY id ASC LIMIT 200",
        &[],
    )
    .await
}

#[axum::debug_handler]
pub async fn document_filter_paging_u3(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(
        &pool,
        "x_cms_data_document",
        "deleted_at IS NULL ORDER BY create_time DESC LIMIT 200",
        &[],
    )
    .await
}

#[axum::debug_handler]
pub async fn document_filter_paging_manager_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    list_from_table_filtered_java(
        &pool,
        "x_cms_data_document",
        "ORDER BY create_time DESC LIMIT 200",
        &[],
    )
    .await
}

#[axum::debug_handler]
pub async fn document_control_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT creator, status FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = match row {
        Some(r) => r,
        None => return Ok(Json(ActionResult::error("document not found"))),
    };
    let creator: String = row.get::<_, Option<String>>("creator").unwrap_or_default();
    let admin = shared::middleware::is_admin(&pool, &session.person_unique).await;
    let allow_manage = admin || creator == session.person_unique;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("allowRead".to_string(), Value::Bool(true)),
            ("allowEdit".to_string(), Value::Bool(allow_manage)),
            ("allowManage".to_string(), Value::Bool(allow_manage)),
            ("controller".to_string(), Value::String(if admin { "admin".to_string() } else { creator })),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_notify_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u3_gate_document(&pool, &session, &id).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            client
                .execute(
                    "INSERT INTO x_cms_log (id, doc_id, person_id, operation_level, operation_type, operation_detail) \
                     VALUES (gen_random_uuid()::text, $1, $2, 'info', 'notify', 'publish notify')",
                    &[&id, &session.person_unique],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("notified".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn document_permission_read_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT creator, author_id, status FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = match row {
        Some(r) => r,
        None => return Ok(Json(ActionResult::error("document not found"))),
    };
    let admin = shared::middleware::is_admin(&pool, &session.person_unique).await;
    let readable = admin
        || row.get::<_, Option<String>>("creator").unwrap_or_default() == session.person_unique
        || row.get::<_, Option<String>>("author_id").unwrap_or_default() == session.person_unique
        || row.get::<_, Option<String>>("status").unwrap_or_default() == "published";
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("readable".to_string(), Value::Bool(readable)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_persons_u3(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let doc = client
        .query_opt(
            "SELECT app_id, category_id, creator FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let doc = match doc {
        Some(d) => d,
        None => return Ok(Json(ActionResult::error("document not found"))),
    };
    let app_id: String = doc.get::<_, Option<String>>("app_id").unwrap_or_default();
    let category_id: String = doc.get::<_, Option<String>>("category_id").unwrap_or_default();
    let rows = client
        .query(
            "SELECT DISTINCT person_id FROM x_cms_permission \
             WHERE deleted_at IS NULL AND person_id IS NOT NULL \
               AND ((app_id <> '' AND app_id = $1) OR (category_id <> '' AND category_id = $2)) \
             UNION \
             SELECT creator FROM x_cms_data_document WHERE id = $3 AND creator IS NOT NULL",
            &[&app_id, &category_id, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let persons: Vec<Value> = rows
        .iter()
        .filter_map(|r| r.try_get::<_, Option<String>>(0).ok().flatten())
        .map(Value::String)
        .collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("count".to_string(), Value::Number(serde_json::Number::from(persons.len() as i64))),
            ("persons".to_string(), Value::Array(persons)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn document_publish_html_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u3_gate_document(&pool, &session, &id).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("document not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let row = client
                .query_opt(
                    "SELECT title, content FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let row = match row {
                Some(r) => r,
                None => return Ok(Json(ActionResult::error("document not found"))),
            };
            let title: String = row.get::<_, Option<String>>("title").unwrap_or_default();
            let content: String = row.get::<_, Option<String>>("content").unwrap_or_default();
            let html = format!("<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>{}</title></head><body>{}</body></html>", title, content);
            u3_field_upsert(&client, &id, "__publish_html__", &html).await?;
            let url = format!("documents/{}.html", id);
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("url".to_string(), Value::String(url)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn document_list_document_data_u3(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids = u2_body_strs(&body, "docIds");
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let docs = if ids.is_empty() {
        client
            .query(
                "SELECT * FROM x_cms_data_document WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 50",
                &[],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT * FROM x_cms_data_document WHERE deleted_at IS NULL AND id = ANY($1)",
                &[&ids],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };
    let mut data: Vec<Value> = Vec::with_capacity(docs.len());
    for doc in &docs {
        let doc_id: String = doc.get("id");
        let field_rows = client
            .query(
                "SELECT field_name, field_value FROM x_cms_data_document_field \
                 WHERE doc_id = $1 AND deleted_at IS NULL",
                &[&doc_id],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        let mut map = serde_json::Map::new();
        for fr in &field_rows {
            let name: String = fr.get("field_name");
            let value: String = fr.get::<_, Option<String>>("field_value").unwrap_or_default();
            map.insert(name, Value::String(value));
        }
        let mut obj = row_to_json(doc);
        if let Some(o) = obj.as_object_mut() {
            o.insert("data".to_string(), Value::Object(map));
        }
        data.push(obj);
    }
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

// ── file 域（canonical 写路径与复制）───────────────────────────────────────

#[axum::debug_handler]
pub async fn file_update_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_file", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("file not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let name = u2_body_str(&body, "name");
            let content_type = u2_body_str(&body, "contentType");
            let affected = client
                .execute(
                    "UPDATE x_cms_file SET \
                     name = COALESCE($2, name), \
                     content_type = COALESCE($3, content_type) \
                     WHERE id = $1 AND deleted_at IS NULL",
                    &[&id, &name, &content_type],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if affected == 0 {
                return Ok(Json(ActionResult::error("file not found")));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("updated".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn file_delete_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "x_cms_file", "creator", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("file not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            soft_delete_by_id(&pool, "x_cms_file", &id).await?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn file_copy_u3(
    pool: Extension<Pool>,
    axum::extract::Path((id, app_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let source = client
        .query_opt(
            "SELECT name, size, content_type, content_base64, creator FROM x_cms_file \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let source = match source {
        Some(s) => s,
        None => return Ok(Json(ActionResult::error("file not found"))),
    };
    let new_id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_cms_file (id, app_id, name, size, content_type, content_base64, creator) \
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[
                &new_id,
                &app_flag,
                &source.get::<_, Option<String>>("name"),
                &source.get::<_, i64>("size"),
                &source.get::<_, Option<String>>("content_type"),
                &source.get::<_, Option<String>>("content_base64"),
                &source.get::<_, Option<String>>("creator"),
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("copiedTo".to_string(), Value::String(new_id)),
            ("appId".to_string(), Value::String(app_flag)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_download_with_app_u3(
    pool: Extension<Pool>,
    axum::extract::Path((flag, app_info_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, size, content_type, creator, create_time::text FROM x_cms_file \
             WHERE id = $1 AND app_id = $2 AND deleted_at IS NULL",
            &[&flag, &app_info_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

// ── fileinfo 域（canonical）────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn fileinfo_binary_base64_u3(
    pool: Extension<Pool>,
    axum::extract::Path((id, size)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT f.original_name, fl.content_base64 FROM x_cms_fileinfo f \
             LEFT JOIN x_cms_file fl ON fl.id = f.file_id \
             WHERE f.id = $1 AND f.deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = match row {
        Some(r) => r,
        None => return Ok(Json(ActionResult::error("fileinfo not found"))),
    };
    let original_name: String = row.get::<_, Option<String>>("original_name").unwrap_or_default();
    let base64_raw: String = row.get::<_, Option<String>>("content_base64").unwrap_or_default();
    let limit = size.parse::<usize>().unwrap_or(0);
    let truncated: String = if limit > 0 && base64_raw.len() > limit {
        base64_raw[..limit].to_string()
    } else {
        base64_raw
    };
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(original_name)),
            ("base64".to_string(), Value::String(truncated)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn fileinfo_upload_with_url_u3(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let doc_id = match u2_body_str(&body, "docId") {
        Some(d) if !d.is_empty() => d,
        _ => return Err(AppError::BadRequest("docId required".to_string())),
    };
    let url = u2_body_str(&body, "url").unwrap_or_default();
    if url.is_empty() {
        return Err(AppError::BadRequest("url required".to_string()));
    }
    let original_name = u2_body_str(&body, "originalName").unwrap_or_else(|| url.clone());
    let size = u2_body_i64(&body, "size").unwrap_or(0);
    let content_type = u2_body_str(&body, "contentType").unwrap_or_default();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "INSERT INTO x_cms_fileinfo (id, doc_id, original_name, size, content_type) \
             VALUES (gen_random_uuid()::text, $1, $2, $3, $4) RETURNING *",
            &[&doc_id, &original_name, &size, &content_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

// ── form 域（canonical getWithAppInfo）─────────────────────────────────────

#[axum::debug_handler]
pub async fn form_get_with_appinfo_u3(
    pool: Extension<Pool>,
    axum::extract::Path((form_flag, app_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT * FROM x_cms_form WHERE id = $1 AND app_id = $2 AND deleted_at IS NULL",
            &[&form_flag, &app_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

// ── permission save 家族（manager/publisher/viewer × appInfo/categoryInfo）──

macro_rules! u3_permission_save_handler {
    ($fn_name:ident, $role:expr, $scope:expr) => {
        #[axum::debug_handler]
        pub async fn $fn_name(
            pool: Extension<Pool>,
            session: Extension<shared::session::Session>,
            axum::extract::Path(id): axum::extract::Path<String>,
            body: axum::extract::Json<Value>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            u3_save_scope_permissions(&pool, &session, $scope, &id, $role, &body.0).await
        }
    };
}

u3_permission_save_handler!(permission_save_manager_app_u3, "manager", "appInfo");
u3_permission_save_handler!(permission_save_publisher_app_u3, "publisher", "appInfo");
u3_permission_save_handler!(permission_save_viewer_app_u3, "viewer", "appInfo");
u3_permission_save_handler!(permission_save_manager_category_u3, "manager", "categoryInfo");
u3_permission_save_handler!(permission_save_publisher_category_u3, "publisher", "categoryInfo");
u3_permission_save_handler!(permission_save_viewer_category_u3, "viewer", "categoryInfo");

// ── script nested import / load ────────────────────────────────────────────

#[axum::debug_handler]
pub async fn script_post_nested_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((unique_name, flag)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_gate_by_sql(
        &pool,
        "SELECT manager AS owner FROM x_cms_appinfo WHERE id = $1 AND deleted_at IS NULL",
        &flag,
        &session.person_unique,
    )
    .await?
    {
        U2Gate::NotFound => Ok(Json(ActionResult::error("application not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let scripts = body
                .get("importedScripts")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            if scripts.is_empty() {
                return Err(AppError::BadRequest("importedScripts required".to_string()));
            }
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let mut imported = 0u64;
            for script in &scripts {
                let name = script.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let content =
                    script.get("scriptContent").and_then(|v| v.as_str()).unwrap_or("");
                imported += client
                    .execute(
                        "INSERT INTO x_cms_script (id, app_id, name, script_content, imported, creator) \
                         VALUES (gen_random_uuid()::text, $1, $2, $3, true, $4)",
                        &[&flag, &name, &content, &session.person_unique],
                    )
                    .await
                    .map_err(|_| AppError::Internal)?;
            }
            let _ = unique_name;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("appId".to_string(), Value::String(flag)),
                    ("imported".to_string(), Value::Number(serde_json::Number::from(imported as i64))),
                ]),
            ))))
        }
    }
}

#[axum::debug_handler]
pub async fn script_load_u3(
    pool: Extension<Pool>,
    axum::extract::Path((unique_name, app_flag)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let imported_only = body
        .get("imported")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let where_clause = if imported_only {
        "app_id = $2 AND imported AND deleted_at IS NULL AND (name = $1 OR id = $1)"
    } else {
        "app_id = $2 AND deleted_at IS NULL AND (name = $1 OR id = $1)"
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            &format!(
                "SELECT id, name, script_content, imported FROM x_cms_script WHERE {}",
                where_clause
            ),
            &[&unique_name, &app_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let scripts: Vec<Value> = rows.iter().map(row_to_json).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(scripts.len() as i64))),
            ("scripts".to_string(), Value::Array(scripts)),
        ]),
    ))))
}

// ── viewrecord ─────────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn viewrecord_by_person_u3(
    pool: Extension<Pool>,
    axum::extract::Path(person): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_from_table_filtered_java(
        &pool,
        "x_cms_viewrecord",
        "deleted_at IS NULL AND person_id = $1",
        &[&person],
    )
    .await
}

#[axum::debug_handler]
pub async fn viewrecord_unread_u3(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id FROM x_cms_data_document \
             WHERE status = 'published' AND deleted_at IS NULL \
               AND id NOT IN (SELECT doc_id FROM x_cms_viewrecord WHERE person_id = $1 AND deleted_at IS NULL) \
             LIMIT 100",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let unread: Vec<Value> = rows
        .iter()
        .map(|r| Value::String(r.get::<_, String>("id")))
        .collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("person".to_string(), Value::String(session.person_unique.clone())),
            ("count".to_string(), Value::Number(serde_json::Number::from(unread.len() as i64))),
            ("docIds".to_string(), Value::Array(unread)),
        ]),
    ))))
}
