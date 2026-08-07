use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

pub async fn work_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, job, application, process, work_status FROM x_work ORDER BY start_time DESC LIMIT 20",
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
                ("job".to_string(), Value::String(row.get("job"))),
                ("application".to_string(), Value::String(row.get("application"))),
                ("process".to_string(), Value::String(row.get("process"))),
                ("workStatus".to_string(), Value::String(row.get("work_status"))),
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

pub async fn task_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, job, work, person, activity, activity_token FROM x_task ORDER BY start_time DESC LIMIT 20",
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
                ("job".to_string(), Value::String(row.get("job"))),
                ("work".to_string(), Value::String(row.get("work"))),
                ("person".to_string(), Value::String(row.get("person"))),
                ("activity".to_string(), Value::String(row.get("activity"))),
                ("activityToken".to_string(), Value::String(row.get("activity_token"))),
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

pub async fn work_completed_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("wc-001".to_string())),
            ("title".to_string(), Value::String("已完成工作1".to_string())),
            ("workStatus".to_string(), Value::String("completed".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("wc-002".to_string())),
            ("title".to_string(), Value::String("已完成工作2".to_string())),
            ("workStatus".to_string(), Value::String("completed".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn ticket_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, ticket_type, job, person, work FROM x_ticket ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("ticketType".to_string(), Value::String(row.get("ticket_type"))),
                ("job".to_string(), Value::String(row.get("job"))),
                ("person".to_string(), Value::String(row.get("person"))),
                ("work".to_string(), Value::String(row.get("work"))),
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
        .route("/jaxrs/processplatform/work/list", get(work_list))
        .route("/jaxrs/processplatform/task/list", get(task_list))
        .route("/jaxrs/processplatform/workcompleted/list", get(work_completed_list))
        .route("/jaxrs/processplatform/ticket/list", get(ticket_list))
        .layer(Extension(pool))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::processplatform_core_entity_router(pool)
}
