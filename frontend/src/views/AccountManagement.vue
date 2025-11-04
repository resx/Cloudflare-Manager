<template>
  <div>
    <n-space vertical size="large">
      <n-card title="账户管理">
        <template #header-extra>
          <n-button type="primary" @click="showAddModal = true">
            <template #icon>
              <n-icon><AddOutline /></n-icon>
            </template>
            添加账户
          </n-button>
        </template>

        <n-data-table
          :columns="columns"
          :data="accountStore.accounts"
          :pagination="false"
          :bordered="false"
        />
      </n-card>
    </n-space>

    <!-- 添加/编辑账户弹窗 -->
    <n-modal v-model:show="showEditModal" preset="dialog" :title="editingAccount ? '编辑账户' : '添加账户'" style="width: 550px">
      <n-form ref="formRef" :model="accountForm" :rules="formRules" label-placement="left" label-width="110">
        <n-alert type="info" style="margin-bottom: 16px; font-size: 13px">
          <strong>必填：</strong>Email + Global API Key（用于所有基础功能）<br>
          <strong>可选：</strong>API Token（仅用于 Analytics 统计分析功能）
        </n-alert>

        <!-- Email + API Key（必填） -->
        <n-divider style="margin: 12px 0; font-weight: bold">基础认证（必填）</n-divider>

        <n-form-item label="邮箱" path="email">
          <n-input v-model:value="accountForm.email" placeholder="your@email.com" />
        </n-form-item>

        <n-form-item label="Global API Key" path="apiKey">
          <n-input
            v-model:value="accountForm.apiKey"
            type="password"
            show-password-on="click"
            placeholder="输入 Global API Key"
          />
        </n-form-item>

        <n-alert type="info" style="margin-bottom: 16px; font-size: 12px">
          在 Cloudflare Dashboard → My Profile → API Tokens → Global API Key → View
        </n-alert>

        <!-- API Token（可选） -->
        <n-divider style="margin: 12px 0; font-weight: bold">Analytics 认证（可选）</n-divider>

        <n-form-item label="API Token">
          <n-input
            v-model:value="accountForm.apiToken"
            type="password"
            show-password-on="click"
            placeholder="留空则 Analytics 功能不可用"
          />
        </n-form-item>

        <n-alert type="warning" style="margin-bottom: 16px; font-size: 12px">
          如需使用 Analytics 功能，请创建具有 <strong>Zone.Analytics (Read)</strong> 权限的 API Token
        </n-alert>

        <!-- 别名 -->
        <n-divider style="margin: 12px 0"></n-divider>

        <n-form-item label="别名">
          <n-input v-model:value="accountForm.alias" placeholder="账户别名（可选）" />
        </n-form-item>
      </n-form>

      <template #action>
        <n-space>
          <n-button @click="showEditModal = false">取消</n-button>
          <n-button type="primary" @click="handleSaveAccount">确认</n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 添加账户弹窗（简化版） -->
    <n-modal v-model:show="showAddModal" preset="dialog" title="添加账户" style="width: 550px">
      <n-form ref="addFormRef" :model="accountForm" :rules="formRules" label-placement="left" label-width="110">
        <n-alert type="info" style="margin-bottom: 16px; font-size: 13px">
          <strong>必填：</strong>Email + Global API Key（用于所有基础功能）<br>
          <strong>可选：</strong>API Token（仅用于 Analytics 统计分析功能）
        </n-alert>

        <!-- Email + API Key（必填） -->
        <n-divider style="margin: 12px 0; font-weight: bold">基础认证（必填）</n-divider>

        <n-form-item label="邮箱" path="email">
          <n-input v-model:value="accountForm.email" placeholder="your@email.com" />
        </n-form-item>

        <n-form-item label="Global API Key" path="apiKey">
          <n-input
            v-model:value="accountForm.apiKey"
            type="password"
            show-password-on="click"
            placeholder="输入 Global API Key"
          />
        </n-form-item>

        <n-alert type="info" style="margin-bottom: 16px; font-size: 12px">
          在 Cloudflare Dashboard → My Profile → API Tokens → Global API Key → View
        </n-alert>

        <!-- API Token（可选） -->
        <n-divider style="margin: 12px 0; font-weight: bold">Analytics 认证（可选）</n-divider>

        <n-form-item label="API Token">
          <n-input
            v-model:value="accountForm.apiToken"
            type="password"
            show-password-on="click"
            placeholder="留空则 Analytics 功能不可用"
          />
        </n-form-item>

        <n-alert type="warning" style="margin-bottom: 16px; font-size: 12px">
          如需使用 Analytics 功能，请创建具有 <strong>Zone.Analytics (Read)</strong> 权限的 API Token
        </n-alert>

        <!-- 别名 -->
        <n-divider style="margin: 12px 0"></n-divider>

        <n-form-item label="别名">
          <n-input v-model:value="accountForm.alias" placeholder="账户别名（可选）" />
        </n-form-item>
      </n-form>

      <template #action>
        <n-space>
          <n-button @click="showAddModal = false">取消</n-button>
          <n-button type="primary" @click="handleAddAccount">确认</n-button>
        </n-space>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, h } from 'vue'
