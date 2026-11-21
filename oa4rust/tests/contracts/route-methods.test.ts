// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readdirSync, readFileSync, statSync, writeFileSync, rmSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { BACKEND_ROUTE_METHODS } from './backend-route-methods.fixture'

/**
 * HTTP 方法契约守卫（优化轮12 institutionalize 轮10 的修复；R314 F1 扩展至 packages/apis）。
 *
 * 匹配口径与 backend-route-methods.fixture 对齐：路径归一为小写、动态段/占位段
 * 折成 {}、去尾斜杠。fixture 由 gen_mcp_tools.py 从各 crate 的 axum .route 注册
 * 重生成，不手写。EXEMPT 仅收录经人工核实的合理例外（当前为空）。
 *
 * R314 F1：原 SRC_ROOTS 只含 desktop/mobile，packages/apis 的封装漂移（R313 三处）
 * 无人守卫。本轮把 apis 纳入扫描，并用「冻结历史基线」收敛存量：
 * - API_LEGACY_MISMATCHES 是纳入当日已存在的 85 条错方法签名快照，**不是合法例外**；
 *   仅表示「历史存量，另案治理」。其中 F3_KNOWN_PENDING 五处为 t80 审计已坐实的
 *   待修漂移，显式标注而非静默放行。
 * - 任何**新增**错方法签名（不在冻结快照、不在 EXEMPT）都会让守卫红；
 *   修复一条存量后应从快照移除并下调 API_UNREGISTERED_BASELINE，只降不升。
 */

const DESKTOP_ROOT = resolve(import.meta.dirname, '../../frontend/apps/desktop/src')
const MOBILE_ROOT = resolve(import.meta.dirname, '../../mobile/src')
const API_ROOT = resolve(import.meta.dirname, '../../frontend/packages/apis/src')
const LEGACY_ROOTS = [DESKTOP_ROOT, MOBILE_ROOT]
const SRC_ROOTS = [...LEGACY_ROOTS, API_ROOT]

// 经人工核实的方法契约例外（当前为空——轮10 已把两处真错配修正）。
const EXEMPT = new Set<string>([])

