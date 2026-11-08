// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function view(name: string): string {
  return readFileSync(resolve(import.meta.dirname, name), 'utf8')
}

describe('桌面端破坏性操作二次确认与重入守卫（R153）', () => {
  it('回收站：彻底删除与清空共用进行中守卫', () => {
    const source = view('RecycleApp.vue')
    expect(source).toContain('const busy = ref(false)')
    expect(source.split('if (busy.value) return').length - 1).toBe(2)
    expect(source.split('busy.value = true').length - 1).toBe(2)
    expect(source.split('busy.value = false').length - 1).toBe(2)
    expect(source).toContain(':disabled="!hasItems || busy"')
    expect(source).toContain(':disabled="busy"')
    // 不可逆动作的二次确认不能被守卫改动吞掉。
    expect(source).toContain('此操作不可恢复')
    expect(source).toContain('所有项目将被永久删除')
  })

  it('日历：结束与删除都有守卫，结束先确认', () => {
    const source = view('CalendarApp.vue')
    expect(source).toContain('const finishingEvent = ref(false)')
    expect(source).toContain('const removingEvent = ref(false)')
    expect(source).toContain('if (finishingEvent.value) return')
    expect(source).toContain('if (removingEvent.value) return')
    const confirmIdx = source.indexOf("confirmMsg('确定结束该事件？结束后不可恢复。')")
    const apiIdx = source.indexOf('/api/calendar/event/update')
    expect(confirmIdx).toBeGreaterThan(-1)
    expect(apiIdx).toBeGreaterThan(-1)
    expect(confirmIdx).toBeLessThan(apiIdx)
    expect(source).toContain(':disabled="finishingEvent"')
    expect(source).toContain(':disabled="removingEvent"')
  })

  it('收集/笔记：删除按行守卫并禁用', () => {
    const pages: Array<[string, string]> = [
      ['CollectApp.vue', '确定删除该收集记录？'],
      ['NoteApp.vue', '确定删除该笔记？'],
    ]
    for (const [file, msg] of pages) {
      const source = view(file)
      expect(source).toContain("const deletingId = ref('')")
      expect(source).toContain('if (deletingId.value) return')
      expect(source).toContain(`confirmMsg('${msg}')`)
      // 请求结束（成功或失败）都必须复位守卫，防止按钮永久卡死。
      expect(source).toContain("onSettled: () => (deletingId.value = '')")
      expect(source).toContain(':disabled="deletingId === item.id"')
    }
  })

  it('文档：删除有守卫、失败可见与禁用', () => {
    const source = view('DocumentApp.vue')
    expect(source).toContain("const deletingId = ref('')")
    const guardIdx = source.indexOf('if (deletingId.value) return')
    const apiIdx = source.indexOf('api.delete(`/api/document/${item.id}`)')
    const resetIdx = source.indexOf("deletingId.value = ''")
    expect(guardIdx).toBeGreaterThan(-1)
    expect(apiIdx).toBeGreaterThan(-1)
    expect(resetIdx).toBeGreaterThan(apiIdx)
    expect(source).toContain(':disabled="deletingId === item.id"')
    expect(source).toContain('删除失败')
  })

  it('文件：删除有守卫、进行中禁用', () => {
    const source = view('FileManager.vue')
    expect(source).toContain("const deletingId = ref('')")
    expect(source).toContain('if (deletingId.value) return')
    expect(source).toContain(':disabled="deletingId === f.id"')
    expect(source).toContain("onSettled: () => (deletingId.value = '')")
    expect(source).toContain('删除失败')
  })
})
