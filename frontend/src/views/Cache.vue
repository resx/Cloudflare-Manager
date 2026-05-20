<template>
  <div class="animate-in space-y-8 pb-12">
    <!-- Header -->
    <header class="flex flex-col md:flex-row justify-between items-start md:items-end gap-4 px-1">
      <div>
        <h1 class="text-3xl font-bold text-foreground tracking-tight">{{ t('cache.title') }}</h1>
        <p class="text-sm text-muted-foreground mt-1 font-medium italic">
          {{ currentZone?.name || t('zones.notSelected') }} · {{ t('cache.subtitle') }}
        </p>
      </div>
      <div class="flex gap-3">
        <IslandButton variant="danger" @click="handlePurgeAllCache" :loading="purging">
          <template #icon><component :is="TrashOutline" class="w-4 h-4" /></template>
          {{ t('cache.purgeAll') }}
        </IslandButton>
      </div>
    </header>

    <div class="grid grid-cols-1 xl:grid-cols-3 gap-8">
      <!-- Left: Settings & Purge -->
      <div class="xl:col-span-2 space-y-8">
        <!-- Quick Purge Actions -->
        <section class="space-y-4">
          <h3 class="text-xs font-bold uppercase tracking-widest text-muted-foreground px-1">{{ t('cache.quickPurge') }}</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <GlassCard 
              :padding="6" 
              class="group cursor-pointer hover:border-primary/40 transition-all active:scale-[0.98]"
              @click="showPurgeByURLModal = true"
            >
              <div class="flex items-center gap-4">
                <div class="w-12 h-12 rounded-2xl bg-primary/10 flex items-center justify-center text-primary group-hover:bg-primary group-hover:text-white transition-all">
                  <component :is="LinkOutline" class="w-6 h-6" />
                </div>
                <div>
                  <h4 class="font-bold text-foreground">{{ t('cache.purgeByURL') }}</h4>
                  <p class="text-xs text-muted-foreground">{{ t('cache.purgeByURLDesc') }}</p>
                </div>
              </div>
            </GlassCard>

            <GlassCard 
              :padding="6" 
              class="group cursor-pointer hover:border-secondary/40 transition-all active:scale-[0.98]"
              @click="showPurgeByTagModal = true"
            >
              <div class="flex items-center gap-4">
                <div class="w-12 h-12 rounded-2xl bg-secondary/10 flex items-center justify-center text-secondary group-hover:bg-secondary group-hover:text-white transition-all">
                  <component :is="PricetagOutline" class="w-6 h-6" />
                </div>
                <div>
                  <h4 class="font-bold text-foreground">{{ t('cache.purgeByTag') }}</h4>
                  <p class="text-xs text-muted-foreground">{{ t('cache.purgeByTagDesc') }}</p>
                </div>
              </div>
            </GlassCard>
          </div>
        </section>

        <!-- Core Settings -->
        <section class="space-y-4">
          <h3 class="text-xs font-bold uppercase tracking-widest text-muted-foreground px-1">{{ t('cache.cachePolicy') }}</h3>
          <div class="space-y-4">
            <!-- Cache Level -->
            <GlassCard :padding="6">
              <div class="flex flex-col md:flex-row gap-6 justify-between items-start md:items-center">
                <div class="space-y-1 max-w-md">
                  <h4 class="font-bold text-foreground flex items-center gap-2">
                    <component :is="LayersOutline" class="w-4 h-4 text-primary" />
                    {{ t('cache.cacheLevel') }}
                  </h4>
                  <p class="text-xs text-muted-foreground">{{ getCacheLevelDescription(cacheLevel) }}</p>
                </div>
                <select 
                  v-model="cacheLevel" 
                  class="w-full md:w-48 bg-foreground/5 border border-border/50 rounded-xl px-4 py-2.5 text-sm outline-none cursor-pointer focus:ring-2 focus:ring-primary/30 transition-all"
                  @change="e => handleCacheLevelChange((e.target as HTMLSelectElement).value)"
                >
                  <option v-for="opt in cacheLevelOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
                </select>
              </div>
            </GlassCard>

            <!-- Browser TTL -->
            <GlassCard :padding="6">
              <div class="flex flex-col md:flex-row gap-6 justify-between items-start md:items-center">
                <div class="space-y-1 max-w-md">
                  <h4 class="font-bold text-foreground flex items-center gap-2">
                    <component :is="TimeOutline" class="w-4 h-4 text-primary" />
                    {{ t('cache.browserTTL') }}
                  </h4>
                  <p class="text-xs text-muted-foreground">{{ t('cache.browserTTLDesc') }}</p>
                </div>
                <select 
                  v-model="browserCacheTTL" 
                  class="w-full md:w-48 bg-foreground/5 border border-border/50 rounded-xl px-4 py-2.5 text-sm outline-none cursor-pointer focus:ring-2 focus:ring-primary/30 transition-all"
                  @change="e => handleBrowserCacheTTLChange(Number((e.target as HTMLSelectElement).value))"
                >
                  <option v-for="opt in browserCacheTTLOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
                </select>
              </div>
            </GlassCard>

            <!-- Toggle Settings Grid -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <GlassCard :padding="5" class="flex items-center justify-between gap-4">
                <div class="space-y-0.5">
                  <h5 class="text-sm font-bold">{{ t('cache.devMode') }}</h5>
                  <p class="text-[10px] text-muted-foreground">{{ t('cache.devModeDesc') }}</p>
                </div>
                <n-switch v-model:value="developmentMode" :loading="updating" @update:value="handleDevelopmentModeChange" size="small" />
              </GlassCard>

              <GlassCard :padding="5" class="flex items-center justify-between gap-4">
                <div class="space-y-0.5">
                  <h5 class="text-sm font-bold">{{ t('cache.sortQuery') }}</h5>
                  <p class="text-[10px] text-muted-foreground">{{ t('cache.sortQueryDesc') }}</p>
                </div>
                <n-switch v-model:value="sortQueryString" :loading="updating" @update:value="handleSortQueryStringChange" size="small" />
              </GlassCard>
            </div>
          </div>
        </section>
      </div>

      <!-- Right: Stats & Info -->
      <div class="space-y-8">
        <h3 class="text-xs font-bold uppercase tracking-widest text-muted-foreground px-1">{{ t('cache.performance24h') }}</h3>
        <GlassCard :padding="8" class="border-primary/20 bg-primary/[0.02] relative overflow-hidden">
          <!-- Background Decoration -->
          <div class="absolute -right-8 -top-8 w-32 h-32 bg-primary/5 rounded-full blur-3xl"></div>
          
          <div class="relative space-y-8">
            <div class="text-center space-y-2">
              <span class="text-xs font-bold text-muted-foreground uppercase tracking-widest">{{ t('cache.avgHitRate') }}</span>
              <div class="text-5xl font-black text-foreground tracking-tighter">
                {{ cacheStats.hitRate }}<span class="text-2xl text-primary">%</span>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-4 pt-6 border-t border-border/30">
              <div class="space-y-1">
                <p class="text-[10px] font-bold text-muted-foreground uppercase">{{ t('analytics.totalRequests') }}</p>
                <p class="text-lg font-bold">{{ formatNumber(cacheStats.requests) }}</p>
              </div>
              <div class="space-y-1">
                <p class="text-[10px] font-bold text-muted-foreground uppercase">{{ t('cache.savings') }}</p>
                <p class="text-lg font-bold text-emerald-500">{{ formatNumber(cacheStats.cached) }}</p>
              </div>
            </div>

            <div class="p-4 bg-foreground/5 rounded-2xl flex items-start gap-3">
              <component :is="InformationCircleOutline" class="w-4 h-4 text-primary mt-0.5 shrink-0" />
              <p class="text-[10px] text-muted-foreground leading-relaxed">
                {{ t('cache.performanceTip') }}
              </p>
            </div>
          </div>
        </GlassCard>

        <div class="p-6 rounded-3xl border border-dashed border-border/50 space-y-3">
          <h4 class="text-xs font-bold uppercase text-muted-foreground">{{ t('cache.nodeStatus') }}</h4>
          <div class="flex items-center gap-2">
            <div class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></div>
            <span class="text-xs font-medium">{{ t('cache.centersSynced') }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Modals -->
    <n-modal v-model:show="showPurgeByURLModal">
      <GlassCard class="w-[500px] max-w-[95vw]" :padding="0">
        <div class="p-6 border-b border-border/50 flex justify-between items-center">
          <h2 class="text-xl font-bold">{{ t('cache.purgeByURL') }}</h2>
          <button @click="showPurgeByURLModal = false" class="text-muted-foreground hover:text-foreground">
            <component :is="CloseOutline" class="w-5 h-5" />
          </button>
        </div>
        <div class="p-8 space-y-4">
          <div class="p-4 bg-primary/5 border border-primary/10 rounded-xl text-xs text-primary font-medium leading-relaxed">
            {{ t('cache.purgeByURLExplain') }}
          </div>
          <textarea
            v-model="purgeURLs"
            class="w-full h-48 bg-slate-950 text-emerald-500 border border-border/50 rounded-xl p-4 font-mono text-xs focus:outline-none shadow-inner"
            placeholder="https://example.com/logo.png&#10;https://example.com/style.css"
          ></textarea>
        </div>
        <div class="p-6 border-t border-border/50 flex justify-end gap-3">
          <IslandButton variant="secondary" @click="showPurgeByURLModal = false">{{ t('common.cancel') }}</IslandButton>
          <IslandButton @click="handlePurgeByURL" :loading="purging">{{ t('cache.executePurge') }}</IslandButton>
        </div>
      </GlassCard>
    </n-modal>

    <n-modal v-model:show="showPurgeByTagModal">
      <GlassCard class="w-[500px] max-w-[95vw]" :padding="0">
        <div class="p-6 border-b border-border/50 flex justify-between items-center">
          <h2 class="text-xl font-bold">{{ t('cache.purgeByTag') }}</h2>
          <button @click="showPurgeByTagModal = false" class="text-muted-foreground hover:text-foreground">
            <component :is="CloseOutline" class="w-5 h-5" />
          </button>
        </div>
        <div class="p-8 space-y-4">
          <div class="p-4 bg-secondary/5 border border-secondary/10 rounded-xl text-xs text-secondary font-medium leading-relaxed">
            {{ t('cache.purgeByTagExplain') }}
          </div>
          <input
            v-model="purgeTags"
            class="w-full bg-foreground/5 border border-border/50 rounded-xl px-4 py-3 text-sm focus:ring-2 focus:ring-secondary/30 outline-none transition-all"
            placeholder="images, static-v1"
          />
        </div>
        <div class="p-6 border-t border-border/50 flex justify-end gap-3">
          <IslandButton variant="secondary" @click="showPurgeByTagModal = false">{{ t('common.cancel') }}</IslandButton>
          <IslandButton variant="secondary" class="!bg-secondary/20 !text-secondary hover:!bg-secondary/30" @click="handlePurgeByTag" :loading="purging">
            {{ t('cache.purgeTagContent') }}
          </IslandButton>
        </div>
      </GlassCard>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, inject, type Ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NSwitch, NModal, useDialog } from 'naive-ui'
