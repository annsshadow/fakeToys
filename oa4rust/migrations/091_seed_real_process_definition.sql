-- S1 双侧匹配种子（计划 rev8）：真实 O2OA activities 流程定义 + Xform 表单定义。
-- 目标：把 workflow 域行为比对从 '{}' 占位（056）转成"可判定的真语义差异"；
-- 前端 ProcessDesigner/FormDesigner 可加载本种子 → Rust 引擎可发起实例（work_start）。
-- 幂等：upsert（ON CONFLICT DO UPDATE），旧环境重放即刷新 056 的 '{}' 占位。

-- ── 1. 流程定义：begin → manual（审批）→ end ────────────────────────────
-- JSON 形状 = oa4rust-web contracts/process-definition.ts 的 serializeProcessDefinition 输出：
-- activities（15 类 O2 activity 的子集）+ routes/routeList（fromActivity/activity/waypoints）
-- + 字段级类型列表（begin/manualList/endList）+ fieldPermissions。
INSERT INTO x_process_definition (id, name, category, process_definition, version, creator)
VALUES (
    'seed-approval-flow',
    'seed-approval-flow',
    'seed',
    '{
      "id": "seed-approval-flow",
      "name": "seed-approval-flow",
      "alias": "seedApprovalFlow",
      "description": "S1 双侧种子：直线审批流程",
      "application": "seed",
      "edition": "1",
      "fieldPermissions": [
        {
          "id": "fp-days",
          "name": "days",
          "read": ["seed-person-a"],
          "write": ["seed-person-a"]
        }
      ],
      "activities": [
        {
          "id": "act-begin",
          "type": "begin",
          "name": "开始",
          "process": "seed-approval-flow",
          "position": "80,80",
          "edition": "1",
          "routeList": ["route-begin-manual"]
        },
        {
          "id": "act-approve",
          "type": "manual",
          "name": "审批",
          "process": "seed-approval-flow",
          "position": "240,80",
          "edition": "1",
          "routeList": ["route-approve-end"],
          "taskIdentityList": ["seed-person-a"],
          "readIdentityList": [],
          "readUnitList": [],
          "reviewIdentityList": [],
          "reviewUnitList": [],
          "form": "seed-leave-form",
          "formScript": "",
          "customData": {}
        },
        {
          "id": "act-end",
          "type": "end",
          "name": "结束",
          "process": "seed-approval-flow",
          "position": "400,80",
          "edition": "1",
          "routeList": []
        }
      ],
      "routes": [
        {
          "id": "route-begin-manual",
          "process": "seed-approval-flow",
          "fromActivity": "act-begin",
          "activity": "act-approve",
          "activityType": "manual",
          "name": "提交",
          "scriptText": "",
          "waypoints": []
        },
        {
          "id": "route-approve-end",
          "process": "seed-approval-flow",
          "fromActivity": "act-approve",
          "activity": "act-end",
          "activityType": "end",
          "name": "通过",
          "scriptText": "",
          "waypoints": []
        }
      ],
      "routeList": [
        {
          "id": "route-begin-manual",
          "process": "seed-approval-flow",
          "fromActivity": "act-begin",
          "activity": "act-approve",
          "activityType": "manual",
          "name": "提交",
          "scriptText": "",
          "waypoints": []
        },
        {
          "id": "route-approve-end",
          "process": "seed-approval-flow",
          "fromActivity": "act-approve",
          "activity": "act-end",
          "activityType": "end",
          "name": "通过",
          "scriptText": "",
          "waypoints": []
        }
      ],
      "begin": {
        "id": "act-begin",
        "type": "begin",
        "name": "开始",
        "process": "seed-approval-flow",
        "position": "80,80"
      },
      "manualList": [
        {
          "id": "act-approve",
          "type": "manual",
          "name": "审批",
          "process": "seed-approval-flow",
          "position": "240,80",
          "taskIdentityList": ["seed-person-a"]
        }
      ],
      "endList": [
        {
          "id": "act-end",
          "type": "end",
          "name": "结束",
          "process": "seed-approval-flow",
          "position": "400,80"
        }
      ]
    }'::jsonb,
    1,
    'system'
)
ON CONFLICT (id) DO UPDATE
    SET process_definition = EXCLUDED.process_definition,
        name = EXCLUDED.name,
        version = EXCLUDED.version;

-- ── 2. 应用 + Xform 表单定义种子（moduleList 契约，W4 运行时可渲染） ──────
INSERT INTO x_cms_appinfo (id, app_type, alias, enabled, creator)
VALUES ('seed-app', 'cms', 'seedApp', true, 'system')
ON CONFLICT (id) DO NOTHING;

INSERT INTO x_cms_form (id, app_id, name, definition, status, creator)
VALUES (
    'seed-leave-form',
    'seed-app',
    'seed-leave-form',
    '{
      "moduleList": {
        "module-days": {
          "id": "module-days",
          "type": "number",
          "name": "days",
          "label": "请假天数",
          "required": true,
          "defaultValue": null
        },
        "module-reason": {
          "id": "module-reason",
          "type": "text",
          "name": "reason",
          "label": "请假事由",
          "required": true,
          "defaultValue": null
        },
        "module-comment": {
          "id": "module-comment",
          "type": "textarea",
          "name": "comment",
          "label": "备注",
          "required": false,
          "defaultValue": ""
        }
      },
      "desktopData": {
        "html": "<div class=\"form\"><span module=\"module-days\"></span><span module=\"module-reason\"></span><span module=\"module-comment\"></span></div>",
        "columns": 1,
        "gutter": 8
      },
      "mobileData": {
        "html": "<div class=\"form\"><span module=\"module-days\"></span><span module=\"module-reason\"></span><span module=\"module-comment\"></span></div>",
        "columns": 1,
        "gutter": 8
      }
    }',
    'published',
    'system'
)
ON CONFLICT (id) DO UPDATE
    SET definition = EXCLUDED.definition,
        status = EXCLUDED.status;

-- 056 的 '{}' 占位行保留（id=test-id 供旧引用兼容），但 workflow 行为比对以
-- seed-approval-flow 为准。
