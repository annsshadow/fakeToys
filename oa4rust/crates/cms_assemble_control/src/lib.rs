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
use search::{search_documents, Document};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


#[axum::debug_handler]
pub async fn application_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, alias, appType, icon, enabled, manager FROM x_cms_appinfo WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("appType".to_string(), Value::String(row.get("appType"))),
                ("icon".to_string(), Value::String(row.get::<_, Option<String>>("icon").unwrap_or_default())),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("manager".to_string(), Value::String(row.get("manager"))),
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
            "SELECT enabled, max_category_count, allow_anonymous FROM x_cms_assemble_control_config ORDER BY create_time LIMIT 1",
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
            "SELECT id, name, enabled FROM x_cms_assemble_control_section ORDER BY create_time",
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

    Ok(Json(ActionResult::success(Value::Array(sections))))
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
            "UPDATE x_cms_assemble_control_config SET enabled = $1, max_category_count = $2, allow_anonymous = $3 WHERE id = (SELECT id FROM x_cms_assemble_control_config ORDER BY create_time LIMIT 1)",
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

async fn list_from_table(
    pool: &Pool,
    table: &str,
    where_clause: &str,
    params: &[(&(dyn ToSql + Sync), &str)],
) -> Result<Value, AppError> {
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

    Ok(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ("data".to_string(), Value::Array(data)),
    ])))
}

