// @vitest-environment node
/**
 * 「选关页通关掉落预览」功能守卫（2026-11-01 新一轮第 39 轮）。
 *
 * 选关页此前只显示血量/波次/难度/体力/星级门槛/敌人组成 —— 玩家
 * 进关前看不到这关能拿多少金币/钥匙，「打这关值不值」只能打完看结算。
 * 本轮新增：
 *   - stage.vue 用 `store.config.drop_rates` 汇总掉落预览
 *   - 展示基础金币 / 三星金币 / 击杀预估（按敌人组成求和）/ 反应加成 /
 *     三星钥匙 / 通关返还体力
 *
 * 该功能消失时编译照过、现有测试全绿（少一个区块没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const STAGE_VUE = join(__dirname, '../pages/stage/stage.vue')

describe('选关页通关掉落预览守卫', () => {
  it('stage.vue 必须从 drop_rates 汇总掉落预览（含击杀预估）', () => {
    const src = readFileSync(STAGE_VUE, 'utf-8')
    expect(src).toContain('store.config?.drop_rates')
    expect(src).toContain('w.spawns.reduce((s, sp) => s + sp.count, 0)')
    expect(src).toContain('killCoin: d.coin_per_kill * totalEnemies')
  })

  it('stage.vue 必须展示基础/三星/击杀/反应/钥匙/返还体力的掉落明细', () => {
    const src = readFileSync(STAGE_VUE, 'utf-8')
    expect(src).toContain('基础金币')
    expect(src).toContain('三星金币')
    expect(src).toContain('击杀金币（约')
    expect(src).toContain('反应加成（每 10 次反应）')
    expect(src).toContain('三星钥匙')
    expect(src).toContain('通关返还体力')
  })

  it('击杀预估随敌人组成变化（约 N 只），敌人预览拉取失败时不显示击杀行', () => {
    const src = readFileSync(STAGE_VUE, 'utf-8')
    expect(src).toContain('dropPreview.totalEnemies > 0')
    expect(src).toContain('dropPreview.totalEnemies')
  })
})