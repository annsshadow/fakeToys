const fs = require('fs');
const libContent = fs.readFileSync('crates/processplatform_service_processing/src/lib.rs', 'utf8');
const stubs = `
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
}`;
const newLib = libContent.replace(/#\[cfg\(test\)\]\n#\[cfg\(test\)\]\nmod tests;/, stubs + '\n#[cfg(test)]\n#[cfg(test)]\nmod tests;');
fs.writeFileSync('crates/processplatform_service_processing/src/lib.rs', newLib, 'utf8');
console.log('lib.rs updated');

const routesContent = fs.readFileSync('crates/processplatform_service_processing/src/routes.rs', 'utf8');
const newRoutes = '        .route("/jaxrs/processplatform/service/processing/task/v2/{id}/remove", delete(crate::task_v2_id_remove))\n        .route("/jaxrs/processplatform/service/processing/task/{id}/add", put(crate::task_add))\n        .route("/jaxrs/processplatform/service/processing/work/reroute/activity/{activityId}/activitytype/{activityType}", put(crate::work_reroute_activity_activitytype))\n        .route("/jaxrs/processplatform/service/processing/work/{id}/rollback", put(crate::work_rollback))';
const newRoutesContent = routesContent.replace('        .layer(Extension(pool))', newRoutes + '\n        .layer(Extension(pool))');
fs.writeFileSync('crates/processplatform_service_processing/src/routes.rs', newRoutesContent, 'utf8');
console.log('routes.rs updated');
