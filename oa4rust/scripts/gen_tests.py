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

def safe_test_name(path: str, method: str, idx: int) -> str:
    """生成安全的测试函数名。"""
    clean = path.replace("/", "_").replace("{", "").replace("}", "").strip("_")
    clean = re.sub(r'_+', '_', clean)
    clean = clean[:40]
    base = f"test_{method.lower()}_{clean}"
    return base

def analyze_test_file(content: str) -> dict:
    """分析测试文件，返回配置信息。"""
    info = {
        "pool_helper": None,
        "pool_helper_is_async": False,
        "has_method_import": False,
        "has_service_ext": False,
    }
    
    # 检测 pool helper - 查找函数定义
    m = re.search(r'async fn (\w+_pool)\s*\(', content)
    if not m:
        m = re.search(r'fn (\w+_pool)\s*\(', content)
    if m:
        info["pool_helper"] = m.group(1)
        # 检查是否是 async
        info["pool_helper_is_async"] = "async fn" in content[max(0, m.start()-20):m.start()+30]
    
    # 检测 Method 导入
    if re.search(r'use axum::http::\{[^}]*Method', content):
        info["has_method_import"] = True
    
    # 检测 ServiceExt 导入
    if 'use tower::util::ServiceExt' in content or 'use tower::ServiceExt' in content:
        info["has_service_ext"] = True
    
    return info

def generate_test_function(crate_name: str, pool_helper: str, is_async: bool, ep: dict, idx: int) -> str:
    """生成单个测试函数。"""
    router_name = get_router_name(crate_name)
    path = ep["rust_path"]
    method = ep["method"]
    test_path = re.sub(r"\{[^}]+\}", "test-id", path)
    test_name = safe_test_name(path, method, idx)
    
    # 构建 pool 获取代码 - 注意：async 函数调用需要 .await
    if is_async:
        pool_code = f"let pool = {pool_helper}().await;"
    else:
        pool_code = f"let pool = {pool_helper}();"
    
    return f'''    #[tokio::test]
    async fn {test_name}() {{
        {pool_code}
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
'''

def add_tests_to_file(tests_file: Path, tests: list, info: dict):
    """将生成的测试添加到 tests.rs 文件中。"""
    pool_helper = info["pool_helper"] or "build_test_pool"
    is_async = info["pool_helper_is_async"]
    
    if not tests_file.exists():
        # 新建文件
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

    # 确保 Method 已导入
    if not info["has_method_import"]:
        if "use axum::http::{Request, StatusCode};" in content:
            content = content.replace(
                "use axum::http::{Request, StatusCode};",
                "use axum::http::{Request, Method, StatusCode};"
            )
        elif re.search(r'use axum::http::\{Request, StatusCode\}', content):
            content = re.sub(
                r'use axum::http::\{Request, StatusCode\}',
                'use axum::http::{Request, Method, StatusCode}',
                content
            )
        elif "use axum::http::{Request" in content:
            content = content.replace(
                "use axum::http::{Request",
                "use axum::http::{Request, Method"
            )

    # 确保 tower import 正确
    if not info["has_service_ext"]:
        if "use tower::ServiceExt;" in content:
            content = content.replace(
                "use tower::ServiceExt;",
                "use tower::util::ServiceExt;"
            )
        elif "use tower::util::ServiceExt;" not in content:
            # 添加 tower import
            if "use shared::" in content:
                content = content.replace(
                    "use shared::",
                    "use tower::util::ServiceExt;\n    use shared::"
                )
            elif "use deadpool_postgres::" in content:
                content = content.replace(
                    "use deadpool_postgres::",
                    "use tower::util::ServiceExt;\n    use deadpool_postgres::"
                )

    # 添加缺失的 pool helper（仅当不存在时）
    if pool_helper not in content:
        # 确定是要添加 sync 还是 async 版本
        if is_async:
            pool_code = f'''
    async fn {pool_helper}() -> deadpool_postgres::Pool {{
        deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ))
        .build()
        .unwrap()
    }}
'''
        else:
            pool_code = f'''
    fn {pool_helper}() -> deadpool_postgres::Pool {{
        deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ))
        .build()
        .unwrap()
    }}
'''
        # 插入到第一个 test 属性之前
        lines = content.split('\n')
        for i, line in enumerate(lines):
            if line.strip().startswith('#[test]') or line.strip().startswith('#[tokio::test]'):
                lines.insert(i, pool_code.strip())
                content = '\n'.join(lines)
                break

    # 收集已存在的测试函数名
    existing_names = set(re.findall(r'async fn (\w+)\(\)', content))
    
    # 过滤掉重复的测试名
    unique_tests = []
    for test in tests:
        m = re.search(r'async fn (\w+)\(\)', test)
        if m:
            name = m.group(1)
            if name not in existing_names:
                unique_tests.append(test)
                existing_names.add(name)
    
    if not unique_tests:
        return

    # 找到 mod tests 块的结束位置并插入测试
    lines = content.split('\n')
    mod_start_idx = -1
    for i, line in enumerate(lines):
        if 'mod tests {' in line:
            mod_start_idx = i
            break

    if mod_start_idx < 0:
        # 没有 mod tests 块，创建一个新的
        header = '''#[cfg(test)]
mod tests {
'''
        body = "\n".join(unique_tests)
        content = content.rstrip() + "\n" + header + body + "}\n"
    else:
        # 找到匹配的结束大括号
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

        if mod_end_idx >= 0:
            new_tests = "\n" + "\n".join(unique_tests) + "\n"
            lines.insert(mod_end_idx, new_tests)
            content = "\n".join(lines)

    tests_file.write_text(content, encoding="utf-8")

if __name__ == "__main__":
    for crate_name in TARGET_CRATES:
        tests_file = BASE / "crates" / crate_name / "src" / "tests.rs"
        
        # 分析现有测试文件
        if tests_file.exists():
            content = tests_file.read_text(encoding="utf-8")
            info = analyze_test_file(content)
        else:
            info = {"pool_helper": None, "pool_helper_is_async": False, 
                    "has_method_import": False, "has_service_ext": False}
        
        # 生成测试
        eps = [e for e in DATA if e["crate_name"] == crate_name]
        if not eps:
            continue
        
        tests = []
        for idx, ep in enumerate(eps, 1):
            test_code = generate_test_function(crate_name, info["pool_helper"] or "build_test_pool", info["pool_helper_is_async"], ep, idx)
            tests.append(test_code)
        
        if tests:
            add_tests_to_file(tests_file, tests, info)
            print(f"Added {len(tests)} tests to {crate_name}/src/tests.rs")
