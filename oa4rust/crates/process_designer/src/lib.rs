use axum::{
    extract::{Extension, Json, Path},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

pub use routes::process_designer_router;

#[derive(Debug, Deserialize)]
pub struct ApplicationCreateRequest {
    pub name: String,
    pub description: Option<String>,
    pub form_definition: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationUpdateRequest {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub form_definition: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationRemoveRequest {
    pub id: String,
}

pub async fn application_list_summary(
    pool: Extension<Pool>,
) -> Json<ActionResult<Value>> {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => {
            return Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("count".to_string(), Value::Number(serde_json::Number::from(0))),
                    ("data".to_string(), Value::Array(vec![])),
                ]),
            )));
        }
    };

    let rows = match client
        .query(
            "SELECT id, name, application_category FROM application ORDER BY name",
            &[],
        )
        .await
        {
            Ok(rows) => rows,
            Err(_) => {
                return Json(ActionResult::success(Value::Object(
                    serde_json::Map::from_iter([
                        ("count".to_string(), Value::Number(serde_json::Number::from(0))),
                        ("data".to_string(), Value::Array(vec![])),
                    ]),
                )));
            }
        };

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                (
                    "applicationCategory".to_string(),
                    Value::String(row.get("application_category")),
                ),
            ]))
        })
        .collect();

    Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    )))
}

pub async fn application_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, description, form_definition, status, create_time FROM PROCESS_APPLICATION WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("formDefinition".to_string(), {
                    let fd: Option<String> = row.get("form_definition");
                    fd.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(Value::Null)
                }),
                ("status".to_string(), Value::String(row.get("status"))),
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

pub async fn application_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, name, description, form_definition, status, create_time FROM PROCESS_APPLICATION WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        (
            "description".to_string(),
            row.get::<_, Option<String>>("description")
                .map(Value::String)
                .unwrap_or(Value::Null),
        ),
        ("formDefinition".to_string(), {
            let fd: Option<String> = row.get("form_definition");
            fd.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(Value::Null)
        }),
        ("status".to_string(), Value::String(row.get("status"))),
        ("createTime".to_string(), Value::String(row.get("create_time"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn application_create(
    pool: Extension<Pool>,
    Json(req): Json<ApplicationCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let description = req.description.unwrap_or_default();
    let form_definition = req.form_definition.unwrap_or_default();
    let status = req.status.unwrap_or_default();

    client
        .execute(
            "INSERT INTO PROCESS_APPLICATION (id, name, description, form_definition, status, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &req.name, &description, &form_definition, &status],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(req.name)),
        (
            "description".to_string(),
            Value::String(description),
        ),
        ("formDefinition".to_string(), Value::String(form_definition)),
        ("status".to_string(), Value::String(status)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn application_update(
    pool: Extension<Pool>,
    Json(req): Json<ApplicationUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE PROCESS_APPLICATION SET name = COALESCE($1, name), description = COALESCE($2, description), form_definition = COALESCE($3, form_definition), status = COALESCE($4, status) WHERE id = $5 AND deleted_at IS NULL",
            &[&req.name, &req.description, &req.form_definition, &req.status, &req.id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("application not found")));
    }

    let row = client
        .query_one(
            "SELECT id, name, description, form_definition, status, create_time FROM PROCESS_APPLICATION WHERE id = $1 AND deleted_at IS NULL",
            &[&req.id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        (
            "description".to_string(),
            row.get::<_, Option<String>>("description")
                .map(Value::String)
                .unwrap_or(Value::Null),
        ),
        ("formDefinition".to_string(), {
            let fd: Option<String> = row.get("form_definition");
            fd.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(Value::Null)
        }),
        ("status".to_string(), Value::String(row.get("status"))),
        ("createTime".to_string(), Value::String(row.get("create_time"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn application_remove(
    pool: Extension<Pool>,
    Json(req): Json<ApplicationRemoveRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PROCESS_APPLICATION SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&req.id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("application not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn designer_get_route(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Json<ActionResult<Value>> {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => {
            return Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("data".to_string(), Value::Null)]),
            )));
        }
    };

    let row = match client
        .query_one("SELECT id, name, process_id, type, description FROM route WHERE id = $1", &[&id])
        .await
        {
            Ok(row) => row,
            Err(_) => {
                return Json(ActionResult::success(Value::Object(
                    serde_json::Map::from_iter([("data".to_string(), Value::Null)]),
                )));
            }
        };

    let route_data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("processId".to_string(), Value::String(row.get("process_id"))),
        ("type".to_string(), Value::String(row.get::<_, String>("type"))),
        (
            "description".to_string(),
            row.get::<_, Option<String>>("description")
                .map(Value::String)
                .unwrap_or(Value::Null),
        ),
    ]));

    Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("data".to_string(), route_data)]),
    )))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::process_designer_router(pool)
}
