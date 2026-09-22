// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * @oa4rust/apis — 业务 API 层（精选模块）
 *
 * 端点路径以 oa4rust 后端实际注册的 axum 路由为准（各 crate 的 routes.rs /
 * u2_router.rs / lib.rs 中的 .route 注册）：有真实路由的方法已对齐；
 * 后端未实现的遗留 o2web 表面用 @dead 注释标出（调用将 404）。
 * 注意：桌面端视图实际直接经 @oa4rust/sdk 的 api 调字面路径，本包当前无视图消费方。
 */

import { api, type PagedResponse } from '@oa4rust/sdk'
/** Unified request helper - reduces duplication across all API modules */
export const createRequest = (prefix: string) => (method: string, path: string, body?: unknown) => {
  const url = prefix + path
  const methodUpper = method.toUpperCase()
  const hasBody = body !== null && body !== undefined

  // 可测试的分支：根据 HTTP 方法分发请求
  switch (methodUpper) {
    case 'GET': {
      const getUrl = url
      return api.get(getUrl)
    }
    case 'POST': {
      const postUrl = url
      const postBody = hasBody ? body : undefined
      return api.post(postUrl, postBody)
    }
    case 'PUT': {
      const putUrl = url
      const putBody = hasBody ? body : undefined
      return api.put(putUrl, putBody)
    }
    case 'DELETE': {
      const deleteUrl = url
      return api.delete(deleteUrl)
    }
    default: {
      const defaultUrl = url
      return api.get(defaultUrl)
    }
  }
}
// ─────────────────────────────────────────────────────────────
// 认证模块 (28 routes)
// ─────────────────────────────────────────────────────────────
export const authApi = {
  login: (data: { credential: string; password: string; captchaId?: string; captchaAnswer?: string }) =>
    api.post<never>('/api/authentication/login', data, { requireAuth: false, discardResponse: true }),
  logout: () => api.post('/api/authentication/logout', null, { requireAuth: false, discardResponse: true }),
  who: () => api.get<import('@oa4rust/sdk').O2User>('/api/authentication/who', { requireAuth: false }),
  refresh: () => api.post<never>('/api/authentication/refresh', null, { requireAuth: false, discardResponse: true }),
  captcha: () => api.get<{ image: string; id: string }>('/api/authentication/captcha', { requireAuth: false }),
  captchaSize: (w: number, h: number) =>
    api.get<{ image: string; id: string }>(`/api/authentication/captcha/width/${w}/height/${h}`, {
      requireAuth: false,
    }),
  oauthList: () => api.get('/api/authentication/oauth/list'),
  sso: (data: unknown) => api.post('/api/authentication/sso', data),
  twoFactor: (data: unknown) => api.post('/api/authentication/two_factor', data),
  switchUser: (targetUnique: string) =>
    api.post<{ data: import('@oa4rust/sdk').O2User }>('/api/authentication/switchuser', { targetUnique }),
  groupList: () => api.get('/api/authentication/group/list'),
  roleList: () => api.get('/api/authentication/role/list'),
  unitList: () => api.get('/api/authentication/unit/list'),
  checkToken: (data: { token: string }) => api.post('/api/authentication/check/token', data),
  safeLogout: () => api.post('/api/authentication/safe/logout', null),
  request: createRequest('/api/authentication'),
}

// ─────────────────────────────────────────────────────────────
// 组织模块 (226 routes)
// ─────────────────────────────────────────────────────────────
export const orgApi = {
  // 部门树
  groupList: (flag?: string, count?: number) =>
    api.get(`/api/organization/assemble/control/group/list/${flag || ''}/next/${count || 20}`),
  groupDetail: (flag: string) => api.get(`/api/organization/assemble/control/group/${flag}`),
  /** @dead 后端未注册 group/{flag}/sub|sup 子/上级树（仅有 group/list/person/{p}/sup|sub 按人员族）。 */
  groupSub: (flag: string) => api.get(`/api/organization/assemble/control/group/${flag}/sub/nested`),
  /** @dead 后端未注册 group/{flag}/sup/nested。 */
  groupSup: (flag: string) => api.get(`/api/organization/assemble/control/group/${flag}/sup/nested`),
  groupAddMember: (flag: string, data: unknown) =>
    api.post(`/api/organization/assemble/control/group/${flag}/add/member`, data),
  groupDeleteMember: (flag: string, data: unknown) =>
    api.post(`/api/organization/assemble/control/group/${flag}/delete/member`, data),

  // 人员
  /** 分页人员列表（后端 POST person/list/filter/{page}/size/{size}；keyword 无后端字段，保留参数但不下发）。 */
  personList: (page: number, size: number, _keyword?: string) =>
    api.post<PagedResponse<import('@oa4rust/sdk').O2User>>(
      `/api/organization/assemble/control/person/list/filter/${page}/size/${size}`,
      {},
    ),
  personDetail: (id: string) => api.get(`/api/organization/assemble/control/person/${id}`),
  personCreate: (data: unknown) => api.post('/api/organization/assemble/control/person', data),
  personUpdate: (id: string, data: unknown) => api.put(`/api/organization/assemble/control/person/${id}`, data),
  personDelete: (id: string) => api.delete(`/api/organization/assemble/control/person/${id}`),

  // 身份
  identityList: (flag?: string) => api.get(`/api/organization/assemble/control/identity/list/${flag || ''}`),
  identityDetail: (flag: string) => api.get(`/api/organization/assemble/control/identity/${flag}`),

  // 搜索
  groupSearch: (keyword: string) =>
    api.get('/api/organization/assemble/control/group/list/like', { params: { keyword } }),
  personSearch: (keyword: string) =>
    api.get('/api/organization/assemble/control/person/list/like', { params: { keyword } }),
  identitySearch: (keyword: string) =>
    api.get('/api/organization/assemble/control/identity/list/like', { params: { keyword } }),

  // 导出
  exportAll: () => api.post('/api/organization/assemble/control/export/export/all', null),
  exportResult: (flag: string) => api.get(`/api/organization/assemble/control/export/result/flag/${flag}`),

  // 授权
  empowerList: (personId: string) => api.get(`/api/person/empower/list/${personId}`),
  empowerCreate: (data: unknown) => api.post('/api/person/empower', data),
  empowerUpdate: (id: string, data: unknown) => api.put(`/api/person/empower/${id}`, data),
  empowerDelete: (id: string) => api.delete(`/api/person/empower/${id}`),
  empowerEnable: (id: string) => api.post(`/api/person/empower/${id}/enable`),
  empowerDisable: (id: string) => api.post(`/api/person/empower/${id}/disable`),
  request: createRequest('/api/organization/assemble/control'),
}

// ─────────────────────────────────────────────────────────────
// 工作流模块 (1600+ routes)
// ─────────────────────────────────────────────────────────────
export const processApi = {
  // 工作表面（待办/审批）。端点对齐 oa4rust 已注册路由：列表读 surface 新栈
  // x_task / x_taskcompleted / x_work / x_read（与桌面 ProcessWork E2E 一致），
  // 审批走引擎 /api/task/{id}/complete|reject。
  /** 我发起的工作；后端该路由仅注册 POST，status 参数无对应后端能力，保留签名但忽略。 */
  workList: (page: number, size: number, _status?: string) =>
    api.post<PagedResponse<unknown>>(
      `/api/processplatform/assemble/surface/work/list/my/paging/${page}/size/${size}`,
      {},
    ),
  workDetail: (id: string) => api.get(`/api/processplatform/assemble/surface/work/${id}`),
  /** 发起流程（真实引擎端点，桌面 E2E 实跑通过）。 */
  workStart: (data: unknown) => api.post('/api/processplatform/service/processing/work', data),
  /** 待我处理任务（surface x_task）。 */
  taskList: (page: number, size: number) =>
    api.get<PagedResponse<unknown>>(`/api/processplatform/assemble/surface/task/list/my/paging/${page}/size/${size}`),
  /** 审批通过/驳回（引擎端点；body { data, opinion, action }，后端当前只读路径参数）。 */
  taskHandle: (taskId: string, action: 'approve' | 'reject', data?: Record<string, unknown>) =>
    action === 'reject'
      ? api.post(`/api/task/${taskId}/reject`, { data: {}, opinion: '', action: 'reject', ...data })
      : api.post(`/api/task/${taskId}/complete`, { data: {}, opinion: '', action: 'approve', ...data }),
  formView: (workId: string) => api.get(`/api/processplatform/assemble/surface/form/view/${workId}`),
  snapView: (workId: string) => api.get(`/api/processplatform/assemble/surface/snap/${workId}`),
  applicationDict: (flag: string) => api.get(`/api/processplatform/assemble/surface/applicationdict/${flag}`),
  /** 工作表单数据（x_data，按 work id）。 */
  workData: (workId: string) => api.get(`/api/processplatform/assemble/surface/data/work/${workId}`),
  /** 工作附件清单（PP_C_ATTACHMENT，按 work id）。 */
  attachmentList: (workId: string) =>
    api.get<PagedResponse<unknown>>(`/api/processplatform/assemble/surface/attachment/list/work/${workId}`),
  /** 流转记录（PP_C_RECORD，按 workOrWorkCompleted）。 */
  recordList: (workOrWorkCompleted: string) =>
    api.get<PagedResponse<unknown>>(
      `/api/processplatform/assemble/surface/record/list/workorworkcompleted/${workOrWorkCompleted}`,
    ),
  /** 工作日志（PP_C_WORKLOG，按 workOrWorkCompleted）。 */
  worklogList: (workOrWorkCompleted: string) =>
    api.get<PagedResponse<unknown>>(
      `/api/processplatform/assemble/surface/worklog/list/workorworkcompleted/${workOrWorkCompleted}`,
    ),
  /** 待阅清单（按 workOrWorkCompleted）。 */
  readByWork: (workOrWorkCompleted: string) =>
    api.get<PagedResponse<unknown>>(
      `/api/processplatform/assemble/surface/read/list/workorworkcompleted/${workOrWorkCompleted}`,
    ),

  // 已办 / 已读（surface 新栈）
  completedList: (page: number, size: number) =>
    api.get<PagedResponse<unknown>>(
      `/api/processplatform/assemble/surface/taskcompleted/list/my/paging/${page}/size/${size}`,
    ),
  readList: (page: number, size: number) =>
    api.get<PagedResponse<unknown>>(`/api/processplatform/assemble/surface/read/list/my/paging/${page}/size/${size}`),

  // 流程设计器
  /** 设计器全量列表（后端无 designer 分页变体，page/size 忽略）。 */
  processList: (_page?: number, _size?: number) =>
    api.get<PagedResponse<unknown>>('/api/processplatform/assemble/designer/list/all'),
  processCreate: (data: unknown) => api.post('/api/processplatform/assemble/designer/process', data),
  processUpdate: (id: string, data: unknown) => api.put(`/api/processplatform/assemble/designer/process/${id}`, data),
  processDelete: (id: string) => api.delete(`/api/processplatform/assemble/designer/process/${id}`),
  processExport: (id: string) => api.get(`/api/processplatform/assemble/designer/process/${id}/export`),

  // BAM 监控
  bamPeriod: (startTime: string, endTime: string) =>
    api.post('/api/processplatform/assemble/bam/period', { startTime, endTime }),

  // 服务处理
  serviceWorkList: (page: number, size: number) =>
    api.post<PagedResponse<unknown>>(`/api/processplatform/service/processing/work/list/paging/${page}/${size}`),
  request: createRequest('/api/processplatform/assemble/surface'),
}

