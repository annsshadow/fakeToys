// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function pageSource(relative: string): string {
  return readFileSync(resolve(import.meta.dirname, relative), 'utf8')
}

describe('移动端写操作防重复 / 防误触（R150）', () => {
  it('日历：新建事件有进行中守卫与禁用态', () => {
    const source = pageSource('calendar/calendar.vue')
    expect(source.split('\n').length).toBeGreaterThan(300)
    expect(source).toContain('const creating = ref(false)')
    expect(source).toContain('if (creating.value) return')
    expect(source).toContain('creating.value = true')
    expect(source).toContain('creating.value = false')
    expect(source).toContain(':class="{ disabled: creating }"')
  })

  it('日历：删除先确认，确认与请求期间不可重入', () => {
    const source = pageSource('calendar/calendar.vue')
    expect(source).toContain('const removing = ref(false)')
    expect(source).toContain('if (!id || removing.value) return')
    const modal = source.indexOf("title: '删除事件'")
    const setBusy = source.indexOf('removing.value = true')
    const api = source.indexOf('calendarApi.coreEventRemove(id)')
    expect(modal).toBeGreaterThan(-1)
    expect(setBusy).toBeGreaterThan(-1)
    expect(api).toBeGreaterThan(-1)
    // 守卫必须先置位再弹确认，确认必须在删除请求之前。
    expect(setBusy).toBeLessThan(modal)
    expect(modal).toBeLessThan(api)
    expect(source).toContain(':class="{ disabled: removing }"')
  })

  it('收藏：删除进行中不可重入且有禁用态', () => {
    const source = pageSource('collect/collect.vue')
    expect(source).toContain('const removingId = ref(')
    expect(source).toContain('if (removingId.value) return')
    expect(source).toContain('removingId.value = row.id')
    expect(source).toContain(`:class="{ disabled: removingId !== '' }"`)
  })

  it('回收站：单条删除与清空共用进行中守卫', () => {
    const source = pageSource('recycle/recycle.vue')
    expect(source).toContain('const busy = ref(false)')
    expect(source.split('busy.value) return').length - 1).toBe(2)
    expect(source.split(':disabled="busy"').length - 1).toBe(2)
  })
})
