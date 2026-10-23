/**
 * 战斗事件 → 音效的映射（2026-10-23）。
 *
 * # 为什么单独一层，而不是在 battle.vue 里写 switch
 *
 * 这一层要处理的是**节流**，而节流规则是「一次 tick 里最多几声」——
 * 那是一条可以被测的行为，不该埋在一个 800 行的页面组件里。
 *
 * # 为什么必须节流
 *
 * 一个 tick 里 `hit` 可能触发几十次（一发穿透弹打中一排敌人）。
 * 逐个播放会有两个后果：
 *
 *  1. **听感上糊成噪声墙**，玩家反而听不出"打中了"；
 *  2. 每次播放都是一次 native 调用，密集时直接吃掉帧预算 ——
 *     这正是 `perf_frame.test.ts` 守着的那类开销。
 *
 * 所以 `hit` 每批最多 2 声，其余"每批一声"。
 * 数字取 2 而不是 1：单声会让高频连射听起来像卡顿。
 */

import type { BattleEvent } from '@/game/engine'
import { playSfx } from './sfx'

/** 一次 tick 内最多播放几声 `hit`。 */
export const MAX_HIT_PER_BATCH = 2

/**
 * 把一批战斗事件翻译成音效。
 *
 * 只关心「有声音反馈」的几类；`card_offer` / `terrain` 等留给视觉层
 * （地形触发已经有自己的动画，再叠音效会喧宾夺主）。
 */
export function playBattleEvents(events: readonly BattleEvent[]): void {
  let hits = 0
  let fired = false
  for (const e of events) {
    switch (e.type) {
      case 'fire':
        // 同一 tick 内多个技能同时开火只响一声，否则四技能齐射是四连音
        if (!fired) {
          playSfx('shot')
          fired = true
        }
        break
      case 'hit':
        if (hits < MAX_HIT_PER_BATCH) {
          playSfx('hit')
          hits++
        }
        break
      case 'reaction':
        playSfx('reaction')
        break
      case 'kill':
        playSfx('kill')
        break
      case 'leak':
        playSfx('leak')
        break
      case 'won':
        playSfx('win')
        break
      case 'lost':
        playSfx('lose')
        break
      default:
        // 其余事件没有音效
        break
    }
  }
}
