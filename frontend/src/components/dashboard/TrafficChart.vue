<template>
  <GlassCard class="w-full h-[400px] flex flex-col">
    <div class="flex justify-between items-center mb-6">
      <h3 class="text-lg font-bold flex items-center gap-2">
        <component :is="AnalyticsOutline" class="w-5 h-5 text-primary" />
        流量趋势 (Last 24 Hours)
      </h3>
      <div class="flex gap-2">
        <GlassBadge variant="info" class="cursor-pointer">Requests</GlassBadge>
        <GlassBadge variant="warning" class="opacity-50 cursor-not-allowed">Bandwidth</GlassBadge>
      </div>
    </div>
    <div class="flex-1 w-full min-h-0">
      <v-chart class="chart" :option="chartOption" autoresize />
    </div>
  </GlassCard>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { use } from 'echarts/core'
import { CanvasRenderer } from 'echarts/renderers'
import { LineChart } from 'echarts/charts'
import {
  GridComponent,
  TooltipComponent,
  LegendComponent,
  TitleComponent,
} from 'echarts/components'
import VChart from 'vue-echarts'
import GlassCard from '../ui/GlassCard.vue'
import GlassBadge from '../ui/GlassBadge.vue'
import { AnalyticsOutline } from '@vicons/ionicons5'
import type { TimeseriesPoint } from '@/api'

use([
  CanvasRenderer,
  LineChart,
  GridComponent,
  TooltipComponent,
  LegendComponent,
  TitleComponent,
])

const props = defineProps<{
  data: TimeseriesPoint[]
}>()

const chartOption = computed(() => {
  const dates = props.data.map(p => new Date(p.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }))
  const requests = props.data.map(p => p.requests)
  const cached = props.data.map(p => p.cached)

  return {
    backgroundColor: 'transparent',
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(255, 255, 255, 0.8)',
      backdropFilter: 'blur(10px)',
      borderWidth: 0,
      textStyle: { color: '#333' },
      extraCssText: 'box-shadow: 0 8px 32px rgba(0,0,0,0.1); border-radius: 12px;'
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '5%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: dates,
      axisLine: { lineStyle: { color: 'rgba(156, 163, 175, 0.3)' } },
      axisLabel: { color: '#9ca3af', fontSize: 10 }
    },
    yAxis: {
      type: 'value',
      splitLine: { lineStyle: { color: 'rgba(156, 163, 175, 0.1)', type: 'dashed' } },
      axisLabel: { color: '#9ca3af', fontSize: 10 }
    },
    series: [
      {
        name: 'Total Requests',
        type: 'line',
        smooth: true,
        data: requests,
        itemStyle: { color: '#3b82f6' },
        lineStyle: { width: 3 },
        areaStyle: {
          color: {
            type: 'linear',
            x: 0, y: 0, x2: 0, y2: 1,
            colorStops: [
              { offset: 0, color: 'rgba(59, 130, 246, 0.2)' },
              { offset: 1, color: 'rgba(59, 130, 246, 0)' }
            ]
          }
        }
      },
      {
        name: 'Cached Requests',
        type: 'line',
        smooth: true,
        data: cached,
        itemStyle: { color: '#10b981' },
        lineStyle: { width: 2, type: 'dashed' },
      }
    ]
  }
})
</script>

<style scoped>
.chart {
  height: 100%;
  width: 100%;
}
</style>
