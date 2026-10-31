<script setup lang="ts">
import {onMounted, ref, onActivated} from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  fetchAnnouncements,
  createAnnouncement,
  updateAnnouncement,
  deleteAnnouncement,
  exportAnnouncementsCsv,
  fetchRedeemCodes,
  fetchAdminRedeemUsages,
  exportRedeemUsagesCsv,
  createRedeemCode,
  createRedeemCodesBatch,
  updateRedeemCode,
  setRedeemCodeEnabled,
  exportRedeemCodesCsv,
  fetchAuditLogs,
  exportAuditLogsCsv,
  type AdminAnnouncementView,
  type AdminAuditLogView,
  type AdminRedeemCodeView,
  type AdminRedeemUsageView,
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

// 兑换码使用记录（2026-10-31 新一轮第 17 轮）。
const redeemUsages = ref<AdminRedeemUsageView[]>([])
const redeemUsageTotal = ref(0)
const redeemUsagePage = ref(1)
const redeemUsageVisible = ref(false)
// 使用记录筛选（2026-10-31 新一轮第 23 轮）：按码搜索 / 用户过滤。
const redeemCodeKeyword = ref('')
const redeemUserId = ref<number | undefined>(undefined)

// 审计筛选（2026-10-31 新一轮第 18 轮）：操作类型 + 日期范围。
const logAction = ref('')
const logDateRange = ref<[string, string] | null>(null)

// 公告筛选（2026-10-31 新一轮第 21 轮）：发布状态 + 创建日期范围。
const annPublished = ref<boolean | undefined>(undefined)
const annDateRange = ref<[string, string] | null>(null)

/** 公告筛选变化：回到第一页并只重载公告（2026-10-31 新一轮第 22 轮）。 */
function applyAnnFilter() {
  annPage.value = 1
  loadAnnouncements()
}

// 兑换码筛选（2026-10-31 新一轮第 20 轮）：码搜索 + 启用/过期状态。
const codeKeyword = ref('')
const codeEnabled = ref<boolean | undefined>(undefined)
const codeExpired = ref<boolean | undefined>(undefined)

/** 兑换码筛选变化：回到第一页并重新加载。 */
function applyCodeFilter() {
  codePage.value = 1
  loadCodes()
}

function nextDay(day: string): string {
  const d = new Date(day + 'T00:00:00Z')
  d.setUTCDate(d.getUTCDate() + 1)
  return d.toISOString().slice(0, 10)
}

async function loadRedeemUsages() {
  try {
    const res = await fetchAdminRedeemUsages(PAGE_SIZE, (redeemUsagePage.value - 1) * PAGE_SIZE, {
      // 按码搜索 / 用户过滤（2026-10-31 新一轮第 23 轮）
      code_keyword: redeemCodeKeyword.value || undefined,
      user_id: redeemUserId.value,
    })
    redeemUsages.value = res.items
    redeemUsageTotal.value = res.total ?? res.items.length
  } catch (e) {
    ElMessage.error(`使用记录加载失败：${(e as Error).message}`)
  }
}

/** 使用记录筛选变化：回到第一页并重新加载（2026-10-31 新一轮第 23 轮）。 */
function applyRedeemUsageFilter() {
  redeemUsagePage.value = 1
  loadRedeemUsages()
}

/** 导出使用记录 CSV（2026-10-31 新一轮第 34 轮）。 */
async function onExportRedeemUsagesCsv() {
  try {
    const blob = await exportRedeemUsagesCsv({
      code_keyword: redeemCodeKeyword.value || undefined,
      user_id: redeemUserId.value,
    })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `redeem_usages_${new Date().toISOString().slice(0, 10)}.csv`
    document.body.appendChild(a)
    a.click()
    a.remove()
    URL.revokeObjectURL(url)
  } catch (e) {
    ElMessage.error(`导出失败：${(e as Error).message}`)
  }
}

const router = useRouter()

/**
 * 打开指定玩家的用户详情（2026-11-01 新一轮第 51 轮）。
 * 跳到用户管理页并携带 user_id，UsersView 挂载时会自动打开详情抽屉。
 */
