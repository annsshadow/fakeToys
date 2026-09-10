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

  it('loads templates from the mounted template-list route', () => {
    expect(source).toContain("api.get('/jaxrs/jpush/template/list')")
    expect(source).not.toContain("api.get('/jaxrs/jpush_assemble_control/template/list')")
    expect(source).toContain('templates.value = r.data ?? []')
  })

  it('does not issue a GET request to the POST-only device creation endpoint', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush/core/entity/device/create')")
    expect(source).not.toContain("queryKey: ['core_entity_device_create']")
  })

  it('does not issue GET requests to the POST-only device binding aliases', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush/assemble/control/device/bind')")
    expect(source).not.toContain("api.get('/jaxrs/jpush_assemble_control/device/bind')")
    expect(source).not.toContain("queryKey: ['assemble_control_device_bind']")
    expect(source).not.toContain("'/jaxrs/jpush_assemble_control/device/bind']")
  })

  it('does not issue a GET request to the POST-only message send endpoint', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush_assemble_control/message/send')")
    expect(source).not.toContain("'/jaxrs/jpush_assemble_control/message/send']")
  })

  it('does not auto-run test sends through either route alias', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush_assemble_control/message/test/send')")
    expect(source).not.toContain("api.get('/jaxrs/jpush/assemble/control/message/test/send')")
    expect(source).not.toContain("queryKey: ['jpush_assemble_control_message_test_send']")
    expect(source).not.toContain("queryKey: ['control_message_test_send']")
  })

  it('does not auto-run destructive admin unbind requests', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush_assemble_control/device/admin/unbind/all/person')")
    expect(source).not.toContain("api.get('/jaxrs/jpush/assemble/control/device/admin/unbind/all/person')")
    expect(source).not.toContain("'/jaxrs/jpush_assemble_control/device/admin/unbind/all/person']")
    expect(source).not.toContain("'/jaxrs/jpush/assemble/control/device/admin/unbind/all/person']")
  })
  it('does not auto-run device unbind with placeholder identifiers', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush_assemble_control/device/unbind/a/b')")
    expect(source).not.toContain("'/jaxrs/jpush_assemble_control/device/unbind/a/b']")
  })
  it('does not auto-run legacy unbind aliases with placeholder names', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush_assemble_control/device/unbind/deviceName/deviceType')")
    expect(source).not.toContain("api.get('/jaxrs/jpush/assemble/control/device/unbind/deviceName/deviceType')")
  })
  it('does not auto-run new unbind with placeholder identifiers', () => {
    expect(source).not.toContain(
      "api.get('/jaxrs/jpush/assemble/control/device/unbind/new/deviceName/deviceType/pushType')",
    )
    expect(source).not.toContain("'/jaxrs/jpush/assemble/control/device/unbind/new/deviceName/deviceType/pushType'")
  })
  it('does not query the unmounted assemble send path', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush/assemble/send')")
    expect(source).not.toContain("queryKey: ['jpush_assemble_send']")
  })
  it('does not query the POST-only device create route', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush/device/create')")
    expect(source).not.toContain("queryKey: ['jpush_device_create']")
  })
  it('does not query the unmounted jpush send path', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush/send')")
    expect(source).not.toContain("queryKey: ['jpush_send']")
  })
  it('does not query the unmounted jpush core list path', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush/core/list')")
    expect(source).not.toContain("queryKey: ['jpush_core_list']")
  })
  it('does not query the unmounted jpush create path', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush/create')")
    expect(source).not.toContain("queryKey: ['jpush_create']")
  })
  it('does not query an unmounted hard-coded jpush detail', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush/get/jpush-001')")
    expect(source).not.toContain("queryKey: ['jpush_get_jpush_001']")
  })
  it('does not query the unmounted jpush list path', () => {
    expect(source).not.toContain("api.get('/jaxrs/jpush/list')")
    expect(source).not.toContain("queryKey: ['jpush_list']")
  })
})
