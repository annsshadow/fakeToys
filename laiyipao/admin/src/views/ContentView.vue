<script setup lang="ts">
import {onMounted, ref, onActivated} from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  fetchAnnouncements,
  createAnnouncement,
  updateAnnouncement,
  deleteAnnouncement,
  fetchRedeemCodes,
  createRedeemCode,
  updateRedeemCode,
  setRedeemCodeEnabled,
  exportRedeemCodesCsv,
  fetchAuditLogs,
  exportAuditLogsCsv,
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
// 编辑中的公告 id（null = 非编辑态，2026-10-31 实质优化第 23 轮）
const editingAnnId = ref<number | null>(null)

// 兑换码表单
const codeForm = ref({
  code: '',
  reward: 'coin:10000',
  max_uses: 100,
  expires_at: '',
})
// 编辑中的兑换码 id（null = 非编辑态，2026-10-31 实质优化第 25 轮）
const editingCodeId = ref<number | null>(null)

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

/**
 * 导出审计日志 CSV（2026-10-30 实质优化第 15 轮）。
 *
 * 合规归档需要全量快照。readonly 角色也可导出（读操作）。
 * 文件名带日期。
 */
async function onExportAuditLogs() {
  try {
    const blob = await exportAuditLogsCsv()
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `audit_logs_${new Date().toISOString().slice(0, 10)}.csv`
    document.body.appendChild(a)
    a.click()
    a.remove()
    URL.revokeObjectURL(url)
    ElMessage.success(`已导出 ${logs.value.length} 条审计日志（当前页）`)
  } catch (e) {
    ElMessage.error(`导出失败：${(e as Error).message}`)
  }
}

/**
 * 导出兑换码 CSV（2026-10-31 实质优化第 37 轮）。
 * 运营需要把已生成码的清单带走做线下核对/客服分发。
 */
async function onExportRedeemCodes() {
  try {
    const blob = await exportRedeemCodesCsv()
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `redeem_codes_${new Date().toISOString().slice(0, 10)}.csv`
    document.body.appendChild(a)
    a.click()
    a.remove()
    URL.revokeObjectURL(url)
    ElMessage.success(`已导出 ${codes.value.length} 条兑换码（当前页）`)
  } catch (e) {
    ElMessage.error(`导出失败：${(e as Error).message}`)
  }
}

async function submitAnnouncement() {
  if (!annForm.value.title.trim() || !annForm.value.body.trim()) {
    ElMessage.error('标题与正文都不能为空')
    return
  }
  try {
    if (editingAnnId.value !== null) {
      await updateAnnouncement(editingAnnId.value, annForm.value)
      ElMessage.success('公告已更新')
    } else {
      await createAnnouncement(annForm.value)
      ElMessage.success('公告已创建')
    }
    editingAnnId.value = null
    annForm.value = { title: '', body: '', published: false }
    await loadAll()
  } catch (e) {
    ElMessage.error((e as Error).message)
  }
}

/**
 * 编辑公告（2026-10-31 实质优化第 23 轮）。
 * 此前只有创建/列表，发错内容无法修改。点击「编辑」把公告回填到表单。
 */
function startEditAnn(a: AdminAnnouncementView) {
  editingAnnId.value = a.id
  annForm.value = { title: a.title, body: a.body, published: a.published }
}

function cancelEditAnn() {
  editingAnnId.value = null
  annForm.value = { title: '', body: '', published: false }
}

/**
 * 删除公告（2026-10-31 实质优化第 23 轮）。
 * 删除不可恢复，必须二次确认；玩家端缓存最迟 30 秒后消失。
 */
async function onDeleteAnn(a: AdminAnnouncementView) {
  try {
    await ElMessageBox.confirm(
      `删除公告「${a.title}」？删除后玩家端将无法看到，且不可恢复。`,
      '删除公告',
      { type: 'warning', confirmButtonText: '删除' },
    )
  } catch {
    return // 取消
  }
  try {
    await deleteAnnouncement(a.id)
    ElMessage.success('公告已删除')
    await loadAll()
  } catch (e) {
    ElMessage.error(`删除失败：${(e as Error).message}`)
  }
}

/**
 * 启用/禁用兑换码（2026-10-31 实质优化第 24 轮）。
 * 运营创建错误码后此前无法停用（只能等过期）。
 * 禁用立即生效（玩家端 Redeem 的 SQL 带 enabled 条件）。
 */
