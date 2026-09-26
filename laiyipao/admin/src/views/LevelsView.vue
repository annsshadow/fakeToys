<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  fetchLevels,
  updateLevel,
  regenerateLevels,
  fetchLevelWaves,
  type AdminLevel,
} from '@/api'

const loading = ref(false)
const levels = ref<AdminLevel[]>([])
const total = ref(0)
const chapter = ref<number | undefined>(undefined)
const keyword = ref('')

// 波次抽屉
const waveDrawer = ref(false)
const waveTarget = ref<AdminLevel | null>(null)
const waveRows = ref<Array<{ wave_index: number; spawns: unknown }>>([])

/** 关卡难度是否在合理区间。低于 1000‰ 表示比基准还简单，高于 4200‰ 已超设计上限。 */
function difficultyTone(d: number): 'success' | 'info' | 'warning' | 'danger' {
  if (d <= 1200) return 'success'
  if (d <= 2400) return 'info'
  if (d <= 3400) return 'warning'
  return 'danger'
}

/** clear_rate 服务端已按百分数下发（clears*100/attempts），这里不要再乘 100 */
function clearRateTone(pct: number): 'success' | 'info' | 'warning' | 'danger' {
  if (pct >= 50) return 'success'
  if (pct >= 25) return 'info'
  if (pct >= 10) return 'warning'
  return 'danger'
}

const TERRAIN_LABEL: Record<string, string> = {
  oil_drum: '油桶',
  tidal_gate: '潮汐闸',
  rotor_vane: '旋转风障',
  collapse_wall: '崩塌掩体',
  charge_tower: '蓄能塔',
}

function terrainText(cfg: unknown): string {
  if (!Array.isArray(cfg) || cfg.length === 0) return '无'
  return (cfg as Array<{ kind: string }>).map((t) => TERRAIN_LABEL[t.kind] ?? t.kind).join('、')
}

async function load() {
  loading.value = true
  try {
    const res = await fetchLevels({ chapter: chapter.value, keyword: keyword.value || undefined })
    levels.value = res.items
    total.value = res.total
  } catch (e) {
    ElMessage.error(`关卡加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

async function toggleEnabled(row: AdminLevel) {
  try {
    const res = await updateLevel(row.id, { enabled: !row.enabled })
    Object.assign(row, res.level)
    ElMessage.success(`${row.name} 已${row.enabled ? '启用' : '停用'}`)
  } catch (e) {
    ElMessage.error((e as Error).message)
  }
}

async function onEdit(row: AdminLevel) {
  try {
    const { value } = await ElMessageBox.prompt(
      `修改「${row.name}」的防线血量（当前 ${row.base_hp}）`,
      '编辑关卡',
      { inputValue: String(row.base_hp), inputPattern: /^\d+$/, inputErrorMessage: '必须是正整数' },
    )
    const res = await updateLevel(row.id, { base_hp: Number(value) })
    Object.assign(row, res.level)
    ElMessage.success('已保存')
  } catch {
    /* 取消 */
  }
}

async function onRegen() {
  try {
    await ElMessageBox.confirm(
      '重新生成会用当前生成器覆盖全部 100 关的波次与难度（自定义的防线血量会保留）。确认？',
      '重新生成关卡',
      { type: 'warning' },
    )
  } catch {
    return
  }
  try {
    const res = await regenerateLevels()
    ElMessage.success(`已重新生成 ${res.generated} 关`)
    await load()
  } catch (e) {
    ElMessage.error((e as Error).message)
  }
}

async function openWaves(row: AdminLevel) {
  waveTarget.value = row
  waveDrawer.value = true
  try {
    const res = await fetchLevelWaves(row.id)
    waveRows.value = res.waves
  } catch (e) {
    ElMessage.error((e as Error).message)
  }
}

/** 把 spawns 摊平成可读文本 */
function spawnText(spawns: unknown): string {
  if (!Array.isArray(spawns)) return '—'
  return (spawns as Array<{ enemy_id: number; count: number; interval: number; delay: number }>)
    .map((s) => `#${s.enemy_id}×${s.count}（间隔 ${s.interval}ms，延迟 ${s.delay}ms）`)
    .join('；')
}

onMounted(load)
</script>

<template>
  <div v-loading="loading">
    <div class="page-title">关卡配置</div>
    <p class="page-subtitle">
      100 关由种子化生成器产出，<strong>难度必须全局单调</strong>（1000‰ → 4200‰）。
      若出现断崖，说明生成器的章节边界算错了 —— 这会让玩家在某一关突然卡死。
    </p>

    <el-card shadow="never">
      <div class="toolbar">
        <el-select v-model="chapter" placeholder="全部章节" clearable style="width: 140px" @change="load">
          <el-option v-for="c in 10" :key="c" :label="`第 ${c} 章`" :value="c" />
        </el-select>
        <el-input
          v-model="keyword"
          placeholder="按名称搜索"
          clearable
          style="width: 220px"
          @keyup.enter="load"
        />
        <el-button @click="load">查询</el-button>
        <el-button type="warning" plain @click="onRegen">重新生成 100 关</el-button>
        <span class="toolbar-spacer" />
        <span class="muted">共 {{ total }} 关</span>
      </div>

      <el-table :data="levels" size="small" stripe>
        <el-table-column prop="id" label="ID" width="60" />
        <el-table-column prop="chapter" label="章节" width="64" />
        <el-table-column prop="name" label="名称" min-width="150" />
        <el-table-column label="难度" width="110">
          <template #default="{ row }">
            <el-tag :type="difficultyTone(row.difficulty)" size="small">
              {{ row.difficulty }}‰
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="base_hp" label="防线血量" width="100" />
        <el-table-column prop="wave_count" label="波次" width="70" />
        <el-table-column prop="energy_cost" label="体力" width="70" />
        <el-table-column label="地形" min-width="140">
          <template #default="{ row }">
            <span :class="{ muted: terrainText(row.terrain_config) === '无' }">
              {{ terrainText(row.terrain_config) }}
            </span>
          </template>
        </el-table-column>
        <el-table-column label="通关率" width="100">
          <template #default="{ row }">
            <el-tag :type="clearRateTone(row.clear_rate)" size="small">
              {{ row.clear_rate.toFixed(1) }}%
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="avg_wave" label="平均波次" width="90" />
        <el-table-column label="状态" width="80">
          <template #default="{ row }">
            <el-tag :type="row.enabled ? 'success' : 'info'" size="small">
              {{ row.enabled ? '启用' : '停用' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="{ row }">
            <el-button size="small" text @click="openWaves(row)">波次</el-button>
            <el-button size="small" text @click="onEdit(row)">改血量</el-button>
            <el-button size="small" text @click="toggleEnabled(row)">
              {{ row.enabled ? '停用' : '启用' }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <el-drawer v-model="waveDrawer" :title="`${waveTarget?.name ?? ''} · 波次编排`" size="46%">
      <el-table :data="waveRows" size="small">
        <el-table-column prop="wave_index" label="波次" width="70" />
        <el-table-column label="刷怪">
          <template #default="{ row }">{{ spawnText(row.spawns) }}</template>
        </el-table-column>
      </el-table>
    </el-drawer>
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
}
</style>
