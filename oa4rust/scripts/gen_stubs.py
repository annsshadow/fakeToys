#!/usr/bin/env python3
"""
Batch generate stub routes for all missing Java @Path endpoints.
"""

import os
import re
import argparse

java_dir = os.path.abspath('../oa/o2server')
rust_dir = os.path.abspath('crates')

# Module mapping: java_module -> (rust_crate, route_prefix)
module_map = {
    'x_organization_assemble_authentication': ('organization_assemble_authentication', '/jaxrs/authentication'),
    'x_organization_assemble_control': ('organization_assemble_control', '/jaxrs/organization/assemble/control'),
    'x_organization_assemble_express': ('organization_assemble_express', '/jaxrs/organization/assemble/express'),
    'x_organization_core_entity': ('organization_core_entity', '/jaxrs/organization/core/entity'),
    'x_organization_core_express': ('organization_core_express', '/jaxrs/organization/core/express'),
    'x_attendance_assemble_control': ('attendance_assemble_control', '/jaxrs/attendance/assemble/control'),
    'x_attendance_core_entity': ('attendance_core_entity', '/jaxrs/attendance/core/entity'),
    'x_processplatform_assemble_bam': ('processplatform_assemble_bam', '/jaxrs/processplatform/assemble/bam'),
    'x_processplatform_assemble_designer': ('processplatform_assemble_designer', '/jaxrs/processplatform/assemble/designer'),
    'x_processplatform_assemble_surface': ('processplatform_assemble_surface', '/jaxrs/processplatform/assemble/surface'),
    'x_processplatform_core_entity': ('processplatform_core_entity', '/jaxrs/processplatform/core/entity'),
    'x_processplatform_core_express': ('processplatform_core_express', '/jaxrs/processplatform/core/express'),
    'x_processplatform_service_processing': ('processplatform_service_processing', '/jaxrs/processplatform/service/processing'),
    'x_process_designer': ('process_designer', '/jaxrs/process/designer'),
    'x_process_express': ('process_express', '/jaxrs/process/express'),
    'x_process_surface': ('process_surface', '/jaxrs/process/surface'),
    'x_process_bam': ('process_bam', '/jaxrs/process/bam'),
    'x_portal_assemble_designer': ('portal_assemble_designer', '/jaxrs/portal/assemble/designer'),
    'x_portal_assemble_surface': ('portal_assemble_surface', '/jaxrs/portal/assemble/surface'),
    'x_portal_core_entity': ('portal_core_entity', '/jaxrs/portal/core/entity'),
    'x_file_assemble_control': ('file_assemble_control', '/jaxrs/file/assemble/control'),
    'x_file_core_entity': ('file_core_entity', '/jaxrs/file/core/entity'),
    'x_query_assemble_designer': ('query_assemble_designer', '/jaxrs/query/assemble/designer'),
    'x_query_assemble_surface': ('query_assemble_surface', '/jaxrs/query/assemble/surface'),
    'x_query_core_entity': ('query_core_entity', '/jaxrs/query/core/entity'),
    'x_query_core_express': ('query_core_express', '/jaxrs/query/core/express'),
    'x_query_service_processing': ('query_service_processing', '/jaxrs/query/service/processing'),
    'x_cms_assemble_control': ('cms_assemble_control', '/jaxrs/cms/assemble/control'),
    'x_cms_core_entity': ('cms_core_entity', '/jaxrs/cms/core/entity'),
    'x_cms_core_express': ('cms_core_express', '/jaxrs/cms/core/express'),
    'x_cms_express': ('cms_express', '/jaxrs/cms/express'),
    'x_bbs_assemble_control': ('bbs_assemble_control', '/jaxrs/bbs/assemble/control'),
    'x_bbs_core_entity': ('bbs_core_entity', '/jaxrs/bbs/core/entity'),
    'x_mind_assemble_control': ('mind_assemble_control', '/jaxrs/mind/assemble/control'),
    'x_mind_core_entity': ('mind_core_entity', '/jaxrs/mind/core/entity'),
    'x_correlation_core_entity': ('correlation_core_entity', '/jaxrs/correlation/core/entity'),
    'x_correlation_core_express': ('correlation_core_express', '/jaxrs/correlation/core/express'),
    'x_correlation_service_processing': ('correlation_service_processing', '/jaxrs/correlation/service/processing'),
    'x_general_assemble_control': ('general_assemble_control', '/jaxrs/general/assemble/control'),
    'x_general_core_entity': ('general_core_entity', '/jaxrs/general/core/entity'),
    'x_hotpic_assemble_control': ('hotpic_assemble_control', '/jaxrs/hotpic/assemble/control'),
    'x_hotpic_core_entity': ('hotpic_core_entity', '/jaxrs/hotpic/core/entity'),
    'x_jpush_assemble_control': ('jpush_assemble_control', '/jaxrs/jpush/assemble/control'),
    'x_jpush_core_entity': ('jpush_core_entity', '/jaxrs/jpush/core/entity'),
    'x_meeting_assemble_control': ('meeting_assemble_control', '/jaxrs/meeting/assemble/control'),
    'x_meeting_core_entity': ('meeting_core_entity', '/jaxrs/meeting/core/entity'),
    'x_message_assemble_communicate': ('message_assemble_communicate', '/jaxrs/message/assemble/communicate'),
    'x_message_core_entity': ('message_core_entity', '/jaxrs/message/core/entity'),
    'x_component_assemble_control': ('component_assemble_control', '/jaxrs/component/assemble/control'),
    'x_component_core_entity': ('component_core_entity', '/jaxrs/component/core/entity'),
    'x_ai_assemble_control': ('ai_assemble_control', '/jaxrs/ai/assemble/control'),
    'x_ai_core_entity': ('ai_core_entity', '/jaxrs/ai/core/entity'),
    'x_base_core_project': ('base', '/jaxrs/base'),
    'x_program_init': ('program_init', '/jaxrs/secret'),
    'x_program_center': ('program_center', '/jaxrs/program_center'),
    'x_program_center_core_entity': ('program_center_core_entity', '/jaxrs/program_center/core/entity'),
    'x_console': ('console', '/jaxrs/console'),
}

