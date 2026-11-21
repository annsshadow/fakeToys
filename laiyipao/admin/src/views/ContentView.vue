<script setup lang="ts">
import PageSkeleton from "@/components/PageSkeleton.vue"
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
  fetchReconcile,
  type AdminAnnouncementView,
  type AdminAuditLogView,
  type AdminRedeemCodeView,
  type AdminRedeemUsageView,
} from '@/api'
import { auditDetailFull, auditDetailPreview, csvDataRowCount, exportDateLabel, fmtTime } from '@/utils/format'
import { cappedTotal } from '@/utils/offsetPager'

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
// R74：补「对象」过滤 —— 「这个兑换码/玩家被谁动过」必须能直接查。
const logAction = ref('')
const logTarget = ref('')
const logDateRange = ref<[string, string] | null>(null)

// R78：一致性对账（只读）。
const reconcileBusy = ref(false)

/** 跑一次一致性对账并弹结果。全绿报成功；有差异逐条列出（check/key/期望/实际）。 */
async function onReconcile() {
  reconcileBusy.value = true
  try {
    const rep = await fetchReconcile()
    if (rep.ok) {
      ElMessage.success(`对账通过：${rep.checks} 项不变量全部一致（耗时 ${rep.duration_ms}ms）`)
      return
    }
    const lines = rep.diffs.map((d) => `${d.check}｜${d.key}｜期望 ${d.expected}｜实际 ${d.actual}`)
    ElMessageBox.alert(
      `发现不一致（${rep.truncated ? '样本已截断，' : ''}按类计数见下）：\n` +
        lines.join('\n') +
        `\n\n各类违规数：${JSON.stringify(rep.violation_counts)}\n` +
        `说明：${rep.notes.join('；')}`,
      `对账发现 ${rep.diffs.length} 处不一致`,
      { confirmButtonText: '知道了', type: 'error' },
    )
  } catch (e) {
    ElMessage.error(`对账失败：${(e as Error).message}`)
  } finally {
    reconcileBusy.value = false
  }
}

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

