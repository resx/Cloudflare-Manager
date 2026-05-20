<template>
  <div class="flex h-screen p-4 gap-4 overflow-hidden bg-background">
    <!-- Sidebar -->
    <aside
      :class="['glass-sidebar transition-all duration-500 ease-in-out flex flex-col z-20', collapsed ? 'w-20' : 'w-64']"
      style="padding: 24px 12px;"
    >
      <!-- Logo Area -->
      <div :class="['pb-8 flex items-center min-h-[40px]', collapsed ? 'justify-center' : 'px-2']">
        <div v-if="!collapsed" class="flex items-center gap-3 overflow-hidden animate-in">
          <div class="w-10 h-10 rounded-2xl bg-primary/10 flex items-center justify-center text-primary shrink-0 shadow-inner">
            <component :is="CloudOutline" class="w-6 h-6" />
          </div>
          <div class="flex flex-col">
            <span class="font-black text-lg tracking-tighter text-foreground leading-none">CF ISLAND</span>
            <span class="text-[10px] font-bold text-muted-foreground/60 tracking-widest uppercase">Management</span>
          </div>
        </div>
        <div v-else class="flex justify-center animate-in">
          <div class="w-12 h-12 rounded-2xl bg-primary/10 flex items-center justify-center text-primary shadow-lg border border-primary/20 cursor-pointer hover:bg-primary/20 transition-all group" @click="collapsed = false" :title="t('layout.expandSidebar')">
            <component :is="CloudOutline" class="w-7 h-7 group-hover:scale-110 transition-transform" />
          </div>
        </div>
      </div>

      <!-- Navigation -->
      <nav class="flex-1 overflow-y-auto px-1 custom-scrollbar space-y-8">
        <!-- Main Menu Section -->
        <section class="space-y-1">
          <div v-if="!collapsed" class="px-3 mb-2 text-[10px] font-black uppercase tracking-[0.2em] text-muted-foreground/50">{{ t('layout.console') }}</div>
          <a
            v-for="item in mainMenuItems"
            :key="item.path"
            @click.prevent="router.push(item.path)"
            :class="['nav-item group relative', { 'active': route.path === item.path, 'justify-center !px-0': collapsed }]"
            @mouseenter="showTooltip($event, item.label)"
            @mouseleave="hideTooltip"
          >
            <component :is="item.icon" class="w-5 h-5 flex-shrink-0 transition-transform group-hover:scale-110" :class="{ 'mr-3': !collapsed }" />
            <span v-if="!collapsed" class="truncate font-bold tracking-tight text-sm">{{ item.label }}</span>
            <div v-if="collapsed && route.path === item.path" class="absolute -right-1 w-1.5 h-1.5 bg-primary rounded-full shadow-[0_0_8px_rgba(var(--primary-rgb),0.8)]"></div>
          </a>
        </section>

        <!-- Zone Menu Section -->
        <section v-if="zones.length > 0" class="space-y-1">
          <div v-if="!collapsed" class="px-3 mb-2 flex items-center justify-between group/title">
            <span class="text-[10px] font-black uppercase tracking-[0.2em] text-muted-foreground/50">{{ t('layout.zoneSettings') }}</span>
            <button
              v-if="zones.length > 1"
              ref="dropdownButton"
              @click="toggleZoneDropdown"
              class="text-primary/40 hover:text-primary transition-all p-1"
            >
              <component :is="ChevronDownOutline" class="w-3 h-3" />
            </button>
          </div>

          <!-- Selected Zone Card (In Sidebar) -->
          <div v-if="!collapsed" class="px-2 mb-4">
            <div class="p-3 rounded-2xl bg-foreground/5 border border-border/40 flex items-center gap-3">
              <div class="w-8 h-8 rounded-xl bg-emerald-500/10 flex items-center justify-center text-emerald-500 shrink-0">
                <component :is="GlobeOutline" class="w-4 h-4" />
              </div>
              <div class="min-w-0">
                <p class="text-xs font-black truncate text-foreground">{{ currentZone?.name || t('zones.notSelected') }}</p>
                <p class="text-[10px] font-bold text-muted-foreground uppercase tracking-tight">Active Zone</p>
              </div>
            </div>
          </div>

          <a
            v-for="item in zoneMenuItems"
            :key="item.path"
            @click.prevent="router.push(item.path)"
            :class="['nav-item group relative', { 'active': route.path === item.path, 'justify-center !px-0': collapsed }]"
            @mouseenter="showTooltip($event, item.label)"
            @mouseleave="hideTooltip"
          >
            <component :is="item.icon" class="w-5 h-5 flex-shrink-0 transition-transform group-hover:scale-110" :class="{ 'mr-3': !collapsed }" />
            <span v-if="!collapsed" class="truncate font-bold tracking-tight text-sm">{{ item.label }}</span>
            <div v-if="!collapsed && item.pro" class="ml-auto">
              <div class="px-1.5 py-0.5 bg-amber-500/10 text-amber-600 rounded-md text-[8px] font-black tracking-widest">PRO</div>
            </div>
          </a>
        </section>
      </nav>

      <!-- Bottom Actions -->
      <div class="pt-4 border-t border-border/30 px-2 space-y-2">
        <button 
          @click="toggleTheme" 
          class="w-full h-10 rounded-xl hover:bg-foreground/5 text-muted-foreground flex items-center gap-3 transition-all px-3"
          :class="{ 'justify-center !px-0': collapsed }"
          :title="t('layout.toggleTheme')"
        >
          <component :is="themeStore.isDark ? MoonOutline : SunnyOutline" class="w-5 h-5" />
          <span v-if="!collapsed" class="text-xs font-bold">{{ themeStore.isDark ? t('layout.darkMode') : t('layout.lightMode') }}</span>
        </button>

        <button 
          @click="collapsed = !collapsed" 
          class="w-full h-10 rounded-xl bg-primary/5 hover:bg-primary/10 text-primary flex items-center gap-3 transition-all px-3 shadow-inner border border-primary/5"
          :class="{ 'justify-center !px-0': collapsed }"
          :title="collapsed ? t('layout.expandSidebar') : t('layout.collapseSidebar')"
        >
          <component :is="collapsed ? ChevronForwardOutline : ChevronBackOutline" class="w-5 h-5" />
          <span v-if="!collapsed" class="text-xs font-black uppercase tracking-widest">{{ t('layout.collapseMenu') }}</span>
        </button>
      </div>
    </aside>

    <!-- Zone Dropdown Overlay -->
    <Teleport to="body">
      <div v-if="showZoneDropdown" class="fixed inset-0 z-[9998] bg-background/20 backdrop-blur-sm" @click="showZoneDropdown = false"></div>
      <div 
        v-if="showZoneDropdown"
        :style="{ left: dropdownPosition.x + 'px', top: dropdownPosition.y + 'px' }"
        class="fixed z-[9999] glass-modal min-w-[320px] max-h-[420px] overflow-hidden animate-in shadow-2xl border border-primary/20"
        @click.stop
      >
        <div class="py-2 overflow-y-auto max-h-[420px] custom-scrollbar">
          <div class="px-6 py-4 border-b border-border/50 bg-foreground/[0.02]">
            <h4 class="text-sm font-black text-foreground uppercase tracking-widest">{{ t('layout.selectZone') }}</h4>
            <p class="text-[10px] text-muted-foreground font-bold mt-0.5">{{ t('layout.zoneCount', { count: zones.length }) }}</p>
          </div>
          <button
            v-for="zone in zones"
            :key="zone.id"
            @click="selectZone(zone.id)"
            :class="[
              'w-full text-left px-6 py-4 text-sm hover:bg-primary/5 transition-all flex items-center justify-between group',
              currentZone?.id === zone.id ? 'bg-primary/10 text-primary font-bold' : 'text-foreground'
            ]"
          >
            <div class="flex-1 min-w-0">
              <div class="truncate font-bold flex items-center gap-2">
                {{ zone.name }}
                <div v-if="zone.status === 'active'" class="w-1.5 h-1.5 rounded-full bg-emerald-500 shadow-[0_0_6px_rgba(16,185,129,0.5)]"></div>
              </div>
              <div class="text-[10px] text-muted-foreground font-bold mt-1 uppercase tracking-tight">{{ zone.status }}</div>
            </div>
            <component :is="CheckmarkOutline" v-if="currentZone?.id === zone.id" class="w-5 h-5 ml-4 flex-shrink-0 text-primary" />
          </button>
        </div>
      </div>
    </Teleport>

    <!-- Sidebar Tooltip -->
    <Teleport to="body">
      <div 
        v-if="tooltip.visible" 
        class="nav-tooltip flex items-center gap-2" 
        :class="{ 'sidebar-tip': tooltip.isSidebar, 'topbar-tip': !tooltip.isSidebar }"
        :style="{ left: tooltip.x + 'px', top: tooltip.y + 'px' }"
      >
        <div class="w-1 h-1 rounded-full bg-primary"></div>
        {{ tooltip.text }}
      </div>
    </Teleport>

    <!-- Main Content Area -->
    <main class="flex-1 flex flex-col gap-4 min-w-0 overflow-hidden">
      <!-- Topbar -->
      <header class="glass-topbar flex justify-between items-center h-20 shrink-0 px-8">
        <div class="flex items-center gap-8">
          <div class="flex flex-col">
            <div class="flex items-center gap-2 text-[10px] font-black uppercase tracking-[0.2em] text-muted-foreground/50">
              <span>{{ t('layout.controlPanel') }}</span>
              <span class="opacity-30">/</span>
              <span class="text-primary">{{ currentTitle }}</span>
            </div>
            <h2 class="text-lg font-bold text-foreground tracking-tight">{{ currentTitle }}</h2>
          </div>

          <!-- New Search Entry Point -->
          <div 
            @click="triggerSearch"
            class="hidden lg:flex items-center gap-3 px-4 py-2 bg-foreground/5 border border-border/40 rounded-2xl cursor-pointer hover:bg-foreground/10 hover:border-primary/30 transition-all group w-64"
          >
            <component :is="SearchOutline" class="w-4 h-4 text-muted-foreground group-hover:text-primary transition-colors" />
            <span class="text-xs font-bold text-muted-foreground/60 flex-1">{{ t('common.search') }}</span>
            <kbd class="px-1.5 py-0.5 bg-foreground/10 border border-border/50 rounded-md text-[9px] font-black text-muted-foreground group-hover:text-primary transition-colors">⌘K</kbd>
          </div>
        </div>

        <div class="flex items-center gap-4">
          <!-- Language Toggle -->
          <button 
            @click="toggleLanguage" 
            class="w-12 h-12 rounded-2xl flex items-center justify-center transition-all border border-border/50 bg-foreground/5 text-muted-foreground hover:bg-foreground/10 hover:text-foreground group"
            @mouseenter="showTooltip($event, t('layout.switchLang'), true)"
            @mouseleave="hideTooltip"
          >
            <component :is="LanguageOutline" class="w-5 h-5" />
          </button>

          <!-- Demo Mode Active Badge -->
          <transition name="page-fade">
            <div v-if="themeStore.isDemoMode" class="hidden xl:flex items-center gap-2 px-3 py-1 bg-amber-500/10 border border-amber-500/20 rounded-full text-amber-600 animate-pulse">
              <div class="w-1.5 h-1.5 rounded-full bg-amber-500"></div>
              <span class="text-[9px] font-black uppercase tracking-widest">{{ t('demoMode.active') }}</span>
            </div>
          </transition>

          <div v-if="accountStore.currentAccount" class="hidden md:flex flex-col items-end mr-2">
            <span class="text-xs font-black text-foreground">{{ accountStore.currentAccount.alias }}</span>
            <span :class="['text-[10px] font-bold text-muted-foreground uppercase tracking-tight italic opacity-60 transition-all', themeStore.isDemoMode ? 'demo-mask' : '']">
              {{ accountStore.currentAccount.accountId }}
            </span>
          </div>

          <!-- Demo Mode Toggle -->
          <button 
            @click="toggleDemoMode" 
            class="w-12 h-12 rounded-2xl flex items-center justify-center transition-all border border-border/50 group relative"
            :class="themeStore.isDemoMode ? 'bg-amber-500 text-white border-amber-400 shadow-[0_0_20px_rgba(245,158,11,0.4)]' : 'bg-foreground/5 text-muted-foreground hover:bg-foreground/10 hover:text-foreground'"
            @mouseenter="showTooltip($event, themeStore.isDemoMode ? t('demoMode.exit') : t('demoMode.enter'), true)"
            @mouseleave="hideTooltip"
          >
            <component :is="themeStore.isDemoMode ? EyeOffOutline : EyeOutline" class="w-5 h-5" />
          </button>

          <n-dropdown 
            v-if="accountStore.currentAccount" 
            :options="accountOptions" 
            @select="handleAccountMenuSelect"
            trigger="click"
            placement="bottom-end"
          >
            <div class="w-12 h-12 rounded-2xl bg-foreground/5 text-foreground flex items-center justify-center text-lg font-black border border-border/50 cursor-pointer hover:bg-primary/10 hover:text-primary hover:border-primary/30 transition-all active:scale-95 group relative">
              {{ (accountStore.currentAccount.alias || 'U')[0].toUpperCase() }}
              <div class="absolute -right-1 -bottom-1 w-4 h-4 bg-emerald-500 border-2 border-background rounded-full"></div>
            </div>
          </n-dropdown>

          <IslandButton
            v-else
            size="small"
            @click="showAccountModal = true"
          >
            {{ t('accounts.addAccount') }}
          </IslandButton>
        </div>
      </header>

      <!-- Main Container -->
      <div class="flex-1 overflow-hidden">
        <GlassCard class="h-full flex flex-col border-border/40 !bg-card !backdrop-blur-[20px]" :padding="0">
          <div class="flex-1 overflow-y-auto p-8 custom-scrollbar">
            <router-view v-slot="{ Component }">
              <transition name="page-fade" mode="out-in">
                <component :is="Component" />
              </transition>
            </router-view>
          </div>
        </GlassCard>
      </div>
    </main>

    <CommandPalette />
    <SmartIsland />

    <!-- Add Account Modal -->
    <Teleport to="body">
      <div v-if="showAccountModal" class="fixed inset-0 bg-background/60 backdrop-blur-xl flex items-center justify-center z-[10000] p-4" @click.self="showAccountModal = false">
        <GlassCard class="w-full max-w-lg shadow-2xl animate-in border-primary/20" :padding="0">
          <div class="p-8 border-b border-border/50 flex justify-between items-center bg-foreground/[0.02]">
            <div>
              <h2 class="text-2xl font-black tracking-tighter">{{ t('layout.connectCloudflare') }}</h2>
              <p class="text-xs text-muted-foreground font-bold uppercase tracking-widest mt-1">{{ t('layout.addCredentials') }}</p>
            </div>
            <button @click="showAccountModal = false" class="w-10 h-10 rounded-2xl bg-foreground/5 flex items-center justify-center text-muted-foreground hover:text-foreground transition-all">
              <component :is="ChevronDownOutline" class="w-5 h-5 rotate-45" />
            </button>
          </div>
          
          <div class="p-10 space-y-8">
            <div class="p-5 bg-primary/5 border border-primary/10 rounded-2xl flex gap-4 items-start">
              <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary shrink-0">
                <component :is="ShieldOutline" class="w-5 h-5" />
              </div>
              <div class="space-y-1">
                <p class="text-sm font-bold text-foreground">{{ t('accounts.privacyTitle') }}</p>
                <p class="text-xs text-muted-foreground leading-relaxed">
                  {{ t('layout.privacyExplain') }}
                </p>
              </div>
            </div>

            <div class="space-y-6">
              <div class="space-y-3">
                <div class="flex justify-between items-center px-1">
                  <label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/60">Cloudflare API Token</label>
                  <a href="https://dash.cloudflare.com/profile/api-tokens" target="_blank" class="text-[10px] text-primary font-black uppercase tracking-widest hover:underline flex items-center gap-1">
                    {{ t('layout.getToken') }}
                    <component :is="RocketOutline" class="w-3 h-3" />
                  </a>
                </div>
                <input
                  v-model="accountForm.apiToken"
                  type="password"
                  class="w-full bg-foreground/5 border border-border/50 rounded-2xl px-5 py-4 text-sm focus:ring-2 focus:ring-primary/30 outline-none transition-all font-mono"
                  :placeholder="t('layout.tokenPlaceholder')"
                />
              </div>

              <!-- Permission Guide -->
              <div class="p-6 bg-foreground/[0.03] border border-border/40 rounded-[24px] space-y-4">
                <p class="text-[10px] font-black text-muted-foreground uppercase tracking-[0.2em] flex items-center gap-2">
                  <component :is="CheckmarkOutline" class="w-3 h-3 text-emerald-500" />
                  {{ t('layout.recommendedPermissions') }}
                </p>
                <div class="grid grid-cols-2 gap-y-3 gap-x-6">
                  <div class="flex items-center gap-2 text-[11px] font-bold text-foreground/80">
                    <div class="w-1 h-1 rounded-full bg-primary/40"></div>
                    Account: Settings (R)
                  </div>
                  <div class="flex items-center gap-2 text-[11px] font-bold text-foreground/80">
                    <div class="w-1 h-1 rounded-full bg-primary/40"></div>
                    Zone: Zone (R/W)
                  </div>
                  <div class="flex items-center gap-2 text-[11px] font-bold text-foreground/80">
                    <div class="w-1 h-1 rounded-full bg-primary/40"></div>
                    Zone: DNS (R/W)
                  </div>
                  <div class="flex items-center gap-2 text-[11px] font-bold text-foreground/80">
                    <div class="w-1 h-1 rounded-full bg-primary/40"></div>
                    Zone: Analytics (R)
                  </div>
                  <div class="flex items-center gap-2 text-[11px] font-bold text-foreground/80">
                    <div class="w-1 h-1 rounded-full bg-primary/40"></div>
                    Worker: Scripts (R/W)
                  </div>
                  <div class="flex items-center gap-2 text-[11px] font-bold text-foreground/80">
                    <div class="w-1 h-1 rounded-full bg-primary/40"></div>
                    User: Details (R)
                  </div>
                </div>
              </div>

              <div class="space-y-2">
                <label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/60 ml-1">{{ t('accounts.alias') }}</label>
                <input
                  v-model="accountForm.alias"
                  class="w-full bg-foreground/5 border border-border/50 rounded-2xl px-5 py-4 text-sm focus:ring-2 focus:ring-primary/30 outline-none transition-all font-bold"
                  :placeholder="t('accounts.aliasPlaceholder')"
                />
              </div>
            </div>
          </div>

          <div class="p-8 border-t border-border/50 flex justify-end gap-4 bg-foreground/[0.01]">
            <IslandButton variant="secondary" @click="showAccountModal = false" class="px-8">{{ t('common.cancel') }}</IslandButton>
            <IslandButton @click="handleAddAccount" class="px-10 shadow-lg shadow-primary/20">{{ t('layout.verifyAndAdd') }}</IslandButton>
          </div>
        </GlassCard>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, provide, h, type Component } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAccountStore } from '@/stores/account'
