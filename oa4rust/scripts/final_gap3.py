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
    'x_organization_assemble_authentication': '/authentication',
    'x_organization_assemble_control': '/organization/assemble/control',
    'x_organization_assemble_express': '/organization/assemble/express',
    'x_organization_core_entity': '/organization/core/entity',
    'x_organization_core_express': '/organization/core/express',
    'x_attendance_assemble_control': '/attendance/assemble/control',
    'x_attendance_core_entity': '/attendance/core/entity',
    'x_processplatform_assemble_bam': '/processplatform/assemble/bam',
    'x_processplatform_assemble_designer': '/processplatform/assemble/designer',
    'x_processplatform_assemble_surface': '/processplatform/assemble/surface',
    'x_processplatform_core_entity': '/processplatform/core/entity',
    'x_processplatform_core_express': '/processplatform/core/express',
    'x_processplatform_service_processing': '/processplatform/service/processing',
    'x_process_designer': '/process/designer',
    'x_process_express': '/process/express',
    'x_process_surface': '/process/surface',
    'x_process_bam': '/process/bam',
    'x_portal_assemble_designer': '/portal/assemble/designer',
    'x_portal_assemble_surface': '/portal/assemble/surface',
    'x_portal_core_entity': '/portal/core/entity',
    'x_file_assemble_control': '/file/assemble/control',
    'x_file_core_entity': '/file/core/entity',
    'x_query_assemble_designer': '/query/assemble/designer',
    'x_query_assemble_surface': '/query/assemble/surface',
    'x_query_core_entity': '/query/core/entity',
    'x_query_core_express': '/query/core/express',
    'x_query_service_processing': '/query/service/processing',
    'x_cms_assemble_control': '/cms/assemble/control',
    'x_cms_core_entity': '/cms/core/entity',
    'x_cms_core_express': '/cms/core/express',
    'x_cms_express': '/cms/express',
    'x_bbs_assemble_control': '/bbs/assemble/control',
    'x_bbs_core_entity': '/bbs/core/entity',
    'x_mind_assemble_control': '/mind/assemble/control',
    'x_mind_core_entity': '/mind/core/entity',
    'x_correlation_core_entity': '/correlation/core/entity',
    'x_correlation_core_express': '/correlation/core/express',
    'x_correlation_service_processing': '/correlation/service/processing',
    'x_general_assemble_control': '/general/assemble/control',
    'x_general_core_entity': '/general/core/entity',
    'x_hotpic_assemble_control': '/hotpic/assemble/control',
    'x_hotpic_core_entity': '/hotpic/core/entity',
    'x_jpush_assemble_control': '/jpush/assemble/control',
    'x_jpush_core_entity': '/jpush/core/entity',
    'x_meeting_assemble_control': '/meeting/assemble/control',
    'x_meeting_core_entity': '/meeting/core/entity',
    'x_message_assemble_communicate': '/message/assemble/communicate',
    'x_message_core_entity': '/message/core/entity',
    'x_component_assemble_control': '/component/assemble/control',
    'x_component_core_entity': '/component/core/entity',
    'x_ai_assemble_control': '/ai/assemble/control',
    'x_ai_core_entity': '/ai/core/entity',
    'x_base_core_project': '/base',
    'x_program_init': '/secret',
    'x_program_center': '/program_center',
    'x_program_center_core_entity': '/program_center/core/entity',
    'x_console': '/console',
}

# 3. 提取所有 Java 路径（添加 /jaxrs 前缀）
java_paths = set()
for java_mod, suffix in java_modules.items():
    prefix = '/jaxrs' + suffix
    for root, dirs, files in os.walk(java_dir):
        for f in files:
            if not f.endswith('.java'):
                continue
            jf = os.path.join(root, f)
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
                            full = prefix + '/' + cls_path + '/' + val
                        else:
                            full = prefix + '/' + val
                        # Normalize: remove double slashes
                        full = re.sub(r'/+', '/', full)
                        java_paths.add(full)

print(f'Java unique paths (with /jaxrs prefix): {len(java_paths)}')

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
    if len(parts) >= 4:
        mod = '/'.join(parts[:4])
        mod_missing[mod] = mod_missing.get(mod, 0) + 1

for mod, cnt in sorted(mod_missing.items(), key=lambda x: -x[1])[:20]:
    print(f'  {cnt:4d}  {mod}')
