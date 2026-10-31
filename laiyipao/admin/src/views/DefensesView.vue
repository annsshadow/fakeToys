<script setup lang="ts">
import {onMounted, ref, computed, onActivated} from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { fetchDefenses, fetchAdminDefenseChallenges, exportDefenseChallengesCsv, deleteDefense, type AdminDefenseView, type AdminDefenseChallengeView } from '@/api'
import { fmtTime } from '@/utils/format'

/** 空态文案（2026-10-21 P5-8）：空表要说清「为什么空」。 */
const emptyText = '还没有人设置防线'
const loading = ref(false)
const rows = ref<AdminDefenseView[]>([])
const total = ref(0)

// 防线挑战记录（2026-10-31 新一轮第 30 轮）。
const challengeVisible = ref(false)
const challenges = ref<AdminDefenseChallengeView[]>([])
const challengeTotal = ref(0)
const challengePage = ref(1)
const challengeDefenseId = ref<number | undefined>(undefined)
const challengeChallengerId = ref<number | undefined>(undefined)

async function loadChallenges() {
  try {
    const res = await fetchAdminDefenseChallenges(50, (challengePage.value - 1) * 50, {
      defense_id: challengeDefenseId.value,
      challenger_id: challengeChallengerId.value,
    })
    challenges.value = res.items
    challengeTotal.value = res.total ?? res.items.length
  } catch (e) {
    ElMessage.error(`挑战记录加载失败：${(e as Error).message}`)
  }
}

/** 打开挑战记录对话框并加载第一页（2026-10-31 新一轮第 30 轮）。 */
function openChallenges() {
  challengeVisible.value = true
  challengePage.value = 1
  loadChallenges()
}

/** 挑战记录筛选变化：回到第一页并重新加载。 */
function applyChallengeFilter() {
  challengePage.value = 1
  loadChallenges()
}

/** 导出挑战记录 CSV（2026-10-31 新一轮第 32 轮）。 */
async function onExportChallengesCsv() {
  try {
    const blob = await exportDefenseChallengesCsv({
      defense_id: challengeDefenseId.value,
      challenger_id: challengeChallengerId.value,
    })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `defense_challenges_${new Date().toISOString().slice(0, 10)}.csv`
    document.body.appendChild(a)
    a.click()
    a.remove()
    URL.revokeObjectURL(url)
  } catch (e) {
    ElMessage.error(`导出失败：${(e as Error).message}`)
  }
}

/** 偷取奖励格式：`100 金币`、`5 钻石`，多项用逗号分隔。 */
function rewardText(loot: Record<string, number> | undefined): string {
  if (!loot) return ''
  const label: Record<string, string> = { coin: '金币', gem: '钻石', energy: '体力', keys: '钥匙' }
  return Object.entries(loot)
    .map(([k, v]) => `${v} ${label[k] ?? k}`)
    .join('、')
}

/**
 * 防线值守（I-5）的关键设计：
 * 服务端**不跑战斗引擎**。挑战结果由客户端本地模拟后上报，
 * 服务端只做「快照没变 + 每日次数 + 资源守恒」的校验。
 *
 * 因此这个列表最该关注的不是胜负，而是：
 * 1. 快照哈希是否稳定（防线被偷改）
 * 2. 胜率是否异常（100% 或 0% 通常意味着伪造）
 */
const suspiciousRows = computed(() =>
  rows.value.filter((r) => {
    const w = r.wins
    const l = r.losses
    if (w + l < 3) return false
    return w / (w + l) > 0.98 || w / (w + l) < 0.02
  }),
)

function winRate(r: AdminDefenseView): string {
  const total2 = r.wins + r.losses
  if (total2 === 0) return '—'
  return `${((r.wins / total2) * 100).toFixed(1)}%`
}

function rateTone(r: AdminDefenseView): 'success' | 'info' | 'warning' | 'danger' {
  const w = r.wins
  const l = r.losses
  const t = w + l
  if (t === 0) return 'info'
  const rate = w / t
  if (rate > 0.98 || rate < 0.02) return 'danger'
  if (rate > 0.75 || rate < 0.25) return 'warning'
  return 'success'
}

const PAGE_SIZE = 50
const page = ref(1)

