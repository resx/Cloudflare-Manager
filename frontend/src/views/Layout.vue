<template>
  <n-layout has-sider style="height: 100vh">
    <n-layout-sider
      bordered
      collapse-mode="width"
      :collapsed-width="64"
      :width="260"
      :collapsed="collapsed"
      show-trigger
      @collapse="collapsed = true"
      @expand="collapsed = false"
    >
      <div class="sidebar-content">
      <!-- 顶部 Logo 和用户区域 -->
      <div :class="['header-section', { collapsed }]">
        <div class="logo-area">
          <div class="logo-icon">
            <n-icon size="32" color="#1e90ff">
              <CloudOutline />
            </n-icon>
          </div>
          <div v-if="!collapsed" class="logo-text">
            <div class="site-name">Cloudflare Manager</div>
            <div class="current-user">
              <n-dropdown
                :options="accountOptions"
                @select="handleAccountChange"
                trigger="click"
              >
                <n-button text size="small" style="padding: 0">
                  <template #icon>
                    <n-icon><PersonOutline /></n-icon>
                  </template>
                  {{ accountStore.currentAccount?.alias || '未选择账户' }}
                  <n-icon style="margin-left: 4px"><ChevronDownOutline /></n-icon>
                </n-button>
              </n-dropdown>
            </div>
          </div>
        </div>
      </div>

      <!-- 全局功能菜单 -->
      <div class="menu-section">
        <div v-if="!collapsed" class="section-title">全局功能</div>
        <n-menu
          :collapsed="collapsed"
          :collapsed-width="64"
          :collapsed-icon-size="22"
          :options="globalMenuOptions"
          :value="activeKey"
          @update:value="handleMenuSelect"
        />
      </div>

      <!-- 域名管理区域 -->
      <div v-if="zones.length > 0" class="menu-section">
        <div v-if="!collapsed" class="section-title domain-section">
          <div class="domain-title">
            <span class="domain-name">{{ currentZone?.name || '选择域名' }}</span>
            <n-dropdown
              v-if="zones.length > 1"
              :options="zoneOptions"
              @select="handleZoneChange"
              trigger="click"
            >
              <n-button
                size="tiny"
                text
                class="zone-switch-btn"
              >
                <template #icon>
                  <n-icon><ChevronDownOutline /></n-icon>
                </template>
              </n-button>
            </n-dropdown>
          </div>
        </div>
        <n-menu
          :collapsed="collapsed"
          :collapsed-width="64"
          :collapsed-icon-size="22"
          :options="zoneMenuOptions"
          :value="activeKey"
          @update:value="handleMenuSelect"
        />
      </div>
      </div>
    </n-layout-sider>

    <n-layout>
      <n-layout-header bordered style="height: 64px; padding: 0 24px; display: flex; align-items: center; justify-content: space-between">
        <n-breadcrumb>
          <n-breadcrumb-item>{{ currentTitle }}</n-breadcrumb-item>
        </n-breadcrumb>

        <n-space align="center">
          <n-dropdown :options="themeOptions" @select="handleThemeSelect">
            <n-button circle quaternary>
              <template #icon>
                <n-icon :component="themeStore.isDark ? MoonOutline : SunnyOutline" />
              </template>
            </n-button>
          </n-dropdown>

          <n-button v-if="accountStore.accounts.length === 0" type="primary" @click="showAccountModal = true">
            添加账户
          </n-button>
        </n-space>
      </n-layout-header>

      <n-layout-content content-style="padding: 24px;" :native-scrollbar="false">
        <router-view />
      </n-layout-content>
    </n-layout>

    <!-- 添加账户弹窗 -->
    <n-modal v-model:show="showAccountModal" preset="dialog" title="添加 Cloudflare 账户" style="width: 600px">
      <n-form ref="formRef" :model="accountForm" :rules="formRules" label-placement="left" label-width="100">
        <n-alert type="warning" style="margin-bottom: 16px; font-size: 13px">
          <strong>安全提示：</strong>请使用 API Token 而不是 Global API Key。API Token 可以限制权限范围，更加安全。
        </n-alert>

        <n-form-item label="API Token" path="apiToken">
          <n-input
            v-model:value="accountForm.apiToken"
            type="password"
            show-password-on="click"
            placeholder="输入您的 API Token"
          />
        </n-form-item>

        <n-alert type="info" style="margin-bottom: 16px; font-size: 12px">
          <div><strong>如何创建 API Token：</strong></div>
          <div style="margin-top: 8px">
            1. 访问 <a href="https://dash.cloudflare.com/profile/api-tokens" target="_blank">Cloudflare Dashboard → API Tokens</a><br>
            2. 点击 "Create Token"<br>
            3. 选择 "Create Custom Token"<br>
            4. 添加以下权限：
          </div>
        </n-alert>

        <n-collapse style="margin-bottom: 16px">
          <n-collapse-item title="所需权限列表（点击展开查看）" name="permissions">
            <div style="font-size: 12px; line-height: 1.8">
              <strong>Account 级别权限：</strong><br>
              • Account Settings - Read<br>
              • Account Analytics - Read<br>
              • Workers Scripts - Edit<br>
              <br>
              <strong>Zone 级别权限：</strong><br>
              • Zone - Read<br>
              • Zone Settings - Edit<br>
              • DNS - Edit<br>
              • Analytics - Read<br>
              • SSL and Certificates - Edit<br>
              • Cache Purge - Purge<br>
              • Page Rules - Edit<br>
              • Firewall Services - Edit<br>
              • Workers Routes - Edit<br>
            </div>
          </n-collapse-item>
        </n-collapse>

        <n-form-item label="别名（可选）">
          <n-input v-model:value="accountForm.alias" placeholder="为账户设置一个别名" />
        </n-form-item>
      </n-form>

      <template #action>
        <n-space>
          <n-button @click="showAccountModal = false">取消</n-button>
          <n-button type="primary" @click="handleAddAccount">确认</n-button>
        </n-space>
      </template>
    </n-modal>
  </n-layout>
