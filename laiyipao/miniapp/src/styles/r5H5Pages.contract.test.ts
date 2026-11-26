import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const root = join(__dirname, '..')
const read = (p: string) => readFileSync(join(root, p), 'utf-8')

const HEAVY = {
  announcements: {
    ai: 'r5AiAnn', cta: 'r5GoBattles',
    source: /items.value|groupedByMonth.value/,
    structs: ['r5-section', 'r3-kpi', 'r3-filter', 'r3-data-grid', 'r3-empty'],
    basis: '公告列表 · 近 30 天',
    kicker: '公告与版本',
    empty: '近 30 天没有公告，去看看战报',
  },
  signin: {
    ai: 'r5AiSignin', cta: 'r5GoTask',
    source: /claimedCount.value|tasks.value|offlineReward.value|calendarDayCount.value/,
    structs: ['r5-section', 'r3-kpi', 'r3-data-grid', 'r3-empty'],
    basis: '签到记录 · 本周期',
    kicker: '每日签到',
    empty: '今日任务已领完，明天再来',
  },
  stage: {
    ai: 'r5AiStage', cta: 'r5GoLevel',
    source: /difficultyCurve.value|terrainIndex.value|chapterStars.value|activeLevelId.value/,
    structs: ['r5-section', 'r3-kpi', 'r3-data-grid', 'r3-empty'],
    basis: '关卡曲线/地形/星数 · 当前存档',
    kicker: '作战推进',
    empty: '章节数据同步中',
  },
} as const

type HeavyPage = keyof typeof HEAVY

const tpl = (src: string) => src.slice(0, src.indexOf('</template>'))
const aiBlock = (src: string, name: string) => {
  const i = src.indexOf('const ' + name + ' = computed')
  expect(i, '缺少 computed ' + name).toBeGreaterThan(-1)
  return src.slice(i, i + 1800)
}

