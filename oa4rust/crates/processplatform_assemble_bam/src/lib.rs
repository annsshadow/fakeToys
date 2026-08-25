use axum::{
    extract::Extension,
    Json, Router, routing::get, routing::post, routing::delete,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::middleware::require_owner;
use shared::session::Session;
use shared::{error::AppError, response::ActionResult};
use uuid::Uuid;

/// 娴佺▼骞冲彴BAM瑁呴厤妯″潡
/// 鎻愪緵BAM锛圔usiness Activity Monitoring锛夌浉鍏崇殑瑁呴厤鏈嶅姟
pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreateBamRequest {
    pub name: Option<String>,
    pub definition: Option<String>,
}

/// 鑾峰彇BAM閰嶇疆
/// 杩斿洖BAM鐨勫綋鍓嶉厤缃俊鎭?
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

/// 鍒涘缓BAM瀹炰緥
/// 鏍规嵁璇锋眰鍒涘缓鏂扮殑BAM鐩戞帶瀹炰緥
pub async fn create_bam(
    pool: Extension<Pool>,
    session: Extension<Session>,
    axum::extract::Json(req): Json<CreateBamRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // x_bam_config 鏃?owner/creator 鍒楋紝鎸夌郴缁熼厤缃鐞嗭細浠?admin 鍙啓
    require_owner(&pool, &session, "").await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let definition = req.definition.unwrap_or_default();

    let result = client
        .execute(
            "INSERT INTO x_bam_config (xid, xname, xdefinition, xenabled) VALUES ($1, $2, $3, true)",
            &[&id, &name, &definition],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("created".to_string(), Value::Bool(result > 0)),
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("definition".to_string(), Value::String(definition)),
        ]),
    ))))
}

/// 鍒楀嚭BAM瀹炰緥
/// 杩斿洖鎸囧畾绫诲埆涓嬬殑鎵€鏈塀AM瀹炰緥鍒楄〃
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

/// 鍒犻櫎BAM瀹炰緥
/// 鏍规嵁ID鍒犻櫎鎸囧畾鐨凚AM鐩戞帶瀹炰緥
pub async fn delete_bam(
    pool: Extension<Pool>,
    session: Extension<Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // x_bam_config 鏃?owner/creator 鍒楋紝鎸夌郴缁熼厤缃鐞嗭細浠?admin 鍙啓
    require_owner(&pool, &session, "").await?;
    // x_bam_config 鏃?deleted_at 鍒楋紝绂佹鐗╃悊鍒犻櫎浠ラ槻鏁版嵁涓㈠け
    let _ = &id;
    Ok(Json(ActionResult::error(
        "physical delete not supported for this entity",
    )))
}

/// 鑾峰彇BAM鐘舵€?
/// 杩斿洖BAM瀹炰緥鐨勫綋鍓嶈繍琛岀姸鎬?
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

