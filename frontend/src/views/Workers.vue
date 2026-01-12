<template>
  <!-- Workers Management - Island Theme with Full Functionality -->
  <div class="animate-in">
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-semibold">Workers 管理</h1>
        <p class="text-sm text-muted-foreground mt-1">管理 Cloudflare Workers 脚本</p>
      </div>
      <button class="btn-island-primary" @click="showCreateModal = true">
        <span class="text-lg mr-2">+</span>
        创建 Worker
      </button>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="metric-card p-12 text-center">
      <div class="text-4xl mb-4">⏳</div>
      <p class="text-muted-foreground">加载 Workers...</p>
    </div>

    <!-- Workers List -->
    <div v-else-if="workers.length > 0" class="space-y-4">
      <div 
        v-for="worker in workers" 
        :key="worker.id"
        class="metric-card p-6 hover:shadow-md transition-shadow"
      >
        <div class="flex items-start justify-between">
          <div class="flex-1">
            <!-- Worker Header -->
            <div class="flex items-center gap-3 mb-3">
              <h3 class="font-semibold text-lg">{{ worker.id }}</h3>
              <span class="px-2 py-1 text-xs rounded-full bg-success text-success-foreground">
                {{ worker.script ? '已部署' : '未部署' }}
              </span>
            </div>

            <!-- Worker Info -->
            <div class="space-y-2 text-sm text-muted-foreground">
              <div>创建时间: {{ formatDate(worker.created_on) }}</div>
              <div>修改时间: {{ formatDate(worker.modified_on) }}</div>
              <div v-if="worker.routes && worker.routes.length > 0">
                路由: {{ worker.routes.join(', ') }}
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex gap-2 ml-4">
            <button
              @click="editWorker(worker)"
              class="btn-island-secondary text-xs h-auto"
            >
              编辑
            </button>
            <button
              @click="deleteWorker(worker)"
              class="px-3 py-1.5 text-xs rounded-lg bg-red-50 text-red-600 hover:bg-red-100"
            >
              删除
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">⚙️</div>
      <h3 class="font-semibold mb-2">暂无 Workers</h3>
      <p class="text-sm text-muted-foreground mb-4">创建您的第一个 Worker 脚本</p>
      <button class="btn-island-primary" @click="showCreateModal = true">
        创建 Worker
      </button>
    </div>

    <!-- Create/Edit Worker Modal -->
    <div v-if="showCreateModal || editingWorker" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="closeModal">
      <div class="bg-white rounded-2xl shadow-lg w-full max-w-4xl max-h-[90vh] overflow-y-auto" @click.stop>
        <div class="p-6 border-b border-border">
          <h2 class="text-xl font-semibold">{{ editingWorker ? '编辑 Worker' : '创建 Worker' }}</h2>
        </div>
        
        <div class="p-6 space-y-4">
          <!-- Worker Name -->
          <div>
            <label class="block text-sm font-medium mb-2">Worker 名称 *</label>
            <input
              v-model="workerForm.name"
              :disabled="!!editingWorker"
              class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
              placeholder="my-worker"
            />
            <p class="text-xs text-muted-foreground mt-1">小写字母、数字和连字符</p>
          </div>

          <!-- Worker Script -->
          <div>
            <label class="block text-sm font-medium mb-2">Worker 代码 *</label>
            <textarea
              v-model="workerForm.script"
              class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50 font-mono"
              rows="15"
              placeholder="addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  return new Response('Hello World!')
}"
            ></textarea>
          </div>

          <!-- Routes -->
          <div>
            <label class="block text-sm font-medium mb-2">路由（可选）</label>
            <div class="space-y-2">
              <div v-for="(route, index) in workerForm.routes" :key="index" class="flex gap-2">
                <input
                  v-model="workerForm.routes[index]"
                  placeholder="example.com/*"
                  class="flex-1 px-3 py-2 bg-background border border-border rounded-lg text-sm"
                />
                <button
                  @click="workerForm.routes.splice(index, 1)"
                  class="px-3 py-2 text-red-600 hover:bg-red-50 rounded-lg"
                >
                  删除
                </button>
              </div>
              <button
                @click="workerForm.routes.push('')"
                class="btn-island-secondary text-xs"
              >
                + 添加路由
              </button>
            </div>
          </div>
        </div>

        <div class="p-6 border-t border-border flex justify-end gap-3">
          <button class="btn-island-secondary" @click="closeModal">取消</button>
          <button class="btn-island-primary" @click="saveWorker">
            {{ editingWorker ? '保存' : '创建' }}
          </button>
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

