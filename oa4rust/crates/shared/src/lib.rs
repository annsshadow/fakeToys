// ──────────────────────────────────────────────────────────────────────────────
// shared — OA4Rust 项目共享库
//
// 本 crate 为 OA4Rust 的各服务二进制 crate 提供：
//   - 统一的 HTTP 错误类型与响应格式（error.rs、response.rs）
//   - 跨服务的中间件：请求追踪日志、全局错误处理（middleware.rs）
//   - 数据库连接池初始化（db.rs）
//   - 路由组装入口（router.rs）
//
// 各二进制 crate（如 oa4rust）通过 `shared::router()` 获取带中间件的 Router，
// 并在启动时调用 `shared::db::create_pool()` 获取数据库连接池。
// ──────────────────────────────────────────────────────────────────────────────

pub mod db;
pub mod error;
pub mod middleware;
pub mod response;
pub mod router;

#[cfg(test)]
mod tests;
