use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

pub async fn work_terminate(
    pool: Extension<Pool>,
    Path(work_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_work SET work_status = $1 WHERE id = $2", &[&"terminated", &work_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(work_id)),
            ("workStatus".to_string(), Value::String("terminated".to_string())),
            ("result".to_string(), Value::String("ok".to_string())),
        ]),
    ))))
}

pub async fn work_retract(
    pool: Extension<Pool>,
    Path(work_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, work_status, activity FROM x_work WHERE id = $1", &[&work_id])
        .await
        .map_err(|_| AppError::Internal)?;

    if rows.is_empty() {
        return Ok(Json(ActionResult::error("work not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(work_id)),
            ("workStatus".to_string(), Value::String("retracted".to_string())),
            ("previousStatus".to_string(), Value::String(rows[0].get("work_status"))),
        ]),
    ))))
}

pub async fn work_processing(
    pool: Extension<Pool>,
    Path(work_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one("SELECT id, title, work_status, activity FROM x_work WHERE id = $1", &[&work_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("title".to_string(), Value::String(row.get("title"))),
            ("workStatus".to_string(), Value::String(row.get("work_status"))),
            ("activity".to_string(), Value::String(row.get("activity"))),
        ]),
    ))))
}

pub async fn task_processing(
    pool: Extension<Pool>,
    Path(task_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one("SELECT id, title, person, activity FROM x_task WHERE id = $1", &[&task_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("title".to_string(), Value::String(row.get("title"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("activity".to_string(), Value::String(row.get("activity"))),
        ]),
    ))))
}

pub async fn work_count_with_person(
    pool: Extension<Pool>,
    Path(person_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT COUNT(*) as count FROM x_task WHERE person = $1 AND work IS NOT NULL",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("count");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("personId".to_string(), Value::String(person_id)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn task_count_with_person(
    pool: Extension<Pool>,
    Path(person_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT COUNT(*) as count FROM x_task WHERE person = $1",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("count");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("personId".to_string(), Value::String(person_id)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub fn processplatform_core_express_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/processplatform/work/terminate/{id}", get(work_terminate))
        .route("/jaxrs/processplatform/work/retract/{id}", get(work_retract))
        .route("/jaxrs/processplatform/work/processing/{id}", get(work_processing))
        .route("/jaxrs/processplatform/task/processing/{id}", get(task_processing))
        .route("/jaxrs/processplatform/work/count/with/person/{id}", get(work_count_with_person))
        .route("/jaxrs/processplatform/task/count/with/person/{id}", get(task_count_with_person))
        .layer(Extension(pool))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::processplatform_core_express_router(pool)
}
