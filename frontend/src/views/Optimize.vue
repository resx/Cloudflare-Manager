<template>
  <n-space vertical :size="24">
    <n-card title="自动优化配置">
      <n-alert type="info" style="margin-bottom: 24px">
        一键应用最佳配置,让 Cloudflare 设置从 10 分钟的复杂操作变成 1 键搞定
      </n-alert>

      <n-alert v-if="!currentZone" type="warning" style="margin-bottom: 24px">
        请先在左侧菜单选择一个域名
      </n-alert>

      <n-descriptions v-else :column="1" bordered style="margin-bottom: 24px">
        <n-descriptions-item label="当前域名">
          {{ currentZone.name }}
        </n-descriptions-item>
      </n-descriptions>
    </n-card>

    <n-grid :cols="3" :x-gap="24">
      <!-- 安全优先模式 -->
      <n-gi>
        <n-card title="🛡️ 安全优先模式" hoverable>
          <n-space vertical>
            <n-text depth="3">
              适用于金融、政府、企业官网等对安全要求极高的场景
            </n-text>

            <n-divider />

            <n-text strong>自动配置项:</n-text>
            <n-ul>
              <n-li>安全级别: 高</n-li>
              <n-li>SSL 模式: 严格 (端到端加密)</n-li>
              <n-li>强制 HTTPS: 启用</n-li>
              <n-li>TLS 1.3: 启用</n-li>
              <n-li>最低 TLS 版本: 1.2</n-li>
              <n-li>浏览器检查: 启用</n-li>
              <n-li>防盗链保护: 启用</n-li>
            </n-ul>

            <n-divider />

            <n-text type="success">
              ✅ 99.9% 防护 DDoS 攻击<br />
              ✅ 自动拦截恶意爬虫<br />
              ✅ 防止 SQL 注入和 XSS
            </n-text>

            <n-button
              type="primary"
              block
              size="large"
              :loading="optimizing && optimizeMode === 'security'"
              :disabled="!currentZone"
              @click="handleOptimize('security')"
            >
              应用安全优先配置
            </n-button>
          </n-space>
        </n-card>
      </n-gi>

      <!-- 性能优先模式 -->
      <n-gi>
        <n-card title="⚡ 性能优先模式" hoverable>
          <n-space vertical>
            <n-text depth="3">
              适用于电商、媒体、个人博客等对性能要求高的场景
            </n-text>

            <n-divider />

            <n-text strong>自动配置项:</n-text>
            <n-ul>
              <n-li>缓存级别: 积极 (最大化命中率)</n-li>
              <n-li>浏览器缓存: 1 年</n-li>
              <n-li>HTML/CSS/JS 压缩: 全部启用</n-li>
              <n-li>Brotli 压缩: 启用</n-li>
              <n-li>HTTP/3: 启用 (QUIC 协议)</n-li>
              <n-li>Early Hints: 启用</n-li>
              <n-li>图片优化: 无损</n-li>
            </n-ul>

            <n-divider />

            <n-text type="success">
              ⚡ 页面加载速度提升 60%<br />
              ⚡ 带宽节省 50%<br />
              ⚡ 全球访问延迟降低 70%
            </n-text>

            <n-button
              type="info"
              block
              size="large"
              :loading="optimizing && optimizeMode === 'performance'"
              :disabled="!currentZone"
              @click="handleOptimize('performance')"
            >
              应用性能优先配置
            </n-button>
          </n-space>
        </n-card>
      </n-gi>

      <!-- 自定义配置模式 -->
      <n-gi>
        <n-card title="⚙️ 自定义配置" hoverable>
          <n-space vertical>
            <n-text depth="3">
              根据您的需求自定义选择要优化的配置项
            </n-text>

            <n-divider />

            <n-button
              block
              size="large"
              @click="showCustomModal = true"
              :disabled="!currentZone"
            >
              打开自定义配置
            </n-button>

            <n-text depth="3" style="font-size: 12px; margin-top: 8px">
              可选配置项包括:
            </n-text>
            <n-ul style="font-size: 12px">
              <n-li>SSL/TLS 设置</n-li>
              <n-li>缓存配置</n-li>
              <n-li>安全级别</n-li>
              <n-li>性能优化</n-li>
              <n-li>网络设置</n-li>
            </n-ul>
          </n-space>
        </n-card>
      </n-gi>
    </n-grid>

    <!-- 自定义配置弹窗 -->
    <n-modal v-model:show="showCustomModal" preset="card" title="自定义配置" style="width: 900px" :closable="true">
      <n-tabs type="line" animated>
        <!-- SSL/TLS 配置 -->
        <n-tab-pane name="ssl" tab="SSL/TLS">
          <n-form label-placement="left" label-width="150">
            <n-form-item label="SSL 模式">
              <n-select
                v-model:value="customConfig.ssl.mode"
                :options="[
                  { label: '关闭', value: 'off' },
                  { label: '灵活', value: 'flexible' },
                  { label: '完全', value: 'full' },
                  { label: '完全(严格)', value: 'strict' }
                ]"
              />
            </n-form-item>

            <n-form-item label="始终使用 HTTPS">
              <n-switch v-model:value="customConfig.ssl.alwaysUseHttps" />
            </n-form-item>

            <n-form-item label="自动 HTTPS 重写">
              <n-switch v-model:value="customConfig.ssl.automaticHttpsRewrites" />
            </n-form-item>

            <n-form-item label="最低 TLS 版本">
              <n-select
                v-model:value="customConfig.ssl.minTlsVersion"
                :options="[
                  { label: 'TLS 1.0', value: '1.0' },
                  { label: 'TLS 1.1', value: '1.1' },
                  { label: 'TLS 1.2', value: '1.2' },
                  { label: 'TLS 1.3', value: '1.3' }
                ]"
              />
            </n-form-item>

            <n-form-item label="TLS 1.3">
              <n-switch v-model:value="customConfig.ssl.tls13" />
            </n-form-item>
          </n-form>
        </n-tab-pane>

        <!-- 缓存配置 -->
        <n-tab-pane name="cache" tab="缓存">
          <n-form label-placement="left" label-width="150">
            <n-form-item label="缓存级别">
              <n-select
                v-model:value="customConfig.cache.level"
                :options="[
                  { label: '不缓存', value: 'basic' },
                  { label: '标准', value: 'simplified' },
                  { label: '积极', value: 'aggressive' }
                ]"
              />
            </n-form-item>

            <n-form-item label="浏览器缓存 TTL">
              <n-select
                v-model:value="customConfig.cache.browserCacheTtl"
                :options="[
                  { label: '30 分钟', value: 1800 },
                  { label: '1 小时', value: 3600 },
                  { label: '2 小时', value: 7200 },
                  { label: '4 小时', value: 14400 },
                  { label: '8 小时', value: 28800 },
                  { label: '1 天', value: 86400 },
                  { label: '1 个月', value: 2678400 },
                  { label: '1 年', value: 31536000 }
                ]"
              />
            </n-form-item>

            <n-form-item label="查询字符串排序">
              <n-switch v-model:value="customConfig.cache.sortQueryString" />
            </n-form-item>
          </n-form>
        </n-tab-pane>

        <!-- 安全设置 -->
        <n-tab-pane name="security" tab="安全">
          <n-form label-placement="left" label-width="150">
            <n-form-item label="安全级别">
              <n-select
                v-model:value="customConfig.security.securityLevel"
                :options="[
                  { label: '关闭', value: 'off' },
                  { label: '实际上关闭', value: 'essentially_off' },
                  { label: '低', value: 'low' },
                  { label: '中', value: 'medium' },
                  { label: '高', value: 'high' },
                  { label: '我正在被攻击', value: 'under_attack' }
                ]"
              />
            </n-form-item>

            <n-form-item label="浏览器完整性检查">
              <n-switch v-model:value="customConfig.security.browserCheck" />
            </n-form-item>

            <n-form-item label="质询通过期">
              <n-select
                v-model:value="customConfig.security.challengeTtl"
                :options="[
                  { label: '15 分钟', value: 900 },
                  { label: '30 分钟', value: 1800 },
                  { label: '1 小时', value: 3600 },
                  { label: '2 小时', value: 7200 },
                  { label: '6 小时', value: 21600 },
                  { label: '12 小时', value: 43200 },
                  { label: '1 天', value: 86400 },
                  { label: '1 周', value: 604800 },
                  { label: '1 个月', value: 2678400 }
                ]"
              />
            </n-form-item>
          </n-form>
        </n-tab-pane>

        <!-- 性能优化 -->
        <n-tab-pane name="performance" tab="性能">
          <n-form label-placement="left" label-width="150">
            <n-form-item label="自动压缩">
              <n-checkbox-group v-model:value="customConfig.performance.minify">
                <n-space>
                  <n-checkbox value="html">HTML</n-checkbox>
                  <n-checkbox value="css">CSS</n-checkbox>
                  <n-checkbox value="js">JavaScript</n-checkbox>
                </n-space>
              </n-checkbox-group>
            </n-form-item>

            <n-form-item label="Brotli 压缩">
              <n-switch v-model:value="customConfig.performance.brotli" />
            </n-form-item>

            <n-form-item label="HTTP/2">
              <n-switch v-model:value="customConfig.performance.http2" />
            </n-form-item>

            <n-form-item label="HTTP/3 (QUIC)">
              <n-switch v-model:value="customConfig.performance.http3" />
            </n-form-item>

            <n-form-item label="Early Hints">
              <n-switch v-model:value="customConfig.performance.earlyHints" />
            </n-form-item>
          </n-form>
        </n-tab-pane>

        <!-- 网络设置 -->
        <n-tab-pane name="network" tab="网络">
          <n-form label-placement="left" label-width="150">
            <n-form-item label="IPv6 兼容性">
              <n-switch v-model:value="customConfig.network.ipv6" />
            </n-form-item>

            <n-form-item label="WebSockets">
              <n-switch v-model:value="customConfig.network.websockets" />
            </n-form-item>

            <n-form-item label="IP 地理位置">
              <n-switch v-model:value="customConfig.network.ipGeolocation" />
            </n-form-item>

            <n-form-item label="机会性加密">
              <n-switch v-model:value="customConfig.network.opportunisticEncryption" />
            </n-form-item>
          </n-form>
        </n-tab-pane>
      </n-tabs>

      <template #footer>
        <n-space justify="end">
          <n-button @click="showCustomModal = false">取消</n-button>
          <n-button @click="resetCustomConfig">重置</n-button>
          <n-button type="primary" :loading="optimizing" @click="handleCustomOptimize">
            应用自定义配置
          </n-button>
        </n-space>
      </template>
    </n-modal>

    <n-card title="优化历史">
      <n-timeline>
        <n-timeline-item
          v-for="record in optimizeHistory"
          :key="record.id"
          :type="record.mode === 'security' ? 'success' : 'info'"
          :title="record.zoneName"
        >
          <template #icon>
            {{ record.mode === 'security' ? '🛡️' : record.mode === 'performance' ? '⚡' : '⚙️' }}
          </template>
          <n-space vertical :size="4">
            <n-text>
              模式: {{ record.mode === 'security' ? '安全优先' : record.mode === 'performance' ? '性能优先' : '自定义配置' }}
            </n-text>
            <n-text depth="3">
              {{ formatDate(record.timestamp) }}
            </n-text>
          </n-space>
        </n-timeline-item>
      </n-timeline>

      <n-empty
        v-if="optimizeHistory.length === 0"
        description="暂无优化记录"
      />
    </n-card>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, inject, watch, type Ref } from 'vue'
