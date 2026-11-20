// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R188 全局表单校验错误无障碍关联的契约测试（WCAG 3.3.1 / 4.1.2）。
 *
 * 全仓表单错误分散在多个视图，没有统一 FormItem 组件。因此标准落在全局运行时：
 * 任何带 data-oa-field-error 标记的错误文本，安装增强器后都会被自动关联到同字段
 * 的控件——控件 aria-invalid=true、aria-describedby 指向带 role=alert 的错误文本。
 * 测试全部通过 jsdom DOM 行为断言，不检查源码字符串。
 */
import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { FIELD_ERROR_MARKER, installFormErrorA11y } from './formErrorA11y'

/** 模拟 XformRuntime 当前渲染结构：字段容器内 label + input + 错误段落。 */
function field(id: string, label: string, errorText: string): HTMLElement {
  const wrap = document.createElement('div')
  wrap.className = 'xform-field'
  wrap.innerHTML = `
    <label for="${id}">${label}</label>
    <input id="${id}" type="text" />
    <p class="xform-error" ${FIELD_ERROR_MARKER}>${errorText}</p>
  `
  return wrap
}

async function flush() {
  await new Promise((resolve) => setTimeout(resolve, 0))
}

let dispose: () => void = () => {}

beforeEach(() => {
  document.body.innerHTML = ''
})

afterEach(() => {
  dispose()
  document.body.innerHTML = ''
})

describe('全局表单错误无障碍增强器（WCAG 3.3.1 / 4.1.2）', () => {
  it('安装后为错误文本补 role=alert/id，并给控件补 aria-invalid 与 aria-describedby', async () => {
    const wrap = field('subject', '标题', '标题为必填项')
    document.body.appendChild(wrap)
    dispose = installFormErrorA11y(document.body)

    const error = wrap.querySelector('p')!
    const input = wrap.querySelector('input')!
    // 观察器是异步的；真实运行时错误由 Vue 异步渲染插入，同样走 MutationObserver。
    await flush()

    expect(error.getAttribute('role')).toBe('alert')
    expect(error.id).toBeTruthy()
    expect(input.getAttribute('aria-invalid')).toBe('true')
    const ids = (input.getAttribute('aria-describedby') || '').split(/\s+/).filter(Boolean)
    expect(ids).toContain(error.getAttribute('id'))
  })

  it('多个字段各自独立关联，不会串线', async () => {
    const a = field('subject', '标题', '标题为必填项')
    const b = field('email', '邮箱', '邮箱格式不正确')
    document.body.append(a, b)
    dispose = installFormErrorA11y(document.body)
    await flush()

    const errorA = a.querySelector('p')!
    const errorB = b.querySelector('p')!
    const inputA = a.querySelector('input')!
    const inputB = b.querySelector('input')!
    expect(inputA.getAttribute('aria-describedby') || '').toContain(errorA.getAttribute('id')!)
    expect(inputA.getAttribute('aria-describedby') || '').not.toContain(errorB.getAttribute('id')!)
    expect(inputB.getAttribute('aria-describedby') || '').toContain(errorB.getAttribute('id')!)
    expect(inputB.getAttribute('aria-describedby') || '').not.toContain(errorA.getAttribute('id')!)
  })

  it('错误文本被移除后，控件不再标 invalid，也不再指向已消失的错误', async () => {
    const wrap = field('subject', '标题', '标题为必填项')
    document.body.appendChild(wrap)
    dispose = installFormErrorA11y(document.body)
    await flush()

    const input = wrap.querySelector('input')!
    expect(input.getAttribute('aria-invalid')).toBe('true')
    wrap.querySelector('p')!.remove()
    await flush()

    expect(input.hasAttribute('aria-invalid')).toBe(false)
    expect(input.hasAttribute('aria-describedby')).toBe(false)
  })

  it('保留控件已有的 aria-describedby 和其它属性，只追加错误 id', async () => {
    const wrap = field('subject', '标题', '标题为必填项')
    wrap.querySelector('input')!.setAttribute('aria-describedby', 'existing-hint')
    wrap.querySelector('input')!.setAttribute('placeholder', '请输入')
    document.body.appendChild(wrap)
    dispose = installFormErrorA11y(document.body)
    await flush()

    const input = wrap.querySelector('input')!
    const ids = (input.getAttribute('aria-describedby') || '').split(/\s+/).filter(Boolean)
    expect(ids).toContain('existing-hint')
    expect(ids).toContain(wrap.querySelector('p')!.getAttribute('id'))
    expect(input.getAttribute('placeholder')).toBe('请输入')
  })

  it('尊重错误文本已有的 id 与 role，不覆盖', async () => {
    const wrap = field('subject', '标题', '标题为必填项')
    const error = wrap.querySelector('p')!
    error.id = 'custom-error-id'
    error.setAttribute('role', 'status')
    document.body.appendChild(wrap)
    dispose = installFormErrorA11y(document.body)
    await flush()

    expect(error.id).toBe('custom-error-id')
    expect(error.getAttribute('role')).toBe('status')
    expect(wrap.querySelector('input')!.getAttribute('aria-describedby') || '').toContain('custom-error-id')
  })

  it('卸载后断开观察器，不再给新插入的错误加关联', async () => {
    dispose = installFormErrorA11y(document.body)
    const wrap = field('subject', '标题', '标题为必填项')
    document.body.appendChild(wrap)
    await flush()
    expect(wrap.querySelector('input')!.getAttribute('aria-invalid')).toBe('true')

    dispose()
    dispose = () => {}
    const wrap2 = field('email', '邮箱', '邮箱格式不正确')
    document.body.appendChild(wrap2)
    await flush()
    expect(wrap2.querySelector('input')!.hasAttribute('aria-invalid')).toBe(false)
  })
})
