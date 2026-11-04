// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 第 124 轮：共享 fmtTime 的行为守卫。
 * 服务端时间戳是 UTC（pgx 解码 + RFC3339 下发），
 * 后台必须渲染成**查看者本地时区**，运营才读得对。
 * 判据与生产同一算法（Date + 本地分量），与测试机时区无关。
 */
import { describe, expect, it } from 'vitest'
import { auditDetailFull, auditDetailPreview, fmtTime, isCancel } from '@/utils/format'

/**
 * R98：R74/R75 的审计 before/after 快照在列表里被 80 字符预览截断。
 * 用真实 update_level 审计行（实测 detail=260 字符，Patch 在 before 之后）
 * 钉住：预览看不到新值，全文必须看得到。
 */
const R98_HEAVY_DETAIL = {
  before: {
    name: '边境哨站 · 1',
    star_targets: [11941, 16916, 19503],
    terrain_config: [{ x: 881, y: 771, kind: 'oil_drum', param: 9 }],
  },
  patch: {
    name: '边境哨站 · 1-改',
    star_targets: [11942, 16917, 19504],
    terrain_config: [{ x: 882, y: 772, kind: 'oil_drum', param: 9 }],
  },
}

function localFmt(iso: string): string {
  const d = new Date(iso)
  const p = (n: number) => String(n).padStart(2, '0')
  return `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())} ${p(d.getHours())}:${p(d.getMinutes())}:${p(d.getSeconds())}`
}

describe('fmtTime 本地时区渲染', () => {
  it('空值 → 占位 —', () => {
    expect(fmtTime('')).toBe('—')
  })

  it('UTC 时间戳渲染成查看者本地时区', () => {
    const iso = '2026-01-02T03:04:05Z'
    expect(fmtTime(iso)).toBe(localFmt(iso))
    // 关键：不再是「原样 slice」（修前会恒等于 2026-01-02 03:04:05）
    // 若本机恰在 UTC，localFmt 也等于原样，但算法一致即判据成立。
  })

  it('带偏移的 RFC3339 也按本地渲染', () => {
    const iso = '2026-01-02T03:04:05+08:00'
    expect(fmtTime(iso)).toBe(localFmt(iso))
  })

  it('非法输入不崩，退回纯展示', () => {
    expect(fmtTime('not-a-time')).toBe('not-a-time')
  })
})

/**
 * 审计 detail 的预览/全文（R98）。
 *
 * 判据对齐真实行：update_level 的 detail 实测 260 字符，jsonb 里的顺序是
 * `before` 在前、`patch` 在后 —— 80 字符预览恰好切在 before 的
 * terrain_config 键上，**新值一个字符都看不到**。
 */
describe('auditDetailPreview / auditDetailFull', () => {
  it('80 字符预览：新值（patch）落在截断之外', () => {
    const preview = auditDetailPreview(R98_HEAVY_DETAIL)
    expect(preview.endsWith('…')).toBe(true)
    expect(preview.length).toBe(81)
    expect(preview).toContain('"before"')
    expect(preview).not.toContain('1-改')
    expect(preview).not.toContain('11942')
  })

  it('全文：完整 before/after、缩进多行、含被截断的末字段', () => {
    const full = auditDetailFull(R98_HEAVY_DETAIL)
    expect(full.split('\n').length).toBeGreaterThan(1)
    expect(full).toContain('"before"')
    expect(full).toContain('"patch"')
    expect(full).toContain('1-改')
    expect(full).toContain('11942')
    expect(full).toContain('"param": 9')
    expect(full.length).toBeGreaterThan(80)
  })

  it('null/字符串/循环引用：占位、原样、不崩', () => {
    expect(auditDetailPreview(null)).toBe('—')
    expect(auditDetailFull(undefined)).toBe('—')
    expect(auditDetailPreview('纯文本')).toBe('纯文本')
    expect(auditDetailFull('纯文本')).toBe('纯文本')
    const cyclic: Record<string, unknown> = {}
    cyclic.self = cyclic
    expect(auditDetailPreview(cyclic)).toBe('[object Object]')
    expect(auditDetailFull(cyclic)).toBe('[object Object]')
  })
})

/**
 * `isCancel` 的行为守卫（第 124 轮 P2-2 的配套判据）。
 *
 * Element Plus 的 `ElMessageBox` 在用户点「取消」时抛的是**字符串** `'cancel'`，
 * 不是 Error。此前各视图用空 catch 把「取消」与「请求失败」一起吞掉，
 * 于是 readonly 账号点写按钮拿到 403 时**屏幕上什么都没有发生**。
 * 所以这个判定必须把两类输入都认出来，且不能把真正的错误误判成取消。
 */
describe('isCancel', () => {
  it('识别 Element Plus 的取消/关闭字符串', () => {
    expect(isCancel('cancel')).toBe(true)
    expect(isCancel('close')).toBe(true)
  })

  it('识别被包成 Error 的取消/关闭', () => {
    expect(isCancel(new Error('cancel'))).toBe(true)
    expect(isCancel(new Error('close'))).toBe(true)
  })

  it('不把真实错误当成取消', () => {
    // 这三类必须为 false，否则 403 / 网络错误会被静默吞掉
    expect(isCancel(new Error('HTTP 403: Forbidden'))).toBe(false)
    expect(isCancel(new Error('network down'))).toBe(false)
    expect(isCancel(new Error(''))).toBe(false)
  })

  it('不把非字符串非 Error 的输入当成取消', () => {
    expect(isCancel(undefined)).toBe(false)
    expect(isCancel(null)).toBe(false)
    expect(isCancel(0)).toBe(false)
    expect(isCancel({ message: 'cancel' })).toBe(false)
    expect(isCancel(['cancel'])).toBe(false)
  })

  it("不做大小写宽松匹配（Cancel 不是约定值）", () => {
    // 放宽大小写会掩盖「有人手写了 'Cancel'」这类真问题
    expect(isCancel('Cancel')).toBe(false)
    expect(isCancel(new Error('CANCEL'))).toBe(false)
  })
})
