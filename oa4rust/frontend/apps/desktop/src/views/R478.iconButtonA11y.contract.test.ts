// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R478 WCAG 4.1.2 纯图标按钮无障碍名称契约：
 * 符号内容遮蔽 title，必须用 aria-label 给按钮中文可访问名。
 * 真父版必 RED（核心断言失败），修复版必 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const dir = import.meta.dirname
const pd = readFileSync(resolve(dir, 'ProcessDesigner.vue'), 'utf8')
const fa = readFileSync(resolve(dir, 'FormApp.vue'), 'utf8')
const cd = readFileSync(resolve(dir, 'ConfigDesignerApp.vue'), 'utf8')
const fm = readFileSync(resolve(dir, 'FileManager.vue'), 'utf8')
const ca = readFileSync(resolve(dir, 'CalendarApp.vue'), 'utf8')

/** 取某按钮模板开标签（按唯一 token 定位） */
function btn(src: string, token: string): string {
  const at = src.indexOf(token)
  if (at < 0) return ''
  const open = src.lastIndexOf('<button', at)
  const close = src.indexOf('>', at)
  return open >= 0 && close > open ? src.slice(open, close + 1) : ''
}

describe('R478 纯图标按钮 aria-label 修复契约', () => {
  it('ProcessDesigner 撤销/重做/清空三按钮有中文 aria-label 且保留 title 与业务', () => {
    const undo = btn(pd, 'title="撤销"')
    const redo = btn(pd, 'title="重做"')
    const clear = btn(pd, 'title="清空"')
    expect(undo).toContain('aria-label="撤销"')
    expect(redo).toContain('aria-label="重做"')
    expect(clear).toContain('aria-label="清空"')
    expect(undo).toContain('@click="undo"')
    expect(redo).toContain('@click="redo"')
    expect(clear).toContain('@click="clearCanvas"')
    expect(pd).toContain('>↩</button>')
    expect(pd).toContain('>↪</button>')
    expect(pd).toContain('>🗑</button>')
  })

  it('FormApp 删除表单按钮有 aria-label 且保留 @click.stop/class', () => {
    const del = btn(fa, '@click.stop="deleteForm(f)"')
    expect(del).toContain('aria-label="删除表单"')
    expect(del).toContain('class="btn-sm btn-del"')
    expect(del).toContain('@click.stop="deleteForm(f)"')
    expect(fa).toContain('>🗑</button>')
  })

  it('ConfigDesigner 编辑/删除/历史删除三按钮有中文 aria-label 且保留 title 与业务', () => {
    const edit = btn(cd, '@click.stop="editItem(item)"')
    const del = btn(cd, '@click.stop="deleteItem(item)"')
    const hist = btn(cd, '@click="configHistory.splice(hi,1)"')
    expect(edit).toContain('aria-label="编辑"')
    expect(del).toContain('aria-label="删除"')
    expect(hist).toContain('aria-label="删除历史记录"')
    expect(edit).toContain('title="编辑"')
    expect(del).toContain('title="删除"')
    expect(edit).toContain('class="si-btn"')
    expect(del).toContain('class="si-btn si-del"')
    expect(cd).toContain('>✏</button>')
    expect(cd).toContain('>🗑</button>')
  })

  it('FileManager 列表删除与可清理文件删除按钮有中文 aria-label 且保留 title/业务', () => {
    const listDel = btn(fm, '@click.stop="deleteFile(f)"')
    const recDel = btn(fm, '@click="deleteFile(f)"')
    expect(listDel).toContain('aria-label="删除文件"')
    expect(recDel).toContain('aria-label="删除可清理文件"')
    expect(listDel).toContain('title="删除"')
    expect(recDel).toContain('title="删除（进入回收站）"')
    expect(listDel).toContain('@click.stop="deleteFile(f)"')
    expect(recDel).toContain('@click="deleteFile(f)"')
  })

  it('Calendar cal-chip 数据驱动空名有 aria-label fallback 且保留原文本渲染', () => {
    expect(ca).toMatch(/:aria-label="c\.name \|\| '选择日历'"/)
    expect(ca).toContain('@click="selectCalendar(c)"')
    expect(ca).toContain('>{{ c.name }}</button>')
  })
})
