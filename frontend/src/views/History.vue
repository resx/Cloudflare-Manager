<template>
  <n-card title="操作历史">
    <n-alert type="info" style="margin-bottom: 16px">
      所有操作记录存储在本地浏览器中
    </n-alert>

    <n-timeline>
      <n-timeline-item
        v-for="record in history"
        :key="record.id"
        :type="getTimelineType(record.type)"
        :title="record.action"
      >
        <n-space vertical :size="4">
          <n-text>{{ record.description }}</n-text>
          <n-text depth="3">{{ formatDate(record.timestamp) }}</n-text>
        </n-space>
      </n-timeline-item>
    </n-timeline>

    <n-empty
      v-if="history.length === 0"
      description="暂无操作记录"
    />
  </n-card>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface HistoryRecord {
  id: string
  type: 'dns' | 'worker' | 'optimize' | 'firewall'
  action: string
  description: string
  timestamp: string
}

const history = ref<HistoryRecord[]>([])

function getTimelineType(type: string) {
  const typeMap: Record<string, any> = {
    dns: 'success',
    worker: 'info',
    optimize: 'warning',
    firewall: 'error'
  }
  return typeMap[type] || 'default'
}

function formatDate(date: string) {
  return new Date(date).toLocaleString('zh-CN')
}

function loadHistory() {
  try {
    const stored = localStorage.getItem('cf_operation_history')
    if (stored) {
      history.value = JSON.parse(stored)
    }
  } catch (error) {
    console.error('Failed to load history:', error)
  }
}

onMounted(() => {
  loadHistory()
})
</script>
