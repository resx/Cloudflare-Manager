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
    return Promise.reject(error)
  }
)

// API 接口定义
export interface Zone {
  id: string
  name: string
  status: string
  name_servers: string[]
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

export const cloudflareApi = {
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
  }
}

export default api
