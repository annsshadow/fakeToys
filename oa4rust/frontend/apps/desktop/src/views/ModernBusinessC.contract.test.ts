// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const DQ = String.fromCharCode(34)
const attr = (name: string, value: string) => `${name}=${DQ}${value}${DQ}`
const clickAttr = (h: string) => attr('@click', h)
const here = dirname(fileURLToPath(import.meta.url))
const read = (name: string) => readFileSync(resolve(here, name), 'utf8')
const attendance = read('AttendanceApp.vue')
const form = read('FormApp.vue')
const role = read('RoleManager.vue')
const templateOf = (source: string) => source.slice(0, source.indexOf(`<script setup lang=${DQ}ts${DQ}>`))

const endpointHandlers = [
  'exportData',
  'loadAttOverview',
  'loadAttOrg',
  'loadV2Meta',
  'loadV2Schedule',
  'loadV2AppealRecord',
  'loadAttBase',
  'loadCoreLists',
  'loadAttTwin',
  'loadAttTwin2',
  'loadScheduleDetail',
  'loadV2ConfigTpl',
  'loadStatisticShow',
  'checkMyRestDate',
  'importLeave',
  "importAttV2Records('rows')",
  "importAttV2Records('daily')",
  "attAppealWrite('analyseRedo')",
  "attAppealWrite('appealById')",
  "attAppealWrite('sync')",
  'reciveMobileDetail',
  'myMobileDetail',
  'reciveDetailById',
  'createAttDetail',
  'toggleAttType',
  'loadAppealDetailFilters',
  'loadHolidaySettingDetails',
  'loadAttendanceDetails',
  'loadStatisticAggregates',
  'loadPersonMonthStats',
  "attV2Write('workplaceCreate')",
  "attV2Write('workplaceDelete')",
  "attV2Write('workplaceListIds')",
  "attV2Write('scheduleListFilter')",
  "attV2Write('shiftUpdate')",
  "attV2Write('appealStart')",
  "attV2Write('appealEnd')",
  "attV2Write('detailStat')",
  "attV2Write('selfHolidayDelete')",
  "attV2Write('holidaySimpleDelete')",
  'attStatFilters',
  "attCoreEntity('recordCreate')",
  "attCoreEntity('recordUpdate')",
  "attCoreEntity('recordDelete')",
  "attCoreEntity('ruleCreate')",
  "attCoreEntity('ruleUpdate')",
  "attCoreEntity('ruleDelete')",
  "attWrite3('adminCreate')",
  "attWrite3('adminDelete')",
  "attWrite3('empCreate')",
  "attWrite3('empDelete')",
  "attWrite3('schedCreate')",
  "attWrite3('schedDelete')",
  "attWrite3('workdayCreate')",
  "attWrite3('workdayDelete')",
  "attWrite3('workdayFilter')",
  "attWrite3('reqlogCreate')",
  "attWrite3('reqlogDelete')",
  "attWrite3('cycleDelete')",
  "attWrite3('importDelete')",
  "attWrite3('appealAudit')",
  "attWrite3('appealCheck')",
  "attWrite3('appealDo')",
  "attWrite3('appealArchive')",
  "attWrite3('appealDelete')",
  "attWrite3('detailAnalyse')",
  "attWrite3('detailAnalyseId')",
  "attWrite3('detailArchive')",
  "attWrite3('ruleToggle')",
  "attWrite3('statDo')",
  "attWrite3('v2Config')",
  "attWrite3('v2ConfigPerson')",
  "attWrite3('v2DetailList')",
  "attWrite3('v2AppealMgr')",
  "attWrite3('uuid')",
  "attV2Ops('appealMgrStatus')",
  "attV2Ops('appealResetStatus')",
  "attV2Ops('detailRebuild')",
  "attV2Ops('groupRebuild')",
  "attV2Ops('groupRefresh')",
  "attV2Ops('recordDelete')",
  "attV2Ops('analyse')",
  "attV2Ops('checkCycle')",
  "attV2Ops('filterNext')",
  "attV2Ops('appealArchive')",
  "attV2Ops('mobileDelete')",
]

