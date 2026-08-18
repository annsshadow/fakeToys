#!/usr/bin/env python3
"""
Scan o2server Maven modules and generate module dependency index.
Usage: python generate_module_index.py <o2server_path> <output_path>
"""

import os
import sys
import re
from pathlib import Path
from collections import defaultdict

def parse_modules(parent_pom_path):
    """Extract module names from parent pom.xml"""
    with open(parent_pom_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Find <modules> section
    modules_section = re.search(r'<modules>(.*?)</modules>', content, re.DOTALL)
    if not modules_section:
        return []
    
    modules_text = modules_section.group(1)
    # Extract module names, ignoring comments
    modules = []
    for line in modules_text.split('\n'):
        line = line.strip()
        if line.startswith('<module>') and not line.startswith('<!--'):
            module_name = re.search(r'<module>(.*?)</module>', line)
            if module_name:
                modules.append(module_name.group(1))
    
    return modules

def parse_dependencies(module_pom_path, known_modules):
    """Extract inter-module dependencies from a module's pom.xml"""
    with open(module_pom_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    deps = []
    # Find all <dependency> sections
    dep_sections = re.findall(r'<dependency>(.*?)</dependency>', content, re.DOTALL)
    for dep in dep_sections:
        artifact = re.search(r'<artifactId>(.*?)</artifactId>', dep)
        if artifact:
            artifact_id = artifact.group(1)
            # Check if this is an inter-module dependency
            if artifact_id in known_modules:
                deps.append(artifact_id)
    
    return deps

def categorize_module(module_name):
    """Categorize module by its suffix"""
    if '_core_entity' in module_name:
        return 'core_entity'
    elif '_core_express' in module_name:
        return 'core_express'
    elif '_assemble_control' in module_name:
        return 'assemble_control'
    elif '_assemble_surface' in module_name:
        return 'assemble_surface'
    elif '_assemble_designer' in module_name:
        return 'assemble_designer'
    elif '_service_processing' in module_name:
        return 'service_processing'
    elif module_name in ['x_base_core_project', 'x_program_center', 'x_program_init', 'x_console']:
        return 'infrastructure'
    else:
        return 'other'

def calculate_scores(modules, dependencies):
    """Calculate priority scores for each module"""
    # Calculate in-degree (how many other modules depend on this one)
    in_degree = defaultdict(int)
    for module, deps in dependencies.items():
        for dep in deps:
            in_degree[dep] += 1
    
    scores = {}
    for module in modules:
        dep_list = dependencies.get(module, [])
        
        # Dimension 1: 依赖少 (fewer dependencies = higher score)
        dep_count = len(dep_list)
        dep_score = max(0, 10 - dep_count)  # 0 deps = 10, 10+ deps = 0
        
        # Dimension 2: 替换杠杆高 (more modules depend on this = higher score)
        leverage = in_degree.get(module, 0)
        leverage_score = min(10, leverage)
        
        # Dimension 3: 业务清晰 (assigned based on module type)
        category = categorize_module(module)
        if category == 'assemble_control':
            clarity_score = 10  # Business logic is clear
        elif category == 'core_entity':
            clarity_score = 8  # Data model is clear
        elif category == 'service_processing':
            clarity_score = 7  # Background jobs
        elif category in ['infrastructure']:
            clarity_score = 5  # Infrastructure, may be complex
        else:
            clarity_score = 6
        
        # Overall score (weighted average)
        overall = round((dep_score + leverage_score + clarity_score) / 3, 1)
        
        scores[module] = {
            'dependencies': dep_count,
            'dep_score': dep_score,
            'leverage': leverage,
            'leverage_score': leverage_score,
            'clarity_score': clarity_score,
            'overall': overall,
            'category': category
        }
    
    return scores

def generate_markdown(modules, dependencies, scores, output_path):
    """Generate module index markdown"""
    output = []
    output.append('# o2server 模块依赖索引')
    output.append('')
    output.append(f'**总计模块数：** {len(modules)}')
    output.append('')
    output.append('## 模块列表')
    output.append('')
    
    # Sort by overall score descending
    sorted_modules = sorted(modules, key=lambda m: scores[m]['overall'], reverse=True)
    
    # Table header
    output.append('| 模块名 | 类别 | 依赖数 | 被依赖数 | 替换杠杆分 | 业务清晰分 | 综合评分 | 首批试点 |')
    output.append('|--------|------|--------|----------|------------|------------|----------|----------|')
    
    for module in sorted_modules:
        s = scores[module]
        category = s['category']
        dep_count = s['dependencies']
        leverage = s['leverage']
        dep_score = s['dep_score']
        clarity_score = s['clarity_score']
        overall = s['overall']
        
        # Mark first batch candidates
        if module in ['x_organization_assemble_authentication', 'x_organization_assemble_control']:
            first_batch = '是'
        else:
            first_batch = ''
        
        output.append(f'| {module} | {category} | {dep_count} | {leverage} | {dep_score} | {clarity_score} | {overall} | {first_batch} |')
    
    output.append('')
    output.append('## 依赖关系图')
    output.append('')
    output.append('### 按依赖数排序（依赖最少的模块优先）')
    output.append('')
    
    # Group by dependency count
    dep_groups = defaultdict(list)
    for module in modules:
        dep_groups[scores[module]['dependencies']].append(module)
    
    for dep_count in sorted(dep_groups.keys()):
        modules_with_count = dep_groups[dep_count]
        output.append(f'**依赖数 = {dep_count}：** {", ".join(sorted(modules_with_count))}')
        output.append('')
    
    output.append('## 替换优先级排序')
    output.append('')
    output.append('评分规则：')
    output.append('- **依赖少分**：依赖数越少得分越高（0 个依赖 = 10 分，每增加一个依赖减 1 分）')
    output.append('- **替换杠杆分**：被其他模块依赖的数量越多得分越高（被 1 个模块依赖 = 1 分，上限 10 分）')
    output.append('- **业务清晰分**：assemble_control = 10 分，core_entity = 8 分，service_processing = 7 分，infrastructure = 5 分')
    output.append('- **综合评分**：三个维度的平均值')
    output.append('')
    output.append('### 首批试点模块')
    output.append('')
    output.append('- **x_organization_assemble_authentication** — 认证服务，综合评分 {:.1f}'.format(scores['x_organization_assemble_authentication']['overall']))
    output.append('  - 依赖数：{}，被依赖数：{}'.format(
        scores['x_organization_assemble_authentication']['dependencies'],
        scores['x_organization_assemble_authentication']['leverage']))
    output.append('  - 包含端点：/jaxrs/authentication/*（login, logout, who, captcha, code, bind, oauth 等）')
    output.append('')
    output.append('- **x_organization_assemble_control** — 组织控制，综合评分 {:.1f}'.format(scores['x_organization_assemble_control']['overall']))
    output.append('  - 依赖数：{}，被依赖数：{}'.format(
        scores['x_organization_assemble_control']['dependencies'],
        scores['x_organization_assemble_control']['leverage']))
    output.append('  - 包含端点：/jaxrs/person/*、/jaxrs/unit/*、/jaxrs/role/*、/jaxrs/group/* 等')
    output.append('')
    output.append('### 优先级排序（Top 10）')
    output.append('')
    output.append('| 排名 | 模块名 | 综合评分 | 说明 |')
    output.append('|------|--------|----------|------|')
    
    for i, module in enumerate(sorted_modules[:10], 1):
        s = scores[module]
        category = s['category']
        if category == 'assemble_control':
            desc = '业务控制层，端点明确'
        elif category == 'core_entity':
            desc = '数据实体层，schema 清晰'
        elif category == 'service_processing':
            desc = '后台服务，相对独立'
        else:
            desc = '基础设施层'
        
        # Mark first batch
        if module in ['x_organization_assemble_authentication', 'x_organization_assemble_control']:
            desc += ' [首批试点]'
        
        output.append(f'| {i} | {module} | {s["overall"]} | {desc} |')
    
    output.append('')
    output.append('## 认证模块详细分析')
    output.append('')
    output.append('### x_organization_assemble_authentication')
    output.append('')
    output.append('- **职责：** 认证服务，处理登录、登出、whoami、captcha、code、bind、oauth 等')
    output.append('- **依赖模块：** {}'.format(', '.join(dependencies.get('x_organization_assemble_authentication', [])) or '无'))
    output.append('- **被依赖模块：** {}'.format(in_degree.get('x_organization_assemble_authentication', 0)))
    output.append('- **关键端点：** /jaxrs/authentication/login, /jaxrs/authentication/logout, /jaxrs/authentication/who, /jaxrs/authentication/captcha, /jaxrs/authentication/code, /jaxrs/authentication/bind, /jaxrs/authentication/oauth')
    output.append('')
    output.append('### x_organization_assemble_control')
    output.append('')
    output.append('- **职责：** 组织控制，处理人员、单位、角色、组等 CRUD 操作')
    output.append('- **依赖模块：** {}'.format(', '.join(dependencies.get('x_organization_assemble_control', [])) or '无'))
    output.append('- **被依赖模块：** {}'.format(in_degree.get('x_organization_assemble_control', 0)))
    output.append('- **关键端点：** /jaxrs/person/*, /jaxrs/unit/*, /jaxrs/role/*, /jaxrs/group/*')
    output.append('')
    output.append('### x_program_init（系统初始化，含 /jaxrs/secret/*）')
    output.append('')
    output.append('- **职责：** 系统初始化，处理密码设置、数据库检查、恢复等')
    output.append('- **依赖模块：** {}'.format(', '.join(dependencies.get('x_program_init', [])) or '无'))
    output.append('- **被依赖模块：** {}'.format(in_degree.get('x_program_init', 0)))
    output.append('- **关键端点：** /jaxrs/secret/check, /jaxrs/secret/set, /jaxrs/secret/cancel')
    output.append('')
    output.append('---')
    output.append('')
    output.append('*生成时间：* 2026-08-03')
    output.append('*生成脚本：* docs/oa/scripts/generate_module_index.py')
    
    # Write to file
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(output))
    
    print(f'Module index written to: {output_path}')
    print(f'Total modules: {len(modules)}')
    print(f'First batch candidates: x_organization_assemble_authentication, x_organization_assemble_control')

if __name__ == '__main__':
    if len(sys.argv) < 3:
        print('Usage: python generate_module_index.py <o2server_path> <output_path>')
        sys.exit(1)
    
    o2server_path = sys.argv[1]
    output_path = sys.argv[2]
    
    parent_pom = os.path.join(o2server_path, 'pom.xml')
    
    print(f'Scanning {o2server_path}...')
    modules = parse_modules(parent_pom)
    print(f'Found {len(modules)} modules')
    
    # Parse dependencies for each module
    dependencies = {}
    for module in modules:
        module_pom = os.path.join(o2server_path, module, 'pom.xml')
        if os.path.exists(module_pom):
            deps = parse_dependencies(module_pom, set(modules))
            dependencies[module] = deps
            print(f'  {module}: {len(deps)} inter-module dependencies')
        else:
            dependencies[module] = []
            print(f'  {module}: pom.xml not found')
    
    # Calculate in-degree for leverage scoring
    in_degree = defaultdict(int)
    for module, deps in dependencies.items():
        for dep in deps:
            in_degree[dep] += 1
    
    # Calculate scores
    scores = calculate_scores(modules, dependencies)
    
    # Generate markdown
    generate_markdown(modules, dependencies, scores, output_path)
    
    print('Done!')
