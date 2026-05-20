<template>
  <div class="animate-in space-y-8">
    <!-- Header -->
    <header class="flex justify-between items-center px-1">
      <div>
        <div class="text-[10px] text-muted-foreground font-black uppercase tracking-[0.2em] mb-1 opacity-60">System Logs</div>
        <h1 class="text-3xl font-black text-foreground tracking-tighter">{{ t('history.title') }}</h1>
        <p class="text-xs text-muted-foreground font-bold mt-1 uppercase tracking-widest">Audit Trails & Activity Logs</p>
      </div>
      <IslandButton variant="secondary" class="border-red-500/20 text-red-500 hover:bg-red-500/5" @click="clearHistory">
        <template #icon><component :is="TrashOutline" class="w-4 h-4" /></template>
        {{ t('history.clear') }}
      </IslandButton>
    </header>

    <!-- Filters Section -->
    <GlassCard class="border-primary/5" :padding="6">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="space-y-2">
          <label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/60 ml-1">{{ t('history.opType') }}</label>
          <div class="relative group">
            <select
              v-model="filters.type"
              class="w-full bg-foreground/5 border border-border/40 rounded-2xl px-4 py-3 text-sm focus:ring-2 focus:ring-primary/30 outline-none appearance-none font-bold"
            >
              <option value="">{{ t('history.allTypes') }}</option>
              <option value="dns">{{ t('history.typeDns') }}</option>
              <option value="firewall">{{ t('history.typeFirewall') }}</option>
              <option value="ssl">{{ t('history.typeSsl') }}</option>
              <option value="cache">{{ t('history.typeCache') }}</option>
              <option value="worker">{{ t('history.typeWorker') }}</option>
            </select>
            <component :is="ChevronDownOutline" class="absolute right-4 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground pointer-events-none group-hover:text-primary transition-colors" />
          </div>
        </div>
        
        <div class="space-y-2">
          <label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/60 ml-1">{{ t('history.timeSpan') }}</label>
          <div class="relative group">
            <select
              v-model="filters.timeRange"
              @change="loadHistory"
              class="w-full bg-foreground/5 border border-border/40 rounded-2xl px-4 py-3 text-sm focus:ring-2 focus:ring-primary/30 outline-none appearance-none font-bold"
            >
              <option value="24h">{{ t('history.time24h') }}</option>
              <option value="7d">{{ t('history.time7d') }}</option>
              <option value="30d">{{ t('history.time30d') }}</option>
              <option value="all">{{ t('history.timeAll') }}</option>
            </select>
            <component :is="ChevronDownOutline" class="absolute right-4 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground pointer-events-none group-hover:text-primary transition-colors" />
          </div>
        </div>
        
        <div class="space-y-2">
          <label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/60 ml-1">{{ t('history.status') }}</label>
          <div class="relative group">
            <select
              v-model="filters.status"
              class="w-full bg-foreground/5 border border-border/40 rounded-2xl px-4 py-3 text-sm focus:ring-2 focus:ring-primary/30 outline-none appearance-none font-bold"
            >
              <option value="">{{ t('history.allStatus') }}</option>
              <option value="success">{{ t('history.success') }}</option>
              <option value="error">{{ t('history.failed') }}</option>
            </select>
            <component :is="ChevronDownOutline" class="absolute right-4 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground pointer-events-none group-hover:text-primary transition-colors" />
          </div>
        </div>
      </div>
    </GlassCard>

    <!-- History List -->
    <div v-if="filteredHistory.length > 0" class="space-y-4">
      <GlassCard 
        v-for="item in filteredHistory" 
        :key="item.id"
        padding="0"
        class="overflow-hidden hover:border-primary/20 transition-all border-border/30 group"
      >
        <div class="flex items-stretch min-h-[100px]">
          <div :class="['w-2 shrink-0 transition-opacity group-hover:opacity-100', item.status === 'success' ? 'bg-emerald-500/40 opacity-60' : 'bg-red-500/40 opacity-60']"></div>
          
          <div class="flex-1 p-6 flex flex-col md:flex-row justify-between md:items-center gap-6">
            <div class="flex items-start gap-6">
              <div :class="['w-14 h-14 rounded-2xl flex items-center justify-center shrink-0 border transition-all group-hover:scale-105', 
                item.status === 'success' ? 'bg-emerald-500/5 text-emerald-500 border-emerald-500/10' : 'bg-red-500/5 text-red-500 border-red-500/10']">
                <component :is="getIcon(item.type)" class="w-7 h-7" />
              </div>
              
              <div class="space-y-1.5">
                <div class="flex items-center gap-3">
                  <h3 class="text-base font-black tracking-tight text-foreground">{{ item.action }}</h3>
                  <GlassBadge :variant="item.status === 'success' ? 'success' : 'error'">
                    {{ item.status === 'success' ? 'SUCCESS' : 'FAILED' }}
                  </GlassBadge>
                </div>
                <p class="text-xs text-muted-foreground leading-relaxed font-medium max-w-2xl">{{ item.description }}</p>
                
                <div class="flex items-center gap-4 pt-1 opacity-60">
                  <div class="flex items-center gap-1.5 text-[10px] font-black uppercase tracking-tighter text-muted-foreground">
                    <component :is="TimeOutline" class="w-3.5 h-3.5" />
                    {{ formatDate(item.timestamp) }}
                  </div>
                  <div v-if="item.user" class="flex items-center gap-1.5 text-[10px] font-black uppercase tracking-tighter text-muted-foreground">
                    <component :is="PersonOutline" class="w-3.5 h-3.5" />
                    {{ item.user }}
                  </div>
                </div>
              </div>
            </div>

            <div class="flex items-center gap-2 md:pl-6 md:border-l border-border/30">
              <IslandButton size="small" variant="secondary" class="border-none bg-foreground/5 hover:bg-foreground/10" @click="copyDetails(item)">
                {{ t('history.copy') }}
              </IslandButton>
            </div>
          </div>
        </div>
      </GlassCard>
    </div>

    <!-- Empty State -->
    <GlassCard v-else class="py-24 flex flex-col items-center justify-center space-y-6 text-center border-dashed border-2">
      <div class="w-20 h-20 rounded-3xl bg-foreground/[0.03] flex items-center justify-center text-muted-foreground/30">
        <component :is="TimeOutline" class="w-10 h-10" />
      </div>
      <div class="space-y-1">
        <h3 class="text-xl font-black tracking-tight">{{ t('history.emptyTitle') }}</h3>
        <p class="text-[10px] text-muted-foreground font-bold uppercase tracking-widest font-mono">No activity found within selected range</p>
      </div>
    </GlassCard>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, type Component } from 'vue'
