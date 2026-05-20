<template>
  <div class="animate-in space-y-6">
    <!-- Header -->
    <header class="flex justify-between items-end px-1">
      <div>
        <h1 class="text-3xl font-bold text-foreground">{{ t('workers.title') }}</h1>
        <p class="text-sm text-muted-foreground mt-1 font-medium">{{ t('workers.subtitle') }}</p>
      </div>
      <IslandButton @click="showCreateModal = true">
        <template #icon><component :is="AddOutline" class="w-4 h-4" /></template>
        {{ t('workers.createWorker') }}
      </IslandButton>
    </header>

    <!-- Loading State -->
    <div v-if="loading" class="py-24 flex flex-col items-center justify-center">
      <div class="w-10 h-10 border-4 border-primary/20 border-t-primary rounded-full animate-spin"></div>
      <p class="mt-4 text-sm text-muted-foreground font-medium">{{ t('workers.loading') }}</p>
    </div>

    <!-- Workers List -->
    <div v-else-if="workers.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <GlassCard 
        v-for="worker in workers" 
        :key="worker.id"
        class="group hover:scale-[1.02] active:scale-[0.98] transition-all duration-300"
        :padding="0"
      >
        <div class="p-6 h-full flex flex-col">
          <!-- Worker Header -->
          <div class="flex items-start justify-between mb-4">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary">
                <component :is="FlashOutline" class="w-6 h-6" />
              </div>
              <div>
                <h3 class="font-bold text-foreground leading-none">{{ worker.id }}</h3>
                <div class="text-[10px] text-muted-foreground mt-1.5 flex items-center gap-1">
                  <div class="w-1 h-1 rounded-full bg-border"></div>
                  {{ formatDate(worker.modified_on) }}
                </div>
              </div>
            </div>
            <GlassBadge variant="success">{{ t('workers.statusDeployed') }}</GlassBadge>
          </div>

          <!-- Routes Summary -->
          <div class="flex-1 space-y-3 mb-6">
            <div v-if="worker.routes && worker.routes.length > 0" class="space-y-1.5">
              <div class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">{{ t('workers.boundRoutes') }}</div>
              <div class="flex flex-wrap gap-1.5">
                <div 
                  v-for="route in worker.routes" 
                  :key="route"
                  class="px-2 py-0.5 bg-foreground/5 rounded-md text-[10px] text-foreground/70 font-mono border border-border/50"
                >
                  {{ route }}
                </div>
              </div>
            </div>
            <div v-else class="text-[10px] text-muted-foreground italic bg-foreground/[0.02] p-3 rounded-lg border border-dashed border-border/50">
              {{ t('workers.noRoutes') }}
            </div>
          </div>

          <!-- Actions -->
          <div class="flex gap-2 pt-4 border-t border-border/30">
            <IslandButton variant="ghost" size="small" class="flex-1" @click="editWorker(worker)">
              {{ t('workers.editScript') }}
            </IslandButton>
            <IslandButton variant="danger" size="small" @click="deleteWorker(worker)">
              <template #icon><component :is="TrashOutline" class="w-4 h-4" /></template>
            </IslandButton>
          </div>
        </div>
      </GlassCard>
    </div>

    <!-- Empty State -->
    <div v-else class="py-32 flex flex-col items-center text-center">
      <GlassCard class="p-8 flex flex-col items-center">
        <div class="w-16 h-16 rounded-2xl bg-muted flex items-center justify-center text-muted-foreground mb-4">
          <component :is="SettingsOutline" class="w-8 h-8" />
        </div>
        <h3 class="text-xl font-bold">{{ t('workers.noWorkers') }}</h3>
        <p class="text-sm text-muted-foreground mt-2 max-w-xs">
          {{ t('workers.noWorkersDesc') }}
        </p>
        <IslandButton class="mt-6" @click="showCreateModal = true">
          {{ t('workers.getStarted') }}
        </IslandButton>
      </GlassCard>
    </div>

    <!-- Create/Edit Modal -->
    <n-modal v-model:show="showModal" transform-origin="center">
      <GlassCard class="w-[900px] max-w-[95vw]" :padding="0">
        <div class="p-6 border-b border-border/50 flex justify-between items-center">
          <h2 class="text-xl font-bold">{{ editingWorker ? t('workers.editTitle') : t('workers.createTitle') }}</h2>
          <button @click="closeModal" class="text-muted-foreground hover:text-foreground">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
          </button>
        </div>
        
        <div class="p-8 grid grid-cols-1 lg:grid-cols-3 gap-8 overflow-y-auto max-h-[75vh] custom-scrollbar">
          <div class="lg:col-span-2 space-y-4">
            <div class="space-y-2">
              <label class="text-xs font-bold ml-1">{{ t('workers.scriptContent') }}</label>
              <div class="relative group">
                <div class="absolute -inset-0.5 bg-gradient-to-br from-primary/20 to-accent/20 rounded-2xl blur opacity-0 group-focus-within:opacity-100 transition duration-500"></div>
                <textarea
                  v-model="workerForm.script"
                  class="relative w-full h-[400px] bg-slate-950 text-slate-200 border border-border/50 rounded-xl p-6 font-mono text-xs focus:outline-none leading-relaxed"
                  placeholder="export default { ... }"
                ></textarea>
              </div>
            </div>
          </div>

          <div class="space-y-6">
            <div class="space-y-2">
              <label class="text-xs font-bold ml-1">{{ t('workers.workerName') }}</label>
              <input
                v-model="workerForm.name"
                :disabled="!!editingWorker"
                class="w-full bg-foreground/5 border border-border/50 rounded-xl px-4 py-3 text-sm focus:ring-2 focus:ring-primary/20 outline-none"
                placeholder="my-awesome-worker"
              />
            </div>

            <div class="space-y-3">
              <div class="flex justify-between items-center px-1">
                <label class="text-xs font-bold">{{ t('workers.routeConfig') }}</label>
                <button @click="workerForm.routes.push('')" class="text-primary text-[10px] font-bold hover:underline">+ {{ t('workers.addRoute') }}</button>
              </div>
              <div class="space-y-2">
                <div v-for="(route, index) in workerForm.routes" :key="index" class="flex gap-2">
                  <input
                    v-model="workerForm.routes[index]"
                    placeholder="example.com/*"
                    class="flex-1 bg-foreground/5 border border-border/50 rounded-lg px-3 py-2 text-xs outline-none focus:ring-1 focus:ring-primary/30"
                  />
                  <button @click="workerForm.routes.splice(index, 1)" class="p-2 text-danger/50 hover:text-danger">
                    <component :is="TrashOutline" class="w-4 h-4" />
                  </button>
                </div>
              </div>
            </div>

            <div class="p-4 bg-primary/5 rounded-2xl border border-primary/10">
              <div class="flex items-center gap-2 text-primary mb-2">
                <component :is="CheckmarkCircleOutline" class="w-4 h-4" />
                <span class="text-[10px] font-bold uppercase tracking-widest">{{ t('workers.deployTipTitle') }}</span>
              </div>
              <p class="text-[10px] text-muted-foreground leading-relaxed">
                {{ t('workers.deployTipDesc') }}
              </p>
            </div>
          </div>
        </div>

        <div class="p-6 border-t border-border/50 flex justify-end gap-3">
          <IslandButton variant="secondary" @click="closeModal">{{ t('common.cancel') }}</IslandButton>
          <IslandButton :loading="saving" @click="saveWorker">
            {{ editingWorker ? t('workers.updateAndDeploy') : t('workers.publishNow') }}
          </IslandButton>
        </div>
      </GlassCard>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NModal } from 'naive-ui'