/// 娴佺▼骞冲彴BAM瑁呴厤璺敱
/// 璺敱鍓嶇紑: /jaxrs/processplatform/assemble/bam/*
pub fn processplatform_assemble_bam_router() -> Router {
    Router::new()
        .route("/jaxrs/processplatform/assemble/bam/get/{id}", get(get_bam_config))
        .route("/jaxrs/processplatform/assemble/bam/create", post(create_bam))
        .route("/jaxrs/processplatform/assemble/bam/list/{category}", get(list_bams))
        .route("/jaxrs/processplatform/assemble/bam/delete/{id}", post(delete_bam))
        .route("/jaxrs/processplatform/assemble/bam/status/{id}", get(get_bam_status))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/task/application", get(crate::period_list_completed_task_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/task/{unit}", get(crate::period_list_completed_task_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/application/{work}", get(crate::period_list_completed_work_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/{work}/{unit}", get(crate::period_list_completed_work_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/task/application/process/activity/by/{count}/{applicationId}/{processId}/{activityId}/{unit}", get(crate::period_list_count_completed_task_application_applicationId_process_processId_activity_activityId_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/task/application/process/activity/{count}/{applicationId}/{processId}/{activityId}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_completed_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/task/application/process/by/activity/{count}/{applicationId}/{processId}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_completed_task_application_applicationId_process_processId_unit_unit_person_person_by_activity))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/task/application/by/process/{count}/{applicationId}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_completed_task_application_applicationId_unit_unit_person_person_by_process))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/task/by/application/{count}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_completed_task_unit_unit_person_person_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/application/process/by/{count}/{work}/{applicationId}/{processId}/{unit}", get(crate::period_list_count_completed_work_application_applicationId_process_processId_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/application/process/{count}/{work}/{applicationId}/{processId}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_completed_work_application_applicationId_process_processId_unit_unit_person_person))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/application/by/process/{count}/{work}/{applicationId}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_completed_work_application_applicationId_unit_unit_person_person_by_process))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/by/application/{count}/{work}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_completed_work_unit_unit_person_person_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/task/application/process/activity/by/{count}/{applicationId}/{processId}/{activityId}/{unit}", get(crate::period_list_count_expired_task_application_applicationId_process_processId_activity_activityId_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/task/application/process/activity/{count}/{applicationId}/{processId}/{activityId}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_expired_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/task/application/process/by/activity/{count}/{applicationId}/{processId}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_expired_task_application_applicationId_process_processId_unit_unit_person_person_by_activity))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/task/application/by/process/{count}/{applicationId}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_expired_task_application_applicationId_unit_unit_person_person_by_process))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/task/by/application/{count}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_expired_task_unit_unit_person_person_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/application/process/by/{count}/{work}/{applicationId}/{processId}/{unit}", get(crate::period_list_count_expired_work_application_applicationId_process_processId_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/application/process/{count}/{work}/{applicationId}/{processId}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_expired_work_application_applicationId_process_processId_unit_unit_person_person))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/application/by/process/{count}/{work}/{applicationId}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_expired_work_application_applicationId_unit_unit_person_person_by_process))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/by/application/{count}/{work}/{unit}/{unit}/{person}/{person}", get(crate::period_list_count_expired_work_unit_unit_person_person_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/task/application/process/activity/by/{count}/{start}/{applicationId}/{processId}/{activityId}/{unit}", post(crate::period_list_count_start_task_application_applicationId_process_processId_activity_activityId_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/task/application/process/activity/{count}/{start}/{applicationId}/{processId}/{activityId}/{unit}/{unit}/{person}/{person}", post(crate::period_list_count_start_task_application_applicationId_process_processId_activity_activityId_unit_unit_person_person))
        .route("/jaxrs/processplatform/assemble/bam/period/list/task/application/process/by/activity/{count}/{start}/{applicationId}/{processId}/{unit}/{unit}/{person}/{person}", post(crate::period_list_count_start_task_application_applicationId_process_processId_unit_unit_person_person_by_activity))
        .route("/jaxrs/processplatform/assemble/bam/period/list/task/application/by/process/{count}/{start}/{applicationId}/{unit}/{unit}/{person}/{person}", post(crate::period_list_count_start_task_application_applicationId_unit_unit_person_person_by_process))
        .route("/jaxrs/processplatform/assemble/bam/period/list/task/by/application/{count}/{start}/{unit}/{unit}/{person}/{person}", post(crate::period_list_count_start_task_unit_unit_person_person_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/application/process/by/{count}/{start}/{work}/{applicationId}/{processId}/{unit}", post(crate::period_list_count_start_work_application_applicationId_process_processId_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/application/process/{count}/{start}/{work}/{applicationId}/{processId}/{unit}/{unit}/{person}/{person}", post(crate::period_list_count_start_work_application_applicationId_process_processId_unit_unit_person_person))
        .route("/jaxrs/processplatform/assemble/bam/period/list/application/by/process/{count}/{start}/{work}/{applicationId}/{unit}/{unit}/{person}/{person}", post(crate::period_list_count_start_work_application_applicationId_unit_unit_person_person_by_process))
        .route("/jaxrs/processplatform/assemble/bam/period/list/by/application/{count}/{start}/{work}/{unit}/{unit}/{person}/{person}", post(crate::period_list_count_start_work_unit_unit_person_person_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/task/application", get(crate::period_list_expired_task_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/task/{unit}", get(crate::period_list_expired_task_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/application/{work}", get(crate::period_list_expired_work_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/{work}/{unit}", get(crate::period_list_expired_work_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/task/application/{start}", post(crate::period_list_start_task_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/task/{start}/{unit}", post(crate::period_list_start_task_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/application/{start}/{work}", post(crate::period_list_start_work_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/{start}/{work}/{unit}", post(crate::period_list_start_work_unit))
        .route("/jaxrs/processplatform/assemble/bam/state/trigger/{category}", post(crate::state_category_trigger))
        // 鈹€鈹€ plan002 U2锛欽ava 绮剧‘璺緞闂悎锛圙ET锛岃 final_coverage_sweep 鍙拌处锛夆攢鈹€
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/task/applicationstubs", get(bam_stubs_completed_task_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/task/unitstubs", get(bam_stubs_completed_task_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/work/applicationstubs", get(bam_stubs_completed_work_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/completed/work/unitstubs", get(bam_stubs_completed_work_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/task/applicationstubs", get(bam_stubs_expired_task_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/task/unitstubs", get(bam_stubs_expired_task_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/work/applicationstubs", get(bam_stubs_expired_work_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/expired/work/unitstubs", get(bam_stubs_expired_work_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/start/task/applicationstubs", get(bam_stubs_start_task_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/start/task/unitstubs", get(bam_stubs_start_task_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/start/work/applicationstubs", get(bam_stubs_start_work_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/start/work/unitstubs", get(bam_stubs_start_work_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/completed/task/application/{applicationId}/process/{processId}/activity/{activityId}/by/unit", get(bam_count_completed_task_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/completed/task/application/{applicationId}/process/{processId}/activity/{activityId}/unit/{unit}/person/{person}", get(bam_count_completed_task_total))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/completed/task/application/{applicationId}/process/{processId}/unit/{unit}/person/{person}/by/activity", get(bam_count_completed_task_by_activity))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/completed/task/application/{applicationId}/unit/{unit}/person/{person}/by/process", get(bam_count_completed_task_by_process))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/completed/task/unit/{unit}/person/{person}/by/application", get(bam_count_completed_task_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/completed/work/application/{applicationId}/process/{processId}/by/unit", get(bam_count_completed_work_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/completed/work/application/{applicationId}/process/{processId}/unit/{unit}/person/{person}", get(bam_count_completed_work_total))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/completed/work/application/{applicationId}/unit/{unit}/person/{person}/by/process", get(bam_count_completed_work_by_process))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/completed/work/unit/{unit}/person/{person}/by/application", get(bam_count_completed_work_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/expired/task/application/{applicationId}/process/{processId}/activity/{activityId}/by/unit", get(bam_count_expired_task_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/expired/task/application/{applicationId}/process/{processId}/activity/{activityId}/unit/{unit}/person/{person}", get(bam_count_expired_task_total))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/expired/task/application/{applicationId}/process/{processId}/unit/{unit}/person/{person}/by/activity", get(bam_count_expired_task_by_activity))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/expired/task/application/{applicationId}/unit/{unit}/person/{person}/by/process", get(bam_count_expired_task_by_process))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/expired/task/unit/{unit}/person/{person}/by/application", get(bam_count_expired_task_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/expired/work/application/{applicationId}/process/{processId}/by/unit", get(bam_count_expired_work_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/expired/work/application/{applicationId}/process/{processId}/unit/{unit}/person/{person}", get(bam_count_expired_work_total))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/expired/work/application/{applicationId}/unit/{unit}/person/{person}/by/process", get(bam_count_expired_work_by_process))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/expired/work/unit/{unit}/person/{person}/by/application", get(bam_count_expired_work_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/start/task/application/{applicationId}/process/{processId}/activity/{activityId}/by/unit", get(bam_count_start_task_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/start/task/application/{applicationId}/process/{processId}/activity/{activityId}/unit/{unit}/person/{person}", get(bam_count_start_task_total))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/start/task/application/{applicationId}/process/{processId}/unit/{unit}/person/{person}/by/activity", get(bam_count_start_task_by_activity))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/start/task/application/{applicationId}/unit/{unit}/person/{person}/by/process", get(bam_count_start_task_by_process))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/start/task/unit/{unit}/person/{person}/by/application", get(bam_count_start_task_by_application))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/start/work/application/{applicationId}/process/{processId}/by/unit", get(bam_count_start_work_by_unit))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/start/work/application/{applicationId}/process/{processId}/unit/{unit}/person/{person}", get(bam_count_start_work_total))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/start/work/application/{applicationId}/unit/{unit}/person/{person}/by/process", get(bam_count_start_work_by_process))
        .route("/jaxrs/processplatform/assemble/bam/period/list/count/start/work/unit/{unit}/person/{person}/by/application", get(bam_count_start_work_by_application))
        .route("/jaxrs/processplatform/assemble/bam/state/applicationtstubs/trigger", get(state_applicationtstubs_trigger))
        .route("/jaxrs/processplatform/assemble/bam/state/category", get(state_category))
        .route("/jaxrs/processplatform/assemble/bam/state/category/trigger", get(state_category_trigger_all))
        .route("/jaxrs/processplatform/assemble/bam/state/summary", get(state_summary))
        .route("/jaxrs/processplatform/assemble/bam/state/running", get(state_running))
        .route("/jaxrs/processplatform/assemble/bam/state/organization", get(state_organization))
        .route("/jaxrs/processplatform/assemble/bam/delete/{id}", delete(delete_bam))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    processplatform_assemble_bam_router().layer(axum::extract::Extension(pool))
}

// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€
// Period statistics 鈥?completed tasks
// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€
// Period count functions 鈥?completed tasks
// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€
// Period count functions 鈥?completed work
// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€
// Period count functions 鈥?expired tasks
// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€
// Period count functions 鈥?expired work
// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€
// Period count functions 鈥?start tasks
// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€
// Period count functions 鈥?start work
// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€
// Period list functions 鈥?expired
// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€
// Period list functions 鈥?start
// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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

// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€
// State statistics
// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

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
        .query("SELECT COALESCE(application, '') AS application FROM x_work WHERE deleted_at IS NULL AND application IS NOT NULL GROUP BY application ORDER BY application", &[])
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
            "SELECT COALESCE(a.\"xapplicationCategory\", '') AS category,
                    COUNT(*) as total,
                    COUNT(*) FILTER (WHERE w.work_status = 'pending') as pending,
                    COUNT(*) FILTER (WHERE w.work_status = 'processing') as processing,
                    COUNT(*) FILTER (WHERE w.work_status = 'completed') as completed,
                    COUNT(*) FILTER (WHERE w.work_status = 'expired') as expired
             FROM x_work w
             LEFT JOIN pp_e_application a ON w.application = a.xid
             WHERE w.deleted_at IS NULL
             GROUP BY COALESCE(a.\"xapplicationCategory\", '')",
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
    session: Extension<Session>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 瑙﹀彂缁熻涓虹郴缁熺骇鎿嶄綔锛屾寜 ownerless 璧勬簮澶勭悊锛氫粎 admin 鍙啓
    require_owner(&pool, &session, "").await?;
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


// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€
// plan002 U2 路 Java 绮剧‘璺緞闂悎锛?2 涓洃鎺х鐐癸級
//
// 鏁版嵁婧愬叏閮ㄤ负鏃㈡湁琛細x_task / x_work锛坢igration 020锛夈€亁_org_person锛?22锛夈€?
// pp_e_application锛?32锛夈€傜粺璁″彛寰勶細
//   completed = 宸插畬鎴愶紱expired = 宸茶秴鏃朵笖鏈畬鎴愶紱start = 宸插惎鍔ㄤ笖鏈畬鎴愩€?
// 杩囨护鍙傛暟涓€寰嬭蛋 $1..$5 鍗犱綅绗︼紙application/process/activity/unit/person锛?
// NULL 琛ㄧず涓嶈繃婊わ級锛屼笉鎷兼帴浠讳綍鐢ㄦ埛杈撳叆銆傛湰鑺傜鐐瑰潎涓哄彧璇昏仛鍚堬紝鏃?IDOR 闈€?
// 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

use deadpool_postgres::tokio_postgres::types::ToSql;

/// 鍛ㄦ湡璋撹瘝锛?瀹炰綋, 鍛ㄦ湡) 鈫?WHERE 鐗囨銆傜函鍑芥暟锛屽崟娴嬪浐鍖栫粺璁″彛寰勩€?
fn period_predicate(kind: &str, period: &str) -> &'static str {
    match (kind, period) {
        ("task", "completed") => "t.task_status = 'completed'",
        ("task", "expired") => {
            "(t.end_time IS NOT NULL AND t.end_time < NOW() AND t.task_status IS DISTINCT FROM 'completed')"
        }
        ("task", "start") => {
            "(t.start_time IS NOT NULL AND t.task_status IS DISTINCT FROM 'completed')"
        }
        (_, "completed") => "w.work_status = 'completed'",
        (_, "expired") => {
            "(w.end_time IS NOT NULL AND w.end_time < NOW() AND w.work_status IS DISTINCT FROM 'completed')"
        }
        _ => "(w.start_time IS NOT NULL AND w.work_status IS DISTINCT FROM 'completed')",
    }
}

#[derive(Default)]
struct PeriodFilter {
    application: Option<String>,
    process: Option<String>,
    activity: Option<String>,
    unit: Option<String>,
    person: Option<String>,
}

/// 缁熶竴鑱氬悎鎵ц锛歡roup = Some(缁村害) 杩斿洖 {count, data:[{key,count}]}锛汵one 杩斿洖 {count}銆?
async fn period_count_query(
    pool: &Pool,
    kind: &str,
    period: &str,
    filter: &PeriodFilter,
    group: Option<&str>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    period_count_query_shaped(pool, kind, period, filter, group, false).await
}

/// 鍚屼笂锛沯ava_shape=true 鏃舵寜 Java 淇″皝杩斿洖瑁告暟缁勶紙data=鍒嗙粍鏁扮粍銆乧ount=缁勬暟锛夈€?
/// 浠呰涓哄姣旀姤鍛婂垪鍑虹殑绔偣鍚敤锛屽叾浣欒矾鐢变繚鎸?{count,data} 褰㈢姸銆?
async fn period_count_query_shaped(
    pool: &Pool,
    kind: &str,
    period: &str,
    filter: &PeriodFilter,
    group: Option<&str>,
    java_shape: bool,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let pred = period_predicate(kind, period);
    let (from, act_col, per_col) = if kind == "task" {
        (
            "FROM x_task t LEFT JOIN x_work w ON t.work = w.id LEFT JOIN x_org_person op ON t.person = op.id",
            "t.activity",
            "t.person",
        )
    } else {
        (
            "FROM x_work w LEFT JOIN x_org_person op ON w.creator = op.id",
            "NULL",
            "w.creator",
        )
    };
    let deleted_pred = if kind == "task" {
        "t.deleted_at IS NULL"
    } else {
        "w.deleted_at IS NULL"
    };
    let (select, group_by) = match group {
        Some(g) => {
            let key_col = match g {
                "unit" => "op.unit_id",
                "application" => "w.application",
                "process" => "w.process",
                "activity" => act_col,
                _ => per_col,
            };
            (
                format!("SELECT COALESCE({}, '') AS key, COUNT(*)::bigint AS cnt", key_col),
                " GROUP BY key ORDER BY cnt DESC, key",
            )
        }
        None => ("SELECT COUNT(*)::bigint AS cnt".to_string(), ""),
    };
    let sql = format!(
        "{select} {from} WHERE {deleted_pred} AND ({pred}) \
         AND ($1::text IS NULL OR w.application = $1::text) \
         AND ($2::text IS NULL OR w.process = $2::text) \
         AND ($3::text IS NULL OR {act_col} = $3::text) \
         AND ($4::text IS NULL OR op.unit_id = $4::text) \
         AND ($5::text IS NULL OR {per_col} = $5::text){group_by}"
    );
    let params: [&(dyn ToSql + Sync); 5] = [
        &filter.application,
        &filter.process,
        &filter.activity,
        &filter.unit,
        &filter.person,
    ];
    let rows = client
        .query(sql.as_str(), &params)
        .await
        .map_err(|_| AppError::Internal)?;

    match group {
        Some(_) => {
            let data: Vec<Value> = rows
                .iter()
                .map(|r| {
                    Value::Object(serde_json::Map::from_iter([
                        ("key".to_string(), Value::String(r.get("key"))),
                        ("count".to_string(), Value::Number(serde_json::Number::from(r.get::<_, i64>("cnt")))),
                    ]))
                })
                .collect();
            let count = data.len() as i64;
            if java_shape {
                return Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("count".to_string(), Value::Number(serde_json::Number::from(count))),
                    ("data".to_string(), Value::Array(data)),
                ]),
            ))))
        }
        None => {
            let total: i64 = rows.first().map(|r| r.get("cnt")).unwrap_or(0);
            // total 鍨嬬鐐?Java 瀹炴祴杩斿洖瑁告暟缁勶紙绌烘暟鎹椂涓?[]锛夛紝璁℃暟鏀句俊灏?
            if java_shape {
                return Ok(Json(ActionResult::java_success(Value::Array(vec![]), total, 0)));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([(
                    "count".to_string(),
                    Value::Number(serde_json::Number::from(total)),
                )]),
            ))))
        }
    }
}