// 使用记录请求序号（2026-11-18 第 59 轮）：旧筛选的慢响应不得覆盖新结果。
let redeemUsageReqSeq = 0
async function loadRedeemUsages() {
  const seq = ++redeemUsageReqSeq
  try {
    const res = await fetchAdminRedeemUsages(PAGE_SIZE, (redeemUsagePage.value - 1) * PAGE_SIZE, {
      // 按码搜索 / 用户过滤（2026-10-31 新一轮第 23 轮）
      code_keyword: redeemCodeKeyword.value || undefined,
      user_id: redeemUserId.value,
    })
    if (seq !== redeemUsageReqSeq) return
    redeemUsages.value = res.items
    redeemUsageTotal.value = res.total ?? res.items.length
  } catch (e) {
    if (seq !== redeemUsageReqSeq) return
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
    a.download = `redeem_usages_${exportDateLabel()}.csv`
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
// 公告请求序号（2026-11-18 第 59 轮）：旧筛选的慢响应不得覆盖新结果。
let annReqSeq = 0
async function loadAnnouncements() {
  const seq = ++annReqSeq
  try {
    const a = await fetchAnnouncements(PAGE_SIZE, (annPage.value - 1) * PAGE_SIZE, {
      published: annPublished.value,
      date_from: annDateRange.value?.[0] || undefined,
      date_to: annDateRange.value?.[1] || undefined,
    })
    if (seq !== annReqSeq) return
    announcements.value = a.items
    annTotal.value = a.total ?? a.items.length
  } catch (e) {
    if (seq !== annReqSeq) return
    ElMessage.error(`公告加载失败：${(e as Error).message}`)
  }
}

/** 只加载兑换码（2026-10-31 新一轮第 22 轮：按 tab 独立加载）。 */
// 兑换码请求序号（2026-11-18 第 59 轮）：旧筛选的慢响应不得覆盖新结果。
let codeReqSeq = 0
async function loadCodes() {
  const seq = ++codeReqSeq
  try {
    const c = await fetchRedeemCodes(PAGE_SIZE, (codePage.value - 1) * PAGE_SIZE, {
      keyword: codeKeyword.value || undefined,
      enabled: codeEnabled.value,
      expired: codeExpired.value,
    })
    if (seq !== codeReqSeq) return
    codes.value = c.items
    codeTotal.value = c.total ?? c.items.length
  } catch (e) {
    if (seq !== codeReqSeq) return
    ElMessage.error(`兑换码加载失败：${(e as Error).message}`)
  }
}

/** 只加载审计日志（2026-10-31 新一轮第 22 轮：按 tab 独立加载）。 */
// 审计日志请求序号（2026-11-18 第 59 轮）：旧筛选的慢响应不得覆盖新结果。
let logReqSeq = 0
async function loadLogs() {
  const seq = ++logReqSeq
  try {
    const l = await fetchAuditLogs(PAGE_SIZE, (logPage.value - 1) * PAGE_SIZE, {
      action: logAction.value || undefined,
      target: logTarget.value || undefined,
      date_from: logDateRange.value?.[0] || undefined,
      date_to: logDateRange.value?.[1] || undefined,
    })
    if (seq !== logReqSeq) return
    logs.value = l.items
    logTotal.value = l.total ?? l.items.length
  } catch (e) {
    if (seq !== logReqSeq) return
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
      // 与列表过滤同口径（2026-10-31 新一轮第 18 轮；R74 补 target）
      action: logAction.value || undefined,
      target: logTarget.value || undefined,
      date_from: logDateRange.value?.[0] || undefined,
      date_to: logDateRange.value?.[1] || undefined,
    })
    // 导出是全量筛选结果，条数必须来自实际 CSV，不能用当前页列表长度（第 58 轮）。
    const exportedCount = csvDataRowCount(await blob.text())
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `audit_logs_${exportDateLabel()}.csv`
    document.body.appendChild(a)
    a.click()
    a.remove()
    URL.revokeObjectURL(url)
    ElMessage.success(`已导出 ${exportedCount} 条审计日志（当前筛选结果）`)
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
    const blob = await exportRedeemCodesCsv({
      keyword: codeKeyword.value || undefined,
      enabled: codeEnabled.value,
      expired: codeExpired.value,
    })
    const exportedCount = csvDataRowCount(await blob.text())
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `redeem_codes_${exportDateLabel()}.csv`
    document.body.appendChild(a)
    a.click()
    a.remove()
    URL.revokeObjectURL(url)
    ElMessage.success(`已导出 ${exportedCount} 条兑换码（当前筛选结果）`)
  } catch (e) {
    ElMessage.error(`导出失败：${(e as Error).message}`)
  }
}

const annSaving = ref(false)
async function submitAnnouncement() {
  if (annSaving.value) return
  if (!annForm.value.title.trim() || !annForm.value.body.trim()) {
    ElMessage.error('标题与正文都不能为空')
    return
  }
  annSaving.value = true
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
    previewPubAt.value = null
    await loadAll()
  } catch (e) {
    ElMessage.error((e as Error).message)
  } finally {
    annSaving.value = false
  }
}

/**
 * 编辑公告（2026-10-31 实质优化第 23 轮）。
 * 此前只有创建/列表，发错内容无法修改。点击「编辑」把公告回填到表单。
 */
function startEditAnn(a: AdminAnnouncementView) {
  editingAnnId.value = a.id
  annForm.value = { title: a.title, body: a.body, published: a.published }
  previewPubAt.value = a.published_at ?? null
}

/**
 * 行级玩家端预览（2026-11-01 新一轮第 69 轮）。
 * 在列表里直接看某条公告的玩家端效果（不必先点编辑再预览）。
 */