import { useMessage, NButton, NSpace, NTag, type DataTableColumns } from 'naive-ui'
import { AddOutline, CreateOutline, TrashOutline, CheckmarkCircleOutline } from '@vicons/ionicons5'
import { useAccountStore, type CloudflareAccount } from '@/stores/account'

const message = useMessage()
const accountStore = useAccountStore()

const showAddModal = ref(false)
const showEditModal = ref(false)
const editingAccount = ref<CloudflareAccount | null>(null)

const accountForm = ref({
  email: '',
  apiKey: '',
  apiToken: '',
  alias: ''
})

const formRules = {
  email: { required: true, message: '请输入邮箱', trigger: 'blur' },
  apiKey: { required: true, message: '请输入 Global API Key', trigger: 'blur' }
}

const columns: DataTableColumns<CloudflareAccount> = [
  {
    title: '别名',
    key: 'alias',
    width: 150,
    render: (row) => {
      const isCurrent = accountStore.currentAccount?.id === row.id
      return h(
        NSpace,
        { align: 'center' },
        {
          default: () => [
            h('span', row.alias),
            isCurrent ? h(NTag, { type: 'success', size: 'small' }, { default: () => '当前' }) : null
          ]
        }
      )
    }
  },
  {
    title: 'Email',
    key: 'email',
    width: 200
  },
  {
    title: 'Global API Key',
    key: 'apiKey',
    width: 150,
    render: () => h('span', '••••••••••••')
  },
  {
    title: 'API Token',
    key: 'apiToken',
    width: 120,
    render: (row) => h(
      NTag,
      { type: row.apiToken ? 'success' : 'default', size: 'small' },
      { default: () => row.apiToken ? '已配置' : '未配置' }
    )
  },
  {
    title: '创建时间',
    key: 'createdAt',
    width: 180,
    render: (row) => new Date(row.createdAt).toLocaleString('zh-CN')
  },
  {
    title: '操作',
    key: 'actions',
    width: 200,
    render: (row) => h(
      NSpace,
      {},
      {
        default: () => [
          accountStore.currentAccount?.id !== row.id
            ? h(
                NButton,
                {
                  size: 'small',
                  type: 'info',
                  onClick: () => handleSwitchAccount(row.id)
                },
                {
                  default: () => '切换',
                  icon: () => h(CheckmarkCircleOutline)
                }
              )
            : null,
          h(
            NButton,
            {
              size: 'small',
              onClick: () => handleEditAccount(row)
            },
            {
              default: () => '编辑',
              icon: () => h(CreateOutline)
            }
          ),
          h(
            NButton,
            {
              size: 'small',
              type: 'error',
              onClick: () => handleDeleteAccount(row)
            },
            {
              default: () => '删除',
              icon: () => h(TrashOutline)
            }
          )
        ]
      }
    )
  }
]

function handleEditAccount(account: CloudflareAccount) {
  editingAccount.value = account
  accountForm.value = {
    email: account.email,
    apiKey: account.apiKey,
    apiToken: account.apiToken || '',
    alias: account.alias
  }
  showEditModal.value = true
}

function handleSaveAccount() {
  if (!editingAccount.value) return

  const success = accountStore.updateAccount(editingAccount.value.id, {
    email: accountForm.value.email,
    apiKey: accountForm.value.apiKey,
    apiToken: accountForm.value.apiToken || undefined,
    alias: accountForm.value.alias || accountForm.value.email
  })

  if (success) {
    message.success('账户更新成功')
    showEditModal.value = false
    editingAccount.value = null
    accountForm.value = {
      email: '',
      apiKey: '',
      apiToken: '',
      alias: ''
    }
  } else {
    message.error('账户更新失败')
  }
}

function handleAddAccount() {
  const account = accountStore.addAccount({
    email: accountForm.value.email,
    apiKey: accountForm.value.apiKey,
    apiToken: accountForm.value.apiToken || undefined,
    alias: accountForm.value.alias || accountForm.value.email
  })

  if (account) {
    message.success('账户添加成功')
    showAddModal.value = false
    accountForm.value = {
      email: '',
      apiKey: '',
      apiToken: '',
      alias: ''
    }
  } else {
    message.error('账户添加失败')
  }
}

function handleDeleteAccount(account: CloudflareAccount) {
  // 不允许删除当前正在使用的账户
  if (accountStore.currentAccount?.id === account.id) {
    message.warning('不能删除当前正在使用的账户，请先切换到其他账户')
    return
  }

  const confirmed = window.confirm(`确定要删除账户 "${account.alias}" 吗？`)
  if (confirmed) {
    accountStore.removeAccount(account.id)
    message.success('账户删除成功')
  }
}

function handleSwitchAccount(accountId: string) {
  accountStore.switchAccount(accountId)
  message.success('账户切换成功')
  // 刷新页面以重新加载域名列表
  window.location.reload()
}
</script>

<style scoped>
:deep(.n-data-table-td) {
  vertical-align: middle;
}
</style>