/// GET /period/list/{period}/task/applicationstubs 鈥斺€?鎸夊簲鐢ㄥ垎缁勭殑浠诲姟閲忔々鍒楄〃銆?
async fn bam_stubs_task(pool: Extension<Pool>, period: &'static str) -> Result<Json<ActionResult<Value>>, AppError> {
    period_count_query_shaped(&pool.0, "task", period, &PeriodFilter::default(), Some("application"), true).await
}

/// GET /period/list/{period}/task/unitstubs 鈥斺€?鎸夊崟浣嶅垎缁勭殑浠诲姟閲忔々鍒楄〃銆?
async fn bam_stubs_task_unit(pool: Extension<Pool>, period: &'static str) -> Result<Json<ActionResult<Value>>, AppError> {
    period_count_query_shaped(&pool.0, "task", period, &PeriodFilter::default(), Some("unit"), true).await
}

/// GET /period/list/{period}/work/*stubs 鈥斺€?鎸夊簲鐢?鍗曚綅鍒嗙粍鐨勫伐浣滈噺妗╁垪琛ㄣ€?
async fn bam_stubs_work(pool: Extension<Pool>, period: &'static str, by_unit: bool) -> Result<Json<ActionResult<Value>>, AppError> {
    let group = if by_unit { "unit" } else { "application" };
    period_count_query_shaped(&pool.0, "work", period, &PeriodFilter::default(), Some(group), true).await
}

