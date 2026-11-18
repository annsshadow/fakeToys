// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment jsdom

/**
 * R207: contract tests for the global "search (⌘K)" shortcut advertised by
 * AppShell's topbar. The advertised shortcut must work when focus is anywhere
 * in the app, and must never steal a view-local search box that uses the same
 * .search-input class.
 */
import { afterEach, describe, expect, test } from 'vitest'
import { installGlobalSearchShortcut } from './globalSearchShortcut'

const MARKED = '<input data-oa-global-search />'

function dispatchShortcut(init: KeyboardEventInit = {}): KeyboardEvent {
  const event = new KeyboardEvent('keydown', {
    key: 'k',
    bubbles: true,
    cancelable: true,
    ...init,
  })
  document.body.dispatchEvent(event)
  return event
}

afterEach(() => {
  document.body.innerHTML = ''
})

describe('installGlobalSearchShortcut (R207 RED → GREEN)', () => {
  test('Ctrl+K focuses the marked global search input from anywhere in the app', () => {
    const dispose = installGlobalSearchShortcut()
    document.body.innerHTML = MARKED
    const globalInput = document.querySelector<HTMLInputElement>('[data-oa-global-search]')!
    const other = document.createElement('button')
    document.body.appendChild(other)
    other.focus()
    expect(document.activeElement).toBe(other)

    const event = dispatchShortcut({ ctrlKey: true })

    expect(event.defaultPrevented).toBe(true)
    expect(document.activeElement).toBe(globalInput)
    dispose()
  })

  test('Meta+K (⌘K) focuses the marked global search input', () => {
    const dispose = installGlobalSearchShortcut()
    document.body.innerHTML = MARKED
    const event = dispatchShortcut({ metaKey: true })
    expect(event.defaultPrevented).toBe(true)
    expect(document.activeElement).toBe(document.querySelector('[data-oa-global-search]'))
    dispose()
  })

  test('view-local .search-input without the marker is not focused', () => {
    const dispose = installGlobalSearchShortcut()
    document.body.innerHTML = `<input class="search-input" />${MARKED}`
    const localInput = document.querySelector<HTMLInputElement>('.search-input')!
    localInput.focus()

    dispatchShortcut({ ctrlKey: true })

    expect(document.activeElement).toBe(document.querySelector('[data-oa-global-search]'))
    dispose()
  })

  test('plain k and other modifier variants are left to the app and their default action', () => {
    const dispose = installGlobalSearchShortcut()
    document.body.innerHTML = MARKED
    for (const init of [{}, { shiftKey: true }, { altKey: true }, { ctrlKey: true, altKey: true }]) {
      const event = dispatchShortcut(init)
      expect(event.defaultPrevented).toBe(false)
      expect(document.activeElement).toBe(document.body)
    }
    dispose()
  })

  test('an input mounted after installation is still reachable by the shortcut', () => {
    const dispose = installGlobalSearchShortcut()
    document.body.innerHTML = ''
    // AppShell is a route component mounted after install(), so lookup must
    // happen at event time rather than at installation time.
    document.body.innerHTML = MARKED
    dispatchShortcut({ ctrlKey: true })
    expect(document.activeElement).toBe(document.querySelector('[data-oa-global-search]'))
    dispose()
  })

  test('dispose removes the listener without leaving a global side effect', () => {
    const dispose = installGlobalSearchShortcut()
    document.body.innerHTML = MARKED
    dispose()

    const event = dispatchShortcut({ ctrlKey: true })

    expect(event.defaultPrevented).toBe(false)
    expect(document.activeElement).toBe(document.body)
  })
})
