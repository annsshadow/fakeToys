// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R356 契约：H5 考勤页月度统计字段错配（R355 审计 P2）。
 * 后端 v2_my_statistic 真实返回 detailByStatus:[{status,count}]，旧 statText 只读平铺 normalCount/abnormalCount，
 * 导致任何真实数据都显示「正常 0 · 异常 0」。源码级 bodyOf 提取 statText + new Function 真执行注入 myStat 桩；
 * 剥离 TS 标注使函数体可执行，不引入新依赖。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, './attendance.vue'), 'utf8')

function bodyOf(source: string, signature: string): string {
  const start = source.indexOf(signature)
  if (start < 0) throw new Error('未找到函数: ' + signature)
  const open = source.indexOf('{', start)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === '{') depth++
    else if (source[i] === '}') {
      depth--
      if (depth === 0) return source.slice(open + 1, i)
    }
  }
  throw new Error('花括号未配对: ' + signature)
}

// 剥离 TS 参数/返回标注（: string），纯字符串替换，无正则转义。
const statBody = bodyOf(src, 'function statText()').split(': string').join('')
const Fn = Object.getPrototypeOf(function () {}).constructor

function runStat(stat: unknown): string {
  const MyStatRef = { value: stat }
  const fn = new Fn('myStat', statBody)
  return String(fn(MyStatRef))
}

describe('R356 attendance.vue statText 月度统计字段契约', () => {
  it('RED核心：后端真实 detailByStatus 聚合 normal=5/abnormal=3 显示正常5异常3', () => {
    const s = { month: '2026-11', detailByStatus: [{ status: 'normal', count: 5 }, { status: 'abnormal', count: 3 }], recordCount: 8 }
    expect(runStat(s)).toBe('2026-11 · 正常 5 · 异常 3')
  })

  it('状态字面量大小写与中文兼容，同状态多行求和', () => {
    const s = { month: '2026-10', detailByStatus: [{ status: 'NORMAL', count: 2 }, { status: 'normal', count: 4 }, { status: '异常', count: 1 }, { status: 'ABNORMAL', count: 2 }] }
    expect(runStat(s)).toBe('2026-10 · 正常 6 · 异常 3')
  })

  it('平铺字段仅作 fallback：detailByStatus 为空时保留旧形状兼容', () => {
    const s = { month: '2026-09', normalCount: 7, abnormalCount: 2, detailByStatus: [] }
    expect(runStat(s)).toBe('2026-09 · 正常 7 · 异常 2')
  })

  it('聚合优先于平铺：两者同时在时以 detailByStatus 为准', () => {
    const s = { month: '2026-08', normalCount: 99, abnormalCount: 99, detailByStatus: [{ status: 'normal', count: 1 }, { status: 'abnormal', count: 4 }] }
    expect(runStat(s)).toBe('2026-08 · 正常 1 · 异常 4')
  })

  it('坏数据容错：非数组/坏元素/坏 count 不抛异常，退化为安全计数', () => {
    expect(runStat({ month: '2026-07', detailByStatus: 'bad', normalCount: 3, abnormalCount: 1 })).toBe('2026-07 · 正常 3 · 异常 1')
    expect(runStat({ month: '2026-07', detailByStatus: [null, { status: 'normal', count: 'oops' }, { status: 'abnormal', count: 2 }] })).toBe('2026-07 · 正常 0 · 异常 2')
  })

  it('month 文案不变：month / yearMonth / 默认本月，空统计仍提示暂无统计', () => {
    expect(runStat({ yearMonth: '2026-06', detailByStatus: [] })).toBe('2026-06 · 正常 0 · 异常 0')
    expect(runStat({ detailByStatus: [] })).toBe('本月 · 正常 0 · 异常 0')
    expect(runStat(null)).toBe('暂无统计')
  })

  it('写读链保护：打卡/请假端点与载荷未动，statText 仍由模板调用', () => {
    expect(src).toContain('attendanceApi.check(kind)')
    expect(src).toContain('attendanceApi.checkFromOut(kind)')
    expect(src).toContain('attendanceApi.selfHolidayApply({ holidayDate: leaveDate.value, reason: leaveReason.value })')
    expect(src).toContain('{{ statText() }}')
  })
})