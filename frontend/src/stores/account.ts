import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface CloudflareCredentials {
  // 主要认证（必需）- 用于大部分 API
  email: string
  apiKey: string  // Global API Key
  // 可选的 API Token - 用于 Analytics 等 GraphQL API
  apiToken?: string
  alias?: string
}

export interface CloudflareAccount {
  id: string
  email: string
  apiKey: string  // Global API Key（必需）
  apiToken?: string  // API Token（可选，用于 Analytics）
  alias: string
  createdAt: string
}

export const useAccountStore = defineStore('account', () => {
  const accounts = ref<CloudflareAccount[]>([])
  const currentAccount = ref<CloudflareAccount | null>(null)

  // 从 localStorage 加载账户
  function loadAccounts() {
    try {
      const stored = localStorage.getItem('cf_accounts')
      if (stored) {
        accounts.value = JSON.parse(stored)
        // 加载当前账户
        const currentId = localStorage.getItem('cf_current_account')
        if (currentId) {
          currentAccount.value = accounts.value.find(acc => acc.id === currentId) || null
        }
      }
    } catch (error) {
      console.error('Failed to load accounts:', error)
    }
  }

  // 保存账户到 localStorage
  function saveAccounts() {
    try {
      localStorage.setItem('cf_accounts', JSON.stringify(accounts.value))
      if (currentAccount.value) {
        localStorage.setItem('cf_current_account', currentAccount.value.id)
      }
    } catch (error) {
      console.error('Failed to save accounts:', error)
    }
  }

  // 添加账户
  function addAccount(credentials: CloudflareCredentials) {
    const newAccount: CloudflareAccount = {
      id: Date.now().toString(),
      email: credentials.email,
      apiKey: credentials.apiKey,
      apiToken: credentials.apiToken,
      alias: credentials.alias || credentials.email,
      createdAt: new Date().toISOString()
    }
    accounts.value.push(newAccount)
    saveAccounts()
    return newAccount
  }

  // 更新账户的 API Token
  function updateApiToken(accountId: string, apiToken: string) {
    const account = accounts.value.find(acc => acc.id === accountId)
    if (account) {
      account.apiToken = apiToken
      saveAccounts()
      return true
    }
    return false
  }

  // 更新账户信息
  function updateAccount(accountId: string, credentials: CloudflareCredentials) {
    const account = accounts.value.find(acc => acc.id === accountId)
    if (account) {
      account.email = credentials.email
      account.apiKey = credentials.apiKey
      account.apiToken = credentials.apiToken
      account.alias = credentials.alias || credentials.email
      saveAccounts()

      // 如果更新的是当前账户，同步更新 currentAccount
      if (currentAccount.value?.id === accountId) {
        currentAccount.value = { ...account }
      }
      return true
    }
    return false
  }

  // 删除账户
  function removeAccount(id: string) {
    accounts.value = accounts.value.filter(acc => acc.id !== id)
    if (currentAccount.value?.id === id) {
      currentAccount.value = accounts.value[0] || null
    }
    saveAccounts()
  }

  // 切换账户
  function switchAccount(id: string) {
    const account = accounts.value.find(acc => acc.id === id)
    if (account) {
      currentAccount.value = account
      localStorage.setItem('cf_current_account', id)
    }
  }

  // 获取当前凭证（返回所有可用的认证信息）
  function getCurrentCredentials(): CloudflareCredentials | null {
    if (!currentAccount.value) return null
    return {
      email: currentAccount.value.email,
      apiKey: currentAccount.value.apiKey,
      apiToken: currentAccount.value.apiToken
    }
  }

  // 初始化
  loadAccounts()

  return {
    accounts,
    currentAccount,
    addAccount,
    updateAccount,
    updateApiToken,
    removeAccount,
    switchAccount,
    getCurrentCredentials,
    loadAccounts
  }
})
