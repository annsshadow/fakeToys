// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端业务 API 层（精选子集）。
 *
 * 所有端点均对齐 oa4rust 后端已注册的 axum 路由（各 crate 的 routes.rs /
 * u2_router.rs / lib.rs 中的 .route 注册），与桌面端 E2E 实跑通过的流程端点一致
 * （/api/task/{id}/complete|reject、processplatform service/processing、
 * message im 族），确保移动端每个按钮打到真实存在的后端能力，而不是 404 的
 * "约定式"路径。类型（O2User）复用 @oa4rust/sdk 的既有契约。
 */

import type { ApiResponse, O2User } from '@oa4rust/sdk'
import { getApiBase, mapi } from './http'

/**
 * 后端 o2server 兼容信封在分页列表端点附带顶层 count（本页条数；
 * 见 shared::response::ActionResult::legacy_success）。sdk 的 ApiResponse
 * 未声明该字段，这里做最小扩展供列表页读取。
 */
export type EnvelopeList<T> = ApiResponse<T[]> & { count?: number; size?: number }

function list<T>(p: Promise<ApiResponse<T[]>>): Promise<EnvelopeList<T>> {
  return p as Promise<EnvelopeList<T>>
}

// ─────────────────────────────────────────────────────────────
// 认证（移动端登录/会话）
// ─────────────────────────────────────────────────────────────
export const authApi = {
  login: (data: { credential: string; password: string; captchaId?: string; captchaAnswer?: string }) =>
    mapi.post<never>('/api/authentication/login', data, { requireAuth: false, discardResponse: true }),
  logout: () => mapi.post<never>('/api/authentication/logout', null, { requireAuth: false, discardResponse: true }),
  who: () => mapi.get<O2User>('/api/authentication/who', { requireAuth: false }),
  refresh: () => mapi.post<never>('/api/authentication/refresh', null, { requireAuth: false, discardResponse: true }),
  captcha: () => mapi.get<{ image: string; id: string }>('/api/authentication/captcha', { requireAuth: false }),
}

// ─────────────────────────────────────────────────────────────
// 流程（待办 / 已办 / 我发起的 + 审批）
//
// 列表读 surface 新栈 x_task / x_taskcompleted / x_work（与桌面 ProcessWork.vue 一致）；
// 审批走 service/processing 引擎的 /api/task/{id}/complete|reject
// （桌面 E2E workflow-runtime 实跑通过的真实流程端点）。
// ─────────────────────────────────────────────────────────────
export interface ProcessTaskRow {
  id: string
  work?: string
  title?: string
  processName?: string
  taskStatus?: string
  createTime?: string
  [key: string]: unknown
}
export interface ProcessWorkRow {
  id: string
  title?: string
  processName?: string
  applicationName?: string
  workStatus?: string
  createTime?: string
  [key: string]: unknown
}

export interface TaskActionPayload {
  data?: Record<string, unknown>
  opinion?: string
  action: 'approve' | 'reject'
}

