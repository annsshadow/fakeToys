# -*- coding: utf-8 -*-
import re

# Read lib.rs
with open("crates/processplatform_service_processing/src/lib.rs", "r", encoding="utf-8") as f:
    content = f.read()

stub_funcs = """
pub async fn task_v2_id_remove(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client.execute("DELETE FROM x_task WHERE id = $1", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Null)))
}

pub async fn task_add(
    pool: Extension<Pool>,
    axum::extract::Path(_id): axum::extract::Path<String>,
    axum::extract::Json(_req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Null)))
}

pub async fn work_reroute_activity_activitytype(
    pool: Extension<Pool>,
    axum::extract::Path((_activity_id, _activity_type)): axum::extract::Path<(String, String)>,
    axum::extract::Json(_req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Null)))
}

pub async fn work_rollback(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool.get().await.map_err(|_| AppError::Internal)?;
    let _ = id;
    Ok(Json(ActionResult::success(Value::Null)))
}
"""

content = content.replace("#[cfg(test)]\n#[cfg(test)]\nmod tests;", stub_funcs + "#[cfg(test)]\n#[cfg(test)]\nmod tests;")

with open("crates/processplatform_service_processing/src/lib.rs", "w", encoding="utf-8") as f:
    f.write(content)
print("lib.rs updated")