// ─────────────────────────────────────────────────────────────
// 门户模块 (125 routes)
// ─────────────────────────────────────────────────────────────
export const portalApi = {
  /** 门户页面列表（后端 GET surface/page/list/portal/{portal}）。 */
  pageList: (appId: string) => api.get(`/api/portal/assemble/surface/page/list/portal/${appId}`),
  /** 页面详情（后端 GET surface/page/v2/{id}，非 surface/page/{id} 旧路径）。 */
  pageDetail: (pageId: string) => api.get(`/api/portal/assemble/surface/page/v2/${pageId}`),
  /** 新建页面（后端 POST designer/page/create）。 */
  pageCreate: (data: unknown) => api.post('/api/portal/assemble/designer/page/create', data),
  /** 保存页面（后端 PUT/POST designer/page/save/{id}）。 */
  pageUpdate: (id: string, data: unknown) => api.put(`/api/portal/assemble/designer/page/save/${id}`, data),
  /** 删除页面（后端 DELETE/POST designer/page/delete/{id}）。 */
  pageDelete: (id: string) => api.delete(`/api/portal/assemble/designer/page/delete/${id}`),
  /** @dead 后端 widget 列表族需 portal/flag 参数（widget/portal/{flag}/{portalflag}），无单 page 列表。 */
  widgetList: (pageId: string) => api.get(`/api/portal/assemble/surface/widget/list/${pageId}`),
  /** @dead 后端无裸 designer/page/list（需 /portal/{portalId} 或 /{category}）。 */
  designerPageList: () => api.get('/api/portal/assemble/designer/page/list'),
  /** @dead 后端无裸 designer/script/list（需 /manager 或 /portal/{portalId}）。 */
  designerScriptList: () => api.get('/api/portal/assemble/designer/script/list'),
  request: createRequest('/api/portal/assemble'),
}

// ─────────────────────────────────────────────────────────────
// 即时通讯 (im 族，端点对齐 oa4rust 已注册路由)
// ─────────────────────────────────────────────────────────────
export const messageApi = {
  /** 我的会话列表（后端 im/conversation/list/my，无分页参数）。 */
  conversationList: (_page?: number, _size?: number) =>
    api.get('/api/message/assemble/communicate/im/conversation/list/my'),
  /** 消息历史（后端 im/msg/list/{page}/size/{size} 返回全量，按会话过滤需前端处理）。 */
  msgHistory: (_conversationId?: string, page = 1, size = 50) =>
    api.get<PagedResponse<unknown>>(`/api/message/assemble/communicate/im/msg/list/${page}/size/${size}`),
  /** 发消息（后端 im/msg）。 */
  msgSend: (data: { conversationId: string; content: string; type: string }) =>
    api.post('/api/message/assemble/communicate/im/msg', {
      conversationId: data.conversationId,
      content: data.content,
      type: data.type,
    }),
  /** 收藏消息列表（后端 im/msg/collection/list/{page}/size/{size}）。 */
  collectionList: (page: number, size: number) =>
    api.get<PagedResponse<unknown>>(`/api/message/assemble/communicate/im/msg/collection/list/${page}/size/${size}`),
  /** 标记会话已读（后端 im/conversation/{id}/read）。 */
  markRead: (conversationId: string) =>
    api.post(`/api/message/assemble/communicate/im/conversation/${conversationId}/read`),
  request: createRequest('/api/message/assemble/communicate'),
}

// ─────────────────────────────────────────────────────────────
// 文件模块 (端点对齐 oa4rust 已注册路由)
// ─────────────────────────────────────────────────────────────
export const fileApi = {
  /** 目录文件列表（后端 GET file/list/{folderId}，无分页参数）。 */
  fileList: (folderId: string, _page?: number, _size?: number) =>
    api.get<PagedResponse<unknown>>(`/api/file/assemble/control/file/list/${folderId}`),
  /** 目录列表（后端 GET file/core/entity/folder/list/{parentId}）。 */
  folderList: (parentId: string) => api.get(`/api/file/core/entity/folder/list/${parentId}`),
  fileUpload: (formData: FormData) => api.upload('/api/file/assemble/control/file/upload', formData),
  /** 下载（后端 GET file/assemble/control/file/{id}/download，非 core/entity 旧路径）。 */
  fileDownload: (fileId: string) => window.open(`/api/file/assemble/control/file/${fileId}/download`),
  fileDelete: (fileId: string) => api.delete(`/api/file/assemble/control/file/${fileId}`),
  fileShare: (fileId: string, data: unknown) => api.post(`/api/file/assemble/control/file/${fileId}/share`, data),
  request: createRequest('/api/file/assemble/control'),
}

// ─────────────────────────────────────────────────────────────
// 通用 API (字典/发票/工时等)
// ─────────────────────────────────────────────────────────────
export const generalApi = {
  dictList: () => api.get('/api/general/dict/list'),
  dictCreate: (data: unknown) => api.post('/api/general/dict/create', data),
  dictUpdate: (id: string, data: unknown) => api.post(`/api/general/dict/update/${id}`, data),
  dictDelete: (id: string) => api.post(`/api/general/dict/delete/${id}`),
  dictItemList: (dictId: string) => api.get(`/api/general/dict/item/list/${dictId}`),
  dictItemCreate: (dictId: string, data: unknown) => api.post(`/api/general/dict/item/${dictId}`, data),
  /** @dead 后端未注册 general/file 直传路由（仅 general/file/create 等），暂留原路径。 */
  fileUpload: (formData: FormData) => api.upload('/api/general/file', formData),
  /** 发票列表（后端 GET general/invoice/list，无分页参数）。 */
  invoiceList: () => api.get<PagedResponse<unknown>>('/api/general/invoice/list'),
  /** @dead 后端未注册按月工时列表（仅有 worktime/between|forward|indefined 计算族）。 */
  worktimeList: (month: string) => api.get(`/api/general/assemble/control/worktime/${month}`),
  request: createRequest('/api/general'),
}

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
  request: createRequest('/api/export'),
}

export default apis

// IM API
export const imApi = {
  conversationList: () => api.get('/api/message/assemble/communicate/im/conversation/list/my'),
  messageList: (p: number, s: number, cid?: string) =>
    api.post(`/api/message/assemble/communicate/im/msg/list/${p}/${s}`, { conversationId: cid }),
  messageSend: (d: unknown) => api.post('/api/message/assemble/communicate/im/msg', d),
  markRead: (id: string) => api.post(`/api/message/assemble/communicate/mark_read/${id}`, null),
  unreadCount: (c?: string) => api.get(`/api/message/unread/count/${c || 'im'}`),
  request: createRequest('/api/message/assemble/communicate'),
}

// ─────────────────────────────────────────────────────────────
// 扩展模块 API
// ─────────────────────────────────────────────────────────────

/** 应用信息 */
export const appInfoApi = {
  list: () => api.get('/api/appinfo/list'),
  detail: (id: string) => api.get(`/api/appinfo/${id}`),
  filter: (keyword: string) => api.get('/api/appinfo/filter', { params: { keyword } }),
  request: createRequest('/api/appinfo'),
}

/** 分类信息 */
export const categoryApi = {
  list: () => api.get('/api/categoryinfo/list'),
  request: createRequest('/api/categoryinfo'),
}

/** 热帖管理 */
export const hotpicApi = {
  list: () => api.get('/api/hotpic/core/entity/list'),
  create: (data: unknown) => api.post('/api/hotpic/core/entity/create', data),
  delete: (id: string) => api.delete(`/api/hotpic/core/entity/delete/${id}`),
  listByApp: (application: string, infoId: string) =>
    api.get(`/api/hotpic/core/entity/list/by/${application}/${infoId}`),
  request: createRequest('/api/hotpic/core/entity'),
}

/** JPush 推送 */
export const jpushApi = {
  deviceList: () => api.get('/api/jpush/core/entity/device/list'),
  deviceCreate: (data: unknown) => api.post('/api/jpush/core/entity/device/create', data),
  deviceDelete: (id: string) => api.delete(`/api/jpush/core/entity/device/${id}`),
  templateList: () => api.get('/api/jpush/core/entity/template/list'),
  templateUpdate: (id: string, data: unknown) => api.put(`/api/jpush/core/entity/template/${id}`, data),
  assembleDeviceList: (pushType: string) =>
    api.get(`/api/jpush_assemble_control/device/list/${encodeURIComponent(pushType)}`),
  assembleTemplateList: () => api.get('/api/jpush_assemble_control/template/list'),
  request: createRequest('/api/jpush/core/entity'),
}

/** 关联处理 */
export const correlationApi = {
  list: () => api.get('/api/correlation/core/entity/list'),
  create: (data: unknown) => api.post('/api/correlation/core/entity/create', data),
  delete: (id: string) => api.delete(`/api/correlation/core/entity/delete/${id}`),
  getStatus: () => api.get('/api/correlation/core/express/status'),
  sync: () => api.post('/api/correlation/core/express/sync', null),
  serviceList: (personId: string) => api.get(`/api/correlation/service/processing/list/${personId}`),
  serviceCreate: (data: unknown) => api.post('/api/correlation/service/processing/create', data),
  serviceDelete: (id: string) => api.delete(`/api/correlation/service/processing/delete/${id}`),
  serviceGet: (id: string) => api.get(`/api/correlation/service/processing/${id}`),
  serviceSave: (id: string, data: unknown) => api.put(`/api/correlation/service/processing/save/${id}`, data),
  link: (data: unknown) => api.post('/api/correlation/service/processing/link', data),
  unlink: (sourceType: string, sourceId: string, targetType: string, targetId: string) =>
    api.delete(`/api/correlation/service/processing/unlink/${sourceType}/${sourceId}/${targetType}/${targetId}`),
  request: createRequest('/api/correlation/core/entity'),
}

