import os, re

java_dir = os.path.abspath('../oa/o2server')
rust_dir = os.path.abspath('crates')

# 1. Rust 已有路由
rust_routes = set()
for crate_name in os.listdir(rust_dir):
    crate_path = os.path.join(rust_dir, crate_name)
    if not os.path.isdir(crate_path) or not os.path.exists(os.path.join(crate_path, 'Cargo.toml')):
        continue
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
                            if r.startswith('/jaxrs/'):
                                rust_routes.add(r)

print(f'Rust routes: {len(rust_routes)}')

# 2. Java 模块映射
java_modules = {
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

# 3. 提取 Java 路径（仅 mapped 模块）
java_paths = set()
for java_mod, prefix in java_modules.items():
    for root, dirs, files in os.walk(java_dir):
        for f in files:
            if not f.endswith('.java'):
                continue
            jf = os.path.join(root, f)
            # 只处理属于该模块的文件
            if java_mod not in jf.replace(os.sep, '/'):
                continue
            with open(jf, 'r', encoding='utf-8') as fh:
                lines = fh.readlines()
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
                        java_paths.add(full)

print(f'Java unique paths (mapped modules): {len(java_paths)}')

# 4. 找缺失
missing = sorted(java_paths - rust_routes)
print(f'Missing routes: {len(missing)}')
for p in missing[:30]:
    print(f'  {p}')

# 5. 按模块统计缺口
print('\n=== Missing by module (top 20) ===')
mod_missing = {}
for p in missing:
    parts = p.split('/')
    if len(parts) >= 3:
        mod = '/'.join(parts[:3])
        mod_missing[mod] = mod_missing.get(mod, 0) + 1

for mod, cnt in sorted(mod_missing.items(), key=lambda x: -x[1])[:20]:
    print(f'  {cnt:4d}  {mod}')
