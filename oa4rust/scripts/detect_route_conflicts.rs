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
            let crate_name = entry.path()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            extract_routes(&content, &crate_name, &mut routes);
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

fn extract_routes(content: &str, crate_name: &str, routes: &mut HashMap<(String, String), Vec<String>>) {
    let lines: Vec<&str> = content.lines().collect();

    for i in 0..lines.len() {
        let line = lines[i].trim();

        if line.contains(".route(") {
            // 提取路径和方法
            if let Some((path, method)) = parse_route_line(line, &lines, i) {
                routes
                    .entry((path, method))
                    .or_default()
                    .push(crate_name.to_string());
            }
        }
    }
}

fn parse_route_line(line: &str, all_lines: &[&str], line_idx: usize) -> Option<(String, String)> {
    // 提取路径：查找 .route("...") 或 .route(
    //     "..."
    // )
    let path = extract_path(line, all_lines, line_idx)?;

    // 提取方法：从当前行或前一行查找 .get( .post( 等
    let method = extract_method(line, all_lines, line_idx)?;

    Some((path, method))
}

fn extract_path(line: &str, all_lines: &[&str], line_idx: usize) -> Option<String> {
    // 尝试从当前行提取
    if let Some(start) = line.find(".route(\"") {
        let rest = &line[start + 8..];
        if let Some(end) = rest.find("\"") {
            return Some(rest[..end].to_string());
        }
    }

    // 尝试从当前行提取（单引号）
    if let Some(start) = line.find(".route('") {
        let rest = &line[start + 8..];
        if let Some(end) = rest.find('\'') {
            return Some(rest[..end].to_string());
        }
    }

    // 尝试从下一行提取（多行 .route("...") ）
    if line.ends_with(".route(") && line_idx + 1 < all_lines.len() {
        let next_line = all_lines[line_idx + 1].trim();
        if let Some(start) = next_line.find('"') {
            let rest = &next_line[start + 1..];
            if let Some(end) = rest.find('"') {
                return Some(rest[..end].to_string());
            }
        }
    }

    None
}

fn extract_method(line: &str, all_lines: &[&str], line_idx: usize) -> Option<String> {
    // 从当前行提取
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
        // 尝试从前一行提取（多行 .route 调用）
        if line_idx > 0 {
            let prev_line = all_lines[line_idx - 1].trim();
            if prev_line.contains(".get(") {
                "GET"
            } else if prev_line.contains(".post(") {
                "POST"
            } else if prev_line.contains(".put(") {
                "PUT"
            } else if prev_line.contains(".delete(") {
                "DELETE"
            } else if prev_line.contains(".patch(") {
                "PATCH"
            } else if prev_line.contains(".head(") {
                "HEAD"
            } else if prev_line.contains(".options(") {
                "OPTIONS"
            } else {
                "UNKNOWN"
            }
        } else {
            "UNKNOWN"
        }
    };

    Some(method.to_string())
}
