<template>
  <n-space vertical :size="24">
    <n-card title="自动优化配置">
      <n-alert type="info" style="margin-bottom: 24px">
        一键应用最佳配置,让 Cloudflare 设置从 10 分钟的复杂操作变成 1 键搞定
      </n-alert>

      <n-form label-placement="left" label-width="120">
        <n-form-item label="选择域名">
          <n-select
            v-model:value="selectedZone"
            :options="zoneOptions"
            placeholder="请选择要优化的域名"
            :loading="loadingZones"
          />
        </n-form-item>
      </n-form>
    </n-card>

    <n-grid :cols="2" :x-gap="24">
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
              :disabled="!selectedZone"
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
              :disabled="!selectedZone"
              @click="handleOptimize('performance')"
            >
              应用性能优先配置
            </n-button>
          </n-space>
        </n-card>
      </n-gi>
    </n-grid>

    <n-card title="优化历史">
      <n-timeline>
        <n-timeline-item
          v-for="record in optimizeHistory"
          :key="record.id"
          :type="record.mode === 'security' ? 'success' : 'info'"
          :title="record.zoneName"
        >
          <template #icon>
            {{ record.mode === 'security' ? '🛡️' : '⚡' }}
          </template>
          <n-space vertical :size="4">
            <n-text>
              模式: {{ record.mode === 'security' ? '安全优先' : '性能优先' }}
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
import { ref, onMounted, computed } from 'vue'
import { useMessage } from 'naive-ui'
import { cloudflareApi, type Zone } from '@/api'
import { useAccountStore } from '@/stores/account'

const accountStore = useAccountStore()
const message = useMessage()

const loadingZones = ref(false)
const optimizing = ref(false)
const optimizeMode = ref<'security' | 'performance' | ''>('')
const zones = ref<Zone[]>([])
const selectedZone = ref('')

interface OptimizeRecord {
  id: string
  zoneId: string
  zoneName: string
  mode: 'security' | 'performance'
  timestamp: string
}

const optimizeHistory = ref<OptimizeRecord[]>([])

const zoneOptions = computed(() =>
  zones.value.map(zone => ({
    label: zone.name,
    value: zone.id
  }))
)

async function loadZones() {
  if (!accountStore.currentAccount) return

  loadingZones.value = true
  try {
    zones.value = await cloudflareApi.getZones()
  } catch (error) {
    message.error('加载域名列表失败')
  } finally {
    loadingZones.value = false
  }
}

async function handleOptimize(mode: 'security' | 'performance') {
  if (!selectedZone.value) {
    message.warning('请先选择域名')
    return
  }

  optimizing.value = true
  optimizeMode.value = mode

  try {
    await cloudflareApi.optimizeZone(selectedZone.value, mode)

    const zone = zones.value.find(z => z.id === selectedZone.value)
    const record: OptimizeRecord = {
      id: Date.now().toString(),
      zoneId: selectedZone.value,
      zoneName: zone?.name || '',
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
  loadZones()
  loadHistory()
})
</script>
