---
title: "OA Component Card Generation from Source Metadata"
date: 2026-08-10
category: tooling-decisions
module: docs/oa/scripts
problem_type: tooling_decision
component: tooling
severity: medium
symptoms:
  - "55 o2server module cards and 86 o2web component cards had empty Responsibility fields"
  - "New contributors couldn't understand module/component purposes without reading source code"
  - "Manual documentation was infeasible at this scale"
root_cause: missing_tooling
resolution_type: tooling_addition
tags: [documentation, component-cards, code-generation, oa-documentation]
related_components:
  - docs/oa/scripts/generate_cards.py
  - docs/oa/scripts/fill_responsibility.py
  - docs/oa/templates/module-card.md
  - docs/oa/templates/component-card.md
applies_when:
  - "Generating documentation for a large number of similar components"
  - "Documentation needs to stay in sync with source code"
  - "Template-based documentation with some manual enrichment is acceptable"
---

# OA Component Card Generation from Source Metadata

## Context

The `docs/oa/` documentation project needed 55 o2server module cards and 86 o2web component cards, each with a `Responsibility` field describing what the module/component does. Manually writing these would take days. The solution was a code-driven generation pipeline: extract metadata from source code, generate card skeletons, then fill Responsibility fields programmatically based on component name and API dependencies.

## Guidance

### Module Cards (o2server)

Generated from `oa/o2server/pom.xml`:
- Module name from `<module>` element
- ArtifactId and packaging from each module's `pom.xml`
- Dependencies from `<dependencies>` declarations
- Responsibility inferred from module name patterns:
  - `x_organization_*` → 组织管理相关
  - `x_processplatform_*` → 流程引擎相关
  - `x_cms_*` → 内容管理相关
  - etc.

### Component Cards (o2web)

Generated from `oa/o2web/source/x_component_*/`:
- Component name from directory name
- Title from `lp/zh-cn.js` (e.g., `"title": "考勤管理"`)
- API dependencies from `Main.js` (e.g., `MWF.Actions.get("x_file_assemble_control")`)
- Responsibility inferred from title and API dependencies

### Generation Script

```python
# docs/oa/scripts/generate_cards.py
import os, re, json

source_dir = r'D:\WORKSPACE\fakeToys\oa\o2web\source'
docs_dir = r'D:\WORKSPACE\fakeToys\docs\oa\modules\o2web'

for d in sorted(os.listdir(source_dir)):
    if not d.startswith('x_component_'):
        continue
    # Extract title from lp/zh-cn.js
    lp_file = os.path.join(source_dir, d, 'lp', 'zh-cn.js')
    title = extract_title(lp_file) if os.path.exists(lp_file) else ''
    
    # Extract API dependencies from Main.js
    actions = extract_actions(os.path.join(source_dir, d, 'Main.js'))
    
    # Generate card
    generate_card(d, title, actions, docs_dir)
```

### Responsibility Filling Strategy

```python
def get_responsibility(name, title, actions):
    # CMS components
    if name.startswith('x_component_cms_'):
        return f'CMS（内容管理）前端组件，负责{title}的界面展示与交互操作。'
    # Process components
    if name.startswith('x_component_process_'):
        return f'流程引擎前端组件，负责{title}的界面展示与交互操作。'
    # ... other patterns ...
    
    # Fallback based on title
    if title:
        return f'{title}前端组件，负责相关功能的界面展示与交互操作。'
    return f'{name}前端组件，负责相关功能的界面展示与交互操作。'
```

### Verification

```bash
# Check all cards have non-empty Responsibility
Select-String -Path "docs/oa/modules/o2web\*.md" -Pattern "## Responsibility" | Measure-Object | Select-Object -ExpandProperty Count
# Expected: 86 (one per card)

# Check no card has empty Responsibility
Get-ChildItem docs/oa/modules/o2web/*.md | ForEach-Object { 
    $c = Get-Content $_.FullName -Raw
    if ($c -match "## Responsibility\s*\n\s*\n") { $_.Name } 
}
# Expected: empty (all cards filled)
```

## Why This Matters

- Scales to any number of components without linear documentation cost
- Source metadata is the single source of truth; cards stay accurate as source changes
- Template standardization ensures consistent format across all cards
- Manual review of generated content is still possible and encouraged for complex components

## When to Apply

- Large numbers of similar documentation items (modules, components, APIs)
- Documentation that must stay synchronized with source code
- When a template with conditional sections can cover all variants

## Examples

**Generated card skeleton:**
```markdown
# x_component_Attendance

## Responsibility

考勤管理组件，负责考勤记录的查看、打卡统计和申诉管理。

## Entry Point

- `oa\o2web\source\x_component_Attendance\Main.js`

## Build Pipeline

- No build scripts detected

## Key Configuration Files

- `$AbnormalExport\listItem.json`: To be described.
- `$Explorer\toolbar.json`: To be described.
```

## Related

- **Script:** `docs/oa/scripts/generate_cards.py`
- **Template:** `docs/oa/templates/component-card.md`
- **Plan:** `docs/plans/2026-08-03-001-feat-oa-project-documentation-plan.md` (U3, U4)
- **Completion:** `docs/plans/2026-08-10-001-prod-readiness-plan.md` (U4, U5)
