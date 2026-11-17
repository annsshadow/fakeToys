// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * 装备图鉴 base_bonus_pct 单位回归（组件行为，不扫源码）。
 *
 * 服务端契约：domain.SeedEquipment.BaseBonusPct 是**千分比**（‰）。
 * 佐证：content.go 的装备 Descr 明写「攻击力 +50‰」且 BaseBonusPct=50；
 * equipment_descr_test.go 用 strconv.FormatInt(BaseBonusPct)+"‰" 拼说明；
 * service/build_parts.go 直接 c.Attack += eq.BaseBonusPct 做千分比叠加。
 * 背包 bag.vue 也把同一字段显示为「攻击 +50‰」。
 *
 * 因此图鉴页把它标成「加成 50%」是单位错误（50‰ 实为 5%）。
 * 这里挂载组件、断言真实渲染文本，业务语义变化一定会让测试变红。
 */
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import Equipment from './equipment.vue'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig } from '../../test/fixtures'

beforeEach(() => {
  installUniMock()
})
afterEach(() => {
  vi.unstubAllGlobals()
})

describe('装备图鉴：base_bonus_pct 按千分比展示', () => {
  it('单件装备的加成显示为 ‰，不能显示成 %', () => {
    const cfg = makeConfig({
      equipment: [
        {
          id: 1,
          code: 'w_p1',
          name: '制式炮管',
          slot: 'weapon',
          tier: 1,
          element: 'kinetic',
          descr: '攻击力 +50‰',
          base_armor: 0,
          base_bonus_pct: 50,
          unlock_level: 1,
        },
      ],
    } as never)
    const { wrapper } = mountPage(Equipment, (store) => {
      store.config = cfg
    })
    const stats = wrapper.find('.rxn.equip .rxn-stats')
    expect(stats.exists()).toBe(true)
    // 服务端 50‰ = 5%，应显示「加成 50‰」而不是「加成 50%」
    expect(stats.text()).toContain('加成 50‰')
    expect(stats.text()).not.toContain('加成 50%')
    wrapper.unmount()
  })
})
