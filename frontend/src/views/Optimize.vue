<template>
  <div class="animate-in space-y-8 pb-12">
    <!-- Header -->
    <header class="px-1">
      <h1 class="text-3xl font-bold text-foreground tracking-tight">{{ t('optimize.title') }}</h1>
      <p class="text-sm text-muted-foreground mt-1 font-medium italic">
        {{ currentZone?.name || t('common.notSelected') }} · {{ t('optimize.subtitle') }}
      </p>
    </header>

    <!-- Presets Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <!-- Security Mode -->
      <GlassCard 
        :padding="8" 
        class="group cursor-pointer border-danger/10 hover:border-danger/40 transition-all active:scale-[0.98] relative overflow-hidden"
        @click="optimizeForSecurity"
      >
        <div class="absolute -right-4 -top-4 w-32 h-32 bg-danger/5 rounded-full blur-3xl group-hover:bg-danger/10 transition-colors"></div>
        
        <div class="relative space-y-6">
          <div class="flex items-center gap-4">
            <div class="w-14 h-14 rounded-2xl bg-danger/10 flex items-center justify-center text-danger shadow-inner">
              <component :is="ShieldCheckmarkOutline" class="w-8 h-8" />
            </div>
            <div>
              <h3 class="text-xl font-bold">{{ t('optimize.securityTitle') }}</h3>
              <p class="text-xs text-muted-foreground">{{ t('optimize.securitySub') }}</p>
            </div>
          </div>

          <div class="space-y-4">
            <p class="text-xs text-muted-foreground leading-relaxed">
              {{ t('optimize.securityDesc') }}
            </p>
            <div class="flex flex-wrap gap-2">
              <GlassBadge variant="danger" class="text-[10px]">{{ t('optimize.strictSSL') }}</GlassBadge>
              <GlassBadge variant="danger" class="text-[10px]">{{ t('optimize.alwaysHTTPS') }}</GlassBadge>
              <GlassBadge variant="danger" class="text-[10px]">{{ t('optimize.wafAdv') }}</GlassBadge>
              <GlassBadge variant="danger" class="text-[10px]">{{ t('optimize.browserCheck') }}</GlassBadge>
            </div>
          </div>

          <IslandButton variant="secondary" class="w-full !bg-danger/10 !text-danger hover:!bg-danger/20 border-none shadow-none">
            {{ t('optimize.syncSecurity') }}
          </IslandButton>
        </div>
      </GlassCard>

      <!-- Speed Mode -->
      <GlassCard 
        :padding="8" 
        class="group cursor-pointer border-primary/10 hover:border-primary/40 transition-all active:scale-[0.98] relative overflow-hidden"
        @click="optimizeForSpeed"
      >
        <div class="absolute -right-4 -top-4 w-32 h-32 bg-primary/5 rounded-full blur-3xl group-hover:bg-primary/10 transition-colors"></div>

        <div class="relative space-y-6">
          <div class="flex items-center gap-4">
            <div class="w-14 h-14 rounded-2xl bg-primary/10 flex items-center justify-center text-primary shadow-inner">
              <component :is="FlashOutline" class="w-8 h-8" />
            </div>
            <div>
              <h3 class="text-xl font-bold">{{ t('optimize.speedTitle') }}</h3>
              <p class="text-xs text-muted-foreground">{{ t('optimize.speedSub') }}</p>
            </div>
          </div>

          <div class="space-y-4">
            <p class="text-xs text-muted-foreground leading-relaxed">
              {{ t('optimize.speedDesc') }}
            </p>
            <div class="flex flex-wrap gap-2">
              <GlassBadge variant="info" class="text-[10px]">{{ t('optimize.http3') }}</GlassBadge>
              <GlassBadge variant="info" class="text-[10px]">{{ t('optimize.brotli') }}</GlassBadge>
              <GlassBadge variant="info" class="text-[10px]">{{ t('optimize.aggressiveCache') }}</GlassBadge>
              <GlassBadge variant="info" class="text-[10px]">{{ t('optimize.ttl1Year') }}</GlassBadge>
            </div>
          </div>

          <IslandButton variant="secondary" class="w-full !bg-primary/10 !text-primary hover:!bg-primary/20 border-none shadow-none">
            {{ t('optimize.syncSpeed') }}
          </IslandButton>
        </div>
      </GlassCard>
    </div>

    <!-- Status Overview -->
    <div class="space-y-4">
      <h3 class="text-xs font-bold uppercase tracking-widest text-muted-foreground px-1 text-center md:text-left">{{ t('optimize.statusTitle') }}</h3>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <GlassCard v-for="(val, key) in statusMap" :key="key" :padding="5" class="text-center space-y-1">
          <p class="text-[10px] font-bold text-muted-foreground uppercase">{{ key }}</p>
          <p class="text-sm font-bold text-foreground">{{ val || '...' }}</p>
        </GlassCard>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, inject, onMounted, computed, watch, type Ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { ShieldCheckmarkOutline, FlashOutline } from '@vicons/ionicons5'
