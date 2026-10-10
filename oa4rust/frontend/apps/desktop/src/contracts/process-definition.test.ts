// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import {
  O2_ACTIVITY_TYPES,
  activityName,
  parseProcessDefinition,
  processCreatePayload,
  serializeProcessDefinition,
  type ProcessCanvasNode,
} from './process-definition'

describe('O2OA process definition adapter', () => {
  it('round-trips all required activity types, routes, permissions, edition and scripts', () => {
    const nodes = O2_ACTIVITY_TYPES.map((type, index) => ({
      id: `n${index}`,
      type,
      activityType: type,
      label: type,
      x: index * 10,
      y: index * 20,
      eventScripts: { beforeExecuteScriptText: `before-${type}` },
      script: type === 'agent' ? 'return true' : '',
    }))
    const edges = nodes.slice(0, -1).map((node, index) => ({
      id: `r${index}`,
      from: node.id,
      to: nodes[index + 1].id,
      condition: `route-${index}`,
      waypoints: [{ x: index, y: index + 1 }],
    }))
    const definition = serializeProcessDefinition(
      { nodes, edges },
      {
        id: 'process-1',
        name: '审批流程',
        application: 'app-1',
        edition: 'v2',
        fieldPermissions: [{ path: 'amount', readActivityList: ['n1'] }],
      },
    )

    expect((definition.activities as Array<Record<string, unknown>>).map((item) => item.type)).toEqual(
      O2_ACTIVITY_TYPES,
    )
    expect(definition.routeList).toHaveLength(edges.length)
    expect(definition.fieldPermissions).toEqual([{ path: 'amount', readActivityList: ['n1'] }])
    expect(definition.edition).toBe('v2')
    expect((definition.agentList as Array<Record<string, unknown>>)[0].afterExecuteScriptText).toBe('return true')

    const parsed = parseProcessDefinition({ processDefinition: definition })
    expect(parsed.canvas.nodes).toHaveLength(nodes.length)
    expect(parsed.canvas.edges[0]).toMatchObject({ from: 'n0', to: 'n1', condition: 'route-0' })
    expect(parsed.canvas.edges[0].waypoints).toEqual([{ x: 0, y: 1 }])
  })

  describe('activityName 级联（修复：无 label/name 时活动名不得序列化为空）', () => {
    const base: ProcessCanvasNode = { id: 'n1', type: 'manual', x: 0, y: 0 }

    it('既无 label 又无 name 时回退到活动类型（回归：原 ?? 兜底为死代码致空名）', () => {
      expect(activityName(base, 'manual')).toBe('manual')
    })

    it('非空 label 优先于 name 与 type', () => {
      expect(activityName({ ...base, label: '人工审批' }, 'manual')).toBe('人工审批')
      expect(activityName({ ...base, label: '人工审批', name: 'N' }, 'manual')).toBe('人工审批')
    })

    it('label 缺省时用非空 name', () => {
      expect(activityName({ ...base, name: 'N' }, 'manual')).toBe('N')
    })

    it('空 label 也应回退（label="" 不得压住 name/type）', () => {
      expect(activityName({ ...base, label: '', name: 'N' }, 'manual')).toBe('N')
      expect(activityName({ ...base, label: '' }, 'manual')).toBe('manual')
    })
  })

  it('serializeProcessDefinition：无 label/name 的节点序列化为 type 而非空名', () => {
    const definition = serializeProcessDefinition(
      {
        nodes: [
          { id: 'n0', type: 'manual', x: 0, y: 0 }, // 无 label、无 name
        ],
        edges: [],
      },
      { id: 'process-x', name: 'P', application: 'app-1' },
    )
    const activities = definition.activities as Array<Record<string, unknown>>
    expect(activities[0].name).toBe('manual') // 修复前为 ''
    expect(activities[0].type).toBe('manual')
  })

  it('processCreatePayload falls back through application → processCategory → "all"', () => {
    // 创建 payload 的 category 决定流程在门户里的归类；三级回退避免空 category。
    const payload = processCreatePayload({ name: 'P', application: 'app-9' })
    expect(payload).toEqual({
      name: 'P',
      description: JSON.stringify({ name: 'P', application: 'app-9' }),
      category: 'app-9',
    })
    expect(processCreatePayload({ name: 'P', processCategory: 'hr' }).category).toBe('hr')
    expect(processCreatePayload({ name: 'P' }).category).toBe('all')
  })

  it('parses legacy per-type list keys (begin/manualList) and assigns default grid positions', () => {
    // 旧版 O2OA 定义没有统一 activities 数组，按类型分列（*List 键）存储；
    // 无 position 字段的活动必须落到默认网格位而不是 0,0 堆叠。
    const parsed = parseProcessDefinition({
      begin: { id: 'b', type: 'begin', name: '开始' },
      manualList: [{ id: 't1', type: 'manual', name: '审批' }],
    })
    const byId = Object.fromEntries(parsed.canvas.nodes.map((node) => [node.id, node]))
    expect(byId.b).toMatchObject({ x: 80, y: 80, activityType: 'begin' })
    expect(byId.t1).toMatchObject({ x: 260, y: 80, activityType: 'manual' })
  })
})