import { useThemeStore } from '@/stores/theme'
import { cloudflareApi, type Zone } from '@/api'
import { toast } from '@/utils/toast'
import GlassCard from '@/components/ui/GlassCard.vue'
import IslandButton from '@/components/ui/IslandButton.vue'
import GlassBadge from '@/components/ui/GlassBadge.vue'
import CommandPalette from '@/components/ui/CommandPalette.vue'
import SmartIsland from '@/components/ui/SmartIsland.vue'
import { NDropdown } from 'naive-ui'
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
  SwapHorizontalOutline,
  CheckmarkOutline,
  EyeOutline,
  EyeOffOutline,
  SearchOutline,
  LanguageOutline
} from '@vicons/ionicons5'

const router = useRouter()
const route = useRoute()
const { t, locale } = useI18n()
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
const tooltip = ref({ visible: false, text: '', x: 0, y: 0, isSidebar: true })

function showTooltip(event: MouseEvent, text: string, force = false) {
  if (!force && !collapsed.value) return
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
  
  // Decide position: right for sidebar, bottom for topbar
  const isSidebar = (event.currentTarget as HTMLElement).closest('.glass-sidebar')
  
  if (isSidebar) {
    tooltip.value = {
      visible: true,
      text,
      x: rect.right + 12,
      y: rect.top + rect.height / 2,
      isSidebar: true
    }
  } else {
    // Default to bottom-center for topbar or other elements
    tooltip.value = {
      visible: true,
      text,
      x: rect.left + rect.width / 2,
      y: rect.bottom + 12,
      isSidebar: false
    }
  }
}