function openUserDetail(userId: number) {
  router.push({ path: '/users', query: { user_id: String(userId) } })
}

/** 打开使用记录对话框并加载第一页（2026-10-31 新一轮第 17 轮）。 */
function openRedeemUsages() {
  redeemUsageVisible.value = true
  redeemUsagePage.value = 1
  loadRedeemUsages()
}

/** 只加载公告（2026-10-31 新一轮第 22 轮：按 tab 独立加载）。 */
async function loadAnnouncements() {
  try {
    const a = await fetchAnnouncements(PAGE_SIZE, (annPage.value - 1) * PAGE_SIZE, {
      published: annPublished.value,
      date_from: annDateRange.value?.[0] || undefined,
      date_to: annDateRange.value ? nextDay(annDateRange.value[1]) : undefined,
    })
    announcements.value = a.items
    annTotal.value = a.total ?? a.items.length
  } catch (e) {
    ElMessage.error(`公告加载失败：${(e as Error).message}`)
  }
}

/** 只加载兑换码（2026-10-31 新一轮第 22 轮：按 tab 独立加载）。 */
async function loadCodes() {
  try {
    const c = await fetchRedeemCodes(PAGE_SIZE, (codePage.value - 1) * PAGE_SIZE, {
      keyword: codeKeyword.value || undefined,
      enabled: codeEnabled.value,
      expired: codeExpired.value,
    })
    codes.value = c.items
    codeTotal.value = c.total ?? c.items.length
  } catch (e) {
    ElMessage.error(`兑换码加载失败：${(e as Error).message}`)
  }
}

/** 只加载审计日志（2026-10-31 新一轮第 22 轮：按 tab 独立加载）。 */
async function loadLogs() {
  try {
    const l = await fetchAuditLogs(PAGE_SIZE, (logPage.value - 1) * PAGE_SIZE, {
      action: logAction.value || undefined,
      date_from: logDateRange.value?.[0] || undefined,
      date_to: logDateRange.value ? nextDay(logDateRange.value[1]) : undefined,
    })
    logs.value = l.items
    logTotal.value = l.total ?? l.items.length
  } catch (e) {
    ElMessage.error(`审计日志加载失败：${(e as Error).message}`)
  }
}

async function loadAll() {
  loading.value = true
  try {
    await Promise.all([loadAnnouncements(), loadCodes(), loadLogs()])
  } catch {
    // 各加载函数已分别报错；这里兜底避免未处理拒绝
  } finally {
    loading.value = false
  }
}

/** 审计筛选变化：回到第一页并只重载审计（2026-10-31 新一轮第 22 轮）。 */
function applyLogFilter() {
  logPage.value = 1
  loadLogs()
}

/**
 * 导出审计日志 CSV（2026-10-30 实质优化第 15 轮）。
 *
 * 合规归档需要全量快照。readonly 角色也可导出（读操作）。
 * 文件名带日期。
 */
