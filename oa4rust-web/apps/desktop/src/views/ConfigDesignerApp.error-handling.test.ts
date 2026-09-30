// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'ConfigDesignerApp.vue'), 'utf8')

describe('ConfigDesignerApp 错误处理不再吞错', () => {
  it('deleteItem 删除失败时提示并中止，不再静默从列表移除', () => {
    // 此前 catch{} 吞掉删除失败却仍从本地列表移除 → 用户误以为删成功
    expect(source).not.toMatch(/await api\.delete\(`\/api\/config\/delete\/\$\{item\.id\}`\)\s*\n\s*\}\s*catch\s*\{\}/)
    expect(source).toContain('toast.error(`删除失败:')
  })

  it('importConfigs 真实 await 每项并成败计数，不再 fire-and-forget/假成功', () => {
    expect(source).toContain('async function importConfigs()')
    expect(source).toContain("await api.post('/api/config/create', item)")
    expect(source).toContain('导入完成：成功 ${ok} / 失败 ${fail}')
    // 不再有空 catch 吞掉导入失败
    expect(source).not.toContain('        } catch {}')
  })

  it('全文件不再残留真空 catch 块', () => {
    expect(source).not.toMatch(/catch\s*(?:\([^)]*\))?\s*\{\s*\}/)
  })
})
