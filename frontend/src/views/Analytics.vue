<template>
  <n-space vertical :size="24">
    <!-- 时间范围选择 -->
    <n-card>
      <n-space>
        <n-select
          v-model:value="timeRange"
          :options="timeRangeOptions"
          style="width: 200px"
          @update:value="handleTimeRangeChange"
        />
        <n-button @click="loadAnalytics" :loading="loading">
          刷新数据
        </n-button>
      </n-space>
    </n-card>

    <!-- 概览统计 -->
    <n-grid :cols="4" :x-gap="16">
      <n-gi>
        <n-card>
          <n-statistic label="总请求数" :value="formatNumber(analyticsData?.stats.totalRequests || 0)">
            <template #suffix>次</template>
          </n-statistic>
        </n-card>
      </n-gi>
      <n-gi>
        <n-card>
          <n-statistic label="缓存命中率" :value="analyticsData?.stats.cacheHitRate.toFixed(1) || '0.0'">
            <template #suffix>%</template>
          </n-statistic>
        </n-card>
      </n-gi>
      <n-gi>
        <n-card>
          <n-statistic label="带宽使用" :value="analyticsData?.stats.bandwidth.toFixed(2) || '0.00'">
            <template #suffix>GB</template>
          </n-statistic>
        </n-card>
      </n-gi>
      <n-gi>
        <n-card>
          <n-statistic label="威胁拦截" :value="formatNumber(analyticsData?.stats.threats || 0)">
            <template #suffix>次</template>
          </n-statistic>
        </n-card>
      </n-gi>
    </n-grid>

    <!-- 请求趋势图表 -->
    <n-card title="请求趋势">
      <n-spin :show="loading">
        <v-chart
          v-if="!loading && analyticsData"
          :option="requestsChartOption"
          style="height: 350px;"
          autoresize
        />
        <n-empty v-else-if="!loading" description="暂无数据" />
      </n-spin>
    </n-card>

    <!-- HTTP 状态码分布 -->
    <n-card title="HTTP 状态码分布">
      <n-spin :show="loading">
        <n-data-table
          :columns="statusCodeColumns"
          :data="analyticsData?.statusCodes || []"
          :bordered="false"
          :pagination="false"
        />
      </n-spin>
    </n-card>

    <!-- 访问地域分布 -->
    <n-card title="访问地域分布（Top 10）">
      <n-spin :show="loading">
        <n-data-table
          :columns="geoColumns"
          :data="analyticsData?.countries || []"
          :bordered="false"
          :pagination="false"
        />
      </n-spin>
    </n-card>

    <!-- 热门内容 -->
    <n-card title="热门内容（Top 10）">
      <n-spin :show="loading">
        <n-data-table
          :columns="contentColumns"
          :data="analyticsData?.content || []"
          :bordered="false"
          :pagination="{ pageSize: 10 }"
        />
      </n-spin>
    </n-card>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useMessage } from 'naive-ui'
import { use } from 'echarts/core'
import { CanvasRenderer } from 'echarts/renderers'
import { LineChart } from 'echarts/charts'
import {
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent
} from 'echarts/components'
import VChart from 'vue-echarts'
import { cloudflareApi, type AnalyticsData } from '@/api'

// 注册 ECharts 组件
use([
  CanvasRenderer,
  LineChart,
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent
])

const message = useMessage()
const loading = ref(false)

const timeRange = ref('24h')
const analyticsData = ref<AnalyticsData | null>(null)

const currentZoneId = computed(() => {
  return localStorage.getItem('currentZoneId') || ''
})

const timeRangeOptions = [
  { label: '最近 24 小时', value: '24h' },
  { label: '最近 7 天', value: '7d' },
  { label: '最近 30 天', value: '30d' }
]

const statusCodeColumns = [
  { title: '状态码', key: 'code', width: 120 },
  { title: '描述', key: 'description' },
  {
    title: '请求数',
    key: 'count',
    width: 150,
    render: (row: any) => formatNumber(row.count)
  },
  {
    title: '占比',
    key: 'percentage',
    width: 100,
    render: (row: any) => `${row.percentage.toFixed(1)}%`
  }
]

const geoColumns = [
  { title: '排名', key: 'rank', width: 80 },
  { title: '国家/地区', key: 'country' },
  {
    title: '请求数',
    key: 'requests',
    width: 150,
    render: (row: any) => formatNumber(row.requests)
  },
  {
    title: '占比',
    key: 'percentage',
    width: 100,
    render: (row: any) => `${row.percentage.toFixed(1)}%`
  }
]

const contentColumns = [
  { title: '排名', key: 'rank', width: 80 },
  { title: 'URL', key: 'url' },
  {
    title: '请求数',
    key: 'requests',
    width: 150,
    render: (row: any) => formatNumber(row.requests)
  },
  { title: '带宽', key: 'bandwidth', width: 120 }
]

// ECharts 配置
const requestsChartOption = computed(() => {
  if (!analyticsData.value) return {}

  const timestamps = analyticsData.value.timeseries.map(point => {
    const date = new Date(point.timestamp)
    return date.toLocaleString('zh-CN', {
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit'
    })
  })

  return {
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'cross'
      }
    },
    legend: {
      data: ['总请求', '缓存命中', '未命中缓存']
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: timestamps
    },
    yAxis: {
      type: 'value',
      name: '请求数'
    },
    series: [
      {
        name: '总请求',
        type: 'line',
        data: analyticsData.value.timeseries.map(point => point.requests),
        smooth: true,
        lineStyle: {
          width: 2
        },
        areaStyle: {
          opacity: 0.3
        }
      },
      {
        name: '缓存命中',
        type: 'line',
        data: analyticsData.value.timeseries.map(point => point.cached),
        smooth: true,
        lineStyle: {
          width: 2
        }
      },
      {
        name: '未命中缓存',
        type: 'line',
        data: analyticsData.value.timeseries.map(point => point.uncached),
        smooth: true,
        lineStyle: {
          width: 2
        }
      }
    ]
  }
})

function formatNumber(num: number): string {
  return num.toLocaleString('zh-CN')
}

async function loadAnalytics() {
  if (!currentZoneId.value) {
    message.warning('请先选择域名')
    return
  }

  loading.value = true
  try {
    analyticsData.value = await cloudflareApi.getAnalytics(currentZoneId.value, timeRange.value)
  } catch (error: any) {
    console.error('Failed to load analytics:', error)
    message.error(error?.response?.data?.error || '加载统计数据失败')
  } finally {
    loading.value = false
  }
}

function handleTimeRangeChange() {
  loadAnalytics()
}

onMounted(() => {
  loadAnalytics()
})
</script>
