<template>
  <!-- Workers Management - Island Theme -->
  <div class="animate-in">
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-semibold">Workers 管理</h1>
        <p class="text-sm text-muted-foreground mt-1">管理 Cloudflare Workers 脚本</p>
      </div>
      <button class="btn-island-primary" @click="showCreateModal = true">
        创建 Worker
      </button>
    </div>

    <!-- Coming Soon -->
    <div class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">⚙️</div>
      <h3 class="font-semibold mb-2">功能即将上线</h3>
      <p class="text-sm text-muted-foreground mb-4">
        Workers 管理功能正在开发中
      </p>
      <button class="btn-island-secondary" @click="$router.push('/worker-templates')">
        查看 Worker 模板
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const showCreateModal = ref(false)
</script>
