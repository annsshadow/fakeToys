<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="dir-view">
    <div class="view-header glass-card">
      <h1>组织通讯录</h1>
      <p class="subtitle">organization/person/unit/identity/group/role — 通讯录查询与成员管理</p>
    </div>
    <div class="tabs glass-card">
      <button v-for="t in tabs" :key="t.key" class="tab-btn" :class="{ on: tab === t.key }" @click="tab = t.key">
        {{ t.label }}
      </button>
    </div>
    <div class="panel glass-card">
      <div class="panel-head">
        <span class="hint">{{ tabHint }}</span>
        <button class="run-btn" :disabled="busy" @click="runCurrentTab">{{ busy ? '查询中…' : '▶ 查询本页' }}</button>
      </div>
      <div v-if="rows.length" class="result-meta">命中 {{ rows.length }} 行 · 最近动作 {{ lastAction }}</div>
      <div class="table-wrap">
        <table v-if="rows.length" class="data-table">
          <thead>
            <tr>
              <th v-for="k in columns" :key="k">{{ k }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(r, i) in rows.slice(0, 60)" :key="i">
              <td v-for="k in columns" :key="k" class="cell">{{ fmt(r[k]) }}</td>
            </tr>
          </tbody>
        </table>
        <div v-else class="empty">点击「查询本页」执行当前页全部真实查询（逐条只读，失败单独提示）</div>
      </div>
    </div>
    <div class="panel glass-card" v-if="tab === 'manage'">
      <div class="panel-head"><span class="hint">成员/属性管理（写动作，admin 门禁，逐项确认）</span></div>
      <div class="manage-row">
        <button class="run-btn" @click="manage('groupAdd')">群组加人</button>
        <button class="run-btn" @click="manage('groupRemove')">群组移除</button>
        <button class="run-btn" @click="manage('dutyUpdate')">职务更新成员</button>
        <button class="run-btn" @click="manage('dutyUpdatePost')">职务更新(POST)</button>
        <button class="run-btn" @click="manage('personAttr')">人员属性保存</button>
        <button class="run-btn" @click="manage('unitAttr')">单位属性保存</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { computed, ref } from 'vue'
import { toast } from '../utils/toast'

type Tab = 'person' | 'unit' | 'identity' | 'group' | 'role' | 'manage'
const tabs: Array<{ key: Tab; label: string }> = [
  { key: 'person', label: '人员' },
  { key: 'unit', label: '单位' },
  { key: 'identity', label: '身份' },
  { key: 'group', label: '群组' },
  { key: 'role', label: '角色/目录' },
  { key: 'manage', label: '成员管理' },
]
const tab = ref<Tab>('person')
const busy = ref(false)
const rows = ref<Record<string, unknown>[]>([])
const lastAction = ref('—')

const tabHint = computed(
  () =>
    ({
      person: '人员查询：全量 / 按单位（直连·嵌套·模糊）/ 上下级 / 按角色·身份·群组·属性 / 登录活跃',
      unit: '单位查询：全量 / 子单位·上级单位（直连·嵌套）/ 层级·类型 / 按身份·人员 / 单位属性·职务',
      identity: '身份查询：全量 / 按人员·单位·群组 / 主职身份',
      group: '群组查询：全量 / 按身份·人员 / 子群组·上级群组（直连·嵌套）',
      role: '角色查询 + 部门树 + 用户清单',
      manage: '群组成员增删 / 职务成员更新 / 人员·单位属性保存（先在上方查询，再点下方写动作）',
    })[tab.value] ?? '',
)

const columns = computed(() => {
  const keys = new Set<string>()
  for (const r of rows.value.slice(0, 30)) for (const k of Object.keys(r)) keys.add(k)
  return [...keys].slice(0, 8)
})
function fmt(v: unknown): string {
  if (v === null || v === undefined) return '—'
  if (typeof v === 'object') return JSON.stringify(v).slice(0, 60)
  return String(v).slice(0, 60) || '—'
}
const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
function take(resp: unknown): Record<string, unknown>[] {
  const d = (resp as any)?.data
  if (Array.isArray(d)) return d as Record<string, unknown>[]
  if (Array.isArray((d as any)?.data)) return (d as any).data
  if (Array.isArray((d as any)?.personList)) return (d as any).personList
  if (Array.isArray((d as any)?.unitList)) return (d as any).unitList
  if (Array.isArray((d as any)?.identityList)) return (d as any).identityList
  if (Array.isArray((d as any)?.groupList)) return (d as any).groupList
  if (Array.isArray((d as any)?.roleList)) return (d as any).roleList
  if (d && typeof d === 'object') return [d as Record<string, unknown>]
  return []
}
async function runAll(fns: Array<() => Promise<void>>): Promise<void> {
  busy.value = true
  for (const f of fns) await f()
  busy.value = false
}
async function collect(label: string, p: Promise<unknown>): Promise<void> {
  const r = await s(p)
  if (r === null) {
    toast.error(`${label} 查询失败`)
    return
  }
  const list = take(r)
  if (list.length) rows.value = list
  lastAction.value = `${label}（${list.length} 行）`
}

// ── 人员 ────────────────────────────────────────────────────────────
function runPerson(): void {
  void runAll([
    async () => collect('全量人员(all/object)', await api.get('/api/person/list/all/object')),
    async () => collect('人员清单(list/object)', await api.post('/api/person/list/object', {})),
    async () => {
      const u = prompt('单位标识（多个用逗号分隔）:', '') || ''
      if (!u) return
      const unitList = u
        .split(',')
        .map((x) => x.trim())
        .filter(Boolean)
      await collect(
        '按单位直查(sub/direct/object)',
        await api.post('/api/person/list/unit/sub/direct/object', { unitList }),
      )
      await collect(
        '按单位嵌套(sub/nested/object)',
        await api.post('/api/person/list/unit/sub/nested/object', { unitList }),
      )
      await collect(
        '按单位模糊(direct/like)',
        await api.post('/api/person/list/unit/sub/direct/like/object', { unitList }),
      )
      await collect(
        '按单位嵌套模糊(nested/like)',
        await api.post('/api/person/list/unit/sub/nested/like/object', { unitList }),
      )
    },
    async () => {
      const p = prompt('人员标识（上下级查询，逗号分隔）:', '') || ''
      if (!p) return
      const personList = p
        .split(',')
        .map((x) => x.trim())
        .filter(Boolean)
      await collect('下属(直连)', await api.post('/api/person/list/person/sub/direct/object', { personList }))
      await collect('下属(嵌套)', await api.post('/api/person/list/person/sub/nested/object', { personList }))
      await collect('上级(直连)', await api.post('/api/person/list/person/sup/direct/object', { personList }))
      await collect('上级(嵌套)', await api.post('/api/person/list/person/sup/nested/object', { personList }))
    },
    async () => {
      const r = prompt('角色标识（逗号分隔）:', '') || ''
      if (r) {
        const roleList = r
          .split(',')
          .map((x) => x.trim())
          .filter(Boolean)
        await collect('按角色查人', await api.post('/api/person/list/role/object', { roleList }))
      }
      const g = prompt('群组标识（逗号分隔）:', '') || ''
      if (g) {
        const groupList = g
          .split(',')
          .map((x) => x.trim())
          .filter(Boolean)
        await collect('按群组查人', await api.post('/api/person/list/group/object', { groupList }))
      }
      await collect('按身份查人', await api.post('/api/person/list/identity/object', {}))
      await collect('人员属性清单', await api.post('/api/person/list/personattribute/object', {}))
    },
    async () => {
      await collect('最近登录人员', await api.post('/api/person/list/login/recent/object', {}))
      const after = prompt('登录时间晚于（YYYY-MM-DD）:', '') || ''
      await collect(
        '指定时间后登录',
        await api.post('/api/person/list/login/after/object', { date: after || '2026-01-01' }),
      )
    },
  ])
}

// ── 单位 ────────────────────────────────────────────────────────────
function runUnit(): void {
  void runAll([
    async () => collect('全量单位(list/object)', await api.post('/api/unit/list/object', {})),
    async () => {
      const u = prompt('单位标识（逗号分隔）:', '') || ''
      const unitList = u
        ? u
            .split(',')
            .map((x) => x.trim())
            .filter(Boolean)
        : []
      await collect('子单位(直连)', await api.post('/api/unit/list/unit/sub/direct/object', { unitList }))
      await collect('子单位(嵌套)', await api.post('/api/unit/list/unit/sub/nested/object', { unitList }))
      await collect('上级单位(直连)', await api.post('/api/unit/list/unit/sup/direct/object', { unitList }))
      await collect('上级单位(嵌套)', await api.post('/api/unit/list/unit/sup/nested/object', { unitList }))
      await collect('单位下人员(嵌套)', await api.post('/api/person/list/unit/sub/nested/object', { unitList }))
      await collect('单位下人员(直连)', await api.post('/api/person/list/unit/sub/direct/object', { unitList }))
      await collect('单位下身份(嵌套)', await api.post('/api/identity/list/unit/sub/nested/object', { unitList }))
    },
    async () => {
      await collect('单位层级', await api.post('/api/unit/list/level/object', {}))
      await collect('按类型查单位(0)', await api.get('/api/unit/list/type/0/object'))
      await collect('单位类型清单', await api.post('/api/unit/list/types/object', {}))
      await collect('身份层级', await api.post('/api/unit/identity/level/object', {}))
      await collect('身份类型', await api.post('/api/unit/identity/type/object', {}))
      await collect('单位属性', await api.post('/api/unit/list/unitattribute/object', {}))
      await collect('单位职务', await api.post('/api/unit/list/unitduty/object', {}))
    },
  ])
}

// ── 身份 ────────────────────────────────────────────────────────────
function runIdentity(): void {
  void runAll([
    async () => collect('身份清单(list/object)', await api.post('/api/identity/list/object', {})),
    async () => {
      const p = prompt('人员标识（逗号分隔）:', '') || ''
      const personList = p
        ? p
            .split(',')
            .map((x) => x.trim())
            .filter(Boolean)
        : []
      await collect('按人员查身份', await api.post('/api/identity/list/person/object', { personList }))
      await collect('主职身份', await api.post('/api/identity/list/major/person/object', { personList }))
      await collect('单位+人员身份', await api.post('/api/identity/list/unit/person/object', { personList }))
    },
    async () => {
      const u = prompt('单位标识（逗号分隔）:', '') || ''
      const unitList = u
        ? u
            .split(',')
            .map((x) => x.trim())
            .filter(Boolean)
        : []
      await collect('按单位(直连)查身份', await api.post('/api/identity/list/unit/sub/direct/object', { unitList }))
      await collect('按单位(嵌套)查身份', await api.post('/api/identity/list/unit/sub/nested/object', { unitList }))
      const g = prompt('群组标识（逗号分隔）:', '') || ''
      const groupList = g
        ? g
            .split(',')
            .map((x) => x.trim())
            .filter(Boolean)
        : []
      await collect('按群组查身份', await api.post('/api/identity/list/group/object', { groupList }))
    },
  ])
}

// ── 群组 ────────────────────────────────────────────────────────────
function runGroup(): void {
  void runAll([
    async () => collect('群组清单(list/object)', await api.post('/api/group/list/object', {})),
    async () => {
      const g = prompt('群组标识（逗号分隔）:', '') || ''
      const groupList = g
        ? g
            .split(',')
            .map((x) => x.trim())
            .filter(Boolean)
        : []
      await collect('子群组(直连)', await api.post('/api/group/list/group/sub/direct/object', { groupList }))
      await collect('子群组(嵌套)', await api.post('/api/group/list/group/sub/nested/object', { groupList }))
      await collect('上级群组(直连)', await api.post('/api/group/list/group/sup/direct/object', { groupList }))
      await collect('上级群组(嵌套)', await api.post('/api/group/list/group/sup/nested/object', { groupList }))
    },
    async () => {
      await collect('按身份查群组', await api.post('/api/group/list/identity/object', {}))
      await collect('按人员查群组', await api.post('/api/group/list/person/object', {}))
    },
  ])
}

// ── 角色 / 目录 ─────────────────────────────────────────────────────
function runRole(): void {
  void runAll([
    async () => collect('角色清单', await api.post('/api/role/list/object', {})),
    async () => {
      const p = prompt('人员标识（逗号分隔）:', '') || ''
      const personList = p
        ? p
            .split(',')
            .map((x) => x.trim())
            .filter(Boolean)
        : []
      await collect('按人员查角色', await api.post('/api/role/list/person/object', { personList }))
    },
    async () => collect('部门树', await api.get('/api/departments/tree')),
    async () => collect('用户清单', await api.get('/api/users/list')),
  ])
}

const RUNNERS: Record<Tab, () => void> = {
  person: runPerson,
  unit: runUnit,
  identity: runIdentity,
  group: runGroup,
  role: runRole,
  manage: () => toast.info('成员管理为写动作，请使用下方按钮逐项执行'),
}
function runCurrentTab(): void {
  rows.value = []
  RUNNERS[tab.value]()
}

// ── 成员管理（写动作）──────────────────────────────────────────────
async function manage(op: string): Promise<void> {
  try {
    if (op === 'groupAdd') {
      const flag = prompt('群组标识 (flag):', '') || ''
      const persons = prompt('新增成员（人员标识，逗号分隔）:', '') || ''
      if (!flag || !persons) return
      await api.put(`/api/organization/assemble/control/group/${encodeURIComponent(flag)}/add/member`, {
        personList: persons
          .split(',')
          .map((x) => x.trim())
          .filter(Boolean),
      })
      toast.success('群组成员已新增')
    } else if (op === 'groupRemove') {
      const flag = prompt('群组标识 (flag):', '') || ''
      const persons = prompt('移除成员（人员标识，逗号分隔）:', '') || ''
      if (!flag || !persons) return
      await api.put(`/api/organization/assemble/control/group/${encodeURIComponent(flag)}/delete/member`, {
        personList: persons
          .split(',')
          .map((x) => x.trim())
          .filter(Boolean),
      })
      toast.success('群组成员已移除')
    } else if (op === 'dutyUpdate') {
      const duty = prompt('职务名称 (unitDuty.name):', '') || ''
      const persons = prompt('成员（身份标识，逗号分隔）:', '') || ''
      if (!duty || !persons) return
      await api.put('/api/organization/assemble/control/unitduty/update/member', {
        unitDuty: { name: duty },
        identityList: persons
          .split(',')
          .map((x) => x.trim())
          .filter(Boolean),
      })
      toast.success('职务成员已更新')
    } else if (op === 'dutyUpdatePost') {
      const duty = prompt('职务名称 (unitDuty.name):', '') || ''
      const persons = prompt('成员（身份标识，逗号分隔）:', '') || ''
      if (!duty || !persons) return
      await api.post('/api/organization/assemble/control/unitduty/update/member', {
        unitDuty: { name: duty },
        identityList: persons
          .split(',')
          .map((x) => x.trim())
          .filter(Boolean),
      })
      toast.success('职务成员已更新(POST)')
    } else if (op === 'personAttr') {
      const flag = prompt('人员属性标识 (flag):', '') || ''
      if (!flag) return
      const value = prompt('属性值 (attributeValue.value):', '') || ''
      await api.put(`/api/organization/assemble/control/personattribute/${encodeURIComponent(flag)}`, {
        attributeValue: { value },
      })
      toast.success('人员属性已保存')
    } else if (op === 'unitAttr') {
      const flag = prompt('单位属性标识 (flag):', '') || ''
      if (!flag) return
      const value = prompt('属性值 (attributeValue.value):', '') || ''
      await api.put(`/api/organization/assemble/control/unitattribute/${encodeURIComponent(flag)}`, {
        attributeValue: { value },
      })
      toast.success('单位属性已保存')
    }
  } catch (e: any) {
    toast.error(`管理动作失败: ${e?.message ?? ''}`)
  }
}
</script>

<style scoped>
.dir-view { display: flex; flex-direction: column; gap: 16px; height: 100% }
.view-header { padding: 16px 24px }
.view-header h1 { font-family: 'Orbitron', sans-serif; font-size: 20px; color: var(--color-primary); margin: 0 0 4px }
.subtitle { font-size: 12px; color: var(--text-muted); margin: 0; font-family: 'JetBrains Mono', monospace }
.tabs { display: flex; gap: 8px; padding: 10px 16px }
.tab-btn { padding: 8px 18px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--text-secondary); font-size: 13px; cursor: pointer }
.tab-btn.on { background: var(--color-primary); color: #000; border-color: var(--color-primary); font-weight: 600 }
.panel { padding: 14px 16px; display: flex; flex-direction: column; gap: 10px; overflow: auto }
.panel-head { display: flex; align-items: center; justify-content: space-between; gap: 12px }
.hint { font-size: 12px; color: var(--text-muted) }
.run-btn { padding: 6px 16px; background: var(--color-primary); color: #000; border: none; border-radius: var(--radius-md); font-size: 12px; font-weight: 600; cursor: pointer }
.run-btn:disabled { opacity: 0.5 }
.result-meta { font-size: 12px; color: var(--text-secondary); font-family: 'JetBrains Mono', monospace }
.table-wrap { overflow: auto }
.data-table { width: 100%; border-collapse: collapse; font-size: 12px }
.data-table th { text-align: left; color: var(--text-muted); padding: 6px 10px; border-bottom: 1px solid var(--border-subtle); position: sticky; top: 0; background: var(--bg-elevated) }
.data-table td { padding: 6px 10px; border-bottom: 1px solid var(--border-subtle); color: var(--text-primary) }
.cell { max-width: 220px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.empty { padding: 30px; text-align: center; color: var(--text-muted); font-size: 13px }
.manage-row { display: flex; flex-wrap: wrap; gap: 8px }
</style>
