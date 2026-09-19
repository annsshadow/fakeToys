/**
 * 移动端业务 API 层（services/index.ts）请求形状表驱动测试。
 *
 * 目的：services/index.ts 里绝大多数方法是「无分支的一行 mapi 转发器」，
 * 端点 URL 本身由 tests/contracts/mobile-endpoints.test.ts 的契约守卫钉死。
 * 这里补一层「动词 + 路径 + 请求体」形状测试：mock 掉传输层 mapi，逐个方法
 * 调用并断言它转发的 verb/path/body 正确——把「每个移动端按钮打到真实后端
 * 能力」这件事从「URL 守卫」上升到「完整请求形状」，同时把 index.ts 的一行
 * 转发器逐行覆盖，避免其成为长期未测死角。
 */
import { beforeEach, describe, expect, it, vi } from 'vitest'

const calls: Array<{ verb: string; path: string; body?: unknown; options?: unknown }> = []

vi.mock('./http', () => ({
  mapi: {
    get: (path: string, options?: unknown) => {
      calls.push({ verb: 'get', path, options })
      return Promise.resolve({ success: true, data: [] })
    },
    post: (path: string, body?: unknown, options?: unknown) => {
      calls.push({ verb: 'post', path, body, options })
      return Promise.resolve({ success: true, data: undefined })
    },
    put: (path: string, body?: unknown, options?: unknown) => {
      calls.push({ verb: 'put', path, body, options })
      return Promise.resolve({ success: true, data: undefined })
    },
    delete: (path: string, options?: unknown) => {
      calls.push({ verb: 'delete', path, options })
      return Promise.resolve({ success: true, data: undefined })
    },
    upload: (path: string, _filePath: string, options?: unknown) => {
      calls.push({ verb: 'upload', path, options })
      return Promise.resolve({ success: true, data: {} })
    },
  },
  getApiBase: () => 'http://o2.local:5432',
  setApiBase: () => {},
}))

import { annApi, apis, attendanceApi, authApi, fileApi, generalApi, messageApi, orgApi, processApi } from './index'

beforeEach(() => {
  calls.length = 0
})

/** 断言最近一次调用是预期的 verb+path(+body)。 */
function last() {
  return calls[calls.length - 1]
}

describe('authApi request shapes', () => {
  it('login forwards credential payload with requireAuth:false', async () => {
    await authApi.login({ credential: 'u', password: 'p' })
    expect(last()).toMatchObject({
      verb: 'post',
      path: '/api/authentication/login',
      body: { credential: 'u', password: 'p' },
      options: { requireAuth: false, discardResponse: true },
    })
  })

  it('who is an anonymous (requireAuth:false) GET', async () => {
    await authApi.who()
    expect(last()).toMatchObject({ verb: 'get', path: '/api/authentication/who', options: { requireAuth: false } })
  })

  it('refresh + logout + captcha pin their endpoints', async () => {
    await authApi.refresh()
    expect(last()).toMatchObject({ verb: 'post', path: '/api/authentication/refresh' })
    await authApi.logout()
    expect(last()).toMatchObject({ verb: 'post', path: '/api/authentication/logout' })
    await authApi.captcha()
    expect(last()).toMatchObject({ verb: 'get', path: '/api/authentication/captcha' })
  })
})

describe('processApi request shapes', () => {
  it('pending/completed/started lists hit the surface+POST-work endpoints', async () => {
    await processApi.pendingList(1, 20)
    expect(last()).toMatchObject({
      verb: 'get',
      path: '/api/processplatform/assemble/surface/task/list/my/paging/1/size/20',
    })
    await processApi.completedList(2, 10)
    expect(last()).toMatchObject({
      verb: 'get',
      path: '/api/processplatform/assemble/surface/taskcompleted/list/my/paging/2/size/10',
    })
    await processApi.startedList(1, 10)
    // 我发起的仅注册 POST
    expect(last()).toMatchObject({
      verb: 'post',
      path: '/api/processplatform/assemble/surface/work/list/my/paging/1/size/10',
    })
  })

  it('complete/reject pin the engine task endpoints with action payload', async () => {
    await processApi.completeTask('t-1', { opinion: 'ok' })
    expect(last()).toMatchObject({
      verb: 'post',
      path: '/api/task/t-1/complete',
      body: { data: {}, opinion: 'ok', action: 'approve' },
    })
    await processApi.rejectTask('t-2')
    expect(last()).toMatchObject({
      verb: 'post',
      path: '/api/task/t-2/reject',
      body: { data: {}, opinion: '', action: 'reject' },
    })
  })

  it('startable/getProcess/getForm/startWork/saveWorkData pin designer+form+processing routes', async () => {
    await processApi.startableProcesses()
    expect(last()).toMatchObject({ verb: 'get', path: '/api/processplatform/assemble/designer/list/all' })
    await processApi.getProcess('p-1')
    expect(last()).toMatchObject({ verb: 'get', path: '/api/processplatform/assemble/designer/get/p-1' })
    await processApi.getForm('flag-1')
    expect(last()).toMatchObject({ verb: 'get', path: '/api/form/flag-1' })
    await processApi.startWork('p-1', '我的流程')
    expect(last()).toMatchObject({
      verb: 'post',
      path: '/api/processplatform/service/processing/work',
      body: { process: 'p-1', title: '我的流程' },
    })
    await processApi.saveWorkData('w-1', { a: 1 })
    expect(last()).toMatchObject({
      verb: 'put',
      path: '/api/processplatform/service/processing/data/work/w-1',
      body: { a: 1 },
    })
  })
})

