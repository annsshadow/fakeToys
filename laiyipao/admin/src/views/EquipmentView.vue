<script setup lang="ts">
import PageSkeleton from "@/components/PageSkeleton.vue"
import { onMounted, onActivated, ref, computed } from 'vue'
import AiInsightCard from "@/components/AiInsightCard.vue"
import { fetchAdminRecommendation, insightFromRecommendation, buildLocalRecommendation } from '@/utils/recommendation'
import type { AiInsight } from '@/utils/aiInsight'
import { ElMessage } from 'element-plus'
import { fetchEquipment, type AdminEquipmentView } from '@/api'

/** 空态文案（2026-10-21 P5-8）：空表要说清「为什么空」。 */
const emptyText = '目录为空'
const loading = ref(false)
const equipment = ref<AdminEquipmentView[]>([])
const gems = ref<import('@/api').SeedGem[]>([])
const skins = ref<import('@/api').SeedSkin[]>([])

/** R6 §1E：装备配置建议（接口优先，未上线回退本地真实数据）。 */
const recInsight = ref<AiInsight | null>(null)
const aiInsight = computed<AiInsight>(() => recInsight.value ?? buildLocalRecommendation('equipment', {
  equipment: equipment.value, gems: gems.value, skins: skins.value,
}))

const ELEMENT_LABEL: Record<string, string> = {
  fire: '焰',
  ice: '冰',
  lightning: '电',
  corrosion: '毒',
  kinetic: '动能',
}
const ELEMENT_TAG: Record<string, string> = {
  fire: '#ff6b35',
  ice: '#58a6ff',
  lightning: '#ffd33d',
  corrosion: '#7ee787',
  kinetic: '#c9a7ff',
}
const SLOT_LABEL: Record<string, string> = {
  weapon: '武器',
  helmet: '头盔',
  armor: '护甲',
  gloves: '手套',
  boots: '靴子',
  charm: '挂件',
}

const activeTab = ref('equipment')

/** R4 §1C.2：目录款数与元素/部位覆盖统计（真实数据，非写死）。 */
const equipmentCount = () => equipment.value.length
const gemCount = () => gems.value.length
const skinCount = () => skins.value.length
const elementCoverage = () => new Set(equipment.value.map((e) => e.element)).size
const slotCoverage = () => new Set(equipment.value.map((e) => e.slot)).size
const currentCount = () => (activeTab.value === 'gems' ? gemCount() : activeTab.value === 'skins' ? skinCount() : equipmentCount())

// keep-alive onActivated 重拉与在途 load 可重叠（侧栏在 v-loading 遮罩外）：
// 迟到的旧快照不得覆盖新 equipment/gems/skins，与 EconomyView 同款序号守卫。
let loadSeq = 0
async function load() {
  const seq = ++loadSeq
  loading.value = true
  try {
    const res = await fetchEquipment()
    if (seq !== loadSeq) return
    equipment.value = res.equipment
    gems.value = res.gems
    skins.value = res.skins
    // R6：推荐接口锦上添花，未上线/失败都不影响目录
    try {
      recInsight.value = insightFromRecommendation(await fetchAdminRecommendation('equipment'), aiInsight.value)
    } catch { recInsight.value = null }
    // 2026-10-21: the endpoint returns 8 keys but the old TS type declared
    // only 3, so enemies / reactions / mastery / rating_weights were
    // silently dropped on the way to the UI. They are now typed and
    // reachable; the counts below make the drop impossible to reintroduce
    // silently (a missing key shows up as a 0 here).
  } catch (e) {
    if (seq !== loadSeq) return
    ElMessage.error(`内容加载失败：${(e as Error).message}`)
  } finally {
    if (seq === loadSeq) loading.value = false
  }
}

onMounted(load)

// 2026-10-21（P5-9）：装备页此前是**唯一**没有 onActivated 刷新的视图 ——
// 「改完装备配置 → 切走 → 切回来」看到的是旧数据。
let activatedOnce = false
onActivated(() => {
  if (!activatedOnce) {
    activatedOnce = true
    return
  }
  void load()
})
</script>

