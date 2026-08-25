// middleware submodule — 共享中间件与权限基础设施
//
// 模块结构：
//   constants.rs — 认证豁免路径、限流配置等常量
//   security.rs  — CORS、SecurityState、安全头、trace 中间件
//   token.rs     — Token 提取与认证中间件
//   rbac.rs      — RBAC 模型（PermissionLevel, PermissionRegistry）、授权中间件
//   routing.rs   — 模块路由（Java/Rust）、行为对比中间件
//   rate_limit_distributed.rs — Redis 分布式限流（plan002 U7c，env 门控）

pub mod constants;
pub mod security;
pub mod token;
pub mod rbac;
pub mod routing;
pub mod rate_limit_distributed;

// Re-export public API for backward compatibility
pub use constants::*;
pub use security::*;
pub use token::*;
pub use rbac::*;
pub use routing::*;
pub use rate_limit_distributed::*;
