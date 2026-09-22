<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { type AttendancePreCheck, attendanceApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'

const pre = ref<AttendancePreCheck | null>(null)
const loading = ref(false)
const checking = ref(false)

function fmtTime(value?: string): string {
  if (!value) return ''
  const d = new Date(value)
  return Number.isNaN(d.valueOf()) ? value : d.toLocaleString('zh-CN')
}
function labelOf(checkInType?: string): string {
  if (!checkInType) return '打卡'
  const t = checkInType.toLowerCase()
  if (t.includes('out') || t === 'off' || t.includes('2')) return '下班'
  if (t.includes('in') || t === 'on' || t.includes('1')) return '上班'
  return checkInType
}

async function load() {
  loading.value = true
  try {
    const resp = await attendanceApi.preCheck()
    pre.value = resp.data ?? null
  } catch {
    pre.value = null
  } finally {
    loading.value = false
  }
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  load()
  void loadMine()
})

async function doCheck(kind: 'checkIn' | 'checkOut'): Promise<void> {
  if (checking.value) return
  checking.value = true
  try {
    const resp = await attendanceApi.check(kind)
    const duplicated = Boolean(resp.data?.duplicated)
    uni.showToast({
      title: duplicated ? '今日已打过此卡' : kind === 'checkIn' ? '上班打卡成功' : '下班打卡成功',
      icon: duplicated ? 'none' : 'success',
    })
    await load()
  } catch (e) {
    uni.showToast({ title: e instanceof Error ? e.message : '打卡失败', icon: 'none' })
  } finally {
    checking.value = false
  }
}

function todayRecords(): AttendancePreCheck['records'] {
  return pre.value?.records ?? []
}

// ── 我的月度考勤（v2/my/*，rev102）───────────────────────────────
interface MyDetailRow {
  id?: string
  date?: string
  status?: string
  [key: string]: unknown
}
const myStat = ref<Record<string, unknown> | null>(null)
const myDetails = ref<MyDetailRow[]>([])
const apiVersion = ref('')
const myLoading = ref(false)

async function loadMine(): Promise<void> {
  myLoading.value = true
  try {
    const [statResp, detailResp, verResp] = await Promise.all([
      attendanceApi.myStatistic(),
      attendanceApi.myDetailList(),
      attendanceApi.myVersion(),
    ])
    myStat.value = (statResp.data ?? null) as Record<string, unknown> | null
    myDetails.value = (detailResp.data ?? []) as MyDetailRow[]
    apiVersion.value = String((verResp.data as { version?: string })?.version ?? '')
  } catch {
    myStat.value = null
    myDetails.value = []
  } finally {
    myLoading.value = false
  }
}

function statText(): string {
  const s = myStat.value
  if (!s) return '暂无统计'
  const pick = (k: string): string => {
    const v = s[k]
    return v === undefined || v === null ? '' : String(v)
  }
  const normal = pick('normalCount') || pick('normal') || '0'
  const abnormal = pick('abnormalCount') || pick('abnormal') || '0'
  const month = pick('month') || pick('yearMonth') || '本月'
  return `${month} · 正常 ${normal} · 异常 ${abnormal}`
}
</script>

<template>
  <view class="page">
    <view class="hero">
      <view class="date">{{ pre?.date || '今日' }}</view>
      <view class="group">{{ pre?.group?.groupName || '未加入考勤组' }}</view>
    </view>

    <view v-if="loading && !pre" class="tip">加载中…</view>

    <template v-else>
      <view class="buttons">
        <button class="btn" :disabled="checking || !pre?.canCheckIn" @tap="doCheck('checkIn')">
          上班打卡
        </button>
        <button class="btn" :disabled="checking || !pre?.canCheckIn" @tap="doCheck('checkOut')">
          下班打卡
        </button>
      </view>
      <view v-if="!pre?.canCheckIn" class="tip">
        你尚未加入任何考勤组，暂无法打卡（请联系管理员配置考勤组）。
      </view>

      <view class="section">
        <view class="section-title">今日记录</view>
        <view v-if="todayRecords().length === 0" class="tip">今日暂无打卡记录</view>
        <view v-for="r in todayRecords()" :key="r.id" class="rec">
          <view class="rec-main">
            <text class="rec-type">{{ labelOf(r.checkInType) }}</text>
            <text class="rec-result">{{ r.checkInResult || '正常' }}</text>
          </view>
          <view class="rec-meta">{{ r.sourceType ? `${r.sourceType} · ` : '' }}{{ fmtTime(r.createTime) }}</view>
        </view>
      </view>

      <view class="section">
        <view class="section-title">
          我的月度考勤<text v-if="apiVersion" class="ver">v{{ apiVersion }}</text>
        </view>
        <view class="stat-line">{{ statText() }}</view>
        <view v-if="myLoading" class="tip">加载中…</view>
        <view v-else-if="myDetails.length === 0" class="tip">暂无考勤明细</view>
        <view v-for="d in myDetails" :key="d.id" class="rec">
          <view class="rec-main">
            <text class="rec-type">{{ d.date || '—' }}</text>
            <text class="rec-result">{{ d.status || '正常' }}</text>
          </view>
        </view>
      </view>
    </template>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: #f5f7fa;
  padding: 32rpx;
  box-sizing: border-box;
}
.hero {
  text-align: center;
  padding: 40rpx 0;
}
.date {
  font-size: 40rpx;
  font-weight: 700;
  color: #263238;
}
.group {
  margin-top: 10rpx;
  font-size: 26rpx;
  color: #90979f;
}
.buttons {
  display: flex;
  gap: 24rpx;
  margin: 40rpx 0;
}
.btn {
  flex: 1;
  background: #2d8cf0;
  color: #fff;
  border-radius: 14rpx;
  font-size: 30rpx;
}
.btn[disabled] {
  background: #c0c4cc;
}
.tip {
  color: #90979f;
  font-size: 26rpx;
  text-align: center;
  padding: 40rpx 0;
}
.section {
  background: #fff;
  border-radius: 16rpx;
  padding: 28rpx;
}
.section-title {
  font-size: 30rpx;
  font-weight: 700;
  color: #263238;
  margin-bottom: 16rpx;
}
.rec {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20rpx 0;
  border-bottom: 1px solid #f0f2f5;
}
.rec:last-child {
  border-bottom: none;
}
.rec-main {
  display: flex;
  gap: 12rpx;
  align-items: center;
}
.rec-type {
  font-size: 28rpx;
  color: #263238;
  font-weight: 600;
}
.rec-result {
  font-size: 24rpx;
  color: #67c23a;
}
.rec-meta {
  font-size: 24rpx;
  color: #90979f;
}
.ver {
  margin-left: 12rpx;
  font-size: 22rpx;
  color: #2d8cf0;
  font-weight: 400;
}
.stat-line {
  font-size: 26rpx;
  color: #5b6572;
  margin-bottom: 16rpx;
}
</style>
