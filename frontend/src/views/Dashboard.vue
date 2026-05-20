<template>
  <div class="animate-in space-y-8 pb-12">
    <!-- Greeting Section -->
    <header class="flex flex-col md:flex-row justify-between items-start md:items-end gap-6 px-1">
      <div class="flex items-center gap-5">
        <div class="w-16 h-16 rounded-2xl bg-primary/10 flex items-center justify-center text-primary border border-primary/20 shadow-[0_0_20px_rgba(var(--primary-rgb),0.1)] shrink-0 relative">
          <component :is="SparklesOutline" class="w-8 h-8" />
          <div class="absolute -right-1 -top-1 w-4 h-4 bg-emerald-500 rounded-full border-4 border-background"></div>
        </div>
        <div>
          <div class="text-[10px] text-muted-foreground font-black uppercase tracking-[0.2em] mb-1 opacity-60">Management Console</div>
          <h1 class="text-3xl font-black text-foreground tracking-tighter">
            {{ t('dashboard.overview') }}
          </h1>
        </div>
      </div>
      <div class="flex gap-3 w-full md:w-auto">
        <IslandButton variant="secondary" class="flex-1 md:flex-none border-border/40" @click="router.push('/history')">
          <template #icon><component :is="TimeOutline" class="w-4 h-4" /></template>
          {{ t('dashboard.auditLog') }}
        </IslandButton>
        <IslandButton class="flex-1 md:flex-none shadow-lg shadow-primary/20" @click="loadDashboardData" :loading="loading">
          <template #icon><component :is="SwapHorizontalOutline" class="w-4 h-4" /></template>
          {{ t('dashboard.refresh') }}
        </IslandButton>
      </div>
    </header>

    <!-- Account Overview Card -->
    <section class="grid grid-cols-1 xl:grid-cols-3 gap-8">
      <!-- Welcome Banner -->
      <div class="xl:col-span-2 relative group">
        <div class="absolute -inset-0.5 bg-gradient-to-r from-primary/30 to-accent/30 opacity-10 blur-2xl group-hover:opacity-20 transition duration-1000"></div>
        <GlassCard class="h-full relative overflow-hidden border-primary/10" :padding="8">
          <div class="absolute -right-10 -bottom-10 w-64 h-64 bg-primary/5 rounded-full blur-3xl"></div>
          
          <div class="relative flex flex-col md:flex-row gap-8 items-center h-full">
            <div class="flex-1 space-y-4 text-center md:text-left">
              <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-primary/10 text-primary text-[10px] font-black uppercase tracking-widest">
                <component :is="CloudOutline" class="w-3 h-3" />
                Network Status: Operational
              </div>
              <h3 class="text-2xl font-black tracking-tight">{{ t('dashboard.realtimeOverview') }}</h3>
              <p class="text-xs text-muted-foreground leading-relaxed max-w-xl font-medium" v-html="t('dashboard.accountStatus', { zones: zones.length, workers: workersCount })">
              </p>
              <div class="flex flex-wrap gap-2 justify-center md:justify-start pt-2">
                <GlassBadge variant="info">{{ t('dashboard.wafReady') }}</GlassBadge>
                <GlassBadge variant="success">{{ t('dashboard.cacheSynced') }}</GlassBadge>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-4 shrink-0 w-full md:w-auto">
              <div class="text-center p-6 rounded-3xl bg-card backdrop-blur-[20px] border border-border/40 hover:border-primary/30 transition-all shadow-sm">
                <div class="text-3xl font-black text-primary tracking-tighter">{{ zones.length }}</div>
                <div class="text-[9px] text-muted-foreground font-black uppercase tracking-widest mt-1">{{ t('dashboard.activeZones') }}</div>
              </div>
              <div class="text-center p-6 rounded-3xl bg-card backdrop-blur-[20px] border border-border/40 hover:border-accent/30 transition-all shadow-sm">
                <div class="text-3xl font-black text-accent tracking-tighter">{{ workersCount }}</div>
                <div class="text-[9px] text-muted-foreground font-black uppercase tracking-widest mt-1">{{ t('dashboard.workers') }}</div>
              </div>
            </div>
          </div>
        </GlassCard>
      </div>

      <!-- Quick Metrics -->
      <div class="space-y-4">
        <GlassCard :padding="6" class="flex items-center gap-4 border-emerald-500/10 !bg-card !backdrop-blur-[20px]">
          <div class="w-12 h-12 rounded-2xl bg-emerald-500/10 flex items-center justify-center text-emerald-500 shadow-inner">
            <component :is="ShieldOutline" class="w-6 h-6" />
          </div>
          <div>
            <p class="text-[10px] font-black text-muted-foreground uppercase tracking-widest">{{ t('dashboard.securityLevel') }}</p>
            <p class="text-lg font-bold">{{ t('dashboard.securityLevelMax') }}</p>
          </div>
        </GlassCard>

        <GlassCard :padding="6" class="flex items-center gap-4 !bg-card !backdrop-blur-[20px]">
          <div class="w-12 h-12 rounded-2xl bg-primary/10 flex items-center justify-center text-primary shadow-inner">
            <component :is="BuildOutline" class="w-6 h-6" />
          </div>
          <div>
            <p class="text-[10px] font-black text-muted-foreground uppercase tracking-widest">{{ t('dashboard.totalRecords') }}</p>
            <p class="text-lg font-bold">{{ t('dashboard.recordsCount', { count: totalDnsRecords }) }}</p>
          </div>
        </GlassCard>

        <GlassCard :padding="6" class="flex items-center gap-4 border-accent/10 !bg-card !backdrop-blur-[20px]">
          <div class="w-12 h-12 rounded-2xl bg-accent/10 flex items-center justify-center text-accent shadow-inner">
            <component :is="GlobeOutline" class="w-6 h-6" />
          </div>
          <div>
            <p class="text-[10px] font-black text-muted-foreground uppercase tracking-widest">{{ t('dashboard.edgeCoverage') }}</p>
            <p class="text-lg font-bold">{{ t('dashboard.datacenters') }}</p>
          </div>
        </GlassCard>
      </div>
    </section>

    <!-- Main Content Area -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <!-- Traffic Chart & Quick Actions -->
      <div class="lg:col-span-2 space-y-8">
        <GlassCard :padding="8">
          <div class="flex items-center justify-between mb-8">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary border border-primary/20">
                <component :is="AnalyticsOutline" class="w-5 h-5" />
              </div>
              <div>
                <h3 class="text-lg font-black tracking-tight">{{ t('dashboard.trafficTrend') }}</h3>
                <p class="text-[10px] text-muted-foreground font-bold uppercase tracking-widest">Global Edge Traffic</p>
              </div>
            </div>
          </div>
          
          <div class="min-h-[400px]">
            <TrafficChart :data="analyticsData" v-if="analyticsData.length > 0" />
            <div v-else class="h-[400px] flex flex-col items-center justify-center bg-card backdrop-blur-[20px] rounded-3xl border border-dashed border-border/50 text-center p-8 space-y-4 shadow-sm">
              <div class="w-16 h-16 rounded-2xl bg-foreground/5 flex items-center justify-center text-muted-foreground opacity-30">
                <component :is="AnalyticsOutline" class="w-8 h-8" />
              </div>
              <div class="space-y-1">
                <p class="text-sm font-bold text-muted-foreground">{{ t('dashboard.noData') }}</p>
                <p class="text-[10px] text-muted-foreground/60 uppercase font-bold tracking-widest">Analytics not available for this account</p>
              </div>
            </div>
          </div>
        </GlassCard>

        <!-- Quick Actions Grid -->
        <div>
          <div class="flex items-center gap-3 mb-6 px-1">
            <div class="w-8 h-8 rounded-lg bg-accent/10 flex items-center justify-center text-accent">
              <component :is="FlashOutline" class="w-4 h-4" />
            </div>
            <h3 class="text-lg font-black tracking-tight">{{ t('dashboard.quickActions') }}</h3>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <GlassCard 
              v-for="action in quickActions" 
              :key="action.path"
              class="group cursor-pointer hover:border-primary/30 transition-all !bg-card !backdrop-blur-[20px]"
              :padding="5"
              @click="router.push(action.path)"
            >
              <div class="flex items-center gap-5">
                <div class="w-14 h-14 rounded-2xl bg-foreground/5 flex items-center justify-center text-muted-foreground group-hover:bg-primary/10 group-hover:text-primary transition-all">
                  <component :is="action.icon" class="w-7 h-7" />
                </div>
                <div>
                  <h4 class="font-black text-foreground group-hover:text-primary transition-colors">{{ action.title }}</h4>
                  <p class="text-xs text-muted-foreground leading-snug mt-0.5 line-clamp-1 italic">{{ action.desc }}</p>
                </div>
              </div>
            </GlassCard>
          </div>
        </div>
      </div>

      <!-- Side Information -->
      <div class="space-y-8">
        <!-- Managed Domains Mini -->
        <GlassCard :padding="6" class="!bg-card !backdrop-blur-[20px]">
          <div class="flex items-center justify-between mb-6">
            <h3 class="text-sm font-black uppercase tracking-widest text-foreground/80 flex items-center gap-2">
              <component :is="GlobeOutline" class="w-4 h-4 text-primary" />
              {{ t('dashboard.activeZones') }}
            </h3>
            <button @click="router.push('/zones')" class="text-[10px] font-black text-primary uppercase tracking-widest hover:underline">{{ t('dashboard.manageAll') }}</button>
          </div>
          <div class="space-y-3">
            <div 
              v-for="zone in zones.slice(0, 5)" 
              :key="zone.id" 
              class="flex items-center justify-between p-4 hover:bg-primary/5 transition-all cursor-pointer group rounded-2xl border border-border/10 hover:border-primary/20"
              @click="router.push('/dns')"
            >
              <div class="flex items-center gap-3 min-w-0">
                <div class="w-9 h-9 rounded-xl bg-foreground/5 flex items-center justify-center border border-border/30 shrink-0 group-hover:border-primary/40 transition-colors">
                  <component :is="GlobeOutline" class="w-5 h-5 text-muted-foreground group-hover:text-primary transition-colors" />
                </div>
                <div class="min-w-0">
                  <div class="font-bold text-sm truncate text-foreground tracking-tight group-hover:text-primary transition-colors">{{ zone.name }}</div>
                  <div class="text-[9px] text-muted-foreground font-black uppercase tracking-widest opacity-60">{{ zone.plan?.name || 'Free Plan' }}</div>
                </div>
              </div>
              <div class="flex items-center gap-2">
                <div v-if="zone.status === 'active'" class="w-1.5 h-1.5 rounded-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]"></div>
                <span class="text-[10px] font-black uppercase tracking-tighter">{{ zone.status }}</span>
              </div>
            </div>
          </div>
        </GlassCard>

        <!-- Audit Log Mini -->
        <GlassCard :padding="6" class="!bg-card !backdrop-blur-[20px]">
          <h3 class="text-sm font-black uppercase tracking-widest text-foreground/80 flex items-center gap-2 mb-6">
            <component :is="TimeOutline" class="w-4 h-4 text-accent" />
            {{ t('dashboard.recentActivity') }}
          </h3>
          <div class="space-y-4 px-1">
            <div v-if="recentHistory.length > 0" class="space-y-4">
              <div 
                v-for="item in recentHistory" 
                :key="item.id"
                class="flex gap-4 group"
              >
                <div class="relative flex flex-col items-center shrink-0">
                  <div :class="['w-2 h-2 rounded-full mt-1.5 z-10', item.status === 'success' ? 'bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.6)]' : 'bg-red-500']"></div>
                  <div class="w-0.5 h-full bg-border/20 absolute top-2 group-last:hidden"></div>
                </div>
                <div class="pb-4 flex-1 min-w-0">
                  <div class="flex justify-between items-start">
                    <p class="text-xs font-bold text-foreground truncate group-hover:text-primary transition-colors">{{ item.action }}</p>
                    <time class="text-[9px] text-muted-foreground font-black whitespace-nowrap ml-2 opacity-60">{{ formatRelativeTime(item.timestamp) }}</time>
                  </div>
                  <p class="text-[10px] text-muted-foreground mt-1 leading-relaxed line-clamp-2 italic">{{ item.description }}</p>
                </div>
              </div>
            </div>
            <div v-else class="py-12 text-center space-y-3 opacity-30">
              <component :is="TimeOutline" class="w-8 h-8 mx-auto" />
              <p class="text-[10px] font-black uppercase tracking-widest">{{ t('dashboard.noActivity') }}</p>
            </div>
          </div>
        </GlassCard>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { cloudflareApi, type Zone, type TimeseriesPoint } from '@/api'
