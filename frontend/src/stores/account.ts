import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface CloudflareCredentials {
  // API Token（必需） - 更安全的认证方式
  apiToken: string
  alias?: string
}

export interface CloudflareAccount {
  id: string
  apiToken: string  // API Token（必需）
  accountId?: string  // Cloudflare Account ID（自动从 API 获取）
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
  async function addAccount(credentials: CloudflareCredentials) {
    // 先创建临时账户对象用于 API 调用
    const tempAccount: CloudflareAccount = {
      id: Date.now().toString(),
      apiToken: credentials.apiToken,
      accountId: undefined,
      alias: credentials.alias || 'Cloudflare Account',
      createdAt: new Date().toISOString()
    }

    // 临时设置为当前账户以便 API 调用能使用凭证
    const previousAccount = currentAccount.value
    currentAccount.value = tempAccount

    try {
      // 自动从 Cloudflare API 获取 Account ID
      const { cloudflareApi } = await import('@/api')
      const cfAccounts = await cloudflareApi.getAccounts()

      // 使用第一个账户的 ID 和名称
      if (cfAccounts && cfAccounts.length > 0) {
        tempAccount.accountId = cfAccounts[0].id
        tempAccount.alias = credentials.alias || cfAccounts[0].name
        console.log('Auto-fetched Account ID:', cfAccounts[0].id, 'for account:', cfAccounts[0].name)
      } else {
        console.warn('No Cloudflare accounts found for this user')
      }
    } catch (error) {
      console.error('Failed to fetch Account ID:', error)
      // 继续保存账户，即使无法获取 Account ID
    }

    // 添加到账户列表
    accounts.value.push(tempAccount)
    currentAccount.value = tempAccount
    saveAccounts()
    return tempAccount
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
  async function updateAccount(accountId: string, credentials: CloudflareCredentials) {
    const account = accounts.value.find(acc => acc.id === accountId)
    if (account) {
      account.apiToken = credentials.apiToken
      account.alias = credentials.alias || account.alias

      // 临时设置为当前账户以便 API 调用能使用凭证
      const previousAccount = currentAccount.value
      currentAccount.value = account

      try {
        // 自动从 Cloudflare API 获取 Account ID
        const { cloudflareApi } = await import('@/api')
        const cfAccounts = await cloudflareApi.getAccounts()

        // 使用第一个账户的 ID 和名称
        if (cfAccounts && cfAccounts.length > 0) {
          account.accountId = cfAccounts[0].id
          if (!credentials.alias) {
            account.alias = cfAccounts[0].name
          }
          console.log('Auto-fetched Account ID:', cfAccounts[0].id, 'for account:', cfAccounts[0].name)
        } else {
          console.warn('No Cloudflare accounts found for this user')
        }
      } catch (error) {
        console.error('Failed to fetch Account ID:', error)
        // 继续保存账户，即使无法获取 Account ID
      } finally {
        // 恢复之前的当前账户（如果不是正在更新的账户）
        if (previousAccount?.id !== accountId) {
          currentAccount.value = previousAccount
        }
      }

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

  // 获取当前凭证（仅返回 API Token 用于认证）
  function getCurrentCredentials(): CloudflareCredentials | null {
    if (!currentAccount.value) return null
    return {
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
