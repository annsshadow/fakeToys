// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

describe('移动端通讯录详情只认最新一次（第85轮）', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'contacts.vue'), 'utf8')
  const personStart = source.indexOf('async function openPerson(')
  const personBody =
    personStart > -1 ? source.slice(personStart, source.indexOf('function orgNameOf', personStart)) : ''
  const unitStart = source.indexOf('async function openUnit(')
  const unitBody = unitStart > -1 ? source.slice(unitStart, source.indexOf('function closeUnitPeople', unitStart)) : ''

  it('引入并创建两个独立的详情令牌', () => {
    expect(source).toContain('createLatestGate')
    expect(source).toContain('personDetailGate')
    expect(source).toContain('unitPeopleGate')
  })

  it('openPerson 过期响应在写身份/角色前被丢弃', () => {
    expect(personBody).toContain('personDetailGate.begin()')
    const guard = personBody.indexOf('if (!personDetailGate.isLatest(token)) return')
    expect(guard).toBeGreaterThan(-1)
    const assign = personBody.indexOf('identities.value = (ids.value.data')
    expect(assign).toBeGreaterThan(guard)
  })

  it('openPerson 过期请求不清掉新请求的 detailLoading', () => {
    const finallyBlock = personBody.slice(personBody.indexOf('finally'))
    expect(finallyBlock).toContain('personDetailGate.isLatest(token)')
  })

  it('openUnit 过期响应在写单位人员前被丢弃', () => {
    expect(unitBody).toContain('unitPeopleGate.begin()')
    const guard = unitBody.indexOf('if (!unitPeopleGate.isLatest(token)) return')
    expect(guard).toBeGreaterThan(-1)
    const assign = unitBody.indexOf('unitPeople.value = result.rows')
    expect(assign).toBeGreaterThan(guard)
  })
})
