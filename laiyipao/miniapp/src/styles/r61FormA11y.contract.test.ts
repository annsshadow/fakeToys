// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R61 表单输入语义与错误播报契约（test-first RED）
 * 真实读 laiyipao/miniapp/src/pages 下相关 .vue 源码，禁止内嵌实现。
 * 缺口：17 处 input 无 aria-label（AX 树裸 textbox），6 处错误/状态节点无 role/aria-live。
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

// 取包含 v-model="xxx" 的 input 开标签块
function inputBlock(src: string, vmodel: string): string | null {
  const vm = src.indexOf('v-model="' + vmodel + '"')
  if (vm < 0) return null
  const start = src.lastIndexOf('<input', vm)
  if (start < 0 || vm - start > 400) return null
  const end = src.indexOf('>', vm)
  return end < 0 ? null : src.slice(start, end + 1)
}

// 取 needle 之前最近的 text/view 开标签块
function tagBefore(src: string, needle: string): string | null {
  const i = src.indexOf(needle)
  if (i < 0) return null
  const t = src.lastIndexOf('<text', i)
  const v = src.lastIndexOf('<view', i)
  const start = Math.max(t, v)
  if (start < 0) return null
  const gt = src.indexOf('>', start)
  return gt < 0 ? null : src.slice(start, gt + 1)
}

// [相对路径, v-model, 期望 aria-label] —— 稳定身份，不依赖行号
const INPUTS: [string, string, string][] = [
  ['pages/announcements/announcements.vue', 'annKeyword', '搜索公告标题 / 正文'],
  ['pages/bag/bag.vue', 'presetName', '预设名（最多 20 字）'],
  ['pages/equipment/equipment.vue', 'keyword', '搜索装备名称 / 部位 / 元素'],
  ['pages/enemies/enemies.vue', 'keyword', '搜索敌人名称 / 类别（如 Boss、飞行）'],
  ['pages/gems/gems.vue', 'keyword', '搜索宝石名称 / 属性'],
  ['pages/login/login.vue', 'username', '用户名'],
  ['pages/login/login.vue', 'password', '口令'],
  ['pages/me/me.vue', 'nicknameDraft', '新昵称（≤64 字）'],
  ['pages/me/me.vue', 'oldPassword', '原口令'],
  ['pages/me/me.vue', 'newPassword', '新口令（至少 10 位）'],
  ['pages/me/me.vue', 'upgradeUsername', '用户名'],
  ['pages/me/me.vue', 'upgradePassword', '密码（至少 10 位）'],
  ['pages/reactions/reactions.vue', 'keyword', '搜索反应名称'],
  ['pages/skills/skills.vue', 'keyword', '搜索技能名称 / 元素 / 主动被动'],
  ['pages/signin/signin.vue', 'code', '输入兑换码'],
  ['pages/skins/skins.vue', 'keyword', '搜索皮肤名称 / 稀有度'],
  ['pages/verify/verify.vue', 'battleId', 'battle_id'],
]

// [相对路径, needle, 类型(error|state), 说明]
const NODES: [string, string, string, string][] = [
  ['pages/login/login.vue', 'form-hint', 'state', 'form-hint 空字段提示'],
  ['pages/login/login.vue', '{{ errMsg }}', 'error', 'login errMsg'],
  ['pages/verify/verify.vue', '{{ errMsg }}', 'error', 'verify errMsg'],
  ['pages/signin/signin.vue', '{{ redeemMsg }}', 'state', 'signin redeemMsg'],
  ['pages/me/me.vue', '{{ passwordMsg }}', 'state', 'me passwordMsg'],
  ['pages/bag/bag.vue', 'v-if="presetSaving"', 'state', 'bag presetSaving'],
]

