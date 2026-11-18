// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 导出文件名日期口径：按 UTC+8 业务日，而非 toISOString 的 UTC 墙钟。
 *
 * 修前各视图用 new Date().toISOString().slice(0, 10)：部署在 UTC+8 时，
 * 本地凌晨 0–8 点对应的 UTC 还是前一天，运营导出的 CSV 文件名带"昨天"
 * 的日期，与业务日（R167 统一 UTC+8）和运营当天看到的日期不一致。
 *
 * 判据用显式 UTC 时刻 + 固定 +8 偏移，完全与测试机时区无关。
 */
import { describe, expect, it } from 'vitest'
import { exportDateLabel } from '@/utils/format'

describe('exportDateLabel 导出文件名日期', () => {
  it('UTC+8 跨日：UTC 前一天 20:00 → 业务日次日', () => {
    // 2026-01-02T20:00:00Z = 北京时间 2026-01-03 04:00
    // toISOString().slice(0,10) 会错误地给出 2026-01-02
    expect(exportDateLabel(new Date('2026-01-02T20:00:00Z'))).toBe('2026-01-03')
  })

  it('UTC+8 边界：UTC 16:00 → 业务日当天 00:00', () => {
    // 2026-01-02T16:00:00Z = 北京时间 2026-01-03 00:00
    expect(exportDateLabel(new Date('2026-01-02T16:00:00Z'))).toBe('2026-01-03')
  })

  it('UTC+8 不跨日：UTC 04:00 → 业务日当天 12:00', () => {
    expect(exportDateLabel(new Date('2026-01-03T04:00:00Z'))).toBe('2026-01-03')
  })

  it('补零：单月单日不丢前导零', () => {
    // 2026-01-01T20:00:00Z = 北京时间 2026-01-02 04:00
    expect(exportDateLabel(new Date('2026-01-01T20:00:00Z'))).toBe('2026-01-02')
  })
})
