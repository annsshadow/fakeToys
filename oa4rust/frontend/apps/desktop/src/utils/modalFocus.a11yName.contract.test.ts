// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom

import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installViewModalManager } from './modalFocus'

/**
 * R211: WCAG 4.1.2 (Name, Role, Value). installViewModalManager turns every
 * hand-written <div class="modal-overlay"> into role="dialog" aria-modal="true",
 * but that role still needs an accessible name. Without one, a screen reader
 * announces only "dialog" and a user cannot tell a 新建 / 编辑 / 详情 modal
 * apart. The 44 CRUD views all put a visible <h3> title in the overlay but
 * give it no id, so no label is wired up today.
 */
const flush = () => new Promise<void>((resolve) => setTimeout(resolve, 0))

function addOverlay(children: HTMLElement[]): HTMLElement {
  const overlay = document.createElement('div')
  overlay.className = 'modal-overlay'
  const box = document.createElement('div')
  box.className = 'modal glass-card'
  box.append(...children)
  overlay.appendChild(box)
  document.body.appendChild(overlay)
  return overlay
}

function title(text: string): HTMLHeadingElement {
  const h = document.createElement('h3')
  h.textContent = text
  return h
}

describe('view modal accessible name (R211 / WCAG 4.1.2)', () => {
  let dispose: () => void = () => {}

  beforeEach(() => {
    document.body.innerHTML = ''
    dispose = installViewModalManager()
  })

  afterEach(() => {
    dispose()
    document.body.innerHTML = ''
  })

  it('names each dialog from its visible heading', async () => {
    const create = addOverlay([title('新建栏目'), document.createElement('input')])
    const edit = addOverlay([title('编辑栏目'), document.createElement('input')])
    await flush()

    // role + modal state alone are not enough: a dialog must have a name.
    const named = [create, edit].map((overlay) => {
      const id = overlay.getAttribute('aria-labelledby')
      expect(id, 'overlay must expose aria-labelledby').toBeTruthy()
      const target = id ? document.getElementById(id) : null
      expect(target, 'aria-labelledby must reference a live element').not.toBeNull()
      return { overlay, target: target as HTMLElement }
    })

    expect(named[0].target.textContent).toBe('新建栏目')
    expect(named[1].target.textContent).toBe('编辑栏目')
    expect(named[0].overlay).not.toBe(named[1].overlay)
    // A stable id on the referenced heading, distinct for each dialog.
    expect(named[0].target.id).not.toBe(named[1].target.id)
    // The labelled heading must live inside the dialog it names.
    expect(named[0].overlay.contains(named[0].target)).toBe(true)
    expect(named[1].overlay.contains(named[1].target)).toBe(true)
  })

  it('removes its injected id and aria-labelledby when the dialog closes', async () => {
    const overlay = addOverlay([title('详情'), document.createElement('input')])
    await flush()
    const id = overlay.getAttribute('aria-labelledby')
    expect(id).toBeTruthy()
    const heading = id ? document.getElementById(id) : null
    expect(heading).not.toBeNull()

    overlay.remove()
    await flush()

    expect(overlay.hasAttribute('aria-labelledby')).toBe(false)
    expect(heading?.hasAttribute('id')).toBe(false)
  })

  it('falls back to an explicit aria-label when the dialog has no heading', async () => {
    const overlay = addOverlay([document.createElement('input')])
    overlay.setAttribute('aria-label', '无标题对话框')
    await flush()

    expect(overlay.getAttribute('role')).toBe('dialog')
    expect(overlay.getAttribute('aria-label')).toBe('无标题对话框')
    expect(overlay.hasAttribute('aria-labelledby')).toBe(false)
  })

  it('does not overwrite author-supplied aria-label or aria-labelledby', async () => {
    // An author-supplied aria-labelledby is left completely untouched.
    const existing = title('表单')
    existing.id = 'author-title'
    const labelled = addOverlay([existing, document.createElement('input')])
    labelled.setAttribute('aria-labelledby', 'author-title')
    await flush()
    expect(labelled.getAttribute('aria-labelledby')).toBe('author-title')
    expect(existing.id).toBe('author-title')

    // An author-supplied aria-label already names the dialog: do not inject one.
    const labelledAlt = addOverlay([title('另建'), document.createElement('input')])
    labelledAlt.setAttribute('aria-label', '作者提供的名称')
    await flush()
    expect(labelledAlt.getAttribute('aria-label')).toBe('作者提供的名称')
    expect(labelledAlt.hasAttribute('aria-labelledby')).toBe(false)
  })

  it('clears injected labelling on dispose', async () => {
    const overlay = addOverlay([title('临时'), document.createElement('input')])
    await flush()
    const id = overlay.getAttribute('aria-labelledby')
    expect(id).toBeTruthy()
    const heading = id ? document.getElementById(id) : null

    dispose()

    expect(overlay.hasAttribute('aria-labelledby')).toBe(false)
    expect(heading?.hasAttribute('id')).toBe(false)
  })
})
