<template>
  <!-- D1 Database - Island Theme with Full Functionality -->
  <div class="animate-in">
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-semibold">D1 数据库</h1>
        <p class="text-sm text-muted-foreground mt-1">无服务器 SQL 数据库</p>
      </div>
      <button class="btn-island-primary" @click="showCreateModal = true">
        <span class="text-lg mr-2">+</span>
        创建数据库
      </button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="metric-card p-12 text-center">
      <div class="text-4xl mb-4">⏳</div>
      <p class="text-muted-foreground">加载数据库...</p>
    </div>

    <!-- Database List -->
    <div v-else-if="databases.length > 0" class="grid grid-cols-1 md:grid-cols-2 gap-5">
      <div 
        v-for="db in databases" 
        :key="db.uuid"
        class="metric-card p-6 hover:border-primary transition-colors cursor-pointer"
        @click="viewDatabase(db)"
      >
        <div class="flex items-start justify-between mb-4">
          <div class="text-3xl">💾</div>
          <button 
            @click.stop="deleteDatabase(db)"
            class="text-muted-foreground hover:text-red-600"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
            </svg>
          </button>
        </div>
        
        <h3 class="font-semibold mb-2">{{ db.name }}</h3>
        <div class="text-xs text-muted-foreground space-y-1">
          <div>UUID: {{ db.uuid }}</div>
          <div v-if="db.size">大小: {{ formatSize(db.size) }}</div>
          <div>创建: {{ formatDate(db.created_at) }}</div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">💾</div>
      <h3 class="font-semibold mb-2">暂无 D1 数据库</h3>
      <p class="text-sm text-muted-foreground mb-4">创建无服务器 SQL 数据库</p>
      <button class="btn-island-primary" @click="showCreateModal = true">
        创建数据库
      </button>
    </div>

    <!-- Create Modal -->
    <div v-if="showCreateModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="showCreateModal = false">
      <div class="bg-white rounded-2xl shadow-lg w-full max-w-xl" @click.stop>
        <div class="p-6 border-b border-border">
          <h2 class="text-xl font-semibold">创建 D1 数据库</h2>
        </div>
        
        <div class="p-6 space-y-4">
          <div>
            <label class="block text-sm font-medium mb-2">数据库名称 *</label>
            <input
              v-model="newDatabase.name"
              class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
              placeholder="my-database"
            />
          </div>
        </div>

        <div class="p-6 border-t border-border flex justify-end gap-3">
          <button class="btn-island-secondary" @click="showCreateModal = false">取消</button>
          <button class="btn-island-primary" @click="createDatabase">创建</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAccountStore } from '@/stores/account'
import { cloudflareApi } from '@/api'
import { toast } from '@/utils/toast'
import { logHistory } from '@/utils/history'

interface D1Database {
  uuid: string
  name: string
  size?: number
  created_at: string
}

const accountStore = useAccountStore()
const loading = ref(false)
const databases = ref<D1Database[]>([])
const showCreateModal = ref(false)

const newDatabase = ref({
  name: ''
})

async function loadDatabases() {
  const accountId = accountStore.currentAccount?.accountId
  if (!accountId) {
    toast.error('请先添加账户并确保账户信息已加载')
    return
  }

  loading.value = true
  try {
    databases.value = await cloudflareApi.listD1Databases(accountId)
  } catch (error) {
    console.error('Failed to load D1 databases:', error)
    toast.error('加载数据库失败')
  } finally {
    loading.value = false
  }
}

async function createDatabase() {
  const accountId = accountStore.currentAccount?.accountId
  if (!accountId) {
    toast.error('请先添加账户')
    return
  }

  if (!newDatabase.value.name) {
    toast.warning('请输入数据库名称')
    return
  }

  try {
    await cloudflareApi.createD1Database({
      account_id: accountId,
      name: newDatabase.value.name
    })
    logHistory.worker('创建 D1 数据库', `数据库: ${newDatabase.value.name}`)
    toast.success('数据库已创建')
    showCreateModal.value = false
    newDatabase.value = { name: '' }
    loadDatabases()
  } catch (error: any) {
    console.error('Failed to create database:', error)
    toast.error(error.message || '创建失败')
  }
}

async function deleteDatabase(db: D1Database) {
  const accountId = accountStore.currentAccount?.accountId
  if (!accountId) {
    toast.error('账户信息缺失')
    return
  }

  if (!confirm(`确定要删除数据库 "${db.name}" 吗？`)) return

  try {
    await cloudflareApi.deleteD1Database(accountId, db.uuid)
    logHistory.worker('删除 D1 数据库', `数据库: ${db.name}`)
    toast.success('数据库已删除')
    loadDatabases()
  } catch (error) {
    console.error('Failed to delete database:', error)
    toast.error('删除失败')
  }
}

function viewDatabase(db: D1Database) {
  toast.info('数据库查询功能即将上线')
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(2) + ' MB'
}

function formatDate(dateString: string): string {
  if (!dateString) return '-'
  return new Date(dateString).toLocaleDateString('zh-CN')
}

onMounted(() => {
  loadDatabases()
})
</script>
