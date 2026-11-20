// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * 结算成功分支必须把服务端下发的 battle_id 存入 settleResult，
 * 否则「查看本局战报」按钮永远不显示、深链永远失效。
 *
 * 服务端 SettleResp.BattleID json:"battle_id" 在 SettleBattle 事务内赋值，
 * 前端 battle.vue 模板 v-if="settleResult.battle_id" 依赖该字段。
 * 本测试把这条契约变成可执行断言：成功分支的 settleResult 构造
 * 必须包含 battle_id: res.battle_id。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const BATTLE_VUE = join(__dirname, '../pages/battle/battle.vue')

describe('结算成功分支 battle_id 契约', () => {
  it('settleResult 成功构造必须写入服务端返回的 battle_id', () => {
    const src = readFileSync(BATTLE_VUE, 'utf-8')
    const start = src.indexOf('res = await api.settleBattle(payload)')
    const end = src.indexOf('if (res.new_max_stage)')
    expect(start).toBeGreaterThan(-1)
    expect(end).toBeGreaterThan(start)
    const successBlock = src.slice(start, end)
    expect(successBlock).toContain('battle_id: res.battle_id')
  })
})
