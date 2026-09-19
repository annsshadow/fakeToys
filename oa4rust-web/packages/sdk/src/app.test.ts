// @vitest-environment node
import { describe, expect, it } from 'vitest'
import { getO2App, getO2Router } from './app'

describe('app accessors before createO2App()', () => {
  it('getO2App throws a clear error when the app has not been created yet', () => {
    expect(() => getO2App()).toThrow('createO2App() not called yet')
  })

  it('getO2Router throws a clear error when the app has not been created yet', () => {
    expect(() => getO2Router()).toThrow('createO2App() not called yet')
  })
})
