// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R213 孤儿表单标签无障碍关联的契约测试（WCAG 1.3.1 Info and Relationships / 4.1.2）。
 *
 * 大量 CRUD 视图用 <div class="form-group"><label>标题</label><input /></div>，
 * label 既无 for 也不包裹控件，读屏器无法把可见文本与输入框关联。全局增强器
 * 安装后应为这类孤儿 label 与紧随其后的 labelable 控件建立双向关联。
 * 全部通过 jsdom DOM 行为断言，不检查源码字符串。
 */
import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installLabelA11y } from './labelA11y'

async function flush() {
  await new Promise((resolve) => setTimeout(resolve, 0))
}

/** 模拟真实视图结构：form-group 内 label 紧随一个 input（无 for、不包裹）。 */
function formGroup(tag: 'input' | 'textarea' | 'select' = 'input'): HTMLElement {
  const wrap = document.createElement('div')
  wrap.className = 'form-group'
  const label = document.createElement('label')
  label.textContent = '标题'
  const control = document.createElement(tag)
  wrap.append(label, control)
  return wrap
}

let dispose: () => void = () => {}

beforeEach(() => {
  document.body.innerHTML = ''
})

afterEach(() => {
  dispose()
  dispose = () => {}
  document.body.innerHTML = ''
})

describe('孤儿表单标签无障碍关联增强器（WCAG 1.3.1 / 4.1.2）', () => {
  it('安装后为孤儿 label 与紧随的 input 建立 id/htmlFor 关联', async () => {
    const group = formGroup('input')
    document.body.appendChild(group)
    dispose = installLabelA11y(document.body)
    await flush()

    const label = group.querySelector('label')!
    const input = group.querySelector('input')!
    expect(input.id).toBeTruthy()
    expect(label.htmlFor).toBe(input.id)
    expect(label.control).toBe(input)
  })

  it('已有 for 的 label 不被覆盖', async () => {
    const group = formGroup('input')
    group.querySelector('label')!.htmlFor = 'preset-id'
    const input = group.querySelector('input')!
    input.id = 'preset-id'
    document.body.appendChild(group)
    dispose = installLabelA11y(document.body)
    await flush()

    expect(group.querySelector('label')!.htmlFor).toBe('preset-id')
    expect(input.id).toBe('preset-id')
  })

  it('包裹控件的 label 保持原样，不额外补 for', async () => {
    const wrap = document.createElement('div')
    wrap.innerHTML = '<label>标题<input type="text" /></label>'
    document.body.appendChild(wrap)
    dispose = installLabelA11y(document.body)
    await flush()

    const label = wrap.querySelector('label')!
    expect(label.hasAttribute('for')).toBe(false)
    expect(label.control).toBe(label.querySelector('input'))
  })

  it('多个 form-group 各自独立关联，不串线', async () => {
    const a = formGroup('input')
    const b = formGroup('input')
    document.body.append(a, b)
    dispose = installLabelA11y(document.body)
    await flush()

    const labelA = a.querySelector('label')!
    const labelB = b.querySelector('label')!
    const inputA = a.querySelector('input')!
    const inputB = b.querySelector('input')!
    expect(labelA.htmlFor).toBe(inputA.id)
    expect(labelB.htmlFor).toBe(inputB.id)
    expect(labelA.htmlFor).not.toBe(labelB.htmlFor)
    expect(inputA.id).not.toBe(inputB.id)
  })

  it('textarea 与 select 同样关联', async () => {
    const ta = formGroup('textarea')
    const sel = formGroup('select')
    document.body.append(ta, sel)
    dispose = installLabelA11y(document.body)
    await flush()

    expect(ta.querySelector('label')!.htmlFor).toBe(ta.querySelector('textarea')!.id)
    expect(sel.querySelector('label')!.htmlFor).toBe(sel.querySelector('select')!.id)
  })

  it('Vue 异步插入的 label + input 也由观察器处理', async () => {
    dispose = installLabelA11y(document.body)
    const group = formGroup('input')
    document.body.appendChild(group)
    await flush()

    const label = group.querySelector('label')!
    const input = group.querySelector('input')!
    expect(label.htmlFor).toBe(input.id)
    expect(input.id).toBeTruthy()
  })

  it('卸载后新增的孤儿 label 不再被处理，且属性被还原', async () => {
    const group = formGroup('input')
    document.body.appendChild(group)
    dispose = installLabelA11y(document.body)
    await flush()
    const label = group.querySelector('label')!
    const input = group.querySelector('input')!
    expect(label.htmlFor).toBe(input.id)

    dispose()
    dispose = () => {}
    expect(label.hasAttribute('for')).toBe(false)
    expect(input.hasAttribute('id')).toBe(false)

    const later = formGroup('input')
    document.body.appendChild(later)
    await flush()
    expect(later.querySelector('label')!.hasAttribute('for')).toBe(false)
  })
})
