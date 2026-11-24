// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R496 SQL语句设计器 .sb-item 键盘可达性契约（WCAG 2.1.1）：
 * :53-55 语句项必须 role=button + tabindex=0 + :aria-pressed + keydown.enter/space 复用 selectStatement(s)，
 * 原 @click、行内编辑/删除 @click.stop、class/结构、selectStatement 函数体零改。真父 RED、修复版 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'QueryStatementDesigner.vue'), 'utf8')

/** 取 .sb-item 多行开标签（从 class="sb-item" 往前找 <div，往后找开标签结束 >） */
function sbItemTag(src: string): string {
  const at = src.indexOf('class="sb-item"')
  if (at < 0) return ''
  const start = src.lastIndexOf('<div', at)
  const end = src.indexOf('>', at)
  return src.slice(start, end < 0 ? src.length : end + 1)
}

describe('R496 SQL语句设计器语句项键盘可达性契约', () => {
  const tag = sbItemTag(source)

  it('.sb-item 加 role=button + tabindex=0 + aria-pressed + keydown.enter/space 复用 selectStatement（核心）', () => {
    expect(tag, '必须找到 .sb-item 开标签').not.toBe('')
    expect(tag).toMatch(/role="button"/)
    expect(tag).toMatch(/tabindex="0"/)
    expect(tag).toMatch(/:aria-pressed="currentStatement\?\.id===s\.id"/)
    expect(tag).toMatch(/@keydown\.enter="selectStatement\(s\)"/)
    expect(tag).toMatch(/@keydown\.space\.prevent="selectStatement\(s\)"/)
  })

  it('业务零改守卫：原 @click=selectStatement(s) 与 v-for/:class active 保留', () => {
    expect(tag).toContain('@click="selectStatement(s)"')
    expect(tag).toContain('v-for="s in filtered"')
    expect(tag).toContain(':key="s.id"')
    expect(tag).toContain(':class="{active: currentStatement?.id===s.id}"')
  })

  it('业务零改守卫：行内编辑/删除按钮 @click.stop 保留且无一调用 selectStatement', () => {
    expect(source).toContain('@click.stop="editStatement(s)"')
    expect(source).toContain('@click.stop="deleteStatement(s)"')
    expect(source).not.toMatch(/@click\.stop="selectStatement/)
  })

  it('业务零改守卫：selectStatement 函数体保留', () => {
    expect(source).toContain('function selectStatement(s: Stmt) {')
    expect(source).toContain('currentStatement.value = s')
    expect(source).toContain("sql.value = s.sql ?? ''")
  })

  it('业务零改守卫：class/图标/信息结构零改', () => {
    expect(tag).toContain('class="sb-item"')
    expect(source).toContain('class="si-icon"')
    expect(source).toContain('class="si-name"')
    expect(source).toContain('class="si-meta"')
  })
})
