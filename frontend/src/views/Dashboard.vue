<template>
  <!-- Island Theme Dashboard (GitLab Style) -->
  <div class="animate-in">
    <!-- Greeting Section -->
    <div class="flex items-center mb-6">
      <div class="w-12 h-12 rounded-full flex items-center justify-center text-2xl mr-4" style="background-color: #e0e7ff; color: #4338ca;">
        ✿
      </div>
      <div>
        <div class="text-xs text-muted-foreground">Today's highlights</div>
        <h1 class="text-3xl font-semibold text-foreground">Hi, {{ accountStore.currentAccount?.alias || 'Moriarty' }}</h1>
      </div>
    </div>

    <!-- Welcome Banner with Gradient -->
    <div class="banner-gradient rounded-lg p-6 mb-8 relative">
      <button 
        class="absolute top-4 right-4 text-muted-foreground hover:text-foreground text-xl leading-none"
        @click="showBanner = false"
        v-if="showBanner"
      >
        ×
      </button>
      <h3 class="text-lg font-semibold mb-2">欢迎使用 Cloudflare 管理平台</h3>
      <p class="text-sm text-muted-foreground leading-relaxed">
        这是一款专为 Cloudflare 用户打造的全功能可视化管理平台，让复杂的 CDN 配置变得简单直观。
        我们提供了域名管理、Workers 部署、DNS 配置、缓存优化等完整功能，帮助您轻松管理 Cloudflare 资源。
      </p>
    </div>

    <!-- Metric Cards Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-5 mb-8">
      <!-- Zones Card -->
      <div class="metric-card h-36">
        <div class="flex justify-between items-start mb-3">
          <span class="text-sm text-muted-foreground">管理的域名</span>
          <span class="text-xl">🌐</span>
        </div>
        <div class="text-3xl font-semibold mb-1">{{ zones.length }}</div>
        <div class="text-xs text-muted-foreground">
          总域名数<br>
          刚刚更新
        </div>
      </div>

      <!-- DNS Records Card -->
      <div class="metric-card h-36">
        <div class="flex justify-between items-start mb-3">
          <span class="text-sm text-muted-foreground">DNS 记录</span>
          <span class="text-xl">🔧</span>
        </div>
        <div class="text-3xl font-semibold mb-1">{{ totalDnsRecords }}</div>
        <div class="text-xs text-muted-foreground">
          DNS 总记录数<br>
          刚刚更新
        </div>
      </div>

      <!-- Accounts Card -->
      <div class="metric-card h-36">
        <div class="flex justify-between items-start mb-3">
          <span class="text-sm text-muted-foreground">账户数量</span>
          <span class="text-xl">👤</span>
        </div>
        <div class="text-3xl font-semibold mb-1">{{ accountStore.accounts.length }}</div>
        <div class="text-xs text-muted-foreground">
          已添加账户<br>
          刚刚更新
        </div>
      </div>

      <!-- Workers Card -->
      <div class="metric-card h-36">
        <div class="flex justify-between items-start mb-3">
          <span class="text-sm text-muted-foreground">Workers 脚本</span>
          <span class="text-xl">⚙️</span>
        </div>
        <div class="text-3xl font-semibold mb-1">{{ workersCount }}</div>
        <div class="text-xs text-muted-foreground">
          已部署脚本<br>
          刚刚更新
        </div>
      </div>
    </div>

    <!-- Quick Access Section -->
    <div class="mb-8">
      <h3 class="text-base font-semibold mb-4">快速访问</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <button 
          @click="$router.push('/quick-deploy')"
          class="metric-card h-24 hover:border-primary transition-colors cursor-pointer text-left"
        >
          <div class="flex items-center gap-3">
            <span class="text-2xl">🚀</span>
            <div>
              <div class="font-medium text-sm">一键加速部署</div>
              <div class="text-xs text-muted-foreground">快速优化配置</div>
            </div>
          </div>
        </button>

        <button 
          @click="$router.push('/dns')"
          class="metric-card h-24 hover:border-primary transition-colors cursor-pointer text-left"
        >
          <div class="flex items-center gap-3">
            <span class="text-2xl">🔧</span>
            <div>
              <div class="font-medium text-sm">管理 DNS 记录</div>
              <div class="text-xs text-muted-foreground">域名解析配置</div>
            </div>
          </div>
        </button>

        <button 
          @click="$router.push('/workers')"
          class="metric-card h-24 hover:border-primary transition-colors cursor-pointer text-left"
        >
          <div class="flex items-center gap-3">
            <span class="text-2xl">⚙️</span>
            <div>
              <div class="font-medium text-sm">Workers 管理</div>
              <div class="text-xs text-muted-foreground">边缘计算脚本</div>
            </div>
          </div>
        </button>

        <button 
          @click="$router.push('/firewall')"
          class="metric-card h-24 hover:border-primary transition-colors cursor-pointer text-left"
        >
          <div class="flex items-center gap-3">
            <span class="text-2xl">🛡️</span>
            <div>
              <div class="font-medium text-sm">配置防火墙</div>
              <div class="text-xs text-muted-foreground">安全规则设置</div>
            </div>
          </div>
        </button>
      </div>
    </div>

    <!-- Recent Zones Section -->
    <div v-if="zones.length > 0">
      <h3 class="text-base font-semibold mb-4">域名列表</h3>
      <div class="metric-card">
        <div class="space-y-3">
          <div 
            v-for="zone in zones.slice(0, 5)" 
            :key="zone.id"
            class="flex items-center justify-between py-2 border-b border-border last:border-b-0"
          >
            <div class="flex items-center gap-3">
              <span class="text-lg">🌐</span>
              <div>
                <div class="font-medium text-sm">{{ zone.name }}</div>
                <div class="text-xs text-muted-foreground">{{ zone.name_servers?.slice(0, 2).join(', ') || '-' }}</div>
              </div>
            </div>
            <span 
              :class="[
                'px-2 py-1 text-xs rounded-full',
                zone.status === 'active' 
                  ? 'bg-success text-success-foreground' 
                  : 'bg-muted text-muted-foreground'
              ]"
            >
              {{ zone.status }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Items Need Attention -->
    <div class="mt-8">
      <h3 class="text-base font-semibold mb-4">需要关注的项目</h3>
      <div class="metric-card flex items-center p-6">
        <div class="check-circle mr-4">✓</div>
        <div class="font-medium text-sm">很好！所有待办事项已完成。</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { cloudflareApi, type Zone } from '@/api'
import { useAccountStore } from '@/stores/account'

const accountStore = useAccountStore()
const loading = ref(false)
const showBanner = ref(true)
const zones = ref<Zone[]>([])
const totalDnsRecords = ref(0)
const workersCount = ref(0)

async function loadDashboardData() {
  if (!accountStore.currentAccount) return

  loading.value = true
  try {
    // Load zones
    zones.value = await cloudflareApi.getZones()

    // Load DNS records count
    let dnsTotal = 0
    for (const zone of zones.value.slice(0, 3)) { // Only count first 3 zones for performance
      const records = await cloudflareApi.getDnsRecords(zone.id)
      dnsTotal += records.length
    }
    totalDnsRecords.value = dnsTotal

    // Load workers count (if API supports it)
    try {
      const workers = await cloudflareApi.listWorkers(accountStore.currentAccount.accountId)
      workersCount.value = workers?.length || 0
    } catch {
      workersCount.value = 0
    }
  } catch (error) {
    console.error('Failed to load dashboard data:', error)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadDashboardData()
})
</script>
