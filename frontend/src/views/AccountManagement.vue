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

        <n-alert type="warning" style="margin-bottom: 16px">
          <strong>安全提示：</strong>本平台使用 API Token 认证，不使用 Global API Key。API Token 更安全，可以限制权限范围和访问时间。
        </n-alert>

        <n-data-table
          :columns="columns"
          :data="accountStore.accounts"
          :pagination="false"
          :bordered="false"
        />
      </n-card>
    </n-space>

    <!-- 添加/编辑账户弹窗 -->
    <n-modal v-model:show="showEditModal" preset="dialog" :title="editingAccount ? '编辑账户' : '添加账户'" style="width: 600px">
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
            4. 添加所需权限（见下方）
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
          <n-button @click="showEditModal = false">取消</n-button>
          <n-button type="primary" :loading="saving" @click="handleSaveAccount">确认</n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 添加账户弹窗 -->
    <n-modal v-model:show="showAddModal" preset="dialog" title="添加账户" style="width: 600px">
      <n-form ref="addFormRef" :model="accountForm" :rules="formRules" label-placement="left" label-width="100">
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
            4. 添加所需权限（见下方）
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
          <n-button @click="showAddModal = false">取消</n-button>
          <n-button type="primary" :loading="saving" @click="handleAddAccount">确认</n-button>
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
const saving = ref(false)
const editingAccount = ref<CloudflareAccount | null>(null)

const accountForm = ref({
  apiToken: '',
  alias: ''
})

const formRules = {
  apiToken: { required: true, message: '请输入 API Token', trigger: 'blur' }
}

const columns: DataTableColumns<CloudflareAccount> = [
  {
    title: '别名',
    key: 'alias',
    width: 200,
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
    title: 'Account ID',
    key: 'accountId',
    width: 250,
    ellipsis: {
      tooltip: true
    },
    render: (row) => row.accountId || h(NTag, { type: 'warning', size: 'small' }, { default: () => '未获取' })
  },
  {
    title: 'API Token',
    key: 'apiToken',
    width: 120,
    render: () => h('span', '••••••••••••')
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
    apiToken: account.apiToken,
    alias: account.alias
  }
  showEditModal.value = true
}

async function handleSaveAccount() {
  if (!editingAccount.value) return

  saving.value = true
  try {
    const success = await accountStore.updateAccount(editingAccount.value.id, {
      apiToken: accountForm.value.apiToken,
      alias: accountForm.value.alias || editingAccount.value.alias
    })

    if (success) {
      message.success('账户更新成功')
      showEditModal.value = false
      editingAccount.value = null
      accountForm.value = {
        apiToken: '',
        alias: ''
      }
    } else {
      message.error('账户更新失败')
    }
  } catch (error: any) {
    message.error(error?.message || '账户更新失败')
  } finally {
    saving.value = false
  }
}

async function handleAddAccount() {
  saving.value = true
  try {
    const account = await accountStore.addAccount({
      apiToken: accountForm.value.apiToken,
      alias: accountForm.value.alias
    })

    if (account) {
      message.success('账户添加成功')
      showAddModal.value = false
      accountForm.value = {
        apiToken: '',
        alias: ''
      }
    } else {
      message.error('账户添加失败')
    }
  } catch (error: any) {
    message.error(error?.message || '账户添加失败')
  } finally {
    saving.value = false
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
