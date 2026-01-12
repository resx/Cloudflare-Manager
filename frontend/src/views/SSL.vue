<template>
  <n-space vertical :size="24">
    <n-card title="SSL/TLS 设置">
      <n-spin :show="loading">
        <n-space vertical :size="16">
          <!-- SSL 加密模式 -->
          <n-form-item label="SSL/TLS 加密模式">
            <n-select
              v-model:value="sslMode"
              :options="sslModeOptions"
              :loading="updating"
              @update:value="handleSSLModeChange"
              style="width: 400px"
            />
            <template #feedback>
              <n-text depth="3" style="font-size: 12px">
                {{ getSslModeDescription(sslMode) }}
              </n-text>
            </template>
          </n-form-item>

          <!-- 始终使用 HTTPS -->
          <n-form-item label="始终使用 HTTPS">
            <n-switch
              v-model:value="alwaysUseHttps"
              :loading="updating"
              @update:value="handleAlwaysHttpsChange"
            >
              <template #checked>已启用</template>
              <template #unchecked>已禁用</template>
            </n-switch>
            <template #feedback>
              <n-text depth="3" style="font-size: 12px">
                自动将所有 HTTP 请求重定向到 HTTPS
              </n-text>
            </template>
          </n-form-item>

          <!-- 自动 HTTPS 重写 -->
          <n-form-item label="自动 HTTPS 重写">
            <n-switch
              v-model:value="automaticHttpsRewrites"
              :loading="updating"
              @update:value="handleAutomaticHttpsRewritesChange"
            >
              <template #checked>已启用</template>
              <template #unchecked>已禁用</template>
            </n-switch>
            <template #feedback>
              <n-text depth="3" style="font-size: 12px">
                自动将不安全的 HTTP 链接重写为 HTTPS
              </n-text>
            </template>
          </n-form-item>

          <!-- 最低 TLS 版本 -->
          <n-form-item label="最低 TLS 版本">
            <n-select
              v-model:value="minTlsVersion"
              :options="tlsVersionOptions"
              :loading="updating"
              @update:value="handleMinTlsVersionChange"
              style="width: 400px"
            />
            <template #feedback>
              <n-text depth="3" style="font-size: 12px">
                设置客户端连接时允许的最低 TLS 版本
              </n-text>
            </template>
          </n-form-item>

          <!-- TLS 1.3 -->
          <n-form-item label="TLS 1.3">
            <n-switch
              v-model:value="tls13"
              :loading="updating"
              @update:value="handleTls13Change"
            >
              <template #checked>已启用</template>
              <template #unchecked>已禁用</template>
            </n-switch>
            <template #feedback>
              <n-text depth="3" style="font-size: 12px">
                启用最新的 TLS 1.3 协议，提供更好的性能和安全性
              </n-text>
            </template>
          </n-form-item>

          <!-- 机会性加密 -->
          <n-form-item label="机会性加密">
            <n-switch
              v-model:value="opportunisticEncryption"
              :loading="updating"
              @update:value="handleOpportunisticEncryptionChange"
            >
              <template #checked>已启用</template>
              <template #unchecked>已禁用</template>
            </n-switch>
            <template #feedback>
              <n-text depth="3" style="font-size: 12px">
                为支持的浏览器提供 HTTP/2 服务器推送加密
              </n-text>
            </template>
          </n-form-item>
        </n-space>
      </n-spin>
    </n-card>

    <!-- SSL 证书信息 -->
    <n-card title="SSL 证书信息">
      <n-spin :show="loading">
        <n-descriptions :column="1" bordered>
          <n-descriptions-item label="证书状态">
            <n-tag v-if="certInfo.status === 'active'" type="success">有效</n-tag>
            <n-tag v-else type="warning">{{ certInfo.status || '未知' }}</n-tag>
          </n-descriptions-item>
          <n-descriptions-item label="证书类型">
            {{ certInfo.type || '-' }}
          </n-descriptions-item>
          <n-descriptions-item label="颁发者">
            {{ certInfo.issuer || '-' }}
          </n-descriptions-item>
          <n-descriptions-item label="签名算法">
            {{ certInfo.signature || '-' }}
          </n-descriptions-item>
        </n-descriptions>
      </n-spin>
    </n-card>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch, inject, type Ref } from 'vue'
import { useMessage } from 'naive-ui'
import { cloudflareApi, type Zone } from '@/api'
import { useAccountStore } from '@/stores/account'
import { toast } from '@/utils/toast'
import { logHistory } from '@/utils/history'

const message = useMessage()
const accountStore = useAccountStore()

// 从 Layout 获取当前域名
const currentZone = inject<Ref<Zone | null>>('currentZone')

const loading = ref(false)
const updating = ref(false)

// SSL 设置
const sslMode = ref('flexible')
const alwaysUseHttps = ref(false)
const automaticHttpsRewrites = ref(false)
const minTlsVersion = ref('1.0')
const tls13 = ref(false)
const opportunisticEncryption = ref(false)