import { 
  TrashOutline, 
  LinkOutline, 
  PricetagOutline, 
  LayersOutline, 
  TimeOutline,
  InformationCircleOutline,
  CloseOutline 
} from '@vicons/ionicons5'
import { cloudflareApi, type Zone } from '@/api'
import { useAccountStore } from '@/stores/account'
import { toast } from '@/utils/toast'
import { logHistory } from '@/utils/history'
import GlassCard from '@/components/ui/GlassCard.vue'
import IslandButton from '@/components/ui/IslandButton.vue'

const { t } = useI18n()
const dialog = useDialog()
const accountStore = useAccountStore()
const currentZone = inject<Ref<Zone | null>>('currentZone')

const loading = ref(false)
const updating = ref(false)
const purging = ref(false)

const showPurgeByURLModal = ref(false)
const showPurgeByTagModal = ref(false)

// 缓存设置
const cacheLevel = ref('aggressive')
const browserCacheTTL = ref(14400)
const developmentMode = ref(false)
const sortQueryString = ref(false)

// 清除缓存表单
const purgeURLs = ref('')
const purgeTags = ref('')

// 缓存统计
const cacheStats = ref({
  requests: 0,
  cached: 0,
  hitRate: 0
})

// 缓存级别选项
const cacheLevelOptions = computed(() => [
  { label: t('cache.levelBasic'), value: 'basic' },
  { label: t('cache.levelSimplified'), value: 'simplified' },
  { label: t('cache.levelAggressive'), value: 'aggressive' }
])

