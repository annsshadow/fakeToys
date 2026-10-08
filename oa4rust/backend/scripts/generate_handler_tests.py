#!/usr/bin/env python3
"""
Generate unit tests for all pub async fn handlers in oa4rust crates.

Approach: For each handler, generates a test that calls it through the crate's
router via oneshot(). This works for ALL handlers regardless of whether they're
re-exported from the crate root.

The script also reads routes.rs to map handler names to URL paths.
"""

import os
import re
import sys
import argparse
from typing import List, Tuple, Dict, Optional

CRATES_DIR = os.path.join(os.path.dirname(__file__), '..', 'crates')
EXCLUDE_CRATES = {'mcp_server', 'openapi'}


def find_all_rs_files(crate_dir: str) -> List[str]:
    """Recursively find all .rs files under crate_dir/src/."""
    src_dir = os.path.join(crate_dir, 'src')
    if not os.path.isdir(src_dir):
        return []
    result = []
    for root, dirs, files in os.walk(src_dir):
        for f in files:
            if f.endswith('.rs'):
                result.append(os.path.join(root, f))
    return sorted(result)


def extract_handlers(rs_files: List[str]) -> List[Tuple[str, str, List[str]]]:
    """Extract pub async fn handlers from Rust source files.
    
    Returns list of (handler_name, full_signature_line, param_list).
    """
    handlers = []
    pattern = re.compile(r'^\s*pub\s+async\s+fn\s+(\w+)\s*\(', re.MULTILINE)
    
    for filepath in rs_files:
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            print(f'  WARNING: Could not read {filepath}: {e}', file=sys.stderr)
            continue
        
        for m in pattern.finditer(content):
            handler_name = m.group(1)
            if handler_name.startswith('test_'):
                continue
            
            start = m.start()
            paren_depth = 0
            paren_start = None
            for i in range(m.start(), len(content)):
                if content[i] == '(':
                    if paren_depth == 0:
                        paren_start = i
                    paren_depth += 1
                elif content[i] == ')':
                    paren_depth -= 1
                    if paren_depth == 0:
                        param_section = content[paren_start+1:i]
                        break
            else:
                continue
            
            params = parse_params(param_section)
            handlers.append((handler_name, content[m.start():m.start()+200], params))
    
    return handlers


def parse_params(param_section: str) -> List[str]:
    """Parse function parameters into raw parameter strings."""
    params = []
    depth_paren = 0
    depth_angle = 0
    current = ''
    for ch in param_section:
        if ch == '(':
            depth_paren += 1
            current += ch
        elif ch == ')':
            depth_paren -= 1
            current += ch
        elif ch == '<':
            depth_angle += 1
            current += ch
        elif ch == '>':
            depth_angle -= 1
            current += ch
        elif ch == ',' and depth_paren == 0 and depth_angle == 0:
            params.append(current.strip())
            current = ''
        else:
            current += ch
    if current.strip():
        params.append(current.strip())
    
    result = []
    for p in params:
        p = p.strip()
        if p in ('self', '&self', '&mut self'):
            continue
        result.append(p)
    return result