import { cloudflareApi, type Zone } from '@/api'
import { useAccountStore } from '@/stores/account'
import { toast } from '@/utils/toast'
import { logHistory } from '@/utils/history'
import { useDialog } from 'naive-ui'
import GlassCard from '@/components/ui/GlassCard.vue'
import IslandButton from '@/components/ui/IslandButton.vue'
import GlassBadge from '@/components/ui/GlassBadge.vue'

const { t } = useI18n()
const dialog = useDialog()
const accountStore = useAccountStore()
const currentZone = inject<Ref<Zone | null>>('currentZone')
const settings = ref<any[]>([])

const statusMap = computed(() => {
  const find = (id: string) => settings.value.find(s => s.id === id)?.value
  return {
    [t('optimize.secLevel')]: find('security_level'),
    [t('optimize.sslMode')]: find('ssl'),
    [t('optimize.cacheLevel')]: find('cache_level'),
    [t('optimize.brotliCompression')]: find('brotli') === 'on' ? t('optimize.on') : t('optimize.off')
  }
})

async function loadSettings() {
  if (!currentZone?.value?.id || !accountStore.currentAccount) return
  try {
    settings.value = await cloudflareApi.getZoneSettings(currentZone.value.id)
  } catch (e: any) {
    console.error('Failed to load settings:', e)
  }
}

async function applyPreset(name: string, payload: any[]) {
  if (!currentZone?.value?.id) return toast.warning(t('optimize.selectZoneErr'))
  
  dialog.warning({
    title: t('optimize.applyPresetTitle', { name }),
    content: t('optimize.applyPresetConfirm'),
    positiveText: t('optimize.confirmApply'),
    negativeText: t('common.cancel'),
    onPositiveClick: async () => {
      try {
        await cloudflareApi.updateZoneSettings(currentZone.value!.id, payload)
        logHistory.home(t('optimize.logOptimize'), name)
        toast.success(t('optimize.applySuccess', { name }))
        await loadSettings()
      } catch (e) {
        toast.error(t('optimize.applyFailed'))
      }
    }
  })
}

const optimizeForSecurity = () => applyPreset(t('optimize.securityTitle'), [
  { id: 'security_level', value: 'high' },
  { id: 'ssl', value: 'strict' },
  { id: 'always_use_https', value: 'on' },
  { id: 'browser_check', value: 'on' }
])

const optimizeForSpeed = () => applyPreset(t('optimize.speedTitle'), [
  { id: 'cache_level', value: 'aggressive' },
  { id: 'brotli', value: 'on' },
  { id: 'http3', value: 'on' },
  { id: 'browser_cache_ttl', value: 31536000 }
])

onMounted(() => loadSettings())
watch(() => currentZone?.value?.id, () => loadSettings())
</script>
