// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

const TARGET_ID = 'main-content'
const LINK_CLASS = 'skip-link'

/**
 * WCAG 2.4.1 Bypass Blocks：安装「跳转到主要内容」链接。
 *
 * 链接插在 Vue 挂载点 #app 之前，所以路由切换/重新挂载都不会清掉它。
 * 主内容区（AdminLayout 的 <el-main>）由 MutationObserver 在挂载后补
 * tabindex=-1，只接收程序化焦点，键盘 Enter 时把焦点移进去。
 *
 * @returns dispose 函数，移除链接并断开观察者。
 */
export function installSkipLink(): () => void {
  if (typeof document === 'undefined') return () => {}

  const link = document.createElement('a')
  link.className = LINK_CLASS
  link.href = '#' + TARGET_ID
  link.textContent = '跳转到主要内容'
  const mountPoint = document.getElementById('app')
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
    // 保留 URL fragment 以维持正常锚点语义，再把键盘焦点移入主内容区。
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
