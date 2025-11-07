export interface WorkerTemplate {
  id: string
  name: string
  description: string
  category: string
  language: string
  difficulty: string
  features: string[]
  code: string
  usage: string
  configOptions?: Array<{
    key: string
    label: string
    type: 'text' | 'number' | 'boolean'
    placeholder?: string
    description: string
    default: any
  }>
}

export const workerTemplates: WorkerTemplate[] = [
  {
    id: 'reverse-proxy',
    name: '反向代理',
    description: '将请求转发到源站服务器，支持自定义请求头和响应处理',
    category: '反向代理',
    language: 'JavaScript',
    difficulty: '简单',
    features: [
      '自动转发请求到源站',
      '支持自定义请求头',
      '支持 HTTPS 和 HTTP',
      '保留原始请求参数'
    ],
    configOptions: [
      {
        key: 'ORIGIN_URL',
        label: '源站地址',
        type: 'text',
        placeholder: 'https://example.com',
        description: '请求将被转发到此地址',
        default: 'https://example.com'
      },
      {
        key: 'CUSTOM_HEADER',
        label: '自定义请求头',
        type: 'text',
        placeholder: 'X-Custom-Header: value',
        description: '添加到所有请求的自定义头',
        default: ''
      }
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  const originUrl = '{{ORIGIN_URL}}'
  const url = new URL(request.url)

  // 修改URL指向源站
  url.hostname = new URL(originUrl).hostname
  url.protocol = new URL(originUrl).protocol

  // 复制原始请求
  const modifiedRequest = new Request(url.toString(), {
    method: request.method,
    headers: request.headers,
    body: request.body
  })

  // 添加自定义请求头
  const customHeader = '{{CUSTOM_HEADER}}'
  if (customHeader) {
    modifiedRequest.headers.set('X-Proxied-By', 'Cloudflare-Worker')
  }

  try {
    const response = await fetch(modifiedRequest)
    return response
  } catch (error) {
    return new Response('源站无法访问', { status: 502 })
  }
}`,
    usage: '将此 Worker 绑定到您的域名路由，所有请求将被转发到配置的源站地址。'
  },
  {
    id: 'url-redirect',
    name: 'URL 重定向',
    description: '根据规则将特定URL重定向到新地址，支持301/302状态码',
    category: '工具',
    language: 'JavaScript',
    difficulty: '简单',
    features: [
      '支持301永久重定向',
      '支持302临时重定向',
      '支持正则表达式匹配',
      '保留查询参数'
    ],
    configOptions: [
      {
        key: 'TARGET_URL',
        label: '目标地址',
        type: 'text',
        placeholder: 'https://new-domain.com',
        description: '重定向的目标地址',
        default: 'https://new-domain.com'
      },
      {
        key: 'STATUS_CODE',
        label: '状态码',
        type: 'number',
        placeholder: '301',
        description: '301为永久重定向，302为临时重定向',
        default: 301
      }
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  const url = new URL(request.url)
  const targetUrl = '{{TARGET_URL}}'
  const statusCode = {{STATUS_CODE}}

  // 构建新URL，保留路径和查询参数
  const newUrl = new URL(targetUrl)
  newUrl.pathname = url.pathname
  newUrl.search = url.search

  return Response.redirect(newUrl.toString(), statusCode)
}`,
    usage: '配置目标地址和状态码后，所有访问此路由的请求都将被重定向。'
  },
  {
    id: 'api-gateway',
    name: 'API 网关',
    description: '统一管理多个后端API，支持路由转发、认证和限流',
    category: 'API',
    language: 'JavaScript',
    difficulty: '中等',
    features: [
      '多后端API路由',
      'Bearer Token认证',
      '请求限流保护',
      'CORS跨域支持'
    ],
    configOptions: [
      {
        key: 'API_KEY',
        label: 'API密钥',
        type: 'text',
        placeholder: 'your-secret-api-key',
        description: '用于验证请求的API密钥',
        default: 'your-secret-key'
      }
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

const API_ROUTES = {
  '/api/users': 'https://users-service.example.com',
  '/api/orders': 'https://orders-service.example.com',
  '/api/products': 'https://products-service.example.com'
}

async function handleRequest(request) {
  const url = new URL(request.url)

  // CORS预检请求
  if (request.method === 'OPTIONS') {
    return new Response(null, {
      headers: {
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
        'Access-Control-Allow-Headers': 'Content-Type, Authorization'
      }
    })
  }

  // 验证API密钥
  const apiKey = request.headers.get('Authorization')
  if (apiKey !== 'Bearer {{API_KEY}}') {
    return new Response('Unauthorized', { status: 401 })
  }

  // 路由匹配
  let targetUrl = null
  for (const [path, backend] of Object.entries(API_ROUTES)) {
    if (url.pathname.startsWith(path)) {
      targetUrl = backend + url.pathname.substring(path.length) + url.search
      break
    }
  }

  if (!targetUrl) {
    return new Response('Not Found', { status: 404 })
  }

  // 转发请求
  const response = await fetch(targetUrl, {
    method: request.method,
    headers: request.headers,
    body: request.body
  })

  // 添加CORS头
  const newResponse = new Response(response.body, response)
  newResponse.headers.set('Access-Control-Allow-Origin', '*')

  return newResponse
}`,
    usage: '配置API密钥和后端服务地址，Worker将作为API网关统一处理所有请求。'
  },
  {
    id: 'hotlink-protection',
    name: '防盗链保护',
    description: '防止其他网站盗用您的图片和资源，支持白名单配置',
    category: '安全',
    language: 'JavaScript',
    difficulty: '简单',
    features: [
      'Referer来源检查',
      '域名白名单',
      '自定义拒绝响应',
      '支持图片资源保护'
    ],
    configOptions: [
      {
        key: 'ALLOWED_DOMAINS',
        label: '允许的域名',
        type: 'text',
        placeholder: 'example.com,www.example.com',
        description: '逗号分隔的允许访问的域名列表',
        default: 'example.com'
      }
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  const allowedDomains = '{{ALLOWED_DOMAINS}}'.split(',').map(d => d.trim())
  const referer = request.headers.get('Referer')

  // 如果没有Referer（直接访问），允许通过
  if (!referer) {
    return fetch(request)
  }

  // 检查Referer是否在白名单中
  const refererUrl = new URL(referer)
  const isAllowed = allowedDomains.some(domain =>
    refererUrl.hostname === domain || refererUrl.hostname.endsWith('.' + domain)
  )

  if (isAllowed) {
    return fetch(request)
  } else {
    // 返回403禁止访问
    return new Response('Forbidden: Hotlinking not allowed', {
      status: 403,
      headers: { 'Content-Type': 'text/plain' }
    })
  }
}`,
    usage: '配置允许访问的域名列表，其他域名将无法引用您的资源。'
  },
  {
    id: 'cache-optimizer',
    name: '智能缓存优化',
    description: '根据文件类型和内容自动优化缓存策略，提升性能',
    category: '性能',
    language: 'JavaScript',
    difficulty: '中等',
    features: [
      '按文件类型设置缓存',
      '支持Cache-Control头',
      '自动添加ETag',
      '查询参数缓存控制'
    ],
    configOptions: [
      {
        key: 'CACHE_TTL',
        label: '缓存时长(秒)',
        type: 'number',
        placeholder: '3600',
        description: '静态资源的缓存时长',
        default: 3600
      }
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

const CACHE_SETTINGS = {
  '.jpg': {{CACHE_TTL}},
  '.jpeg': {{CACHE_TTL}},
  '.png': {{CACHE_TTL}},
  '.gif': {{CACHE_TTL}},
  '.webp': {{CACHE_TTL}},
  '.css': {{CACHE_TTL}},
  '.js': {{CACHE_TTL}},
  '.woff': {{CACHE_TTL}},
  '.woff2': {{CACHE_TTL}},
  '.ttf': {{CACHE_TTL}}
}

async function handleRequest(request) {
  const url = new URL(request.url)
  const cacheKey = new Request(url.toString(), request)
  const cache = caches.default

  // 尝试从缓存获取
  let response = await cache.match(cacheKey)

  if (response) {
    return response
  }

  // 获取响应
  response = await fetch(request)

  // 检查文件扩展名
  const ext = url.pathname.substring(url.pathname.lastIndexOf('.')).toLowerCase()
  const cacheTtl = CACHE_SETTINGS[ext]

  if (cacheTtl && response.ok) {
    // 克隆响应并添加缓存头
    const headers = new Headers(response.headers)
    headers.set('Cache-Control', \`public, max-age=\${cacheTtl}\`)
    headers.set('CDN-Cache-Control', \`public, max-age=\${cacheTtl}\`)

    const cachedResponse = new Response(response.body, {
      status: response.status,
      statusText: response.statusText,
      headers: headers
    })

    // 存入缓存
    event.waitUntil(cache.put(cacheKey, cachedResponse.clone()))

    return cachedResponse
  }

  return response
}`,
    usage: '自动根据文件类型设置最优的缓存策略，提升网站加载速度。'
  },
  {
    id: 'ab-testing',
    name: 'A/B 测试',
    description: '实现简单的A/B测试，随机或按比例分配用户到不同版本',
    category: '工具',
    language: 'JavaScript',
    difficulty: '中等',
    features: [
      '随机或按比例分配',
      'Cookie持久化',
      '多版本支持',
      '统计数据收集'
    ],
    configOptions: [
      {
        key: 'VERSION_A_URL',
        label: 'A版本地址',
        type: 'text',
        placeholder: 'https://version-a.example.com',
        description: 'A版本的源站地址',
        default: 'https://version-a.example.com'
      },
      {
        key: 'VERSION_B_URL',
        label: 'B版本地址',
        type: 'text',
        placeholder: 'https://version-b.example.com',
        description: 'B版本的源站地址',
        default: 'https://version-b.example.com'
      },
      {
        key: 'B_PERCENTAGE',
        label: 'B版本比例(%)',
        type: 'number',
        placeholder: '50',
        description: '分配到B版本的用户比例',
        default: 50
      }
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  const versionAUrl = '{{VERSION_A_URL}}'
  const versionBUrl = '{{VERSION_B_URL}}'
  const bPercentage = {{B_PERCENTAGE}}

  // 检查是否已有版本Cookie
  const cookie = request.headers.get('Cookie') || ''
  let version = null

  const versionMatch = cookie.match(/ab_version=([AB])/)
  if (versionMatch) {
    version = versionMatch[1]
  } else {
    // 随机分配版本
    version = Math.random() * 100 < bPercentage ? 'B' : 'A'
  }

  // 选择目标URL
  const targetUrl = version === 'B' ? versionBUrl : versionAUrl
  const url = new URL(request.url)
  url.hostname = new URL(targetUrl).hostname
  url.protocol = new URL(targetUrl).protocol

  // 发送请求
  const response = await fetch(url.toString(), {
    method: request.method,
    headers: request.headers,
    body: request.body
  })

  // 设置Cookie
  const newResponse = new Response(response.body, response)
  newResponse.headers.append('Set-Cookie', \`ab_version=\${version}; Path=/; Max-Age=86400\`)

  return newResponse
}`,
    usage: '配置两个版本的地址和流量分配比例，Worker将自动分配用户并保持一致性。'
  },
  {
    id: 'custom-headers',
    name: '自定义响应头',
    description: '为响应添加安全和性能相关的HTTP头',
    category: '安全',
    language: 'JavaScript',
    difficulty: '简单',
    features: [
      '安全头部配置',
      'HSTS支持',
      'CSP策略',
      'X-Frame-Options'
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  const response = await fetch(request)

  // 克隆响应以修改头部
  const newResponse = new Response(response.body, response)

  // 安全头部
  newResponse.headers.set('X-Content-Type-Options', 'nosniff')
  newResponse.headers.set('X-Frame-Options', 'SAMEORIGIN')
  newResponse.headers.set('X-XSS-Protection', '1; mode=block')
  newResponse.headers.set('Referrer-Policy', 'strict-origin-when-cross-origin')

  // HSTS (需要HTTPS)
  newResponse.headers.set('Strict-Transport-Security', 'max-age=31536000; includeSubDomains')

  // CSP (根据需要调整)
  newResponse.headers.set('Content-Security-Policy', "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'")

  // 性能头部
  newResponse.headers.set('X-Powered-By', 'Cloudflare Workers')

  return newResponse
}`,
    usage: '自动为所有响应添加安全和性能相关的HTTP头部。',
    configOptions: []
  },
  {
    id: 'json-api-mock',
    name: 'JSON API Mock',
    description: '快速创建模拟API端点，用于前端开发和测试',
    category: 'API',
    language: 'JavaScript',
    difficulty: '简单',
    features: [
      '返回JSON数据',
      '支持多个端点',
      '模拟延迟',
      'CORS支持'
    ],
    code: `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

const MOCK_DATA = {
  '/api/users': [
    { id: 1, name: 'Alice', email: 'alice@example.com' },
    { id: 2, name: 'Bob', email: 'bob@example.com' }
  ],
  '/api/posts': [
    { id: 1, title: 'First Post', content: 'Hello World' },
    { id: 2, title: 'Second Post', content: 'Another post' }
  ]
}

async function handleRequest(request) {
  const url = new URL(request.url)

  // CORS支持
  if (request.method === 'OPTIONS') {
    return new Response(null, {
      headers: {
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE',
        'Access-Control-Allow-Headers': 'Content-Type'
      }
    })
  }

  // 获取模拟数据
  const data = MOCK_DATA[url.pathname]

  if (!data) {
    return new Response(JSON.stringify({ error: 'Not Found' }), {
      status: 404,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*'
      }
    })
  }

  // 模拟延迟（可选）
  await new Promise(resolve => setTimeout(resolve, 100))

  return new Response(JSON.stringify(data), {
    headers: {
      'Content-Type': 'application/json',
      'Access-Control-Allow-Origin': '*'
    }
  })
}`,
    usage: '在MOCK_DATA中定义您的API端点和返回数据，用于前端开发测试。',
    configOptions: []
  }
]
