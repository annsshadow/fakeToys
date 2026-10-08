// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import vue from '@vitejs/plugin-vue'
import { createServer } from 'vite'
import { describe, expect, it } from 'vitest'

interface CompiledComponent {
  props?: Record<string, unknown>
  emits?: string[]
}

const componentPath = resolve(import.meta.dirname, 'OrganizationSelector.vue')
const sdkPath = resolve(import.meta.dirname, '../../../sdk/src')

async function loadComponent(): Promise<CompiledComponent> {
  const server = await createServer({
    configFile: false,
    logLevel: 'silent',
    plugins: [vue()],
    resolve: { alias: { '@oa4rust/sdk': sdkPath } },
    // watch 必须全禁：workspace 根 oa4rust/ 下还有 backend/target（海量 Rust 构建
    // 产物），chokidar 递归监听会把 ssrLoadModule 拖到分钟级（实测 82s → 0.3s）。
    server: { middlewareMode: true, watch: { ignored: ['**'] } },
  })
  try {
    const loaded = await server.ssrLoadModule(componentPath)
    return loaded.default as CompiledComponent
  } finally {
    await server.close()
  }
}

describe('OrganizationSelector component contract', () => {
  it('compiles as a Vue component and exposes the reusable selection contract', async () => {
    const component = await loadComponent()
    const props = component.props ?? {}
    const emits = component.emits ?? []

    expect(component).toBeTruthy()
    expect(Object.keys(props)).toEqual(
      expect.arrayContaining(['modelValue', 'types', 'mode', 'disabled', 'max', 'searcher']),
    )
    expect(emits).toEqual(expect.arrayContaining(['update:modelValue', 'change', 'error']))
  })

  it('connects search results and selection to the public events', () => {
    const source = readFileSync(componentPath, 'utf8')

    expect(source).toContain('await props.searcher(activeType.value, keyword.value.trim())')
    expect(source).toContain("emit('update:modelValue', value)")
    expect(source).toContain("emit('change', value)")
  })

  it('preserves type-aware item keys and single/multiple selection guards', () => {
    const source = readFileSync(componentPath, 'utf8')

    expect(source).toContain('return `' + '${item.type}:${item.id}' + '`')
    expect(source).toContain("if (props.mode === 'single')")
    expect(source).toContain('if (props.max && props.modelValue.length >= props.max) return')
  })
})
