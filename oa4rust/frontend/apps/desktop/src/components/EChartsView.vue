<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="chart-wrap">
    <div class="chart-container" ref="chartRef" role="img" :aria-label="chartDescription"></div>
    <table class="chart-data-sr" data-oa-chart-data>
      <caption>{{ chartDescription }}</caption>
      <thead>
        <tr>
          <th v-for="col in columns" :key="col" scope="col">{{ col }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(row, ri) in data" :key="ri">
          <template v-for="(col, ci) in columns" :key="col">
            <th v-if="ci === 0" scope="row">{{ row[col] }}</th>
            <td v-else>{{ row[col] }}</td>
          </template>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { BarChart, LineChart, PieChart } from 'echarts/charts'
import { GridComponent, TooltipComponent } from 'echarts/components'
import * as echarts from 'echarts/core'
import { CanvasRenderer } from 'echarts/renderers'
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { describeChart } from '../utils/chartA11y'

echarts.use([BarChart, LineChart, PieChart, GridComponent, TooltipComponent, CanvasRenderer])

/** 图表调色板从主题令牌读取，亮/暗主题切换后自动跟随；令牌缺失时才回退。 */
const CHART_PALETTE = [
  '--color-primary',
  '--color-warning',
  '--color-success',
  '--color-error',
  '--color-accent',
  '--color-info',
]
function readPalette(): string[] {
  const styles = getComputedStyle(document.documentElement)
  return CHART_PALETTE.map((token) => styles.getPropertyValue(token).trim() || '#64748b')
}

const props = defineProps<{
  data: any[]
  columns: string[]
  chartType: 'bar' | 'pie' | 'line' | 'table'
  dimension?: string // Column to use as X/dimension
  metric?: string // Column to aggregate
}>()

const chartRef = ref<HTMLElement | null>(null)
let chart: ReturnType<typeof echarts.init> | null = null
let resizeObserver: ResizeObserver | null = null

const chartTypeMap = computed(() => props.chartType)

/** Canvas 图表的文字替代（WCAG 1.1.1）：短名称 + 等价数据表。 */
const chartDescription = computed(() =>
  describeChart(props.data, props.columns, props.chartType, props.dimension, props.metric),
)

watch(
  [() => props.data, () => props.chartType, () => props.dimension, () => props.metric],
  () => {
    if (!chartRef.value || !props.data.length) return
    if (!chart) chart = echarts.init(chartRef.value)
    const dims = props.dimension ? [props.dimension] : props.columns.slice(0, 2)
    const metrics = props.metric ? [props.metric] : props.columns.filter((c) => c !== dims[0]).slice(0, 3)

    let option: any = {}
    const type = props.chartType

    if (type === 'bar') {
      option = {
        backgroundColor: 'transparent',
        tooltip: { trigger: 'axis' },
        xAxis: {
          type: 'category',
          data: props.data.map((r: any) => r[dims[0]] || r['']),
          axisLabel: { color: 'var(--text-muted)', fontSize: 10 },
        },
        yAxis: { type: 'value', axisLabel: { color: 'var(--text-muted)', fontSize: 10 } },
        series: metrics.map((m: string, i: number) => ({
          name: m,
          type: 'bar',
          data: props.data.map((r: any) => Number(r[m]) || 0),
          itemStyle: { color: readPalette()[i % 4] },
        })),
      }
    } else if (type === 'pie') {
      option = {
        backgroundColor: 'transparent',
        tooltip: { trigger: 'item' },
        series: [
          {
            type: 'pie',
            radius: ['40%', '70%'],
            data: props.data.map((r: any) => ({ name: r[dims[0]] || '', value: Number(r[metrics[0]]) || 1 })),
            label: { color: 'var(--text-muted)', fontSize: 10 },
            itemStyle: {
              color: (params: any) => readPalette()[params.dataIndex % 6],
            },
          },
        ],
      }
    } else if (type === 'line') {
      option = {
        backgroundColor: 'transparent',
        tooltip: { trigger: 'axis' },
        xAxis: {
          type: 'category',
          data: props.data.map((r: any) => r[dims[0]] || ''),
          axisLabel: { color: 'var(--text-muted)', fontSize: 10 },
        },
        yAxis: { type: 'value', axisLabel: { color: 'var(--text-muted)', fontSize: 10 } },
        series: metrics.map((m: string, i: number) => ({
          name: m,
          type: 'line',
          data: props.data.map((r: any) => Number(r[m]) || 0),
          smooth: true,
          itemStyle: { color: readPalette()[i % 3] },
          areaStyle: { opacity: 0.1 },
        })),
      }
    }

    chart.setOption(option, true)
  },
  { immediate: true, deep: true },
)

onMounted(() => {
  if (!chartRef.value || typeof ResizeObserver === 'undefined') return
  resizeObserver = new ResizeObserver(() => chart?.resize())
  resizeObserver.observe(chartRef.value)
})

onBeforeUnmount(() => {
  resizeObserver?.disconnect()
  resizeObserver = null
  chart?.dispose()
  chart = null
})
</script>

<style scoped>
.chart-wrap { width: 100%; }
.chart-container { width: 100%; height: 300px; }
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
