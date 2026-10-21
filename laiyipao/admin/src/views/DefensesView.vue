<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { fetchDefenses, type AdminDefenseView } from '@/api'
import { fmtTime } from '@/utils/format'

const loading = ref(false)
const rows = ref<AdminDefenseView[]>([])
const total = ref(0)

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

async function load() {
  loading.value = true
  try {
    const res = await fetchDefenses(100)
    rows.value = res.items
    total.value = res.total
  } catch (e) {
    ElMessage.error(`防线加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
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
        <el-button @click="load">刷新</el-button>
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
      </el-table>
    </el-card>
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
