<template>
  <!-- Accounts Management - Island Theme -->
  <div class="animate-in">
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-semibold">账户管理</h1>
        <p class="text-sm text-muted-foreground mt-1">管理您的 Cloudflare API 账户</p>
      </div>
      <div class="flex gap-3">
        <button class="btn-island-secondary text-sm" @click="exportAccounts">
          📤 导出
        </button>
        <button class="btn-island-secondary text-sm" @click="showImportModal = true">
          📥 导入
        </button>
        <button class="btn-island-primary" @click="showAddModal = true">
          <span class="text-lg mr-2">+</span>
          添加账户
        </button>
      </div>
    </div>

    <!-- Accounts Grid -->
    <div v-if="accountStore.accounts.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
      <div 
        v-for="account in accountStore.accounts" 
        :key="account.id"
        :class="[
          'metric-card p-6 cursor-pointer transition-all',
          accountStore.currentAccount?.id === account.id ? 'border-primary' : 'hover:border-primary/50'
        ]"
        @click="switchAccount(account.id)"
      >
        <!-- Account Avatar -->
        <div class="flex items-start justify-between mb-4">
          <div class="w-12 h-12 rounded-full flex items-center justify-center text-lg font-semibold" 
               :style="{ backgroundColor: getAccountColor(account.id) + '20', color: getAccountColor(account.id) }">
            {{ account.alias[0].toUpperCase() }}
          </div>
          <button 
            @click.stop="deleteAccount(account.id)"
            class="text-muted-foreground hover:text-red-600 transition-colors"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
            </svg>
          </button>
        </div>

        <!-- Account Info -->
        <h3 class="font-semibold mb-2">{{ account.alias }}</h3>
        <div class="text-xs text-muted-foreground space-y-1">
          <div class="flex items-center">
            <span class="mr-2">🔑</span>
            <span>Token: {{ maskToken(account.apiToken) }}</span>
          </div>
          <div v-if="account.accountId" class="flex items-center">
            <span class="mr-2">🆔</span>
            <span>ID: {{ account.accountId.substring(0, 8) }}...</span>
          </div>
        </div>

        <!-- Active Badge -->
        <div v-if="accountStore.currentAccount?.id === account.id" class="mt-4">
          <span class="px-2 py-1 text-xs rounded-full bg-success text-success-foreground">
            当前使用
          </span>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">👤</div>
      <h3 class="font-semibold mb-2">暂无账户</h3>
      <p class="text-sm text-muted-foreground mb-4">添加您的第一个 Cloudflare 账户开始使用</p>
      <button class="btn-island-primary" @click="showAddModal = true">
        添加账户
      </button>
    </div>

    <!-- Account Info Section -->
    <div v-if="accountStore.currentAccount" class="mt-8">
      <h2 class="text-lg font-semibold mb-4">当前账户信息</h2>
      <div class="metric-card p-6">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div>
            <div class="text-sm text-muted-foreground mb-1">账户别名</div>
            <div class="font-medium">{{ accountStore.currentAccount.alias }}</div>
          </div>
          <div>
            <div class="text-sm text-muted-foreground mb-1">账户 ID</div>
            <div class="font-mono text-sm">{{ accountStore.currentAccount.accountId || '加载中...' }}</div>
          </div>
          <div>
            <div class="text-sm text-muted-foreground mb-1">API Token</div>
            <div class="font-mono text-sm">{{ maskToken(accountStore.currentAccount.apiToken) }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Add Account Modal -->
    <div v-if="showAddModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="showAddModal = false">
      <div class="bg-white rounded-2xl shadow-lg w-full max-w-xl" @click.stop>
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
            <label class="block text-sm font-medium mb-2">别名 *</label>
            <input
              v-model="accountForm.alias"
              class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
              placeholder="为账户设置一个别名，例如：公司账户"
            />
          </div>
        </div>

        <div class="p-6 border-t border-border flex justify-end gap-3">
          <button class="btn-island-secondary" @click="showAddModal = false">取消</button>
          <button class="btn-island-primary" @click="handleAddAccount">添加账户</button>
        </div>
      </div>
    </div>

    <!-- Import Modal -->
    <div v-if="showImportModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="showImportModal = false">
      <div class="bg-white rounded-2xl shadow-lg w-full max-w-xl" @click.stop>
        <div class="p-6 border-b border-border">
          <h2 class="text-xl font-semibold">导入账户</h2>
        </div>
        
        <div class="p-6">
          <label class="block text-sm font-medium mb-2">JSON 数据</label>
          <textarea
            v-model="importText"
            class="w-full px-3 py-2.5 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50 font-mono"
            rows="10"
            placeholder='粘贴导出的 JSON 数据'
          ></textarea>
        </div>

        <div class="p-6 border-t border-border flex justify-end gap-3">
          <button class="btn-island-secondary" @click="showImportModal = false">取消</button>
          <button class="btn-island-primary" @click="handleImport">导入</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useAccountStore } from '@/stores/account'
import { toast } from '@/utils/toast'

const accountStore = useAccountStore()
const showAddModal = ref(false)
const showImportModal = ref(false)
const importText = ref('')

const accountForm = ref({
  apiToken: '',
  alias: ''
})

const accountColors = ['#1d4ed8', '#7c3aed', '#059669', '#dc2626', '#ea580c']

function getAccountColor(accountId: string): string {
  const index = accountId.charCodeAt(0) % accountColors.length
  return accountColors[index]
}

function maskToken(token: string): string {
  if (!token) return ''
  return token.substring(0, 8) + '...' + token.substring(token.length - 4)
}

function switchAccount(accountId: string) {
  accountStore.switchAccount(accountId)
  toast.success('已切换账户')
}

async function deleteAccount(accountId: string) {
  if (confirm('确定要删除这个账户吗？')) {
    accountStore.removeAccount(accountId)
    toast.success('账户已删除')
  }
}

async function handleAddAccount() {
  if (!accountForm.value.apiToken.trim() || !accountForm.value.alias.trim()) {
    toast.warning('请填写所有必填字段')
    return
  }

  const account = await accountStore.addAccount({
    apiToken: accountForm.value.apiToken,
    alias: accountForm.value.alias
  })

  if (account) {
    showAddModal.value = false
    accountForm.value = { apiToken: '', alias: '' }
    toast.success('账户添加成功')
  }
}

function exportAccounts() {
  const data = JSON.stringify(accountStore.accounts, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `cloudflare-accounts-${Date.now()}.json`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
  toast.success('账户已导出')
}

function handleImport() {
  try {
    const accounts = JSON.parse(importText.value)
    if (!Array.isArray(accounts)) {
      toast.error('无效的JSON格式')
      return
    }
    
    let imported = 0
    accounts.forEach(acc => {
      if (acc.apiToken && acc.alias) {
        accountStore.addAccount(acc)
        imported++
      }
    })
    
    showImportModal.value = false
    importText.value = ''
    toast.success(`成功导入 ${imported} 个账户`)
  } catch (error) {
    toast.error('导入失败：JSON 格式错误')
  }
}
</script>
