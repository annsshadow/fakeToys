// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R207: the topbar advertises "搜索 (⌘K)" but AppShell binds its handler to the
 * input itself, so the hint only worked while the input was already focused.
 * Capture Ctrl+K / Meta+K globally and focus the marked topbar input, exactly
 * as the shortcut hint promises. A dedicated marker is used because many CRUD
 * views have their own .search-input boxes that must not be hijacked; the
 * lookup happens on each event so a late-mounted AppShell is still reachable.
 */
export function installGlobalSearchShortcut(root: Document = document): () => void {
  function onKeyDown(e: KeyboardEvent): void {
    if (e.defaultPrevented || e.altKey || e.shiftKey) return
    if (e.key !== 'k' && e.key !== 'K') return
    if (!e.ctrlKey && !e.metaKey) return
    const input = root.querySelector<HTMLInputElement>('input[data-oa-global-search]')
    if (!input) return
    e.preventDefault()
    input.focus()
    input.select()
  }

  root.addEventListener('keydown', onKeyDown)
  return () => root.removeEventListener('keydown', onKeyDown)
}
