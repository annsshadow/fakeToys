// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R303 热帖管理列表响应结构契约：/api/hotpic/core/entity/list 返回
 * { data: { count, data: [...] } }，doSearch 必须解包成数组，否则 stats.filter 抛 TypeError。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const s = readFileSync(resolve(import.meta.dirname, 'HotpicApp.vue'), 'utf8')

describe('R303 热帖管理列表响应结构', () => {
  it('doSearch 解包 {count,data} 包装，items 赋值为数组', () => {
    expect(s).toMatch(/const payload: any = \(r as any\)\?\.data/)
    expect(s).toMatch(/Array\.isArray\(payload\?\.data\)/)
    expect(s).toMatch(/items\.value\s*=\s*list/)
  })
  it('不把裸 r.data 包装对象直接赋给 items', () => {
    expect(s).not.toMatch(/items\.value\s*=\s*r\.data\s*\?\?\s*\[\]/)
  })
  it('保留 loadError 错误态', () => {
    expect(s).toContain('loadError.value = describeListError')
  })
})
