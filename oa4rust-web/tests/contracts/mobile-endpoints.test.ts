import { readdirSync, readFileSync, statSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/**
 * 移动端端点契约守卫（S3 同型）：apps/mobile 全源码出现过的每个 /jaxrs 端点，
 * 必须命中后端 oa4rust 实际注册的 axum 路由（白名单抄自各 crate 的
 * routes.rs / u2_router.rs 注册字面量）。历史教训：mobile 初版服务层镜像了
 * desktop apis 的"约定式"路径（如 task/{id}/handle、conversation/list/paging），
 * 其中 13 条在后端根本不存在，实跑必 404——该测试把端点与真实路由表钉死。
 */

// 白名单：oa4rust 后端实际注册、且移动端允许调用的路由（占位符与后端一致）。
const REGISTERED_BACKEND_ROUTES: string[] = [
  // 认证（auth crate）
  '/jaxrs/authentication/login',
  '/jaxrs/authentication/logout',
  '/jaxrs/authentication/who',
  '/jaxrs/authentication/refresh',
  '/jaxrs/authentication/captcha',
  // 流程 surface 列表（processplatform_assemble_surface/routes.rs，GET/POST 见后端注册）
  '/jaxrs/processplatform/assemble/surface/task/list/my/paging/{page}/size/{size}',
  '/jaxrs/processplatform/assemble/surface/taskcompleted/list/my/paging/{page}/size/{size}',
  '/jaxrs/processplatform/assemble/surface/work/list/my/paging/{page}/size/{size}',
  // 流程引擎审批（processplatform_service_processing/routes.rs，桌面 E2E 实跑通过）
  '/jaxrs/task/{id}/complete',
  '/jaxrs/task/{id}/reject',
  // IM 会话（message_assemble_communicate/routes.rs）
  '/jaxrs/message/assemble/communicate/im/conversation/list/my',
  '/jaxrs/message/assemble/communicate/im/msg/list/{page}/size/{size}',
  '/jaxrs/message/assemble/communicate/im/msg',
  '/jaxrs/message/assemble/communicate/im/conversation/{id}/read',
  // 文件（file_assemble_control/routes.rs）
  '/jaxrs/file/assemble/control/file/list/{id}',
  '/jaxrs/file/assemble/control/file/{id}/download',
  // 组织（organization_assemble_control u2 路由）
  '/jaxrs/organization/assemble/control/person/list/like/mockputtopost',
  '/jaxrs/organization/assemble/control/person/{flag}',
  // 通用（general crate）
  '/jaxrs/general/dict/list',
  // IM 发起单聊（message_assemble_communicate，创建 single 会话）
  '/jaxrs/message/assemble/communicate/im/conversation',
  // 流程发起（processplatform designer + service_processing，桌面 ProcessWork 同源端点）
  '/jaxrs/processplatform/assemble/designer/list/{category}',
  '/jaxrs/processplatform/assemble/designer/get/{id}',
  '/jaxrs/form/{id}',
  '/jaxrs/processplatform/service/processing/work',
  '/jaxrs/processplatform/service/processing/data/work/{id}',
  // 考勤本人打卡（attendance_assemble_control v2 mobile）
  '/jaxrs/attendance/assemble/control/v2/mobile/check/pre',
  '/jaxrs/attendance/assemble/control/v2/mobile/check',
  // 附件二进制上传（file_assemble_control attachment folder，落点 FILE_FILE）
  '/jaxrs/attachment/upload/folder/{folderId}',
  // 附件存储 FILE_FILE 列表 / 下载（P4：上传落点在此，非 x_file 我的文件）
  '/jaxrs/attachment/list/editor/{owner}',
  '/jaxrs/attachment/{id}/download',
]

const mobileSrcRoot = resolve(import.meta.dirname, '../../apps/mobile/src')

function collectMobileEndpointPaths(): string[] {
  const files: string[] = []
  function walk(dir: string): void {
    for (const name of readdirSync(dir)) {
      const full = resolve(dir, name)
      const st = statSync(full)
      if (st.isDirectory()) walk(full)
      else if (/\.(ts|vue)$/.test(name)) files.push(full)
    }
  }
  walk(mobileSrcRoot)
  const paths = new Set<string>()
  for (const file of files) {
    const text = readFileSync(file, 'utf8')
    for (const m of text.matchAll(/\/jaxrs\/[A-Za-z0-9_\-/{}$.]+/g)) {
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
  it('every /jaxrs path referenced by mobile source hits a registered backend route', () => {
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
      '/jaxrs/task/',
      'message/assemble/communicate/im',
      'file/assemble/control',
      'organization/assemble/control',
      'general/dict',
      'processplatform/service/processing',
      'attendance/assemble/control/v2',
      '/jaxrs/attachment/upload/folder/',
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
