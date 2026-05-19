<template>
  <!-- Floating Island Layout -->
  <div class="flex h-screen p-4 gap-4">
    
    <!-- Sidebar - Floating Island -->
    <aside
      :class="['glass-sidebar transition-all duration-300 flex flex-col rounded-2xl', collapsed ? 'w-16' : 'w-60']"
      style="padding: 20px 10px;"
    >
      <!-- Logo -->
      <div class="px-3 pb-5">
        <div v-if="!collapsed" class="font-bold text-lg text-foreground flex items-center gap-2">
          <component :is="CloudOutline" class="w-5 h-5 text-primary" />
          <span>CF Manager</span>
        </div>
        <div v-else class="flex justify-center">
          <component :is="CloudOutline" class="w-6 h-6 text-primary" />
        </div>
      </div>

      <!-- Navigation -->
      <nav class="flex-1 overflow-y-auto">
        <!-- Main Menu Section -->
        <div class="mb-4">
          <div v-if="!collapsed" class="section-title">主菜单</div>
          <a
            v-for="item in mainMenuItems"
            :key="item.path"
            @click.prevent="router.push(item.path)"
            :class="['nav-item', { 'active': route.path === item.path, 'justify-center': collapsed }]"
            @mouseenter="showTooltip($event, item.label)"
            @mouseleave="hideTooltip"
          >
            <component :is="item.icon" class="w-[18px] h-[18px] flex-shrink-0" :class="{ 'mr-3': !collapsed }" />
            <span v-if="!collapsed">{{ item.label }}</span>
          </a>
        </div>

        <!-- Zone Menu Section -->
        <div v-if="zones.length > 0" class="relative">
          <!-- Domain Selector Dropdown -->
          <div v-if="!collapsed" class="section-title flex items-center justify-between">
            <span class="truncate flex-1 max-w-[150px]" :title="currentZone?.name">{{ currentZone?.name || '选择域名' }}</span>
            <!-- Multi-zone dropdown -->
            <div v-if="zones.length > 1" class="relative">
              <button
                ref="dropdownButton"
                @click="toggleZoneDropdown"
                class="text-primary hover:text-primary/80 transition-colors p-1 rounded hover:bg-muted"
              >
                <component :is="ChevronDownOutline" class="w-4 h-4" />
              </button>
            </div>
          </div>

          <!-- Zone menu items -->
          <a
            v-for="item in zoneMenuItems"
            :key="item.path"
            @click.prevent="router.push(item.path)"
            :class="['nav-item', { 'active': route.path === item.path, 'justify-center': collapsed }]"
            @mouseenter="showTooltip($event, item.label)"
            @mouseleave="hideTooltip"
          >
            <component :is="item.icon" class="w-[18px] h-[18px] flex-shrink-0" :class="{ 'mr-3': !collapsed }" />
            <span v-if="!collapsed" class="flex-1">{{ item.label }}</span>
            <span v-if="!collapsed && item.pro" class="glass-badge glass-badge-warning text-[10px]">Pro+</span>
          </a>
        </div>
      </nav>

      <!-- Collapse Button -->
      <div class="mt-auto pt-3 px-3">
        <button @click="collapsed = !collapsed" class="w-full text-sm text-muted-foreground hover:text-foreground flex items-center justify-center">
          <component :is="collapsed ? ChevronForwardOutline : ChevronBackOutline" class="w-4 h-4" />
          <span v-if="!collapsed" class="ml-2">收起侧栏</span>
        </button>
      </div>
    </aside>

    <!-- Floating Zone Dropdown (Teleport to body) -->
    <Teleport to="body">
      <div v-if="showZoneDropdown" class="fixed inset-0 z-[9998]" @click="showZoneDropdown = false"></div>
      <div 
        v-if="showZoneDropdown"
        :style="{ left: dropdownPosition.x + 'px', top: dropdownPosition.y + 'px' }"
        class="fixed z-[9999] glass-modal min-w-[280px] max-h-[420px] overflow-hidden"
        @click.stop
      >
        <div class="py-2 overflow-y-auto max-h-[420px]">
          <div class="px-4 py-2 text-xs font-semibold text-muted-foreground border-b border-border">
            选择域名 ({{ zones.length }})
          </div>
          <button
            v-for="zone in zones"
            :key="zone.id"
            @click="selectZone(zone.id)"
            :class="[
              'w-full text-left px-4 py-3 text-sm hover:bg-muted transition-all flex items-center justify-between',
              currentZone?.id === zone.id ? 'bg-accent/50 text-accent-foreground font-medium' : 'text-foreground'
            ]"
          >
            <div class="flex-1 min-w-0">
              <div class="truncate font-medium">{{ zone.name }}</div>
              <div class="text-xs text-muted-foreground truncate mt-0.5">{{ zone.status }}</div>
            </div>
            <svg v-if="currentZone?.id === zone.id" class="w-5 h-5 ml-3 flex-shrink-0 text-primary" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
            </svg>
          </button>
        </div>
      </div>
    </Teleport>

    <!-- Sidebar Tooltip -->
    <Teleport to="body">
      <div
        v-if="tooltip.visible"
        class="nav-tooltip"
        :style="{ left: tooltip.x + 'px', top: tooltip.y + 'px' }"
      >
        {{ tooltip.text }}
      </div>
    </Teleport>

    <!-- Main Content Area -->
    <main class="flex-1 flex flex-col gap-4 min-w-0">
      <!-- Floating Topbar -->
      <div class="glass-topbar flex justify-between items-center">
        <div class="text-sm text-muted-foreground">你的工作 / {{ currentTitle }}</div>
        <div class="flex items-center gap-3">
          <!-- Theme Toggle -->
          <button
            @click="toggleTheme"
            class="w-9 h-9 rounded-lg hover:bg-white/30 flex items-center justify-center transition-colors"
          >
            <component :is="themeStore.isDark ? MoonOutline : SunnyOutline" class="w-[18px] h-[18px]" />
          </button>

          <!-- User Avatar -->
          <div v-if="accountStore.currentAccount" class="w-9 h-9 rounded-full bg-primary/10 text-primary flex items-center justify-center text-sm font-semibold">
            {{ (accountStore.currentAccount.alias || 'U')[0].toUpperCase() }}
          </div>

          <!-- Add Account -->
          <button
            v-else
            class="btn-island-primary"
            @click="showAccountModal = true"
          >
            添加账户
          </button>
        </div>
      </div>

      <!-- The Island Container (content only) -->
      <div class="island-container flex-1 overflow-y-auto" style="padding: 32px 40px;">
        <router-view />
      </div>
    </main>

    <!-- Add Account Modal -->
    <div v-if="showAccountModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="showAccountModal = false">
      <div class="glass-modal w-full max-w-xl" @click.stop style="max-height: 90vh; overflow-y: auto;">
        <div class="p-6 border-b border-border">
          <h2 class="text-xl font-semibold">添加 Cloudflare 账户</h2>
        </div>
        
        <div class="p-6 space-y-4">
          <div class="alert-warning">
            <strong>安全提示：</strong>请使用 API Token 而不是 Global API Key
          </div>

          <div>
            <label class="block text-sm font-medium mb-2">API Token *</label>
            <input
              v-model="accountForm.apiToken"
              type="password"
              class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
              placeholder="输入您的 Cloudflare API Token"
            />
          </div>

          <div>
            <label class="block text-sm font-medium mb-2">别名（可选）</label>
            <input
              v-model="accountForm.alias"
              class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
              placeholder="为账户设置一个别名"
            />
          </div>
        </div>

        <div class="p-6 border-t border-border flex justify-end gap-3">
          <button class="btn-island-secondary" @click="showAccountModal = false">取消</button>
          <button class="btn-island-primary" @click="handleAddAccount">添加账户</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, provide, type Component } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAccountStore } from '@/stores/account'
