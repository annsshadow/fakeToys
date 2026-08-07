use axum::{
    extract::Extension,
    Json, Router, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

pub async fn get_portal(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String("Portal".to_string())),
        ("description".to_string(), Value::String("Portal description".to_string())),
    ])))))
}

pub async fn list_portal() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("portal-1".to_string())),
            ("name".to_string(), Value::String("Portal 1".to_string())),
        ]))
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn list_portal_category() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("cat-1".to_string())),
            ("name".to_string(), Value::String("Category 1".to_string())),
            ("count".to_string(), Value::Number(serde_json::Number::from(0_i64))),
        ]))
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn page_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, category, content, creator, create_time, update_time FROM x_portal_page ORDER BY update_time DESC LIMIT 50",
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
                ("category".to_string(), Value::String(row.get("category"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn get_page(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, category, content, creator, create_time, update_time FROM x_portal_page WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn create_page(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = payload.get("name").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("name is required".to_string()))?;
    let category = payload.get("category").and_then(|v| v.as_str()).unwrap_or_default();
    let content = payload.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system");

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_portal_page (id, name, category, content, creator, create_time, update_time) VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &category, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name.to_string())),
        ("category".to_string(), Value::String(category.to_string())),
    ])))))
}

pub async fn save_page(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let content = payload.get("content").and_then(|v| v.as_str()).unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_portal_page SET content = $1, update_time = NOW() WHERE id = $2",
            &[&content, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("page not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn delete_page(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM x_portal_page WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("page not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn widget_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, alias, category, portal FROM x_widget ORDER BY name",
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
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("portal".to_string(), Value::String(row.get("portal"))),
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

pub async fn script_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, alias, validated FROM x_script ORDER BY name",
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
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("validated".to_string(), Value::Bool(row.get("validated"))),
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

pub async fn dict_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, app_name, app_data, creator, create_time FROM x_portal_dict WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("appData".to_string(), Value::String(row.get("app_data"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
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

pub fn portal_router() -> Router {
    Router::new()
        .route("/jaxrs/portal/{id}", get(get_portal))
        .route("/jaxrs/portal/list", get(list_portal))
        .route("/jaxrs/portalcategory/list", get(list_portal_category))
        .route("/jaxrs/portal/page/{id}", get(get_page))
        .route("/jaxrs/portal/page/create", post(create_page))
        .route("/jaxrs/portal/page/save/{id}", post(save_page))
        .route("/jaxrs/portal/page/delete/{id}", post(delete_page))
        .route("/jaxrs/portal/dict/list", get(dict_list))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    portal_router().layer(axum::extract::Extension(pool))
}
