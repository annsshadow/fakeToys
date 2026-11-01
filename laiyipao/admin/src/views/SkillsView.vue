<script setup lang="ts">
import {onMounted, ref, onActivated} from 'vue'
import { useAdminRole } from '@/utils/adminRole'
import { ElMessage, ElMessageBox } from 'element-plus'
import { isCancel } from '@/utils/format'
import { fetchSkills, updateSkill, type AdminSkillView, type SeedRecipe } from '@/api'

/** 空态文案（2026-10-21 P5-8）：空表要说清「为什么空」。 */
const emptyText = '技能表为空'
const loading = ref(false)
// 2026-10-21（P5-6 / 二轮审计 P2-3）：readonly 账号看不到写入口。
const { isReadonly } = useAdminRole()
const skills = ref<AdminSkillView[]>([])
const recipes = ref<SeedRecipe[]>([])
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
function dpsPerHeat(s: AdminSkillView): number {
  if (s.heat_cost <= 0) return s.base_damage
  const shotsPerSec = 1000 / Math.max(1, s.cooldown_ms)
  return (s.base_damage * shotsPerSec) / s.heat_cost
}

function heatTone(s: AdminSkillView): 'success' | 'info' | 'warning' | 'danger' {
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

/**
 * 技能编辑（第 23 轮收紧）：只开放 **name / descr** 两个展示字段。
 *
 * ⚠️ 此前白名单还有 11 个战斗数值/语义键（base_damage / heat_cost / … /
 * kind），界面也提供了编辑入口 —— 那是一个会炸的入口：
 * 结算的 S 段校验按库值重算，客户端引擎按 /config 静态种子算同一段；
 * 在后台改 base_damage 之后，所有带 replay_skills 的真实结算都会 422
 * （enforce 模式默认开）。战斗数值只能随版本发布（domain 种子），
 * 运行时改库 = 两端失配，所以它们从服务端白名单移除，这里同步只留展示字段。
 * 数值列保持只读展示（DPS 效率列照旧），「为什么改不了」写在这里。
 */
const editOpen = ref(false)
const editSaving = ref(false)
const editRow = ref<AdminSkillView | null>(null)
const editForm = ref({
  name: '',
  descr: '',
})

function openEdit(row: AdminSkillView) {
  editRow.value = row
  editForm.value = {
    name: row.name,
    descr: row.descr,
  }
  editOpen.value = true
}

async function submitEdit() {
  if (editSaving.value || !editRow.value) return
  const f = editForm.value
  if (!f.name.trim()) {
    ElMessage.error('技能名不能为空')
    return
  }
  editSaving.value = true
  try {
    const res = await updateSkill(editRow.value.id, { ...f, name: f.name.trim() })
    Object.assign(editRow.value, res.skill)
    editOpen.value = false
    ElMessage.success('已保存')
  } catch (e) {
    ElMessage.error((e as Error).message)
  } finally {
    editSaving.value = false
  }
}

async function onEditDescr(row: AdminSkillView) {
  try {
    const { value } = await ElMessageBox.prompt('修改技能描述', '编辑技能', {
      inputValue: row.descr,
      inputType: 'textarea',
    })
    const res = await updateSkill(row.id, { descr: value })
    Object.assign(row, res.skill)
    ElMessage.success('已保存')
  } catch (e) {
    if (isCancel(e)) return
    ElMessage.error(`保存失败：${(e as Error).message}`)
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
const asRow = (r: unknown) => r as AdminSkillView
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
            <el-button v-if="!isReadonly" size="small" text @click="openEdit(asRow(row))">编辑</el-button>
            <el-button v-if="!isReadonly" size="small" text @click="onEditDescr(asRow(row))">描述</el-button>
          </template>
        </el-table-column>
        <template #empty>
          <el-empty :description="emptyText" :image-size="60" />
        </template>
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
    <!--
      技能编辑对话框（第 23 轮收紧）：只有 name / descr 两个可写字段。
      战斗数值随版本发布（domain 种子），运行时改库会让 S 段校验两端失配。
    -->
    <el-dialog v-model="editOpen" title="编辑技能" width="560px">
      <p class="muted" style="margin: 0 0 12px">
        伤害 / 热量 / 冷却等战斗数值为版本内只读：结算校验按发布配置重算，
        在此处修改会造成客户端与服务端失配（全部结算被拒）。
      </p>
      <el-form label-width="100px" size="small">
        <el-form-item label="名称">
          <el-input v-model="editForm.name" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input v-model="editForm.descr" type="textarea" :rows="2" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editOpen = false">取消</el-button>
        <el-button type="primary" :loading="editSaving" @click="submitEdit">保存</el-button>
      </template>
    </el-dialog>
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