import { useAccountStore } from '@/stores/account'
import { historyLogger, type HistoryItem } from '@/utils/history'
import GlassCard from '@/components/ui/GlassCard.vue'
import IslandButton from '@/components/ui/IslandButton.vue'
import GlassBadge from '@/components/ui/GlassBadge.vue'
import TrafficChart from '@/components/dashboard/TrafficChart.vue'
import {
  SparklesOutline,
  GlobeOutline,
  BuildOutline,
  SettingsOutline,
  RocketOutline,
  ShieldOutline,
  CloudOutline,
  ChevronForwardOutline,
  AnalyticsOutline,
  TimeOutline,
  SwapHorizontalOutline,
  FlashOutline
} from '@vicons/ionicons5'

const router = useRouter()
const { t, locale } = useI18n()
const accountStore = useAccountStore()
const loading = ref(false)
const zones = ref<Zone[]>([])
const totalDnsRecords = ref(0)
const workersCount = ref(0)
const analyticsData = ref<TimeseriesPoint[]>([])
const recentHistory = ref<HistoryItem[]>([])

const quickActions = computed(() => [
  { title: t('common.optimize'), desc: locale.value === 'zh-CN' ? '全自动化边缘配置优化方案' : 'Automated edge configuration optimization', path: '/optimize', icon: FlashOutline },
  { title: t('common.dns'), desc: locale.value === 'zh-CN' ? '高性能域名解析与负载均衡' : 'High-performance DNS and load balancing', path: '/dns', icon: BuildOutline },
  { title: t('common.workers'), desc: locale.value === 'zh-CN' ? 'Serverless 边缘计算开发与分发' : 'Serverless edge computing development', path: '/workers', icon: SettingsOutline },
  { title: t('common.firewall'), desc: locale.value === 'zh-CN' ? 'WAF 拦截与 DDoS 防护设置' : 'WAF filtering and DDoS protection', path: '/firewall', icon: ShieldOutline },
])