export const processApi = {
  /** 待我处理（x_task，task_status active/pending/processing）。 */
  pendingList: (page: number, size: number) =>
    list(mapi.get<ProcessTaskRow[]>(`/api/processplatform/assemble/surface/task/list/my/paging/${page}/size/${size}`)),
  /** 我已办（x_taskcompleted）。 */
  completedList: (page: number, size: number) =>
    list(
      mapi.get<ProcessTaskRow[]>(
        `/api/processplatform/assemble/surface/taskcompleted/list/my/paging/${page}/size/${size}`,
      ),
    ),
  /** 我发起的（x_work）。注意后端此路由仅注册 POST。 */
  startedList: (page: number, size: number) =>
    list(
      mapi.post<ProcessWorkRow[]>(`/api/processplatform/assemble/surface/work/list/my/paging/${page}/size/${size}`, {}),
    ),
  /** 审批通过：任务置 completed，自动认领下一活动或收尾工作。后端仅读 opinion。 */
  completeTask: (taskId: string, payload?: Omit<TaskActionPayload, 'action'>) =>
    mapi.post<never>(
      `/api/task/${taskId}/complete`,
      { opinion: payload?.opinion ?? '' },
      {
        discardResponse: true,
      },
    ),
  /** 驳回：任务回退并记录处理意见。后端仅读 opinion。 */
  rejectTask: (taskId: string, payload?: Omit<TaskActionPayload, 'action'>) =>
    mapi.post<never>(
      `/api/task/${taskId}/reject`,
      { opinion: payload?.opinion ?? '' },
      {
        discardResponse: true,
      },
    ),
  /** 可发起的流程清单（桌面 ProcessWork「发起流程」同源端点）。 */
  startableProcesses: () =>
    list(mapi.get<Record<string, unknown>[]>('/api/processplatform/assemble/designer/list/all')),
  /** 流程定义（用于取绑定表单 flag）。 */
  getProcess: (processId: string) =>
    mapi.get<Record<string, unknown>>(`/api/processplatform/assemble/designer/get/${processId}`),
  /** 表单定义（O2OA moduleList JSON；移动端做简版渲染）。 */
  getForm: (formFlag: string) => mapi.get<Record<string, unknown>>(`/api/form/${formFlag}`),
  /** 发起：创建工作实例，返回 work id（与桌面 submitStart 一致）。 */
  startWork: (processId: string, title: string) =>
    mapi.post<{ id?: string }>('/api/processplatform/service/processing/work', { process: processId, title }),
  /** 填报：把表单数据写回工作实例（仅当流程绑定了表单时调用）。 */
  saveWorkData: (workId: string, values: Record<string, unknown>) =>
    mapi.put<never>(`/api/processplatform/service/processing/data/work/${workId}`, values, { discardResponse: true }),
  /** 工作详情：表单数据（x_data，按 work id）。 */
  workData: (workId: string) =>
    mapi.get<Record<string, unknown>>(`/api/processplatform/assemble/surface/data/work/${workId}`),
  /** 工作详情：附件清单（PP_C_ATTACHMENT，按 work id）。 */
  attachments: (workId: string) =>
    list(
      mapi.get<Record<string, unknown>[]>(`/api/processplatform/assemble/surface/attachment/list/work/${workId}`),
    ),
  /** 工作详情：流转记录（PP_C_RECORD，按 workOrWorkCompleted）。 */
  records: (workId: string) =>
    list(
      mapi.get<Record<string, unknown>[]>(
        `/api/processplatform/assemble/surface/record/list/workorworkcompleted/${workId}`,
      ),
    ),
  /** 工作详情：工作日志（PP_C_WORKLOG，按 workOrWorkCompleted）。 */
  worklogs: (workId: string) =>
    list(
      mapi.get<Record<string, unknown>[]>(
        `/api/processplatform/assemble/surface/worklog/list/workorworkcompleted/${workId}`,
      ),
    ),
}

// ─────────────────────────────────────────────────────────────
// 消息（IM 会话 / 收发）
// ─────────────────────────────────────────────────────────────
export interface ConversationRow {
  id: string
  name?: string
  type?: string
  lastMessage?: string
  [key: string]: unknown
}
export interface MessageRow {
  id: string
  conversationId?: string
  content?: string
  sender?: string
  type?: string
  createTime?: string
  [key: string]: unknown
}

/**
 * 取消息行所属会话 ID。返回空串表示无会话归属。
 */
export function messageConversationId(row: Record<string, unknown>): string {
  const plain = row.conversationId
  return typeof plain === 'string' ? plain : ''
}

export const messageApi = {
  conversationList: () =>
    list(mapi.get<ConversationRow[]>('/api/message/assemble/communicate/im/conversation/list/my')),
  /** 会话内消息历史（后端按 body.conversationId 过滤）。 */
  msgHistory: (conversationId: string, page: number, size: number) =>
    list(
      mapi.post<MessageRow[]>(`/api/message/assemble/communicate/im/msg/list/${page}/size/${size}`, {
        conversationId,
      }),
    ),
  /** 真实写入 x_message（sent=true 表示落库成功）。 */
  send: (conversationId: string, content: string, sender: string) =>
    mapi.post<{ sent?: boolean }>('/api/message/assemble/communicate/im/msg', {
      conversationId,
      content,
      sender,
      type: 'text',
    }),
  markRead: (conversationId: string) =>
    mapi.post(`/api/message/assemble/communicate/im/conversation/${conversationId}/read`, undefined),
  /**
   * 发起单聊：后端 im_conversation 创建一条 type=single 会话并返回新会话 id。
   * 传对方姓名作为会话名；随后用 send() 在该会话内发消息即可。
   */
  startConversation: (name: string) =>
    mapi.post<{ id?: string; name?: string; type?: string; created?: boolean }>(
      '/api/message/assemble/communicate/im/conversation',
      {
        name,
        type: 'single',
      },
    ),
}

