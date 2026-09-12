export const O2_ACTIVITY_TYPES = [
  'begin',
  'manual',
  'choice',
  'condition',
  'split',
  'merge',
  'embed',
  'publish',
  'delay',
  'invoke',
  'service',
  'agent',
  'cancel',
  'parallel',
  'end',
] as const

export type O2ActivityType = (typeof O2_ACTIVITY_TYPES)[number]
export type JsonObject = Record<string, unknown>

export interface ProcessCanvasNode extends JsonObject {
  id: string
  type: string
  x: number
  y: number
  label?: string
  w?: number
  h?: number
}

export interface ProcessCanvasEdge extends JsonObject {
  id: string
  from: string
  to: string
  label?: string
  condition?: string
}

export interface ProcessCanvasDefinition {
  nodes: ProcessCanvasNode[]
  edges: ProcessCanvasEdge[]
}

const LIST_KEY: Record<O2ActivityType, string> = {
  begin: 'begin',
  manual: 'manualList',
  choice: 'choiceList',
  condition: 'conditionList',
  split: 'splitList',
  merge: 'mergeList',
  embed: 'embedList',
  publish: 'publishList',
  delay: 'delayList',
  invoke: 'invokeList',
  service: 'serviceList',
  agent: 'agentList',
  cancel: 'cancelList',
  parallel: 'parallelList',
  end: 'endList',
}

const ACTIVITY_TO_NODE: Record<O2ActivityType, string> = {
  begin: 'start', manual: 'task', choice: 'gate_xor', condition: 'gate_or', split: 'gate_and',
  merge: 'merge', embed: 'subprocess', publish: 'publish', delay: 'timer', invoke: 'invoke',
  service: 'service', agent: 'script', cancel: 'cancel', parallel: 'parallel', end: 'end',
}

const NODE_TO_ACTIVITY: Record<string, O2ActivityType> = {
  start: 'begin', task: 'manual', approval: 'manual', gate_xor: 'choice', gate_or: 'condition',
  gate_and: 'split', subprocess: 'embed', timer: 'delay', script: 'agent',
  parallel: 'parallel', end: 'end', publish: 'publish', invoke: 'invoke', service: 'service',
  cancel: 'cancel', merge: 'merge', split: 'split', choice: 'choice', condition: 'condition',
  manual: 'manual', begin: 'begin', embed: 'embed', delay: 'delay', agent: 'agent',
}

const SCRIPT_KEYS = [
  'beforeArriveScriptText', 'afterArriveScriptText', 'beforeExecuteScriptText',
  'afterExecuteScriptText', 'beforeInquireScriptText', 'afterInquireScriptText',
] as const

function object(value: unknown): JsonObject {
  return value !== null && typeof value === 'object' && !Array.isArray(value) ? (value as JsonObject) : {}
}

function text(value: unknown): string {
  return typeof value === 'string' ? value : ''
}

function position(value: unknown, index: number): { x: number; y: number } {
  if (typeof value === 'string') {
    const [x, y] = value.split(',').map(Number)
    if (Number.isFinite(x) && Number.isFinite(y)) return { x, y }
  }
  return { x: 80 + (index % 4) * 180, y: 80 + Math.floor(index / 4) * 110 }
}

function activityType(node: ProcessCanvasNode): O2ActivityType {
  const explicit = text(node.activityType)
  return O2_ACTIVITY_TYPES.includes(explicit as O2ActivityType)
    ? (explicit as O2ActivityType)
    : NODE_TO_ACTIVITY[node.type] ?? 'manual'
}

function activityArrays(definition: JsonObject): JsonObject[] {
  const activities = Array.isArray(definition.activities) ? definition.activities.map(object) : []
  if (activities.length) return activities
  const result: JsonObject[] = []
  for (const type of O2_ACTIVITY_TYPES) {
    const value = definition[LIST_KEY[type]]
    if (type === 'begin') {
      if (value) result.push({ ...object(value), type })
    } else if (Array.isArray(value)) {
      result.push(...value.map((item) => ({ ...object(item), type })))
    }
  }
  return result
}

