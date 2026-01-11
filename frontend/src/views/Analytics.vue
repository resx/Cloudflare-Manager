<template>
  <!-- Analytics - Island Theme -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">分析统计</h1>
      <p class="text-sm text-muted-foreground mt-1">
        管理域名: <span class="font-medium">{{ currentZone?.name || '未选择' }}</span>
      </p>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
      <div class="metric-card h-32">
        <div class="text-sm text-muted-foreground mb-2">请求总数</div>
        <div class="text-3xl font-semibold">-</div>
      </div>
      <div class="metric-card h-32">
        <div class="text-sm text-muted-foreground mb-2">带宽使用</div>
        <div class="text-3xl font-semibold">-</div>
      </div>
      <div class="metric-card h-32">
        <div class="text-sm text-muted-foreground mb-2">威胁拦截</div>
        <div class="text-3xl font-semibold">-</div>
      </div>
    </div>

    <div class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">📈</div>
      <h3 class="font-semibold mb-2">分析数据</h3>
      <p class="text-sm text-muted-foreground">
        详细的流量分析功能即将上线
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, type Ref } from 'vue'
import type { Zone } from '@/api'

const currentZone = inject<Ref<Zone | null>>('currentZone')
</script>
