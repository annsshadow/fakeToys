use axum::{
    extract::{Extension, Multipart, Path},
    http::HeaderMap,
    Json,
};
use deadpool_postgres::Pool;
use serde::Serialize;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::response::ActionResult;

use auth::SessionManager;
use base64::Engine;
use uuid::Uuid;

// 允许的 MIME 类型白名单（Java 契约：jpeg/png/webp）
const ALLOWED_MIME_TYPES: &[&str] = &["image/jpeg", "image/png", "image/webp"];

// 最大文件大小（5MB）
pub const MAX_FILE_SIZE: u64 = 5 * 1024 * 1024;

// 头像存储目录（相对工作目录，不映射为 web 可访问路径）
pub const AVATAR_DIR: &str = "data/avatar";

// 头像信息响应 DTO
#[derive(Debug, Serialize)]
pub struct AvatarInfo {
    pub id: String,
    pub person_unique: String,
    pub mime_type: String,
    pub size: i64,
    pub url: String,
}

/// MIME 白名单校验
pub fn is_supported_mime(mime: &str) -> bool {
    ALLOWED_MIME_TYPES.contains(&mime)
}

/// 上传当前登录用户头像（PUT /jaxrs/person/icon）
///
/// 接收 multipart/form-data 文件，校验 MIME 白名单与 5MB 大小上限后写入
/// 本地目录 `data/avatar/{uuid}.{ext}`，DB `auth_person.icon` 存相对文件名。
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `session_manager`: 会话管理器
/// - `headers`: 请求头，从中提取 Bearer token
/// - `mut form`: multipart 表单，包含头像文件字段
pub async fn upload(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    mut form: Multipart,
) -> Result<Json<ActionResult<AvatarInfo>>, AppError> {
    let token = shared::middleware::extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    // 从 multipart 中提取文件字段（按文件名判断，跳过普通表单字段）
    let mut file_data: Option<Vec<u8>> = None;
    let mut mime_type: Option<String> = None;
    let mut filename: Option<String> = None;

    while let Some(field) = form
        .next_field()
        .await
        .map_err(|_| AppError::BadRequest("表单解析失败".to_string()))?
    {
        if field.file_name().is_none() {
            continue;
        }
        mime_type = field.content_type().map(|s| s.to_string());
        filename = field.file_name().map(|f| f.to_string());
        file_data = Some(
            field
                .bytes()
                .await
                .map_err(|_| AppError::BadRequest("文件读取失败".to_string()))?
                .to_vec(),
        );
        break;
    }

    let data = match file_data {
        Some(d) => d,
        None => return Ok(Json(ActionResult::error("未提供头像文件"))),
    };

    // 文件大小校验（5MB 上限）
    if data.len() as u64 > MAX_FILE_SIZE {
        return Ok(Json(ActionResult::error("文件大小超过限制（最大 5MB）")));
    }

    // MIME 类型白名单校验
    let mime = mime_type.as_deref().unwrap_or("");
    if !is_supported_mime(mime) {
        return Ok(Json(ActionResult::error(
            "不支持的文件类型，仅允许: jpg, png, webp",
        )));
    }

    let ext = get_extension(mime, &filename);
    let avatar_id = Uuid::new_v4().to_string();
    let rel_path = format!("{AVATAR_DIR}/{avatar_id}.{ext}");

    // 写入本地存储目录（不接入 file 模块、不承担文件迁移方案）
    tokio::fs::create_dir_all(AVATAR_DIR)
        .await
        .map_err(|_| AppError::Internal)?;
    tokio::fs::write(&rel_path, &data)
        .await
        .map_err(|_| AppError::Internal)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE auth_person SET icon = $1, updated_at = NOW() \
             WHERE unique_id = $2 AND locked = false AND deleted_at IS NULL",
            &[&rel_path, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let info = AvatarInfo {
        id: avatar_id,
        person_unique: session.person_unique,
        mime_type: mime.to_string(),
        size: data.len() as i64,
        url: rel_path,
    };

    Ok(Json(ActionResult::success(info)))
}

/// 获取当前登录用户头像（GET /jaxrs/person/icon）
pub async fn get_current_icon(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = shared::middleware::extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    load_icon_for_unique(&pool, &session.person_unique).await
}

/// 获取指定用户头像（GET /jaxrs/icon/{person}，flag 支持 unique_id/name/id）
pub async fn get_icon(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(person): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 校验请求方已登录
    let token = shared::middleware::extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let _session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT icon, unique_id FROM auth_person \
             WHERE (unique_id = $1 OR name = $1 OR id = $1) AND locked = false AND deleted_at IS NULL",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(r) => {
            let icon: Option<String> = r.get("icon");
            Ok(render_icon(icon).await)
        }
        None => Ok(Json(ActionResult::error("用户不存在"))),
    }
}

/// 按 unique_id 读取用户头像文件并返回 base64
async fn load_icon_for_unique(
    pool: &Pool,
    person_unique: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT icon FROM auth_person WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(r) => {
            let icon: Option<String> = r.get("icon");
            Ok(render_icon(icon).await)
        }
        None => Ok(Json(ActionResult::error("用户不存在"))),
    }
}

/// 读取存储文件并以 base64 返回；无头像或文件缺失时返回 exists=false
async fn render_icon(icon: Option<String>) -> Json<ActionResult<Value>> {
    let rel_path = icon.filter(|s| !s.is_empty());
    match rel_path {
        Some(rel) => match tokio::fs::read(&rel).await {
            Ok(bytes) => {
                let avatar = base64::engine::general_purpose::STANDARD.encode(&bytes);
                Json(ActionResult::success(json!({
                    "avatar": avatar,
                    "mimeType": mime_from_path(&rel),
                    "exists": true,
                })))
            }
            Err(_) => Json(ActionResult::success(json!({
                "avatar": "",
                "exists": false,
            }))),
        },
        None => Json(ActionResult::success(json!({
            "avatar": "",
            "exists": false,
        }))),
    }
}

/// 依据存储路径推导 MIME 类型
fn mime_from_path(rel: &str) -> String {
    if rel.ends_with(".jpg") || rel.ends_with(".jpeg") {
        "image/jpeg".to_string()
    } else if rel.ends_with(".png") {
        "image/png".to_string()
    } else if rel.ends_with(".webp") {
        "image/webp".to_string()
    } else {
        "application/octet-stream".to_string()
    }
}

/// 根据 MIME 类型或文件名提取文件扩展名
pub(crate) fn get_extension(mime: &str, filename: &Option<String>) -> String {
    match mime {
        "image/jpeg" => "jpg".to_string(),
        "image/png" => "png".to_string(),
        "image/webp" => "webp".to_string(),
        _ => {
            // 从文件名推断扩展名
            if let Some(name) = filename {
                if let Some(ext) = name.split('.').last() {
                    return ext.to_lowercase();
                }
            }
            "bin".to_string()
        }
    }
}