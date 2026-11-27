// @vitest-environment jsdom
// R622c OrgDirectory 搜索漏筛真修复契约(临时验证文件,交付中保留为契约)
import { readFileSync } from 'node:fs'
import { describe, expect, it } from 'vitest'

const APP = 'frontend/apps/desktop/src/views/OrgDirectoryApp.vue'
const src = readFileSync(APP, 'utf8')

describe('R622c OrgDirectory 搜索漏筛真修复', () => {
  it('loadPersonFilter 构造带搜索词的 body 交服务端,不再传空对象', () => {
    const i = src.indexOf('async function loadPersonFilter')
    const seg = src.slice(i, i + 1400)
    expect(seg).toContain('const body')
    expect(seg).toContain('body.name')
    expect(seg).toContain('body.unitId')
    expect(seg).not.toContain('{},\n')
    expect(seg).toContain('body,')
  })

  it('模板移除客户端过滤与 slice(0,20) 截断', () => {
    expect(src).not.toContain('rows.filter(row =>')
    expect(src).not.toContain('.slice(0, 20)')
    expect(src).toContain('v-for="(r, i) in rows"')
  })

  it('部门快捷设 unitId 并触发第一页重载', () => {
    expect(src).toContain('applyDepartmentFilter')
    const i = src.indexOf('function applyDepartmentFilter')
    const fn = src.slice(i, i + 500)
    expect(fn).toContain('personPage.value = 1')
    expect(fn).toContain('loadPersonFilter')
    expect(fn).toContain('unitFilter.value = dept')
  })

  it('总页数基于服务端 count(body 修复后 count 即筛选结果)', () => {
    expect(src).toContain('personCount.value = Number(page?.count')
    expect(src).toContain('Math.ceil(personCount.value / personPageSize)')
  })
})
