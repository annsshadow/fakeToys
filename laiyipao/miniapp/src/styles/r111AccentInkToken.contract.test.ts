import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const root = join(__dirname, '..')
const read = (p: string) => readFileSync(join(root, p), 'utf-8')

const tokens = read('styles/tokens.css')
const common = read('styles/common.css')
const r3Contract = read('styles/r3SecondaryPages.contract.test.ts')
const adminTokens = read('../../admin/src/styles/tokens.css')

function rootBlock(css: string): string {
  const m = css.match(/page,\s*:root\s*\{([\s\S]*?)\}/)
  return m ? m[1] : ''
}
function tokenValue(css: string, name: string): string | null {
  const i = css.indexOf(name + ':')
  if (i < 0) return null
  const rest = css.slice(i + name.length)
  const j = rest.indexOf(';')
  return j < 0 ? null : rest.slice(1, j).trim()
}
function ruleBlock(css: string, selector: string): string {
  const i = css.indexOf(selector)
  if (i < 0) return ''
  const open = css.indexOf('{', i)
  const close = css.indexOf('}', open)
  return open < 0 || close < 0 ? '' : css.slice(open, close)
}

describe('R111 --accent-ink 令牌真源缺失契约', () => {
  it('G1 tokens.css :root 色段必须真实定义 --accent-ink: #ff8659', () => {
    const rb = rootBlock(tokens)
    expect(rb, ':root 块必须可解析').not.toBe('')
    expect(rb, ':root 内必须定义 --accent-ink: #ff8659').toMatch(/--accent-ink\s*:\s*#ff8659\s*;/i)
  })

  it('G2 --accent-ink 必须与 --fire-ink 同值（双端同值口径）', () => {
    const rb = rootBlock(tokens)
    const fireInk = tokenValue(rb, '--fire-ink')
    const ink = tokenValue(rb, '--accent-ink')
    expect(fireInk, '--fire-ink 原定义必须为 #ff8659').toBe('#ff8659')
    expect(ink, '--accent-ink 必须先定义').not.toBeNull()
    expect(ink, '--accent-ink 必须与 --fire-ink 同值').toBe(fireInk)
  })

  it('守卫 现有 r3 契约只查 var(--accent-ink) 字样不查定义，本契约补定义检查', () => {
    expect(r3Contract).toContain('var(--accent-ink)')
    expect(r3Contract).not.toMatch(/--accent-ink\s*:\s*#/)
    expect(typeof rootBlock(tokens), '本契约已补令牌定义检查(G1/G2)').toBe('string')
  })

  it('不变量① common.css 三消费点 var(--accent-ink) 仍在', () => {
    for (const sel of ['.r3-row-value', '.r3-filter .r3-chip.active', '.r3-empty-action']) {
      expect(ruleBlock(common, sel), sel + ' 规则块必须消费 var(--accent-ink)').toContain('var(--accent-ink)')
    }
  })

  it('不变量② tokens.css 令牌三锚点不变', () => {
    expect(tokens).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/)
    expect(tokens).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/)
    expect(tokens).toMatch(/--lyp-r6-version:\s*lyp-tokens-r6/)
  })

  it('不变量③ --fire-ink: #ff8659 原定义不变且唯一', () => {
    const m = tokens.match(/--fire-ink\s*:\s*#ff8659\s*;/g)
    expect(m, '--fire-ink 原定义必须保持唯一一处').not.toBeNull()
    expect(m!.length).toBe(1)
  })

  it('不变量④ admin --el-fire-ink: #ff8659 不被改', () => {
    expect(adminTokens).toMatch(/--el-fire-ink:\s*#ff8659/)
  })

  it('不变量⑤ 业务样式零 --r111-', () => {
    expect(tokens).not.toContain('--r111-')
    expect(common).not.toContain('--r111-')
  })
})
