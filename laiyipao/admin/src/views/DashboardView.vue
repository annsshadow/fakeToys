<script setup lang="ts">
import PageSkeleton from "@/components/PageSkeleton.vue"
import {onMounted, ref, computed, onActivated} from 'vue'
import { ElMessage } from 'element-plus'
import * as echarts from 'echarts/core'
import { BarChart, LineChart, RadarChart } from 'echarts/charts'
import {
  GridComponent,
  TooltipComponent,
  LegendComponent,
  RadarComponent,
} from 'echarts/components'
import { CanvasRenderer } from 'echarts/renderers'

import { fetchDashboard, type Dashboard } from '@/api'
import { fetchReactionLabels, labelOfReaction, type ReactionLabels } from '@/reactions'
import { describeDashboardChart, type ChartRow } from '@/utils/chartA11y'
import { buildAiInsight, type AiInsight } from '@/utils/aiInsight'
import AiInsightCard from '@/components/AiInsightCard.vue'

echarts.use([
  BarChart,
  LineChart,
  RadarChart,
  GridComponent,
  TooltipComponent,
  LegendComponent,
  RadarComponent,
  CanvasRenderer,
])

const loading = ref(true)
const data = ref<Dashboard | null>(null)
const loadError = ref('')

const dauRef = ref<HTMLElement | null>(null)
const funnelRef = ref<HTMLElement | null>(null)
const reactionRef = ref<HTMLElement | null>(null)
let dauChart: echarts.ECharts | null = null
let funnelChart: echarts.ECharts | null = null
let reactionChart: echarts.ECharts | null = null

/**
 * 反应 key → 中文名。
 *
 * ⚠️ 这里原本是一张**硬编码**的 `REACTION_LABEL`（7 个反应逐个写死）。
 *
 * 它当时**恰好是全的**（`domain/elements.go` 里正好 7 个 ReactionKey 常量），
 * 所以看不出任何问题 —— 但它是**纯重复**：名字的真源在服务端，
 * 连 `ReactionSpec.Name` 的注释都写着「反应的中文名，后台系统/运营看板要用」。
 *
 * 复制一份的真实代价：有人加第 8 个反应而忘了改这张表时，
 * 症状**不是报错**，而是图表 y 轴上悄悄出现一个英文 key。这种漂移很难被发现。
 *
 * 取名与查名的逻辑已收敛到 `@/reactions`（BattlesView 也有同一张表的另一份，
 * R38 只消除了本页面这一份 —— 教训见 `@/reactions` 的文件头）。
 */
const reactionLabel = ref<ReactionLabels>({})

function labelOf(key: string): string {
  return labelOfReaction(reactionLabel.value, key)
}

const verificationRate = computed(() => {
  const v = data.value?.verification
  if (!v || v.checked === 0) return '—'
  return `${((v.matched / v.checked) * 100).toFixed(1)}%`
})

const verificationTone = computed(() => {
  const v = data.value?.verification
  if (!v || v.checked === 0) return 'info'
  return v.mismatched === 0 ? 'success' : 'danger'
})

/** 第88轮：canvas 图对读屏不可见，抽出等价文本与数据表（WCAG 1.1.1）。 */
const dauRows = computed<ChartRow[]>(() => (data.value?.daily_active ?? []).map((x) => ({ label: x.date, value: x.dau })))
const reactionRows = computed<ChartRow[]>(() =>
  [...(data.value?.reaction_usage ?? [])]
    .sort((a, b) => a.count - b.count)
    .map((x) => ({ label: labelOf(x.reaction), value: x.count })),
)
const funnelRows = computed<ChartRow[]>(() =>
  (data.value?.stage_funnel ?? []).slice(0, 30).map((x) => ({ label: `第${x.level_id}关`, value: x.attempts })),
)
const dauDesc = computed(() => describeDashboardChart({ title: '近 14 日活跃', chartType: 'line', unit: '日', metric: '活跃', rows: dauRows.value }))
const reactionDesc = computed(() => describeDashboardChart({ title: '元素反应使用分布', chartType: 'bar', unit: '种', metric: '触发', rows: reactionRows.value }))
const funnelDesc = computed(() => describeDashboardChart({ title: '关卡漏斗', chartType: 'bar', unit: '关', metric: '尝试', rows: funnelRows.value }))

/** F-10-2：AI 洞察卡结论由当前数据确定性推导（规则引擎，非硬编码占位）。 */
const aiInsight = computed<AiInsight>(() => buildAiInsight(data.value, labelOf))

