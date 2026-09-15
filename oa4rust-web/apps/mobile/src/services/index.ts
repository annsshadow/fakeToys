/**
 * 移动端业务 API 层（精选子集）。
 *
 * 所有端点均对齐 oa4rust 后端已注册的 axum 路由（各 crate 的 routes.rs /
 * u2_router.rs / lib.rs 中的 .route 注册），与桌面端 E2E 实跑通过的流程端点一致
 * （/jaxrs/task/{id}/complete|reject、processplatform service/processing、
 * message im 族），确保移动端每个按钮打到真实存在的后端能力，而不是 404 的
 * "约定式"路径。类型（O2User）复用 @oa4rust/sdk 的既有契约。
 */

import type { ApiResponse, O2User } from '@oa4rust/sdk'
import { getApiBase, mapi } from './http'

/**
 * 后端 Java 兼容信封在分页列表端点附带顶层 count（本页条数；
 * 见 shared::response::ActionResult::java_success）。sdk 的 ApiResponse
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
    mapi.post<never>('/jaxrs/authentication/login', data, { requireAuth: false, discardResponse: true }),
  logout: () => mapi.post<never>('/jaxrs/authentication/logout', null, { requireAuth: false, discardResponse: true }),
  who: () => mapi.get<O2User>('/jaxrs/authentication/who', { requireAuth: false }),
  refresh: () => mapi.post<never>('/jaxrs/authentication/refresh', null, { requireAuth: false, discardResponse: true }),
  captcha: () => mapi.get<{ image: string; id: string }>('/jaxrs/authentication/captcha', { requireAuth: false }),
}

// ─────────────────────────────────────────────────────────────
// 流程（待办 / 已办 / 我发起的 + 审批）
//
// 列表读 surface 新栈 x_task / x_taskcompleted / x_work（与桌面 ProcessWork.vue 一致）；
// 审批走 service/processing 引擎的 /jaxrs/task/{id}/complete|reject
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
    list(
      mapi.get<ProcessTaskRow[]>(`/jaxrs/processplatform/assemble/surface/task/list/my/paging/${page}/size/${size}`),
    ),
  /** 我已办（x_taskcompleted）。 */
  completedList: (page: number, size: number) =>
    list(
      mapi.get<ProcessTaskRow[]>(
        `/jaxrs/processplatform/assemble/surface/taskcompleted/list/my/paging/${page}/size/${size}`,
      ),
    ),
  /** 我发起的（x_work）。注意后端此路由仅注册 POST。 */
  startedList: (page: number, size: number) =>
    list(
      mapi.post<ProcessWorkRow[]>(
        `/jaxrs/processplatform/assemble/surface/work/list/my/paging/${page}/size/${size}`,
        {},
      ),
    ),
  /** 审批通过：任务置 completed，自动认领下一活动或收尾工作。 */
  completeTask: (taskId: string, payload?: Omit<TaskActionPayload, 'action'>) =>
    mapi.post<never>(
      `/jaxrs/task/${taskId}/complete`,
      { data: {}, opinion: '', action: 'approve', ...payload },
      {
        discardResponse: true,
      },
    ),
  /** 驳回：任务回退并记录处理意见。 */
  rejectTask: (taskId: string, payload?: Omit<TaskActionPayload, 'action'>) =>
    mapi.post<never>(
      `/jaxrs/task/${taskId}/reject`,
      { data: {}, opinion: '', action: 'reject', ...payload },
      {
        discardResponse: true,
      },
    ),
  /** 可发起的流程清单（桌面 ProcessWork「发起流程」同源端点）。 */
  startableProcesses: () =>
    list(mapi.get<Record<string, unknown>[]>('/jaxrs/processplatform/assemble/designer/list/all')),
  /** 流程定义（用于取绑定表单 flag）。 */
  getProcess: (processId: string) =>
    mapi.get<Record<string, unknown>>(`/jaxrs/processplatform/assemble/designer/get/${processId}`),
  /** 表单定义（O2OA moduleList JSON；移动端做简版渲染）。 */
  getForm: (formFlag: string) => mapi.get<Record<string, unknown>>(`/jaxrs/form/${formFlag}`),
  /** 发起：创建工作实例，返回 work id（与桌面 submitStart 一致）。 */
  startWork: (processId: string, title: string) =>
    mapi.post<{ id?: string }>('/jaxrs/processplatform/service/processing/work', { process: processId, title }),
  /** 填报：把表单数据写回工作实例（仅当流程绑定了表单时调用）。 */
  saveWorkData: (workId: string, values: Record<string, unknown>) =>
    mapi.put<never>(`/jaxrs/processplatform/service/processing/data/work/${workId}`, values, { discardResponse: true }),
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
 * 取消息行所属会话 ID。后端生成 handler（O2OA 遗留约定）把会话键序列化为
 * 带引号字面量 `"conversationId"`（JSON 键本身含引号字符），前端解析后需按该
 * 键读取；同时兼容普通键。返回空串表示无会话归属。
 */
