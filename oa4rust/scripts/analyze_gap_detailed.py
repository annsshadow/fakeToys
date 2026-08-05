import os, re, json

java_dir = os.path.abspath('../oa/o2server')
rust_dir = os.path.abspath('crates')

# 1. 提取所有 Rust 路由（按 crate 分组）
rust_routes_by_crate = {}
for root, dirs, files in os.walk(rust_dir):
    crate_name = os.path.basename(root)
    if crate_name == 'crates' or not os.path.exists(os.path.join(root, 'Cargo.toml')):
        continue
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

# 2. 提取所有 Java @Path（按模块分组）
java_paths_by_module = {}
module_prefix_map = {
    'x_organization_assemble_authentication': '/jaxrs/authentication',
    'x_organization_assemble_control': '/jaxrs/organization/assemble/control',
    'x_organization_assemble_express': '/jaxrs/organization/assemble/express',
    'x_organization_core_entity': '/jaxrs/organization/core/entity',
    'x_organization_core_express': '/jaxrs/organization/core/express',
    'x_attendance_assemble_control': '/jaxrs/attendance/assemble/control',
    'x_attendance_core_entity': '/jaxrs/attendance/core/entity',
    'x_processplatform_assemble_bam': '/jaxrs/processplatform/assemble/bam',
    'x_processplatform_assemble_designer': '/jaxrs/processplatform/assemble/designer',
    'x_processplatform_assemble_surface': '/jaxrs/processplatform/assemble/surface',
    'x_processplatform_core_entity': '/jaxrs/processplatform/core/entity',
    'x_processplatform_core_express': '/jaxrs/processplatform/core/express',
    'x_processplatform_service_processing': '/jaxrs/processplatform/service/processing',
    'x_process_designer': '/jaxrs/process/designer',
    'x_process_express': '/jaxrs/process/express',
    'x_process_surface': '/jaxrs/process/surface',
    'x_process_bam': '/jaxrs/process/bam',
    'x_portal_assemble_designer': '/jaxrs/portal/assemble/designer',
    'x_portal_assemble_surface': '/jaxrs/portal/assemble/surface',
    'x_portal_core_entity': '/jaxrs/portal/core/entity',
    'x_file_assemble_control': '/jaxrs/file/assemble/control',
    'x_file_core_entity': '/jaxrs/file/core/entity',
    'x_query_assemble_designer': '/jaxrs/query/assemble/designer',
    'x_query_assemble_surface': '/jaxrs/query/assemble/surface',
    'x_query_core_entity': '/jaxrs/query/core/entity',
    'x_query_core_express': '/jaxrs/query/core/express',
    'x_query_service_processing': '/jaxrs/query/service/processing',
    'x_cms_assemble_control': '/jaxrs/cms/assemble/control',
    'x_cms_core_entity': '/jaxrs/cms/core/entity',
    'x_cms_core_express': '/jaxrs/cms/core/express',
    'x_cms_express': '/jaxrs/cms/express',
    'x_bbs_assemble_control': '/jaxrs/bbs/assemble/control',
    'x_bbs_core_entity': '/jaxrs/bbs/core/entity',
    'x_mind_assemble_control': '/jaxrs/mind/assemble/control',
    'x_mind_core_entity': '/jaxrs/mind/core/entity',
    'x_correlation_core_entity': '/jaxrs/correlation/core/entity',
    'x_correlation_core_express': '/jaxrs/correlation/core/express',
    'x_correlation_service_processing': '/jaxrs/correlation/service/processing',
    'x_general_assemble_control': '/jaxrs/general/assemble/control',
    'x_general_core_entity': '/jaxrs/general/core/entity',
    'x_hotpic_assemble_control': '/jaxrs/hotpic/assemble/control',
    'x_hotpic_core_entity': '/jaxrs/hotpic/core/entity',
    'x_jpush_assemble_control': '/jaxrs/jpush/assemble/control',
    'x_jpush_core_entity': '/jaxrs/jpush/core/entity',
    'x_meeting_assemble_control': '/jaxrs/meeting/assemble/control',
    'x_meeting_core_entity': '/jaxrs/meeting/core/entity',
    'x_message_assemble_communicate': '/jaxrs/message/assemble/communicate',
    'x_message_core_entity': '/jaxrs/message/core/entity',
    'x_component_assemble_control': '/jaxrs/component/assemble/control',
    'x_component_core_entity': '/jaxrs/component/core/entity',
    'x_ai_assemble_control': '/jaxrs/ai/assemble/control',
    'x_ai_core_entity': '/jaxrs/ai/core/entity',
    'x_base_core_project': '/jaxrs/base',
    'x_program_init': '/jaxrs/secret',
    'x_program_center': '/jaxrs/program_center',
    'x_program_center_core_entity': '/jaxrs/program_center/core/entity',
    'x_console': '/jaxrs/console',
}

for java_mod, prefix in module_prefix_map.items():
    java_paths_by_module[java_mod] = set()
    for root, dirs, files in os.walk(java_dir):
        for f in files:
            if not f.endswith('.java'):
                continue
            jf = os.path.join(root, f)
            try:
                with open(jf, 'r', encoding='utf-8') as fh:
                    lines = fh.readlines()
            except:
                continue
            # 只处理属于该模块的文件
            if java_mod not in jf.replace('\\', '/'):
                continue
            cls = None
            for line in lines:
                m = re.match(r'^\s*@Path\(\"([^\"]+)\"\)', line)
                if m:
                    val = m.group(1)
                    indent = len(line) - len(line.lstrip())
                    if indent == 0:
                        cls = val
                    else:
                        if cls:
                            full = f'{cls}/{val}'
                        else:
                            full = val
                        if full.startswith(prefix):
                            java_paths_by_module[java_mod].add(full)

# 3. 对比缺口
print('=== Route gap analysis ===')
total_missing = 0
for java_mod, prefix in module_prefix_map.items():
    rust_routes = rust_routes_by_crate.get(java_mod.replace('x_', ''), set())
    java_paths = java_paths_by_module.get(java_mod, set())
    missing = java_paths - rust_routes
    if missing:
        total_missing += len(missing)
        print(f'\n{java_mod} ({len(missing)} missing):')
        for p in sorted(missing)[:10]:
            print(f'  {p}')
        if len(missing) > 10:
            print(f'  ... and {len(missing) - 10} more')

print(f'\nTotal missing routes: {total_missing}')
