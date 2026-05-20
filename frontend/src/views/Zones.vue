<template>
  <div class="animate-in space-y-6">
    <!-- Header -->
    <header class="flex justify-between items-end px-1">
      <div>
        <h1 class="text-3xl font-bold text-foreground">{{ t('zones.title') }}</h1>
        <p class="text-sm text-muted-foreground mt-1 font-medium">{{ t('zones.subtitle') }}</p>
      </div>
      <IslandButton disabled>
        {{ t('zones.addZone') }}
      </IslandButton>
    </header>

    <!-- Content Area -->
    <GlassCard :padding="0" class="overflow-hidden">
      <!-- Loading State -->
      <div v-if="loading" class="py-20 flex flex-col items-center justify-center">
        <div class="w-10 h-10 border-4 border-primary/20 border-t-primary rounded-full animate-spin"></div>
        <p class="mt-4 text-sm text-muted-foreground font-medium">{{ t('common.syncing') }}</p>
      </div>

      <!-- Zones Table -->
      <div v-else-if="zones.length > 0" class="overflow-x-auto">
        <table class="w-full">
          <thead>
            <tr class="bg-foreground/5">
              <th class="text-left py-4 px-6 text-xs font-bold uppercase tracking-wider text-muted-foreground">{{ t('zones.zoneInfo') }}</th>
              <th class="text-left py-4 px-6 text-xs font-bold uppercase tracking-wider text-muted-foreground">{{ t('zones.status') }}</th>
              <th class="text-left py-4 px-6 text-xs font-bold uppercase tracking-wider text-muted-foreground">{{ t('zones.plan') }}</th>
              <th class="text-left py-4 px-6 text-xs font-bold uppercase tracking-wider text-muted-foreground">{{ t('zones.nsServers') }}</th>
              <th class="text-right py-4 px-6 text-xs font-bold uppercase tracking-wider text-muted-foreground">{{ t('zones.actions') }}</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-border/50">
            <tr 
              v-for="zone in zones" 
              :key="zone.id"
              class="hover:bg-foreground/[0.02] transition-colors group"
            >
              <td class="py-4 px-6">
                <div class="font-bold text-sm text-foreground group-hover:text-primary transition-colors">{{ zone.name }}</div>
                <div class="text-[10px] text-muted-foreground font-mono mt-0.5">{{ zone.id }}</div>
              </td>
              <td class="py-4 px-6">
                <GlassBadge :variant="zone.status === 'active' ? 'success' : 'info'">
                  {{ zone.status.toUpperCase() }}
                </GlassBadge>
              </td>
              <td class="py-4 px-6 text-sm">
                <div class="flex items-center gap-2">
                  <span class="w-2 h-2 rounded-full bg-primary/40"></span>
                  {{ zone.plan?.name || 'Free Plan' }}
                </div>
              </td>
              <td class="py-4 px-6">
                <div class="text-[10px] text-muted-foreground font-medium space-y-1">
                  <div v-for="ns in zone.name_servers?.slice(0, 2)" :key="ns" class="flex items-center gap-1">
                    <div class="w-1 h-1 rounded-full bg-border"></div>
                    {{ ns }}
                  </div>
                </div>
              </td>
              <td class="py-4 px-6 text-right">
                <IslandButton
                  size="small"
                  variant="secondary"
                  @click="goToZoneDetail(zone)"
                >
                  {{ t('zones.manage') }}
                </IslandButton>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Empty State -->
      <div v-else class="py-24 flex flex-col items-center text-center px-6">
        <div class="w-16 h-16 rounded-3xl bg-muted flex items-center justify-center text-muted-foreground mb-6 shadow-inner">
          <component :is="accountStore.currentAccount ? GlobeOutline : ShieldOutline" class="w-8 h-8" />
        </div>
        <template v-if="accountStore.currentAccount">
          <h3 class="text-xl font-bold">{{ t('zones.noZones') }}</h3>
          <p class="text-sm text-muted-foreground max-w-xs mt-3 leading-relaxed">
            {{ t('zones.noZonesDesc') }}
          </p>
        </template>
        <template v-else>
          <h3 class="text-xl font-bold">{{ t('zones.linkAccount') }}</h3>
          <p class="text-sm text-muted-foreground max-w-xs mt-3 leading-relaxed">
            {{ t('zones.linkAccountDesc') }}
          </p>
          <IslandButton class="mt-8" @click="router.push('/accounts')">
            {{ t('zones.gotoAccounts') }}
          </IslandButton>
        </template>
      </div>
    </GlassCard>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { GlobeOutline, ShieldOutline } from '@vicons/ionicons5'
import { useRouter } from 'vue-router'
import { cloudflareApi, type Zone } from '@/api'
import { useAccountStore } from '@/stores/account'
import GlassCard from '@/components/ui/GlassCard.vue'
import IslandButton from '@/components/ui/IslandButton.vue'
import GlassBadge from '@/components/ui/GlassBadge.vue'

const { t } = useI18n()
const router = useRouter()
const accountStore = useAccountStore()
const loading = ref(false)
const zones = ref<any[]>([])

async function loadZones() {
  if (!accountStore.currentAccount) return
  
  loading.value = true
  try {
    zones.value = await cloudflareApi.getZones()
  } catch (error: any) {
    if (!error.silent) {
      console.error('Failed to load zones:', error)
    }
  } finally {
    loading.value = false
  }
}

function goToZoneDetail(zone: any) {
  localStorage.setItem('currentZoneId', zone.id)
  router.push('/dns').then(() => {
    window.location.reload()
  })
}

onMounted(() => {
  loadZones()
})
</script>
