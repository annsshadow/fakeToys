// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

const OVERLAY_SELECTOR = '.modal-overlay'
const CONFIRM_OVERLAY_ID = 'oa4-confirm-overlay'
const FOCUSABLE_SELECTOR = [
  'a[href]',
  'button:not([disabled])',
  'input:not([disabled]):not([type="hidden"])',
  'select:not([disabled])',
  'textarea:not([disabled])',
  '[tabindex]:not([tabindex="-1"])',
].join(',')
const ATTRIBUTE_MARKER = 'data-oa4-modal-managed'
const TITLE_SELECTOR = 'h1,h2,h3,h4,h5,h6,[role="heading"]'
const TITLE_ID_PREFIX = 'oa4-modal-title-'
let titleIdCounter = 0

type ManagedModal = {
  opener: HTMLElement | null
  restoreFocus: boolean
  /** Heading we injected an id into, so we can undo it on close/dispose. */
  titledBy: HTMLElement | null
}

/** A dialog's accessible name comes from a visible heading (WCAG 4.1.2). */
function findTitle(overlay: HTMLElement): HTMLElement | null {
  // Respect an author-supplied labelling: leave naming entirely alone.
  if (overlay.hasAttribute('aria-labelledby') || overlay.hasAttribute('aria-label')) return null
  return overlay.querySelector<HTMLElement>(TITLE_SELECTOR)
}

function focusableWithin(root: HTMLElement): HTMLElement[] {
  return Array.from(root.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR)).filter(
    (el) => !el.hasAttribute('disabled') && el.getAttribute('aria-hidden') !== 'true',
  )
}

function isHiddenByAncestor(node: Node): boolean {
  let current: HTMLElement | null = node instanceof HTMLElement ? node : null
  while (current) {
    if (current.getAttribute('aria-hidden') === 'true') return true
    current = current.parentElement
  }
  return false
}

function visibleModalOverlays(): HTMLElement[] {
  return Array.from(document.querySelectorAll<HTMLElement>(OVERLAY_SELECTOR)).filter(
    (node) => !isHiddenByAncestor(node),
  )
}

/** Siblings on the path from the overlay up to body — the page behind the modal. */
function backgroundRoots(overlay: HTMLElement): HTMLElement[] {
  const roots: HTMLElement[] = []
  let child: HTMLElement = overlay
  let parent = overlay.parentElement
  while (parent && parent !== document.body) {
    for (const sibling of Array.from(parent.children)) {
      if (sibling !== child && sibling instanceof HTMLElement && !sibling.id.startsWith(CONFIRM_OVERLAY_ID)) {
        roots.push(sibling)
      }
    }
    child = parent
    parent = parent.parentElement
  }
  return roots
}

/**
 * Global keyboard/focus behavior for the hand-written modal overlays used by
 * the CRUD views. One install covers every `<div class="modal-overlay"
 * @click.self="...">` without touching the 90+ view files.
 *
 * - Escape dispatches a bubbling click on the overlay itself, triggering the
 *   view's existing `@click.self` close handler (no state coupling).
 * - Tab is trapped inside the topmost modal; focus moves in on open and
 *   returns to the opener on close.
 * - The page behind the modal is aria-hidden while it is visible.
 *
 * The programmatic confirmMsg overlay manages its own focus and is excluded.
 *
 * @returns a dispose function that removes every listener and side effect.
 */
