<script setup lang="ts">
import {onMounted, ref, onActivated} from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  fetchLevels,
  updateLevel,
  regenerateLevels,
  fetchLevelWaves,
  type AdminLevelView,
} from '@/api'
import { useAdminRole } from '@/utils/adminRole'

/** 空态文案（2026-10-21 P5-8）：空表要说清「为什么空」。 */
const emptyText = '没有匹配的关卡'
const loading = ref(false)
const levels = ref<AdminLevelView[]>([])
// 2026-10-21（P5-6）：readonly 账号看不到写入口。
const { isReadonly } = useAdminRole()
const total = ref(0)
const chapter = ref<number | undefined>(undefined)
/**
 * 章节下拉选项从**首次无筛选的全量数据**推导，不硬编码章数。
 * 旧实现写死 `v-for="c in 10"`，而关卡内容实际只有 6 章（levelgen 的
 * Chapters 表）——选「第 7~10 章」必然 0 行，运营以为「该章无数据」，
 * 其实是那个章根本不存在。内容加章后这里自动跟上，不用再改 UI。
 */
const chapterOptions = ref<number[]>([])
/** 全量章节（只有无筛选的拉取才更新它 —— 见 load 里的注释）。 */
const allChapters = ref<number[]>([])
const keyword = ref('')

