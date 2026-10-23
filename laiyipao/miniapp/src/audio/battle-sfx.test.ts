// @vitest-environment jsdom
/**
 * 战斗事件 → 音效映射（2026-10-23）。
 *
 * 核心判据只有一条：**节流真的生效**。
 * 一个 tick 里 `hit` 可能来几十次，逐个播放会把声音糊成噪声墙，
 * 而且每次播放都是一次 native 调用（与 `perf_frame.test.ts` 守的是同一类开销）。
 *
 * 断言方式：把 `playSfx` 打桩记账，直接数它被叫了几次 ——
 * 不去检查「有没有调用音效模块」这种结构性问题。
 */
import { describe, it, expect, vi, beforeEach } from 'vitest'

vi.mock('./sfx', () => ({ playSfx: vi.fn() }))

import { playSfx } from './sfx'
import { playBattleEvents, MAX_HIT_PER_BATCH } from './battle-sfx'
import type { BattleEvent } from '@/game/engine'

const playMock = vi.mocked(playSfx)

beforeEach(() => {
  playMock.mockClear()
})

describe('audio/battle-sfx', () => {
  it('一次 tick 内几十次命中只播 MAX_HIT_PER_BATCH 声', () => {
    const events = Array.from({ length: 40 }, () => ({ type: 'hit' })) as unknown as BattleEvent[]
    playBattleEvents(events)
    expect(playMock.mock.calls.filter((c) => c[0] === 'hit').length).toBe(MAX_HIT_PER_BATCH)
  })

  it('多个技能同时开火只响一声（四技能齐射不该是四连音）', () => {
    const events = [
      { type: 'fire', slot: 0, skillId: 1 },
      { type: 'fire', slot: 1, skillId: 2 },
      { type: 'fire', slot: 2, skillId: 3 },
      { type: 'fire', slot: 3, skillId: 4 },
    ] as unknown as BattleEvent[]
    playBattleEvents(events)
    expect(playMock.mock.calls.filter((c) => c[0] === 'shot').length).toBe(1)
  })

  it('反应 / 击杀 / 漏怪 / 胜负各自映射到对应音效', () => {
    playBattleEvents([
      { type: 'reaction' },
      { type: 'kill' },
      { type: 'leak' },
      { type: 'won' },
    ] as unknown as BattleEvent[])
    expect(playMock.mock.calls.map((c) => c[0])).toEqual(['reaction', 'kill', 'leak', 'win'])

    playMock.mockClear()
    playBattleEvents([{ type: 'lost' }] as unknown as BattleEvent[])
    expect(playMock.mock.calls.map((c) => c[0])).toEqual(['lose'])
  })

  it('没有音效的事件不发声（选牌 / 地形 / 波次）', () => {
    playBattleEvents([
      { type: 'card_offer', cards: [] },
      { type: 'wave_start', index: 0 },
      { type: 'terrain', kind: 'oil_drum', x: 0n, y: 0n },
      { type: 'overheat' },
    ] as unknown as BattleEvent[])
    expect(playMock).not.toHaveBeenCalled()
  })

  it('空批次不发声（每 tick 都会调用，绝大多数是空的）', () => {
    playBattleEvents([])
    expect(playMock).not.toHaveBeenCalled()
  })
})
