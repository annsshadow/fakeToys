use axum::{
    Json,
    extract::Extension,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult, response::row_to_json};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::NaiveDateTime;

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
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;    let row = client
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
            ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
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
            ("\"startTime\"".to_string(), Value::String(row.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(row.get("end_time"))),
            ("tasks".to_string(), Value::Array(task_list)),
        ]),
    ))))
}

pub async fn create_process(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreateProcessRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;    let id = Uuid::new_v4().to_string();
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
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;

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
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    tx.execute("UPDATE x_work SET work_status = $1 WHERE id = $2", &[&"processing", &id])
        .await
        .map_err(|_| AppError::Internal)?;

    let task_id = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_task (id, title, work, activity, activity_token, person, start_time) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&task_id, &"Process Task", &id, &"start", &"", &""],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;

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
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;    let row = client
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
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;    client
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

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::router(pool)
}

// ──────────────────────────────────────────────────────────────────────────────
// Work operations
// ──────────────────────────────────────────────────────────────────────────────

pub async fn work_id_processing(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, process, application, work_status, creator, create_time FROM x_work WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_work SET work_status = $1 WHERE id = $2", &[&"processing", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn work_v2_id_terminate(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    tx.execute("UPDATE x_work SET work_status = $1, end_time = NOW() WHERE id = $2", &[&"terminated", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    let id2 = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_record (id, work_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id2, &id, &"terminate", &"workflow terminated", &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("terminated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_v2_id_retract(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_work SET work_status = $1 WHERE id = $2", &[&"retracted", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_task SET task_status = $1 WHERE work = $2", &[&"cancelled", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("retracted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_v2_id_goback(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    tx.execute("UPDATE x_work SET work_status = $1 WHERE id = $2", &[&"pending", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    tx.execute("UPDATE x_task SET task_status = $1 WHERE work = $2 AND task_status = $3", &[&"pending", &id, &"active"])
        .await
        .map_err(|_| AppError::Internal)?;
    let id2 = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_record (id, work_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id2, &id, &"goback", &"workflow back to pending", &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("goback".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn work_v2_id_rollback(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let snap_row = client
        .query_opt(
            "SELECT id, work_id, snap_type, snap_data FROM x_snap WHERE work_id = $1 ORDER BY create_time DESC LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if let Some(snap) = snap_row {
        let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
        let snap_data_raw: String = snap.get("snap_data");
        let snap_data: Value = serde_json::from_str(&snap_data_raw).unwrap_or(Value::Null);
        let id2 = Uuid::new_v4().to_string();
        tx.execute(
                "INSERT INTO x_record (id, work_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
                &[&id2, &id, &"rollback", &snap_data.to_string(), &"system"],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        tx.commit().await.map_err(|_| AppError::Internal)?;
        return Ok(Json(ActionResult::success(snap_data)));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("rolled_back".to_string(), Value::Bool(false))]),
    ))))
}

pub async fn work_v2_id_add_split(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, title, process, application, work_status, creator FROM x_work WHERE id = $1 FOR UPDATE",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let title: String = row.get("title");
    let new_id = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_work (id, title, process, application, work_status, creator, create_time, start_time) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())",
            &[&new_id, &title, &row.get::<_, String>("process"), &row.get::<_, Option<String>>("application").unwrap_or_default(), &"pending", &row.get::<_, String>("creator")],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.execute(
            "INSERT INTO x_record (id, work_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&Uuid::new_v4().to_string(), &id, &"split", &format!("split to {}", new_id), &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("splitId".to_string(), Value::String(new_id)),
        ]),
    ))))
}

pub async fn work_v2_id_reroute(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    tx.execute("UPDATE x_work SET work_status = $1 WHERE id = $2", &[&"rerouted", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    tx.execute("UPDATE x_task SET task_status = $1 WHERE work = $2 AND task_status != $3", &[&"cancelled", &id, &"cancelled"])
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("rerouted".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn work_id_draft(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, work_id, content, creator, create_time FROM x_draft WHERE work_id = $1 AND deleted_at IS NULL LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(row_to_json(&r)))),
        None => Err(AppError::NotFound),
    }
}

pub async fn work_id_manual_append_identity(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, process, application, work_status, creator FROM x_work WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn work_id_projection(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, work, activity, activity_token, person, task_status FROM x_task WHERE work = $1 AND task_status != $2",
            &[&id, &"completed"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn work_id_series_series_activitytoken_activityToken_processing_signal(
    pool: Extension<Pool>,
    axum::extract::Path((series, activity_token)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, work, activity, activity_token, person, task_status, start_time FROM x_task WHERE activity_token = $1 AND task_status = $2",
            &[&activity_token, &"processing"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("series".to_string(), Value::String(series)),
            ("tasks".to_string(), Value::Array(list)),
        ]),
    ))))
}

pub async fn work_process_processId(
    pool: Extension<Pool>,
    axum::extract::Path(process_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, name, category, version, status, creator, create_time FROM x_process_definition WHERE id = $1",
            &[&process_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn work_process_processId_name_name_serial(
    pool: Extension<Pool>,
    axum::extract::Path((process_id, _name)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let max_val = client
        .query_opt(
            r"SELECT MAX(CAST(SUBSTRING(name FROM '\d+') AS INTEGER)) AS max_n FROM x_process_definition WHERE id = $1 AND name SIMILAR TO $2 || '\d+'",
            &[&process_id, &_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let next = max_val.and_then(|r| r.get::<_, Option<i64>>("max_n")).unwrap_or(0) + 1;
    let serial = format!("{}_{}", _name, next);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("serial".to_string(), Value::String(serial))]),
    ))))
}

