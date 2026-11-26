// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R301 工作台实质升级契约（C4）：针对 t11 findings 的信息层级与数据可信度。
 * 硬约束：日程/公告不得硬编码假数据；快捷入口必须可见；统计卡须有可视化分布。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'Dashboard.vue'), 'utf8')
const template = source.slice(source.indexOf('<template>'), source.indexOf('</template>'))
const script = source.slice(source.indexOf('<script setup'), source.indexOf('</script>'))

describe('R301 工作台信息层级与数据可视化实质升级', () => {
  it('快捷入口从 quickApps 数据渲染，不再是已定义却隐藏的死代码', () => {
    expect(template, '缺少快捷入口区块').toContain('quick-access-section')
    expect(template, '未用 v-for 渲染 quickApps').toMatch(/v-for="app in quickApps"/)
    expect(template, '缺少快捷入口 testid').toContain('data-testid="dashboard-quick-grid"')
  })

  it('今日日程改为真实接口取数，不再硬编码「部门例会」等假安排', () => {
    expect(template, '仍存在硬编码假日程').not.toContain('部门例会')
    expect(template, '仍存在硬编码假日程').not.toContain('项目评审')
    expect(template, '缺少日程空态引导').toContain('暂无日程安排')
    expect(script, '日程未调用真实接口').toMatch(/api\.get\(['\"]\/api\/calendar\/event\/list\//)
    expect(script, '缺少日程加载函数').toContain('loadTodaySchedule')
  })

  it('通知公告由真实消息数据填充，不再伪造「报销申请已通过」等动态', () => {
    expect(script, '仍存在伪造动态').not.toContain('您的报销申请已通过审批')
    expect(script, '仍存在伪造动态').not.toContain('张三 给您发了一条消息')
    expect(script, '公告未调用真实接口').toMatch(/api\.get\(['\"]\/api\/message\//)
    expect(template, '缺少公告空态').toContain('暂无通知公告')
  })

  it('统计卡带可视化分布条，数据来自同源计数而非固定占位', () => {
    expect(template, '统计卡缺少分布条').toContain('stat-distribution')
    expect(script, '统计卡仍把今日会议写死为 0').not.toMatch(/label:\s*'今日会议',\s*value:\s*0\b/)
    expect(script, '缺少统计分布计算').toContain('statDistribution')
  })

  it('工作台与工作台以外页面共用 ai-page 语义容器与唯一 h1', () => {
    expect(template).toContain('class="dashboard ai-page ai-page-dashboard"')
    expect((template.match(/<h1/g) || []).length).toBe(1)
    expect(template).toContain('data-testid="dashboard-h1"')
  })
})
