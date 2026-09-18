import { describe, expect, it, vi, beforeEach } from 'vitest'
import type { Mock } from 'vitest'

// Mock the api singleton from @oa4rust/sdk
const mockGet: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockPost: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockPut: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockDelete: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockUpload: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })

// Mock the entire @oa4rust/sdk module
vi.mock('@oa4rust/sdk', () => ({
  api: {
    get: mockGet,
    post: mockPost,
    put: mockPut,
    delete: mockDelete,
    upload: mockUpload,
  },
}))

describe('API modules execution coverage', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  describe('apis.auth functions', () => {
    it('auth.login should call api.post with correct parameters', async () => {
      const { apis } = await import('./index.ts')
      await apis.auth.login({ credential: 'testuser', password: 'testpass' })
      
      expect(mockPost).toHaveBeenCalledTimes(1)
      expect(mockPost).toHaveBeenCalledWith(
        '/api/authentication/login',
        { credential: 'testuser', password: 'testpass' },
        { requireAuth: false, discardResponse: true }
      )
    })

    it('auth.logout should call api.post', async () => {
      const { apis } = await import('./index.ts')
      await apis.auth.logout()
      
      expect(mockPost).toHaveBeenCalledTimes(1)
      expect(mockPost).toHaveBeenCalledWith(
        '/api/authentication/logout',
        null,
        { requireAuth: false, discardResponse: true }
      )
    })

    it('auth.who should call api.get', async () => {
      const { apis } = await import('./index.ts')
      await apis.auth.who()
      
      expect(mockGet).toHaveBeenCalledTimes(1)
      expect(mockGet).toHaveBeenCalledWith(
        '/api/authentication/who',
        { requireAuth: false }
      )
    })

    it('auth.refresh should call api.post', async () => {
      const { apis } = await import('./index.ts')
      await apis.auth.refresh()
      
      expect(mockPost).toHaveBeenCalledTimes(1)
      expect(mockPost).toHaveBeenCalledWith(
        '/api/authentication/refresh',
        null,
        { requireAuth: false, discardResponse: true }
      )
    })

    it('auth.captcha should call api.get', async () => {
      const { apis } = await import('./index.ts')
      await apis.auth.captcha()
      
      expect(mockGet).toHaveBeenCalledTimes(1)
      expect(mockGet).toHaveBeenCalledWith(
        '/api/authentication/captcha',
        { requireAuth: false }
      )
    })

    it('auth.captchaSize should call api.get with dimensions', async () => {
      const { apis } = await import('./index.ts')
      await apis.auth.captchaSize(200, 80)
      
      expect(mockGet).toHaveBeenCalledTimes(1)
      expect(mockGet).toHaveBeenCalledWith(
        '/api/authentication/captcha/width/200/height/80',
        { requireAuth: false }
      )
    })

    it('auth.checkToken should call api.post', async () => {
      const { apis } = await import('./index.ts')
      await apis.auth.checkToken({ token: 'abc123' })
      
      expect(mockPost).toHaveBeenCalledTimes(1)
      expect(mockPost).toHaveBeenCalledWith(
        '/api/authentication/check/token',
        { token: 'abc123' }
      )
    })
  })

  describe('apis.org functions', () => {
    it('org.groupList should call api.get', async () => {
      const { apis } = await import('./index.ts')
      await apis.org.groupList()
      
      expect(mockGet).toHaveBeenCalled()
      expect(mockGet).toHaveBeenCalledWith(
        expect.stringContaining('/api/organization/assemble/control/group/list')
      )
    })

    it('org.personDetail should call api.get', async () => {
      const { apis } = await import('./index.ts')
      await apis.org.personDetail('user123')
      
      expect(mockGet).toHaveBeenCalledWith('/api/organization/assemble/control/person/user123')
    })

    it('org.personCreate should call api.post', async () => {
      const { apis } = await import('./index.ts')
      await apis.org.personCreate({ name: 'test' })
      
      expect(mockPost).toHaveBeenCalledWith('/api/organization/assemble/control/person', { name: 'test' })
    })

    it('org.personUpdate should call api.put', async () => {
      const { apis } = await import('./index.ts')
      await apis.org.personUpdate('user123', { name: 'updated' })
      
      expect(mockPut).toHaveBeenCalledWith('/api/organization/assemble/control/person/user123', { name: 'updated' })
    })

    it('org.personDelete should call api.delete', async () => {
      const { apis } = await import('./index.ts')
      await apis.org.personDelete('user123')
      
      expect(mockDelete).toHaveBeenCalledWith('/api/organization/assemble/control/person/user123')
    })

    it('org.identityList should call api.get', async () => {
      const { apis } = await import('./index.ts')
      await apis.org.identityList()
      
      expect(mockGet).toHaveBeenCalledWith('/api/organization/assemble/control/identity/list/')
    })
  })

  describe('apis.file functions', () => {
    it('file.fileList should call api.get', async () => {
      const { apis } = await import('./index.ts')
      await apis.file.fileList('folder123')
      
      expect(mockGet).toHaveBeenCalledWith('/api/file/assemble/control/file/list/folder123')
    })

    it('file.folderList should call api.get', async () => {
      const { apis } = await import('./index.ts')
      await apis.file.folderList('parent123')
      
      expect(mockGet).toHaveBeenCalledWith('/api/file/core/entity/folder/list/parent123')
    })

    it('file.fileDelete should call api.delete', async () => {
      const { apis } = await import('./index.ts')
      await apis.file.fileDelete('file123')
      
      expect(mockDelete).toHaveBeenCalledWith('/api/file/assemble/control/file/file123')
    })
  })

  describe('apis.general functions', () => {
    it('general.dictList should call api.get', async () => {
      const { apis } = await import('./index.ts')
      await apis.general.dictList()
      
      expect(mockGet).toHaveBeenCalledWith('/api/general/dict/list')
    })

    it('general.dictCreate should call api.post', async () => {
      const { apis } = await import('./index.ts')
      await apis.general.dictCreate({ name: 'testdict' })
      
      expect(mockPost).toHaveBeenCalledWith('/api/general/dict/create', { name: 'testdict' })
    })

    it('general.dictUpdate should call api.post', async () => {
      const { apis } = await import('./index.ts')
      await apis.general.dictUpdate('dict123', { name: 'updated' })
      
      expect(mockPost).toHaveBeenCalledWith('/api/general/dict/update/dict123', { name: 'updated' })
    })

    it('general.dictDelete should call api.post', async () => {
      const { apis } = await import('./index.ts')
      await apis.general.dictDelete('dict123')
      
      expect(mockPost).toHaveBeenCalledWith('/api/general/dict/delete/dict123')
    })
  })

  describe('apis.process functions', () => {
    it('process.workList should call api.post', async () => {
      const { apis } = await import('./index.ts')
      await apis.process.workList(1, 10)
      
      expect(mockPost).toHaveBeenCalled()
    })

    it('process.processList should call api.get', async () => {
      const { apis } = await import('./index.ts')
      await apis.process.processList()
      
      expect(mockGet).toHaveBeenCalledWith('/api/processplatform/assemble/designer/list/all')
    })

    it('process.workStart should call api.post', async () => {
      const { apis } = await import('./index.ts')
      await apis.process.workStart({ processId: 'proc1' })
      
      expect(mockPost).toHaveBeenCalled()
    })

    it('process.taskHandle with approve should call correct endpoint', async () => {
      const { apis } = await import('./index.ts')
      await apis.process.taskHandle('task123', 'approve', { data: { key: 'value' } })
      
      expect(mockPost).toHaveBeenCalledWith(
        '/api/task/task123/complete',
        expect.objectContaining({ action: 'approve' })
      )
    })

    it('process.taskHandle with reject should call correct endpoint', async () => {
      const { apis } = await import('./index.ts')
      await apis.process.taskHandle('task123', 'reject', { data: {} })
      
      expect(mockPost).toHaveBeenCalledWith(
        '/api/task/task123/reject',
        expect.objectContaining({ action: 'reject' })
      )
    })
  })

  describe('apis.portal functions', () => {
    it('portal.pageList should call api.get', async () => {
      const { apis } = await import('./index.ts')
      await apis.portal.pageList('app123')
      
      expect(mockGet).toHaveBeenCalledWith('/api/portal/assemble/surface/page/list/portal/app123')
    })

    it('portal.pageDetail should call api.get', async () => {
      const { apis } = await import('./index.ts')
      await apis.portal.pageDetail('page123')
      
      expect(mockGet).toHaveBeenCalledWith('/api/portal/assemble/surface/page/v2/page123')
    })

    it('portal.pageCreate should call api.post', async () => {
      const { apis } = await import('./index.ts')
      await apis.portal.pageCreate({ name: 'test' })
      
      expect(mockPost).toHaveBeenCalledWith('/api/portal/assemble/designer/page/create', { name: 'test' })
    })
  })

  describe('apis.message functions', () => {
    it('message.conversationList should call api.get', async () => {
      const { apis } = await import('./index.ts')
      await apis.message.conversationList()
      
      expect(mockGet).toHaveBeenCalledWith('/api/message/assemble/communicate/im/conversation/list/my')
    })

    it('message.msgSend should call api.post', async () => {
      const { apis } = await import('./index.ts')
      await apis.message.msgSend({ conversationId: 'conv1', content: 'hello', type: 'text' })
      
      expect(mockPost).toHaveBeenCalledWith(
        '/api/message/assemble/communicate/im/msg',
        expect.objectContaining({
          conversationId: 'conv1',
          '"conversationId"': 'conv1',
          content: 'hello',
          type: 'text',
        })
      )
    })
  })

  describe('export structures', () => {
    it('apis should contain all base modules', async () => {
      const { apis } = await import('./index.ts')
      
      expect(apis).toHaveProperty('auth')
      expect(apis).toHaveProperty('org')
      expect(apis).toHaveProperty('process')
      expect(apis).toHaveProperty('portal')
      expect(apis).toHaveProperty('message')
      expect(apis).toHaveProperty('file')
      expect(apis).toHaveProperty('general')
    })

    it('oa4rustApis should contain extended modules', async () => {
      const { oa4rustApis } = await import('./index.ts')
      
      expect(oa4rustApis).toHaveProperty('auth')
      expect(oa4rustApis).toHaveProperty('org')
      expect(oa4rustApis).toHaveProperty('process')
      expect(oa4rustApis).toHaveProperty('message')
      expect(oa4rustApis).toHaveProperty('im')
      expect(oa4rustApis).toHaveProperty('file')
      expect(oa4rustApis).toHaveProperty('general')
    })
  })

  describe('oa4rustApis extended modules', () => {
    it('oa4rustApis.mind should be available', async () => {
      const { oa4rustApis } = await import('./index.ts')
      expect(oa4rustApis).toHaveProperty('mind')
      await oa4rustApis.mind.folderTree()
      expect(mockGet).toHaveBeenCalled()
    })

    it('oa4rustApis.document should be available', async () => {
      const { oa4rustApis } = await import('./index.ts')
      expect(oa4rustApis).toHaveProperty('document')
      await oa4rustApis.document.list()
      expect(mockGet).toHaveBeenCalled()
    })

    it('oa4rustApis.programCenter should be available', async () => {
      const { oa4rustApis } = await import('./index.ts')
      expect(oa4rustApis).toHaveProperty('programCenter')
      await oa4rustApis.programCenter.agentList()
      expect(mockGet).toHaveBeenCalled()
    })

    it('oa4rustApis.queryView should be available', async () => {
      const { oa4rustApis } = await import('./index.ts')
      expect(oa4rustApis).toHaveProperty('queryView')
      await oa4rustApis.queryView.list('view1')
      expect(mockGet).toHaveBeenCalled()
    })
  })

  describe('imApi functions', () => {
    it('im.conversationList should call api.get', async () => {
      const { imApi } = await import('./index.ts')
      await imApi.conversationList()
      expect(mockGet).toHaveBeenCalled()
    })
    
    it('im.messageList should call api.post', async () => {
      const { imApi } = await import('./index.ts')
      await imApi.messageList(1, 50, 'conv1')
      expect(mockPost).toHaveBeenCalled()
    })
    
    it('im.messageSend should call api.post', async () => {
      const { imApi } = await import('./index.ts')
      await imApi.messageSend({ conversationId: 'conv1', content: 'hello', type: 'text' })
      expect(mockPost).toHaveBeenCalled()
    })
    
    it('im.markRead should call api.post', async () => {
      const { imApi } = await import('./index.ts')
      await imApi.markRead('msg1')
      expect(mockPost).toHaveBeenCalled()
    })
    
    it('im.unreadCount should call api.get', async () => {
      const { imApi } = await import('./index.ts')
      await imApi.unreadCount('im')
      expect(mockGet).toHaveBeenCalled()
    })
  })

  describe('fileApi extended tests', () => {
    it('file.fileList should call api.get with folder', async () => {
      const { fileApi } = await import('./index.ts')
      await fileApi.fileList('folder123')
      expect(mockGet).toHaveBeenCalledWith('/api/file/assemble/control/file/list/folder123')
    })
    
    it('file.folderList should call api.get with parent', async () => {
      const { fileApi } = await import('./index.ts')
      await fileApi.folderList('parent123')
      expect(mockGet).toHaveBeenCalledWith('/api/file/core/entity/folder/list/parent123')
    })
  })

  describe('exportApi functions', () => {
    it('exportApi.result should call api.get with flag', async () => {
      const { exportApi } = await import('./index.ts')
      await exportApi.result('export-flag-123')
      expect(mockGet).toHaveBeenCalledWith('/api/export/result/flag/export-flag-123')
    })
  })

  describe('createRequest function', () => {
    it('createRequest creates a bound request function', async () => {
      const { createRequest } = await import('./index.ts')
      const request = createRequest('/api/test')
      expect(typeof request).toBe('function')
    })
  })
})
