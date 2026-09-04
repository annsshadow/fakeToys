/**
 * @oa4rust/apis — 业务 API 层
 * 全部覆盖 oa4rust 3892 条路由定义
 */

import { api, type ApiResponse, type PagedResponse } from '@oa4rust/sdk';

// ─────────────────────────────────────────────────────────────
// 认证模块 (28 routes)
// ─────────────────────────────────────────────────────────────
export const authApi = {
  login: (data: { username: string; password: string; captchaId?: string; captchaAnswer?: string }) =>
    api.post<{ data: { token: string; person: import('@oa4rust/sdk').O2User } }>('/jaxrs/authentication/login', data),
  logout: () => api.post('/jaxrs/authentication/logout', null),
  who: () => api.get<{ data: import('@oa4rust/sdk').O2User }>('/jaxrs/authentication/who'),
  refresh: () => api.post<{ data: { token: string } }>('/jaxrs/authentication/refresh', null),
  captcha: () => api.get<{ data: { image: string; id: string } }>('/jaxrs/authentication/captcha'),
  captchaSize: (w: number, h: number) =>
    api.get<{ data: { image: string; id: string } }>(`/jaxrs/authentication/captcha/width/${w}/height/${h}`),
  oauthList: () => api.get('/jaxrs/authentication/oauth/list'),
  sso: (data: unknown) => api.post('/jaxrs/authentication/sso', data),
  twoFactor: (data: unknown) => api.post('/jaxrs/authentication/two_factor', data),
  switchUser: (targetUnique: string) =>
    api.post<{ data: import('@oa4rust/sdk').O2User }>('/jaxrs/authentication/switchuser', { targetUnique }),
  groupList: () => api.get('/jaxrs/authentication/group/list'),
  roleList: () => api.get('/jaxrs/authentication/role/list'),
  unitList: () => api.get('/jaxrs/authentication/unit/list'),
  checkToken: (data: { token: string }) => api.post('/jaxrs/authentication/check/token', data),
  safeLogout: () => api.post('/jaxrs/authentication/safe/logout', null),
};

// ─────────────────────────────────────────────────────────────
// 组织模块 (226 routes)
// ─────────────────────────────────────────────────────────────
export const orgApi = {
  // 部门树
  groupList: (flag?: string, count?: number) =>
    api.get(`/jaxrs/organization/assemble/control/group/list/${flag || ''}/next/${count || 20}`),
  groupDetail: (flag: string) => api.get(`/jaxrs/organization/assemble/control/group/${flag}`),
  groupSub: (flag: string) => api.get(`/jaxrs/organization/assemble/control/group/${flag}/sub/nested`),
  groupSup: (flag: string) => api.get(`/jaxrs/organization/assemble/control/group/${flag}/sup/nested`),
  groupAddMember: (flag: string, data: unknown) =>
    api.post(`/jaxrs/organization/assemble/control/group/${flag}/add/member`, data),
  groupDeleteMember: (flag: string, data: unknown) =>
    api.post(`/jaxrs/organization/assemble/control/group/${flag}/delete/member`, data),

  // 人员
  personList: (page: number, size: number, keyword?: string) =>
    api.post<PagedResponse<import('@oa4rust/sdk').O2User>>(
      `/jaxrs/organization/assemble/control/person/list/paging/${page}/${size}`,
      { keyword },
    ),
  personDetail: (id: string) => api.get(`/jaxrs/organization/assemble/control/person/${id}`),
  personCreate: (data: unknown) => api.post('/jaxrs/organization/assemble/control/person', data),
  personUpdate: (id: string, data: unknown) => api.put(`/jaxrs/organization/assemble/control/person/${id}`, data),
  personDelete: (id: string) => api.delete(`/jaxrs/organization/assemble/control/person/${id}`),

  // 身份
  identityList: (flag?: string) => api.get(`/jaxrs/organization/assemble/control/identity/list/${flag || ''}`),
  identityDetail: (flag: string) => api.get(`/jaxrs/organization/assemble/control/identity/${flag}`),

  // 搜索
  groupSearch: (keyword: string) =>
    api.get('/jaxrs/organization/assemble/control/group/list/like', { params: { keyword } }),
  personSearch: (keyword: string) =>
    api.get('/jaxrs/organization/assemble/control/person/list/like', { params: { keyword } }),
  identitySearch: (keyword: string) =>
    api.get('/jaxrs/organization/assemble/control/identity/list/like', { params: { keyword } }),

  // 导出
  exportAll: () => api.post('/jaxrs/organization/assemble/control/export/export/all', null),
  exportResult: (flag: string) => api.get(`/jaxrs/organization/assemble/control/export/result/flag/${flag}`),

  // 授权
  empowerList: (personId: string) => api.get(`/jaxrs/person/empower/list/${personId}`),
  empowerCreate: (data: unknown) => api.post('/jaxrs/person/empower', data),
  empowerUpdate: (id: string, data: unknown) => api.put(`/jaxrs/person/empower/${id}`, data),
  empowerDelete: (id: string) => api.delete(`/jaxrs/person/empower/${id}`),
  empowerEnable: (id: string) => api.post(`/jaxrs/person/empower/${id}/enable`),
  empowerDisable: (id: string) => api.post(`/jaxrs/person/empower/${id}/disable`),
};

// ─────────────────────────────────────────────────────────────
// 工作流模块 (1600+ routes)
// ─────────────────────────────────────────────────────────────
export const processApi = {
  // 工作表面（待办/审批）
  workList: (page: number, size: number, status?: string) =>
    api.post<PagedResponse<unknown>>(`/jaxrs/processplatform/assemble/surface/work/list/paging/${page}/${size}`, { status }),
  workDetail: (id: string) => api.get(`/jaxrs/processplatform/assemble/surface/work/${id}`),
  workStart: (data: unknown) => api.post('/jaxrs/processplatform/assemble/surface/work/start', data),
  taskList: (page: number, size: number) =>
    api.post<PagedResponse<unknown>>(`/jaxrs/processplatform/assemble/surface/task/list/paging/${page}/${size}`),
  taskHandle: (taskId: string, action: string, data?: Record<string, unknown>) =>
    api.post(`/jaxrs/processplatform/assemble/surface/task/${taskId}/handle`, { action, ...data }),
  dataList: (page: number, size: number, processId?: string) =>
    api.post<PagedResponse<unknown>>(`/jaxrs/processplatform/assemble/surface/data/list/paging/${page}/${size}`, { processId }),
  attachmentList: (workId: string) => api.get(`/jaxrs/processplatform/assemble/surface/attachment/list/${workId}`),
  reviewList: (workId: string) => api.get(`/jaxrs/processplatform/assemble/surface/review/list/${workId}`),
  formView: (workId: string) => api.get(`/jaxrs/processplatform/assemble/surface/form/view/${workId}`),
  snapView: (workId: string) => api.get(`/jaxrs/processplatform/assemble/surface/snap/${workId}`),
  applicationDict: (flag: string) => api.get(`/jaxrs/processplatform/assemble/surface/applicationdict/${flag}`),

  // 已办
  completedList: (page: number, size: number) =>
    api.post<PagedResponse<unknown>>(`/jaxrs/processplatform/assemble/surface/workcompleted/list/paging/${page}/${size}`),
  readList: (page: number, size: number) =>
    api.post<PagedResponse<unknown>>(`/jaxrs/processplatform/assemble/surface/read/list/paging/${page}/${size}`),

  // 流程设计器
  processList: (page: number, size: number) =>
    api.post<PagedResponse<unknown>>(`/jaxrs/processplatform/assemble/designer/process/list/paging/${page}/${size}`),
  processCreate: (data: unknown) => api.post('/jaxrs/processplatform/assemble/designer/process', data),
  processUpdate: (id: string, data: unknown) => api.put(`/jaxrs/processplatform/assemble/designer/process/${id}`, data),
  processDelete: (id: string) => api.delete(`/jaxrs/processplatform/assemble/designer/process/${id}`),
  processExport: (id: string) => api.get(`/jaxrs/processplatform/assemble/designer/process/${id}/export`),

  // BAM 监控
  bamPeriod: (startTime: string, endTime: string) =>
    api.post('/jaxrs/processplatform/assemble/bam/period', { startTime, endTime }),
  bamTrace: (workId: string) => api.get(`/jaxrs/processplatform/assemble/bam/trace/${workId}`),

  // 服务处理
  serviceWorkList: (page: number, size: number) =>
    api.post<PagedResponse<unknown>>(`/jaxrs/processplatform/service/processing/work/list/paging/${page}/${size}`),
};