export function parseProcessDefinition(input: unknown): {
  canvas: ProcessCanvasDefinition
  definition: JsonObject
} {
  const envelope = object(input)
  const definition = object(envelope.processDefinition ?? envelope.process_definition ?? envelope.definition ?? envelope)
  const activities = activityArrays(definition)
  const nodes = activities.map((activity, index) => {
    const type = text(activity.type) as O2ActivityType
    const pos = position(activity.position, index)
    const eventScripts = Object.fromEntries(SCRIPT_KEYS.map((key) => [key, text(activity[key])]))
    return {
      ...activity,
      id: text(activity.id) || `activity-${index + 1}`,
      type: ACTIVITY_TO_NODE[type] ?? type ?? 'task',
      activityType: type || 'manual',
      label: text(activity.name) || text(activity.label) || type,
      x: pos.x,
      y: pos.y,
      w: Number(activity.width) || undefined,
      h: Number(activity.height) || undefined,
      assignee: text(activity.assignee) || (Array.isArray(activity.taskIdentityList) ? activity.taskIdentityList.join(',') : ''),
      script: text(activity.scriptText) || text(activity.afterExecuteScriptText),
      eventScripts,
    } satisfies ProcessCanvasNode
  })
  const routes = Array.isArray(definition.routeList)
    ? definition.routeList.map(object)
    : Array.isArray(definition.routes) ? definition.routes.map(object) : []
  const owner = new Map<string, string>()
  for (const activity of activities) {
    const ids = Array.isArray(activity.routeList) ? activity.routeList : activity.route ? [activity.route] : []
    for (const id of ids) owner.set(String(id), text(activity.id))
  }
  const edges = routes.map((route, index) => ({
    ...route,
    id: text(route.id) || `route-${index + 1}`,
    from: text(route.fromActivity) || text(route.from) || owner.get(text(route.id)) || '',
    to: text(route.activity) || text(route.to),
    label: text(route.name) || text(route.label),
    condition: text(route.scriptText) || text(route.condition),
    waypoints: Array.isArray(route.waypoints) ? route.waypoints : [],
  }))
  return { canvas: { nodes, edges }, definition }
}

export interface SerializeProcessOptions {
  id?: string
  name: string
  alias?: string
  description?: string
  application?: string
  edition?: string
  fieldPermissions?: unknown[]
  base?: JsonObject
  routeWaypoints?: Record<string, unknown[]>
}

export function serializeProcessDefinition(
  canvas: ProcessCanvasDefinition,
  options: SerializeProcessOptions,
): JsonObject {
  const definition: JsonObject = {
    ...(options.base ?? {}),
    id: options.id ?? text(options.base?.id),
    name: options.name,
    alias: options.alias ?? '',
    description: options.description ?? '',
    application: options.application ?? text(options.base?.application),
    edition: options.edition ?? text(options.base?.edition),
    fieldPermissions: options.fieldPermissions ?? options.base?.fieldPermissions ?? [],
  }
  const routesByNode = new Map<string, string[]>()
  for (const edge of canvas.edges) {
    const list = routesByNode.get(edge.from) ?? []
    list.push(edge.id)
    routesByNode.set(edge.from, list)
  }
  const activities = canvas.nodes.map((node) => {
    const type = activityType(node)
    const eventScripts = object(node.eventScripts)
    const activity: JsonObject = {
      ...node,
      ...eventScripts,
      id: node.id,
      process: options.id ?? text(options.base?.id),
      type,
      name: node.label ?? text(node.name) ?? type,
      position: `${Math.round(node.x)},${Math.round(node.y)}`,
      edition: options.edition ?? text(node.edition),
      routeList: routesByNode.get(node.id) ?? [],
      taskIdentityList: Array.isArray(node.taskIdentityList)
        ? node.taskIdentityList
        : text(node.assignee).split(',').map((value) => value.trim()).filter(Boolean),
      readIdentityList: Array.isArray(node.readIdentityList) ? node.readIdentityList : [],
      readUnitList: Array.isArray(node.readUnitList) ? node.readUnitList : [],
      reviewIdentityList: Array.isArray(node.reviewIdentityList) ? node.reviewIdentityList : [],
      reviewUnitList: Array.isArray(node.reviewUnitList) ? node.reviewUnitList : [],
    }
    if (node.script) {
      activity.scriptText = node.script
      activity.afterExecuteScriptText = node.script
    }
    if ((routesByNode.get(node.id)?.length ?? 0) === 1 && !['manual', 'choice', 'condition', 'parallel'].includes(type)) {
      activity.route = routesByNode.get(node.id)?.[0]
    }
    delete activity.x
    delete activity.y
    delete activity.w
    delete activity.h
    delete activity.label
    delete activity.activityType
    delete activity.eventScripts
    delete activity.assignee
    return activity
  })
  const nodeById = new Map(canvas.nodes.map((node) => [node.id, node]))
  const routeList = canvas.edges.map((edge) => ({
    ...edge,
    id: edge.id,
    process: options.id ?? text(options.base?.id),
    fromActivity: edge.from,
    activity: edge.to,
    activityType: nodeById.has(edge.to) ? activityType(nodeById.get(edge.to) as ProcessCanvasNode) : '',
    name: edge.label ?? text(edge.name),
    scriptText: edge.condition ?? text(edge.scriptText),
    waypoints: options.routeWaypoints?.[edge.id] ?? edge.waypoints ?? [],
  }))
  definition.activities = activities
  definition.routes = routeList
  definition.routeList = routeList
  for (const type of O2_ACTIVITY_TYPES) {
    const matches = activities.filter((activity) => activity.type === type)
    definition[LIST_KEY[type]] = type === 'begin' ? (matches[0] ?? null) : matches
  }
  return definition
}

export function processCreatePayload(definition: JsonObject): JsonObject {
  return {
    name: text(definition.name),
    description: JSON.stringify(definition),
    category: text(definition.application) || text(definition.processCategory) || 'all',
  }
}