// 证书信息
const certInfo = ref({
  status: '',
  type: '',
  issuer: '',
  signature: ''
})

// SSL 模式选项
const sslModeOptions = [
  { label: '关闭 (Off)', value: 'off' },
  { label: '灵活 (Flexible)', value: 'flexible' },
  { label: '完全 (Full)', value: 'full' },
  { label: '完全(严格) (Full Strict)', value: 'strict' }
]

// TLS 版本选项
const tlsVersionOptions = [
  { label: 'TLS 1.0', value: '1.0' },
  { label: 'TLS 1.1', value: '1.1' },
  { label: 'TLS 1.2', value: '1.2' },
  { label: 'TLS 1.3', value: '1.3' }
]

function getSslModeDescription(mode: string): string {
  const descriptions: Record<string, string> = {
    off: '不使用 HTTPS 加密（不推荐）',
    flexible: 'Cloudflare 到访客使用 HTTPS，Cloudflare 到源服务器可以使用 HTTP',
    full: 'Cloudflare 到访客和源服务器都使用 HTTPS，但不验证源服务器证书',
    strict: 'Cloudflare 到访客和源服务器都使用 HTTPS，并验证源服务器证书（推荐）'
  }
  return descriptions[mode] || ''
}

async function loadSSLSettings() {
  if (!currentZone?.value?.id) {
    console.log('No currentZone available for SSL settings')
    return
  }

  console.log('Loading SSL settings for zone:', currentZone.value.name)
  loading.value = true
  try {
    const settings = await cloudflareApi.getZoneSettings(currentZone.value.id)

    // 解析设置
    settings.forEach((setting: any) => {
      switch (setting.id) {
        case 'ssl':
          sslMode.value = setting.value
          break
        case 'always_use_https':
          alwaysUseHttps.value = setting.value === 'on'
          break
        case 'automatic_https_rewrites':
          automaticHttpsRewrites.value = setting.value === 'on'
          break
        case 'min_tls_version':
          minTlsVersion.value = setting.value
          break
        case 'tls_1_3':
          tls13.value = setting.value === 'on'
          break
        case 'opportunistic_encryption':
          opportunisticEncryption.value = setting.value === 'on'
          break
      }
    })

    // 获取 SSL 证书信息
    try {
      const certificates = await cloudflareApi.getSslCertificates(currentZone.value.id)
      if (certificates && certificates.length > 0) {
        const cert = certificates[0]
        const detail = cert.certificates && cert.certificates.length > 0 ? cert.certificates[0] : null

        certInfo.value = {
          status: cert.status || 'unknown',
          type: cert.type || 'Universal SSL',
          issuer: detail?.issuer || '-',
          signature: detail?.signature || '-'
        }
      } else {
        // 如果没有证书数据，使用默认值
        certInfo.value = {
          status: 'active',
          type: 'Universal SSL',
          issuer: '-',
          signature: '-'
        }
      }
    } catch (error) {
      console.warn('Failed to load SSL certificates:', error)
      // 如果获取证书失败，使用默认值
      certInfo.value = {
        status: 'unknown',
        type: 'Universal SSL',
        issuer: '-',
        signature: '-'
      }
    }
  } catch (error: any) {
    message.error(error?.message || '加载 SSL 设置失败')
  } finally {
    loading.value = false
  }
}

async function updateSetting(id: string, value: any) {
  if (!currentZone?.value?.id) {
    message.error('未选择域名')
    return
  }

  updating.value = true
  try {
    await cloudflareApi.updateZoneSettings(currentZone.value.id, [{ id, value }])
    logHistory.ssl('更新 SSL/TLS 设置', `模式：${sslMode.value}，TLS版本：${minTlsVersion.value}`)
    toast.success('SSL 设置已更新')
  } catch (error: any) {
    message.error(error?.message || '更新设置失败')
    // 重新加载设置以恢复旧值
    await loadSSLSettings()
  } finally {
    updating.value = false
  }
}

function handleSSLModeChange(value: string) {
  updateSetting('ssl', value)
}

function handleAlwaysHttpsChange(value: boolean) {
  updateSetting('always_use_https', value ? 'on' : 'off')
}

function handleAutomaticHttpsRewritesChange(value: boolean) {
  updateSetting('automatic_https_rewrites', value ? 'on' : 'off')
}

function handleMinTlsVersionChange(value: string) {
  updateSetting('min_tls_version', value)
}

function handleTls13Change(value: boolean) {
  updateSetting('tls_1_3', value ? 'on' : 'off')
}

function handleOpportunisticEncryptionChange(value: boolean) {
  updateSetting('opportunistic_encryption', value ? 'on' : 'off')
}

onMounted(() => {
  loadSSLSettings()
})

// 监听 currentZone 变化，自动重新加载 SSL 设置
watch(() => currentZone?.value?.id, () => {
  loadSSLSettings()
})
</script>
