// @vitest-environment node
import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'

const source = readFileSync(new URL('CalendarApp.vue', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1'), 'utf-8')

const styleBlocks = (src: string): string => {
  const re = /<style[^>]*>([\s\S]*?)<\/style>/g
  let out = ''
  let m: RegExpExecArray | null
  while ((m = re.exec(src)) !== null) out += '\n' + m[1]
  return out
}

const styleText = styleBlocks(source)

const ruleFor = (selector: string): string => {
  return styleText.split('\n').filter(l => l.includes(selector)).join('\n')
}

describe('R449 CalendarApp 新建日历表单文本令牌修复', () => {
  const cases = [
    ['.cal-create-label', '--text-secondary'],
    ['.cal-create-color', '--text-primary'],
    ['.cal-create-cancel', '--text-secondary'],
  ] as const

  for (const [selector, expected] of cases) {
    it(selector + ' 的 color 使用已定义令牌 ' + expected, () => {
      const rule = ruleFor(selector)
      expect(rule).not.toBe('')
      expect(rule).toMatch(new RegExp('color:\\s*var\\(' + expected + '\\)'))
    })

    it(selector + ' 不再引用未定义 ai-text-* 令牌', () => {
      const rule = ruleFor(selector)
      expect(rule).not.toBe('')
      expect(rule).not.toMatch(/--ai-text-(primary|secondary)/)
    })
  }

  it('scoped 块三处令牌全部改为已定义令牌且无 ai-text 残留', () => {
    expect(styleText).toContain('color: var(--text-secondary)')
    expect(styleText).toContain('color: var(--text-primary)')
    expect(styleText).not.toMatch(/--ai-text-(primary|secondary)/)
  })
})
