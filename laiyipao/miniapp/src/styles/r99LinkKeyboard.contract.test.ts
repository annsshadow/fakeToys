// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R99 可点 text.link 键盘可达性契约（test-first RED）。
 * 真实 fs.readFileSync 读 laiyipao/miniapp/src/pages 下 4 个 .vue，禁止内嵌实现副本。
 * 缺口：19 处可点 text.link 缺 role="button" / tabindex="0" / @keydown.enter，
 * 视觉可点（common.css .link 主色 + >=56rpx 点击盒）但键盘 Tab 不可达（WCAG 2.1.1）。
 * 范式：R60 role=button 四件套；R95 me:11；R96 裸 view CTA。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'

const dir = dirname(fileURLToPath(import.meta.url))
const srcRoot = join(dir, '..')

function read(rel: string): string {
  return readFileSync(join(srcRoot, rel), 'utf8')
}

// 稳健定位：以唯一 @click 表达式反查所在 <text> 开标签块（不依赖行号，容忍上游插行漂移）
function linkBlock(src: string, click: string): string | null {
  const ci = src.indexOf(click)
  if (ci < 0) return null
  const start = src.lastIndexOf('<text', ci)
  if (start < 0 || ci - start > 500) return null
  const end = src.indexOf('>', ci)
  if (end < 0 || end - ci > 200) return null
  return src.slice(start, end + 1)
}

type Case = { name: string; file: string; handler: string }
const CASES: Case[] = [
  { name: 'announcements 清除搜索', file: 'pages/announcements/announcements.vue', handler: "annKeyword = ''" },
  { name: 'bag 熔炼', file: 'pages/bag/bag.vue', handler: 'doSynthesize(r)' },
  { name: 'bag 装备', file: 'pages/bag/bag.vue', handler: 'toggleEquip(s.id)' },
  { name: 'bag 去商城买一颗', file: 'pages/bag/bag.vue', handler: 'goShop' },
  { name: 'bag 保存当前构筑', file: 'pages/bag/bag.vue', handler: 'doSavePreset' },
  { name: 'bag 点这里填名字', file: 'pages/bag/bag.vue', handler: 'focusPreset' },
  { name: 'me 复制', file: 'pages/me/me.vue', handler: 'copyUserId' },
  { name: 'me 改名', file: 'pages/me/me.vue', handler: 'startEditNickname' },
  { name: 'me 验真', file: 'pages/me/me.vue', handler: 'goVerify(b)' },
  { name: 'me 详情', file: 'pages/me/me.vue', handler: 'toggleBattleDetail(b)' },
  { name: 'bag 查看全部配方', file: 'pages/bag/bag.vue', handler: 'showAllRecipes = !showAllRecipes' },
  { name: 'bag 升级', file: 'pages/bag/bag.vue', handler: 'doUpgrade(s.id)' },
  { name: 'bag 佩戴宝石', file: 'pages/bag/bag.vue', handler: 'doEquipGem(g)' },
  { name: 'bag 卸下宝石', file: 'pages/bag/bag.vue', handler: 'doUnequipGem(g)' },
  { name: 'bag 洗练宝石', file: 'pages/bag/bag.vue', handler: 'doWash(g)' },
  { name: 'bag 应用预设', file: 'pages/bag/bag.vue', handler: 'doApplyPreset(p)' },
  { name: 'bag 设为当前预设', file: 'pages/bag/bag.vue', handler: 'doSetCurrent(p)' },
  { name: 'bag 删除预设', file: 'pages/bag/bag.vue', handler: 'doDeletePreset(p)' },
  { name: 'index 空维补导', file: 'pages/index/index.vue', handler: 'go(d.hint)' },
]

const SRC: Record<string, string> = {}
for (const c of CASES) if (!SRC[c.file]) SRC[c.file] = read(c.file)

describe('R99 可点 link 键盘可达性契约', () => {
  // 缺口断言：19 处 RED 必挂，每处逐一校验三属性 + 同名 handler
  for (const c of CASES) {
    it(c.name + ' 须 role=button + tabindex=0 + @keydown.enter 同名 handler', () => {
      const click = '@click="' + c.handler + '"'
      const block = linkBlock(SRC[c.file], click)
      expect(block, '必须能定位 ' + c.file + ' 的 ' + click).not.toBeNull()
      expect(block!, '须补 role="button"').toContain('role="button"')
      expect(block!, '须补 tabindex="0"').toContain('tabindex="0"')
      expect(block!, '须补 @keydown.enter="' + c.handler + '"（handler 与 @click 同一表达式）')
        .toContain('@keydown.enter="' + c.handler + '"')
    })
  }

  it('19 处原 @click 入口全部保留（不变量）', () => {
    for (const c of CASES) {
      expect(SRC[c.file], c.name + ' 原 @click="' + c.handler + '" 须保留')
        .toContain('@click="' + c.handler + '"')
    }
  })

  it('令牌三锚点不动且无 --r99-（不变量）', () => {
    const tokens = read('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toMatch(/--r99-/)
  })

  it('业务源码零 --r99- 令牌（不变量）', () => {
    for (const f of Object.keys(SRC)) expect(SRC[f], f).not.toMatch(/--r99-/)
  })

  it('禁用态 2 处保持无 role（防误改，不变量）', () => {
    const mastery = read('pages/mastery/mastery.vue')
    const shop = read('pages/shop/shop.vue')
    const mb = linkBlock(mastery, '@click="resetMastery"')
    const sb = linkBlock(shop, '@click="doBuy(it)"')
    expect(mb, '须能定位 mastery 重铸 link').not.toBeNull()
    expect(sb, '须能定位 shop 购买 link').not.toBeNull()
    expect(mb!, 'mastery.vue:66 禁用态不在本轮范围，须保持无 role').not.toContain('role="button"')
    expect(sb!, 'shop.vue:129 禁用态不在本轮范围，须保持无 role').not.toContain('role="button"')
  })

  it('common.css .link 与全局键盘焦点样式保留（不变量）', () => {
    const common = read('styles/common.css')
    expect(common).toContain('.link {')
    expect(common).toContain('color: var(--accent)')
    expect(common).toContain('[role="button"]')
    expect(common).toContain('cursor: pointer')
    expect(common).toContain('[tabindex]):focus-visible')
  })
})
