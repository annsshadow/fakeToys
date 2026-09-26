<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { ElMessage } from 'element-plus'
import {
  fetchAnnouncements,
  createAnnouncement,
  fetchRedeemCodes,
  createRedeemCode,
  fetchAuditLogs,
} from '@/api'

const activeTab = ref('announcements')
const loading = ref(false)

const announcements = ref<Array<Record<string, unknown>>>([])
const codes = ref<Array<Record<string, unknown>>>([])
const logs = ref<Array<Record<string, unknown>>>([])

// 公告表单
const annForm = ref({ title: '', body: '', published: false })

// 兑换码表单
const codeForm = ref({
  code: '',
  reward: 'coin:10000',
  max_uses: 100,
  expires_at: '',
})

function str(o: Record<string, unknown>, k: string): string {
  const v = o[k]
  return v === undefined || v === null ? '' : String(v)
}

function num(o: Record<string, unknown>, k: string): number {
  const v = Number(o[k] ?? 0)
  return Number.isFinite(v) ? v : 0
}

function fmtTime(s: string): string {
  if (!s) return '—'
  return s.replace('T', ' ').slice(0, 19)
}

async function loadAll() {
  loading.value = true
  try {
    const [a, c, l] = await Promise.all([fetchAnnouncements(), fetchRedeemCodes(), fetchAuditLogs(100)])
    announcements.value = a.items
    codes.value = c.items
    logs.value = l.items
  } catch (e) {
    ElMessage.error(`运营数据加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

async function submitAnnouncement() {
  if (!annForm.value.title.trim() || !annForm.value.body.trim()) {
    ElMessage.error('标题与正文都不能为空')
    return
  }
  try {
    await createAnnouncement(annForm.value)
    ElMessage.success('公告已创建')
    annForm.value = { title: '', body: '', published: false }
    await loadAll()
  } catch (e) {
    ElMessage.error((e as Error).message)
  }
}

/** 兑换码奖励格式：货币:数量，支持逗号分隔多项 */
function parseReward(s: string): Record<string, number> | null {
  const out: Record<string, number> = {}
  for (const part of s.split(/[,，]/).map((x) => x.trim()).filter(Boolean)) {
    const m = part.match(/^(\w+)\s*[:：]\s*(\d+)$/)
    if (!m) return null
    out[m[1]] = Number(m[2])
  }
  return Object.keys(out).length ? out : null
}

async function submitCode() {
  const reward = parseReward(codeForm.value.reward)
  if (!reward) {
    ElMessage.error('奖励格式错误，应为「coin:10000, gem:100」')
    return
  }
  if (!codeForm.value.code.trim()) {
    ElMessage.error('兑换码不能为空')
    return
  }
  try {
    await createRedeemCode({
      code: codeForm.value.code.trim(),
      reward,
      max_uses: codeForm.value.max_uses,
      expires_at: codeForm.value.expires_at || undefined,
    })
    ElMessage.success('兑换码已创建')
    codeForm.value = { code: '', reward: 'coin:10000', max_uses: 100, expires_at: '' }
    await loadAll()
  } catch (e) {
    ElMessage.error((e as Error).message)
  }
}

function rewardText(v: unknown): string {
  const o = typeof v === 'string' ? (JSON.parse(v || '{}') as Record<string, number>) : (v as Record<string, number>)
  return Object.entries(o ?? {})
    .map(([k, n]) => `${k}×${n}`)
    .join(' · ')
}

onMounted(loadAll)
</script>

<template>
  <div v-loading="loading">
    <div class="page-title">公告与兑换码</div>
    <p class="page-subtitle">
      兑换码是<strong>客服与内测</strong>的主要补偿通道。所有后台改动都会记入审计日志。
    </p>

    <el-tabs v-model="activeTab">
      <!-- 公告 -->
      <el-tab-pane label="公告" name="announcements">
        <el-row :gutter="12">
          <el-col :span="10">
            <el-card shadow="never" header="发布公告">
              <el-form label-position="top">
                <el-form-item label="标题">
                  <el-input v-model="annForm.title" placeholder="例如：1.2 版本更新说明" />
                </el-form-item>
                <el-form-item label="正文">
                  <el-input v-model="annForm.body" type="textarea" :rows="6" />
                </el-form-item>
                <el-form-item>
                  <el-checkbox v-model="annForm.published">立即发布</el-checkbox>
                </el-form-item>
                <el-button type="primary" @click="submitAnnouncement">创建</el-button>
              </el-form>
            </el-card>
          </el-col>
          <el-col :span="14">
            <el-card shadow="never" header="已有公告">
              <el-table :data="announcements" size="small" stripe>
                <el-table-column prop="id" label="ID" width="55" />
                <el-table-column prop="title" label="标题" min-width="140" />
                <el-table-column label="状态" width="76">
                  <template #default="{ row }">
                    <el-tag :type="num(row, 'published') === 1 ? 'success' : 'info'" size="small">
                      {{ num(row, 'published') === 1 ? '已发布' : '草稿' }}
                    </el-tag>
                  </template>
                </el-table-column>
                <el-table-column label="创建时间" width="155">
                  <template #default="{ row }">{{ fmtTime(str(row, 'created_at')) }}</template>
                </el-table-column>
              </el-table>
            </el-card>
          </el-col>
        </el-row>
      </el-tab-pane>

      <!-- 兑换码 -->
      <el-tab-pane label="兑换码" name="codes">
        <el-card shadow="never" header="生成兑换码">
          <el-form :inline="true">
            <el-form-item label="码">
              <el-input v-model="codeForm.code" placeholder="LYP-2026-XXXX" style="width: 180px" />
            </el-form-item>
            <el-form-item label="奖励">
              <el-input v-model="codeForm.reward" placeholder="coin:10000, gem:100" style="width: 220px" />
            </el-form-item>
            <el-form-item label="次数上限">
              <el-input-number v-model="codeForm.max_uses" :min="1" :max="100000" />
            </el-form-item>
            <el-form-item label="过期时间（可空）">
              <el-input v-model="codeForm.expires_at" placeholder="2026-12-31T23:59:59Z" style="width: 200px" />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="submitCode">生成</el-button>
            </el-form-item>
          </el-form>
        </el-card>

        <el-card shadow="never" style="margin-top: 12px">
          <el-table :data="codes" size="small" stripe>
            <el-table-column prop="id" label="ID" width="55" />
            <el-table-column prop="code" label="兑换码" min-width="150" />
            <el-table-column label="奖励" min-width="180">
              <template #default="{ row }">{{ rewardText(row.reward) }}</template>
            </el-table-column>
            <el-table-column label="已用 / 上限" width="110">
              <template #default="{ row }">
                {{ num(row, 'used_count') }} / {{ num(row, 'max_uses') }}
              </template>
            </el-table-column>
            <el-table-column label="过期" width="155">
              <template #default="{ row }">{{ fmtTime(str(row, 'expires_at')) }}</template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-tab-pane>

      <!-- 审计日志 -->
      <el-tab-pane label="审计日志" name="logs">
        <el-card shadow="never">
          <el-table :data="logs" size="small" stripe>
            <el-table-column prop="id" label="ID" width="55" />
            <el-table-column prop="admin_username" label="管理员" width="110" />
            <el-table-column prop="action" label="操作" width="150" />
            <el-table-column prop="target_type" label="对象类型" width="110" />
            <el-table-column prop="target_id" label="对象 ID" width="90" />
            <el-table-column prop="reason" label="原因" min-width="180" />
            <el-table-column label="时间" width="155">
              <template #default="{ row }">{{ fmtTime(str(row, 'created_at')) }}</template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style scoped></style>
