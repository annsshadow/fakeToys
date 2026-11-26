// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R307 组织/部门新建入口契约：按钮、create 调用、成功刷新、t14/t28 兜底不回退

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const viewerSource = readFileSync(resolve(import.meta.dirname, 'OrgViewer.vue'), 'utf8')
const dirSource = readFileSync(resolve(import.meta.dirname, 'OrgDirectoryApp.vue'), 'utf8')
const viewerScript = viewerSource.slice(viewerSource.indexOf('<script'), viewerSource.indexOf('</script>'))
const viewerTemplate = viewerSource.slice(viewerSource.indexOf('<template>'), viewerSource.lastIndexOf('</template>'))
const dirScript = dirSource.slice(dirSource.indexOf('<script'), dirSource.indexOf('</script>'))
const dirTemplate = dirSource.slice(dirSource.indexOf('<template>'), dirSource.lastIndexOf('</template>'))

describe('R307 OrgViewer 新建组织组入口', () => {
  it('模板有新建组按钮、名称输入与提交按钮', () => {
    expect(viewerTemplate).toMatch(/data-testid="org-create-group-btn"/)
    expect(viewerTemplate).toMatch(/data-testid="org-create-group-name"/)
    expect(viewerTemplate).toMatch(/data-testid="org-create-group-submit"/)
    expect(viewerTemplate).toMatch(/新建组织组/)
  })
  it('调用 organizationControlApi.create(group,{name}) 且成功后刷新树', () => {
    expect(viewerScript).toMatch(/api\.post\(\s*'\/api\/organization\/assemble\/control\/group'\s*,/)
    expect(viewerScript).toMatch(/name:\s*newGroupName\.value\.trim\(\)/)
    expect(viewerScript).toMatch(/await withTimeout\(\s*api\.post\(\s*'\/api\/organization\/assemble\/control\/group'/)
    expect(viewerScript).toMatch(/createGroupFromForm[\s\S]{0,600}?void loadOrgTree\(\)/)
    expect(viewerScript).toMatch(/groupCreateError\.value\s*=/)
  })
  it('t14 兜底不回退：group/list/like PUT、重试与 onMounted 仍在', () => {
    expect(viewerScript).toMatch(/await withTimeout\(\s*api\.put\(\s*'\/api\/organization\/assemble\/control\/group\/list\/like'/)
    expect(viewerTemplate).toMatch(/data-testid="org-tree-retry"/)
    expect(viewerScript).toMatch(/onMounted\(\s*\(\)\s*=>\s*\{\s*void loadOrgTree\(\)\s*\}\s*\)/)
  })
})

describe('R307 OrgDirectoryApp 新建部门入口', () => {
  it('模板有新建部门按钮、名称与父级输入', () => {
    expect(dirTemplate).toMatch(/data-testid="org-create-unit-btn"/)
    expect(dirTemplate).toMatch(/data-testid="org-create-unit-name"/)
    expect(dirTemplate).toMatch(/data-testid="org-create-unit-parent"/)
    expect(dirTemplate).toMatch(/新建部门/)
  })
  it('调用 create(unit,{name,parentId}) 且成功后刷新部门树', () => {
    expect(dirScript).toMatch(/api\.post\(\s*'\/api\/organization\/assemble\/control\/unit'\s*,/)
    expect(dirScript).toMatch(/parentId:\s*newDeptParent\.value/)
    expect(dirScript).toMatch(/await withTimeout\(\s*api\.post\(\s*'\/api\/organization\/assemble\/control\/unit'/)
    expect(dirScript).toMatch(/createUnitFromForm[\s\S]{0,600}?void loadDeptTree\(\)/)
    expect(dirScript).toMatch(/unitCreateError\.value\s*=/)
  })
  it('t28 兜底不回退：部门树 withTimeout、重试、名册与 onMounted', () => {
    expect(dirScript).toMatch(/await withTimeout\(\s*api\.get\(\s*'\/api\/departments\/tree'/)
    expect(dirTemplate).toMatch(/data-testid="dept-tree-retry"/)
    expect(dirTemplate).toMatch(/data-testid="org-load-retry"/)
    expect(dirScript).toMatch(/onMounted\(\s*\(\)\s*=>\s*\{[\s\S]{0,160}?void loadDeptTree\(\)/)
  })
})
