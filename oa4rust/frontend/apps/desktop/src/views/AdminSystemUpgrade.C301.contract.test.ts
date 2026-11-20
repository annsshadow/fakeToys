// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/** R301 C7 管理区/系统域实质升级契约：权限成员明细可视化、表单可读性、表格密度切换。 */
import { readFileSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = dirname(fileURLToPath(import.meta.url))
const read = (p: string) => readFileSync(resolve(here, p), 'utf8')
const role = read('RoleManager.vue')
const three = read('ThreeMemberApp.vue')
const adminTable = read('../components/admin/AdminTable.vue')
const templateOf = (s: string) => s.slice(0, s.indexOf('<script setup lang="ts">'))

describe('R301 C7 管理区与系统域实质升级', () => {
  it('角色成员点击后展示权限可视化明细弹窗，而非只有 toast 人数', () => {
    const tpl = templateOf(role)
    expect(tpl).toContain('data-testid="role-member-detail"')
    expect(tpl).toContain('data-testid="role-member-count"')
    expect(tpl).toContain('data-testid="role-member-list"')
    expect(tpl).toContain('data-testid="role-member-empty"')
    expect(role).toMatch(/const memberDetail = ref/)
    expect(role).toMatch(/roleMembers\s*=\s*ref/)
    expect(role).toContain('person/list/role/')
    expect(role).toMatch(/memberDetail\.value\s*=\s*\{/)
  })

  it('角色列表的 Flag 有可读标识说明，原始值降为辅助信息', () => {
    const tpl = templateOf(role)
    expect(tpl).toContain('class="flag-label"')
    expect(tpl).toContain('标识')
    expect(tpl).toContain('data-testid="role-flag"')
  })

  it('三方成员表单与表格去技术化：单位可读、状态徽标、可读标签', () => {
    const tpl = templateOf(three)
    expect(tpl).toContain('data-testid="three-unit-cell"')
    expect(tpl).toContain('data-testid="three-status-badge"')
    expect(tpl).toContain('data-testid="three-status-select"')
    expect(tpl).toContain('data-testid="three-unit-input"')
    expect(tpl).toContain('>状态</label>')
    expect(tpl).not.toContain('>单位 ID</label>')
    expect(tpl).not.toContain('placeholder="unitId"')
    expect(tpl).not.toContain('placeholder="active / disabled"')
    expect(three).toMatch(/const statusOptions\s*=/)
    expect(three).toMatch(/statusLabel\(/)
    expect(three).toMatch(/unitLabel\(/)
    expect(three).toContain('statusDes')
  })

  it('AdminTable 原语提供紧凑/标准密度切换且不破坏现有 props/events', () => {
    expect(adminTable).toContain('data-testid="admin-table-density"')
    expect(adminTable).toMatch(/density\s*=\s*ref/)
    expect(adminTable).toContain('admin-table--compact')
    expect(adminTable).toContain('紧凑')
    expect(adminTable).toContain('标准')
    expect(adminTable).toContain('columns: AdminTableColumn[]')
    expect(adminTable).toContain('@retry="emit(\'retry\')"')
  })

  it('保留现有角色/三方成员关键操作与真实端点', () => {
    for (const id of ['role-page', 'role-refresh', 'role-create', 'role-members', 'role-edit', 'role-delete']) {
      expect(role).toContain('data-testid="' + id + '"')
    }
    expect(three).toContain('data-testid="three-member-page"')
    for (const ep of [
      '/api/authentication/role/list',
      '/api/organization/assemble/control/person/list/role/',
      '/api/organization/assemble/control/threemember/list',
      '/api/organization/assemble/control/threemember/create',
    ]) {
      expect(role + three, ep).toContain(ep)
    }
  })
})
