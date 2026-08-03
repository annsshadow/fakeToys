# Low-Code Capabilities

This section describes the platform's low-code and no-code customization surfaces: form designer, page/portal designer, and process designer.

## Form Designer

The form designer lets administrators build data-entry forms without writing code.

- Located in `x_component_process_FormDesigner`
- Backed by `x_processplatform_assemble_designer` and `x_query_assemble_designer`
- Forms are bound to `DataRecord` entities and rendered client-side by `x_component_process_Xform`
- Supported field types include text, number, date, select, checkbox, radio, attachment, and sub-form

Typical flow:
1. Designer defines fields, validation rules, and layout.
2. Form definition is stored as a module resource or `AppInfo` configuration.
3. Runtime renderer (`x_component_process_Xform`) loads the definition and binds it to a process or standalone view.

## Page / Portal Designer

The portal designer composes role-based workspaces from pages, widgets, and menus.

- Located in `x_component_portal_PageDesigner` and `x_component_portal_WidgetDesigner`
- Backed by `x_portal_assemble_designer`
- Pages are versioned (`Page`, `PageVersion`) and can contain scripted widgets
- Portal hierarchy: `Portal` → `Page` → `Widget` → `Script`

Typical flow:
1. Designer creates or edits a page layout.
2. Widgets are configured with data sources (REST actions, process data, queries).
3. Script designer adds Express logic for dynamic behavior.
4. Portal is assigned to identity groups for access control.

## Process Designer

The process designer defines workflow topologies and routing rules.

- Located in `x_component_process_ProcessDesigner`
- Backed by `x_processplatform_assemble_designer`
- Processes are stored as `ProcessUnit` entities
- Nodes support assignee expressions (identity, role, department, script)
- Events, gateways, and sub-processes are supported

Typical flow:
1. Designer draws the process flow.
2. Each node's assignee rule is configured via identity selector or Express script.
3. Form binding is attached to user tasks.
4. Process is published and becomes available in the task center.

## Scripting and Expressions

The platform uses **Express** as its scripting language for process conditions, portal scripts, query expressions, and data validations.

- Express scripts are evaluated server-side by `x_*_core_express` modules.
- Scripts can reference entity fields, context variables, and action results.
- The script designer components (`x_component_cms_ScriptDesigner`, `x_component_portal_ScriptDesigner`, `x_component_process_ScriptDesigner`) provide syntax-aware editing.

## Secondary Development Entry Points

| Surface | Component | Backend Module |
|---------|-----------|----------------|
| Form designer | `x_component_process_FormDesigner` | `x_processplatform_assemble_designer` |
| Page designer | `x_component_portal_PageDesigner` | `x_portal_assemble_designer` |
| Process designer | `x_component_process_ProcessDesigner` | `x_processplatform_assemble_designer` |
| Query designer | `x_component_query_StatDesigner` | `x_query_assemble_designer` |
| Script designer | `x_component_cms_ScriptDesigner` | `x_cms_assemble_control` |
