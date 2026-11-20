import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const src = readFileSync(fileURLToPath(new URL('./FileManager.vue', import.meta.url)), 'utf8')
const template = src.slice(src.indexOf('<template>'), src.indexOf('</template>'))

describe('FileManager 业务契约', () => {
  it('默认视图提供我的文件 / 共享 / 回收三个业务栏', () => {
    expect(template).toMatch(/<section[^>]*class="[^"]*\bfiles-mine\b/)
    expect(template).toMatch(/<section[^>]*class="[^"]*\bfiles-shared\b/)
    expect(template).toMatch(/<section[^>]*class="[^"]*\bfiles-recycle\b/)
    expect(template).toContain('>我的文件<')
    expect(template).toContain('>共享<')
    expect(template).toContain('>回收<')
    expect(template).toContain('data-testid="files-mine-list"')
    expect(template).toContain('data-testid="files-shared-list"')
    expect(template).toContain('data-testid="files-recycle-list"')
  })

  it('文件列表、上传与搜索使用真实业务控件', () => {
    expect(template).toContain('<form')
    expect(template).toContain('type="submit"')
    expect(template).toContain('<ul')
    expect(template).toMatch(/v-for="[^"]*\bitem\b/)
    expect(template).toMatch(/type="file"/)
    expect(template).toContain('item-body')
  })

  it('默认视图不渲染 /api/* 端点调试按钮', () => {
    expect(template).not.toContain('/api/')
    for (const handler of [
      'assembleFolderList',
      'assembleFileList',
      'assembleFileDetail',
      'assembleFileDownload',
      'shareListMy',
      'shareListToMe',
      'shareCreate',
      'shareCancel',
      'recycleList',
      'recycleResume',
      'recycleDelete',
      'recycleEmpty',
      'fileMove',
      'fileCopy',
      'fileRename',
      'fileDelete',
      'folderCreate',
      'folderRename',
      'folderDelete',
      'fileImport',
      'fileExport',
    ]) {
      expect(template).not.toContain(handler)
    }
  })
})
