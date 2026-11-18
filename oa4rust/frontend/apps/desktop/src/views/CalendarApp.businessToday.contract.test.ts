// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R251 日历「今天」业务日计算提升契约（源码结构性性能回归防线）。
 *
 * 缺陷机理：CalendarApp.vue 的 calendarCells computed 中，const businessToday =
 * calendarBusinessToday() 位于「本月」for (let d = 1; d <= daysInMonth; d++) 循环体
 * 内部。calendarBusinessToday() 每次执行都会 new Date() 并进行 UTC+8 偏移与多次
 * getUTC* 计算；渲染一次日历网格会重复执行 28~31 次，而结果在整个 computed
 * 求值期间恒定不变。业务日判定（isToday）是纯日期比较，与循环变量 d 无关，
 * 属于典型的循环内不变式外提缺口。
 *
 * 本契约锁定源码结构：businessToday 必须在「本月」循环之前求值一次，且循环体
 * 内不再调用 calendarBusinessToday()。如果有人把它挪回循环内，或改为在循环内
 * 多次调用，本测试立即失败——修复必须保持业务日语义不变（仍走
 * calendarBusinessToday() 的 UTC+8 口径，不得改用 new Date().getDate()）。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { describe, expect, it } from 'vitest'

const here = path.dirname(new URL(import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1'))
const frontendRoot = path.resolve(here, '../../../..')
const source = readFileSync(path.join(frontendRoot, 'apps/desktop/src/views/CalendarApp.vue'), 'utf8')

/** 取 calendarCells computed 的定义体。 */
function calendarCellsBody(): string {
  const start = source.indexOf('const calendarCells = computed(() => {')
  expect(start, 'calendarCells computed 不存在').toBeGreaterThan(-1)
  const open = source.indexOf('{', start)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    const ch = source[i]
    if (ch === '{') depth += 1
    else if (ch === '}') {
      depth -= 1
      if (depth === 0) return source.slice(open + 1, i)
    }
  }
  throw new Error('calendarCells computed 未闭合')
}

/** 取「本月] 循环体。 */
function currentMonthLoop(body: string): string {
  const marker = body.indexOf('// 本月')
  expect(marker, '「本月」注释不存在').toBeGreaterThan(-1)
  const forStart = body.indexOf('for (let d = 1; d <= daysInMonth; d++) {', marker)
  expect(forStart, '「本月」for 循环不存在').toBeGreaterThan(-1)
  const open = body.indexOf('{', forStart)
  let depth = 0
  for (let i = open; i < body.length; i++) {
    const ch = body[i]
    if (ch === '{') depth += 1
    else if (ch === '}') {
      depth -= 1
      if (depth === 0) return body.slice(open + 1, i)
    }
  }
  throw new Error('「本月」for 循环未闭合')
}

describe('日历「今天」业务日计算提升（R251）', () => {
  const body = calendarCellsBody()
  const beforeLoop = body.slice(0, body.indexOf('// 本月'))
  const loop = currentMonthLoop(body)

  it('calendarCells 中业务日只计算一次，且位于「本月」循环之前', () => {
    const calls = beforeLoop.match(/calendarBusinessToday\s*\(/g) ?? []
    expect(calls.length, 'businessToday 必须在循环外通过 calendarBusinessToday 求值一次').toBe(1)
  })

  it('「本月」循环体内不得调用 calendarBusinessToday（避免 N 次重复计算）', () => {
    const calls = loop.match(/calendarBusinessToday\s*\(/g) ?? []
    expect(calls.length, '循环体内仍有 calendarBusinessToday 调用').toBe(0)
  })

  it('「本月」循环仍消费循环外求值的 businessToday 计算 isToday（语义不变）', () => {
    expect(loop, 'isToday 必须继续用 businessToday 判定').toMatch(/businessToday\.day/)
    expect(loop, 'isToday 必须继续用 businessToday 判定月份').toMatch(/businessToday\.month/)
    expect(loop, 'isToday 必须继续用 businessToday 判定年份').toMatch(/businessToday\.year/)
    expect(beforeLoop, 'businessToday 声明必须在循环外').toMatch(
      /const businessToday\s*=\s*calendarBusinessToday\s*\(\s*\)/,
    )
  })
})