</template>

<script setup lang="ts">
import { ref, computed, h, onMounted, watch, provide } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useMessage, NIcon, NTag, NSpace } from 'naive-ui'
import type { MenuOption, DropdownOption } from 'naive-ui'
import {
  CloudOutline,
  PersonOutline,
  ChevronDownOutline,
  HomeOutline,
  RocketOutline,
  SpeedometerOutline,
  TimeOutline,
  ServerOutline,
  FolderOutline,
  KeyOutline,
  CodeSlashOutline,
  GitNetworkOutline,
  LockClosedOutline,
  FlashOutline,
  ShieldCheckmarkOutline,
  StatsChartOutline,
  DocumentTextOutline,
  RibbonOutline,
  SunnyOutline,
  MoonOutline,
  ContrastOutline,
  GlobeOutline,
  FileTrayFullOutline,
  CloudUploadOutline
} from '@vicons/ionicons5'
import { useAccountStore } from '@/stores/account'
import { useThemeStore } from '@/stores/theme'
import { cloudflareApi, type Zone } from '@/api'

const router = useRouter()
const route = useRoute()
const message = useMessage()
const accountStore = useAccountStore()
const themeStore = useThemeStore()

const collapsed = ref(false)
const showAccountModal = ref(false)
const zones = ref<Zone[]>([])
const currentZone = ref<Zone | null>(null)

const accountForm = ref({
  apiToken: '',
  alias: ''
})

const formRules = {
  apiToken: { required: true, message: '请输入 API Token', trigger: 'blur' }
}

// 菜单图标渲染
function renderIcon(icon: any) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

// 渲染带标签的菜单项
function renderLabelWithTag(label: string, tagText?: string, tagType?: 'warning' | 'error' | 'info') {
  if (!tagText) return label

  return () => h(
    NSpace,
    { align: 'center', size: 8 },
    {
      default: () => [
        h('span', label),
        h(NTag, { type: tagType || 'warning', size: 'small' }, { default: () => tagText })
      ]
    }
  )
}

