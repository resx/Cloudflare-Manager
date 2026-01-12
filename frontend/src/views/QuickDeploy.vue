<template>
  <!-- Quick Deploy - Island Theme with Full Functionality -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">一键加速部署</h1>
      <p class="text-sm text-muted-foreground mt-1">
        快速部署 Cloudflare Worker 实现 CDN 加速
      </p>
    </div>

    <!-- Step Guide -->
    <div class="banner-gradient rounded-lg p-6 mb-6">
      <h3 class="text-lg font-semibold mb-2">🚀 三步完成部署</h3>
      <div class="text-sm text-muted-foreground space-y-1">
        <div>1. 填写源站域名和访问域名</div>
        <div>2. 配置缓存策略（可选）</div>
        <div>3. 点击部署，自动创建 Worker</div>
      </div>
    </div>

    <!-- Deployment Form -->
    <div class="metric-card p-6 mb-6">
      <h3 class="font-semibold mb-4">部署配置</h3>
      
      <div class="space-y-4">
        <!-- Source Domain -->
        <div>
          <label class="block text-sm font-medium mb-2">源站域名 *</label>
          <input
            v-model="form.sourceDomain"
            class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
            placeholder="example.com"
          />
          <p class="text-xs text-muted-foreground mt-1">要加速的原始网站域名</p>
        </div>

        <!-- Worker Name -->
        <div>
          <label class="block text-sm font-medium mb-2">Worker 名称 *</label>
          <input
            v-model="form.workerName"
            class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
            placeholder="cdn-accelerator"
          />
          <p class="text-xs text-muted-foreground mt-1">Worker 脚本名称（小写字母、数字、连字符）</p>
        </div>

        <!-- Route Pattern -->
        <div>
          <label class="block text-sm font-medium mb-2">路由模式 *</label>
          <input
            v-model="form.routePattern"
            class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
            placeholder="cdn.example.com/*"
          />
          <p class="text-xs text-muted-foreground mt-1">Worker 将响应此路由的请求</p>
        </div>

        <!-- Cache TTL -->
        <div>
          <label class="block text-sm font-medium mb-2">缓存时间（秒）</label>
          <select
            v-model="form.cacheTTL"
            class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
          >
            <option :value="0">不缓存</option>
            <option :value="300">5 分钟</option>
            <option :value="1800">30 分钟</option>
            <option :value="3600">1 小时</option>
            <option :value="7200">2 小时</option>
            <option :value="14400">4 小时</option>
            <option :value="86400">1 天</option>
            <option :value="604800">1 周</option>
            <option :value="2592000">1 个月</option>
          </select>
        </div>

        <!-- Custom Headers -->
        <div>
          <label class="flex items-center gap-2 text-sm font-medium mb-2">
            <input
              type="checkbox"
              v-model="form.addHeaders"
              class="w-4 h-4 rounded border-border"
            />
            添加自定义响应头
          </label>
          <div v-if="form.addHeaders" class="space-y-2 ml-6">
            <div v-for="(header, index) in form.headers" :key="index" class="flex gap-2">
              <input
                v-model="header.key"
                placeholder="Header 名称"
                class="flex-1 px-3 py-2 bg-background border border-border rounded-lg text-sm"
              />
              <input
                v-model="header.value"
                placeholder="Header 值"
                class="flex-1 px-3 py-2 bg-background border border-border rounded-lg text-sm"
              />
              <button
                @click="form.headers.splice(index, 1)"
                class="px-3 py-2 text-red-600 hover:bg-red-50 rounded-lg"
              >
                删除
              </button>
            </div>
            <button
              @click="form.headers.push({ key: '', value: '' })"
              class="btn-island-secondary text-xs"
            >
              + 添加 Header
            </button>
          </div>
        </div>

        <!-- Deploy Button -->
        <div class="flex gap-3 pt-4">
          <button
            @click="deployWorker"
            :disabled="deploying || !isFormValid"
            class="btn-island-primary flex-1"
          >
            {{ deploying ? '部署中...' : '🚀 开始部署' }}
          </button>
          <button
            @click="previewCode"
            class="btn-island-secondary"
          >
            预览代码
          </button>
        </div>
      </div>
    </div>

    <!-- Deployment Result -->
    <div v-if="deployResult" class="metric-card p-6 mb-6">
      <h3 class="font-semibold mb-3 text-success-foreground">✅ 部署成功！</h3>
      <div class="space-y-2 text-sm">
        <div class="flex justify-between">
          <span class="text-muted-foreground">Worker ID:</span>
          <span class="font-mono">{{ deployResult.id }}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground">Worker 名称:</span>
          <span class="font-medium">{{ deployResult.name }}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground">访问地址:</span>
          <code class="text-xs bg-muted px-2 py-1 rounded">{{ deployResult.url }}</code>
        </div>
      </div>
    </div>

    <!-- Code Preview Modal -->
    <div v-if="showCodePreview" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="showCodePreview = false">
      <div class="bg-white rounded-2xl shadow-lg w-full max-w-3xl max-h-[90vh] overflow-y-auto" @click.stop>
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
          <button class="btn-island-secondary" @click="copyCode">复制代码</button>
          <button class="btn-island-primary" @click="showCodePreview = false">关闭</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { cloudflareApi } from '@/api'