interface Worker {
  id: string
  script?: string
  created_on: string
  modified_on: string
  routes?: string[]
}

const accountStore = useAccountStore()
const loading = ref(false)
const workers = ref<Worker[]>([])
const showCreateModal = ref(false)
const editingWorker = ref<Worker | null>(null)

const workerForm = ref({
  name: '',
  script: '',
  routes: [] as string[],
})

async function loadWorkers() {
  const accountId = accountStore.currentAccount?.accountId
  if (!accountId) {
    toast.error('请先添加账户并确保账户信息已加载')
    return
  }

  loading.value = true
  try {
    workers.value = await cloudflareApi.listWorkers(accountId)
  } catch (error) {
    console.error('Failed to load workers:', error)
    toast.error('加载 Workers 失败')
  } finally {
    loading.value = false
  }
}

function editWorker(worker: Worker) {
  editingWorker.value = worker
  workerForm.value = {
    name: worker.id,
    script: worker.script || '',
    routes: worker.routes || [],
  }
}

async function deleteWorker(worker: Worker) {
  const accountId = accountStore.currentAccount?.accountId
  if (!accountId) {
    toast.error('账户信息缺失')
    return
  }

  if (!confirm(`确定要删除 Worker "${worker.id}" 吗？`)) return

  try {
    await cloudflareApi.deleteWorker(accountId, worker.id)
    logHistory.worker('删除 Worker', `Worker: ${worker.id}`)
    toast.success('Worker 已删除')
    loadWorkers()
  } catch (error) {
    console.error('Failed to delete worker:', error)
    toast.error('删除失败')
  }
}

async function saveWorker() {
  const accountId = accountStore.currentAccount?.accountId
  if (!accountId) {
    toast.error('请先添加账户')
    return
  }

  if (!workerForm.value.name || !workerForm.value.script) {
    toast.warning('请填写所有必填字段')
    return
  }

  try {
    if (editingWorker.value) {
      await cloudflareApi.uploadWorker(accountId, workerForm.value.name, workerForm.value.script)
      logHistory.worker('更新 Worker', `Worker: ${workerForm.value.name}`)
      toast.success('Worker 已更新')
    } else {
      await cloudflareApi.uploadWorker(accountId, workerForm.value.name, workerForm.value.script)
      logHistory.worker('创建 Worker', `Worker: ${workerForm.value.name}`)
      toast.success('Worker 已创建')
      
      // Add routes if any
      for (const route of workerForm.value.routes.filter(r => r)) {
        try {
          await cloudflareApi.createWorkerRoute(route, workerForm.value.name)
        } catch (err) {
          console.error('Failed to create route:', err)
        }
      }
    }

    closeModal()
    loadWorkers()
  } catch (error: any) {
    console.error('Failed to save worker:', error)
    toast.error(error.message || '保存失败')
  }
}

function closeModal() {
  showCreateModal.value = false
  editingWorker.value = null
  workerForm.value = {
    name: '',
    script: '',
    routes: [],
  }
}

function formatDate(dateString: string): string {
  if (!dateString) return '-'
  return new Date(dateString).toLocaleString('zh-CN')
}

onMounted(() => {
  loadWorkers()
})
</script>
