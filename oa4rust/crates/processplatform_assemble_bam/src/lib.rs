use axum::{
    extract::Extension,
    Json, Router, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use uuid::Uuid;

/// 流程平台BAM装配模块
/// 提供BAM（Business Activity Monitoring）相关的装配服务
pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreateBamRequest {
    pub name: Option<String>,
    pub definition: Option<String>,
}

/// 获取BAM配置
/// 返回BAM的当前配置信息
pub async fn get_bam_config(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xname, xdefinition, xenabled FROM x_bam_config WHERE xid = $1 LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if rows.is_empty() {
        return Ok(Json(ActionResult::error("not found")));
    }

    let row = &rows[0];
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(row.get("xname"))),
            ("enabled".to_string(), Value::Bool(row.get("xenabled"))),
            ("definition".to_string(), Value::String(row.get("xdefinition"))),
        ]),
    ))))
}

/// 创建BAM实例
/// 根据请求创建新的BAM监控实例
pub async fn create_bam(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreateBamRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let definition = req.definition.unwrap_or_default();

    client
        .execute(
            "INSERT INTO x_bam_config (xid, xname, xdefinition, xenabled) VALUES ($1, $2, $3, true)",
            &[&id, &name, &definition],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("created".to_string(), Value::Bool(true)),
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("definition".to_string(), Value::String(definition)),
        ]),
    ))))
}

/// 列出BAM实例
/// 返回指定类别下的所有BAM实例列表
pub async fn list_bams(
    pool: Extension<Pool>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xcategory FROM x_bam_config WHERE xcategory = $1 ORDER BY \"xcreateTime\" DESC LIMIT 100",
            &[&category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("name".to_string(), Value::String(row.get("xname"))),
                ("category".to_string(), Value::String(row.get("xcategory"))),
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

/// 删除BAM实例
/// 根据ID删除指定的BAM监控实例
pub async fn delete_bam(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("DELETE FROM x_bam_config WHERE xid = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 获取BAM状态
/// 返回BAM实例的当前运行状态
pub async fn get_bam_status(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let task_count_rows = client
        .query(
            "SELECT COUNT(*) as cnt FROM x_pp_c_task WHERE xbamConfig = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let active_metrics: i64 = if !task_count_rows.is_empty() {
        task_count_rows[0].get("cnt")
    } else {
        0
    };

    let status = if active_metrics > 0 { "running" } else { "idle" };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("status".to_string(), Value::String(status.to_string())),
            ("activeMetrics".to_string(), Value::Number(serde_json::Number::from(active_metrics))),
        ]),
    ))))
}

/// 流程平台BAM装配路由
/// 路由前缀: /jaxrs/processplatform/assemble/bam/*
pub fn processplatform_assemble_bam_router() -> Router {
    Router::new()
        .route("/jaxrs/processplatform/assemble/bam/get/{id}", get(get_bam_config))
        .route("/jaxrs/processplatform/assemble/bam/create", post(create_bam))
        .route("/jaxrs/processplatform/assemble/bam/list/{category}", get(list_bams))
        .route("/jaxrs/processplatform/assemble/bam/delete/{id}", post(delete_bam))
        .route("/jaxrs/processplatform/assemble/bam/status/{id}", get(get_bam_status))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    processplatform_assemble_bam_router().layer(axum::extract::Extension(pool))
}

// ──────────────────────────────────────────────────────────────────────────────
// Period statistics — completed tasks
// ──────────────────────────────────────────────────────────────────────────────

