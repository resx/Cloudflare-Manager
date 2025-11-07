<template>
  <n-space vertical :size="24">
    <n-card title="Workers KV 存储管理">
      <template #header-extra>
        <n-button type="primary" @click="handleCreateNamespace">
          <template #icon>
            <n-icon><AddOutline /></n-icon>
          </template>
          创建 Namespace
        </n-button>
      </template>

      <n-alert type="info" style="margin-bottom: 16px">
        Workers KV 是一个全局、低延迟的键值存储服务。非常适合存储配置数据、用户会话、缓存等。
      </n-alert>

      <n-spin :show="loadingNamespaces">
        <n-empty v-if="namespaces.length === 0" description="暂无 KV Namespace">
          <template #extra>
            <n-button size="small" @click="handleCreateNamespace">
              创建第一个 Namespace
            </n-button>
          </template>
        </n-empty>

        <n-grid v-else :cols="2" :x-gap="16" :y-gap="16">
          <n-grid-item v-for="namespace in namespaces" :key="namespace.id">
            <n-card
              :title="namespace.title"
              hoverable
              :segmented="{ content: true, footer: 'soft' }"
            >
              <template #header-extra>
                <n-dropdown :options="getNamespaceActions(namespace)" @select="(key) => handleNamespaceAction(key, namespace)">
                  <n-button size="small" circle quaternary>
                    <template #icon>
                      <n-icon><EllipsisVerticalOutline /></n-icon>
                    </template>
                  </n-button>
                </n-dropdown>
              </template>

              <n-space vertical size="small">
                <n-text depth="3" style="font-size: 12px">
                  Namespace ID: <n-text code>{{ namespace.id }}</n-text>
                </n-text>
              </n-space>

              <template #footer>
                <n-space justify="space-between">
                  <n-text depth="3" style="font-size: 12px">
                    {{ keyCounts[namespace.id] !== undefined ? `${keyCounts[namespace.id]} 个键` : '加载中...' }}
                  </n-text>
                  <n-button size="small" @click="handleManageKeys(namespace)">
                    <template #icon>
                      <n-icon><KeyOutline /></n-icon>
                    </template>
                    管理键值
                  </n-button>
                </n-space>
              </template>
            </n-card>
          </n-grid-item>
        </n-grid>
      </n-spin>
    </n-card>

    <!-- 创建 Namespace 弹窗 -->
    <n-modal v-model:show="showCreateModal" preset="card" title="创建 KV Namespace" style="width: 500px">
      <n-form ref="createFormRef" :model="createForm" label-placement="left" label-width="120">
        <n-form-item label="Namespace 名称" required>
          <n-input
            v-model:value="createForm.title"
            placeholder="例如: my-kv-store"
          />
          <template #feedback>
            <n-text depth="3" style="font-size: 12px">
              名称将用于在 Workers 中引用此 Namespace
            </n-text>
          </template>
        </n-form-item>
      </n-form>

      <template #footer>
        <n-space justify="end">
          <n-button @click="showCreateModal = false">取消</n-button>
          <n-button type="primary" :loading="creating" @click="handleConfirmCreate">
            创建
          </n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 管理键值弹窗 -->
    <n-modal v-model:show="showKeysModal" preset="card" :title="`管理键值 - ${currentNamespace?.title}`" style="width: 900px">
      <n-space vertical :size="16">
        <n-space justify="space-between">
          <n-input-group>
            <n-input
              v-model:value="keySearchPrefix"
              placeholder="搜索键名前缀..."
              clearable
              style="width: 300px"
              @keyup.enter="loadKeys"
            >
              <template #prefix>
                <n-icon><SearchOutline /></n-icon>
              </template>
            </n-input>
            <n-button @click="loadKeys">
              <template #icon>
                <n-icon><SearchOutline /></n-icon>
              </template>
            </n-button>
          </n-input-group>

          <n-button type="primary" @click="handleAddKey">
            <template #icon>
              <n-icon><AddOutline /></n-icon>
            </template>
            添加键值
          </n-button>
        </n-space>

        <n-spin :show="loadingKeys">
          <n-empty v-if="keys.length === 0" description="此 Namespace 中暂无数据">
            <template #extra>
              <n-button size="small" @click="handleAddKey">
                添加第一个键值对
              </n-button>
            </template>
          </n-empty>

          <n-list v-else bordered>
            <n-list-item v-for="key in keys" :key="key.name">
              <template #prefix>
                <n-icon size="20" color="#1e90ff">
                  <KeyOutline />
                </n-icon>
              </template>

              <n-space vertical size="small">
                <n-text strong>{{ key.name }}</n-text>
                <n-text v-if="key.expiration" depth="3" style="font-size: 12px">
                  过期时间: {{ formatExpiration(key.expiration) }}
                </n-text>
              </n-space>

              <template #suffix>
                <n-space>
                  <n-button size="small" @click="handleViewValue(key)">
                    <template #icon>
                      <n-icon><EyeOutline /></n-icon>
                    </template>
                    查看
                  </n-button>
                  <n-button size="small" @click="handleEditKey(key)">
                    <template #icon>
                      <n-icon><CreateOutline /></n-icon>
                    </template>
                    编辑
                  </n-button>
                  <n-button size="small" type="error" @click="handleDeleteKey(key)">
                    <template #icon>
                      <n-icon><TrashOutline /></n-icon>
                    </template>
                    删除
                  </n-button>
                </n-space>
              </template>
            </n-list-item>
          </n-list>
        </n-spin>
      </n-space>
    </n-modal>

    <!-- 添加/编辑键值弹窗 -->
    <n-modal v-model:show="showKeyModal" preset="card" :title="editingKey ? '编辑键值' : '添加键值'" style="width: 600px">
      <n-form ref="keyFormRef" :model="keyForm" label-placement="left" label-width="120">
        <n-form-item label="键名" required>
          <n-input
            v-model:value="keyForm.key"
            placeholder="键名（支持字母、数字、下划线、连字符）"
            :disabled="!!editingKey"
          />
        </n-form-item>

        <n-form-item label="值" required>
          <n-input
            v-model:value="keyForm.value"
            type="textarea"
            placeholder="输入值内容（支持 JSON、文本等）"
            :rows="8"
          />
        </n-form-item>

        <n-form-item label="过期时间(秒)">
          <n-input-number
            v-model:value="keyForm.expirationTtl"
            placeholder="留空表示永不过期"
            :min="60"
            style="width: 100%"
          />
          <template #feedback>
            <n-text depth="3" style="font-size: 12px">
              设置后，键值将在指定秒数后自动删除
            </n-text>
          </template>
        </n-form-item>
      </n-form>

      <template #footer>
        <n-space justify="end">
          <n-button @click="showKeyModal = false">取消</n-button>
          <n-button type="primary" :loading="savingKey" @click="handleConfirmSaveKey">
            保存
          </n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 查看值弹窗 -->
    <n-modal v-model:show="showValueModal" preset="card" title="查看键值" style="width: 700px">
      <n-spin :show="loadingValue">
        <n-space vertical :size="16">
          <n-descriptions :column="1" bordered>
            <n-descriptions-item label="键名">
              <n-text code>{{ viewingKey?.name }}</n-text>
            </n-descriptions-item>
            <n-descriptions-item v-if="viewingKey?.expiration" label="过期时间">
              {{ formatExpiration(viewingKey.expiration) }}
            </n-descriptions-item>
          </n-descriptions>

          <n-divider>值内容</n-divider>

          <n-code
            :code="currentValue"
            :language="isJsonValue(currentValue) ? 'json' : 'text'"
            :show-line-numbers="true"
            style="max-height: 400px; overflow: auto"
          />
        </n-space>
      </n-spin>

      <template #footer>
        <n-space justify="end">
          <n-button @click="showValueModal = false">关闭</n-button>
          <n-button type="primary" @click="handleCopyValue">
            <template #icon>
              <n-icon><CopyOutline /></n-icon>
            </template>
            复制
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, inject, type Ref } from 'vue'
import { useMessage, useDialog } from 'naive-ui'
import {
  AddOutline,
  KeyOutline,
  SearchOutline,
  EyeOutline,
  CreateOutline,
  TrashOutline,
  CopyOutline,
  EllipsisVerticalOutline
} from '@vicons/ionicons5'
import { cloudflareApi, type KVNamespace, type KVKey, type CloudflareAccount } from '@/api'
import { useAccountStore } from '@/stores/account'

