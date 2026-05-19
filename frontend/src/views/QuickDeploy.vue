<template>
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">一键优选</h1>
      <p class="text-sm text-muted-foreground mt-1">通过 Cloudflare 加速您的网站访问</p>
    </div>

    <!-- Mode Selector -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
      <button
        @click="mode = 'worker'"
        :class="['metric-card p-5 text-left transition-all', mode === 'worker' ? 'border-primary ring-2 ring-primary/20' : '']"
      >
        <div class="flex items-center gap-3 mb-3">
          <component :is="SettingsOutline" class="w-5 h-5 text-primary" />
          <span class="font-semibold">Worker 反代</span>
        </div>
        <p class="text-xs text-muted-foreground mb-3">在 Cloudflare 边缘节点运行代理脚本，将请求转发到源站并缓存响应</p>
        <div class="text-xs space-y-1">
          <div class="text-green-600">+ 部署简单，一键完成</div>
          <div class="text-green-600">+ 可自定义逻辑（改写URL、加Header）</div>
          <div class="text-green-600">+ 无需额外域名或支付方式</div>
          <div class="text-muted-foreground">- 每次请求消耗 Worker 配额（免费10万次/天）</div>
          <div class="text-muted-foreground">- 有 CPU 执行时间限制（免费版10ms）</div>
          <div class="text-muted-foreground">- 无法使用优选 IP 加速</div>
        </div>
      </button>
      <button
        @click="mode = 'saas'"
        :class="['metric-card p-5 text-left transition-all', mode === 'saas' ? 'border-primary ring-2 ring-primary/20' : '']"
      >
        <div class="flex items-center gap-3 mb-3">
          <component :is="GlobeOutline" class="w-5 h-5 text-primary" />
          <span class="font-semibold">SaaS 优选</span>
          <span class="glass-badge glass-badge-info text-[10px]">推荐</span>
        </div>
        <p class="text-xs text-muted-foreground mb-3">通过 Cloudflare for SaaS 自定义主机名，配合优选 CNAME/IP 实现最佳加速</p>
        <div class="text-xs space-y-1">
          <div class="text-green-600">+ 走 CDN 原生缓存层，性能最优</div>
          <div class="text-green-600">+ 不消耗 Worker 配额，无请求限制</div>
          <div class="text-green-600">+ 支持优选 IP/CNAME，大陆访问更快</div>
          <div class="text-green-600">+ 自动 SSL 证书签发和续期</div>
          <div class="text-muted-foreground">- 需绑定支付方式开通（前100主机名免费）</div>
          <div class="text-muted-foreground">- 配置步骤较多，需手动操作 Dashboard</div>
        </div>
      </button>
    </div>

    <!-- Worker Mode -->
    <template v-if="mode === 'worker'">
      <div class="banner-gradient rounded-lg p-5 mb-6">
        <h3 class="font-semibold mb-1 flex items-center gap-2">
          <component :is="RocketOutline" class="w-5 h-5 text-primary" /> 三步完成部署
        </h3>
        <div class="text-sm text-muted-foreground space-y-0.5">
          <div>1. 填写源站域名和 Worker 名称</div>
          <div>2. 配置缓存策略（可选）</div>
          <div>3. 点击部署，自动创建 Worker</div>
        </div>
      </div>

      <div class="metric-card p-6 mb-6">
        <h3 class="font-semibold mb-4">部署配置</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium mb-2">源站域名 *</label>
            <input v-model="workerForm.sourceDomain" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="example.com" />
            <p class="text-xs text-muted-foreground mt-1">要加速的原始网站域名</p>
          </div>
          <div>
            <label class="block text-sm font-medium mb-2">Worker 名称 *</label>
            <input v-model="workerForm.workerName" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="cdn-accelerator" />
          </div>
          <div>
            <label class="block text-sm font-medium mb-2">路由模式 *</label>
            <input v-model="workerForm.routePattern" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="cdn.example.com/*" />
          </div>
          <div>
            <label class="block text-sm font-medium mb-2">缓存时间</label>
            <select v-model="workerForm.cacheTTL" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50">
              <option :value="0">不缓存</option>
              <option :value="300">5 分钟</option>
              <option :value="3600">1 小时</option>
              <option :value="86400">1 天</option>
              <option :value="604800">1 周</option>
            </select>
          </div>
          <div class="flex gap-3 pt-4">
            <button @click="deployWorker" :disabled="deploying || !isWorkerFormValid" class="btn-island-primary flex-1">
              <template v-if="deploying">部署中...</template>
              <template v-else><component :is="RocketOutline" class="w-4 h-4 mr-1" /> 开始部署</template>
            </button>
            <button @click="showCodePreview = true" class="btn-island-secondary">预览代码</button>
          </div>
        </div>
      </div>

      <div v-if="deployResult" class="metric-card p-6">
        <h3 class="font-semibold mb-3 flex items-center gap-2">
          <component :is="CheckmarkCircleOutline" class="w-5 h-5 text-green-600" /> 部署成功
        </h3>
        <div class="space-y-2 text-sm">
          <div class="flex justify-between"><span class="text-muted-foreground">Worker:</span><span class="font-mono">{{ deployResult.name }}</span></div>
          <div class="flex justify-between"><span class="text-muted-foreground">路由:</span><code class="text-xs bg-muted px-2 py-1 rounded">{{ deployResult.url }}</code></div>
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
          <component :is="ShieldCheckmarkOutline" class="w-5 h-5 text-primary" /> 前置准备
        </h3>
        <div class="space-y-4">
          <div class="alert-info">
            <p class="font-medium mb-2">开通 Cloudflare for SaaS</p>
            <p class="text-sm">需要在 Cloudflare 账户中绑定支付方式（支持信用卡/虚拟卡），以启用自定义主机名功能。每月前 100 个自定义主机名免费。</p>
          </div>
          <div class="text-sm space-y-2">
            <p class="font-medium">操作步骤：</p>
            <ol class="list-decimal list-inside space-y-1 text-muted-foreground">
              <li>登录 Cloudflare Dashboard</li>
              <li>进入目标域名 → SSL/TLS → 自定义主机名</li>
              <li>按提示绑定支付方式并启用功能</li>
            </ol>
          </div>
          <div class="flex justify-end pt-2">
            <button class="btn-island-primary" @click="saasStep = 1">已完成，下一步</button>
          </div>
        </div>
      </div>

      <!-- Step 1: Fallback Origin -->
      <div v-if="saasStep === 1" class="metric-card p-6">
        <h3 class="font-semibold mb-4 flex items-center gap-2">
          <component :is="ServerOutline" class="w-5 h-5 text-primary" /> 配置回退源
        </h3>
        <div class="space-y-4">
          <p class="text-sm text-muted-foreground">回退源是自定义主机名流量的目标地址。需要创建一条代理的 DNS 记录指向您的源站。</p>
          <div>
            <label class="block text-sm font-medium mb-2">回退源子域名 *</label>
            <input v-model="saasForm.fallbackDomain" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="fallback.example.com" />
            <p class="text-xs text-muted-foreground mt-1">建议使用 fallback 或 proxy-fallback 作为子域名前缀</p>
          </div>
          <div>
            <label class="block text-sm font-medium mb-2">源站 IP 地址 *</label>
            <input v-model="saasForm.originIP" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="1.2.3.4" />
          </div>
          <div class="alert-info text-sm">
            <p class="font-medium mb-1">需要手动操作：</p>
            <p>在 Cloudflare Dashboard → SSL/TLS → 自定义主机名 页面，将 <code class="bg-muted px-1 rounded">{{ saasForm.fallbackDomain || 'fallback.example.com' }}</code> 设置为回退源。</p>
          </div>
          <div class="flex justify-between pt-2">
            <button class="btn-island-secondary" @click="saasStep = 0">上一步</button>
            <div class="flex gap-3">
              <button class="btn-island-secondary" @click="createFallbackDns" :disabled="!saasForm.fallbackDomain || !saasForm.originIP">创建 DNS 记录</button>
              <button class="btn-island-primary" @click="saasStep = 2">下一步</button>
            </div>
          </div>
        </div>
      </div>

      <!-- Step 2: Custom Hostname -->
      <div v-if="saasStep === 2" class="metric-card p-6">
        <h3 class="font-semibold mb-4 flex items-center gap-2">
          <component :is="LinkOutline" class="w-5 h-5 text-primary" /> 添加自定义主机名
        </h3>
        <div class="space-y-4">
          <p class="text-sm text-muted-foreground">在 Cloudflare Dashboard 中添加自定义主机名，将用户的访问域名关联到您的回退源。</p>
          <div>
            <label class="block text-sm font-medium mb-2">优选访问域名 *</label>
            <input v-model="saasForm.customHostname" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="cdn.yourdomain.com" />
            <p class="text-xs text-muted-foreground mt-1">用户最终通过此域名访问您的网站</p>
          </div>
          <div class="alert-info text-sm">
            <p class="font-medium mb-2">在 Cloudflare Dashboard 操作：</p>
            <ol class="list-decimal list-inside space-y-1">
              <li>进入 SSL/TLS → 自定义主机名 → 添加自定义主机名</li>
              <li>输入域名：<code class="bg-muted px-1 rounded cursor-pointer" @click="copyText(saasForm.customHostname)">{{ saasForm.customHostname || 'cdn.yourdomain.com' }}</code></li>
              <li>验证方式选择 TXT 记录</li>
              <li>将 Cloudflare 提供的 TXT 验证记录添加到您的 DNS</li>
            </ol>
          </div>
          <div class="flex justify-between pt-2">
            <button class="btn-island-secondary" @click="saasStep = 1">上一步</button>
            <button class="btn-island-primary" @click="saasStep = 3">下一步</button>
          </div>
        </div>
      </div>

      <!-- Step 3: DNS Optimization -->
      <div v-if="saasStep === 3" class="metric-card p-6">
        <h3 class="font-semibold mb-4 flex items-center gap-2">
          <component :is="GlobeOutline" class="w-5 h-5 text-primary" /> DNS 优选配置
        </h3>
        <div class="space-y-4">
          <p class="text-sm text-muted-foreground">将访问域名的 DNS 指向优选 CNAME 或 IP，让用户连接到最快的 Cloudflare 节点。</p>
          <div>
            <label class="block text-sm font-medium mb-2">优选方式</label>
            <select v-model="saasForm.optimizeType" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50">
              <option value="cname">CNAME 优选域名（推荐）</option>
              <option value="ip">自定义优选 IP</option>
            </select>
          </div>
          <div v-if="saasForm.optimizeType === 'cname'">
            <label class="block text-sm font-medium mb-2">选择优选域名</label>
            <select v-model="saasForm.optimizeCname" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50">
              <option v-for="item in cnamePresets" :key="item.value" :value="item.value">{{ item.label }}</option>
            </select>
            <input v-if="saasForm.optimizeCname === 'custom'" v-model="saasForm.customCname" class="w-full mt-2 px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="输入自定义优选域名" />
          </div>
          <div v-else>
            <label class="block text-sm font-medium mb-2">优选 IP 地址</label>
            <input v-model="saasForm.optimizeIP" class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50" placeholder="104.16.x.x" />
          </div>
          <div class="alert-warning text-sm">
            <p class="font-medium mb-1">DNS 配置说明：</p>
            <p>在 <code class="bg-muted px-1 rounded">{{ saasForm.customHostname || 'cdn.yourdomain.com' }}</code> 的 DNS 管理处，添加以下记录（不开启代理/灰色云朵）：</p>
            <div class="mt-2 bg-muted/50 p-3 rounded-lg font-mono text-xs">
              <template v-if="saasForm.optimizeType === 'cname'">
                {{ saasForm.customHostname || 'cdn' }} CNAME {{ resolvedCname }}
              </template>
              <template v-else>
                {{ saasForm.customHostname || 'cdn' }} A {{ saasForm.optimizeIP || '104.16.x.x' }}
              </template>
            </div>
          </div>
          <div class="flex justify-between pt-2">
            <button class="btn-island-secondary" @click="saasStep = 2">上一步</button>
            <button class="btn-island-primary" @click="saasStep = 4">完成配置</button>
          </div>
        </div>
      </div>

      <!-- Step 4: Complete -->
      <div v-if="saasStep === 4" class="metric-card p-6">
        <h3 class="font-semibold mb-4 flex items-center gap-2">
          <component :is="CheckmarkCircleOutline" class="w-5 h-5 text-green-600" /> 配置完成
        </h3>
        <div class="space-y-4">
          <div class="space-y-3 text-sm">
            <div class="flex justify-between py-2 border-b border-border">
              <span class="text-muted-foreground">回退源</span>
              <span class="font-mono">{{ saasForm.fallbackDomain }}</span>
            </div>
            <div class="flex justify-between py-2 border-b border-border">
              <span class="text-muted-foreground">源站 IP</span>
              <span class="font-mono">{{ saasForm.originIP }}</span>
            </div>
            <div class="flex justify-between py-2 border-b border-border">
              <span class="text-muted-foreground">访问域名</span>
              <span class="font-mono">{{ saasForm.customHostname }}</span>
            </div>
            <div class="flex justify-between py-2">
              <span class="text-muted-foreground">优选指向</span>
              <span class="font-mono">{{ saasForm.optimizeType === 'cname' ? resolvedCname : saasForm.optimizeIP }}</span>
            </div>
          </div>
          <div class="alert-info text-sm">
            <p class="font-medium mb-1">证书自动续期（推荐）：</p>
            <p>将 <code class="bg-muted px-1 rounded">_acme-challenge.{{ saasForm.customHostname }}</code> 设置为 CNAME 记录，指向 Cloudflare 提供的 DCV 委派域名，即可实现证书自动续期。</p>
          </div>
          <div class="flex justify-between pt-2">
            <button class="btn-island-secondary" @click="saasStep = 0">重新配置</button>
          </div>
        </div>
      </div>
    </template>

    <!-- Code Preview Modal -->
    <div v-if="showCodePreview" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="showCodePreview = false">
      <div class="glass-modal w-full max-w-3xl max-h-[90vh] overflow-y-auto" @click.stop>
        <div class="p-6 border-b border-border flex justify-between items-center">
          <h2 class="text-xl font-semibold">Worker 代码预览</h2>
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
          <button class="btn-island-secondary" @click="copyText(generatedCode)">复制代码</button>
          <button class="btn-island-primary" @click="showCodePreview = false">关闭</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
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
const saasSteps = ['前置准备', '配置回退源', '自定义主机名', 'DNS 优选', '完成']
const saasForm = ref({
  fallbackDomain: '',
  originIP: '',
  customHostname: '',
  optimizeType: 'cname' as 'cname' | 'ip',
  optimizeCname: 'cdn.anycast.eu.org',
  customCname: '',
  optimizeIP: '',
})