// ─────────────────────────────────────────────────────────────
// 门户模块 (125 routes)
// ─────────────────────────────────────────────────────────────
export const portalApi = {
  pageList: (appId: string) => api.get(`/jaxrs/portal/assemble/surface/page/list/${appId}`),
  pageDetail: (pageId: string) => api.get(`/jaxrs/portal/assemble/surface/page/${pageId}`),
  pageCreate: (data: unknown) => api.post('/jaxrs/portal/assemble/surface/page', data),
  pageUpdate: (id: string, data: unknown) => api.put(`/jaxrs/portal/assemble/surface/page/${id}`, data),
  pageDelete: (id: string) => api.delete(`/jaxrs/portal/assemble/surface/page/${id}`),
  widgetList: (pageId: string) => api.get(`/jaxrs/portal/assemble/surface/widget/list/${pageId}`),
  designerPageList: () => api.get('/jaxrs/portal/assemble/designer/page/list'),
  designerScriptList: () => api.get('/jaxrs/portal/assemble/designer/script/list'),
};

// ─────────────────────────────────────────────────────────────
// 即时通讯 (64 routes)
// ─────────────────────────────────────────────────────────────
export const messageApi = {
  conversationList: (page: number, size: number) =>
    api.post<PagedResponse<unknown>>(`/jaxrs/message/assemble/communicate/conversation/list/paging/${page}/${size}`),
  msgHistory: (conversationId: string, page: number, size: number) =>
    api.post<PagedResponse<unknown>>(
      `/jaxrs/message/assemble/communicate/history/${conversationId}/paging/${page}/${size}`,
    ),
  msgSend: (data: { conversationId: string; content: string; type: string }) =>
    api.post('/jaxrs/message/assemble/communicate/im/send', data),
  collectionList: (page: number, size: number) =>
    api.post<PagedResponse<unknown>>(`/jaxrs/message/assemble/communicate/collection/list/paging/${page}/${size}`),
  markRead: (conversationId: string) =>
    api.post(`/jaxrs/message/assemble/communicate/conversation/${conversationId}/read`, null),
};

// ─────────────────────────────────────────────────────────────
// 文件模块 (32 routes)
// ─────────────────────────────────────────────────────────────
export const fileApi = {
  fileList: (folderId?: string, page?: number, size?: number) =>
    api.post<PagedResponse<unknown>>(`/jaxrs/file/assemble/control/file/list`, { folderId, page, size }),
  folderList: (parentId?: string) =>
    api.get(`/jaxrs/file/assemble/control/folder/list/${parentId || ''}`),
  fileUpload: (formData: FormData) => api.upload('/jaxrs/file/assemble/control/file/upload', formData),
  fileDownload: (fileId: string) => window.open(`/jaxrs/file/core/entity/file/${fileId}/download`),
  fileDelete: (fileId: string) => api.delete(`/jaxrs/file/assemble/control/file/${fileId}`),
  fileShare: (fileId: string, data: unknown) => api.post(`/jaxrs/file/assemble/control/file/${fileId}/share`, data),
  attachmentList: (fileId: string) => api.get(`/jaxrs/file/assemble/control/attachment/list/${fileId}`),
};

// ─────────────────────────────────────────────────────────────
// 通用 API (字典/发票/工时等)
// ─────────────────────────────────────────────────────────────
export const generalApi = {
  dictList: () => api.get('/jaxrs/general/dict/list'),
  dictCreate: (data: unknown) => api.post('/jaxrs/general/dict', data),
  dictUpdate: (id: string, data: unknown) => api.put(`/jaxrs/general/dict/${id}`, data),
  dictDelete: (id: string) => api.delete(`/jaxrs/general/dict/${id}`),
  dictItemList: (dictId: string) => api.get(`/jaxrs/general/dict/item/list/${dictId}`),
  dictItemCreate: (dictId: string, data: unknown) => api.post(`/jaxrs/general/dict/item/${dictId}`, data),
  fileUpload: (formData: FormData) => api.upload('/jaxrs/general/file', formData),
  invoiceList: (page: number, size: number) =>
    api.post<PagedResponse<unknown>>(`/jaxrs/general/invoice/list/paging/${page}/${size}`),
  worktimeList: (month: string) => api.get(`/jaxrs/general/assemble/control/worktime/${month}`),
};

// ─────────────────────────────────────────────────────────────
// 导出所有 API
// ─────────────────────────────────────────────────────────────
export const apis = {
  auth: authApi,
  org: orgApi,
  process: processApi,
  portal: portalApi,
  message: messageApi,
  file: fileApi,
  general: generalApi,
};

export default apis;

// IM API
export const imApi = {
  conversationList: () => api.get('/jaxrs/message/assemble/communicate/im/conversation/list/my'),
  messageList: (p: number, s: number, cid?: string) => api.post(`/jaxrs/message/assemble/communicate/im/msg/list/${p}/${s}`, { conversationId: cid }),
  messageSend: (d: unknown) => api.post('/jaxrs/message/assemble/communicate/im/msg', d),
  markRead: (id: string) => api.post(`/jaxrs/message/assemble/communicate/mark_read/${id}`, null),
  unreadCount: (c?: string) => api.get(`/jaxrs/message/unread/count/${c || 'im'}`),
};

// ─────────────────────────────────────────────────────────────
// 扩展模块 API
// ─────────────────────────────────────────────────────────────

/** 应用信息 */
export const appInfoApi = {
  list: () => api.get('/jaxrs/appinfo/list'),
  detail: (id: string) => api.get(`/jaxrs/appinfo/${id}`),
  filter: (keyword: string) => api.get('/jaxrs/appinfo/filter', { params: { keyword } }),
};

/** 分类信息 */
export const categoryApi = {
  list: () => api.get('/jaxrs/categoryinfo/list'),
};

/** 热帖管理 */
export const hotpicApi = {
  list: () => api.get('/jaxrs/hotpic/core/entity/list'),
  create: (data: unknown) => api.post('/jaxrs/hotpic/core/entity/create', data),
  delete: (id: string) => api.delete(`/jaxrs/hotpic/core/entity/delete/${id}`),
  listByApp: (application: string, infoId: string) =>
    api.get(`/jaxrs/hotpic/core/entity/list/by/${application}/${infoId}`),
};

/** JPush 推送 */
export const jpushApi = {
  deviceList: () => api.get('/jaxrs/jpush/core/entity/device/list'),
  deviceCreate: (data: unknown) => api.post('/jaxrs/jpush/core/entity/device/create', data),
  deviceDelete: (id: string) => api.delete(`/jaxrs/jpush/core/entity/device/${id}`),
  templateList: () => api.get('/jaxrs/jpush/core/entity/template/list'),
  templateUpdate: (id: string, data: unknown) => api.put(`/jaxrs/jpush/core/entity/template/${id}`, data),
  assembleDeviceList: () => api.get('/jaxrs/jpush_assemble_control/device/list'),
  assembleTemplateList: () => api.get('/jaxrs/jpush_assemble_control/template/list'),
};

/** 关联处理 */
export const correlationApi = {
  list: () => api.get('/jaxrs/correlation/core/entity/list'),
  create: (data: unknown) => api.post('/jaxrs/correlation/core/entity/create', data),
  delete: (id: string) => api.delete(`/jaxrs/correlation/core/entity/delete/${id}`),
  getStatus: () => api.get('/jaxrs/correlation/core/express/status'),
  sync: () => api.post('/jaxrs/correlation/core/express/sync', null),
  serviceList: (personId: string) => api.get(`/jaxrs/correlation/service/processing/list/${personId}`),
  serviceCreate: (data: unknown) => api.post('/jaxrs/correlation/service/processing/create', data),
  serviceDelete: (id: string) => api.delete(`/jaxrs/correlation/service/processing/delete/${id}`),
  serviceGet: (id: string) => api.get(`/jaxrs/correlation/service/processing/${id}`),
  serviceSave: (id: string, data: unknown) => api.put(`/jaxrs/correlation/service/processing/save/${id}`, data),
  link: (data: unknown) => api.post('/jaxrs/correlation/service/processing/link', data),
  unlink: (sourceType: string, sourceId: string, targetType: string, targetId: string) =>
    api.delete(`/jaxrs/correlation/service/processing/unlink/${sourceType}/${sourceId}/${targetType}/${targetId}`),
};

/** 分享管理 */
export const shareApi = {
  list: () => api.get('/jaxrs/share/list'),
};

/** 缓存管理 */
export const cacheApi = {
  detail: () => api.get('/jaxrs/cache/detail'),
  flushCommonScript: () => api.post('/jaxrs/cache/commonscript/flush', null),
  flushConfig: () => api.post('/jaxrs/cache/config/flush', null),
};

/** 系统资源 */
export const sysResourceApi = {
  list: () => api.get('/jaxrs/sysresource/list'),
};

/** 日志查看 */
export const logApi = {
  list: () => api.get('/jaxrs/log/list'),
};

