<template>
  <!-- Certificates - Island Theme -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">证书管理</h1>
      <p class="text-sm text-muted-foreground mt-1">
        管理域名: <span class="font-medium">{{ currentZone?.name || '未选择' }}</span>
      </p>
    </div>

    <div class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">🏆</div>
      <h3 class="font-semibold mb-2">SSL/TLS 证书</h3>
      <p class="text-sm text-muted-foreground mb-4">
        Cloudflare 自动为您的域名提供免费 SSL 证书
      </p>
      <button class="btn-island-primary" @click="$router.push('/ssl')">
        前往 SSL 设置
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, type Ref } from 'vue'
import type { Zone } from '@/api'

const currentZone = inject<Ref<Zone | null>>('currentZone')
</script>
