import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'JPushApp.vue'), 'utf8')

describe('JPushApp contracts', () => {
  it('loads the visible device list from the mounted jpush route', () => {
    expect(source).toContain("api.get('/jaxrs/jpush_assemble_control/device/list/jpush')")
    expect(source).not.toContain("api.get('/jaxrs/jpush_assemble_control/device/list')")
    expect(source).toMatch(/loadDevices\(\)[\s\S]*loadTemplates\(\)/)
    expect(source).toContain('devices.value = r.data ?? []')
  })
})