/** 控制台 */
export const consoleApi = {
  list: () => api.get('/jaxrs/console/list'),
};

/** 导出 */
export const exportApi = {
  result: (flag: string) => api.get(`/jaxrs/export/result/flag/${flag}`),
};

/** 导入 */
export const importApi = {
  execute: (id: string) => api.post(`/jaxrs/importmodel/id/${id}/execute`, null),
};

/** 附件 */
export const attachmentApi = {
  list: (id: string) => api.get(`/jaxrs/attachment2/list/${id}`),
  upload: (formData: FormData) => api.upload('/jaxrs/attachment2/upload', formData),
  delete: (id: string) => api.delete(`/jaxrs/attachment2/${id}`),
};

/** 匿名访问 */
export const anonymousApi = {
  surfaceAppdict: (flag: string) => api.get(`/jaxrs/anonymous/surface/appdict/${flag}`),
};

/** 数据文档 */
export const dataApi = {
  documentDetail: (id: string, path0?: string) =>
    api.get(path0 ? `/jaxrs/data/document/${id}/${path0}` : `/jaxrs/data/document/${id}`),
};

/** 更新 apis 对象 */
const additionalApis = {
  appInfo: appInfoApi,
  category: categoryApi,
  hotpic: hotpicApi,
  jpush: jpushApi,
  correlation: correlationApi,
  share: shareApi,
  cache: cacheApi,
  sysResource: sysResourceApi,
  log: logApi,
  console: consoleApi,
  exportApi,
  importApi,
  attachment: attachmentApi,
  anonymous: anonymousApi,
  data: dataApi,
};

// Merge into apis export
Object.assign(apis, additionalApis);

// ─────────────────────────────────────────────────────────────
// 工作流深化 (processplatform service/processing)
// ─────────────────────────────────────────────────────────────
export const processServiceApi = {
  taskList: (page: number, size: number) =>
    api.post(`/jaxrs/processplatform/service/processing/task/list/paging/${page}/${size}`, {}),
  workList: (page: number, size: number) =>
    api.post(`/jaxrs/processplatform/service/processing/work/list/paging/${page}/${size}`, {}),
  applicationDict: (flag: string) =>
    api.get(`/jaxrs/processplatform/service/processing/applicationdict/${flag}`),
};

// ─────────────────────────────────────────────────────────────
// 查询视图 (queryview — 119 routes)
// ─────────────────────────────────────────────────────────────
export const queryViewApi = {
  list: (queryFlag: string) => api.get(`/jaxrs/queryview/list/${queryFlag}`),
  listAll: () => api.get('/jaxrs/queryview/list'),
  execute: (view: string, params?: Record<string, string>) =>
    api.post(`/jaxrs/queryview/execute/${view}`, params ?? {}),
  executeV2: (view: string, params?: unknown) =>
    api.post(`/jaxrs/queryview/execute/v2/${view}`, params),
  bundle: (view: string) => api.get(`/jaxrs/queryview/bundle/${view}`),
  bundleV2: (view: string) => api.post(`/jaxrs/queryview/bundle/v2/${view}`, {}),
  excel: (view: string) => api.get(`/jaxrs/queryview/excel/${view}`),
  flag: (viewFlag: string) => api.get(`/jaxrs/queryview/flag/${viewFlag}`),
  importModelList: (query: string, flag: string) =>
    api.get(`/jaxrs/queryview/importmodel/list/${query}/${flag}`),
  importModelExecute: (data: unknown) =>
    api.post('/jaxrs/queryview/importmodel/execute', data),
  moreLikeThis: (params: unknown) =>
    api.post('/jaxrs/queryview/morelikethis', params),
  neural: (params: unknown) =>
    api.post('/jaxrs/queryview/neural', params),
  search: (keyword: string) =>
    api.get('/jaxrs/queryview/search', { params: { keyword } }),
  stat: (params: unknown) =>
    api.post('/jaxrs/queryview/stat', params),
  statement: (params: unknown) =>
    api.post('/jaxrs/queryview/statement', params),
  tableList: (page: number, size: number) =>
    api.post(`/jaxrs/queryview/table/list/paging/${page}/${size}`, {}),
  tableRow: (flag: string, rowId: string) =>
    api.get(`/jaxrs/queryview/table/row/${flag}/${rowId}`),
  viewDetail: (id: string) => api.get(`/jaxrs/queryview/view/${id}`),
  viewList: (page: number, size: number) =>
    api.post(`/jaxrs/queryview/view/list/paging/${page}/${size}`, {}),
};

// ─────────────────────────────────────────────────────────────
// 程序中心 (program_center — 319 routes)
// ─────────────────────────────────────────────────────────────
export const programCenterApi = {
  // Agent 管理
  agentList: () => api.get('/jaxrs/program_center/agent/list'),
  agentCreate: (data: unknown) => api.post('/jaxrs/program_center/agent/create', data),
  agentGet: (flag: string) => api.get(`/jaxrs/program_center/agent/${flag}`),
  agentSave: (id: string, data: unknown) => api.put(`/jaxrs/program_center/agent/save/${id}`, data),
  agentEnable: (flag: string) => api.post(`/jaxrs/program_center/agent/${flag}/enable`, null),
  agentDisable: (flag: string) => api.post(`/jaxrs/program_center/agent/${flag}/disable`, null),
  // Application 管理
  appList: () => api.get('/jaxrs/program_center/application/list'),
  appCreate: (data: unknown) => api.post('/jaxrs/program_center/application/create', data),
  appSave: (id: string, data: unknown) => api.put(`/jaxrs/program_center/application/save/${id}`, data),
  // AppStyle 样式
  appStyleCurrent: () => api.get('/jaxrs/program_center/appstyle/current'),
  appStyleUpdate: (data: unknown) => api.put('/jaxrs/program_center/appstyle/current/update', data),
  appStyleImage: (appId: string) => api.get(`/jaxrs/program_center/appstyle/image/application/top/${appId}`),
  // Script 脚本
  scriptList: () => api.get('/jaxrs/program_center/script/list'),
  scriptGet: (flag: string) => api.get(`/jaxrs/program_center/script/${flag}`),
  // Dict 字典
  dictList: () => api.get('/jaxrs/program_center/dict/list'),
  dictCreate: (data: unknown) => api.post('/jaxrs/program_center/dict/create', data),
  // Config 配置
  configList: () => api.get('/jaxrs/program_center/config/list'),
  configGet: (key: string) => api.get(`/jaxrs/program_center/config/${key}`),
  // Market 市场
  marketList: (page: number, size: number) =>
    api.post(`/jaxrs/program_center/market/list/paging/${page}/${size}`, {}),
  marketGet: (flag: string) => api.get(`/jaxrs/program_center/market/${flag}`),
  // Module 模块
  moduleList: () => api.get('/jaxrs/program_center/module/list'),
  moduleInvoke: (flag: string, data: unknown) =>
    api.post(`/jaxrs/program_center/invoke/${flag}`, data),
  // Schedule 计划任务
  scheduleList: () => api.get('/jaxrs/program_center/schedule/list'),
  scheduleCreate: (data: unknown) => api.post('/jaxrs/program_center/schedule/create', data),
  // Deploy 部署
  deploy: (data: unknown) => api.post('/jaxrs/program_center/deploy', data),
  // Code 代码
  codeList: () => api.get('/jaxrs/program_center/code/list'),
  // Error Logs
  promptErrorLogList: (page: number, size: number) =>
    api.post(`/jaxrs/program_center/prompterrorlog/list/paging/${page}/${size}`, {}),
  unexpectedErrorLogList: (page: number, size: number) =>
    api.post(`/jaxrs/program_center/unexpectederrorlog/list/paging/${page}/${size}`, {}),
  warnLogList: (page: number, size: number) =>
    api.post(`/jaxrs/program_center/warnlog/list/paging/${page}/${size}`, {}),
  // Captcha
  captchaList: () => api.get('/jaxrs/program_center/captcha/list'),
  captchaCreate: (data: unknown) => api.post('/jaxrs/program_center/captcha/create', data),
  captchaGet: (id: string) => api.get(`/jaxrs/program_center/captcha/${id}`),
  // Data Structure
  dataStructureList: () => api.get('/jaxrs/program_center/datastructure/list'),
  // SSO callbacks
  andfxPull: (data: unknown) => api.post('/jaxrs/program_center/andfx/pull', data),
  dingdingCode: (code: string) => api.get(`/jaxrs/program_center/dingding/code/${code}`),
  mpweixinMenu: () => api.get('/jaxrs/program_center/mpweixin/menu'),
  qywxCode: (code: string) => api.get(`/jaxrs/program_center/qiyeweixin/code/${code}`),
  welinkCode: (code: string) => api.get(`/jaxrs/program_center/welink/code/${code}`),
  zwdCode: (code: string) => api.get(`/jaxrs/program_center/zhengwudingding/code/${code}`),
};