function hideTooltip() {
  tooltip.value.visible = false
}

const toggleLanguage = () => {
  const newLocale = locale.value === 'zh-CN' ? 'en-US' : 'zh-CN'
  locale.value = newLocale
  localStorage.setItem('cf_language', newLocale)
  toast.success(newLocale === 'zh-CN' ? '已切换至中文' : 'Switched to English')
}

// Localized Navigation Items
const mainMenuItems = computed(() => [
  { label: t('common.dashboard'), path: '/dashboard', icon: HomeOutline },
  { label: t('common.zones'), path: '/zones', icon: GlobeOutline },
  { label: t('quickDeploy.title'), path: '/quick-deploy', icon: RocketOutline },
  { label: t('common.optimize'), path: '/optimize', icon: FlashOutline },
  { label: t('common.dns'), path: '/dns', icon: BuildOutline },
  { label: t('common.firewall'), path: '/firewall', icon: ShieldOutline },
  { label: t('common.workers'), path: '/workers', icon: SettingsOutline },
  { label: t('common.analytics'), path: '/analytics', icon: AnalyticsOutline },
  { label: t('common.accounts'), path: '/accounts', icon: PersonOutline },
  { label: t('common.history'), path: '/history', icon: TimeOutline },
])

const accountForm = ref({
  apiToken: '',
  alias: ''
})



