<template>
  <!-- WAF - Island Theme -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">Web 应用防火墙 (WAF)</h1>
      <p class="text-sm text-muted-foreground mt-1">
        管理域名: <span class="font-medium">{{ currentZone?.name || '未选择' }}</span>
      </p>
    </div>

    <div class="banner-gradient rounded-lg p-6 mb-6">
      <h3 class="text-lg font-semibold mb-2">🔥 WAF 规则集</h3>
      <p class="text-sm text-muted-foreground">
        检测和阻止常见的 Web 应用攻击，包括 SQL 注入、XSS 等
      </p>
    </div>

    <div class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">🔥</div>
      <h3 class="font-semibold mb-2">WAF 功能</h3>
      <p class="text-sm text-muted-foreground mb-4">
        此功能需要 Pro 套餐或更高版本
      </p>
      <button class="btn-island-secondary" @click="$router.push('/firewall')">
        使用基础防火墙规则
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, type Ref } from 'vue'
import type { Zone } from '@/api'

const currentZone = inject<Ref<Zone | null>>('currentZone')
</script>