// 浏览器缓存 TTL 选项（秒）
const browserCacheTTLOptions = computed(() => [
  { label: `30 ${t('common.minutes')}`, value: 1800 },
  { label: `1 ${t('common.hours')}`, value: 3600 },
  { label: `2 ${t('common.hours')}`, value: 7200 },
  { label: `4 ${t('common.hours')}`, value: 14400 },
  { label: `8 ${t('common.hours')}`, value: 28800 },
  { label: `16 ${t('common.hours')}`, value: 57600 },
  { label: `1 ${t('common.days')}`, value: 86400 },
  { label: `2 ${t('common.days')}`, value: 172800 },
  { label: `1 ${t('common.months')}`, value: 2678400 },
  { label: `1 ${t('common.years')}`, value: 31536000 }
])

function getCacheLevelDescription(level: string): string {
  const descriptions: Record<string, string> = {
    basic: t('cache.levelBasicDesc'),
    simplified: t('cache.levelSimplifiedDesc'),
    aggressive: t('cache.levelAggressiveDesc')
  }
  return descriptions[level] || ''
}

function formatNumber(num: number): string {
  return num.toLocaleString()
}

async function loadCacheSettings() {
  if (!currentZone?.value?.id || !accountStore.currentAccount) {
    loading.value = false
    return
  }

  loading.value = true
  try {
    const settings = await cloudflareApi.getZoneSettings(currentZone.value.id)

    settings.forEach((setting: any) => {
      switch (setting.id) {
        case 'cache_level': cacheLevel.value = setting.value; break
        case 'browser_cache_ttl': browserCacheTTL.value = setting.value; break
        case 'development_mode': developmentMode.value = setting.value === 'on'; break
        case 'sort_query_string_for_cache': sortQueryString.value = setting.value === 'on'; break
      }
    })

    try {
      const analytics = await cloudflareApi.getAnalytics(currentZone.value.id, '24h')
      cacheStats.value = {
        requests: analytics.stats.totalRequests,
        cached: analytics.timeseries.reduce((sum, point) => sum + point.cached, 0),
        hitRate: Math.round(analytics.stats.cacheHitRate * 10) / 10
      }
    } catch (e) { /* ignore */ }
  } catch (error: any) {
    if (!error.silent) toast.error(t('cache.syncFailed'))
  } finally {
    loading.value = false
  }
}