// ─────────────────────────────────────────────────────────────
// 文件 / 文档
// ─────────────────────────────────────────────────────────────
export interface FileRow {
  id: string
  name?: string
  path?: string
  size?: number
  creator?: string
  createTime?: string
  [key: string]: unknown
}

export const fileApi = {
  /** folderId 对应 x_file.folder_id（移动端以当前用户 unique 作为"我的文件"目录）。 */
  fileList: (folderId: string) => list(mapi.get<FileRow[]>(`/api/file/assemble/control/file/list/${folderId}`)),
  /** 真实下载路由；拼 apiBase 以支持原生 App / 小程序绝对地址场景。 */
  fileDownloadUrl: (fileId: string) => `${getApiBase()}/api/file/assemble/control/file/${fileId}/download`,
  /**
   * 二进制上传：multipart（file 字段 + 可选 name）写入附件存储 FILE_FILE。
   * filePath 为本端临时文件路径（uni.chooseImage/chooseMessageFile 返回值）。
   * 落点是附件存储（FILE_FILE），而非「我的文件」x_file 列表。
   */
  upload: (folderId: string, filePath: string, fileName: string) =>
    mapi.upload<Record<string, unknown>>(`/api/attachment/upload/folder/${folderId}`, filePath, {
      name: 'file',
      formData: { name: fileName },
    }),
  /** 附件存储 FILE_FILE：当前用户的附件清单（上传落点即在此列出，而非 x_file）。 */
  attachmentList: (owner: string) => list(mapi.get<FileRow[]>(`/api/attachment/list/editor/${owner}`)),
  /** 附件存储 FILE_FILE 下载（拼 apiBase 支持原生 App / 小程序绝对地址）。 */
  attachmentDownloadUrl: (attId: string) => `${getApiBase()}/api/attachment/${attId}/download`,
}

// ─────────────────────────────────────────────────────────────
// 考勤（移动端本人打卡）
// ─────────────────────────────────────────────────────────────
export interface AttendancePreCheck {
  date?: string
  group?: { id?: string; groupName?: string; checkType?: string; workPlaceIdList?: string } | null
  canCheckIn?: boolean
  records?: Array<{
    id?: string
    checkInType?: string
    checkInResult?: string
    sourceType?: string
    createTime?: string
  }>
}

export const attendanceApi = {
  /** 今日状态：命中考勤组 + 今日已有打卡记录。 */
  preCheck: () => mapi.get<AttendancePreCheck>('/api/attendance/assemble/control/v2/mobile/check/pre'),
  /** 打卡：checkInType 取 'checkIn'（上班）/ 'checkOut'（下班）；重复打卡返回 duplicated=true。 */
  check: (checkInType: 'checkIn' | 'checkOut') =>
    mapi.post<{ id?: string; duplicated?: boolean }>('/api/attendance/assemble/control/v2/mobile/check', {
      checkInType,
      sourceType: '移动端',
    }),
  /** 出勤统计（按日聚合，工作台今日概览用）。 */
  statistics: () =>
    list(mapi.get<Array<{ date?: string; records?: number; status?: string }>>('/api/attendance/assemble/control/statistics/list')),
}

// ─────────────────────────────────────────────────────────────
// 组织（通讯录）
// ─────────────────────────────────────────────────────────────
export interface PersonRow {
  id?: string
  unique?: string
  flag?: string
  name?: string
  mobile?: string
  email?: string
  [key: string]: unknown
}

export interface UnitRow {
  id?: string
  name?: string
  parentId?: string
  level?: number
  [key: string]: unknown
}
export interface NamedRow {
  id?: string
  name?: string
  [key: string]: unknown
}

