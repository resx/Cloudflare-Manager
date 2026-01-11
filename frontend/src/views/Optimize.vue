<template>
  <!-- Auto Optimize - Island Theme -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">自动优化</h1>
      <p class="text-sm text-muted-foreground mt-1">
        管理域名: <span class="font-medium">{{ currentZone?.name || '未选择' }}</span>
      </p>
    </div>

    <!-- Quick Actions -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
      <div class="metric-card p-6 cursor-pointer hover:border-primary transition-colors" @click="optimizeForSecurity">
        <div class="flex items-start gap-4">
          <div class="text-4xl">🛡️</div>
          <div class="flex-1">
            <h3 class="font-semibold mb-2">安全优先模式</h3>
            <p class="text-sm text-muted-foreground mb-3">
              适用于金融、政府、企业官网等对安全要求极高的场景
            </p>
            <ul class="text-xs text-muted-foreground space-y-1">
              <li>✓ 安全级别：高</li>
              <li>✓ SSL模式：严格</li>
              <li>✓ 强制HTTPS</li>
              <li>✓ 浏览器检查</li>
            </ul>
          </div>
        </div>
      </div>

      <div class="metric-card p-6 cursor-pointer hover:border-primary transition-colors" @click="optimizeForSpeed">
        <div class="flex items-start gap-4">
          <div class="text-4xl">⚡</div>
          <div class="flex-1">
            <h3 class="font-semibold mb-2">速度优先模式</h3>
            <p class="text-sm text-muted-foreground mb-3">
              适用于电商、媒体、个人博客等对性能要求高的场景
            </p>
            <ul class="text-xs text-muted-foreground space-y-1">
              <li>✓ 缓存级别：积极</li>
              <li>✓ Brotli压缩</li>
              <li>✓ HTTP/3启用</li>
              <li>✓ 浏览器缓存：1年</li>
            </ul>
          </div>
        </div>
      </div>
    </div>

    <!-- Current Settings -->
    <div class="metric-card p-6">
      <h2 class="text-lg font-semibold mb-4">当前优化状态</h2>
      <div v-if="currentSettings" class="space-y-3">
        <div class="flex justify-between text-sm">
          <span class="text-muted-foreground">安全级别</span>
          <span class="font-medium">{{ currentSettings.security_level || '-' }}</span>
        </div>
        <div class="flex justify-between text-sm">
          <span class="text-muted-foreground">SSL模式</span>
          <span class="font-medium">{{ currentSettings.ssl || '-' }}</span>
        </div>
        <div class="flex justify-between text-sm">
          <span class="text-muted-foreground">缓存级别</span>
          <span class="font-medium">{{ currentSettings.cache_level || '-' }}</span>
        </div>
        <div class="flex justify-between text-sm">
          <span class="text-muted-foreground">Brotli压缩</span>
          <span class="font-medium">{{ currentSettings.brotli ? '启用' : '关闭' }}</span>
        </div>
      </div>
      <div v-else class="text-center text-muted-foreground py-4">
        加载中...
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, inject, onMounted, type Ref } from 'vue'
import { cloudflareApi, type Zone } from '@/api'
import { toast } from '@/utils/toast'

const currentZone = inject<Ref<Zone | null>>('currentZone')
const currentSettings = ref<any>(null)

async function loadCurrentSettings() {
  if (!currentZone?.value) return
  try {
    const settings = await cloudflareApi.getZoneSettings(currentZone.value.id)
    currentSettings.value = {
      security_level: settings.security_level?.value,
      ssl: settings.ssl?.value,
      cache_level: settings.cache_level?.value,
      brotli: settings.brotli?.value === 'on',
    }
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
}

async function optimizeForSecurity() {
  if (!currentZone?.value) {
    toast.warning('请先选择域名')
    return
  }

  if (!confirm('确定要应用安全优先配置吗？')) return

  try {
    const updates = [
      cloudflareApi.updateZoneSetting(currentZone.value.id, 'security_level', 'high'),
      cloudflareApi.updateZoneSetting(currentZone.value.id, 'ssl', 'strict'),
      cloudflareApi.updateZoneSetting(currentZone.value.id, 'always_use_https', 'on'),
      cloudflareApi.updateZoneSetting(currentZone.value.id, 'browser_check', 'on'),
    ]
    await Promise.all(updates)
    toast.success('安全优先配置已应用')
    loadCurrentSettings()
  } catch (error) {
    console.error('Failed to optimize for security:', error)
    toast.error('配置失败')
  }
}

async function optimizeForSpeed() {
  if (!currentZone?.value) {
    toast.warning('请先选择域名')
    return
  }

  if (!confirm('确定要应用速度优先配置吗？')) return

  try {
    const updates = [
      cloudflareApi.updateZoneSetting(currentZone.value.id, 'cache_level', 'aggressive'),
      cloudflareApi.updateZoneSetting(currentZone.value.id, 'brotli', 'on'),
      cloudflareApi.updateZoneSetting(currentZone.value.id, 'http3', 'on'),
      cloudflareApi.updateZoneSetting(currentZone.value.id, 'browser_cache_ttl', 31536000),
    ]
    await Promise.all(updates)
    toast.success('速度优先配置已应用')
    loadCurrentSettings()
  } catch (error) {
    console.error('Failed to optimize for speed:', error)
    toast.error('配置失败')
  }
}

onMounted(() => {
  loadCurrentSettings()
})
</script>
