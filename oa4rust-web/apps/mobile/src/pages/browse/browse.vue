<script setup lang="ts">
/**
 * 只读浏览（阶段 G / F4）——移动端以「看」为主：门户页面 / 查询视图 / CMS 栏目。
 * 三个数据源分页签，各自只读列表 + 详情。
 */
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { cmsApi, portalApi, queryviewApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'

type TabKey = 'portal' | 'query' | 'cms'

const tabs: Array<{ key: TabKey; label: string }> = [
  { key: 'portal', label: '门户' },
  { key: 'query', label: '查询' },
  { key: 'cms', label: '栏目' },
]

type Row = Record<string, unknown>

const tab = ref<TabKey>('portal')
const rows = ref<Row[]>([])
const loading = ref(false)
const detail = ref<Row | null>(null)
const detailLoading = ref(false)

function nameOf(r: Row): string {
  return String(r.name ?? r.title ?? r.flag ?? r.id ?? '未命名')
}
function descOf(r: Row): string {
  const v = r.description ?? r.alias ?? r.category ?? r.type
  return v ? String(v) : ''
}

async function load(): Promise<void> {
  loading.value = true
  detail.value = null
  try {
    if (tab.value === 'portal') {
      const r = await portalApi.pageList('default')
      rows.value = (r.data ?? []) as Row[]
    } else if (tab.value === 'query') {
      // queryFlag='all' 表示不限查询（后端 list/query/{queryFlag}）
      const r = await queryviewApi.viewList('all')
      rows.value = (r.data ?? []) as Row[]
    } else {
      const r = await cmsApi.columnList()
      rows.value = (r.data ?? []) as Row[]
    }
  } catch {
    rows.value = []
  } finally {
    loading.value = false
  }
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  void load()
})

function switchTab(key: TabKey): void {
  if (tab.value === key) return
  tab.value = key
  rows.value = []
  void load()
}

async function open(r: Row): Promise<void> {
  detailLoading.value = true
  detail.value = null
  try {
    const id = String(r.id ?? r.flag ?? '')
    if (!id) {
      detail.value = r
      return
    }
    if (tab.value === 'portal') {
      detail.value = ((await portalApi.pageDetail(id)) as { data?: Row }).data ?? r
    } else if (tab.value === 'query') {
      const view = String(r.view_flag ?? r.viewFlag ?? 'view')
      detail.value = ((await queryviewApi.execute(view, id)) as { data?: Row }).data ?? r
    } else {
      const resp = await cmsApi.documentSearch(nameOf(r))
      const list = (resp.data ?? []) as Row[]
      detail.value = { 文档数: list.length, 栏目: nameOf(r), 首篇: list[0] ? nameOf(list[0]) : '—' }
    }
  } catch (e) {
    uni.showToast({ title: e instanceof Error ? e.message : '加载失败', icon: 'none' })
  } finally {
    detailLoading.value = false
  }
}

function detailText(): string {
  if (!detail.value) return ''
  try {
    return JSON.stringify(detail.value, null, 2)
  } catch {
    return String(detail.value)
  }
}
</script>

<template>
  <view class="page">
    <view class="tabs">
      <view v-for="t in tabs" :key="t.key" class="tab" :class="{ on: tab === t.key }" @tap="switchTab(t.key)">
        {{ t.label }}
      </view>
    </view>

    <view v-if="loading" class="tip">加载中…</view>
    <view v-else-if="rows.length === 0" class="tip">暂无数据</view>

    <view v-for="(r, i) in rows" :key="String(r.id ?? i)" class="card" @tap="open(r)">
      <view class="card-title">{{ nameOf(r) }}</view>
      <view v-if="descOf(r)" class="card-desc">{{ descOf(r) }}</view>
    </view>

    <view v-if="detailLoading" class="tip">详情加载中…</view>
    <view v-else-if="detail" class="detail">
      <view class="detail-head">
        <text class="detail-title">详情</text>
        <text class="detail-close" @tap="detail = null">关闭</text>
      </view>
      <text class="detail-body">{{ detailText() }}</text>
    </view>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: #f5f7fa;
  padding: 24rpx;
  box-sizing: border-box;
}
.tabs {
  display: flex;
  gap: 12rpx;
  margin-bottom: 24rpx;
}
.tab {
  flex: 1;
  text-align: center;
  padding: 16rpx 0;
  font-size: 28rpx;
  color: #5b6572;
  background: #fff;
  border-radius: 12rpx;
}
.tab.on {
  background: #2d8cf0;
  color: #fff;
  font-weight: 600;
}
.card {
  background: #fff;
  border-radius: 16rpx;
  padding: 24rpx;
  margin-bottom: 20rpx;
}
.card-title {
  font-size: 30rpx;
  font-weight: 700;
  color: #263238;
}
.card-desc {
  margin-top: 8rpx;
  font-size: 24rpx;
  color: #90979f;
}
.detail {
  background: #fff;
  border-radius: 16rpx;
  padding: 24rpx;
  margin-top: 8rpx;
}
.detail-head {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16rpx;
}
.detail-title {
  font-size: 28rpx;
  font-weight: 700;
  color: #263238;
}
.detail-close {
  font-size: 26rpx;
  color: #2d8cf0;
}
.detail-body {
  font-size: 22rpx;
  color: #5b6572;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-all;
}
.tip {
  color: #90979f;
  font-size: 26rpx;
  text-align: center;
  padding: 80rpx 0;
}
</style>