import { useI18n } from 'vue-i18n'
import { 
  GlobeOutline, 
  ShieldOutline, 
  LockClosedOutline, 
  SpeedometerOutline, 
  SettingsOutline, 
  TimeOutline, 
  PersonOutline, 
  DocumentTextOutline,
  TrashOutline,
  ChevronDownOutline
} from '@vicons/ionicons5'
import { historyLogger, type HistoryItem } from '@/utils/history'
import { toast } from '@/utils/toast'
import GlassCard from '@/components/ui/GlassCard.vue'
import IslandButton from '@/components/ui/IslandButton.vue'
import GlassBadge from '@/components/ui/GlassBadge.vue'

const { t, locale } = useI18n()

const filters = ref({
  type: '',
  timeRange: '7d' as '24h' | '7d' | '30d' | 'all',
  status: '',
})

const history = ref<HistoryItem[]>([])

const filteredHistory = computed(() => {
  return history.value.filter(item => {
    if (filters.value.type && item.type !== filters.value.type) return false
    if (filters.value.status && item.status !== filters.value.status) return false
    return true
  })
})

function loadHistory() {
  history.value = historyLogger.getByTimeRange(filters.value.timeRange)
}

function clearHistory() {
  if (!confirm(t('history.clearConfirm'))) return
  historyLogger.clear()
  loadHistory()
  toast.success(t('history.clearSuccess'))
}

function copyDetails(item: HistoryItem) {
  navigator.clipboard.writeText(`${item.action}: ${item.description} (${item.timestamp})`)
  toast.success(t('history.copySuccess'))
}

function getIcon(type: string): Component {
  const icons: Record<string, Component> = {
    dns: GlobeOutline,
    firewall: ShieldOutline,
    ssl: LockClosedOutline,
    cache: SpeedometerOutline,
    worker: SettingsOutline,
  }
  return icons[type] || DocumentTextOutline
}

function formatDate(dateString: string): string {
  const date = new Date(dateString)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  if (diff < 60000) return t('history.justNow')
  if (diff < 3600000) return t('history.minutesAgo', { count: Math.floor(diff / 60000) })
  if (diff < 86400000) return t('history.hoursAgo', { count: Math.floor(diff / 3600000) })
  if (diff < 604800000) return t('history.daysAgo', { count: Math.floor(diff / 86400000) })
  return date.toLocaleString(locale.value === 'zh-CN' ? 'zh-CN' : 'en-US')
}

onMounted(() => loadHistory())
</script>