def extract_java_paths(java_module, prefix):
    """Extract all @Path routes from a Java module."""
    paths = set()
    for root, dirs, files in os.walk(java_dir):
        for f in files:
            if not f.endswith('.java'):
                continue
            jf = os.path.join(root, f)
            if java_module not in jf.replace(os.sep, '/'):
                continue
            try:
                with open(jf, 'r', encoding='utf-8') as fh:
                    lines = fh.readlines()
            except:
                continue
            cls_path = None
            for line in lines:
                m = re.match(r'^(\t*)@Path\(\"([^\"]+)\"\)', line)
                if m:
                    indent = len(m.group(1))
                    val = m.group(2)
                    if indent == 0:
                        cls_path = val
                    else:
                        if cls_path:
                            full = prefix + '/' + cls_path + '/' + val
                        else:
                            full = prefix + '/' + val
                        full = re.sub(r'/+', '/', full)
                        paths.add(full)
    return sorted(paths)

def get_existing_rust_routes(crate_name):
    """Get existing Rust routes for a crate."""
    crate_path = os.path.join(rust_dir, crate_name)
    if not os.path.exists(crate_path):
        return set()
    
    routes = set()
    for root, dirs, files in os.walk(crate_path):
        for f in files:
            if f.endswith('.rs'):
                with open(os.path.join(root, f), 'r', encoding='utf-8') as fh:
                    for line in fh:
                        m = re.search(r'\.route\("([^"]+)"', line)
                        if m:
                            r = m.group(1)
                            if r.endswith('/health') or r == '/hello/world':
                                continue
                            routes.add(r)
    return routes

def func_name_from_path(path):
    """Convert a route path to a valid Rust function name."""
    name = path.replace('/jaxrs/', '')
    name = re.sub(r'[/{}]', '_', name)
    name = name.replace('-', '_')
    name = re.sub(r'_+', '_', name)
    name = name.strip('_')
    return f'stub_{name}'

def generate_stub_handler(path, method='get'):
    """Generate a stub handler function."""
    func_name = func_name_from_path(path)
    
    if method == 'post':
        method_fn = 'post'
    elif method == 'put':
        method_fn = 'put'
    elif method == 'delete':
        method_fn = 'delete'
    else:
        method_fn = 'get'
    
    route_line = f'    .route("{path}", {method_fn}({func_name}))'
    handler_code = f'''
/// Stub handler for {path}
/// TODO: Implement real business logic
pub async fn {func_name}() -> Result<Json<ActionResult<Value>>, AppError> {{
    Ok(Json(ActionResult::success(Value::Null)))
}}
'''
    return route_line, handler_code, func_name

