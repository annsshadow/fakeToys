// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function src(name: string): string {
  return readFileSync(resolve(import.meta.dirname, name), 'utf8')
}

function count(haystack: string, needle: string): number {
  return haystack.split(needle).length - 1
}

describe('R298 t4 admin restyle - normal management UI, not endpoint dumps', () => {
  it('ServerApp uses standard header and groups dangerous ops under 运维操作 with confirmations', () => {
    const s = src('ServerApp.vue')
    expect(s).toContain('AdminPageHeader')
    expect(s).toContain('运维操作')
    expect(s).toContain("confirmMsg('确定要停止服务器？所有连接将被断开。')")
    expect(s).toContain("confirmMsg('确定清理该类型缓存？')")
    expect(s).toContain("confirmMsg('确定删除该部署？')")
    for (const fn of [
      'loadSysStatus',
      'loadGeneralMeta',
      'loadBaseMeta',
      'loadServerTwin',
      'loadServerTwin2',
      'loadServerTwin3',
      'serverConsoleOps',
      'loadExecuteStatus',
      'executeCommand',
      'stopServer',
    ]) {
      expect(s, 'lost ' + fn).toContain('function ' + fn)
    }
  })

  it('ProgramCenterApp is list + detail drawer with section nav and keeps endpoint coverage', () => {
    const s = src('ProgramCenterApp.vue')
    expect(s).toContain('AdminPageHeader')
    expect(s).toContain('selectedDetail')
    expect(s).toContain('detail-drawer')
    expect(s).toContain('v-if="selectedDetail"')
    expect(s).toContain('admin-section-nav')
    for (const tab of ['agent', 'application', 'script', 'dict', 'market', 'invoke', 'config', 'style']) {
      expect(s, 'lost tab ' + tab).toContain(tab)
    }
    const calls = count(s, 'api.get(') + count(s, 'api.post(') + count(s, 'api.put(') + count(s, 'api.delete(')
    expect(calls, 'ProgramCenterApp endpoint coverage must stay >= 250').toBeGreaterThanOrEqual(250)
  })

  it('ControlPanelApp overview page uses standard header and /app/admin links', () => {
    const s = src('ControlPanelApp.vue')
    expect(s).toContain('AdminPageHeader')
    expect(s).toContain('stat-card')
    const oldPaths = s.match(/path: '\/app\/(?!admin\/)[^']+'/g) ?? []
    expect(oldPaths, 'legacy /app/* paths remain: ' + oldPaths.join(', ')).toEqual([])
    expect(s).not.toContain("to='/app/program'")
  })

  it('DesignCenterApp overview page uses standard header and /app/admin links', () => {
    const s = src('DesignCenterApp.vue')
    expect(s).toContain('AdminPageHeader')
    expect(s).toContain('stat-val')
    const oldHrefs = s.match(/href: '\/app\/(?!admin\/)[^']+'/g) ?? []
    expect(oldHrefs, 'legacy hrefs remain: ' + oldHrefs.join(', ')).toEqual([])
    const oldPaths = s.match(/path: '\/app\/(?!admin\/)[^']+'/g) ?? []
    expect(oldPaths, 'legacy paths remain: ' + oldPaths.join(', ')).toEqual([])
    expect(s).toContain('aria-label="搜索设计器"')
  })

  it('PortalApp no longer dumps 40 endpoint buttons in its header', () => {
    const s = src('PortalApp.vue')
    expect(s).toContain('AdminPageHeader')
    expect(s).toContain('端点操作')
    const header = s.slice(s.indexOf('<template>'), s.indexOf('</AdminPageHeader>'))
    expect(count(header, 'class="new-page-btn ghost"'), 'header still dumps endpoint buttons').toBeLessThan(8)
    for (const fn of [
      'loadPortalList',
      'loadPortalSurface',
      'loadPortalResources',
      'loadPortalDetail',
      'loadPortalSurfaceEntities',
      'loadPortalMobileFacets',
      'loadPortalTwin',
      'designerCreate',
      'designerMisc',
    ]) {
      expect(s, 'lost ' + fn).toContain('function ' + fn)
    }
  })

  it('restructured views retain list error and skeleton affordances', () => {
    for (const f of [
      'PortalApp.vue',
      'ServerApp.vue',
      'ProgramCenterApp.vue',
      'LogViewerApp.vue',
      'DeploymentApp.vue',
      'CommonApp.vue',
      'FileInfoApp.vue',
    ]) {
      const s = src(f)
      expect(s, f + ' lost ListLoadError').toContain('ListLoadError')
      expect(s, f + ' lost skeleton/loading state').toMatch(/skeleton|skel|loading/)
    }
  })
})
