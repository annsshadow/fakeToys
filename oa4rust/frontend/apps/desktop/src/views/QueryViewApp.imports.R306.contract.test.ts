// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/** R306-A 真 bug：QueryViewApp 模板使用 AdminPageHeader/AdminFormSection/AdminToolbar
 *  但 script 只 void 未导入，运行时 ReferenceError 导致整页白屏。 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, 'QueryViewApp.vue'), 'utf8')

describe('QueryViewApp 管理原语真实导入契约', () => {
  it('模板引用的三个 admin 组件必须从 barrel 显式 import', () => {
    expect(src).toMatch(/<AdminPageHeader\b/)
    expect(src).toMatch(/<AdminFormSection\b/)
    expect(src).toMatch(/<AdminToolbar\b/)
    expect(src).toMatch(/import\s*\{\s*ADMIN_HIDE_PAGE_TITLE,\s*(ADMIN_LAYOUT_HIDE_TITLE,\s*)?AdminFormSection,\s*AdminPageHeader,\s*AdminToolbar,?\s*\}\s*from\s*['"]\.\.\/components\/admin\/['"]/)
  })

  it('不得用 void 占位代替真实组件导入', () => {
    expect(src).not.toMatch(/void\s+AdminFormSection/)
    expect(src).not.toMatch(/void\s+AdminPageHeader/)
    expect(src).not.toMatch(/void\s+AdminToolbar/)
  })
})
