import os, re

java_dir = os.path.abspath('../oa/o2server')
rust_dir = os.path.abspath('crates')

# 1. 提取所有 Rust 路由（按 crate 分组）
rust_routes_by_crate = {}
for crate_name in os.listdir(rust_dir):
    crate_path = os.path.join(rust_dir, crate_name)
    if not os.path.isdir(crate_path) or not os.path.exists(os.path.join(crate_path, 'Cargo.toml')):
        continue
    for root, dirs, files in os.walk(crate_path):
        for f in files:
            if f.endswith('.rs'):
                fpath = os.path.join(root, f)
                with open(fpath, 'r', encoding='utf-8') as fh:
                    for line in fh:
                        m = re.search(r'\.route\("([^"]+)"', line)
                        if m:
                            r = m.group(1)
                            if not r.endswith('/health') and r != '/hello/world' and r.startswith('/jaxrs/'):
                                if crate_name not in rust_routes_by_crate:
                                    rust_routes_by_crate[crate_name] = set()
                                rust_routes_by_crate[crate_name].add(r)

# 2. 模块映射
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

# 3. 对比缺口
print('=== Route gap analysis ===')
total_missing = 0
for java_mod, rust_crate in module_crate_map.items():
    rust_routes = rust_routes_by_crate.get(rust_crate, set())
    print(f'{java_mod} -> {rust_crate}: {len(rust_routes)} routes')
    if len(rust_routes) < 5:
        print(f'  WARNING: very few routes!')
        for r in sorted(rust_routes)[:5]:
            print(f'    {r}')

print(f'\nTotal crates with routes: {len(rust_routes_by_crate)}')
print(f'Total rust routes: {sum(len(v) for v in rust_routes_by_crate.values())}')
