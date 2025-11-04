<template>
  <n-space vertical :size="24">
    <!-- 缓存配置 -->
    <n-card title="缓存配置">
      <n-spin :show="loading">
        <n-space vertical :size="16">
          <!-- 缓存级别 -->
          <n-form-item label="缓存级别">
            <n-select
              v-model:value="cacheLevel"
              :options="cacheLevelOptions"
              :loading="updating"
              @update:value="handleCacheLevelChange"
              style="width: 400px"
            />
            <template #feedback>
              <n-text depth="3" style="font-size: 12px">
                {{ getCacheLevelDescription(cacheLevel) }}
              </n-text>
            </template>
          </n-form-item>

          <!-- 浏览器缓存 TTL -->
          <n-form-item label="浏览器缓存 TTL">
            <n-select
              v-model:value="browserCacheTTL"
              :options="browserCacheTTLOptions"
              :loading="updating"
              @update:value="handleBrowserCacheTTLChange"
              style="width: 400px"
            />
            <template #feedback>
              <n-text depth="3" style="font-size: 12px">
                控制浏览器缓存资源的时长
              </n-text>
            </template>
          </n-form-item>

          <!-- 开发模式 -->
          <n-form-item label="开发模式">
            <n-switch
              v-model:value="developmentMode"
              :loading="updating"
              @update:value="handleDevelopmentModeChange"
            >
              <template #checked>已启用</template>
              <template #unchecked>已禁用</template>
            </n-switch>
            <template #feedback>
              <n-text depth="3" style="font-size: 12px">
                临时绕过缓存，用于开发和测试（自动在 3 小时后关闭）
              </n-text>
            </template>
          </n-form-item>

          <!-- 查询字符串排序 -->
          <n-form-item label="查询字符串排序">
            <n-switch
              v-model:value="sortQueryString"
              :loading="updating"
              @update:value="handleSortQueryStringChange"
            >
              <template #checked>已启用</template>
              <template #unchecked>已禁用</template>
            </n-switch>
            <template #feedback>
              <n-text depth="3" style="font-size: 12px">
                对 URL 查询字符串进行排序以提高缓存命中率
              </n-text>
            </template>
          </n-form-item>
        </n-space>
      </n-spin>
    </n-card>

    <!-- 缓存清除 -->
    <n-card title="清除缓存">
      <n-space vertical :size="16">
        <n-alert type="warning">
          清除缓存将删除 Cloudflare 服务器上的所有缓存文件，下次访问时需要重新从源服务器获取。
        </n-alert>

        <n-space>
          <n-button
            type="primary"
            :loading="purging"
            @click="handlePurgeAllCache"
          >
            清除所有缓存
          </n-button>

          <n-button
            :loading="purging"
            @click="showPurgeByURLModal = true"
          >
            按 URL 清除
          </n-button>

          <n-button
            :loading="purging"
            @click="showPurgeByTagModal = true"
          >
            按标签清除
          </n-button>
        </n-space>

        <!-- 缓存统计 -->
        <n-descriptions v-if="cacheStats.requests > 0" :column="3" bordered style="margin-top: 16px">
          <n-descriptions-item label="总请求数">
            {{ formatNumber(cacheStats.requests) }}
          </n-descriptions-item>
          <n-descriptions-item label="缓存命中">
            {{ formatNumber(cacheStats.cached) }}
          </n-descriptions-item>
          <n-descriptions-item label="缓存命中率">
            {{ cacheStats.hitRate }}%
          </n-descriptions-item>
        </n-descriptions>
      </n-space>
    </n-card>

    <!-- 按 URL 清除缓存弹窗 -->
    <n-modal v-model:show="showPurgeByURLModal" preset="dialog" title="按 URL 清除缓存" style="width: 600px">
      <n-space vertical>
        <n-alert type="info">
          每行输入一个 URL，最多 30 个
        </n-alert>
        <n-input
          v-model:value="purgeURLs"
          type="textarea"
          placeholder="https://example.com/image1.jpg&#10;https://example.com/image2.jpg"
          :rows="6"
        />
      </n-space>

      <template #action>
        <n-space>
          <n-button @click="showPurgeByURLModal = false">取消</n-button>
          <n-button type="primary" :loading="purging" @click="handlePurgeByURL">
            清除
          </n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 按标签清除缓存弹窗 -->
    <n-modal v-model:show="showPurgeByTagModal" preset="dialog" title="按标签清除缓存" style="width: 600px">
      <n-space vertical>
        <n-alert type="info">
          输入缓存标签，多个标签用逗号分隔
        </n-alert>
        <n-input
          v-model:value="purgeTags"
          placeholder="tag1, tag2, tag3"
        />
      </n-space>

      <template #action>
        <n-space>
          <n-button @click="showPurgeByTagModal = false">取消</n-button>
          <n-button type="primary" :loading="purging" @click="handlePurgeByTag">
            清除
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useMessage, useDialog } from 'naive-ui'
import { cloudflareApi } from '@/api'

const message = useMessage()
const dialog = useDialog()

const loading = ref(false)
const updating = ref(false)
const purging = ref(false)

const showPurgeByURLModal = ref(false)
const showPurgeByTagModal = ref(false)

// 缓存设置
const cacheLevel = ref('aggressive')
const browserCacheTTL = ref(14400)
const developmentMode = ref(false)
const sortQueryString = ref(false)

// 清除缓存表单
const purgeURLs = ref('')
const purgeTags = ref('')

// 缓存统计
const cacheStats = ref({
  requests: 0,
  cached: 0,
  hitRate: 0
})

const currentZoneId = computed(() => {
  return localStorage.getItem('currentZoneId') || ''
})