export const orgApi = {
  /** 全员 / 按姓名模糊搜索（POST mockputtopost 别名，body.key 为空返回全员）。 */
  personSearch: (key?: string) =>
    list(mapi.post<PersonRow[]>('/api/organization/assemble/control/person/list/like/mockputtopost', { key })),
  /** 人员详情。 */
  personDetail: (flag: string) => mapi.get<PersonRow>(`/api/organization/assemble/control/person/${flag}`),
  /** 单位清单（阶段 G / F1：通讯录组织维度）。 */
  unitList: () => list(mapi.get<UnitRow[]>('/api/unit/list/all')),
  /** 单位树（后端此路由仅注册 POST）。 */
  unitTree: () => list(mapi.post<UnitRow[]>('/api/unit/list/unit/tree', {})),
  /** 群组清单（group/list/like 返回全部群组，无需 flag）。 */
  groupList: () => list(mapi.get<NamedRow[]>('/api/organization/assemble/control/group/list/like')),
  /** 某群组的下级（嵌套）。 */
  groupSubNested: (flag: string) =>
    list(mapi.get<NamedRow[]>(`/api/organization/assemble/control/group/list/${flag}/sub/nested`)),
  /** 某人的身份清单。 */
  personIdentities: (personFlag: string) =>
    list(mapi.get<NamedRow[]>(`/api/organization/assemble/control/identity/list/person/${personFlag}`)),
  /** 某人的角色清单。 */
  personRoles: (personFlag: string) =>
    list(mapi.get<NamedRow[]>(`/api/organization/assemble/control/role/list/person/${personFlag}`)),
}

// ─────────────────────────────────────────────────────────────
// 公告（x_ai_ann，ai crate 已注册实装路由）
// ─────────────────────────────────────────────────────────────
export interface AnnRow {
  id?: string
  title?: string
  content?: string
  category?: string
  status?: string
  creator?: string
  createTime?: string
  [key: string]: unknown
}

export const annApi = {
  /** 公告列表（create_time 倒序，后端实表 x_ai_ann）。 */
  list: () => list(mapi.get<AnnRow[]>('/api/ai/assemble/control/ann/list')),
}

// ─────────────────────────────────────────────────────────────
// 会议（阶段 G / F2：我的会议 + 接受/拒绝/确认/签到）
//
// 全部对齐 meeting_assemble_control 已注册路由（见 routes.rs）。
// ─────────────────────────────────────────────────────────────
export interface MeetingRow {
  id: string
  title?: string
  name?: string
  startTime?: string
  endTime?: string
  status?: string
  buildingId?: string
  roomId?: string
  [key: string]: unknown
}

export const meetingApi = {
  /** 我申请/参与的会议（分页）。 */
  applied: (page: number, size: number) =>
    list(mapi.get<MeetingRow[]>(`/api/meeting/assemble/control/meeting/list/apply/${page}/size/${size}`)),
  /** 待我接受的邀请。 */
  invitedWait: () => list(mapi.get<MeetingRow[]>('/api/meeting/assemble/control/meeting/list/invited/wait')),
  /** 待我确认（已接受、待确认）。 */
  waitConfirm: () => list(mapi.get<MeetingRow[]>('/api/meeting/assemble/control/meeting/list/wait/confirm')),
  /** 未来 N 天会议。 */
  comingDay: (count: number) =>
    list(mapi.get<MeetingRow[]>(`/api/meeting/assemble/control/meeting/list/coming/day/${count}`)),
  detail: (id: string) => mapi.get<MeetingRow>(`/api/meeting/assemble/control/meeting/${id}`),
  /** 接受邀请。 */
  accept: (id: string) =>
    mapi.post<never>(`/api/meeting/assemble/control/meeting/${id}/accept`, null, { discardResponse: true }),
  /** 拒绝邀请。 */
  reject: (id: string) =>
    mapi.post<never>(`/api/meeting/assemble/control/meeting/${id}/reject`, null, { discardResponse: true }),
  /** 确认出席。 */
  confirmAllow: (id: string) =>
    mapi.post<never>(`/api/meeting/assemble/control/meeting/${id}/confirm/allow`, null, {
      discardResponse: true,
    }),
  /** 确认缺席。 */
  confirmDeny: (id: string) =>
    mapi.post<never>(`/api/meeting/assemble/control/meeting/${id}/confirm/deny`, null, {
      discardResponse: true,
    }),
  /** 签到。 */
  checkin: (id: string) =>
    mapi.post<never>(`/api/meeting/assemble/control/meeting/${id}/checkin`, null, { discardResponse: true }),
  /** 签到码（二维码内容）。 */
  checkinCode: (id: string) => mapi.get<{ code?: string }>(`/api/meeting/assemble/control/meeting/${id}/checkin/code`),
}

