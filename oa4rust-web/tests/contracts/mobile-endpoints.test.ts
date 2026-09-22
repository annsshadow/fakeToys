// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readdirSync, readFileSync, statSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/**
 * 移动端端点契约守卫（S3 同型）：apps/mobile 全源码出现过的每个 /api 端点，
 * 必须命中后端 oa4rust 实际注册的 axum 路由（白名单抄自各 crate 的
 * routes.rs / u2_router.rs 注册字面量）。历史教训：mobile 初版服务层镜像了
 * desktop apis 的"约定式"路径（如 task/{id}/handle、conversation/list/paging），
 * 其中 13 条在后端根本不存在，实跑必 404——该测试把端点与真实路由表钉死。
 */

// 白名单：oa4rust 后端实际注册、且移动端允许调用的路由（占位符与后端一致）。
const REGISTERED_BACKEND_ROUTES: string[] = [
  // 认证（auth crate）
  '/api/authentication/login',
  '/api/authentication/logout',
  '/api/authentication/who',
  '/api/authentication/refresh',
  '/api/authentication/captcha',
  // 流程 surface 列表（processplatform_assemble_surface/routes.rs，GET/POST 见后端注册）
  '/api/processplatform/assemble/surface/task/list/my/paging/{page}/size/{size}',
  '/api/processplatform/assemble/surface/taskcompleted/list/my/paging/{page}/size/{size}',
  '/api/processplatform/assemble/surface/work/list/my/paging/{page}/size/{size}',
  // 流程引擎审批（processplatform_service_processing/routes.rs，桌面 E2E 实跑通过）
  '/api/task/{id}/complete',
  '/api/task/{id}/reject',
  // IM 会话（message_assemble_communicate/routes.rs）
  '/api/message/assemble/communicate/im/conversation/list/my',
  '/api/message/assemble/communicate/im/msg/list/{page}/size/{size}',
  '/api/message/assemble/communicate/im/msg',
  '/api/message/assemble/communicate/im/conversation/{id}/read',
  // 文件（file_assemble_control/routes.rs）
  '/api/file/assemble/control/file/list/{id}',
  '/api/file/assemble/control/file/{id}/download',
  // 组织（organization_assemble_control u2 路由）
  '/api/organization/assemble/control/person/list/like/mockputtopost',
  '/api/organization/assemble/control/person/{flag}',
  // 通用（general crate）
  '/api/general/dict/list',
  // IM 发起单聊（message_assemble_communicate，创建 single 会话）
  '/api/message/assemble/communicate/im/conversation',
  // 流程发起（processplatform designer + service_processing，桌面 ProcessWork 同源端点）
  '/api/processplatform/assemble/designer/list/{category}',
  '/api/processplatform/assemble/designer/get/{id}',
  '/api/form/{id}',
  '/api/processplatform/service/processing/work',
  '/api/processplatform/service/processing/data/work/{id}',
  // 工作详情侧栏（processplatform_assemble_surface/routes.rs：data/attachment/record/worklog）
  '/api/processplatform/assemble/surface/data/work/{id}',
  '/api/processplatform/assemble/surface/attachment/list/work/{workId}',
  '/api/processplatform/assemble/surface/record/list/workorworkcompleted/{workOrWorkCompleted}',
  '/api/processplatform/assemble/surface/worklog/list/workorworkcompleted/{workOrWorkCompleted}',
  // 考勤本人打卡（attendance_assemble_control v2 mobile）
  '/api/attendance/assemble/control/v2/mobile/check/pre',
  '/api/attendance/assemble/control/v2/mobile/check',
  // 附件二进制上传（file_assemble_control attachment folder，落点 FILE_FILE）
  '/api/attachment/upload/folder/{folderId}',
  // 附件存储 FILE_FILE 列表 / 下载（P4：上传落点在此，非 x_file 我的文件）
  '/api/attachment/list/editor/{owner}',
  '/api/attachment/{id}/download',
  // 公告（ai crate 已实装 x_ai_ann 家族，工作台公告区）
  '/api/ai/assemble/control/ann/list',
  // ── 阶段 G（移动端能力扩展）：会议 / 日历 / 组织增强 / 只读浏览 / 回收站 / 搜索 / 统计 / 推送 ──
  // 会议（meeting_assemble_control/routes.rs）——F2
  '/api/meeting/assemble/control/meeting/list/apply/{page}/size/{size}',
  '/api/meeting/assemble/control/meeting/list/invited/wait',
  '/api/meeting/assemble/control/meeting/list/wait/confirm',
  '/api/meeting/assemble/control/meeting/list/coming/day/{count}',
  '/api/meeting/assemble/control/meeting/{id}',
  '/api/meeting/assemble/control/meeting/{id}/accept',
  '/api/meeting/assemble/control/meeting/{id}/reject',
  '/api/meeting/assemble/control/meeting/{id}/confirm/allow',
  '/api/meeting/assemble/control/meeting/{id}/confirm/deny',
  '/api/meeting/assemble/control/meeting/{id}/checkin',
  '/api/meeting/assemble/control/meeting/{id}/checkin/code',
  // 日历（calendar_assemble_control/routes.rs）——F3
  '/api/calendar_assemble_control/calendar/list/my',
  '/api/calendar_assemble_control/calendar/list/public',
  '/api/calendar_assemble_control/calendar/{id}',
  '/api/calendar_assemble_control/event/list/filter',
  '/api/calendar_assemble_control/event/{id}',
  '/api/calendar_assemble_control/event/rfc/{id}',
  '/api/calendar_assemble_control/get/control/config',
  '/api/calendar_assemble_control/list/control/calendars',
  // 日历核心事件管理（calendar/routes.rs）——rev100 三端配合
  '/api/calendar/event/list/{calendarId}',
  '/api/calendar/event/create',
  '/api/calendar/event/remove',
  // 组织增强：单位 / 群组 / 身份 / 角色——F1
  '/api/unit/list/all',
  '/api/unit/list/unit/tree',
  '/api/organization/assemble/control/group/list/like',
  '/api/organization/assemble/control/group/list/{flag}/sub/nested',
  '/api/organization/assemble/control/identity/list/person/{personFlag}',
  '/api/organization/assemble/control/role/list/person/{personFlag}',
  // 门户表面 / 查询表面 / CMS 只读——F4
  '/api/portal/assemble/surface/page/list/portal/{portal}',
  '/api/portal/assemble/surface/page/v2/{id}',
  '/api/portal/assemble/surface/page/v2/{id}/mobile',
  '/api/portal/assemble/surface/page/v2/{flag}/portal/{portalFlag}',
  '/api/queryview/view/list/query/{queryFlag}',
  '/api/queryview/execute/{view}/{id}',
  '/api/queryview/execute/v2/{view}/{id}/{page}/{size}',
  '/api/queryview/bundle/{view}/{id}',
  '/api/queryview/bundle/v2/{view}/{id}',
  '/api/queryview/list',
  '/api/cms/core/entity/column/list',
  '/api/cms/core/entity/column_manager/list',
  '/api/cms_assemble_control/document/search',
  // 回收站 / 搜索 / 统计 / 推送——F5
  '/api/recycle/list',
  '/api/recycle/{id}',
  '/api/recycle/delete/{id}',
  '/api/recycle/empty',
  '/api/queryview/search',
  '/api/bbs/assemble/control/subject/search',
  '/api/attendance/assemble/control/statistics/list',
  '/api/attendance/assemble/control/attendancestatisticalcycle/list/all',
  '/api/attendance/assemble/control/attendancestatisticalcycle/cycleDetail/{year}/{month}',
  '/api/attendance/assemble/control/dingding/statistic/person/year/{year}/month/{month}',
  '/api/jpush/assemble/control/device/list/{pushType}',
  '/api/jpush/assemble/control/device/config/push/type',
  '/api/jpush/assemble/control/list/control/apps',
]