describe('R5 H5 五页视觉与可见 AI 契约', () => {
  it('令牌双锚点保留、R5 只复用既有组件且有具名轻量层', () => {
    const tokens = read('styles/tokens.css')
    expect(tokens).toMatch(/--lyp-token-version:\s*lyp-tokens-v1/)
    expect(tokens).toMatch(/--lyp-r3-version:\s*lyp-tokens-v3-r3/)
    expect(tokens).not.toMatch(/--r5-/)
    expect(tokens).not.toMatch(/lyp-tokens-v4/)
    const css = read('styles/common.css')
    expect(css).toMatch(/LYP-R5-H5-V1/)
    expect(css).toContain('.r5-section')
    expect(css).toContain('.r5-ai-note')
    expect(css).toContain('.r3-ai')
    expect(css).toContain('.r3-kpi')
    expect(css).toContain('.r3-data-grid')
    expect(css).toContain('.r3-empty')
    expect(css, '轻量 AI 行必须消费 AI 令牌').toMatch(/\.r5-ai-note[\s\S]{0,500}--ai-accent/)
    expect(css, '轻量 AI 行文字必须走 ink 阶').toMatch(/\.r5-ai-note[\s\S]{0,500}--ai-ink/)
    expect(css, 'R5 入场动效必须可关闭').toMatch(/prefers-reduced-motion[\s\S]{0,300}\.r5-ai-note[\s\S]{0,120}animation:\s*none/)
  })

  for (const page of Object.keys(HEAVY) as HeavyPage[]) {
    const cfg = HEAVY[page]

    it(page + ' 具备 R5 指定结构、可见 AI 徽章与可行动 CTA', () => {
      const src = read('pages/' + page + '/' + page + '.vue')
      const markup = tpl(src)
      for (const cls of cfg.structs) expect(markup, page + ' 缺少 ' + cls).toContain(cls)
      expect(markup, page + ' 缺眉题').toContain(cfg.kicker)
      expect(markup, page + ' AI 徽章必须是可见汉字').toMatch(/<text class="r3-ai-badge">AI<\/text>/)
      expect(markup, page + ' 必须复用 r3 AI 卡').toContain('r3-ai')
      expect(markup, page + ' 必须标注规则引擎').toContain('规则引擎')
      expect(markup, page + ' 必须显示依据').toContain('依据：' + cfg.basis + ' · 更新于本次进入')
      expect(markup, page + ' AI 结论必须绑定 computed').toContain(cfg.ai + '.conclusion')
      expect(markup, page + ' CTA 必须可点击').toContain('@click="' + cfg.cta + '()"')
      expect(markup, page + ' 智能空态必须有下一步动作').toContain(cfg.empty)
      expect(markup, page + ' 空态四段槽位').toMatch(/r3-empty-slot[\s\S]{0,400}r3-empty-title[\s\S]{0,400}r3-empty-reason[\s\S]{0,400}r3-empty-action/)
    })

    it(page + ' 的 AI 结论来自真实数据且无低透明度伪智能', () => {
      const src = read('pages/' + page + '/' + page + '.vue')
      const block = aiBlock(src, cfg.ai)
      expect(block, page + ' 必须引用页面真实字段').toMatch(cfg.source)
      expect(block, page + ' 低透明度网格不算 AI').not.toMatch(/0?\.022|0?\.06\b/)
      expect(block, page + ' 不得编造置信度').not.toMatch(/置信度|GPT|大模型|实时学习/)
      expect(src, page + ' CTA 函数缺失').toContain('function ' + cfg.cta + '(')
    })
  }

  it('login 使用不打断提交的轻量 AI note，不新增输入或按钮', () => {
    const src = read('pages/login/login.vue')
    const markup = tpl(src)
    expect(markup).toContain('section-kicker r5-section')
    expect(markup).toContain('欢迎回来')
    expect(markup).toMatch(/<view[^>]*class="[^"]*r5-ai-note[^"]*"[^>]*role="note"/)
    expect(markup).toContain('<text class="r3-ai-badge">AI</text>')
    expect(markup).toContain('r5AiLogin.conclusion')
    expect(markup).toContain('依据：本机缓存 · 上次登录 · 更新于本次进入')
    expect(markup).toContain('登录')
    expect(markup, '登录按钮仍是唯一主操作').toMatch(/@click="submit"/)
    const noteStart = markup.indexOf('class="r5-ai-note')
    const note = markup.slice(noteStart, markup.indexOf('</view>', noteStart))
    expect(note, 'AI note 不得新增可点击按钮').not.toMatch(/@click=|<input|<button/)
    expect(src).toContain('const r5AiLogin = computed')
    const block = aiBlock(src, 'r5AiLogin')
    expect(block).toMatch(/usernamePrefilled.value|loadLastUsername/)
    expect(block).not.toMatch(/0?\.022|0?\.06\b|置信度|GPT|大模型/)
    expect(src).toContain('loadLastUsername()')
  })

  it('verify 使用轻量 AI 复核行，只复用既有重放按钮且不插入步骤', () => {
    const src = read('pages/verify/verify.vue')
    const markup = tpl(src)
    expect(markup).toContain('section-kicker r5-section')
    expect(markup).toContain('战斗验真')
    expect(markup).toMatch(/<view[^>]*class="[^"]*r5-ai-note[^"]*"[^>]*role="note"/)
    expect(markup).toContain('<text class="r3-ai-badge">AI</text>')
    expect(markup).toContain('r5AiVerify.conclusion')
    expect(markup).toContain('依据：复现信息 · 本次验真 · 更新于本次进入')
    expect(markup, '必须复用本地重放').toContain('@click="r5FocusReplay"')
    expect(markup, '验真后可去战报对照').toContain('@click="r5GoBattles"')
    expect(markup, '原步骤条不能被替换为 AI 步骤').toContain('class="steps"')
    expect(markup).toContain('@click="doReplay"')
    expect(src).toContain('const r5AiVerify = computed')
    const block = aiBlock(src, 'r5AiVerify')
    expect(block).toMatch(/replayInfo.value|verifyHistory.value|result.value/)
    expect(block).not.toMatch(/0?\.022|0?\.06\b|置信度|GPT|大模型/)
    expect(src).toMatch(/function r5FocusReplay\(/)
    expect(src).toMatch(/function r5GoBattles\(/)
  })

  it('首页反应说明与 damage.ts 一致，不宣称攻击力无关', () => {
    const markup = tpl(read('pages/index/index.vue'))
    expect(markup, '不得宣称反应伤害与攻击力无关').not.toMatch(/反应伤害与面板攻击力无关/)
    expect(markup, '必须承认攻击力有贡献').toMatch(/反应伤害[\s\S]{0,80}攻击力[\s\S]{0,40}贡献/)
    expect(markup, '必须给出受限口径').toMatch(/30%|300‰|30‰/)
    expect(markup, '必须标注数据来源').toMatch(/damage\.ts|实测为准/)
  })

  it('五页共同的 AI 反模式与既有数据入口守卫', () => {
    for (const p of ['announcements', 'login', 'signin', 'stage', 'verify']) {
      const markup = tpl(read('pages/' + p + '/' + p + '.vue'))
      expect(markup, p + ' 禁止假思考动画').not.toMatch(/AI 思考中|AI思考中|思考中\.\.\./)
      expect(markup, p + ' 禁止裸暂无数据').not.toMatch(/>\s*暂无数据\s*</)
    }
    expect(read('pages/announcements/announcements.vue')).toContain('fetchAnnouncements(20)')
    expect(read('pages/signin/signin.vue')).toContain('switchScope(s.key)')
    expect(read('pages/stage/stage.vue')).toContain('switchChapter(ch.id)')
    expect(read('pages/verify/verify.vue')).toContain('fetchReplay()')
  })
})