// ─────────────────────────────────────────────────────────────
// 日历（阶段 G / F3：我的日历 + 日程事件）
// ─────────────────────────────────────────────────────────────
export interface CalendarRow {
  id: string
  name?: string
  type?: string
  [key: string]: unknown
}
export interface CalendarEventRow {
  id: string
  title?: string
  startTime?: string
  endTime?: string
  calendarId?: string
  [key: string]: unknown
}

export const calendarApi = {
  /** 我的日历。 */
  myCalendars: () => list(mapi.get<CalendarRow[]>('/api/calendar_assemble_control/calendar/list/my')),
  /** 公开日历。 */
  publicCalendars: () => list(mapi.get<CalendarRow[]>('/api/calendar_assemble_control/calendar/list/public')),
  calendarDetail: (id: string) => mapi.get<CalendarRow>(`/api/calendar_assemble_control/calendar/${id}`),
  /** 事件列表（filter，body 传时间范围；后端此路由仅注册 PUT）。 */
  eventsFilter: (payload: Record<string, unknown>) =>
    list(mapi.put<CalendarEventRow[]>('/api/calendar_assemble_control/event/list/filter', payload)),
  eventDetail: (id: string) => mapi.get<CalendarEventRow>(`/api/calendar_assemble_control/event/${id}`),
  /** iCalendar(RFC5545) 内容。 */
  eventRfc: (id: string) => mapi.get<{ rfc?: string }>(`/api/calendar_assemble_control/event/rfc/${id}`),
  /** 日历控制配置。 */
  controlConfig: () => mapi.get<Record<string, unknown>>('/api/calendar_assemble_control/get/control/config'),
  /** 我可见的日历清单（控制面）。 */
  controlCalendars: () =>
    list(mapi.get<CalendarRow[]>('/api/calendar_assemble_control/list/control/calendars')),
}

// ─────────────────────────────────────────────────────────────
// 门户表面 / 查询表面 / 内容 CMS（阶段 G / F4：只读浏览）
// ─────────────────────────────────────────────────────────────
export const portalApi = {
  /** 门户页面清单。 */
  pageList: (portal: string) =>
    list(mapi.get<Record<string, unknown>[]>(`/api/portal/assemble/surface/page/list/portal/${portal}`)),
  pageDetail: (id: string) => mapi.get<Record<string, unknown>>(`/api/portal/assemble/surface/page/v2/${id}`),
  /** 移动端页面视图（含移动布局）。 */
  pageMobile: (id: string) => mapi.get<Record<string, unknown>>(`/api/portal/assemble/surface/page/v2/${id}/mobile`),
  pageByFlag: (flag: string, portalFlag: string) =>
    mapi.get<Record<string, unknown>>(`/api/portal/assemble/surface/page/v2/${flag}/portal/${portalFlag}`),
}

export const queryviewApi = {
  /** 视图清单（按查询 flag 过滤；'all' 表示不限）。 */
  viewList: (queryFlag: string) =>
    list(mapi.get<Record<string, unknown>[]>(`/api/queryview/view/list/query/${queryFlag}`)),
  /** 执行视图（handler 只读 id，view 段为语义占位）。 */
  execute: (view: string, id: string) =>
    mapi.get<Record<string, unknown>>(`/api/queryview/execute/${view}/${id}`),
  /** 分页执行。 */
  executeV2: (view: string, id: string, page: number, size: number) =>
    mapi.get<Record<string, unknown>>(`/api/queryview/execute/v2/${view}/${id}/${page}/${size}`),
  /** 视图 + 数据一次性返回。 */
  bundle: (view: string, id: string) => mapi.get<Record<string, unknown>>(`/api/queryview/bundle/${view}/${id}`),
  bundleV2: (view: string, id: string) => mapi.get<Record<string, unknown>>(`/api/queryview/bundle/v2/${view}/${id}`),
  /** 查询清单。 */
  list: () => list(mapi.get<Record<string, unknown>[]>('/api/queryview/list')),
}

