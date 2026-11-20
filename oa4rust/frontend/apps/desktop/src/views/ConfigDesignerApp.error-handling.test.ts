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

  it('列表从 /api/config/system 真实装载，不再引用未定义的 data', () => {
    // 修复前：items = ref(data.value ?? []) 中 data 从未定义，setup 阶段直接 ReferenceError。
    expect(source).not.toContain('data.value')
    expect(source).toContain("api.get('/api/config/system')")
    expect(source).toContain('items.length > 0 && loadError')
    expect(source.indexOf('filteredItems.length===0 && loadError')).toBeLessThan(
      source.indexOf('class="empty">暂无配置'),
    )
  })

  it('全文件不再残留真空 catch 块', () => {
    expect(source).not.toMatch(/catch\s*(?:\([^)]*\))?\s*\{\s*\}/)
  })

  it('copyConfig 真实 await copyText 并按成败提示，不再裸调 writeText 误报成功', () => {
    // 修复前：navigator.clipboard.writeText 未 await 也无 catch，且无条件 toast.info('已复制')，
    // 复制失败（权限/CSP/非安全上下文）时用户被误报成功且产生 unhandledrejection。
    expect(source).toContain('async function copyConfig()')
    expect(source).toContain('await copyText(config.value)')
    expect(source).toContain("import { confirmMsg, copyText, toast } from '../utils/toast'")
    expect(source).not.toContain('navigator.clipboard.writeText(config.value)')
    expect(source).not.toContain("toast.info('已复制')")
  })
})