import { useThemeStore } from '@/stores/theme'
import { cloudflareApi, type Zone } from '@/api'
import {
  HomeOutline,
  GlobeOutline,
  PersonOutline,
  SettingsOutline,
  KeyOutline,
  ServerOutline,
  DocumentTextOutline,
  RocketOutline,
  FlashOutline,
  TimeOutline,
  BuildOutline,
  LockClosedOutline,
  ShieldOutline,
  FlameOutline,
  TimerOutline,
  AnalyticsOutline,
  DocumentOutline,
  RibbonOutline,
  CloudOutline,
  MoonOutline,
  SunnyOutline,
  ChevronBackOutline,
  ChevronForwardOutline,
  ChevronDownOutline,
} from '@vicons/ionicons5'

const router = useRouter()
const route = useRoute()
const accountStore = useAccountStore()
const themeStore = useThemeStore()

const collapsed = ref(false)
const showAccountModal = ref(false)
const showZoneDropdown = ref(false)
const dropdownButton = ref<HTMLElement | null>(null)
const dropdownPosition = ref({ x: 0, y: 0 })
const zones = ref<Zone[]>([])
const currentZone = ref<Zone | null>(null)

// Tooltip state
const tooltip = ref({ visible: false, text: '', x: 0, y: 0 })

function showTooltip(event: MouseEvent, text: string) {
  if (!collapsed.value) return
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
  tooltip.value = {
    visible: true,
    text,
    x: rect.right + 12,
    y: rect.top + rect.height / 2,
  }
}

function hideTooltip() {
  tooltip.value.visible = false
}

const accountForm = ref({
  apiToken: '',
  alias: ''
})

const mainMenuItems: { label: string; path: string; icon: Component }[] = [
  { label: 'Home', path: '/dashboard', icon: HomeOutline },
  { label: '域名管理', path: '/zones', icon: GlobeOutline },
  { label: '账户管理', path: '/accounts', icon: PersonOutline },
  { label: 'Workers', path: '/workers', icon: SettingsOutline },
  { label: 'Workers KV', path: '/workers-kv', icon: KeyOutline },
  { label: 'D1 数据库', path: '/d1', icon: ServerOutline },
  { label: '模板库', path: '/worker-templates', icon: DocumentTextOutline },
  { label: '一键优选', path: '/quick-deploy', icon: RocketOutline },
  { label: '自动优化', path: '/optimize', icon: FlashOutline },
  { label: '操作历史', path: '/history', icon: TimeOutline },
]

