<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { fetchSkills, updateSkill, type AdminSkill } from '@/api'

const loading = ref(false)
const skills = ref<AdminSkill[]>([])
const recipes = ref<Array<Record<string, number>>>([])
const activeElement = ref<string>('')
const activeKind = ref<string>('')

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
const FAMILY_LABEL: Record<string, string> = {
  flame: '焰系',
  frost: '冰系',
  volt: '电系',
  kinetic: '物理系',
  light: '光系',
  blight: '毒系',
  drone: '无人机',
  engineering: '工程',
}

/**
 * 热量与伤害的比值（DPS 效率）。
 *
 * 为什么要看这个比值：单看 base_damage 会被高热量技能显得弱，
 * 但热量是 I-2 的核心约束 —— 玩家只有 100 点热量预算。
 * 比值低的技能意味着「同样的热量预算换不到伤害」，是数值失衡的早期信号。
 */
function dpsPerHeat(s: AdminSkill): number {
  if (s.heat_cost <= 0) return s.base_damage
  const shotsPerSec = 1000 / Math.max(1, s.cooldown_ms)
  return (s.base_damage * shotsPerSec) / s.heat_cost
}

function heatTone(s: AdminSkill): 'success' | 'info' | 'warning' | 'danger' {
  const r = dpsPerHeat(s)
  if (r >= 8) return 'success'
  if (r >= 4) return 'info'
  if (r >= 2) return 'warning'
  return 'danger'
}