function previewAnnouncement(a: AdminAnnouncementView) {
  previewForm.value = { title: a.title, body: a.body, published: a.published }
  previewPubAt.value = a.published_at ?? null
  showPreviewDialog()
}

/**
 * 复制公告正文（2026-11-01 新一轮第 98 轮）。
 * 运营把公告文案贴到群/工单/其它渠道时，无需手动选中。
 */
async function copyAnnBody(a: AdminAnnouncementView) {
  if (!a.body) return
  try {
    await navigator.clipboard.writeText(a.body)
    ElMessage.success('公告正文已复制')
  } catch {
    ElMessage.error('复制失败，请手动选择复制')
  }
}

function cancelEditAnn() {
  editingAnnId.value = null
  annForm.value = { title: '', body: '', published: false }
  previewPubAt.value = null
}

/**
 * 公告玩家端预览（2026-11-01 新一轮第 54 轮）。
 * 运营发布前想确认玩家看到的实际效果（标题/正文/发布时间），
 * 此前只能先发布再在玩家端看。这里用当前表单内容打开预览对话框。
 */
const previewVisible = ref(false)
/**
 * 第 55 轮：玩家端预览的发布时间必须显示服务端真实 published_at，
 * 而不是打开预览的此刻。行级预览/编辑时回填，新建时保持 null。
 */
const previewPubAt = ref<string | null>(null)
/**
 * 第 70 轮：预览对话框使用独立表单快照，绝不改写正在编辑的 annForm。
 * 此前列表行级「预览」直接把被预览公告写进 annForm，但 editingAnnId 不变，
 * 编辑 A 时预览 B 后点「更新」会把 B 的内容写回 A（用户可见数据损坏）。
 */
const previewForm = ref({ title: '', body: '', published: false })

function showPreviewDialog() {
  if (!previewForm.value.title.trim() || !previewForm.value.body.trim()) {
    ElMessage.error('标题与正文都不能为空')
    return
  }
  previewVisible.value = true
}

function openPreview() {
  previewForm.value = { ...annForm.value }
  showPreviewDialog()
}

function closePreview() {
  previewVisible.value = false
}

/**
 * 公告发布/撤回快捷切换（2026-11-01 新一轮第 84 轮）。
 * 发错状态（误发/误撤回）不必进编辑再改勾选：行内一键切换。
 * 撤回是可见性变化，玩家端缓存会被服务端立即失效（第 59 轮）。
 */