// 防线名/玩家昵称搜索 + owner_id 过滤（2026-10-31 新一轮第 19 轮）。
const keyword = ref('')
const ownerId = ref<number | undefined>(undefined)

async function load() {
  loading.value = true
  try {
    const res = await fetchDefenses(PAGE_SIZE, (page.value - 1) * PAGE_SIZE, {
      keyword: keyword.value || undefined,
      owner_id: ownerId.value,
    })
    rows.value = res.items
    total.value = res.total
  } catch (e) {
    ElMessage.error(`防线加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

/**
 * 删除玩家防线（2026-10-31 实质优化第 26 轮）。
 * 违规防线名（侮辱/广告等）运营此前只能看到、无法处理。
 * 删除会连带该防线的挑战记录（CASCADE），需二次确认。
 */
async function onDeleteDefense(r: AdminDefenseView) {
  try {
    await ElMessageBox.confirm(
      `删除「${r.owner_name}」的防线「${r.name}」？该玩家将失去防线，相关挑战记录也会删除。`,
      '删除防线',
      { type: 'warning', confirmButtonText: '删除' },
    )
  } catch {
    return // 取消
  }
  try {
    await deleteDefense(r.id)
    ElMessage.success('防线已删除')
    await load()
  } catch (e) {
    ElMessage.error(`删除失败：${(e as Error).message}`)
  }
}

/**
 * `el-table` 插槽给出的行类型是 element-plus 内部的 `DefaultRow`，
 * 它不能直接传给形参是 `AdminDefenseView` 的函数（TS2345），
 * 所以在调用点集中断言一次（编译后被擦除，运行时行为不变）。
 *
 * ⚠️ 值得记一笔：这些调用此前**根本没有被类型检查**——
 * `components.d.ts` 不存在时 `el-table` 是未知组件、插槽行是 `any`，
 * 于是 22 处调用全部静默通过。也就是说「参数名拼错」这类错误在过去查不出来。
 */
const asRow = (r: unknown) => r as AdminDefenseView

onMounted(load)

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
load()
})
</script>

<template>
  <div v-loading="loading">
    <div class="page-title">防线值守</div>
    <p class="page-subtitle">
      玩家把构筑固化成防线快照，其他玩家挑战它时<strong>在客户端用同一套引擎本地模拟</strong>，
      胜者窃取 10% 资源。服务端不跑战斗引擎，因此不需要维护两套引擎保持同步。
    </p>

    <el-alert
      v-if="suspiciousRows.length > 0"
      type="warning"
      show-icon
      :closable="false"
      :title="`${suspiciousRows.length} 条防线胜率异常（接近 100% 或 0%）`"
      description="正常防线在样本量足够时胜率应在中间区间。极端胜率通常意味着挑战结果是伪造上报的，建议抽查战报哈希。"
      style="margin-bottom: 12px"
    />

    <el-card shadow="never">
      <div class="toolbar">
        <!-- 防线搜索（2026-10-31 新一轮第 19 轮） -->
        <el-input
          v-model="keyword"
          placeholder="按防线名 / 玩家昵称搜索"
          clearable
          style="width: 220px"
          @keyup.enter="((page = 1), load())"
          @clear="((page = 1), load())"
        />
        <el-input-number
          v-model="ownerId"
          :controls="false"
          placeholder="属主 ID"
          style="width: 120px"
        />
        <el-button @click="((page = 1), load())">查询</el-button>
        <el-pagination
          v-if="total > PAGE_SIZE"
          v-model:current-page="page"
          :page-size="PAGE_SIZE"
          :total="total"
          layout="prev, pager, next, total"
          size="small"
          background
          @current-change="load"
        />
        <el-button @click="load">刷新</el-button>
        <el-button @click="openChallenges">挑战记录</el-button>
        <span class="toolbar-spacer" />
        <span class="muted">共 {{ total }} 条防线</span>
      </div>

      <el-table :data="rows" size="small" stripe>
        <el-table-column prop="id" label="ID" width="60" />
        <el-table-column prop="owner_id" label="属主 ID" width="80" />
        <el-table-column prop="owner_name" label="属主" min-width="120" />
        <el-table-column label="防线名" min-width="120">
          <template #default="{ row }">{{ row.name || '（未命名）' }}</template>
        </el-table-column>
        <el-table-column label="战力" width="90">
          <template #default="{ row }">{{ row.power }}</template>
        </el-table-column>
        <el-table-column label="元素覆盖" width="94">
          <template #default="{ row }">{{ row.element_coverage }}/5</template>
        </el-table-column>
        <el-table-column label="专精已点" width="94">
          <template #default="{ row }">{{ row.mastery_done }}</template>
        </el-table-column>
        <el-table-column label="战绩" width="110">
          <template #default="{ row }">
            {{ row.wins }} 胜 / {{ row.losses }} 负
          </template>
        </el-table-column>
        <el-table-column label="胜率" width="90">
          <template #default="{ row }">
            <el-tag :type="rateTone(asRow(row))" size="small">{{ winRate(asRow(row)) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="护盾至" width="155">
          <template #default="{ row }">{{ fmtTime(row.shielded_until) }}</template>
        </el-table-column>
        <el-table-column label="快照哈希" min-width="150">
          <template #default="{ row }">
            <code class="hash">{{ row.snapshot_hash ? row.snapshot_hash.slice(0, 16) + '…' : '—' }}</code>
          </template>
        </el-table-column>
        <el-table-column label="更新时间" width="155">
          <template #default="{ row }">{{ fmtTime(row.updated_at) }}</template>
        </el-table-column>
        <!-- 删除（2026-10-31 实质优化第 26 轮） -->
        <el-table-column label="操作" width="70" fixed="right">
          <template #default="{ row }">
            <el-button size="small" text type="danger" @click="onDeleteDefense(asRow(row))">删除</el-button>
          </template>
        </el-table-column>
        <template #empty>
          <el-empty :description="emptyText" :image-size="60" />
        </template>
</el-table>
    </el-card>

    <!-- 防线挑战记录（2026-10-31 新一轮第 30 轮） -->
    <el-dialog v-model="challengeVisible" title="防线挑战记录" width="760px">
      <div style="display: flex; align-items: center; flex-wrap: wrap; gap: 8px; margin-bottom: 8px">
        <el-input-number
          v-model="challengeDefenseId"
          :controls="false"
          placeholder="防线 ID"
          style="width: 120px"
        />
        <el-input-number
          v-model="challengeChallengerId"
          :controls="false"
          placeholder="挑战者 ID"
          style="width: 130px"
        />
        <el-button @click="applyChallengeFilter">查询</el-button>
        <el-button @click="onExportChallengesCsv">导出 CSV</el-button>
      </div>
      <el-table :data="challenges" size="small" stripe max-height="480">
        <el-table-column prop="id" label="ID" width="60" />
        <el-table-column prop="defense_id" label="防线 ID" width="80" />
        <el-table-column prop="defense_name" label="防线名" min-width="100" />
        <el-table-column prop="challenger_nickname" label="挑战者" min-width="110" />
        <el-table-column prop="challenger_id" label="挑战者 ID" width="90" />
        <el-table-column label="结果" width="80">
          <template #default="{ row }">
            <el-tag :type="row.won ? 'danger' : 'success'" size="small">
              {{ row.won ? '挑战成功' : '防守成功' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="偷取" min-width="120">
          <template #default="{ row }">{{ rewardText(row.loot) }}</template>
        </el-table-column>
        <el-table-column label="时间" width="155">
          <template #default="{ row }">{{ fmtTime(row.created_at) }}</template>
        </el-table-column>
        <template #empty>
          <el-empty description="暂无挑战记录" :image-size="60" />
        </template>
      </el-table>
      <el-pagination
        v-if="challengeTotal > 50"
        v-model:current-page="challengePage"
        :page-size="50"
        :total="challengeTotal"
        layout="prev, pager, next, total"
        size="small"
        background
        @current-change="loadChallenges"
      />
    </el-dialog>
  </div>
</template>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 12px;
}
.toolbar-spacer {
  flex: 1;
}
.muted {
  color: var(--lyp-muted);
  font-size: 12px;
}
.hash {
  font-family: ui-monospace, monospace;
  font-size: 11px;
  color: var(--lyp-muted);
}
</style>
