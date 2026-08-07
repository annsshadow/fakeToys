//! Rust vs Java 行为对比测试框架
//!
//! 该模块提供并行调用 Rust 和 Java 端点并对比响应的能力，
//! 用于确保 Rust 实现与 Java 后端功能等效。

pub mod comparator;
pub mod reporter;

pub use comparator::{ComparisonResult, EndpointComparator};
pub use reporter::{ComparisonReport, ReportFormat};
