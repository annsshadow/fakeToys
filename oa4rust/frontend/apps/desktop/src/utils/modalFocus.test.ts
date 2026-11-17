// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom

import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installViewModalManager } from './modalFocus'

/** Let MutationObserver deliver its records in jsdom. */
const flush = () => new Promise<void>((resolve) => setTimeout(resolve, 0))

/** Build the same hand-written modal shape used by the 44 CRUD views. */
function buildModal() {
  document.body.innerHTML = ''
  const opener = document.createElement('button')
  opener.textContent = '新建'
  document.body.appendChild(opener)
  opener.focus()

  const overlay = document.createElement('div')
  overlay.className = 'modal-overlay'
  const box = document.createElement('div')
  box.className = 'modal'
  const input = document.createElement('input')
  const cancel = document.createElement('button')
  cancel.textContent = '取消'
  const save = document.createElement('button')
  save.textContent = '保存'
  box.append(input, cancel, save)
  overlay.appendChild(box)
  document.body.appendChild(overlay)

  let closeCount = 0
  // Mirrors Vue's @click.self="closeModal" in the real views.
  overlay.addEventListener('click', (e) => {
    if (e.target === overlay) {
      closeCount += 1
      overlay.remove()
    }
  })
  return { opener, overlay, input, cancel, save, getCloseCount: () => closeCount }
}

describe('view modal keyboard and focus manager (R162)', () => {
  let dispose: () => void = () => {}

  beforeEach(() => {
    dispose = installViewModalManager()
  })

  afterEach(() => {
    dispose()
    document.body.innerHTML = ''
  })

  it('Escape invokes the overlay @click.self handler, closes the modal and restores opener focus', async () => {
    const { opener, overlay, input, getCloseCount } = buildModal()
    await flush()

    // Accessibility: screen readers must see it as a dialog.
    expect(overlay.getAttribute('role')).toBe('dialog')
    expect(overlay.getAttribute('aria-modal')).toBe('true')
    // Opening a modal moves keyboard focus inside it.
    expect(document.activeElement).toBe(input)

    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }))
    expect(getCloseCount()).toBe(1)
    expect(document.body.contains(overlay)).toBe(false)

    await flush()
    expect(document.activeElement).toBe(opener)
  })

  it('traps Tab forward and backward inside the topmost modal', async () => {
    const { overlay, input, save } = buildModal()
    await flush()
    expect(document.activeElement).toBe(input)

    save.focus()
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Tab', bubbles: true }))
    expect(document.activeElement).toBe(input)

    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Tab', bubbles: true, shiftKey: true }))
    expect(document.activeElement).toBe(save)

    expect(overlay.contains(document.activeElement)).toBe(true)
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Tab', bubbles: true, shiftKey: true }))
    expect(overlay.contains(document.activeElement)).toBe(true)
  })

  it('pulls focus back when a background control is focused while a modal is open', async () => {
    const { overlay, input } = buildModal()
    await flush()

    const background = document.createElement('button')
    document.body.appendChild(background)
    background.focus()

    expect(document.activeElement).toBe(input)
    expect(overlay.contains(document.activeElement)).toBe(true)
  })

  it('does not interfere with the confirmMsg overlay, which manages its own focus', async () => {
    const { overlay } = buildModal()
    await flush()

    const confirmOverlay = document.createElement('div')
    confirmOverlay.id = 'oa4-confirm-overlay'
    const confirmBtn = document.createElement('button')
    confirmOverlay.appendChild(confirmBtn)
    document.body.appendChild(confirmOverlay)
    await flush()

    confirmBtn.focus()
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Tab', bubbles: true }))
    expect(document.activeElement).toBe(confirmBtn)
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }))
    expect(document.body.contains(overlay)).toBe(true)
  })

  it('keeps keyboard focus in the topmost modal when several are open', async () => {
    const first = buildModal()
    await flush()

    // Stack a second overlay without clearing the first (buildModal resets body).
    const second = document.createElement('div')
    second.className = 'modal-overlay'
    const secondInput = document.createElement('input')
    second.appendChild(secondInput)
    document.body.appendChild(second)
    let secondCloses = 0
    second.addEventListener('click', (e) => {
      if (e.target === second) {
        secondCloses += 1
        second.remove()
      }
    })
    await flush()

    expect(second.contains(document.activeElement)).toBe(true)
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Tab', bubbles: true }))
    expect(second.contains(document.activeElement)).toBe(true)

    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }))
    expect(secondCloses).toBe(1)
    await flush()
    expect(document.body.contains(second)).toBe(false)
    expect(first.overlay.contains(document.activeElement)).toBe(true)

    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }))
    await flush()
    expect(document.body.contains(first.overlay)).toBe(false)
  })

  it('stops managing modals after dispose', async () => {
    dispose()
    const { getCloseCount } = buildModal()
    await flush()

    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }))
    expect(getCloseCount()).toBe(0)
  })
})
