// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const read = (rel: string) => readFileSync(new URL(rel, import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1'), 'utf8')

describe('R455 PC AI 视觉渲染 4 项可达真缺陷修复', () => {
  it('BBSForum .ai-gradient-text 重复类提权使 color:transparent 生效且保留渐变裁剪', () => {
    const src = read('./BBSForum.vue')
    const m = src.match(/\.ai-gradient-text\.ai-gradient-text\s*\{([^}]*)\}/)
    expect(m, 'BBSForum 应有 .ai-gradient-text.ai-gradient-text 提权规则').not.toBeNull()
    const body = m![1]
    expect(body).toMatch(/background:\s*var\(--ai-gradient-text\)/)
    expect(body).toMatch(/background-clip:\s*text/)
    expect(body).toMatch(/color:\s*transparent/)
  })

  it('BamApp .ai-gradient-text 重复类提权使 color:transparent 生效且保留渐变裁剪', () => {
    const src = read('./BamApp.vue')
    const m = src.match(/\.ai-gradient-text\.ai-gradient-text\s*\{([^}]*)\}/)
    expect(m, 'BamApp 应有 .ai-gradient-text.ai-gradient-text 提权规则').not.toBeNull()
    const body = m![1]
    expect(body).toMatch(/background:\s*var\(--ai-gradient-text\)/)
    expect(body).toMatch(/background-clip:\s*text/)
    expect(body).toMatch(/color:\s*transparent/)
  })

  it('FindDesignerApp .ai-skeleton-line 用三色 linear-gradient 且保留 ai-shimmer', () => {
    const src = read('./FindDesignerApp.vue')
    const m = src.match(/\.ai-skeleton-line\s*\{([^}]*)\}/)
    expect(m).not.toBeNull()
    const body = m![1]
    expect(body).toMatch(/background:\s*linear-gradient\(/)
    expect(body).toMatch(/var\(--ai-skeleton-base\)\s*25%/)
    expect(body).toMatch(/var\(--ai-skeleton-shine\)\s*50%/)
    expect(body).toMatch(/var\(--ai-skeleton-base\)\s*75%/)
    expect(body).toMatch(/animation:\s*ai-shimmer/)
  })

  it('CalendarApp 无 blur() 包裹且至少 2 处直接 var(--ai-glass-blur)', () => {
    const src = read('./CalendarApp.vue')
    expect(src.match(/blur\(var\(--ai-glass-blur\)\)/g), '不应存在 blur(var()) 双重包裹').toBeNull()
    const direct = src.match(/backdrop-filter:\s*var\(--ai-glass-blur\)/g) || []
    expect(direct.length).toBeGreaterThanOrEqual(2)
  })

  it('MeetingApp 无 blur() 包裹且至少 2 处直接 var(--ai-glass-blur)', () => {
    const src = read('./MeetingApp.vue')
    expect(src.match(/blur\(var\(--ai-glass-blur\)\)/g), '不应存在 blur(var()) 双重包裹').toBeNull()
    const direct = src.match(/backdrop-filter:\s*var\(--ai-glass-blur\)/g) || []
    expect(direct.length).toBeGreaterThanOrEqual(2)
  })

  it('BBSForum .bbs-header 在 glass 覆盖规则之后有高特异性渐变提权', () => {
    const src = read('./BBSForum.vue')
    const glassIdx = src.indexOf('.ai-page .bbs-header{')
    expect(glassIdx, '应有 .ai-page .bbs-header glass 覆盖规则').toBeGreaterThan(-1)
    const after = src.slice(glassIdx)
    const m = after.match(/\.ai-page \.bbs-header\.bbs-header\s*\{([^}]*)\}/)
    expect(m, 'glass 规则之后应有 .ai-page .bbs-header.bbs-header 提权规则').not.toBeNull()
    expect(m![1]).toMatch(/background:\s*var\(--ai-gradient-surface\)/)
  })

  it('BBSForum h1 文案与字号零改', () => {
    const src = read('./BBSForum.vue')
    expect(src).toContain('>团队论坛</h1>')
    expect(src).toMatch(/\.header-left h1\s*\{[^}]*font-size:\s*20px/)
  })

  it('BamApp h1 文案与字号零改', () => {
    const src = read('./BamApp.vue')
    expect(src).toContain('>业务活动监控</h1>')
    expect(src).toMatch(/\.view-header h1\s*\{[^}]*font-size:\s*20px/)
  })
})
