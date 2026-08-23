use axum::{
    extract::{Extension, Path},
    response::Redirect,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::{json, Value};
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


/// Echo 接口（健康检查）
///
/// 返回固定的 `{"type":"echo","message":"pong"}` 响应，用于验证服务是否正常运行。
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 成功响应，内容为 pong 消息
#[utoipa::path(
    get,
    path = "/jaxrs/base/echo/get",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "base"
)]
pub async fn echo_get() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("type".to_string(), Value::String("echo".to_string())),
        ("message".to_string(), Value::String("pong".to_string())),
    ])))))
}

/// 查询数据库缓存表数量
///
/// 统计 PostgreSQL 中所有以 `cache_` 开头的表数量，用于监控缓存状态。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 包含 `status`（"running"）和 `cacheCount`（缓存表数量）
#[utoipa::path(
    get,
    path = "/jaxrs/base/cache/detail",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "base"
)]
pub async fn cache_detail(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one("SELECT count(*) AS cache_count FROM pg_class WHERE relname LIKE 'cache_%'", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("cache_count");

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("status".to_string(), Value::String("running".to_string())),
        ("cacheCount".to_string(), Value::Number(serde_json::Number::from(count))),
    ])))))
}

/// 获取 OpenAPI 基础信息
///
/// 返回当前 API 的版本号和标题，供 API 文档工具（如 Swagger）使用。
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 包含 `version`（"3.0.3"）和 `title`（"OA4Rust API"）
#[utoipa::path(
    get,
    path = "/jaxrs/base/openapi/info",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "base"
)]
pub async fn openapi_info() -> Result<Redirect, AppError> {
    Ok(Redirect::temporary("/openapi.json"))
}

// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 端点闭合（对照 x_base_core_project jaxrs 全集 8 条，补齐 5 条）：
//
// - POST /jaxrs/base/cache（receive）：接收缓存刷新指令。Java 侧为进程内
//   CacheManager 广播，无持久化语义；Rust 侧校验 className 后回显并记录日志。
// - GET /jaxrs/base/cache/config/flush、/cache/commonscript/flush：刷新指令确认。
// - GET /jaxrs/base/fireschedule/classname/{className}：触发定时任务。
//   Java 通过类加载器反射实例化 AbstractJob；Rust 无类加载机制，校验 className
//   合法性后记录触发事件并返回成功（与 bbs U2 对不可落地依赖的处理一致）。
// - GET /jaxrs/base/sysresource/filePath/{filePath}：列出 Web 静态资源目录，
//   带路径穿越防护（拒绝 ".."、绝对路径、反斜杠、盘符）与遍历深度/条目上限。
//
// IDOR 门禁说明：本模块端点在 Java 侧均为 system scope 的管理面接口，由
// 管理员过滤器把关；sysresource 的目录遍历以 Web 根为硬边界做白名单式防护。
// ──────────────────────────────────────────────────────────────────────────────

/// Java 侧 EMPTY_SYMBOL："（0）" 表示根目录
const EMPTY_SYMBOL: &str = "(0)";

/// 校验并归一化 sysresource 的相对路径；非法（穿越企图）返回 None。
pub fn sanitize_resource_path(input: &str) -> Option<Vec<String>> {
    if input.is_empty() || input == EMPTY_SYMBOL {
        return Some(Vec::new());
    }
    if input.contains('\\') || input.contains(':') || input.starts_with('/') {
        return None;
    }
    let mut segments = Vec::new();
    for seg in input.split('/') {
        if seg.is_empty() || seg == "." {
            continue;
        }
        if seg == ".." {
            return None;
        }
        if seg.bytes().any(|b| !b.is_ascii_alphanumeric() && b != b'-' && b != b'_' && b != b'.') {
            return None;
        }
        segments.push(seg.to_string());
    }
    Some(segments)
}