// packages/apis 纳入守卫当日的历史存量错方法快照（85 条），不是合法例外，
// 是「另案治理」的冻结基线。每条都必须在此显式登记；新增签名不会被这里吸收。
const API_LEGACY_MISMATCHES = new Set<string>([
  "POST /api/organization/assemble/control/group/{}/add/member",
  "POST /api/organization/assemble/control/group/{}/delete/member",
  "GET /api/organization/assemble/control/person/list/like",
  "POST /api/organization/assemble/control/export/export/all",
  "DELETE /api/processplatform/assemble/designer/process/{}",
  "POST /api/general/dict/item/{}",
  "PUT /api/jpush/core/entity/template/{}",
  "POST /api/correlation/core/express/sync",
  "DELETE /api/correlation/service/processing/delete/{}",
  "PUT /api/correlation/service/processing/save/{}",
  "DELETE /api/correlation/service/processing/unlink/{}/{}/{}/{}",
  "POST /api/cache/commonscript/flush",
  "POST /api/cache/config/flush",
  "GET /api/queryview/importmodel/list/{}/{}",
  "GET /api/queryview/search",
  "POST /api/program_center/agent/{}/enable",
  "POST /api/program_center/agent/{}/disable",
  "POST /api/program_center/invoke/{}",
  "GET /api/mind/folder/{}",
  "POST /api/mind/assemble/control/folder/move/{}",
  "GET /api/mind/assemble/control/mind/filter/list/{}/next/{}",
  "GET /api/mind/assemble/control/mind/filter/recycle/{}/next/{}",
  "POST /api/mind/assemble/control/mind/restore/{}",
  "GET /api/mind/assemble/control/mind/{}/icon/size/{}",
  "POST /api/mind/assemble/control/mind/share/{}",
  "POST /api/mind/assemble/control/mind/share/{}/cancel",
  "PUT /api/mind/assemble/control/config/update",
  "POST /api/document/publish/{}",
  "POST /api/document/achive/{}",
  "PUT /api/calendar_assemble_control/calendar/{}",
  "PUT /api/calendar_assemble_control/event/{}",
  "DELETE /api/calendar_assemble_control/event/{}",
  "GET /api/calendar_assemble_control/event/single/{}",
  "GET /api/calendar_assemble_control/event/after/{}",
  "GET /api/calendar_assemble_control/event/all/{}",
  "POST /api/calendar_assemble_control/calendar/follow/{}",
  "POST /api/calendar_assemble_control/calendar/follow/{}/cancel",
  "GET /api/calendar_assemble_control/setting",
  "PUT /api/calendar_assemble_control/setting",
  "PUT /api/attendance/assemble/control/attendanceadmin/{}",
  "GET /api/query/assemble/designer/designer/search",
  "GET /api/query/assemble/designer/importmodel/list/{}/{}",
  "DELETE /api/recycle/{}",
  "POST /api/recycle/empty",
  "POST /api/server/execute",
  "POST /api/server/stop",
  "PUT /api/fileinfo/update/{}/content",
  "GET /api/component_assemble_control/component/{}",
  "POST /api/externaldatasources/check",
  "POST /api/externaldatasources/set/cancel",
  "GET /api/identity/list",
  "GET /api/categoryinfo/extcontent",
  "GET /api/categoryinfo/filter/list/{}/next/{}/app/{}",
  "POST /api/processplatform/assemble/surface/handover/{}",
  "PUT /api/processplatform/assemble/surface/save/{}",
  "GET /api/organization/assemble/control/unit/list/like",
  "POST /api/attendance/assemble/control/attendancedetail/filter/list",
  "GET /api/attendance/assemble/control/attendanceadmin",
  "GET /api/attendance/assemble/control/attendanceappealinfo/check",
  "GET /api/attendance/assemble/control/attendancedetail",
  "GET /api/attendance/assemble/control/attendancedetail/recive",
  "GET /api/attendance/assemble/control/attendancedetail/recivesingle",
  "GET /api/attendance/assemble/control/attendanceemployeeconfig",
  "GET /api/attendance/assemble/control/attendanceschedulesetting",
  "GET /api/attendance/assemble/control/attendanceselfholiday",
  "GET /api/attendance/assemble/control/attendancesetting",
  "GET /api/attendance/assemble/control/attendancestatisticalcycle",
  "GET /api/attendance/assemble/control/attendancestatisticrequirelog",
  "GET /api/attendance/assemble/control/attendanceworkdayconfig",
  "GET /api/attendance/assemble/control/attendanceworkdayconfig/filter",
  "GET /api/attendance/assemble/control/selfholidaysimple",
  "GET /api/attendance/assemble/control/v2/groupschedule",
  "GET /api/attendance/assemble/control/v2/leave",
  "GET /api/portal/assemble/surface/page/{}",
  "GET /api/portal/assemble/surface/page/{}",
  "GET /api/message/assemble/communicate/connector",
  "PUT /api/ai_assemble_control/config/update/mcp/{}",
  "PUT /api/ai_assemble_control/config/update/model/{}",
  "DELETE /api/ai_assemble_control/config/delete/model/{}",
  "PUT /api/ai_assemble_control/config/save",
  "DELETE /api/ai_assemble_control/file/delete/{}",
  "DELETE /api/ai_assemble_control/index/delete/{}",
  "POST /api/ai_assemble_control/index/sync/to/knowledge",
  "GET /api/role/list",
  "GET /api/express/role/list",
])

// t80 R314 审计 F3 已坐实的 5 处封装方法/路径漂移（本任务不修，另案），
// 它们也在上面的冻结快照内；这里单独显式标注「已知待修」，不并入 EXEMPT。
const F3_KNOWN_PENDING = new Set<string>([
  'POST /api/general/dict/item/{}',
  'PUT /api/jpush/core/entity/template/{}',
  'GET /api/queryview/importmodel/list/{}/{}',
  'GET /api/queryview/search',
  'POST /api/program_center/invoke/{}',
])

// 未注册路径调用基线。legacy 仍是原 860；apis 包纳入新增 167（1027-860），
// 分开计数避免 apis 存量下降掩盖 legacy 新增。只允许下降：修好一批就同步下调。
const UNREGISTERED_BASELINE = 860
const API_UNREGISTERED_BASELINE = 187

// fixture 唯一路径数下限（O16-0 恢复 129 条后的实测值）。
const MIN_REGISTERED_PATHS = 4170

function norm(p: string): string {
  return p
    .replace(/\$\{[^}]*\}/g, '{}')
    .replace(/\{[^}]*\}/g, '{}')
    .replace(/\/+$/, '')
    .toLowerCase()
}

type Call = { method: string; path: string; file: string; source: 'legacy' | 'apis' }