describe('R61 表单输入语义与错误播报契约', () => {
  it('17 处 input 均带与可见占位/label 一致的 aria-label（WCAG 4.1.2）', () => {
    const cache = new Map<string, string>()
    const bad: string[] = []
    for (const [rel, vm, label] of INPUTS) {
      let src = cache.get(rel)
      if (src === undefined) { src = read(rel); cache.set(rel, src) }
      const block = inputBlock(src, vm)
      const want = 'aria-label="' + label + '"'
      if (block === null) bad.push(rel + ' v-model=' + vm + ' 未找到 input 标签块')
      else if (block.indexOf(want) < 0)
        bad.push(rel + ' v-model=' + vm + ' 缺 ' + want)
    }
    expect(bad, '以下 input 无可访问名，AX 树为裸 textbox: ' + bad.join('; ')).toEqual([])
  })

  it('错误/状态节点含 role=alert 或 role=status+aria-live=polite（WCAG 3.3.1）', () => {
    const cache = new Map<string, string>()
    const bad: string[] = []
    for (const [rel, needle, kind, name] of NODES) {
      let src = cache.get(rel)
      if (src === undefined) { src = read(rel); cache.set(rel, src) }
      const tag = tagBefore(src, needle)
      if (tag === null) { bad.push(name + ': 未找到节点标签块'); continue }
      const alert = tag.indexOf('role="alert"') >= 0
      const status = tag.indexOf('role="status"') >= 0 && tag.indexOf('aria-live="polite"') >= 0
      const ok = kind === 'error' ? (alert || status) : status
      if (!ok) bad.push(name + ': 缺 role=alert 或 role=status+aria-live=polite')
    }
    expect(bad, '以下反馈节点读屏器不会播报: ' + bad.join('; ')).toEqual([])
  })

  it('R60 键盘锚点与占位惯例不回退（不变量）', () => {
    const r60 = read('styles/r60A11yKeyboard.contract.test.ts')
    expect(r60, 'R60 契约必须仍钉 tabindex').toContain('tabindex="0"')
    expect(r60, 'R60 契约必须仍钉 @keydown').toContain('@keydown')
    const battles = read('pages/battles/battles.vue')
    const battle = read('pages/battle/battle.vue')
    const index = read('pages/index/index.vue')
    expect(battles, 'R43 r20 CTA 须保留').toContain('class="r20-ai-cta" data-r20-cta role="button" @click="goStage()"')
    expect(battle, 'R44 r29 CTA 须保留').toContain('class="r29-result-timeline-cta" @click.stop="openReactionTimeline"')
    expect(index, 'R43 ai-cta 须保留').toContain('@click="go(aiInsight.hint)"')
    expect(index, 'R48 去战斗 CTA 须保留').toContain('@click="goStage"')
    const cache = new Map<string, string>()
    const bad: string[] = []
    for (const [rel, vm] of INPUTS) {
      let src = cache.get(rel)
      if (src === undefined) { src = read(rel); cache.set(rel, src) }
      const block = inputBlock(src, vm)
      if (block !== null && block.indexOf('placeholder-class="ph"') < 0)
        bad.push(rel + ' v-model=' + vm + ' 丢 placeholder-class=ph')
    }
    expect(bad, 'R55 占位惯例不得回退: ' + bad.join('; ')).toEqual([])
  })

  it('令牌三锚点不变且不新增 --r61-（不变量）', () => {
    const tokens = read('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toContain('--r61-')
  })

  it('17 处 input 挂 v-aria-native，main.ts 注册 aria-native 指令桥接 native input（WCAG 4.1.2）', () => {
    const cache = new Map<string, string>()
    const bad: string[] = []
    for (const [rel, vm] of INPUTS) {
      let src = cache.get(rel)
      if (src === undefined) { src = read(rel); cache.set(rel, src) }
      const block = inputBlock(src, vm)
      if (block === null) bad.push(rel + ' v-model=' + vm + ' 未找到 input 标签块')
      else if (block.indexOf('v-aria-native') < 0)
        bad.push(rel + ' v-model=' + vm + ' 缺 v-aria-native（静态 aria-label 编译后落在 uni-input 外层，AX 树裸名）')
    }
    expect(bad, '以下 input 未挂运行时桥接指令 v-aria-native，编译后原生 input 仍无可访问名: ' + bad.join('; ')).toEqual([])

    // 引号归一，实现用单/双引号表达同一运行时机制时均认，避免纯格式假阳性
    const main = read('main.ts').split('"').join("'")
    expect(main, 'main.ts 须注册 app.directive(aria-native) 运行时指令').toContain("app.directive('aria-native'")
    expect(main, 'aria-native 指令须 querySelector 原生 input.uni-input-input').toContain("querySelector('input.uni-input-input')")
    expect(main, 'aria-native 指令须 setAttribute 可访问名到原生 input').toContain("setAttribute('aria-label'")
  })
})