import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const here = dirname(fileURLToPath(import.meta.url))
const root = join(here, '..')
const defenses = readFileSync(join(root, 'src/views/DefensesView.vue'), 'utf8')
const battles = readFileSync(join(root, 'src/views/BattlesView.vue'), 'utf8')
const tokens = readFileSync(join(root, 'src/styles/tokens.css'), 'utf8')

function funcBody(src: string, signature: string): string {
  const m = src.match(new RegExp('function ' + signature + '\\(\\)\\s*\\{[\\s\\S]*?\\n\\}'))
  return m ? m[0] : ''
}

function lineWith(src: string, needle: string): string {
  return src.split('\n').find(l => l.includes(needle)) ?? ''
}

describe('R94 DefensesView 空态 CTA 与 reason 一一对应契约', () => {
  it('it1 resetFilters 函数存在且含四个清零赋值 + page=1 + load()', () => {
    const body = funcBody(defenses, 'resetFilters')
    expect(body, '必须新增 function resetFilters()').not.toBe('')
    expect(body).toContain("keyword.value = ''")
    expect(body).toContain('ownerId.value = undefined')
    expect(body).toContain('shieldedFilter.value = undefined')
    expect(body).toContain('page.value = 1')
    expect(body).toContain('load()')
  })

  it('it2 空态 CTA 文案含「清空筛选」且 @click=resetFilters', () => {
    const cta = lineWith(defenses, 'empty-state-action')
    expect(cta, 'CTA 文案必须改为「清空筛选」').toContain('清空筛选')
    expect(cta, 'CTA 必须绑定 resetFilters').toContain('@click="resetFilters"')
    expect(cta, '不得保留旧文案「刷新防线」').not.toContain('刷新防线')
    expect(cta, '不得保留旧的 inline 清页刷新').not.toContain('((page = 1), load())')
  })

  it('it3 reason 含「清空筛选」且 CTA 兑现承诺（一一对应）', () => {
    const reason = lineWith(defenses, 'empty-state-reason')
    expect(reason, 'reason 必须保留「清空筛选」承诺').toContain('清空筛选')
    const cta = lineWith(defenses, 'empty-state-action')
    expect(cta, 'CTA 必须兑现「清空筛选」').toContain('清空筛选')
  })

  it('it4 loadSeq 竞态守卫与 fetchDefenses 参数不变量保留', () => {
    expect(defenses).toContain('let loadSeq = 0')
    expect((defenses.match(/(\+\+loadSeq|seq !== loadSeq)/g) ?? []).length).toBeGreaterThanOrEqual(3)
    expect((defenses.match(/seq === loadSeq/g) ?? []).length).toBeGreaterThanOrEqual(1)
    expect(defenses).toContain('fetchDefenses(PAGE_SIZE, (page.value - 1) * PAGE_SIZE')
  })

  it('it5 令牌三锚点保留 + DefensesView 零 --r94-（不变量）', () => {
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(defenses).not.toContain('--r94-')
  })

  it('it6 对照 BattlesView resetFilters 范式未回退（不变量）', () => {
    const body = funcBody(battles, 'resetFilters')
    expect(body, 'BattlesView 必须仍保留 resetFilters 范式').not.toBe('')
    expect(body).toContain('page.value = 1')
    expect(body).toContain('load()')
  })
})
