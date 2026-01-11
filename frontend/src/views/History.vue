<template>
  <!-- Operation History - Island Theme with Full Functionality -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">操作历史</h1>
      <p class="text-sm text-muted-foreground mt-1">查看所有操作记录</p>
    </div>

    <!-- Filters -->
    <div class="metric-card p-4 mb-6">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label class="block text-sm font-medium mb-2">操作类型</label>
          <select
            v-model="filters.type"
            class="w-full px-3 py-2 bg-background border border-border rounded-lg text-sm"
          >
            <option value="">全部</option>
            <option value="dns">DNS 记录</option>
            <option value="firewall">防火墙</option>
            <option value="ssl">SSL/TLS</option>
            <option value="cache">缓存</option>
            <option value="worker">Workers</option>
          </select>
        </div>
        
        <div>
          <label class="block text-sm font-medium mb-2">时间范围</label>
          <select
            v-model="filters.timeRange"
            class="w-full px-3 py-2 bg-background border border-border rounded-lg text-sm"
          >
            <option value="24h">最近24小时</option>
            <option value="7d">最近7天</option>
            <option value="30d">最近30天</option>
            <option value="all">全部</option>
          </select>
        </div>
        
        <div>
          <label class="block text-sm font-medium mb-2">状态</label>
          <select
            v-model="filters.status"
            class="w-full px-3 py-2 bg-background border border-border rounded-lg text-sm"
          >
            <option value="">全部</option>
            <option value="success">成功</option>
            <option value="error">失败</option>
          </select>
        </div>
      </div>
    </div>

    <!-- History List -->
    <div v-if="filteredHistory.length > 0" class="space-y-3">
      <div 
        v-for="item in filteredHistory" 
        :key="item.id"
        class="metric-card p-4 hover:shadow-md transition-shadow"
      >
        <div class="flex items-start justify-between">
          <div class="flex-1">
            <div class="flex items-center gap-3 mb-2">
              <span class="text-xl">{{ getIcon(item.type) }}</span>
              <h3 class="font-semibold">{{ item.action }}</h3>
              <span :class="[
                'px-2 py-1 text-xs rounded-full',
                item.status === 'success' 
                  ? 'bg-success text-success-foreground' 
                  : 'bg-red-100 text-red-700'
              ]">
                {{ item.status === 'success' ? '成功' : '失败' }}
              </span>
            </div>
            
            <div class="text-sm text-muted-foreground space-y-1">
              <div>{{ item.description }}</div>
              <div class="flex items-center gap-4">
                <span>🕐 {{ formatDate(item.timestamp) }}</span>
                <span v-if="item.user">👤 {{ item.user }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">🕒</div>
      <h3 class="font-semibold mb-2">暂无操作记录</h3>
      <p class="text-sm text-muted-foreground">您的操作记录将显示在这里</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

interface HistoryItem {
  id: string
  type: string
  action: string
  description: string
  status: 'success' | 'error'
  timestamp: string
  user?: string
}

const filters = ref({
  type: '',
  timeRange: '7d',
  status: '',
})

const history = ref<HistoryItem[]>([
  {
    id: '1',
    type: 'dns',
    action: '添加 DNS 记录',
    description: '为 example.com 添加了 A 记录',
    status: 'success',
    timestamp: new Date().toISOString(),
    user: '当前账户',
  },
  {
    id: '2',
    type: 'firewall',
    action: '创建防火墙规则',
    description: '添加了"阻止特定国家访问"规则',
    status: 'success',
    timestamp: new Date(Date.now() - 3600000).toISOString(),
    user: '当前账户',
  },
  {
    id: '3',
    type: 'ssl',
    action: '更新 SSL 设置',
    description: '启用了 TLS 1.3',
    status: 'success',
    timestamp: new Date(Date.now() - 7200000).toISOString(),
    user: '当前账户',
  },
])

const filteredHistory = computed(() => {
  return history.value.filter(item => {
    if (filters.value.type && item.type !== filters.value.type) return false
    if (filters.value.status && item.status !== filters.value.status) return false
    
    // Time range filter
    const itemTime = new Date(item.timestamp).getTime()
    const now = Date.now()
    
    if (filters.value.timeRange === '24h' && now - itemTime > 86400000) return false
    if (filters.value.timeRange === '7d' && now - itemTime > 604800000) return false
    if (filters.value.timeRange === '30d' && now - itemTime > 2592000000) return false
    
    return true
  })
})

function getIcon(type: string): string {
  const icons: Record<string, string> = {
    dns: '🌐',
    firewall: '🛡️',
    ssl: '🔒',
    cache: '💨',
    worker: '⚙️',
  }
  return icons[type] || '📝'
}

function formatDate(dateString: string): string {
  const date = new Date(dateString)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  
  if (diff < 60000) return '刚刚'
  if (diff < 3600000) return `${Math.floor(diff / 60000)} 分钟前`
  if (diff < 86400000) return `${Math.floor(diff / 3600000)} 小时前`
  if (diff < 604800000) return `${Math.floor(diff / 86400000)} 天前`
  
  return date.toLocaleString('zh-CN')
}

onMounted(() => {
  // Load history from localStorage or API
})
</script>