const cnamePresets = [
  { label: 'cdn.anycast.eu.org (全球)', value: 'cdn.anycast.eu.org' },
  { label: 'cdn-all.xn--b6gac.eu.org (全球)', value: 'cdn-all.xn--b6gac.eu.org' },
  { label: 'cloudflare.182682.xyz (亚洲优化)', value: 'cloudflare.182682.xyz' },
  { label: '自定义域名', value: 'custom' },
]

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
  navigator.clipboard.writeText(text)
  toast.success('已复制到剪贴板')
}

async function createFallbackDns() {
  try {
    const zones = await cloudflareApi.getZones()
    const domain = saasForm.value.fallbackDomain
    const rootDomain = domain.split('.').slice(-2).join('.')
    const zone = zones.find((z: { name: string }) => z.name === rootDomain)
    if (!zone) {
      toast.error('未找到对应域名，请确认域名已添加到 Cloudflare')
      return
    }
    await cloudflareApi.createDnsRecord({
      zoneId: zone.id,
      type: 'A',
      name: domain,
      content: saasForm.value.originIP,
      proxied: true,
    })
    toast.success('DNS 记录创建成功')
  } catch (error: any) {
    toast.error(error.message || '创建 DNS 记录失败')
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
    logHistory.worker('一键优选部署', `Worker: ${workerForm.value.workerName}`)
    toast.success('Worker 部署成功')
  } catch (error: any) {
    toast.error(error.message || '部署失败')
  } finally {
    deploying.value = false
  }
}
</script>