// ─────────────────────────────────────────────────────────────
// 思维导图 (mind — 40 routes)
// ─────────────────────────────────────────────────────────────
export const mindApi = {
  folderTree: () => api.get('/jaxrs/mind/folder/tree/my'),
  folderList: () => api.get('/jaxrs/mind/folder/list'),
  folderDetail: (id: string) => api.get(`/jaxrs/mind/folder/${id}`),
  folderSave: (data: unknown) => api.post('/jaxrs/mind/assemble/control/folder/save', data),
  folderMove: (folderId: string, data: unknown) =>
    api.post(`/jaxrs/mind/assemble/control/folder/move/${folderId}`, data),
  mindList: (id: string, page: number) =>
    api.get(`/jaxrs/mind/mind/list/${id}/${page}`),
  mindDetail: (id: string) => api.get(`/jaxrs/mind/mind/${id}`),
  mindSave: (data: unknown) => api.post('/jaxrs/mind/assemble/control/mind/save', data),
  mindVersionList: (id: string) =>
    api.get(`/jaxrs/mind/mind/list/${id}/version`),
  mindVersion: (id: string, version: string) =>
    api.get(`/jaxrs/mind/mind/version/${id}/${version}`),
  mindFilterList: (id: string, page: number) =>
    api.get(`/jaxrs/mind/assemble/control/mind/filter/list/${id}/next/${page}`),
  mindRecycled: (id: string, page: number) =>
    api.get(`/jaxrs/mind/assemble/control/mind/filter/recycle/${id}/next/${page}`),
  mindRestore: (id: string) =>
    api.post(`/jaxrs/mind/assemble/control/mind/restore/${id}`, null),
  mindDelete: (id: string) =>
    api.delete(`/jaxrs/mind/assemble/control/mind/${id}/destorymind`),
  mindIcon: (id: string, size: number) =>
    api.get(`/jaxrs/mind/assemble/control/mind/${id}/icon/size/${size}`),
  mindShare: (id: string) =>
    api.post(`/jaxrs/mind/assemble/control/mind/share/${id}`, null),
  mindUnshare: (id: string) =>
    api.post(`/jaxrs/mind/assemble/control/mind/share/${id}/cancel`, null),
  mindView: (id: string) => api.get(`/jaxrs/mind/assemble/control/mind/view/${id}`),
  config: () => api.get('/jaxrs/mind/assemble/control/config'),
  configUpdate: (data: unknown) => api.put('/jaxrs/mind/assemble/control/config/update', data),
};

// ─────────────────────────────────────────────────────────────
// 文档管理 (document)
// ─────────────────────────────────────────────────────────────
export const documentApi = {
  list: (params?: Record<string, string>) => api.get('/jaxrs/document/list', { params }),
  detail: (id: string) => api.get(`/jaxrs/document/${id}`),
  create: (data: unknown) => api.post('/jaxrs/document/document', data),
  update: (id: string, data: unknown) => api.put(`/jaxrs/document/document/${id}`, data),
  delete: (id: string) => api.delete(`/jaxrs/document/${id}`),
  draftList: (page: number, size: number) =>
    api.post(`/jaxrs/document/draft/list/paging/${page}/${size}`, {}),
  filterList: (params: unknown) => api.post('/jaxrs/document/filter/list', params),
  batch: (data: unknown) => api.post('/jaxrs/document/batch', data),
  categoryList: () => api.get('/jaxrs/document/category/list'),
  cipherList: (id: string) => api.get(`/jaxrs/document/cipher/list/${id}`),
  publish: (id: string) => api.post(`/jaxrs/document/publish/${id}`, null),
  achive: (id: string) => api.post(`/jaxrs/document/achive/${id}`, null),
};

// ─────────────────────────────────────────────────────────────
// 日历深化 (calendar_assemble_control)
// ─────────────────────────────────────────────────────────────
export const calendarDeepApi = {
  calendarList: (params?: Record<string, string>) =>
    api.get('/jaxrs/calendar_assemble_control/calendar/list', { params }),
  calendarDetail: (id: string) => api.get(`/jaxrs/calendar_assemble_control/calendar/${id}`),
  calendarCreate: (data: unknown) => api.post('/jaxrs/calendar_assemble_control/calendar', data),
  calendarUpdate: (id: string, data: unknown) => api.put(`/jaxrs/calendar_assemble_control/calendar/${id}`, data),
  calendarDelete: (id: string) => api.delete(`/jaxrs/calendar_assemble_control/calendar/${id}`),
  eventList: (id: string, page: number, size: number) =>
    api.post(`/jaxrs/calendar_assemble_control/event/list/${id}/paging/${page}/${size}`, {}),
  eventCreate: (data: unknown) => api.post('/jaxrs/calendar_assemble_control/event', data),
  eventUpdate: (id: string, data: unknown) => api.put(`/jaxrs/calendar_assemble_control/event/${id}`, data),
  eventDelete: (id: string) => api.delete(`/jaxrs/calendar_assemble_control/event/${id}`),
  eventSingle: (id: string) => api.get(`/jaxrs/calendar_assemble_control/event/single/${id}`),
  eventAfter: (id: string) => api.get(`/jaxrs/calendar_assemble_control/event/after/${id}`),
  eventAll: (id: string) => api.get(`/jaxrs/calendar_assemble_control/event/all/${id}`),
  follow: (id: string) => api.post(`/jaxrs/calendar_assemble_control/calendar/follow/${id}`, null),
  followCancel: (id: string) => api.post(`/jaxrs/calendar_assemble_control/calendar/follow/${id}/cancel`, null),
  isManager: (id: string) => api.get(`/jaxrs/calendar_assemble_control/calendar/ismanager/${id}`),
  rfc: (id: string) => api.get(`/jaxrs/calendar_assemble_control/event/rfc/${id}`),
  setting: () => api.get('/jaxrs/calendar_assemble_control/setting'),
  settingUpdate: (data: unknown) => api.put('/jaxrs/calendar_assemble_control/setting', data),
  messageList: () => api.get('/jaxrs/calendar_assemble_control/message/list'),
};

// ─────────────────────────────────────────────────────────────
// 考勤深化
// ─────────────────────────────────────────────────────────────
export const attendanceDeepApi = {
  adminList: () => api.get('/jaxrs/attendance/assemble/control/attendanceadmin/list/all'),
  adminGet: (id: string) => api.get(`/jaxrs/attendance/assemble/control/attendanceadmin/${id}`),
  adminUpdate: (id: string, data: unknown) => api.put(`/jaxrs/attendance/assemble/control/attendanceadmin/${id}`, data),
  appealList: () => api.get('/jaxrs/attendance/appeal/list'),
  appealSubmit: (data: unknown) => api.post('/jaxrs/attendance/appeal/submit', data),
  appealArchive: (id: string) => api.post(`/jaxrs/attendance/appeal/archive/${id}`, null),
  appealAudit: (id: string, data: unknown) => api.put(`/jaxrs/attendance/appeal/audit/${id}`, data),
  detailAnalyse: (startDate: string, endDate: string) =>
    api.get(`/jaxrs/attendance/assemble/control/attendancedetail/analyse/${startDate}/${endDate}`),
  detailArchive: (id: string) => api.post(`/jaxrs/attendance/assemble/control/attendancedetail/archive/${id}`, null),
  detailCheck: (cycleYear: number, cycleMonth: number) =>
    api.get(`/jaxrs/attendance/assemble/control/attendancedetail/checkDetailWithPersonByCycle/${cycleYear}/${cycleMonth}`),
  statisticalList: (params: unknown) => api.post('/jaxrs/attendance/statistical/list', params),
  ruleList: () => api.get('/jaxrs/attendance/rule/list'),
  employeeList: (params: unknown) => api.post('/jaxrs/attendance/employee/list', params),
};