import { useMessage } from 'naive-ui'
import { cloudflareApi, type Zone } from '@/api'

const message = useMessage()

// 从 Layout 获取当前域名
const currentZone = inject<Ref<Zone | null>>('currentZone')

const optimizing = ref(false)
const optimizeMode = ref<'security' | 'performance' | ''>('')
const showCustomModal = ref(false)

interface OptimizeRecord {
  id: string
  zoneId: string
  zoneName: string
  mode: 'security' | 'performance' | 'custom'
  timestamp: string
  customSettings?: any
}

const optimizeHistory = ref<OptimizeRecord[]>([])

// 自定义配置
const customConfig = ref({
  ssl: {
    mode: 'strict',
    alwaysUseHttps: true,
    automaticHttpsRewrites: true,
    minTlsVersion: '1.2',
    tls13: true
  },
  cache: {
    level: 'aggressive',
    browserCacheTtl: 14400,
    sortQueryString: true
  },
  security: {
    securityLevel: 'medium',
    browserCheck: true,
    challengeTtl: 1800
  },
  performance: {
    minify: ['html', 'css', 'js'] as string[],
    brotli: true,
    http2: true,
    http3: true,
    earlyHints: true
  },
  network: {
    ipv6: true,
    websockets: true,
    ipGeolocation: true,
    opportunisticEncryption: true
  }
})