const zoneMenuItems = computed(() => {
  if (!currentZone.value) return []
  return [
    { label: t('common.dns'), path: '/dns', icon: BuildOutline },
    { label: t('common.ssl'), path: '/ssl', icon: LockClosedOutline },
    { label: t('common.cache'), path: '/cache', icon: FlashOutline },
    { label: t('common.firewall'), path: '/firewall', icon: ShieldOutline },
    { label: t('common.waf'), path: '/waf', icon: FlameOutline, pro: true },
    { label: t('common.analytics'), path: '/analytics', icon: AnalyticsOutline },
    { label: t('common.pageRules'), path: '/page-rules', icon: DocumentOutline },
  ]
})

const currentTitle = computed(() => {
  const titleKey = route.meta.title as string
  if (!titleKey) return 'Overview'
  return t(titleKey)
})

const accountOptions = computed(() => {
  const options: any[] = [
    {
      label: accountStore.currentAccount?.alias || t('layout.currentAccount'),
      key: 'current',
      disabled: true
    },
    { type: 'divider', key: 'd1' },
    {
      label: t('layout.manageAccounts'),
      key: 'manage',
      icon: () => h(PersonOutline, { class: 'w-4 h-4' })
    },
    {
      label: t('layout.quickSwitch'),
      key: 'switch',
      icon: () => h(SwapHorizontalOutline, { class: 'w-4 h-4' }),
      children: accountStore.accounts.map(acc => ({
        label: acc.alias,
        key: `switch-${acc.id}`,
        disabled: acc.id === accountStore.currentAccount?.id
      }))
    }
  ]
  return options
})

