// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// middleware submodule — 共享中间件与权限基础设施
//
// 模块结构：
//   constants.rs — 认证豁免路径、限流配置等常量
//   security.rs  — CORS、SecurityState、安全头、trace 中间件
//   token.rs     — Token 提取与认证中间件
//   rbac.rs      — RBAC 模型（PermissionLevel, PermissionRegistry）、授权中间件

pub mod constants;
pub mod contract_alias;
pub mod rbac;
pub mod security;
pub mod token;

// Re-export public API for backward compatibility
pub use constants::*;
pub use rbac::*;
pub use security::*;
pub use token::*;