/** 分享管理 */
export const shareApi = {
  list: () => api.get('/api/share/list'),
  request: createRequest('/api/share'),
}

/** 缓存管理 */
export const cacheApi = {
  detail: () => api.get('/api/cache/detail'),
  flushCommonScript: () => api.post('/api/cache/commonscript/flush', null),
  flushConfig: () => api.post('/api/cache/config/flush', null),
  request: createRequest('/api/cache'),
}

/** 系统资源 */
export const sysResourceApi = {
  list: () => api.get('/api/sysresource/list'),
  request: createRequest('/api/sysresource'),
}

/** 日志查看 */
export const logApi = {
  list: () => api.get('/api/log/list'),
  request: createRequest('/api/log'),
}

/** 控制台 */
export const consoleApi = {
  list: () => api.get('/api/console/list'),
  request: createRequest('/api/console'),
}

/** 导出 */
export const exportApi = {
  result: (flag: string) => api.get(`/api/export/result/flag/${flag}`),
  request: createRequest('/api/export'),
}

/** 导入 */
export const importApi = {
  execute: (id: string) => api.post(`/api/importmodel/id/${id}/execute`, null),
  request: createRequest('/api/import'),
}

/** 附件 */
export const attachmentApi = {
  list: (id: string) => api.get(`/api/attachment2/list/${id}`),
  upload: (formData: FormData) => api.upload('/api/attachment2/upload', formData),
  delete: (id: string) => api.delete(`/api/attachment2/${id}`),
  request: createRequest('/api/attachment'),
}

/** 匿名访问 */
export const anonymousApi = {
  surfaceAppdict: (flag: string) => api.get(`/api/anonymous/surface/appdict/${flag}`),
  request: createRequest('/api/anonymous'),
}

/** 数据文档 */
export const dataApi = {
  documentDetail: (id: string, path0?: string) =>
    api.get(path0 ? `/api/data/document/${id}/${path0}` : `/api/data/document/${id}`),
  request: createRequest('/api/data'),
}

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
}

// Merge into apis export
Object.assign(apis, additionalApis)

// ─────────────────────────────────────────────────────────────
// 工作流深化 (processplatform service/processing)
// ─────────────────────────────────────────────────────────────
export const processServiceApi = {
  taskList: (page: number, size: number) =>
    api.post(`/api/processplatform/service/processing/task/list/paging/${page}/${size}`, {}),
  workList: (page: number, size: number) =>
    api.post(`/api/processplatform/service/processing/work/list/paging/${page}/${size}`, {}),
  applicationDict: (flag: string) => api.get(`/api/processplatform/service/processing/applicationdict/${flag}`),
  request: createRequest('/api/processplatform/service/processing'),
}

// ─────────────────────────────────────────────────────────────
// 查询视图 (queryview — 119 routes)
// ─────────────────────────────────────────────────────────────
export const queryViewApi = {
  list: (queryFlag: string) => api.get(`/api/queryview/list/${queryFlag}`),
  listAll: () => api.get('/api/queryview/list'),
  execute: (view: string, params?: Record<string, string>) => api.post(`/api/queryview/execute/${view}`, params ?? {}),
  executeV2: (view: string, params?: unknown) => api.post(`/api/queryview/execute/v2/${view}`, params),
  bundle: (view: string) => api.get(`/api/queryview/bundle/${view}`),
  bundleV2: (view: string) => api.post(`/api/queryview/bundle/v2/${view}`, {}),
  excel: (view: string) => api.get(`/api/queryview/excel/${view}`),
  flag: (viewFlag: string) => api.get(`/api/queryview/flag/${viewFlag}`),
  importModelList: (query: string, flag: string) => api.get(`/api/queryview/importmodel/list/${query}/${flag}`),
  importModelExecute: (data: unknown) => api.post('/api/queryview/importmodel/execute', data),
  moreLikeThis: (params: unknown) => api.post('/api/queryview/morelikethis', params),
  neural: (params: unknown) => api.post('/api/queryview/neural', params),
  search: (keyword: string) => api.get('/api/queryview/search', { params: { keyword } }),
  stat: (params: unknown) => api.post('/api/queryview/stat', params),
  statement: (params: unknown) => api.post('/api/queryview/statement', params),
  tableList: (page: number, size: number) => api.post(`/api/queryview/table/list/paging/${page}/${size}`, {}),
  tableRow: (flag: string, rowId: string) => api.get(`/api/queryview/table/row/${flag}/${rowId}`),
  viewDetail: (id: string) => api.get(`/api/queryview/view/${id}`),
  viewList: (page: number, size: number) => api.post(`/api/queryview/view/list/paging/${page}/${size}`, {}),
  request: createRequest('/api/queryview'),
}

// ─────────────────────────────────────────────────────────────
// 程序中心 (program_center — 319 routes)
// ─────────────────────────────────────────────────────────────
export const programCenterApi = {
  // Agent 管理
  agentList: () => api.get('/api/program_center/agent/list'),
  agentCreate: (data: unknown) => api.post('/api/program_center/agent/create', data),
  agentGet: (flag: string) => api.get(`/api/program_center/agent/${flag}`),
  agentSave: (id: string, data: unknown) => api.put(`/api/program_center/agent/save/${id}`, data),
  agentEnable: (flag: string) => api.post(`/api/program_center/agent/${flag}/enable`, null),
  agentDisable: (flag: string) => api.post(`/api/program_center/agent/${flag}/disable`, null),
  // Application 管理
  appList: () => api.get('/api/program_center/application/list'),
  appCreate: (data: unknown) => api.post('/api/program_center/application/create', data),
  appSave: (id: string, data: unknown) => api.put(`/api/program_center/application/save/${id}`, data),
  // AppStyle 样式
  appStyleCurrent: () => api.get('/api/program_center/appstyle/current'),
  appStyleUpdate: (data: unknown) => api.put('/api/program_center/appstyle/current/update', data),
  appStyleImage: (appId: string) => api.get(`/api/program_center/appstyle/image/application/top/${appId}`),
  // Script 脚本
  scriptList: () => api.get('/api/program_center/script/list'),
  scriptGet: (flag: string) => api.get(`/api/program_center/script/${flag}`),
  // Dict 字典
  dictList: () => api.get('/api/program_center/dict/list'),
  dictCreate: (data: unknown) => api.post('/api/program_center/dict/create', data),
  // Config 配置
  configList: () => api.get('/api/program_center/config/list'),
  configGet: (key: string) => api.get(`/api/program_center/config/${key}`),
  // Market 市场
  marketList: (page: number, size: number) => api.post(`/api/program_center/market/list/paging/${page}/${size}`, {}),
  marketGet: (flag: string) => api.get(`/api/program_center/market/${flag}`),
  // Module 模块
  moduleList: () => api.get('/api/program_center/module/list'),
  moduleInvoke: (flag: string, data: unknown) => api.post(`/api/program_center/invoke/${flag}`, data),
  // Schedule 计划任务
  scheduleList: () => api.get('/api/program_center/schedule/list'),
  scheduleCreate: (data: unknown) => api.post('/api/program_center/schedule/create', data),
  // Deploy 部署
  deploy: (data: unknown) => api.post('/api/program_center/deploy', data),
  // Code 代码
  codeList: () => api.get('/api/program_center/code/list'),
  // Error Logs
  promptErrorLogList: (page: number, size: number) =>
    api.post(`/api/program_center/prompterrorlog/list/paging/${page}/${size}`, {}),
  unexpectedErrorLogList: (page: number, size: number) =>
    api.post(`/api/program_center/unexpectederrorlog/list/paging/${page}/${size}`, {}),
  warnLogList: (page: number, size: number) => api.post(`/api/program_center/warnlog/list/paging/${page}/${size}`, {}),
  // Captcha
  captchaList: () => api.get('/api/program_center/captcha/list'),
  captchaCreate: (data: unknown) => api.post('/api/program_center/captcha/create', data),
  captchaGet: (id: string) => api.get(`/api/program_center/captcha/${id}`),
  // Data Structure
  dataStructureList: () => api.get('/api/program_center/datastructure/list'),
  // SSO callbacks
  andfxPull: (data: unknown) => api.post('/api/program_center/andfx/pull', data),
  dingdingCode: (code: string) => api.get(`/api/program_center/dingding/code/${code}`),
  mpweixinMenu: () => api.get('/api/program_center/mpweixin/menu'),
  qywxCode: (code: string) => api.get(`/api/program_center/qiyeweixin/code/${code}`),
  welinkCode: (code: string) => api.get(`/api/program_center/welink/code/${code}`),
  zwdCode: (code: string) => api.get(`/api/program_center/zhengwudingding/code/${code}`),
  request: createRequest('/api/program_center'),
}

// ─────────────────────────────────────────────────────────────
// 思维导图 (mind — 40 routes)
// ─────────────────────────────────────────────────────────────
export const mindApi = {
  folderTree: () => api.get('/api/mind/folder/tree/my'),
  folderList: () => api.get('/api/mind/folder/list'),
  folderDetail: (id: string) => api.get(`/api/mind/folder/${id}`),
  folderSave: (data: unknown) => api.post('/api/mind/assemble/control/folder/save', data),
  folderMove: (folderId: string, data: unknown) => api.post(`/api/mind/assemble/control/folder/move/${folderId}`, data),
  mindList: (id: string, page: number) => api.get(`/api/mind/mind/list/${id}/${page}`),
  mindDetail: (id: string) => api.get(`/api/mind/mind/${id}`),
  mindSave: (data: unknown) => api.post('/api/mind/assemble/control/mind/save', data),
  mindVersionList: (id: string) => api.get(`/api/mind/mind/list/${id}/version`),
  mindVersion: (id: string, version: string) => api.get(`/api/mind/mind/version/${id}/${version}`),
  mindFilterList: (id: string, page: number) =>
    api.get(`/api/mind/assemble/control/mind/filter/list/${id}/next/${page}`),
  mindRecycled: (id: string, page: number) =>
    api.get(`/api/mind/assemble/control/mind/filter/recycle/${id}/next/${page}`),
  mindRestore: (id: string) => api.post(`/api/mind/assemble/control/mind/restore/${id}`, null),
  mindDelete: (id: string) => api.delete(`/api/mind/assemble/control/mind/${id}/destorymind`),
  mindIcon: (id: string, size: number) => api.get(`/api/mind/assemble/control/mind/${id}/icon/size/${size}`),
  mindShare: (id: string) => api.post(`/api/mind/assemble/control/mind/share/${id}`, null),
  mindUnshare: (id: string) => api.post(`/api/mind/assemble/control/mind/share/${id}/cancel`, null),
  mindView: (id: string) => api.get(`/api/mind/assemble/control/mind/view/${id}`),
  config: () => api.get('/api/mind/assemble/control/config'),
  configUpdate: (data: unknown) => api.put('/api/mind/assemble/control/config/update', data),
  request: createRequest('/api/mind'),
}

