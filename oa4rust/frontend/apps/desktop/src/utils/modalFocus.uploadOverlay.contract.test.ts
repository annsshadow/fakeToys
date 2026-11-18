// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom

import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installViewModalManager } from './modalFocus'

const flush = () => new Promise<void>((resolve) => setTimeout(resolve, 0))

/**
 * FileManager.vue detail and upload dialogs use .upload-overlay instead of
 * .modal-overlay. They share the same @click.self pattern and must receive the
 * same keyboard/dialog semantics; otherwise keyboard users cannot dismiss them
 * and screen readers see an unnamed generic div (WCAG 2.1.2 / 4.1.2).
 */
function buildUploadOverlay() {
  document.body.innerHTML = ''
  const opener = document.createElement('button')
  opener.textContent = '上传'
  document.body.appendChild(opener)
  opener.focus()

  const overlay = document.createElement('div')
  overlay.className = 'upload-overlay'
  const dialog = document.createElement('div')
  dialog.className = 'upload-dialog'
  const heading = document.createElement('h3')
  heading.textContent = '上传文件'
  const close = document.createElement('button')
  close.textContent = '关闭'
  const fileInput = document.createElement('input')
  fileInput.type = 'file'
  // DOM order: close (first focusable), fileInput (last focusable).
  dialog.append(heading, close, fileInput)
  overlay.appendChild(dialog)
  document.body.appendChild(overlay)

  let closeCount = 0
  overlay.addEventListener('click', (e) => {
    if (e.target === overlay) {
      closeCount += 1
      overlay.remove()
    }
  })
  return { opener, overlay, heading, close, fileInput, getCloseCount: () => closeCount }
}

describe('upload-overlay keyboard and dialog semantics (R230)', () => {
  let dispose: () => void = () => {}

  beforeEach(() => {
    dispose = installViewModalManager()
  })
  afterEach(() => {
    dispose()
    document.body.innerHTML = ''
  })

  it('Escape closes the upload overlay and restores opener focus', async () => {
    const { opener, overlay, close, getCloseCount } = buildUploadOverlay()
    await flush()

    expect(overlay.getAttribute('role')).toBe('dialog')
    expect(overlay.getAttribute('aria-modal')).toBe('true')
    expect(document.activeElement).toBe(close)

    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }))
    expect(getCloseCount()).toBe(1)
    expect(document.body.contains(overlay)).toBe(false)

    await flush()
    expect(document.activeElement).toBe(opener)
  })

  it('names the dialog from its heading and traps Tab inside it', async () => {
    const { overlay, heading, close, fileInput } = buildUploadOverlay()
    await flush()

    expect(overlay.getAttribute('aria-labelledby')).toBeTruthy()
    expect(document.getElementById(overlay.getAttribute('aria-labelledby')!)).toBe(heading)

    fileInput.focus()
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Tab', bubbles: true }))
    expect(document.activeElement).toBe(close)
    expect(overlay.contains(document.activeElement)).toBe(true)
  })
})