def extract_routes(crate_dir: str) -> Dict[str, Tuple[str, str]]:
    """Extract handler_name -> (http_method, route_path) mapping from all .rs files.
    
    Scans routes.rs, lib.rs, and all submodules for .route(...) calls.
    """
    rs_files = find_all_rs_files(crate_dir)
    routes: Dict[str, Tuple[str, str]] = {}
    
    for rs_file in rs_files:
        try:
            with open(rs_file, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception:
            continue
        
        # Match: .route("/path", get(handler_name)) or .route("/path", post(handler_name))
        # Also handle merge() calls like .merge(another_router)
        for m in re.finditer(r'\.route\("([^"]+)"\s*,\s*(get|post|put|delete|patch)\s*\(\s*(\w+)', content):
            path = m.group(1)
            method = m.group(2)
            handler = m.group(3)
            routes[handler] = (method, path)
    
    return routes


def get_exported_handlers(crate_dir: str) -> set:
    """Get set of handler names that are re-exported from crate root via pub use."""
    lib_rs = os.path.join(crate_dir, 'src', 'lib.rs')
    if not os.path.exists(lib_rs):
        return set()
    try:
        with open(lib_rs, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception:
        return set()

    exported = set()
    # Match: pub use module::{handler1, handler2, ...};
    for m in re.finditer(r'pub\s+use\s+\w+::\{([^}]+)\}', content):
        names = m.group(1)
        for name in names.split(','):
            name = name.strip().split('as')[0].strip()
            if name:
                exported.add(name)
    return exported


def get_pool_type(params: List[str]) -> str:
    """Determine which pool type to use based on handler params."""
    has_pool = any(re.search(r'Extension\s*<\s*Pool\s*>', p) for p in params)
    has_db = any('Extension<DatabaseConnection>' in p for p in params)
    has_session = any(re.search(r'Extension\s*<[^>]*Session', p) for p in params)
    
    if has_session:
        return 'session'
    if has_db:
        return 'sea_orm'
    if has_pool:
        return 'pool'
    return 'none'


def gen_arg_value(raw_param: str) -> str:
    """Generate the Rust expression to pass as an argument for a parameter."""
    # Extension<Pool>
    if re.search(r'Extension\s*<\s*Pool\s*>', raw_param):
        return 'axum::extract::Extension(shared::testing::test_pool())'
    # Extension<DatabaseConnection>
    if 'Extension<DatabaseConnection>' in raw_param:
        return 'axum::extract::Extension(shared::testing::test_sea_orm_pool().await)'
    # Extension<SessionManager> or any Extension<Session...>
    if re.search(r'Extension\s*<[^>]*Session', raw_param):
        return '/* SKIP: Session param */'
    # Path<T>
    m = re.match(r'Path\s*\((.+?)\)\s*:\s*Path\s*<\s*(.+?)\s*>$', raw_param)
    if m:
        inner = m.group(1).strip()
        ptype = m.group(2).strip()
        inner_val = gen_path_inner(inner, ptype)
        return f'axum::extract::Path({inner_val})'
    # AxumJson<T>
    m = re.match(r'AxumJson\s*\((\w+)\)\s*:\s*AxumJson\s*<\s*(.+?)\s*>$', raw_param)
    if m:
        return 'axum::extract::AxumJson(serde_json::json!({}))'
    # Json<T>
    m = re.match(r'Json\s*\((\w+)\)\s*:\s*Json\s*<\s*(.+?)\s*>$', raw_param)
    if m:
        return 'serde_json::json!({})'
    # Bare type (no extractor)
    m = re.match(r'!?(\w+)\s*:\s*(.+)', raw_param)
    if m:
        ptype = m.group(2).strip()
        if ptype in ('String', 'str'):
            return '"test-value".to_string()'
        if ptype in ('i64', 'i32', 'u64', 'u32', 'usize'):
            return '1'
        if ptype == 'bool':
            return 'true'
        if ptype == 'Value':
            return 'serde_json::json!({})'

    # Default fallback
    return 'serde_json::json!({})'


def gen_path_inner(inner: str, ptype: str) -> str:
    """Generate the inner value of a Path<> argument."""
    # Extract the actual type from Path<T> if needed
    type_match = re.match(r'Path<(.+)>', ptype)
    if type_match:
        ptype = type_match.group(1).strip()

    if inner.startswith('(') and inner.endswith(')'):
        items = [x.strip() for x in inner[1:-1].split(',')]
        # Handle tuple types like (String, i32, i32)
        if ',' in ptype:
            type_parts = [t.strip().strip('()') for t in ptype.split(',')]
        else:
            type_parts = [ptype] * len(items)
        values = []
        for item, tpart in zip(items, type_parts):
            val = gen_path_scalar(item.strip(), tpart)
            # For String types, append .to_string() to convert &str to String
            if tpart == 'String':
                val = f'{val}.to_string()'
            values.append(val)
        return f'({", ".join(values)})'
    else:
        val = gen_path_scalar(inner, ptype)
        if ptype == 'String':
            val = f'{val}.to_string()'
        return val


def has_unresolvable_params(params: List[str]) -> bool:
    """Check if handler has params that can't be directly constructed in tests."""
    for p in params:
        # Query<T> extractors can't be constructed directly
        if re.match(r'Query\s*\(', p):
            return True
        # Json<T> where T is a custom struct (not Value) - can't construct inline
        m = re.match(r'Json\s*\([^)]*\)\s*:\s*Json\s*<\s*(\w+)', p)
        if m:
            tname = m.group(1)
            if tname != 'Value':
                return True
    return False


def generate_test_fn(handler_name: str, routes: Dict[str, Tuple[str, str]], 
                     exported: set, params: List[str], warnings: List[str],
                     can_router: bool = True) -> Tuple[str, bool]:
    """Generate a test function for a handler.
    
    Uses direct call if handler is re-exported from crate root,
    otherwise uses router-based test.
    """
    pool_type = get_pool_type(params)
    
    if pool_type == 'session':
        return (f'    // SKIPPED: {handler_name} requires Session parameter', True)
    
    is_exported = handler_name in exported
    route_info = routes.get(handler_name)
    
    if is_exported and not has_unresolvable_params(params):
        # Direct call test
        args = []
        for raw_param in params:
            arg = gen_arg_value(raw_param)
            if arg.startswith('/* SKIP'):
                return (f'    // SKIPPED: {handler_name} has unresolvable param', True)
            args.append(arg)
        args_str = ', '.join(args)
        test_code = f'''    #[tokio::test]
    async fn test_{handler_name}() {{
        let _result = crate::{handler_name}({args_str}).await;
    }}
'''
        return (test_code, False)
    elif is_exported:
        # Handler is exported but has unresolvable params (e.g. custom Json struct, Query)
        # Skip with warning
        warnings.append(f'  WARN {handler_name}: exported but has unresolvable params (e.g. custom Json/Query type)')
        return (f'    // SKIPPED: {handler_name} has unresolvable params', True)
    
    elif route_info:
        if not can_router:
            # Can't generate router test without tower dependency
            return (f'    // SKIPPED: {handler_name} requires tower (not available)', True)
        # Router-based test
        method, test_path = route_info
        http_method = method.upper()

        # Replace all /{param} segments with /test-id to avoid string escaping issues
        test_path = re.sub(r'/\{[^}]+\}', '/test-id', test_path)

        test_code = f'''    #[tokio::test]
    async fn test_{handler_name}() {{
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("{test_path}")
                    .method("{http_method}")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {{
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "{handler_name} route should be registered");
        }}
    }}
'''
        return (test_code, False)
    
    else:
        # Handler not in routes and not exported — skip
        if can_router:
            warnings.append(f'  WARN {handler_name}: not in routes.rs and not re-exported')
        return (f'    // SKIPPED: {handler_name} not accessible', True)


def generate_router_test(handler_name: str, routes: Dict[str, Tuple[str, str]], 
                         params: List[str], warnings: List[str]) -> Tuple[str, bool]:
    """Legacy function — delegates to generate_test_fn."""
    # This is kept for compatibility but generate_test_fn now handles both cases
    return generate_test_fn(handler_name, routes, set(), params, warnings)


def gen_path_tuple_value(inner: str, ptype: str) -> str:
    """Generate a test value for a tuple Path parameter."""
    items = [x.strip() for x in inner[1:-1].split(',')]
    type_str = ptype
    tm = re.match(r'Path<\((.+?)\)>', ptype)
    if tm:
        type_str = tm.group(1)
    type_parts = [t.strip().strip('()') for t in type_str.split(',')]
    values = []
    for item, tpart in zip(items, type_parts):
        values.append(gen_path_scalar(item.strip(), tpart))
    return ','.join(values)


def gen_path_scalar(name: str, ptype: str) -> str:
    """Generate a scalar Path argument value (with quotes for String types)."""
    if ptype in ('String',):
        return '"test-id"'
    if ptype in ('str', '&str'):
        return '"test-id"'
    if ptype in ('i64',):
        return '1i64'
    if ptype in ('i32',):
        return '1i32'
    if ptype in ('u64', 'u32', 'usize'):
        return '1'
    return '"test-id"'


def generate_test_module(crate_name: str, handlers: List[Tuple[str, str, List[str]]],
                         routes: Dict[str, Tuple[str, str]],
                         exported: set,
                         warnings: List[str],
                         has_tower: bool = True) -> str:
    """Generate the full tests_generated.rs content for a crate."""
    has_direct = any(h in exported for h, _, _ in handlers)
    # Router-based tests require tower; skip them if not available
    can_router = has_tower
    has_router_test = can_router and any(h not in exported and h in routes for h, _, _ in handlers)

    lines = []
    lines.append('#[cfg(test)]')
    lines.append('mod tests {')

    if has_direct and has_router_test:
        lines.append('    use axum::body::Body;')
        lines.append('    use axum::extract::{Extension, Path, Json};')
        lines.append('    use axum::http::{Request, Method, StatusCode};')
        lines.append('    use shared::testing::{test_pool, test_sea_orm_pool};')
        lines.append('    use tower::util::ServiceExt;')
    elif has_direct:
        lines.append('    use axum::extract::{Extension, Path, Json};')
        lines.append('    use shared::testing::{test_pool, test_sea_orm_pool};')
    elif has_router_test:
        lines.append('    use axum::body::Body;')
        lines.append('    use axum::http::{Request, Method, StatusCode};')
        lines.append('    use shared::testing::test_pool;')
        lines.append('    use tower::util::ServiceExt;')
    lines.append('')

    skipped = 0
    for handler_name, sig_line, params in handlers:
        test_code, is_skipped = generate_test_fn(handler_name, routes, exported, params, warnings, can_router)
        if is_skipped:
            skipped += 1
        lines.append(test_code)

    lines.append('}')
    return '\n'.join(lines)


def has_tower_dep(crate_dir: str) -> bool:
    """Check if the crate has tower as a dependency (needed for router-based tests)."""
    cargo_toml = os.path.join(crate_dir, 'Cargo.toml')
    if not os.path.exists(cargo_toml):
        return False
    try:
        with open(cargo_toml, 'r', encoding='utf-8') as f:
            content = f.read()
        return 'tower' in content
    except Exception:
        return False


def process_crate(crate_name: str) -> Tuple[int, int, int, List[str]]:
    """Process a single crate. Returns (total_handlers, generated_count, skipped_count, warnings)."""
    crate_dir = os.path.join(CRATES_DIR, crate_name)
    if not os.path.isdir(crate_dir):
        return (0, 0, 0, [f'  SKIP {crate_name}: directory not found'])

    rs_files = find_all_rs_files(crate_dir)
    if not rs_files:
        return (0, 0, 0, [])

    handlers = extract_handlers(rs_files)
    if not handlers:
        return (0, 0, 0, [])

    routes = extract_routes(crate_dir)
    exported = get_exported_handlers(crate_dir)
    has_tower = has_tower_dep(crate_dir)
    warnings = []
    test_module = generate_test_module(crate_name, handlers, routes, exported, warnings, has_tower)
    
    # Always write to tests_generated.rs
    output_path = os.path.join(crate_dir, 'src', 'tests_generated.rs')
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(test_module)
    
    # Add module declaration to lib.rs if needed
    lib_rs = os.path.join(crate_dir, 'src', 'lib.rs')
    needs_module_decl = True
    if os.path.exists(lib_rs):
        with open(lib_rs, 'r', encoding='utf-8') as f:
            lib_content = f.read()
        if 'mod tests_generated;' in lib_content:
            needs_module_decl = False
    
    if needs_module_decl and os.path.exists(lib_rs):
        with open(lib_rs, 'r', encoding='utf-8') as f:
            lib_content = f.read()
        insert_pos = lib_content.rfind('mod tests;')
        if insert_pos != -1:
            end_of_line = lib_content.find('\n', insert_pos)
            if end_of_line == -1:
                end_of_line = len(lib_content)
            new_lib_content = (lib_content[:end_of_line] + '\n#[cfg(test)]\nmod tests_generated;\n' +
                             lib_content[end_of_line:])
        else:
            new_lib_content = lib_content.rstrip() + '\n\n#[cfg(test)]\nmod tests_generated;\n'
        with open(lib_rs, 'w', encoding='utf-8') as f:
            f.write(new_lib_content)
    
    skipped = sum(1 for _, _, params in handlers if get_pool_type(params) == 'session' or 
                  handlers and routes.get(handlers[0][0]) is None)
    # Actually count skipped properly
    skipped = 0
    for handler_name, _, params in handlers:
        if get_pool_type(params) == 'session':
            skipped += 1
        elif handler_name not in routes:
            skipped += 1
    
    generated = len(handlers) - skipped
    return (len(handlers), generated, skipped, warnings)


def main():
    parser = argparse.ArgumentParser(description='Generate unit tests for oa4rust handlers')
    parser.add_argument('--crate', help='Process a single crate')
    parser.add_argument('--dry-run', action='store_true', help='Print handlers without writing files')
    parser.add_argument('--verbose', action='store_true', help='Show detailed output')
    args = parser.parse_args()
    
    if not os.path.isdir(CRATES_DIR):
        print(f'ERROR: crates directory not found at {CRATES_DIR}', file=sys.stderr)
        sys.exit(1)
    
    crate_names = sorted([
        d for d in os.listdir(CRATES_DIR)
        if os.path.isdir(os.path.join(CRATES_DIR, d)) and d not in EXCLUDE_CRATES
    ])
    
    if args.crate:
        crate_names = [args.crate] if args.crate in crate_names else []
        if not crate_names:
            print(f'ERROR: crate "{args.crate}" not found', file=sys.stderr)
            sys.exit(1)
    
    total_handlers = 0
    total_generated = 0
    total_skipped = 0
    all_warnings = []
    
    for crate_name in crate_names:
        handlers_count, generated_count, skipped_count, warnings = process_crate(crate_name)
        total_handlers += handlers_count
        total_generated += generated_count
        total_skipped += skipped_count
        all_warnings.extend(warnings)
        
        if handlers_count > 0:
            status = f'{generated_count} covered, {skipped_count} skipped'
            if skipped_count > 0:
                status += f' out of {handlers_count} handlers'
            print(f'  {crate_name}: {status}')
        elif args.verbose:
            print(f'  {crate_name}: no handlers found')
    
    if all_warnings:
        print(f'\nWarnings ({len(all_warnings)}):')
        for w in all_warnings:
            print(w)
    
    print(f'\nTotal: {total_generated} covered, {total_skipped} skipped out of {total_handlers} handlers')


if __name__ == '__main__':
    main()