pub async fn bam_stubs_completed_task_by_application(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    bam_stubs_task(pool, "completed").await
}
pub async fn bam_stubs_completed_task_by_unit(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    bam_stubs_task_unit(pool, "completed").await
}
pub async fn bam_stubs_completed_work_by_application(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    bam_stubs_work(pool, "completed", false).await
}
pub async fn bam_stubs_completed_work_by_unit(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    bam_stubs_work(pool, "completed", true).await
}
pub async fn bam_stubs_expired_task_by_application(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    bam_stubs_task(pool, "expired").await
}
pub async fn bam_stubs_expired_task_by_unit(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    bam_stubs_task_unit(pool, "expired").await
}
pub async fn bam_stubs_expired_work_by_application(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    bam_stubs_work(pool, "expired", false).await
}
pub async fn bam_stubs_expired_work_by_unit(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    bam_stubs_work(pool, "expired", true).await
}
pub async fn bam_stubs_start_task_by_application(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    bam_stubs_task(pool, "start").await
}
pub async fn bam_stubs_start_task_by_unit(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    bam_stubs_task_unit(pool, "start").await
}
pub async fn bam_stubs_start_work_by_application(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    bam_stubs_work(pool, "start", false).await
}
pub async fn bam_stubs_start_work_by_unit(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    bam_stubs_work(pool, "start", true).await
}