const zoneMenuItems = computed(() => {
  if (!currentZone.value) return []
  return [
    { label: 'DNS 记录', path: '/dns', icon: BuildOutline },
    { label: 'SSL/TLS', path: '/ssl', icon: LockClosedOutline },
    { label: '缓存', path: '/cache', icon: FlashOutline },
    { label: '防火墙', path: '/firewall', icon: ShieldOutline },
    { label: 'WAF', path: '/waf', icon: FlameOutline, pro: true },
    { label: '速率限制', path: '/rate-limits', icon: TimerOutline, pro: true },
    { label: '分析', path: '/analytics', icon: AnalyticsOutline },
    { label: '页面规则', path: '/page-rules', icon: DocumentOutline },
    { label: '证书', path: '/certificates', icon: RibbonOutline, pro: true },
  ]
})

const currentTitle = computed(() => route.meta.title as string || 'Home')

function toggleZoneDropdown() {
  if (!showZoneDropdown.value && dropdownButton.value) {
    const rect = dropdownButton.value.getBoundingClientRect()
    const viewportHeight = window.innerHeight
    const dropdownHeight = 420 // max-height of dropdown
    
    // Calculate available space below and above the button
    const spaceBelow = viewportHeight - rect.bottom
    const spaceAbove = rect.top
    
    // Decide whether to open upward or downward
    const openUpward = spaceBelow < dropdownHeight && spaceAbove > spaceBelow
    
    if (openUpward) {
      // Position above the button
      dropdownPosition.value = {
        x: rect.left,
        y: rect.top - Math.min(dropdownHeight, spaceAbove - 8) // 8px gap
      }
    } else {
      // Position below the button
      dropdownPosition.value = {
        x: rect.left,
        y: rect.bottom + 8 // 8px below button
      }
    }
  }
  showZoneDropdown.value = !showZoneDropdown.value
}

function selectZone(zoneId: string) {
  const zone = zones.value.find(z => z.id === zoneId)
  if (zone) {
    currentZone.value = zone
    localStorage.setItem('currentZoneId', zone.id)
    showZoneDropdown.value = false
    console.log('Switched to zone:', zone.name)
  }
}

// Close dropdown when clicking outside
function handleClickOutside(event: MouseEvent) {
  if (showZoneDropdown.value && !dropdownButton.value?.contains(event.target as Node)) {
    showZoneDropdown.value = false
  }
}

async function loadZones() {
  if (!accountStore.currentAccount) return

  try {
    zones.value = await cloudflareApi.getZones()
    if (zones.value.length > 0) {
      const savedZoneId = localStorage.getItem('currentZoneId')
      const savedZone = savedZoneId ? zones.value.find(z => z.id === savedZoneId) : null
      currentZone.value = savedZone || zones.value[0]
      if (currentZone.value) {
        localStorage.setItem('currentZoneId', currentZone.value.id)
      }
    }
  } catch (error) {
    console.error('Failed to load zones:', error)
  }
}

function toggleTheme() {
  themeStore.setTheme(themeStore.isDark ? 'light' : 'dark')
}

async function handleAddAccount() {
  if (!accountForm.value.apiToken.trim()) return

  const account = await accountStore.addAccount({
    apiToken: accountForm.value.apiToken,
    alias: accountForm.value.alias || 'Cloudflare 账户'
  })

  if (account) {
    accountStore.switchAccount(account.id)
    showAccountModal.value = false
    accountForm.value = { apiToken: '', alias: '' }
    loadZones()
  }
}

onMounted(() => {
  if (accountStore.accounts.length === 0) {
    showAccountModal.value = true
  } else {
    loadZones()
  }
  
  // Add click outside listener
  document.addEventListener('click', handleClickOutside)
})

provide('currentZone', currentZone)

watch(() => accountStore.currentAccount, () => {
  loadZones()
})
</script>

<style scoped>
/* Section Title */
.section-title {
  font-size: 12px;
  color: #888;
  margin: 20px 12px 10px;
  font-weight: 600;
}

/* Navigation items are styled via island-theme.css .nav-item class */

/* Custom scrollbar for sidebar navigation - matches Island Theme background */
nav::-webkit-scrollbar {
  width: 6px;
}

nav::-webkit-scrollbar-track {
  background: transparent; /* Transparent to match sidebar background */
}

nav::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.1); /* Very subtle gray */
  border-radius: 3px;
}

nav::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.2); /* Slightly darker on hover */
}

/* Firefox scrollbar */
nav {
  scrollbar-width: thin;
  scrollbar-color: rgba(0, 0, 0, 0.1) transparent;
}
</style>

