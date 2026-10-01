// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'index.ts'), 'utf8')
const value = 'value /?#'
const encoded = 'value%20%2F%3F%23'
const expectedImplementations = [
  `api.post(\`/api/query/service/neural/generate/\${encodeURIComponent(model_flag)}\`, body)`,
  `api.get(\`/api/person/empower/\${encodeURIComponent(id)}\`)`,
  `api.get(\`/api/person/empower/\${encodeURIComponent(id)}/enable\`)`,
  `api.get(\`/api/person/empower/\${encodeURIComponent(id)}/disable\`)`,
  `api.post(\`/api/person/empower/\${encodeURIComponent(id)}/enable\`, body)`,
  `api.post(\`/api/person/empower/\${encodeURIComponent(id)}/disable\`, body)`,
  `\`/api/person/empower/manager/list/paging/\${encodeURIComponent(page)}/size/\${encodeURIComponent(size)}\``,
  `api.get(\`/ws/realtime/room/\${encodeURIComponent(room_id)}\`)`,
  `api.get(\`/ws/realtime/room/\${encodeURIComponent(room_id)}/stats\`)`,
  `api.get(\`/api/base/sysresource/filePath/\${encodeURIComponent(filePath)}\`)`,
  `api.get(\`/api/base/fireschedule/classname/\${encodeURIComponent(className)}\`)`,
]

function encodePath(prefix: string, ...parameters: string[]): string {
  return prefix + parameters.map((parameter) => encodeURIComponent(parameter)).join('/')
}

const jpushRoute = '/api/jpush_assemble_control/device/list/'

function assembleDeviceListPath(pushType: string): string {
  return `${jpushRoute}${encodeURIComponent(pushType)}`
}

describe('jpushApi', () => {
  it('includes and encodes the push type required by the assemble device-list route', () => {
    expect(assembleDeviceListPath('jpush')).toBe(`${jpushRoute}jpush`)
    expect(assembleDeviceListPath('vendor/mobile push')).toBe(`${jpushRoute}vendor%2Fmobile%20push`)
    expect(source).toContain('assembleDeviceList: (pushType: string) =>')
    expect(source).toContain('api.get(`/api/jpush_assemble_control/device/list/${encodeURIComponent(pushType)}`)')
    expect(source).not.toContain("assembleDeviceList: () => api.get('/api/jpush_assemble_control/device/list')")
  })
})

describe('API path parameters', () => {
  it('encodes values that could otherwise change route segments', () => {
    expect(encodePath('/route/', value)).toBe(`/route/${encoded}`)
    expect(encodePath('/paging/', value, value)).toBe(`/paging/${encoded}/${encoded}`)
  })

  it('interpolates every affected parameter at its request call site', () => {
    for (const implementation of expectedImplementations) expect(source).toContain(implementation)
  })

  it('does not leave named path parameters in request URL literals', () => {
    expect(source).not.toMatch(/api\.(?:get|post|put|delete)\(['"][^'"]*:[A-Za-z_]/)
  })

  it('uses the backend realtime route without a legacy or double-slash prefix', () => {
    expect(source).toContain("getrealtime: () => api.get('/ws/realtime')")
    expect(source).not.toContain('/api//ws/realtime')
  })
})
