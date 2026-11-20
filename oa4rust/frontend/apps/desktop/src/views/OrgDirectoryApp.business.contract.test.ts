// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/**
 * 组织通讯录必须是任务导向的业务界面（搜索 / 部门树 / 人员列表），
 * 而不是把组织 / 认证 / 成员管理端点平铺成调试按钮。仅断言 <template> 段。
 */
describe('OrgDirectoryApp 组织通讯录业务视图契约', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'OrgDirectoryApp.vue'), 'utf8')
  const template = source.slice(source.indexOf('<template>'), source.indexOf('</template>'))

  it('提供「人员搜索 / 组织架构 / 人员列表」三个业务区块', () => {
    for (const [title, testid] of [
      ['人员搜索', 'org-search-section'],
      ['组织架构', 'org-tree-section'],
      ['人员列表', 'org-person-section'],
    ] as const) {
      expect(template, `缺少业务标题: ${title}`).toContain(title)
      expect(template, `缺少业务区块 testid: ${testid}`).toContain(`data-testid="${testid}"`)
    }
  })

  it('人员搜索区有搜索输入框与提交按钮组成的真实表单', () => {
    const section = template.slice(
      template.indexOf('class="search-section'),
      template.indexOf('class="directory-grid"'),
    )
    expect(section).toContain('<form')
    expect(section).toMatch(/<input[^>]*type="search"/)
    expect(section).toMatch(/<button[^>]*type="submit"/)
    expect(section).toContain('data-testid="org-search-input"')
    expect(section).toContain('data-testid="org-search-button"')
  })

  it('组织架构区是可点击筛选的部门树列表', () => {
    const section = template.slice(template.indexOf('class="tree-section'), template.indexOf('class="person-section'))
    expect(section).toContain('<ul')
    expect(section).toContain('data-testid="org-department-tree"')
    expect(section).toContain('data-testid="org-department-node"')
    expect(section).toContain('产品部')
  })

  it('人员列表区有列表、分页与空态', () => {
    const section = template.slice(template.indexOf('class="person-section'))
    expect(section).toContain('data-testid="org-person-list"')
    expect(section).toContain('data-testid="org-person-pager"')
    expect(section).toContain('data-testid="org-person-empty"')
    expect(section).toContain('personCount')
  })

  it('页面副标题与区块标题为业务语义，不含 /api/ 路径', () => {
    expect(template).not.toContain('/api/')
    expect(template).toContain('<h1>组织通讯录</h1>')
    expect((template.match(/<h2>/g) ?? []).length).toBeGreaterThanOrEqual(3)
  })

  it('不渲染组织认证、成员管理等端点调试按钮', () => {
    expect(template).not.toMatch(
      /@click="[^"]*(?:manage\(|orgAuth\(|runCurrentTab|runPerson|runUnit|runIdentity|runGroup|runRole)/,
    )
    expect(template).not.toContain('▶ 查询本页')
  })
})
