// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R112 R7 对比弹层键盘闭环契约（test-first RED）。
 * 真实 fs.readFileSync 读两页 vue / tokens.css / r7ImpactApply 契约 / battle.vue，禁止内嵌实现副本。
 * 12 缺口（两页各 6）：mask dialog+ref+esc+tab、两 setup ref、open 触发源+nextTick+focus、
 *   close 焦点归还、内联关闭统一、trapR7CompareTab。
 * 6 不变量：两步确认、失败保留原配置+请重试、令牌+零--r112-、R73/R103 闭环、假守卫记录、v-if/@click.stop。
 * 焦点边界（队长裁定）：CTA 为 UNI-TEXT，H5 focus() 不生效，移入优先 focus 弹层根 view(UNI-VIEW)；
 *   归还目标 CTA 若落 BODY，按 R103 先例由契约静态守源码，QA 如实披露不冒充真浏览器 PASS。
 *   dialog 语义/ESC 关闭/Tab 陷阱属 DOM 层，必须真浏览器坐实。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const BS = String.fromCharCode(92)
const hasRef = (s: string, n: string) => new RegExp(n + BS + 's*=' + BS + 's*ref').test(s)

/** 按唯一 class 锚点定位开标签（从 <tag 到 >），不依赖行号。 */
function tagBefore(src: string, needle: string, tag: string) {
  const k = src.indexOf(needle)
  if (k < 0) return ''
  const start = src.lastIndexOf('<' + tag, k)
  if (start < 0 || start > k) return ''
  const end = src.indexOf('>', k)
  return end < 0 ? src.slice(start) : src.slice(start, end + 1)
}
/** 括号配对取函数体（含大括号），不依赖行号。 */
function blockAfter(text: string, keyword: string) {
  const k = text.indexOf(keyword)
  if (k < 0) return ''
  const open = text.indexOf('{', k)
  if (open < 0) return ''
  let depth = 0
  for (let i = open; i < text.length; i++) {
    const ch = text[i]
    if (ch === '{') depth++
    else if (ch === '}') { depth--; if (depth === 0) return text.slice(open, i + 1) }
  }
  return ''
}

const PAGES = [
  { name: 'skills', write: 'persistLoadout' },
  { name: 'equipment', write: 'applyEquipmentImpact' }
]

