<template>
  <n-space vertical :size="24">
    <n-card title="多账户管理">
      <template #header-extra>
        <n-space>
          <n-button @click="handleExportAccounts">
            导出账户
          </n-button>
          <n-button @click="showImportModal = true">
            导入账户
          </n-button>
          <n-button type="primary" @click="showAddModal = true">
            添加账户
          </n-button>
        </n-space>
      </template>

      <n-alert type="info" style="margin-bottom: 16px">
        所有账户凭证仅存储在浏览器本地,绝不上传到服务器。数据完全安全。
      </n-alert>

      <n-list bordered>
        <n-list-item v-for="account in accountStore.accounts" :key="account.id">
          <n-thing>
            <template #avatar>
              <n-avatar>
                {{ account.alias[0].toUpperCase() }}
              </n-avatar>
            </template>
            <template #header>
              {{ account.alias }}
              <n-tag
                v-if="accountStore.currentAccount?.id === account.id"
                type="success"
                size="small"
                style="margin-left: 8px"
              >
                当前
              </n-tag>
            </template>
            <template #description>
              {{ account.email }}
            </template>
            <template #footer>
              <n-text depth="3">添加时间: {{ formatDate(account.createdAt) }}</n-text>
            </template>
            <template #action>
              <n-space>
                <n-button
                  v-if="accountStore.currentAccount?.id !== account.id"
                  size="small"
                  @click="handleSwitch(account.id)"
                >
                  切换
                </n-button>
                <n-popconfirm
                  @positive-click="handleDelete(account.id)"
                >
                  <template #trigger>
                    <n-button size="small" type="error" secondary>
                      删除
                    </n-button>
                  </template>
                  确定要删除此账户吗?
                </n-popconfirm>
              </n-space>
            </template>
          </n-thing>
        </n-list-item>
      </n-list>
    </n-card>

    <!-- 添加账户弹窗 -->
    <n-modal v-model:show="showAddModal" preset="dialog" title="添加 Cloudflare 账户">
      <n-form ref="formRef" :model="accountForm" :rules="formRules" label-placement="left" label-width="100">
        <n-form-item label="邮箱" path="email">
          <n-input v-model:value="accountForm.email" placeholder="your@email.com" />
        </n-form-item>
        <n-form-item label="API Key" path="apiKey">
          <n-input
            v-model:value="accountForm.apiKey"
            type="password"
            show-password-on="click"
            placeholder="Global API Key"
          />
        </n-form-item>
        <n-form-item label="别名" path="alias">
          <n-input v-model:value="accountForm.alias" placeholder="账户别名（可选）" />
        </n-form-item>

        <n-alert type="warning" style="margin-top: 16px">
          API Key 获取方法:<br />
          1. 登录 Cloudflare 控制台<br />
          2. 点击右上角头像 → 配置文件<br />
          3. API 令牌 → 下拉到 API 密钥 → 查看或创建 Global API Key
        </n-alert>
      </n-form>

      <template #action>
        <n-space>
          <n-button @click="showAddModal = false">取消</n-button>
          <n-button type="primary" @click="handleAddAccount">确认</n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 导入账户弹窗 -->
    <n-modal v-model:show="showImportModal" preset="dialog" title="导入账户" style="width: 600px">
      <n-alert type="warning" style="margin-bottom: 16px">
        导入的账户数据将与现有账户合并,相同ID的账户将被覆盖
      </n-alert>

      <n-form>
        <n-form-item label="选择文件">
          <n-upload
            :max="1"
            accept=".json"
            :custom-request="handleImportFile"
            @before-upload="beforeUpload"
          >
            <n-button>选择 JSON 文件</n-button>
          </n-upload>
        </n-form-item>

        <n-form-item label="或粘贴JSON">
          <n-input
            v-model:value="importJson"
            type="textarea"
            :rows="8"
            placeholder="粘贴导出的账户JSON数据"
          />
        </n-form-item>
      </n-form>

      <template #action>
        <n-space>
          <n-button @click="showImportModal = false">取消</n-button>
          <n-button type="primary" @click="handleImportAccounts">
            导入
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useMessage } from 'naive-ui'
import { useAccountStore, type CloudflareAccount } from '@/stores/account'

const accountStore = useAccountStore()
const message = useMessage()
const showAddModal = ref(false)
const showImportModal = ref(false)
const importJson = ref('')

const accountForm = ref({
  email: '',
  apiKey: '',
  alias: ''
})

const formRules = {
  email: { required: true, message: '请输入邮箱', trigger: 'blur' },
  apiKey: { required: true, message: '请输入 API Key', trigger: 'blur' }
}

function formatDate(date: string) {
  return new Date(date).toLocaleString('zh-CN')
}

function handleAddAccount() {
  const account = accountStore.addAccount(accountForm.value)
  if (account) {
    message.success('账户添加成功')
    showAddModal.value = false
    accountForm.value = { email: '', apiKey: '', alias: '' }
  }
}

function handleSwitch(accountId: string) {
  accountStore.switchAccount(accountId)
  message.success('账户切换成功,正在刷新...')
  setTimeout(() => {
    window.location.reload()
  }, 500)
}

function handleDelete(accountId: string) {
  accountStore.removeAccount(accountId)
  message.success('账户已删除')
}

function handleExportAccounts() {
  try {
    const accounts = accountStore.accounts
    const dataStr = JSON.stringify(accounts, null, 2)
    const dataBlob = new Blob([dataStr], { type: 'application/json' })
    const url = URL.createObjectURL(dataBlob)
    const link = document.createElement('a')
    link.href = url
    link.download = `cloudflare-accounts-${Date.now()}.json`
    link.click()
    URL.revokeObjectURL(url)
    message.success('账户导出成功')
  } catch (error) {
    message.error('导出失败')
  }
}

function beforeUpload(data: { file: { file: File } }) {
  const file = data.file.file
  if (file.type !== 'application/json') {
    message.error('只支持JSON文件')
    return false
  }
  return true
}

function handleImportFile(options: any) {
  const file = options.file.file
  const reader = new FileReader()

  reader.onload = (e) => {
    try {
      const content = e.target?.result as string
      importJson.value = content
      message.success('文件读取成功,请点击导入按钮')
    } catch (error) {
      message.error('文件读取失败')
    }
  }

  reader.readAsText(file)
}

function handleImportAccounts() {
  try {
    if (!importJson.value.trim()) {
      message.error('请选择文件或粘贴JSON数据')
      return
    }

    const accounts: CloudflareAccount[] = JSON.parse(importJson.value)

    if (!Array.isArray(accounts)) {
      message.error('JSON格式不正确,应为账户数组')
      return
    }

    // 验证数据格式
    for (const acc of accounts) {
      if (!acc.email || !acc.apiKey) {
        message.error('账户数据缺少必要字段')
        return
      }
    }

    // 合并账户
    const existingIds = new Set(accountStore.accounts.map(a => a.id))
    let imported = 0
    let updated = 0

    for (const acc of accounts) {
      if (existingIds.has(acc.id)) {
        // 更新现有账户
        const index = accountStore.accounts.findIndex(a => a.id === acc.id)
        accountStore.accounts[index] = acc
        updated++
      } else {
        // 添加新账户
        accountStore.accounts.push(acc)
        imported++
      }
    }

    // 保存到localStorage
    localStorage.setItem('cf_accounts', JSON.stringify(accountStore.accounts))

    message.success(`导入成功: 新增 ${imported} 个,更新 ${updated} 个`)
    showImportModal.value = false
    importJson.value = ''
    accountStore.loadAccounts()
  } catch (error) {
    message.error('导入失败: JSON格式错误')
  }
}
</script>