const message = useMessage()
const dialog = useDialog()
const accountStore = useAccountStore()

const loadingNamespaces = ref(false)
const namespaces = ref<KVNamespace[]>([])
const keyCounts = ref<Record<string, number>>({})

const showCreateModal = ref(false)
const creating = ref(false)
const createForm = ref({
  title: ''
})

const showKeysModal = ref(false)
const currentNamespace = ref<KVNamespace | null>(null)
const loadingKeys = ref(false)
const keys = ref<KVKey[]>([])
const keySearchPrefix = ref('')

const showKeyModal = ref(false)
const editingKey = ref<KVKey | null>(null)
const savingKey = ref(false)
const keyForm = ref({
  key: '',
  value: '',
  expirationTtl: null as number | null
})

const showValueModal = ref(false)
const viewingKey = ref<KVKey | null>(null)
const loadingValue = ref(false)
const currentValue = ref('')

// 获取当前账户 ID
function getCurrentAccountId(): string | null {
  const account = accountStore.currentAccount
  if (!account?.cloudflareAccountId) {
    message.error('未找到 Cloudflare 账户 ID，请先验证账户')
    return null
  }
  return account.cloudflareAccountId
}

async function loadNamespaces() {
  const accountId = getCurrentAccountId()
  if (!accountId) return

  loadingNamespaces.value = true
  try {
    namespaces.value = await cloudflareApi.listKVNamespaces(accountId)

    // 为每个 namespace 加载键数量
    for (const ns of namespaces.value) {
      loadKeyCount(ns)
    }
  } catch (error: any) {
    console.error('Load KV namespaces error:', error)
    message.error(error?.message || '加载 KV Namespace 失败')
  } finally {
    loadingNamespaces.value = false
  }
}

