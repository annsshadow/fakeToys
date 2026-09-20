<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>单元管理</h1>
      <p class="subtitle">/api/unit/* — 组织单元与身份映射</p>
    </div>
    <div class="content-panel glass-card">
      <div class="toolbar">
        <input v-model="keyword" placeholder="搜索单元..." class="search-input" @keyup.enter="doSearch" />
        <button class="btn-primary" @click="doSearch">搜索</button>
        <button class="btn-primary" @click="loadUnits">刷新全部</button>
      </div>
      <div class="list-panel">
        <div v-if="loading" class="loading-row"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <div v-else-if="units.length===0" class="empty"><div class="ei">🏗️</div><p>暂无单元数据</p></div>
        <div v-else class="item-grid">
          <div v-for="u in units" :key="u.id" class="item-card glass-card">
            <div class="ic">🏗️</div>
            <div class="ib">
              <div class="it">{{ u.name || u.title || '未命名单元' }}</div>
              <div class="im">flag: {{ u.flag || u.unitFlag || u.id }}</div>
              <div class="meta">{{ u.desc || u.description || '' }}</div>
            </div>
            <button class="btn-sm" @click="checkUnit(u)">验证</button>
            <button class="btn-sm" @click="openUnit(u)">管理</button>
          </div>
        </div>
      </div>
    </div>
    <div v-if="selectedUnit" class="content-panel glass-card">
      <div class="detail-head">
        <h2>{{ selectedUnit.name || selectedUnit.title || '单位' }} — 属性与职务</h2>
        <button class="btn-sm" @click="selectedUnit = null">关闭</button>
      </div>
      <div class="detail-cols">
        <section class="dcol">
          <div class="dcol-head"><span>单位属性</span><button class="mini-add" @click="addAttr">+ 新增</button></div>
          <div v-if="attrs.length === 0" class="empty-sm">暂无属性</div>
          <div v-else class="dlist">
            <div v-for="a in attrs" :key="a.id" class="drow">
              <div class="dmain"><span class="dname">{{ a.attributeKey }}</span><span class="dsub">{{ a.attributeValue || '—' }}</span></div>
              <button class="dcol-del" @click="removeAttr(a)">删除</button>
            </div>
          </div>
        </section>
        <section class="dcol">
          <div class="dcol-head"><span>单位职务</span><button class="mini-add" @click="addDuty">+ 新增</button></div>
          <div v-if="duties.length === 0" class="empty-sm">暂无职务</div>
          <div v-else class="dlist">
            <div v-for="d in duties" :key="d.id" class="drow">
              <div class="dmain"><span class="dname">{{ d.name }}</span><span class="dsub">{{ d.identityId || '未指派' }}</span></div>
              <button class="dcol-del" @click="removeDuty(d)">删除</button>
            </div>
          </div>
        </section>
        <section class="dcol">
          <div class="dcol-head"><span>单位身份（{{ identities.length }}）</span></div>
          <div v-if="identities.length === 0" class="empty-sm">暂无身份</div>
          <div v-else class="dlist">
            <div v-for="idn in identities" :key="idn.id" class="drow">
              <div class="dmain"><span class="dname">{{ idn.name || idn.personName || idn.id }}</span><span class="dsub">{{ idn.unitName || '' }}</span></div>
            </div>
          </div>
        </section>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { ref } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

type UnitItem = {
  id: string
  name?: string
  title?: string
  flag?: string
  unitFlag?: string
  desc?: string
  description?: string
}

const keyword = ref('')
const loading = ref(false)
const units = ref<UnitItem[]>([])

let allUnits: UnitItem[] = []
async function loadUnits() {
  loading.value = true
  try {
    const r = await api.get('/api/unit/list')
    allUnits = r.data ?? []
    units.value = allUnits
  } catch {
    units.value = []
  } finally {
    loading.value = false
  }
}
function doSearch() {
  // /api/unit/list 无关键字参数，按已加载列表做本地名称过滤
  const kw = keyword.value.trim().toLowerCase()
  units.value = kw
    ? allUnits.filter((u) => (u.name || u.title || '').toLowerCase().includes(kw))
    : allUnits
}

async function checkUnit(u: UnitItem) {
  try {
    await api.get(`/api/unit/check/${u.flag || u.id}`)
  } catch (e: any) {
    toast.error('验证失败: : ' + (e?.message ?? ''))
  }
}

loadUnits()

// 选中单位 → 属性/职务管理（organization/assemble/control unitattribute + unitduty 族）
interface Attr {
  id: string
  attributeKey: string
  attributeValue?: string
}
interface Duty {
  id: string
  name: string
  identityId?: string
}
const selectedUnit = ref<UnitItem | null>(null)
const attrs = ref<Attr[]>([])
const duties = ref<Duty[]>([])
const identities = ref<Array<{ id: string; name?: string; personName?: string; unitName?: string }>>([])
function unitKey(u: UnitItem) {
  return u.flag || u.unitFlag || u.id
}
async function openUnit(u: UnitItem) {
  selectedUnit.value = u
  await Promise.all([loadAttrs(), loadDuties(), loadIdentities()])
}
async function loadIdentities() {
  if (!selectedUnit.value) return
  try {
    // GET organization/assemble/control/identity/list/unit/{unitFlag} —— 单位下身份
    const r: any = await api.get(
      '/api/organization/assemble/control/identity/list/unit/' + unitKey(selectedUnit.value),
    )
    identities.value = (r.data ?? []) as Array<{ id: string; name?: string; personName?: string; unitName?: string }>
  } catch {
    identities.value = []
  }
}
async function loadAttrs() {
  if (!selectedUnit.value) return
  try {
    const r: any = await api.get(
      '/api/organization/assemble/control/unitattribute/list/unit/' + unitKey(selectedUnit.value),
    )
    attrs.value = (r.data ?? []) as Attr[]
  } catch {
    attrs.value = []
  }
}
async function addAttr() {
  if (!selectedUnit.value) return
  const name = prompt('属性名 (attributeKey):')
  if (!name) return
  const value = prompt('属性值（可选）:', '') ?? ''
  try {
    // 后端 unit_attribute_create 读取 name(=attributeKey)/unitId(单位 flag)/attributeValue
    await api.post('/api/organization/assemble/control/unitattribute', {
      name,
      unitId: unitKey(selectedUnit.value),
      attributeValue: value,
    })
    loadAttrs()
  } catch (e: any) {
    toast.error('新增属性失败: ' + (e?.message ?? ''))
  }
}
async function removeAttr(a: Attr) {
  if (!(await confirmMsg('确定删除属性「' + a.attributeKey + '」？'))) return
  try {
    await api.delete('/api/organization/assemble/control/unitattribute/' + a.id)
    loadAttrs()
  } catch (e: any) {
    toast.error('删除属性失败: ' + (e?.message ?? ''))
  }
}
async function loadDuties() {
  if (!selectedUnit.value) return
  try {
    const r: any = await api.get(
      '/api/organization/assemble/control/unitduty/list/unit/' + unitKey(selectedUnit.value),
    )
    duties.value = (r.data ?? []) as Duty[]
  } catch {
    duties.value = []
  }
}
async function addDuty() {
  if (!selectedUnit.value) return
  const name = prompt('职务名称:')
  if (!name) return
  try {
    // 后端 duty_create 读取 name(必填)/unitId(单位 flag)/identityList
    await api.post('/api/organization/assemble/control/unitduty', {
      name,
      unitId: unitKey(selectedUnit.value),
      identityList: [],
    })
    loadDuties()
  } catch (e: any) {
    toast.error('新增职务失败: ' + (e?.message ?? ''))
  }
}
async function removeDuty(d: Duty) {
  if (!(await confirmMsg('确定删除职务「' + d.name + '」？'))) return
  try {
    await api.delete('/api/organization/assemble/control/unitduty/' + d.id)
    loadDuties()
  } catch (e: any) {
    toast.error('删除职务失败: ' + (e?.message ?? ''))
  }
}

const list_person_sup_nested_ref = ref<any[]>([])
const unit_list_level_ref = ref<any[]>([])
const unit_list_unit_tree_ref = ref<any[]>([])
const unit_list_identity_ref = ref<any[]>([])
const unit_check_ref = ref<any[]>([])
const unit_list_unitattribute_object_ref = ref<any[]>([])
const unit_identity_type_ref = ref<any[]>([])
const unit_list_identity_object_ref = ref<any[]>([])
const unit_ref = ref<any[]>([])
const list_unit_sup_nested_ref = ref<any[]>([])
const check_unit_has_unit_ref = ref<any[]>([])
const unit_list_person_ref = ref<any[]>([])
const list_type_dept_object_ref = ref<any[]>([])
const unit_list_object_ref = ref<any[]>([])
const check_unit_has_identity_ref = ref<any[]>([])
const unit_list_unitattribute_ref = ref<any[]>([])
const list_unit_sub_nested_ref = ref<any[]>([])
const unit_identity_level_ref = ref<any[]>([])
const unit_identity_type_object_ref = ref<any[]>([])
const list_level_name_object_ref = ref<any[]>([])
const unit_list_unitduty_object_ref = ref<any[]>([])
const unit_identity_ref = ref<any[]>([])
const list_unit_sub_direct_ref = ref<any[]>([])
const unit_list_person_object_ref = ref<any[]>([])
const unit_list_all_object_ref = ref<any[]>([])
const api_or_150_data = ref<any[]>([])
const api_or_60_data = ref<any[]>([])
const organization_assemble_authentication_sso_encrypt_client_u2c_key_u2key_1234_95d5ba_ref = ref<any[]>([])
const api_or_174_data = ref<any[]>([])
const api_organizati_494_data = ref<any[]>([])
const api_organizati_346_data = ref<any[]>([])
const api_organizati_256_data = ref<any[]>([])
const api_organizati_285_data = ref<any[]>([])
const api_organizati_824_data = ref<any[]>([])
const api_organizati_923_data = ref<any[]>([])
const api_organizati_439_data = ref<any[]>([])
const organization_assemble_control_unit_list_test_unit_sub_nested_ref = ref<any[]>([])
const organization_assemble_control_unit_list_test_unit_sup_nested_ref = ref<any[]>([])
const organization_assemble_control_unit_list_test_unit_sup_nested_type_company_ref = ref<any[]>([])
const api_organizati_842_data = ref<any[]>([])
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.toolbar{display:flex;gap:8px}
.search-input{flex:1;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:8px 12px;font-size:14px}
.search-input:focus{outline:none;border-color:var(--color-primary)}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.list-panel{flex:1}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:12px}
.item-card{display:flex;align-items:center;gap:12px;padding:14px;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:600;color:var(--text-primary)}
.im{font-size:12px;color:var(--text-muted);margin-top:2px}
.meta{font-size:11px;color:var(--text-muted);margin-top:2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.btn-sm{padding:4px 10px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-sm);font-size:12px;cursor:pointer}
.btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.detail-head{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px;padding-bottom:12px;border-bottom:1px solid var(--border-subtle)}
.detail-head h2{font-size:16px;color:var(--color-primary);margin:0}
.detail-cols{display:grid;grid-template-columns:1fr 1fr 1fr;gap:16px}
.dcol-head{display:flex;align-items:center;justify-content:space-between;font-size:13px;color:var(--color-primary);font-weight:600;margin-bottom:10px}
.mini-add{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--color-primary);background:var(--color-primary-soft);color:var(--color-primary);cursor:pointer;font-size:12px}
.dlist{display:flex;flex-direction:column;gap:8px}
.drow{display:flex;align-items:center;gap:12px;padding:10px 12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.dmain{display:flex;flex-direction:column;gap:2px;flex:1;min-width:0}
.dname{font-weight:600;color:var(--text-primary);font-size:13px}
.dsub{font-size:12px;color:var(--text-muted)}
.dcol-del{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--color-error);background:var(--color-error-glow);color:var(--color-error);cursor:pointer;font-size:12px}
.empty-sm{color:var(--text-muted);font-size:13px;padding:16px;text-align:center}
@media(max-width:768px){.detail-cols{grid-template-columns:1fr}}@media(min-width:769px) and (max-width:1100px){.detail-cols{grid-template-columns:1fr 1fr}}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
</style>
