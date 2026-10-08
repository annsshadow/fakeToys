// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment jsdom
/**
 * xform.ts 的 DOMParser 分支（domRootIds）只在有 DOM 的运行时生效：
 * 桌面（浏览器）用 pcData.html 的元素 id 计算布局根，并剔除被其它模块
 * pid 嵌套的节点；node 单测环境走 Object.keys(modules) 兜底。
 * 这里用 jsdom 钉死真实分支，防止 DOMParser 缺失时的兜底静默改变桌面行为。
 */
import { describe, expect, it } from 'vitest'
import { parseFormDefinition } from './xform'

describe('xform DOM root resolution (DOMParser branch)', () => {
  it('derives desktop roots from pcData.html element ids and drops pid-nested modules', () => {
    const parsed = parseFormDefinition({
      name: 'f',
      application: 'a',
      html: '<div id="m1"></div><div id="m2"></div>',
      pcData: {
        json: {
          moduleList: {
            m1: { id: 'm1', type: 'Textfield' },
            m2: { id: 'm2', type: 'Textfield', pid: 'm1' }, // m2 嵌在 m1 下，不应作为根
          },
        },
      },
    })
    expect(parsed.layouts.desktop.root).toEqual(['m1'])
  })

  it('keeps only element ids that actually exist in moduleList', () => {
    const parsed = parseFormDefinition({
      html: '<div id="known"></div><div id="ghost"></div>',
      moduleList: {
        known: { id: 'known', type: 'Textfield' },
      },
    })
    expect(parsed.layouts.desktop.root).toEqual(['known'])
  })
})
