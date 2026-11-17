// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * 第七批「实质优化 100 轮」其它页面功能守卫。
 * 只认完整功能（雷达图 / 矩阵 / 对比 / 图表），不认 hover/描边/计数角标式小打磨。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const read = (rel: string) => readFileSync(join(__dirname, rel), 'utf-8')

describe('第七批 其它页面实质功能守卫', () => {
  it('G61 首页构筑五维雷达图', () => {
    const src = read('../pages/index/index.vue')
    expect(src).toContain('radar-svg')
    expect(src).toContain('radarPolygonActual')
    expect(src).toContain('radar-ring')
    expect(src).toContain('radarAxisEnds')
  })

  it('G73 敌人抗性雷达图', () => {
    const src = read('../pages/enemies/enemies.vue')
    expect(src).toContain('enemyResistPoints')
    expect(src).toContain('resistScale')
    expect(src).toContain('expandedEnemy')
    expect(src).toContain('rr-shape')
  })

  it('G74 元素反应覆盖矩阵（5×5）', () => {
    const src = read('../pages/reactions/reactions.vue')
    expect(src).toContain('reactionMatrix')
    expect(src).toContain('lookupReaction')
    expect(src).toContain('matrix-cell')
    expect(src).toContain('元素反应覆盖矩阵')
  })

  it('G91 战斗评级徽章 + G92 下次目标', () => {
    const src = read('../pages/battle/battle.vue')
    expect(src).toContain('battleGrade')
    expect(src).toContain('grade-badge')
    expect(src).toContain('nextGoal')
    expect(src).toContain('下次目标')
  })

  it('G98 技能图鉴合成技能专区（补齐图鉴最大内容缺口）', () => {
    const src = read('../pages/skills/skills.vue')
    expect(src).toContain('compositeSkills')
    expect(src).toContain('composite_skills')
    expect(src).toContain('合成技能')
    expect(src).toContain('materialA')
  })

  it('G99 验真冻结构筑分区快照', () => {
    const src = read('../pages/verify/verify.vue')
    expect(src).toContain('buildSnapshotCard')
    expect(src).toContain('build-snapshot')
    expect(src).toContain('主动槽位')
  })

  it('G100 已佩戴宝石词条总加成', () => {
    const src = read('../pages/bag/bag.vue')
    expect(src).toContain('equippedAffixTotal')
    expect(src).toContain('gat-chip')
    expect(src).toContain('已佩戴词条总加成')
  })

  it('G88 战报星级分布', () => {
    const src = read('../pages/battles/battles.vue')
    expect(src).toContain('starDistribution')
    expect(src).toContain('sd-fill')
    expect(src).toContain('星级分布')
  })

  it('G89 挑战战绩统计', () => {
    const src = read('../pages/me/me.vue')
    expect(src).toContain('challengeStats')
    expect(src).toContain('cs-dot')
    expect(src).toContain('挑战胜负统计')
  })

  it('G90 皮肤被动机制分类', () => {
    const src = read('../pages/skins/skins.vue')
    expect(src).toContain('passiveGroups')
    expect(src).toContain('passive-chip')
    expect(src).toContain('被动机制分类')
  })

  it('G85 本章难度曲线', () => {
    const src = read('../pages/stage/stage.vue')
    expect(src).toContain('difficultyCurve')
    expect(src).toContain('diff-line')
    expect(src).toContain('本章难度曲线')
  })

  it('G86 公告时间筛选', () => {
    const src = read('../pages/announcements/announcements.vue')
    expect(src).toContain('annFilters')
    expect(src).toContain('filteredItems')
    expect(src).toContain('近 7 天')
  })

  it('G87 首页闯关进度环', () => {
    const src = read('../pages/index/index.vue')
    expect(src).toContain('stageRing')
    expect(src).toContain('stage-ring-label')
    expect(src).toContain('闯关进度')
  })

  it('G82 商城今日产出总览', () => {
    const src = read('../pages/shop/shop.vue')
    expect(src).toContain('todaySupply')
    expect(src).toContain('supply-chip')
    expect(src).toContain('今日还可获取')
  })

  it('G83 关卡胜率分布', () => {
    const src = read('../pages/battles/battles.vue')
    expect(src).toContain('winRateByLevel')
    expect(src).toContain('wr-fill')
    expect(src).toContain('关卡胜率')
  })

  it('G84 消费统计', () => {
    const src = read('../pages/me/me.vue')
    expect(src).toContain('spendSummary')
    expect(src).toContain('spend-chip')
    expect(src).toContain('消费统计摘要')
  })

  it('G79 累计星级进度环', () => {
    const src = read('../pages/me/me.vue')
    expect(src).toContain('starRing')
    expect(src).toContain('sr-fill')
    expect(src).toContain('累计星级进度')
  })

  it('G80 宝石词条覆盖统计', () => {
    const src = read('../pages/gems/gems.vue')
    expect(src).toContain('affixCoverage')
    expect(src).toContain('affix-chip')
    expect(src).toContain('affixName')
  })

  it('G81 装备各部位最强', () => {
    const src = read('../pages/equipment/equipment.vue')
    expect(src).toContain('bestPerSlot')
    expect(src).toContain('bs-row')
    expect(src).toContain('各部位最强')
  })

  it('G76 榜单分数分布柱状图', () => {
    const src = read('../pages/rank/rank.vue')
    expect(src).toContain('scoreDistribution')
    expect(src).toContain('dist-bar')
    expect(src).toContain('分数分布')
  })

  it('G71 专精投入总览', () => {
    const src = read('../pages/mastery/mastery.vue')
    expect(src).toContain('familyOverview')
    expect(src).toContain('totalInvested')
    expect(src).toContain('fo-fill')
  })

  it('G78 元素使用频次构成条（诚实标注非伤害量）', () => {
    const src = read('../pages/battles/battles.vue')
    expect(src).toContain('elementMix')
    expect(src).toContain('mix-bar')
    expect(src).toContain('使用频次构成')
  })

  it('G66 技能元素×类型覆盖矩阵', () => {
    const src = read('../pages/skills/skills.vue')
    expect(src).toContain('skillMatrix')
    expect(src).toContain('kindCols')
    expect(src).toContain('sm-cell')
  })

  it('G63 选关全章节星级总览', () => {
    const src = read('../pages/stage/stage.vue')
    expect(src).toContain('chapterOverview')
    expect(src).toContain('allStarCap')
    expect(src).toContain('overview-fill')
    expect(src).toContain('全局进度')
  })

  it('G75 皮肤稀有度分布环图', () => {
    const src = read('../pages/skins/skins.vue')
    expect(src).toContain('rarityDonut')
    expect(src).toContain('stroke-dasharray')
    expect(src).toContain('donut-seg')
  })

  it('G65 背包装备换装建议', () => {
    const src = read('../pages/bag/bag.vue')
    expect(src).toContain('gearSuggestions')
    expect(src).toContain('GearSuggestion')
    expect(src).toContain('换装建议')
    expect(src).toContain('gear-gain')
  })

  it('G70 战绩趋势折线', () => {
    const src = read('../pages/battles/battles.vue')
    expect(src).toContain('const trend = computed')
    expect(src).toContain('trend-line')
    expect(src).toContain('trend-win')
    expect(src).toContain('战绩趋势')
  })
})