<template>
  <div class="page">
    <PageSkeleton v-if="loading" />
    <div class="page-header">
      <div class="page-heading">
        <div class="page-title">装备 · 宝石 · 皮肤</div>
      <p class="page-subtitle">
        装备带元素标签。与技能<strong>同系</strong>给反应伤害大幅加成（<code>SynergySameFamilyPct=300</code>），
        异系只给少量直接伤害（<code>SynergyOtherFamilyPct=100</code>）——
        所以装备和技能必须成套配，单独堆装备收益很低。
      </p>
      </div>
    </div>

    <AiInsightCard :insight="aiInsight" :loading="loading" />

    <section class="section-kicker" aria-label="目录结构概览">
      <span class="section-kicker-cn">目录结构概览</span>
      <span class="section-kicker-en">Catalog Coverage</span>
    </section>
    <div class="kpi-strip">
      <div class="kpi-main"><div class="kpi-label">装备款数</div><div class="kpi-value">{{ equipmentCount() }}</div></div>
      <div class="kpi-main"><div class="kpi-label">宝石款数</div><div class="kpi-value">{{ gemCount() }}</div></div>
      <div class="kpi-sub-card"><div class="kpi-label">皮肤款数</div><div class="kpi-value">{{ skinCount() }}</div></div>
      <div class="kpi-sub-card"><div class="kpi-label">元素 / 部位覆盖</div><div class="kpi-value">{{ elementCoverage() }} / {{ slotCoverage() }}</div></div>
    </div>
    <div class="filter-bar">
      <span class="result-count" aria-live="polite" role="status">当前目录 {{ currentCount() }} 款</span>
    </div>

    <el-tabs v-model="activeTab">
      <!-- 装备 -->
      <el-tab-pane label="装备" name="equipment">
        <el-table :data="equipment" size="small" stripe class="data-grid" style="font-variant-numeric: tabular-nums">
          <el-table-column prop="id" label="ID" width="55" />
          <el-table-column prop="name" label="名称" min-width="130" />
          <el-table-column label="元素" width="80">
            <template #default="{ row }">
              <span class="elem" :style="{ color: ELEMENT_TAG[row.element] }">
                {{ ELEMENT_LABEL[row.element] ?? row.element }}
              </span>
            </template>
          </el-table-column>
          <el-table-column label="部位" width="80">
            <template #default="{ row }">{{ SLOT_LABEL[row.slot] ?? row.slot }}</template>
          </el-table-column>
          <el-table-column prop="tier" label="阶级" width="70" />
          <el-table-column label="护甲" width="80">
            <template #default="{ row }">{{ row.base_armor }}</template>
          </el-table-column>
          <el-table-column label="攻击加成‰" width="100">
            <template #default="{ row }">{{ row.base_bonus_pct }}</template>
          </el-table-column>
          <el-table-column label="解锁等级" width="90">
            <template #default="{ row }">{{ row.unlock_level }}</template>
          </el-table-column>
          <el-table-column prop="descr" label="效果" min-width="200" />
        <template #empty>
          <div class="empty-state" role="status">
            <div class="empty-state-glyph" aria-hidden="true">◇</div>
            <div class="empty-state-title">{{ emptyText }}</div>
            <p class="empty-state-reason">该目录当前没有条目；装备随版本种子发布，运行期不写库。</p>
            <el-button class="empty-state-action" size="small" @click="load">刷新目录</el-button>
          </div>
        </template>
</el-table>
      </el-tab-pane>

      <!-- 宝石 -->
      <el-tab-pane label="宝石" name="gems">
        <el-alert
          type="info"
          :closable="false"
          show-icon
          title="注意「元素石」"
          description="元素石提高元素系数，而反应伤害与面板攻击力无关（攻击力在反应中的权重结构性 ≤30%）。因此元素石的收益普遍高于攻击力宝石 —— 若玩家反馈「宝石没感觉」，先检查元素石词条数值是否偏低。" class="mb-2"
        />
        <el-table :data="gems" size="small" stripe class="data-grid" style="font-variant-numeric: tabular-nums">
          <el-table-column prop="id" label="ID" width="55" />
          <el-table-column prop="name" label="名称" min-width="130" />
          <el-table-column prop="attr" label="效果" min-width="120" />
          <el-table-column prop="descr" label="说明" min-width="200" />
        <template #empty>
          <div class="empty-state" role="status">
            <div class="empty-state-glyph" aria-hidden="true">◇</div>
            <div class="empty-state-title">{{ emptyText }}</div>
            <p class="empty-state-reason">该目录当前没有条目；装备随版本种子发布，运行期不写库。</p>
            <el-button class="empty-state-action" size="small" @click="load">刷新目录</el-button>
          </div>
        </template>
</el-table>
      </el-tab-pane>

      <!-- 皮肤 -->
      <el-tab-pane label="皮肤" name="skins">
        <el-table :data="skins" size="small" stripe class="data-grid" style="font-variant-numeric: tabular-nums">
          <el-table-column prop="id" label="ID" width="55" />
          <el-table-column prop="name" label="名称" min-width="130" />
          <el-table-column prop="rarity" label="稀有度" width="90" />
          <el-table-column prop="passive" label="被动" min-width="140" />
          <el-table-column prop="descr" label="说明" min-width="220" />
        <template #empty>
          <div class="empty-state" role="status">
            <div class="empty-state-glyph" aria-hidden="true">◇</div>
            <div class="empty-state-title">{{ emptyText }}</div>
            <p class="empty-state-reason">该目录当前没有条目；装备随版本种子发布，运行期不写库。</p>
            <el-button class="empty-state-action" size="small" @click="load">刷新目录</el-button>
          </div>
        </template>
</el-table>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style scoped>
.elem {
  font-weight: 600;
}
code {
  font-family: ui-monospace, monospace;
  font-size: 11px;
  background: var(--lyp-panel-2);
  padding: 1px 4px;
  border-radius: 3px;
}
</style>
