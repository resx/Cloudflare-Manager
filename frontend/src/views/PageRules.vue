<template>
  <!-- Page Rules - Island Theme -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">页面规则</h1>
      <p class="text-sm text-muted-foreground mt-1">
        管理域名: <span class="font-medium">{{ currentZone?.name || '未选择' }}</span>
      </p>
    </div>

    <div class="alert-info mb-6">
      <strong>提示：</strong>页面规则允许您为特定 URL 模式自定义 Cloudflare 设置
    </div>

    <div class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">📄</div>
      <h3 class="font-semibold mb-2">页面规则管理</h3>
      <p class="text-sm text-muted-foreground mb-4">
        此功能需要付费套餐支持
      </p>
      <button class="btn-island-primary" disabled>
        创建页面规则
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, type Ref } from 'vue'
import type { Zone } from '@/api'

const currentZone = inject<Ref<Zone | null>>('currentZone')
</script>
