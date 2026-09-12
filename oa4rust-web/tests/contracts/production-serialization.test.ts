import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const views = resolve(import.meta.dirname, '../../apps/desktop/src/views')
const readView = (name: string) => readFileSync(resolve(views, name), 'utf8')

describe('S3 production serializers use the pinned contracts', () => {
  it('serializes ProcessDesigner output as the engine activities contract', () => {
    const source = readView('ProcessDesigner.vue')

    expect(source).toContain('processDefinition:')
    expect(source).toContain('activities:')
    expect(source).toContain('fieldPermissions:')
    expect(source).toContain('waypoints:')
    expect(source).not.toContain('config: processDef.value')
  })

  it('serializes FormDesigner output as O2OA moduleList instead of the legacy fields array', () => {
    const source = readView('FormDesigner.vue')

    expect(source).toContain('moduleList:')
    expect(source).toContain('fieldList:')
    expect(source).not.toContain('fields: currentForm.value.fields.map')
  })

  it('sends the QueryDesigner SQL definition in the backend query field', () => {
    const source = readView('QueryDesigner.vue')

    expect(source).toContain('query: mform.value.sql')
    expect(source).not.toContain('sql: mform.value.sql')
  })

  it('nests PortalDesigner flag and layout in backend page content', () => {
    const source = readView('PortalDesigner.vue')

    expect(source).toContain('content: {')
    expect(source).toContain('layout: JSON.parse(form.value.layout)')
  })
})
