#!/usr/bin/env python3
"""
Generate stub Rust routes from Java @Path annotations.
This script reads Java source files and generates axum route stubs.
"""

import os
import re
import argparse
from collections import defaultdict

java_dir = os.path.abspath('../oa/o2server')
rust_dir = os.path.abspath('crates')

# Java module to Rust crate name mapping
module_crate_map = {
    'x_organization_assemble_authentication': 'organization_assemble_authentication',
    'x_organization_assemble_control': 'organization_assemble_control',
    'x_organization_assemble_express': 'organization_assemble_express',
    'x_organization_core_entity': 'organization_core_entity',
    'x_organization_core_express': 'organization_core_express',
    'x_attendance_assemble_control': 'attendance_assemble_control',
    'x_attendance_core_entity': 'attendance_core_entity',
    'x_processplatform_assemble_bam': 'processplatform_assemble_bam',
    'x_processplatform_assemble_designer': 'processplatform_assemble_designer',
    'x_processplatform_assemble_surface': 'processplatform_assemble_surface',
    'x_processplatform_core_entity': 'processplatform_core_entity',
    'x_processplatform_core_express': 'processplatform_core_express',
    'x_processplatform_service_processing': 'processplatform_service_processing',
    'x_process_designer': 'process_designer',
    'x_process_express': 'process_express',
    'x_process_surface': 'process_surface',
    'x_process_bam': 'process_bam',
    'x_portal_assemble_designer': 'portal_assemble_designer',
    'x_portal_assemble_surface': 'portal_assemble_surface',
    'x_portal_core_entity': 'portal_core_entity',
    'x_file_assemble_control': 'file_assemble_control',
    'x_file_core_entity': 'file_core_entity',
    'x_query_assemble_designer': 'query_assemble_designer',
    'x_query_assemble_surface': 'query_assemble_surface',
    'x_query_core_entity': 'query_core_entity',
    'x_query_core_express': 'query_core_express',
    'x_query_service_processing': 'query_service_processing',
    'x_cms_assemble_control': 'cms_assemble_control',
    'x_cms_core_entity': 'cms_core_entity',
    'x_cms_core_express': 'cms_core_express',
    'x_cms_express': 'cms_express',
    'x_bbs_assemble_control': 'bbs_assemble_control',
    'x_bbs_core_entity': 'bbs_core_entity',
    'x_mind_assemble_control': 'mind_assemble_control',
    'x_mind_core_entity': 'mind_core_entity',
    'x_correlation_core_entity': 'correlation_core_entity',
    'x_correlation_core_express': 'correlation_core_express',
    'x_correlation_service_processing': 'correlation_service_processing',
    'x_general_assemble_control': 'general_assemble_control',
    'x_general_core_entity': 'general_core_entity',
    'x_hotpic_assemble_control': 'hotpic_assemble_control',
    'x_hotpic_core_entity': 'hotpic_core_entity',
    'x_jpush_assemble_control': 'jpush_assemble_control',
    'x_jpush_core_entity': 'jpush_core_entity',
    'x_meeting_assemble_control': 'meeting_assemble_control',
    'x_meeting_core_entity': 'meeting_core_entity',
    'x_message_assemble_communicate': 'message_assemble_communicate',
    'x_message_core_entity': 'message_core_entity',
    'x_component_assemble_control': 'component_assemble_control',
    'x_component_core_entity': 'component_core_entity',
    'x_ai_assemble_control': 'ai_assemble_control',
    'x_ai_core_entity': 'ai_core_entity',
    'x_base_core_project': 'base',
    'x_program_init': 'program_init',
    'x_program_center': 'program_center',
    'x_program_center_core_entity': 'program_center_core_entity',
    'x_console': 'console',
}

def extract_java_paths(java_mod):
    """Extract all @Path routes from a Java module."""
    paths = set()
    for root, dirs, files in os.walk(java_dir):
        for f in files:
            if not f.endswith('.java'):
                continue
            jf = os.path.join(root, f)
            if java_mod not in jf.replace(os.sep, '/'):
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
                            full = f'{cls_path}/{val}'
                        else:
                            full = val
                        paths.add(full)
    return paths

def get_rust_routes_for_crate(crate_name):
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

def generate_stub_handler(route_path, method='get'):
    """Generate a stub handler function name from a route path."""
    # Convert path to function name
    # e.g., /jaxrs/work/list/{id} -> work_list_id
    func_name = route_path.replace('/jaxrs/', '').replace('/', '_').replace('{', '').replace('}', '')
    # Remove leading underscore
    func_name = func_name.strip('_')
    # Add prefix
    func_name = f'stub_{func_name}'
    # Ensure valid Rust identifier
    func_name = re.sub(r'[^a-zA-Z0-9_]', '_', func_name)
    # Remove double underscores
    func_name = re.sub(r'_+', '_', func_name)
    return func_name

def generate_stub_code(crate_name, route_path, method='get'):
    """Generate stub Rust code for a route."""
    func_name = generate_stub_handler(route_path, method)
    
    # Determine HTTP method
    method_upper = method.upper()
    
    stub = f'''
    .route("{route_path}", {method_upper}({func_name}))

/// Stub handler for {route_path}
/// TODO: Implement real logic
pub async fn {func_name}(
    pool: axum::Extension<deadpool_postgres::Pool>,
) -> Result<axum::Json<shared::response::ActionResult<serde_json::Value>>, shared::error::AppError> {{
    let _client = pool.get().await.map_err(|_| shared::error::AppError::Internal)?;
    Ok(axum::Json(shared::response::ActionResult::success(serde_json::json!(null))))
}}
'''
    return stub

def main():
    parser = argparse.ArgumentParser(description='Generate stub Rust routes from Java @Path')
    parser.add_argument('--module', help='Java module name (e.g., x_processplatform_core_entity)')
    parser.add_argument('--crate', help='Rust crate name (e.g., processplatform_core_entity)')
    parser.add_argument('--all', action='store_true', help='Process all modules')
    parser.add_argument('--output', default='stubs', help='Output directory')
    args = parser.parse_args()
    
    if not args.module and not args.all:
        parser.print_help()
        return
    
    modules_to_process = []
    if args.all:
        modules_to_process = list(module_crate_map.items())
    elif args.module:
        if args.module in module_crate_map:
            modules_to_process = [(args.module, module_crate_map[args.module])]
        else:
            print(f'Unknown module: {args.module}')
            return
    
    for java_mod, rust_crate in modules_to_process:
        print(f'\n=== Processing {java_mod} -> {rust_crate} ===')
        
        # Get Java paths
        java_paths = extract_java_paths(java_mod)
        print(f'Java paths: {len(java_paths)}')
        
        # Get existing Rust routes
        rust_routes = get_rust_routes_for_crate(rust_crate)
        print(f'Existing Rust routes: {len(rust_routes)}')
        
        # Find missing
        missing = java_paths - rust_routes
        print(f'Missing routes: {len(missing)}')
        
        if not missing:
            continue
        
        # Generate stub code
        stub_code = []
        for path in sorted(missing):
            stub_code.append(generate_stub_code(rust_crate, path))
        
        # Write to file
        output_file = os.path.join(args.output, f'{rust_crate}_stubs.rs')
        with open(output_file, 'w', encoding='utf-8') as fh:
            fh.write('\n'.join(stub_code))
        
        print(f'Generated {len(missing)} stubs in {output_file}')

if __name__ == '__main__':
    main()
