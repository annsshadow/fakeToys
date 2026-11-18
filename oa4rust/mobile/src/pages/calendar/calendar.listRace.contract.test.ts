// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

// mobile vitest 环境未装配 .vue 插件，按仓内源码断言惯例守卫。
const source = readFileSync(resolve(import.meta.dirname, 'calendar.vue'), 'utf8')

describe('日历列表只认最新一次（R252）', () => {
  const loadStart = source.indexOf('async function load(')
  const loadBody = loadStart > -1 ? source.slice(loadStart, source.indexOf('onShow(', loadStart)) : ''

  it('今日/本周 tab 在加载中仍可点击：列表并发真实存在，不是被 disabled 挡掉的假竞态', () => {
    const tabLine = source.split('\n').find((l) => l.includes('@tap="switchRange(r.key)"')) ?? ''
    expect(tabLine).toContain('switchRange')
    expect(tabLine.includes('disabled')).toBe(false)
    const switchStart = source.indexOf('function switchRange(')
    const switchBody = switchStart > -1 ? source.slice(switchStart, source.indexOf('// 事件详情', switchStart)) : ''
    expect(switchBody.includes('if (loading.value)')).toBe(false)
  })

  it('load() 在发起请求前登记列表令牌（与详情门分离的独立列表门）', () => {
    expect(source.includes('const listGate = createLatestGate()')).toBe(true)
    const begin = loadBody.indexOf('const token = listGate.begin()')
    expect(begin).toBeGreaterThan(-1)
    expect(begin).toBeLessThan(loadBody.indexOf('await calendarApi.myCalendars()'))
  })

  it('过期列表响应不写 calendars/events/loadError，也不提前复位新请求的 loading', () => {
    const guard = loadBody.indexOf('if (!listGate.isLatest(token)) return')
    expect(guard).toBeGreaterThan(-1)
    expect(loadBody.indexOf('calendars.value =')).toBeGreaterThan(guard)
    expect(loadBody.indexOf('events.value =')).toBeGreaterThan(guard)
    expect(loadBody.indexOf('loadError.value = describeLoadError')).toBeGreaterThan(guard)
    expect(loadBody.indexOf('loading.value = false')).toBeGreaterThan(guard)
  })

  it('过期请求的 catch/finally 不弹错误、不提前清 loading', () => {
    const catchBlock = loadBody.slice(loadBody.indexOf('catch ('), loadBody.indexOf('finally'))
    expect(catchBlock).toContain('listGate.isLatest(token)')
    const finallyBlock = loadBody.slice(loadBody.indexOf('finally'))
    expect(finallyBlock).toContain('listGate.isLatest(token)')
  })
})
