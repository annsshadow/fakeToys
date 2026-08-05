use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

// CMS 内容实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CmsContent {
    pub id: String,
    pub title: String,
    pub category_id: String,
    pub status: String,
}

/// 获取 CMS 内容列表
/// 从数据库查询 x_cms_content 表
pub async fn content_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, category_id, status FROM x_cms_content ORDER BY create_time DESC LIMIT 20",
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
                ("categoryId".to_string(), Value::String(row.get("category_id"))),
                ("status".to_string(), Value::String(row.get("status"))),
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

/// 获取 CMS 内容详情
/// 从数据库查询 x_cms_content 表，按 ID 查询
pub async fn content_detail(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, title, category_id, status, content FROM x_cms_content WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("title".to_string(), Value::String(row.get("title"))),
        ("categoryId".to_string(), Value::String(row.get("category_id"))),
        ("status".to_string(), Value::String(row.get("status"))),
        (
            "content".to_string(),
            row.get::<_, Option<String>>("content")
                .map(Value::String)
                .unwrap_or(Value::Null),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 创建 CMS 核心服务路由
/// 注册以下路由：
/// - /jaxrs/cms/core/express/content/list - 内容列表
/// - /jaxrs/cms/core/express/content/detail/{id} - 内容详情
pub fn cms_core_express_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/cms/core/express/content/list", get(content_list))
        .route("/jaxrs/cms/core/express/content/detail/{id}", get(content_detail))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/cms_core_express/health", axum::routing::get(|| async { "TODO: cms_core_express - real implementation needed" }))
}