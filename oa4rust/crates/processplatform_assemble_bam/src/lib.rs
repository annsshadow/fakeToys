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
            "SELECT xname, xdefinition, xenabled FROM X.BAM_CONFIG WHERE xid = $1 LIMIT 1",
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
            "INSERT INTO X.BAM_CONFIG (xid, xname, xdefinition, xenabled) VALUES ($1, $2, $3, true)",
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
            "SELECT xid, xname, xcategory FROM X.BAM_CONFIG WHERE xcategory = $1 ORDER BY xcreateTime DESC LIMIT 100",
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
        .execute("DELETE FROM X.BAM_CONFIG WHERE xid = $1", &[&id])
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
            "SELECT COUNT(*) as cnt FROM X.PP_C_TASK WHERE xbamConfig = $1",
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

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    processplatform_assemble_bam_router().layer(axum::extract::Extension(pool))
}



pub async fn period_list_completed_task_application() -> Result<Json<ActionResult<Value>>, AppError> { Err(AppError::NotImplemented) }

pub async fn period_list_completed_task_unit() -> Result<Json<ActionResult<Value>>, AppError> { Err(AppError::NotImplemented) }

pub async fn period_list_completed_work_application() -> Result<Json<ActionResult<Value>>, AppError> { Err(AppError::NotImplemented) }

pub async fn period_list_completed_work_unit() -> Result<Json<ActionResult<Value>>, AppError> { Err(AppError::NotImplemented) }

pub async fn period_list_count_completed_task_application_applicationId_process_processId_activity_activityId_by_unit() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_completed_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_completed_task_application_applicationId_process_processId_unit_unit_person_person_by_activity() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_completed_task_application_applicationId_unit_unit_person_person_by_process() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_completed_task_unit_unit_person_person_by_application() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_completed_work_application_applicationId_process_processId_by_unit() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_completed_work_application_applicationId_process_processId_unit_unit_person_person() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_completed_work_application_applicationId_unit_unit_person_person_by_process() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_completed_work_unit_unit_person_person_by_application() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_expired_task_application_applicationId_process_processId_activity_activityId_by_unit() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_expired_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_expired_task_application_applicationId_process_processId_unit_unit_person_person_by_activity() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_expired_task_application_applicationId_unit_unit_person_person_by_process() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_expired_task_unit_unit_person_person_by_application() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_expired_work_application_applicationId_process_processId_by_unit() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_expired_work_application_applicationId_process_processId_unit_unit_person_person() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_expired_work_application_applicationId_unit_unit_person_person_by_process() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_expired_work_unit_unit_person_person_by_application() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_start_task_application_applicationId_process_processId_activity_activityId_by_unit() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_start_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_start_task_application_applicationId_process_processId_unit_unit_person_person_by_activity() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_start_task_application_applicationId_unit_unit_person_person_by_process() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_start_task_unit_unit_person_person_by_application() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_start_work_application_applicationId_process_processId_by_unit() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_start_work_application_applicationId_process_processId_unit_unit_person_person() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_start_work_application_applicationId_unit_unit_person_person_by_process() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_count_start_work_unit_unit_person_person_by_application() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn period_list_expired_task_application() -> Result<Json<ActionResult<Value>>, AppError> { Err(AppError::NotImplemented) }

pub async fn period_list_expired_task_unit() -> Result<Json<ActionResult<Value>>, AppError> { Err(AppError::NotImplemented) }

pub async fn period_list_expired_work_application() -> Result<Json<ActionResult<Value>>, AppError> { Err(AppError::NotImplemented) }

pub async fn period_list_expired_work_unit() -> Result<Json<ActionResult<Value>>, AppError> { Err(AppError::NotImplemented) }

pub async fn period_list_start_task_application() -> Result<Json<ActionResult<Value>>, AppError> { Err(AppError::NotImplemented) }

pub async fn period_list_start_task_unit() -> Result<Json<ActionResult<Value>>, AppError> { Err(AppError::NotImplemented) }

pub async fn period_list_start_work_application() -> Result<Json<ActionResult<Value>>, AppError> { Err(AppError::NotImplemented) }

pub async fn period_list_start_work_unit() -> Result<Json<ActionResult<Value>>, AppError> { Err(AppError::NotImplemented) }

pub async fn state_applicationtstubs_trigger() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn state_category() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn state_category_trigger() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn state_organization() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn state_running() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}

pub async fn state_summary() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(AppError::NotImplemented)
}
