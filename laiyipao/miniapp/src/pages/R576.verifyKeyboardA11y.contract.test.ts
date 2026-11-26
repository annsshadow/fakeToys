import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

const ROOT = resolve(__dirname, '../..')
const src = readFileSync(resolve(ROOT, 'src/pages/verify/verify.vue'), 'utf8')

/** 从起始锚点行向下扫到开标签结束(trimEnd().endsWith('>'))，返回该开标签块 */
function blockFrom(anchor: string): string {
  const lines = src.split('\n')
  const i = lines.findIndex((l) => l.includes(anchor))
  if (i < 0) return ''
  let out = ''
  for (let j = i; j < Math.min(lines.length, i + 20); j++) {
    out += lines[j] + '\n'
    if (lines[j].trimEnd().endsWith('>')) break
  }
  return out
}

/** 单行开标签所在行(用于 doReplay 单行修复) */
function lineWith(anchor: string): string {
  return src.split('\n').find((l) => l.includes(anchor)) || ''
}

function hasA11y(block: string, ariaExpect: RegExp, handler: string): boolean {
  return /role="button"/.test(block)
    && /tabindex="0"/.test(block)
    && ariaExpect.test(block)
    && new RegExp('@keydown\\.enter="' + handler + '"').test(block)
    && new RegExp('@keydown\\.space\\.prevent="' + handler + '"').test(block)
}

describe('R576 verify.vue 4类裸@click伪按钮键盘可达契约 (WCAG SC2.1.1/4.1.2)', () => {
  it('核心1 取复现信息按钮: role/tabindex/aria-label/@keydown复用fetchReplay', () => {
    const b = blockFrom('style="width: 180rpx; height: 72rpx"')
    expect(hasA11y(b, /:aria-label="fetching \? '获取中' : '取复现信息'"/, 'fetchReplay')).toBe(true)
    expect(b).toContain('@click="fetchReplay"')
  })

  it('核心2 本地重放按钮(单行): role/tabindex/aria-label/@keydown复用doReplay', () => {
    const l = lineWith('style="margin-top: 16rpx"')
    expect(hasA11y(l, /aria-label="本地重放并提交验真"/, 'doReplay')).toBe(true)
    expect(l).toContain('@click="doReplay"')
  })

  it('核心3 提交伪造哈希按钮: role/tabindex/aria-label/@keydown复用doFalsify', () => {
    const b = blockFrom('style="margin-top: 12rpx"')
    expect(hasA11y(b, /:aria-label="falsifying \? '提交中' : '提交伪造哈希'"/, 'doFalsify')).toBe(true)
    expect(b).toContain('@click="doFalsify"')
  })

  it('核心4 验真记录回填行: role/tabindex/动态aria-label/@keydown复用refillBattleId', () => {
    const b = blockFrom('class="kv vh-row tap"')
    expect(hasA11y(b, /:aria-label="'回填战报 #' \+ h\.id/, 'refillBattleId\\(h\\.id\\)')).toBe(true)
    expect(b).toContain('@click="refillBattleId(h.id)"')
    const a11y = src.match(/role="button"/g) || []
    expect(a11y.length).toBe(4)
  })

  it('守卫5 业务handler函数体零改', () => {
    expect(src).toContain('async function fetchReplay() {')
    expect(src).toContain('errMsg.value = ')
    expect(src).toContain('function doReplay() {')
    expect(src).toContain('function doFalsify() {')
    expect(src).toContain('function refillBattleId(id: number) {')
    expect(src).toContain('battleId.value = String(id)')
    expect(src).toContain('await api.getReplay(id)')
    expect(src).toContain('await api.verifyReplay(id, hash)')
  })

  it('守卫6 数据源/样式/main.ts零改,无全局增强器', () => {
    expect(src).toContain("import * as api from '@/api/client'")
    expect(src).toContain("import { useGameStore } from '@/store/game'")
    expect(src).toContain('<style scoped>')
    expect(src).toContain('.vh-row { cursor: pointer; }')
    expect(src).not.toMatch(/MutationObserver/)
    expect(src).not.toMatch(/document\.querySelectorAll/)
  })

  it('守卫7 已修页正面对照零改+无tablist语义', () => {
    expect(src).not.toMatch(/role="tab"/)
    expect(src).not.toMatch(/role="tablist"/)
    expect(src).not.toMatch(/aria-selected/)
    // 全页仍仅4处@click(修复只追加keydown,不删原click)
    const clicks = src.match(/@click="/g) || []
    expect(clicks.length).toBe(4)
  })

  it('守卫8 无原生button标签(避免与已修页混淆)', () => {
    expect(src).not.toMatch(/<button/)
  })
})