// 全局功能菜单
const globalMenuOptions: MenuOption[] = [
  {
    label: '域名管理',
    key: '/zones',
    icon: renderIcon(GlobeOutline)
  },
  {
    label: '账户管理',
    key: '/accounts',
    icon: renderIcon(PersonOutline)
  },
  {
    label: '一键加速',
    key: '/quick-deploy',
    icon: renderIcon(RocketOutline)
  },
  {
    label: 'Workers 管理',
    key: '/workers',
    icon: renderIcon(CodeSlashOutline)
  },
  {
    label: '自动优化',
    key: '/optimize',
    icon: renderIcon(SpeedometerOutline)
  },
  {
    label: '操作历史',
    key: '/history',
    icon: renderIcon(TimeOutline)
  },
  {
    label: 'D1 数据库',
    key: '/d1',
    icon: renderIcon(FileTrayFullOutline),
    disabled: true
  },
  {
    label: 'R2 存储桶',
    key: '/r2',
    icon: renderIcon(CloudUploadOutline),
    disabled: true
  },
  {
    label: 'Workers KV',
    key: '/workers-kv',
    icon: renderIcon(KeyOutline),
    disabled: true
  },
  {
    label: 'Worker 模板库',
    key: '/worker-templates',
    icon: renderIcon(CodeSlashOutline),
    disabled: true
  },
  {
    label: 'Cloudflare Tunnels',
    key: '/tunnels',
    icon: renderIcon(GitNetworkOutline),
    disabled: true
  }
]

// 域名管理菜单（针对当前选中的域名）
const zoneMenuOptions = computed<MenuOption[]>(() => {
  if (!currentZone.value) return []

  return [
    {
      label: 'DNS 记录',
      key: '/dns',
      icon: renderIcon(ServerOutline)
    },
    {
      label: 'SSL/TLS',
      key: '/ssl',
      icon: renderIcon(LockClosedOutline)
    },
    {
      label: '缓存管理',
      key: '/cache',
      icon: renderIcon(FlashOutline)
    },
    {
      label: '防火墙',
      key: '/firewall',
      icon: renderIcon(ShieldCheckmarkOutline)
    },
    {
      label: renderLabelWithTag('WAF 规则', 'Pro+', 'warning'),
      key: '/waf',
      icon: renderIcon(ShieldCheckmarkOutline)
    },
    {
      label: renderLabelWithTag('速率限制', 'Pro+', 'warning'),
      key: '/rate-limits',
      icon: renderIcon(SpeedometerOutline)
    },
    {
      label: '统计分析',
      key: '/analytics',
      icon: renderIcon(StatsChartOutline)
    },
    {
      label: renderLabelWithTag('页面规则', '用户Token', 'info'),
      key: '/page-rules',
      icon: renderIcon(DocumentTextOutline)
    },
    {
      label: renderLabelWithTag('证书管理', 'Business+', 'warning'),
      key: '/certificates',
      icon: renderIcon(RibbonOutline)
    }
  ]
})

const activeKey = computed(() => route.path)
const currentTitle = computed(() => route.meta.title as string || '控制台')

const accountOptions = computed<DropdownOption[]>(() => [
  ...accountStore.accounts.map(acc => ({
    label: acc.alias,
    key: acc.id,
    icon: renderIcon(PersonOutline)
  })),
  {
    type: 'divider' as const,
    key: 'divider'
  },
  {
    label: '添加账户',
    key: 'add-account',
    icon: () => h(NIcon, null, { default: () => h('span', '+') })
  }
])

const zoneOptions = computed<DropdownOption[]>(() =>
  zones.value.map(zone => ({
    label: zone.name,
    key: zone.id,
    icon: renderIcon(GlobeOutline)
  }))
)

const themeOptions: DropdownOption[] = [
  {
    label: '亮色主题',
    key: 'light',
    icon: renderIcon(SunnyOutline)
  },
  {
    label: '暗色主题',
    key: 'dark',
    icon: renderIcon(MoonOutline)
  },
  {
    label: '跟随系统',
    key: 'auto',
    icon: renderIcon(ContrastOutline)
  }
]