type BamPath2 = axum::extract::Path<(String, String)>;
type BamPath3 = axum::extract::Path<(String, String, String)>;
type BamPath4 = axum::extract::Path<(String, String, String, String)>;
type BamPath5 = axum::extract::Path<(String, String, String, String, String)>;

/// completed/task 浜旂鍒囩墖锛圝ava 璺緞閫愪竴瀵归綈锛夈€?
pub async fn bam_count_completed_task_by_unit(pool: Extension<Pool>, p: BamPath3) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr, ac) = p.0;
    period_count_query(&pool.0, "task", "completed", &PeriodFilter { application: Some(a), process: Some(pr), activity: Some(ac), ..Default::default() }, Some("unit")).await
}
pub async fn bam_count_completed_task_total(pool: Extension<Pool>, p: BamPath5) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr, ac, u, pe) = p.0;
    period_count_query_shaped(&pool.0, "task", "completed", &PeriodFilter { application: Some(a), process: Some(pr), activity: Some(ac), unit: Some(u), person: Some(pe) }, None, true).await
}
pub async fn bam_count_completed_task_by_activity(pool: Extension<Pool>, p: BamPath4) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr, u, pe) = p.0;
    period_count_query(&pool.0, "task", "completed", &PeriodFilter { application: Some(a), process: Some(pr), unit: Some(u), person: Some(pe), ..Default::default() }, Some("activity")).await
}
pub async fn bam_count_completed_task_by_process(pool: Extension<Pool>, p: BamPath3) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, u, pe) = p.0;
    period_count_query(&pool.0, "task", "completed", &PeriodFilter { application: Some(a), unit: Some(u), person: Some(pe), ..Default::default() }, Some("process")).await
}
pub async fn bam_count_completed_task_by_application(pool: Extension<Pool>, p: BamPath2) -> Result<Json<ActionResult<Value>>, AppError> {
    let (u, pe) = p.0;
    period_count_query(&pool.0, "task", "completed", &PeriodFilter { unit: Some(u), person: Some(pe), ..Default::default() }, Some("application")).await
}