pub async fn work_manual_after_processing(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, title, process, application, work_status, creator FROM x_work WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.execute(
            "INSERT INTO x_record (id, work_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&Uuid::new_v4().to_string(), &id, &"manual", &"manual processing completed", &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Task operations
// ──────────────────────────────────────────────────────────────────────────────

pub async fn task_id_processing(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, work, activity, activity_token, person, task_status, start_time FROM x_task WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_task SET task_status = $1 WHERE id = $2", &[&"processing", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn task_id_urge(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    tx.execute("UPDATE x_task SET task_status = $1 WHERE id = $2", &[&"urged", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, title, work, activity, activity_token, person FROM x_task WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let id2 = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_record (id, task_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id2, &id, &"urge", &"task urged", &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn task_id_replace(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, work, activity, activity_token, person, task_status FROM x_task WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let work: String = row.get("work");
    let person: String = row.get("person");
    tx.execute("UPDATE x_task SET task_status = $1 WHERE id = $2", &[&"replaced", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    let new_id = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&new_id, &"Replaced Task", &work, &row.get::<_, String>("activity"), &row.get::<_, String>("activity_token"), &person, &"pending"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("newId".to_string(), Value::String(new_id)),
        ]),
    ))))
}

pub async fn task_id_press(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_task SET task_status = $1 WHERE id = $2", &[&"pressed", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("pressed".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn task_id_expire(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_task SET task_status = $1 WHERE id = $2", &[&"expired", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("expired".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn task_id_pass_expired(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_task SET task_status = $1 WHERE id = $2", &[&"pending", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("passed".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn task_id_will(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, work, activity, activity_token, person, task_status FROM x_task WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn task_v2_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, work, activity, activity_token, person, task_status, start_time, end_time FROM x_task WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn task_v2_id_pause(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_task SET task_status = $1 WHERE id = $2", &[&"paused", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("paused".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn task_v2_id_reset(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_task SET task_status = $1, start_time = NULL, end_time = NULL WHERE id = $2", &[&"pending", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("reset".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn task_v2_id_resume(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_task SET task_status = $1, start_time = NOW() WHERE id = $2", &[&"active", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("resumed".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn task_v3_id_add(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, title, process, application, work_status FROM x_work WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let new_task_id = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&new_task_id, &row.get::<_, String>("title"), &id, &"default", &"", &"", &"pending"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("workId".to_string(), Value::String(id)),
            ("taskId".to_string(), Value::String(new_task_id)),
        ]),
    ))))
}

pub async fn task_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, work, activity, activity_token, person, task_status, start_time, end_time FROM x_task WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn taskcompleted_next_task_identity(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work, activity_token FROM x_task WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let work: String = row.get("work");
    let activity_token: String = row.get("activity_token");
    let next_row = client
        .query_opt(
            "SELECT id, title, activity, activity_token, person, task_status FROM x_task WHERE work = $1 AND activity_token = $2 AND task_status = $3 LIMIT 1",
            &[&work, &activity_token, &"pending"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match next_row {
        Some(r) => Ok(Json(ActionResult::success(row_to_json(&r)))),
        None => Err(AppError::NotFound),
    }
}

pub async fn taskcompleted_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, completed_time, creator, create_time FROM x_workcompleted WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn taskcompleted_id_press_work_work(
    pool: Extension<Pool>,
    axum::extract::Path((completed_id, work_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    tx.execute("UPDATE x_workcompleted SET work_id = $1 WHERE id = $2", &[&work_id, &completed_id])
        .await
        .map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, work_id, completed_time, creator FROM x_workcompleted WHERE id = $1",
            &[&completed_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Helper operations
// ──────────────────────────────────────────────────────────────────────────────

pub async fn snap_upload(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<serde_json::Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = Uuid::new_v4().to_string();
    let work_id: String = req.get("workId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let snap_type: String = req.get("snapType").and_then(|v| v.as_str()).unwrap_or("snap").to_string();
    let snap_data = req.get("data").cloned().unwrap_or(Value::Null);
    client
        .execute(
            "INSERT INTO x_snap (id, work_id, snap_type, snap_data, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &work_id, &snap_type, &snap_data.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

pub async fn snap_work_workId_type_abandoned(
    pool: Extension<Pool>,
    axum::extract::Path((work_id, _type)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_snap (id, work_id, snap_type, snap_data, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &work_id, &"abandoned", &"null"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

pub async fn snap_work_workId_type_snap(
    pool: Extension<Pool>,
    axum::extract::Path((work_id, _type)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, work_id, snap_type, snap_data, create_time FROM x_snap WHERE work_id = $1 AND snap_type = $2 ORDER BY create_time DESC LIMIT 1",
            &[&work_id, &_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(row_to_json(&r)))),
        None => Err(AppError::NotFound),
    }
}

pub async fn snap_work_workId_type_suspend(
    pool: Extension<Pool>,
    axum::extract::Path((work_id, _type)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_snap (id, work_id, snap_type, snap_data, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &work_id, &"suspend", &"null"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

pub async fn snap_workcompleted_workCompletedId_type_abandonedworkcompleted(
    pool: Extension<Pool>,
    axum::extract::Path((work_completed_id, _type)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let id = Uuid::new_v4().to_string();
    let work_id_row = tx
        .query_one(
            "SELECT work_id FROM x_workcompleted WHERE id = $1",
            &[&work_completed_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let work_id: String = work_id_row.get("work_id");
    tx.execute(
            "INSERT INTO x_snap (id, work_id, snap_type, snap_data, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &work_id, &"abandonedworkcompleted", &"null"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

pub async fn snap_workcompleted_workCompletedId_type_snapworkcompleted(
    pool: Extension<Pool>,
    axum::extract::Path((work_completed_id, _type)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, completed_time, creator FROM x_workcompleted WHERE id = $1",
            &[&work_completed_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let work_id: String = row.get("work_id");
    let snap_rows = client
        .query(
            "SELECT id, work_id, snap_type, snap_data, create_time FROM x_snap WHERE work_id = $1 AND snap_type = $2 ORDER BY create_time DESC",
            &[&work_id, &_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let snaps: Vec<Value> = snap_rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("workCompleted".to_string(), row_to_json(&row)),
            ("snaps".to_string(), Value::Array(snaps)),
        ]),
    ))))
}

pub async fn snap_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, snap_type, snap_data, create_time FROM x_snap WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn snap_id_restore(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let snap_row = tx
        .query_one(
            "SELECT id, work_id, snap_type, snap_data FROM x_snap WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let work_id: String = snap_row.get("work_id");
    let snap_data_raw: String = snap_row.get("snap_data");
    let snap_data: Value = serde_json::from_str(&snap_data_raw).unwrap_or(Value::Null);
    tx.execute("UPDATE x_work SET work_status = $1 WHERE id = $2", &[&"restored", &work_id])
        .await
        .map_err(|_| AppError::Internal)?;
    let id2 = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_record (id, work_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id2, &work_id, &"restore", &format!("restored from snap {}", id), &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(snap_data)))
}

pub async fn touch_cleanevent(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("DELETE FROM x_record WHERE record_type = $1 AND create_time < NOW() - INTERVAL '30 days'", &[&"event"])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("cleaned".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn touch_deletedraft(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_draft SET deleted_at = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("deleted".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn touch_handoverjob(
    pool: Extension<Pool>,
    axum::extract::Path((work_id, person)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_job (id, work_id, person, activity_token, job_status, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &work_id, &person, &"handover", &"pending"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("workId".to_string(), Value::String(work_id)), ("person".to_string(), Value::String(person))]),
    ))))
}

pub async fn touch_loglongdetained(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.id, t.work, t.person, t.task_status, t.start_time FROM x_task t JOIN x_work w ON t.work = w.id WHERE t.task_status = $1 AND t.start_time < NOW() - INTERVAL '24 hours' AND t.end_time IS NULL",
            &[&"active"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn touch_merge(
    pool: Extension<Pool>,
    axum::extract::Path((id1, id2)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row1 = tx
        .query_one(
            "SELECT id, title, process, application, work_status FROM x_work WHERE id = $1",
            &[&id1],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let _row2 = tx
        .query_one(
            "SELECT id, title, process, application, work_status FROM x_work WHERE id = $2",
            &[&id2],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.execute("UPDATE x_work SET work_status = $1 WHERE id = $2", &[&"merged", &id2])
        .await
        .map_err(|_| AppError::Internal)?;
    let id3 = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_workcompleted (id, work_id, completed_time, creator, create_time) VALUES ($1, $2, NOW(), $3, NOW())",
            &[&id3, &id1, &row1.get::<_, String>("creator")],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("mergedFrom".to_string(), Value::String(id2)),
            ("result".to_string(), row_to_json(&row1)),
        ]),
    ))))
}

pub async fn touch_mergeitem(
    pool: Extension<Pool>,
    axum::extract::Path((work_id, item_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, work, activity, activity_token, person FROM x_task WHERE id = $1",
            &[&item_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_task SET work = $1 WHERE id = $2", &[&work_id, &item_id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn touch_touchdelay(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, title, work, activity, activity_token, person, task_status FROM x_task WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let work_id: String = row.get("work");
    let id2 = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_record (id, work_id, task_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id2, &work_id, &id, &"delay", &"task delayed", &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn touch_urge(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    tx.execute("UPDATE x_task SET task_status = $1 WHERE id = $2", &[&"urged", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    let id2 = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_record (id, task_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id2, &id, &"urge", &"task urged via touch", &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("urged".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn review_create_work(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<serde_json::Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = Uuid::new_v4().to_string();
    let work_id: String = req.get("workId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let reviewer: String = req.get("reviewer").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let comment: String = req.get("comment").and_then(|v| v.as_str()).unwrap_or("").to_string();
    client
        .execute(
            "INSERT INTO x_review (id, work_id, reviewer, comment, status, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &work_id, &reviewer, &comment, &"pending"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("workId".to_string(), Value::String(work_id))]),
    ))))
}

pub async fn review_create_workcompleted(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<serde_json::Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = Uuid::new_v4().to_string();
    let work_completed_id: String = req.get("workCompletedId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let reviewer: String = req.get("reviewer").and_then(|v| v.as_str()).unwrap_or("").to_string();
    client
        .execute(
            "INSERT INTO x_review (id, work_id, reviewer, comment, status, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &work_completed_id, &reviewer, &"work completed review", &"pending"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("workCompletedId".to_string(), Value::String(work_completed_id))]),
    ))))
}

pub async fn review_init_review(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, work_id, reviewer, comment, status, create_time FROM x_review WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.execute("UPDATE x_review SET status = $1 WHERE id = $2", &[&"initiated", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn review_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, reviewer, comment, status, create_time FROM x_review WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn data_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, person, activity_token, job_status, create_time FROM x_job WHERE id = $1",
            &[&job_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn data_job_job_path(
    pool: Extension<Pool>,
    axum::extract::Path((job_id, _path)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, person, activity_token, job_status FROM x_job WHERE id = $1",
            &[&job_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let work_id: String = row.get("work_id");
    let data_row = client
        .query_opt(
            "SELECT id, title, process, application, work_status FROM x_work WHERE id = $1",
            &[&work_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match data_row {
        Some(r) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("job".to_string(), row_to_json(&row)),
                ("workData".to_string(), row_to_json(&r)),
            ]),
        )))),
        None => Ok(Json(ActionResult::success(row_to_json(&row)))),
    }
}

pub async fn data_work_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, process, application, work_status, creator, create_time, start_time, end_time FROM x_work WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn data_work_id_delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_work SET deleted_at = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("deleted".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn data_work_id_path(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    match _path.as_str() {
        "tasks" => {
            let rows = client
                .query(
                    "SELECT id, title, activity, activity_token, person, task_status, start_time, end_time FROM x_task WHERE work = $1",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
            Ok(Json(ActionResult::success(Value::Array(list))))
        }
        "reviews" => {
            let rows = client
                .query(
                    "SELECT id, work_id, reviewer, comment, status, create_time FROM x_review WHERE work_id = $1",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
            Ok(Json(ActionResult::success(Value::Array(list))))
        }
        "snapshots" => {
            let rows = client
                .query(
                    "SELECT id, work_id, snap_type, snap_data, create_time FROM x_snap WHERE work_id = $1 ORDER BY create_time DESC",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
            Ok(Json(ActionResult::success(Value::Array(list))))
        }
        _ => {
            let row = client
                .query_one(
                    "SELECT id, title, process, application, work_status, creator FROM x_work WHERE id = $1",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(row_to_json(&row))))
        }
    }
}

pub async fn work_list(
    pool: Extension<Pool>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let application = params.get("application").map(|s| s.as_str()).unwrap_or("");
    let page: u64 = params.get("page").and_then(|s| s.parse().ok()).unwrap_or(1);
    let size: u64 = params.get("size").and_then(|s| s.parse().ok()).unwrap_or(20);
    let offset = (page - 1) * size;

    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count_row = client
        .query_one(
            "SELECT COUNT(*) FROM x_work WHERE deleted_at IS NULL AND ($1 = '' OR application = $1)",
            &[&application],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = count_row.get("count");

    let rows = client
        .query(
            "SELECT id, title, process, COALESCE(application, '') as application, work_status, creator, to_char(create_time, 'YYYY-MM-DD HH24:MI:SS') as create_time FROM x_work WHERE deleted_at IS NULL AND ($1 = '' OR application = $1) ORDER BY create_time DESC LIMIT $2::bigint OFFSET $3::bigint",
            &[&application, &(size as i64), &(offset as i64)],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("process".to_string(), Value::String(row.get("process"))),
                ("application".to_string(), Value::String(row.get("application"))),
                ("workStatus".to_string(), Value::String(row.get("work_status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(total))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn process_id_complex(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;

    let work_row = client
        .query_opt(
            "SELECT id, title, process, application, work_status, creator, create_time FROM x_work WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let work = match work_row {
        Some(r) => r,
        None => return Err(AppError::Internal),
    };

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

    let reviews = client
        .query(
            "SELECT id, work_id, reviewer, comment, status, create_time FROM x_review WHERE work_id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let review_list: Vec<Value> = reviews.iter().map(|row| row_to_json(row)).collect();

    let snaps = client
        .query(
            "SELECT id, work_id, snap_type, snap_data, create_time FROM x_snap WHERE work_id = $1 ORDER BY create_time DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let snap_list: Vec<Value> = snaps.iter().map(|row| row_to_json(row)).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("work".to_string(), row_to_json(&work)),
            ("tasks".to_string(), Value::Array(task_list)),
            ("reviews".to_string(), Value::Array(review_list)),
            ("snaps".to_string(), Value::Array(snap_list)),
        ]),
    ))))
}

pub async fn data_work_id_path_delete(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    match _path.as_str() {
        "tasks" => {
            client
                .execute("UPDATE x_task SET task_status = $1 WHERE work = $2", &[&"deleted", &id])
                .await
                .map_err(|_| AppError::Internal)?;
        }
        "reviews" => {
            client
                .execute("UPDATE x_review SET deleted_at = NOW() WHERE work_id = $1", &[&id])
                .await
                .map_err(|_| AppError::Internal)?;
        }
        _ => {}
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id)), ("path".to_string(), Value::String(_path)), ("deleted".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn data_workcompleted_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, completed_time, creator, create_time FROM x_workcompleted WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn data_workcompleted_id_path(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, completed_time, creator FROM x_workcompleted WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let work_id: String = row.get("work_id");
    match _path.as_str() {
        "records" => {
            let rows = client
                .query(
                    "SELECT id, work_id, task_id, record_type, content, creator, create_time FROM x_record WHERE work_id = $1",
                    &[&work_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
            Ok(Json(ActionResult::success(Value::Array(list))))
        }
        "snapshots" => {
            let rows = client
                .query(
                    "SELECT id, work_id, snap_type, snap_data, create_time FROM x_snap WHERE work_id = $1",
                    &[&work_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
            Ok(Json(ActionResult::success(Value::Array(list))))
        }
        _ => Ok(Json(ActionResult::success(row_to_json(&row)))),
    }
}

pub async fn documentversion_work_work(
    pool: Extension<Pool>,
    axum::extract::Path(work_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, work_id, version, content, creator, create_time FROM x_document_version WHERE work_id = $1 ORDER BY version DESC",
            &[&work_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn event_add_update_table(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<serde_json::Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = Uuid::new_v4().to_string();
    let table: String = req.get("table").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let record_id: String = req.get("recordId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let content = req.get("content").cloned().unwrap_or(Value::Null);
    let type_val: String = req.get("type").and_then(|v| v.as_str()).unwrap_or("update").to_string();
    client
        .execute(
            "INSERT INTO x_record (id, work_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &record_id, &type_val, &content.to_string(), &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("table".to_string(), Value::String(table)),
            ("recordId".to_string(), Value::String(record_id)),
        ]),
    ))))
}

pub async fn form_suitable_activity_activityId(
    pool: Extension<Pool>,
    axum::extract::Path(activity_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, activity, activity_token, person, task_status FROM x_task WHERE activity = $1 AND task_status != $2",
            &[&activity_id, &"completed"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn job_v2_job_person_person_view(
    pool: Extension<Pool>,
    axum::extract::Path((job_id, person)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, person, activity_token, job_status, create_time FROM x_job WHERE id = $1 AND person = $2",
            &[&job_id, &person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn job_v2_job_projection(
    pool: Extension<Pool>,
    axum::extract::Path(job_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, person, activity_token, job_status FROM x_job WHERE id = $1",
            &[&job_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let work_id: String = row.get("work_id");
    let tasks = client
        .query(
            "SELECT id, title, activity, activity_token, person, task_status FROM x_task WHERE work = $1 AND task_status = $2",
            &[&work_id, &"active"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let task_list: Vec<Value> = tasks.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("job".to_string(), row_to_json(&row)),
            ("tasks".to_string(), Value::Array(task_list)),
        ]),
    ))))
}

pub async fn job_job(
    pool: Extension<Pool>,
    axum::extract::Path(person): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT j.id, j.work_id, j.person, j.activity_token, j.job_status, j.create_time, w.title FROM x_job j JOIN x_work w ON j.work_id = w.id WHERE j.person = $1 AND j.job_status = $2 ORDER BY j.create_time DESC",
            &[&person, &"pending"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn readcompleted_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, person, completed_time FROM x_readcompleted WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn record_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, work_id, task_id, record_type, content, creator, create_time FROM x_record WHERE record_type = $1",
            &[&job_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn record_task_processing(
    pool: Extension<Pool>,
    axum::extract::Path(task_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, work_id, task_id, record_type, content, creator, create_time FROM x_record WHERE task_id = $1 AND record_type = $2 ORDER BY create_time DESC",
            &[&task_id, &"processing"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn record_work_processing(
    pool: Extension<Pool>,
    axum::extract::Path(work_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, work_id, task_id, record_type, content, creator, create_time FROM x_record WHERE work_id = $1 AND record_type = $2 ORDER BY create_time DESC",
            &[&work_id, &"processing"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn record_work_terminate(
    pool: Extension<Pool>,
    axum::extract::Path(work_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, work_id, record_type, content, creator, create_time FROM x_record WHERE work_id = $1 AND record_type = $2 ORDER BY create_time DESC",
            &[&work_id, &"terminate"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn record_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, task_id, record_type, content, creator, create_time FROM x_record WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn service_work_id_touch(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, process, application, work_status, creator, create_time FROM x_work WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let id2 = Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_record (id, work_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id2, &id, &"touch", &"work touched", &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn attachment_copy_work_workId(
    pool: Extension<Pool>,
    axum::extract::Path((source_id, work_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, name, content, creator FROM x_attachment WHERE id = $1",
            &[&source_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let new_id = Uuid::new_v4().to_string();
    let name: String = row.get("name");
    let content: String = row.get("content");
    let creator: String = row.get("creator");
    client
        .execute(
            "INSERT INTO x_attachment (id, work_id, workcompleted_id, name, content, creator, create_time) VALUES ($1, $2, NULL, $3, $4, $5, NOW())",
            &[&new_id, &work_id, &name, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(new_id)), ("workId".to_string(), Value::String(work_id))]),
    ))))
}

pub async fn attachment_copy_workcompleted_workCompletedId(
    pool: Extension<Pool>,
    axum::extract::Path((source_id, work_completed_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, name, content, creator FROM x_attachment WHERE id = $1",
            &[&source_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let new_id = Uuid::new_v4().to_string();
    let name: String = row.get("name");
    let content: String = row.get("content");
    let creator: String = row.get("creator");
    client
        .execute(
            "INSERT INTO x_attachment (id, work_id, workcompleted_id, name, content, creator, create_time) VALUES ($1, NULL, $2, $3, $4, $5, NOW())",
            &[&new_id, &work_completed_id, &name, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(new_id)), ("workCompletedId".to_string(), Value::String(work_completed_id))]),
    ))))
}

pub async fn attachment_edit_id_text(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, name, content, creator, create_time FROM x_attachment WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn attachment_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, workcompleted_id, name, content, creator, create_time FROM x_attachment WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn attachment_id_work_workId(
    pool: Extension<Pool>,
    axum::extract::Path((_id, work_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, work_id, name, content, creator, create_time FROM x_attachment WHERE work_id = $1",
            &[&work_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn attachment_id_workcompleted_workCompletedId(
    pool: Extension<Pool>,
    axum::extract::Path((_id, work_completed_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, workcompleted_id, name, content, creator, create_time FROM x_attachment WHERE workcompleted_id = $1",
            &[&work_completed_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn applicationdict_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, name, category, version, status, creator, create_time FROM x_process_definition WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn applicationdict_id_path0_data(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, name, category, version, status FROM x_process_definition WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match _path.as_str() {
        "activities" => {
            let rows = client
                .query(
                    "SELECT id, title, activity, activity_token, person, task_status FROM x_task WHERE work = $1",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("activity".to_string(), row_to_json(&row)), ("tasks".to_string(), Value::Array(list))]),
            ))))
        }
        _ => Ok(Json(ActionResult::success(row_to_json(&row)))),
    }
}

pub async fn applicationdict_id_path0_path1_data(
    pool: Extension<Pool>,
    axum::extract::Path((id, _p1, _p2)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, work, activity, activity_token, person, task_status FROM x_task WHERE work = $1 AND activity = $2",
            &[&id, &_p1],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn applicationdict_id_path0_path1_path2_data(
    pool: Extension<Pool>,
    axum::extract::Path((_id, _p1, _p2, _p3)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, activity, activity_token, person, task_status FROM x_task WHERE activity = $1 AND task_status = $2",
            &[&_p1, &_p2],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn applicationdict_id_path0_path1_path2_path3_data(
    pool: Extension<Pool>,
    axum::extract::Path((id, _p1, _p2, _p3, _p4)): axum::extract::Path<(String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, work, activity, activity_token, person, task_status FROM x_task WHERE work = $1 AND activity = $2 AND task_status = $3",
            &[&id, &_p1, &_p2],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn applicationdict_id_path0_path1_path2_path3_path4_data(
    pool: Extension<Pool>,
    axum::extract::Path((id, _p1, _p2, _p3, _p4, _p5)): axum::extract::Path<(String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work, activity, activity_token, person, task_status FROM x_task WHERE work = $1 AND activity = $2 AND activity_token = $3",
            &[&id, &_p1, &_p2],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn applicationdict_id_path0_path1_path2_path3_path4_path5_data(
    pool: Extension<Pool>,
    axum::extract::Path((_id, _p1, _p2, _p3, _p4, _p5, _p6)): axum::extract::Path<(String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, process, application, work_status FROM x_work WHERE application = $1 AND work_status = $2 ORDER BY create_time DESC LIMIT 10",
            &[&_p1, &_p2],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn applicationdict_id_path0_path1_path2_path3_path4_path5_path6_data(
    pool: Extension<Pool>,
    axum::extract::Path((id, _p1, _p2, _p3, _p4, _p5, _p6, _p7)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, process, application, work_status FROM x_work WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn applicationdict_id_path0_path1_path2_path3_path4_path5_path6_path7_data(
    pool: Extension<Pool>,
    axum::extract::Path((_id, _p1, _p2, _p3, _p4, _p5, _p6, _p7, _p8)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, process, application, work_status FROM x_work WHERE work_status = $1 ORDER BY create_time DESC",
            &[&_p1],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn workcompleted_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path((process_id, flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT wc.id, wc.work_id, wc.completed_time, wc.creator, w.title, w.process FROM x_workcompleted wc JOIN x_work w ON wc.work_id = w.id WHERE w.process = $1 AND wc.creator = $2 ORDER BY wc.completed_time DESC",
            &[&process_id, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(list))))
}

pub async fn workcompleted_shift_time(
    pool: Extension<Pool>,
    axum::extract::Path((id, _time)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_workcompleted SET completed_time = $1 WHERE id = $2",
            &[&_time, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, completed_time, creator FROM x_workcompleted WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

pub async fn workcompleted_flag_merge(
    pool: Extension<Pool>,
    axum::extract::Path((id1, id2)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row1 = client
        .query_one(
            "SELECT id, work_id, completed_time, creator FROM x_workcompleted WHERE id = $1",
            &[&id1],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_workcompleted SET work_id = $1 WHERE id = $2", &[&row1.get::<_, String>("work_id"), &id2])
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute("DELETE FROM x_workcompleted WHERE id = $1", &[&id2])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row1))))
}

pub async fn workcompleted_flag_rollback(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, work_id, completed_time, creator FROM x_workcompleted WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let work_id: String = row.get("work_id");
    client
        .execute("UPDATE x_work SET work_status = $1 WHERE id = $2", &[&"pending", &work_id])
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute("DELETE FROM x_workcompleted WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    let snap_row = client
        .query_opt(
            "SELECT id, snap_data FROM x_snap WHERE work_id = $1 ORDER BY create_time DESC LIMIT 1",
            &[&work_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match snap_row {
        Some(s) => {
            let raw: String = s.get("snap_data");
            let v: Value = serde_json::from_str(&raw).unwrap_or(Value::Null);
            Ok(Json(ActionResult::success(v)))
        }
        None => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([("rolled_back".to_string(), Value::Bool(true))]),
        )))),
    }
}

pub async fn work_v3_retract(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, process, application, work_status FROM x_work WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_work SET work_status = $1 WHERE id = $2", &[&"retracted", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_task SET task_status = $1 WHERE work = $2", &[&"cancelled", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(row_to_json(&row))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Workflow execution semantics
// ──────────────────────────────────────────────────────────────────────────────

pub async fn work_start(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, title, process, application, work_status FROM x_work WHERE id = $1 FOR UPDATE",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let status: String = row.get("work_status");
    if status != "pending" {
        tx.commit().await.map_err(|_| AppError::Internal)?;
        return Err(AppError::BadRequest(format!("work status is {}，无法启动", status)));
    }
    tx.execute("UPDATE x_work SET work_status = $1, start_time = NOW() WHERE id = $2", &[&"processing", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    let task_id = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&task_id, &"Start Event", &id, &"start", &"", &"system", &"active"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("taskId".to_string(), Value::String(task_id)),
            ("status".to_string(), Value::String("processing".to_string())),
        ]),
    ))))
}

pub async fn work_complete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, title, process, application, work_status, creator FROM x_work WHERE id = $1 FOR UPDATE",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let status: String = row.get("work_status");
    if status != "processing" {
        tx.commit().await.map_err(|_| AppError::Internal)?;
        return Err(AppError::BadRequest(format!("work status is {}，无法完成归档", status)));
    }
    tx.execute("UPDATE x_work SET work_status = $1, end_time = NOW() WHERE id = $2", &[&"completed", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    tx.execute("UPDATE x_task SET task_status = $1 WHERE work = $2 AND task_status != $3", &[&"completed", &id, &"completed"])
        .await
        .map_err(|_| AppError::Internal)?;
    let completed_id = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_workcompleted (id, work_id, completed_time, creator, create_time) VALUES ($1, $2, NOW(), $3, NOW())",
            &[&completed_id, &id, &row.get::<_, String>("creator")],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("workCompletedId".to_string(), Value::String(completed_id)),
            ("status".to_string(), Value::String("completed".to_string())),
        ]),
    ))))
}

pub async fn task_claim(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, title, work, activity, activity_token, person, task_status FROM x_task WHERE id = $1 FOR UPDATE",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let task_status: String = row.get("task_status");
    if task_status != "pending" {
        tx.commit().await.map_err(|_| AppError::Internal)?;
        return Err(AppError::BadRequest(format!("task status is {}，无法认领", task_status)));
    }
    tx.execute("UPDATE x_task SET task_status = $1, start_time = NOW() WHERE id = $2", &[&"active", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    let id2 = Uuid::new_v4().to_string();
    let work_id: String = row.get("work");
    tx.execute(
            "INSERT INTO x_record (id, work_id, task_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id2, &work_id, &id, &"claim", &"task claimed", &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("claimed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_complete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, title, work, activity, activity_token, person, task_status FROM x_task WHERE id = $1 FOR UPDATE",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let task_status: String = row.get("task_status");
    if task_status != "active" && task_status != "processing" {
        tx.commit().await.map_err(|_| AppError::Internal)?;
        return Err(AppError::BadRequest(format!("task status is {}，无法完成", task_status)));
    }
    tx.execute("UPDATE x_task SET task_status = $1, end_time = NOW() WHERE id = $2", &[&"completed", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    let work_id: String = row.get("work");
    let activity_token: String = row.get("activity_token");
    let id2 = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_record (id, work_id, task_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id2, &work_id, &id, &"complete", &"task completed", &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let next_row = tx
        .query_opt(
            "SELECT id, title, activity, activity_token, person FROM x_task WHERE work = $1 AND activity_token = $2 AND task_status = $3 LIMIT 1",
            &[&work_id, &activity_token, &"pending"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if let Some(next) = next_row {
        let next_id: String = next.get("id");
        tx.execute("UPDATE x_task SET task_status = $1, start_time = NOW() WHERE id = $2", &[&"active", &next_id])
            .await
            .map_err(|_| AppError::Internal)?;
        let id3 = Uuid::new_v4().to_string();
        tx.execute(
                "INSERT INTO x_record (id, work_id, task_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
                &[&id3, &work_id, &next_id, &"auto_claim", &"next task auto claimed", &"system"],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("completed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_reject(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, title, work, activity, activity_token, person, task_status FROM x_task WHERE id = $1 FOR UPDATE",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let task_status: String = row.get("task_status");
    if task_status != "active" && task_status != "processing" {
        tx.commit().await.map_err(|_| AppError::Internal)?;
        return Err(AppError::BadRequest(format!("task status is {}，无法退回", task_status)));
    }
    tx.execute("UPDATE x_task SET task_status = $1 WHERE id = $2", &[&"rejected", &id])
        .await
        .map_err(|_| AppError::Internal)?;
    let work_id: String = row.get("work");
    let activity_token: String = row.get("activity_token");
    let id2 = Uuid::new_v4().to_string();
    let reason = format!("task rejected from {}", activity_token);
    tx.execute(
            "INSERT INTO x_record (id, work_id, task_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id2, &work_id, &id, &"reject", &reason, &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let prev_row = tx
        .query_opt(
            "SELECT id FROM x_task WHERE work = $1 AND activity_token = $2 AND task_status = $3 LIMIT 1",
            &[&work_id, &activity_token, &"completed"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if let Some(prev) = prev_row {
        let prev_id: String = prev.get("id");
        tx.execute("UPDATE x_task SET task_status = $1 WHERE id = $2", &[&"pending", &prev_id])
            .await
            .map_err(|_| AppError::Internal)?;
    }
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("rejected".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn task_transfer(
    pool: Extension<Pool>,
    axum::extract::Path((id, new_person)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_one(
            "SELECT id, title, work, activity, activity_token, person, task_status FROM x_task WHERE id = $1 FOR UPDATE",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let task_status: String = row.get("task_status");
    if task_status != "active" && task_status != "pending" && task_status != "processing" {
        tx.commit().await.map_err(|_| AppError::Internal)?;
        return Err(AppError::BadRequest(format!("task status is {}，无法转办", task_status)));
    }
    let old_person: String = row.get("person");
    tx.execute("UPDATE x_task SET person = $1 WHERE id = $2", &[&new_person, &id])
        .await
        .map_err(|_| AppError::Internal)?;
    let work_id: String = row.get("work");
    let id2 = Uuid::new_v4().to_string();
    let content = format!("transferred from {} to {}", old_person, new_person);
    tx.execute(
            "INSERT INTO x_record (id, work_id, task_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id2, &work_id, &id, &"transfer", &content, &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("newPerson".to_string(), Value::String(new_person)),
        ]),
    ))))
}

pub async fn gateway_join(
    pool: Extension<Pool>,
    axum::extract::Path((work_id, activity_token)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let rows = tx
        .query(
            "SELECT id, task_status FROM x_task WHERE work = $1 AND activity_token = $2",
            &[&work_id, &activity_token],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if rows.is_empty() {
        tx.commit().await.map_err(|_| AppError::Internal)?;
        return Err(AppError::NotFound);
    }
    let all_completed = rows.iter().all(|r| {
        let status: String = r.get("task_status");
        status == "completed"
    });
    if !all_completed {
        tx.commit().await.map_err(|_| AppError::Internal)?;
        return Err(AppError::BadRequest("not all tasks completed for this gateway".to_string()));
    }
    let id2 = Uuid::new_v4().to_string();
    tx.execute(
            "INSERT INTO x_record (id, work_id, record_type, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id2, &work_id, &"gateway_join", &format!("gateway {} joined", activity_token), &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("workId".to_string(), Value::String(work_id)),
            ("activityToken".to_string(), Value::String(activity_token)),
            ("joined".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn gateway_fork(
    pool: Extension<Pool>,
    axum::extract::Path(gateway_instance_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let transitions = client
        .query(
            "SELECT id, work_id, to_activity, condition FROM x_process_transition \
             WHERE gateway_instance = $1 AND deleted_at IS NULL",
            &[&gateway_instance_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut created_tasks = Vec::new();
    for row in &transitions {
        let work_id: String = row.get("work_id");
        let to_activity: String = row.get("to_activity");
        let task_id = Uuid::new_v4().to_string();
        let token_id = Uuid::new_v4().to_string();
        let title = format!("Fork Task: {}", to_activity);
        client
            .execute(
                "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7)",
                &[&task_id, &title, &work_id, &to_activity, &token_id.to_string(), &"", &"pending"],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        created_tasks.push(Value::String(task_id));
    }

    Ok(Json(ActionResult::success(Value::Array(created_tasks))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Timer (DB-persisted, with cancel / restore / cron support)
// ──────────────────────────────────────────────────────────────────────────────

pub mod timer {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tokio::time::{interval, Duration};

    #[derive(Clone)]
    pub struct TimerRegistry {
        pub jobs: Arc<RwLock<HashMap<String, TimerJob>>>,
        pool: Option<Pool>,
    }

    impl Default for TimerRegistry {
        fn default() -> Self {
            Self {
                jobs: Arc::new(RwLock::new(HashMap::new())),
                pool: None,
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct TimerJob {
        pub id: String,
        pub work_id: String,
        pub task_id: Option<String>,
        pub fire_at: NaiveDateTime,
        pub cron: Option<String>,
        pub created_at: NaiveDateTime,
        pub kind: String,
    }

    impl TimerRegistry {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_pool(pool: Pool) -> Self {
            Self {
                jobs: Arc::new(RwLock::new(HashMap::new())),
                pool: Some(pool),
            }
        }

        pub fn start_background(&self) {
            if self.pool.is_none() {
                return;
            }
            let pool = self.pool.as_ref().unwrap().clone();
            let registry = self.clone();
            tokio::spawn(async move {
                let mut ticker = interval(Duration::from_secs(30));
                loop {
                    ticker.tick().await;
                    let jobs = registry.jobs.read().await.clone();
                    let now = chrono::Utc::now().naive_utc();
                    for job in jobs.values() {
                        if job.fire_at <= now && job.cron.is_none() {
                            let _ = Self::fire(&pool, job).await;
                        }
                    }
                }
            });
        }

        async fn fire(pool: &Pool, job: &TimerJob) -> Result<(), AppError> {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            match job.kind.as_str() {
                "expire" => {
                    if let Some(ref task_id) = job.task_id {
                        client
                            .execute(
                                "UPDATE x_task SET task_status = $1 WHERE id = $2 AND task_status = $3",
                                &[&"expired", task_id, &"active"],
                            )
                            .await
                            .map_err(|_| AppError::Internal)?;
                    }
                }
                _ => {}
            }
            client
                .execute(
                    "UPDATE x_timer_job SET fired_at = NOW() WHERE id = $1",
                    &[&job.id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(())
        }

    pub async fn start(
        &self,
        work_id: String,
        task_id: Option<String>,
        fire_at: NaiveDateTime,
        cron: Option<String>,
    ) -> Result<String, AppError> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().naive_utc();
        let kind = if cron.is_some() { "cron" } else { "once" };
        let fire_at_str = fire_at.format("%Y-%m-%d %H:%M:%S").to_string();
        let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

            let job = TimerJob {
                id: id.clone(),
                work_id: work_id.clone(),
                task_id: task_id.clone(),
                fire_at,
                cron: cron.clone(),
                created_at: now,
                kind: kind.to_string(),
            };

        if let Some(ref pool) = self.pool {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            client
                .execute(
                    "INSERT INTO x_timer_job (id, work_id, task_id, fire_at, cron, created_at, kind) \
                     VALUES ($1, $2, $3, $4, $5, $6, $7)",
                    &[&id, &work_id, &task_id, &fire_at_str, &cron, &now_str, &kind],
                )
                .await
                .map_err(|_| AppError::Internal)?;
        }

        let mut jobs = self.jobs.write().await;
        jobs.insert(id.clone(), job);

        Ok(id)
    }

        pub async fn cancel(&self, job_id: &str) -> Result<(), AppError> {
            if let Some(ref pool) = self.pool {
                let client = pool.get().await.map_err(|_| AppError::Internal)?;
                client
                    .execute(
                        "UPDATE x_timer_job SET cancelled_at = NOW() WHERE id = $1",
                        &[&job_id],
                    )
                    .await
                    .map_err(|_| AppError::Internal)?;
            }

            let mut jobs = self.jobs.write().await;
            jobs.remove(job_id);

            Ok(())
        }

        pub async fn restore(&self, pool: &Pool) -> Result<(), AppError> {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let rows = client
                .query(
                    "SELECT id, work_id, task_id, fire_at, cron, created_at, kind \
                     FROM x_timer_job \
                     WHERE fired_at IS NULL AND cancelled_at IS NULL",
                    &[],
                )
                .await
                .map_err(|_| AppError::Internal)?;

            let mut jobs = self.jobs.write().await;
            for row in rows.iter() {
                let fire_at_str: String = row.get("fire_at");
                let created_at_str: String = row.get("created_at");
                let fire_at = NaiveDateTime::parse_from_str(&fire_at_str, "%Y-%m-%d %H:%M:%S")
                    .unwrap_or_else(|_| chrono::Utc::now().naive_utc());
                let created_at = NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
                    .unwrap_or_else(|_| chrono::Utc::now().naive_utc());
                let job = TimerJob {
                    id: row.get("id"),
                    work_id: row.get("work_id"),
                    task_id: row.get("task_id"),
                    fire_at,
                    cron: row.get("cron"),
                    created_at,
                    kind: row.get("kind"),
                };
                jobs.insert(job.id.clone(), job);
            }

            Ok(())
        }

        pub async fn register(&self, job: TimerJob) {
            let mut jobs = self.jobs.write().await;
            jobs.insert(job.id.clone(), job);
        }
    }
}

pub async fn start_timer(
    timer: Extension<timer::TimerRegistry>,
    axum::extract::Json(req): axum::extract::Json<serde_json::Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let work_id: String = req.get("workId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let task_id: Option<String> = req.get("taskId").and_then(|v| v.as_str()).map(|s| s.to_string());
    let fire_at_str: String = req.get("fireAt").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let cron: Option<String> = req.get("cron").and_then(|v| v.as_str()).map(|s| s.to_string());

    let fire_at = if cron.is_some() {
        chrono::Utc::now().naive_utc()
    } else {
        NaiveDateTime::parse_from_str(&fire_at_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| AppError::BadRequest("invalid fireAt format, expected YYYY-MM-DD HH:MM:SS".to_string()))?
    };

    let job_id = timer.start(work_id.clone(), task_id, fire_at, cron).await?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("jobId".to_string(), Value::String(job_id)),
            ("workId".to_string(), Value::String(work_id)),
        ]),
    ))))
}

pub async fn cancel_timer(
    timer: Extension<timer::TimerRegistry>,
    axum::extract::Path(job_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    timer.cancel(&job_id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("jobId".to_string(), Value::String(job_id)),
            ("cancelled".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[cfg(test)]
mod tests_generated;
