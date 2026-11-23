// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R11 真实缺陷（release-ops R10 部署观察）：skills/equipment 收益预估直接插值
 * 原始 impact 浮点，出现 5.400000000000001 这类长尾；展示层必须统一格式化。
 * gems/battles 的属性差分/战报差值同样不能裸插值浮点。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = (p: string) => readFileSync(resolve(process.cwd(), 'src/pages', p), 'utf8')

const rawBlocks = (text: string, predicate: (b: string) => boolean) =>
  (text.match(/\{\{[\s\S]*?\}\}/g) ?? []).filter(predicate)

describe('R11 收益数值展示统一格式化', () => {
  for (const page of ['skills/skills.vue', 'equipment/equipment.vue']) {
    it(`${page} 所有 impact 数值插值都走 fmt，不存在长浮点裸值`, () => {
      const text = src(page)
      expect(text, 'fmt helper must exist').toMatch(/function fmt\s*\(/)
      const fields = ['estimatedDamagePerWave','damagePerWaveDelta','damagePerWavePct','powerDelta','ratingTotalDelta','attackPermille','armorPermille','ratingTotal']
      for (const field of fields) {
        const raw = rawBlocks(text, b => b.includes('r7Impact.') && b.includes(field) && !/fmt\w*\(/.test(b))
        expect(raw, `${page} ${field} 裸插值: ${raw.join(' | ')}`).toEqual([])
      }
    })
  }
  for (const page of ['gems/gems.vue','battles/battles.vue','battle/battle.vue']) {
    it(`${page} 差分行数值插值都走 fmt`, () => {
      const text = src(page)
      expect(text, 'fmt helper must exist').toMatch(/function fmt\s*\(/)
      const raw = rawBlocks(text, b => /row\.(before|after|delta)/.test(b) && !/fmt\w*\(/.test(b))
      expect(raw, `${page} 裸插值: ${raw.join(' | ')}`).toEqual([])
    })
  }
})