/**
 * F-10-4：ECharts 配色只从设计令牌读取，不写死十六进制。
 * 令牌真源是 styles/tokens.css 的 :root 变量，运行时用 getComputedStyle 取值；
 * 令牌改色时图表跟着变，不再有第二份会漂移的魔法色。
 */
function chartToken(name: string): string {
  return getComputedStyle(document.documentElement).getPropertyValue(name).trim()
}

function renderCharts() {
  const d = data.value
  if (!d) return
  const axis = { axisLine: { lineStyle: { color: chartToken('--lyp-border') } }, axisLabel: { color: chartToken('--lyp-dim') } }

  if (dauRef.value) {
    dauChart ??= echarts.init(dauRef.value)
    dauChart.setOption({
      grid: { left: 40, right: 16, top: 24, bottom: 28 },
      tooltip: { trigger: 'axis' },
      xAxis: { type: 'category', data: d.daily_active.map((x) => x.date), ...axis },
      yAxis: { type: 'value', ...axis },
      series: [
        {
          type: 'line',
          smooth: true,
          data: d.daily_active.map((x) => x.dau),
          lineStyle: { color: chartToken('--el-fire'), width: 2 },
          areaStyle: { color: chartToken('--el-fire-soft') },
        },
      ],
    })
  }

  if (funnelRef.value) {
    funnelChart ??= echarts.init(funnelRef.value)
    const items = (d.stage_funnel ?? []).slice(0, 30)
    funnelChart.setOption({
      grid: { left: 44, right: 16, top: 24, bottom: 28 },
      tooltip: { trigger: 'axis' },
      legend: { data: ['尝试', '通关'], textStyle: { color: chartToken('--lyp-dim') } },
      xAxis: { type: 'category', data: items.map((x) => `第${x.level_id}关`), ...axis },
      yAxis: { type: 'value', ...axis },
      series: [
        { name: '尝试', type: 'bar', data: items.map((x) => x.attempts), itemStyle: { color: chartToken('--lyp-border') } },
        { name: '通关', type: 'bar', data: items.map((x) => x.clears), itemStyle: { color: chartToken('--el-corrosion') } },
      ],
    })
  }

  if (reactionRef.value) {
    reactionChart ??= echarts.init(reactionRef.value)
    const items = [...d.reaction_usage].sort((a, b) => a.count - b.count)
    reactionChart.setOption({
      grid: { left: 90, right: 24, top: 16, bottom: 24 },
      tooltip: { trigger: 'item' },
      xAxis: { type: 'value', ...axis },
      yAxis: {
        type: 'category',
        data: items.map((x) => labelOf(x.reaction)),
        ...axis,
      },
      series: [
        {
          type: 'bar',
          data: items.map((x) => x.count),
          itemStyle: { color: chartToken('--el-kinetic') },
          label: { show: true, position: 'right', color: chartToken('--lyp-dim') },
        },
      ],
    })
  }
}