// ─────────────────────────────────────────────────────────────
// 文档管理 (document)
// ─────────────────────────────────────────────────────────────
export const documentApi = {
  list: (params?: Record<string, string>) => api.get('/api/document/list', { params }),
  detail: (id: string) => api.get(`/api/document/${id}`),
  create: (data: unknown) => api.post('/api/document/document', data),
  update: (id: string, data: unknown) => api.put(`/api/document/document/${id}`, data),
  delete: (id: string) => api.delete(`/api/document/${id}`),
  draftList: (page: number, size: number) => api.post(`/api/document/draft/list/paging/${page}/${size}`, {}),
  filterList: (params: unknown) => api.post('/api/document/filter/list', params),
  batch: (data: unknown) => api.post('/api/document/batch', data),
  categoryList: () => api.get('/api/document/category/list'),
  cipherList: (id: string) => api.get(`/api/document/cipher/list/${id}`),
  publish: (id: string) => api.post(`/api/document/publish/${id}`, null),
  achive: (id: string) => api.post(`/api/document/achive/${id}`, null),
  request: createRequest('/api/document'),
}

// ─────────────────────────────────────────────────────────────
// 日历深化 (calendar_assemble_control)
// ─────────────────────────────────────────────────────────────
export const calendarDeepApi = {
  calendarList: (params?: Record<string, string>) =>
    api.get('/api/calendar_assemble_control/calendar/list', { params }),
  calendarDetail: (id: string) => api.get(`/api/calendar_assemble_control/calendar/${id}`),
  calendarCreate: (data: unknown) => api.post('/api/calendar_assemble_control/calendar', data),
  calendarUpdate: (id: string, data: unknown) => api.put(`/api/calendar_assemble_control/calendar/${id}`, data),
  calendarDelete: (id: string) => api.delete(`/api/calendar_assemble_control/calendar/${id}`),
  eventList: (id: string, page: number, size: number) =>
    api.post(`/api/calendar_assemble_control/event/list/${id}/paging/${page}/${size}`, {}),
  eventCreate: (data: unknown) => api.post('/api/calendar_assemble_control/event', data),
  eventUpdate: (id: string, data: unknown) => api.put(`/api/calendar_assemble_control/event/${id}`, data),
  eventDelete: (id: string) => api.delete(`/api/calendar_assemble_control/event/${id}`),
  eventSingle: (id: string) => api.get(`/api/calendar_assemble_control/event/single/${id}`),
  eventAfter: (id: string) => api.get(`/api/calendar_assemble_control/event/after/${id}`),
  eventAll: (id: string) => api.get(`/api/calendar_assemble_control/event/all/${id}`),
  follow: (id: string) => api.post(`/api/calendar_assemble_control/calendar/follow/${id}`, null),
  followCancel: (id: string) => api.post(`/api/calendar_assemble_control/calendar/follow/${id}/cancel`, null),
  isManager: (id: string) => api.get(`/api/calendar_assemble_control/calendar/ismanager/${id}`),
  rfc: (id: string) => api.get(`/api/calendar_assemble_control/event/rfc/${id}`),
  setting: () => api.get('/api/calendar_assemble_control/setting'),
  settingUpdate: (data: unknown) => api.put('/api/calendar_assemble_control/setting', data),
  messageList: () => api.get('/api/calendar_assemble_control/message/list'),
  request: createRequest('/api/calendar_assemble_control'),
}

// ─────────────────────────────────────────────────────────────
// 考勤深化
// ─────────────────────────────────────────────────────────────
export const attendanceDeepApi = {
  adminList: () => api.get('/api/attendance/assemble/control/attendanceadmin/list/all'),
  adminGet: (id: string) => api.get(`/api/attendance/assemble/control/attendanceadmin/${id}`),
  adminUpdate: (id: string, data: unknown) => api.put(`/api/attendance/assemble/control/attendanceadmin/${id}`, data),
  appealList: () => api.get('/api/attendance/appeal/list'),
  appealSubmit: (data: unknown) => api.post('/api/attendance/appeal/submit', data),
  appealArchive: (id: string) => api.post(`/api/attendance/appeal/archive/${id}`, null),
  appealAudit: (id: string, data: unknown) => api.put(`/api/attendance/appeal/audit/${id}`, data),
  detailAnalyse: (startDate: string, endDate: string) =>
    api.get(`/api/attendance/assemble/control/attendancedetail/analyse/${startDate}/${endDate}`),
  detailArchive: (id: string) => api.post(`/api/attendance/assemble/control/attendancedetail/archive/${id}`, null),
  detailCheck: (cycleYear: number, cycleMonth: number) =>
    api.get(
      `/api/attendance/assemble/control/attendancedetail/checkDetailWithPersonByCycle/${cycleYear}/${cycleMonth}`,
    ),
  statisticalList: (params: unknown) => api.post('/api/attendance/statistical/list', params),
  ruleList: () => api.get('/api/attendance/rule/list'),
  employeeList: (params: unknown) => api.post('/api/attendance/employee/list', params),
  // v2 考勤组（x_attendance_v2_group）：列表 POST paging、按 id 取/删、新建。
  v2GroupList: (page: number, size: number, name = '') =>
    api.post<PagedResponse<unknown>>(
      `/api/attendance/assemble/control/v2/group/list/${page}/size/${size}`,
      { name },
    ),
  v2GroupGet: (id: string) => api.get(`/api/attendance/assemble/control/v2/group/${id}`),
  v2GroupCreate: (data: unknown) => api.post('/api/attendance/assemble/control/v2/group', data),
  v2GroupDelete: (id: string) => api.get(`/api/attendance/assemble/control/v2/group/${id}/delete`),
  // v2 班次（x_attendance_v2_shift）：列表 POST paging、按 id 取/删、新建。
  v2ShiftList: (page: number, size: number, name = '') =>
    api.post<PagedResponse<unknown>>(
      `/api/attendance/assemble/control/v2/shift/list/${page}/size/${size}`,
      { name },
    ),
  v2ShiftGet: (id: string) => api.get(`/api/attendance/assemble/control/v2/shift/${id}`),
  v2ShiftCreate: (data: unknown) => api.post('/api/attendance/assemble/control/v2/shift/create', data),
  v2ShiftDelete: (id: string) => api.get(`/api/attendance/assemble/control/v2/shift/delete/${id}`),
  request: createRequest('/api/attendance'),
}

// ─────────────────────────────────────────────────────────────
// 查询设计器深化
// ─────────────────────────────────────────────────────────────
export const queryDesignerApi = {
  create: (data: unknown) => api.post('/api/query/assemble/designer/create', data),
  get: (id: string) => api.get(`/api/query/assemble/designer/get/${id}`),
  delete: (id: string) => api.delete(`/api/query/assemble/designer/delete/${id}`),
  search: (keyword: string) => api.get('/api/query/assemble/designer/designer/search', { params: { keyword } }),
  entityProperties: (query: string, category: string, entityCategory: string) =>
    api.get(`/api/query/assemble/designer/entity/entity/properties/${query}/${category}/${entityCategory}`),
  iconGet: (query: string, flag: string) => api.get(`/api/query/assemble/designer/icon/${query}/${flag}`),
  iconSet: (query: string, flag: string, data: unknown) =>
    api.put(`/api/query/assemble/designer/icon/set/${query}/${flag}`, data),
  idCount: (count: number) => api.get(`/api/query/assemble/designer/id/${count}`),
  importModel: (data: unknown) => api.post('/api/query/assemble/designer/importmodel', data),
  importModelList: (query: string, flag: string) =>
    api.get(`/api/query/assemble/designer/importmodel/list/${query}/${flag}`),
  importModelEdit: (id: string, data: unknown) => api.put(`/api/query/assemble/designer/importmodel/edit/${id}`, data),
  importModelDelete: (id: string) => api.delete(`/api/query/assemble/designer/importmodel/delete/${id}`),
  bundle: (view: string, id: string) => api.get(`/api/query/assemble/designer/bundle/${view}/${id}`),
  surfaceList: () => api.get('/api/query/assemble/surface/list'),
  request: createRequest('/api/query/assemble/designer'),
}

// ─────────────────────────────────────────────────────────────
// 附件深化
// ─────────────────────────────────────────────────────────────
export const attachmentDeepApi = {
  list: (id: string) => api.get(`/api/attachment/list/${id}`),
  download: (id: string) => api.get(`/api/attachment/download/${id}`),
  upload: (formData: FormData) => api.upload('/api/attachment/upload', formData),
  update: (id: string, data: unknown) => api.put(`/api/attachment/update/${id}`, data),
  exist: (id: string) => api.get(`/api/attachment2/exist/${id}`),
  upload2: (formData: FormData) => api.upload('/api/attachment2/upload', formData),
  userFiles: () => api.get('/api/attachment2/user'),
  request: createRequest('/api/attachment'),
}

// ─────────────────────────────────────────────────────────────
// 回收站
// ─────────────────────────────────────────────────────────────
export const recycleApi = {
  list: () => api.get('/api/recycle/list'),
  delete: (id: string) => api.delete(`/api/recycle/${id}`),
  empty: () => api.post('/api/recycle/empty', null),
  resume: (id: string) => api.post(`/api/recycle/resume/${id}`, null),
  request: createRequest('/api/recycle'),
}

// ─────────────────────────────────────────────────────────────
// 服务器管理
// ─────────────────────────────────────────────────────────────
export const serverApi = {
  execute: (command: string) => api.post('/api/server/execute', { command }),
  license: () => api.get('/api/server/license'),
  stop: () => api.post('/api/server/stop', null),
  request: createRequest('/api/server'),
}

