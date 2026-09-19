// @vitest-environment jsdom
/**
 * renderWidget 挂载路径测试：node 环境（widget.test.ts）只钉了「未注册即报错」，
 * 真正的 createApp/mount/provide 需要真实 DOM。这里用 render 函数组件避免
 * runtime-only 构建缺失模板编译器的限制。
 */
import { describe, expect, it } from 'vitest'
import { h, inject } from 'vue'
import type { O2Desktop } from './types.js'
import { defineWidget, renderWidget } from './widget'

const renderFnComp = {
  props: { value: { type: String, default: '' } },
  setup(props: { value: string }) {
    const desktop = inject('desktop', null)
    const widgetName = inject('widgetName', '')
    return () => h('div', [String(props.value), '|', String(widgetName), '|', desktop ? '1' : '0'])
  },
}

describe('renderWidget mount path', () => {
  it('mounts the widget with call-site options overriding definition options', () => {
    defineWidget('w-override', renderFnComp as never, { value: 'def' })
    const container = document.createElement('div')
    document.body.appendChild(container)

    renderWidget(container, 'w-override', { value: 'ovr' }, { app: 'main' } as unknown as O2Desktop)

    expect(container.textContent).toBe('ovr|w-override|1')
  })

  it('keeps the definition options when the call site omits options', () => {
    defineWidget('w-keep-def', renderFnComp as never, { value: 'def' })
    const container = document.createElement('div')
    document.body.appendChild(container)

    renderWidget(container, 'w-keep-def')

    expect(container.textContent).toBe('def|w-keep-def|0')
  })

  it('clears stale content before mounting the widget', () => {
    defineWidget('w-clean', renderFnComp as never)
    const container = document.createElement('div')
    container.innerHTML = '<span>stale</span>'
    document.body.appendChild(container)

    renderWidget(container, 'w-clean')

    expect(container.querySelector('span')).toBeNull()
    expect(container.textContent).toBe('|w-clean|0')
  })
})
