// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R64 战报详情辅助面板竞态守卫契约（test-first RED）
 * 真实读 laiyipao/miniapp/src/pages/battles/battles.vue 源码，禁止内嵌实现副本。
 * 缺口：growthCompare / r14Advice 是无序守卫的单一 ref，快速切换两场未缓存战报时旧响应可晚到覆盖新面板。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'

const dir = dirname(fileURLToPath(import.meta.url))
const srcRoot = join(dir, '..')

function read(rel: string): string {
  return readFileSync(join(srcRoot, rel), 'utf8')
}

const BATTLES = 'pages/battles/battles.vue'

/** 提取具名函数块（大括号配对），不依赖行号。 */
function funcBlock(src: string, name: string): string | null {
  const i = src.indexOf('function ' + name + '(')
  if (i < 0) return null
  const brace = src.indexOf('{', i)
  if (brace < 0) return null
  let depth = 0
  for (let j = brace; j < src.length; j++) {
    const c = src[j]
    if (c === '{') depth++
    else if (c === '}') {
      depth--
      if (depth === 0) return src.slice(i, j + 1)
    }
  }
  return null
}

/** 通用序号守卫：seq !== detailPanelSeq / panelSeq !== detailSeq 等均可。 */
const SEQ_GUARD = /\b\w*[Ss]eq\b\s*!==?\s*\b\w*[Ss]eq\b/
/** 通用序号递增：++detailPanelSeq / detailPanelSeq++ 均可。 */
const SEQ_INC = /\+\+\s*\w*[Ss]eq\b|\b\w*[Ss]eq\b\s*\+\+/

/**
 * 语义判据：写 ref 前，每个 await 恢复后都必须复核序号；仅入口守卫不算通过。
 * t120 QA 实证的假绿即「入口守卫 + await 后无复核」，本判据必须真实检出。
 */
function guardedAfterAllAwaits(block: string, wi: number): boolean {
  const awaitIdxs: number[] = []
  const re = /\bawait\b/g
  let m: RegExpExecArray | null
  while ((m = re.exec(block)) !== null) {
    if (m.index < wi) awaitIdxs.push(m.index)
  }
  if (!awaitIdxs.length) return false
  const last = awaitIdxs[awaitIdxs.length - 1]!
  return SEQ_GUARD.test(block.slice(last, wi))
}


describe('R64 战报详情辅助面板竞态守卫契约（test-first RED）', () => {
  const src = read(BATTLES)

  it('growthCompare.value 写入前每个 await 恢复后都须复核序号（语义判据）', () => {
    const block = funcBlock(src, 'loadGrowthCompare')
    expect(block, 'loadGrowthCompare 函数块须存在').not.toBeNull()
    const wi = block!.indexOf('growthCompare.value =')
    expect(wi, 'loadGrowthCompare 须含 growthCompare.value 写入').toBeGreaterThanOrEqual(0)
    expect(guardedAfterAllAwaits(block!, wi),
      'growthCompare.value 写入前，每个 await 恢复后都须复核 seq !== detailPanelSeq 再写；仅入口单次守卫不算'
    ).toBe(true)
  })

  it('r14Advice.value 主写入前须复核 fetchAdviceAction 的 await 结果（语义判据）', () => {
    const block = funcBlock(src, 'loadR14Advice')
    expect(block, 'loadR14Advice 函数块须存在').not.toBeNull()
    const wi = block!.lastIndexOf('r14Advice.value =')
    expect(wi, 'loadR14Advice 须含 r14Advice.value 主写入').toBeGreaterThanOrEqual(0)
    expect(guardedAfterAllAwaits(block!, wi),
      'await fetchAdviceAction 恢复后、r14Advice.value 主写入前须复核 seq !== detailPanelSeq；入口单次守卫不算'
    ).toBe(true)
  })

  it('loadDetail 开头（首个 await 前）须递增展开序号', () => {
    const block = funcBlock(src, 'loadDetail')
    expect(block, 'loadDetail 函数块须存在').not.toBeNull()
    const ai = block!.indexOf('await ')
    const head = ai < 0 ? block! : block!.slice(0, ai)
    expect(SEQ_INC.test(head),
      'loadDetail 开头（首个 await 前）须递增序号（如 const seq = ++detailPanelSeq）并传入两个辅助加载函数; 源码开头未见序号递增'
    ).toBe(true)
  })

  it('不变量：details 按 id 缓存与 computed detail 绑定保留', () => {
    expect(src, 'details.value[it.id] = d 按 id 缓存须保留').toContain('details.value[it.id] = d')
    expect(src, 'computed detail 读 details[expandedId] 须保留').toMatch(/const detail = computed[\s\S]{0,160}details\.value\[expandedId\.value\]/)
  })

  it('不变量：applyAdviceAction 权威调用保留', () => {
    expect(src, 'r14ConfirmApply 中 applyAdviceAction 调用须保留').toContain('applyAdviceAction(a, store)')
  })
})