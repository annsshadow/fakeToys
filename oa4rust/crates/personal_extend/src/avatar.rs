use axum::{
    extract::{Extension, Multipart, Path},
    http::HeaderMap,
    Json,
};
use deadpool_postgres::Pool;
use serde::Serialize;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;

use auth::SessionManager;
use base64::Engine;
use uuid::Uuid;

// 允许的 MIME 类型白名单
const ALLOWED_MIME_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/webp",
    "image/bmp",
];

// 最大文件大小（5MB）
const MAX_FILE_SIZE: u64 = 5 * 1024 * 1024;

// 头像信息响应 DTO
#[derive(Debug, Serialize)]
pub struct AvatarInfo {
    pub id: String,
    pub person_unique: String,
    pub mime_type: String,
    pub size: i64,
    pub url: String,
}

// 上传头像
//
// 接收 multipart/form-data 文件，验证 MIME 类型后保存到数据库 avatar 字段。
// 需要当前用户已登录，且文件符合安全要求。
//
// # 参数
// - `pool`: 数据库连接池
// - `session_manager`: 会话管理器
// - `mut form`: multipart 表单，包含 avatar 文件字段
pub async fn upload(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: Extension<HeaderMap>,
    mut form: Multipart,
) -> Result<Json<ActionResult<AvatarInfo>>, AppError> {
    let token = extract_bearer_token(&headers)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    // 从 multipart 中提取文件
    let mut file_data: Option<Vec<u8>> = None;
    let mut mime_type: Option<String> = None;
    let mut filename: Option<String> = None;

    while let Some(field) = form
        .next_field()
        .await
        .map_err(|_| AppError::BadRequest("表单解析失败".to_string()))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name == "avatar" {
            // 尝试获取 MIME 类型
            mime_type = field
                .content_type()
                .map(|s| s.to_string());
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
    }

    let data = file_data.ok_or(AppError::BadRequest("未提供头像文件".to_string()))?;

    // 文件大小校验
    if data.len() as u64 > MAX_FILE_SIZE {
        return Ok(Json(ActionResult::error("文件大小超过限制（最大 5MB）")));
    }

    // MIME 类型白名单校验
    let mime = mime_type.as_deref().unwrap_or("");
    if !ALLOWED_MIME_TYPES.contains(&mime) {
        return Ok(Json(
            ActionResult::error("不支持的文件类型，仅允许: jpg, png, gif, webp, bmp"),
        ));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 生成头像存储路径
    let avatar_id = Uuid::new_v4().to_string();
    let file_path = format!(
        "/uploads/avatars/{}.{}",
        avatar_id,
        get_extension(mime, &filename)
    );

    // 将头像数据存入数据库（以 base64 存储，实际生产环境应存文件系统/对象存储）
    let avatar_base64 = base64::engine::general_purpose::STANDARD.encode(&data);

    // 更新 auth_person 表的 avatar 字段
    client
        .execute(
            "UPDATE auth_person SET avatar = $1, updated_at = NOW() WHERE unique_id = $2 AND locked = false",
            &[&avatar_base64, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let info = AvatarInfo {
        id: avatar_id,
        person_unique: session.person_unique,
        mime_type: mime.to_string(),
        size: data.len() as i64,
        url: file_path,
    };

    Ok(Json(ActionResult::success(info)))
}

// 获取头像
//
// 根据用户唯一标识从 auth_person 表读取头像数据（base64 编码）。
// 返回头像数据和存在标志，供前端直接使用。
//
// # 参数
// - `pool`: 数据库连接池
// - `id`: 路径参数，用户的唯一标识（unique_id）
pub async fn get_avatar(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT avatar FROM auth_person WHERE unique_id = $1 AND locked = false",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let avatar: Option<String> = row.get("avatar");

    match avatar {
        Some(data) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("avatar".to_string(), Value::String(data)),
                ("exists".to_string(), Value::Bool(true)),
            ],
        ))))),
        None => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("avatar".to_string(), Value::String("".to_string())),
                ("exists".to_string(), Value::Bool(false)),
            ],
        ))))),
    }
}

/// 根据 MIME 类型或文件名提取文件扩展名
pub(crate) fn get_extension(mime: &str, filename: &Option<String>) -> String {
    match mime {
        "image/jpeg" => "jpg".to_string(),
        "image/png" => "png".to_string(),
        "image/gif" => "gif".to_string(),
        "image/webp" => "webp".to_string(),
        "image/bmp" => "bmp".to_string(),
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

/// 从 Authorization header 中提取 Bearer token
pub(crate) fn extract_bearer_token(headers: &HeaderMap) -> Result<String, AppError> {
    let auth = headers
        .get(axum::http::header::AUTHORIZATION)
        .ok_or(AppError::Unauthorized)?
        .to_str()
        .map_err(|_| AppError::Unauthorized)?;

    let prefix = "Bearer ";
    if !auth.starts_with(prefix) {
        return Err(AppError::Unauthorized);
    }

    Ok(auth[prefix.len()..].to_string())
}