async function loadDashboardData() {
  if (!accountStore.currentAccount) return

  loading.value = true
  recentHistory.value = historyLogger.getAll().slice(0, 5)

  try {
    zones.value = await cloudflareApi.getZones()

    if (zones.value.length > 0) {
      try {
        const analytics = await cloudflareApi.getAnalytics(zones.value[0].id, '24h')
        analyticsData.value = analytics.timeseries || []
      } catch (e) { /* silent */ }
    }

    // Records count
    let dnsTotal = 0
    for (const zone of zones.value.slice(0, 3)) {
      try {
        const records = await cloudflareApi.getDnsRecords(zone.id)
        dnsTotal += records.length
      } catch(e){}
    }
    totalDnsRecords.value = dnsTotal

    // Workers
    try {
      const workers = await cloudflareApi.listWorkers(accountStore.currentAccount.accountId)
      workersCount.value = workers?.length || 0
    } catch {
      workersCount.value = 0
    }
  } catch (error: any) {
    if (!error.silent) console.error('Dashboard loading failed')
  } finally {
    loading.value = false
  }
}

function formatRelativeTime(timestamp: string) {
  const date = new Date(timestamp)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const isZH = locale.value === 'zh-CN'

  if (diff < 60000) return isZH ? '刚刚' : 'just now'
  if (diff < 3600000) return `${Math.floor(diff / 60000)}m ${isZH ? '前' : 'ago'}`
  if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ${isZH ? '前' : 'ago'}`
  return date.toLocaleDateString(isZH ? 'zh-CN' : 'en-US', { month: 'short', day: 'numeric' })
}

onMounted(() => loadDashboardData())
watch(() => accountStore.currentAccount?.id, (newId) => {
  if (newId) loadDashboardData()
  else {
    zones.value = []; totalDnsRecords.value = 0; workersCount.value = 0
    analyticsData.value = []; recentHistory.value = []
  }
})
</script>
