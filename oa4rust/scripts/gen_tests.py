"""
gen_tests.py — 为低覆盖率 crate 生成路由存在性测试。
"""
import json
import re
from pathlib import Path
from collections import defaultdict

BASE = Path("D:/WORKSPACE/fakeToys/oa4rust")
DATA = json.load(open(BASE / "target" / "endpoints" / "endpoints.json", encoding="utf-8"))

TARGET_CRATES = [
    "program_center",
    "organization_assemble_control",
    "general_assemble_control",
    "file_assemble_control",
    "attendance_assemble_control",
    "cms_assemble_control",
    "processplatform_service_processing",
    "processplatform_assemble_surface",
    "message_assemble_communicate",
    "meeting_assemble_control",
]

def get_router_name(crate_name: str) -> str:
    lib_path = BASE / "crates" / crate_name / "src" / "lib.rs"
    if lib_path.exists():
        try:
            content = lib_path.read_text(encoding="utf-8")
            m = re.search(r"pub fn (\w+.*?router\s*\()", content)
            if m:
                return m.group(1).split("(")[0].strip()
        except UnicodeDecodeError:
            pass
    routes_path = BASE / "crates" / crate_name / "src" / "routes.rs"
    if routes_path.exists():
        try:
            content = routes_path.read_text(encoding="utf-8")
            m = re.search(r"pub fn (\w+.*?router\s*\()", content)
            if m:
                return m.group(1).split("(")[0].strip()
        except UnicodeDecodeError:
            pass
    return "router"

def make_unique_name(base_name: str, used: set) -> str:
    if base_name not in used:
        used.add(base_name)
        return base_name
    idx = 2
    while f"{base_name}_{idx}" in used:
        idx += 1
    name = f"{base_name}_{idx}"
    used.add(name)
    return name

def generate_test_functions(crate_name: str):
    eps = [e for e in DATA if e["crate_name"] == crate_name]
    if not eps:
        return []

    router_name = get_router_name(crate_name)
    tests = []
    used_names = set()

    by_method = defaultdict(list)
    for ep in eps:
        by_method[ep["method"]].append(ep)

    for method in ["GET", "POST", "PUT", "DELETE"]:
        eps_method = by_method.get(method, [])
        if not eps_method:
            continue
        for ep in eps_method:
            path = ep["rust_path"]
            test_path = re.sub(r"\{[^}]+\}", "test-id", path)
            base_name = f"test_{method.lower()}_{path.replace('/', '_').replace('{', '').replace('}', '').strip('_')}"
            base_name = re.sub(r'_+', '_', base_name)[:50]
            test_name = make_unique_name(base_name, used_names)
            tests.append(f'''    #[tokio::test]
    async fn {test_name}() {{
        let pool = build_test_pool();
        let app = crate::{router_name}(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("{test_path}")
                    .method(Method::{method})
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }}
''')
    return tests

def add_tests_to_file(tests_file: Path, tests: list):
    """Add tests inside the mod tests block."""
    if not tests_file.exists():
        header = '''#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use tower::util::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }
'''
        body = "\n".join(tests)
        tests_file.write_text(header + body + "}\n", encoding="utf-8")
        return

    content = tests_file.read_text(encoding="utf-8")

    # Check if mod tests exists
    if "mod tests {" not in content:
        header = '''#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use tower::util::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }
'''
        body = "\n".join(tests)
        tests_file.write_text(content.rstrip() + "\n" + header + body + "}\n", encoding="utf-8")
        return

    # Ensure Method is imported
    if "Method" not in content:
        content = content.replace(
            "use axum::http::{Request, StatusCode};",
            "use axum::http::{Request, Method, StatusCode};"
        )
        if "Method" not in content:
            content = content.replace(
                "use axum::http::{Request",
                "use axum::http::{Request, Method"
            )

    # Ensure deadpool imports exist (only if not already present)
    if "use deadpool_postgres::{Manager, Pool};" not in content and "use deadpool_postgres" not in content:
        # Add after tower import
        if "use tower::util::ServiceExt;" in content:
            content = content.replace(
                "use tower::util::ServiceExt;",
                "use tower::util::ServiceExt;\n    use deadpool_postgres::{Manager, Pool};\n    use deadpool_postgres::tokio_postgres::{Config, NoTls};"
            )
        elif "use tower::ServiceExt;" in content:
            content = content.replace(
                "use tower::ServiceExt;",
                "use tower::util::ServiceExt;\n    use deadpool_postgres::{Manager, Pool};\n    use deadpool_postgres::tokio_postgres::{Config, NoTls};"
            )

    # Ensure build_test_pool exists (only if not already present)
    if "fn build_test_pool" not in content:
        pool_code = '''
    fn build_test_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }
'''
        # Insert before first test
        lines = content.split('\n')
        for i, line in enumerate(lines):
            if line.strip().startswith('#[test]') or line.strip().startswith('#[tokio::test]'):
                lines.insert(i, pool_code.strip())
                content = '\n'.join(lines)
                break

    # Find the mod tests block and insert before its closing brace
    lines = content.split('\n')
    mod_start_idx = -1
    for i, line in enumerate(lines):
        if 'mod tests {' in line:
            mod_start_idx = i
            break

    if mod_start_idx < 0:
        return

    # Find matching closing brace
    brace_count = 0
    mod_end_idx = -1
    for i in range(mod_start_idx, len(lines)):
        for ch in lines[i]:
            if ch == '{':
                brace_count += 1
            elif ch == '}':
                brace_count -= 1
                if brace_count == 0:
                    mod_end_idx = i
                    break
        if mod_end_idx >= 0:
            break

    if mod_end_idx < 0:
        return

    # Insert tests before the closing brace
    new_tests = "\n" + "\n".join(tests) + "\n"
    lines.insert(mod_end_idx, new_tests)
    content = "\n".join(lines)
    tests_file.write_text(content, encoding="utf-8")

if __name__ == "__main__":
    for crate_name in TARGET_CRATES:
        tests = generate_test_functions(crate_name)
        if not tests:
            continue
        tests_file = BASE / "crates" / crate_name / "src" / "tests.rs"
        add_tests_to_file(tests_file, tests)
        print(f"Added {len(tests)} tests to {crate_name}/src/tests.rs")
