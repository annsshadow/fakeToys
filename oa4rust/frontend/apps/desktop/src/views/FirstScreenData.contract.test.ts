// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R302 首屏数据与空态引导契约：
 * 真浏览器实测五页首屏均自动加载真实数据、无长期 loading；
 * 但文件管理与笔记空态是「死胡同」（只有文案、无下一步动作），
 * 与验收②「明确引导空态」相悖。此契约锁定引导按钮必须复用既有业务函数。
 * 仅读源码，不改接口与业务逻辑。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string): string =>
  readFileSync(resolve(import.meta.dirname, name), 'utf8')

const fileManager = read('FileManager.vue')
const noteApp = read('NoteApp.vue')
const meeting = read('MeetingApp.vue')
const bbs = read('BBSForum.vue')
const recycle = read('RecycleApp.vue')

const templateOf = (src: string): string =>
  src.slice(src.indexOf('<template>'), src.lastIndexOf('</template>'))

describe('R302 首屏数据与空态引导契约', () => {
  it('目标页首屏自动加载真实数据（不靠用户手动触发）', () => {
    expect(fileManager).toMatch(/\nloadFiles\(\)\s*$/m)
    expect(meeting).toContain('onMounted(loadMeetings)')
    expect(recycle).toMatch(/\nloadItems\(\)\s*$/m)
    expect(noteApp).toContain('useQuery({')
    expect(bbs).toContain('useQuery({')
  })

  it('文件管理空态给出可操作上传引导，复用 handleUpload', () => {
    const tpl = templateOf(fileManager)
    const i = tpl.indexOf('当前文件夹为空')
    expect(i).toBeGreaterThan(-1)
    const block = tpl.slice(i, i + 260)
    expect(block).toMatch(/handleUpload/)
    expect(block).toMatch(/上传/)
  })

  it('笔记空态给出可操作新建引导，复用 openCreate', () => {
    const tpl = templateOf(noteApp)
    const i = tpl.indexOf('暂无笔记')
    expect(i).toBeGreaterThan(-1)
    const block = tpl.slice(i, i + 260)
    expect(block).toMatch(/openCreate/)
    expect(block).toMatch(/新建/)
  })

  it('会议与论坛空态保留引导动作（回归保护）', () => {
    expect(meeting).toContain('点击「新建会议」预订第一场')
    expect(bbs).toContain('来发起第一个讨论吧')
  })
})
