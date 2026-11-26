// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R301 t12 框架层实质升级：命令面板声明 aria-modal="true"，必须真正把键盘焦点
 * 限制在面板内并在关闭后归还触发者，否则读屏/键盘用户会 Tab 到被遮挡的背景页。
 * 契约直接执行 focus trap 工具在 jsdom DOM 上验证行为，而不是只做源码正则断言。
 */
// @vitest-environment jsdom
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

type TrapHandle = { update(): void; release(): void }

function fixture() {
  document.body.innerHTML = `
    <button id="outside-before">背景按钮</button>
    <button id="trigger" data-testid="command-palette-trigger">打开</button>
    <div id="backdrop">
      <section role="dialog" aria-modal="true" data-testid="command-palette">
        <input data-oa-command-search aria-label="命令面板搜索" />
        <button class="command-item" type="button">页面一</button>
        <button class="command-item" type="button">页面二</button>
      </section>
    </div>
    <button id="outside-after">背景按钮二</button>
  `
  return {
    dialog: document.querySelector('#backdrop') as HTMLElement,
    input: document.querySelector('[data-oa-command-search]') as HTMLInputElement,
    first: document.querySelector('.command-item') as HTMLButtonElement,
    last: document.querySelectorAll('.command-item')[1] as HTMLButtonElement,
    trigger: document.querySelector('#trigger') as HTMLButtonElement,
    before: document.querySelector('#outside-before') as HTMLButtonElement,
    after: document.querySelector('#outside-after') as HTMLButtonElement,
  }
}

function pressTab(target: Element, shift = false) {
  const evt = new KeyboardEvent('keydown', { key: 'Tab', shiftKey: shift, bubbles: true, cancelable: true })
  target.ownerDocument.dispatchEvent(evt)
  return evt
}

async function loadTrap() {
  const mod = await import('./focusTrap')
  return mod.installFocusTrap
}

describe('R301 t12 命令面板焦点陷阱契约', () => {
  it('导出 installFocusTrap 工具并在挂载时把焦点移入面板', async () => {
    const installFocusTrap = await loadTrap()
    expect(typeof installFocusTrap).toBe('function')
    const f = fixture()
    const handle: TrapHandle = installFocusTrap(f.dialog)
    expect(document.activeElement).toBe(f.input)
    handle.release()
  })

  it('Tab 在面板内循环，不逃逸到背景按钮', async () => {
    const installFocusTrap = await loadTrap()
    const f = fixture()
    const handle: TrapHandle = installFocusTrap(f.dialog)
    f.last.focus()
    const evt = pressTab(f.last)
    expect(evt.defaultPrevented).toBe(true)
    expect(document.activeElement).toBe(f.input)
    handle.release()
  })

  it('Shift+Tab 反向循环，不逃逸到背景按钮', async () => {
    const installFocusTrap = await loadTrap()
    const f = fixture()
    const handle: TrapHandle = installFocusTrap(f.dialog)
    f.input.focus()
    const evt = pressTab(f.input, true)
    expect(evt.defaultPrevented).toBe(true)
    expect(document.activeElement).toBe(f.last)
    handle.release()
  })

  it('release 后焦点归还触发者，并移除键盘监听', async () => {
    const installFocusTrap = await loadTrap()
    const f = fixture()
    f.trigger.focus()
    const handle: TrapHandle = installFocusTrap(f.dialog, f.trigger)
    handle.release()
    expect(document.activeElement).toBe(f.trigger)
    const before = document.activeElement
    f.input.focus()
    pressTab(f.input)
    expect(document.activeElement).not.toBe(before)
  })


  it('空面板也能用 tabindex=-1 接住焦点，不逃逸背景', async () => {
    const installFocusTrap = await loadTrap()
    document.body.innerHTML = '<button id="bg">背景</button><section id="empty-dialog"></section>'
    const empty = document.querySelector('#empty-dialog') as HTMLElement
    const handle: TrapHandle = installFocusTrap(empty)
    expect(document.activeElement).toBe(empty)
    handle.release()
  })

  it('AppShell 命令面板集成焦点陷阱与关闭归还，且保留原生导航契约锚点', () => {
    const shell = readFileSync(resolve(import.meta.dirname, 'AppShell.vue'), 'utf8')
    expect(shell).toContain("import { installFocusTrap, type FocusTrapHandle } from './focusTrap'")
    expect(shell).toContain('ref="commandPaletteRoot"')
    expect(shell).toContain('commandFocusTrap = installFocusTrap(commandPaletteRoot.value, commandTrigger)')
    expect(shell).toContain('commandFocusTrap?.release()')
    expect(shell).toContain('onUnmounted(() => {')
    expect(shell).toContain('role="dialog"')
    expect(shell).toContain('aria-modal="true"')
    expect(shell).toContain('tabindex="-1"')
    expect(shell).toContain('data-oa-global-search')
  })
})