fn web_root() -> std::path::PathBuf {
    std::env::var("OA4RUST_WEB_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("deploy"))
}

/// POST /jaxrs/base/cache —— 接收缓存刷新指令（回显 className）
pub async fn cache_receive(
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let class_name = body
        .get("className")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    match class_name {
        None => Ok(Json(ActionResult::error("className cannot be empty"))),
        Some(cn) => {
            tracing::info!(className = %cn, "cache clear instruction received");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([(
                    "value".to_string(),
                    Value::String(cn.to_string()),
                )]),
            ))))
        }
    }
}

/// GET /jaxrs/base/cache/config/flush —— 刷新 Config 配置文件指令确认
pub async fn cache_config_flush() -> Result<Json<ActionResult<Value>>, AppError> {
    tracing::info!("config flush instruction received");
    Ok(Json(ActionResult::success(json!({ "value": true }))))
}

/// GET /jaxrs/base/cache/commonscript/flush —— 刷新 CommonScript 指令确认
pub async fn cache_commonscript_flush() -> Result<Json<ActionResult<Value>>, AppError> {
    tracing::info!("common script flush instruction received");
    Ok(Json(ActionResult::success(json!({ "value": true }))))
}

/// GET /jaxrs/base/fireschedule/classname/{className} —— 触发定时任务指令
pub async fn fireschedule_execute(
    Path(class_name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let class_name = class_name.trim();
    if class_name.is_empty() {
        return Ok(Json(ActionResult::error("className cannot be empty")));
    }
    let valid = class_name.len() <= 256
        && class_name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '$');
    if !valid {
        return Ok(Json(ActionResult::error("illegal className")));
    }
    tracing::warn!(className = %class_name, "fire schedule instruction received (no in-process job registry; recorded only)");
    Ok(Json(ActionResult::success(json!({ "value": true }))))
}

/// GET /jaxrs/base/sysresource/filePath/{filePath} —— 列出静态资源（带穿越防护）
pub async fn sysresource_list(
    Path(file_path): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let segments = match sanitize_resource_path(&file_path) {
        Some(s) => s,
        None => return Ok(Json(ActionResult::error("illegal filePath"))),
    };
    let mut dir = web_root();
    for seg in &segments {
        dir.push(seg);
    }
    if !dir.is_dir() {
        return Ok(Json(ActionResult::error("filePath not exist!")));
    }

    // 递归列举，限制深度 3 层、总条目 1000，防巨型目录拖垮响应
    let mut files: Vec<Value> = Vec::new();
    let mut folders: Vec<Value> = Vec::new();
    let root_display = segments.join("/");
    walk_resource_dir(&dir, &root_display, 0, &mut files, &mut folders);

    Ok(Json(ActionResult::success(json!({
        "files": files,
        "folders": folders,
    }))))
}

const RESOURCE_WALK_MAX_DEPTH: usize = 3;
const RESOURCE_WALK_MAX_ENTRIES: usize = 1000;

fn walk_resource_dir(
    dir: &std::path::Path,
    display_prefix: &str,
    depth: usize,
    files: &mut Vec<Value>,
    folders: &mut Vec<Value>,
) -> bool {
    if depth > RESOURCE_WALK_MAX_DEPTH
        || files.len() + folders.len() >= RESOURCE_WALK_MAX_ENTRIES
    {
        return false;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return true;
    };
    for entry in entries.flatten() {
        if files.len() + folders.len() >= RESOURCE_WALK_MAX_ENTRIES {
            return false;
        }
        let Ok(ft) = entry.file_type() else { continue };
        let name = entry.file_name().to_string_lossy().to_string();
        let path = if display_prefix.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", display_prefix, name)
        };
        if ft.is_dir() {
            folders.push(json!({ "name": name, "path": path }));
            if !walk_resource_dir(&entry.path(), &path, depth + 1, files, folders) {
                return false;
            }
        } else {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            let last_modified = entry
                .metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            files.push(json!({
                "name": name,
                "path": path,
                "size": size,
                "lastModified": last_modified,
            }));
        }
    }
    true
}

/// 构建基础模块路由
///
/// 委托给 `routes::build_router` 构建完整路由树。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Router`: Axum 路由实例
pub fn base_router(pool: Pool) -> Router {
    routes::build_router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::base_router(pool)
}
