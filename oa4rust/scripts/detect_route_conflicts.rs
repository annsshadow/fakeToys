#!/usr/bin/env rust-script
//! 检测 oa4rust 中重复注册的路由（同 path + method）。
//!
//! 在 CI 中运行，确保新增 crate 不会引入路由冲突。
//! 用法：cargo run --bin detect_route_conflicts

use std::collections::HashMap;
use std::path::Path;

fn main() {
    let crate_dirs: Vec<_> = std::fs::read_dir("crates")
        .expect("failed to read crates directory")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("src").join("lib.rs").exists())
        .collect();

    let mut routes: HashMap<(String, String), Vec<String>> = HashMap::new();

    for entry in crate_dirs {
        let lib_path = entry.path().join("src").join("lib.rs");
        if let Ok(content) = std::fs::read_to_string(&lib_path) {
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with(".route(") || line.contains(".route(") {
                    // 提取路由路径和方法
                    extract_route(&line, &entry.path(), &mut routes);
                }
            }
        }
    }

    let mut conflicts = 0;
    for ((path, method), crates) in routes.iter() {
        if crates.len() > 1 {
            println!(
                "CONFLICT: {} {} registered in: {:?}",
                method, path, crates
            );
            conflicts += 1;
        }
    }

    if conflicts > 0 {
        eprintln!("\nFound {} route conflicts", conflicts);
        std::process::exit(1);
    } else {
        println!("No route conflicts found");
    }
}

fn extract_route(line: &str, crate_path: &Path, routes: &mut HashMap<(String, String), Vec<String>>) {
    // 简单提取：查找 .route("...") 模式
    if let Some(start) = line.find(".route(\"") {
        let rest = &line[start + 8..];
        if let Some(end) = rest.find("\"") {
            let path = rest[..end].to_string();
            let crate_name = crate_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            // 从上下文推断 HTTP 方法
            let method = if line.contains(".get(") {
                "GET"
            } else if line.contains(".post(") {
                "POST"
            } else if line.contains(".put(") {
                "PUT"
            } else if line.contains(".delete(") {
                "DELETE"
            } else if line.contains(".patch(") {
                "PATCH"
            } else if line.contains(".head(") {
                "HEAD"
            } else if line.contains(".options(") {
                "OPTIONS"
            } else {
                "UNKNOWN"
            };

            routes
                .entry((path, method.to_string()))
                .or_default()
                .push(crate_name);
        }
    }
}