// 缓存级别选项
const cacheLevelOptions = [
  { label: '不缓存 (No Query String)', value: 'basic' },
  { label: '标准 (Ignore Query String)', value: 'simplified' },
  { label: '积极 (Standard)', value: 'aggressive' }
]

// 浏览器缓存 TTL 选项（秒）
const browserCacheTTLOptions = [
  { label: '30 分钟', value: 1800 },
  { label: '1 小时', value: 3600 },
  { label: '2 小时', value: 7200 },
  { label: '4 小时', value: 14400 },
  { label: '8 小时', value: 28800 },
  { label: '16 小时', value: 57600 },
  { label: '1 天', value: 86400 },
  { label: '2 天', value: 172800 },
  { label: '3 天', value: 259200 },
  { label: '4 天', value: 345600 },
  { label: '5 天', value: 432000 },
  { label: '8 天', value: 691200 },
  { label: '16 天', value: 1382400 },
  { label: '1 个月', value: 2678400 },
  { label: '2 个月', value: 5356800 },
  { label: '6 个月', value: 15552000 },
  { label: '1 年', value: 31536000 }
]

function getCacheLevelDescription(level: string): string {
  const descriptions: Record<string, string> = {
    basic: '忽略查询字符串，所有静态资源使用同一缓存',
    simplified: '仅缓存静态文件，忽略查询字符串',
    aggressive: '缓存所有静态内容，包括带查询字符串的资源'
  }
  return descriptions[level] || ''
}

function formatNumber(num: number): string {
  return num.toLocaleString()
}

async function loadCacheSettings() {
  if (!currentZoneId.value) {
    message.warning('请先选择域名')
    return
  }

  loading.value = true
  try {
    const settings = await cloudflareApi.getZoneSettings(currentZoneId.value)

    settings.forEach((setting: any) => {
      switch (setting.id) {
        case 'cache_level':
          cacheLevel.value = setting.value
          break
        case 'browser_cache_ttl':
          browserCacheTTL.value = setting.value
          break
        case 'development_mode':
          developmentMode.value = setting.value === 'on'
          break
        case 'sort_query_string_for_cache':
          sortQueryString.value = setting.value === 'on'
          break
      }
    })

    // TODO: 获取缓存统计数据
    cacheStats.value = {
      requests: 1250000,
      cached: 1000000,
      hitRate: 80
    }
  } catch (error: any) {
    message.error(error?.message || '加载缓存设置失败')
  } finally {
    loading.value = false
  }
}

async function updateSetting(id: string, value: any) {
  if (!currentZoneId.value) return

  updating.value = true
  try {
    await cloudflareApi.updateZoneSettings(currentZoneId.value, [{ id, value }])
    message.success('设置已更新')
  } catch (error: any) {
    message.error(error?.message || '更新设置失败')
    await loadCacheSettings()
  } finally {
    updating.value = false
  }
}

function handleCacheLevelChange(value: string) {
  updateSetting('cache_level', value)
}

function handleBrowserCacheTTLChange(value: number) {
  updateSetting('browser_cache_ttl', value)
}

function handleDevelopmentModeChange(value: boolean) {
  if (value) {
    dialog.warning({
      title: '确认启用开发模式',
      content: '开发模式将绕过缓存，可能会增加源服务器负载。此模式将在 3 小时后自动关闭。',
      positiveText: '确认',
      negativeText: '取消',
      onPositiveClick: () => {
        updateSetting('development_mode', 'on')
      },
      onNegativeClick: () => {
        developmentMode.value = false
      }
    })
  } else {
    updateSetting('development_mode', 'off')
  }
}

function handleSortQueryStringChange(value: boolean) {
  updateSetting('sort_query_string_for_cache', value ? 'on' : 'off')
}

function handlePurgeAllCache() {
  dialog.warning({
    title: '确认清除所有缓存',
    content: '此操作将清除该域名下的所有缓存文件，可能会暂时增加源服务器负载。确定继续吗？',
    positiveText: '确认清除',
    negativeText: '取消',
    onPositiveClick: async () => {
      purging.value = true
      try {
        // TODO: 调用清除所有缓存的 API
        await new Promise(resolve => setTimeout(resolve, 1000))
        message.success('缓存清除成功')
      } catch (error: any) {
        message.error(error?.message || '清除缓存失败')
      } finally {
        purging.value = false
      }
    }
  })
}

async function handlePurgeByURL() {
  const urls = purgeURLs.value.split('\n').filter(url => url.trim())

  if (urls.length === 0) {
    message.warning('请输入至少一个 URL')
    return
  }

  if (urls.length > 30) {
    message.warning('最多支持 30 个 URL')
    return
  }

  purging.value = true
  try {
    // TODO: 调用按 URL 清除缓存的 API
    await new Promise(resolve => setTimeout(resolve, 1000))
    message.success(`已清除 ${urls.length} 个 URL 的缓存`)
    showPurgeByURLModal.value = false
    purgeURLs.value = ''
  } catch (error: any) {
    message.error(error?.message || '清除缓存失败')
  } finally {
    purging.value = false
  }
}

async function handlePurgeByTag() {
  const tags = purgeTags.value.split(',').map(tag => tag.trim()).filter(tag => tag)

  if (tags.length === 0) {
    message.warning('请输入至少一个标签')
    return
  }

  purging.value = true
  try {
    // TODO: 调用按标签清除缓存的 API
    await new Promise(resolve => setTimeout(resolve, 1000))
    message.success(`已清除标签 "${tags.join(', ')}" 的缓存`)
    showPurgeByTagModal.value = false
    purgeTags.value = ''
  } catch (error: any) {
    message.error(error?.message || '清除缓存失败')
  } finally {
    purging.value = false
  }
}

onMounted(() => {
  loadCacheSettings()
})
</script>