/// completed/work 鍥涚鍒囩墖銆?
pub async fn bam_count_completed_work_by_unit(pool: Extension<Pool>, p: BamPath2) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr) = p.0;
    period_count_query(&pool.0, "work", "completed", &PeriodFilter { application: Some(a), process: Some(pr), ..Default::default() }, Some("unit")).await
}
pub async fn bam_count_completed_work_total(pool: Extension<Pool>, p: BamPath4) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr, u, pe) = p.0;
    period_count_query_shaped(&pool.0, "work", "completed", &PeriodFilter { application: Some(a), process: Some(pr), unit: Some(u), person: Some(pe), ..Default::default() }, None, true).await
}
pub async fn bam_count_completed_work_by_process(pool: Extension<Pool>, p: BamPath3) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, u, pe) = p.0;
    period_count_query(&pool.0, "work", "completed", &PeriodFilter { application: Some(a), unit: Some(u), person: Some(pe), ..Default::default() }, Some("process")).await
}
pub async fn bam_count_completed_work_by_application(pool: Extension<Pool>, p: BamPath2) -> Result<Json<ActionResult<Value>>, AppError> {
    let (u, pe) = p.0;
    period_count_query(&pool.0, "work", "completed", &PeriodFilter { unit: Some(u), person: Some(pe), ..Default::default() }, Some("application")).await
}