async function load() {
  loading.value = true
  try {
    const res = await fetchSkills()
    skills.value = res.items
    recipes.value = res.recipes
  } catch (e) {
    ElMessage.error(`技能加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

/** 关键：改任何一项都必须知道整体影响，因此逐字段确认而不是整行提交 */
async function patch(row: AdminSkill, field: keyof AdminSkill, label: string) {
  try {
    const { value } = await ElMessageBox.prompt(
      `修改「${row.name}」的${label}（当前 ${row[field]}）`,
      '编辑技能',
      { inputValue: String(row[field]), inputPattern: /^-?\d+$/, inputErrorMessage: '必须是整数' },
    )
    const res = await updateSkill(row.id, { [field]: Number(value) } as Partial<AdminSkill>)
    Object.assign(row, res.skill)
    ElMessage.success('已保存')
  } catch {
    /* 取消 */
  }
}

async function onEditDescr(row: AdminSkill) {
  try {
    const { value } = await ElMessageBox.prompt('修改技能描述', '编辑技能', {
      inputValue: row.descr,
      inputType: 'textarea',
    })
    const res = await updateSkill(row.id, { descr: value })
    Object.assign(row, res.skill)
    ElMessage.success('已保存')
  } catch {
    /* 取消 */
  }
}

// ⚠️ 键名必须与 Go 侧 SeedRecipe 的 json tag 一致（a/b/output/out_tier）。
// 曾经写成 PascalCase（A/B/Output/OutTier），结果每条配方都渲染成
// "#undefined + #undefined → #undefined（Tundefined）"，且不报任何错。
function recipeText(r: Record<string, number>): string {
  return `#${r.a} + #${r.b} → #${r.output}（T${r.out_tier}）`
}

onMounted(load)

/**
 * `el-table` 插槽给出的行类型是 element-plus 内部的 `DefaultRow`，
 * 它不能直接传给形参是 `AdminSkill` 的函数（TS2345），所以在调用点集中断言一次。
 *
 * cast 编译后被擦除，**运行时行为与之前逐字节相同**。
 *
 * ⚠️ 值得记一笔的是：这些调用此前**根本没有被类型检查**。
 * `components.d.ts` 不存在时 `el-table` 是未知组件、插槽行是 `any`，
 * 于是 22 处调用全部静默通过 —— 也就是说，
 * 「参数名拼错」或「形参类型改窄」这类错误在过去是**查不出来**的。
 * 断言让它们重新受检；但它断言的是**形状**，
 * 字段名本身拼错仍要靠运行时的 `undefined` 暴露。
 */
const asRow = (r: unknown) => r as AdminSkill
</script>

<template>
  <div v-loading="loading">
    <div class="page-title">技能与合成配方</div>
    <p class="page-subtitle">
      <strong>热量效率</strong>是 I-2 的平衡核心：玩家总热量预算只有 100 点。
      效率（每点热量换到的 DPS）过低的技能会让玩家在"想用"与"用得起"之间反复挫败。
    </p>

    <el-card shadow="never">
      <div class="toolbar">
        <el-radio-group v-model="activeElement" size="small">
          <el-radio-button value="">全部元素</el-radio-button>
          <el-radio-button v-for="(label, key) in ELEMENT_LABEL" :key="key" :value="key">
            {{ label }}
          </el-radio-button>
        </el-radio-group>
        <el-select v-model="activeKind" placeholder="全部类型" clearable size="small" style="width: 130px">
          <el-option label="主动" value="active" />
          <el-option label="被动" value="passive" />
        </el-select>
        <span class="toolbar-spacer" />
        <span class="muted">共 {{ skills.length }} 个技能 / {{ recipes.length }} 条配方</span>
      </div>

      <el-table
        :data="
          skills.filter(
            (s) =>
              (!activeElement || s.element === activeElement) &&
              (!activeKind || s.kind === activeKind),
          )
        "
        size="small"
        stripe
      >
        <el-table-column prop="id" label="ID" width="55" />
        <el-table-column label="元素" width="80">
          <template #default="{ row }">
            <span class="elem" :style="{ color: ELEMENT_TAG[row.element] }">
              {{ ELEMENT_LABEL[row.element] ?? row.element }}
            </span>
          </template>
        </el-table-column>
        <el-table-column prop="name" label="名称" min-width="120" />
        <el-table-column label="系别" width="90">
          <template #default="{ row }">{{ FAMILY_LABEL[row.family] ?? row.family }}</template>
        </el-table-column>
        <el-table-column prop="base_damage" label="伤害" width="76" />
        <el-table-column prop="heat_cost" label="热量" width="70" />
        <el-table-column prop="cooldown_ms" label="冷却(ms)" width="94" />
        <el-table-column label="热量效率" width="100">
          <template #default="{ row }">
            <el-tag :type="heatTone(asRow(row))" size="small">{{ dpsPerHeat(asRow(row)).toFixed(1) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="pierce" label="穿透" width="66" />
        <el-table-column prop="aoe_radius" label="溅射" width="70" />
        <el-table-column prop="apply_stacks" label="层数" width="66" />
        <el-table-column prop="unlock_level" label="解锁关" width="86" />
        <el-table-column label="操作" width="140" fixed="right">
          <template #default="{ row }">
            <el-button size="small" text @click="patch(asRow(row), 'base_damage', '基础伤害')">伤害</el-button>
            <el-button size="small" text @click="patch(asRow(row), 'heat_cost', '热量消耗')">热量</el-button>
            <el-button size="small" text @click="onEditDescr(asRow(row))">描述</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <el-card shadow="never" header="合成配方" style="margin-top: 12px">
      <div class="recipe-grid">
        <div v-for="(r, i) in recipes" :key="i" class="recipe">
          {{ recipeText(r) }}
        </div>
      </div>
      <p class="muted" style="margin-top: 10px">
        合成技能是 I-1 的深度来源：两级元素搭配能打出基础搭配打不出的高阶反应。
      </p>
    </el-card>
  </div>
</template>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
  flex-wrap: wrap;
}
.toolbar-spacer {
  flex: 1;
}
.muted {
  color: var(--lyp-muted);
  font-size: 12px;
}
.elem {
  font-weight: 600;
}
.recipe-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(210px, 1fr));
  gap: 6px;
}
.recipe {
  font-size: 12px;
  color: var(--lyp-muted);
  font-variant-numeric: tabular-nums;
}
</style>