export const cmsApi = {
  /** CMS 栏目清单。 */
  columnList: () => list(mapi.get<Record<string, unknown>[]>('/api/cms/core/entity/column/list')),
  /** 栏目管理清单（含未发布）。 */
  columnManagerList: () =>
    list(mapi.get<Record<string, unknown>[]>('/api/cms/core/entity/column_manager/list')),
  /** 文档全文检索（后端 GET，条件走 query：q 关键字 + limit）。 */
  documentSearch: (q: string, limit = 20) =>
    list(
      mapi.get<Record<string, unknown>[]>('/api/cms_assemble_control/document/search', {
        params: { q, limit: String(limit) },
      }),
    ),
}

// ─────────────────────────────────────────────────────────────
// 回收站 / 搜索 / 统计（阶段 G / F5）
// ─────────────────────────────────────────────────────────────
export const recycleApi = {
  list: () => list(mapi.get<Record<string, unknown>[]>('/api/recycle/list')),
  detail: (id: string) => mapi.get<Record<string, unknown>>(`/api/recycle/${id}`),
  remove: (id: string) => mapi.delete<never>(`/api/recycle/delete/${id}`, { discardResponse: true }),
  /** 清空回收站（后端此路由仅注册 DELETE）。 */
  empty: () => mapi.delete<never>('/api/recycle/empty', { discardResponse: true }),
}

export const searchApi = {
  /** 全局检索（queryview，后端仅注册 POST，读取键为 key）。 */
  global: (keyword: string) => list(mapi.post<Record<string, unknown>[]>('/api/queryview/search', { key: keyword })),
  /** 论坛主题检索。 */
  bbsSubject: (keyword: string) =>
    list(
      mapi.get<Record<string, unknown>[]>(
        `/api/bbs/assemble/control/subject/search?keyword=${encodeURIComponent(keyword)}`,
      ),
    ),
}

export const statisticsApi = {
  /** 考勤统计周期清单。 */
  attendanceCycles: () =>
    list(mapi.get<Record<string, unknown>[]>('/api/attendance/assemble/control/attendancestatisticalcycle/list/all')),
  /** 某月统计明细。 */
  attendanceCycleDetail: (year: number, month: number) =>
    mapi.get<Record<string, unknown>>(
      `/api/attendance/assemble/control/attendancestatisticalcycle/cycleDetail/${year}/${month}`,
    ),
  /** 个人月度统计。 */
  attendancePersonMonth: (year: number, month: number) =>
    mapi.get<Record<string, unknown>>(
      `/api/attendance/assemble/control/dingding/statistic/person/year/${year}/month/${month}`,
    ),
}

// ─────────────────────────────────────────────────────────────
// 推送设备（阶段 G / F5：设备注册状态只读）
// ─────────────────────────────────────────────────────────────
export const pushApi = {
  /** 某推送类型下的设备清单。 */
  deviceList: (pushType: string) =>
    list(mapi.get<Record<string, unknown>[]>(`/api/jpush/assemble/control/device/list/${pushType}`)),
  /** 推送类型配置。 */
  pushTypeConfig: () =>
    mapi.get<Record<string, unknown>>('/api/jpush/assemble/control/device/config/push/type'),
  /** 推送应用清单。 */
  apps: () => list(mapi.get<Record<string, unknown>[]>('/api/jpush/assemble/control/list/control/apps')),
}

// ─────────────────────────────────────────────────────────────
// 通用（字典）
// ─────────────────────────────────────────────────────────────
export const generalApi = {
  dictList: () => mapi.get('/api/general/dict/list'),
}

export const apis = {
  auth: authApi,
  process: processApi,
  message: messageApi,
  file: fileApi,
  attendance: attendanceApi,
  org: orgApi,
  ann: annApi,
  general: generalApi,
  meeting: meetingApi,
  calendar: calendarApi,
  portal: portalApi,
  queryview: queryviewApi,
  cms: cmsApi,
  recycle: recycleApi,
  search: searchApi,
  statistics: statisticsApi,
  push: pushApi,
}

export default apis