const mobileSrcRoot = resolve(import.meta.dirname, '../../apps/mobile/src')

function collectMobileEndpointPaths(): string[] {
  const files: string[] = []
  function walk(dir: string): void {
    for (const name of readdirSync(dir)) {
      const full = resolve(dir, name)
      const st = statSync(full)
      if (st.isDirectory()) walk(full)
      else if (/\.(ts|vue)$/.test(name) && !name.endsWith('.test.ts')) files.push(full)
    }
  }
  walk(mobileSrcRoot)
  const paths = new Set<string>()
  for (const file of files) {
    const text = readFileSync(file, 'utf8')
    for (const m of text.matchAll(/\/api\/[A-Za-z0-9_\-/{}$.]+/g)) {
      paths.add(m[0])
    }
  }
  return [...paths]
}

/** 把候选路径归一化为段数组：模板变量 ${...} 与纯数字段 → *。 */
function segsOf(path: string): string[] {
  return path
    .toLowerCase()
    .split('/')
    .filter(Boolean)
    .map((s) => (s.includes('${') || /^\d+$/.test(s) ? '*' : s))
}

function segsOfPattern(pattern: string): string[] {
  return pattern
    .toLowerCase()
    .split('/')
    .filter(Boolean)
    .map((s) => (s.startsWith('{') ? '*' : s))
}

function matchesAnyRegistered(path: string): boolean {
  const cand = segsOf(path)
  for (const pattern of REGISTERED_BACKEND_ROUTES) {
    const pat = segsOfPattern(pattern)
    if (pat.length !== cand.length) continue
    if (pat.every((s, i) => s === '*' || cand[i] === '*' || s === cand[i])) return true
  }
  return false
}

describe('mobile endpoints are all registered in the oa4rust backend', () => {
  it('every /api path referenced by mobile source hits a registered backend route', () => {
    const used = collectMobileEndpointPaths()
    expect(used.length).toBeGreaterThan(0)
    const unregistered = used.filter((p) => !matchesAnyRegistered(p))
    // 已知豁免：纯静态文本里出现的非端点片段（本应没有——出现即说明有 404 风险）。
    expect(unregistered).toEqual([])
  })

  it('each mobile business family has at least one live endpoint pinned', () => {
    const used = collectMobileEndpointPaths().join('\n')
    for (const family of [
      'authentication',
      'processplatform/assemble/surface',
      '/api/task/',
      'message/assemble/communicate/im',
      'file/assemble/control',
      'organization/assemble/control',
      'general/dict',
      'processplatform/service/processing',
      'attendance/assemble/control/v2',
      '/api/attachment/upload/folder/',
    ]) {
      expect(used.includes(family), `family ${family} missing from mobile sources`).toBe(true)
    }
  })

  it('the allowlist only contains routes that are actually used by mobile', () => {
    const usedSegs = new Set(collectMobileEndpointPaths().map((p) => p))
    const stale = REGISTERED_BACKEND_ROUTES.filter((pattern) => {
      const pats = segsOfPattern(pattern)
      return ![...usedSegs].some((u) => {
        const cand = segsOf(u)
        return cand.length === pats.length && pats.every((s, i) => s === '*' || cand[i] === '*' || s === cand[i])
      })
    })
    expect(stale, `allowlist entries no longer used by mobile: ${stale.join(', ')}`).toEqual([])
  })
})
