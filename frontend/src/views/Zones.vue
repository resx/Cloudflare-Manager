<template>
  <!-- Zones Management - Island Theme -->
  <div class="animate-in">
    <!-- Header -->
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-semibold">域名列表</h1>
        <p class="text-sm text-muted-foreground mt-1">管理您的 Cloudflare 域名</p>
      </div>
      <button class="btn-island-primary" disabled title="功能开发中">
        添加域名
      </button>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="metric-card p-12 text-center">
      <div class="text-4xl mb-4">⏳</div>
      <p class="text-muted-foreground">加载域名列表...</p>
    </div>

    <!-- Zones Table -->
    <div v-else-if="zones.length > 0" class="metric-card p-6">
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead>
            <tr class="border-b border-border">
              <th class="text-left py-3 px-4 text-sm font-semibold text-foreground">域名</th>
              <th class="text-left py-3 px-4 text-sm font-semibold text-foreground">状态</th>
              <th class="text-left py-3 px-4 text-sm font-semibold text-foreground">套餐</th>
              <th class="text-left py-3 px-4 text-sm font-semibold text-foreground">NS 服务器</th>
              <th class="text-center py-3 px-4 text-sm font-semibold text-foreground">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr 
              v-for="zone in zones" 
              :key="zone.id"
              class="border-b border-border last:border-b-0 hover:bg-muted transition-colors"
            >
              <td class="py-3 px-4">
                <div class="font-medium text-sm">{{ zone.name }}</div>
                <div class="text-xs text-muted-foreground">ID: {{ zone.id.substring(0, 8) }}...</div>
              </td>
              <td class="py-3 px-4">
                <span :class="[
                  'px-2 py-1 text-xs rounded-full',
                  zone.status === 'active' 
                    ? 'bg-success text-success-foreground' 
                    : 'bg-muted text-muted-foreground'
                ]">
                  {{ zone.status }}
                </span>
              </td>
              <td class="py-3 px-4">
                <span class="text-sm">{{ zone.plan?.name || 'Free' }}</span>
              </td>
              <td class="py-3 px-4">
                <div class="text-xs text-muted-foreground space-y-1">
                  <div v-for="ns in zone.name_servers?.slice(0, 2)" :key="ns">{{ ns }}</div>
                </div>
              </td>
              <td class="py-3 px-4 text-center">
                <button
                  @click="goToZoneDetail(zone)"
                  class="btn-island-secondary text-xs h-8"
                >
                  管理
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">🌐</div>
      <h3 class="font-semibold mb-2">暂无域名</h3>
      <p class="text-sm text-muted-foreground">请在 Cloudflare 控制台添加域名</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { cloudflareApi, type Zone } from '@/api'

const router = useRouter()
const loading = ref(false)
const zones = ref<Zone[]>([])

async function loadZones() {
  loading.value = true
  try {
    zones.value = await cloudflareApi.getZones()
  } catch (error) {
    console.error('Failed to load zones:', error)
  } finally {
    loading.value = false
  }
}

function goToZoneDetail(zone: Zone) {
  // Set the zone in localStorage AND update parent's currentZone
  localStorage.setItem('currentZoneId', zone.id)
  // Trigger a page reload or emit event to update parent
  window.location.href = '/dns'
}

onMounted(() => {
  loadZones()
})
</script>
