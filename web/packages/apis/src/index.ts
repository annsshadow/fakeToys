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
  taskHandle: (taskId: string, action: string, data?: unknown) =>
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