/// expired/task 浜旂鍒囩墖銆?
pub async fn bam_count_expired_task_by_unit(pool: Extension<Pool>, p: BamPath3) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr, ac) = p.0;
    period_count_query(&pool.0, "task", "expired", &PeriodFilter { application: Some(a), process: Some(pr), activity: Some(ac), ..Default::default() }, Some("unit")).await
}
pub async fn bam_count_expired_task_total(pool: Extension<Pool>, p: BamPath5) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr, ac, u, pe) = p.0;
    period_count_query_shaped(&pool.0, "task", "expired", &PeriodFilter { application: Some(a), process: Some(pr), activity: Some(ac), unit: Some(u), person: Some(pe) }, None, true).await
}
pub async fn bam_count_expired_task_by_activity(pool: Extension<Pool>, p: BamPath4) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr, u, pe) = p.0;
    period_count_query(&pool.0, "task", "expired", &PeriodFilter { application: Some(a), process: Some(pr), unit: Some(u), person: Some(pe), ..Default::default() }, Some("activity")).await
}
pub async fn bam_count_expired_task_by_process(pool: Extension<Pool>, p: BamPath3) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, u, pe) = p.0;
    period_count_query(&pool.0, "task", "expired", &PeriodFilter { application: Some(a), unit: Some(u), person: Some(pe), ..Default::default() }, Some("process")).await
}
pub async fn bam_count_expired_task_by_application(pool: Extension<Pool>, p: BamPath2) -> Result<Json<ActionResult<Value>>, AppError> {
    let (u, pe) = p.0;
    period_count_query(&pool.0, "task", "expired", &PeriodFilter { unit: Some(u), person: Some(pe), ..Default::default() }, Some("application")).await
}

/// expired/work 鍥涚鍒囩墖銆?
pub async fn bam_count_expired_work_by_unit(pool: Extension<Pool>, p: BamPath2) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr) = p.0;
    period_count_query(&pool.0, "work", "expired", &PeriodFilter { application: Some(a), process: Some(pr), ..Default::default() }, Some("unit")).await
}
pub async fn bam_count_expired_work_total(pool: Extension<Pool>, p: BamPath4) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr, u, pe) = p.0;
    period_count_query_shaped(&pool.0, "work", "expired", &PeriodFilter { application: Some(a), process: Some(pr), unit: Some(u), person: Some(pe), ..Default::default() }, None, true).await
}
pub async fn bam_count_expired_work_by_process(pool: Extension<Pool>, p: BamPath3) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, u, pe) = p.0;
    period_count_query(&pool.0, "work", "expired", &PeriodFilter { application: Some(a), unit: Some(u), person: Some(pe), ..Default::default() }, Some("process")).await
}
pub async fn bam_count_expired_work_by_application(pool: Extension<Pool>, p: BamPath2) -> Result<Json<ActionResult<Value>>, AppError> {
    let (u, pe) = p.0;
    period_count_query(&pool.0, "work", "expired", &PeriodFilter { unit: Some(u), person: Some(pe), ..Default::default() }, Some("application")).await
}