async function updateSetting(id: string, value: any) {
  if (!currentZone?.value?.id) return

  updating.value = true
  try {
    await cloudflareApi.updateZoneSettings(currentZone.value.id, [{ id, value }])
    logHistory.cache('配置更新', `已调整缓存策略: ${id}`)
    toast.success(t('common.updateSuccess'))
  } catch (error: any) {
    toast.error(t('common.updateFailed'))
    await loadCacheSettings()
  } finally {
    updating.value = false
  }
}

const handleCacheLevelChange = (val: string) => updateSetting('cache_level', val)
const handleBrowserCacheTTLChange = (val: number) => updateSetting('browser_cache_ttl', val)
const handleSortQueryStringChange = (val: boolean) => updateSetting('sort_query_string_for_cache', val ? 'on' : 'off')

function handleDevelopmentModeChange(value: boolean) {
  if (value) {
    dialog.warning({
      title: t('cache.devModeEnable'),
      content: t('cache.devModeExplain'),
      positiveText: t('common.confirm'),
      negativeText: t('common.cancel'),
      onPositiveClick: () => updateSetting('development_mode', 'on'),
      onNegativeClick: () => developmentMode.value = false
    })
  } else {
    updateSetting('development_mode', 'off')
  }
}

function handlePurgeAllCache() {
  if (!currentZone?.value?.id) return

  dialog.error({
    title: t('cache.purgeAllTitle'),
    content: t('cache.purgeAllExplain'),
    positiveText: t('cache.purgeConfirm'),
    negativeText: t('common.cancel'),
    onPositiveClick: async () => {
      purging.value = true
      try {
        await cloudflareApi.purgeCache({
          zone_id: currentZone.value!.id,
          purge_everything: true
        })
        logHistory.cache('全站清理', '手动清空所有边缘缓存')
        toast.success(t('cache.purgeSuccess'))
      } catch (error: any) {
        toast.error(t('common.updateFailed'))
      } finally {
        purging.value = false
      }
    }
  })
}

async function handlePurgeByURL() {
  if (!currentZone?.value?.id) return
  const urls = purgeURLs.value.split('\n').filter(url => url.trim())
  if (urls.length === 0) return toast.warning(t('cache.inputURL'))

  purging.value = true
  try {
    await cloudflareApi.purgeCache({ zone_id: currentZone.value.id, files: urls })
    logHistory.cache('精准清理', `清除了 ${urls.length} 个资源`)
    toast.success(t('cache.purgeSuccess'))
    showPurgeByURLModal.value = false
    purgeURLs.value = ''
  } catch (error: any) {
    toast.error(t('common.updateFailed'))
  } finally {
    purging.value = false
  }
}

async function handlePurgeByTag() {
  if (!currentZone?.value?.id) return
  const tags = purgeTags.value.split(',').map(tag => tag.trim()).filter(tag => tag)
  if (tags.length === 0) return toast.warning(t('cache.inputTag'))

  purging.value = true
  try {
    await cloudflareApi.purgeCache({ zone_id: currentZone.value.id, tags: tags })
    logHistory.cache('标签清理', `清除了标签: ${tags.join(', ')}`)
    toast.success(t('cache.purgeSuccess'))
    showPurgeByTagModal.value = false
    purgeTags.value = ''
  } catch (error: any) {
    toast.error(t('common.updateFailed'))
  } finally {
    purging.value = false
  }
}

onMounted(() => loadCacheSettings())
watch(() => currentZone?.value?.id, () => loadCacheSettings())
</script>
