<template>
  <div class="animate-in space-y-8 pb-12">
    <!-- World Map Section -->
    <WorldMap />

    <!-- Header -->
    <header class="flex flex-col md:flex-row justify-between items-start md:items-end gap-4 px-1">
      <div>
        <h1 class="text-3xl font-bold text-foreground tracking-tight">{{ t('analytics.title') }}</h1>
        <p class="text-sm text-muted-foreground mt-1 font-medium italic">
          {{ currentZone?.name || t('zones.notSelected') }} · {{ t('analytics.subtitle') }}
        </p>
      </div>
      <div class="flex gap-2">
        <GlassBadge variant="info">{{ t('analytics.realtime') }}</GlassBadge>
      </div>
    </header>

    <!-- Overview Stats -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <GlassCard :padding="6" class="relative overflow-hidden group">
        <div class="absolute -right-4 -bottom-4 w-24 h-24 bg-primary/5 rounded-full blur-2xl group-hover:bg-primary/10 transition-colors"></div>
        <p class="text-[10px] font-bold text-muted-foreground uppercase tracking-widest mb-2">{{ t('analytics.totalRequests') }}</p>
        <div class="text-3xl font-black tracking-tighter">{{ formatNumber(stats.totalRequests) }}</div>
        <p class="text-[10px] text-emerald-500 font-bold mt-2">{{ t('analytics.operational') }}</p>
      </GlassCard>

      <GlassCard :padding="6" class="relative overflow-hidden group border-secondary/20">
        <div class="absolute -right-4 -bottom-4 w-24 h-24 bg-secondary/5 rounded-full blur-2xl group-hover:bg-secondary/10 transition-colors"></div>
        <p class="text-[10px] font-bold text-muted-foreground uppercase tracking-widest mb-2">{{ t('analytics.totalBandwidth') }}</p>
        <div class="text-3xl font-black tracking-tighter">{{ formatBytes(stats.totalBandwidth) }}</div>
        <p class="text-[10px] text-primary font-bold mt-2">{{ t('analytics.cdnActive') }}</p>
      </GlassCard>

      <GlassCard :padding="6" class="relative overflow-hidden group border-danger/20">
        <div class="absolute -right-4 -bottom-4 w-24 h-24 bg-danger/5 rounded-full blur-2xl group-hover:bg-danger/10 transition-colors"></div>
        <p class="text-[10px] font-bold text-muted-foreground uppercase tracking-widest mb-2">{{ t('analytics.threats') }}</p>
        <div class="text-3xl font-black tracking-tighter text-danger">{{ stats.threats }}</div>
        <p class="text-[10px] text-muted-foreground font-bold mt-2">{{ t('analytics.wafFiltered') }}</p>
      </GlassCard>
    </div>

    <!-- Visualization Mockup -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
      <GlassCard :padding="8" class="space-y-6">
        <h3 class="text-sm font-bold uppercase tracking-widest text-muted-foreground">{{ t('analytics.httpStatus') }}</h3>
        <div class="space-y-6">
          <div v-for="item in httpStats" :key="item.label" class="space-y-2">
            <div class="flex justify-between text-xs font-bold">
              <span>{{ item.label }}</span>
              <span>{{ item.value }}%</span>
            </div>
            <div class="h-2 w-full bg-foreground/5 rounded-full overflow-hidden">
              <div 
                class="h-full rounded-full transition-all duration-1000" 
                :class="item.color"
                :style="{ width: loading ? '0%' : item.value + '%' }"
              ></div>
            </div>
          </div>
        </div>
      </GlassCard>

      <GlassCard :padding="8" class="flex flex-col items-center justify-center text-center space-y-6 border-dashed">
        <div class="w-20 h-20 rounded-full bg-foreground/5 flex items-center justify-center border border-border/50">
          <component :is="BarChartOutline" class="w-10 h-10 text-muted-foreground opacity-50" />
        </div>
        <div class="max-w-xs space-y-2">
          <h4 class="font-bold">{{ t('analytics.moreTitle') }}</h4>
          <p class="text-xs text-muted-foreground leading-relaxed">
            {{ t('analytics.moreDesc') }}
          </p>
        </div>
        <IslandButton variant="secondary" size="small" disabled>{{ t('analytics.fullReport') }}</IslandButton>
      </GlassCard>
    </div>

    <!-- History Log (Simplified for Analytics context) -->
    <GlassCard :padding="6">
      <h3 class="text-xs font-bold uppercase tracking-widest text-muted-foreground mb-4">{{ t('analytics.anomalySummary') }}</h3>
      <div class="text-xs text-muted-foreground text-center py-8 italic">
        {{ t('analytics.noAnomaly') }}
      </div>
    </GlassCard>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, inject, type Ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { BarChartOutline } from '@vicons/ionicons5'
import { cloudflareApi, type Zone } from '@/api'
import { useAccountStore } from '@/stores/account'
import { toast } from '@/utils/toast'
import TrafficChart from '@/components/dashboard/TrafficChart.vue'
import WorldMap from '@/components/analytics/WorldMap.vue'
import GlassCard from '@/components/ui/GlassCard.vue'
import GlassBadge from '@/components/ui/GlassBadge.vue'
import IslandButton from '@/components/ui/IslandButton.vue'

const { t } = useI18n()
const accountStore = useAccountStore()
const currentZone = inject<Ref<Zone | null>>('currentZone')
const loading = ref(true)

const stats = ref({
  totalRequests: 0,
  totalBandwidth: 0,
  threats: 0,
  cacheHitRate: 0
})

const httpStats = ref([
  { label: '2xx Success', value: 92, color: 'bg-emerald-500' },
  { label: '3xx Redirection', value: 5, color: 'bg-primary' },
  { label: '4xx Client Errors', value: 2, color: 'bg-amber-500' },
  { label: '5xx Server Errors', value: 1, color: 'bg-danger' }
])

function formatNumber(num: number) {
  return num.toLocaleString()
}

function formatBytes(bytes: number) {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

async function loadAnalytics() {
  if (!currentZone?.value?.id || !accountStore.currentAccount) {
    loading.value = false
    return
  }

  loading.value = true
  try {
    const data = await cloudflareApi.getAnalytics(currentZone.value.id, '24h')
    stats.value = {
      totalRequests: data.stats.totalRequests,
      totalBandwidth: data.stats.totalBandwidth,
      threats: data.stats.totalThreats,
      cacheHitRate: data.stats.cacheHitRate
    }
  } catch (error: any) {
    if (!error.silent) toast.error(t('analytics.syncFailed'))
  } finally {
    loading.value = false
  }
}

onMounted(() => loadAnalytics())
watch(() => currentZone?.value?.id, () => loadAnalytics())
</script>
