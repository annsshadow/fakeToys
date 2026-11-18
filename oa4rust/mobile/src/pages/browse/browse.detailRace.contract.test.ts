// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

describe('移动端只读浏览详情只认最新一次（第84轮）', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'browse.vue'), 'utf8')
  const openStart = source.indexOf('async function open(')
  const openBody = openStart > -1 ? source.slice(openStart, source.indexOf('function detailText', openStart)) : ''

  it('open() 引入并创建详情请求序号令牌', () => {
    expect(source).toContain('createLatestGate')
    expect(source).toContain('detailGate')
    expect(openBody).toContain('detailGate.begin()')
  })

  it('过期详情响应在写 detail 之前被丢弃', () => {
    const guard = openBody.indexOf('if (!detailGate.isLatest(token)) return')
    expect(guard).toBeGreaterThan(-1)
    const assign = openBody.indexOf('detail.value = fetched')
    expect(assign).toBeGreaterThan(guard)
  })

  it('过期请求的错误不弹 toast、也不提前清掉新请求的 loading', () => {
    const catchBlock = openBody.slice(openBody.indexOf('catch ('), openBody.indexOf('finally'))
    expect(catchBlock).toContain('detailGate.isLatest(token)')
    const finallyBlock = openBody.slice(openBody.indexOf('finally'))
    expect(finallyBlock).toContain('detailGate.isLatest(token)')
  })
})
