use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ApplicationDict {
    pub id: String,
    pub name: String,
    pub application: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ApplicationDictItem {
    pub id: String,
    pub dict_id: String,
    pub name: String,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GeneralFile {
    pub id: String,
    pub name: String,
    pub mime_type: String,
    pub size: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Invoice {
    pub id: String,
    pub number: String,
    pub date: String,
    pub amount: f64,
    pub status: String,
}

pub async fn dict_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, application FROM x_general_application_dict ORDER BY name LIMIT 20",
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
                ("application".to_string(), Value::String(row.get("application"))),
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

pub async fn dict_item_list(
    pool: Extension<Pool>,
    axum::extract::Path(dict_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, dict_id, name, value FROM x_general_application_dict_item WHERE dict_id = $1",
            &[&dict_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("dictId".to_string(), Value::String(row.get("dict_id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("value".to_string(), Value::String(row.get("value"))),
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

pub async fn dict_create(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let application = payload.get("application").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    client
        .execute(
            "INSERT INTO x_general_application_dict (id, name, application) VALUES ($1, $2, $3)",
            &[&id, &name, &application],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("application".to_string(), Value::String(application)),
    ])))))
}

pub async fn dict_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, application FROM x_general_application_dict WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("application".to_string(), Value::String(row.get("application"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

pub async fn dict_update(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let application = payload.get("application").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    let result = client
        .execute(
            "UPDATE x_general_application_dict SET name = $1, application = $2 WHERE id = $3",
            &[&name, &application, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("dict not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("saved".to_string(), Value::Bool(true)),
        ("name".to_string(), Value::String(name)),
    ])))))
}

pub async fn dict_delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_general_application_dict WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("dict not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

pub async fn dict_item_create(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let dict_id = payload.get("dictId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let value = payload.get("value").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    client
        .execute(
            "INSERT INTO x_general_application_dict_item (id, dict_id, name, value) VALUES ($1, $2, $3, $4)",
            &[&id, &dict_id, &name, &value],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("dictId".to_string(), Value::String(dict_id)),
        ("name".to_string(), Value::String(name)),
        ("value".to_string(), Value::String(value)),
    ])))))
}

pub async fn dict_item_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, dict_id, name, value FROM x_general_application_dict_item WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("dictId".to_string(), Value::String(row.get("dict_id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("value".to_string(), Value::String(row.get("value"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("dict item not found"))),
    }
}

pub async fn dict_item_update(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let dict_id = payload.get("dictId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let value = payload.get("value").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    let result = client
        .execute(
            "UPDATE x_general_application_dict_item SET dict_id = $1, name = $2, value = $3 WHERE id = $4",
            &[&dict_id, &name, &value, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("dict item not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("saved".to_string(), Value::Bool(true)),
        ("name".to_string(), Value::String(name)),
    ])))))
}

pub async fn dict_item_delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_general_application_dict_item WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("dict item not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

pub async fn file_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, mime_type, size FROM x_general_file ORDER BY create_time DESC LIMIT 20",
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
                ("mimeType".to_string(), Value::String(row.get("mime_type"))),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
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

pub async fn invoice_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, number, date, amount, status FROM x_general_invoice ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("number".to_string(), Value::String(row.get("number"))),
                ("date".to_string(), Value::String(row.get("date"))),
                ("amount".to_string(), Value::Number(serde_json::Number::from_f64(row.get::<_, f64>("amount")).unwrap_or_else(|| serde_json::Number::from(0)))),
                ("status".to_string(), Value::String(row.get("status"))),
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

pub async fn file_create(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let mime_type = payload.get("mimeType").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let size = payload.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    client
        .execute(
            "INSERT INTO x_general_file (id, name, mime_type, size, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &name, &mime_type, &size, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("mimeType".to_string(), Value::String(mime_type)),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
    ])))))
}

pub async fn file_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, mime_type, size, creator, create_time FROM x_general_file WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("mimeType".to_string(), Value::String(row.get("mime_type"))),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_update(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let mime_type = payload.get("mimeType").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    let result = client
        .execute(
            "UPDATE x_general_file SET name = $1, mime_type = $2 WHERE id = $3",
            &[&name, &mime_type, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("file not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("saved".to_string(), Value::Bool(true)),
        ("name".to_string(), Value::String(name)),
    ])))))
}

pub async fn file_delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_general_file WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("file not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

pub async fn file_download(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, mime_type, size, creator, create_time FROM x_general_file WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("mimeType".to_string(), Value::String(row.get("mime_type"))),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn invoice_create(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let number = payload.get("number").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let date = payload.get("date").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let amount = payload.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or("draft").to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    client
        .execute(
            "INSERT INTO x_general_invoice (id, number, date, amount, status, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id, &number, &date, &amount, &status, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("number".to_string(), Value::String(number)),
        ("date".to_string(), Value::String(date)),
        ("amount".to_string(), Value::Number(serde_json::Number::from_f64(amount).unwrap_or_else(|| serde_json::Number::from(0)))),
        ("status".to_string(), Value::String(status)),
    ])))))
}

pub async fn invoice_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, number, date, amount, status, creator, create_time FROM x_general_invoice WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("number".to_string(), Value::String(row.get("number"))),
                ("date".to_string(), Value::String(row.get("date"))),
                ("amount".to_string(), Value::Number(serde_json::Number::from_f64(row.get::<_, f64>("amount")).unwrap_or_else(|| serde_json::Number::from(0)))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("invoice not found"))),
    }
}

pub async fn invoice_update(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let number = payload.get("number").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let date = payload.get("date").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let amount = payload.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    let result = client
        .execute(
            "UPDATE x_general_invoice SET number = $1, date = $2, amount = $3, status = $4 WHERE id = $5",
            &[&number, &date, &amount, &status, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("invoice not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("saved".to_string(), Value::Bool(true)),
        ("number".to_string(), Value::String(number)),
    ])))))
}

pub async fn invoice_delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_general_invoice WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("invoice not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

pub fn general_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/general/dict/list", get(dict_list))
        .route("/jaxrs/general/dict/create", post(dict_create))
        .route("/jaxrs/general/dict/{id}", get(dict_get))
        .route("/jaxrs/general/dict/update/{id}", post(dict_update))
        .route("/jaxrs/general/dict/delete/{id}", post(dict_delete))
        .route("/jaxrs/general/dict/item/list/{dictId}", get(dict_item_list))
        .route("/jaxrs/general/dict/item/create", post(dict_item_create))
        .route("/jaxrs/general/dict/item/{id}", get(dict_item_get))
        .route("/jaxrs/general/dict/item/update/{id}", post(dict_item_update))
        .route("/jaxrs/general/dict/item/delete/{id}", post(dict_item_delete))
        .route("/jaxrs/general/file/list", get(file_list))
        .route("/jaxrs/general/file/create", post(file_create))
        .route("/jaxrs/general/file/{id}", get(file_get))
        .route("/jaxrs/general/file/update/{id}", post(file_update))
        .route("/jaxrs/general/file/delete/{id}", post(file_delete))
        .route("/jaxrs/general/file/download/{id}", get(file_download))
        .route("/jaxrs/general/invoice/list", get(invoice_list))
        .route("/jaxrs/general/invoice/create", post(invoice_create))
        .route("/jaxrs/general/invoice/{id}", get(invoice_get))
        .route("/jaxrs/general/invoice/update/{id}", post(invoice_update))
        .route("/jaxrs/general/invoice/delete/{id}", post(invoice_delete))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/general_core_entity/health", axum::routing::get(|| async { "TODO: general_core_entity - real implementation needed" }))
}