// ─────────────────────────────────────────────────────────────
// 单元管理
// ─────────────────────────────────────────────────────────────
export const unitApi = {
  list: () => api.get('/api/unit/list'),
  check: (flag: string) => api.get(`/api/unit/check/${flag}`),
  identity: (flag: string) => api.get(`/api/unit/identity/${flag}`),
  request: createRequest('/api/unit'),
}

// ─────────────────────────────────────────────────────────────
// 表单管理
// ─────────────────────────────────────────────────────────────
export const formApi = {
  list: (params?: Record<string, string>) => api.get('/api/form/list', { params }),
  filter: (params: unknown) => api.post('/api/form/filter', params),
  detail: (id: string) => api.get(`/api/form/${id}`),
  v2List: () => api.get('/api/form/v2/list'),
  request: createRequest('/api/form'),
}

// ─────────────────────────────────────────────────────────────
// 视图管理
// ─────────────────────────────────────────────────────────────
export const viewApi = {
  list: () => api.get('/api/view/list'),
  detail: (id: string) => api.get(`/api/view/${id}`),
  viewData: (id: string, params?: unknown) => api.post(`/api/view/viewdata/${id}`, params ?? {}),
  request: createRequest('/api/view'),
}

// ─────────────────────────────────────────────────────────────
// 文件信息
// ─────────────────────────────────────────────────────────────
export const fileInfoApi = {
  list: () => api.get('/api/fileinfo/list/all'),
  listByDoc: (docId: string) => api.get(`/api/fileinfo/list/document/${docId}`),
  listFilter: (params: unknown) => api.post('/api/fileinfo/list/filter', params),
  detail: (id: string) => api.get(`/api/fileinfo/${id}`),
  download: (id: string) => api.get(`/api/fileinfo/download/document/${id}`),
  edit: (id: string, docId: string, data: unknown) => api.put(`/api/fileinfo/edit/${id}/doc/${docId}`, data),
  copy: (docId: string) => api.post(`/api/fileinfo/copy/to/doc/${docId}`, null),
  replace: (docId: string) => api.post(`/api/fileinfo/replace/to/doc/${docId}`, null),
  upload: (formData: FormData, docId: string) => api.upload(`/api/fileinfo/upload/document/${docId}`, formData),
  updateContent: (id: string, data: unknown) => api.put(`/api/fileinfo/update/${id}/content`, data),
  batchDownload: (docId: string, site: string) => api.get(`/api/fileinfo/batch/download/doc/${docId}/site/${site}`),
  request: createRequest('/api/fileinfo'),
}

// ─────────────────────────────────────────────────────────────
// 授权日志 / 推荐 / 评论
// ─────────────────────────────────────────────────────────────
export const empowerLogApi = {
  list: (p?: Record<string, string>) => api.get('/api/empowerlog/list', { params: p }),
  request: createRequest('/api/empowerlog'),
}
export const commendApi = {
  list: () => api.get('/api/commend/list'),
  detail: (id: string) => api.get(`/api/commend/${id}`),
  request: createRequest('/api/commend'),
}
export const commentApi = {
  list: () => api.get('/api/comment/list'),
  detail: (id: string) => api.get(`/api/comment/${id}`),
  request: createRequest('/api/comment'),
}
export const complexApi = {
  folderList: () => api.get('/api/complex/folder/list'),
  topFiles: () => api.get('/api/complex/top'),
  request: createRequest('/api/complex'),
}
export const componentApi = {
  list: () => api.get('/api/component_assemble_control/component/list'),
  create: (data: unknown) => api.post('/api/component_assemble_control/component/create', data),
  get: (id: string) => api.get(`/api/component_assemble_control/component/${id}`),
  save: (id: string, data: unknown) => api.put(`/api/component_assemble_control/component/save/${id}`, data),
  delete: (id: string) => api.delete(`/api/component_assemble_control/component/delete/${id}`),
  request: createRequest('/api/component_assemble_control'),
}
export const configApi = {
  isSet: (k: string) => api.get(`/api/config/is/${k}`),
  systemConfig: () => api.get('/api/config/system'),
  request: createRequest('/api/config'),
}
export const editorApi = { list: () => api.get('/api/editor/list') }
export const externalDataSourceApi = {
  list: () => api.get('/api/externaldatasources/list'),
  check: (d: unknown) => api.post('/api/externaldatasources/check', d),
  validate: (d: unknown) => api.post('/api/externaldatasources/validate', d),
  set: (d: unknown) => api.post('/api/externaldatasources/set', d),
  cancel: () => api.post('/api/externaldatasources/set/cancel', null),
}
export const groupApi = {
  list: (f?: string) => api.get(f ? `/api/group/${f}` : '/api/group/list'),
  has: (f: string) => api.get(`/api/group/has/${f}`),
}
export const identityApi = {
  list: () => api.get('/api/identity/list'),
  detail: (id: string) => api.get(`/api/identity/${id}`),
}
export const imageApi = {
  encode: (d: unknown) => api.post('/api/image/encode', d),
  resize: (d: unknown) => api.post('/api/image/resize', d),
}
export const uuidApi = { random: () => api.get('/api/uuid/random') }
export const personAttributeApi = {
  list: () => api.get('/api/personattribute/list'),
  append: (d: unknown) => api.post('/api/personattribute/append', d),
  set: (d: unknown) => api.put('/api/personattribute/set', d),
}
export const unitAttributeApi = {
  list: () => api.get('/api/unitattribute/list'),
  append: (d: unknown) => api.post('/api/unitattribute/append', d),
  set: (d: unknown) => api.put('/api/unitattribute/set', d),
}
export const unitDutyApi = {
  find: (n: string) => api.get(`/api/unitduty/find/${n}`),
  list: () => api.get('/api/unitduty/list'),
}
export const viewCategoryApi = {
  list: () => api.get('/api/viewcategory/list'),
  detail: (id: string) => api.get(`/api/viewcategory/${id}`),
}
export const viewFieldConfigApi = {
  list: () => api.get('/api/viewfieldconfig/list'),
  detail: (id: string) => api.get(`/api/viewfieldconfig/${id}`),
}
export const templateFormApi = {
  list: () => api.get('/api/templateform/list'),
  detail: (id: string) => api.get(`/api/templateform/${id}`),
}
export const exportDetailApi = { appInfo: () => api.get('/api/export/appInfo') }
export const importDetailApi = { appInfo: () => api.get('/api/import/appInfo') }
export const categoryDetailApi = {
  alias: (a: string) => api.get(`/api/categoryinfo/alias/${a}`),
  bind: (categoryId: string, view: string) => api.post(`/api/categoryinfo/bind/${categoryId}/view/${view}`, null),
  erase: (id: string) => api.delete(`/api/categoryinfo/erase/category/${id}`),
  extContent: () => api.get('/api/categoryinfo/extContent'),
  filterList: (id: string, count: number, appId: string) =>
    api.get(`/api/categoryinfo/filter/list/${id}/next/${count}/app/${appId}`),
  flag: () => api.get('/api/categoryinfo/flag'),
  list: () => api.get('/api/categoryinfo/list'),
  detail: (id: string) => api.get(`/api/categoryinfo/${id}`),
}
export const appConfigApi = { get: (appId: string) => api.get(`/api/appconfig/${appId}`) }
export const appDictApi = {
  list: (flag: string) => api.get(`/api/surface/appdict/${flag}`),
  appInfo: (flag: string, appId: string) => api.get(`/api/surface/appdict/${flag}/appInfo/${appId}`),
}

// ─────────────────────────────────────────────────────────────
// 统一导出包（供视图使用）
// ─────────────────────────────────────────────────────────────
export const oa4rustApis = {
  // 原有
  auth: authApi,
  org: orgApi,
  process: processApi,
  portal: portalApi,
  message: messageApi,
  im: imApi,
  file: fileApi,
  general: generalApi,
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
  // 新增
  processService: processServiceApi,
  queryView: queryViewApi,
  programCenter: programCenterApi,
  mind: mindApi,
  document: documentApi,
  calendarDeep: calendarDeepApi,
  attendanceDeep: attendanceDeepApi,
  queryDesigner: queryDesignerApi,
  attachmentDeep: attachmentDeepApi,
  recycle: recycleApi,
  server: serverApi,
  unit: unitApi,
  form: formApi,
  view: viewApi,
  fileInfo: fileInfoApi,
  empowerLog: empowerLogApi,
  commend: commendApi,
  comment: commentApi,
  complex: complexApi,
  component: componentApi,
  config: configApi,
  editor: editorApi,
  externalDataSource: externalDataSourceApi,
  group: groupApi,
  identity: identityApi,
  image: imageApi,
  uuid: uuidApi,
  categoryDetail: categoryDetailApi,
  appConfig: appConfigApi,
  appDict: appDictApi,
  personAttribute: personAttributeApi,
  unitAttribute: unitAttributeApi,
  unitDuty: unitDutyApi,
  viewCategory: viewCategoryApi,
  viewFieldConfig: viewFieldConfigApi,
  templateForm: templateFormApi,
  exportDetail: exportDetailApi,
  importDetail: importDetailApi,
}

// ─────────────────────────────────────────────────────────────
// 工作流表面 (processplatform_assemble_surface — 963 routes)
// ─────────────────────────────────────────────────────────────
export const processplatformSurfaceApi = {
  openapi: () => api.get('/api/processplatform/assemble/surface/openapi'),
  get: (id: string) => api.get(`/api/processplatform/assemble/surface/get/${id}`),
  sign: (id: string) => api.get(`/api/processplatform/assemble/surface/sign/${id}`),
  snap: (id: string) => api.get(`/api/processplatform/assemble/surface/snap/${id}`),
  task: (id: string) => api.get(`/api/processplatform/assemble/surface/task/${id}`),
  work: (id: string) => api.get(`/api/processplatform/assemble/surface/work/${id}`),
  draft: (id: string) => api.get(`/api/processplatform/assemble/surface/draft/${id}`),
  route: (id: string) => api.get(`/api/processplatform/assemble/surface/route/${id}`),
  form: (flag: string) => api.get(`/api/processplatform/assemble/surface/form/${flag}`),
  review: (id: string) => api.get(`/api/processplatform/assemble/surface/review/${id}`),
  preview: (id: string) => api.get(`/api/processplatform/assemble/surface/preview/${id}`),
  handover: (id: string) => api.post(`/api/processplatform/assemble/surface/handover/${id}`, null),
  create: (data: unknown) => api.post('/api/processplatform/assemble/surface/create', data),
  save: (id: string, data: unknown) => api.put(`/api/processplatform/assemble/surface/save/${id}`, data),
  delete: (id: string) => api.delete(`/api/processplatform/assemble/surface/${id}`),
  request: createRequest('/api/processplatform/assemble/surface'),
}