pub async fn period_list_completed_task_application(
    pool: Extension<Pool>,
    axum::extract::Path(application_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.id, t.title, t.person, t.task_status, t.start_time, t.end_time, w.application
             FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'completed' AND w.application = $1
             ORDER BY t.end_time DESC",
            &[&application_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("title".to_string(), Value::String(r.get("title"))),
            ("person".to_string(), Value::String(r.get("person"))),
            ("taskStatus".to_string(), Value::String(r.get("task_status"))),
            ("\"startTime\"".to_string(), Value::String(r.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(r.get("end_time"))),
            ("application".to_string(), Value::String(r.get("application"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_completed_task_unit(
    pool: Extension<Pool>,
    axum::extract::Path(unit_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.id, t.title, t.person, t.task_status, t.start_time, t.end_time
             FROM x_task t WHERE t.task_status = 'completed' AND t.person IN (
                 SELECT id FROM x_org_person WHERE unit_id = $1
             ) ORDER BY t.end_time DESC",
            &[&unit_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("title".to_string(), Value::String(r.get("title"))),
            ("person".to_string(), Value::String(r.get("person"))),
            ("taskStatus".to_string(), Value::String(r.get("task_status"))),
            ("\"startTime\"".to_string(), Value::String(r.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(r.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_completed_work_application(
    pool: Extension<Pool>,
    axum::extract::Path(application_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.id, w.title, w.work_status, w.start_time, w.end_time
             FROM x_work w WHERE w.work_status = 'completed' AND w.application = $1
             ORDER BY w.end_time DESC",
            &[&application_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("title".to_string(), Value::String(r.get("title"))),
            ("workStatus".to_string(), Value::String(r.get("work_status"))),
            ("\"startTime\"".to_string(), Value::String(r.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(r.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_completed_work_unit(
    pool: Extension<Pool>,
    axum::extract::Path(unit_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.id, w.title, w.work_status, w.start_time, w.end_time
             FROM x_work w WHERE w.work_status = 'completed' AND w.creator IN (
                 SELECT id FROM x_org_person WHERE unit_id = $1
             ) ORDER BY w.end_time DESC",
            &[&unit_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("title".to_string(), Value::String(r.get("title"))),
            ("workStatus".to_string(), Value::String(r.get("work_status"))),
            ("\"startTime\"".to_string(), Value::String(r.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(r.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Period count functions — completed tasks
// ──────────────────────────────────────────────────────────────────────────────

pub async fn period_list_count_completed_task_application_applicationId_process_processId_activity_activityId_by_unit(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id, activity_id)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'completed' AND w.application = $1 AND w.process = $2 AND t.activity = $3
             GROUP BY t.person",
            &[&application_id, &process_id, &activity_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_count_completed_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id, activity_id, unit_id, person_id)): axum::extract::Path<(String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'completed' AND w.application = $1 AND w.process = $2 AND t.activity = $3 AND t.person = $4",
            &[&application_id, &process_id, &activity_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let cnt: i64 = if !rows.is_empty() { rows[0].get("cnt") } else { 0 };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(cnt))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn period_list_count_completed_task_application_applicationId_process_processId_unit_unit_person_person_by_activity(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id, unit_id, person_id)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.activity, COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'completed' AND w.application = $1 AND w.process = $2 AND t.person = $3
             GROUP BY t.activity",
            &[&application_id, &process_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("activity".to_string(), Value::String(r.get("activity"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_count_completed_task_application_applicationId_unit_unit_person_person_by_process(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, unit_id, person_id)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.process, COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'completed' AND w.application = $1 AND t.person = $2
             GROUP BY w.process",
            &[&application_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("process".to_string(), Value::String(r.get("process"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_count_completed_task_unit_unit_person_person_by_application(
    pool: Extension<Pool>,
    axum::extract::Path((unit_id, person_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.application, COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'completed' AND t.person = $1
             GROUP BY w.application",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("application".to_string(), Value::String(r.get("application"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Period count functions — completed work
// ──────────────────────────────────────────────────────────────────────────────

pub async fn period_list_count_completed_work_application_applicationId_process_processId_by_unit(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'completed' AND application = $1 AND process = $2",
            &[&application_id, &process_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let cnt: i64 = if !rows.is_empty() { rows[0].get("cnt") } else { 0 };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(cnt))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn period_list_count_completed_work_application_applicationId_process_processId_unit_unit_person_person(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id, person_id)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'completed' AND application = $1 AND process = $2 AND creator = $3",
            &[&application_id, &process_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let cnt: i64 = if !rows.is_empty() { rows[0].get("cnt") } else { 0 };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(cnt))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn period_list_count_completed_work_application_applicationId_unit_unit_person_person_by_process(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, person_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT process, COUNT(*) as cnt FROM x_work WHERE work_status = 'completed' AND application = $1 AND creator = $2 GROUP BY process",
            &[&application_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("process".to_string(), Value::String(r.get("process"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_count_completed_work_unit_unit_person_person_by_application(
    pool: Extension<Pool>,
    axum::extract::Path((person_id)): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT application, COUNT(*) as cnt FROM x_work WHERE work_status = 'completed' AND creator = $1 GROUP BY application",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("application".to_string(), Value::String(r.get("application"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Period count functions — expired tasks
// ──────────────────────────────────────────────────────────────────────────────

pub async fn period_list_count_expired_task_application_applicationId_process_processId_activity_activityId_by_unit(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id, activity_id)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.person, COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'expired' AND w.application = $1 AND w.process = $2 AND t.activity = $3
             GROUP BY t.person",
            &[&application_id, &process_id, &activity_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("person".to_string(), Value::String(r.get("person"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_count_expired_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id, activity_id, person_id)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'expired' AND w.application = $1 AND w.process = $2 AND t.activity = $3 AND t.person = $4",
            &[&application_id, &process_id, &activity_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let cnt: i64 = if !rows.is_empty() { rows[0].get("cnt") } else { 0 };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(cnt))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn period_list_count_expired_task_application_applicationId_process_processId_unit_unit_person_person_by_activity(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id, person_id)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.activity, COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'expired' AND w.application = $1 AND w.process = $2 AND t.person = $3
             GROUP BY t.activity",
            &[&application_id, &process_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("activity".to_string(), Value::String(r.get("activity"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_count_expired_task_application_applicationId_unit_unit_person_person_by_process(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, person_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.process, COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'expired' AND w.application = $1 AND t.person = $2
             GROUP BY w.process",
            &[&application_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("process".to_string(), Value::String(r.get("process"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_count_expired_task_unit_unit_person_person_by_application(
    pool: Extension<Pool>,
    axum::extract::Path(person_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.application, COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'expired' AND t.person = $1
             GROUP BY w.application",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("application".to_string(), Value::String(r.get("application"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Period count functions — expired work
// ──────────────────────────────────────────────────────────────────────────────

pub async fn period_list_count_expired_work_application_applicationId_process_processId_by_unit(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'expired' AND application = $1 AND process = $2",
            &[&application_id, &process_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let cnt: i64 = if !rows.is_empty() { rows[0].get("cnt") } else { 0 };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(cnt))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn period_list_count_expired_work_application_applicationId_process_processId_unit_unit_person_person(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id, person_id)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'expired' AND application = $1 AND process = $2 AND creator = $3",
            &[&application_id, &process_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let cnt: i64 = if !rows.is_empty() { rows[0].get("cnt") } else { 0 };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(cnt))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn period_list_count_expired_work_application_applicationId_unit_unit_person_person_by_process(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, person_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT process, COUNT(*) as cnt FROM x_work WHERE work_status = 'expired' AND application = $1 AND creator = $2 GROUP BY process",
            &[&application_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("process".to_string(), Value::String(r.get("process"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_count_expired_work_unit_unit_person_person_by_application(
    pool: Extension<Pool>,
    axum::extract::Path(person_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT application, COUNT(*) as cnt FROM x_work WHERE work_status = 'expired' AND creator = $1 GROUP BY application",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("application".to_string(), Value::String(r.get("application"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Period count functions — start tasks
// ──────────────────────────────────────────────────────────────────────────────

pub async fn period_list_count_start_task_application_applicationId_process_processId_activity_activityId_by_unit(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id, activity_id)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.person, COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'started' AND w.application = $1 AND w.process = $2 AND t.activity = $3
             GROUP BY t.person",
            &[&application_id, &process_id, &activity_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("person".to_string(), Value::String(r.get("person"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_count_start_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id, activity_id, person_id)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'started' AND w.application = $1 AND w.process = $2 AND t.activity = $3 AND t.person = $4",
            &[&application_id, &process_id, &activity_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let cnt: i64 = if !rows.is_empty() { rows[0].get("cnt") } else { 0 };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(cnt))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn period_list_count_start_task_application_applicationId_process_processId_unit_unit_person_person_by_activity(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id, person_id)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.activity, COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'started' AND w.application = $1 AND w.process = $2 AND t.person = $3
             GROUP BY t.activity",
            &[&application_id, &process_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("activity".to_string(), Value::String(r.get("activity"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_count_start_task_application_applicationId_unit_unit_person_person_by_process(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, person_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.process, COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'started' AND w.application = $1 AND t.person = $2
             GROUP BY w.process",
            &[&application_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("process".to_string(), Value::String(r.get("process"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_count_start_task_unit_unit_person_person_by_application(
    pool: Extension<Pool>,
    axum::extract::Path(person_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.application, COUNT(*) as cnt FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'started' AND t.person = $1
             GROUP BY w.application",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("application".to_string(), Value::String(r.get("application"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Period count functions — start work
// ──────────────────────────────────────────────────────────────────────────────

pub async fn period_list_count_start_work_application_applicationId_process_processId_by_unit(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'started' AND application = $1 AND process = $2",
            &[&application_id, &process_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let cnt: i64 = if !rows.is_empty() { rows[0].get("cnt") } else { 0 };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(cnt))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn period_list_count_start_work_application_applicationId_process_processId_unit_unit_person_person(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, process_id, person_id)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'started' AND application = $1 AND process = $2 AND creator = $3",
            &[&application_id, &process_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let cnt: i64 = if !rows.is_empty() { rows[0].get("cnt") } else { 0 };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(cnt))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn period_list_count_start_work_application_applicationId_unit_unit_person_person_by_process(
    pool: Extension<Pool>,
    axum::extract::Path((application_id, person_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT process, COUNT(*) as cnt FROM x_work WHERE work_status = 'started' AND application = $1 AND creator = $2 GROUP BY process",
            &[&application_id, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("process".to_string(), Value::String(r.get("process"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_count_start_work_unit_unit_person_person_by_application(
    pool: Extension<Pool>,
    axum::extract::Path(person_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT application, COUNT(*) as cnt FROM x_work WHERE work_status = 'started' AND creator = $1 GROUP BY application",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("application".to_string(), Value::String(r.get("application"))),
            ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Period list functions — expired
// ──────────────────────────────────────────────────────────────────────────────

pub async fn period_list_expired_task_application(
    pool: Extension<Pool>,
    axum::extract::Path(application_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.id, t.title, t.person, t.task_status, t.start_time, t.end_time
             FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'expired' AND w.application = $1
             ORDER BY t.end_time DESC",
            &[&application_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("title".to_string(), Value::String(r.get("title"))),
            ("person".to_string(), Value::String(r.get("person"))),
            ("taskStatus".to_string(), Value::String(r.get("task_status"))),
            ("\"startTime\"".to_string(), Value::String(r.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(r.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_expired_task_unit(
    pool: Extension<Pool>,
    axum::extract::Path(unit_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.id, t.title, t.person, t.task_status, t.start_time, t.end_time
             FROM x_task t WHERE t.task_status = 'expired' AND t.person IN (
                 SELECT id FROM x_org_person WHERE unit_id = $1
             ) ORDER BY t.end_time DESC",
            &[&unit_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("title".to_string(), Value::String(r.get("title"))),
            ("person".to_string(), Value::String(r.get("person"))),
            ("taskStatus".to_string(), Value::String(r.get("task_status"))),
            ("\"startTime\"".to_string(), Value::String(r.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(r.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_expired_work_application(
    pool: Extension<Pool>,
    axum::extract::Path(application_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.id, w.title, w.work_status, w.start_time, w.end_time
             FROM x_work w WHERE w.work_status = 'expired' AND w.application = $1
             ORDER BY w.end_time DESC",
            &[&application_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("title".to_string(), Value::String(r.get("title"))),
            ("workStatus".to_string(), Value::String(r.get("work_status"))),
            ("\"startTime\"".to_string(), Value::String(r.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(r.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_expired_work_unit(
    pool: Extension<Pool>,
    axum::extract::Path(unit_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.id, w.title, w.work_status, w.start_time, w.end_time
             FROM x_work w WHERE w.work_status = 'expired' AND w.creator IN (
                 SELECT id FROM x_org_person WHERE unit_id = $1
             ) ORDER BY w.end_time DESC",
            &[&unit_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("title".to_string(), Value::String(r.get("title"))),
            ("workStatus".to_string(), Value::String(r.get("work_status"))),
            ("\"startTime\"".to_string(), Value::String(r.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(r.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Period list functions — start
// ──────────────────────────────────────────────────────────────────────────────

pub async fn period_list_start_task_application(
    pool: Extension<Pool>,
    axum::extract::Path(application_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.id, t.title, t.person, t.task_status, t.start_time, t.end_time
             FROM x_task t JOIN x_work w ON t.work = w.id
             WHERE t.task_status = 'started' AND w.application = $1
             ORDER BY t.start_time DESC",
            &[&application_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("title".to_string(), Value::String(r.get("title"))),
            ("person".to_string(), Value::String(r.get("person"))),
            ("taskStatus".to_string(), Value::String(r.get("task_status"))),
            ("\"startTime\"".to_string(), Value::String(r.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(r.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_start_task_unit(
    pool: Extension<Pool>,
    axum::extract::Path(unit_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.id, t.title, t.person, t.task_status, t.start_time, t.end_time
             FROM x_task t WHERE t.task_status = 'started' AND t.person IN (
                 SELECT id FROM x_org_person WHERE unit_id = $1
             ) ORDER BY t.start_time DESC",
            &[&unit_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("title".to_string(), Value::String(r.get("title"))),
            ("person".to_string(), Value::String(r.get("person"))),
            ("taskStatus".to_string(), Value::String(r.get("task_status"))),
            ("\"startTime\"".to_string(), Value::String(r.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(r.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_start_work_application(
    pool: Extension<Pool>,
    axum::extract::Path(application_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.id, w.title, w.work_status, w.start_time, w.end_time
             FROM x_work w WHERE w.work_status = 'started' AND w.application = $1
             ORDER BY w.start_time DESC",
            &[&application_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("title".to_string(), Value::String(r.get("title"))),
            ("workStatus".to_string(), Value::String(r.get("work_status"))),
            ("\"startTime\"".to_string(), Value::String(r.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(r.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn period_list_start_work_unit(
    pool: Extension<Pool>,
    axum::extract::Path(unit_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.id, w.title, w.work_status, w.start_time, w.end_time
             FROM x_work w WHERE w.work_status = 'started' AND w.creator IN (
                 SELECT id FROM x_org_person WHERE unit_id = $1
             ) ORDER BY w.start_time DESC",
            &[&unit_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("title".to_string(), Value::String(r.get("title"))),
            ("workStatus".to_string(), Value::String(r.get("work_status"))),
            ("\"startTime\"".to_string(), Value::String(r.get("start_time"))),
            ("\"endTime\"".to_string(), Value::String(r.get("end_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ──────────────────────────────────────────────────────────────────────────────
// State statistics
// ──────────────────────────────────────────────────────────────────────────────

pub async fn state_applicationtstubs_trigger(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let total_works = client
        .query("SELECT COUNT(*) as cnt FROM x_work WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total_works: i64 = if !total_works.is_empty() { total_works[0].get("cnt") } else { 0 };

    let pending_works = client
        .query("SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'pending' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let pending_works: i64 = if !pending_works.is_empty() { pending_works[0].get("cnt") } else { 0 };

    let triggered_apps = client
        .query("SELECT DISTINCT application FROM x_work WHERE deleted_at IS NULL AND application IS NOT NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = triggered_apps.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("application".to_string(), Value::String(r.get("application"))),
            ("totalWorks".to_string(), Value::Number(serde_json::Number::from(total_works))),
            ("pendingWorks".to_string(), Value::Number(serde_json::Number::from(pending_works))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn state_category(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT category, COUNT(*) as total,
                    COUNT(*) FILTER (WHERE work_status = 'pending') as pending,
                    COUNT(*) FILTER (WHERE work_status = 'processing') as processing,
                    COUNT(*) FILTER (WHERE work_status = 'completed') as completed,
                    COUNT(*) FILTER (WHERE work_status = 'expired') as expired
             FROM x_work WHERE deleted_at IS NULL GROUP BY category",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|r| {
        Value::Object(serde_json::Map::from_iter([
            ("category".to_string(), Value::String(r.get("category"))),
            ("total".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("total")))),
            ("pending".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("pending")))),
            ("processing".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("processing")))),
            ("completed".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("completed")))),
            ("expired".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("expired")))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn state_category_trigger(
    pool: Extension<Pool>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT COUNT(*) as total,
                    COUNT(*) FILTER (WHERE work_status = 'pending') as pending,
                    COUNT(*) FILTER (WHERE work_status = 'processing') as processing,
                    COUNT(*) FILTER (WHERE work_status = 'completed') as completed
             FROM x_work WHERE deleted_at IS NULL AND category = $1",
            &[&category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let row = if !rows.is_empty() { &rows[0] } else {
        return Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("category".to_string(), Value::String(category)),
                ("total".to_string(), Value::Number(serde_json::Number::from(0))),
                ("pending".to_string(), Value::Number(serde_json::Number::from(0))),
                ("processing".to_string(), Value::Number(serde_json::Number::from(0))),
                ("completed".to_string(), Value::Number(serde_json::Number::from(0))),
            ]),
        ))))
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("category".to_string(), Value::String(category)),
            ("total".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("total")))),
            ("pending".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("pending")))),
            ("processing".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("processing")))),
            ("completed".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("completed")))),
        ]),
    ))))
}

pub async fn state_organization(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let total_persons = client
        .query("SELECT COUNT(*) as cnt FROM x_org_person WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total_persons: i64 = if !total_persons.is_empty() { total_persons[0].get("cnt") } else { 0 };

    let total_units = client
        .query("SELECT COUNT(*) as cnt FROM x_org_unit WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total_units: i64 = if !total_units.is_empty() { total_units[0].get("cnt") } else { 0 };

    let total_groups = client
        .query("SELECT COUNT(*) as cnt FROM x_org_group WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total_groups: i64 = if !total_groups.is_empty() { total_groups[0].get("cnt") } else { 0 };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("totalPersons".to_string(), Value::Number(serde_json::Number::from(total_persons))),
            ("totalUnits".to_string(), Value::Number(serde_json::Number::from(total_units))),
            ("totalGroups".to_string(), Value::Number(serde_json::Number::from(total_groups))),
        ]),
    ))))
}

pub async fn state_running(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let pending_works = client
        .query("SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'pending' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let pending_works: i64 = if !pending_works.is_empty() { pending_works[0].get("cnt") } else { 0 };

    let processing_works = client
        .query("SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'processing' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let processing_works: i64 = if !processing_works.is_empty() { processing_works[0].get("cnt") } else { 0 };

    let pending_tasks = client
        .query("SELECT COUNT(*) as cnt FROM x_task WHERE task_status = 'pending' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let pending_tasks: i64 = if !pending_tasks.is_empty() { pending_tasks[0].get("cnt") } else { 0 };

    let processing_tasks = client
        .query("SELECT COUNT(*) as cnt FROM x_task WHERE task_status = 'processing' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let processing_tasks: i64 = if !processing_tasks.is_empty() { processing_tasks[0].get("cnt") } else { 0 };

    let started_tasks = client
        .query("SELECT COUNT(*) as cnt FROM x_task WHERE task_status = 'started' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let started_tasks: i64 = if !started_tasks.is_empty() { started_tasks[0].get("cnt") } else { 0 };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("pendingWork".to_string(), Value::Number(serde_json::Number::from(pending_works))),
            ("processingWork".to_string(), Value::Number(serde_json::Number::from(processing_works))),
            ("pendingTask".to_string(), Value::Number(serde_json::Number::from(pending_tasks))),
            ("processingTask".to_string(), Value::Number(serde_json::Number::from(processing_tasks))),
            ("startedTask".to_string(), Value::Number(serde_json::Number::from(started_tasks))),
        ]),
    ))))
}

pub async fn state_summary(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let total_works = client
        .query("SELECT COUNT(*) as cnt FROM x_work WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total_works: i64 = if !total_works.is_empty() { total_works[0].get("cnt") } else { 0 };

    let total_tasks = client
        .query("SELECT COUNT(*) as cnt FROM x_task WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total_tasks: i64 = if !total_tasks.is_empty() { total_tasks[0].get("cnt") } else { 0 };

    let completed_works = client
        .query("SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'completed' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let completed_works: i64 = if !completed_works.is_empty() { completed_works[0].get("cnt") } else { 0 };

    let completed_tasks = client
        .query("SELECT COUNT(*) as cnt FROM x_task WHERE task_status = 'completed' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let completed_tasks: i64 = if !completed_tasks.is_empty() { completed_tasks[0].get("cnt") } else { 0 };

    let pending_works = client
        .query("SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'pending' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let pending_works: i64 = if !pending_works.is_empty() { pending_works[0].get("cnt") } else { 0 };

    let pending_tasks = client
        .query("SELECT COUNT(*) as cnt FROM x_task WHERE task_status = 'pending' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let pending_tasks: i64 = if !pending_tasks.is_empty() { pending_tasks[0].get("cnt") } else { 0 };

    let processing_works = client
        .query("SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'processing' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let processing_works: i64 = if !processing_works.is_empty() { processing_works[0].get("cnt") } else { 0 };

    let expired_works = client
        .query("SELECT COUNT(*) as cnt FROM x_work WHERE work_status = 'expired' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let expired_works: i64 = if !expired_works.is_empty() { expired_works[0].get("cnt") } else { 0 };

    let expired_tasks = client
        .query("SELECT COUNT(*) as cnt FROM x_task WHERE task_status = 'expired' AND deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let expired_tasks: i64 = if !expired_tasks.is_empty() { expired_tasks[0].get("cnt") } else { 0 };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("totalWork".to_string(), Value::Number(serde_json::Number::from(total_works))),
            ("totalTask".to_string(), Value::Number(serde_json::Number::from(total_tasks))),
            ("completedWork".to_string(), Value::Number(serde_json::Number::from(completed_works))),
            ("completedTask".to_string(), Value::Number(serde_json::Number::from(completed_tasks))),
            ("pendingWork".to_string(), Value::Number(serde_json::Number::from(pending_works))),
            ("pendingTask".to_string(), Value::Number(serde_json::Number::from(pending_tasks))),
            ("processingWork".to_string(), Value::Number(serde_json::Number::from(processing_works))),
            ("expiredWork".to_string(), Value::Number(serde_json::Number::from(expired_works))),
            ("expiredTask".to_string(), Value::Number(serde_json::Number::from(expired_tasks))),
        ]),
    ))))
}