async function onExportAuditLogs() {
  try {
    const blob = await exportAuditLogsCsv({
      // 与列表过滤同口径（2026-10-31 新一轮第 18 轮）
      action: logAction.value || undefined,
      date_from: logDateRange.value?.[0] || undefined,
      date_to: logDateRange.value ? nextDay(logDateRange.value[1]) : undefined,
    })
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

/**
 * 行级玩家端预览（2026-11-01 新一轮第 69 轮）。
 * 在列表里直接看某条公告的玩家端效果（不必先点编辑再预览）。
 */
function previewAnnouncement(a: AdminAnnouncementView) {
  annForm.value = { title: a.title, body: a.body, published: a.published }
  openPreview()
}

function cancelEditAnn() {
  editingAnnId.value = null
  annForm.value = { title: '', body: '', published: false }
}

/**
 * 公告玩家端预览（2026-11-01 新一轮第 54 轮）。
 * 运营发布前想确认玩家看到的实际效果（标题/正文/发布时间），
 * 此前只能先发布再在玩家端看。这里用当前表单内容打开预览对话框。
 */
const previewVisible = ref(false)

function openPreview() {
  if (!annForm.value.title.trim() || !annForm.value.body.trim()) {
    ElMessage.error('标题与正文都不能为空')
    return
  }
  previewVisible.value = true
}

function closePreview() {
  previewVisible.value = false
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
 * 导出公告 CSV（2026-10-31 实质优化第 38 轮）。
 * 运营需要把公告清单带走做归档。readonly 可导出。
 */
async function onExportAnnouncements() {
  try {
    const blob = await exportAnnouncementsCsv()
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `announcements_${new Date().toISOString().slice(0, 10)}.csv`
    document.body.appendChild(a)
    a.click()
    a.remove()
    URL.revokeObjectURL(url)
    ElMessage.success(`已导出 ${announcements.value.length} 条公告（当前页）`)
  } catch (e) {
    ElMessage.error(`导出失败：${(e as Error).message}`)
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
 * 批量生成兑换码（2026-11-01 新一轮第 57 轮）。
 * 用当前表单的奖励/次数/过期时间，一次生成 batchCount 个随机码。
 */
const batchCount = ref(10)
const batchLoading = ref(false)
/** 本次批量生成的码（2026-11-01 新一轮第 72 轮）：生成后弹窗展示供复制分发。 */
const batchGeneratedCodes = ref<AdminRedeemCodeView[]>([])
const batchCodesVisible = ref(false)

async function submitCodeBatch() {
  const reward = parseReward(codeForm.value.reward)
  if (!reward) {
    ElMessage.error('奖励格式错误，应为「coin:10000, gem:100」')
    return
  }
  const count = Math.floor(batchCount.value)
  if (!count || count < 1 || count > 100) {
    ElMessage.error('批量数量必须在 1..100')
    return
  }
  batchLoading.value = true
  try {
    const res = await createRedeemCodesBatch({
      count,
      reward,
      max_uses: codeForm.value.max_uses,
      expires_at: codeForm.value.expires_at || undefined,
    })
    batchGeneratedCodes.value = res.items ?? []
    batchCodesVisible.value = true
    ElMessage.success(`已批量生成 ${res.items.length} 个兑换码`)
    await loadAll()
  } catch (e) {
    ElMessage.error(`批量生成失败：${(e as Error).message}`)
  } finally {
    batchLoading.value = false
  }
}

/** 复制单个码到剪贴板（2026-11-01 新一轮第 72 轮）。 */
async function copyRedeemCode(code: string) {
  try {
    await navigator.clipboard.writeText(code)
    ElMessage.success(`已复制 ${code}`)
  } catch {
    ElMessage.error('复制失败，请手动选择复制')
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

/**
 * 审计操作类型 → 中文（2026-11-01 新一轮第 56 轮）。
 *
 * 服务端 action 是英文机器值（ban_user / grant_currency …），
 * 运营读审计时看到的是一串下划线英文。这里映射成中文；
 * 未知值回退原值（fail-loud：不吞掉新动作，宁可见英文也不猜错）。
 */
const ACTION_LABEL: Record<string, string> = {
  ban_user: '封禁用户',
  unban_user: '解封用户',
  grant_currency: '发放货币',
  revoke_currency: '回收货币',
  reset_password: '重置密码',
  create_announcement: '创建公告',
  update_announcement: '更新公告',
  delete_announcement: '删除公告',
  create_redeem_code: '创建兑换码',
  update_redeem_code: '更新兑换码',
  set_redeem_code_enabled: '启停兑换码',
  update_level: '更新关卡',
  update_shop: '更新商城商品',
  set_recompute_mode: '切换重算档位',
  delete_defense: '删除防线',
  verify_battle: '验真战报',
  update_loadout: '更新出战技能',
  mastery_reset: '重置专精',
  wash_gem: '洗练宝石',
  upgrade_skill: '升级技能',
  synthesize_skill: '熔炼技能',
  allocate_mastery: '分配专精',
  sign_in: '签到',
  claim_task: '领取任务',
  buy_item: '商城购买',
  redeem_code: '兑换码使用',
  settle_battle: '战斗结算',
  abandon_battle: '放弃战斗',
  challenge_defense: '挑战防线',
  save_defense: '设置防线',
  delete_my_defense: '删除防线',
  claim_offline_reward: '领取离线收益',
  change_password: '修改密码',
  upgrade_account: '游客转正',
  update_nickname: '修改昵称',
}

function actionLabel(action: string): string {
  return ACTION_LABEL[action] ?? action
}

/**
 * 用户相关后台操作集合（2026-11-01 新一轮第 68 轮）。
 * 这些 action 的 target 是**纯用户 id**（后台运营操作），可以点击跳转用户详情。
 * 玩家侧审计（update_loadout 等）的 target 是 `user#<id>` 格式，不在此列。
 */
const USER_TARGET_ACTIONS = new Set([
  'ban_user',
  'unban_user',
  'grant_currency',
  'revoke_currency',
  'reset_password',
])

function isUserTargetAction(action: string): boolean {
  return USER_TARGET_ACTIONS.has(action)
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
                <!-- 玩家端预览（2026-11-01 新一轮第 54 轮） -->
                <el-button @click="openPreview">预览玩家端</el-button>
                <el-button v-if="editingAnnId !== null" @click="cancelEditAnn">取消编辑</el-button>
              </el-form>
            </el-card>
          </el-col>
          <el-col :span="14" :xs="24" :sm="24" :md="14">
            <el-card shadow="never" header="已有公告">
              <div style="display: flex; align-items: center; flex-wrap: wrap; gap: 8px; margin-bottom: 8px">
                <!-- 公告筛选（2026-10-31 新一轮第 21 轮） -->
                <el-select v-model="annPublished" placeholder="发布状态" clearable style="width: 110px" @change="applyAnnFilter">
                  <el-option label="已发布" :value="true" />
                  <el-option label="草稿" :value="false" />
                </el-select>
                <el-date-picker
                  v-model="annDateRange"
                  type="daterange"
                  range-separator="至"
                  start-placeholder="创建开始日期"
                  end-placeholder="创建结束日期"
                  value-format="YYYY-MM-DD"
                  style="width: 260px"
                />
                <el-button @click="applyAnnFilter">查询</el-button>
                <span style="flex: 1" />
                <span class="muted" style="margin-right: 8px">共 {{ annTotal }} 条</span>
                <el-button size="small" @click="onExportAnnouncements">导出 CSV</el-button>
              </div>
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
                    <el-button size="small" text @click="previewAnnouncement(row as AdminAnnouncementView)">预览</el-button>
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
                @current-change="loadAnnouncements"
              />
            </el-card>
          </el-col>
        </el-row>

        <!-- 公告玩家端预览（2026-11-01 新一轮第 54 轮） -->
        <el-dialog v-model="previewVisible" title="玩家端预览" width="520px">
          <div style="border: 1rpx solid var(--el-border-color); border-radius: 8px; padding: 16px 20px">
            <div style="display: flex; justify-content: space-between; align-items: baseline; gap: 16px">
              <strong>{{ annForm.title }}</strong>
              <span class="muted" style="font-size: 12px">
                {{ annForm.published ? new Date().toLocaleString() : '未发布（仅草稿）' }}
              </span>
            </div>
            <div style="margin-top: 10px; white-space: pre-wrap; color: var(--el-text-color-regular); line-height: 1.7">
              {{ annForm.body }}
            </div>
          </div>
          <template #footer>
            <el-button type="primary" @click="closePreview">关闭</el-button>
          </template>
        </el-dialog>
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
              <!-- 批量生成（2026-11-01 新一轮第 57 轮）：复用当前奖励/次数/过期时间 -->
              <el-input-number
                v-model="batchCount"
                :min="1"
                :max="100"
                :disabled="editingCodeId !== null || batchLoading"
                style="width: 110px; margin-left: 8px"
              />
              <el-button :disabled="editingCodeId !== null || batchLoading" @click="submitCodeBatch">
                {{ batchLoading ? '批量生成中…' : '批量生成' }}
              </el-button>
              <el-button v-if="editingCodeId !== null" @click="cancelEditCode">取消编辑</el-button>
            </el-form-item>
          </el-form>
        </el-card>

        <el-card shadow="never" style="margin-top: 12px">
          <div style="display: flex; align-items: center; flex-wrap: wrap; gap: 8px; margin-bottom: 8px">
            <!-- 兑换码筛选（2026-10-31 新一轮第 20 轮） -->
            <el-input
              v-model="codeKeyword"
              placeholder="按码搜索（如 LYP-）"
              clearable
              style="width: 180px"
              @keyup.enter="applyCodeFilter"
              @clear="applyCodeFilter"
            />
            <el-select v-model="codeEnabled" placeholder="启用状态" clearable style="width: 110px" @change="applyCodeFilter">
              <el-option label="启用" :value="true" />
              <el-option label="停用" :value="false" />
            </el-select>
            <el-select v-model="codeExpired" placeholder="过期状态" clearable style="width: 110px" @change="applyCodeFilter">
              <el-option label="已过期" :value="true" />
              <el-option label="未过期" :value="false" />
            </el-select>
            <el-button @click="applyCodeFilter">查询</el-button>
            <span style="flex: 1" />
            <span class="muted" style="margin-right: 8px">共 {{ codeTotal }} 条</span>
            <el-button size="small" @click="onExportRedeemCodes">导出 CSV</el-button>
            <el-button size="small" @click="openRedeemUsages">使用记录</el-button>
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
            <el-table-column label="操作" width="150" fixed="right">
              <template #default="{ row }">
                <!-- 复制单个码（2026-11-01 新一轮第 76 轮）：客服/内测分发 -->
                <el-button size="small" text type="primary" @click="copyRedeemCode(row.code)">复制</el-button>
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
            @current-change="loadCodes"
          />
        </el-card>

        <!-- 兑换码使用记录（2026-10-31 新一轮第 17 轮） -->
        <el-dialog v-model="redeemUsageVisible" title="兑换码使用记录" width="720px">
          <div style="display: flex; align-items: center; flex-wrap: wrap; gap: 8px; margin-bottom: 8px">
            <!-- 使用记录筛选（2026-10-31 新一轮第 23 轮） -->
            <el-input
              v-model="redeemCodeKeyword"
              placeholder="按码搜索"
              clearable
              style="width: 180px"
              @keyup.enter="applyRedeemUsageFilter"
              @clear="applyRedeemUsageFilter"
            />
            <el-input-number
              v-model="redeemUserId"
              :controls="false"
              placeholder="用户 ID"
              style="width: 120px"
            />
            <el-button @click="applyRedeemUsageFilter">查询</el-button>
            <el-button @click="onExportRedeemUsagesCsv">导出 CSV</el-button>
          </div>
          <el-table :data="redeemUsages" size="small" stripe max-height="480">
            <el-table-column prop="id" label="ID" width="60" />
            <el-table-column prop="code" label="兑换码" min-width="150" />
            <el-table-column label="玩家" min-width="120">
              <!-- 点击跳转用户详情（2026-11-01 新一轮第 51 轮）：
                   核查补偿发放（谁用了码、是不是恶意批量）时直达玩家详情。 -->
              <template #default="{ row }">
                <el-link
                  v-if="row.nickname"
                  type="primary"
                  underline="never"
                  @click="openUserDetail(row.user_id)"
                >
                  {{ row.nickname }}
                </el-link>
                <span v-else class="muted">—</span>
              </template>
            </el-table-column>
            <el-table-column prop="user_id" label="用户 ID" width="90" />
            <el-table-column label="奖励" min-width="160">
              <template #default="{ row }">{{ rewardText(row.reward) }}</template>
            </el-table-column>
            <el-table-column label="使用时间" width="160">
              <template #default="{ row }">{{ fmtTime(row.created_at) }}</template>
            </el-table-column>
            <template #empty>
              <el-empty description="暂无使用记录" :image-size="60" />
            </template>
          </el-table>
          <el-pagination
            v-if="redeemUsageTotal > PAGE_SIZE"
            v-model:current-page="redeemUsagePage"
            :page-size="PAGE_SIZE"
            :total="redeemUsageTotal"
            layout="prev, pager, next, total"
            size="small"
            background
            @current-change="loadRedeemUsages"
          />
        </el-dialog>

        <!-- 批量生成的码（2026-11-01 新一轮第 72 轮）：生成后展示并支持复制分发 -->
        <el-dialog v-model="batchCodesVisible" title="批量生成的兑换码" width="560px">
          <div style="margin-bottom: 8px" class="muted">共 {{ batchGeneratedCodes.length }} 个码，点击码可复制。</div>
          <div style="max-height: 400px; overflow-y: auto">
            <div v-for="c in batchGeneratedCodes" :key="c.id" class="batch-code-row">
              <span class="mono">{{ c.code }}</span>
              <el-button size="small" text type="primary" @click="copyRedeemCode(c.code)">复制</el-button>
            </div>
          </div>
          <template #footer>
            <el-button type="primary" @click="batchCodesVisible = false">关闭</el-button>
          </template>
        </el-dialog>
      </el-tab-pane>

      <!-- 审计日志 -->
      <el-tab-pane label="审计日志" name="logs">
        <el-card shadow="never">
          <div style="display: flex; align-items: center; flex-wrap: wrap; gap: 8px; margin-bottom: 8px">
            <!-- 审计筛选（2026-10-31 新一轮第 18 轮） -->
            <el-input
              v-model="logAction"
              placeholder="操作类型（如 update_redeem）"
              clearable
              style="width: 200px"
              @keyup.enter="applyLogFilter"
              @clear="applyLogFilter"
            />
            <el-date-picker
              v-model="logDateRange"
              type="daterange"
              range-separator="至"
              start-placeholder="开始日期"
              end-placeholder="结束日期"
              value-format="YYYY-MM-DD"
              style="width: 260px"
            />
            <el-button @click="applyLogFilter">查询</el-button>
            <span style="flex: 1" />
            <span class="muted" style="margin-right: 8px">共 {{ logTotal }} 条</span>
            <el-button size="small" @click="onExportAuditLogs">导出 CSV</el-button>
          </div>
          <el-table :data="logs" size="small" stripe>
            <el-table-column prop="id" label="ID" width="55" />
            <el-table-column prop="username" label="管理员" width="110" />
            <el-table-column label="操作" width="150">
              <!-- 中文标签（2026-11-01 新一轮第 56 轮），未知回退原值 -->
              <template #default="{ row }">{{ actionLabel(row.action) }}</template>
            </el-table-column>
            <!--
              第 121 轮：旧列 admin_username / target_type / target_id / reason
              在服务端响应（AdminListAuditLogs）里**都不存在**——
              服务端字段是 username / action / target / detail。
              「对象类型」没有独立列（action 已含语义），合并成「对象」= target。
            -->
            <el-table-column label="对象" width="130">
              <!-- 用户相关操作的对象可点击跳转（2026-11-01 新一轮第 68 轮） -->
              <template #default="{ row }">
                <el-link
                  v-if="isUserTargetAction(row.action) && /^\d+$/.test(String(row.target))"
                  type="primary"
                  underline="never"
                  @click="openUserDetail(Number(row.target))"
                >
                  {{ row.target }}
                </el-link>
                <span v-else>{{ row.target }}</span>
              </template>
            </el-table-column>
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
            @current-change="loadLogs"
          />
        </el-card>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style scoped>
.batch-code-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8rpx 12rpx;
  border-bottom: 1rpx solid var(--el-border-color-lighter);
  font-size: 13px;
}

.batch-code-row .mono {
  font-family: monospace;
  word-break: break-all;
}
</style>