export function messageConversationId(row: Record<string, unknown>): string {
  const quoted = row['"conversationId"']
  if (typeof quoted === 'string' && quoted) return quoted
  const plain = row.conversationId
  return typeof plain === 'string' ? plain : ''
}

export const messageApi = {
  conversationList: () =>
    list(mapi.get<ConversationRow[]>('/jaxrs/message/assemble/communicate/im/conversation/list/my')),
  msgHistory: (page: number, size: number) =>
    list(mapi.get<MessageRow[]>(`/jaxrs/message/assemble/communicate/im/msg/list/${page}/size/${size}`)),
  /**
   * 真实写入 x_message（sent=true 表示落库成功）。后端按带引号键
   * `"conversationId"` 读取会话归属，这里同时下发两种键保证可读。
   */
  send: (conversationId: string, content: string, sender: string) =>
    mapi.post<{ sent?: boolean }>('/jaxrs/message/assemble/communicate/im/msg', {
      ['"conversationId"']: conversationId,
      conversationId,
      content,
      sender,
      type: 'text',
    }),
  markRead: (conversationId: string) =>
    mapi.post(`/jaxrs/message/assemble/communicate/im/conversation/${conversationId}/read`, undefined),
  /**
   * 发起单聊：后端 im_conversation 创建一条 type=single 会话并返回新会话 id。
   * 传对方姓名作为会话名；随后用 send() 在该会话内发消息即可。
   */
  startConversation: (name: string) =>
    mapi.post<{ id?: string; name?: string; type?: string; created?: boolean }>(
      '/jaxrs/message/assemble/communicate/im/conversation',
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
  fileList: (folderId: string) => list(mapi.get<FileRow[]>(`/jaxrs/file/assemble/control/file/list/${folderId}`)),
  /** 真实下载路由；拼 apiBase 以支持原生 App / 小程序绝对地址场景。 */
  fileDownloadUrl: (fileId: string) => `${getApiBase()}/jaxrs/file/assemble/control/file/${fileId}/download`,
  /**
   * 二进制上传：multipart（file 字段 + 可选 name）写入附件存储 FILE_FILE。
   * filePath 为本端临时文件路径（uni.chooseImage/chooseMessageFile 返回值）。
   * 落点是附件存储（FILE_FILE），而非「我的文件」x_file 列表。
   */
  upload: (folderId: string, filePath: string, fileName: string) =>
    mapi.upload<Record<string, unknown>>(`/jaxrs/attachment/upload/folder/${folderId}`, filePath, {
      name: 'file',
      formData: { name: fileName },
    }),
  /** 附件存储 FILE_FILE：当前用户的附件清单（上传落点即在此列出，而非 x_file）。 */
  attachmentList: (owner: string) => list(mapi.get<FileRow[]>(`/jaxrs/attachment/list/editor/${owner}`)),
  /** 附件存储 FILE_FILE 下载（拼 apiBase 支持原生 App / 小程序绝对地址）。 */
  attachmentDownloadUrl: (attId: string) => `${getApiBase()}/jaxrs/attachment/${attId}/download`,
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
  preCheck: () => mapi.get<AttendancePreCheck>('/jaxrs/attendance/assemble/control/v2/mobile/check/pre'),
  /** 打卡：checkInType 取 'checkIn'（上班）/ 'checkOut'（下班）；重复打卡返回 duplicated=true。 */
  check: (checkInType: 'checkIn' | 'checkOut') =>
    mapi.post<{ id?: string; duplicated?: boolean }>('/jaxrs/attendance/assemble/control/v2/mobile/check', {
      checkInType,
      sourceType: '移动端',
    }),
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

export const orgApi = {
  /** 全员 / 按姓名模糊搜索（POST mockputtopost 别名，body.key 为空返回全员）。 */
  personSearch: (key?: string) =>
    list(mapi.post<PersonRow[]>('/jaxrs/organization/assemble/control/person/list/like/mockputtopost', { key })),
  /** 人员详情。 */
  personDetail: (flag: string) => mapi.get<PersonRow>(`/jaxrs/organization/assemble/control/person/${flag}`),
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
  list: () => list(mapi.get<AnnRow[]>('/jaxrs/ai/assemble/control/ann/list')),
}

// ─────────────────────────────────────────────────────────────
// 通用（字典）
// ─────────────────────────────────────────────────────────────
export const generalApi = {
  dictList: () => mapi.get('/jaxrs/general/dict/list'),
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
}

export default apis
