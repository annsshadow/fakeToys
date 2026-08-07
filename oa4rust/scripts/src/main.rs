use std::collections::HashMap;
use std::path::Path;

fn main() {
    let crates_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("crates");

    let crate_dirs: Vec<_> = std::fs::read_dir(&crates_dir)
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
    for ((path, method), crate_list) in routes.iter() {
        let unique_crates: std::collections::HashSet<_> = crate_list.iter().collect();
        if unique_crates.len() > 1 {
            println!(
                "CONFLICT: {} {} registered in: {:?}",
                method, path, unique_crates
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
    let path = extract_path(line, all_lines, line_idx)?;
    let method = extract_method(line, all_lines, line_idx)?;
    Some((path, method))
}

fn extract_path(line: &str, all_lines: &[&str], line_idx: usize) -> Option<String> {
    if let Some(start) = line.find(".route(\"") {
        let rest = &line[start + 8..];
        if let Some(end) = rest.find("\"") {
            return Some(rest[..end].to_string());
        }
    }

    if let Some(start) = line.find(".route('") {
        let rest = &line[start + 8..];
        if let Some(end) = rest.find('\'') {
            return Some(rest[..end].to_string());
        }
    }

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
