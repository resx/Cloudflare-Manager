<template>
  <!-- Workers KV - Island Theme with Full Functionality -->
  <div class="animate-in">
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-semibold">Workers KV</h1>
        <p class="text-sm text-muted-foreground mt-1">键值对存储管理</p>
      </div>
      <button class="btn-island-primary" @click="showCreateModal = true">
        <span class="text-lg mr-2">+</span>
        创建命名空间
      </button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="metric-card p-12 text-center">
      <div class="text-4xl mb-4">⏳</div>
      <p class="text-muted-foreground">加载命名空间...</p>
    </div>

    <!-- Namespaces List -->
    <div v-else-if="namespaces.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
      <div 
        v-for="ns in namespaces" 
        :key="ns.id"
        class="metric-card p-6 cursor-pointer hover:border-primary transition-colors"
        @click="viewNamespace(ns)"
      >
        <div class="flex items-start justify-between mb-4">
          <div class="text-3xl">🔑</div>
          <button 
            @click.stop="deleteNamespace(ns)"
            class="text-muted-foreground hover:text-red-600"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
            </svg>
          </button>
        </div>
        
        <h3 class="font-semibold mb-2">{{ ns.title }}</h3>
        <div class="text-xs text-muted-foreground space-y-1">
          <div>ID: {{ ns.id }}</div>
          <div v-if="ns.key_count !== undefined">键数: {{ ns.key_count }}</div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">🔑</div>
      <h3 class="font-semibold mb-2">暂无 KV 命名空间</h3>
      <p class="text-sm text-muted-foreground mb-4">创建命名空间来存储键值对数据</p>
      <button class="btn-island-primary" @click="showCreateModal = true">
        创建命名空间
      </button>
    </div>

    <!-- Create Modal -->
    <div v-if="showCreateModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="showCreateModal = false">
      <div class="bg-white rounded-2xl shadow-lg w-full max-w-xl" @click.stop>
        <div class="p-6 border-b border-border">
          <h2 class="text-xl font-semibold">创建 KV 命名空间</h2>
        </div>
        
        <div class="p-6 space-y-4">
          <div>
            <label class="block text-sm font-medium mb-2">命名空间名称 *</label>
            <input
              v-model="newNamespace.title"
              class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
              placeholder="my-kv-namespace"
            />
          </div>
        </div>

        <div class="p-6 border-t border-border flex justify-end gap-3">
          <button class="btn-island-secondary" @click="showCreateModal = false">取消</button>
          <button class="btn-island-primary" @click="createNamespace">创建</button>
        </div>
      </div>
    </div>

    <!-- View Namespace Modal -->
    <div v-if="selectedNamespace" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="selectedNamespace = null">
      <div class="bg-white rounded-2xl shadow-lg w-full max-w-4xl max-h-[90vh] overflow-y-auto" @click.stop>
        <div class="p-6 border-b border-border flex justify-between items-center">
          <h2 class="text-xl font-semibold">{{ selectedNamespace.title }}</h2>
          <button @click="selectedNamespace = null" class="text-muted-foreground hover:text-foreground">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </div>
        
        <div class="p-6">
          <div class="flex justify-between items-center mb-4">
            <h3 class="font-semibold">键值对列表</h3>
            <button class="btn-island-primary text-sm" @click="showAddKeyModal = true">
              添加键值对
            </button>
          </div>
          
          <div class="metric-card p-12 text-center">
            <p class="text-sm text-muted-foreground">键值对管理功能即将上线</p>
          </div>
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

interface KVNamespace {
  id: string
  title: string
  key_count?: number
}

const accountStore = useAccountStore()
const loading = ref(false)
const namespaces = ref<KVNamespace[]>([])
const showCreateModal = ref(false)
const showAddKeyModal = ref(false)
const selectedNamespace = ref<KVNamespace | null>(null)

const newNamespace = ref({
  title: ''
})

async function loadNamespaces() {
  const accountId = accountStore.currentAccount?.accountId
  if (!accountId) {
    toast.error('请先添加账户并确保账户信息已加载')
    return
  }

  loading.value = true
  try {
    namespaces.value = await cloudflareApi.listKVNamespaces(accountId)
  } catch (error) {
    console.error('Failed to load KV namespaces:', error)
    toast.error('加载命名空间失败')
  } finally {
    loading.value = false
  }
}

async function createNamespace() {
  const accountId = accountStore.currentAccount?.accountId
  if (!accountId) {
    toast.error('请先添加账户')
    return
  }

  if (!newNamespace.value.title) {
    toast.warning('请输入命名空间名称')
    return
  }

  try {
    await cloudflareApi.createKVNamespace({
      account_id: accountId,
      title: newNamespace.value.title
    })
    logHistory.worker('创建 KV 命名空间', `命名空间: ${newNamespace.value.title}`)
    toast.success('命名空间已创建')
    showCreateModal.value = false
    newNamespace.value = { title: '' }
    loadNamespaces()
  } catch (error: any) {
    console.error('Failed to create namespace:', error)
    toast.error(error.message || '创建失败')
  }
}

async function deleteNamespace(ns: KVNamespace) {
  const accountId = accountStore.currentAccount?.accountId
  if (!accountId) {
    toast.error('账户信息缺失')
    return
  }

  if (!confirm(`确定要删除命名空间 "${ns.title}" 吗？`)) return

  try {
    await cloudflareApi.deleteKVNamespace(accountId, ns.id)
    logHistory.worker('删除 KV 命名空间', `命名空间: ${ns.title}`)
    toast.success('命名空间已删除')
    loadNamespaces()
  } catch (error) {
    console.error('Failed to delete namespace:', error)
    toast.error('删除失败')
  }
}

function viewNamespace(ns: KVNamespace) {
  selectedNamespace.value = ns
}

onMounted(() => {
  loadNamespaces()
})
</script>