// ─────────────────────────────────────────────────────────────
// 查询设计器深化
// ─────────────────────────────────────────────────────────────
export const queryDesignerApi = {
  create: (data: unknown) => api.post('/jaxrs/query/assemble/designer/create', data),
  get: (id: string) => api.get(`/jaxrs/query/assemble/designer/get/${id}`),
  delete: (id: string) => api.delete(`/jaxrs/query/assemble/designer/delete/${id}`),
  search: (keyword: string) => api.get('/jaxrs/query/assemble/designer/designer/search', { params: { keyword } }),
  entityProperties: (query: string, category: string, entityCategory: string) =>
    api.get(`/jaxrs/query/assemble/designer/entity/entity/properties/${query}/${category}/${entityCategory}`),
  iconGet: (query: string, flag: string) =>
    api.get(`/jaxrs/query/assemble/designer/icon/${query}/${flag}`),
  iconSet: (query: string, flag: string, data: unknown) =>
    api.put(`/jaxrs/query/assemble/designer/icon/set/${query}/${flag}`, data),
  idCount: (count: number) => api.get(`/jaxrs/query/assemble/designer/id/${count}`),
  importModel: (data: unknown) => api.post('/jaxrs/query/assemble/designer/importmodel', data),
  importModelList: (query: string, flag: string) =>
    api.get(`/jaxrs/query/assemble/designer/importmodel/list/${query}/${flag}`),
  importModelEdit: (id: string, data: unknown) => api.put(`/jaxrs/query/assemble/designer/importmodel/edit/${id}`, data),
  importModelDelete: (id: string) => api.delete(`/jaxrs/query/assemble/designer/importmodel/delete/${id}`),
  bundle: (view: string, id: string) => api.get(`/jaxrs/query/assemble/designer/bundle/${view}/${id}`),
  surfaceList: () => api.get('/jaxrs/query/assemble/surface/list'),
};

// ─────────────────────────────────────────────────────────────
// 附件深化
// ─────────────────────────────────────────────────────────────
export const attachmentDeepApi = {
  list: (id: string) => api.get(`/jaxrs/attachment/list/${id}`),
  download: (id: string) => api.get(`/jaxrs/attachment/download/${id}`),
  upload: (formData: FormData) => api.upload('/jaxrs/attachment/upload', formData),
  update: (id: string, data: unknown) => api.put(`/jaxrs/attachment/update/${id}`, data),
  exist: (id: string) => api.get(`/jaxrs/attachment2/exist/${id}`),
  upload2: (formData: FormData) => api.upload('/jaxrs/attachment2/upload', formData),
  userFiles: () => api.get('/jaxrs/attachment2/user'),
};

// ─────────────────────────────────────────────────────────────
// 回收站
// ─────────────────────────────────────────────────────────────
export const recycleApi = {
  list: () => api.get('/jaxrs/recycle/list'),
  delete: (id: string) => api.delete(`/jaxrs/recycle/${id}`),
  empty: () => api.post('/jaxrs/recycle/empty', null),
  resume: (id: string) => api.post(`/jaxrs/recycle/resume/${id}`, null),
};

// ─────────────────────────────────────────────────────────────
// 服务器管理
// ─────────────────────────────────────────────────────────────
export const serverApi = {
  execute: (command: string) => api.post('/jaxrs/server/execute', { command }),
  license: () => api.get('/jaxrs/server/license'),
  stop: () => api.post('/jaxrs/server/stop', null),
};

// ─────────────────────────────────────────────────────────────
// 单元管理
// ─────────────────────────────────────────────────────────────
export const unitApi = {
  list: () => api.get('/jaxrs/unit/list'),
  check: (flag: string) => api.get(`/jaxrs/unit/check/${flag}`),
  identity: (flag: string) => api.get(`/jaxrs/unit/identity/${flag}`),
};

// ─────────────────────────────────────────────────────────────
// 表单管理
// ─────────────────────────────────────────────────────────────
export const formApi = {
  list: (params?: Record<string, string>) => api.get('/jaxrs/form/list', { params }),
  filter: (params: unknown) => api.post('/jaxrs/form/filter', params),
  detail: (id: string) => api.get(`/jaxrs/form/${id}`),
  v2List: () => api.get('/jaxrs/form/v2/list'),
};

// ─────────────────────────────────────────────────────────────
// 视图管理
// ─────────────────────────────────────────────────────────────
export const viewApi = {
  list: () => api.get('/jaxrs/view/list'),
  detail: (id: string) => api.get(`/jaxrs/view/${id}`),
  viewData: (id: string, params?: unknown) =>
    api.post(`/jaxrs/view/viewdata/${id}`, params ?? {}),
};

// ─────────────────────────────────────────────────────────────
// 文件信息
// ─────────────────────────────────────────────────────────────
export const fileInfoApi = {
  list: () => api.get('/jaxrs/fileinfo/list/all'),
  listByDoc: (docId: string) => api.get(`/jaxrs/fileinfo/list/document/${docId}`),
  listFilter: (params: unknown) => api.post('/jaxrs/fileinfo/list/filter', params),
  detail: (id: string) => api.get(`/jaxrs/fileinfo/${id}`),
  download: (id: string) => api.get(`/jaxrs/fileinfo/download/document/${id}`),
  edit: (id: string, docId: string, data: unknown) =>
    api.put(`/jaxrs/fileinfo/edit/${id}/doc/${docId}`, data),
  copy: (docId: string) => api.post(`/jaxrs/fileinfo/copy/to/doc/${docId}`, null),
  replace: (docId: string) => api.post(`/jaxrs/fileinfo/replace/to/doc/${docId}`, null),
  upload: (formData: FormData, docId: string) =>
    api.upload(`/jaxrs/fileinfo/upload/document/${docId}`, formData),
  updateContent: (id: string, data: unknown) =>
    api.put(`/jaxrs/fileinfo/update/${id}/content`, data),
  batchDownload: (docId: string, site: string) =>
    api.get(`/jaxrs/fileinfo/batch/download/doc/${docId}/site/${site}`),
};

// ─────────────────────────────────────────────────────────────
// 授权日志 / 推荐 / 评论
// ─────────────────────────────────────────────────────────────
export const empowerLogApi = {
  list: (p?: Record<string, string>) => api.get('/jaxrs/empowerlog/list', { params: p }),
};
export const commendApi = {
  list: () => api.get('/jaxrs/commend/list'),
  detail: (id: string) => api.get(`/jaxrs/commend/${id}`),
};
export const commentApi = {
  list: () => api.get('/jaxrs/comment/list'),
  detail: (id: string) => api.get(`/jaxrs/comment/${id}`),
};
export const complexApi = {
  folderList: () => api.get('/jaxrs/complex/folder/list'),
  topFiles: () => api.get('/jaxrs/complex/top'),
};
export const componentApi = {
  list: () => api.get('/jaxrs/component_assemble_control/component/list'),
  create: (data: unknown) => api.post('/jaxrs/component_assemble_control/component/create', data),
  get: (id: string) => api.get(`/jaxrs/component_assemble_control/component/${id}`),
  save: (id: string, data: unknown) => api.put(`/jaxrs/component_assemble_control/component/save/${id}`, data),
  delete: (id: string) => api.delete(`/jaxrs/component_assemble_control/component/delete/${id}`),
};
export const configApi = {
  isSet: (k: string) => api.get(`/jaxrs/config/is/${k}`),
  systemConfig: () => api.get('/jaxrs/config/system'),
};
export const editorApi = { list: () => api.get('/jaxrs/editor/list') };
export const externalDataSourceApi = {
  list: () => api.get('/jaxrs/externaldatasources/list'),
  check: (d: unknown) => api.post('/jaxrs/externaldatasources/check', d),
  validate: (d: unknown) => api.post('/jaxrs/externaldatasources/validate', d),
  set: (d: unknown) => api.post('/jaxrs/externaldatasources/set', d),
  cancel: () => api.post('/jaxrs/externaldatasources/set/cancel', null),
};
export const groupApi = {
  list: (f?: string) => api.get(f ? `/jaxrs/group/${f}` : '/jaxrs/group/list'),
  has: (f: string) => api.get(`/jaxrs/group/has/${f}`),
};
export const identityApi = {
  list: () => api.get('/jaxrs/identity/list'),
  detail: (id: string) => api.get(`/jaxrs/identity/${id}`),
};
export const imageApi = {
  encode: (d: unknown) => api.post('/jaxrs/image/encode', d),
  resize: (d: unknown) => api.post('/jaxrs/image/resize', d),
};
export const uuidApi = { random: () => api.get('/jaxrs/uuid/random') };
export const personAttributeApi = {
  list: () => api.get('/jaxrs/personattribute/list'),
  append: (d: unknown) => api.post('/jaxrs/personattribute/append', d),
  set: (d: unknown) => api.put('/jaxrs/personattribute/set', d),
};
export const unitAttributeApi = {
  list: () => api.get('/jaxrs/unitattribute/list'),
  append: (d: unknown) => api.post('/jaxrs/unitattribute/append', d),
  set: (d: unknown) => api.put('/jaxrs/unitattribute/set', d),
};
export const unitDutyApi = {
  find: (n: string) => api.get(`/jaxrs/unitduty/find/${n}`),
  list: () => api.get('/jaxrs/unitduty/list'),
};
export const viewCategoryApi = {
  list: () => api.get('/jaxrs/viewcategory/list'),
  detail: (id: string) => api.get(`/jaxrs/viewcategory/${id}`),
};
export const viewFieldConfigApi = {
  list: () => api.get('/jaxrs/viewfieldconfig/list'),
  detail: (id: string) => api.get(`/jaxrs/viewfieldconfig/${id}`),
};
export const templateFormApi = {
  list: () => api.get('/jaxrs/templateform/list'),
  detail: (id: string) => api.get(`/jaxrs/templateform/${id}`),
};
export const exportDetailApi = { appInfo: () => api.get('/jaxrs/export/appInfo') };
export const importDetailApi = { appInfo: () => api.get('/jaxrs/import/appInfo') };
export const categoryDetailApi = {
  alias: (a: string) => api.get(`/jaxrs/categoryinfo/alias/${a}`),
  bind: (categoryId: string, view: string) => api.post(`/jaxrs/categoryinfo/bind/${categoryId}/view/${view}`, null),
  erase: (id: string) => api.delete(`/jaxrs/categoryinfo/erase/category/${id}`),
  extContent: () => api.get('/jaxrs/categoryinfo/extContent'),
  filterList: (id: string, count: number, appId: string) =>
    api.get(`/jaxrs/categoryinfo/filter/list/${id}/next/${count}/app/${appId}`),
  flag: () => api.get('/jaxrs/categoryinfo/flag'),
  list: () => api.get('/jaxrs/categoryinfo/list'),
  detail: (id: string) => api.get(`/jaxrs/categoryinfo/${id}`),
};
export const appConfigApi = { get: (appId: string) => api.get(`/jaxrs/appconfig/${appId}`) };
export const appDictApi = {
  list: (flag: string) => api.get(`/jaxrs/surface/appdict/${flag}`),
  appInfo: (flag: string, appId: string) => api.get(`/jaxrs/surface/appdict/${flag}/appInfo/${appId}`),
};

