// @vitest-environment node
/**
 * 「选关页敌人组成预览」功能守卫（2026-10-31 新一轮第 29 轮）。
 *
 * 选关页此前只显示血量/波次/难度/地形/星级门槛，玩家不知道这关会
 * 遇到什么敌人。本轮新增：
 *   - stage.vue watch selected 时调用 `api.fetchLevel` 拉取完整波次
 *   - 模板展示每波敌人（名称 × 数量）
 *
 * 该功能消失时编译照过、现有测试全绿（少一个区块没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const STAGE_VUE = join(__dirname, '../pages/stage/stage.vue')

describe('选关页敌人组成预览守卫', () => {
  it('stage.vue 必须按需拉取完整关卡波次', () => {
    const src = readFileSync(STAGE_VUE, 'utf-8')
    expect(src).toContain('api.fetchLevel(lv.id)')
    expect(src).toContain('detailWaves.value = detail.waves')
  })

  it('stage.vue 必须展示每波敌人名称与数量', () => {
    const src = readFileSync(STAGE_VUE, 'utf-8')
    expect(src).toContain('enemyName(sp.enemy_id)')
    expect(src).toContain('×{{ sp.count }}')
  })
})