async fn list_from_table_filtered(
    pool: &Pool,
    table: &str,
    where_clause: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<Value, AppError> {
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

    Ok(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ("data".to_string(), Value::Array(data)),
    ])))
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
            &format!("SELECT * FROM {} WHERE id = $1 AND deleted_at IS NULL", table),
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
    client
        .execute(
            &format!("UPDATE {} SET deleted_at = NOW() WHERE id = $1", table),
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Value::Object(serde_json::Map::from_iter([
        ("deleted".to_string(), Value::Bool(true)),
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
    let data = list_from_table_filtered(&pool, "x_cms_data_document", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn anonymous_document_filter_list_id_next_count_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_data_document", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn anonymous_document_filter_list_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_data_document", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn anonymous_document_filter_list_page_size_size_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_data_document", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn anonymous_document_id_view(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, content, author_id, status, publish_time, creator, create_time FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_fileinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
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
            "SELECT id, alias, app_type, icon, enabled, manager FROM x_cms_appinfo WHERE alias = $1 AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_filter_list_id_next_count_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_filter_list_id_prev_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_filter_list_id_prev_count_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_get_user_publish_appId(
    pool: Extension<Pool>,
    axum::extract::Path(app_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, alias, app_type, icon, enabled, manager FROM x_cms_appinfo WHERE id = $1 AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_appType_manager(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_has_document(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_has_document_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_has_document_type_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_manage(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_manage_type_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_user_publish(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_user_publish_type_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_user_publish_with_process(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_user_view(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_user_view_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_user_view_all_type_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_user_view_article_type_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_user_view_data(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_list_user_view_data_type_appType(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_appId_icon_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((app_id, icon_size)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, alias, app_type, icon, enabled, manager FROM x_cms_appinfo WHERE id = $1 AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_appinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn appinfo_id_control(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, alias, app_type, icon, enabled, manager FROM x_cms_appinfo WHERE id = $1 AND deleted_at IS NULL",
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
            "SELECT id, app_id, category_id, person_id, role_type, permission_level FROM x_cms_permission WHERE app_id = $1 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
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
            "SELECT id, name, parent_id, app_id, sort_order, status, ext_content, creator, create_time FROM x_cms_categoryinfo WHERE alias = $1 AND deleted_at IS NULL",
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
            "SELECT id, name, parent_id, app_id, sort_order, status, ext_content, creator, create_time FROM x_cms_categoryinfo WHERE id = $1 AND deleted_at IS NULL",
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
            "INSERT INTO x_cms_categoryinfo (id, name, parent_id, app_id) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET name = $2, parent_id = $3, app_id = $4",
            &[&category_id, &name, &parent_id, &app_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
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
            "SELECT id, ext_content FROM x_cms_categoryinfo WHERE id = $1 AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_filter_list_id_next_count_app_appId_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_filter_list_id_prev_count_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_filter_list_id_prev_count_app_appId_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_filter_list_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_filter_list_page_size_size_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_list_manage_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_list_objects(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_list_publish_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_list_view_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_list_view_app_appId_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_list_view_app_appId_data(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_flag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_categoryinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn categoryinfo_id_control(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, parent_id, app_id, sort_order, status, ext_content, creator, create_time FROM x_cms_categoryinfo WHERE id = $1 AND deleted_at IS NULL",
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
            "SELECT id, name, parent_id, app_id, sort_order, status, ext_content, creator, create_time FROM x_cms_categoryinfo WHERE id = $1 AND deleted_at IS NULL",
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
            "SELECT id, app_id, category_id, person_id, role_type, permission_level FROM x_cms_permission WHERE category_id = $1 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

// ─── comment_* / commend_* stubs ────────────────────────────────────────────

#[axum::debug_handler]
pub async fn commend_list_paging_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_commend", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn commend_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_commend", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn comment_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn comment_list_id_next_count_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn comment_list_id_prev_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn comment_list_id_prev_count_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn comment_list_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn comment_list_page_size_size_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn comment_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_comment", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn comment_id_commend(
    pool: Extension<Pool>,
    axum::extract::Path((comment_id, person_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_cms_commend (id, doc_id, person_id) VALUES (gen_random_uuid()::text, (SELECT doc_id FROM x_cms_comment WHERE id = $1), $2)",
            &[&comment_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
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
    client
        .execute(
            "DELETE FROM x_cms_commend WHERE doc_id = (SELECT doc_id FROM x_cms_comment WHERE id = $1) AND person_id = $2",
            &[&comment_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// ─── correlation_* stubs ────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn correlation_doc_docId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_correlation", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
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
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn correlation_list_doc_docId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_correlation", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn correlation_list_doc_docId_site_site(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_correlation", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
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
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
}

// ─── data_document_* stubs ──────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn data_document_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_data_document", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn data_document_id_array_data(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, content, author_id, status, publish_time, creator, create_time FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL",
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
            Ok(Json(ActionResult::success(Value::Array(data))))
        }
        None => Ok(Json(ActionResult::success(Value::Array(vec![])))),
    }
}

#[axum::debug_handler]
pub async fn data_document_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_data_document", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let title = body.get("title").and_then(|v| v.as_str()).unwrap_or("");
    let content = body.get("content").and_then(|v| v.as_str()).unwrap_or("");
    let author_id = body.get("authorId").and_then(|v| v.as_str()).unwrap_or("");
    let status = body.get("status").and_then(|v| v.as_str()).unwrap_or("draft");
    client
        .execute(
            "INSERT INTO x_cms_data_document (id, title, content, author_id, status) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET title = $2, content = $3, author_id = $4, status = $5",
            &[&id, &title, &content, &author_id, &status],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, path0)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, field_name, field_value, create_time FROM x_cms_data_document_field WHERE doc_id = $1 AND field_name = $2 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, path0)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_data_document_field SET deleted_at = NOW() WHERE doc_id = $1 AND field_name = $2",
            &[&doc_id, &path0],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, path0)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let field_value = body.get("fieldValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (doc_id, field_name) DO UPDATE SET field_value = $3",
            &[&doc_id, &path0, &field_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_data_document_field", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, path0, path1)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_data_document_field SET deleted_at = NOW() WHERE doc_id = $1 AND field_name = $2",
            &[&doc_id, &path0],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, path0, path1)): axum::extract::Path<(String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let field_value = body.get("fieldValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (doc_id, field_name) DO UPDATE SET field_value = $3",
            &[&doc_id, &path0, &field_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, path0, path1, path2)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, field_name, field_value, create_time FROM x_cms_data_document_field WHERE doc_id = $1 AND deleted_at IS NULL",
            &[&doc_id],
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, path0, path1, path2)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_data_document_field SET deleted_at = NOW() WHERE doc_id = $1 AND field_name = $2",
            &[&doc_id, &path0],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, path0, path1, path2)): axum::extract::Path<(String, String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let field_value = body.get("fieldValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (doc_id, field_name) DO UPDATE SET field_value = $3",
            &[&doc_id, &path0, &field_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, path0, path1, path2, path3)): axum::extract::Path<(String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, field_name, field_value, create_time FROM x_cms_data_document_field WHERE doc_id = $1 AND deleted_at IS NULL",
            &[&doc_id],
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, path0, path1, path2, path3)): axum::extract::Path<(String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_data_document_field SET deleted_at = NOW() WHERE doc_id = $1 AND field_name = $2",
            &[&doc_id, &path0],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, path0, path1, path2, path3)): axum::extract::Path<(String, String, String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let field_value = body.get("fieldValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (doc_id, field_name) DO UPDATE SET field_value = $3",
            &[&doc_id, &path0, &field_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_path4(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _p0, _p1, _p2, _p3, _p4)): axum::extract::Path<(String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, field_name, field_value, create_time FROM x_cms_data_document_field WHERE doc_id = $1 AND deleted_at IS NULL",
            &[&doc_id],
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_path4_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _p0, _p1, _p2, _p3, _p4)): axum::extract::Path<(String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_data_document_field SET deleted_at = NOW() WHERE doc_id = $1 AND field_name = $2",
            &[&doc_id, &_p0],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_path4_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _p0, _p1, _p2, _p3, _p4)): axum::extract::Path<(String, String, String, String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let field_value = body.get("fieldValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (doc_id, field_name) DO UPDATE SET field_value = $3",
            &[&doc_id, &_p0, &field_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_path4_path5(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _p0, _p1, _p2, _p3, _p4, _p5)): axum::extract::Path<(String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, field_name, field_value, create_time FROM x_cms_data_document_field WHERE doc_id = $1 AND deleted_at IS NULL",
            &[&doc_id],
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _p0, _p1, _p2, _p3, _p4, _p5)): axum::extract::Path<(String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_data_document_field SET deleted_at = NOW() WHERE doc_id = $1 AND field_name = $2",
            &[&doc_id, &_p0],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_path4_path5_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _p0, _p1, _p2, _p3, _p4, _p5)): axum::extract::Path<(String, String, String, String, String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let field_value = body.get("fieldValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (doc_id, field_name) DO UPDATE SET field_value = $3",
            &[&doc_id, &_p0, &field_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_path4_path5_path6(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _p0, _p1, _p2, _p3, _p4, _p5, _p6)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, field_name, field_value, create_time FROM x_cms_data_document_field WHERE doc_id = $1 AND deleted_at IS NULL",
            &[&doc_id],
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _p0, _p1, _p2, _p3, _p4, _p5, _p6)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_data_document_field SET deleted_at = NOW() WHERE doc_id = $1 AND field_name = $2",
            &[&doc_id, &_p0],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _p0, _p1, _p2, _p3, _p4, _p5, _p6)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let field_value = body.get("fieldValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (doc_id, field_name) DO UPDATE SET field_value = $3",
            &[&doc_id, &_p0, &field_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_path4_path5_path6_path7(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _p0, _p1, _p2, _p3, _p4, _p5, _p6, _p7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, field_name, field_value, create_time FROM x_cms_data_document_field WHERE doc_id = $1 AND deleted_at IS NULL",
            &[&doc_id],
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _p0, _p1, _p2, _p3, _p4, _p5, _p6, _p7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_data_document_field SET deleted_at = NOW() WHERE doc_id = $1 AND field_name = $2",
            &[&doc_id, &_p0],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, _p0, _p1, _p2, _p3, _p4, _p5, _p6, _p7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let field_value = body.get("fieldValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_data_document_field (id, doc_id, field_name, field_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (doc_id, field_name) DO UPDATE SET field_value = $3",
            &[&doc_id, &_p0, &field_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// ─── design_* / document_cipher_* stubs ─────────────────────────────────────

#[axum::debug_handler]
pub async fn design_appdict_list_appInfo_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_surface_appdict", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn design_appdict_list_paging_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_surface_appdict", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn design_appdict_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_surface_appdict", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn design_appdict_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_surface_appdict", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
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
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
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
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_info_flag ILIKE $1 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn document_cipher_filter_list_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_document_cipher", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn document_cipher_filter_list_page_size_size_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_document_cipher", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
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
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
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
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn document_cipher_id_permission_read_person_person(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, person_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, cipher_text, person_id, create_time FROM x_cms_document_cipher WHERE doc_id = $1 AND person_id = $2 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// ─── file* / fileinfo_* stubs ───────────────────────────────────────────────

#[axum::debug_handler]
pub async fn file_list_appInfo_appInfoFlag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_file", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn file_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_file", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn file_list_id_prev_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_file", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn file_flag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_file", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn file_flag_appInfo_appInfoFlag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_file", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn file_flag_appInfo_appInfoFlag_content(
    pool: Extension<Pool>,
    axum::extract::Path((app_info_flag, app_info_flag2)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, size, content_type, content_base64, creator, create_time FROM x_cms_file WHERE app_id = $1 AND deleted_at IS NULL LIMIT 1",
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
            "SELECT id, app_id, name, size, content_type, creator, create_time FROM x_cms_file WHERE app_id = $1 AND deleted_at IS NULL LIMIT 1",
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
    soft_delete_by_id(&pool, "x_cms_file", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn file_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_file", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn file_id_content(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, content_base64, content_type FROM x_cms_file WHERE id = $1 AND deleted_at IS NULL",
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
            "SELECT id, name, size, content_type, content_base64 FROM x_cms_file WHERE id = $1 AND deleted_at IS NULL",
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
    client
        .execute(
            "INSERT INTO x_cms_file (id, name, content_base64, content_type, size) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET name = $2, content_base64 = $3, content_type = $4, size = $5",
            &[&id, &name, &content_base64, &content_type, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
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
    client
        .execute(
            "INSERT INTO x_cms_file (id, name, content_base64, content_type, size) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET name = $2, content_base64 = $3, content_type = $4, size = $5",
            &[&id, &name, &content_base64, &content_type, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn anonymous_fileinfo_download_document_id(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time FROM x_cms_fileinfo WHERE doc_id = $1 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn anonymous_fileinfo_download_document_id_stream(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time FROM x_cms_fileinfo WHERE doc_id = $1 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn fileinfo_batch_download_doc_docId_site_site(
    pool: Extension<Pool>,
    axum::extract::Path((doc_id, site)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time FROM x_cms_fileinfo WHERE doc_id = $1 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
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
    client
        .execute(
            "INSERT INTO x_cms_fileinfo (id, doc_id, file_id, original_name, size, content_type) VALUES (gen_random_uuid()::text, $1, $2, $3, $4, $5)",
            &[&doc_id, &file_id, &original_name, &size, &content_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn fileinfo_download_document_id(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time FROM x_cms_fileinfo WHERE doc_id = $1 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn fileinfo_download_document_id_stream(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time FROM x_cms_fileinfo WHERE doc_id = $1 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn fileinfo_download_transfer_flag_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time FROM x_cms_fileinfo WHERE id = $1 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
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
    client
        .execute(
            "UPDATE x_cms_fileinfo SET original_name = $1, size = $2, content_type = $3 WHERE file_id = $4 AND doc_id = $5",
            &[&original_name, &size, &content_type, &file_id, &doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
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
    client
        .execute(
            "UPDATE x_cms_fileinfo SET original_name = $1, size = $2, content_type = $3 WHERE file_id = $4 AND doc_id = $5",
            &[&original_name, &size, &content_type, &file_id, &doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn fileinfo_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_fileinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn fileinfo_list_document_documentId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_fileinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn fileinfo_list_filter(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_fileinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
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
    client
        .execute(
            "UPDATE x_cms_fileinfo SET original_name = $1, size = $2, content_type = $3 WHERE file_id = $4 AND doc_id = $5",
            &[&original_name, &size, &content_type, &file_id, &doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
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
    client
        .execute(
            "UPDATE x_cms_fileinfo SET original_name = $1, size = $2, content_type = $3 WHERE doc_id = $4 AND file_id = $5",
            &[&original_name, &size, &content_type, &doc_id, &attachment_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
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
    client
        .execute(
            "UPDATE x_cms_fileinfo SET original_name = $1, size = $2, content_type = $3 WHERE doc_id = $4 AND file_id = $5",
            &[&original_name, &size, &content_type, &doc_id, &attachment_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
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
    client
        .execute(
            "UPDATE x_cms_fileinfo SET original_name = $1, size = $2, content_type = $3 WHERE id = $4",
            &[&original_name, &size, &content_type, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
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
    client
        .execute(
            "INSERT INTO x_cms_fileinfo (id, doc_id, original_name, size, content_type) VALUES (gen_random_uuid()::text, $1, $2, $3, $4)",
            &[&doc_id, &original_name, &size, &content_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
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
    client
        .execute(
            "INSERT INTO x_cms_fileinfo (id, doc_id, original_name, size, content_type) VALUES (gen_random_uuid()::text, $1, $2, $3, $4)",
            &[&doc_id, &original_name, &size, &content_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
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
    client
        .execute(
            "INSERT INTO x_cms_fileinfo (id, doc_id, original_name, size, content_type) VALUES (gen_random_uuid()::text, $1, $2, $3, $4)",
            &[&doc_id, &original_name, &size, &content_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
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
    client
        .execute(
            "INSERT INTO x_cms_fileinfo (id, doc_id, original_name, size, content_type) VALUES (gen_random_uuid()::text, $1, $2, $3, $4)",
            &[&doc_id, &original_name, &size, &content_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn fileinfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_fileinfo", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn fileinfo_id_binary_base64_size(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, original_name, size, content_type FROM x_cms_fileinfo WHERE id = $1 AND deleted_at IS NULL",
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
    client
        .execute(
            "UPDATE x_cms_fileinfo SET create_time = NOW() WHERE file_id = $1 AND doc_id = $2",
            &[&file_id, &doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn fileinfo_id_document_documentId(
    pool: Extension<Pool>,
    axum::extract::Path((file_id, doc_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, file_id, original_name, size, content_type FROM x_cms_fileinfo WHERE file_id = $1 AND doc_id = $2 AND deleted_at IS NULL",
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
    soft_delete_by_id(&pool, "x_cms_fileinfo", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn fileinfo_id_online_info(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, doc_id, file_id, original_name, size, content_type, upload_person, create_time FROM x_cms_fileinfo WHERE id = $1 AND deleted_at IS NULL",
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
            "SELECT id, doc_id, file_id, original_name, size, content_type FROM x_cms_fileinfo WHERE id = $1 AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn form_filter_list_id_next_count_app_appId_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn form_filter_list_id_prev_count_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn form_filter_list_id_prev_count_app_appId_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn form_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn form_list_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn form_list_formfield_appInfo_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_form_field", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn form_list_id_formfield(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_form_field", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn anonymous_form_v2_lookup_document_docId(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status FROM x_cms_form_v2 WHERE id = (SELECT form_id FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL) AND deleted_at IS NULL",
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
            "SELECT id, app_id, name, definition, status FROM x_cms_form_v2 WHERE id = (SELECT form_id FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL) AND deleted_at IS NULL",
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
            "SELECT id, app_id, name, definition, status FROM x_cms_form_v2 WHERE id = $1 AND deleted_at IS NULL",
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
            "SELECT id, app_id, name, definition, status FROM x_cms_form_v2 WHERE id = $1 AND deleted_at IS NULL",
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
            "SELECT id, app_id, name, definition, status FROM x_cms_form WHERE id = $1 AND deleted_at IS NULL",
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
            "SELECT id, app_id, name, definition, status, creator, create_time FROM x_cms_form WHERE app_id = $1 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn form_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_form", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn form_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    soft_delete_by_id(&pool, "x_cms_form", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
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
    client
        .execute(
            "INSERT INTO x_cms_form (id, name, definition, status) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET name = $2, definition = $3, status = $4",
            &[&id, &name, &definition, &status],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn form_v2_lookup_document_docId(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status, creator, create_time FROM x_cms_form_v2 WHERE id = (SELECT form_id FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL) AND deleted_at IS NULL",
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
            "SELECT id, app_id, name, definition, status, creator, create_time FROM x_cms_form_v2 WHERE id = (SELECT form_id FROM x_cms_data_document WHERE id = $1 AND deleted_at IS NULL) AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_form_v2", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn form_v2_id_mobile(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status, creator, create_time FROM x_cms_form_v2 WHERE id = $1 AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_form_v2", &format!("deleted_at IS NULL AND id = '{}'", form_id), &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn formversion_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status, creator, create_time FROM x_cms_form_v2 WHERE id = $1 AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_log", "", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn log_filter_list_id_prev_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_log", "", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn log_list_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_log", "", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn log_list_category_categoryId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_log", "", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn log_list_document_documentId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_log", "", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn log_list_filter_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_log", "", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn log_list_level_operationLevel(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_log", "", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn log_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_log", "", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

// ─── output_* stubs ─────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn output_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_output", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn output_appInfoFlag_select(
    pool: Extension<Pool>,
    axum::extract::Path(app_info_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_id, name, config, creator, create_time FROM x_cms_output WHERE app_id = $1 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
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
    client
        .execute(
            "INSERT INTO x_cms_output (id, app_id, name, config) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = $2, config = $3",
            &[&app_info_flag, &name, &config],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// ─── permission_* stubs ─────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn permission_appInfo_id_manageable(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'manage'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_appInfo_id_managers(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'manager'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_appInfo_id_publishers(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'publisher'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_appInfo_id_viewers(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'viewer'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_category_id_managers(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'manager'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_category_id_publishers(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'publisher'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_category_id_viewers(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'viewer'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_categoryInfo_id_manageable(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'manage'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_management_refresh_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_cms_permission SET deleted_at = NOW() WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn permission_management_refresh_category_categoryId(
    pool: Extension<Pool>,
    axum::extract::Path(category_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_cms_permission SET deleted_at = NOW() WHERE category_id = $1 AND deleted_at IS NULL", &[&category_id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn permission_manager_appInfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'manager'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_manager_categoryInfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'manager'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_publisher_appInfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'publisher'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_publisher_categoryInfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'publisher'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_viewer_appInfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'viewer'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn permission_viewer_categoryInfo_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_permission", "deleted_at IS NULL AND role_type = 'viewer'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
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
            "SELECT id, doc_id, person_id, create_time FROM x_cms_comment WHERE content ILIKE $1 AND deleted_at IS NULL",
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
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn script_list_app_appId_name_name(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn script_list_app_flag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn script_list_manager(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn script_list_paging_page_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn script_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn script_list_id_prev_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn script_flag_appInfo_appInfoFlag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn script_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_script", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
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
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn script_uniqueName_app_flag(
    pool: Extension<Pool>,
    axum::extract::Path((app_flag, unique_name)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, unique_name, script_content, imported, creator, create_time FROM x_cms_script WHERE app_id = $1 AND unique_name = $2 AND deleted_at IS NULL",
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
            "SELECT id, app_id, name, unique_name, script_content, imported, creator, create_time FROM x_cms_script WHERE app_id = $1 AND unique_name = $2 AND imported = true AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_script", &format!("deleted_at IS NULL AND id = '{}'", script_id), &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn scriptversion_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status, creator, create_time FROM x_cms_form_v2 WHERE id = $1 AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_searchfilter", "deleted_at IS NULL AND filter_type = 'archive'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn searchfilter_list_draft_filter_category_categoryId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_searchfilter", "deleted_at IS NULL AND filter_type = 'draft'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn searchfilter_list_publish_filter_category_categoryId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_searchfilter", "deleted_at IS NULL AND filter_type = 'publish'", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

// ─── surface_appdict_* stubs ────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn anonymous_surface_appdict_list_appInfo_appInfoFlag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_surface_appdict", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("pathLevels".to_string(), Value::String(row.get::<_, Option<String>>("path_levels").unwrap_or_default())),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
            ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, path1)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, path1, path2)): axum::extract::Path<(String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, path1, path2, path3)): axum::extract::Path<(String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, path1, path2, path3, path4)): axum::extract::Path<(String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4, _p5)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4, _p5, _p6)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4, _p5, _p6, _p7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn surface_appdict_list_appInfo_appInfoFlag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_surface_appdict", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("pathLevels".to_string(), Value::String(row.get::<_, Option<String>>("path_levels").unwrap_or_default())),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag)): axum::extract::Path<(String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data_value = body.get("dataValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_surface_appdict (id, app_info_flag, app_dict_flag, data_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (id) DO UPDATE SET data_value = $3",
            &[&app_info_flag, &app_dict_flag, &data_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_surface_appdict SET deleted_at = NOW() WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, path1)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, path1)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_surface_appdict SET deleted_at = NOW() WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, path1, path2)): axum::extract::Path<(String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, path1, path2)): axum::extract::Path<(String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_surface_appdict SET deleted_at = NOW() WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, path1, path2)): axum::extract::Path<(String, String, String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data_value = body.get("dataValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_surface_appdict (id, app_info_flag, app_dict_flag, data_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (id) DO UPDATE SET data_value = $3",
            &[&app_info_flag, &app_dict_flag, &data_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, path1, path2, path3)): axum::extract::Path<(String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, path1, path2, path3)): axum::extract::Path<(String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_surface_appdict SET deleted_at = NOW() WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, path1, path2, path3)): axum::extract::Path<(String, String, String, String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data_value = body.get("dataValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_surface_appdict (id, app_info_flag, app_dict_flag, data_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (id) DO UPDATE SET data_value = $3",
            &[&app_info_flag, &app_dict_flag, &data_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4)): axum::extract::Path<(String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4)): axum::extract::Path<(String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_surface_appdict SET deleted_at = NOW() WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4)): axum::extract::Path<(String, String, String, String, String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data_value = body.get("dataValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_surface_appdict (id, app_info_flag, app_dict_flag, data_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (id) DO UPDATE SET data_value = $3",
            &[&app_info_flag, &app_dict_flag, &data_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4, _p5)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4, _p5)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_surface_appdict SET deleted_at = NOW() WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4, _p5)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data_value = body.get("dataValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_surface_appdict (id, app_info_flag, app_dict_flag, data_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (id) DO UPDATE SET data_value = $3",
            &[&app_info_flag, &app_dict_flag, &data_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4, _p5, _p6)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4, _p5, _p6)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_surface_appdict SET deleted_at = NOW() WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4, _p5, _p6)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data_value = body.get("dataValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_surface_appdict (id, app_info_flag, app_dict_flag, data_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (id) DO UPDATE SET data_value = $3",
            &[&app_info_flag, &app_dict_flag, &data_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4, _p5, _p6, _p7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, app_info_flag, app_dict_flag, path_levels, data_value, creator, create_time FROM x_cms_surface_appdict WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3 AND deleted_at IS NULL",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("appInfoFlag".to_string(), Value::String(row.get("app_info_flag"))),
            ("appDictFlag".to_string(), Value::String(row.get("app_dict_flag"))),
            ("dataValue".to_string(), Value::String(row.get::<_, Option<String>>("data_value").unwrap_or_default())),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4, _p5, _p6, _p7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_cms_surface_appdict SET deleted_at = NOW() WHERE app_dict_flag = $1 AND app_info_flag = $2 AND path_levels::text ILIKE $3",
            &[&app_dict_flag, &app_info_flag, &format!("%{}%", path0)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((app_dict_flag, app_info_flag, path0, _p1, _p2, _p3, _p4, _p5, _p6, _p7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String, String)>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data_value = body.get("dataValue").and_then(|v| v.as_str()).unwrap_or("");
    client
        .execute(
            "INSERT INTO x_cms_surface_appdict (id, app_info_flag, app_dict_flag, data_value) VALUES (gen_random_uuid()::text, $1, $2, $3) ON CONFLICT (id) DO UPDATE SET data_value = $3",
            &[&app_info_flag, &app_dict_flag, &data_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// ─── templateform_* stubs ───────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn templateform_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_form_v2", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn templateform_list_category(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_form_v2", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn templateform_list_category_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_form_v2", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn templateform_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, app_id, name, definition, status, creator, create_time FROM x_cms_form_v2 WHERE id = $1 AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_view", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn view_list_app_appId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_view", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn view_list_category_categoryId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_view", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn view_list_form_formId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_view", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn view_viewdata_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_viewrecord", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn view_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_view", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
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
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn viewcategory_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_viewcategory", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn viewcategory_list_category_categoryId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_viewcategory", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn viewcategory_list_view_viewId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_viewcategory", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn viewcategory_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_viewcategory", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
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
    let data = list_from_table_filtered(&pool, "x_cms_viewfieldconfig", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn viewfieldconfig_list_view_viewId(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_viewfieldconfig", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn viewfieldconfig_id(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_viewfieldconfig", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
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
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

#[axum::debug_handler]
pub async fn viewrecord_document_docId_filter_list_id_next_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = list_from_table_filtered(&pool, "x_cms_viewrecord", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn viewrecord_document_docId_has_view(
    pool: Extension<Pool>,
    axum::extract::Path(doc_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT COUNT(*)::bigint AS cnt FROM x_cms_viewrecord WHERE doc_id = $1 AND deleted_at IS NULL",
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
    let data = list_from_table_filtered(&pool, "x_cms_viewrecord", "deleted_at IS NULL", &[]).await?;
    Ok(Json(ActionResult::success(data)))
}

// ─── image / input helpers (STUB: no corresponding DB table, business logic unclear) ──

// STUB: image_encode_base64 - image processing utility, no DB table mapping

#[axum::debug_handler]
pub async fn image_encode_base64(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// STUB: image_encode_base64_size_size - image processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn image_encode_base64_size_size(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// STUB: image_resize_id_id_width_width_height_height - image processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn image_resize_id_id_width_width_height_height(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// STUB: input_compare - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_compare(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// STUB: input_compare_mockputtopost - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_compare_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// STUB: input_cover - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_cover(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// STUB: input_cover_mockputtopost - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_cover_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// STUB: input_create - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_create(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
}

// STUB: input_create_mockputtopost - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_create_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
}

// STUB: input_prepare_cover - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_prepare_cover(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// STUB: input_prepare_cover_mockputtopost - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_prepare_cover_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

// STUB: input_prepare_create - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_prepare_create(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
}

// STUB: input_prepare_create_mockputtopost - input processing utility, no DB table mapping
#[axum::debug_handler]
pub async fn input_prepare_create_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
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
            "SELECT COUNT(*) FROM x_cms_commend WHERE doc_id = $1 AND deleted_at IS NULL",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let count: i64 = count_row.get("count");

    let rows = client
        .query(
            "SELECT id, doc_id, person_id, create_time FROM x_cms_commend WHERE doc_id = $1 AND deleted_at IS NULL ORDER BY create_time DESC LIMIT $2::bigint OFFSET $3::bigint",
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

    let results = search_documents(&pool, &query, limit)
        .await
        .map_err(|_| AppError::Internal)?;

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

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
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
