<template>
  <!-- Worker Templates - Island Theme with Full Library -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">{{ t('workerTemplates.title') }}</h1>
      <p class="text-sm text-muted-foreground mt-1">{{ t('workerTemplates.subtitle') }}</p>
    </div>

    <!-- Search & Filter -->
    <div class="metric-card p-4 mb-6">
      <div class="flex gap-4">
        <input
          v-model="searchQuery"
          :placeholder="t('workerTemplates.searchPlaceholder')"
          class="flex-1 px-3 py-2 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
        />
        <select
          v-model="categoryFilter"
          class="px-3 py-2 bg-background border border-border rounded-lg text-sm"
        >
          <option value="">{{ t('workerTemplates.allCategories') }}</option>
          <option value="Starter">{{ t('workerTemplates.cat.starter') }}</option>
          <option value="API">{{ t('workerTemplates.cat.api') }}</option>
          <option value="Performance">{{ t('workerTemplates.cat.performance') }}</option>
          <option value="Security">{{ t('workerTemplates.cat.security') }}</option>
          <option value="Media">{{ t('workerTemplates.cat.media') }}</option>
          <option value="Experiment">{{ t('workerTemplates.cat.experiment') }}</option>
        </select>
      </div>
    </div>

    <!-- Templates Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div 
        v-for="template in filteredTemplates" 
        :key="template.id"
        class="metric-card p-6 hover:border-primary transition-all cursor-pointer group"
        @click="viewTemplate(template)"
      >
        <div class="text-4xl mb-4"><component :is="template.icon" class="w-7 h-7" /></div>
        
        <h3 class="font-semibold mb-2 group-hover:text-primary transition-colors">
          {{ template.name }}
        </h3>
        
        <p class="text-sm text-muted-foreground mb-4 line-clamp-2">
          {{ template.description }}
        </p>
        
        <div class="flex items-center justify-between">
          <span class="px-2 py-1 text-xs rounded-full bg-primary/10 text-primary">
            {{ template.category }}
          </span>
          <span class="text-xs text-muted-foreground">
            {{ template.difficulty }}
          </span>
        </div>
      </div>
    </div>

    <!-- Template Detail Modal -->
    <div v-if="selectedTemplate" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="selectedTemplate = null">
      <div class="glass-modal w-full max-w-4xl max-h-[90vh] overflow-y-auto" @click.stop>
        <div class="p-6 border-b border-border flex justify-between items-center">
          <div class="flex items-center gap-3">
            <span class="text-4xl"><component :is="selectedTemplate.icon" class="w-7 h-7" /></span>
            <div>
              <h2 class="text-xl font-semibold">{{ selectedTemplate.name }}</h2>
              <p class="text-sm text-muted-foreground">{{ selectedTemplate.category }}</p>
            </div>
          </div>
          <button @click="selectedTemplate = null" class="text-muted-foreground hover:text-foreground">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </div>
        
        <div class="p-6 space-y-6">
          <!-- Description -->
          <div>
            <h3 class="font-semibold mb-2">{{ t('workerTemplates.descHeader') }}</h3>
            <p class="text-sm text-muted-foreground">{{ selectedTemplate.fullDescription }}</p>
          </div>

          <!-- Features -->
          <div v-if="selectedTemplate.features">
            <h3 class="font-semibold mb-2">{{ t('workerTemplates.featHeader') }}</h3>
            <ul class="space-y-1 text-sm text-muted-foreground">
              <li v-for="feature in selectedTemplate.features" :key="feature" class="flex items-start gap-2">
                <component :is="CheckmarkOutline" class="w-4 h-4 text-primary inline mt-0.5" />
                <span>{{ feature }}</span>
              </li>
            </ul>
          </div>

          <!-- Code Preview -->
          <div>
            <h3 class="font-semibold mb-2">{{ t('workerTemplates.codeHeader') }}</h3>
            <pre class="bg-muted p-4 rounded-lg text-xs overflow-x-auto"><code>{{ selectedTemplate.code }}</code></pre>
          </div>
        </div>

        <div class="p-6 border-t border-border flex justify-end gap-3">
          <button class="btn-island-secondary" @click="copyCode(selectedTemplate.code)">
            {{ t('workerTemplates.copyCode') }}
          </button>
          <button class="btn-island-primary" @click="useTemplate(selectedTemplate)">
            {{ t('workerTemplates.useTemplate') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, type Component } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { toast } from '@/utils/toast'
import { HandLeftOutline, EnterOutline, FlashOutline, LockClosedOutline, ImageOutline, BeakerOutline, CheckmarkOutline } from '@vicons/ionicons5'

interface WorkerTemplate {
  id: string
  name: string
  description: string
  fullDescription: string
  category: string
  difficulty: string
  icon: Component
  features?: string[]
  code: string
}

const { t } = useI18n()
const router = useRouter()
const searchQuery = ref('')
const categoryFilter = ref('')
const selectedTemplate = ref<WorkerTemplate | null>(null)

const templates = computed<WorkerTemplate[]>(() => [
  {
    id: '1',
    name: t('workerTemplates.t1.name'),
    description: t('workerTemplates.t1.desc'),
    fullDescription: t('workerTemplates.t1.fullDesc'),
    category: t('workerTemplates.cat.starter'),
    difficulty: t('workerTemplates.diff.easy'),
    icon: HandLeftOutline,
    features: [
      t('workerTemplates.t1.feat1'),
      t('workerTemplates.t1.feat2'),
      t('workerTemplates.t1.feat3'),
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  return new Response('Hello World!', {
    headers: { 'content-type': 'text/plain' },
  })
}`,
  },
  {
    id: '2',
    name: t('workerTemplates.t2.name'),
    description: t('workerTemplates.t2.desc'),
    fullDescription: t('workerTemplates.t2.fullDesc'),
    category: t('workerTemplates.cat.api'),
    difficulty: t('workerTemplates.diff.medium'),
    icon: EnterOutline,
    features: [
      t('workerTemplates.t2.feat1'),
      t('workerTemplates.t2.feat2'),
      t('workerTemplates.t2.feat3'),
      t('workerTemplates.t2.feat4'),
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  const url = new URL(request.url)
  
  // 路由匹配
  if (url.pathname === '/api/users') {
    return handleUsers(request)
  } else if (url.pathname === '/api/posts') {
    return handlePosts(request)
  }
  
  return new Response('Not Found', { status: 404 })
}

async function handleUsers(request) {
  const users = [
    { id: 1, name: 'Alice' },
    { id: 2, name: 'Bob' },
  ]
  
  return new Response(JSON.stringify(users), {
    headers: {
      'content-type': 'application/json',
      'access-control-allow-origin': '*',
    },
  })
}

async function handlePosts(request) {
  // 实现帖子处理逻辑
  return new Response(JSON.stringify([]), {
    headers: { 'content-type': 'application/json' },
  })
}`,
  },
  {
    id: '3',
    name: t('workerTemplates.t3.name'),
    description: t('workerTemplates.t3.desc'),
    fullDescription: t('workerTemplates.t3.fullDesc'),
    category: t('workerTemplates.cat.performance'),
    difficulty: t('workerTemplates.diff.medium'),
    icon: FlashOutline,
    features: [
      t('workerTemplates.t3.feat1'),
      t('workerTemplates.t3.feat2'),
      t('workerTemplates.t3.feat3'),
      t('workerTemplates.t3.feat4'),
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  const url = new URL(request.url)
  
  // 修改源站地址
  url.hostname = 'origin.example.com'
  
  // 创建缓存键
  const cacheKey = new Request(url.toString(), request)
  const cache = caches.default
  
  // 检查缓存
  let response = await cache.match(cacheKey)
  
  if (!response) {
    // 缓存未命中，从源站获取
    response = await fetch(request, {
      cf: {
        cacheTtl: 3600,
        cacheEverything: true,
      }
    })
    
    // 克隆响应用于缓存
    response = new Response(response.body, response)
    response.headers.set('Cache-Control', 'public, max-age=3600')
    
    // 存入缓存
    event.waitUntil(cache.put(cacheKey, response.clone()))
  }
  
  return response
}`,
  },
  {
    id: '4',
    name: t('workerTemplates.t4.name'),
    description: t('workerTemplates.t4.desc'),
    fullDescription: t('workerTemplates.t4.fullDesc'),
    category: t('workerTemplates.cat.security'),
    difficulty: t('workerTemplates.diff.hard'),
    icon: LockClosedOutline,
    features: [
      t('workerTemplates.t4.feat1'),
      t('workerTemplates.t4.feat2'),
      t('workerTemplates.t4.feat3'),
      t('workerTemplates.t4.feat4'),
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  // 获取 Authorization 头
  const authHeader = request.headers.get('Authorization')
  
  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    return new Response('Unauthorized', { status: 401 })
  }
  
  const token = authHeader.substring(7)
  
  try {
    // 验证 JWT (简化示例)
    const payload = await verifyJWT(token)
    
    // 验证通过，继续处理请求
    return new Response(JSON.stringify({
      message: '认证成功',
      user: payload.sub,
    }), {
      headers: { 'content-type': 'application/json' },
    })
  } catch (err) {
    return new Response('Invalid Token', { status: 403 })
  }
}

async function verifyJWT(token) {
  // JWT 验证逻辑（需要添加实际的验证代码）
  const parts = token.split('.')
  if (parts.length !== 3) throw new Error('Invalid token')
  
  // 解析 payload
  const payload = JSON.parse(atob(parts[1]))
  return payload
}`,
  },
  {
    id: '5',
    name: t('workerTemplates.t5.name'),
    description: t('workerTemplates.t5.desc'),
    fullDescription: t('workerTemplates.t5.fullDesc'),
    category: t('workerTemplates.cat.media'),
    difficulty: t('workerTemplates.diff.medium'),
    icon: ImageOutline,
    features: [
      t('workerTemplates.t5.feat1'),
      t('workerTemplates.t5.feat2'),
      t('workerTemplates.t5.feat3'),
      t('workerTemplates.t5.feat4'),
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  const url = new URL(request.url)
  
  // 解析图片参数
  const width = url.searchParams.get('w')
  const quality = url.searchParams.get('q') || '85'
  
  // 获取原始图片
  const imageUrl = url.pathname.substring(1)
  const imageRequest = new Request(imageUrl)
  
  // 使用 Cloudflare Image Resizing
  const options = {
    cf: {
      image: {
        width: width ? parseInt(width) : undefined,
        quality: parseInt(quality),
        format: 'auto', // 自动选择最佳格式
      }
    }
  }
  
  return fetch(imageRequest, options)
}`,
  },
  {
    id: '6',
    name: t('workerTemplates.t6.name'),
    description: t('workerTemplates.t6.desc'),
    fullDescription: t('workerTemplates.t6.fullDesc'),
    category: t('workerTemplates.cat.experiment'),
    difficulty: t('workerTemplates.diff.medium'),
    icon: BeakerOutline,
    features: [
      t('workerTemplates.t6.feat1'),
      t('workerTemplates.t6.feat2'),
      t('workerTemplates.t6.feat3'),
      t('workerTemplates.t6.feat4'),
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  const url = new URL(request.url)
  
  // 检查是否已有测试分组
  const cookies = request.headers.get('cookie') || ''
  let variant = cookies.match(/variant=([AB])/)?.[1]
  
  if (!variant) {
    // 随机分配
    variant = Math.random() < 0.5 ? 'A' : 'B'
  }
  
  // 根据分组返回不同内容
  let html
  if (variant === 'A') {
    html = '<h1>版本 A</h1><p>原始版本</p>'
  } else {
    html = '<h1>版本 B</h1><p>测试版本</p>'
  }
  
  return new Response(html, {
    headers: {
      'content-type': 'text/html',
      'set-cookie': \`variant=\${variant}; Path=/; Max-Age=86400\`,
    },
  })
}`,
  },
])

const filteredTemplates = computed(() => {
  return templates.value.filter(template => {
    const matchesSearch = template.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
                         template.description.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchesCategory = !categoryFilter.value || 
                            template.category === t(`workerTemplates.cat.${categoryFilter.value.toLowerCase()}`)
    return matchesSearch && matchesCategory
  })
})

function viewTemplate(template: WorkerTemplate) {
  selectedTemplate.value = template
}

function copyCode(code: string) {
  navigator.clipboard.writeText(code)
  toast.success(t('workerTemplates.copySuccess'))
}

function useTemplate(template: WorkerTemplate) {
  // Store template code in localStorage for use in Workers page
  localStorage.setItem('workerTemplate', JSON.stringify({
    name: template.name.toLowerCase().replace(/\s+/g, '-'),
    script: template.code,
  }))
  
  toast.success(t('workerTemplates.useSuccess'))
  setTimeout(() => {
    router.push('/workers')
  }, 1000)
}
</script>