// ─────────────────────────────────────────────────────────────
// 统一导出包（供视图使用）
// ─────────────────────────────────────────────────────────────
export const oa4rustApis = {
  // 原有
  auth: authApi, org: orgApi, process: processApi, portal: portalApi,
  message: messageApi, im: imApi, file: fileApi, general: generalApi,
  appInfo: appInfoApi, category: categoryApi, hotpic: hotpicApi,
  jpush: jpushApi, correlation: correlationApi, share: shareApi,
  cache: cacheApi, sysResource: sysResourceApi, log: logApi,
  console: consoleApi, exportApi, importApi, attachment: attachmentApi,
  anonymous: anonymousApi, data: dataApi,
  // 新增
  processService: processServiceApi, queryView: queryViewApi,
  programCenter: programCenterApi, mind: mindApi, document: documentApi,
  calendarDeep: calendarDeepApi, attendanceDeep: attendanceDeepApi,
  queryDesigner: queryDesignerApi, attachmentDeep: attachmentDeepApi,
  recycle: recycleApi, server: serverApi, unit: unitApi,
  form: formApi, view: viewApi, fileInfo: fileInfoApi,
  empowerLog: empowerLogApi, commend: commendApi, comment: commentApi, complex: complexApi, component: componentApi,
  config: configApi, editor: editorApi, externalDataSource: externalDataSourceApi,
  group: groupApi, identity: identityApi, image: imageApi, uuid: uuidApi,
  categoryDetail: categoryDetailApi, appConfig: appConfigApi, appDict: appDictApi,
  personAttribute: personAttributeApi, unitAttribute: unitAttributeApi,
  unitDuty: unitDutyApi, viewCategory: viewCategoryApi,
  viewFieldConfig: viewFieldConfigApi, templateForm: templateFormApi,
  exportDetail: exportDetailApi, importDetail: importDetailApi,
};

