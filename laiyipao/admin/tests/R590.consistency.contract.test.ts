// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R590 UsersView 最近战报结果列跨页一致性契约（修复英文直出 win/lose）。
 * 纯源码字符串断言：结果列改 template 中文映射 win→胜 / lose→负，
 * value/result 英文枚举判断保留，label/width 不变。
 * 口径去伪：admin BattlesView 列表 :395 与 me.vue :728 均为 胜/负；
 *   me.vue :381 为 胜利/失败、BattlesView 详情 :474 为 通关/失败（三方并存，据实披露）；
 *   本修复按队长指定对齐 admin 列表 胜/负 口径。
 * 守卫钉死：筛选请求参数/后端枚举/其他列/fmtTime/筛选逻辑/他页/玩家端/业务 handler/样式零改。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (p: string) => readFileSync(resolve(process.cwd(), p), 'utf8')
const src = read('src/views/UsersView.vue')
const count = (hay: string, needle: string) => hay.split(needle).length - 1

describe('R590 UsersView 最近战报结果列中文映射', () => {
  it('核心1 结果列改 template 中文映射（win→胜 / lose→负，英文枚举判断保留）', () => {
    expect(src).toContain('<el-table-column label="结果" width="70">')
    expect(src).toContain("{{ row.result === 'win' ? '胜' : '负' }}")
    expect(src).toContain('<template #default="{ row }">{{ row.result ===')
  })

  it('核心2 结果列不再英文直出 prop=result（原生自闭合列已移除）', () => {
    expect(src).not.toContain('prop="result" label="结果"')
    expect(src).not.toContain('<el-table-column prop="result"')
  })

  it('核心3 中文映射与 admin BattlesView 列表、me.vue 事件流口径一致（胜/负）', () => {
    const battles = read('src/views/BattlesView.vue')
    const me = read('../miniapp/src/pages/me/me.vue')
    expect(battles).toContain("{{ row.result === 'win' ? '胜' : '负' }}")
    expect(me).toContain("'胜' : '负'")
    expect(src).toContain("'胜' : '负'")
  })

  it('守卫4 筛选请求参数零改：fetchUsers 参数全保留、无 status 参数新增', () => {
    expect(src).toContain('const res = await fetchUsers({')
    for (const p of ['keyword: keyword.value || undefined','limit: limit.value','offset: offset.value','date_from: dateRange.value?.[0] || undefined','date_to: dateRange.value?.[1] || undefined','is_guest: isGuestFilter.value','last_login_from: lastLoginRange.value?.[0] || undefined','last_login_to: lastLoginRange.value?.[1] || undefined','suspicious: suspiciousFilter.value']) {
      expect(src).toContain(p)
    }
    const fetchBlock = src.slice(src.indexOf('const res = await fetchUsers({'), src.indexOf('const res = await fetchUsers({') + 800)
    expect(fetchBlock).not.toContain('status')
  })

  it('守卫5 业务 handler / state / 序号守卫 / asRow 零改', () => {
    expect(src).toContain('async function openDetail(u: AdminUserView) {')
    expect(src).toContain('const res = await fetchUserDetail(u.id)')
    expect(src).toContain('let detailReqSeq = 0')
    expect(src).toContain('let listReqSeq = 0')
    expect(src).toContain('const asRow = (r: unknown) => r as AdminUserView')
    expect(src).toContain('async function load() {')
  })

  it('守卫6 其他列零改（id/level_id/score/kills/leaked/created_at 经 fmtTime）', () => {
    expect(src).toContain('<el-table-column prop="id" label="ID" width="60" />')
    expect(src).toContain('<el-table-column prop="level_id" label="关卡" width="70" />')
    expect(src).toContain('<el-table-column prop="score" label="分数" width="100" />')
    expect(src).toContain('<el-table-column prop="kills" label="击杀" width="70" />')
    expect(src).toContain('<el-table-column prop="leaked" label="漏怪" width="70" />')
    expect(src).toContain('<el-table-column prop="created_at" label="时间" min-width="160">')
    expect(src).toContain('{{ fmtTime(row.created_at) }}')
  })

  it('守卫7 他页 / 玩家端 / R588 EconomyView 零改、无全局增强器', () => {
    const battles = read('src/views/BattlesView.vue')
    const me = read('../miniapp/src/pages/me/me.vue')
    const eco = read('src/views/EconomyView.vue')
    expect(battles).toContain("{{ detail.result === 'win' ? '通关' : '失败' }}")
    expect(me).toContain("{{ b.result === 'win' ? '胜利' : '失败' }}")
    expect(eco).toContain('function orderStatusText(s: string): string {')
    expect(eco).toContain('{{ orderStatusText(row.status) }}')
    expect(src).not.toContain('MutationObserver')
    expect(src).not.toContain('document.querySelector')
    expect(count(src, '<el-table :data="detail.recent_battles"')).toBe(1)
  })

  it('守卫8 既有 UsersView 测试断言已随行为更新为胜（win→胜，旧英文直出断言不可回退）', () => {
    const test = read('tests/UsersView.test.ts')
    expect(test).toContain("expect(text).toContain('胜')")
    expect(test).not.toContain("expect(text).toContain('win')")
  })
})
