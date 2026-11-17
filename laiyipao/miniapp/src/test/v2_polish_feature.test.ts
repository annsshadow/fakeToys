// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * 第六批「重新全面优化」结构守卫（2026-11-17）。
 *
 * 这些改动删掉后编译照过、行为测试也可能全绿（少一段图例文案、
 * 少一个关卡名、少一个数量角标都不会报错）——用源码扫描固化，
 * 保证 100 轮打磨的成果不会被后续改动悄悄抹掉。
 *
 * R1/R2/R3 由对应组件测试 / 渲染测试守卫，这里覆盖 R4–R7 的呈现标记。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const read = (rel: string) => readFileSync(join(__dirname, rel), 'utf-8')

describe('第六批重新全面优化结构守卫', () => {
  it('R4 战斗图例必须解释敌人视觉语言', () => {
    const src = read('../pages/battle/battle.vue')
    expect(src).toContain('敌人外观（怎么分辨敌种）')
    expect(src).toContain('本体色 + 五官徽记')
    expect(src).toContain('三角=飞行')
  })

  it('R5 战斗 HUD 必须常驻当前关卡名', () => {
    const src = read('../pages/battle/battle.vue')
    expect(src).toContain('levelTitle')
    expect(src).toContain('class="hp-level"')
    expect(src).toContain('第 ${lv.id} 关 · ${lv.name}')
  })

  it('R6 离线收益按钮必须区分「领取 / 已领完」', () => {
    const src = read('../pages/signin/signin.vue')
    expect(src).toContain("offlineReward.coin > 0 ? '领取' : '已领完'")
  })

  it('R7 选关章节 chip 必须显示关卡数', () => {
    const src = read('../pages/stage/stage.vue')
    expect(src).toContain('chapterLevelCount(ch.id)')
    expect(src).toContain('chapter-count')
  })

  it('R8–R27 全局可用性标记齐全', () => {
    const css = read('../styles/common.css')
    for (const marker of [
      'R8 主按钮按压位移',
      'R9 禁用按钮明确不可点',
      'R10 卡片标题轻微字距',
      'R11/R12 次级文字行高',
      'R13 图鉴描述行高放宽',
      'R14 统计小块加浅描边',
      'R15 chip 加浅描边',
      'R16 文字链接下划线偏移',
      'R17 空态块留白',
      'R18 数值统一等宽数字',
      'R19 键值行分隔线',
      'R20 输入控件状态过渡',
      'R21 结果计数字距',
      'R22 清除搜索按钮按压反馈',
      'R23 筛选 chip 描边',
      'R24 页签透明描边占位',
      'R25 小按钮最小点击区',
      'R26 省略号文字限制最大宽度',
      'R27 减少动效时关闭新增的按压位移',
    ]) {
      expect(css, marker).toContain(marker)
    }
    // 真实样式声明也在（防止只留注释）
    expect(css).toContain('.btn-primary:active')
    expect(css).toContain('.btn-disabled')
    expect(css).toContain('.kv:last-child')
    expect(css).toContain('text-underline-offset: 4rpx')
  })

  it('R28–R34 战斗视觉深化标记齐全', () => {
    const css = read('../render/canvas.ts')
    for (const marker of [
      'R28 地面投影',
      'R29 飞行高度提示',
      'R30 BOSS 警示光晕',
      'R31 远程单位准星底纹',
      'R32 冰系霜纹',
      'R33 电系电弧',
      'R34 装甲板',
    ]) {
      expect(css, marker).toContain(marker)
    }
    expect(css).toContain('paintEnemyDetail')
    expect(css).toContain('paintEnemyTrait')
  })

  it('R96 用户点名缺陷修复标记齐全（洗练反馈/空态引导/钥匙/取信息/排名/记录空态）', () => {
    expect(read('../pages/bag/bag.vue')).toContain('washingId === g.id')
    expect(read('../pages/battles/battles.vue')).toContain('function goStage()')
    expect(read('../pages/shop/shop.vue')).toContain('wallet.keys ?? 0')
    expect(read('../pages/verify/verify.vue')).toContain('fetching.value = true')
    expect(read('../pages/rank/rank.vue')).toContain('排名暂不可用')
    expect(read('../pages/me/me.vue')).toContain('noRecordsYet')
  })

  it('R97 图鉴与核心页信息缺口修复标记齐全', () => {
    expect(read('../pages/announcements/announcements.vue')).toContain('loadFailed')
    expect(read('../pages/gems/gems.vue')).toContain('bestQualityOf')
    expect(read('../pages/skills/skills.vue')).toContain('需 Lv.')
    expect(read('../pages/equipment/equipment.vue')).toContain('（T{{ e.tier }}）')
    expect(read('../pages/skins/skins.vue')).toContain('countByRarity')
    expect(read('../pages/enemies/enemies.vue')).toContain('CATEGORY_FILTERS')
    expect(read('../pages/reactions/reactions.vue')).toContain('当前不可达')
    expect(read('../pages/index/index.vue')).toContain('res-glyph')
  })

  it('R98 表单/反馈/选关/战斗标记齐全', () => {
    const login = read('../pages/login/login.vue')
    expect(login).toContain('input-error')
    expect(login).toContain('usernamePrefilled')
    expect(read('../pages/signin/signin.vue')).toContain('claimableCount')
    expect(read('../pages/stage/stage.vue')).toContain('starsLoaded')
    expect(read('../pages/battle/battle.vue')).toContain('牌堆剩 {{ deck.size }} 张')
    expect(read('../pages/battle/battle.vue')).toContain('helpSeen')
  })

  it('R99 全端交互反馈 CSS 标记齐全', () => {
    const css = read('../styles/common.css')
    for (const marker of [
      'R73 英雄卡内发光', 'R74 公告行 hover', 'R75 任务行 hover',
      'R76 技能行 hover', 'R77 专精节点 hover', 'R78 榜单行 hover',
      'R79 挑战历史行 hover', 'R80 资源胶囊 hover', 'R81 图鉴行 hover',
      'R82 手牌 hover', 'R83 倍速 chip hover', 'R84 章节 chip hover',
      'R85 关卡格 hover', 'R86 开始按钮呼吸光', 'R87 chip.tap 按压反馈统一',
      'R88 小按钮按压反馈', 'R89 输入框 hover 边框', 'R90 搜索空态留白',
      'R91 空态块渐变底', 'R92 图例行 hover', 'R93 图例按钮首次未读脉冲',
      'R94 我的排名卡描边', 'R95 减少动效时关闭本批新增位移',
    ]) {
      expect(css, marker).toContain(marker)
    }
  })

  it('R100 第六批全量关键标记扫描（任一被删即红）', () => {
    const markers: Array<[string, string]> = [
      ['../pages/signin/signin.vue', 'offlineRewardStatus'],
      ['../pages/stage/stage.vue', 'activeLevelId'],
      ['../render/canvas.ts', 'ENEMY_TYPE_VISUAL'],
      ['../render/canvas.ts', 'paintEnemyTrait'],
      ['../styles/common.css', '.btn-primary:active'],
      ['../styles/common.css', '.hand-card:hover'],
      ['../pages/bag/bag.vue', 'washingId === g.id'],
      ['../pages/battles/battles.vue', 'function goStage()'],
      ['../pages/shop/shop.vue', 'affordHint'],
      ['../pages/verify/verify.vue', 'falsifying'],
      ['../pages/mastery/mastery.vue', 'tab-invest'],
      ['../pages/login/login.vue', 'usernamePrefilled'],
      ['../test/v2_polish_feature.test.ts', 'R100 第六批全量关键标记扫描'],
    ]
    for (const [file, marker] of markers) {
      expect(read(file), `${file} :: ${marker}`).toContain(marker)
    }
  })

  it('R1–R3 的关键实现标记仍齐全（防止被迁移/清理误删）', () => {
    const signin = read('../pages/signin/signin.vue')
    const me = read('../pages/me/me.vue')
    const stage = read('../pages/stage/stage.vue')
    const canvas = read('../render/canvas.ts')
    expect(signin).toContain('offlineRewardStatus')
    expect(me).not.toContain('claimOffline()')
    expect(stage).toContain('activeLevelId')
    expect(stage).toContain("pageScrollTo({ selector: '#level-detail'")
    expect(canvas).toContain('ENEMY_TYPE_VISUAL')
  })
})