def inject_stubs(crate_lib_path, routes_code, handlers_code):
    """Inject stub code into an existing lib.rs file."""
    with open(crate_lib_path, 'r', encoding='utf-8') as fh:
        content = fh.read()
    
    # Check if stubs already exist
    if '// AUTO-GENERATED STUBS' in content:
        print(f'  Stubs already exist, skipping')
        return False
    
    # Ensure axum::routing imports exist
    if 'use axum::routing::get;' not in content:
        content = content.replace('use axum::{', 'use axum::{\n    routing::get,\n    routing::post,\n    routing::put,\n    routing::delete,')
    
    # Find the router function and replace it
    router_pattern = r'(pub fn router\([^)]*\)\s*->\s*\w+\s*\{)(.*?)(\}\s*)$'
    
    def replace_router(match):
        fn_start = match.group(1)
        fn_body = match.group(2)
        fn_end = match.group(3)
        
        new_body = '\n    Router::new()\n'
        for line in routes_code:
            new_body += line + '\n'
        new_body += '    '
        
        return fn_start + new_body + fn_end
    
    new_content = re.sub(router_pattern, replace_router, content, flags=re.DOTALL)
    
    if new_content == content:
        # Router pattern not found, append to end
        new_content = content + '\n\n// AUTO-GENERATED STUBS - DO NOT EDIT\n' + handlers_code
    else:
        # Router replaced, append handlers before the router
        handlers_pos = new_content.rfind('pub fn router')
        if handlers_pos != -1:
            new_content = new_content[:handlers_pos] + '// AUTO-GENERATED STUBS - DO NOT EDIT\n' + handlers_code + '\n' + new_content[handlers_pos:]
    
    with open(crate_lib_path, 'w', encoding='utf-8') as fh:
        fh.write(new_content)
    
    return True

def main():
    parser = argparse.ArgumentParser(description='Generate stub routes for O2OA Rust')
    parser.add_argument('--module', help='Specific module to process')
    parser.add_argument('--crate', help='Specific crate to process')
    parser.add_argument('--all', action='store_true', help='Process all modules')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be done')
    args = parser.parse_args()
    
    if not args.module and not args.all:
        parser.print_help()
        return
    
    modules_to_process = []
    if args.all:
        modules_to_process = list(module_map.items())
    elif args.module:
        if args.module in module_map:
            modules_to_process = [(args.module, module_map[args.module])]
        else:
            print(f'Unknown module: {args.module}')
            return
    elif args.crate:
        for java_mod, (rust_crate, prefix) in module_map.items():
            if rust_crate == args.crate:
                modules_to_process = [(java_mod, (rust_crate, prefix))]
                break
        if not modules_to_process:
            print(f'Unknown crate: {args.crate}')
            return
    
    total_generated = 0
    for java_mod, (rust_crate, prefix) in modules_to_process:
        print(f'\n=== Processing {java_mod} -> {rust_crate} ===')
        
        # Get Java paths
        java_paths = extract_java_paths(java_mod, prefix)
        print(f'Java paths: {len(java_paths)}')
        
        # Get existing Rust routes
        rust_routes = get_existing_rust_routes(rust_crate)
        print(f'Existing Rust routes: {len(rust_routes)}')
        
        # Find missing
        missing = [p for p in java_paths if p not in rust_routes]
        print(f'Missing routes: {len(missing)}')
        
        if not missing:
            print('Nothing to do!')
            continue
        
        # Generate route lines and handler code
        routes_code = []
        handlers_code = ''
        handler_funcs = set()
        
        for path in missing:
            route_line, handler_code, func_name = generate_stub_handler(path, 'get')
            routes_code.append(route_line)
            if func_name not in handler_funcs:
                handler_funcs.add(func_name)
                handlers_code += handler_code
        
        if args.dry_run:
            print(f'Would generate {len(missing)} stubs for {rust_crate}')
            continue
        
        # Inject into crate
        crate_lib = os.path.join(rust_dir, rust_crate, 'src', 'lib.rs')
        if os.path.exists(crate_lib):
            if inject_stubs(crate_lib, routes_code, handlers_code):
                print(f'Generated {len(missing)} stubs in {crate_lib}')
                total_generated += len(missing)
            else:
                print(f'Skipped {rust_crate} (already has stubs)')
        else:
            print(f'Warning: {crate_lib} not found')
    
    print(f'\nTotal generated: {total_generated} stubs')

if __name__ == '__main__':
    main()
