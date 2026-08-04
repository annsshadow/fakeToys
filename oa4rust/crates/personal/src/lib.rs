use axum::{
    extract::Extension,
    routing::{get, post, put},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use shared::error::AppError;
use shared::response::ActionResult;

pub mod password;
pub mod reset;

// --- 数据模型 ---

#[derive(Debug, Deserialize)]
pub struct EditPersonRequest {
    pub name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PersonInfo {
    pub id: String,
    pub unique: String,
    pub name: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub icon: Option<String>,
}

// --- 处理器 ---

/// 查询当前用户个人信息
///
/// 从 auth_person 表读取首条未锁定人员记录，返回姓名、手机号、邮箱、头像等基本信息。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Ok(Json<ActionResult<PersonInfo>>)` : 查询成功，返回用户信息
/// - `Err(AppError::NotFound)`: 未找到有效用户记录
pub async fn get_person(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<PersonInfo>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person WHERE locked = false LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let info = PersonInfo {
        id: row.get("id"),
        unique: row.get("unique_id"),
        name: row.get("name"),
        mobile: row.get("mobile"),
        email: row.get("email"),
        icon: row.get("icon"),
    };

    Ok(Json(ActionResult::success(info)))
}

/// 更新当前用户个人信息
///
/// 支持部分更新（name、mobile、email 均为可选字段），未提供的字段保留原值。
/// 更新后同时刷新 updated_at 时间戳。
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `req`: 更新请求体，包含可选的 `name`、`mobile`、`email` 字段
///
/// # 返回
/// - `Ok(Json<ActionResult<PersonInfo>>)` : 更新成功，返回更新后的完整用户信息
/// - `Err(AppError)`: 数据库错误
pub async fn edit_person(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<EditPersonRequest>,
) -> Result<Json<ActionResult<PersonInfo>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email FROM auth_person WHERE locked = false LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let person_id: String = row.get("id");

    let name = req.name.unwrap_or_else(|| row.get("name"));
    let db_mobile: Option<String> = row.get("mobile");
    let db_email: Option<String> = row.get("email");
    let mobile = req.mobile.or(db_mobile);
    let email = req.email.or(db_email);

    client
        .execute(
            "UPDATE auth_person SET name = $1, mobile = $2, email = $3, updated_at = NOW() WHERE id = $4",
            &[&name, &mobile, &email, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let updated = PersonInfo {
        id: person_id,
        unique: row.get("unique_id"),
        name,
        mobile,
        email,
        icon: row.get("icon"),
    };

    Ok(Json(ActionResult::success(updated)))
}

// --- 路由注册 ---

/// 构建个人中心模块路由
///
/// 注册个人信息查询/更新、密码修改、密码重置等接口。
/// 挂载 ResetCodeStore 中间件层用于存储重置验证码。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Router`: Axum 路由实例
pub fn router(pool: Pool) -> Router {
    let reset_store = reset::ResetCodeStore::new();

    Router::new()
        .route("/jaxrs/person", get(get_person))
        .route("/jaxrs/person", put(edit_person))
        .route("/jaxrs/person/mockputtopost", post(edit_person))
        .route("/jaxrs/password", put(password::change))
        .route("/jaxrs/password/mockputtopost", post(password::change))
        .route("/jaxrs/reset/code", post(reset::send_code))
        .route("/jaxrs/reset/check", post(reset::check_code))
        .route("/jaxrs/reset/set", post(reset::reset_password))
        .layer(Extension(pool))
        .layer(Extension(reset_store))
}

#[cfg(test)]
mod tests;
