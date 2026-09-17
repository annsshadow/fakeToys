// @vitest-environment node
import { afterEach, describe, expect, it, vi } from 'vitest'
import { defineWidget, defineWidgets, getAllWidgets, getWidget, loadComponent, renderWidget } from './widget'

const CompA = { name: 'CompA', render: () => null } as never
const CompB = { name: 'CompB', render: () => null } as never

afterEach(() => {
  vi.restoreAllMocks()
})

describe('widget registry', () => {
  it('defineWidget registers and getWidget retrieves a definition', () => {
    defineWidget('w-alpha', CompA, { size: 12 })
    expect(getWidget('w-alpha')).toEqual({ name: 'w-alpha', component: CompA, options: { size: 12 } })
  })

  it('an undefined options is kept as undefined (caller may omit it)', () => {
    defineWidget('w-no-opt', CompA)
    expect(getWidget('w-no-opt')!.options).toBeUndefined()
  })

  it('redefining a widget replaces the previous definition', () => {
    defineWidget('w-swap', CompA)
    defineWidget('w-swap', CompB, { v: 2 })
    expect(getWidget('w-swap')!.component).toBe(CompB)
  })

  it('getWidget returns undefined for an unknown name', () => {
    expect(getWidget('w-does-not-exist')).toBeUndefined()
  })

  it('defineWidgets registers a batch and getAllWidgets reflects it', () => {
    defineWidgets([
      ['w-batch-a', CompA],
      ['w-batch-b', CompB, { x: 1 }],
    ])
    expect(getAllWidgets().map((w) => w.name)).toEqual(expect.arrayContaining(['w-batch-a', 'w-batch-b']))
  })

  it('loadComponent is the defineWidget alias', () => {
    expect(loadComponent).toBe(defineWidget)
  })
})

describe('renderWidget', () => {
  it('an unknown widget logs an error and does not touch the DOM', () => {
    const errorSpy = vi.spyOn(console, 'error').mockImplementation(() => {})
    const container = { innerHTML: 'old' } as unknown as HTMLElement
    renderWidget(container, 'nope')
    expect(errorSpy).toHaveBeenCalledWith('[oa4rust-sdk] Widget "nope" not found')
    expect(container.innerHTML).toBe('old')
  })
})