async function onToggleAnnPublished(a: AdminAnnouncementView) {
  const next = !a.published
  try {
    await updateAnnouncement(a.id, { title: a.title, body: a.body, published: next })
    ElMessage.success(next ? '公告已发布' : '公告已撤回')
    await loadAll()
  } catch (e) {
    ElMessage.error(`发布状态切换失败：${(e as Error).message}`)
  }
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
    const blob = await exportAnnouncementsCsv({
      published: annPublished.value,
      date_from: annDateRange.value?.[0] || undefined,
      date_to: annDateRange.value?.[1] || undefined,
    })
    const exportedCount = csvDataRowCount(await blob.text())
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `announcements_${exportDateLabel()}.csv`
    document.body.appendChild(a)
    a.click()
    a.remove()
    URL.revokeObjectURL(url)
    ElMessage.success(`已导出 ${exportedCount} 条公告（当前筛选结果）`)
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

const codeSaving = ref(false)
async function submitCode() {
  if (codeSaving.value) return
  const reward = parseReward(codeForm.value.reward)
  if (!reward) {
    ElMessage.error('奖励格式错误，应为「coin:10000, gem:100」')
    return
  }
  if (editingCodeId.value === null && !codeForm.value.code.trim()) {
    ElMessage.error('兑换码不能为空')
    return
  }
  codeSaving.value = true
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
  } finally {
    codeSaving.value = false
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
  if (batchLoading.value) return
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

/**
 * 审计详情文案（R98）。
 *
 * 列表只放 80 字符预览，完整 detail 在抽屉里看：
 * R74/R75 的 before/after 快照（单个 update_level 实测 260 字符）在 80
 * 字符处只剩 `{"before":…`，**新值 `patch` 完全不可见** —— 审计证据
 * 存在却读不到。抽屉是「详情页 UI」的最小形态，不改变列表扫读速度。
 */
const auditDetailVisible = ref(false)
const auditDetailRow = ref<AdminAuditLogView | null>(null)

function openAuditDetail(row: AdminAuditLogView) {
  auditDetailRow.value = row
  auditDetailVisible.value = true
}

/** 复制完整详情：工单/归档直接粘贴，不依赖人工选中。 */
async function copyAuditDetail() {
  const text = auditDetailFull(auditDetailRow.value?.detail)
  if (text === '—') return
  try {
    await navigator.clipboard.writeText(text)
    ElMessage.success('审计详情已复制')
  } catch {
    ElMessage.error('复制失败，请手动选择复制')
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
  update_level: '更新关卡',
  update_skill: '更新技能信息',
  regenerate_levels: '重生成关卡',
  update_shop_item: '更新商城商品',
  set_recompute_mode: '切换重算档位',
  delete_defense: '删除防线',
  verify_replay: '验真战报',
  set_redeem_enabled: '启停兑换码',
  logout: '退出登录',
  // 历史行别名：旧版本审计表里可能已经有这些 action，保留标签而不是让它们
  // 回退成英文（守卫只要求「服务端现有 action 都有标签」，不禁止别名）。
  update_shop: '更新商城商品（旧）',
  verify_battle: '验真战报（旧）',
  set_redeem_code_enabled: '启停兑换码（旧）',
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

/**
 * 复制审计对象（2026-11-01 新一轮第 95 轮）。
 * 运营把 target（用户/公告/兑换码 id）贴到工单时无需手动选中。
 */
async function copyAuditTarget(target: string) {
  if (!target) return
  try {
    await navigator.clipboard.writeText(target)
    ElMessage.success(`已复制 ${target}`)
  } catch {
    ElMessage.error('复制失败，请手动选择复制')
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
  <div class="page">
    <PageSkeleton v-if="loading" />
    <div class="page-header">
      <div class="page-heading">
        <div class="page-title">公告与兑换码</div>
      <p class="page-subtitle">
        兑换码是<strong>客服与内测</strong>的主要补偿通道。所有后台改动都会记入审计日志。
      </p>
      </div>
    </div>


    <section class="section-kicker" aria-label="内容运营概览">
      <span class="section-kicker-cn">内容运营概览</span>
      <span class="section-kicker-en">Content Operations</span>
    </section>
    <div class="kpi-strip">
      <div class="kpi-main"><div class="kpi-label">公告</div><div class="kpi-value">{{ annTotal }}</div></div>
      <div class="kpi-main"><div class="kpi-label">兑换码</div><div class="kpi-value">{{ codeTotal }}</div></div>
      <div class="kpi-sub-card"><div class="kpi-label">审计记录</div><div class="kpi-value">{{ logTotal }}</div></div>
      <div class="kpi-sub-card"><div class="kpi-label">当前面板</div><div class="kpi-value">{{ activeTab === 'announcements' ? '公告' : activeTab === 'codes' ? '兑换码' : '审计' }}</div></div>
    </div>
    <div class="filter-bar">
      <span class="result-count" aria-live="polite" role="status">共 {{ annTotal + codeTotal + logTotal }} 条内容/审计记录</span>
    </div>

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
                <el-button type="primary" :loading="annSaving" @click="submitAnnouncement">
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
              <div class="row-wrap mb-1">
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
              <el-table :data="announcements" size="small" stripe class="data-grid" style="font-variant-numeric: tabular-nums">
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
                    <el-button
                      size="small"
                      text
                      :type="row.published ? 'warning' : 'success'"
                      @click="onToggleAnnPublished(row as AdminAnnouncementView)"
                    >
                      {{ row.published ? '撤回' : '发布' }}
                    </el-button>
                    <el-button size="small" text type="primary" @click="copyAnnBody(row as AdminAnnouncementView)">复制正文</el-button>
                    <el-button size="small" text @click="previewAnnouncement(row as AdminAnnouncementView)">预览</el-button>
                    <el-button size="small" text @click="startEditAnn(row as AdminAnnouncementView)">编辑</el-button>
                    <el-button size="small" text type="danger" @click="onDeleteAnn(row as AdminAnnouncementView)">删除</el-button>
                  </template>
                </el-table-column>
        <template #empty>
          <div class="empty-state" role="status">
            <div class="empty-state-glyph" aria-hidden="true">◇</div>
            <div class="empty-state-title">{{ emptyText }}</div>
            <p class="empty-state-reason">该面板当前无公告；可用上方表单发布，或切换筛选条件。</p>
            <el-button class="empty-state-action" size="small" @click="loadAnnouncements">刷新公告</el-button>
          </div>
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
                :total="cappedTotal(annTotal, PAGE_SIZE)"
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
          <div style="border: 1px solid var(--el-border-color); border-radius: 8px; padding: 16px 20px">
            <div style="display: flex; justify-content: space-between; align-items: baseline; gap: 16px">
              <strong>{{ previewForm.title }}</strong>
              <span class="muted" style="font-size: 12px">
                {{ previewForm.published ? (previewPubAt ? fmtTime(previewPubAt) : '发布后生成') : '未发布（仅草稿）' }}
              </span>
            </div>
            <div style="margin-top: 10px; white-space: pre-wrap; color: var(--el-text-color-regular); line-height: 1.7">
              {{ previewForm.body }}
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
              <el-button type="primary" :loading="codeSaving" @click="submitCode">
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

        <el-card shadow="never" class="mt-2">
          <div class="row-wrap mb-1">
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
          <el-table :data="codes" size="small" stripe class="data-grid" style="font-variant-numeric: tabular-nums">
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
          <div class="empty-state" role="status">
            <div class="empty-state-glyph" aria-hidden="true">◇</div>
            <div class="empty-state-title">{{ emptyText }}</div>
            <p class="empty-state-reason">该面板当前无兑换码；可生成新码或切换筛选条件。</p>
            <el-button class="empty-state-action" size="small" @click="loadCodes">刷新兑换码</el-button>
          </div>
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
            :total="cappedTotal(codeTotal, PAGE_SIZE)"
            layout="prev, pager, next, total"
            size="small"
            background
            @current-change="loadCodes"
          />
        </el-card>

        <!-- 兑换码使用记录（2026-10-31 新一轮第 17 轮） -->
        <el-dialog v-model="redeemUsageVisible" title="兑换码使用记录" width="720px">
          <div class="row-wrap mb-1">
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
            <el-table-column label="兑换码" min-width="180">
              <!-- 复制码（2026-11-01 新一轮第 94 轮）：核查/工单 -->
              <template #default="{ row }">
                <span class="mono">{{ row.code }}</span>
                <el-button size="small" text type="primary" @click="copyRedeemCode(row.code)">复制</el-button>
              </template>
            </el-table-column>
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
            :total="cappedTotal(redeemUsageTotal, PAGE_SIZE)"
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
          <div class="row-wrap mb-1">
            <!-- 审计筛选（2026-10-31 新一轮第 18 轮） -->
            <el-input
              v-model="logAction"
              placeholder="操作类型（如 update_redeem）"
              clearable
              style="width: 200px"
              @keyup.enter="applyLogFilter"
              @clear="applyLogFilter"
            />
            <!-- R74：对象过滤（兑换码 id / user#id 等 target 精确匹配） -->
            <el-input
              v-model="logTarget"
              placeholder="对象（如兑换码 id / user#12）"
              clearable
              style="width: 190px"
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
            <!-- R78：一致性对账（只读）。把增量表/流水/兑换计数与直接聚合对在一起 -->
            <el-button size="small" :loading="reconcileBusy" @click="onReconcile">一致性对账</el-button>
            <el-button size="small" @click="onExportAuditLogs">导出 CSV</el-button>
          </div>
          <el-table :data="logs" size="small" stripe class="data-grid" style="font-variant-numeric: tabular-nums">
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
            <el-table-column label="对象" width="160">
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
                <!-- 复制对象（2026-11-01 新一轮第 95 轮）：追查贴工单 -->
                <el-button size="small" text type="primary" @click="copyAuditTarget(row.target)">复制</el-button>
              </template>
            </el-table-column>
            <el-table-column label="详情" min-width="180">
              <template #default="{ row }">
                <span class="audit-detail-preview">{{ auditDetailPreview(row.detail) }}</span>
                <!-- R98：80 字符预览会切掉 R74/R75 的 before/after 证据，完整内容进抽屉 -->
                <el-button
                  v-if="row.detail !== null && row.detail !== undefined"
                  size="small"
                  text
                  type="primary"
                  @click="openAuditDetail(row as AdminAuditLogView)"
                >
                  展开
                </el-button>
              </template>
            </el-table-column>
            <el-table-column label="时间" width="155">
              <template #default="{ row }">{{ fmtTime(row.created_at) }}</template>
            </el-table-column>
        <template #empty>
          <div class="empty-state" role="status">
            <div class="empty-state-glyph" aria-hidden="true">◇</div>
            <div class="empty-state-title">{{ emptyText }}</div>
            <p class="empty-state-reason">当前筛选条件下无审计记录；审计日志只读，操作会实时写入。</p>
            <el-button class="empty-state-action" size="small" @click="loadLogs">刷新日志</el-button>
          </div>
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
            :total="cappedTotal(logTotal, PAGE_SIZE)"
            layout="prev, pager, next, total"
            size="small"
            background
            @current-change="loadLogs"
          />
          <!-- R98：审计详情抽屉——列表预览之外唯一能看到完整 before/after 的地方 -->
          <el-drawer v-model="auditDetailVisible" title="审计详情" size="46%">
            <template v-if="auditDetailRow">
              <el-descriptions :column="1" size="small" border>
                <el-descriptions-item label="记录 ID">{{ auditDetailRow.id ?? '—' }}</el-descriptions-item>
                <el-descriptions-item label="管理员">{{ auditDetailRow.username || '—' }}</el-descriptions-item>
                <el-descriptions-item label="操作">{{ actionLabel(auditDetailRow.action) }}</el-descriptions-item>
                <el-descriptions-item label="对象">{{ auditDetailRow.target || '—' }}</el-descriptions-item>
                <el-descriptions-item label="时间">{{ fmtTime(auditDetailRow.created_at) }}</el-descriptions-item>
              </el-descriptions>
              <div class="audit-detail-actions">
                <el-button size="small" @click="copyAuditDetail">复制完整详情</el-button>
              </div>
              <pre class="audit-detail-full">{{ auditDetailFull(auditDetailRow.detail) }}</pre>
            </template>
          </el-drawer>
        </el-card>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style scoped>
.audit-detail-preview {
  margin-right: 4px;
}

.audit-detail-actions {
  margin: 12px 0 8px;
}

.audit-detail-full {
  margin: 0;
  padding: 12px;
  background: var(--el-fill-color-light);
  border-radius: 4px;
  font-size: 12px;
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-all;
}

.batch-code-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border-bottom: 1px solid var(--el-border-color-lighter);
  font-size: 13px;
}

.batch-code-row .mono {
  font-family: monospace;
  word-break: break-all;
}
</style>