/// start/task 浜旂鍒囩墖銆?
pub async fn bam_count_start_task_by_unit(pool: Extension<Pool>, p: BamPath3) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr, ac) = p.0;
    period_count_query(&pool.0, "task", "start", &PeriodFilter { application: Some(a), process: Some(pr), activity: Some(ac), ..Default::default() }, Some("unit")).await
}
pub async fn bam_count_start_task_total(pool: Extension<Pool>, p: BamPath5) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr, ac, u, pe) = p.0;
    period_count_query_shaped(&pool.0, "task", "start", &PeriodFilter { application: Some(a), process: Some(pr), activity: Some(ac), unit: Some(u), person: Some(pe) }, None, true).await
}
pub async fn bam_count_start_task_by_activity(pool: Extension<Pool>, p: BamPath4) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr, u, pe) = p.0;
    period_count_query(&pool.0, "task", "start", &PeriodFilter { application: Some(a), process: Some(pr), unit: Some(u), person: Some(pe), ..Default::default() }, Some("activity")).await
}
pub async fn bam_count_start_task_by_process(pool: Extension<Pool>, p: BamPath3) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, u, pe) = p.0;
    period_count_query(&pool.0, "task", "start", &PeriodFilter { application: Some(a), unit: Some(u), person: Some(pe), ..Default::default() }, Some("process")).await
}
pub async fn bam_count_start_task_by_application(pool: Extension<Pool>, p: BamPath2) -> Result<Json<ActionResult<Value>>, AppError> {
    let (u, pe) = p.0;
    period_count_query(&pool.0, "task", "start", &PeriodFilter { unit: Some(u), person: Some(pe), ..Default::default() }, Some("application")).await
}

/// start/work 鍥涚鍒囩墖銆?
pub async fn bam_count_start_work_by_unit(pool: Extension<Pool>, p: BamPath2) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr) = p.0;
    period_count_query(&pool.0, "work", "start", &PeriodFilter { application: Some(a), process: Some(pr), ..Default::default() }, Some("unit")).await
}
pub async fn bam_count_start_work_total(pool: Extension<Pool>, p: BamPath4) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, pr, u, pe) = p.0;
    period_count_query_shaped(&pool.0, "work", "start", &PeriodFilter { application: Some(a), process: Some(pr), unit: Some(u), person: Some(pe), ..Default::default() }, None, true).await
}
pub async fn bam_count_start_work_by_process(pool: Extension<Pool>, p: BamPath3) -> Result<Json<ActionResult<Value>>, AppError> {
    let (a, u, pe) = p.0;
    period_count_query(&pool.0, "work", "start", &PeriodFilter { application: Some(a), unit: Some(u), person: Some(pe), ..Default::default() }, Some("process")).await
}
pub async fn bam_count_start_work_by_application(pool: Extension<Pool>, p: BamPath2) -> Result<Json<ActionResult<Value>>, AppError> {
    let (u, pe) = p.0;
    period_count_query(&pool.0, "work", "start", &PeriodFilter { unit: Some(u), person: Some(pe), ..Default::default() }, Some("application")).await
}

/// GET /state/category/trigger 鈥斺€?鏃犲弬瑙﹀彂寮忓叏閲忓垎绫诲揩鐓э紙鐪熷疄鑱氬悎 SQL锛夈€?
pub async fn state_category_trigger_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT COALESCE(a.\"xapplicationCategory\", '') AS category,
                    COUNT(*) as total,
                    COUNT(*) FILTER (WHERE w.work_status = 'pending') as pending,
                    COUNT(*) FILTER (WHERE w.work_status = 'processing') as processing,
                    COUNT(*) FILTER (WHERE w.work_status = 'completed') as completed,
                    COUNT(*) FILTER (WHERE w.work_status = 'expired') as expired
             FROM x_work w
             LEFT JOIN pp_e_application a ON w.application = a.xid
             WHERE w.deleted_at IS NULL
             GROUP BY COALESCE(a.\"xapplicationCategory\", '')",
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
            ("triggered".to_string(), Value::Bool(true)),
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}
