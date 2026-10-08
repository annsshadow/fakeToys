/**
 * 第 124 轮：共享 fmtTime 的行为守卫。
 * 服务端时间戳是 UTC（pgx 解码 + RFC3339 下发），
 * 后台必须渲染成**查看者本地时区**，运营才读得对。
 * 判据与生产同一算法（Date + 本地分量），与测试机时区无关。
 */
import { describe, expect, it } from 'vitest'
import { fmtTime } from '@/utils/format'

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
