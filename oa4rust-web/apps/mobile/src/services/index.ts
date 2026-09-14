/**
 * 移动端业务 API 层（精选子集）。
 *
 * 端点路径与 @oa4rust/apis（桌面端）保持一致，确保与 oa4rust 后端的契约对齐；
 * 但移动端只暴露高频、适合触控交互的模块，避免把 3892 条路由全部拖进移动包。
 * 类型（O2User / OrgGroup / PagedResponse）复用 @oa4rust/sdk 的既有契约。
 */

import type { O2User, OrgGroup, PagedResponse } from '@oa4rust/sdk'
import { mapi } from './http'

// ─────────────────────────────────────────────────────────────
// 认证（移动端登录/会话）
// ─────────────────────────────────────────────────────────────
export const authApi = {
  login: (data: { credential: string; password: string; captchaId?: string; captchaAnswer?: string }) =>
    mapi.post<never>('/jaxrs/authentication/login', data, { requireAuth: false, discardResponse: true }),
  logout: () => mapi.post<never>('/jaxrs/authentication/logout', null, { requireAuth: false, discardResponse: true }),
  who: () => mapi.get<O2User>('/jaxrs/authentication/who', { requireAuth: false }),
  refresh: () => mapi.post<never>('/jaxrs/authentication/refresh', null, { requireAuth: false, discardResponse: true }),
  captcha: () => mapi.get<{ image: string; id: string }>('/jaxrs/authentication/captcha', { requireAuth: false }),
}

// ─────────────────────────────────────────────────────────────
// 工作台 / 门户
// ─────────────────────────────────────────────────────────────
export const portalApi = {
  pageList: (appId: string) => mapi.get(`/jaxrs/portal/assemble/surface/page/list/${appId}`),
  widgetList: (pageId: string) => mapi.get(`/jaxrs/portal/assemble/surface/widget/list/${pageId}`),
}

// ─────────────────────────────────────────────────────────────
// 消息（IM 会话）
// ─────────────────────────────────────────────────────────────
export const messageApi = {
  conversationList: (page: number, size: number) =>
    mapi.post<PagedResponse<unknown>>(
      `/jaxrs/message/assemble/communicate/conversation/list/paging/${page}/${size}`,
      undefined,
    ),
  msgHistory: (conversationId: string, page: number, size: number) =>
    mapi.post<PagedResponse<unknown>>(
      `/jaxrs/message/assemble/communicate/history/${conversationId}/paging/${page}/${size}`,
      undefined,
    ),
  markRead: (conversationId: string) =>
    mapi.post(`/jaxrs/message/assemble/communicate/conversation/${conversationId}/read`, undefined),
}

// ─────────────────────────────────────────────────────────────
// 流程（待办 / 审批）
// ─────────────────────────────────────────────────────────────
export const processApi = {
  workList: (page: number, size: number, status?: string) =>
    mapi.post<PagedResponse<unknown>>(`/jaxrs/processplatform/assemble/surface/work/list/paging/${page}/${size}`, {
      status,
    }),
  taskList: (page: number, size: number) =>
    mapi.post<PagedResponse<unknown>>(
      `/jaxrs/processplatform/assemble/surface/task/list/paging/${page}/${size}`,
      undefined,
    ),
  taskHandle: (taskId: string, action: string, data?: Record<string, unknown>) =>
    mapi.post(`/jaxrs/processplatform/assemble/surface/task/${taskId}/handle`, { action, ...data }),
  completedList: (page: number, size: number) =>
    mapi.post<PagedResponse<unknown>>(
      `/jaxrs/processplatform/assemble/surface/workcompleted/list/paging/${page}/${size}`,
      undefined,
    ),
}

// ─────────────────────────────────────────────────────────────
// 文件 / 文档
// ─────────────────────────────────────────────────────────────
export const fileApi = {
  fileList: (folderId?: string, page?: number, size?: number) =>
    mapi.post<PagedResponse<unknown>>('/jaxrs/file/assemble/control/file/list', {
      folderId,
      page,
      size,
    }),
  folderList: (parentId?: string) => mapi.get(`/jaxrs/file/assemble/control/folder/list/${parentId || ''}`),
  /** 返回下载 URL，由调用方经 uni.downloadFile + uni.openDocument 处理。 */
  fileDownloadUrl: (fileId: string) => `/jaxrs/file/core/entity/file/${fileId}/download`,
  attachmentList: (fileId: string) => mapi.get(`/jaxrs/file/assemble/control/attachment/list/${fileId}`),
}

// ─────────────────────────────────────────────────────────────
// 组织（人员 / 部门）
// ─────────────────────────────────────────────────────────────
export const orgApi = {
  personList: (page: number, size: number, keyword?: string) =>
    mapi.post<PagedResponse<O2User>>(`/jaxrs/organization/assemble/control/person/list/paging/${page}/${size}`, {
      keyword,
    }),
  groupList: (flag?: string, count?: number) =>
    mapi.get(`/jaxrs/organization/assemble/control/group/list/${flag || ''}/next/${count || 20}`),
  groupDetail: (flag: string) =>
    mapi.get<{ groups?: OrgGroup[] }>(`/jaxrs/organization/assemble/control/group/${flag}`),
}

// ─────────────────────────────────────────────────────────────
// 通用（字典 / 工时）
// ─────────────────────────────────────────────────────────────
export const generalApi = {
  dictList: () => mapi.get('/jaxrs/general/dict/list'),
  worktimeList: (month: string) => mapi.get(`/jaxrs/general/assemble/control/worktime/${month}`),
}

export const apis = {
  auth: authApi,
  portal: portalApi,
  message: messageApi,
  process: processApi,
  file: fileApi,
  org: orgApi,
  general: generalApi,
}

export default apis