// ─────────────────────────────────────────────────────────────
// CMS 内容管理 (cms_assemble_control — 405 routes)
// ─────────────────────────────────────────────────────────────
export const cmsApi = {
  log: (id: string) => api.get(`/api/cms/log/${id}`),
  file: (id: string) => api.get(`/api/cms/file/${id}`),
  form: (id: string) => api.get(`/api/cms/form/${id}`),
  view: (id: string) => api.get(`/api/cms/view/${id}`),
  script: (id: string) => api.get(`/api/cms/script/${id}`),
  outputList: () => api.get('/api/cms/output/list'),
  create: (type: string, data: unknown) => api.post(`/api/cms/${type}`, data),
  update: (type: string, id: string, data: unknown) => api.put(`/api/cms/${type}/${id}`, data),
  delete: (type: string, id: string) => api.delete(`/api/cms/${type}/${id}`),
  comment: (id: string) => api.get(`/api/cms/comment/${id}`),
  document: (id: string) => api.get(`/api/cms/document/${id}`),
  request: createRequest('/api/cms'),
}

// ─────────────────────────────────────────────────────────────
// 组织控制 (organization_assemble_control — 235 routes)
// ─────────────────────────────────────────────────────────────
export const organizationControlApi = {
  identity: (flag: string) => api.get(`/api/organization/assemble/control/identity/${flag}`),
  role: (flag: string) => api.get(`/api/organization/assemble/control/role/${flag}`),
  unit: (flag: string) => api.get(`/api/organization/assemble/control/unit/${flag}`),
  group: (flag: string) => api.get(`/api/organization/assemble/control/group/${flag}`),
  person: (flag: string) => api.get(`/api/organization/assemble/control/person/${flag}`),
  getRoot: () => api.get('/api/organization/assemble/control/unit/get/root'),
  listTop: () => api.get('/api/organization/assemble/control/unit/list/top'),
  roleListLike: () => api.get('/api/organization/assemble/control/role/list/like'),
  unitListLike: () => api.get('/api/organization/assemble/control/unit/list/like'),
  groupListLike: () => api.get('/api/organization/assemble/control/group/list/like'),
  identityListLike: () => api.get('/api/organization/assemble/control/identity/list/like'),
  create: (type: string, data: unknown) => api.post(`/api/organization/assemble/control/${type}`, data),
  update: (type: string, flag: string, data: unknown) =>
    api.put(`/api/organization/assemble/control/${type}/${flag}`, data),
  delete: (type: string, flag: string) => api.delete(`/api/organization/assemble/control/${type}/${flag}`),
  request: createRequest('/api/organization/assemble/control'),
}

// ─────────────────────────────────────────────────────────────
// 考勤控制 (attendance_assemble_control — 228 routes)
// ─────────────────────────────────────────────────────────────
export const attendanceControlApi = {
  ruleList: () => api.get('/api/attendance/assemble/control/rule/list'),
  v2Config: () => api.get('/api/attendance/assemble/control/v2/config'),
  uuid: () => api.get('/api/attendance/assemble/control/uuid/random'),
  statistic: () => api.post('/api/attendance/assemble/control/statistic/do', null),
  v2Group: (id: string) => api.get(`/api/attendance/assemble/control/v2/group/${id}`),
  v2MyVersion: () => api.get('/api/attendance/assemble/control/v2/my/version'),
  v2Shift: (id: string) => api.get(`/api/attendance/assemble/control/v2/shift/${id}`),
  qywxSyncList: () => api.get('/api/attendance/assemble/control/qywx/sync/list'),
  workplace: (data: unknown) => api.post('/api/attendance/assemble/control/workplace', data),
  v2Workplace: (data: unknown) => api.post('/api/attendance/assemble/control/v2/workplace', data),
  admin: (data: unknown) => api.post('/api/attendance/assemble/control/attendanceadmin', data),
  toggleRule: (id: string) => api.put(`/api/attendance/assemble/control/rule/${id}/toggle`, null),
  auditAppeal: (id: string, data: unknown) =>
    api.put(`/api/attendance/assemble/control/attendanceappealInfo/audit/${id}`, data),
  checkDetail: (params: unknown) => api.post('/api/attendance/assemble/control/attendancedetail/filter/list', params),
  deleteWorkplace: (id: string) => api.delete(`/api/attendance/assemble/control/workplace/${id}`),
  deleteAdmin: (id: string) => api.delete(`/api/attendance/assemble/control/attendanceadmin/${id}`),
  deleteDetail: (id: string) => api.delete(`/api/attendance/assemble/control/attendancedetail/${id}`),
  dingdingSync: () => api.delete('/api/attendance/assemble/control/dingding/all'),
  qywxSync: () => api.delete('/api/attendance/assemble/control/qywx/all'),
  attendanceadmin: () => api.get('/api/attendance/assemble/control/attendanceadmin'),
  attendanceadminX: (id: string) => api.get(`/api/attendance/assemble/control/attendanceadmin/${id}`),
  attendanceappealInfoAudit: (id: string, data: unknown) =>
    api.put(`/api/attendance/assemble/control/attendanceappealInfo/audit/${id}`, data),
  attendanceappealInfoCheck: () => api.get('/api/attendance/assemble/control/attendanceappealInfo/check'),
  attendanceappealInfo: (id: string) => api.get(`/api/attendance/assemble/control/attendanceappealInfo/${id}`),
  attendanceconfigList: () => api.get('/api/attendance/assemble/control/attendanceconfig/list'),
  attendanceconfigSave: (data: unknown) => api.post('/api/attendance/assemble/control/attendanceconfig/save', data),
  attendancedetail: () => api.get('/api/attendance/assemble/control/attendancedetail'),
  attendancedetailAnalyse: (start: string, end: string) =>
    api.get(`/api/attendance/assemble/control/attendancedetail/analyse/${start}/${end}`),
  attendancedetailRecive: () => api.get('/api/attendance/assemble/control/attendancedetail/recive'),
  attendancedetailReciveSingle: () => api.get('/api/attendance/assemble/control/attendancedetail/reciveSingle'),
  attendancedetailX: (id: string) => api.get(`/api/attendance/assemble/control/attendancedetail/${id}`),
  attendanceemployeeconfig: () => api.get('/api/attendance/assemble/control/attendanceemployeeconfig'),
  attendanceemployeeconfigX: (id: string) => api.get(`/api/attendance/assemble/control/attendanceemployeeconfig/${id}`),
  attendanceimportfileinfoX: (id: string) => api.get(`/api/attendance/assemble/control/attendanceimportfileinfo/${id}`),
  attendanceschedulesetting: () => api.get('/api/attendance/assemble/control/attendanceschedulesetting'),
  attendanceschedulesettingX: (id: string) =>
    api.get(`/api/attendance/assemble/control/attendanceschedulesetting/${id}`),
  attendanceselfholiday: () => api.get('/api/attendance/assemble/control/attendanceselfholiday'),
  attendanceselfholidayX: (id: string) => api.get(`/api/attendance/assemble/control/attendanceselfholiday/${id}`),
  attendancesetting: () => api.get('/api/attendance/assemble/control/attendancesetting'),
  attendancesettingX: (id: string) => api.get(`/api/attendance/assemble/control/attendancesetting/${id}`),
  attendancestatisticalcycle: () => api.get('/api/attendance/assemble/control/attendancestatisticalcycle'),
  attendancestatisticalcycleX: (id: string) =>
    api.get(`/api/attendance/assemble/control/attendancestatisticalcycle/${id}`),
  attendancestatisticrequirelog: () => api.get('/api/attendance/assemble/control/attendancestatisticrequirelog'),
  attendancestatisticrequirelogX: (id: string) =>
    api.get(`/api/attendance/assemble/control/attendancestatisticrequirelog/${id}`),
  attendanceworkdayconfig: () => api.get('/api/attendance/assemble/control/attendanceworkdayconfig'),
  attendanceworkdayconfigFilter: () => api.get('/api/attendance/assemble/control/attendanceworkdayconfig/filter'),
  attendanceworkdayconfigX: (id: string) => api.get(`/api/attendance/assemble/control/attendanceworkdayconfig/${id}`),
  selfholidaysimple: () => api.get('/api/attendance/assemble/control/selfholidaysimple'),
  v2Groupschedule: () => api.get('/api/attendance/assemble/control/v2/groupschedule'),
  v2Leave: () => api.get('/api/attendance/assemble/control/v2/leave'),
  attendanceappealInfoX: (id: string) => api.get(`/api/attendance/assemble/control/attendanceappealInfo/${id}`),
  attendanceadminListAll: () => api.get('/api/attendance/assemble/control/attendanceadmin/list/all'),
  attendanceappealInfoFilterList: (page: number, size: number) =>
    api.get(`/api/attendance/assemble/control/attendanceappealInfo/filter/list/${page}/${size}`),
  attendanceappealInfoManagerList: (page: number, size: number) =>
    api.get(`/api/attendance/assemble/control/attendanceappealInfo/manager/list/${page}/${size}`),
  attendanceappealInfoWorkflowAppeal: () =>
    api.get('/api/attendance/assemble/control/attendanceappealInfo/workflow/appeal'),
  attendanceappealInfoWorkflowSync: () =>
    api.post('/api/attendance/assemble/control/attendanceappealInfo/workflow/sync', null),
  attendancedetailAnalyseId: (id: string) =>
    api.get(`/api/attendance/assemble/control/attendancedetail/analyse/id/${id}`),
  attendancedetailAnalyseRedo: () => api.post('/api/attendance/assemble/control/attendancedetail/analyse/redo', null),
  attendancedetailFilterListTopUnit: () =>
    api.get('/api/attendance/assemble/control/attendancedetail/filter/list/topUnit'),
  attendancedetailFilterListUnit: () => api.get('/api/attendance/assemble/control/attendancedetail/filter/list/unit'),
  attendancedetailFilterListUser: () => api.get('/api/attendance/assemble/control/attendancedetail/filter/list/user'),
  attendancedetailListPersonsNonesign: () =>
    api.get('/api/attendance/assemble/control/attendancedetail/list/persons/nonesign'),
  attendancedetailMobileFilterList: () =>
    api.get('/api/attendance/assemble/control/attendancedetail/mobile/filter/list'),
  attendancedetailMobileMy: () => api.get('/api/attendance/assemble/control/attendancedetail/mobile/my'),
  request: createRequest('/api/attendance/assemble/control'),
}
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
  shareList: () => api.get('/api/file/assemble/control/share/list'),
  share: (id: string) => api.get(`/api/file/assemble/control/share/${id}`),
  top: () => api.get('/api/file/assemble/control/complex/top'),
  editorList: () => api.get('/api/file/assemble/control/editor/list'),
  folder: (id: string) => api.get(`/api/file/assemble/control/folder/${id}`),
  fileId: (id: string) => api.get(`/api/file/assemble/control/file/id/${id}`),
  folder2: (id: string) => api.get(`/api/file/assemble/control/folder2/${id}`),
  recycleList: () => api.get('/api/file/assemble/control/recycle/list'),
  shareCreate: (data: unknown) => api.post('/api/file/assemble/control/share', data),
  config: (data: unknown) => api.post('/api/file/assemble/control/config', data),
  folderCreate: (data: unknown) => api.post('/api/file/assemble/control/folder', data),
  folder2Create: (data: unknown) => api.post('/api/file/assemble/control/folder2', data),
  folderUpdate: (id: string, data: unknown) => api.put(`/api/file/assemble/control/folder/${id}`, data),
  folder2Update: (id: string, data: unknown) => api.put(`/api/file/assemble/control/folder2/${id}`, data),
  emptyRecycle: () => api.delete('/api/file/assemble/control/recycle/empty'),
  deleteShare: (id: string) => api.delete(`/api/file/assemble/control/share/${id}`),
  deleteFolder: (id: string) => api.delete(`/api/file/assemble/control/folder/${id}`),
  request: createRequest('/api/file/assemble/control'),
}

