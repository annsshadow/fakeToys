// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R302 文档管理列表端点契约：管理视图不得调用需要 ids 的 /document/list/document（会 400），
 * 应统一走 /document/filter/list/{page}/size/{size}/manager（管理员全量口径，200）。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const s = readFileSync(resolve(import.meta.dirname, 'DocumentApp.vue'), 'utf8')

describe('R302 文档管理列表端点', () => {
  it('doSearch 不再调用需要 ids 的 /document/list/document', () => {
    expect(s).not.toMatch(/api\.post\(['"]\/api\/document\/list\/document['"]/)
  })
  it('主列表加载走 filter/list/.../manager 端点', () => {
    expect(s).toMatch(/api\.post\(['"]\/api\/document\/filter\/list\/1\/size\/50\/manager['"]/)
  })
  it('保留 loadError 错误态', () => {
    expect(s).toContain('loadError.value = describeListError')
  })
})