// keep-alive onActivated 重拉与在途 load 可重叠（侧栏在 v-loading 遮罩外）：
// 迟到的旧快照不得覆盖新 data / 触发过期 renderCharts，与 EconomyView 同款序号守卫。
let loadSeq = 0
async function load() {
  const seq = ++loadSeq
  loading.value = true
  loadError.value = ''
  try {
    // 反应名与看板数据并发取：两者都只读、互不依赖，并发省一次往返。
    //
    // 用 Promise.all（一个失败就整体失败）是**合适的**，因为
    // `GET /admin/reactions` 是纯内存返回（`domain.AllReactionSpecs()`，
    // 不查库、不可能失败）。所以「反应名拿不到导致整页报错」这个场景不存在。
    //
    // 兜底 `labelOfReaction` 里的 `|| key` 覆盖的是另一种情况：
    // 后台连的是**旧版服务端**、响应里没有某个新反应 —— 那时只影响坐标轴文案。
    const [dash, labels] = await Promise.all([fetchDashboard(), fetchReactionLabels()])
    if (seq !== loadSeq) return
    data.value = dash
    reactionLabel.value = labels
    await new Promise((r) => setTimeout(r, 30))
    if (seq !== loadSeq) return
    renderCharts()
  } catch (e) {
    if (seq !== loadSeq) return
    loadError.value = (e as Error).message
    ElMessage.error(`看板数据加载失败：${(e as Error).message}`)
  } finally {
    if (seq === loadSeq) loading.value = false
  }
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
</script>

<template>
  <div class="page">
    <PageSkeleton v-if="loading" />
    <div class="page-header">
      <div class="page-heading">
        <div class="page-title">数据看板</div>
      <p class="page-subtitle">
        重点关注两件事：<strong>反应使用分布</strong>（验证 I-1 元素机制是否真的被使用，而不是玩家全在堆数值）
        与<strong>验真一致率</strong>（I-6 可复现分数的完整性）
      </p>
      </div>
    </div>

    <el-alert
      v-if="loadError"
      type="error"
      :closable="false"
      show-icon
      :title="`后端不可达：${loadError}`"
      description="请先启动 server（go run ./cmd/api）并确认已执行数据库迁移。" class="mb-3"
    />

    <AiInsightCard :insight="aiInsight" :loading="loading" />

    <template v-if="data">
      <section class="stat-section">
        <div class="section-kicker"><span class="section-kicker-bar" />核心指标<em>CORE METRICS</em></div>
        <div class="kpi-primary">
          <el-card shadow="never" class="kpi-card kpi-card--hero">
            <div class="kpi-label">注册用户</div>
            <div class="kpi-value">{{ data.users.total.toLocaleString() }}</div>
            <div class="kpi-sub">游客 {{ data.users.guests }} · 微信 {{ data.users.wechat }} · 今日新增 {{ data.users.new_today }}</div>
          </el-card>
          <el-card shadow="never" class="kpi-card kpi-card--hero">
            <div class="kpi-label">战斗总数</div>
            <div class="kpi-value">{{ data.battles.total.toLocaleString() }}</div>
            <div class="kpi-sub">今日 {{ data.battles.today }} · 胜率 {{ ((data.battles.wins / Math.max(data.battles.total, 1)) * 100).toFixed(1) }}%</div>
          </el-card>
          <el-card shadow="never" class="kpi-card kpi-card--hero">
            <div class="kpi-label">平均战力 / 关卡</div>
            <div class="kpi-value">{{ data.progression.avg_power.toLocaleString() }}</div>
            <div class="kpi-sub">平均最高关 {{ data.progression.avg_max_stage.toFixed(1) }}</div>
          </el-card>
          <el-card shadow="never" class="kpi-card kpi-card--hero">
            <div class="kpi-label">验真一致率</div>
            <div class="kpi-value">
              <el-tag :type="verificationTone as any" size="large">{{ verificationRate }}</el-tag>
            </div>
            <div class="kpi-sub">已验 {{ data.verification.checked }} · 不一致 {{ data.verification.mismatched }}</div>
          </el-card>
        </div>
      </section>

      <section class="stat-section">
        <div class="section-kicker"><span class="section-kicker-bar" />经济与防线<em>ECONOMY &amp; DEFENSE</em></div>
        <div class="kpi-secondary">
          <el-card shadow="never" class="kpi-card kpi-card--sub">
            <div class="kpi-label">金币流入 / 流出</div>
            <div class="kpi-value">{{ data.economy.coin_in.toLocaleString() }}</div>
            <div class="kpi-sub">流出 {{ data.economy.coin_out.toLocaleString() }}</div>
          </el-card>
          <el-card shadow="never" class="kpi-card kpi-card--sub">
            <div class="kpi-label">钻石流入 / 流出</div>
            <div class="kpi-value">{{ data.economy.gem_in.toLocaleString() }}</div>
            <div class="kpi-sub">流出 {{ data.economy.gem_out.toLocaleString() }}</div>
          </el-card>
          <el-card shadow="never" class="kpi-card kpi-card--sub">
            <div class="kpi-label">订单</div>
            <div class="kpi-value">{{ data.economy.orders_paid }}</div>
            <div class="kpi-sub">待支付 {{ data.economy.orders_pending }}</div>
            <div class="kpi-sub">今日签到 {{ data.economy.sign_ins_today }} · 今日兑换 {{ data.economy.redeems_today }}</div>
          </el-card>
          <el-card shadow="never" class="kpi-card kpi-card--sub">
            <div class="kpi-label">封禁用户</div>
            <div class="kpi-value">{{ data.users.banned }}</div>
            <div class="kpi-sub">100 关通关人次 {{ data.progression.stage_100_clears }}</div>
            <div class="kpi-sub">防线 {{ data.defenses.total }} · 今日挑战 {{ data.defenses.challenges_today }}</div>
          </el-card>
        </div>
      </section>

      <section class="stat-section charts-section">
        <div class="section-kicker"><span class="section-kicker-bar" />趋势与分布<em>ANALYTICS</em></div>
        <div class="charts-grid">
          <el-card shadow="never" class="chart-card chart-primary" header="近 14 日活跃">
            <div ref="dauRef" class="chart-canvas" role="img" :aria-label="dauDesc.label" style="height: 260px" />
            <table class="chart-data-sr">
              <caption>近 14 日活跃数据</caption>
              <thead><tr><th scope="col">日期</th><th scope="col">活跃用户数</th></tr></thead>
              <tbody>
                <tr v-for="row in dauDesc.rows" :key="row.label">
                  <th scope="row">{{ row.label }}</th><td>{{ row.value }}</td>
                </tr>
              </tbody>
            </table>
          </el-card>
          <el-card shadow="never" class="chart-card" header="元素反应使用分布（I-1 健康度）">
            <div ref="reactionRef" class="chart-canvas" role="img" :aria-label="reactionDesc.label" style="height: 240px" />
            <table class="chart-data-sr">
              <caption>元素反应使用分布数据</caption>
              <thead><tr><th scope="col">反应</th><th scope="col">触发次数</th></tr></thead>
              <tbody>
                <tr v-for="row in reactionDesc.rows" :key="row.label">
                  <th scope="row">{{ row.label }}</th><td>{{ row.value }}</td>
                </tr>
              </tbody>
            </table>
            <el-alert
              v-if="data.reaction_usage.length === 0"
              type="info"
              :closable="false"
              title="尚无反应触发记录"
              description="若上线后该分布长期集中在一两种反应，说明 5 元素设计没有真正生效，需要调整抗性矩阵。"
              style="margin-top: 8px"
            />
          </el-card>
          <el-card shadow="never" class="chart-card" header="关卡漏斗（前 30 关）">
            <div ref="funnelRef" class="chart-canvas" role="img" :aria-label="funnelDesc.label" style="height: 240px" />
            <table class="chart-data-sr">
              <caption>关卡漏斗数据（尝试次数）</caption>
              <thead><tr><th scope="col">关卡</th><th scope="col">尝试次数</th></tr></thead>
              <tbody>
                <tr v-for="row in funnelDesc.rows" :key="row.label">
                  <th scope="row">{{ row.label }}</th><td>{{ row.value }}</td>
                </tr>
              </tbody>
            </table>
          </el-card>
        </div>
      </section>
    </template>
  </div>
</template>

<style scoped>
/* F-10-3 标杆重构：CSS Grid 替代等权 el-row，建立「核心 → 次级 → 图表」三级信息层级 */
.stat-section { margin-bottom: 18px; }
.section-kicker {
  display: flex; align-items: center; gap: 8px;
  font-size: 11px; font-weight: 600; letter-spacing: .12em;
  color: var(--lyp-text-2); text-transform: uppercase;
  margin-bottom: 10px;
}
.section-kicker-bar {
  width: 3px; height: 12px; border-radius: 2px;
  background: var(--lyp-accent);
}
.section-kicker em {
  font-style: normal; font-size: 10px; font-weight: 400;
  letter-spacing: .08em; color: var(--lyp-dim);
}
.kpi-primary, .kpi-secondary {
  display: grid; gap: 12px;
  grid-template-columns: repeat(4, minmax(0, 1fr));
}
.kpi-secondary { grid-template-columns: repeat(4, minmax(0, 1fr)); }
.kpi-card { border: 1px solid var(--lyp-border); }
/* 核心指标：更强的数值与左侧强调条 */
.kpi-card--hero { position: relative; background: var(--lyp-panel-2); }
.kpi-card--hero::before {
  content: ''; position: absolute; left: 0; top: 16px; bottom: 16px;
  width: 2px; border-radius: 2px; background: var(--lyp-accent); opacity: .65;
}
.kpi-card--hero :deep(.kpi-value) { font-size: 30px; font-weight: 650; letter-spacing: -.02em; }
/* 次级指标：收敛字号，弱化视觉权重 */
.kpi-card--sub { background: var(--lyp-panel); }
.kpi-card--sub :deep(.kpi-value) { font-size: 20px; font-weight: 600; color: var(--lyp-text-2); }
/* 图表主次：主图满宽，反应/漏斗并排 */
.charts-grid {
  display: grid; gap: 12px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}
.chart-card { border: 1px solid var(--lyp-border); }
.chart-primary { grid-column: 1 / -1; }
.chart-primary :deep(.el-card__header) { border-bottom-color: var(--lyp-border); }
@media (max-width: 900px) {
  .kpi-primary, .kpi-secondary, .charts-grid { grid-template-columns: 1fr; }
}
.chart-data-sr {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}
</style>

