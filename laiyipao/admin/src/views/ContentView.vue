<script setup lang="ts">
import {onMounted, ref, onActivated} from 'vue'
import { ElMessage } from 'element-plus'
import {
  fetchAnnouncements,
  createAnnouncement,
  fetchRedeemCodes,
  createRedeemCode,
  fetchAuditLogs,
  type AdminAnnouncementView,
  type AdminAuditLogView,
  type AdminRedeemCodeView,
} from '@/api'
import { fmtTime } from '@/utils/format'

const activeTab = ref('announcements')
/** 空态文案（2026-10-21 P5-8）：空表要说清「为什么空」。 */
const emptyText = '暂无记录'
const loading = ref(false)

const announcements = ref<AdminAnnouncementView[]>([])
const annTotal = ref(0)
const codes = ref<AdminRedeemCodeView[]>([])
const codeTotal = ref(0)
const logs = ref<AdminAuditLogView[]>([])
const logTotal = ref(0)

// 公告表单
const annForm = ref({ title: '', body: '', published: false })

// 兑换码表单
const codeForm = ref({
  code: '',
  reward: 'coin:10000',
  max_uses: 100,
  expires_at: '',
})

/** 三张表的分页（2026-10-21 P2-6）：此前固定 50/50/100 条且**没有分页器**，
 *  而接口返回的 `total` 是全量 —— 更早的公告 / 兑换码 / 审计记录**根本看不到**。 */
const PAGE_SIZE = 50
const annPage = ref(1)
const codePage = ref(1)
const logPage = ref(1)

