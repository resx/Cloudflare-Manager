import axios from 'axios'
import type { AxiosInstance, AxiosRequestConfig } from 'axios'
import { useAccountStore } from '@/stores/account'

const api: AxiosInstance = axios.create({
  baseURL: '/api',
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json'
  }
})

// 请求拦截器 - 自动添加凭证
api.interceptors.request.use(
  (config) => {
    const accountStore = useAccountStore()
    const credentials = accountStore.getCurrentCredentials()

    // 确保所有请求都有 credentials
    if (credentials) {
      // 如果没有 data，初始化为空对象
      if (!config.data) {
        config.data = {}
      }

      // 添加 credentials
      config.data = {
        credentials,
        ...config.data
      }
    }

    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// 响应拦截器
api.interceptors.response.use(
  (response) => {
    return response.data
  },
  (error) => {
    console.error('API Error:', error)

    // 提取错误信息
    let errorMessage = '请求失败'
    let isFeatureLimited = false

    if (error.response) {
      const { status, data } = error.response

      // 处理后端返回的错误
      if (data && typeof data === 'object') {
        if (data.error) {
          errorMessage = data.error

          // 检测是否是 API Token 权限问题
          const authErrorKeywords = [
            'Authentication error',
            'code": Number(10000)',
            'authentication failed',
            'invalid token',
            'unauthorized'
          ]

          const isAuthError = authErrorKeywords.some(keyword =>
            errorMessage.toLowerCase().includes(keyword.toLowerCase())
          )

          if (isAuthError) {
            errorMessage = 'API Token 权限不足。请确保您的 API Token 包含所需的权限。\n\n常见权限问题：\n• Workers KV: 需要 "Workers KV Storage - Edit"\n• D1 数据库: 需要 "D1 - Edit"\n• Workers 管理: 需要 "Workers Scripts - Edit"\n\n请访问 Cloudflare Dashboard > Profile > API Tokens 更新权限。'
            isFeatureLimited = true
          }

          // 检测是否是套餐限制或权限问题
          const limitKeywords = [
            'entitlement',
            'subscription',
            'plan',
            'upgrade',
            'feature not available',
            'not entitled',
            'requires a paid plan',
            'only available to',
            'custom certificate',
            'does not support account owned tokens',
            'user token required',
            'token type not supported'
          ]

          isFeatureLimited = limitKeywords.some(keyword =>
            errorMessage.toLowerCase().includes(keyword)
          ) || isFeatureLimited
        }
      }

      // HTTP 状态码
      if (status === 403) {
        errorMessage = '权限不足或功能未授权'
        isFeatureLimited = true
      } else if (status === 402) {
        errorMessage = '此功能需要付费套餐'
        isFeatureLimited = true
      } else if (status === 400 && !data?.error) {
        errorMessage = '请求参数错误'
      }
    } else if (error.request) {
      errorMessage = '网络请求失败，请检查网络连接'
    }

    // 构造增强的错误对象
    const enhancedError = new Error(errorMessage)
    Object.assign(enhancedError, {
      originalError: error,
      isFeatureLimited,
      status: error.response?.status
    })

    return Promise.reject(enhancedError)
  }
)

// API 接口定义
export interface Zone {
  id: string
  name: string
  status: string
  name_servers: string[]
}

export interface CloudflareAccount {
  id: string
  name: string
  settings?: any
}

export interface DnsRecord {
  id?: string
  zone_id?: string  // 可选，因为 Cloudflare API 返回的记录可能不包含此字段
  type: string
  name: string
  content: string
  ttl: number
  proxied: boolean
  priority?: number
}

export interface FirewallRule {
  id?: string
  filter: {
    id?: string
    expression: string
    description?: string
  }
  action: string
  description?: string
  paused: boolean
}

export interface DeployWorkerRequest {
  zone_id: string
  script_name: string
  target_url: string
  access_domain: string
  cache_ttl: number
  cdn_node: string
}

export interface Worker {
  id: string
  etag?: string
  created_on?: string
  modified_on?: string
}

export interface WorkerRoute {
  id: string
  pattern: string
  script?: string
}

export interface ZoneSetting {
  id: string
  value: any
  modified_on?: string
}

export interface UpdateSetting {
  id: string
  value: any
}

export interface AnalyticsStats {
  totalRequests: number
  cacheHitRate: number
  bandwidth: number
  threats: number
}

export interface TimeseriesPoint {
  timestamp: string
  requests: number
  cached: number
  uncached: number
}

export interface StatusCodeStat {
  code: string
  description: string
  count: number
  percentage: number
}

export interface CountryStat {
  rank: number
  country: string
  requests: number
  percentage: number
}

export interface ContentStat {
  rank: number
  url: string
  requests: number
  bandwidth: string
}

export interface AnalyticsData {
  stats: AnalyticsStats
  timeseries: TimeseriesPoint[]
  statusCodes: StatusCodeStat[]
  countries: CountryStat[]
  content: ContentStat[]
}

// 缓存清除相关
export interface PurgeCacheRequest {
  zone_id: string
  purge_everything?: boolean
  files?: string[]
  tags?: string[]
}

export interface PurgeCacheResponse {
  id: string
}

// SSL 证书相关
export interface SslCertificate {
  id: string
  type: string
  status: string
  primary_certificate?: string
  certificates?: CertificateDetail[]
  hosts?: string[]
}

export interface CertificateDetail {
  id: string
  status: string
  issuer: string
  signature: string
  serial_number: string
  expires_on: string
  uploaded_on: string
}

// 自定义 SSL 证书相关
export interface CustomCertificate {
  id: string
  status: string
  issuer: string
  signature: string
  expires_on: string
  uploaded_on: string
  modified_on: string
  hosts: string[]
  bundle_method?: string
}

export interface UploadCustomCertificateRequest {
  zone_id: string
  certificate: string  // PEM format
  private_key: string  // PEM format
  bundle_method?: string  // ubiquitous, optimal, force
}

// 页面规则相关
export interface PageRule {
  id?: string
  targets: PageRuleTarget[]
  actions: PageRuleAction[]
  priority?: number
  status?: string
}

export interface PageRuleTarget {
  target: string  // "url"
  constraint: PageRuleConstraint
}

export interface PageRuleConstraint {
  operator: string  // "matches"
  value: string  // URL pattern
}

export interface PageRuleAction {
  id: string
  value: any
}

// WAF 规则相关
export interface WafPackage {
  id: string
  name: string
  description: string
  detection_mode: string
  sensitivity?: string
  action_mode?: string
}

export interface WafRule {
  id: string
  description: string
  priority: string
  group: WafRuleGroup
  mode: string
  allowed_modes: string[]
}

export interface WafRuleGroup {
  id: string
  name: string
}

// Rate Limiting 相关
export interface RateLimit {
  id: string
  disabled: boolean
  description: string
  match_request: MatchRequest
  threshold: number
  period: number
  action: RateLimitAction
}

export interface MatchRequest {
  url: string
  methods?: string[]
  schemes?: string[]
}

export interface RateLimitAction {
  mode: string  // simulate, ban, challenge, js_challenge
  timeout?: number
  response?: RateLimitResponse
}

export interface RateLimitResponse {
  content_type: string
  body: string
}

export interface CreateRateLimitRequest {
  zone_id: string
  disabled: boolean
  description: string
  match_request: MatchRequest
  threshold: number
  period: number
  action: RateLimitAction
}

// Workers KV 相关
export interface KVNamespace {
  id: string
  title: string
  supports_url_encoding?: boolean
}

export interface KVKey {
  name: string
  expiration?: number
  metadata?: any
}

export interface KVKeyValue {
  key: string
  value: string
  metadata?: any
}

export interface CreateKVNamespaceRequest {
  account_id: string
  title: string
}

export interface WriteKVRequest {
  account_id: string
  namespace_id: string
  key: string
  value: string
  expiration_ttl?: number
  metadata?: any
}

export interface DeleteKVKeyRequest {
  account_id: string
  namespace_id: string
  key: string
}

// D1 Database 相关
export interface D1Database {
  uuid: string
  name: string
  version: string
  created_at: string
}

export interface D1QueryResult {
  results: any[]
  success: boolean
  meta: {
    changed_db: boolean
    changes: number
    duration: number
    last_row_id: number
    rows_read: number
    rows_written: number
    size_after: number
  }
}

export interface CreateD1DatabaseRequest {
  account_id: string
  name: string
}

export interface DeleteD1DatabaseRequest {
  account_id: string
  database_id: string
}

export interface ExecuteD1QueryRequest {
  account_id: string
  database_id: string
  query: string
}

export const cloudflareApi = {
  // Cloudflare 账户
  async getAccounts(): Promise<CloudflareAccount[]> {
    const res = await api.post('/cloudflare/accounts', {})
    return res.data || []
  },

  // Zone 相关
  async getZones(): Promise<Zone[]> {
    const res = await api.post('/cloudflare/zones', {})
    return res.data || []
  },

  // DNS 记录
  async getDnsRecords(zoneId: string): Promise<DnsRecord[]> {
    const res = await api.post('/cloudflare/dns/records', { zone_id: zoneId })
    return res.data || []
  },

  async createDnsRecord(record: DnsRecord): Promise<DnsRecord> {
    const res = await api.post('/cloudflare/dns/records/create', record)
    return res.data
  },

  async updateDnsRecord(record: DnsRecord): Promise<DnsRecord> {
    const res = await api.post('/cloudflare/dns/records/update', record)
    return res.data
  },

  async deleteDnsRecord(zoneId: string, recordId: string): Promise<string> {
    const res = await api.post('/cloudflare/dns/records/delete', {
      zone_id: zoneId,
      record_id: recordId
    })
    return res.data
  },

  // 防火墙规则
  async getFirewallRules(zoneId: string): Promise<FirewallRule[]> {
    const res = await api.post('/cloudflare/firewall/rules', { zone_id: zoneId })
    return res.data || []
  },

  async createFirewallRule(zoneId: string, rule: FirewallRule): Promise<FirewallRule> {
    const res = await api.post('/cloudflare/firewall/rules/create', {
      zone_id: zoneId,
      rule
    })
    return res.data
  },

  async updateFirewallRule(zoneId: string, ruleId: string, rule: FirewallRule): Promise<FirewallRule> {
    const res = await api.post('/cloudflare/firewall/rules/update', {
      zone_id: zoneId,
      rule_id: ruleId,
      rule
    })
    return res.data
  },

  async deleteFirewallRule(zoneId: string, ruleId: string): Promise<string> {
    const res = await api.post('/cloudflare/firewall/rules/delete', {
      zone_id: zoneId,
      rule_id: ruleId
    })
    return res.data
  },

  // Worker 部署
  async deployWorker(request: DeployWorkerRequest): Promise<string> {
    const res = await api.post('/cloudflare/workers/deploy', request)
    return res.data
  },

  // 获取 Worker 列表
  async listWorkers(accountId: string): Promise<Worker[]> {
    const res = await api.post('/cloudflare/workers/list', { account_id: accountId })
    return res.data || []
  },

  // 获取单个 Worker
  async getWorker(accountId: string, scriptName: string): Promise<string> {
    const res = await api.post('/cloudflare/workers/get', {
      account_id: accountId,
      script_name: scriptName
    })
    return res.data
  },

  // 删除 Worker
  async deleteWorker(accountId: string, scriptName: string): Promise<string> {
    const res = await api.post('/cloudflare/workers/delete', {
      account_id: accountId,
      script_name: scriptName
    })
    return res.data
  },

  // 上传/更新 Worker
  async uploadWorker(accountId: string, scriptName: string, scriptContent: string): Promise<any> {
    const res = await api.post('/cloudflare/workers/upload', {
      account_id: accountId,
      script_name: scriptName,
      script_content: scriptContent
    })
    return res.data
  },

  // 获取 Worker 路由列表
  async getWorkerRoutes(zoneId: string): Promise<WorkerRoute[]> {
    const res = await api.post('/cloudflare/workers/routes', { zone_id: zoneId })
    return res.data || []
  },

  // 创建 Worker 路由
  async createWorkerRoute(zoneId: string, pattern: string, scriptName: string): Promise<WorkerRoute> {
    const res = await api.post('/cloudflare/workers/routes/create', {
      zone_id: zoneId,
      pattern: pattern,
      script_name: scriptName
    })
    return res.data
  },

  // 删除 Worker 路由
  async deleteWorkerRoute(zoneId: string, routeId: string): Promise<string> {
    const res = await api.post('/cloudflare/workers/routes/delete', {
      zone_id: zoneId,
      route_id: routeId
    })
    return res.data
  },

  // Zone 设置
  async getZoneSettings(zoneId: string): Promise<ZoneSetting[]> {
    const res = await api.post('/cloudflare/zone/settings', { zone_id: zoneId })
    return res.data || []
  },

  async updateZoneSettings(zoneId: string, settings: UpdateSetting[]): Promise<string> {
    const res = await api.post('/cloudflare/zone/settings/update', {
      zone_id: zoneId,
      settings
    })
    return res.data
  },

  // 自动优化
  async optimizeZone(zoneId: string, mode: 'security' | 'performance'): Promise<string> {
    const res = await api.post('/cloudflare/zone/optimize', {
      zone_id: zoneId,
      mode
    })
    return res.data
  },

  // Analytics 统计分析
  async getAnalytics(zoneId: string, timeRange: string): Promise<AnalyticsData> {
    const res = await api.post('/cloudflare/analytics', {
      zone_id: zoneId,
      time_range: timeRange
    })
    return res.data
  },

  // 缓存清除
  async purgeCache(request: PurgeCacheRequest): Promise<PurgeCacheResponse> {
    const res = await api.post('/cloudflare/cache/purge', request)
    return res.data
  },

  // SSL 证书
  async getSslCertificates(zoneId: string): Promise<SslCertificate[]> {
    const res = await api.post('/cloudflare/ssl/certificates', { zone_id: zoneId })
    return res.data || []
  },

  // 自定义 SSL 证书
  async getCustomCertificates(zoneId: string): Promise<CustomCertificate[]> {
    const res = await api.post('/cloudflare/ssl/custom', { zone_id: zoneId })
    return res.data || []
  },

  async uploadCustomCertificate(request: UploadCustomCertificateRequest): Promise<CustomCertificate> {
    const res = await api.post('/cloudflare/ssl/custom/upload', request)
    return res.data
  },

  async deleteCustomCertificate(zoneId: string, certificateId: string): Promise<string> {
    const res = await api.post('/cloudflare/ssl/custom/delete', {
      zone_id: zoneId,
      certificate_id: certificateId
    })
    return res.data
  },

  // 页面规则
  async getPageRules(zoneId: string): Promise<PageRule[]> {
    const res = await api.post('/cloudflare/pagerules', { zone_id: zoneId })
    return res.data || []
  },

  async createPageRule(zoneId: string, rule: PageRule): Promise<PageRule> {
    const res = await api.post('/cloudflare/pagerules/create', {
      zone_id: zoneId,
      rule
    })
    return res.data
  },

  async updatePageRule(zoneId: string, ruleId: string, rule: PageRule): Promise<PageRule> {
    const res = await api.post('/cloudflare/pagerules/update', {
      zone_id: zoneId,
      rule_id: ruleId,
      rule
    })
    return res.data
  },

  async deletePageRule(zoneId: string, ruleId: string): Promise<string> {
    const res = await api.post('/cloudflare/pagerules/delete', {
      zone_id: zoneId,
      rule_id: ruleId
    })
    return res.data
  },

  // WAF 规则管理
  async getWafPackages(zoneId: string): Promise<WafPackage[]> {
    const res = await api.post('/cloudflare/waf/packages', { zone_id: zoneId })
    return res.data || []
  },

  async getWafRules(zoneId: string, packageId: string): Promise<WafRule[]> {
    const res = await api.post('/cloudflare/waf/rules', {
      zone_id: zoneId,
      package_id: packageId
    })
    return res.data || []
  },

  async updateWafRule(zoneId: string, packageId: string, ruleId: string, mode: string): Promise<WafRule> {
    const res = await api.post('/cloudflare/waf/rules/update', {
      zone_id: zoneId,
      package_id: packageId,
      rule_id: ruleId,
      mode: mode
    })
    return res.data
  },

  async updateWafPackage(zoneId: string, packageId: string, sensitivity?: string, actionMode?: string): Promise<WafPackage> {
    const res = await api.post('/cloudflare/waf/packages/update', {
      zone_id: zoneId,
      package_id: packageId,
      sensitivity: sensitivity,
      action_mode: actionMode
    })
    return res.data
  },

  // Rate Limiting
  async getRateLimits(zoneId: string): Promise<RateLimit[]> {
    const res = await api.post('/cloudflare/ratelimits', { zone_id: zoneId })
    return res.data || []
  },

  async createRateLimit(request: CreateRateLimitRequest): Promise<RateLimit> {
    const res = await api.post('/cloudflare/ratelimits/create', request)
    return res.data
  },

  async updateRateLimit(zoneId: string, rateLimitId: string, request: CreateRateLimitRequest): Promise<RateLimit> {
    const res = await api.post('/cloudflare/ratelimits/update', {
      zone_id: zoneId,
      rate_limit_id: rateLimitId,
      ...request
    })
    return res.data
  },

  async deleteRateLimit(zoneId: string, rateLimitId: string): Promise<string> {
    const res = await api.post('/cloudflare/ratelimits/delete', {
      zone_id: zoneId,
      rate_limit_id: rateLimitId
    })
    return res.data
  },

  // Workers KV
  async listKVNamespaces(accountId: string): Promise<KVNamespace[]> {
    const res = await api.post('/cloudflare/kv/namespaces', { account_id: accountId })
    return res.data || []
  },

  async createKVNamespace(request: CreateKVNamespaceRequest): Promise<KVNamespace> {
    const res = await api.post('/cloudflare/kv/namespaces/create', request)
    return res.data
  },

  async deleteKVNamespace(accountId: string, namespaceId: string): Promise<string> {
    const res = await api.post('/cloudflare/kv/namespaces/delete', {
      account_id: accountId,
      namespace_id: namespaceId
    })
    return res.data
  },

  async listKVKeys(accountId: string, namespaceId: string, prefix?: string): Promise<KVKey[]> {
    const res = await api.post('/cloudflare/kv/keys', {
      account_id: accountId,
      namespace_id: namespaceId,
      prefix: prefix
    })
    return res.data || []
  },

  async readKVValue(accountId: string, namespaceId: string, key: string): Promise<string> {
    const res = await api.post('/cloudflare/kv/read', {
      account_id: accountId,
      namespace_id: namespaceId,
      key: key
    })
    return res.data
  },

  async writeKVValue(request: WriteKVRequest): Promise<string> {
    const res = await api.post('/cloudflare/kv/write', request)
    return res.data
  },

  async deleteKVKey(request: DeleteKVKeyRequest): Promise<string> {
    const res = await api.post('/cloudflare/kv/delete', request)
    return res.data
  },

  // D1 Database
  async listD1Databases(accountId: string): Promise<D1Database[]> {
    const res = await api.post('/cloudflare/d1/databases', { account_id: accountId })
    return res.data || []
  },

  async createD1Database(request: CreateD1DatabaseRequest): Promise<D1Database> {
    const res = await api.post('/cloudflare/d1/databases/create', request)
    return res.data
  },

  async deleteD1Database(accountId: string, databaseId: string): Promise<string> {
    const res = await api.post('/cloudflare/d1/databases/delete', {
      account_id: accountId,
      database_id: databaseId
    })
    return res.data
  },

  async executeD1Query(request: ExecuteD1QueryRequest): Promise<D1QueryResult> {
    const res = await api.post('/cloudflare/d1/query', request)
    return res.data
  }
}

export default api
