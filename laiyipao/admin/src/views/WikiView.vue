<script setup lang="ts">
/**
 * 玩家玩法文档站。
 *
 * 为什么要放在运营后台里：它读的是同一份 /api/v1/config，
 * 因此技能数值、抗性、反应链说明永远与线上版本一致 ——
 * 不会出现"文档写的和游戏里不一样"这种最伤玩家信任的问题。
 */
import { onMounted, ref, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { fetchDashboard } from '@/api'

interface ConfigResp {
  levels: Array<{ id: number; chapter: number; name: string; is_boss: boolean; terrain: unknown[]; difficulty: number; base_hp: number; wave_count: number; energy_cost: number; element_cap: number; max_reaction_tier: number; star_targets: number[] }>
  enemies: Array<{ id: number; code: string; name: string; category: string; hp: number; resist: Record<string, number>; descr: string }>
  skills: Array<{ id: number; name: string; element: string; family: string; kind: string; base_damage: number; heat_cost: number; cooldown_ms: number; pierce: number; aoe_radius: number; apply_stacks: number; descr: string }>
  composite_skills: Array<{ id: number; name: string; element: string; descr: string }>
  // ⚠️ 字段名必须与 Go 侧 json tag 逐字一致（snake_case）。
  // 这两行曾经写成 PascalCase，而服务端一直下发 snake_case，
  // 于是反应表整列为空、章节下拉框没有选项 —— 且不报任何错。
  reactions: Array<{ key: string; name: string; base_coef: number; attack_weight_pct: number; status_duration_ms: number; aoe_radius: number; dispel_shield: boolean; amplify_pct: number }>
  chapters: Array<{ id: number; name: string; start_level: number; end_level: number; terrain_kind: string; boss_enemy_id: number }>
  rating_weights: Record<string, number>
  mastery_families: Array<{ family: string; name: string; nodes: Array<{ name: string; layer: number }> }>
}

const loading = ref(false)
const cfg = ref<ConfigResp | null>(null)
const reactUsage = ref<Array<{ reaction: string; count: number }>>([])
const activeTab = ref('basics')

const ELEMENT_LABEL: Record<string, string> = {
  fire: '焰',
  ice: '冰',
  lightning: '电',
  corrosion: '毒',
  kinetic: '动能',
}
const ELEMENT_COLOR: Record<string, string> = {
  fire: '#ff6b35',
  ice: '#58a6ff',
  lightning: '#ffd33d',
  corrosion: '#7ee787',
  kinetic: '#c9a7ff',
}
const CATEGORY_LABEL: Record<string, string> = {
  normal: '普通',
  ranged: '远程',
  flying: '飞行',
  special: '特殊',
  elite: '精英',
  boss: 'BOSS',
}

/** 抗性条：正数为抗（减伤），负数为弱（增伤），范围 ±50% */
function resistPct(v: number): number {
  return Math.max(-50, Math.min(50, v / 10))
}

function resistColor(v: number): string {
  if (v >= 0) return '#58a6ff'
  return '#7ee787'
}

const activeChapter = ref(1)
const chapterLevels = computed(() =>
  (cfg.value?.levels ?? []).filter((l) => l.chapter === activeChapter.value),
)

/** 玩家实际最常用哪条反应 —— 文档站给出"主流打法"提示 */
const topReaction = computed(() => {
  const sorted = [...reactUsage.value].sort((a, b) => b.count - a.count)
  return sorted[0]
})

async function load() {
  loading.value = true
  try {
    // config 端点无需管理员令牌，因此直接走底层 api 客户端而不经 admin 封装
    const { api } = await import('@/api/client')
    const [data, dash] = await Promise.all([
      api.get<ConfigResp>('/config', { auth: false }),
      fetchDashboard().catch(() => null),
    ])
    cfg.value = data
    reactUsage.value = dash?.reaction_usage ?? []
  } catch (e) {
    ElMessage.error(`配置加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

onMounted(load)
</script>

<template>
  <div v-loading="loading" class="wiki">
    <div class="page-title">玩家玩法文档站</div>
    <p class="page-subtitle">
      本页数据直接来自线上 <code>/api/v1/config</code>，与游戏内实际数值同源 ——
      改配置即改文档，不存在「文档说的和游戏里不一样」。
    </p>

    <el-tabs v-model="activeTab" type="border-card">
      <!-- 基础规则 -->
      <el-tab-pane label="核心规则" name="basics">
        <el-alert
          type="warning"
          show-icon
          :closable="false"
          title="为什么「战力」不是最重要的数字"
          description="本作的伤害分三段：直接伤害吃养成，元素层数伤害与反应伤害只吃元素投入，且反应伤害中攻击力的权重被结构性限制在 30% 以内。堆面板能变强，但打不过抗性矩阵 —— 搭配才是主要胜负手。"
          style="margin-bottom: 14px"
        />

        <h4>5 元素</h4>
        <p>
          敌人对每种元素有独立抗性（±50%）。两种元素叠加会触发<strong>反应链</strong>，共 7 条。
          动能不能作为「已附着元素」触发反应，它只能后手打出破甲。
        </p>

        <h4>反应链一览</h4>
        <el-table :data="cfg?.reactions ?? []" size="small" border>
          <el-table-column prop="name" label="反应" width="120" />
          <el-table-column label="系数" width="90">
            <template #default="{ row }">{{ row.base_coef }}‰</template>
          </el-table-column>
          <el-table-column label="攻击力权重" width="130">
            <template #default="{ row }">
              <el-tag :type="row.attack_weight_pct >= 300 ? 'warning' : 'info'" size="small">
                {{ row.attack_weight_pct }}‰
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column label="溅射半径" width="100">
            <template #default="{ row }">{{ row.aoe_radius || '单体' }}</template>
          </el-table-column>
        </el-table>
        <p class="note">
          「攻击力权重」就是这一条反应里由面板攻击力贡献的比例上限。全部 ≤30% ——
          这是保证低养成玩家能靠技巧翻盘的结构性约束，不是可以随数值膨胀绕过的软性限制。
        </p>

        <h4>构筑评分（I-7）</h4>
        <p>主指标不是总战力，而是这五个维度：</p>
        <el-table :data="Object.entries(cfg?.rating_weights ?? {}).map(([k, v]) => ({ k, v }))" size="small" border>
          <el-table-column prop="k" label="维度" />
          <el-table-column prop="v" label="权重" />
        </el-table>
      </el-tab-pane>

      <!-- 元素矩阵 -->
      <el-tab-pane label="敌人抗性图鉴" name="enemies">
        <p class="note">
          绿色为<strong>弱点</strong>（负抗性，伤害放大），蓝色为<strong>抗性</strong>（减伤）。
          全部 BOSS 都被设计成「必有解」：任意一只都至少有两到三种元素是弱点。
        </p>
        <el-table :data="cfg?.enemies ?? []" size="small" border stripe>
          <el-table-column prop="id" label="ID" width="55" />
          <el-table-column prop="name" label="名称" min-width="120" />
          <el-table-column label="类型" width="80">
            <template #default="{ row }">
              <el-tag :type="row.category === 'boss' ? 'danger' : 'info'" size="small">
                {{ CATEGORY_LABEL[row.category] ?? row.category }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="hp" label="血量" width="90" />
          <el-table-column label="五维抗性" min-width="260">
            <template #default="{ row }">
              <div v-for="e in ['fire', 'ice', 'lightning', 'corrosion', 'kinetic']" :key="e" class="resist-row">
                <span class="resist-name" :style="{ color: ELEMENT_COLOR[e] }">{{ ELEMENT_LABEL[e] }}</span>
                <div class="resist-bar">
                  <div class="resist-zero" />
                  <div
                    class="resist-fill"
                    :style="{
                      width: Math.abs(resistPct(row.resist?.[e] ?? 0)) * 2 + '%',
                      background: resistColor(row.resist?.[e] ?? 0),
                      left: (row.resist?.[e] ?? 0) >= 0 ? '50%' : `${50 - Math.abs(resistPct(row.resist?.[e] ?? 0)) * 2}%`,
                    }"
                  />
                </div>
                <span class="resist-num">{{ (row.resist?.[e] ?? 0) / 10 }}%</span>
              </div>
            </template>
          </el-table-column>
          <el-table-column prop="descr" label="说明" min-width="150" />
        </el-table>
      </el-tab-pane>

      <!-- 技能 -->
      <el-tab-pane label="技能图鉴" name="skills">
        <el-table :data="cfg?.skills ?? []" size="small" border stripe>
          <el-table-column prop="id" label="ID" width="55" />
          <el-table-column prop="name" label="名称" min-width="120" />
          <el-table-column label="元素" width="80">
            <template #default="{ row }">
              <span :style="{ color: ELEMENT_COLOR[row.element], fontWeight: 600 }">
                {{ ELEMENT_LABEL[row.element] ?? row.element }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="family" label="系别" width="90" />
          <el-table-column prop="base_damage" label="伤害" width="76" />
          <el-table-column prop="heat_cost" label="热量" width="70" />
          <el-table-column prop="cooldown_ms" label="冷却" width="86" />
          <el-table-column prop="apply_stacks" label="层数" width="66" />
          <el-table-column prop="descr" label="说明" min-width="180" />
        </el-table>

        <h4 style="margin-top: 16px">合成技能</h4>
        <el-table :data="cfg?.composite_skills ?? []" size="small" border stripe>
          <el-table-column prop="id" label="ID" width="55" />
          <el-table-column prop="name" label="名称" min-width="140" />
          <el-table-column label="元素" width="80">
            <template #default="{ row }">
              <span :style="{ color: ELEMENT_COLOR[row.element] }">{{ ELEMENT_LABEL[row.element] }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="descr" label="说明" min-width="220" />
        </el-table>
      </el-tab-pane>

      <!-- 专精 -->
      <el-tab-pane label="专精树" name="mastery">
        <p class="note">
          每层只能选 2 个节点（4 选 2），且第 2、3 层必须先点亮前一层。
          专精点不够点满任何一系 —— <strong>必须做方向选择</strong>。
        </p>
        <div v-for="f in cfg?.mastery_families ?? []" :key="f.family" class="family-block">
          <div class="family-name">{{ f.name }}</div>
          <div v-for="layer in 3" :key="layer" class="layer-row">
            <span class="layer-label">第 {{ layer }} 层</span>
            <span v-for="n in f.nodes.filter((x) => x.layer === layer)" :key="n.name" class="node">
              {{ n.name }}
            </span>
          </div>
        </div>
      </el-tab-pane>

      <!-- 关卡 -->
      <el-tab-pane label="关卡一览" name="levels">
        <el-select v-model="activeChapter" style="width: 160px; margin-bottom: 12px">
          <el-option v-for="c in cfg?.chapters ?? []" :key="c.id" :label="c.name" :value="c.id" />
        </el-select>
        <el-table :data="chapterLevels" size="small" border stripe>
          <el-table-column prop="id" label="关" width="60" />
          <el-table-column prop="name" label="名称" min-width="140" />
          <el-table-column prop="difficulty" label="难度" width="80" />
          <el-table-column prop="base_hp" label="防线血量" width="100" />
          <el-table-column prop="wave_count" label="波次" width="70" />
          <el-table-column prop="energy_cost" label="体力" width="70" />
          <el-table-column prop="element_cap" label="层数上限" width="94" />
          <el-table-column prop="max_reaction_tier" label="反应阶" width="86" />
          <el-table-column label="星级门槛" min-width="160">
            <template #default="{ row }">{{ row.star_targets.join(' / ') }}</template>
          </el-table-column>
        </el-table>
      </el-tab-pane>

      <!-- 实战统计 -->
      <el-tab-pane label="当前版本实战分布" name="live">
        <p class="note">
          下面是<strong>真实玩家数据</strong>（不是设计意图）。如果某条反应长期没人用，
          说明它的触发条件太苛刻，需要调整 —— 而不是怪玩家不会玩。
        </p>
        <el-table :data="[...reactUsage].sort((a, b) => b.count - a.count)" size="small" border>
          <el-table-column prop="reaction" label="反应" width="180" />
          <el-table-column prop="count" label="触发次数" />
          <el-table-column label="占比">
            <template #default="{ row }">
              <el-progress
                :percentage="
                  Math.round(
                    (row.count /
                      Math.max(1, reactUsage.reduce((s, x) => s + x.count, 0))) *
                      100,
                  )
                "
              />
            </template>
          </el-table-column>
        </el-table>
        <p v-if="topReaction" class="note">
          当前主流打法是「{{ topReaction.reaction }}」。若你卡在某一关，
          试试换一个<strong>还没被多数人用到</strong>的搭配。
        </p>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style scoped>
.wiki {
  max-width: 1180px;
}
.note {
  font-size: 12px;
  color: var(--lyp-muted);
  margin: 8px 0 0;
}
code {
  font-family: ui-monospace, monospace;
  font-size: 11px;
  background: #1c2430;
  padding: 1px 4px;
  border-radius: 3px;
}
h4 {
  margin: 18px 0 8px;
}
.resist-row {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 1px 0;
}
.resist-name {
  width: 34px;
  font-size: 11px;
  flex-shrink: 0;
}
.resist-bar {
  position: relative;
  flex: 1;
  height: 9px;
  background: #1c2430;
  border-radius: 2px;
  overflow: hidden;
}
.resist-zero {
  position: absolute;
  left: 50%;
  top: 0;
  bottom: 0;
  width: 1px;
  background: #30363d;
}
.resist-fill {
  position: absolute;
  top: 0;
  bottom: 0;
}
.resist-num {
  width: 44px;
  text-align: right;
  font-size: 11px;
  color: var(--lyp-muted);
  font-variant-numeric: tabular-nums;
}
.family-block {
  border: 1px solid var(--lyp-border);
  border-radius: 6px;
  padding: 10px 14px;
  margin-bottom: 10px;
}
.family-name {
  font-weight: 600;
  margin-bottom: 6px;
}
.layer-row {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 3px 0;
}
.layer-label {
  width: 54px;
  font-size: 11px;
  color: var(--lyp-muted);
  flex-shrink: 0;
}
.node {
  font-size: 11px;
  background: #1c2430;
  border-radius: 3px;
  padding: 2px 7px;
}
</style>