// ─────────────────────────────────────────────────────────────
// 会议控制 (meeting_assemble_control — 109 routes)
// ─────────────────────────────────────────────────────────────
export const meetingControlApi = {
  roomList: () => api.get('/api/meeting/assemble/control/room/list'),
  room: (id: string) => api.get(`/api/meeting/assemble/control/room/${id}`),
  openmeeting: () => api.get('/api/meeting/assemble/control/openmeeting'),
  meeting: (id: string) => api.get(`/api/meeting/assemble/control/meeting/${id}`),
  buildingList: () => api.get('/api/meeting/assemble/control/building/list'),
  building: (id: string) => api.get(`/api/meeting/assemble/control/building/${id}`),
  attachment: (id: string) => api.get(`/api/meeting/assemble/control/attachment/${id}`),
  roomCreate: (data: unknown) => api.post('/api/meeting/assemble/control/room', data),
  config: (data: unknown) => api.post('/api/meeting/assemble/control/config', data),
  create: (data: unknown) => api.post('/api/meeting/assemble/control/create', data),
  meetingCreate: (data: unknown) => api.post('/api/meeting/assemble/control/meeting', data),
  buildingCreate: (data: unknown) => api.post('/api/meeting/assemble/control/building', data),
  roomUpdate: (id: string, data: unknown) => api.put(`/api/meeting/assemble/control/room/${id}`, data),
  meetingUpdate: (id: string, data: unknown) => api.put(`/api/meeting/assemble/control/meeting/${id}`, data),
  buildingUpdate: (id: string, data: unknown) => api.put(`/api/meeting/assemble/control/building/${id}`, data),
  roomDelete: (id: string) => api.delete(`/api/meeting/assemble/control/room/${id}`),
  meetingDelete: (id: string) => api.delete(`/api/meeting/assemble/control/meeting/${id}`),
  buildingDelete: (id: string) => api.delete(`/api/meeting/assemble/control/building/${id}`),
  request: createRequest('/api/meeting/assemble/control'),
}

// ─────────────────────────────────────────────────────────────
// 门户表面 (portal_assemble_surface — 72 routes)
// ─────────────────────────────────────────────────────────────
export const portalSurfaceApi = {
  list: () => api.get('/api/portal/assemble/surface/list'),
  preview: (id: string) => api.get(`/api/portal/assemble/surface/${id}/preview`),
  dictX: (flag: string) => api.get(`/api/portal/assemble/surface/dict/${flag}`),
  fileX: (flag: string) => api.get(`/api/portal/assemble/surface/file/${flag}`),
  getFull: (id: string) => api.get(`/api/portal/assemble/surface/get/${id}`),
  inputCompare: () => api.get('/api/portal/assemble/surface/input/compare'),
  inputCover: () => api.get('/api/portal/assemble/surface/input/cover'),
  inputCreate: () => api.post('/api/portal/assemble/surface/input/create', null),
  listAll: () => api.get('/api/portal/assemble/surface/list/all'),
  outputList: () => api.get('/api/portal/assemble/surface/output/list'),
  page: (id: string) => api.get(`/api/portal/assemble/surface/page/${id}`),
  pageCreate: (data: unknown) => api.post('/api/portal/assemble/surface/page/create', data),
  pageX: (flag: string) => api.get(`/api/portal/assemble/surface/page/${flag}`),
  pageversionX: (flag: string) => api.get(`/api/portal/assemble/surface/pageversion/${flag}`),
  portalAll: () => api.get('/api/portal/assemble/surface/portal/all'),
  portalList: () => api.get('/api/portal/assemble/surface/portal/list'),
  portalX: (flag: string) => api.get(`/api/portal/assemble/surface/portal/${flag}`),
  portalcategoryList: () => api.get('/api/portal/assemble/surface/portalcategor y/list'),
  saveX: (id: string, data: unknown) => api.put(`/api/portal/assemble/surface/save/${id}`, data),
  scriptX: (flag: string) => api.get(`/api/portal/assemble/surface/script/${flag}`),
  scriptversion: (id: string) => api.get(`/api/portal/assemble/surface/scriptversion/${id}`),
  templatepage: () => api.get('/api/portal/assemble/surface/templatepage'),
  templatepageList: () => api.get('/api/portal/assemble/surface/templatepage/list'),
  templatepageX: (flag: string) => api.get(`/api/portal/assemble/surface/templatepage/${flag}`),
  widget: () => api.get('/api/portal/assemble/surface/widget'),
  widgetX: (flag: string) => api.get(`/api/portal/assemble/surface/widget/${flag}`),
  get: (id: string) => api.get(`/api/portal/assemble/surface/get/${id}`),
  getLayout: () => api.get('/api/portal/assemble/surface/get/layout'),
  file: (flag: string) => api.get(`/api/portal/assemble/surface/file/${flag}`),
  script: (id: string) => api.get(`/api/portal/assemble/surface/script/${id}`),
  publish: (data: Record<string, unknown>) => api.post('/api/portal/assemble/surface/publish', data),
  create: (data: Record<string, unknown>) => api.post('/api/portal/assemble/surface/create', data),
  saveLayout: (data: Record<string, unknown>) => api.put('/api/portal/assemble/surface/save/layout', data),
  deleteLayout: (data: Record<string, unknown>) => api.delete('/api/portal/assemble/surface/delete/layout', data),
  request: createRequest('/api/portal/assemble/surface'),
}

// ─────────────────────────────────────────────────────────────
// 通用控制 (general_assemble_control — 94 routes)
// ─────────────────────────────────────────────────────────────
export const generalControlApi = {
  status: () => api.get('/api/general/assemble/control/status'),
  areaList: () => api.get('/api/general/assemble/control/area/list'),
  area: (id: string) => api.get(`/api/general/assemble/control/area/${id}`),
  qrCodeList: () => api.get('/api/general/assemble/control/qrcode/list'),
  qrCode: (id: string) => api.get(`/api/general/assemble/control/qrcode/${id}`),
  attendScopeList: () => api.get('/api/general/assemble/control/attendscope/list'),
  office: (data: unknown) => api.post('/api/general/assemble/control/office', data),
  qrCodeCreate: (data: unknown) => api.post('/api/general/assemble/control/qrcode', data),
  areaCreate: (data: unknown) => api.post('/api/general/assemble/control/area/create', data),
  invoiceCreate: (data: unknown) => api.post('/api/general/assemble/control/invoice/create', data),
  statusUpdate: (data: unknown) => api.put('/api/general/assemble/control/status/update', data),
  areaUpdate: (id: string, data: unknown) => api.put(`/api/general/assemble/control/area/update/${id}`, data),
  areaDelete: (id: string) => api.delete(`/api/general/assemble/control/area/delete/${id}`),
  qrCodeDelete: (id: string) => api.delete(`/api/general/assemble/control/qrcode/delete/${id}`),
  request: createRequest('/api/general/assemble/control'),
}

// ─────────────────────────────────────────────────────────────
// 消息通信 (message_assemble_communicate — 78 routes)
// ─────────────────────────────────────────────────────────────
export const messageCommunicateApi = {
  connector: () => api.get('/api/message/assemble/communicate/connector'),
  mass: (id: string) => api.get(`/api/message/assemble/communicate/mass/${id}`),
  wsList: () => api.get('/api/message/assemble/communicate/ws/list/person'),
  wsCount: () => api.get('/api/message/assemble/communicate/ws/count/person'),
  imManagerConfig: () => api.get('/api/message/assemble/communicate/im/manager/config'),
  receive: (consume: string) => api.get(`/api/message/assemble/communicate/receive/${consume}`),
  imMsgList: () => api.get('/api/message/assemble/communicate/im/msg/list/object'),
  imMsgRevoke: (id: string) => api.get(`/api/message/assemble/communicate/im/msg/revoke/${id}`),
  ws: (data: unknown) => api.post('/api/message/assemble/communicate/ws', data),
  massCreate: (data: unknown) => api.post('/api/message/assemble/communicate/mass', data),
  send: (data: unknown) => api.post('/api/message/assemble/communicate/send', data),
  imMsg: (data: unknown) => api.post('/api/message/assemble/communicate/im/msg', data),
  markRead: (id: string) => api.post(`/api/message/assemble/communicate/mark_read/${id}`, null),
  imConversation: (data: unknown) => api.post('/api/message/assemble/communicate/im/conversation', data),
  imConversationUpdate: (id: string, data: unknown) =>
    api.put(`/api/message/assemble/communicate/im/conversation/${id}`, data),
  imConversationRead: (id: string) => api.put(`/api/message/assemble/communicate/im/conversation/${id}/read`, null),
  massDelete: (id: string) => api.delete(`/api/message/assemble/communicate/mass/${id}`),
  imConversationGroup: (id: string) => api.delete(`/api/message/assemble/communicate/im/conversation/${id}/group`),
  request: createRequest('/api/message/assemble/communicate'),
}