import { toast } from '@/utils/toast'
import { logHistory } from '@/utils/history'

const deploying = ref(false)
const showCodePreview = ref(false)
const deployResult = ref<any>(null)

const form = ref({
  sourceDomain: '',
  workerName: '',
  routePattern: '',
  cacheTTL: 3600,
  addHeaders: false,
  headers: [] as Array<{ key: string; value: string }>,
})

const isFormValid = computed(() => {
  return form.value.sourceDomain && form.value.workerName && form.value.routePattern
})

const generatedCode = computed(() => {
  return generateWorkerScript()
})

function generateWorkerScript(): string {
  const headers = form.value.addHeaders 
    ? form.value.headers.filter(h => h.key && h.value)
    : []

  return `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  // 修改目标 URL
  const url = new URL(request.url)
  url.hostname = '${form.value.sourceDomain}'
  
  // 创建新请求
  const newRequest = new Request(url.toString(), {
    method: request.method,
    headers: request.headers,
    body: request.body,
  })
  
  // 获取响应
  const response = await fetch(newRequest, {
    cf: {
      cacheTtl: ${form.value.cacheTTL},
      cacheEverything: true,
    }
  })
  
  // 创建新响应
  const newResponse = new Response(response.body, response)
  
  // 设置缓存控制${form.value.cacheTTL > 0 ? `
  newResponse.headers.set('Cache-Control', 'public, max-age=${form.value.cacheTTL}')` : ''}
  ${headers.length > 0 ? `
  // 添加自定义 Headers
${headers.map(h => `  newResponse.headers.set('${h.key}', '${h.value}')`).join('\n')}` : ''}
  
  return newResponse
}
`.trim()
}

function previewCode() {
  showCodePreview.value = true
}

function copyCode() {
  navigator.clipboard.writeText(generatedCode.value)
  toast.success('代码已复制到剪贴板')
}

async function deployWorker() {
  if (!isFormValid.value) {
    toast.warning('请填写所有必填字段')
    return
  }

  deploying.value = true
  deployResult.value = null

  try {
    const script = generateWorkerScript()
    
    // Create worker
    const result = await cloudflareApi.createWorker(form.value.workerName, script)
    
    // Add route if zone is available
    if (form.value.routePattern) {
      try {
        await cloudflareApi.createWorkerRoute(form.value.routePattern, form.value.workerName)
      } catch (err) {
        console.error('Failed to create route:', err)
      }
    }
    
    deployResult.value = {
      id: result.id || 'N/A',
      name: form.value.workerName,
      url: form.value.routePattern,
    }
    
    logHistory.worker('一键加速部署', `Worker: ${form.value.workerName}, 源站: ${form.value.sourceDomain}`)
    toast.success('Worker 部署成功！')
  } catch (error: any) {
    console.error('Failed to deploy worker:', error)
    toast.error(error.message || '部署失败')
  } finally {
    deploying.value = false
  }
}
</script>
