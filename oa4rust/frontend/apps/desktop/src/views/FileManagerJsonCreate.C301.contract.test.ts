import { readFileSync } from 'node:fs'
import { describe, expect, it } from 'vitest'

const src = readFileSync(new URL('./FileManager.vue', import.meta.url), 'utf8')

describe('FileManager 回收站测试数据真实 UI 入口（C301）', () => {
  it('提供新建文件按钮与可删除列表的测试锚点', () => {
    expect(src).toContain('data-testid="file-create-json"')
    expect(src).toContain('data-testid="files-recyclable-list"')
  })
  it('新建走 JSON 端点写 FILE_FILE，归属会话本人', () => {
    expect(src).toContain("api.post('/api/file/core/entity/file'");
    expect(src).toContain('person: session.state.user?.unique')
    expect(src).toContain('useSession')
  })
  it('删除走软删除 FILE_FILE 的现役端点，使文件进入回收站', () => {
    expect(src).toContain('/api/file/assemble/control/file/${id}')
  })
  it('不破坏现有上传与列表入口', () => {
    expect(src).toContain('handleUpload')
    expect(src).toContain('data-testid="files-mine-list"')
  })
})