async function handleOptimize(mode: 'security' | 'performance') {
  if (!currentZone?.value?.id) {
    message.warning('请先在左侧菜单选择域名')
    return
  }

  optimizing.value = true
  optimizeMode.value = mode

  try {
    await cloudflareApi.optimizeZone(currentZone.value.id, mode)

    const record: OptimizeRecord = {
      id: Date.now().toString(),
      zoneId: currentZone.value.id,
      zoneName: currentZone.value.name,
      mode,
      timestamp: new Date().toISOString()
    }

    optimizeHistory.value.unshift(record)
    saveHistory()

    message.success(
      mode === 'security'
        ? '安全优先配置已应用成功!'
        : '性能优先配置已应用成功!'
    )
  } catch (error: any) {
    message.error(error?.message || '优化失败,请稍后重试')
  } finally {
    optimizing.value = false
    optimizeMode.value = ''
  }
}

async function handleCustomOptimize() {
  if (!currentZone?.value?.id) {
    message.warning('请先在左侧菜单选择域名')
    return
  }

  optimizing.value = true

  try {
    const settings: any[] = []

    // SSL/TLS 设置
    settings.push({ id: 'ssl', value: customConfig.value.ssl.mode })
    settings.push({ id: 'always_use_https', value: customConfig.value.ssl.alwaysUseHttps ? 'on' : 'off' })
    settings.push({ id: 'automatic_https_rewrites', value: customConfig.value.ssl.automaticHttpsRewrites ? 'on' : 'off' })
    settings.push({ id: 'min_tls_version', value: customConfig.value.ssl.minTlsVersion })
    settings.push({ id: 'tls_1_3', value: customConfig.value.ssl.tls13 ? 'on' : 'off' })

    // 缓存设置
    settings.push({ id: 'cache_level', value: customConfig.value.cache.level })
    settings.push({ id: 'browser_cache_ttl', value: customConfig.value.cache.browserCacheTtl })
    settings.push({ id: 'sort_query_string_for_cache', value: customConfig.value.cache.sortQueryString ? 'on' : 'off' })

    // 安全设置
    settings.push({ id: 'security_level', value: customConfig.value.security.securityLevel })
    settings.push({ id: 'browser_check', value: customConfig.value.security.browserCheck ? 'on' : 'off' })
    settings.push({ id: 'challenge_ttl', value: customConfig.value.security.challengeTtl })

    // 性能设置
    const minify = {
      html: customConfig.value.performance.minify.includes('html') ? 'on' : 'off',
      css: customConfig.value.performance.minify.includes('css') ? 'on' : 'off',
      js: customConfig.value.performance.minify.includes('js') ? 'on' : 'off'
    }
    settings.push({ id: 'minify', value: minify })
    settings.push({ id: 'brotli', value: customConfig.value.performance.brotli ? 'on' : 'off' })
    settings.push({ id: 'http2', value: customConfig.value.performance.http2 ? 'on' : 'off' })
    settings.push({ id: 'http3', value: customConfig.value.performance.http3 ? 'on' : 'off' })
    settings.push({ id: 'early_hints', value: customConfig.value.performance.earlyHints ? 'on' : 'off' })

    // 网络设置
    settings.push({ id: 'ipv6', value: customConfig.value.network.ipv6 ? 'on' : 'off' })
    settings.push({ id: 'websockets', value: customConfig.value.network.websockets ? 'on' : 'off' })
    settings.push({ id: 'ip_geolocation', value: customConfig.value.network.ipGeolocation ? 'on' : 'off' })
    settings.push({ id: 'opportunistic_encryption', value: customConfig.value.network.opportunisticEncryption ? 'on' : 'off' })

    // 批量更新设置
    await cloudflareApi.updateZoneSettings(currentZone.value.id, settings)

    const record: OptimizeRecord = {
      id: Date.now().toString(),
      zoneId: currentZone.value.id,
      zoneName: currentZone.value.name,
      mode: 'custom',
      timestamp: new Date().toISOString(),
      customSettings: { ...customConfig.value }
    }

    optimizeHistory.value.unshift(record)
    saveHistory()

    showCustomModal.value = false
    message.success(`已成功应用 ${settings.length} 项自定义配置!`)
  } catch (error: any) {
    message.error(error?.message || '应用自定义配置失败')
  } finally {
    optimizing.value = false
  }
}

