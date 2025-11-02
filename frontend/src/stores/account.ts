import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface CloudflareCredentials {
  email: string
  apiKey: string
  alias?: string
}

export interface CloudflareAccount {
  id: string
  email: string
  apiKey: string
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
      alias: credentials.alias || credentials.email,
      createdAt: new Date().toISOString()
    }
    accounts.value.push(newAccount)
    saveAccounts()
    return newAccount
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

  // 获取当前凭证
  function getCurrentCredentials(): CloudflareCredentials | null {
    if (!currentAccount.value) return null
    return {
      email: currentAccount.value.email,
      apiKey: currentAccount.value.apiKey
    }
  }

  // 初始化
  loadAccounts()

  return {
    accounts,
    currentAccount,
    addAccount,
    removeAccount,
    switchAccount,
    getCurrentCredentials,
    loadAccounts
  }
})
