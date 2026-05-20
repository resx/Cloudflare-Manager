<template>
  <div class="animate-in space-y-8 pb-12">
    <!-- Header -->
    <header class="flex flex-col md:flex-row justify-between items-start md:items-end gap-4 px-1">
      <div>
        <h1 class="text-3xl font-bold text-foreground tracking-tight">{{ t('ssl.title') }}</h1>
        <p class="text-sm text-muted-foreground mt-1 font-medium italic">
          {{ currentZone?.name || t('zones.notSelected') }} · {{ t('ssl.subtitle') }}
        </p>
      </div>
      <div class="flex gap-2">
        <GlassBadge variant="success" v-if="certInfo.status === 'active'">
          {{ t('ssl.statusActive') }}
        </GlassBadge>
      </div>
    </header>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <!-- Left: Settings -->
      <div class="lg:col-span-2 space-y-6">
        <h3 class="text-xs font-bold uppercase tracking-widest text-muted-foreground px-1">{{ t('ssl.encryptionConfig') }}</h3>
        
        <!-- SSL Mode Card -->
        <GlassCard :padding="6">
          <div class="flex flex-col md:flex-row gap-6 justify-between items-start md:items-center">
            <div class="space-y-1 max-w-md">
              <h4 class="font-bold text-foreground flex items-center gap-2">
                <component :is="LockClosedOutline" class="w-4 h-4 text-primary" />
                {{ t('ssl.encryptionMode') }}
              </h4>
              <p class="text-xs text-muted-foreground leading-relaxed">
                {{ getSslModeDescription(sslMode) }}
              </p>
            </div>
            <select 
              v-model="sslMode" 
              class="w-full md:w-48 bg-foreground/5 border border-border/50 rounded-xl px-4 py-2.5 text-sm outline-none cursor-pointer focus:ring-2 focus:ring-primary/30 transition-all"
              @change="handleSSLModeChange"
            >
              <option v-for="opt in sslModeOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
            </select>
          </div>
        </GlassCard>

        <!-- Toggle Settings Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Always Use HTTPS -->
          <GlassCard :padding="5" class="flex items-center justify-between gap-4">
            <div class="space-y-0.5">
              <h5 class="text-sm font-bold">{{ t('ssl.alwaysHttps') }}</h5>
              <p class="text-[10px] text-muted-foreground">{{ t('ssl.alwaysHttpsDesc') }}</p>
            </div>
            <n-switch v-model:value="alwaysUseHttps" :loading="updating" @update:value="handleAlwaysHttpsChange" size="small" />
          </GlassCard>

          <!-- Automatic HTTPS Rewrites -->
          <GlassCard :padding="5" class="flex items-center justify-between gap-4">
            <div class="space-y-0.5">
              <h5 class="text-sm font-bold">{{ t('ssl.httpsRewrites') }}</h5>
              <p class="text-[10px] text-muted-foreground">{{ t('ssl.httpsRewritesDesc') }}</p>
            </div>
            <n-switch v-model:value="automaticHttpsRewrites" :loading="updating" @update:value="handleAutomaticHttpsRewritesChange" size="small" />
          </GlassCard>

          <!-- TLS 1.3 -->
          <GlassCard :padding="5" class="flex items-center justify-between gap-4">
            <div class="space-y-0.5">
              <h5 class="text-sm font-bold">TLS 1.3</h5>
              <p class="text-[10px] text-muted-foreground">{{ t('ssl.tls13Desc') }}</p>
            </div>
            <n-switch v-model:value="tls13" :loading="updating" @update:value="handleTls13Change" size="small" />
          </GlassCard>

          <!-- Opportunistic Encryption -->
          <GlassCard :padding="5" class="flex items-center justify-between gap-4">
            <div class="space-y-0.5">
              <h5 class="text-sm font-bold">{{ t('ssl.opportunistic') }}</h5>
              <p class="text-[10px] text-muted-foreground">{{ t('ssl.opportunisticDesc') }}</p>
            </div>
            <n-switch v-model:value="opportunisticEncryption" :loading="updating" @update:value="handleOpportunisticEncryptionChange" size="small" />
          </GlassCard>
        </div>

        <!-- Min TLS Version -->
        <GlassCard :padding="6">
          <div class="flex flex-col md:flex-row gap-6 justify-between items-start md:items-center">
            <div class="space-y-1 max-w-md">
              <h4 class="font-bold text-foreground">{{ t('ssl.minTls') }}</h4>
              <p class="text-xs text-muted-foreground leading-relaxed">
                {{ t('ssl.minTlsDesc') }}
              </p>
            </div>
            <select 
              v-model="minTlsVersion" 
              class="w-full md:w-48 bg-foreground/5 border border-border/50 rounded-xl px-4 py-2.5 text-sm outline-none cursor-pointer focus:ring-2 focus:ring-primary/30 transition-all"
              @change="handleMinTlsVersionChange"
            >
              <option v-for="opt in tlsVersionOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
            </select>
          </div>
        </GlassCard>
      </div>

      <!-- Right: Certificate Info -->
      <div class="space-y-6">
        <h3 class="text-xs font-bold uppercase tracking-widest text-muted-foreground px-1">{{ t('ssl.certStatus') }}</h3>
        <GlassCard :padding="6" class="space-y-6 border-primary/20 bg-primary/[0.02]">
          <div class="flex flex-col items-center text-center">
            <div class="w-16 h-16 rounded-3xl bg-primary/10 flex items-center justify-center text-primary mb-4 shadow-inner">
              <component :is="ShieldCheckmarkOutline" class="w-8 h-8" />
            </div>
            <div class="space-y-1">
              <h4 class="font-bold text-lg">{{ t('ssl.certTrusted') }}</h4>
              <p class="text-xs text-muted-foreground font-medium">{{ t('ssl.issuer') }}: {{ certInfo.issuer }}</p>
            </div>
          </div>

          <div class="space-y-4 pt-6 border-t border-border/30">
            <div class="flex justify-between items-center text-xs">
              <span class="text-muted-foreground">{{ t('ssl.validation') }}</span>
              <span class="font-bold" :class="certInfo.status === 'active' ? 'text-emerald-500' : 'text-amber-500'">{{ certInfo.status.toUpperCase() }}</span>
            </div>
            <div class="flex justify-between items-center text-xs">
              <span class="text-muted-foreground">{{ t('ssl.certType') }}</span>
              <span class="text-foreground font-bold">{{ certInfo.type }}</span>
            </div>
            <div class="flex justify-between items-center text-xs">
              <span class="text-muted-foreground">{{ t('ssl.signature') }}</span>
              <span class="text-foreground font-mono opacity-80">{{ certInfo.signature }}</span>
            </div>
          </div>

          <div class="pt-2">
            <IslandButton variant="secondary" class="w-full text-xs" size="small" disabled>
              {{ t('ssl.viewChain') }}
            </IslandButton>
          </div>
        </GlassCard>

        <div class="p-4 bg-amber-500/5 border border-amber-500/10 rounded-2xl">
          <p class="text-[10px] text-amber-600/80 dark:text-amber-400/80 leading-relaxed font-medium">
            {{ t('ssl.tip') }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, inject, type Ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { NSwitch } from 'naive-ui'
import { 
  LockClosedOutline, 
  ShieldCheckmarkOutline, 
  InformationCircleOutline,
  ShieldOutline 
} from '@vicons/ionicons5'
import { cloudflareApi, type Zone } from '@/api'
import { useAccountStore } from '@/stores/account'
import { toast } from '@/utils/toast'
import { logHistory } from '@/utils/history'
import GlassCard from '@/components/ui/GlassCard.vue'
import IslandButton from '@/components/ui/IslandButton.vue'
import GlassBadge from '@/components/ui/GlassBadge.vue'

const { t } = useI18n()
const accountStore = useAccountStore()

// 从 Layout 获取当前域名
const currentZone = inject<Ref<Zone | null>>('currentZone')

const loading = ref(false)
const updating = ref(false)

// SSL 设置
const sslMode = ref('flexible')
const alwaysUseHttps = ref(false)
const automaticHttpsRewrites = ref(false)
const minTlsVersion = ref('1.0')
const tls13 = ref(false)
const opportunisticEncryption = ref(false)

// 证书信息
const certInfo = ref({
  status: 'inactive',
  type: 'Universal SSL',
  issuer: '-',
  signature: '-'
})

// SSL 模式选项
const sslModeOptions = [
  { label: 'Off', value: 'off' },
  { label: 'Flexible', value: 'flexible' },
  { label: 'Full', value: 'full' },
  { label: 'Full Strict', value: 'strict' }
]

// TLS 版本选项
const tlsVersionOptions = [
  { label: 'TLS 1.0', value: '1.0' },
  { label: 'TLS 1.1', value: '1.1' },
  { label: 'TLS 1.2', value: '1.2' },
  { label: 'TLS 1.3', value: '1.3' }
]

function getSslModeDescription(mode: string): string {
  const descriptions: Record<string, string> = {
    off: t('ssl.modeOffDesc'),
    flexible: t('ssl.modeFlexibleDesc'),
    full: t('ssl.modeFullDesc'),
    strict: t('ssl.modeStrictDesc')
  }
  return descriptions[mode] || ''
}

async function loadSSLSettings() {
  if (!currentZone?.value?.id || !accountStore.currentAccount) {
    loading.value = false
    return
  }

  loading.value = true
  try {
    const settings = await cloudflareApi.getZoneSettings(currentZone.value.id)

    // 解析设置
    settings.forEach((setting: any) => {
      switch (setting.id) {
        case 'ssl': sslMode.value = setting.value; break
        case 'always_use_https': alwaysUseHttps.value = setting.value === 'on'; break
        case 'automatic_https_rewrites': automaticHttpsRewrites.value = setting.value === 'on'; break
        case 'min_tls_version': minTlsVersion.value = setting.value; break
        case 'tls_1_3': tls13.value = setting.value === 'on'; break
        case 'opportunistic_encryption': opportunisticEncryption.value = setting.value === 'on'; break
      }
    })

    // 获取 SSL 证书信息
    try {
      const certificates = await cloudflareApi.getSslCertificates(currentZone.value.id)
      if (certificates && certificates.length > 0) {
        const cert = certificates[0]
        const detail = cert.certificates && cert.certificates.length > 0 ? cert.certificates[0] : null
        certInfo.value = {
          status: cert.status || 'unknown',
          type: cert.type || 'Universal SSL',
          issuer: detail?.issuer || 'Cloudflare Managed',
          signature: detail?.signature || 'ECDSA'
        }
      }
    } catch (e) { /* silent fail */ }
  } catch (error: any) {
    if (!error.silent) toast.error(t('ssl.syncFailed'))
  } finally {
    loading.value = false
  }
}

async function updateSetting(id: string, value: any) {
  if (!currentZone?.value?.id) return

  updating.value = true
  try {
    await cloudflareApi.updateZoneSettings(currentZone.value.id, [{ id, value }])
    logHistory.ssl('安全配置更新', `已调整 ${id} 项配置`)
    toast.success(t('common.updateSuccess'))
  } catch (error: any) {
    toast.error(t('common.updateFailed'))
    await loadSSLSettings()
  } finally {
    updating.value = false
  }
}

function handleSSLModeChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value
  updateSetting('ssl', value)
}

function handleAlwaysHttpsChange(value: boolean) {
  updateSetting('always_use_https', value ? 'on' : 'off')
}

function handleAutomaticHttpsRewritesChange(value: boolean) {
  updateSetting('automatic_https_rewrites', value ? 'on' : 'off')
}

function handleMinTlsVersionChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value
  updateSetting('min_tls_version', value)
}

function handleTls13Change(value: boolean) {
  updateSetting('tls_1_3', value ? 'on' : 'off')
}

function handleOpportunisticEncryptionChange(value: boolean) {
  updateSetting('opportunistic_encryption', value ? 'on' : 'off')
}

onMounted(() => {
  loadSSLSettings()
})

// 监听 currentZone 变化，自动重新加载 SSL 设置
watch(() => currentZone?.value?.id, () => {
  loadSSLSettings()
})
</script>