describe('messageApi request shapes', () => {
  it('conversation/history/markRead/startConversation pin the IM communicate routes', async () => {
    await messageApi.conversationList()
    expect(last()).toMatchObject({
      verb: 'get',
      path: '/api/message/assemble/communicate/im/conversation/list/my',
    })
    await messageApi.msgHistory(1, 30)
    expect(last()).toMatchObject({
      verb: 'get',
      path: '/api/message/assemble/communicate/im/msg/list/1/size/30',
    })
    await messageApi.markRead('c-1')
    expect(last()).toMatchObject({
      verb: 'post',
      path: '/api/message/assemble/communicate/im/conversation/c-1/read',
    })
    await messageApi.startConversation('张三')
    expect(last()).toMatchObject({
      verb: 'post',
      path: '/api/message/assemble/communicate/im/conversation',
      body: { name: '张三', type: 'single' },
    })
  })
})

describe('fileApi + attachment request shapes', () => {
  it('fileList/attachmentList pin control+FILE_FILE list routes', async () => {
    await fileApi.fileList('folder-1')
    expect(last()).toMatchObject({
      verb: 'get',
      path: '/api/file/assemble/control/file/list/folder-1',
    })
    await fileApi.attachmentList('owner-1')
    expect(last()).toMatchObject({ verb: 'get', path: '/api/attachment/list/editor/owner-1' })
  })

  it('upload pins the multipart attachment endpoint with file field + name', async () => {
    await fileApi.upload('folder-1', '/tmp/x.png', 'x.png')
    expect(last()).toMatchObject({
      verb: 'upload',
      path: '/api/attachment/upload/folder/folder-1',
      options: { name: 'file', formData: { name: 'x.png' } },
    })
  })

  it('download URL builders join the configured apiBase (native/offline targets)', () => {
    expect(fileApi.fileDownloadUrl('f-1')).toBe('http://o2.local:5432/api/file/assemble/control/file/f-1/download')
    expect(fileApi.attachmentDownloadUrl('a-1')).toBe('http://o2.local:5432/api/attachment/a-1/download')
  })
})

describe('attendance / org / ann / general request shapes', () => {
  it('attendance preCheck + check pin the mobile check routes', async () => {
    await attendanceApi.preCheck()
    expect(last()).toMatchObject({
      verb: 'get',
      path: '/api/attendance/assemble/control/v2/mobile/check/pre',
    })
    await attendanceApi.check('checkIn')
    expect(last()).toMatchObject({
      verb: 'post',
      path: '/api/attendance/assemble/control/v2/mobile/check',
      body: { checkInType: 'checkIn', sourceType: '移动端' },
    })
  })

  it('org personSearch (empty key → everyone) + personDetail pin the person routes', async () => {
    await orgApi.personSearch()
    expect(last()).toMatchObject({
      verb: 'post',
      path: '/api/organization/assemble/control/person/list/like/mockputtopost',
      body: { key: undefined },
    })
    await orgApi.personDetail('flag-9')
    expect(last()).toMatchObject({
      verb: 'get',
      path: '/api/organization/assemble/control/person/flag-9',
    })
  })

  it('ann list + general dictList pin the announcement + dict routes', async () => {
    await annApi.list()
    expect(last()).toMatchObject({ verb: 'get', path: '/api/ai/assemble/control/ann/list' })
    await generalApi.dictList()
    expect(last()).toMatchObject({ verb: 'get', path: '/api/general/dict/list' })
  })
})

describe('apis facade re-exports every domain', () => {
  it('exposes all service groups so views can import a single entry', () => {
    expect(apis).toMatchObject({
      auth: authApi,
      process: processApi,
      message: messageApi,
      file: fileApi,
      attendance: attendanceApi,
      org: orgApi,
      ann: annApi,
      general: generalApi,
    })
  })
})
