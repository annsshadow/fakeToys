use axum::{
    extract::{Extension, Json, Path},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::{option_to_json, row_opt_json, ActionResult}};

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
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            if let Some(val) = row_opt_json::<String>(row, "description") {
                map.insert("description".to_string(), val);
            }
            if let Some(val) = option_to_json::<Value>(row.get::<_, Option<String>>("form_definition").and_then(|s| serde_json::from_str(&s).ok())) {
                map.insert("formDefinition".to_string(), val);
            }
            map.insert("status".to_string(), Value::String(row.get("status")));
            map.insert("createTime".to_string(), Value::String(row.get("create_time")));
            Value::Object(map)
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

    let mut result_map = serde_json::Map::new();
    result_map.insert("id".to_string(), Value::String(row.get("id")));
    result_map.insert("name".to_string(), Value::String(row.get("name")));
    if let Some(val) = row_opt_json::<String>(&row, "description") {
        result_map.insert("description".to_string(), val);
    }
    if let Some(val) = option_to_json::<Value>(row.get::<_, Option<String>>("form_definition").and_then(|s| serde_json::from_str(&s).ok())) {
        result_map.insert("formDefinition".to_string(), val);
    }
    result_map.insert("status".to_string(), Value::String(row.get("status")));
    result_map.insert("createTime".to_string(), Value::String(row.get("create_time")));
    let result = Value::Object(result_map);

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

    let mut result_map = serde_json::Map::new();
    result_map.insert("id".to_string(), Value::String(row.get("id")));
    result_map.insert("name".to_string(), Value::String(row.get("name")));
    if let Some(val) = row_opt_json::<String>(&row, "description") {
        result_map.insert("description".to_string(), val);
    }
    if let Some(val) = option_to_json::<Value>(row.get::<_, Option<String>>("form_definition").and_then(|s| serde_json::from_str(&s).ok())) {
        result_map.insert("formDefinition".to_string(), val);
    }
    result_map.insert("status".to_string(), Value::String(row.get("status")));
    result_map.insert("createTime".to_string(), Value::String(row.get("create_time")));
    let result = Value::Object(result_map);

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

    let mut route_map = serde_json::Map::new();
    route_map.insert("id".to_string(), Value::String(row.get("id")));
    route_map.insert("name".to_string(), Value::String(row.get("name")));
    route_map.insert("processId".to_string(), Value::String(row.get("process_id")));
    route_map.insert("type".to_string(), Value::String(row.get::<_, String>("type")));
    if let Some(val) = row_opt_json::<String>(&row, "description") {
        route_map.insert("description".to_string(), val);
    }
    let route_data = Value::Object(route_map);

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
