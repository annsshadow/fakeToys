<template>
  <div class="chart-container" ref="chartRef"></div>
</template>

<script setup lang="ts">
import { ref, watch, onBeforeUnmount, computed } from 'vue'
import * as echarts from 'echarts'

const props = defineProps<{
  data: any[]
  columns: string[]
  chartType: 'bar' | 'pie' | 'line' | 'table'
  dimension?: string   // Column to use as X/dimension
  metric?: string      // Column to aggregate
}>()

const chartRef = ref<HTMLElement | null>(null)
let chart: echarts.ECharts | null = null

const chartTypeMap = computed(() => props.chartType)

watch([() => props.data, () => props.chartType, () => props.dimension, () => props.metric], () => {
  if (!chartRef.value || !props.data.length) return
  if (!chart) chart = echarts.init(chartRef.value)
  const dims = props.dimension ? [props.dimension] : props.columns.slice(0, 2)
  const metrics = props.metric ? [props.metric] : props.columns.filter(c => c !== dims[0]).slice(0, 3)

  let option: any = {}
  const type = props.chartType

  if (type === 'bar') {
    option = {
      backgroundColor: 'transparent',
      tooltip: { trigger: 'axis' },
      xAxis: { type: 'category', data: props.data.map((r: any) => r[dims[0]] || r['']), axisLabel: { color: 'var(--text-muted)', fontSize: 10 } },
      yAxis: { type: 'value', axisLabel: { color: 'var(--text-muted)', fontSize: 10 } },
      series: metrics.map((m: string, i: number) => ({
        name: m, type: 'bar', data: props.data.map((r: any) => Number(r[m]) || 0),
        itemStyle: { color: ['#00d4ff', '#f59e0b', '#10b981', '#ef4444'][i % 4] }
      }))
    }
  } else if (type === 'pie') {
    option = {
      backgroundColor: 'transparent',
      tooltip: { trigger: 'item' },
      series: [{
        type: 'pie', radius: ['40%', '70%'],
        data: props.data.map((r: any) => ({ name: r[dims[0]] || '', value: Number(r[metrics[0]]) || 1 })),
        label: { color: 'var(--text-muted)', fontSize: 10 },
        itemStyle: { color: (params: any) => ['#00d4ff', '#f59e0b', '#10b981', '#ef4444', '#8b5cf6', '#ec4899'][params.dataIndex % 6] }
      }]
    }
  } else if (type === 'line') {
    option = {
      backgroundColor: 'transparent',
      tooltip: { trigger: 'axis' },
      xAxis: { type: 'category', data: props.data.map((r: any) => r[dims[0]] || ''), axisLabel: { color: 'var(--text-muted)', fontSize: 10 } },
      yAxis: { type: 'value', axisLabel: { color: 'var(--text-muted)', fontSize: 10 } },
      series: metrics.map((m: string, i: number) => ({
        name: m, type: 'line', data: props.data.map((r: any) => Number(r[m]) || 0),
        smooth: true, itemStyle: { color: ['#00d4ff', '#f59e0b', '#10b981'][i % 3] },
        areaStyle: { opacity: 0.1 }
      }))
    }
  }

  chart.setOption(option, true)
}, { immediate: true, deep: true })

onBeforeUnmount(() => { chart?.dispose(); chart = null })
</script>

<style scoped>
.chart-container { width: 100%; height: 300px; }
</style>
