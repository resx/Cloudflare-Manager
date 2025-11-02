<template>
  <n-layout has-sider style="height: 100vh">
    <n-layout-sider
      bordered
      collapse-mode="width"
      :collapsed-width="64"
      :width="240"
      :collapsed="collapsed"
      show-trigger
      @collapse="collapsed = true"
      @expand="collapsed = false"
    >
      <div class="logo">
        <n-text v-if="!collapsed" style="font-size: 18px; font-weight: bold">
          Cloudflare 管理平台
        </n-text>
        <n-text v-else style="font-size: 16px; font-weight: bold">CF</n-text>
      </div>

      <n-menu
        :collapsed="collapsed"
        :collapsed-width="64"
        :collapsed-icon-size="22"
        :options="menuOptions"
        :value="activeKey"
        @update:value="handleMenuSelect"
      />
    </n-layout-sider>

    <n-layout>
      <n-layout-header bordered style="height: 64px; padding: 0 24px; display: flex; align-items: center; justify-content: space-between">
        <n-text style="font-size: 20px; font-weight: 500">{{ currentTitle }}</n-text>

        <n-space align="center">
          <n-dropdown :options="themeOptions" @select="handleThemeSelect">
            <n-button circle quaternary>
              <template #icon>
                <n-icon :component="themeStore.isDark ? MoonOutline : SunnyOutline" />
              </template>
            </n-button>
          </n-dropdown>

          <n-select
            v-if="accountStore.accounts.length > 0"
            :value="accountStore.currentAccount?.id"
            :options="accountOptions"
            style="width: 240px"
            @update:value="handleAccountChange"
          />
          <n-button v-else type="primary" @click="showAccountModal = true">
            添加账户
          </n-button>
        </n-space>
      </n-layout-header>

      <n-layout-content content-style="padding: 24px;" :native-scrollbar="false">
        <router-view />
      </n-layout-content>
    </n-layout>

    <!-- 添加账户弹窗 -->
    <n-modal v-model:show="showAccountModal" preset="dialog" title="添加 Cloudflare 账户">
      <n-form ref="formRef" :model="accountForm" :rules="formRules" label-placement="left" label-width="100">
        <n-form-item label="邮箱" path="email">
          <n-input v-model:value="accountForm.email" placeholder="your@email.com" />
        </n-form-item>
        <n-form-item label="API Key" path="apiKey">
          <n-input v-model:value="accountForm.apiKey" type="password" show-password-on="click" placeholder="Global API Key" />
        </n-form-item>
        <n-form-item label="别名" path="alias">
          <n-input v-model:value="accountForm.alias" placeholder="账户别名（可选）" />
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
import { ref, computed, h } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { NIcon } from 'naive-ui'
import type { MenuOption, DropdownOption } from 'naive-ui'
import {
  HomeOutline,
  PeopleOutline,
  RocketOutline,
  SpeedometerOutline,
  ServerOutline,
  ShieldCheckmarkOutline,
  TimeOutline,
  SunnyOutline,
  MoonOutline,
  ContrastOutline
} from '@vicons/ionicons5'
import { useAccountStore } from '@/stores/account'
import { useThemeStore } from '@/stores/theme'

const router = useRouter()
const route = useRoute()
const accountStore = useAccountStore()
const themeStore = useThemeStore()

const collapsed = ref(false)
const showAccountModal = ref(false)

const accountForm = ref({
  email: '',
  apiKey: '',
  alias: ''
})

const formRules = {
  email: { required: true, message: '请输入邮箱', trigger: 'blur' },
  apiKey: { required: true, message: '请输入 API Key', trigger: 'blur' }
}

// 菜单配置
function renderIcon(icon: any) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const menuOptions: MenuOption[] = [
  {
    label: '控制台',
    key: '/dashboard',
    icon: renderIcon(HomeOutline)
  },
  {
    label: '多账户管理',
    key: '/accounts',
    icon: renderIcon(PeopleOutline)
  },
  {
    label: '一键加速部署',
    key: '/quick-deploy',
    icon: renderIcon(RocketOutline)
  },
  {
    label: '自动优化',
    key: '/optimize',
    icon: renderIcon(SpeedometerOutline)
  },
  {
    label: 'DNS 记录管理',
    key: '/dns',
    icon: renderIcon(ServerOutline)
  },
  {
    label: '防火墙规则',
    key: '/firewall',
    icon: renderIcon(ShieldCheckmarkOutline)
  },
  {
    label: '操作历史',
    key: '/history',
    icon: renderIcon(TimeOutline)
  }
]

const activeKey = computed(() => route.path)
const currentTitle = computed(() => route.meta.title as string || '控制台')

const accountOptions = computed(() =>
  accountStore.accounts.map(acc => ({
    label: acc.alias,
    value: acc.id
  }))
)

const themeOptions: DropdownOption[] = [
  {
    label: '亮色主题',
    key: 'light',
    icon: () => h(NIcon, null, { default: () => h(SunnyOutline) })
  },
  {
    label: '暗色主题',
    key: 'dark',
    icon: () => h(NIcon, null, { default: () => h(MoonOutline) })
  },
  {
    label: '跟随系统',
    key: 'auto',
    icon: () => h(NIcon, null, { default: () => h(ContrastOutline) })
  }
]

function handleMenuSelect(key: string) {
  router.push(key)
}

function handleAccountChange(accountId: string) {
  accountStore.switchAccount(accountId)
  window.location.reload() // 重新加载以更新所有数据
}

function handleThemeSelect(key: string) {
  themeStore.setTheme(key as 'light' | 'dark' | 'auto')
}

function handleAddAccount() {
  const account = accountStore.addAccount(accountForm.value)
  if (account) {
    accountStore.switchAccount(account.id)
    showAccountModal.value = false
    accountForm.value = { email: '', apiKey: '', alias: '' }
  }
}

// 如果没有账户,自动弹出添加窗口
if (accountStore.accounts.length === 0) {
  showAccountModal.value = true
}
</script>

<style scoped>
.logo {
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-bottom: 1px solid #efeff5;
}
</style>
