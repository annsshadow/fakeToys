use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

pub async fn work_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, creator_id, status, form_data, create_time, update_time FROM PROCESS_WORK WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("creatorId".to_string(), Value::String(row.get("creator_id"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("formData".to_string(), {
                    let fd: Option<String> = row.get("form_data");
                    fd.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(Value::Null)
                }),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
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

pub async fn work_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, creator_id, status, form_data, create_time, update_time FROM PROCESS_WORK WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("title".to_string(), Value::String(row.get("title"))),
        ("creatorId".to_string(), Value::String(row.get("creator_id"))),
        ("status".to_string(), Value::String(row.get("status"))),
        ("formData".to_string(), {
            let fd: Option<String> = row.get("form_data");
            fd.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(Value::Null)
        }),
        ("createTime".to_string(), Value::String(row.get("create_time"))),
        ("updateTime".to_string(), Value::String(row.get("update_time"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn task_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, work_id, title, assignee_id, status, create_time FROM PROCESS_TASK WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("workId".to_string(), Value::String(row.get("work_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("assigneeId".to_string(), Value::String(row.get("assignee_id"))),
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

pub async fn task_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, title, assignee_id, status, create_time FROM PROCESS_TASK WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("workId".to_string(), Value::String(row.get("work_id"))),
        ("title".to_string(), Value::String(row.get("title"))),
        ("assigneeId".to_string(), Value::String(row.get("assignee_id"))),
        ("status".to_string(), Value::String(row.get("status"))),
        ("createTime".to_string(), Value::String(row.get("create_time"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn ticket_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, work_id, title, description, status, create_time FROM PROCESS_TICKET WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("workId".to_string(), Value::String(row.get("work_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("description".to_string(), {
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null)
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

pub async fn workcompleted_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, work_id, result, complete_time FROM PROCESS_WORK_COMPLETED WHERE deleted_at IS NULL ORDER BY complete_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("workId".to_string(), Value::String(row.get("work_id"))),
                ("result".to_string(), Value::String(row.get("result"))),
                ("completeTime".to_string(), Value::String(row.get("complete_time"))),
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

pub fn processplatform_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/process/work/list", get(work_list))
        .route("/jaxrs/process/work/{id}", get(work_get))
        .route("/jaxrs/process/task/list", get(task_list))
        .route("/jaxrs/process/task/{id}", get(task_get))
        .route("/jaxrs/process/ticket/list", get(ticket_list))
        .route("/jaxrs/process/workcompleted/list", get(workcompleted_list))
        .layer(Extension(pool))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::processplatform_core_entity_router(pool)
}
