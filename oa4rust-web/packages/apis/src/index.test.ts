import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'index.ts'), 'utf8')
const route = '/jaxrs/jpush_assemble_control/device/list/'

function assembleDeviceListPath(pushType: string): string {
  return `${route}${encodeURIComponent(pushType)}`
}

describe('jpushApi', () => {
  it('includes and encodes the push type required by the assemble device-list route', () => {
    expect(assembleDeviceListPath('jpush')).toBe(`${route}jpush`)
    expect(assembleDeviceListPath('vendor/mobile push')).toBe(`${route}vendor%2Fmobile%20push`)
    expect(source).toContain('assembleDeviceList: (pushType: string) =>')
    expect(source).toContain('api.get(`/jaxrs/jpush_assemble_control/device/list/${encodeURIComponent(pushType)}`)')
    expect(source).not.toContain("assembleDeviceList: () => api.get('/jaxrs/jpush_assemble_control/device/list')")
  })
})