/**
 * Extra API modules for remaining backend crates
 * Auto-generated — do not edit manually
 */

// ─────────────────────────────────────────────────────────────
// ai_core_entity (3 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const ai_core_entityApi = {
  getlist: () => api.get('/api/ai/core/entity/app/list'),
  getlist_1: () => api.get('/api/ai/core/entity/model/list'),
  getlist_2: () => api.get('/api/ai/core/entity/conversation/list'),
  // Generic fallback for 0 remaining paths
  request: createRequest('/api/'),
}

// ─────────────────────────────────────────────────────────────
// query_service (3 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const query_serviceApi = {
  getlist: () => api.get('/api/query/service/neural/list'),
  postexecute: (body?: unknown) => api.post('/api/query/service/processing/execute', body),
  postmodelflag: (model_flag: string, body?: unknown) =>
    api.post(`/api/query/service/neural/generate/${encodeURIComponent(model_flag)}`, body),
  // Generic fallback for 0 remaining paths
  request: createRequest('/api/'),
}

// ─────────────────────────────────────────────────────────────
// query_service_processing (4 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const query_service_processingApi = {
  getstatus: () => api.get('/api/query/service/processing/status'),
  postbatch: (body?: unknown) => api.post('/api/query/service/processing/batch', body),
  postreset: (body?: unknown) => api.post('/api/query/service/processing/reset', body),
  postprocess: (body?: unknown) => api.post('/api/query/service/processing/process', body),
  // Generic fallback for 0 remaining paths
  request: createRequest('/api/'),
}

// ─────────────────────────────────────────────────────────────
// empower (16 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const empowerApi = {
  getid: (id: string) => api.get(`/api/person/empower/${encodeURIComponent(id)}`),
  getto: () => api.get('/api/person/empower/list/to'),
  getenable: (id: string) => api.get(`/api/person/empower/${encodeURIComponent(id)}/enable`),
  getdisable: (id: string) => api.get(`/api/person/empower/${encodeURIComponent(id)}/disable`),
  getenable_4: () => api.get('/api/person/empower/list/to/enable'),
  getcurrentperson: () => api.get('/api/person/empower/list/currentperson'),
  getenable_6: () => api.get('/api/person/empower/list/currentperson/enable'),
  postempower: (body?: unknown) => api.post('/api/person/empower', body),
  postmanager: (body?: unknown) => api.post('/api/person/empower/manager', body),
  postenable: (id: string, body?: unknown) => api.post(`/api/person/empower/${encodeURIComponent(id)}/enable`, body),
  postdisable: (id: string, body?: unknown) => api.post(`/api/person/empower/${encodeURIComponent(id)}/disable`, body),
  postsize: (page: string, size: string, body?: unknown) =>
    api.post(
      `/api/person/empower/manager/list/paging/${encodeURIComponent(page)}/size/${encodeURIComponent(size)}`,
      body,
    ),
  // Generic fallback for 4 remaining paths
  request: createRequest('/api/'),
}

// ─────────────────────────────────────────────────────────────
// realtime (3 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const realtimeApi = {
  getrealtime: () => api.get('/ws/realtime'),
  getroomid: (room_id: string) => api.get(`/ws/realtime/room/${encodeURIComponent(room_id)}`),
  getstats: (room_id: string) => api.get(`/ws/realtime/room/${encodeURIComponent(room_id)}/stats`),
  // Generic fallback for 0 remaining paths
  request: createRequest('/api/'),
}

// ─────────────────────────────────────────────────────────────
// base (9 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const baseApi = {
  getecho: () => api.get('/api/base/echo'),
  getget: () => api.get('/api/base/echo/get'),
  getdetail: () => api.get('/api/base/cache/detail'),
  getinfo: () => api.get('/api/base/openapi/info'),
  getflush: () => api.get('/api/base/cache/config/flush'),
  getflush_5: () => api.get('/api/base/cache/commonscript/flush'),
  getfilePath: (filePath: string) => api.get(`/api/base/sysresource/filePath/${encodeURIComponent(filePath)}`),
  getclassName: (className: string) => api.get(`/api/base/fireschedule/classname/${encodeURIComponent(className)}`),
  postcache: (body?: unknown) => api.post('/api/base/cache', body),
  // Generic fallback for 0 remaining paths
  request: createRequest('/api/'),
}

// ─────────────────────────────────────────────────────────────
// preview (2 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const previewApi = {
  postupload: (body?: unknown) => api.post('/api//preview/upload', body),
  postconvert: (body?: unknown) => api.post('/api//preview/convert', body),
  // Generic fallback for 0 remaining paths
  request: createRequest('/api/'),
}

// ─────────────────────────────────────────────────────────────
// signature (3 unique endpoints)
// ─────────────────────────────────────────────────────────────
export const signatureApi = {
  postsign: (body?: unknown) => api.post('/api//signature/pdf/sign', body),
  poststatus: (body?: unknown) => api.post('/api//signature/pdf/status', body),
  postverify: (body?: unknown) => api.post('/api//signature/pdf/verify', body),
  // Generic fallback for 0 remaining paths
  request: createRequest('/api/'),
}

// ─────────────────────────────────────────────────────────────
// AI 聊天 / 配置 (ai + ai_assemble_control — ~90 routes)
// ─────────────────────────────────────────────────────────────
export const aiApi = {
  chatList: (page: number, size: number) => api.get(`/api/ai_assemble_control/chat/list/paging/${page}/${size}`),
  chatCompletionList: (clueId: string, page: number, size: number) =>
    api.get(`/api/ai_assemble_control/chat/list/completion/${clueId}/paging/${page}/${size}`),
  chatDelete: (clueId: string) => api.delete(`/api/ai_assemble_control/chat/delete/${clueId}`),
  chatCompletion: (data: unknown) => api.post('/api/ai_assemble_control/chat/completion', data),
  chatCompletionStream: (data: unknown) => api.post('/api/ai_assemble_control/chat/completion/stream', data),
  configGet: () => api.get('/api/ai_assemble_control/config/get'),
  configBase: () => api.get('/api/ai_assemble_control/config/base/config'),
  configListMcp: (page: number, size: number) =>
    api.get(`/api/ai_assemble_control/config/list/mcp/paging/${page}/${size}`),
  configListModel: (page: number, size: number) =>
    api.get(`/api/ai_assemble_control/config/list/model/paging/${page}/${size}`),
  configEnableModel: () => api.get('/api/ai_assemble_control/config/list/enable/model'),
  configCreateMcp: (data: unknown) => api.post('/api/ai_assemble_control/config/create/mcp', data),
  configCreateModel: (data: unknown) => api.post('/api/ai_assemble_control/config/create/model', data),
  configUpdateMcp: (flag: string, data: unknown) => api.put(`/api/ai_assemble_control/config/update/mcp/${flag}`, data),
  configUpdateModel: (flag: string, data: unknown) =>
    api.put(`/api/ai_assemble_control/config/update/model/${flag}`, data),
  configDeleteMcp: (flag: string) => api.delete(`/api/ai_assemble_control/config/delete/mcp/${flag}`),
  configDeleteModel: (flag: string) => api.delete(`/api/ai_assemble_control/config/delete/model/${flag}`),
  configGetMcp: (flag: string) => api.get(`/api/ai_assemble_control/config/get/mcp/${flag}`),
  configGetModel: (flag: string) => api.get(`/api/ai_assemble_control/config/get/model/${flag}`),
  configSave: (data: unknown) => api.put('/api/ai_assemble_control/config/save', data),
  getControlConfig: () => api.get('/api/ai_assemble_control/get/ai/control/config'),
  getUsageStats: () => api.get('/api/ai_assemble_control/get/usage/stats'),
  listModels: () => api.get('/api/ai_assemble_control/list/ai/models'),
  fileList: (page: number, size: number) => api.get(`/api/ai_assemble_control/file/list/paging/${page}/${size}`),
  file: (flag: string) => api.get(`/api/ai_assemble_control/file/${flag}`),
  fileDownload: (flag: string) => api.get(`/api/ai_assemble_control/file/${flag}/download`),
  fileDelete: (flag: string) => api.delete(`/api/ai_assemble_control/file/delete/${flag}`),
  fileUpload: (data: unknown) => api.post('/api/ai_assemble_control/file/upload', data),
  indexDelete: (flag: string) => api.delete(`/api/ai_assemble_control/index/delete/${flag}`),
  indexList: (page: number, size: number) => api.get(`/api/ai_assemble_control/index/list/paging/${page}/${size}`),
  indexSync: (data: unknown) => api.post('/api/ai_assemble_control/index/sync/to/knowledge', data),
  appList: () => api.get('/api/ai/core/entity/app/list'),
  modelList: () => api.get('/api/ai/core/entity/model/list'),
  conversationList: () => api.get('/api/ai/core/entity/conversation/list'),
  request: createRequest('/api/ai'),
}

// ─────────────────────────────────────────────────────────────
// 角色管理 (role — ~9 routes)
// ─────────────────────────────────────────────────────────────
export const roleApi = {
  list: () => api.get('/api/role/list'),
  get: (flag: string) => api.get(`/api/role/${flag}`),
  create: (data: unknown) => api.post('/api/role', data),
  update: (flag: string, data: unknown) => api.put(`/api/role/${flag}`, data),
  delete: (flag: string) => api.delete(`/api/role/${flag}`),
  listNext: (flag: string, count: number) => api.get(`/api/role/list/${flag}/next/${count}`),
  listPrev: (flag: string, count: number) => api.get(`/api/role/list/${flag}/prev/${count}`),
  expressList: () => api.get('/api/express/role/list'),
  request: createRequest('/api/role'),
}

export const extraApis = {
  ai: aiApi,
  role: roleApi,
  ai_core_entity: ai_core_entityApi,
  query_service: query_serviceApi,
  query_service_processing: query_service_processingApi,
  empower: empowerApi,
  realtime: realtimeApi,
  base: baseApi,
  preview: previewApi,
  signature: signatureApi,
}
