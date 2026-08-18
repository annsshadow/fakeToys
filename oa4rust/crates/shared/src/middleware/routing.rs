use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::env;
use std::sync::OnceLock;

// ──────────────────────────────────────────────────────────────────────────────
// module_routing: U9 灰度迁移 Feature Flag
//
// 通过环境变量 MODULE_ROUTING 控制每个模块前缀路由到 Rust 还是 Java。
// 格式: MODULE_ROUTING=attendance:rust,calendar:java,control:rust
// 未设置或未明确声明的模块默认路由到 Rust（true）。
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ModuleRouting {
    java_prefixes: Vec<String>,
}

impl ModuleRouting {
    pub fn from_env() -> Self {
        let raw = env::var("MODULE_ROUTING").unwrap_or_default();
        let mut java_prefixes = Vec::new();
        for part in raw.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some((module, target)) = part.split_once(':') {
                let module = module.trim();
                let target = target.trim().to_lowercase();
                if target == "java" {
                    java_prefixes.push(module.to_string());
                }
            }
        }
        Self { java_prefixes }
    }

    /// 检查给定路径是否应路由到 Rust。若应路由到 Java 返回 false。
    pub fn is_rust(&self, path: &str) -> bool {
        let path_segs: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        if path_segs.len() >= 2 && path_segs[0] == "jaxrs" {
            let module = path_segs[1];
            !self.java_prefixes.iter().any(|p| p == module)
        } else {
            true
        }
    }
}

fn module_routing() -> &'static ModuleRouting {
    static INSTANCE: OnceLock<ModuleRouting> = OnceLock::new();
    INSTANCE.get_or_init(ModuleRouting::from_env)
}

/// 如果请求路径属于配置为 Java 的模块，返回 true。
pub fn should_route_to_java(path: &str) -> bool {
    !module_routing().is_rust(path)
}

// ──────────────────────────────────────────────────────────────────────────────
// behavior_comparison_middleware
//
// 行为对比测试中间件：当请求携带 X-Behavior-Comparison: true 头时，
// 记录请求路径、方法、响应状态码和响应体（前 4KB），用于 Rust vs Java
// 端点行为对比。仅用于测试环境，生产环境自动禁用。
// ──────────────────────────────────────────────────────────────────────────────
pub async fn behavior_comparison_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    let is_comparison = request
        .headers()
        .get("x-behavior-comparison")
        .map(|v| v == "true")
        .unwrap_or(false);

    if !is_comparison {
        return next.run(request).await;
    }

    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let query = request.uri().query().map(|q| format!("?{}", q)).unwrap_or_default();

    let response = next.run(request).await;

    let status = response.status();
    let headers = response.headers().clone();
    let body_bytes = axum::body::to_bytes(response.into_body(), 4 * 1024).await.unwrap_or_default();
    let body_str = String::from_utf8_lossy(&body_bytes);

    tracing::info!(
        method = %method,
        path = %path,
        query = %query,
        status = %status.as_u16(),
        body_preview = %body_str.chars().take(500).collect::<String>(),
        "behavior_comparison"
    );

    let new_body = axum::body::Body::from(body_bytes);
    let mut new_response = Response::new(new_body);
    *new_response.headers_mut() = headers;
    new_response
}
