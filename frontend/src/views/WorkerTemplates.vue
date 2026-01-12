<template>
  <!-- Worker Templates - Island Theme with Full Library -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">Worker 模板库</h1>
      <p class="text-sm text-muted-foreground mt-1">快速开始使用预建的 Worker 模板</p>
    </div>

    <!-- Search & Filter -->
    <div class="metric-card p-4 mb-6">
      <div class="flex gap-4">
        <input
          v-model="searchQuery"
          placeholder="搜索模板..."
          class="flex-1 px-3 py-2 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
        />
        <select
          v-model="categoryFilter"
          class="px-3 py-2 bg-background border border-border rounded-lg text-sm"
        >
          <option value="">全部分类</option>
          <option value="入门">入门</option>
          <option value="API">API</option>
          <option value="性能">性能</option>
          <option value="安全">安全</option>
          <option value="媒体">媒体</option>
          <option value="实验">实验</option>
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
        <div class="text-4xl mb-4">{{ template.icon }}</div>
        
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
      <div class="bg-white rounded-2xl shadow-lg w-full max-w-4xl max-h-[90vh] overflow-y-auto" @click.stop>
        <div class="p-6 border-b border-border flex justify-between items-center">
          <div class="flex items-center gap-3">
            <span class="text-4xl">{{ selectedTemplate.icon }}</span>
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
            <h3 class="font-semibold mb-2">描述</h3>
            <p class="text-sm text-muted-foreground">{{ selectedTemplate.fullDescription }}</p>
          </div>

          <!-- Features -->
          <div v-if="selectedTemplate.features">
            <h3 class="font-semibold mb-2">功能特性</h3>
            <ul class="space-y-1 text-sm text-muted-foreground">
              <li v-for="feature in selectedTemplate.features" :key="feature" class="flex items-start gap-2">
                <span class="text-success mt-0.5">✓</span>
                <span>{{ feature }}</span>
              </li>
            </ul>
          </div>

          <!-- Code Preview -->
          <div>
            <h3 class="font-semibold mb-2">代码预览</h3>
            <pre class="bg-muted p-4 rounded-lg text-xs overflow-x-auto"><code>{{ selectedTemplate.code }}</code></pre>
          </div>
        </div>

        <div class="p-6 border-t border-border flex justify-end gap-3">
          <button class="btn-island-secondary" @click="copyCode(selectedTemplate.code)">
            复制代码
          </button>
          <button class="btn-island-primary" @click="useTemplate(selectedTemplate)">
            使用模板
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { toast } from '@/utils/toast'

interface WorkerTemplate {
  id: string
  name: string
  description: string
  fullDescription: string
  category: string
  difficulty: string
  icon: string
  features?: string[]
  code: string
}

const router = useRouter()
const searchQuery = ref('')
const categoryFilter = ref('')
const selectedTemplate = ref<WorkerTemplate | null>(null)

const templates = ref<WorkerTemplate[]>([
  {
    id: '1',
    name: 'Hello World',
    description: '最简单的 Worker 示例，返回一个 Hello World 响应',
    fullDescription: '这是一个最基础的 Worker 模板，演示了如何创建一个简单的 HTTP 响应。适合初学者了解 Workers 的基本结构。',
    category: '入门',
    difficulty: '简单',
    icon: '👋',
    features: [
      '基础的事件监听器',
      'Response 对象使用',
      '简单的HTTP响应',
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
    name: 'API Gateway',
    description: 'RESTful API 网关，支持路由分发和请求转发',
    fullDescription: 'API 网关模板提供了路由匹配、请求转发和错误处理功能，可以作为微服务架构的入口。',
    category: 'API',
    difficulty: '中等',
    icon: '🚪',
    features: [
      '路由匹配和分发',
      'JSON 响应处理',
      'CORS 支持',
      '错误处理',
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
    name: 'CDN 加速',
    description: '内容分发和缓存优化，提升网站访问速度',
    fullDescription: '通过边缘缓存和智能路由，显著提升静态资源的访问速度。支持自定义缓存策略和缓存清除。',
    category: '性能',
    difficulty: '中等',
    icon: '⚡',
    features: [
      '边缘缓存',
      '自定义 TTL',
      '缓存头优化',
      '源站保护',
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
    name: 'JWT 身份验证',
    description: 'JWT Token 验证和用户身份认证',
    fullDescription: '实现基于 JWT 的身份验证系统，保护 API 端点，验证用户令牌。',
    category: '安全',
    difficulty: '高级',
    icon: '🔐',
    features: [
      'JWT 解析和验证',
      '权限检查',
      '令牌刷新',
      '安全头设置',
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
    name: '图片优化',
    description: '自动图片格式转换和尺寸优化',
    fullDescription: '利用 Cloudflare Image Resizing 实现图片自动优化，支持 WebP、AVIF 格式转换和智能裁剪。',
    category: '媒体',
    difficulty: '中等',
    icon: '🖼️',
    features: [
      '自动格式转换',
      '尺寸调整',
      '质量优化',
      '懒加载支持',
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
    name: 'A/B 测试',
    description: '流量分割和 A/B 测试实验',
    fullDescription: '实现流量分割，支持多变量测试，帮助优化转化率和用户体验。',
    category: '实验',
    difficulty: '中等',
    icon: '🧪',
    features: [
      '流量随机分配',
      '多变量支持',
      'Cookie 持久化',
      '统计数据收集',
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
    const matchesCategory = !categoryFilter.value || template.category === categoryFilter.value
    return matchesSearch && matchesCategory
  })
})

function viewTemplate(template: WorkerTemplate) {
  selectedTemplate.value = template
}

function copyCode(code: string) {
  navigator.clipboard.writeText(code)
  toast.success('代码已复制到剪贴板')
}

function useTemplate(template: WorkerTemplate) {
  // Store template code in localStorage for use in Workers page
  localStorage.setItem('workerTemplate', JSON.stringify({
    name: template.name.toLowerCase().replace(/\s+/g, '-'),
    script: template.code,
  }))
  
  toast.success('模板已保存，正在跳转到 Workers 页面...')
  setTimeout(() => {
    router.push('/workers')
  }, 1000)
}
</script>