async function loadKeyCount(namespace: KVNamespace) {
  const accountId = getCurrentAccountId()
  if (!accountId) return

  try {
    const keys = await cloudflareApi.listKVKeys(accountId, namespace.id)
    keyCounts.value[namespace.id] = keys.length
  } catch (error) {
    console.error(`Failed to load key count for ${namespace.id}:`, error)
    keyCounts.value[namespace.id] = 0
  }
}

function handleCreateNamespace() {
  createForm.value = { title: '' }
  showCreateModal.value = true
}

async function handleConfirmCreate() {
  const accountId = getCurrentAccountId()
  if (!accountId) return

  if (!createForm.value.title) {
    message.warning('请输入 Namespace 名称')
    return
  }

  creating.value = true
  try {
    await cloudflareApi.createKVNamespace({
      account_id: accountId,
      title: createForm.value.title
    })
    message.success('Namespace 创建成功')
    showCreateModal.value = false
    await loadNamespaces()
  } catch (error: any) {
    message.error(error?.message || '创建失败')
  } finally {
    creating.value = false
  }
}

function getNamespaceActions(namespace: KVNamespace) {
  return [
    {
      label: '删除 Namespace',
      key: 'delete',
      props: {
        style: { color: 'red' }
      }
    }
  ]
}

function handleNamespaceAction(key: string, namespace: KVNamespace) {
  if (key === 'delete') {
    handleDeleteNamespace(namespace)
  }
}

function handleDeleteNamespace(namespace: KVNamespace) {
  const accountId = getCurrentAccountId()
  if (!accountId) return

  dialog.warning({
    title: '确认删除',
    content: `确定要删除 Namespace "${namespace.title}" 吗？此操作将删除其中的所有键值对，且无法撤销。`,
    positiveText: '删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await cloudflareApi.deleteKVNamespace(accountId, namespace.id)
        message.success('Namespace 已删除')
        await loadNamespaces()
      } catch (error: any) {
        message.error(error?.message || '删除失败')
      }
    }
  })
}

