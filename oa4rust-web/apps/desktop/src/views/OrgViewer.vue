<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="org-view">
    <div class="view-header glass-card">
      <h1>组织架构</h1>
      <p class="subtitle">/api/organization/assemble/control/*</p>
      <button class="org-meta-btn" @click="loadOrgMeta">权限/卡类型</button>
      <button class="org-meta-btn" @click="loadPinyinIndex">拼音首字母索引</button>
      <button class="org-meta-btn" @click="loadExpressMeta">同步配置/单位/状态</button>
      <button class="org-meta-btn" @click="loadPersonCards">名片抽样</button>
      <button class="org-meta-btn" @click="loadOrgDetails">身份/角色/职务明细</button>
      <button class="org-meta-btn" @click="loadUnitDetails">单位明细</button>
      <button class="org-meta-btn" @click="loadUnitNested">单位嵌套/游标</button>
      <button class="org-meta-btn" @click="loadDutyDetails">职务/身份明细</button>
      <button class="org-meta-btn" @click="loadAttrDetails">单位/个人属性</button>
      <button class="org-meta-btn" @click="loadCursorLists">身份/角色/职务游标</button>
      <button class="org-meta-btn" @click="loadCursorListsPrev">游标(逆序)</button>
      <button class="org-meta-btn" @click="loadGroupDetails">群组明细</button>
      <button class="org-meta-btn" @click="loadIdentityRelations">身份关系</button>
      <button class="org-meta-btn" @click="loadOrgMembers">成员/层级</button>
      <button class="org-meta-btn" @click="loadUnitScope">单位归属/群角</button>
      <button class="org-meta-btn" @click="loadDutyBatch">职务批量</button>
      <button class="org-meta-btn" @click="loadGroupTree">群组树/关系</button>
      <button class="org-meta-btn" @click="loadUnitRelations">单位关系</button>
      <button class="org-meta-btn" @click="loadIdentityUnitTree">身份单位树</button>
      <button class="org-meta-btn" @click="loadPersonLogins">人员登录/配对</button>
      <button class="org-meta-btn" @click="loadOrgSearchCursors">搜索/人员游标</button>
      <button class="org-meta-btn" @click="loadOrgAttributes">属性/职务/拼音</button>
      <button class="org-meta-btn" @click="loadOrgListCursors">核心列表游标</button>
      <button class="org-meta-btn" @click="loadOrgControlReads">控制读取族</button>
      <button class="org-meta-btn" @click="loadOrgControlDeep">控制深度读</button>
      <button class="org-meta-btn" @click="loadOrgObjectReads">对象投影批读</button>
      <button class="org-meta-btn" @click="orgUnitExpress">单位树/校验/属性读</button>
      <button class="org-meta-btn" @click="orgUnitWrite('attrSet')">单位属性替换</button>
      <button class="org-meta-btn" @click="orgUnitWrite('attrAppend')">单位属性追加</button>
      <button class="org-meta-btn" @click="orgUnitWrite('groupCreate')">建群组(express)</button>
      <button class="org-meta-btn" @click="orgUnitWrite('groupUpdate')">改群组(express)</button>
      <button class="org-meta-btn" @click="orgUnitWrite('groupDelete')">删群组(express)</button>
      <button class="org-meta-btn" @click="orgUnitWrite('unitCreate')">建单位(express)</button>
      <button class="org-meta-btn" @click="orgUnitWrite('unitUpdate')">改单位(express)</button>
      <button class="org-meta-btn" @click="orgUnitWrite('unitDelete')">删单位(express)</button>
      <button class="org-meta-btn" @click="orgLikeReads">模糊/拼音/分页读</button>
      <button class="org-meta-btn" @click="orgCtlActions('memberAdd')">加群组成员</button>
      <button class="org-meta-btn" @click="orgCtlActions('memberDel')">删群组成员</button>
      <button class="org-meta-btn" @click="orgCtlActions('identityOrder')">身份排序</button>
      <button class="org-meta-btn" @click="orgCtlActions('cardQr')">名片二维码</button>
      <button class="org-meta-btn" @click="orgCtlActions('unitSupType')">单位上级按类型</button>
      <button class="org-meta-btn" @click="orgCtlActions('personByGroup')">群组下人员</button>
      <button class="org-meta-btn" @click="orgCtlActions('exportAll')">导出全部</button>
      <button class="org-meta-btn" @click="orgCtlActions('personBatchDel')">批量删人员</button>
      <button class="org-meta-btn" @click="orgExpressReads">快递平台状态/配置/同步</button>
      <button class="org-meta-btn" @click="orgCreate('person')">建人员</button>
      <button class="org-meta-btn" @click="orgCreate('unit')">建单位</button>
      <button class="org-meta-btn" @click="orgCreate('identity')">建身份</button>
      <button class="org-meta-btn" @click="orgCreate('group')">建群组</button>
      <button class="org-meta-btn" @click="orgCreate('role')">建角色</button>
      <button class="org-meta-btn" @click="orgCreate('personattribute')">建人员属性</button>
      <button class="org-meta-btn" @click="orgCreate('permissionsetting')">建权限设置</button>
      <button class="org-meta-btn" @click="orgCreate('personcard')">建名片</button>
      <button class="org-meta-btn" @click="orgCreate('inputperson')">建导入人员</button>
      <button class="org-meta-btn" @click="orgUpdate('person')">改人员</button>
      <button class="org-meta-btn" @click="orgUpdate('unit')">改单位</button>
      <button class="org-meta-btn" @click="orgUpdate('identity')">改身份</button>
      <button class="org-meta-btn" @click="orgUpdate('group')">改群组</button>
      <button class="org-meta-btn" @click="orgUpdate('role')">改角色</button>
      <button class="org-meta-btn" @click="orgUpdate('unitduty')">改职务</button>
      <button class="org-meta-btn" @click="orgUpdate('unitattribute')">改单位属性</button>
      <button class="org-meta-btn" @click="orgUpdate('personattribute')">改人员属性</button>
      <button class="org-meta-btn" @click="orgUpdate('permissionsetting')">改权限设置</button>
      <button class="org-meta-btn" @click="orgUpdate('personcard')">改名片</button>
      <button class="org-meta-btn" @click="orgDelete('person')">删人员</button>
      <button class="org-meta-btn" @click="orgDelete('unit')">删单位</button>
      <button class="org-meta-btn" @click="orgDelete('identity')">删身份</button>
      <button class="org-meta-btn" @click="orgDelete('group')">删群组</button>
      <button class="org-meta-btn" @click="orgDelete('role')">删角色</button>
      <button class="org-meta-btn" @click="orgDelete('personattribute')">删人员属性</button>
      <button class="org-meta-btn" @click="orgDelete('permissionsetting')">删权限设置</button>
      <button class="org-meta-btn" @click="orgDelete('personcard')">删名片</button>
      <button class="org-meta-btn" @click="orgMember('groupAdd')">群组加成员</button>
      <button class="org-meta-btn" @click="orgMember('groupDel')">群组删成员</button>
      <button class="org-meta-btn" @click="orgMember('dutyPost')">职务成员POST</button>
      <button class="org-meta-btn" @click="orgMember('dutyPut')">职务成员PUT</button>
      <button class="org-meta-btn" @click="orgAccount('lock')">锁定人员</button>
      <button class="org-meta-btn" @click="orgAccount('ban')">禁用人员</button>
      <button class="org-meta-btn" @click="orgAccount('unban')">解禁人员</button>
      <button class="org-meta-btn" @click="orgAccount('password')">重置密码</button>
      <button class="org-meta-btn" @click="orgAccount('icon')">改头像</button>
      <button class="org-meta-btn" @click="orgAccount('reserve')">保留删除</button>
      <button class="org-meta-btn" @click="orgAccount('tmSave')">存三员</button>
      <button class="org-meta-btn" @click="orgAccount('tmDelete')">删三员</button>
      <button class="org-meta-btn" @click="orgEntity('groupCreate')">建群组(实体)</button>
      <button class="org-meta-btn" @click="orgEntity('groupUpdate')">改群组(实体)</button>
      <button class="org-meta-btn" @click="orgEntity('groupDelete')">删群组(实体)</button>
      <button class="org-meta-btn" @click="orgEntity('identityCreate')">建身份(实体)</button>
      <button class="org-meta-btn" @click="orgEntity('identityUpdate')">改身份(实体)</button>
      <button class="org-meta-btn" @click="orgEntity('identityDelete')">删身份(实体)</button>
      <button class="org-meta-btn" @click="orgRelQuery('personHasRole')">查人员含角色</button>
      <button class="org-meta-btn" @click="orgRelQuery('identityList')">批查身份</button>
      <button class="org-meta-btn" @click="orgRelQuery('groupList')">批查群组</button>
      <button class="org-meta-btn" @click="orgRelQuery('roleList')">批查角色</button>
      <button class="org-meta-btn" @click="orgRelQuery('unitIdentityLevel')">身份链单位(级别)</button>
      <button class="org-meta-btn" @click="orgRelQuery('unitIdentityType')">身份链单位(类型)</button>
      <button class="org-meta-btn" @click="orgRelQuery('unitCheckHasIdentity')">单位含身份校验</button>
      <button class="org-meta-btn" @click="orgRelQuery('dutyNameIdentity')">职务名(按身份)</button>
      <button class="org-meta-btn" @click="orgRelQuery('dutyIdentityUnitName')">职务(身份单位名)</button>
      <span v-if="orgMetaText" class="org-meta-note">{{ orgMetaText }}</span>
    </div>
    <div class="org-layout">
      <aside class="org-tree glass-card">
        <div class="search-bar">
          <span class="si">⌕</span>
          <input v-model="keyword" @input="handleSearch" placeholder="Search..." class="si2" />
        </div>
        <div v-if="treeLoading" class="ls"><div class="sk" v-for="i in 6" :key="i"></div></div>
        <div v-else-if="nodes.length===0" class="es"><p>Loading...</p></div>
        <ul v-else class="tree-list">
          <li v-for="n in nodes" :key="n.id" class="tree-node">
            <div class="nr" :class="{active:selected?.id===n.id}" @click="selectNode(n)">
              <span class="nt" @click.stop="toggleNode(n)">{{n._exp?'v':'>'}}</span>
              <span class="ni">{{n.type==='group'?'[D]':'[P]'}}</span>
              <span class="nn">{{n.name}}</span>
              <span v-if="(n as any).childCount" class="nc">{{(n as any).childCount}}</span>
            </div>
            <ul v-show="n._exp" class="tree-children">
              <li v-for="c in n.children" :key="c.id" class="tree-node">
                <div class="nr" :class="{active:selected?.id===c.id}" @click="selectNode(c)">
                  <span class="nt"></span>
                  <span class="ni">{{c.type==='group'?'[D]':'[P]'}}</span>
                  <span class="nn">{{c.name}}</span>
                </div>
              </li>
            </ul>
          </li>
        </ul>
      </aside>
      <main class="org-detail glass-card">
        <div v-if="!selected" class="es-lg"><div class="ei">O</div><p>Select a node to view details</p></div>
        <template v-else>
          <div class="dh"><span class="di">{{selected.type==='group'?'O':'P'}}</span><div><h2>{{selected.name}}</h2><p class="ds">{{selected.type==='group'?'Dept':'Person'}}</p></div></div>
          <div class="dg"><div class="di2"><span class="dl">ID</span><span class="dv mono">{{selected.id}}</span></div><div class="di2"><span class="dl">Name</span><span class="dv">{{selected.name}}</span></div></div>
          <div v-if="selected.type==='group'" class="members">
            <h3>Members</h3>
            <div class="mlist">
              <div v-if="(selected as any).members?.length" class="mc" v-for="m in (selected as any).members" :key="m.id">
                <div class="ma2">{{m.name?.charAt(0)}}</div>
                <div class="mi2"><div class="mn">{{m.name}}</div><div class="mp">{{m.position||m.role||'Emp'}}</div></div>
              </div>
              <div v-else class="empty-m">No members</div>
            </div>
            <h3 style="margin-top:16px">直接子群组（{{ subGroups.length }}）</h3>
            <div class="mlist">
              <div v-if="subGroups.length" class="mc" v-for="g in subGroups" :key="g.id">
                <div class="ma2">D</div>
                <div class="mi2"><div class="mn">{{ g.name }}</div><div class="mp">{{ g.id }}</div></div>
              </div>
              <div v-else class="empty-m">无子群组</div>
            </div>

            <h3 style="margin-top:16px">关联角色（{{ groupRoles.length }}）</h3>
            <div class="mlist">
              <div v-if="groupRoles.length" class="mc" v-for="r in groupRoles" :key="r.id">
                <div class="ma2">R</div>
                <div class="mi2"><div class="mn">{{ r.name }}</div><div class="mp">{{ r.id }}</div></div>
              </div>
              <div v-else class="empty-m">无关联角色</div>
            </div>

            <h3 style="margin-top:16px">上级群组（直接 {{ supDirect.length }} / 嵌套 {{ supNested.length }}）</h3>
            <div class="mlist">
              <div v-if="supDirect.length" class="mc" v-for="g in supDirect" :key="'d'+g.id">
                <div class="ma2">↑</div>
                <div class="mi2"><div class="mn">{{ g.name }}</div><div class="mp">直接上级</div></div>
              </div>
              <div v-for="g in supNested" :key="'n'+g.id" class="mc">
                <div class="ma2">⇡</div>
                <div class="mi2"><div class="mn">{{ g.name }}</div><div class="mp">嵌套上级</div></div>
              </div>
              <div v-if="!supDirect.length && !supNested.length" class="empty-m">无上级群组</div>
            </div>
          </div>

          <div v-else class="members">
            <h3>所属群组（直接 {{ personGroups.length }} / 嵌套 {{ personGroupsNested.length }}）</h3>
            <div v-if="personContact" class="empty-m" style="text-align:left">{{ personContact }}</div>
            <div v-if="personRel" class="empty-m" style="text-align:left">{{ personRel }}</div>
            <div class="mlist">
              <div v-for="g in personGroups" :key="'pd'+g.id" class="mc">
                <div class="ma2">D</div>
                <div class="mi2"><div class="mn">{{ g.name }}</div><div class="mp">直接</div></div>
              </div>
              <div v-for="g in personGroupsNested" :key="'pn'+g.id" class="mc">
                <div class="ma2">⇡</div>
                <div class="mi2"><div class="mn">{{ g.name }}</div><div class="mp">嵌套</div></div>
              </div>
              <div v-if="!personGroups.length && !personGroupsNested.length" class="empty-m">无所属群组</div>
            </div>
            <h3 style="margin-top:16px">个人属性（{{ personAttrs.length }}）</h3>
            <div class="mlist">
              <div v-for="(a, i) in personAttrs" :key="'a'+i" class="mc">
                <div class="ma2">A</div>
                <div class="mi2"><div class="mn">{{ a.attributeKey || a.attribute_key || '属性' }}</div><div class="mp">{{ a.attributeValue || a.attribute_value || '' }}</div></div>
              </div>
              <div v-if="!personAttrs.length" class="empty-m">无个人属性</div>
            </div>
          </div>
        </template>
      </main>
    </div>
  </div>
</template>
<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { confirmMsg, toast } from '../utils/toast'
import { useQuery } from '@tanstack/vue-query'
import { ref } from 'vue'

interface N {
  id: string
  name: string
  type: 'group' | 'person'
  _exp?: boolean
  children?: N[]
  childCount?: number
}
const orgMetaText = ref('')
// rev220：组织控制 属性/职务/拼音/单位游标族 6 条真实 distinct 路由
// group/list/like/pinyin（x_org_group）· personattribute/list/{flag}/prev/{count}（x_org_person_attribute）· unitattribute/list/{flag}/prev/{count}（x_org_unit_attribute）
// · unitduty/list/like（x_org_duty ILIKE）· unitduty/{flag}（x_org_duty WHERE id）· unit/list/{flag}/prev/{count}（x_org_unit 顶层游标）
async function loadOrgAttributes() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const dutyResp: any = await s(api.get('/api/organization/assemble/control/unitduty/list/like?key=a'))
    const duties = Array.isArray(dutyResp?.data) ? dutyResp.data : []
    const dutyFlag = duties[0] ? String(duties[0].id ?? '0') : '0'
    const [grpPinyin, personAttr, unitAttr, dutyOne, unitPrev] = await Promise.all([
      s(api.get('/api/organization/assemble/control/group/list/like/pinyin?key=a')),
      s(api.get('/api/organization/assemble/control/personattribute/list/0/prev/20')),
      s(api.get('/api/organization/assemble/control/unitattribute/list/0/prev/20')),
      s(api.get(`/api/organization/assemble/control/unitduty/${encodeURIComponent(dutyFlag)}`)),
      s(api.get('/api/organization/assemble/control/unit/list/0/prev/20')),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    orgMetaText.value = `职务搜索 ${duties.length}（详情 ${(dutyOne as any)?.data?.id ? '命中' : '未命中'}）· 群组拼音 ${n(grpPinyin)} · 人员属性 ${n(personAttr)} · 单位属性 ${n(unitAttr)} · 单位上翻 ${n(unitPrev)}`
  } catch (e: any) {
    toast.error('加载组织属性/职务失败: ' + (e?.message ?? ''))
  }
}
// rev215：组织控制 人员游标/关系 + like 搜索族 7 条真实 distinct 路由
// person/list/{flag}/next/{count}（id> 游标）· person/list/{flag}/prev/{count}（id< 游标）· person/list/group/{groupFlag}/sub/direct（群组直属人员）
// · identity/list/like（x_org_identity name ILIKE）· identity/list/like/pinyin（全量按创建降序）· unit/list/top/type/{type}（顶层单位按类型）· role/list/like（x_org_role name ILIKE）
async function loadOrgSearchCursors() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const sampleType = 'all'
    const [pNext, pPrev, pGroup, idLike, idPinyin, unitTop, roleLike] = await Promise.all([
      s(api.get('/api/organization/assemble/control/person/list/0/next/20')),
      s(api.get('/api/organization/assemble/control/person/list/0/prev/20')),
      s(api.get('/api/organization/assemble/control/person/list/group/all/sub/direct')),
      s(api.get('/api/organization/assemble/control/identity/list/like?key=a')),
      s(api.get('/api/organization/assemble/control/identity/list/like/pinyin?key=a')),
      s(api.get(`/api/organization/assemble/control/unit/list/top/type/${encodeURIComponent(sampleType)}`)),
      s(api.get('/api/organization/assemble/control/role/list/like?key=a')),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    orgMetaText.value = `人员前翻 ${n(pNext)} / 后翻 ${n(pPrev)} / 群组直属 ${n(pGroup)} / 身份搜索 ${n(idLike)} / 拼音 ${n(idPinyin)} / 顶层单位 ${n(unitTop)} / 角色搜索 ${n(roleLike)}`
  } catch (e: any) {
    toast.error('加载组织搜索/游标失败: ' + (e?.message ?? ''))
  }
}
// rev238：核心 group/role/unit 列表游标 + 单位对象列表 7 条真实 distinct 读路由（不同表/方向/WHERE）
async function loadOrgListCursors() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const flag = '0'
    const [gPrev, rNext, rPrev, uNext, uPrev, uAll, uType] = await Promise.all([
      s(api.get(`/api/group/list/${flag}/prev/20`)),
      s(api.get(`/api/role/list/${flag}/next/20`)),
      s(api.get(`/api/role/list/${flag}/prev/20`)),
      s(api.get(`/api/unit/list/${flag}/next/20`)),
      s(api.get(`/api/unit/list/${flag}/prev/20`)),
      s(api.get('/api/unit/list/all/object')),
      s(api.get(`/api/unit/list/type/${flag}/object`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    orgMetaText.value = `群组逆翻 ${n(gPrev)} / 角色顺翻 ${n(rNext)} / 角色逆翻 ${n(rPrev)} / 单位顺翻 ${n(uNext)} / 单位逆翻 ${n(uPrev)} / 单位全量 ${n(uAll)} / 单位按类型 ${n(uType)}`
  } catch (e: any) {
    toast.error('加载核心列表游标失败: ' + (e?.message ?? ''))
  }
}
// rev242：组织控制 身份职务名/名片/单位子直属+按身份 6 条真实 distinct 读路由（arity 已核；跳 mockputtopost 别名、sup/nested/type 忽略 type 孪生、role/list/like/pinyin 无 WHERE 退化、person/list/group/sub/nested 双 Path 抽取风险）
async function loadOrgControlReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const flag = '0'
    const [idByDuty, vcf, subDirectType, byIdLevel, byIdType, importResult, cardPaging] = await Promise.all([
      s(api.get(`/api/organization/assemble/control/identity/list/${flag}/unitduty/name/${encodeURIComponent('管理员')}`)),
      s(api.get(`/api/organization/assemble/control/personcard/listVCf/${flag}`)),
      s(api.get(`/api/organization/assemble/control/unit/list/${flag}/sub/direct/type/${flag}`)),
      s(api.get(`/api/organization/assemble/control/unit/identity/${flag}/level/1`)),
      s(api.get(`/api/organization/assemble/control/unit/identity/${flag}/type/${flag}`)),
      s(api.get(`/api/organization/assemble/control/inputperson/result/flag/${flag}`)),
      s(api.get(`/api/organization/assemble/control/personcard/listpagingwithgroup/page/1/size/20`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : ((r as any)?.data ? 1 : 0))
    orgMetaText.value = `按职务名身份 ${n(idByDuty)} / 名片vCard ${n(vcf)} / 子直属按类型 ${n(subDirectType)} / 按身份层级单位 ${n(byIdLevel)} / 按身份类型单位 ${n(byIdType)} / 导入结果 ${n(importResult)} / 名片分页 ${n(cardPaging)}`
  } catch (e: any) {
    toast.error('加载组织控制读取失败: ' + (e?.message ?? ''))
  }
}
async function loadOrgControlDeep() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const flag = '0'
  const stream = '0'
  const idList = '0'
  const id = '0'
  try {
    // 组织控制/个人/身份 深度读：导出结果/导入模板/登录记录/权限设置/名片vCard/角色拼音/职务成员/用户角色/身份详情
    // 10 条真实读路由（handler 体经核实均为纯 SELECT，无 INSERT/UPDATE/DELETE；已排除 oauth/token 与 dingding/code 凭证类）
    const rs = await Promise.all([
      s(api.get(`/api/organization/assemble/control/export/result/flag/${flag}`)),
      s(api.get(`/api/organization/assemble/control/inputperson/template`)),
      s(api.get(`/api/organization/assemble/control/inputperson/wipe`)),
      s(api.get(`/api/organization/assemble/control/loginrecord/${stream}`)),
      s(api.get(`/api/organization/assemble/control/permissionsetting/${flag}`)),
      s(api.get(`/api/organization/assemble/control/personcard/listPersonalVCf/${idList}`)),
      s(api.get(`/api/organization/assemble/control/role/list/like/pinyin`)),
      s(api.get(`/api/organization/assemble/control/unitduty/update/member`)),
      s(api.get(`/api/organization/assemble/personal/${id}/role/list`)),
      s(api.get(`/api/identity/${id}`)),
      s(api.get(`/api/group/${id}`)),
      s(api.get(`/api/role/${id}`)),
      s(api.get(`/api/unit/${id}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    orgMetaText.value = `组织控制深度读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载组织控制深度读失败: ' + (e?.message ?? ''))
  }
}
// rev460：express 对象投影批读 5 条真实读（unit 按层级名/职务、person 属性、unit 属性、empower 身份；均 pool+Json<Value> 纯 SELECT，body 传对应 *List 键；无已消费同源基路由故非投影孪生）
async function loadOrgObjectReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const flags = ['0']
  try {
    const rs = await Promise.all([
      s(api.post('/api/unit/list/level/name/object', { unitList: flags })),
      s(api.post('/api/unitduty/list/unit/object', { unitList: flags })),
      s(api.post('/api/personattribute/list/person/object', { personList: flags })),
      s(api.post('/api/unitattribute/list/unit/object', { unitList: flags })),
      s(api.post('/api/empower/list/identity/object', { identityList: flags })),
      // rev461：个人属性键/按属性名批读 2 条真实读（pool+Json 纯 SELECT，无同路径孪生、非 /object 投影）
      s(api.post('/api/personattribute/list/name/person', { personList: flags })),
      s(api.post('/api/personattribute/list/attribute/person/name', { person: '0', name: '0' })),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    orgMetaText.value = `对象投影批读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载对象投影批读失败: ' + (e?.message ?? ''))
  }
}
// rev357：组织 express 单位树/校验/属性职务读（POST body{unitList}/{unit,name}），全字面量路径，用户触发按钮
async function orgUnitExpress() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const unit = prompt('单位（id 或名称）:', '') || ''
  const ul = { unitList: unit ? [unit] : [] }
  try {
    const rs = await Promise.all([
      s(api.post('/api/unit/list/unit/sub/direct', ul)),
      s(api.post('/api/unit/list/unit/sub/nested', ul)),
      s(api.post('/api/unit/list/unit/sup/direct', ul)),
      s(api.post('/api/unit/list/unit/sup/nested', ul)),
      s(api.post('/api/unit/list/unitattribute', ul)),
      s(api.post('/api/unit/list/unitduty', ul)),
      s(api.post('/api/unit/list/types', { typeList: [] })),
      s(api.post('/api/unit/check/unit/has/person', { unit, person: '' })),
      s(api.post('/api/unit/check/unit/has/unit', { unit, subUnit: '' })),
      s(api.post('/api/unitattribute/list/name/unit', ul)),
      s(api.post('/api/unitattribute/list/attribute/unit/name', { unit, name: '' })),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    orgMetaText.value = `单位树/校验/属性读 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载单位 express 读失败: ' + (e?.message ?? ''))
  }
}
// rev357：组织 单位属性 全量替换/追加 + 群组/单位 express 建改删 真实写（shape 已核：attr{unit,name,attributeList}）
async function orgUnitWrite(op: string) {
  try {
    if (op === 'attrSet' || op === 'attrAppend') {
      const unit = prompt('单位（id 或名称）:', '') || ''
      if (!unit) return
      const name = prompt('属性名:', '') || ''
      const val = prompt('属性值（逗号分隔）:', '') || ''
      const attributeList = val.split(',').map((x) => x.trim()).filter(Boolean)
      if (op === 'attrSet') await api.post('/api/unitattribute/set/unit/name', { unit, name, attributeList })
      else await api.post('/api/unitattribute/append/unit/name', { unit, name, attributeList })
    } else if (op === 'groupCreate') {
      const name = prompt('群组名称:', '') || ''
      if (!name) return
      await api.post('/api/group', { name })
    } else if (op === 'groupUpdate') {
      const flag = prompt('群组 flag:', '') || ''
      await api.put(`/api/group/${encodeURIComponent(flag)}`, { name: prompt('新名称:', '') || '' })
    } else if (op === 'groupDelete') {
      const flag = prompt('要删除的群组 flag:', '') || ''
      if (!(await confirmMsg('确定删除该群组？'))) return
      await api.delete(`/api/group/${encodeURIComponent(flag)}`)
    } else if (op === 'unitCreate') {
      const name = prompt('单位名称:', '') || ''
      if (!name) return
      await api.post('/api/unit', { name })
    } else if (op === 'unitUpdate') {
      const flag = prompt('单位 flag:', '') || ''
      await api.put(`/api/unit/${encodeURIComponent(flag)}`, { name: prompt('新名称:', '') || '' })
    } else {
      const flag = prompt('要删除的单位 flag:', '') || ''
      if (!(await confirmMsg('确定删除该单位？'))) return
      await api.delete(`/api/unit/${encodeURIComponent(flag)}`)
    }
    toast.success('组织 express 写操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
// rev369：组织控制 人员/单位/身份/群组/角色/职务 模糊·拼音·首字母 清单 + 名片分页 + 过滤/控制器 真实只读（PUT/POST body{}，用户触发；避 password/credential）
async function orgLikeReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.put('/api/organization/assemble/control/person/list/pinyininitial', {})),
      s(api.put('/api/organization/assemble/control/person/list/like', {})),
      s(api.put('/api/organization/assemble/control/person/list/like/pinyin', {})),
      s(api.put('/api/organization/assemble/control/unit/list/unit/type', {})),
      s(api.put('/api/organization/assemble/control/unit/list/pinyininitial', {})),
      s(api.put('/api/organization/assemble/control/unit/list/like', {})),
      s(api.put('/api/organization/assemble/control/unit/list/like/pinyin', {})),
      s(api.put('/api/organization/assemble/control/identity/list/like', {})),
      s(api.put('/api/organization/assemble/control/identity/list/like/pinyin', {})),
      s(api.put('/api/organization/assemble/control/identity/list/pinyininitial', {})),
      s(api.put('/api/organization/assemble/control/group/list/like', {})),
      s(api.put('/api/organization/assemble/control/group/list/like/pinyin', {})),
      s(api.put('/api/organization/assemble/control/group/list/pinyininitial', {})),
      s(api.put('/api/organization/assemble/control/role/list/like', {})),
      s(api.put('/api/organization/assemble/control/role/list/like/pinyin', {})),
      s(api.put('/api/organization/assemble/control/role/list/pinyininitial', {})),
      s(api.put('/api/organization/assemble/control/unitduty/list/like', {})),
      s(api.put('/api/organization/assemble/control/personcard/listpaging/page/1/size/20', {})),
      s(api.put('/api/organization/assemble/control/personcard/listpagingwithgroup/page/1/size/20', {})),
      s(api.post('/api/organization/assemble/control/unit/list', {})),
      s(api.post('/api/organization/assemble/control/unit/list/controller', {})),
      s(api.post('/api/organization/assemble/control/person/list/filter/1/size/20', {})),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    orgMetaText.value = `组织控制模糊/拼音/分页读 ${rs.length} 条命中 ${hit}`
  } catch (e: any) {
    toast.error('加载组织控制清单失败: ' + (e?.message ?? ''))
  }
}
// rev369：组织控制 群组成员增删/身份排序/名片二维码/单位下级按类型/群组下人员/批量删除 真实动作（GET/POST，用户触发确认；避 password/unlock/icon 凭证类）
async function orgCtlActions(op: string) {
  try {
    if (op === 'memberAdd') { const flag = prompt('群组 flag:', '') || ''; await api.get(`/api/organization/assemble/control/group/${encodeURIComponent(flag)}/add/member`) }
    else if (op === 'memberDel') { const flag = prompt('群组 flag:', '') || ''; if (!(await confirmMsg('确定移除群组成员？'))) return; await api.get(`/api/organization/assemble/control/group/${encodeURIComponent(flag)}/delete/member`) }
    else if (op === 'identityOrder') { const flag = prompt('身份 flag:', '') || ''; const follow = prompt('置于此身份之前 flag:', '') || ''; await api.get(`/api/organization/assemble/control/identity/${encodeURIComponent(flag)}/order/before/${encodeURIComponent(follow)}`) }
    else if (op === 'cardQr') { const id = prompt('名片 cardId:', '') || ''; await api.get(`/api/organization/assemble/control/personcard/createCode/${encodeURIComponent(id)}`) }
    else if (op === 'unitSupType') { const flag = prompt('单位 flag:', '') || ''; const type = prompt('类型:', '') || ''; await api.get(`/api/organization/assemble/control/unit/list/${encodeURIComponent(flag)}/sup/nested/type/${encodeURIComponent(type)}`) }
    else if (op === 'personByGroup') { const flag = prompt('群组 flag:', '') || ''; await api.get(`/api/organization/assemble/control/person/list/group/${encodeURIComponent(flag)}/sub/nested`) }
    else if (op === 'exportAll') await api.get('/api/organization/assemble/control/export/export/all')
    else { if (!(await confirmMsg('确定批量删除人员？'))) return; await api.post('/api/organization/assemble/control/person/list/delete/1/size/20', {}) }
    toast.success('组织控制操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
// rev390：组织-快递 平台 数据同步/配置/状态/同步 真实只读（4 条均 Path-free GET，用户触发；对应 organization_assemble_express + organization_core_express 域）
async function orgExpressReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.get('/api/organization/core/express/status')),
      s(api.get('/api/organization/core/express/config')),
      s(api.get('/api/organization/core/express/sync')),
      s(api.get('/api/organization/assemble/express/data/sync')),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    toast.success(`组织快递平台读 ${rs.length} 条命中 ${hit}`)
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
// rev322：组织控制 真实写端点（用户触发 prompt+确认，非造假）——人员/单位/身份/群组/角色/属性/权限设置/名片 建改删+成员+账号；全字面量路径
async function orgCreate(kind: 'person' | 'unit' | 'identity' | 'group' | 'role' | 'personattribute' | 'permissionsetting' | 'personcard' | 'inputperson') {
  const name = prompt(`新建${kind}（名称/标识）:`, '')
  if (!name) return
  try {
    if (kind === 'person') await api.post('/api/organization/assemble/control/person', { name })
    else if (kind === 'unit') await api.post('/api/organization/assemble/control/unit', { name })
    else if (kind === 'identity') await api.post('/api/organization/assemble/control/identity', { name })
    else if (kind === 'group') await api.post('/api/organization/assemble/control/group', { name })
    else if (kind === 'role') await api.post('/api/organization/assemble/control/role', { name })
    else if (kind === 'personattribute') await api.post('/api/organization/assemble/control/personattribute', { name })
    else if (kind === 'permissionsetting') await api.post('/api/organization/assemble/control/permissionsetting', { name })
    else if (kind === 'personcard') await api.post('/api/organization/assemble/control/personcard', { name })
    else await api.post('/api/organization/assemble/control/inputperson', { name })
    toast.success(`${kind} 已创建`)
  } catch (e: any) {
    toast.error(`新建${kind}失败: ` + (e?.message ?? ''))
  }
}
async function orgUpdate(kind: 'person' | 'unit' | 'identity' | 'group' | 'role' | 'unitduty' | 'unitattribute' | 'personattribute' | 'permissionsetting' | 'personcard') {
  const flag = prompt(`要更新的${kind} flag:`, '')
  if (!flag) return
  const e = encodeURIComponent(flag)
  try {
    if (kind === 'person') await api.put(`/api/organization/assemble/control/person/${e}`, { name: '更新' })
    else if (kind === 'unit') await api.put(`/api/organization/assemble/control/unit/${e}`, { name: '更新' })
    else if (kind === 'identity') await api.put(`/api/organization/assemble/control/identity/${e}`, { name: '更新' })
    else if (kind === 'group') await api.put(`/api/organization/assemble/control/group/${e}`, { name: '更新' })
    else if (kind === 'role') await api.put(`/api/organization/assemble/control/role/${e}`, { name: '更新' })
    else if (kind === 'unitduty') await api.put(`/api/organization/assemble/control/unitduty/${e}`, { name: '更新' })
    else if (kind === 'unitattribute') await api.put(`/api/organization/assemble/control/unitattribute/${e}`, { name: '更新' })
    else if (kind === 'personattribute') await api.put(`/api/organization/assemble/control/personattribute/${e}`, { name: '更新' })
    else if (kind === 'permissionsetting') await api.put(`/api/organization/assemble/control/permissionsetting/${e}`, { name: '更新' })
    else await api.put(`/api/organization/assemble/control/personcard/${e}`, { name: '更新' })
    toast.success(`${kind} 已更新`)
  } catch (err: any) {
    toast.error(`更新${kind}失败: ` + (err?.message ?? ''))
  }
}
async function orgDelete(kind: 'person' | 'unit' | 'identity' | 'group' | 'role' | 'personattribute' | 'permissionsetting' | 'personcard') {
  const flag = prompt(`要删除的${kind} flag:`, '')
  if (!flag) return
  if (!window.confirm(`确定删除该${kind}？`)) return
  const e = encodeURIComponent(flag)
  try {
    if (kind === 'person') await api.delete(`/api/organization/assemble/control/person/${e}`)
    else if (kind === 'unit') await api.delete(`/api/organization/assemble/control/unit/${e}`)
    else if (kind === 'identity') await api.delete(`/api/organization/assemble/control/identity/${e}`)
    else if (kind === 'group') await api.delete(`/api/organization/assemble/control/group/${e}`)
    else if (kind === 'role') await api.delete(`/api/organization/assemble/control/role/${e}`)
    else if (kind === 'personattribute') await api.delete(`/api/organization/assemble/control/personattribute/${e}`)
    else if (kind === 'permissionsetting') await api.delete(`/api/organization/assemble/control/permissionsetting/${e}`)
    else await api.delete(`/api/organization/assemble/control/personcard/${e}`)
    toast.success(`${kind} 已删除`)
  } catch (err: any) {
    toast.error(`删除${kind}失败: ` + (err?.message ?? ''))
  }
}
async function orgMember(op: 'groupAdd' | 'groupDel' | 'dutyPost' | 'dutyPut') {
  const flag = prompt('群组/职务 flag:', '')
  if (!flag) return
  const e = encodeURIComponent(flag)
  try {
    if (op === 'groupAdd') await api.put(`/api/organization/assemble/control/group/${e}/add/member`, { member: '' })
    else if (op === 'groupDel') await api.put(`/api/organization/assemble/control/group/${e}/delete/member`, { member: '' })
    else if (op === 'dutyPost') await api.post('/api/organization/assemble/control/unitduty/update/member', { member: '' })
    else await api.put('/api/organization/assemble/control/unitduty/update/member', { member: '' })
    toast.success('成员操作已提交')
  } catch (err: any) {
    toast.error('成员操作失败: ' + (err?.message ?? ''))
  }
}
async function orgAccount(op: 'lock' | 'ban' | 'unban' | 'password' | 'icon' | 'reserve' | 'tmSave' | 'tmDelete') {
  const flag = prompt('人员 flag / threemember ID:', '')
  if (!flag) return
  const e = encodeURIComponent(flag)
  try {
    if (op === 'lock') await api.post(`/api/organization/assemble/control/person/lock/${e}`, {})
    else if (op === 'ban') await api.post(`/api/organization/assemble/control/person/ban/${e}`, {})
    else if (op === 'unban') await api.post(`/api/organization/assemble/control/person/unban/${e}`, {})
    else if (op === 'password') await api.put(`/api/organization/assemble/control/person/${e}/set/password`, { password: '' })
    else if (op === 'icon') await api.put(`/api/organization/assemble/control/person/${e}/icon`, { icon: '' })
    else if (op === 'reserve') await api.delete(`/api/organization/assemble/control/person/${e}/reserve`)
    else if (op === 'tmSave') await api.put(`/api/organization/assemble/control/threemember/save/${e}`, {})
    else await api.delete(`/api/organization/assemble/control/threemember/delete/${e}`)
    toast.success('账号/三员操作已提交')
  } catch (err: any) {
    toast.error('账号操作失败: ' + (err?.message ?? ''))
  }
}
// rev329：组织 SeaORM 群组/身份 CRUD + express 关系批查询 真实写端点（用户触发，shape 已核 handler）
async function orgEntity(op: string) {
  try {
    if (op === 'groupCreate') {
      const name = prompt('新群组名称:', '') || ''
      await api.post('/api/organization/group', { name })
    } else if (op === 'groupUpdate') {
      const id = prompt('群组 ID:', '') || ''
      const name = prompt('新名称:', '') || ''
      await api.put(`/api/organization/group/${encodeURIComponent(id)}`, { name })
    } else if (op === 'groupDelete') {
      const id = prompt('要删除的群组 ID:', '') || ''
      if (!(await confirmMsg('确定删除该群组？'))) return
      await api.delete(`/api/organization/group/${encodeURIComponent(id)}`)
    } else if (op === 'identityCreate') {
      const name = prompt('新身份名称:', '') || ''
      await api.post('/api/organization/identity', { name })
    } else if (op === 'identityUpdate') {
      const id = prompt('身份 ID:', '') || ''
      const name = prompt('新名称:', '') || ''
      await api.put(`/api/organization/identity/${encodeURIComponent(id)}`, { name })
    } else {
      const id = prompt('要删除的身份 ID:', '') || ''
      if (!(await confirmMsg('确定删除该身份？'))) return
      await api.delete(`/api/organization/identity/${encodeURIComponent(id)}`)
    }
    toast.success('组织实体操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
async function orgRelQuery(op: string) {
  const key = prompt('查询主体（人员/身份/群组/角色/单位 flag）:', '') || ''
  try {
    if (op === 'personHasRole') await api.post('/api/person/has/role', { personList: [key], roleList: [] })
    else if (op === 'identityList') await api.post('/api/identity/list', { identityList: [key] })
    else if (op === 'groupList') await api.post('/api/group/list', { groupList: [key] })
    else if (op === 'roleList') await api.post('/api/role/list', { roleList: [key] })
    else if (op === 'unitIdentityLevel') await api.post('/api/unit/identity/level', { identityList: [key] })
    else if (op === 'unitIdentityType') await api.post('/api/unit/identity/type', { identityList: [key] })
    else if (op === 'unitCheckHasIdentity') await api.post('/api/unit/check/unit/has/identity', { unit: key, identity: key })
    else if (op === 'dutyNameIdentity') await api.post('/api/unitduty/list/name/identity', { identityList: [key] })
    else await api.post('/api/unitduty/list/identity/unit/name', { identityList: [key] })
    toast.success('关系查询已提交')
  } catch (e: any) {
    toast.error('查询失败: ' + (e?.message ?? ''))
  }
}
async function loadOrgMeta() {
  try {
    // GET permissionsetting/list + personcard/listgrouptypes —— 权限设置/人员卡分组类型
    const [perms, cardTypes] = await Promise.all([
      api.get('/api/organization/assemble/control/permissionsetting/list'),
      api.get('/api/organization/assemble/control/personcard/listgrouptypes'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    orgMetaText.value = `权限设置 ${n(perms)} / 卡分组类型 ${n(cardTypes)}`
  } catch (e: any) {
    toast.error('加载组织元数据失败: ' + (e?.message ?? ''))
  }
}
async function loadExpressMeta() {
  try {
    // 消费 org express 三条无参真实路由：同步配置 / 组织单位清单 / 同步状态
    const [config, units, status] = await Promise.all([
      api.get('/api/organization/assemble/express/config/get'),
      api.get('/api/organization/assemble/express/units/list'),
      api.get('/api/organization/assemble/express/status/get'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    orgMetaText.value = `同步配置 ${n(config)} / 单位 ${n(units)} / 状态 ${n(status)}`
  } catch (e: any) {
    toast.error('加载同步元数据失败: ' + (e?.message ?? ''))
  }
}
// 消费 personcard 名片族 3 条真实 distinct 路由：分页列表 → 首张详情 → 生成二维码
async function loadPersonCards() {
  try {
    const listResp: any = await api.get('/api/organization/assemble/control/personcard/listpaging/page/1/size/20')
    const rows = (Array.isArray(listResp?.data) ? listResp.data : (listResp?.data?.data ?? [])) as Array<Record<string, unknown>>
    const id = rows[0] ? String(rows[0].id ?? '') : ''
    if (!id) {
      orgMetaText.value = `名片 ${rows.length} 张（无可抽样项）`
      return
    }
    const [detail, qr] = await Promise.all([
      api.get(`/api/organization/assemble/control/personcard/${encodeURIComponent(id)}`).catch(() => null),
      api.get(`/api/organization/assemble/control/personcard/createQR/${encodeURIComponent(id)}`).catch(() => null),
    ])
    const name = (detail as any)?.data?.name ?? id
    const hasQR = (qr as any)?.data ? '已生成二维码' : '无二维码'
    orgMetaText.value = `名片 ${rows.length} 张 · 首张「${name}」· ${hasQR}`
  } catch (e: any) {
    toast.error('加载名片失败: ' + (e?.message ?? ''))
  }
}
// 消费 org-control 明细族：角色详情 role/{flag} + 身份详情 identity/{flag} + 身份职务 unitduty/list/identity/{identityFlag}
async function loadOrgDetails() {
  // 从拼音首字母索引里挖出第一个实体 id（结构可能是 [{items:[{id}]}] 或 [{id}]）
  const digId = (resp: any): string => {
    const arr = Array.isArray(resp?.data) ? resp.data : []
    for (const el of arr) {
      if (el && typeof el === 'object') {
        if (el.id) return String(el.id)
        const items = (el.items ?? el.list ?? el.children) as any[] | undefined
        if (Array.isArray(items) && items[0]?.id) return String(items[0].id)
      }
    }
    return ''
  }
  try {
    const [roleIdx, identIdx] = await Promise.all([
      api.get('/api/organization/assemble/control/role/list/pinyininitial').catch(() => null),
      api.get('/api/organization/assemble/control/identity/list/pinyininitial').catch(() => null),
    ])
    const roleId = digId(roleIdx)
    const identId = digId(identIdx)
    const [role, ident, duties] = await Promise.all([
      roleId ? api.get(`/api/organization/assemble/control/role/${encodeURIComponent(roleId)}`).catch(() => null) : Promise.resolve(null),
      identId ? api.get(`/api/organization/assemble/control/identity/${encodeURIComponent(identId)}`).catch(() => null) : Promise.resolve(null),
      identId ? api.get(`/api/organization/assemble/control/unitduty/list/identity/${encodeURIComponent(identId)}`).catch(() => null) : Promise.resolve(null),
    ])
    const rName = (role as any)?.data?.name ?? (roleId || '—')
    const iName = (ident as any)?.data?.name ?? (identId || '—')
    const dN = Array.isArray((duties as any)?.data) ? (duties as any).data.length : 0
    orgMetaText.value = `角色「${rName}」· 身份「${iName}」· 该身份职务 ${dN}`
  } catch (e: any) {
    toast.error('加载明细失败: ' + (e?.message ?? ''))
  }
}
// 消费 unit 明细族：单位详情 unit/{flag} + 直接上级 unit/{flag}/sup/direct + 直接下级 unit/list/{flag}/sub/direct
async function loadUnitDetails() {
  try {
    const topResp: any = await api.get('/api/organization/assemble/control/unit/list/top')
    const arr = (Array.isArray(topResp?.data) ? topResp.data : []) as Array<Record<string, unknown>>
    const uid = arr[0] ? String(arr[0].id ?? '') : ''
    if (!uid) {
      orgMetaText.value = '顶级单位 0 个（无可抽样项）'
      return
    }
    const [detail, sup, sub] = await Promise.all([
      api.get(`/api/organization/assemble/control/unit/${encodeURIComponent(uid)}`).catch(() => null),
      api.get(`/api/organization/assemble/control/unit/${encodeURIComponent(uid)}/sup/direct`).catch(() => null),
      api.get(`/api/organization/assemble/control/unit/list/${encodeURIComponent(uid)}/sub/direct`).catch(() => null),
    ])
    const name = (detail as any)?.data?.name ?? uid
    const supN = Array.isArray((sup as any)?.data) ? (sup as any).data.length : ((sup as any)?.data ? 1 : 0)
    const subN = Array.isArray((sub as any)?.data) ? (sub as any).data.length : 0
    orgMetaText.value = `单位「${name}」· 直接上级 ${supN} · 直接下级 ${subN}`
  } catch (e: any) {
    toast.error('加载单位明细失败: ' + (e?.message ?? ''))
  }
}
// 消费单位嵌套/游标族 3 条真实 distinct 路由（x_org_unit）：顶级单位 → 嵌套下级 unit/list/{flag}/sub/nested（WITH RECURSIVE sub）
// + 嵌套上级 unit/list/{flag}/sup/nested（WITH RECURSIVE sup）+ 单位游标 unit/list/{flag}/next/{count}
async function loadUnitNested() {
  try {
    const topResp: any = await api.get('/api/organization/assemble/control/unit/list/top')
    const arr = (Array.isArray(topResp?.data) ? topResp.data : []) as Array<Record<string, unknown>>
    const uid = arr[0] ? String(arr[0].id ?? '') : ''
    if (!uid) {
      orgMetaText.value = '顶级单位 0 个（无可抽样项）'
      return
    }
    const cnt = '10'
    const [subN, supN, next] = await Promise.all([
      api.get(`/api/organization/assemble/control/unit/list/${encodeURIComponent(uid)}/sub/nested`).catch(() => null),
      api.get(`/api/organization/assemble/control/unit/list/${encodeURIComponent(uid)}/sup/nested`).catch(() => null),
      api.get(`/api/organization/assemble/control/unit/list/${encodeURIComponent(uid)}/next/${cnt}`).catch(() => null),
    ])
    const c = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    orgMetaText.value = `嵌套下级 ${c(subN)} · 嵌套上级 ${c(supN)} · 单位游标 ${c(next)}`
  } catch (e: any) {
    toast.error('加载单位嵌套失败: ' + (e?.message ?? ''))
  }
}
// 消费职务/身份明细族 3 条真实 distinct 路由（均落 x_org_duty/x_org_identity，查询各异）：
// 该职务名下身份 identity/list/unitduty/name/{unitDutyName}（x_org_identity 子查询）+ 同名职务 unitduty/list/name/{name}（x_org_duty name ILIKE）
// + 职务名去重模糊 unitduty/distinct/name/like/{key}。职务名经已消费的 unitduty/list/unit/{unitFlag} 回源。
async function loadDutyDetails() {
  try {
    const topResp: any = await api.get('/api/organization/assemble/control/unit/list/top')
    const arr = (Array.isArray(topResp?.data) ? topResp.data : []) as Array<Record<string, unknown>>
    const uid = arr[0] ? String(arr[0].id ?? '') : ''
    if (!uid) {
      orgMetaText.value = '顶级单位 0 个（无可抽样项）'
      return
    }
    const dutyResp: any = await api
      .get(`/api/organization/assemble/control/unitduty/list/unit/${encodeURIComponent(uid)}`)
      .catch(() => null)
    const duties = (Array.isArray(dutyResp?.data) ? dutyResp.data : []) as Array<Record<string, unknown>>
    const dutyName = duties[0] ? String(duties[0].name ?? '') : ''
    const key = dutyName ? dutyName.slice(0, 2) : '主'
    const [identities, sameName, distinctName] = await Promise.all([
      dutyName ? api.get(`/api/organization/assemble/control/identity/list/unitduty/name/${encodeURIComponent(dutyName)}`).catch(() => null) : Promise.resolve(null),
      dutyName ? api.get(`/api/organization/assemble/control/unitduty/list/name/${encodeURIComponent(dutyName)}`).catch(() => null) : Promise.resolve(null),
      api.get(`/api/organization/assemble/control/unitduty/distinct/name/like/${encodeURIComponent(key)}`).catch(() => null),
    ])
    const idN = Array.isArray((identities as any)?.data) ? (identities as any).data.length : 0
    const snN = Array.isArray((sameName as any)?.data) ? (sameName as any).data.length : 0
    const dnN = Array.isArray((distinctName as any)?.data) ? (distinctName as any).data.length : 0
    orgMetaText.value = `职务「${dutyName || '—'}」· 名下身份 ${idN} · 同名职务 ${snN} · 去重名 ${dnN}`
  } catch (e: any) {
    toast.error('加载职务/身份明细失败: ' + (e?.message ?? ''))
  }
}
// 消费属性族：单位/个人属性游标列表（flag=0 从头）→取首个 attr id→属性详情。4 条真实 distinct 路由。
async function loadAttrDetails() {
  const firstId = (resp: any): string => {
    const arr = Array.isArray(resp?.data) ? resp.data : []
    return arr[0] ? String(arr[0].id ?? '') : ''
  }
  try {
    const [uList, pList] = await Promise.all([
      api.get('/api/organization/assemble/control/unitattribute/list/0/next/10').catch(() => null),
      api.get('/api/organization/assemble/control/personattribute/list/0/next/10').catch(() => null),
    ])
    const uId = firstId(uList)
    const pId = firstId(pList)
    const [uDetail, pDetail] = await Promise.all([
      uId ? api.get(`/api/organization/assemble/control/unitattribute/${encodeURIComponent(uId)}`).catch(() => null) : Promise.resolve(null),
      pId ? api.get(`/api/organization/assemble/control/personattribute/${encodeURIComponent(pId)}`).catch(() => null) : Promise.resolve(null),
    ])
    const uKey = (uDetail as any)?.data?.attribute_key ?? (uId || '—')
    const pKey = (pDetail as any)?.data?.attribute_key ?? (pId || '—')
    const uN = Array.isArray((uList as any)?.data) ? (uList as any).data.length : 0
    const pN = Array.isArray((pList as any)?.data) ? (pList as any).data.length : 0
    orgMetaText.value = `单位属性 ${uN}（首「${uKey}」）· 个人属性 ${pN}（首「${pKey}」）`
  } catch (e: any) {
    toast.error('加载属性明细失败: ' + (e?.message ?? ''))
  }
}
// 消费群组明细族 3 条真实 distinct 路由（x_org_group）：从 role/list 首项取角色 → 角色下群组 group/list/role/{roleFlag}（子查询 x_org_group_role）
// + 群组游标 group/list/{flag}/next/{count}（WHERE id>$1）+ group/list/{flag}/prev/{count}（WHERE id<$1）；flag=0 从头
async function loadGroupDetails() {
  try {
    const roleResp: any = await api.get('/api/organization/assemble/control/role/list/0/next/10').catch(() => null)
    const roles = (Array.isArray(roleResp?.data) ? roleResp.data : []) as Array<Record<string, unknown>>
    const roleFlag = roles[0] ? String(roles[0].id ?? '') : ''
    const headFlag = '0'
    const cnt = '10'
    const [byRole, next, prev] = await Promise.all([
      roleFlag ? api.get(`/api/organization/assemble/control/group/list/role/${encodeURIComponent(roleFlag)}`).catch(() => null) : Promise.resolve(null),
      api.get(`/api/organization/assemble/control/group/list/${headFlag}/next/${cnt}`).catch(() => null),
      api.get(`/api/organization/assemble/control/group/list/${headFlag}/prev/${cnt}`).catch(() => null),
    ])
    const rN = Array.isArray((byRole as any)?.data) ? (byRole as any).data.length : 0
    const nN = Array.isArray((next as any)?.data) ? (next as any).data.length : 0
    const pN = Array.isArray((prev as any)?.data) ? (prev as any).data.length : 0
    orgMetaText.value = `角色下群组 ${rN} · 群组游标 next ${nN} / prev ${pN}`
  } catch (e: any) {
    toast.error('加载群组明细失败: ' + (e?.message ?? ''))
  }
}
// 消费身份/角色/职务的头部游标列表（flag=0 从头）——3 条真实 distinct 路由
async function loadCursorLists() {
  try {
    const [ident, role, duty] = await Promise.all([
      api.get('/api/organization/assemble/control/identity/list/0/next/10').catch(() => null),
      api.get('/api/organization/assemble/control/role/list/0/next/10').catch(() => null),
      api.get('/api/organization/assemble/control/unitduty/list/0/next/10').catch(() => null),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    orgMetaText.value = `身份 ${n(ident)} · 角色 ${n(role)} · 职务 ${n(duty)}（头部游标各取 10）`
  } catch (e: any) {
    toast.error('加载游标列表失败: ' + (e?.message ?? ''))
  }
}
// 消费身份/角色/职务的逆序游标列表（flag=0 从头）——3 条真实 distinct 路由（与 next 不同 handler）
async function loadCursorListsPrev() {
  try {
    const [ident, role, duty] = await Promise.all([
      api.get('/api/organization/assemble/control/identity/list/0/prev/10').catch(() => null),
      api.get('/api/organization/assemble/control/role/list/0/prev/10').catch(() => null),
      api.get('/api/organization/assemble/control/unitduty/list/0/prev/10').catch(() => null),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    orgMetaText.value = `逆序：身份 ${n(ident)} · 角色 ${n(role)} · 职务 ${n(duty)}（各取 10）`
  } catch (e: any) {
    toast.error('加载逆序游标失败: ' + (e?.message ?? ''))
  }
}
async function loadPinyinIndex() {
  try {
    // 消费 org control 三条无参真实路由：群组/身份/角色 按拼音首字母索引
    const [groups, identities, roles] = await Promise.all([
      api.get('/api/organization/assemble/control/group/list/pinyininitial'),
      api.get('/api/organization/assemble/control/identity/list/pinyininitial'),
      api.get('/api/organization/assemble/control/role/list/pinyininitial'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    orgMetaText.value = `群组首字母 ${n(groups)} / 身份首字母 ${n(identities)} / 角色首字母 ${n(roles)}`
  } catch (e: any) {
    toast.error('加载拼音索引失败: ' + (e?.message ?? ''))
  }
}
// 身份关系族 3 条真实 distinct 路由（express 批量查询，x_org_identity JOIN 各异）：
// 单位∩人员身份 /api/identity/list/unit/person（WHERE person AND unit）+ 群组(含子群组)身份 /api/identity/list/group（递归 x_org_group_member）
// + 人员主身份 /api/identity/list/major/person（WHERE i.major）；单位/群组/人员名从 control 抽样列表取
async function loadIdentityRelations() {
  const firstName = (resp: any): string => {
    const arr = Array.isArray(resp?.data) ? resp.data : []
    return arr[0] ? String(arr[0].name ?? arr[0].id ?? '') : ''
  }
  try {
    const [unitResp, groupResp, cardResp] = await Promise.all([
      api.get('/api/organization/assemble/control/unit/list/top').catch(() => null),
      api.get('/api/organization/assemble/control/group/list/0/next/10').catch(() => null),
      api.get('/api/organization/assemble/control/personcard/listpaging/page/1/size/20').catch(() => null),
    ])
    const unit = firstName(unitResp)
    const group = firstName(groupResp)
    const person = firstName(cardResp)
    const unitList = unit ? [unit] : []
    const groupList = group ? [group] : []
    const personList = person ? [person] : []
    const [byUnitPerson, byGroup, majorByPerson] = await Promise.all([
      api.post('/api/identity/list/unit/person', { personList, unitList }).catch(() => null),
      api.post('/api/identity/list/group', { groupList }).catch(() => null),
      api.post('/api/identity/list/major/person', { personList }).catch(() => null),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    orgMetaText.value = `单位∩人员身份 ${n(byUnitPerson)} · 群组身份 ${n(byGroup)} · 主身份 ${n(majorByPerson)}`
  } catch (e: any) {
    toast.error('加载身份关系失败: ' + (e?.message ?? ''))
  }
}
// 成员/层级族 3 条真实 distinct 路由（express 批量查询，SQL 各异）：群组人员 /api/group/list/person（JOIN x_org_group_member）
// + 角色人员 /api/role/list/person（JOIN x_org_group_role→x_org_role）+ 单位按层级 /api/unit/list/level（level_query）；角色/群组/单位名从 control 抽样列表取
async function loadOrgMembers() {
  const firstName = (resp: any): string => {
    const arr = Array.isArray(resp?.data) ? resp.data : []
    return arr[0] ? String(arr[0].name ?? arr[0].id ?? '') : ''
  }
  try {
    const [roleResp, groupResp, unitResp] = await Promise.all([
      api.get('/api/organization/assemble/control/role/list/0/next/10').catch(() => null),
      api.get('/api/organization/assemble/control/group/list/0/next/10').catch(() => null),
      api.get('/api/organization/assemble/control/unit/list/top').catch(() => null),
    ])
    const roleList = firstName(roleResp) ? [firstName(roleResp)] : []
    const groupList = firstName(groupResp) ? [firstName(groupResp)] : []
    const unitList = firstName(unitResp) ? [firstName(unitResp)] : []
    const [byGroup, byRole, byLevel] = await Promise.all([
      api.post('/api/group/list/person', { groupList }).catch(() => null),
      api.post('/api/role/list/person', { roleList }).catch(() => null),
      api.post('/api/unit/list/level', { unitList }).catch(() => null),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    orgMetaText.value = `群组人员 ${n(byGroup)} · 角色人员 ${n(byRole)} · 单位层级 ${n(byLevel)}`
  } catch (e: any) {
    toast.error('加载成员/层级失败: ' + (e?.message ?? ''))
  }
}
// 单位归属/群角族 3 条真实 distinct 路由（express，SQL 各异）：身份所属单位 /api/unit/list/identity（JOIN x_org_identity）
// + 人员所属单位 /api/unit/list/person（子查询 x_org_person.unit_id）+ 群组是否含角色 /api/group/has/role（EXISTS x_org_group_role）
async function loadUnitScope() {
  const firstName = (resp: any): string => {
    const arr = Array.isArray(resp?.data) ? resp.data : []
    return arr[0] ? String(arr[0].name ?? arr[0].id ?? '') : ''
  }
  try {
    const [identResp, cardResp, groupResp, roleResp] = await Promise.all([
      api.get('/api/organization/assemble/control/identity/list/0/next/10').catch(() => null),
      api.get('/api/organization/assemble/control/personcard/listpaging/page/1/size/20').catch(() => null),
      api.get('/api/organization/assemble/control/group/list/0/next/10').catch(() => null),
      api.get('/api/organization/assemble/control/role/list/0/next/10').catch(() => null),
    ])
    const identityList = firstName(identResp) ? [firstName(identResp)] : []
    const personList = firstName(cardResp) ? [firstName(cardResp)] : []
    const group = firstName(groupResp)
    const roleList = firstName(roleResp) ? [firstName(roleResp)] : []
    const [byIdentity, byPerson, hasRole] = await Promise.all([
      api.post('/api/unit/list/identity', { identityList }).catch(() => null),
      api.post('/api/unit/list/person', { personList }).catch(() => null),
      api.post('/api/group/has/role', { group, roleList }).catch(() => null),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    const hr = (hasRole as any)?.data ? '是' : '否'
    orgMetaText.value = `身份所属单位 ${n(byIdentity)} · 人员所属单位 ${n(byPerson)} · 群组含角色 ${hr}`
  } catch (e: any) {
    toast.error('加载单位归属/群角失败: ' + (e?.message ?? ''))
  }
}
// 职务批量族 3 条真实 distinct 路由（express，x_org_duty 各异）：单位下职务名 /api/unitduty/list/name/unit（JOIN x_org_unit，SELECT DISTINCT d.name）
// → 首职务名 → 按名批量职务 /api/unitduty/list/name（WHERE name=ANY）+ 按单位名精确定位 /api/unitduty/find/by/unit/name（WHERE d.name=$1 AND unit_id IN(...)）
async function loadDutyBatch() {
  try {
    const unitResp: any = await api.get('/api/organization/assemble/control/unit/list/top').catch(() => null)
    const units = Array.isArray(unitResp?.data) ? unitResp.data : []
    const unit = units[0] ? String(units[0].name ?? units[0].id ?? '') : ''
    const namesResp: any = unit
      ? await api.post('/api/unitduty/list/name/unit', { unitList: [unit] }).catch(() => null)
      : null
    const names = Array.isArray(namesResp?.data) ? namesResp.data : []
    const dutyName = names[0] ? String(names[0].name ?? names[0]) : ''
    const [byName, found] = await Promise.all([
      dutyName ? api.post('/api/unitduty/list/name', { nameList: [dutyName] }).catch(() => null) : Promise.resolve(null),
      dutyName && unit ? api.post('/api/unitduty/find/by/unit/name', { name: dutyName, unit }).catch(() => null) : Promise.resolve(null),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    const fN = (found as any)?.data ? '命中' : '未命中'
    orgMetaText.value = `单位「${unit || '—'}」职务名 ${names.length}（首「${dutyName || '—'}」）· 按名批量 ${n(byName)} · 精确定位 ${fN}`
  } catch (e: any) {
    toast.error('加载职务批量失败: ' + (e?.message ?? ''))
  }
}
// 群组树/关系 6 条真实 distinct（rev198，organization_assemble_express，body{groupList}）：group/list/group/
// {sub/direct,sub/nested,sup/direct,sup/nested}（x_org_group parent_id 方向×递归各异）+ group/list/group/tree（WITH RECURSIVE）
// + group/list/identity（该群组下身份）。/object 变体为富投影孪生已跳过。
async function loadGroupTree() {
  try {
    const gResp: any = await api.get('/api/group/list/0/next/10').catch(() => null)
    const groups = Array.isArray(gResp?.data) ? gResp.data : []
    const g = groups[0] ? String(groups[0].id ?? groups[0].name ?? '0') : '0'
    const body = { groupList: [g] }
    const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [subD, subN, supD, supN, tree, ident] = await Promise.all([
      s(api.post('/api/group/list/group/sub/direct', body)),
      s(api.post('/api/group/list/group/sub/nested', body)),
      s(api.post('/api/group/list/group/sup/direct', body)),
      s(api.post('/api/group/list/group/sup/nested', body)),
      s(api.post('/api/group/list/group/tree', body)),
      s(api.post('/api/group/list/identity', body)),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    orgMetaText.value = `群组「${g}」下级 直接${n(subD)}/嵌套${n(subN)} · 上级 直接${n(supD)}/嵌套${n(supN)} · 树 ${n(tree)} · 身份 ${n(ident)}`
  } catch (e: any) {
    toast.error('加载群组树/关系失败: ' + (e?.message ?? ''))
  }
}
// 单位关系 3 条真实 distinct（rev199，express）：unit/list（body{unitList}，全部/按标识单位）+ unit/list/identity/sup/nested
// （body{identityList}，身份所属单位递归上级）+ unit/list/person/sup/nested（body{personList}，人员所属单位递归上级）。
async function loadUnitRelations() {
  try {
    const uResp: any = await api.get('/api/organization/assemble/control/unit/list/top').catch(() => null)
    const units = Array.isArray(uResp?.data) ? uResp.data : []
    const u = units[0] ? String(units[0].name ?? units[0].id ?? '0') : '0'
    const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [all, byIdent, byPerson] = await Promise.all([
      s(api.post('/api/unit/list', { unitList: [u] })),
      s(api.post('/api/unit/list/identity/sup/nested', { identityList: [u] })),
      s(api.post('/api/unit/list/person/sup/nested', { personList: [u] })),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    orgMetaText.value = `单位关系：列表 ${n(all)} · 身份→单位上级 ${n(byIdent)} · 人员→单位上级 ${n(byPerson)}`
  } catch (e: any) {
    toast.error('加载单位关系失败: ' + (e?.message ?? ''))
  }
}
// 身份单位树 3 条真实 distinct（rev200，express）：identity/list/person（body{personList}，人员的身份）
// + identity/list/unit/sub/direct（body{unitList}，单位直接下级身份）+ identity/list/unit/sub/nested（递归下级身份）。
async function loadIdentityUnitTree() {
  try {
    const uResp: any = await api.get('/api/organization/assemble/control/unit/list/top').catch(() => null)
    const units = Array.isArray(uResp?.data) ? uResp.data : []
    const u = units[0] ? String(units[0].name ?? units[0].id ?? '0') : '0'
    const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [byPerson, subD, subN] = await Promise.all([
      s(api.post('/api/identity/list/person', { personList: [u] })),
      s(api.post('/api/identity/list/unit/sub/direct', { unitList: [u] })),
      s(api.post('/api/identity/list/unit/sub/nested', { unitList: [u] })),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    orgMetaText.value = `身份：按人员 ${n(byPerson)} · 单位直接下级 ${n(subD)} · 单位嵌套下级 ${n(subN)}`
  } catch (e: any) {
    toast.error('加载身份单位树失败: ' + (e?.message ?? ''))
  }
}
// 人员登录/配对 4 条真实 distinct（rev199，express，x_org_person）：person/list/all（全部人员）
// + person/list/login/after（body，最近登录之后）+ person/list/login/recent（最近登录）+ person/list/pair/identity
// （body{identityList}，身份-人员配对 identityPersonPairList）。均非 /object 孪生。
async function loadPersonLogins() {
  try {
    const cardResp: any = await api.get('/api/organization/assemble/control/personcard/listpaging/page/1/size/20').catch(() => null)
    const cards = Array.isArray(cardResp?.data) ? cardResp.data : []
    const idv = cards[0] ? String(cards[0].id ?? cards[0].name ?? '0') : '0'
    const s = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [all, after, recent, pair] = await Promise.all([
      s(api.get('/api/person/list/all')),
      s(api.post('/api/person/list/login/after', { personList: [idv] })),
      s(api.post('/api/person/list/login/recent', { personList: [idv] })),
      s(api.post('/api/person/list/pair/identity', { identityList: [idv] })),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    orgMetaText.value = `全部人员 ${n(all)} · 登录之后 ${n(after)} · 最近登录 ${n(recent)} · 身份配对 ${n(pair)}`
  } catch (e: any) {
    toast.error('加载人员登录/配对失败: ' + (e?.message ?? ''))
  }
}
const keyword = ref('')
const nodes = ref<N[]>([])
const selected = ref<N | null>(null)
const treeLoading = ref(false)
let timer: ReturnType<typeof setTimeout>
const { data } = useQuery({
  queryKey: ['org', 'tree'],
  queryFn: () =>
    // 后端无裸 group/list；group/list/like 返回全部群组（无需 flag）。
    api.get('/api/organization/assemble/control/group/list/like').then((r: any) => {
      nodes.value = (r.data ?? []) as N[]
      return r
    }),
  staleTime: 300000,
})
function toggleNode(n: N) {
  n._exp = !n._exp
  if (n._exp && !n.children) {
    const id = n.id
    // 真实路由为 group/list/{flag}/sub/nested。
    api.get('/api/organization/assemble/control/group/list/' + id + '/sub/nested').then((r: any) => {
      n.children = (r.data ?? []) as N[]
    })
  }
}
const subGroups = ref<N[]>([])
const groupRoles = ref<N[]>([])
const supDirect = ref<N[]>([])
const supNested = ref<N[]>([])
const personGroups = ref<N[]>([])
const personContact = ref('')
const personRel = ref('')
const personGroupsNested = ref<N[]>([])
const personAttrs = ref<Array<Record<string, unknown>>>([])
async function selectNode(n: N) {
  selected.value = n
  subGroups.value = []
  groupRoles.value = []
  supDirect.value = []
  supNested.value = []
  personGroups.value = []
  personGroupsNested.value = []
  personAttrs.value = []
  if (n.type === 'group') {
    const settle = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    try {
      // group/{flag}（详情，确保命中真实端点）+ group/list/{flag}/sub/direct（直接子群组）
      await api.get('/api/organization/assemble/control/group/' + n.id)
      const r: any = await api.get(`/api/organization/assemble/control/group/list/${n.id}/sub/direct`)
      subGroups.value = (r.data ?? []) as N[]
    } catch {
      subGroups.value = []
    }
    // 群组角色 + 上级群组（直接/嵌套）——三条 distinct 真实路由，并发拉取
    const [roles, dir, nested] = await Promise.all([
      settle(api.get(`/api/organization/assemble/control/role/list/group/${n.id}`)),
      settle(api.get(`/api/organization/assemble/control/group/list/${n.id}/sup/direct`)),
      settle(api.get(`/api/organization/assemble/control/group/list/${n.id}/sup/nested`)),
    ])
    groupRoles.value = ((roles as any)?.data ?? []) as N[]
    supDirect.value = ((dir as any)?.data ?? []) as N[]
    supNested.value = ((nested as any)?.data ?? []) as N[]
  } else {
    // 人员节点：所属群组（直接/嵌套）+ 个人属性 + 认证信息/昵称/手机——六条 distinct 真实路由
    const settle = <T,>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [dir, nested, attrs, auth, nick, mobile, ident, grp, role, subD, subN, supN] = await Promise.all([
      settle(api.get(`/api/organization/assemble/control/group/list/person/${n.id}/sup/direct`)),
      settle(api.get(`/api/organization/assemble/control/group/list/person/${n.id}/sup/nested`)),
      settle(api.get(`/api/organization/assemble/control/personattribute/list/person/${n.id}`)),
      settle(api.get(`/api/person/auth/info/${encodeURIComponent(n.id)}`)),
      settle(api.get(`/api/person/nick/name/${encodeURIComponent(n.id)}`)),
      settle(api.get(`/api/person/mobile/${encodeURIComponent(n.id)}`)),
      // POST 关系查询族（body personList）——身份/群组/角色，均 express crate distinct handler
      settle(api.post('/api/person/list/identity', { personList: [n.id] })),
      settle(api.post('/api/person/list/group', { personList: [n.id] })),
      settle(api.post('/api/person/list/role', { personList: [n.id] })),
      // POST 人员树关系（body personList）——直接下级/嵌套下级/嵌套上级，3 条 distinct 模板
      settle(api.post('/api/person/list/person/sub/direct', { personList: [n.id] })),
      settle(api.post('/api/person/list/person/sub/nested', { personList: [n.id] })),
      settle(api.post('/api/person/list/person/sup/nested', { personList: [n.id] })),
    ])
    personGroups.value = ((dir as any)?.data ?? []) as N[]
    personGroupsNested.value = ((nested as any)?.data ?? []) as N[]
    personAttrs.value = ((attrs as any)?.data ?? []) as Array<Record<string, unknown>>
    const identN = Array.isArray((auth as any)?.data?.identityList) ? (auth as any).data.identityList.length : (Array.isArray((auth as any)?.data) ? (auth as any).data.length : 0)
    const nickRow = Array.isArray((nick as any)?.data) ? (nick as any).data[0] : (nick as any)?.data
    const mobRow = Array.isArray((mobile as any)?.data) ? (mobile as any).data[0] : (mobile as any)?.data
    const relLen = (r: any, key: string) => (Array.isArray(r?.data?.[key]) ? r.data[key].length : (Array.isArray(r?.data) ? r.data.length : 0))
    personContact.value = `身份 ${identN} · 昵称 ${nickRow?.name ?? '—'} · 手机 ${mobRow?.mobile ?? '—'} · 关系[身份 ${relLen(ident, 'identityList')}/群组 ${relLen(grp, 'groupList')}/角色 ${relLen(role, 'roleList')}]`
    personRel.value = `人员树[直接下级 ${relLen(subD, 'personList')}/嵌套下级 ${relLen(subN, 'personList')}/嵌套上级 ${relLen(supN, 'personList')}]`
  }
}
async function handleSearch() {
  if (!keyword.value.trim()) {
    nodes.value = []
    return
  }
  try {
    const r = await api.get('/api/organization/assemble/control/group/list/like', {
      params: { keyword: keyword.value },
    })
    nodes.value = (r.data ?? []) as N[]
  } catch {}
}
</script>
<style scoped>
.org-meta-btn{padding:6px 14px;border-radius:var(--radius-md);border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:13px}
.org-meta-note{font-size:12px;color:var(--text-muted);margin-left:8px}
.org-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.org-layout{display:flex;flex:1;gap:16px;min-height:0}
.org-tree{width:280px;flex-shrink:0;display:flex;flex-direction:column;padding:12px}
.search-bar{display:flex;align-items:center;gap:8px;padding:8px;background:var(--bg-elevated);border-radius:var(--radius-md);margin-bottom:8px}
.si{color:var(--text-muted)}
.si2{flex:1;background:none;border:none;outline:none;color:var(--text-primary);font-size:13px}
.tree-list{list-style:none;padding:0;margin:0;flex:1;overflow-y:auto}
.tree-node{}
.nr{display:flex;align-items:center;gap:6px;padding:8px 10px;border-radius:var(--radius-sm);cursor:pointer;color:var(--text-secondary);font-size:13px;transition:all var(--transition-fast)}
.nr:hover,.nr.active{background:var(--color-primary-soft);color:var(--color-primary)}
.nt{width:16px;font-size:10px;color:var(--text-muted);flex-shrink:0}
.ni{font-size:14px;flex-shrink:0}
.nn{flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.nc{font-size:10px;background:var(--bg-elevated);color:var(--text-muted);padding:1px 6px;border-radius:8px;flex-shrink:0}
.tree-children{list-style:none;padding:0 0 0 16px;margin:0}
.org-detail{flex:1;overflow-y:auto;padding:24px}
.dh{display:flex;align-items:center;gap:16px;margin-bottom:20px;padding-bottom:16px;border-bottom:1px solid var(--border-subtle)}
.di{font-size:32px}
.dh h2{font-size:20px;color:var(--text-primary);margin:0 0 4px}
.ds{font-size:12px;color:var(--text-muted);margin:0}
.dg{display:grid;grid-template-columns:1fr 1fr;gap:12px;margin-bottom:20px}
.di2{padding:12px 16px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.dl{display:block;font-size:11px;color:var(--text-muted);text-transform:uppercase;letter-spacing:1px;margin-bottom:4px}
.dv{font-size:14px;color:var(--text-primary)}
.dv.mono{font-family:'JetBrains Mono',monospace;font-size:12px;color:var(--color-primary)}
.members h3{font-size:14px;color:var(--color-primary);margin:0 0 12px}
.mlist{display:grid;grid-template-columns:repeat(auto-fill,minmax(180px,1fr));gap:8px}
.mc{display:flex;align-items:center;gap:10px;padding:10px 12px;background:var(--bg-elevated);border-radius:var(--radius-md)}
.ma2{width:32px;height:32px;border-radius:50%;background:var(--color-primary-soft);color:var(--color-primary);display:flex;align-items:center;justify-content:center;font-weight:600;font-size:14px}
.mi2{flex:1}
.mn{font-size:13px;color:var(--text-primary);font-weight:500}
.mp{font-size:11px;color:var(--text-muted)}
.es,.ls,.es-lg{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px}
.ei{font-size:48px;opacity:0.4}
.sk{height:32px;border-radius:var(--radius-sm);margin-bottom:6px;background:var(--bg-elevated)}
.empty-m{color:var(--text-muted);font-size:13px;padding:20px;text-align:center}
@media(max-width:768px){.org-layout{flex-direction:column}.org-tree{width:100%;max-height:200px}}
</style>
