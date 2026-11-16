// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 反应 → 触发元素对（UI 展示用派生表）。
 *
 * `REACTION_TABLE` 是引擎侧实现细节（不导出）；`lookupReaction` 是公开 API。
 * 这里对五元素两两穷举反查得到每条反应的全部触发顺序对，
 * 供战斗页图例与反应图鉴共用 —— 同一张表派生，天然与引擎一致。
 * 纯静态派生数据（不落库、不进任何哈希）。
 */
import { ELEMENT_ORDER, lookupReaction, type Element, type ReactionKey } from '@/game/elements'

export type ElementPair = [Element, Element]

export const REACTION_PAIRS: Record<ReactionKey, ElementPair[]> = (() => {
  const out = {} as Record<ReactionKey, ElementPair[]>
  for (const a of ELEMENT_ORDER) {
    for (const b of ELEMENT_ORDER) {
      if (a === b) continue
      const k = lookupReaction(a, b)
      if (k) (out[k] ??= []).push([a, b])
    }
  }
  return out
})()
