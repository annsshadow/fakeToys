// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom

import { describe, expect, it } from 'vitest'
import { createApp, defineComponent, h, nextTick, ref } from 'vue'

interface Row {
  id: string
  name: string
  app_id: string
  status: string
  definition: string
}

/** 与 TemplateApp.vue 的表格行同构：5 个 <td> + 3 个操作按钮。 */
function rowsOf(n: number): Row[] {
  return Array.from({ length: n }, (_, i) => ({
    id: `tpl-${i}`,
    name: `模板 ${i}`,
    app_id: `app-${i % 40}`,
    status: i % 2 ? 'published' : 'draft',
    definition: `{"moduleList":{"m${i}":true}}`,
  }))
}

const TableList = (rows: { value: Row[] }) =>
  defineComponent({
    setup: () => () =>
      h('table', { class: 'data-table' }, [
        h('thead', [h('tr', [h('th', '名称'), h('th', '应用'), h('th', '状态'), h('th', '定义'), h('th', '操作')])]),
        h(
          'tbody',
          rows.value.map((item) =>
            h('tr', { key: item.id }, [
              h('td', item.name),
              h('td', { class: 'mono' }, item.app_id),
              h('td', item.status),
              h('td', { class: 'val-cell mono' }, item.definition),
              h('td', [
                h('button', { class: 'btn-sm' }, '详情'),
                h('button', { class: 'btn-sm' }, '编辑'),
                h('button', { class: 'btn-sm btn-del' }, '删除'),
              ]),
            ]),
          ),
        ),
      ]),
  })

async function measure(n: number): Promise<{ mount: number; update: number; domNodes: number }> {
  const rows = ref<Row[]>(rowsOf(n))
  const host = document.createElement('div')
  document.body.appendChild(host)
  const app = createApp(TableList(rows))
  const t0 = performance.now()
  app.mount(host)
  const mount = performance.now() - t0
  const domNodes = host.querySelectorAll('tbody tr').length

  const next = rowsOf(n).map((r, i) => (i === 0 ? { ...r, name: '模板 0（已更新）' } : { ...r }))
  const t1 = performance.now()
  rows.value = next
  // 必须等 patch 完成再停表：Vue 的更新是微任务批处理，同步赋值本身几乎不耗时。
  await nextTick()
  const update = performance.now() - t1
  app.unmount()
  host.remove()
  return { mount, update, domNodes }
}

function median(samples: number[]): number {
  const sorted = [...samples].sort((a, b) => a - b)
  return sorted[Math.floor(sorted.length / 2)] ?? 0
}

describe('R132 大列表渲染（组件级、jsdom，不是真机指标）', () => {
  it('1000 行全量 v-for 的挂载/更新耗时与分页、虚拟滚动对照', async () => {
    const scenarios: Array<{ label: string; n: number }> = [
      { label: '全量 1000 行', n: 1000 },
      { label: '全量 300 行', n: 300 },
      { label: '分页 100 行/页', n: 100 },
      { label: '虚拟滚动视口 20 行', n: 20 },
    ]
    const runs = 5
    for (const { label, n } of scenarios) {
      const mounts: number[] = []
      const updates: number[] = []
      let domNodes = 0
      for (let i = 0; i < runs; i += 1) {
        const r = await measure(n)
        mounts.push(r.mount)
        updates.push(r.update)
        domNodes = r.domNodes
      }
      // 确定性断言：DOM 行数 = 数据行数（不是靠计时判断正确性）
      expect(domNodes).toBe(n)
      console.log(
        `${label}: 挂载 p50=${median(mounts).toFixed(2)}ms (min ${Math.min(...mounts).toFixed(2)}) ` +
          `更新 p50=${median(updates).toFixed(2)}ms (min ${Math.min(...updates).toFixed(2)}) ` +
          `DOM 行数=${domNodes}`,
      )
      await nextTick()
    }
  })
})
