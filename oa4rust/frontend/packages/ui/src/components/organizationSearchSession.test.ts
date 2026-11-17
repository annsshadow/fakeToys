// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { createOrganizationSearchSession } from './organizationSearchSession'

interface Item {
  type: string
  name: string
}

function deferred<T>() {
  let resolve!: (value: T) => void
  const promise = new Promise<T>((r) => {
    resolve = r
  })
  return { promise, resolve }
}

describe('组织选择器搜索会话', () => {
  it('切换类型后，旧类型在途搜索的结果不得展示给用户', async () => {
    const session = createOrganizationSearchSession()
    let displayed: Item[] = []
    const pending = deferred<Item[]>()

    const unitAttempt = session.begin('unit')
    const unitRun = pending.promise.then((found) => {
      if (session.shouldApply(unitAttempt)) displayed = found
    })

    // 组织搜索仍在飞行时，用户切到“人员”（并未发起新搜索）
    session.switchType('person')
    pending.resolve([{ type: 'unit', name: '研发部' }])
    await unitRun

    // 用户切到人员后，绝不能看到旧的组织结果
    expect(displayed).toEqual([])
  })

  it('切换类型后再搜索新类型，只展示新类型的最新结果', async () => {
    const session = createOrganizationSearchSession()
    let displayed: Item[] = []
    const stale = deferred<Item[]>()

    const unit = session.begin('unit')
    const staleRun = stale.promise.then((found) => {
      if (session.shouldApply(unit)) displayed = found
    })
    session.switchType('person')
    const person = session.begin('person')
    const personFound = [{ type: 'person', name: '张三' }]
    if (session.shouldApply(person)) displayed = personFound
    stale.resolve([{ type: 'unit', name: '研发部' }])
    await staleRun

    expect(displayed).toEqual(personFound)
  })

  it('最新搜索结果正常展示，loading 由最新请求收起', () => {
    const session = createOrganizationSearchSession()
    const attempt = session.begin('person')
    expect(session.shouldApply(attempt)).toBe(true)
    expect(session.isLatest(attempt)).toBe(true)
  })

  it('切换类型后旧请求仍负责收起 loading，按钮不会永久停在搜索中', () => {
    const session = createOrganizationSearchSession()
    const attempt = session.begin('unit')
    session.switchType('person')
    expect(session.shouldApply(attempt)).toBe(false)
    expect(session.isLatest(attempt)).toBe(true)
  })
})
