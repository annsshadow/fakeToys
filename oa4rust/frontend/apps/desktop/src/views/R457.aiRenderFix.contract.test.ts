// @vitest-environment node
import { readFileSync } from 'node:fs'
import { describe, it, expect } from 'vitest'

function readSrc(rel: string) {
  return readFileSync(new URL(rel, import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1'), 'utf8')
}
function countOcc(hay: string, needle: string) { return hay.split(needle).length - 1 }
function ruleBody(src: string, selector: string) {
  const i = src.indexOf(selector)
  if (i < 0) return ''
  const s = src.indexOf('{', i)
  const e = src.indexOf('}', s)
  return s >= 0 && e > s ? src.slice(s + 1, e) : ''
}
function skelOk(src: string, selector: string) {
  const b = ruleBody(src, selector)
  return b.includes('linear-gradient') && b.includes('var(--ai-skeleton-shine)') && b.includes('background-size:200% 100%') && b.includes('animation:ai-shimmer')
}

describe('R457 AI 渲染修复契约：4 双重 blur + 4 页纯色骨架', () => {
  const org = readSrc('./OrgDirectoryApp.vue')
  const pw = readSrc('./ProcessWork.vue')
  const common = readSrc('./CommonApp.vue')
  const fm = readSrc('./FileManager.vue')
  const pwd = readSrc('./PortalWidgetDesignerApp.vue')
  const svc = readSrc('./ServiceInvokeDesignerApp.vue')

  it('OrgDirectoryApp 三处 backdrop-filter 去 blur 包裹且直接消费令牌', () => {
    expect(org.includes('backdrop-filter:blur(var(--ai-glass-blur))')).toBe(false)
    expect(countOcc(org, 'backdrop-filter: var(--ai-glass-blur)')).toBeGreaterThanOrEqual(3)
  })

  it('ProcessWork work-hero backdrop-filter 去 blur 包裹且直接消费令牌', () => {
    expect(pw.includes('backdrop-filter:blur(var(--ai-glass-blur))')).toBe(false)
    expect(countOcc(pw, 'backdrop-filter: var(--ai-glass-blur)')).toBeGreaterThanOrEqual(1)
  })

  it('CommonApp .skel 三色渐变 + size200% + shimmer', () => { expect(skelOk(common, '.skel{')).toBe(true) })
  it('FileManager .skeleton-row 三色渐变 + size200% + shimmer', () => { expect(skelOk(fm, '.skeleton-row{')).toBe(true) })
  it('FileManager .sk 三色渐变 + size200% + shimmer', () => { expect(skelOk(fm, '.sk{')).toBe(true) })
  it('PortalWidgetDesignerApp .skel 三色渐变 + size200% + shimmer', () => { expect(skelOk(pwd, '.skel{')).toBe(true) })
  it('ServiceInvokeDesignerApp .skel 三色渐变 + size200% + shimmer', () => { expect(skelOk(svc, '.skel{')).toBe(true) })

  it('令牌定义零回归：blur 与 shine 令牌仍存在', () => {
    const t = readSrc('../../../../packages/ui/src/theme/ai-tokens.css')
    expect(t.includes('--ai-glass-blur: blur(12px)')).toBe(true)
    expect(t.includes('--ai-skeleton-shine:')).toBe(true)
  })

  const gradPages = [
    './PortalDesigner.vue', './PortalDictDesignerApp.vue', './PortalPageDesignerApp.vue',
    './QueryDesigner.vue', './QueryExplorerApp.vue'
  ]
  for (const p of gradPages) {
    it('零回归 GRAD ' + p + ' 渐变骨架定义不动', () => {
      const s = readSrc(p)
      expect(s.includes('var(--ai-skeleton-shine)')).toBe(true)
      expect(s.includes('linear-gradient')).toBe(true)
      expect(s.includes('background-size:200% 100%')).toBe(true)
    })
  }
})