async function loadAll() {
  loading.value = true
  try {
    const [a, c, l] = await Promise.all([
      fetchAnnouncements(PAGE_SIZE, (annPage.value - 1) * PAGE_SIZE),
      fetchRedeemCodes(PAGE_SIZE, (codePage.value - 1) * PAGE_SIZE),
      fetchAuditLogs(PAGE_SIZE, (logPage.value - 1) * PAGE_SIZE),
    ])
    announcements.value = a.items
    annTotal.value = a.total ?? a.items.length
    codes.value = c.items
    codeTotal.value = c.total ?? c.items.length
    logs.value = l.items
    logTotal.value = l.total ?? l.items.length
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

/**
 * 兑换码奖励展示。
 *
 * 2026-10-21：服务端现在直接下发**对象**（`map[string]int`），
 * 不再是 JSON 字符串 —— 旧的 parse 兜底会把「字段名不对」显示成「无奖励」。
 */
function rewardText(v: Record<string, number> | undefined): string {
  return Object.entries(v ?? {})
    .map(([k, n]) => `${k}×${n}`)
    .join(' · ')
}

/** 审计日志的 detail 是 JSON 对象（如 {currency, amount} / 技能 patch）。
 *  修前表格用 prop="reason"（服务端根本没有这列）→ 恒空白。
 *  现在读 detail 并渲染成可读文本。 */
function detailText(v: unknown): string {
  if (v === null || v === undefined) return '—'
  if (typeof v === 'string') return v
  try {
    const s = JSON.stringify(v)
    return s.length > 80 ? s.slice(0, 80) + '…' : s
  } catch {
    return String(v)
  }
}

onMounted(loadAll)

/**
 * 2026-10-21（P5-9）：缓存视图重新可见时刷新数据。
 *
 * 布局用 `keep-alive`（max=4）缓存视图，而每个视图此前**只在
 * `onMounted` 加载一次** —— 于是「封禁一个用户 → 切到别的页 →
 * 切回来」看到的还是**旧数据**，运营会以为操作没生效。
 *
 * `onActivated` 是 keep-alive 视图重新可见时的钩子。
 */
/**
   * `onActivated` 在**首次挂载**时也会触发（vue 给 keep-alive 的子组件
   * 打标记并把钩子排进 post-render 队列），而 `onMounted` 已经加载过一次 ——
   * 两者都留着 = 每进一次页面发**两份**请求，还留下一个
   * 「后到的先渲染」的乱序窗口。
   *
   * 所以：首次的 `onActivated` 跳过，之后的才刷新。
   */
  let activatedOnce = false
onActivated(() => {
    if (!activatedOnce) {
    activatedOnce = true
    return
  }
loadAll()
})
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
          <el-col :span="10" :xs="24" :sm="24" :md="10">
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
          <el-col :span="14" :xs="24" :sm="24" :md="14">
            <el-card shadow="never" header="已有公告">
              <el-table :data="announcements" size="small" stripe>
                <el-table-column prop="id" label="ID" width="55" />
                <el-table-column prop="title" label="标题" min-width="140" />
                <!--
                  2026-10-21（P5-10）：此前只有标题，正文（`body`）与
                  发布时间（`published_at`）服务端**都返回了**但界面不显示 ——
                  运营无法确认「我发的公告到底写了什么」。
                -->
                <el-table-column label="正文" min-width="220">
                  <template #default="{ row }">
                    <span class="clamp-2">{{ row.body || '—' }}</span>
                  </template>
                </el-table-column>
                <el-table-column label="状态" width="76">
                  <template #default="{ row }">
                    <el-tag :type="row.published ? 'success' : 'info'" size="small">
                      {{ row.published ? '已发布' : '草稿' }}
                    </el-tag>
                  </template>
                </el-table-column>
                <el-table-column label="发布时间" width="155">
                  <template #default="{ row }">{{ fmtTime(row.published_at) }}</template>
                </el-table-column>
                <el-table-column label="创建时间" width="155">
                  <template #default="{ row }">{{ fmtTime(row.created_at) }}</template>
                </el-table-column>
        <template #empty>
          <el-empty :description="emptyText" :image-size="60" />
        </template>
</el-table>
              <!--
                2026-10-21（二轮审计 P2-6）：分页器。
                此前这里只有前 50 条 —— 接口早就支持 offset，
                但页面既没有分页器，也没有「共 N 条」，
                于是**更早的公告 / 兑换码 / 审计记录根本看不到**。
              -->
              <el-pagination
                v-if="annTotal > PAGE_SIZE"
                v-model:current-page="annPage"
                :page-size="PAGE_SIZE"
                :total="annTotal"
                layout="prev, pager, next, total"
                size="small"
                background
                @current-change="loadAll"
              />
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
                {{ row.used_count }} / {{ row.max_uses }}
              </template>
            </el-table-column>
            <el-table-column label="过期" width="155">
              <template #default="{ row }">{{ fmtTime(row.expires_at) }}</template>
            </el-table-column>
            <!-- 2026-10-21（P5-10）：`enabled` 服务端一直返回，界面不显示。 -->
            <el-table-column label="状态" width="80">
              <template #default="{ row }">
                <el-tag :type="row.enabled ? 'success' : 'info'" size="small">
                  {{ row.enabled ? '可用' : '停用' }}
                </el-tag>
              </template>
            </el-table-column>
        <template #empty>
          <el-empty :description="emptyText" :image-size="60" />
        </template>
</el-table>
          <!--
            2026-10-21（二轮审计 P2-6）：分页器。
            此前这里只有前 50 条 —— 接口早就支持 offset，
            但页面既没有分页器，也没有「共 N 条」，
            于是**更早的公告 / 兑换码 / 审计记录根本看不到**。
          -->
          <el-pagination
            v-if="codeTotal > PAGE_SIZE"
            v-model:current-page="codePage"
            :page-size="PAGE_SIZE"
            :total="codeTotal"
            layout="prev, pager, next, total"
            size="small"
            background
            @current-change="loadAll"
          />
        </el-card>
      </el-tab-pane>

      <!-- 审计日志 -->
      <el-tab-pane label="审计日志" name="logs">
        <el-card shadow="never">
          <el-table :data="logs" size="small" stripe>
            <el-table-column prop="id" label="ID" width="55" />
            <el-table-column prop="username" label="管理员" width="110" />
            <el-table-column prop="action" label="操作" width="150" />
            <!--
              第 121 轮：旧列 admin_username / target_type / target_id / reason
              在服务端响应（AdminListAuditLogs）里**都不存在**——
              服务端字段是 username / action / target / detail。
              「对象类型」没有独立列（action 已含语义），合并成「对象」= target。
            -->
            <el-table-column prop="target" label="对象" width="130" />
            <el-table-column label="详情" min-width="180">
              <template #default="{ row }">{{ detailText(row.detail) }}</template>
            </el-table-column>
            <el-table-column label="时间" width="155">
              <template #default="{ row }">{{ fmtTime(row.created_at) }}</template>
            </el-table-column>
        <template #empty>
          <el-empty :description="emptyText" :image-size="60" />
        </template>
</el-table>
          <!--
            2026-10-21（二轮审计 P2-6）：分页器。
            此前这里只有前 50 条 —— 接口早就支持 offset，
            但页面既没有分页器，也没有「共 N 条」，
            于是**更早的公告 / 兑换码 / 审计记录根本看不到**。
          -->
          <el-pagination
            v-if="logTotal > PAGE_SIZE"
            v-model:current-page="logPage"
            :page-size="PAGE_SIZE"
            :total="logTotal"
            layout="prev, pager, next, total"
            size="small"
            background
            @current-change="loadAll"
          />
        </el-card>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style scoped></style>
