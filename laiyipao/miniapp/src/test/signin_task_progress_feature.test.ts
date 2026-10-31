// @vitest-environment node
/**
 * 「任务进度条」功能守卫（2026-11-01 新一轮第 36 轮）。
 *
 * 任务行此前只显示 `progress / target` 文本，完成度不直观。本轮在
 * signin.vue 增加：
 *   - `taskPercent` 函数（计算完成度百分比）
 *   - 任务行进度条（宽度 = 完成度）
 *
 * 该功能消失时编译照过、现有测试全绿（少一个进度条没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const SIGNIN_VUE = join(__dirname, '../pages/signin/signin.vue')

describe('任务进度条功能守卫', () => {
  it('signin.vue 必须计算任务完成度', () => {
    const src = readFileSync(SIGNIN_VUE, 'utf-8')
    expect(src).toContain('function taskPercent')
  })

  it('signin.vue 必须渲染任务进度条', () => {
    const src = readFileSync(SIGNIN_VUE, 'utf-8')
    expect(src).toContain('class="task-progress"')
    expect(src).toContain("taskPercent(t)")
  })
})