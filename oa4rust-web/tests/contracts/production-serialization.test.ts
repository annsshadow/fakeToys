import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const appDir = resolve(import.meta.dirname, '../../apps/desktop/src')
const views = resolve(appDir, 'views')
const contracts = resolve(appDir, 'contracts')
const readView = (name: string) => readFileSync(resolve(views, name), 'utf8')
const readContract = (name: string) => readFileSync(resolve(contracts, name), 'utf8')

// S3 守卫：生产视图必须通过契约层序列化（contracts/*），不允许回退到
// 旧的内联自定义 schema。深行为由 contracts/*.test.ts 单测覆盖，这里钉
// 视图→契约的接线与旧形状的消失。
describe('S3 production serializers use the pinned contracts', () => {
  it('serializes ProcessDesigner output as the engine activities contract', () => {
    const source = readView('ProcessDesigner.vue')
    const contract = readContract('process-definition.ts')

    // 视图只负责画布，落库形状由契约层产出：O2OA activities + 字段权限 + 路径点
    expect(source).toContain('serializeProcessDefinition(')
    expect(contract).toContain('definition.activities = activities')
    expect(contract).toContain('fieldPermissions:')
    expect(contract).toContain('waypoints:')
    // 旧的自定义 {config:{nodes,edges}} 信封与演示兜底必须消失（W3）
    expect(source).not.toContain('config: processDef.value')
    expect(source).not.toContain('__fakeProcesses')
  })

  it('serializes FormDesigner output as O2OA moduleList via the xform contract', () => {
    const source = readView('FormDesigner.vue')
    const contract = readContract('xform.ts')

    expect(source).toContain('serializeFormDefinition(')
    expect(source).toContain('formSavePayload(')
    expect(contract).toContain('moduleList:')
    // 保存路径必须走契约序列化（导出工具保留 fields 数组属合法交换格式）
    const saveStart = source.indexOf('async function saveForm()')
    expect(saveStart).toBeGreaterThan(-1)
    const saveBody = source.slice(saveStart, source.indexOf('\nfunction ', saveStart))
    expect(saveBody).not.toContain('fields: currentForm.value.fields.map')
  })

  it('sends the QueryDesigner SQL definition in the backend query field', () => {
    const source = readView('QueryDesigner.vue')

    // 后端 create/save 的 CreateDesignerRequest 只读 query 字段，
    // 发 sql 会被静默丢弃
    expect(source).toContain('query: mform.value.sql')
    expect(source).not.toContain('sql: mform.value.sql')
  })

  it('nests PortalDesigner widgets in the backend page content field', () => {
    const source = readView('PortalDesigner.vue')
    const contract = readContract('designer.ts')

    expect(source).toContain('content: serializePortalContent(widgets.value)')
    expect(source).toContain('designerPaths.portalSave(')
    expect(contract).toContain('serializePortalContent')
    expect(contract).toContain('parsePortalContent')
  })
})