// 波次抽屉
const waveDrawer = ref(false)
const waveTarget = ref<AdminLevelView | null>(null)
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
    // 只在**无章节筛选**时从全量推导下拉选项：筛选后 items 只剩该章，
    // 拿它推导会把其它章从下拉里删掉，选项本身就漏了。
    // ⚠️ 2026-10-21（二轮审计 P2-5）：关键词搜索**也会**把下拉选项挤掉 ——
    // 搜索命中的一关可能只在第 1 章，于是 `chapterOptions` 塌成 [1]，
    // 运营再也选不到第 3 章（清空搜索才恢复）。
    //
    // 所以：只要**没有**章节筛选，就从**全量**关卡推导选项，
    // 而不是从（可能已被过滤的）items 推导。
    //
    // ⚠️ 2026-10-21（二轮审计 P2-5）：章节下拉选项**不能**从被过滤过的
    // `res.items` 推导。
    //
    // 搜索「龙」→ 只有关键词命中的关卡（可能都在第 1 章）→
    // 选项塌成 [1] → 运营**再也选不到第 3 章**（清空搜索才恢复）。
    //
    // 修法：只在**无筛选**的那次拉取里更新「全量章节」，之后一直用它。
    if (chapter.value === undefined && !keyword.value) {
      const seen = new Set<number>()
      for (const l of res.items) seen.add(l.chapter)
      allChapters.value = [...seen].sort((a, b) => a - b)
    }
    chapterOptions.value = allChapters.value
    levels.value = res.items
    total.value = res.total
  } catch (e) {
    ElMessage.error(`关卡加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

async function toggleEnabled(row: AdminLevelView) {
  const next = !row.enabled
  // 2026-10-21（P5-2）：停用一关会影响**所有玩家**（他们点进去拿不到关卡数据），
  // 此前一次点击直接生效，唯一的反馈是「已停用」三个字。
  try {
    await ElMessageBox.confirm(
      next
        ? `启用「${row.name}」？该关将重新出现在玩家的选关列表里。`
        : `停用「${row.name}」？所有玩家都将无法进入这一关。`,
      next ? '启用关卡' : '停用关卡',
      { type: next ? 'info' : 'warning', confirmButtonText: next ? '启用' : '停用' },
    )
  } catch {
    return // 取消
  }
  try {
    const res = await updateLevel(row.id, { enabled: next })
    Object.assign(row, res.level)
    ElMessage.success(`${row.name} 已${row.enabled ? '启用' : '停用'}`)
  } catch (e) {
    ElMessage.error((e as Error).message)
  }
}

/**
 * 完整的关卡编辑（2026-10-21 P5-3）。
 *
 * 此前 UI 只有两个可写入口（`base_hp` 与 `enabled`），
 * 而服务端的白名单有 **9** 个键 —— 于是 7 个字段
 * （name / wave_count / difficulty / energy_cost / star_targets /
 *   terrain_config / is_boss）**在界面上根本不存在**。
 *
 * 结果：运营想调「波次数」或「星级门槛」，只能去改数据库。
 * 一个后台不能做的事，就是**不存在的能力**。
 *
 * 之前用 `ElMessageBox.prompt` 逐字段弹窗 —— 一次只能改一个字段，
 * 改 7 个字段要弹 7 次。这里改成**一个对话框改一整行**。
 */
const editOpen = ref(false)
const editSaving = ref(false)
const editRow = ref<AdminLevelView | null>(null)
const editForm = ref({
  name: '',
  base_hp: 0,
  wave_count: 0,
  difficulty: 0,
  energy_cost: 0,
  star_targets: [0, 0, 0] as number[],
  is_boss: false,
})

/** 星级门槛的输入（逗号分隔的三个分数）。 */
const starTargetsText = ref('0, 0, 0')

function openEdit(row: AdminLevelView) {
  editRow.value = row
  editForm.value = {
    name: row.name,
    base_hp: row.base_hp,
    wave_count: row.wave_count,
    difficulty: row.difficulty,
    energy_cost: row.energy_cost,
    star_targets: [...(row.star_targets ?? [])],
    is_boss: row.is_boss,
  }
  starTargetsText.value = (row.star_targets ?? []).join(', ')
  editOpen.value = true
}

/**
 * 解析星级门槛输入。
 *
 * 三个非负整数（1★/2★/3★ 的分数门槛），逗号或空格分隔。
 * 解析失败**不静默** —— 静默会用空数组提交，服务端整单拒绝，
 * 而运营看到的是一句与输入无关的 400。
 */
function parseStarTargets(text: string): number[] | null {
  const parts = text
    .split(/[,，\s]+/)
    .map((x) => x.trim())
    .filter(Boolean)
  if (parts.length !== 3) return null
  const nums: number[] = []
  for (const p of parts) {
    if (!/^\d+$/.test(p)) return null
    nums.push(Number(p))
  }
  return nums
}

async function submitEdit() {
  if (editSaving.value || !editRow.value) return
  const stars = parseStarTargets(starTargetsText.value)
  if (!stars) {
    ElMessage.error('星级门槛格式应为三个非负整数（如 1000, 5000, 12000）')
    return
  }
  if (editForm.value.base_hp <= 0) {
    ElMessage.error('防线血量必须是正整数')
    return
  }
  if (editForm.value.wave_count < 1 || editForm.value.wave_count > 50) {
    ElMessage.error('波次数必须在 1..50')
    return
  }
  if (editForm.value.difficulty <= 0) {
    ElMessage.error('难度系数必须是正整数')
    return
  }
  if (editForm.value.energy_cost < 0 || editForm.value.energy_cost > 100) {
    ElMessage.error('体力消耗必须在 0..100')
    return
  }
  editSaving.value = true
  try {
    const res = await updateLevel(editRow.value.id, {
      name: editForm.value.name.trim() || editRow.value.name,
      base_hp: editForm.value.base_hp,
      wave_count: editForm.value.wave_count,
      difficulty: editForm.value.difficulty,
      energy_cost: editForm.value.energy_cost,
      star_targets: stars,
      is_boss: editForm.value.is_boss,
    })
    Object.assign(editRow.value, res.level)
    editOpen.value = false
    // 不显示「已保存」——那会让运营以为玩家侧也变了（第 23 条：生成器权威）
    ElMessage.success('已保存到规划表（不影响线上玩家）')
  } catch (e) {
    ElMessage.error((e as Error).message)
  } finally {
    editSaving.value = false
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

async function openWaves(row: AdminLevelView) {
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
 * 它不能直接传给形参是 `AdminLevel` 的函数（TS2345），所以在调用点集中断言一次。
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
const asRow = (r: unknown) => r as AdminLevelView
</script>

<template>
  <div v-loading="loading">
    <div class="page-title">关卡配置</div>
    <p class="page-subtitle">
      100 关由种子化生成器产出，<strong>难度必须全局单调</strong>（1000‰ → 4200‰）。
      若出现断崖，说明生成器的章节边界算错了 —— 这会让玩家在某一关突然卡死。
    </p>
    <p class="page-subtitle">
      <strong>注意：本页的编辑只写 `levels` 表（规划记录），不影响线上玩家。</strong>
      玩家侧关卡由生成器按种子产出（`domain.GenerateLevel`），从不查库 ——
      回执里 `applied_to_players` 恒为 false 就是这个意思。
      要让改动真正生效，需要产品决策改走「表权威」路线（已知边界第 23 条）。
    </p>

    <el-card shadow="never">
      <div class="toolbar">
        <el-select v-model="chapter" placeholder="全部章节" clearable style="width: 140px" @change="load">
          <el-option v-for="c in chapterOptions" :key="c" :label="`第 ${c} 章`" :value="c" />
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
            <el-button size="small" text @click="openWaves(asRow(row))">波次</el-button>
            <el-button v-if="!isReadonly" size="small" text @click="openEdit(asRow(row))">编辑</el-button>
            <el-button v-if="!isReadonly" size="small" text @click="toggleEnabled(asRow(row))">
              {{ row.enabled ? '停用' : '启用' }}
            </el-button>
          </template>
        </el-table-column>
        <template #empty>
          <el-empty :description="emptyText" :image-size="60" />
        </template>
</el-table>
    </el-card>

    <!--
      完整的关卡编辑对话框（2026-10-21 P5-3）。
      服务端白名单有 9 个键，此前界面只有 2 个入口 ——
      另 7 个字段在后台**根本不存在**。
    -->
    <el-dialog v-model="editOpen" title="编辑关卡" width="520px">
      <el-form label-width="110px" size="small">
        <el-form-item label="名称">
          <el-input v-model="editForm.name" placeholder="关卡名" />
        </el-form-item>
        <el-form-item label="防线血量">
          <el-input-number v-model="editForm.base_hp" :min="1" :max="9999999" />
        </el-form-item>
        <el-form-item label="波次数">
          <el-input-number v-model="editForm.wave_count" :min="1" :max="50" />
        </el-form-item>
        <el-form-item label="难度系数">
          <el-input-number v-model="editForm.difficulty" :min="1" :max="9999" />
        </el-form-item>
        <el-form-item label="体力消耗">
          <el-input-number v-model="editForm.energy_cost" :min="0" :max="100" />
        </el-form-item>
        <el-form-item label="星级门槛">
          <el-input v-model="starTargetsText" placeholder="1★, 2★, 3★ 的分数门槛" />
          <div class="muted" style="font-size: 12px; margin-top: 4px">
            三个非负整数，逗号分隔。例如 <code>1000, 5000, 12000</code>
          </div>
        </el-form-item>
        <el-form-item label="BOSS 关">
          <el-switch v-model="editForm.is_boss" />
        </el-form-item>
      </el-form>
      <el-alert
        type="warning"
        :closable="false"
        show-icon
        title="这些修改记录在规划表里，不影响线上玩家"
        description="玩家侧的关卡由生成器产出（第 23 条产品决策）。后台改的值不会立刻生效 —— 那是「已生效」与「已记录」的区别。"
        style="margin-bottom: 8px"
      />
      <template #footer>
        <el-button @click="editOpen = false">取消</el-button>
        <el-button type="primary" :loading="editSaving" @click="submitEdit">保存</el-button>
      </template>
    </el-dialog>

    <el-drawer v-model="waveDrawer" :title="`${waveTarget?.name ?? ''} · 波次编排`" size="46%">
      <el-table :data="waveRows" size="small">
        <el-table-column prop="wave_index" label="波次" width="70" />
        <el-table-column label="刷怪">
          <template #default="{ row }">{{ spawnText(row.spawns) }}</template>
        </el-table-column>
        <template #empty>
          <el-empty :description="emptyText" :image-size="60" />
        </template>
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