function collectCalls(roots: string[] = SRC_ROOTS): Call[] {
  const out: Call[] = []
  const re = /\b(?:api|mapi)\.(?:[A-Za-z_$][\w$]*\.)*(get|post|put|delete)(?:<[^>]*>)?\(\s*[`'"](\/api\/[^`'"?]*)/g
  function walk(dir: string): void {
    for (const name of readdirSync(dir)) {
      const full = resolve(dir, name)
      const st = statSync(full)
      if (st.isDirectory()) walk(full)
      else if (/\.(ts|vue)$/.test(name) && !name.endsWith('.test.ts')) {
        const text = readFileSync(full, 'utf8')
        for (const m of text.matchAll(re)) {
          out.push({
            method: m[1]!.toUpperCase(),
            path: norm(m[2]!),
            file: name,
            source: full.startsWith(API_ROOT) ? 'apis' : 'legacy',
          })
        }
      }
    }
  }
  for (const root of roots) walk(root)
  return out
}

type Verdict = 'ok' | 'exempt' | 'legacy' | 'mismatch' | 'unregistered'

function classify(call: Call): Verdict {
  const methods = BACKEND_ROUTE_METHODS[call.path]
  if (!methods) return 'unregistered'
  if (methods.includes(call.method)) return 'ok'
  const sig = `${call.method} ${call.path}`
  if (EXEMPT.has(sig)) return 'exempt'
  if (call.source === 'apis' && API_LEGACY_MISMATCHES.has(sig)) return 'legacy'
  return 'mismatch'
}

describe('前端 HTTP 方法契约', () => {
  it('每个前端 (method, path) 调用命中后端注册的方法集（历史存量冻结收敛）', () => {
    const mismatches: string[] = []
    const unregisteredLegacy: { method: string; path: string; file: string }[] = []
    const unregisteredApis: { method: string; path: string; file: string }[] = []
    for (const call of collectCalls()) {
      const v = classify(call)
      if (v === 'mismatch') mismatches.push(`${call.file}: ${call.method} ${call.path} — 后端仅 [${BACKEND_ROUTE_METHODS[call.path]!.join(', ')}]`)
      else if (v === 'unregistered') (call.source === 'apis' ? unregisteredApis : unregisteredLegacy).push(call)
    }
    expect(mismatches, `方法错配（405 风险）:\n${mismatches.join('\n')}`).toEqual([])

    const sample = unregisteredLegacy.concat(unregisteredApis)
      .slice(0, 20)
      .map(({ method, path, file }) => `${file}: ${method} ${path}`)
      .join('\n')
    expect(
      unregisteredLegacy.length,
      `legacy 未注册路径调用 ${unregisteredLegacy.length} 条，超过基线 ${UNREGISTERED_BASELINE}。\n${sample}`,
    ).toBeLessThanOrEqual(UNREGISTERED_BASELINE)
    expect(
      unregisteredApis.length,
      `apis 未注册路径调用 ${unregisteredApis.length} 条，超过冻结基线 ${API_UNREGISTERED_BASELINE}（R314 F1 纳入日存量）。新增调用要么改到已注册路由，要么下调基线并说明。\n${sample}`,
    ).toBeLessThanOrEqual(API_UNREGISTERED_BASELINE)

    expect(
      Object.keys(BACKEND_ROUTE_METHODS).length,
      `后端方法 fixture 路径数低于下限 ${MIN_REGISTERED_PATHS}——有真实路由被删或生成器漏扫。` ,
    ).toBeGreaterThanOrEqual(MIN_REGISTERED_PATHS)
  })

  it('packages/apis 新漂移能被守卫捕获（注入临时错方法封装，验证后移除）', () => {
    const probeFile = resolve(API_ROOT, '__r314_guard_probe.ts')
    const probeSig = 'GET /api/document'
    writeFileSync(probeFile, "import { api } from './index'\nexport const probe = () => api.get('/api/document')\n")
    try {
      const calls = collectCalls([API_ROOT])
      const found = calls.find((c) => `${c.method} ${c.path}` === probeSig)
      expect(found, '探针封装必须被 F1 纳入后的守卫扫到').toBeDefined()
      expect(classify(found!), '新增错方法签名必须被判为 mismatch（不被冻结基线吸收）').toBe('mismatch')
      expect(API_LEGACY_MISMATCHES.has(probeSig), '探针签名不得在历史基线内').toBe(false)
      // 对照：冻结基线内条目与 F3 五处的标注口径
      expect(classify({ method: 'POST', path: '/api/ai_assemble_control/index/sync/to/knowledge', file: 'index.ts', source: 'apis' })).toBe('legacy')
      expect(F3_KNOWN_PENDING.has('GET /api/queryview/search')).toBe(true)
      expect(EXEMPT.has('GET /api/queryview/search')).toBe(false)
    } finally {
      rmSync(probeFile, { force: true })
    }
  })
})
