// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// t31 RED→GREEN：PC 考勤统计卡必须由真实明细派生，无数据归零，禁止固定 45/42/3/2。
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import ts from 'typescript'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(__dirname, './AttendanceApp.vue'), 'utf8')
const script = source.match(/<script setup[^>]*>([\s\S]*?)<\/script>/)?.[1] ?? ''
const js = ts.transpileModule(script, {
  compilerOptions: { target: ts.ScriptTarget.ES2020, module: ts.ModuleKind.ESNext },
}).outputText

/** 从转译后的脚本中按大括号配对提取纯函数源码，使其可被真实执行。 */
function extractFn(name: string): string {
  const i = js.indexOf('function ' + name + '(')
  if (i < 0) return ''
  const open = js.indexOf('{', i)
  let depth = 0
  for (let p = open; p < js.length; p++) {
    if (js[p] === '{') depth++
    else if (js[p] === '}') {
      depth--
      if (depth === 0) return js.slice(i, p + 1)
    }
  }
  return ''
}

const fnSrc = extractFn('deriveAttendanceStats')
const derive = fnSrc
  ? (new Function(fnSrc + '; return deriveAttendanceStats;')() as (rows: any[]) => Array<{ label: string; value: number }>)
  : null

const val = (rows: any[], label: string) => derive?.(rows).find((s) => s.label === label)?.value

describe('PC 考勤统计由真实明细派生契约', () => {
  it('统计卡不再硬编码 45/42/3/2', () => {
    expect(script).not.toMatch(/value:\s*45\b/)
    expect(script).not.toMatch(/value:\s*42\b/)
    expect(script).not.toMatch(/value:\s*3\b/)
    expect(script).not.toMatch(/value:\s*2\b/)
  })

  it('存在可执行的 deriveAttendanceStats 纯函数，且 stats computed 由 records 派生', () => {
    expect(derive).toBeTypeOf('function')
    expect(script).toMatch(/const stats = computed\(\(\) => deriveAttendanceStats\(records\.value\)\)/)
  })

  it('空明细时四项统计全部归零', () => {
    expect(derive?.([]).map((s) => s.value)).toEqual([0, 0, 0, 0])
  })

  it('非空明细统计与明细同口径（人员×日期去重 / 正常 / 迟到 / 请假）', () => {
    const rows = [
      { id: '1', personName: '张三', date: '2026-11-01', checkInTime: '09:00', status: '1' },
      { id: '2', personName: '张三', date: '2026-11-02', checkInTime: '09:30', status: '2', isLate: true },
      { id: '3', personName: '李四', date: '2026-11-01', status: '3' },
    ]
    expect(val(rows, '应出勤')).toBe(3)
    expect(val(rows, '实际出勤')).toBe(2)
    expect(val(rows, '迟到')).toBe(1)
    expect(val(rows, '请假')).toBe(1)
  })

  it('业务明细接口与 testid 保持不变', () => {
    expect(script).toContain('/api/attendance/assemble/control/attendancedetail/filter/list')
    expect(source).toContain('data-testid="attendance-stats"')
  })
})
