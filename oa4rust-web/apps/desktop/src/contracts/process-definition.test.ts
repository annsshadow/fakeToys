import { describe, expect, it } from 'vitest'
import { O2_ACTIVITY_TYPES, parseProcessDefinition, serializeProcessDefinition } from './process-definition'

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
})
