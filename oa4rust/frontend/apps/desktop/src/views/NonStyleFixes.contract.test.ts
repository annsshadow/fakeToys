// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const views = fileURLToPath(new URL('.', import.meta.url))
function source(name: string): string {
  return readFileSync(views + name, 'utf8')
}
const file = source('FileManager.vue')
const role = source('RoleManager.vue')
const proc = source('ProcessWork.vue')

const fileNewCall = '/api/file/assemble/control/file/list/' + String.fromCharCode(36) + "{folderId || 'root'}"
const fileOldCall = '/api/file/assemble/control/file/list/' + String.fromCharCode(36) + "{folderId || ''}"
const procRawTitle = '{{ opened.title || opened.processName'

describe('非样式问题修复契约', () => {
  it('文件列表空目录使用 root 路径段，不再请求空路径', () => {
    expect(file).toContain(fileNewCall)
    expect(file).not.toContain(fileOldCall)
  })

  it('角色列表调用后端实际注册的角色列表端点', () => {
    expect(role).toMatch(/api\.get\(['"]\/api\/authentication\/role\/list['"]\)/)
    expect(role).not.toMatch(/api\.post\(['"]\/api\/role\/list['"]\)/)
    expect(role).toContain('loadRoles()')
  })

  it('流程办理弹窗标题经 displayTitle 清洗', () => {
    expect(proc).toContain('displayTitle(opened')
    expect(proc).not.toContain(procRawTitle)
  })
})