async function onToggleRedeemCode(c: AdminRedeemCodeView) {
  const next = !c.enabled
  try {
    await ElMessageBox.confirm(
      next
        ? `重新启用兑换码「${c.code}」？`
        : `停用兑换码「${c.code}」？玩家将立即无法兑换，且会写入审计日志。`,
      next ? '启用兑换码' : '停用兑换码',
      { type: next ? 'info' : 'warning', confirmButtonText: next ? '启用' : '停用' },
    )
  } catch {
    return // 取消
  }
  try {
    const res = await setRedeemCodeEnabled(c.id, next)
    Object.assign(c, res.item)
    ElMessage.success(next ? '已启用' : '已停用')
  } catch (e) {
    ElMessage.error(`操作失败：${(e as Error).message}`)
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
  if (editingCodeId.value === null && !codeForm.value.code.trim()) {
    ElMessage.error('兑换码不能为空')
    return
  }
  try {
    if (editingCodeId.value !== null) {
      await updateRedeemCode(editingCodeId.value, {
        reward,
        max_uses: codeForm.value.max_uses,
        expires_at: codeForm.value.expires_at || undefined,
      })
      ElMessage.success('兑换码已更新')
      editingCodeId.value = null
    } else {
      await createRedeemCode({
        code: codeForm.value.code.trim(),
        reward,
        max_uses: codeForm.value.max_uses,
        expires_at: codeForm.value.expires_at || undefined,
      })
      ElMessage.success('兑换码已创建')
    }
    codeForm.value = { code: '', reward: 'coin:10000', max_uses: 100, expires_at: '' }
    await loadAll()
  } catch (e) {
    ElMessage.error((e as Error).message)
  }
}

/**
 * 编辑兑换码（2026-10-31 实质优化第 25 轮）。
 * 改奖励/次数/过期时间此前只能停用后重建（码会变，已分享的全作废）。
 */
function startEditCode(c: AdminRedeemCodeView) {
  editingCodeId.value = c.id
  codeForm.value = {
    code: c.code,
    reward: Object.entries(c.reward ?? {})
      .map(([k, v]) => `${k}:${v}`)
      .join(', '),
    max_uses: c.max_uses,
    expires_at: c.expires_at ? new Date(c.expires_at).toISOString().slice(0, 19) + 'Z' : '',
  }
}

function cancelEditCode() {
  editingCodeId.value = null
  codeForm.value = { code: '', reward: 'coin:10000', max_uses: 100, expires_at: '' }
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
            <el-card shadow="never" :header="editingAnnId !== null ? '编辑公告' : '发布公告'">
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
                <el-button type="primary" @click="submitAnnouncement">
                  {{ editingAnnId !== null ? '更新' : '创建' }}
                </el-button>
                <el-button v-if="editingAnnId !== null" @click="cancelEditAnn">取消编辑</el-button>
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
                <!-- 编辑/删除（2026-10-31 实质优化第 23 轮） -->
                <el-table-column label="操作" width="110" fixed="right">
                  <template #default="{ row }">
                    <el-button size="small" text @click="startEditAnn(row as AdminAnnouncementView)">编辑</el-button>
                    <el-button size="small" text type="danger" @click="onDeleteAnn(row as AdminAnnouncementView)">删除</el-button>
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
        <el-card shadow="never" :header="editingCodeId !== null ? '编辑兑换码' : '生成兑换码'">
          <el-form :inline="true">
            <el-form-item label="码">
              <el-input v-model="codeForm.code" placeholder="LYP-2026-XXXX" style="width: 180px"
                :disabled="editingCodeId !== null" />
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
              <el-button type="primary" @click="submitCode">
                {{ editingCodeId !== null ? '更新' : '生成' }}
              </el-button>
              <el-button v-if="editingCodeId !== null" @click="cancelEditCode">取消编辑</el-button>
            </el-form-item>
          </el-form>
        </el-card>

        <el-card shadow="never" style="margin-top: 12px">
          <div style="display: flex; align-items: center; justify-content: flex-end; margin-bottom: 8px">
            <span class="muted" style="margin-right: 8px">共 {{ codeTotal }} 条</span>
            <el-button size="small" @click="onExportRedeemCodes">导出 CSV</el-button>
          </div>
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
            <!-- 编辑/启用/禁用（2026-10-31 实质优化第 24/25 轮） -->
            <el-table-column label="操作" width="120" fixed="right">
              <template #default="{ row }">
                <el-button size="small" text @click="startEditCode(row as AdminRedeemCodeView)">编辑</el-button>
                <el-button size="small" text :type="row.enabled ? 'danger' : 'success'"
                  @click="onToggleRedeemCode(row as AdminRedeemCodeView)">
                  {{ row.enabled ? '停用' : '启用' }}
                </el-button>
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
          <div style="display: flex; align-items: center; justify-content: flex-end; margin-bottom: 8px">
            <span class="muted" style="margin-right: 8px">共 {{ logTotal }} 条</span>
            <el-button size="small" @click="onExportAuditLogs">导出 CSV</el-button>
          </div>
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