async function handleManageKeys(namespace: KVNamespace) {
  currentNamespace.value = namespace
  keySearchPrefix.value = ''
  showKeysModal.value = true
  await loadKeys()
}

async function loadKeys() {
  if (!currentNamespace.value) return

  const accountId = getCurrentAccountId()
  if (!accountId) return

  loadingKeys.value = true
  try {
    keys.value = await cloudflareApi.listKVKeys(
      accountId,
      currentNamespace.value.id,
      keySearchPrefix.value || undefined
    )
  } catch (error: any) {
    console.error('Load KV keys error:', error)
    message.error(error?.message || '加载键列表失败')
  } finally {
    loadingKeys.value = false
  }
}

function handleAddKey() {
  editingKey.value = null
  keyForm.value = {
    key: '',
    value: '',
    expirationTtl: null
  }
  showKeyModal.value = true
}

function handleEditKey(key: KVKey) {
  editingKey.value = key
  keyForm.value = {
    key: key.name,
    value: '',
    expirationTtl: null
  }
  showKeyModal.value = true

  // 加载当前值
  loadKeyValueForEdit(key)
}

async function loadKeyValueForEdit(key: KVKey) {
  if (!currentNamespace.value) return

  const accountId = getCurrentAccountId()
  if (!accountId) return

  try {
    const value = await cloudflareApi.readKVValue(accountId, currentNamespace.value.id, key.name)
    keyForm.value.value = value
  } catch (error: any) {
    console.error('Load key value error:', error)
    message.error(error?.message || '加载键值失败')
  }
}

async function handleConfirmSaveKey() {
  if (!currentNamespace.value) return

  const accountId = getCurrentAccountId()
  if (!accountId) return

  if (!keyForm.value.key) {
    message.warning('请输入键名')
    return
  }

  if (!keyForm.value.value) {
    message.warning('请输入值')
    return
  }

  savingKey.value = true
  try {
    await cloudflareApi.writeKVValue({
      account_id: accountId,
      namespace_id: currentNamespace.value.id,
      key: keyForm.value.key,
      value: keyForm.value.value,
      expiration_ttl: keyForm.value.expirationTtl || undefined
    })
    message.success(editingKey.value ? '键值已更新' : '键值已添加')
    showKeyModal.value = false
    await loadKeys()
  } catch (error: any) {
    message.error(error?.message || '保存失败')
  } finally {
    savingKey.value = false
  }
}

async function handleViewValue(key: KVKey) {
  if (!currentNamespace.value) return

  const accountId = getCurrentAccountId()
  if (!accountId) return

  viewingKey.value = key
  showValueModal.value = true
  loadingValue.value = true
  currentValue.value = ''

  try {
    currentValue.value = await cloudflareApi.readKVValue(accountId, currentNamespace.value.id, key.name)
  } catch (error: any) {
    console.error('Read KV value error:', error)
    message.error(error?.message || '读取值失败')
    currentValue.value = '加载失败'
  } finally {
    loadingValue.value = false
  }
}

function handleDeleteKey(key: KVKey) {
  if (!currentNamespace.value) return

  const accountId = getCurrentAccountId()
  if (!accountId) return

  dialog.warning({
    title: '确认删除',
    content: `确定要删除键 "${key.name}" 吗？此操作无法撤销。`,
    positiveText: '删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await cloudflareApi.deleteKVKey({
          account_id: accountId,
          namespace_id: currentNamespace.value!.id,
          key: key.name
        })
        message.success('键已删除')
        await loadKeys()
      } catch (error: any) {
        message.error(error?.message || '删除失败')
      }
    }
  })
}

function formatExpiration(timestamp: number): string {
  return new Date(timestamp * 1000).toLocaleString('zh-CN')
}

function isJsonValue(value: string): boolean {
  try {
    JSON.parse(value)
    return true
  } catch {
    return false
  }
}

async function handleCopyValue() {
  try {
    await navigator.clipboard.writeText(currentValue.value)
    message.success('已复制到剪贴板')
  } catch (error) {
    message.error('复制失败')
  }
}

onMounted(() => {
  loadNamespaces()
})
</script>
