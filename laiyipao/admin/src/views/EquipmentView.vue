<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { ElMessage } from 'element-plus'
import { fetchEquipment } from '@/api'

const loading = ref(false)
const equipment = ref<Record<string, unknown>[]>([])
const gems = ref<Record<string, unknown>[]>([])
const skins = ref<Record<string, unknown>[]>([])

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

function str(o: Record<string, unknown>, k: string): string {
  const v = o[k]
  return v === undefined || v === null ? '' : String(v)
}

async function load() {
  loading.value = true
  try {
    const res = await fetchEquipment()
    equipment.value = res.equipment
    gems.value = res.gems
    skins.value = res.skins
  } catch (e) {
    ElMessage.error(`内容加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

onMounted(load)
</script>

<template>
  <div v-loading="loading">
    <div class="page-title">装备 · 宝石 · 皮肤</div>
    <p class="page-subtitle">
      装备带元素标签。与技能<strong>同系</strong>给反应伤害大幅加成（<code>SynergySameFamilyPct=300</code>），
      异系只给少量直接伤害（<code>SynergyOtherFamilyPct=100</code>）——
      所以装备和技能必须成套配，单独堆装备收益很低。
    </p>

    <el-tabs v-model="activeTab">
      <!-- 装备 -->
      <el-tab-pane label="装备" name="equipment">
        <el-table :data="equipment" size="small" stripe>
          <el-table-column prop="id" label="ID" width="55" />
          <el-table-column prop="name" label="名称" min-width="130" />
          <el-table-column label="元素" width="80">
            <template #default="{ row }">
              <span class="elem" :style="{ color: ELEMENT_TAG[str(row, 'element')] }">
                {{ ELEMENT_LABEL[str(row, 'element')] ?? str(row, 'element') }}
              </span>
            </template>
          </el-table-column>
          <el-table-column label="部位" width="80">
            <template #default="{ row }">{{ SLOT_LABEL[str(row, 'slot')] ?? str(row, 'slot') }}</template>
          </el-table-column>
          <el-table-column prop="tier" label="阶级" width="70" />
          <el-table-column prop="descr" label="效果" min-width="200" />
        </el-table>
      </el-tab-pane>

      <!-- 宝石 -->
      <el-tab-pane label="宝石" name="gems">
        <el-alert
          type="info"
          :closable="false"
          show-icon
          title="注意「元素石」"
          description="元素石提高元素系数，而反应伤害与面板攻击力无关（攻击力在反应中的权重结构性 ≤30%）。因此元素石的收益普遍高于攻击力宝石 —— 若玩家反馈「宝石没感觉」，先检查元素石词条数值是否偏低。"
          style="margin-bottom: 12px"
        />
        <el-table :data="gems" size="small" stripe>
          <el-table-column prop="id" label="ID" width="55" />
          <el-table-column prop="name" label="名称" min-width="130" />
          <el-table-column prop="descr" label="效果" min-width="200" />
          <el-table-column prop="descr" label="说明" min-width="160" />
        </el-table>
      </el-tab-pane>

      <!-- 皮肤 -->
      <el-tab-pane label="皮肤" name="skins">
        <el-table :data="skins" size="small" stripe>
          <el-table-column prop="id" label="ID" width="55" />
          <el-table-column prop="name" label="名称" min-width="130" />
          <el-table-column prop="descr" label="说明" min-width="220" />
          <el-table-column prop="price" label="价格" width="100" />
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
  background: #1c2430;
  padding: 1px 4px;
  border-radius: 3px;
}
</style>
