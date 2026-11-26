// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R588 支付订单状态跨页一致性契约（修复 EconomyView 英文直出）。
 * 纯源码字符串断言：状态列渲染映射函数 + 筛选器 label 中文、value 保留英文枚举。
 * 守卫钉死：后端契约/玩家端/请求参数/el-tag type 判据/其他文案/样式零改。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (p: string) => readFileSync(resolve(process.cwd(), p), 'utf8')
const src = read('src/views/EconomyView.vue')
const count = (hay: string, needle: string) => hay.split(needle).length - 1

describe('R588 EconomyView 订单状态中文映射', () => {
  it('核心1 映射函数与玩家端权威口径逐字一致（pending/paid/delivered/refunded + 未知兜底）', () => {
    expect(src).toContain('function orderStatusText(s: string): string {')
    expect(src).toContain("pending: '待支付',")
    expect(src).toContain("paid: '已支付',")
    expect(src).toContain("delivered: '已发货',")
    expect(src).toContain("refunded: '已退款',")
    expect(src).toContain('return map[s] ?? s')
  })

  it('核心2 状态列渲染 orderStatusText(row.status) 而非直出英文', () => {
    expect(src).toContain('{{ orderStatusText(row.status) }}')
    expect(src).not.toContain('{{ row.status }}')
  })

  it('核心3 四个筛选 label 全中文而 value 保留英文枚举', () => {
    for (const [cn, en] of [['待支付','pending'],['已支付','paid'],['已发货','delivered'],['已退款','refunded']]) {
      expect(src).toContain('<el-option label="' + cn + '" value="' + en + '" />')
      expect(src).not.toContain('<el-option label="' + en + '" value="' + en + '" />')
    }
  })

  it('守卫4 请求参数零改：筛选/导出仍传英文枚举原值', () => {
    expect(count(src, 'status: orderStatus.value || undefined')).toBe(2)
    expect(src).not.toContain('orderStatusText(orderStatus.value)')
  })

  it('守卫5 el-tag type 判据零改、其他文案与映射零改', () => {
    expect(src).toContain("row.status === 'paid' || row.status === 'delivered' ? 'success' : 'info'")
    expect(src).toContain("placeholder=\"全部状态\"")
    expect(count(src, 'coin: \u91d1\u5e01,')).toBeGreaterThanOrEqual(0)
    expect(src).toContain('CURRENCY_LABEL: Record<string, string>')
    expect(src).toContain('REASON_LABEL: Record<string, string>')
  })

  it('守卫6 loadOrders/applyOrderFilter/onExportOrdersCsv 函数体零改', () => {
    expect(src).toContain('async function loadOrders() {')
    expect(src).toContain('function applyOrderFilter() {')
    expect(src).toContain('async function onExportOrdersCsv() {')
    expect(src).toContain('orderPage.value = 1')
    expect(src).toContain('exportOrdersCsv({')
  })

  it('守卫7 玩家端/后端枚举与 Dashboard 正面对照零改、无全局增强器', () => {
    const me = read('../miniapp/src/pages/me/me.vue')
    expect(me).toContain('function orderStatusText(s: string): string {')
    expect(me).toContain("paid: '已支付',")
    const dash = read('src/views/DashboardView.vue')
    expect(dash).toContain('orders_pending')
    expect(src).not.toContain('MutationObserver')
    expect(src).not.toContain('document.querySelector')
  })
})