// 加载域名列表
async function loadZones() {
  if (!accountStore.currentAccount) return

  try {
    zones.value = await cloudflareApi.getZones()
    if (zones.value.length > 0) {
      // 优先使用 localStorage 中保存的域名
      const savedZoneId = localStorage.getItem('currentZoneId')
      const savedZone = savedZoneId ? zones.value.find(z => z.id === savedZoneId) : null

      if (savedZone) {
        currentZone.value = savedZone
        console.log('Loaded zones, setting currentZone to saved:', savedZone.name)
      } else {
        // 如果没有保存或找不到，使用第一个域名
        currentZone.value = zones.value[0]
        localStorage.setItem('currentZoneId', currentZone.value.id)
        console.log('Loaded zones, setting currentZone to first:', currentZone.value.name)
      }
    }
  } catch (error) {
    console.error('Failed to load zones:', error)
  }
}

function handleMenuSelect(key: string) {
  router.push(key)
}

function handleAccountChange(key: string) {
  if (key === 'add-account') {
    showAccountModal.value = true
    return
  }

  accountStore.switchAccount(key)
  zones.value = []
  currentZone.value = null
  loadZones()

  // 如果当前在域名相关页面，跳转到域名管理
  if (['/dns', '/ssl', '/cache', '/firewall', '/analytics', '/page-rules', '/certificates'].includes(route.path)) {
    router.push('/zones')
  }
}

function handleZoneChange(key: string) {
  const zone = zones.value.find(z => z.id === key)
  if (zone) {
    currentZone.value = zone
    // 保存当前选中的域名 ID 到 localStorage
    localStorage.setItem('currentZoneId', zone.id)
  }
}

function handleThemeSelect(key: string) {
  themeStore.setTheme(key as 'light' | 'dark' | 'auto')
}

async function handleAddAccount() {
  const account = await accountStore.addAccount({
    apiToken: accountForm.value.apiToken,
    alias: accountForm.value.alias
  })

  if (account) {
    accountStore.switchAccount(account.id)
    showAccountModal.value = false
    accountForm.value = {
      apiToken: '',
      alias: ''
    }
    loadZones()
  }
}

// 初始化
onMounted(() => {
  if (accountStore.accounts.length === 0) {
    showAccountModal.value = true
  } else {
    loadZones()
  }
})

// 提供 currentZone 给子组件使用
provide('currentZone', currentZone)

// 同步 currentZone 的函数
function syncCurrentZone() {
  const savedZoneId = localStorage.getItem('currentZoneId')
  if (savedZoneId && zones.value.length > 0 && currentZone.value?.id !== savedZoneId) {
    const zone = zones.value.find(z => z.id === savedZoneId)
    if (zone) {
      console.log('Syncing currentZone to:', zone.name)
      currentZone.value = zone
    }
  }
}

// 监听路由变化，同步 currentZone
watch(() => route.path, () => {
  syncCurrentZone()
}, { flush: 'post' })

// 监听 zones 变化，同步 currentZone（确保 zones 加载完成后同步）
watch(zones, () => {
  syncCurrentZone()
}, { flush: 'post' })
</script>

<style scoped>
/* 侧边栏滚动容器 - 隐藏滚动条 */
.sidebar-content {
  height: 100%;
  overflow-y: auto;
  overflow-x: hidden;

  /* 隐藏滚动条 - Webkit 浏览器 (Chrome, Safari, Edge) */
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE 和 Edge */
}

.sidebar-content::-webkit-scrollbar {
  display: none; /* Chrome, Safari, Opera */
}

.header-section {
  padding: 16px;
  border-bottom: 1px solid var(--n-border-color);
  transition: all 0.3s;
}

.header-section.collapsed {
  padding: 16px 8px;
}

.logo-area {
  display: flex;
  align-items: flex-start;
  gap: 12px;
}

.logo-icon {
  flex-shrink: 0;
}

.logo-text {
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

.site-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--n-text-color);
  line-height: 1.4;
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.current-user {
  font-size: 12px;
  color: var(--n-text-color-2);
  line-height: 1.4;
  overflow: hidden;
}

.menu-section {
  margin-top: 16px;
}

.section-title {
  padding: 8px 16px;
  font-size: 12px;
  font-weight: 600;
  color: var(--n-text-color-3);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.domain-section {
  padding: 12px 16px;
}

.domain-title {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  gap: 8px;
}

.domain-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--n-text-color);
  text-transform: none;
  letter-spacing: normal;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.zone-switch-btn {
  flex-shrink: 0;
  padding: 0 !important;
  min-width: 24px;
}
</style>
