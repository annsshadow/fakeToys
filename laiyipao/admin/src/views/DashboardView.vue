<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
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

import { fetchDashboard, fetchReactions, type DashboardResp } from '@/api'

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
const data = ref<DashboardResp | null>(null)
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
 * 它当前**恰好是全的**（`domain/elements.go` 里正好 7 个 ReactionKey 常量），
 * 所以看不出任何问题 —— 但它是**纯重复**：名字的真源在服务端，
 * 连 `ReactionSpec.Name` 的注释都写着「反应的中文名，后台系统/运营看板要用」。
 *
 * 复制一份的真实代价：有人加第 8 个反应而忘了改这张表时，
 * 症状**不是报错**，而是图表 y 轴上悄悄出现一个英文 key
 * （靠 `LABEL[x] || x` 的兜底）。这种漂移很难被发现。
 *
 * 所以改成从 `GET /admin/reactions` 取 —— 服务端就是 `domain.AllReactionSpecs()`，
 * 不多不少。加反应时后台自动跟上，不需要改两处。
 *
 * 兜底仍保留 `x`（原始 key）：万一后台连的是旧版服务端、缺少新反应，
 * 那时至少还能显示 key，而不是显示 `undefined`。
 */
const reactionLabel = ref<Record<string, string>>({})

function labelOf(key: string): string {
  return reactionLabel.value[key] || key
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

function renderCharts() {
  const d = data.value
  if (!d) return
  const axis = { axisLine: { lineStyle: { color: '#30363d' } }, axisLabel: { color: '#8b949e' } }

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
          lineStyle: { color: '#ff6b35', width: 2 },
          areaStyle: { color: 'rgba(255,107,53,0.12)' },
        },
      ],
    })
  }

  if (funnelRef.value) {
    funnelChart ??= echarts.init(funnelRef.value)
    const items = d.stage_funnel.slice(0, 30)
    funnelChart.setOption({
      grid: { left: 44, right: 16, top: 24, bottom: 28 },
      tooltip: { trigger: 'axis' },
      legend: { data: ['尝试', '通关'], textStyle: { color: '#8b949e' } },
      xAxis: { type: 'category', data: items.map((x) => `第${x.level_id}关`), ...axis },
      yAxis: { type: 'value', ...axis },
      series: [
        { name: '尝试', type: 'bar', data: items.map((x) => x.attempts), itemStyle: { color: '#30363d' } },
        { name: '通关', type: 'bar', data: items.map((x) => x.clears), itemStyle: { color: '#7ee787' } },
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
          itemStyle: { color: '#c9a7ff' },
          label: { show: true, position: 'right', color: '#8b949e' },
        },
      ],
    })
  }
}

async function load() {
  loading.value = true
  loadError.value = ''
  try {
    // 反应名与看板数据并发取：两者都只读、互不依赖，并发省一次往返。
    //
    // 用 Promise.all（一个失败就整体失败）是**合适的**，因为
    // `GET /admin/reactions` 是纯内存返回（`domain.AllReactionSpecs()`，
    // 不查库、不可能失败）。所以「反应名拿不到导致整页报错」这个场景不存在。
    //
    // 兜底 `labelOf` 里的 `|| key` 覆盖的是另一种情况：
    // 后台连的是**旧版服务端**、响应里没有某个新反应 —— 那时只影响坐标轴文案。
    const [dash, specs] = await Promise.all([fetchDashboard(), fetchReactions()])
    data.value = dash
    reactionLabel.value = Object.fromEntries(specs.map((r) => [r.key, r.name]))
    await new Promise((r) => setTimeout(r, 30))
    renderCharts()
  } catch (e) {
    loadError.value = (e as Error).message
    ElMessage.error(`看板数据加载失败：${(e as Error).message}`)
  } finally {
    loading.value = false
  }
}

onMounted(load)
</script>

