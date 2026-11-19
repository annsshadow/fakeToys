// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'ScriptWorkbench.vue'), 'utf8')
const openStart = source.indexOf('async function open(item: ScriptListItem): Promise<void> {')
const openSrc = source.slice(openStart, source.indexOf('function createNew()', openStart))
const createSrc = source.slice(source.indexOf('function createNew()'), source.indexOf('async function save()'))
const removeSrc = source.slice(source.indexOf('async function remove()'), source.indexOf('function restore('))

describe('脚本工作台详情回填竞态守卫（R274）', () => {
  it('竞态真实可达：列表项加载中仍可连点，open 没有在途拦截', () => {
    const itemLine = source.split('\n').find((line) => line.includes('@click="open(item)"')) ?? ''
    expect(itemLine).toContain('open(item)')
    expect(itemLine.includes('disabled')).toBe(false)
    // 打开不同脚本不应被 loading/在途标志挡住，否则竞态不可达。
    expect(openSrc.includes('if (loading.value) return')).toBe(false)
    expect(openSrc.includes('if (saving.value) return')).toBe(false)
  })

  it('open 用递增序号只认最新详情，过期响应不得覆盖当前表单与版本', () => {
    expect(source).toContain('let detailSeq = 0')
    expect(openSrc).toContain('const seq = ++detailSeq')
    const begin = openSrc.indexOf('const seq = ++detailSeq')
    const load = openSrc.indexOf('await props.adapter.load(item.id)')
    const guard = openSrc.indexOf('if (seq !== detailSeq) return')
    expect(begin).toBeGreaterThan(-1)
    expect(begin).toBeLessThan(load)
    expect(guard).toBeGreaterThan(load)
    // 守卫必须在 form/versions 回填之前，旧详情写不进新脚本编辑器。
    expect(guard).toBeLessThan(openSrc.indexOf('form.value = { name: detail.name'))
    expect(guard).toBeLessThan(openSrc.indexOf('versions.value = loaded'))
  })

  it('load 与 versions 两段 await 后都校验，catch 中过期错误也不落地', () => {
    const guards = openSrc.split('if (seq !== detailSeq) return').length - 1
    expect(guards).toBeGreaterThanOrEqual(3)
    const versionsAwait = openSrc.indexOf('await props.adapter.versions(item.id)')
    const versionsGuard = openSrc.indexOf('if (seq !== detailSeq) return', versionsAwait)
    expect(versionsGuard).toBeGreaterThan(versionsAwait)
    expect(versionsGuard).toBeLessThan(openSrc.indexOf('versions.value = loaded'))
    const catchStart = openSrc.indexOf('catch (error: unknown)')
    expect(openSrc.indexOf('if (seq !== detailSeq) return', catchStart)).toBeGreaterThan(catchStart)
  })

  it('新建与删除都使在途详情失效，旧脚本不得回填到空白表单', () => {
    expect(createSrc).toContain('detailSeq++')
    expect(removeSrc).toContain('detailSeq++')
  })
})