function handleAccountMenuSelect(key: string) {
  if (key === 'manage') {
    router.push('/accounts')
  } else if (key.startsWith('switch-')) {
    const id = key.replace('switch-', '')
    accountStore.switchAccount(id)
    toast.success(t('accounts.switched'))
    window.location.reload() 
  }
}

function toggleZoneDropdown() {
  if (!showZoneDropdown.value && dropdownButton.value) {
    const rect = dropdownButton.value.getBoundingClientRect()
    dropdownPosition.value = { x: rect.left, y: rect.bottom + 12 }
  }
  showZoneDropdown.value = !showZoneDropdown.value
}

function selectZone(zoneId: string) {
  const zone = zones.value.find(z => z.id === zoneId)
  if (zone) {
    currentZone.value = zone
    localStorage.setItem('currentZoneId', zone.id)
    showZoneDropdown.value = false
    toast.success(t('layout.zoneSwitched', { name: zone.name }))
  }
}

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
      if (currentZone.value) localStorage.setItem('currentZoneId', currentZone.value.id)
    }
  } catch (e) {
    console.error('Failed to load zones')
  }
}

function toggleTheme() {
  themeStore.setTheme(themeStore.isDark ? 'light' : 'dark')
}

const triggerSearch = () => {
  if ((window as any).commandPalette) {
    (window as any).commandPalette.open()
  }
}

const toggleDemoMode = () => {
  themeStore.toggleDemoMode()
}

async function handleAddAccount() {
  if (!accountForm.value.apiToken.trim()) return
  const account = await accountStore.addAccount({
    apiToken: accountForm.value.apiToken,
    alias: accountForm.value.alias || 'New Account'
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
  document.addEventListener('click', handleClickOutside)
})

provide('currentZone', currentZone)
watch(() => accountStore.currentAccount, () => loadZones())
</script>

<style>
/* Global Glass Scrollbar */
.custom-scrollbar::-webkit-scrollbar {
  width: 5px;
  height: 5px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(var(--primary-rgb), 0.1);
  border-radius: 10px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(var(--primary-rgb), 0.3);
}

/* Base styles for scrollbar */
* {
  scrollbar-width: thin;
  scrollbar-color: rgba(128, 128, 128, 0.2) transparent;
}
</style>

