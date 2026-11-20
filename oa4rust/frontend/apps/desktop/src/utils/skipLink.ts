// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

const TARGET_ID = 'main-content'
const LINK_CLASS = 'skip-link'

/**
 * Install a WCAG 2.4.1 "skip to main content" link.
 *
 * The link is inserted before the Vue mount point, so a route change inside
 * #o2-app-root can never remove it. A MutationObserver handles the fact that
 * install runs before app.mount(), and re-applies focusability if a view
 * replaces the main element later.
 *
 * @returns a dispose function that removes the link and observers.
 */
export function installSkipLink(): () => void {
  if (typeof document === 'undefined') return () => {}

  const link = document.createElement('a')
  link.className = LINK_CLASS
  link.href = `#${TARGET_ID}`
  link.textContent = '跳转到主要内容'
  const mountPoint = document.getElementById('o2-app-root')
  if (mountPoint?.parentElement) {
    mountPoint.parentElement.insertBefore(link, mountPoint)
  } else {
    document.body.insertBefore(link, document.body.firstChild)
  }

  let target: HTMLElement | null = null
  const prepareTarget = () => {
    const next = document.getElementById(TARGET_ID)
    if (!next || next === target) return
    target = next
    target.setAttribute('tabindex', '-1')
  }

  const onClick = () => {
    prepareTarget()
    if (!target) return
    // Keep the fragment in the URL for normal anchor semantics, then move
    // keyboard focus so the next Tab starts inside the main content.
    target.focus()
  }
  link.addEventListener('click', onClick)

  const observer = new MutationObserver(prepareTarget)
  observer.observe(document.body, { childList: true, subtree: true })
  prepareTarget()

  return () => {
    link.removeEventListener('click', onClick)
    link.remove()
    observer.disconnect()
  }
}
