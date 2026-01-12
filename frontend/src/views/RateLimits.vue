<template>
  <!-- Rate Limits - Island Theme -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">速率限制</h1>
      <p class="text-sm text-muted-foreground mt-1">
        管理域名: <span class="font-medium">{{ currentZone?.name || '未选择' }}</span>
      </p>
    </div>

    <div class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">⏱️</div>
      <h3 class="font-semibold mb-2">速率限制规则</h3>
      <p class="text-sm text-muted-foreground mb-4">
        限制客户端的请求频率，防止滥用
      </p>
      <p class="text-xs text-muted-foreground">
        可以使用防火墙规则中的 rate() 函数实现类似功能
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, type Ref } from 'vue'
import type { Zone } from '@/api'

const currentZone = inject<Ref<Zone | null>>('currentZone')
</script>
