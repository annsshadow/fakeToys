// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'BBSForum.vue'), 'utf8')

describe('BBSForum 版块详情回填竞态守卫', () => {
  it('可达性：版块详情按钮无 disabled，三条在途请求回填到同一浮层，无 loading 短路', () => {
    const openBtn = source.split('\n').find((l) => l.includes('@click.stop="openSectionInfo(sec)"')) ?? ''
    expect(openBtn).toContain('openSectionInfo(sec)')
    expect(openBtn.includes('disabled')).toBe(false)
    expect(source).toContain('v-if="sectionInfo.open"')
    expect(source).toContain('v-if="sectionInfo.loading"')
  })

  it('openSectionInfo 用递增序号只认最新一次，过期响应不得覆盖当前版块详情', () => {
    const start = source.indexOf('async function openSectionInfo(sec: Section): Promise<void> {')
    expect(start).toBeGreaterThan(-1)
    const slice = source.slice(start, source.indexOf('// 帖子列表', start))
    const take = slice.indexOf('const seq = ++detailSeq')
    expect(take).toBeGreaterThan(-1)
    expect(take).toBeLessThan(slice.indexOf('await Promise.all'))
    const guard = slice.indexOf('if (seq !== detailSeq) return')
    expect(guard).toBeGreaterThan(take)
    expect(guard).toBeLessThan(slice.indexOf('sectionInfo.value.name = String'))
    const guard2 = slice.indexOf('if (seq !== detailSeq) return', guard + 1)
    expect(guard2).toBeGreaterThan(guard)
    expect(guard2).toBeLessThan(slice.indexOf('sectionInfo.value.loading = false'))
  })
})
