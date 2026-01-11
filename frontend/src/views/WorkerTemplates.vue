<template>
  <!-- Worker Templates - Island Theme -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">Worker 模板库</h1>
      <p class="text-sm text-muted-foreground mt-1">快速开始使用预建的 Worker 模板</p>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <div v-for="template in templates" :key="template.id" class="metric-card p-6 hover:border-primary transition-colors cursor-pointer">
        <div class="text-3xl mb-3">{{ template.icon }}</div>
        <h3 class="font-semibold mb-2">{{ template.name }}</h3>
        <p class="text-sm text-muted-foreground mb-4">{{ template.description }}</p>
        <div class="flex gap-2">
          <span class="px-2 py-1 text-xs rounded-full bg-primary/10 text-primary">{{ template.category }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const templates = ref([
  {
    id: 1,
    name: 'Hello World',
    description: '基础的 Worker 示例',
    category: '入门',
    icon: '👋',
  },
  {
    id: 2,
    name: 'API Gateway',
    description: 'API 网关和路由',
    category: 'API',
    icon: '🚪',
  },
  {
    id: 3,
    name: 'CDN 加速',
    description: '内容分发和缓存',
    category: '性能',
    icon: '⚡',
  },
  {
    id: 4,
    name: '身份验证',
    description: 'JWT 身份验证',
    category: '安全',
    icon: '🔐',
  },
  {
    id: 5,
    name: '图片处理',
    description: '图片优化和转换',
    category: '媒体',
    icon: '🖼️',
  },
  {
    id: 6,
    name: 'A/B 测试',
    description: '流量分割测试',
    category: '实验',
    icon: '🧪',
  },
])
</script>
