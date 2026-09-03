use axum::{
    extract::Extension, Json, Router,
    routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub const JAVA_BASE: &str = "/jaxrs/component_assemble_control";
pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


#[axum::debug_handler]
pub async fn get_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT enabled, max_component_count, allow_custom_components FROM x_component_assemble_control_config WHERE id = 'default'",
            &[],
        )
        .await
        .ok();

    let (enabled, max_component_count, allow_custom_components) = match row {
        Some(r) => (
            r.get("enabled"),
            r.get::<_, i64>("max_component_count"),
            r.get("allow_custom_components"),
        ),
        None => (true, 500i64, true),
    };

    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(enabled)),
        ("maxComponentCount".to_string(), Value::Number(serde_json::Number::from(max_component_count))),
        ("allowCustomComponents".to_string(), Value::Bool(allow_custom_components)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_control_categories(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT DISTINCT type FROM CPT_COMPONENT WHERE deleted_at IS NULL ORDER BY type",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut categories = Vec::new();
    for row in rows.iter() {
        let comp_type: String = row.get("type");
        let cnt_row = client
            .query_one(
                "SELECT COUNT(*) as cnt FROM CPT_COMPONENT WHERE type = $1 AND deleted_at IS NULL",
                &[&comp_type],
            )
            .await
            .ok();
        let enabled = cnt_row
            .map(|r| r.get::<_, i64>("cnt") > 0)
            .unwrap_or(false);
        categories.push(Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(comp_type.clone())),
            ("name".to_string(), Value::String(if comp_type == "system" { "System Components".to_string() } else { "Custom Components".to_string() })),
            ("enabled".to_string(), Value::Bool(enabled)),
        ])));
    }

    let total_categories = categories.len();
    Ok(Json(ActionResult::java_success(Value::Array(categories), total_categories as i64, 0)))
}

#[axum::debug_handler]
pub async fn update_control_config(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = body.0;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let enabled = config
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let max_component_count = config
        .get("maxComponentCount")
        .and_then(|v| v.as_i64())
        .unwrap_or(500);
    let allow_custom_components = config
        .get("allowCustomComponents")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let config_value = serde_json::json!({
        "enabled": enabled,
        "maxComponentCount": max_component_count,
        "allowCustomComponents": allow_custom_components,
    });

    let result = client
        .execute(
            "INSERT INTO x_component_assemble_control_config (id, enabled, max_component_count, allow_custom_components) \
             VALUES ($1, $2, $3, $4) \
             ON CONFLICT (id) DO UPDATE SET enabled = $2, max_component_count = $3, allow_custom_components = $4",
            &[&"default", &enabled, &max_component_count, &allow_custom_components],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let updated = result > 0;

    tracing::info!("Updated component assemble control config: {:?}", config_value);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(updated)),
            ("config".to_string(), config_value),
        ]),
    ))))
}

pub fn component_assemble_control_router(pool: Pool) -> Router {
    routes::router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::component_assemble_control_router(pool)
}


#[derive(Debug, serde::Deserialize)]
pub struct ComponentRequest {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub component_type: Option<String>,
}

#[axum::debug_handler]
pub async fn list_components(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, type, creator, create_time FROM x_component WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn get_component(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, type, creator, create_time FROM x_component WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("component not found"))),
    }
}

#[axum::debug_handler]
pub async fn create_component(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<ComponentRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let component_type = req.component_type.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_component (id, name, type, creator, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &name, &component_type, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("type".to_string(), Value::String(component_type)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[axum::debug_handler]
pub async fn save_component(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<ComponentRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.name.unwrap_or_default();
    let component_type = req.component_type.unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_component SET name = $1, type = $2 WHERE id = $3 AND deleted_at IS NULL",
            &[&name, &component_type, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("component not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(result > 0)),
            ("name".to_string(), Value::String(name)),
            ("type".to_string(), Value::String(component_type)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn delete_component(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_component SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("component not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn component_delete_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_component SET deleted_at = NOW() WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(result > 0)),
            ("count".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn status_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT COUNT(*) as total, COUNT(CASE WHEN deleted_at IS NULL THEN 1 END) as active FROM x_component",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let total: i64 = row.get("total");
    let active: i64 = row.get("active");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("total".to_string(), Value::Number(serde_json::Number::from(total))),
            ("active".to_string(), Value::Number(serde_json::Number::from(active))),
            ("deleted".to_string(), Value::Number(serde_json::Number::from(total - active))),
        ]),
    ))))
}
