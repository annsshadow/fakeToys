// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readdirSync, readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { ApiError, PermissionError } from '@oa4rust/sdk'
import { describe, expect, it } from 'vitest'
import { describeBatchOutcome, runTwinBatch } from './twinBatch'

describe('孪生端点批处理结果（R139）', () => {
  it('全部成功：绿色，条数来自真实结果', async () => {
    const outcome = await runTwinBatch([Promise.resolve(1), Promise.resolve(2), Promise.resolve(3)])
    expect(outcome).toMatchObject({ total: 3, okCount: 3, failedCount: 0 })
    expect(describeBatchOutcome('热图孪生端点', outcome)).toEqual({
      tone: 'success',
      message: '热图孪生端点：3 条已提交',
    })
  })

  it('全部失败：红色，且绝不能出现「已提交」', async () => {
    const outcome = await runTwinBatch([
      Promise.reject(new PermissionError('forbidden')),
      Promise.reject(new ApiError('HTTP 404', 404)),
    ])
    expect(outcome).toMatchObject({ total: 2, okCount: 0, failedCount: 2 })
    const summary = describeBatchOutcome('热图孪生端点', outcome)
    expect(summary.tone).toBe('error')
    expect(summary.message).toContain('2 条全部失败')
    expect(summary.message).not.toContain('已提交')
    // 403/404 经 mutationErrorMessage 变成可读文案，而不是英文/HTTP 码
    expect(summary.message).toContain('没有权限执行该操作')
  })

  it('部分失败：不能假装全成功，成功/失败分别计数', async () => {
    const outcome = await runTwinBatch([
      Promise.resolve(1),
      Promise.resolve(2),
      Promise.reject(new ApiError('boom', 500)),
    ])
    expect(outcome).toMatchObject({ total: 3, okCount: 2, failedCount: 1 })
    const summary = describeBatchOutcome('热图孪生端点', outcome)
    expect(summary.tone).toBe('warning')
    expect(summary.message).toBe('热图孪生端点：成功 2 条 / 失败 1 条（操作失败，请重试）')
  })

  it('空批次：报错而不是静默成功', async () => {
    const summary = describeBatchOutcome('热图孪生端点', await runTwinBatch([]))
    expect(summary.tone).toBe('error')
    expect(summary.message).not.toContain('已提交')
  })
})

// —— 存量基线：本轮只迁移 HotpicApp 一处做示范，其余 29 个文件按需迁移（不逐个改）。——
// 这份基线的作用是「防回退 / 防新增」：新复制一行 const s、或把某个成功提示写死，
// 都会让下面的 toEqual 变红；有意迁移时需要同步下调基线。
const LEGACY_INLINE_S: Record<string, number> = {
  AIChatApp: 5,
  AttendanceApp: 9,
  BamApp: 3,
  BBSForum: 5,
  CalendarApp: 2,
  CmsIndexApp: 8,
  DocumentApp: 3,
  FileManager: 9,
  HotpicApp: 1,
  IMChat: 3,
  JPushApp: 1,
  MeetingApp: 8,
  MindApp: 1,
  OrgDirectoryApp: 1,
  OrgViewer: 18,
  Personal: 8,
  PortalApp: 5,
  PortalDesigner: 4,
  ProcessDesigner: 1,
  ProcessFormDesignerApp: 1,
  ProcessManagerApp: 12,
  ProcessTaskCenterApp: 13,
  ProcessWork: 12,
  ProgramCenterApp: 21,
  QueryDesigner: 3,
  QueryManagerDeep: 3,
  QueryViewApp: 7,
  RecycleApp: 1,
  SearchApp: 1,
  ServerApp: 3,
}

// 旧写法里「失败也计入已提交」的行：`toast.success(`...${rs.length} 条已提交`)`
const LEGACY_TWIN_SUCCESS_LINES: Record<string, number> = {
  AIChatApp: 2,
  AttendanceApp: 2,
  BBSForum: 2,
  CalendarApp: 1,
  CmsIndexApp: 3,
  DocumentApp: 3,
  FileManager: 4,
  IMChat: 1,
  MeetingApp: 2,
  OrgViewer: 5,
  Personal: 2,
  PortalApp: 2,
  ProcessWork: 9,
  ProgramCenterApp: 9,
  QueryManagerDeep: 2,
  QueryViewApp: 3,
  RecycleApp: 1,
  SearchApp: 1,
  ServerApp: 3,
}

describe('孪生端点契约：存量可查、增量可拦（R139）', () => {
  const viewsDir = import.meta.dirname
  const vueFiles = readdirSync(viewsDir).filter((f) => f.endsWith('.vue'))
  const sourceOf = (f: string) => readFileSync(resolve(viewsDir, f), 'utf8')
  const countByFile = (pattern: RegExp) => {
    const out: Record<string, number> = {}
    for (const f of vueFiles) {
      const n = (sourceOf(f).match(pattern) ?? []).length
      if (n > 0) out[f.replace(/\.vue$/, '')] = n
    }
    return out
  }

  it('内联 const s 的数量与基线一致（新增复制即红）', () => {
    expect(countByFile(/const s = <T>/g)).toEqual(LEGACY_INLINE_S)
  })

  it('存量「rs.length 已提交」提示与基线一致（写死/删改即红）', () => {
    expect(countByFile(/toast\.success\(`[^`]*\$\{rs\.length\}/g)).toEqual(LEGACY_TWIN_SUCCESS_LINES)
  })

  it('示范文件已迁移到共享 util，不再自己吞错计数', () => {
    const hotpic = sourceOf('HotpicApp.vue')
    expect(hotpic).toContain('runTwinBatch([')
    expect(hotpic).toContain('describeBatchOutcome(')
    expect(hotpic).not.toContain('热图孪生端点 ${rs.length} 条已提交')
  })
})