describe('R112 R7 对比弹层键盘闭环契约', () => {
  const tokens = readSrc('styles/tokens.css')
  const oldGuard = readSrc('styles/r7ImpactApply.contract.test.ts')
  const battle = readSrc('pages/battle/battle.vue')

  for (const pg of PAGES) {
    const src = readSrc('pages/' + pg.name + '/' + pg.name + '.vue')
    const tpl = src.slice(0, src.indexOf('</template>'))
    const mask = () => tagBefore(tpl, 'class="r7-compare-mask"', 'view')
    const openBody = () => blockAfter(src, 'function r7OpenCompare')
    const closeBody = () => blockAfter(src, 'function closeR7Compare')
    const trapBody = () => blockAfter(src, 'function trapR7CompareTab')

    it(pg.name + ' G1 mask 补 dialog 语义 + ref + ESC/Tab 守卫', () => {
      const tag = mask()
      expect(tag, '须定位 r7-compare-mask 开标签').not.toBe('')
      expect(tag, '须有 role=dialog').toContain('role="dialog"')
      expect(tag, '须有 aria-modal=true').toContain('aria-modal="true"')
      expect(tag, '须有 aria-label 当前 vs 推荐·确认替换').toContain('aria-label="当前 vs 推荐·确认替换"')
      expect(tag, '须有 ref=r7CompareRef').toContain('ref="r7CompareRef"')
      expect(tag, '须有 ESC 关闭').toContain('@keydown.esc="closeR7Compare"')
      expect(tag, '须有 Tab 陷阱').toContain('@keydown.tab="trapR7CompareTab"')
    })

    it(pg.name + ' G2 setup 增 r7CompareRef/r7TriggerEl 两 ref', () => {
      for (const n of ['r7CompareRef', 'r7TriggerEl']) {
        expect(hasRef(src, n), n + ' 须以 ref(null) 声明').toBe(true)
      }
    })

    it(pg.name + ' G3 open 记录触发源 + nextTick 移入焦点到弹层根 view', () => {
      const body = openBody()
      expect(body, 'r7OpenCompare 必须存在').not.toBe('')
      expect(body, '须记录 document.activeElement 触发源').toContain('document.activeElement')
      expect(body, '须存入 r7TriggerEl').toContain('r7TriggerEl.value')
      expect(body, '须用 nextTick').toContain('nextTick')
      expect(body, '焦点目标须为 r7CompareRef.value').toContain('r7CompareRef.value')
      expect(body, '须调用 focus').toContain('focus(')
    })

    it(pg.name + ' G4 新增 closeR7Compare 关弹层后 focus 归还触发源', () => {
      const body = closeBody()
      expect(body, 'closeR7Compare 必须存在').not.toBe('')
      expect(body, '须关闭弹层').toContain('r7CompareOpen.value = false')
      expect(body, '须从 r7TriggerEl 取回触发源').toContain('r7TriggerEl.value')
      expect(body, '须 focus 归还').toContain('focus(')
    })

    it(pg.name + ' G5 三处内联关闭统一改调 closeR7Compare', () => {
      expect(tpl.split('r7CompareOpen = false').length - 1, '模板不得残留内联关闭').toBe(0)
      expect(tpl.split('closeR7Compare').length - 1, '遮罩/取消/完成至少 3 处引用 closeR7Compare').toBeGreaterThanOrEqual(3)
    })

    it(pg.name + ' G6 新增 trapR7CompareTab 复刻 R103 首尾循环', () => {
      const body = trapBody()
      expect(body, 'trapR7CompareTab 必须存在').not.toBe('')
      expect(body, '须以 r7CompareRef.value 取根节点').toContain('r7CompareRef.value')
      expect(body, '须查询可聚焦节点').toContain('querySelectorAll')
      expect(body, '须含 tabindex 选择器').toContain('[tabindex="0"]')
      expect(body, '须含 role=button 选择器').toContain('[role="button"]')
    })
  }

  it('I1 不变量：两步确认（open 只预览、confirm 才写入）', () => {
    for (const pg of PAGES) {
      const src = readSrc('pages/' + pg.name + '/' + pg.name + '.vue')
      const open = blockAfter(src, 'function r7OpenCompare')
      const confirm = blockAfter(src, 'function r7ConfirmApply')
      expect(open, pg.name + ' open 须设 r7CompareOpen=true').toContain('r7CompareOpen.value = true')
      expect(open, pg.name + ' open 不得写入').not.toContain(pg.write)
      expect(confirm, pg.name + ' confirm 须写入').toContain(pg.write)
    }
  })

  it('I2 不变量：写入失败保留原配置 + 请重试', () => {
    for (const pg of PAGES) {
      const src = readSrc('pages/' + pg.name + '/' + pg.name + '.vue')
      const confirm = blockAfter(src, 'function r7ConfirmApply')
      expect(confirm).toContain('r7Applying.value = true')
      expect(confirm).toContain('r7ApplyError.value')
      expect(confirm, pg.name + ' 须有请重试').toContain('重试')
      expect(confirm, pg.name + ' 须声明原配置未改动').toContain('未改动')
    }
  })

  it('I3 不变量：令牌三锚点 + 零 --r112-', () => {
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    for (const pg of PAGES) {
      const src = readSrc('pages/' + pg.name + '/' + pg.name + '.vue')
      expect(src + tokens, '业务源码不得新增 --r112-').not.toContain('--r112-')
    }
  })

  it('I4 不变量：R73 图例 / R103 时间线闭环不动', () => {
    const legend = tagBefore(battle, 'class="legend-overlay"', 'view')
    expect(legend).not.toBe('')
    expect(legend).toContain('role="dialog"')
    expect(legend).toContain('aria-modal="true"')
    expect(legend).toContain('@keydown.tab="trapLegendTab"')
    const timeline = tagBefore(battle, 'class="r29-timeline-overlay"', 'view')
    expect(timeline).not.toBe('')
    expect(timeline).toContain('role="dialog"')
    expect(timeline).toContain('@keydown.esc="closeReactionTimeline"')
    expect(timeline).toContain('@keydown.tab="trapTimelineTab"')
    expect(blockAfter(battle, 'function trapLegendTab'), 'trapLegendTab 须保留').not.toBe('')
    expect(blockAfter(battle, 'function trapTimelineTab'), 'trapTimelineTab 须保留').not.toBe('')
  })

  it('I5 不变量：r7ImpactApply 假守卫保留记录，语义检查由本契约补齐', () => {
    expect(oldGuard, '旧假守卫仍只查类名').toContain('.r7-compare-mask')
    expect(oldGuard, '旧守卫令牌标记保留').toContain('LYP-R7-IMPACT-APPLY-V1')
    const self = readSrc('styles/r112R7CompareKeyboard.contract.test.ts')
    expect(self, '本契约须补 role=dialog 语义').toContain('role="dialog"')
    expect(self, '本契约须补 aria-modal').toContain('aria-modal="true"')
    expect(self, '本契约须补 ESC').toContain('@keydown.esc')
    expect(self, '本契约须补 Tab').toContain('@keydown.tab')
  })

  it('I6 不变量：mask v-if 与内层 @click.stop 保留', () => {
    for (const pg of PAGES) {
      const src = readSrc('pages/' + pg.name + '/' + pg.name + '.vue')
      expect(tagBefore(src, 'class="r7-compare-mask"', 'view'), pg.name + ' mask v-if 须保留').toContain('v-if="r7CompareOpen"')
      expect(src, pg.name + ' 内层 @click.stop 须保留').toContain('@click.stop')
    }
  })
})