import { 
  SettingsOutline, 
  CheckmarkCircleOutline, 
  AddOutline, 
  FlashOutline, 
  TrashOutline 
} from '@vicons/ionicons5'
import { useAccountStore } from '@/stores/account'
import { cloudflareApi } from '@/api'
import { toast } from '@/utils/toast'
import { logHistory } from '@/utils/history'
import GlassCard from '@/components/ui/GlassCard.vue'
import IslandButton from '@/components/ui/IslandButton.vue'
import GlassBadge from '@/components/ui/GlassBadge.vue'

const { t, locale } = useI18n()

interface Worker {
  id: string
  script?: string
  created_on: string
  modified_on: string
  routes?: string[]
}

const accountStore = useAccountStore()
const loading = ref(false)
const saving = ref(false)
const workers = ref<Worker[]>([])
const showCreateModal = ref(false)
const editingWorker = ref<Worker | null>(null)

const showModal = computed({
  get: () => showCreateModal.value || !!editingWorker.value,
  set: (val) => { if(!val) closeModal() }
})

const workerForm = ref({
  name: '',
  script: '',
  routes: [] as string[],
})

async function loadWorkers() {
  const accountId = accountStore.currentAccount?.accountId
  if (!accountId) return

  loading.value = true
  try {
    workers.value = await cloudflareApi.listWorkers(accountId)
  } catch (error) {
    toast.error(t('workers.loadFailed'))
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
  if (!accountId || !confirm(t('workers.deleteConfirm', { name: worker.id }))) return

  try {
    await cloudflareApi.deleteWorker(accountId, worker.id)
    logHistory.worker('删除 Worker', `Worker: ${worker.id}`)
    toast.success(t('common.deleteSuccess'))
    loadWorkers()
  } catch (error) {
    toast.error(t('common.deleteFailed'))
  }
}

async function saveWorker() {
  const accountId = accountStore.currentAccount?.accountId
  if (!accountId) return

  if (!workerForm.value.name || !workerForm.value.script) {
    toast.warning(t('workers.invalidInput'))
    return
  }

  saving.value = true
  try {
    await cloudflareApi.uploadWorker(accountId, workerForm.value.name, workerForm.value.script)
    
    if (!editingWorker.value) {
      for (const route of workerForm.value.routes.filter(r => r)) {
        try { await cloudflareApi.createWorkerRoute(route, workerForm.value.name) } catch(e){}
      }
      logHistory.worker('创建 Worker', `Worker: ${workerForm.value.name}`)
      toast.success(t('workers.published'))
    } else {
      logHistory.worker('更新 Worker', `Worker: ${workerForm.value.name}`)
      toast.success(t('workers.updated'))
    }

    closeModal()
    loadWorkers()
  } catch (error: any) {
    toast.error(error.message || t('common.updateFailed'))
  } finally {
    saving.value = false
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
  return new Date(dateString).toLocaleDateString(locale.value, {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

onMounted(loadWorkers)
</script>