<template>
  <div v-loading="loading">
    <div class="page-title">数据看板</div>
    <p class="page-subtitle">
      重点关注两件事：<strong>反应使用分布</strong>（验证 I-1 元素机制是否真的被使用，而不是玩家全在堆数值）
      与<strong>验真一致率</strong>（I-6 可复现分数的完整性）
    </p>

    <el-alert
      v-if="loadError"
      type="error"
      :closable="false"
      show-icon
      :title="`后端不可达：${loadError}`"
      description="请先启动 server（go run ./cmd/api）并确认已执行数据库迁移。"
      style="margin-bottom: 16px"
    />

    <template v-if="data">
      <el-row :gutter="12" style="margin-bottom: 12px">
        <el-col :span="6">
          <el-card shadow="never">
            <div class="kpi-label">注册用户</div>
            <div class="kpi-value">{{ data.users.total.toLocaleString() }}</div>
            <div class="kpi-sub">
              游客 {{ data.users.guests }} · 微信 {{ data.users.wechat }} · 今日新增
              {{ data.users.new_today }}
            </div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card shadow="never">
            <div class="kpi-label">战斗总数</div>
            <div class="kpi-value">{{ data.battles.total.toLocaleString() }}</div>
            <div class="kpi-sub">
              今日 {{ data.battles.today }} · 胜率
              {{ ((data.battles.wins / Math.max(data.battles.total, 1)) * 100).toFixed(1) }}%
            </div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card shadow="never">
            <div class="kpi-label">平均战力 / 关卡</div>
            <div class="kpi-value">{{ data.progression.avg_power.toLocaleString() }}</div>
            <div class="kpi-sub">平均最高关 {{ data.progression.avg_max_stage.toFixed(1) }}</div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card shadow="never">
            <div class="kpi-label">验真一致率</div>
            <div class="kpi-value">
              <el-tag :type="verificationTone as any" size="large">{{ verificationRate }}</el-tag>
            </div>
            <div class="kpi-sub">
              已验 {{ data.verification.checked }} · 不一致 {{ data.verification.mismatched }}
            </div>
          </el-card>
        </el-col>
      </el-row>

      <el-row :gutter="12" style="margin-bottom: 12px">
        <el-col :span="6">
          <el-card shadow="never">
            <div class="kpi-label">金币流入 / 流出</div>
            <div class="kpi-value">{{ data.economy.coin_in.toLocaleString() }}</div>
            <div class="kpi-sub">流出 {{ data.economy.coin_out.toLocaleString() }}</div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card shadow="never">
            <div class="kpi-label">钻石流入 / 流出</div>
            <div class="kpi-value">{{ data.economy.gem_in.toLocaleString() }}</div>
            <div class="kpi-sub">流出 {{ data.economy.gem_out.toLocaleString() }}</div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card shadow="never">
            <div class="kpi-label">订单</div>
            <div class="kpi-value">{{ data.economy.orders_paid }}</div>
            <div class="kpi-sub">待支付 {{ data.economy.orders_pending }}</div>
          </el-card>
        </el-col>
        <el-col :span="6">
          <el-card shadow="never">
            <div class="kpi-label">封禁用户</div>
            <div class="kpi-value">{{ data.users.banned }}</div>
            <div class="kpi-sub">100 关通关人次 {{ data.progression.stage_100_clears }}</div>
          </el-card>
        </el-col>
      </el-row>

      <el-row :gutter="12">
        <el-col :span="12">
          <el-card shadow="never" header="近 14 日活跃">
            <div ref="dauRef" style="height: 240px" />
          </el-card>
        </el-col>
        <el-col :span="12">
          <el-card shadow="never" header="元素反应使用分布（I-1 健康度）">
            <div ref="reactionRef" style="height: 240px" />
            <el-alert
              v-if="data.reaction_usage.length === 0"
              type="info"
              :closable="false"
              title="尚无反应触发记录"
              description="若上线后该分布长期集中在一两种反应，说明 5 元素设计没有真正生效，需要调整抗性矩阵。"
              style="margin-top: 8px"
            />
          </el-card>
        </el-col>
      </el-row>

      <el-card shadow="never" header="关卡漏斗（前 30 关）" style="margin-top: 12px">
        <div ref="funnelRef" style="height: 260px" />
      </el-card>
    </template>
  </div>
</template>

<style scoped>
.kpi-label {
  font-size: 12px;
  color: var(--lyp-muted);
}
.kpi-value {
  font-size: 26px;
  font-weight: 600;
  margin: 6px 0 4px;
  font-variant-numeric: tabular-nums;
}
.kpi-sub {
  font-size: 12px;
  color: #6e7681;
}
</style>
