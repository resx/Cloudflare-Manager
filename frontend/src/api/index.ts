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

    if (credentials && config.data) {
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
  zone_id: string
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
  }
}

export default api