describe('ModernBusinessC 三页重构契约', () => {
  describe('考勤页', () => {
    it('全部 86 个端点按钮/@click 逐字保留，无删减', () => {
      for (const handler of endpointHandlers) {
        expect(attendance, handler).toContain(clickAttr(handler))
      }
    })

    it('使用约定 modern 类名做 hero 化页头与统计卡', () => {
      const tpl = templateOf(attendance)
      expect(tpl).toMatch(/class="[^"]*modern-page/)
      expect(tpl).toMatch(/class="[^"]*modern-hero/)
      expect(tpl).toContain('modern-section')
      expect(tpl).toContain('modern-stat')
    })

    it('页头不再暴露 API 路径副标题', () => {
      const tpl = templateOf(attendance)
      expect(tpl).not.toContain('/api/attendance/assemble/control')
      expect(tpl).toContain(attr('data-testid', 'attendance-page'))
    })

    it('端点按钮收纳进默认收起的分组高级操作面板', () => {
      const tpl = templateOf(attendance)
      expect(tpl).toContain('class="advanced-operations"')
      expect(tpl).toContain(attr('data-testid', 'advanced-operations-toggle'))
      expect(tpl).toContain(attr('data-testid', 'advanced-operations-panel'))
      expect(attendance).toMatch(/showAdvanced\s*=\s*ref\(false\)/)
      for (const group of [
        '汇总导出',
        '排班配置',
        '打卡记录',
        '申诉假期',
        '统计',
        '管理员与员工配置',
        '导入导出',
        '其它',
      ]) {
        expect(tpl, group).toContain(group)
      }
      const marker = attr('data-testid', 'advanced-operations-panel')
      const panel = tpl.slice(tpl.indexOf(marker))
      for (const handler of endpointHandlers) {
        expect(panel, handler).toContain(clickAttr(handler))
      }
    })
  })

  describe('表单页', () => {
    it('保留全部 tab 与 API', () => {
      const tpl = templateOf(form)
      for (const tab of ['list', 'v2', 'versions', 'templates']) {
        expect(tpl, tab).toContain(attr('data-testid', `form-tab-${tab}`))
      }
      for (const api of ['/api/form/list/all', '/api/form']) {
        expect(form, api).toContain(api)
      }
      expect(tpl).toMatch(/class="[^"]*modern-page/)
      expect(tpl).toContain(attr('data-testid', 'form-page'))
    })

    it('列表标题截断显示并支持 hover title，flag 空行不显示', () => {
      const tpl = templateOf(form)
      expect(tpl).toContain(`:title=${DQ}f.name`)
      expect(form).toMatch(/text-overflow\s*:\s*ellipsis/)
      expect(tpl).toContain(attr('v-if', 'f.flag || f.formFlag'))
    })
  })

  describe('角色页', () => {
    it('移除页面自带 view-header/api 副标题，AdminLayout 标题不重复', () => {
      const tpl = templateOf(role)
      expect(tpl).not.toContain('view-header')
      expect(tpl).not.toContain('/api/role')
      expect((tpl.match(/<h1[\s>]/g) ?? []).length).toBe(0)
      expect((tpl.match(/角色管理/g) ?? []).length).toBe(0)
      expect(tpl).toMatch(/class="[^"]*modern-page/)
      expect(tpl).toContain(attr('data-testid', 'role-page'))
    })

    it('保留刷新/新建/成员/编辑/删除与 HTTP 400 错误态', () => {
      const tpl = templateOf(role)
      for (const testid of ['role-refresh', 'role-create', 'role-members', 'role-edit', 'role-delete']) {
        expect(tpl, testid).toContain(attr('data-testid', testid))
      }
      for (const api of ['/api/role/list', '/api/role', '/api/organization/assemble/control/person/list/role/']) {
        expect(role, api).toContain(api)
      }
      expect(role).toContain(attr('data-testid', 'role-list-error'))
      expect(role).toContain('describeListError')
    })
  })
})
