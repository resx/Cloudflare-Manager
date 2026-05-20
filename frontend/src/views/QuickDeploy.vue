<template>
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">{{ t('quickDeploy.title') }}</h1>
      <p class="text-sm text-muted-foreground mt-1">{{ t('quickDeploy.subtitle') }}</p>
    </div>

    <!-- Mode Selector -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
      <button
        @click="mode = 'worker'"
        :class="['metric-card p-5 text-left transition-all', mode === 'worker' ? 'border-primary ring-2 ring-primary/20' : '']"
      >
        <div class="flex items-center gap-3 mb-3">
          <component :is="SettingsOutline" class="w-5 h-5 text-primary" />
          <span class="font-semibold">{{ t('quickDeploy.workerProxy') }}</span>
        </div>
        <p class="text-xs text-muted-foreground mb-3">{{ t('quickDeploy.workerProxyDesc') }}</p>
        <div class="text-xs space-y-1">
          <div class="text-green-600">{{ t('quickDeploy.workerProxyPros.pro1') }}</div>
          <div class="text-green-600">{{ t('quickDeploy.workerProxyPros.pro2') }}</div>
          <div class="text-green-600">{{ t('quickDeploy.workerProxyPros.pro3') }}</div>
          <div class="text-muted-foreground">{{ t('quickDeploy.workerProxyPros.con1') }}</div>
          <div class="text-muted-foreground">{{ t('quickDeploy.workerProxyPros.con2') }}</div>
          <div class="text-muted-foreground">{{ t('quickDeploy.workerProxyPros.con3') }}</div>
        </div>
      </button>
      <button
        @click="mode = 'saas'"
        :class="['metric-card p-5 text-left transition-all', mode === 'saas' ? 'border-primary ring-2 ring-primary/20' : '']"
      >
        <div class="flex items-center gap-3 mb-3">
          <component :is="GlobeOutline" class="w-5 h-5 text-primary" />
          <span class="font-semibold">{{ t('quickDeploy.saasOptimize') }}</span>
          <span class="glass-badge glass-badge-info text-[10px]">{{ t('common.recommended') || '推荐' }}</span>
        </div>
        <p class="text-xs text-muted-foreground mb-3">{{ t('quickDeploy.saasOptimizeDesc') }}</p>
        <div class="text-xs space-y-1">
          <div class="text-green-600">{{ t('quickDeploy.saasOptimizePros.pro1') }}</div>
          <div class="text-green-600">{{ t('quickDeploy.saasOptimizePros.pro2') }}</div>
          <div class="text-green-600">{{ t('quickDeploy.saasOptimizePros.pro3') }}</div>
          <div class="text-green-600">{{ t('quickDeploy.saasOptimizePros.pro4') }}</div>
          <div class="text-green-600">{{ t('quickDeploy.saasOptimizePros.pro5') }}</div>
          <div class="text-muted-foreground">{{ t('quickDeploy.saasOptimizePros.con1') }}</div>
        </div>
      </button>
    </div>

    <!-- Worker Mode -->
    <template v-if="mode === 'worker'">
      <div class="banner-gradient rounded-lg p-5 mb-6">
        <h3 class="font-semibold mb-1 flex items-center gap-2">
          <component :is="RocketOutline" class="w-5 h-5 text-primary" /> {{ t('quickDeploy.workerSteps') }}
        </h3>
        <div class="text-sm text-muted-foreground space-y-0.5">
          <div>{{ t('quickDeploy.workerStep1') }}</div>
          <div>{{ t('quickDeploy.workerStep2') }}</div>
          <div>{{ t('quickDeploy.workerStep3') }}</div>
        </div>
      </div>

      <div class="metric-card p-6 mb-6">
        <h3 class="font-semibold mb-4">{{ t('quickDeploy.deployConfig') }}</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium mb-2">{{ t('quickDeploy.sourceDomain') }} *</label>
            <input v-model="workerForm.sourceDomain" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="example.com" />
            <p class="text-xs text-muted-foreground mt-1">{{ t('quickDeploy.sourceDomainTip') }}</p>
          </div>
          <div>
            <label class="block text-sm font-medium mb-2">{{ t('quickDeploy.workerName') }} *</label>
            <input v-model="workerForm.workerName" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="cdn-accelerator" />
          </div>
          <div>
            <label class="block text-sm font-medium mb-2">{{ t('quickDeploy.routePattern') }} *</label>
            <input v-model="workerForm.routePattern" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="cdn.example.com/*" />
          </div>
          <div>
            <label class="block text-sm font-medium mb-2">{{ t('quickDeploy.cacheTTL') }}</label>
            <select v-model="workerForm.cacheTTL" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50">
              <option :value="0">{{ t('quickDeploy.noCache') }}</option>
              <option :value="300">{{ t('quickDeploy.cache5m') }}</option>
              <option :value="3600">{{ t('quickDeploy.cache1h') }}</option>
              <option :value="86400">{{ t('quickDeploy.cache1d') }}</option>
              <option :value="604800">{{ t('quickDeploy.cache1w') }}</option>
            </select>
          </div>
          <div class="flex gap-3 pt-4">
            <button @click="deployWorker" :disabled="deploying || !isWorkerFormValid" class="btn-island-primary flex-1">
              <template v-if="deploying">{{ t('quickDeploy.deploying') }}</template>
              <template v-else><component :is="RocketOutline" class="w-4 h-4 mr-1" /> {{ t('quickDeploy.startDeploy') }}</template>
            </button>
            <button @click="showCodePreview = true" class="btn-island-secondary">{{ t('quickDeploy.previewCode') }}</button>
          </div>
        </div>
      </div>

      <div v-if="deployResult" class="metric-card p-6">
        <h3 class="font-semibold mb-3 flex items-center gap-2">
          <component :is="CheckmarkCircleOutline" class="w-5 h-5 text-green-600" /> {{ t('quickDeploy.deploySuccess') }}
        </h3>
        <div class="space-y-2 text-sm">
          <div class="flex justify-between"><span class="text-muted-foreground">Worker:</span><span class="font-mono">{{ deployResult.name }}</span></div>
          <div class="flex justify-between"><span class="text-muted-foreground">{{ t('quickDeploy.route') }}:</span><code class="text-xs bg-muted px-2 py-1 rounded">{{ deployResult.url }}</code></div>
        </div>
      </div>
    </template>

    <!-- SaaS Mode -->
    <template v-if="mode === 'saas'">
      <!-- Step Indicator -->
      <div class="flex items-center gap-2 mb-6">
        <template v-for="(step, i) in saasSteps" :key="i">
          <div :class="['w-8 h-8 rounded-full flex items-center justify-center text-xs font-semibold transition-all', saasStep > i ? 'bg-primary text-white' : saasStep === i ? 'bg-primary/20 text-primary border border-primary' : 'bg-muted text-muted-foreground']">
            {{ i + 1 }}
          </div>
          <div v-if="i < saasSteps.length - 1" :class="['flex-1 h-0.5 rounded transition-all', saasStep > i ? 'bg-primary' : 'bg-muted']"></div>
        </template>
      </div>

      <!-- Step 0: Prerequisites -->
      <div v-if="saasStep === 0" class="metric-card p-6">
        <h3 class="font-semibold mb-4 flex items-center gap-2">
          <component :is="ShieldCheckmarkOutline" class="w-5 h-5 text-primary" /> {{ t('quickDeploy.prereqTitle') }}
        </h3>
        <div class="space-y-4">
          <div class="alert-info">
            <p class="font-medium mb-2">{{ t('quickDeploy.prereqDesc1') }}</p>
            <p class="text-sm">{{ t('quickDeploy.prereqDesc2') }}</p>
          </div>
          <div class="text-sm space-y-2">
            <p class="font-medium">{{ t('quickDeploy.stepsTitle') }}</p>
            <ol class="list-decimal list-inside space-y-1 text-muted-foreground">
              <li>{{ t('quickDeploy.stepItem1') }}</li>
              <li>{{ t('quickDeploy.stepItem2') }}</li>
              <li>{{ t('quickDeploy.stepItem3') }}</li>
            </ol>
          </div>
          <div class="flex justify-end pt-2">
            <button class="btn-island-primary" @click="saasStep = 1">{{ t('quickDeploy.completedNext') }}</button>
          </div>
        </div>
      </div>

      <!-- Step 1: Fallback Origin -->
      <div v-if="saasStep === 1" class="metric-card p-6">
        <h3 class="font-semibold mb-4 flex items-center gap-2">
          <component :is="ServerOutline" class="w-5 h-5 text-primary" /> {{ t('quickDeploy.fallbackTitle') }}
        </h3>
        <div class="space-y-4">
          <p class="text-sm text-muted-foreground">{{ t('quickDeploy.fallbackDesc') }}</p>
          <div>
            <label class="block text-sm font-medium mb-2">{{ t('quickDeploy.fallbackSubdomain') }} *</label>
            <input v-model="saasForm.fallbackDomain" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="fallback.example.com" />
            <p class="text-xs text-muted-foreground mt-1">{{ t('quickDeploy.fallbackSubdomainTip') }}</p>
          </div>
          <div>
            <label class="block text-sm font-medium mb-2">{{ t('quickDeploy.originIP') }} *</label>
            <input v-model="saasForm.originIP" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="1.2.3.4" />
          </div>
          <div class="flex justify-between pt-2">
            <button class="btn-island-secondary" @click="saasStep = 0">{{ t('quickDeploy.prevStep') }}</button>
            <button class="btn-island-primary" @click="autoSetupFallback" :disabled="!saasForm.fallbackDomain || !saasForm.originIP || saasLoading">
              <template v-if="saasLoading">{{ t('quickDeploy.settingFallback') }}</template>
              <template v-else>{{ t('quickDeploy.setupFallback') }}</template>
            </button>
          </div>
        </div>
      </div>

      <!-- Step 2: Custom Hostname -->
      <div v-if="saasStep === 2" class="metric-card p-6">
        <h3 class="font-semibold mb-4 flex items-center gap-2">
          <component :is="LinkOutline" class="w-5 h-5 text-primary" /> {{ t('quickDeploy.hostnameTitle') }}
        </h3>
        <div class="space-y-4">
          <p class="text-sm text-muted-foreground">{{ t('quickDeploy.hostnameDesc') }}</p>
          <div>
            <label class="block text-sm font-medium mb-2">{{ t('quickDeploy.customHostname') }} *</label>
            <input v-model="saasForm.customHostname" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="cdn.yourdomain.com" />
            <p class="text-xs text-muted-foreground mt-1">{{ t('quickDeploy.customHostnameTip') }}</p>
          </div>
          <div class="flex justify-between pt-2">
            <button class="btn-island-secondary" @click="saasStep = 1">{{ t('quickDeploy.prevStep') }}</button>
            <button class="btn-island-primary" @click="autoCreateHostname" :disabled="!saasForm.customHostname || saasLoading">
              <template v-if="saasLoading">{{ t('quickDeploy.creatingHostname') }}</template>
              <template v-else>{{ t('quickDeploy.createHostname') }}</template>
            </button>
          </div>
        </div>
      </div>

      <!-- Step 3: DNS Optimization -->
      <div v-if="saasStep === 3" class="metric-card p-6">
        <h3 class="font-semibold mb-4 flex items-center gap-2">
          <component :is="GlobeOutline" class="w-5 h-5 text-primary" /> {{ t('quickDeploy.dnsOptTitle') }}
        </h3>
        <div class="space-y-4">
          <p class="text-sm text-muted-foreground">{{ t('quickDeploy.dnsOptDesc') }}</p>
          <div>
            <label class="block text-sm font-medium mb-2">{{ t('quickDeploy.optType') }}</label>
            <select v-model="saasForm.optimizeType" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50">
              <option value="cname">{{ t('quickDeploy.optCname') }}</option>
              <option value="ip">{{ t('quickDeploy.optIP') }}</option>
            </select>
          </div>
          <div v-if="saasForm.optimizeType === 'cname'">
            <label class="block text-sm font-medium mb-2">{{ t('quickDeploy.selectOptCname') }}</label>
            <select v-model="saasForm.optimizeCname" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50">
              <option v-for="item in cnamePresets" :key="item.value" :value="item.value">{{ item.label }}</option>
            </select>
            <input v-if="saasForm.optimizeCname === 'custom'" v-model="saasForm.customCname" class="w-full mt-2 px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" :placeholder="t('quickDeploy.customCnamePlaceholder')" />
          </div>
          <div v-else>
            <label class="block text-sm font-medium mb-2">{{ t('quickDeploy.optIPAddress') }}</label>
            <input v-model="saasForm.optimizeIP" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="104.16.x.x" />
          </div>
          <div class="flex justify-between pt-2">
            <button class="btn-island-secondary" @click="saasStep = 2">{{ t('quickDeploy.prevStep') }}</button>
            <button class="btn-island-primary" @click="autoCreateOptimizeDns" :disabled="saasLoading">
              <template v-if="saasLoading">{{ t('quickDeploy.deploying') }}</template>
              <template v-else>{{ t('quickDeploy.configureDns') }}</template>
            </button>
          </div>
        </div>
      </div>

      <!-- Step 4: Complete -->
      <div v-if="saasStep === 4" class="metric-card p-6">
        <h3 class="font-semibold mb-4 flex items-center gap-2">
          <component :is="CheckmarkCircleOutline" class="w-5 h-5 text-green-600" /> {{ t('quickDeploy.configComplete') }}
        </h3>
        <div class="space-y-4">
          <div class="space-y-3 text-sm">
            <div class="flex justify-between py-2 border-b border-border">
              <span class="text-muted-foreground">{{ t('quickDeploy.fallbackOrigin') }}</span>
              <span class="font-mono">{{ saasForm.fallbackDomain }}</span>
            </div>
            <div class="flex justify-between py-2 border-b border-border">
              <span class="text-muted-foreground">{{ t('quickDeploy.originIP') }}</span>
              <span class="font-mono">{{ saasForm.originIP }}</span>
            </div>
            <div class="flex justify-between py-2 border-b border-border">
              <span class="text-muted-foreground">{{ t('quickDeploy.accessDomain') }}</span>
              <span class="font-mono">{{ saasForm.customHostname }}</span>
            </div>
            <div class="flex justify-between py-2">
              <span class="text-muted-foreground">{{ t('quickDeploy.optTarget') }}</span>
              <span class="font-mono">{{ saasForm.optimizeType === 'cname' ? resolvedCname : saasForm.optimizeIP }}</span>
            </div>
          </div>
          <div class="alert-info text-sm">
            <p class="font-medium mb-1">{{ t('quickDeploy.autoRenewTitle') }}</p>
            <p>{{ t('quickDeploy.autoRenewDesc', { domain: saasForm.customHostname }) }}</p>
          </div>
          <div class="flex justify-between pt-2">
            <button class="btn-island-secondary" @click="saasStep = 0">{{ t('quickDeploy.reconfigure') }}</button>
          </div>
        </div>
      </div>
    </template>

    <!-- Code Preview Modal -->
    <div v-if="showCodePreview" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="showCodePreview = false">
      <div class="glass-modal w-full max-w-3xl max-h-[90vh] overflow-y-auto" @click.stop>
        <div class="p-6 border-b border-border flex justify-between items-center">
          <h2 class="text-xl font-semibold">{{ t('quickDeploy.previewTitle') }}</h2>
          <button @click="showCodePreview = false" class="text-muted-foreground hover:text-foreground">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </div>
        <div class="p-6">
          <pre class="bg-muted p-4 rounded-lg text-xs overflow-x-auto"><code>{{ generatedCode }}</code></pre>
        </div>
        <div class="p-6 border-t border-border flex justify-end gap-3">
          <button class="btn-island-secondary" @click="copyText(generatedCode)">{{ t('quickDeploy.copyCode') }}</button>
          <button class="btn-island-primary" @click="showCodePreview = false">{{ t('quickDeploy.close') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import {
  RocketOutline,
  CheckmarkCircleOutline,
  SettingsOutline,
  GlobeOutline,
  ShieldCheckmarkOutline,
  ServerOutline,
  LinkOutline,
} from '@vicons/ionicons5'
import { cloudflareApi } from '@/api'
import { toast } from '@/utils/toast'
import { logHistory } from '@/utils/history'

const { t } = useI18n()
const mode = ref<'worker' | 'saas'>('saas')
const deploying = ref(false)
const showCodePreview = ref(false)
const deployResult = ref<{ name: string; url: string } | null>(null)

// Worker form
const workerForm = ref({
  sourceDomain: '',
  workerName: '',
  routePattern: '',
  cacheTTL: 3600,
})

// SaaS form
const saasStep = ref(0)
const saasSteps = computed(() => [
  t('quickDeploy.saasSteps.step0'),
  t('quickDeploy.saasSteps.step1'),
  t('quickDeploy.saasSteps.step2'),
  t('quickDeploy.saasSteps.step3'),
  t('quickDeploy.saasSteps.step4')
])
const saasLoading = ref(false)
const saasForm = ref({
  fallbackDomain: '',
  originIP: '',
  customHostname: '',
  optimizeType: 'cname' as 'cname' | 'ip',
  optimizeCname: 'cdn.anycast.eu.org',
  customCname: '',
  optimizeIP: '',
  hostnameResult: null as any,
})

const cnamePresets = computed(() => [
  { label: 'cdn.anycast.eu.org (全球)', value: 'cdn.anycast.eu.org' },
  { label: 'cdn-all.xn--b6gac.eu.org (全球)', value: 'cdn-all.xn--b6gac.eu.org' },
  { label: 'cloudflare.182682.xyz (亚洲优化)', value: 'cloudflare.182682.xyz' },
  { label: t('quickDeploy.customDomain') || '自定义域名', value: 'custom' },
])

const resolvedCname = computed(() => {
  if (saasForm.value.optimizeCname === 'custom') return saasForm.value.customCname
  return saasForm.value.optimizeCname
})

const isWorkerFormValid = computed(() => {
  return workerForm.value.sourceDomain && workerForm.value.workerName && workerForm.value.routePattern
})

const generatedCode = computed(() => generateWorkerScript())

function generateWorkerScript(): string {
  return `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  const url = new URL(request.url)
  url.hostname = '${workerForm.value.sourceDomain}'

  const newRequest = new Request(url.toString(), {
    method: request.method,
    headers: request.headers,
    body: request.body,
  })

  const response = await fetch(newRequest, {
    cf: {
      cacheTtl: ${workerForm.value.cacheTTL},
      cacheEverything: true,
    }
  })

  const newResponse = new Response(response.body, response)${workerForm.value.cacheTTL > 0 ? `
  newResponse.headers.set('Cache-Control', 'public, max-age=${workerForm.value.cacheTTL}')` : ''}

  return newResponse
}`.trim()
}

function copyText(text: string) {
  if (!text) return
  navigator.clipboard.writeText(text)
  toast.success(t('quickDeploy.clipboardCopied'))
}

async function findZoneForDomain(domain: string) {
  const zones = await cloudflareApi.getZones()
  const parts = domain.split('.')
  for (let i = 0; i < parts.length - 1; i++) {
    const candidate = parts.slice(i).join('.')
    const zone = zones.find((z: { name: string }) => z.name === candidate)
    if (zone) return zone
  }
  return null
}

async function autoSetupFallback() {
  saasLoading.value = true
  try {
    const zone = await findZoneForDomain(saasForm.value.fallbackDomain)
    if (!zone) {
      toast.error(t('quickDeploy.zoneNotFound'))
      return
    }
    // 1. 创建 A 记录指向源站（开启代理）
    await cloudflareApi.createDnsRecord({
      zoneId: zone.id,
      type: 'A',
      name: saasForm.value.fallbackDomain,
      content: saasForm.value.originIP,
      proxied: true,
    })
    // 2. 设置为回退源
    await cloudflareApi.setFallbackOrigin(zone.id, saasForm.value.fallbackDomain)
    toast.success(t('quickDeploy.fallbackSuccess'))
    saasStep.value = 2
  } catch (error: any) {
    toast.error(error.message || t('quickDeploy.fallbackFailed'))
  } finally {
    saasLoading.value = false
  }
}

async function autoCreateHostname() {
  saasLoading.value = true
  try {
    const zone = await findZoneForDomain(saasForm.value.fallbackDomain)
    if (!zone) {
      toast.error(t('quickDeploy.domainNotFound'))
      return
    }
    // 1. 创建自定义主机名
    const result = await cloudflareApi.createCustomHostname(zone.id, saasForm.value.customHostname)
    saasForm.value.hostnameResult = result

    // 2. 自动添加验证 TXT 记录
    const validationRecords = result?.ssl?.validation_records || []
    const hostnameZone = await findZoneForDomain(saasForm.value.customHostname)

    if (hostnameZone && validationRecords.length > 0) {
      for (const rec of validationRecords) {
        if (rec.txt_name && rec.txt_value) {
          await cloudflareApi.createDnsRecord({
            zoneId: hostnameZone.id,
            type: 'TXT',
            name: rec.txt_name,
            content: rec.txt_value,
            proxied: false,
          })
        }
        if (rec.cname && rec.cname_target) {
          await cloudflareApi.createDnsRecord({
            zoneId: hostnameZone.id,
            type: 'CNAME',
            name: rec.cname,
            content: rec.cname_target,
            proxied: false,
          })
        }
      }
      toast.success(t('quickDeploy.hostnameSuccessVerification'))
    } else {
      toast.success(t('quickDeploy.hostnameSuccess'))
    }
    saasStep.value = 3
  } catch (error: any) {
    toast.error(error.message || t('quickDeploy.hostnameFailed'))
  } finally {
    saasLoading.value = false
  }
}

async function autoCreateOptimizeDns() {
  saasLoading.value = true
  try {
    const zone = await findZoneForDomain(saasForm.value.customHostname)
    if (!zone) {
      toast.error(t('quickDeploy.zoneNotFoundAccess'))
      return
    }

    if (saasForm.value.optimizeType === 'cname') {
      await cloudflareApi.createDnsRecord({
        zoneId: zone.id,
        type: 'CNAME',
        name: saasForm.value.customHostname,
        content: resolvedCname.value,
        proxied: false,
      })
    } else {
      await cloudflareApi.createDnsRecord({
        zoneId: zone.id,
        type: 'A',
        name: saasForm.value.customHostname,
        content: saasForm.value.optimizeIP,
        proxied: false,
      })
    }
    toast.success(t('quickDeploy.dnsSuccess'))
    saasStep.value = 4
  } catch (error: any) {
    toast.error(error.message || t('quickDeploy.dnsFailed'))
  } finally {
    saasLoading.value = false
  }
}

async function deployWorker() {
  if (!isWorkerFormValid.value) return
  deploying.value = true
  deployResult.value = null
  try {
    const script = generateWorkerScript()
    await cloudflareApi.createWorker(workerForm.value.workerName, script)
    if (workerForm.value.routePattern) {
      try {
        await cloudflareApi.createWorkerRoute(workerForm.value.routePattern, workerForm.value.workerName)
      } catch (err) {
        console.error('Failed to create route:', err)
      }
    }
    deployResult.value = { name: workerForm.value.workerName, url: workerForm.value.routePattern }
    logHistory.worker(t('quickDeploy.workerDeployTitle'), `Worker: ${workerForm.value.workerName}`)
    toast.success(t('quickDeploy.workerDeploySuccess'))
  } catch (error: any) {
    toast.error(error.message || t('quickDeploy.deployFailed'))
  } finally {
    deploying.value = false
  }
}
</script>