export function installViewModalManager(): () => void {
  if (typeof document === 'undefined') return () => {}

  const managed = new Map<HTMLElement, ManagedModal>()
  let hiddenRoots: HTMLElement[] = []
  let disposed = false
  let ignoreFocusIn = false

  const viewOverlays = (): HTMLElement[] =>
    visibleModalOverlays().filter((node) => !node.id.startsWith(CONFIRM_OVERLAY_ID))

  const applyHiddenRoots = (next: HTMLElement[]): void => {
    for (const root of hiddenRoots) if (!next.includes(root)) root.removeAttribute('aria-hidden')
    for (const root of next) root.setAttribute('aria-hidden', 'true')
    hiddenRoots = next
  }

  const releaseManaged = (overlay: HTMLElement): void => {
    const record = managed.get(overlay)
    if (!record) return
    managed.delete(overlay)
    overlay.removeAttribute(ATTRIBUTE_MARKER)
    overlay.removeAttribute('role')
    overlay.removeAttribute('aria-modal')
    if (record.titledBy?.id.startsWith(TITLE_ID_PREFIX)) {
      record.titledBy.removeAttribute('id')
    }
    overlay.removeAttribute('aria-labelledby')
    if (record.restoreFocus && record.opener?.isConnected) record.opener.focus()
  }

  const sync = (): void => {
    if (disposed) return
    const views = viewOverlays()

    for (const overlay of Array.from(managed.keys())) {
      if (!views.includes(overlay)) releaseManaged(overlay)
    }

    const top = views.length ? views[views.length - 1] : null
    for (const overlay of views) {
      if (managed.has(overlay)) continue
      const opener = document.activeElement instanceof HTMLElement ? document.activeElement : null
      const title = findTitle(overlay)
      let titledBy: HTMLElement | null = null
      if (title) {
        if (title.id) {
          // Author-supplied id: reference it as-is and leave it untouched.
          titledBy = null
        } else {
          title.id = `${TITLE_ID_PREFIX}${++titleIdCounter}`
          titledBy = title
        }
        if (!overlay.hasAttribute('aria-labelledby')) {
          overlay.setAttribute('aria-labelledby', title.id)
        }
      }
      managed.set(overlay, {
        opener: overlay.contains(opener) ? null : opener,
        restoreFocus: false,
        titledBy,
      })
      overlay.setAttribute(ATTRIBUTE_MARKER, '')
      overlay.setAttribute('role', 'dialog')
      overlay.setAttribute('aria-modal', 'true')
      ignoreFocusIn = true
      ;(focusableWithin(overlay)[0] ?? overlay).focus()
      ignoreFocusIn = false
    }

    for (const overlay of views) {
      const record = managed.get(overlay)
      if (record) record.restoreFocus = overlay === top
    }
    applyHiddenRoots(top ? backgroundRoots(top) : [])
  }

  // confirmMsg (R160) installs its own keydown trap while it is open; view
  // modal trapping is suspended so keyboard events reach that dialog first.
  const confirmActive = (): boolean => document.getElementById(CONFIRM_OVERLAY_ID) !== null

  const onKeyDown = (event: KeyboardEvent): void => {
    if (disposed || confirmActive()) return
    const views = viewOverlays()
    const overlay = views.length ? views[views.length - 1] : null
    if (!overlay || !managed.has(overlay)) return

    if (event.key === 'Escape') {
      event.preventDefault()
      event.stopImmediatePropagation()
      overlay.dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true }))
      return
    }

    if (event.key !== 'Tab') return
    const focusable = focusableWithin(overlay)
    if (!focusable.length) {
      event.preventDefault()
      overlay.focus()
      return
    }
    const first = focusable[0]
    const last = focusable[focusable.length - 1]
    const active = document.activeElement
    if (event.shiftKey && (active === first || active === overlay || !overlay.contains(active))) {
      event.preventDefault()
      last.focus()
    } else if (!event.shiftKey && (active === last || !overlay.contains(active))) {
      event.preventDefault()
      first.focus()
    }
  }

  const onFocusIn = (event: FocusEvent): void => {
    if (disposed || ignoreFocusIn || confirmActive()) return
    const views = viewOverlays()
    const overlay = views.length ? views[views.length - 1] : null
    if (!overlay || !managed.has(overlay)) return
    const target = event.target
    if (target instanceof Node && overlay.contains(target)) return

    ignoreFocusIn = true
    ;(focusableWithin(overlay)[0] ?? overlay).focus()
    ignoreFocusIn = false
  }

  const observer = new MutationObserver(sync)
  observer.observe(document.body, {
    childList: true,
    subtree: true,
    attributes: true,
    attributeFilter: ['class', 'style', 'hidden'],
  })

  document.addEventListener('keydown', onKeyDown, true)
  document.addEventListener('focusin', onFocusIn, true)
  sync()

  return () => {
    if (disposed) return
    disposed = true
    observer.disconnect()
    document.removeEventListener('keydown', onKeyDown, true)
    document.removeEventListener('focusin', onFocusIn, true)
    applyHiddenRoots([])
    for (const overlay of Array.from(managed.keys())) {
      const record = managed.get(overlay)
      overlay.removeAttribute(ATTRIBUTE_MARKER)
      overlay.removeAttribute('role')
      overlay.removeAttribute('aria-modal')
      overlay.removeAttribute('aria-labelledby')
      if (record?.titledBy?.id.startsWith(TITLE_ID_PREFIX)) {
        record.titledBy.removeAttribute('id')
      }
      managed.delete(overlay)
    }
  }
}
