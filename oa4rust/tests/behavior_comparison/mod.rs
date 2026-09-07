#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//! Rust vs Java 行为对比测试框架
//!
//! 该模块提供并行调用 Rust 和 Java 端点并对比响应的能力，
//! 用于确保 Rust 实现与 Java 后端功能等效。

pub mod allowlist;
pub mod comparator;
pub mod endpoints;
pub mod reporter;

#[allow(unused_imports)]
pub use allowlist::{AllowlistEntry, DiffAllowlist};
#[allow(unused_imports)]
pub use comparator::{ComparisonResult, ComparisonStatus, EndpointComparator, EndpointDef};
#[allow(unused_imports)]
pub use reporter::{ComparisonReport, ReportFormat};
