use axum::{
    Json, Router,
    extract::Extension,
    routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use uuid::Uuid;

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreateProcessRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
}

pub async fn get_process(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;    let row = client
        .query_one(
            "SELECT id, title, process, application, work_status, creator, create_time, start_time, end_time FROM x_work WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let tasks = client
        .query(
            "SELECT id, title, activity, activity_token, person, start_time, end_time FROM x_task WHERE work = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let task_list: Vec<Value> = tasks.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("title".to_string(), Value::String(row.get("title"))),
            ("activity".to_string(), Value::String(row.get("activity"))),
            ("activityToken".to_string(), Value::String(row.get("activity_token"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("startTime".to_string(), Value::String(row.get("start_time"))),
            ("endTime".to_string(), Value::String(row.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("title".to_string(), Value::String(row.get("title"))),
            ("process".to_string(), Value::String(row.get("process"))),
            ("application".to_string(), Value::String(row.get("application"))),
            ("workStatus".to_string(), Value::String(row.get("work_status"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
            ("startTime".to_string(), Value::String(row.get("start_time"))),
            ("endTime".to_string(), Value::String(row.get("end_time"))),
            ("tasks".to_string(), Value::Array(task_list)),
        ]),
    ))))
}

pub async fn create_process(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreateProcessRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;    let id = Uuid::new_v4().to_string();
    let title = req.name.unwrap_or_default();
    let application = req.category.clone().unwrap_or_default();
    let process = req.category.unwrap_or_else(|| "default".to_string());

    client
        .execute(
            "INSERT INTO x_work (id, title, process, application, work_status, creator, create_time, start_time) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())",
            &[&id, &title, &process, &application, &"pending", &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

pub async fn list_processes(
    pool: Extension<Pool>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, process, application, work_status, creator, create_time FROM x_work WHERE application = $1 ORDER BY create_time DESC",
            &[&category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("title"))),
                ("category".to_string(), Value::String(row.get("application"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn execute_process(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;    client
        .execute("UPDATE x_work SET work_status = $1 WHERE id = $2", &[&"processing", &id])
        .await
        .map_err(|_| AppError::Internal)?;

    let task_id = Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_task (id, title, work, activity, activity_token, person, start_time) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&task_id, &"Process Task", &id, &"start", &"", &""],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("taskId".to_string(), Value::String(task_id)),
            ("status".to_string(), Value::String("processing".to_string())),
        ]),
    ))))
}

pub async fn get_process_instance(
    pool: Extension<Pool>,
    axum::extract::Path(execution_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;    let row = client
        .query_one(
            "SELECT id, title, process, application, work_status, creator, create_time, start_time, end_time FROM x_work WHERE id = $1",
            &[&execution_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let tasks = client
        .query(
            "SELECT id, title, activity, activity_token, person, start_time, end_time FROM x_task WHERE work = $1",
            &[&execution_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let task_list: Vec<Value> = tasks.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("title".to_string(), Value::String(row.get("title"))),
            ("activity".to_string(), Value::String(row.get("activity"))),
            ("activityToken".to_string(), Value::String(row.get("activity_token"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("startTime".to_string(), Value::String(row.get("start_time"))),
            ("endTime".to_string(), Value::String(row.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("title".to_string(), Value::String(row.get("title"))),
            ("process".to_string(), Value::String(row.get("process"))),
            ("application".to_string(), Value::String(row.get("application"))),
            ("workStatus".to_string(), Value::String(row.get("work_status"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
            ("startTime".to_string(), Value::String(row.get("start_time"))),
            ("endTime".to_string(), Value::String(row.get("end_time"))),
            ("tasks".to_string(), Value::Array(task_list)),
        ]),
    ))))
}

pub async fn cancel_process_instance(
    pool: Extension<Pool>,
    axum::extract::Path(execution_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;    client
        .execute("UPDATE x_work SET work_status = $1, end_time = NOW() WHERE id = $2", &[&"cancelled", &execution_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("executionId".to_string(), Value::String(execution_id)),
            ("cancelled".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub fn processplatform_service_processing_router() -> Router {
    Router::new()
        .route("/jaxrs/processplatform/service/processing/get/{id}", get(get_process))
        .route("/jaxrs/processplatform/service/processing/create", post(create_process))
        .route("/jaxrs/processplatform/service/processing/list/{category}", get(list_processes))
        .route("/jaxrs/processplatform/service/processing/execute/{id}", post(execute_process))
        .route("/jaxrs/processplatform/service/processing/instance/{executionId}", get(get_process_instance))
        .route("/jaxrs/processplatform/service/processing/cancel/{executionId}", post(cancel_process_instance))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    processplatform_service_processing_router().layer(axum::extract::Extension(pool))
}



pub async fn applicationdict_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn applicationdict_id_path0_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn applicationdict_id_path0_path1_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn applicationdict_id_path0_path1_path2_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn applicationdict_id_path0_path1_path2_path3_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn applicationdict_id_path0_path1_path2_path3_path4_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn applicationdict_id_path0_path1_path2_path3_path4_path5_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn applicationdict_id_path0_path1_path2_path3_path4_path5_path6_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn applicationdict_id_path0_path1_path2_path3_path4_path5_path6_path7_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_copy_work_workId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_copy_workcompleted_workCompletedId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_edit_id_text() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_id_work_workId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_id_workcompleted_workCompletedId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_job_job() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_job_job_path() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_work_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_work_id_delete() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_work_id_path() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_work_id_path_delete() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_workcompleted_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_workcompleted_id_path() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn documentversion_work_work() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn event_add_update_table() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn form_suitable_activity_activityId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn job_v2_job_person_person_view() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn job_v2_job_projection() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn job_job() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn readcompleted_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn record_job_job() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn record_task_processing() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn record_work_processing() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn record_work_terminate() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn record_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn review_create_work() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn review_create_workcompleted() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn review_init_review() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn review_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn service_work_id_touch() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn snap_upload() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn snap_work_workId_type_abandoned() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn snap_work_workId_type_snap() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn snap_work_workId_type_suspend() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn snap_workcompleted_workCompletedId_type_abandonedworkcompleted() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn snap_workcompleted_workCompletedId_type_snapworkcompleted() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn snap_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn snap_id_restore() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_v2_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_v2_id_pause() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_v2_id_reset() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_v2_id_resume() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_v3_id_add() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_id_expire() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_id_pass_expired() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_id_press() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_id_processing() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_id_replace() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_id_urge() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_id_will() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn taskcompleted_next_task_identity() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn taskcompleted_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn taskcompleted_id_press_work_work() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn touch_cleanevent() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn touch_deletedraft() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn touch_handoverjob() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn touch_loglongdetained() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn touch_merge() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn touch_mergeitem() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn touch_touchdelay() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn touch_urge() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_manual_after_processing() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_process_processId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_process_processId_name_name_serial() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_v2_id_add_split() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_v2_id_goback() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_v2_id_reroute() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_v2_id_retract() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_v2_id_rollback() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_v2_id_terminate() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_v3_retract() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_id_draft() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_id_manual_append_identity() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_id_processing() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_id_projection() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_id_series_series_activitytoken_activityToken_processing_signal() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn workcompleted_process_processFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn workcompleted_shift_time() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn workcompleted_flag_merge() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn workcompleted_flag_rollback() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

