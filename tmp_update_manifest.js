// 临时脚本：批量更新 W13 manifest 条目为 implemented（跑完即删）
const fs = require('fs');
const p = './docs/audits/w13-w14-repository-closure.manifest.json';
const m = JSON.parse(fs.readFileSync(p, 'utf8'));
const V = 'oa4rust-web/apps/desktop/src/views/';
const R = 'oa4rust-web/apps/desktop/src/main.ts';
const defs = {
  'CmsDictDesignerApp.vue': {
    acc: 'CMS 字典项数据驱动 CRUD：list/create/save/delete 实挂 x_cms_surface_appdict（app_info_flag/app_dict_flag/data_value，参数化写、软删、删除需确认）',
    be: [['oa4rust/crates/cms_assemble_control/src/routes.rs', '"/jaxrs/cms/assemble/control/dict/create"']],
    fe2: 'api.post(createEp, payload)',
  },
  'CmsFormDesignerApp.vue': {
    acc: 'CMS 表单定义数据驱动 CRUD：list/create/save/delete 实挂 x_cms_form（name/app_id/definition JSON 编辑、参数化写、软删、删除需确认）',
    be: [['oa4rust/crates/cms_assemble_control/src/routes.rs', '"/jaxrs/cms/assemble/control/form/create"']],
    fe2: 'api.post(createEp, payload)',
  },
  'CmsViewDesignerApp.vue': {
    acc: 'CMS 视图定义数据驱动 CRUD：list/create/save/delete 实挂 x_cms_view（name/app_id/category_id/view_config JSON，参数化写、软删、删除需确认）',
    be: [['oa4rust/crates/cms_assemble_control/src/routes.rs', '"/jaxrs/cms/assemble/control/view/create"']],
    fe2: 'api.post(createEp, payload)',
  },
  'CmsXformApp.vue': {
    acc: 'CMS XForm 定义数据驱动 CRUD：list/create/save/delete 实挂 x_cms_form（name/app_id/definition JSON 编辑、参数化写、软删、删除需确认）',
    be: [['oa4rust/crates/cms_assemble_control/src/routes.rs', '"/jaxrs/cms/assemble/control/xform/create"']],
    fe2: 'api.post(createEp, payload)',
  },
  'TemplateApp.vue': {
    acc: '模板数据驱动 CRUD：list/create/save/delete 实挂 x_cms_form_v2（name/app_id/definition JSON、参数化写、软删、删除需确认）',
    be: [['oa4rust/crates/cms_assemble_control/src/routes.rs', '"/jaxrs/templateform/create"']],
    fe2: 'api.post(createEp, payload)',
  },
  'LogViewerApp.vue': {
    acc: '日志只读查看：GET /jaxrs/log/list 实挂 x_cms_log（app_id/operation_type/operation_level/person_id/ip_address/operation_detail 检索展示；日志家族无写语义，不声明写能力）',
    be: [['oa4rust/crates/cms_assemble_control/src/routes.rs', '"/jaxrs/log/list"']],
    fe2: 'api.get(listEp)',
  },
  'PortalDictDesignerApp.vue': {
    acc: '门户字典数据驱动 CRUD：list/create/save/delete 实挂 x_portal_dict（name/appName/appData JSON 编辑、参数化写、软删、删除需确认）',
    be: [['oa4rust/crates/portal_assemble_designer/src/lib.rs', '"/jaxrs/portal/assemble/designer/dict/create"']],
    fe2: 'api.post(createEp, payload)',
  },
  'PortalPageDesignerApp.vue': {
    acc: '门户页面设计：bare list 实挂 x_portal_page + U2 类型化 create/save/delete（content JSON 校验+空值 {} 兜底防 NULL 降级、IDOR 门禁、删除需确认）',
    be: [['oa4rust/crates/portal_assemble_designer/src/lib.rs', '"/jaxrs/portal/assemble/designer/page/create"']],
    fe2: 'api.post(createEp, {',
  },
  'PortalWidgetDesignerApp.vue': {
    acc: '门户组件数据驱动 CRUD：list/create/save/delete 实挂 x_portal_widget（name/portalId/category/config JSON、参数化写、软删、删除需确认）',
    be: [['oa4rust/crates/portal_assemble_designer/src/lib.rs', '"/jaxrs/portal/assemble/designer/widget/create"']],
    fe2: 'api.post(createEp, payload)',
  },
  'ProcessDictDesignerApp.vue': {
    acc: '流程字典数据驱动 CRUD：list/create/save/delete 实挂 pp_e_applicationdict（name/application 类型化字段、参数化写、软删、删除需确认）',
    be: [['oa4rust/crates/processplatform_assemble_designer/src/routes.rs', '"/jaxrs/processplatform/assemble/designer/dict/create"']],
    fe2: 'api.post(createEp, payload)',
  },
  'ProcessFormDesignerApp.vue': {
    acc: '流程表单数据驱动 CRUD：list/create/save/delete 实挂 pp_e_form（name/application 类型化字段、参数化写、软删、删除需确认）',
    be: [['oa4rust/crates/processplatform_assemble_designer/src/routes.rs', '"/jaxrs/processplatform/assemble/designer/form/create"']],
    fe2: 'api.post(createEp, payload)',
  },
  'ProcessXformApp.vue': {
    acc: '流程 XForm 数据驱动 CRUD：list/create/save/delete 实挂迁移 097 x_pp_xform（name/flag/desc 类型化字段、参数化写、软删、删除需确认）',
    be: [
      ['oa4rust/crates/processplatform_assemble_designer/src/routes.rs', '"/jaxrs/processplatform/assemble/designer/xform/create"'],
      ['oa4rust/migrations/097_create_pp_xform_table.sql', 'CREATE TABLE IF NOT EXISTS'],
    ],
    fe2: 'api.post(createEp, payload)',
  },
  'ProcessManagerApp.vue': {
    acc: '流程定义只读管理：GET process_manager/list 实挂 x_process_definition（name/category/status/version/creator 展示+检索；定义写入口在设计器家族，不在此声明写能力）',
    be: [['oa4rust/crates/processplatform_assemble_surface/src/routes.rs', '"/jaxrs/processplatform/assemble/surface/process_manager/list"']],
    fe2: 'api.get(listEp)',
  },
  'ProcessTaskCenterApp.vue': {
    acc: '任务中心可操作闭环：task/list 实挂 x_task 全量列表 + 开始处理 POST task/processing/{id} + 催办 GET task/urge/{id} + 删除 DELETE task/{id}（已注册端点，写操作需确认）',
    be: [['oa4rust/crates/processplatform_service_processing/src/routes.rs', '"/jaxrs/processplatform/service/processing/task/list"']],
    fe2: 'api.post(`/jaxrs/processplatform/service/processing/task/processing/`',
  },
  'QueryExplorerApp.vue': {
    acc: '查询视图只读浏览：GET explorer/list 实挂 x_query_view（name/queryType 检索展示；视图写入口在设计器家族，不在此声明写能力）',
    be: [['oa4rust/crates/query_assemble_surface/src/lib.rs', '"/jaxrs/query/assemble/surface/explorer/list"']],
    fe2: 'api.get(listEp)',
  },
  'QueryImporterDesignerApp.vue': {
    acc: '导入模型数据驱动 CRUD：list/create/save/delete 实挂 x_query_import_model（name/modelFlag/queryFlag/content 类型化字段、参数化写、软删、删除需确认）',
    be: [['oa4rust/crates/query_assemble_designer/src/lib.rs', '"/jaxrs/query/assemble/designer/importer/create"']],
    fe2: 'api.post(createEp, payload)',
  },
  'QueryStatDesignerApp.vue': {
    acc: '统计定义数据驱动 CRUD：list/create/save/delete 实挂 x_query_stat（name/queryFlag/statType 类型化字段、参数化写、软删；delete 走 POST 通道、删除需确认）',
    be: [['oa4rust/crates/query_assemble_designer/src/lib.rs', '"/jaxrs/query/assemble/designer/stat/create"']],
    fe2: 'api.post(createEp, payload)',
  },
  'ThreeMemberApp.vue': {
    acc: '三方成员数据驱动 CRUD：list/create/save/delete 实挂 x_org_person（name/mobile/email/unitId/status 类型化字段、参数化写、软删、删除需确认）',
    be: [['oa4rust/crates/organization_assemble_control/src/u2_router.rs', '"/jaxrs/organization/assemble/control/threemember/create"']],
    fe2: 'api.post(createEp, payload)',
  },
  'ServiceInvokeDesignerApp.vue': {
    acc: '服务调用定义数据驱动 CRUD：list/create/save/delete 实挂 x_program_invoke（name/alias/category 类型化字段、参数化写、删除需确认）',
    be: [['oa4rust/crates/program_center/src/routes.rs', '"/jaxrs/program_center/invoke/create"']],
    fe2: 'api.post(createEp, payload)',
  },
};
const viewToListEp = {
  CmsDictDesignerApp: 'list',
  CmsFormDesignerApp: 'list',
  CmsViewDesignerApp: 'list',
  CmsXformApp: 'list',
  TemplateApp: 'list',
  LogViewerApp: 'list',
  PortalDictDesignerApp: 'list',
  PortalPageDesignerApp: 'list',
  PortalWidgetDesignerApp: 'list',
  ProcessDictDesignerApp: 'list',
  ProcessFormDesignerApp: 'list',
  ProcessXformApp: 'list',
  ProcessManagerApp: 'list',
  ProcessTaskCenterApp: 'list',
  QueryExplorerApp: 'list',
  QueryImporterDesignerApp: 'list',
  QueryStatDesignerApp: 'list',
  ThreeMemberApp: 'list',
  ServiceInvokeDesignerApp: 'list',
};
let updated = 0;
for (const e of m.entries) {
  if (e.workItem !== 'W13' || !defs[e.view]) continue;
  const d = defs[e.view];
  const viewBase = e.view.replace(/\.vue$/, '');
  const listEp = viewToListEp[viewBase]
    ? `const listEp = '/jaxrs' + '' // placeholder`
    : null;
  // 从 view 文件实际读取真实 listEp 字面量，保证 evidence.contains 真实存在
  const viewSrc = fs.readFileSync(`./oa4rust-web/apps/desktop/src/views/${e.view}`, 'utf8');
  const listEpMatch = viewSrc.match(/const listEp = '([^']+)'/);
  if (!listEpMatch) {
    console.error('NO listEp in', e.view);
    process.exitCode = 1;
    continue;
  }
  const fe2Ok = viewSrc.includes(d.fe2);
  if (!fe2Ok) {
    console.error('fe2 not found in', e.view, ':', d.fe2);
    process.exitCode = 1;
    continue;
  }
  e.status = 'implemented';
  e.replacementClaim = true;
  delete e.decision;
  delete e.blockerCodes;
  e.acceptance = d.acc;
  e.evidence = {
    frontend: [
      { file: V + e.view, contains: `const listEp = '${listEpMatch[1]}'` },
      { file: V + e.view, contains: d.fe2 },
    ],
    routing: [{ file: R, contains: `import('./views/${e.view}')` }],
    backend: d.be.map((x) => ({ file: x[0], contains: x[1] })),
  };
  updated++;
}
const counts = {};
for (const e of m.entries) counts[e.status] = (counts[e.status] || 0) + 1;
m.summary.byStatus = {
  implemented: counts.implemented || 0,
  out_of_scope: counts.out_of_scope || 0,
  still_blocked: counts.still_blocked || 0,
};
fs.writeFileSync(p, JSON.stringify(m, null, 2) + '\n');
console.log('updated', updated, JSON.stringify(counts));