// ─────────────────────────────────────────────────────────────
// 工作流表面 (processplatform_assemble_surface — 963 routes)
// ─────────────────────────────────────────────────────────────
export const processplatformSurfaceApi = {
  openapi: () => api.get("/jaxrs/processplatform/assemble/surface/openapi"),
  get: (id: string) => api.get(`/jaxrs/processplatform/assemble/surface/get/${id}`),
  sign: (id: string) => api.get(`/jaxrs/processplatform/assemble/surface/sign/${id}`),
  snap: (id: string) => api.get(`/jaxrs/processplatform/assemble/surface/snap/${id}`),
  task: (id: string) => api.get(`/jaxrs/processplatform/assemble/surface/task/${id}`),
  work: (id: string) => api.get(`/jaxrs/processplatform/assemble/surface/work/${id}`),
  draft: (id: string) => api.get(`/jaxrs/processplatform/assemble/surface/draft/${id}`),
  route: (id: string) => api.get(`/jaxrs/processplatform/assemble/surface/route/${id}`),
  form: (flag: string) => api.get(`/jaxrs/processplatform/assemble/surface/form/${flag}`),
  review: (id: string) => api.get(`/jaxrs/processplatform/assemble/surface/review/${id}`),
  preview: (id: string) => api.get(`/jaxrs/processplatform/assemble/surface/preview/${id}`),
  handover: (id: string) => api.post(`/jaxrs/processplatform/assemble/surface/handover/${id}`, null),
  create: (data: unknown) => api.post("/jaxrs/processplatform/assemble/surface/create", data),
  save: (id: string, data: unknown) => api.put(`/jaxrs/processplatform/assemble/surface/save/${id}`, data),
  delete: (id: string) => api.delete(`/jaxrs/processplatform/assemble/surface/${id}`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/processplatform/assemble/surface" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// CMS 内容管理 (cms_assemble_control — 405 routes)
// ─────────────────────────────────────────────────────────────
export const cmsApi = {
  log: (id: string) => api.get(`/jaxrs/cms/log/${id}`),
  file: (id: string) => api.get(`/jaxrs/cms/file/${id}`),
  form: (id: string) => api.get(`/jaxrs/cms/form/${id}`),
  view: (id: string) => api.get(`/jaxrs/cms/view/${id}`),
  script: (id: string) => api.get(`/jaxrs/cms/script/${id}`),
  outputList: () => api.get("/jaxrs/cms/output/list"),
  create: (type: string, data: unknown) => api.post(`/jaxrs/cms/${type}`, data),
  update: (type: string, id: string, data: unknown) => api.put(`/jaxrs/cms/${type}/${id}`, data),
  delete: (type: string, id: string) => api.delete(`/jaxrs/cms/${type}/${id}`),
  comment: (id: string) => api.get(`/jaxrs/cms/comment/${id}`),
  document: (id: string) => api.get(`/jaxrs/cms/document/${id}`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/cms" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 组织控制 (organization_assemble_control — 235 routes)
// ─────────────────────────────────────────────────────────────
export const organizationControlApi = {
  identity: (flag: string) => api.get(`/jaxrs/organization/assemble/control/identity/${flag}`),
  role: (flag: string) => api.get(`/jaxrs/organization/assemble/control/role/${flag}`),
  unit: (flag: string) => api.get(`/jaxrs/organization/assemble/control/unit/${flag}`),
  group: (flag: string) => api.get(`/jaxrs/organization/assemble/control/group/${flag}`),
  person: (flag: string) => api.get(`/jaxrs/organization/assemble/control/person/${flag}`),
  getRoot: () => api.get("/jaxrs/organization/assemble/control/unit/get/root"),
  listTop: () => api.get("/jaxrs/organization/assemble/control/unit/list/top"),
  roleListLike: () => api.get("/jaxrs/organization/assemble/control/role/list/like"),
  unitListLike: () => api.get("/jaxrs/organization/assemble/control/unit/list/like"),
  groupListLike: () => api.get("/jaxrs/organization/assemble/control/group/list/like"),
  identityListLike: () => api.get("/jaxrs/organization/assemble/control/identity/list/like"),
  create: (type: string, data: unknown) => api.post(`/jaxrs/organization/assemble/control/${type}`, data),
  update: (type: string, flag: string, data: unknown) => api.put(`/jaxrs/organization/assemble/control/${type}/${flag}`, data),
  delete: (type: string, flag: string) => api.delete(`/jaxrs/organization/assemble/control/${type}/${flag}`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/organization/assemble/control" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 考勤控制 (attendance_assemble_control — 228 routes)
// ─────────────────────────────────────────────────────────────
export const attendanceControlApi = {
  ruleList: () => api.get("/jaxrs/attendance/assemble/control/rule/list"),
  v2Config: () => api.get("/jaxrs/attendance/assemble/control/v2/config"),
  uuid: () => api.get("/jaxrs/attendance/assemble/control/uuid/random"),
  statistic: () => api.post("/jaxrs/attendance/assemble/control/statistic/do", null),
  v2Group: (id: string) => api.get(`/jaxrs/attendance/assemble/control/v2/group/${id}`),
  v2MyVersion: () => api.get("/jaxrs/attendance/assemble/control/v2/my/version"),
  v2Shift: (id: string) => api.get(`/jaxrs/attendance/assemble/control/v2/shift/${id}`),
  qywxSyncList: () => api.get("/jaxrs/attendance/assemble/control/qywx/sync/list"),
  workplace: (data: unknown) => api.post("/jaxrs/attendance/assemble/control/workplace", data),
  v2Workplace: (data: unknown) => api.post("/jaxrs/attendance/assemble/control/v2/workplace", data),
  admin: (data: unknown) => api.post("/jaxrs/attendance/assemble/control/attendanceadmin", data),
  toggleRule: (id: string) => api.put(`/jaxrs/attendance/assemble/control/rule/${id}/toggle`, null),
  auditAppeal: (id: string, data: unknown) => api.put(`/jaxrs/attendance/assemble/control/attendanceappealInfo/audit/${id}`, data),
  checkDetail: (params: unknown) => api.post("/jaxrs/attendance/assemble/control/attendancedetail/filter/list", params),
  deleteWorkplace: (id: string) => api.delete(`/jaxrs/attendance/assemble/control/workplace/${id}`),
  deleteAdmin: (id: string) => api.delete(`/jaxrs/attendance/assemble/control/attendanceadmin/${id}`),
  deleteDetail: (id: string) => api.delete(`/jaxrs/attendance/assemble/control/attendancedetail/${id}`),
  dingdingSync: () => api.delete("/jaxrs/attendance/assemble/control/dingding/all"),
  qywxSync: () => api.delete("/jaxrs/attendance/assemble/control/qywx/all"),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/attendance/assemble/control" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};
// ─────────────────────────────────────────────────────────────
// 工作流表面 (processplatform_assemble_surface — 963 routes)
// ─────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────
// CMS 内容管理 (cms_assemble_control — 405 routes)
// ─────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────
// 组织控制 (organization_assemble_control — 235 routes)
// ─────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────
// 考勤控制 (attendance_assemble_control — 228 routes)
// ─────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────
// 文件控制 (file_assemble_control — 182 routes)
// ─────────────────────────────────────────────────────────────
export const fileControlApi = {
  shareList: () => api.get("/jaxrs/file/assemble/control/share/list"),
  share: (id: string) => api.get(`/jaxrs/file/assemble/control/share/${id}`),
  top: () => api.get("/jaxrs/file/assemble/control/complex/top"),
  editorList: () => api.get("/jaxrs/file/assemble/control/editor/list"),
  folder: (id: string) => api.get(`/jaxrs/file/assemble/control/folder/${id}`),
  fileId: (id: string) => api.get(`/jaxrs/file/assemble/control/file/id/${id}`),
  folder2: (id: string) => api.get(`/jaxrs/file/assemble/control/folder2/${id}`),
  recycleList: () => api.get("/jaxrs/file/assemble/control/recycle/list"),
  shareCreate: (data: unknown) => api.post("/jaxrs/file/assemble/control/share", data),
  config: (data: unknown) => api.post("/jaxrs/file/assemble/control/config", data),
  folderCreate: (data: unknown) => api.post("/jaxrs/file/assemble/control/folder", data),
  folder2Create: (data: unknown) => api.post("/jaxrs/file/assemble/control/folder2", data),
  folderUpdate: (id: string, data: unknown) => api.put(`/jaxrs/file/assemble/control/folder/${id}`, data),
  folder2Update: (id: string, data: unknown) => api.put(`/jaxrs/file/assemble/control/folder2/${id}`, data),
  emptyRecycle: () => api.delete("/jaxrs/file/assemble/control/recycle/empty"),
  deleteShare: (id: string) => api.delete(`/jaxrs/file/assemble/control/share/${id}`),
  deleteFolder: (id: string) => api.delete(`/jaxrs/file/assemble/control/folder/${id}`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/file/assemble/control" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 会议控制 (meeting_assemble_control — 109 routes)
// ─────────────────────────────────────────────────────────────
export const meetingControlApi = {
  roomList: () => api.get("/jaxrs/meeting/assemble/control/room/list"),
  room: (id: string) => api.get(`/jaxrs/meeting/assemble/control/room/${id}`),
  openmeeting: () => api.get("/jaxrs/meeting/assemble/control/openmeeting"),
  meeting: (id: string) => api.get(`/jaxrs/meeting/assemble/control/meeting/${id}`),
  buildingList: () => api.get("/jaxrs/meeting/assemble/control/building/list"),
  building: (id: string) => api.get(`/jaxrs/meeting/assemble/control/building/${id}`),
  attachment: (id: string) => api.get(`/jaxrs/meeting/assemble/control/attachment/${id}`),
  roomCreate: (data: unknown) => api.post("/jaxrs/meeting/assemble/control/room", data),
  config: (data: unknown) => api.post("/jaxrs/meeting/assemble/control/config", data),
  create: (data: unknown) => api.post("/jaxrs/meeting/assemble/control/create", data),
  meetingCreate: (data: unknown) => api.post("/jaxrs/meeting/assemble/control/meeting", data),
  buildingCreate: (data: unknown) => api.post("/jaxrs/meeting/assemble/control/building", data),
  roomUpdate: (id: string, data: unknown) => api.put(`/jaxrs/meeting/assemble/control/room/${id}`, data),
  meetingUpdate: (id: string, data: unknown) => api.put(`/jaxrs/meeting/assemble/control/meeting/${id}`, data),
  buildingUpdate: (id: string, data: unknown) => api.put(`/jaxrs/meeting/assemble/control/building/${id}`, data),
  roomDelete: (id: string) => api.delete(`/jaxrs/meeting/assemble/control/room/${id}`),
  meetingDelete: (id: string) => api.delete(`/jaxrs/meeting/assemble/control/meeting/${id}`),
  buildingDelete: (id: string) => api.delete(`/jaxrs/meeting/assemble/control/building/${id}`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/meeting/assemble/control" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 门户表面 (portal_assemble_surface — 72 routes)
// ─────────────────────────────────────────────────────────────
export const portalSurfaceApi = {
  list: () => api.get("/jaxrs/portal/assemble/surface/list"),
  preview: (id: string) => api.get(`/jaxrs/portal/assemble/surface/${id}/preview`),
  get: (id: string) => api.get(`/jaxrs/portal/assemble/surface/get/${id}`),
  getLayout: () => api.get("/jaxrs/portal/assemble/surface/get/layout"),
  file: (flag: string) => api.get(`/jaxrs/portal/assemble/surface/file/${flag}`),
  script: (id: string) => api.get(`/jaxrs/portal/assemble/surface/script/${id}`),
  widget: (id: string) => api.get(`/jaxrs/portal/assemble/surface/widget/${id}`),
  publish: (data: Record<string, unknown>) => api.post("/jaxrs/portal/assemble/surface/publish", data),
  create: (data: Record<string, unknown>) => api.post("/jaxrs/portal/assemble/surface/create", data),
  saveLayout: (data: Record<string, unknown>) => api.put("/jaxrs/portal/assemble/surface/save/layout", data),
  deleteLayout: (data: Record<string, unknown>) => api.delete("/jaxrs/portal/assemble/surface/delete/layout", data),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/portal/assemble/surface" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 通用控制 (general_assemble_control — 94 routes)
// ─────────────────────────────────────────────────────────────
export const generalControlApi = {
  status: () => api.get("/jaxrs/general/assemble/control/status"),
  areaList: () => api.get("/jaxrs/general/assemble/control/area/list"),
  area: (id: string) => api.get(`/jaxrs/general/assemble/control/area/${id}`),
  qrCodeList: () => api.get("/jaxrs/general/assemble/control/qrcode/list"),
  qrCode: (id: string) => api.get(`/jaxrs/general/assemble/control/qrcode/${id}`),
  attendScopeList: () => api.get("/jaxrs/general/assemble/control/attendscope/list"),
  office: (data: unknown) => api.post("/jaxrs/general/assemble/control/office", data),
  qrCodeCreate: (data: unknown) => api.post("/jaxrs/general/assemble/control/qrcode", data),
  areaCreate: (data: unknown) => api.post("/jaxrs/general/assemble/control/area/create", data),
  invoiceCreate: (data: unknown) => api.post("/jaxrs/general/assemble/control/invoice/create", data),
  statusUpdate: (data: unknown) => api.put("/jaxrs/general/assemble/control/status/update", data),
  areaUpdate: (id: string, data: unknown) => api.put(`/jaxrs/general/assemble/control/area/update/${id}`, data),
  areaDelete: (id: string) => api.delete(`/jaxrs/general/assemble/control/area/delete/${id}`),
  qrCodeDelete: (id: string) => api.delete(`/jaxrs/general/assemble/control/qrcode/delete/${id}`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/general/assemble/control" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// 消息通信 (message_assemble_communicate — 78 routes)
// ─────────────────────────────────────────────────────────────
export const messageCommunicateApi = {
  connector: () => api.get("/jaxrs/message/assemble/communicate/connector"),
  mass: (id: string) => api.get(`/jaxrs/message/assemble/communicate/mass/${id}`),
  wsList: () => api.get("/jaxrs/message/assemble/communicate/ws/list/person"),
  wsCount: () => api.get("/jaxrs/message/assemble/communicate/ws/count/person"),
  imManagerConfig: () => api.get("/jaxrs/message/assemble/communicate/im/manager/config"),
  receive: (consume: string) => api.get(`/jaxrs/message/assemble/communicate/receive/${consume}`),
  imMsgList: () => api.get("/jaxrs/message/assemble/communicate/im/msg/list/object"),
  imMsgRevoke: (id: string) => api.get(`/jaxrs/message/assemble/communicate/im/msg/revoke/${id}`),
  ws: (data: unknown) => api.post("/jaxrs/message/assemble/communicate/ws", data),
  massCreate: (data: unknown) => api.post("/jaxrs/message/assemble/communicate/mass", data),
  send: (data: unknown) => api.post("/jaxrs/message/assemble/communicate/send", data),
  imMsg: (data: unknown) => api.post("/jaxrs/message/assemble/communicate/im/msg", data),
  markRead: (id: string) => api.post(`/jaxrs/message/assemble/communicate/mark_read/${id}`, null),
  imConversation: (data: unknown) => api.post("/jaxrs/message/assemble/communicate/im/conversation", data),
  imConversationUpdate: (id: string, data: unknown) => api.put(`/jaxrs/message/assemble/communicate/im/conversation/${id}`, data),
  imConversationRead: (id: string) => api.put(`/jaxrs/message/assemble/communicate/im/conversation/${id}/read`, null),
  massDelete: (id: string) => api.delete(`/jaxrs/message/assemble/communicate/mass/${id}`),
  imConversationGroup: (id: string) => api.delete(`/jaxrs/message/assemble/communicate/im/conversation/${id}/group`),
  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/message/assemble/communicate" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },
}

/**
 * Extra API modules for remaining backend crates
 * Auto-generated — do not edit manually
 */


// ─────────────────────────────────────────────────────────────
// ai_core_entity (3 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const ai_core_entityApi = {
  getlist: () => api.get("/jaxrs/ai/core/entity/app/list"),
  getlist_1: () => api.get("/jaxrs/ai/core/entity/model/list"),
  getlist_2: () => api.get("/jaxrs/ai/core/entity/conversation/list"),
  // Generic fallback for 0 remaining paths
  request: (method: string, path: string, body?: unknown) => {
    const url = '/jaxrs/' + path;
    if (method === 'GET') return api.get(url);
    if (method === 'POST') return api.post(url, body);
    if (method === 'PUT') return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// query_service (3 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const query_serviceApi = {
  getlist: () => api.get("/jaxrs/query/service/neural/list"),
  postexecute: (body?: unknown) => api.post("/jaxrs/query/service/processing/execute", body),
  postmodelflag: (model_flag: string, body?: unknown) => api.post("/jaxrs/query/service/neural/generate/:model_flag", body),
  // Generic fallback for 0 remaining paths
  request: (method: string, path: string, body?: unknown) => {
    const url = '/jaxrs/' + path;
    if (method === 'GET') return api.get(url);
    if (method === 'POST') return api.post(url, body);
    if (method === 'PUT') return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// query_service_processing (4 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const query_service_processingApi = {
  getstatus: () => api.get("/jaxrs/query/service/processing/status"),
  postbatch: (body?: unknown) => api.post("/jaxrs/query/service/processing/batch", body),
  postreset: (body?: unknown) => api.post("/jaxrs/query/service/processing/reset", body),
  postprocess: (body?: unknown) => api.post("/jaxrs/query/service/processing/process", body),
  // Generic fallback for 0 remaining paths
  request: (method: string, path: string, body?: unknown) => {
    const url = '/jaxrs/' + path;
    if (method === 'GET') return api.get(url);
    if (method === 'POST') return api.post(url, body);
    if (method === 'PUT') return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// empower (16 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const empowerApi = {
  getid: (id: string) => api.get("/jaxrs/person/empower/:id"),
  getto: () => api.get("/jaxrs/person/empower/list/to"),
  getenable: (id: string) => api.get("/jaxrs/person/empower/:id/enable"),
  getdisable: (id: string) => api.get("/jaxrs/person/empower/:id/disable"),
  getenable_4: () => api.get("/jaxrs/person/empower/list/to/enable"),
  getcurrentperson: () => api.get("/jaxrs/person/empower/list/currentperson"),
  getenable_6: () => api.get("/jaxrs/person/empower/list/currentperson/enable"),
  postempower: (body?: unknown) => api.post("/jaxrs/person/empower", body),
  postmanager: (body?: unknown) => api.post("/jaxrs/person/empower/manager", body),
  postenable: (id: string, body?: unknown) => api.post("/jaxrs/person/empower/:id/enable", body),
  postdisable: (id: string, body?: unknown) => api.post("/jaxrs/person/empower/:id/disable", body),
  postsize: (page: string, size: string, body?: unknown) => api.post("/jaxrs/person/empower/manager/list/paging/:page/size/:size", body),
  // Generic fallback for 4 remaining paths
  request: (method: string, path: string, body?: unknown) => {
    const url = '/jaxrs/' + path;
    if (method === 'GET') return api.get(url);
    if (method === 'POST') return api.post(url, body);
    if (method === 'PUT') return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// realtime (3 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const realtimeApi = {
  getrealtime: () => api.get("/jaxrs//ws/realtime"),
  getroomid: (room_id: string) => api.get("/jaxrs//ws/realtime/room/:room_id"),
  getstats: (room_id: string) => api.get("/jaxrs//ws/realtime/room/:room_id/stats"),
  // Generic fallback for 0 remaining paths
  request: (method: string, path: string, body?: unknown) => {
    const url = '/jaxrs/' + path;
    if (method === 'GET') return api.get(url);
    if (method === 'POST') return api.post(url, body);
    if (method === 'PUT') return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// base (9 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const baseApi = {
  getecho: () => api.get("/jaxrs/base/echo"),
  getget: () => api.get("/jaxrs/base/echo/get"),
  getdetail: () => api.get("/jaxrs/base/cache/detail"),
  getinfo: () => api.get("/jaxrs/base/openapi/info"),
  getflush: () => api.get("/jaxrs/base/cache/config/flush"),
  getflush_5: () => api.get("/jaxrs/base/cache/commonscript/flush"),
  getfilePath: (filePath: string) => api.get("/jaxrs/base/sysresource/filePath/:filePath"),
  getclassName: (className: string) => api.get("/jaxrs/base/fireschedule/classname/:className"),
  postcache: (body?: unknown) => api.post("/jaxrs/base/cache", body),
  // Generic fallback for 0 remaining paths
  request: (method: string, path: string, body?: unknown) => {
    const url = '/jaxrs/' + path;
    if (method === 'GET') return api.get(url);
    if (method === 'POST') return api.post(url, body);
    if (method === 'PUT') return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// preview (2 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const previewApi = {
  postupload: (body?: unknown) => api.post("/jaxrs//preview/upload", body),
  postconvert: (body?: unknown) => api.post("/jaxrs//preview/convert", body),
  // Generic fallback for 0 remaining paths
  request: (method: string, path: string, body?: unknown) => {
    const url = '/jaxrs/' + path;
    if (method === 'GET') return api.get(url);
    if (method === 'POST') return api.post(url, body);
    if (method === 'PUT') return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// signature (3 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const signatureApi = {
  postsign: (body?: unknown) => api.post("/jaxrs//signature/pdf/sign", body),
  poststatus: (body?: unknown) => api.post("/jaxrs//signature/pdf/status", body),
  postverify: (body?: unknown) => api.post("/jaxrs//signature/pdf/verify", body),
  // Generic fallback for 0 remaining paths
  request: (method: string, path: string, body?: unknown) => {
    const url = '/jaxrs/' + path;
    if (method === 'GET') return api.get(url);
    if (method === 'POST') return api.post(url, body);
    if (method === 'PUT') return api.put(url, body);
    return api.delete(url);
  },
};

// ─────────────────────────────────────────────────────────────
// Unified export object
// ─────────────────────────────────────────────────────────────
export const extraApis = {
  ai_core_entity: ai_core_entityApi,
  query_service: query_serviceApi,
  query_service_processing: query_service_processingApi,
  empower: empowerApi,
  realtime: realtimeApi,
  base: baseApi,
  preview: previewApi,
  signature: signatureApi,
};