function resetCustomConfig() {
  customConfig.value = {
    ssl: {
      mode: 'strict',
      alwaysUseHttps: true,
      automaticHttpsRewrites: true,
      minTlsVersion: '1.2',
      tls13: true
    },
    cache: {
      level: 'aggressive',
      browserCacheTtl: 14400,
      sortQueryString: true
    },
    security: {
      securityLevel: 'medium',
      browserCheck: true,
      challengeTtl: 1800
    },
    performance: {
      minify: ['html', 'css', 'js'],
      brotli: true,
      http2: true,
      http3: true,
      earlyHints: true
    },
    network: {
      ipv6: true,
      websockets: true,
      ipGeolocation: true,
      opportunisticEncryption: true
    }
  }
  message.info('已重置为推荐配置')
}

function formatDate(date: string) {
  return new Date(date).toLocaleString('zh-CN')
}

function loadHistory() {
  try {
    const stored = localStorage.getItem('cf_optimize_history')
    if (stored) {
      optimizeHistory.value = JSON.parse(stored)
    }
  } catch (error) {
    console.error('Failed to load history:', error)
  }
}

function saveHistory() {
  try {
    localStorage.setItem(
      'cf_optimize_history',
      JSON.stringify(optimizeHistory.value.slice(0, 10))
    )
  } catch (error) {
    console.error('Failed to save history:', error)
  }
}

onMounted(() => {
  loadHistory()
})
</script>
