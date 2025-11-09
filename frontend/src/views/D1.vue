<template>
  <n-space vertical :size="24">
    <n-card title="D1 数据库管理">
      <template #header-extra>
        <n-button type="primary" @click="handleCreateDatabase">
          <template #icon>
            <n-icon><AddOutline /></n-icon>
          </template>
          创建数据库
        </n-button>
      </template>

      <n-alert type="info" style="margin-bottom: 16px">
        D1 是 Cloudflare 的无服务器 SQL 数据库，基于 SQLite 构建，提供全球边缘数据库服务。
      </n-alert>

      <n-spin :show="loadingDatabases">
        <n-empty v-if="databases.length === 0" description="暂无数据库">
          <template #extra>
            <n-button size="small" @click="handleCreateDatabase">
              创建第一个数据库
            </n-button>
          </template>
        </n-empty>

        <n-grid v-else :cols="2" :x-gap="16" :y-gap="16">
          <n-grid-item v-for="database in databases" :key="database.uuid">
            <n-card
              :title="database.name"
              hoverable
              :segmented="{ content: true, footer: 'soft' }"
            >
              <template #header-extra>
                <n-dropdown :options="getDatabaseActions(database)" @select="(key) => handleDatabaseAction(key, database)">
                  <n-button size="small" circle quaternary>
                    <template #icon>
                      <n-icon><EllipsisVerticalOutline /></n-icon>
                    </template>
                  </n-button>
                </n-dropdown>
              </template>

              <n-space vertical size="small">
                <n-descriptions :column="1" size="small">
                  <n-descriptions-item label="UUID">
                    <n-text code style="font-size: 11px">{{ database.uuid }}</n-text>
                  </n-descriptions-item>
                  <n-descriptions-item label="版本">
                    {{ database.version }}
                  </n-descriptions-item>
                  <n-descriptions-item label="创建时间">
                    {{ formatDate(database.created_at) }}
                  </n-descriptions-item>
                </n-descriptions>
              </n-space>

              <template #footer>
                <n-button size="small" @click="handleOpenQuery(database)" block>
                  <template #icon>
                    <n-icon><CodeSlashOutline /></n-icon>
                  </template>
                  SQL 查询
                </n-button>
              </template>
            </n-card>
          </n-grid-item>
        </n-grid>
      </n-spin>
    </n-card>

    <!-- 创建数据库弹窗 -->
    <n-modal v-model:show="showCreateModal" preset="card" title="创建 D1 数据库" style="width: 500px">
      <n-form ref="createFormRef" :model="createForm" label-placement="left" label-width="120">
        <n-form-item label="数据库名称" required>
          <n-input
            v-model:value="createForm.name"
            placeholder="例如: my-database"
          />
          <template #feedback>
            <n-text depth="3" style="font-size: 12px">
              数据库名称只能包含字母、数字、连字符和下划线
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

    <!-- SQL 查询弹窗 -->
    <n-modal v-model:show="showQueryModal" preset="card" :title="`SQL 查询 - ${currentDatabase?.name}`" style="width: 1000px">
      <n-space vertical :size="16">
        <n-alert type="info">
          在下方输入 SQL 查询语句。支持 SELECT、INSERT、UPDATE、DELETE 等标准 SQLite 语法。
        </n-alert>

        <n-tabs type="segment">
          <n-tab-pane name="query" tab="执行查询">
            <n-space vertical :size="16">
              <n-input
                v-model:value="queryText"
                type="textarea"
                placeholder="输入 SQL 查询，例如: SELECT * FROM users LIMIT 10"
                :rows="6"
              />

              <n-space>
                <n-button type="primary" :loading="executing" @click="handleExecuteQuery">
                  <template #icon>
                    <n-icon><PlayOutline /></n-icon>
                  </template>
                  执行查询
                </n-button>
                <n-button @click="queryText = ''">
                  清空
                </n-button>
              </n-space>

              <!-- 查询结果 -->
              <n-card v-if="queryResult" title="查询结果" size="small">
                <template #header-extra>
                  <n-space size="small">
                    <n-tag size="small" :type="queryResult.success ? 'success' : 'error'">
                      {{ queryResult.success ? '成功' : '失败' }}
                    </n-tag>
                    <n-text depth="3" style="font-size: 12px">
                      耗时: {{ queryResult.meta?.duration.toFixed(2) }}ms
                    </n-text>
                  </n-space>
                </template>

                <n-space vertical :size="12">
                  <n-text depth="3" style="font-size: 12px">
                    读取行数: {{ queryResult.meta?.rows_read }} |
                    写入行数: {{ queryResult.meta?.rows_written }} |
                    更改数: {{ queryResult.meta?.changes }}
                  </n-text>

                  <n-divider style="margin: 8px 0" />

                  <n-empty v-if="queryResult.results.length === 0" description="查询无结果" size="small" />

                  <n-data-table
                    v-else
                    :columns="resultColumns"
                    :data="queryResult.results"
                    :max-height="400"
                    :scroll-x="1200"
                    size="small"
                  />
                </n-space>
              </n-card>
            </n-space>
          </n-tab-pane>

          <n-tab-pane name="common" tab="常用查询">
            <n-space vertical :size="12">
              <n-text depth="2">快速选择常用 SQL 语句：</n-text>

              <n-list bordered>
                <n-list-item v-for="(query, index) in commonQueries" :key="index">
                  <n-space justify="space-between" style="width: 100%">
                    <n-space vertical size="small">
                      <n-text strong>{{ query.name }}</n-text>
                      <n-text depth="3" style="font-size: 12px">{{ query.description }}</n-text>
                      <n-text code style="font-size: 11px">{{ query.sql }}</n-text>
                    </n-space>
                    <n-button size="small" @click="queryText = query.sql">
                      使用
                    </n-button>
                  </n-space>
                </n-list-item>
              </n-list>
            </n-space>
          </n-tab-pane>
        </n-tabs>
      </n-space>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useMessage, useDialog } from 'naive-ui'
import {
  AddOutline,
  CodeSlashOutline,
  PlayOutline,
  EllipsisVerticalOutline
} from '@vicons/ionicons5'
import { cloudflareApi, type D1Database, type D1QueryResult } from '@/api'
import { useAccountStore } from '@/stores/account'
import type { DataTableColumns } from 'naive-ui'

const message = useMessage()
const dialog = useDialog()
const accountStore = useAccountStore()

const loadingDatabases = ref(false)
const databases = ref<D1Database[]>([])

const showCreateModal = ref(false)
const creating = ref(false)
const createForm = ref({
  name: ''
})

const showQueryModal = ref(false)
const currentDatabase = ref<D1Database | null>(null)
const queryText = ref('')
const executing = ref(false)
const queryResult = ref<D1QueryResult | null>(null)

// 常用查询
const commonQueries = [
  {
    name: '创建用户表',
    description: '创建一个包含 id、name、email 的用户表',
    sql: 'CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, email TEXT UNIQUE, created_at DATETIME DEFAULT CURRENT_TIMESTAMP);'
  },
  {
    name: '查询所有表',
    description: '列出数据库中的所有表',
    sql: "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name;"
  },
  {
    name: '查询表结构',
    description: '查看指定表的列信息（修改 users 为你的表名）',
    sql: 'PRAGMA table_info(users);'
  },
  {
    name: '插入示例数据',
    description: '向 users 表插入一条记录',
    sql: "INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com');"
  },
  {
    name: '查询前10条',
    description: '查询 users 表的前 10 条记录',
    sql: 'SELECT * FROM users LIMIT 10;'
  }
]

// 动态生成结果表格列
const resultColumns = computed<DataTableColumns<any>>(() => {
  if (!queryResult.value || queryResult.value.results.length === 0) {
    return []
  }

  const firstRow = queryResult.value.results[0]
  return Object.keys(firstRow).map(key => ({
    title: key,
    key: key,
    ellipsis: {
      tooltip: true
    }
  }))
})

// 获取当前账户 ID
function getCurrentAccountId(): string | null {
  const account = accountStore.currentAccount
  if (!account?.accountId) {
    message.error('未找到 Cloudflare 账户 ID，请先验证账户')
    return null
  }
  return account.accountId
}

async function loadDatabases() {
  const accountId = getCurrentAccountId()
  if (!accountId) return

  loadingDatabases.value = true
  try {
    databases.value = await cloudflareApi.listD1Databases(accountId)
  } catch (error: any) {
    console.error('Load D1 databases error:', error)
    message.error(error?.message || '加载 D1 数据库失败')
  } finally {
    loadingDatabases.value = false
  }
}

function handleCreateDatabase() {
  createForm.value = { name: '' }
  showCreateModal.value = true
}

async function handleConfirmCreate() {
  const accountId = getCurrentAccountId()
  if (!accountId) return

  if (!createForm.value.name) {
    message.warning('请输入数据库名称')
    return
  }

  creating.value = true
  try {
    await cloudflareApi.createD1Database({
      account_id: accountId,
      name: createForm.value.name
    })
    message.success('数据库创建成功')
    showCreateModal.value = false
    await loadDatabases()
  } catch (error: any) {
    message.error(error?.message || '创建失败')
  } finally {
    creating.value = false
  }
}

function getDatabaseActions(database: D1Database) {
  return [
    {
      label: '删除数据库',
      key: 'delete',
      props: {
        style: { color: 'red' }
      }
    }
  ]
}

function handleDatabaseAction(key: string, database: D1Database) {
  if (key === 'delete') {
    handleDeleteDatabase(database)
  }
}

function handleDeleteDatabase(database: D1Database) {
  const accountId = getCurrentAccountId()
  if (!accountId) return

  dialog.warning({
    title: '确认删除',
    content: `确定要删除数据库 "${database.name}" 吗？此操作将删除数据库中的所有数据，且无法撤销。`,
    positiveText: '删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await cloudflareApi.deleteD1Database(accountId, database.uuid)
        message.success('数据库已删除')
        await loadDatabases()
      } catch (error: any) {
        message.error(error?.message || '删除失败')
      }
    }
  })
}

function handleOpenQuery(database: D1Database) {
  currentDatabase.value = database
  queryText.value = ''
  queryResult.value = null
  showQueryModal.value = true
}

async function handleExecuteQuery() {
  if (!currentDatabase.value) return

  const accountId = getCurrentAccountId()
  if (!accountId) return

  if (!queryText.value.trim()) {
    message.warning('请输入 SQL 查询语句')
    return
  }

  executing.value = true
  try {
    queryResult.value = await cloudflareApi.executeD1Query({
      account_id: accountId,
      database_id: currentDatabase.value.uuid,
      query: queryText.value
    })

    if (queryResult.value.success) {
      message.success('查询执行成功')
    } else {
      message.error('查询执行失败')
    }
  } catch (error: any) {
    message.error(error?.message || '查询失败')
    queryResult.value = null
  } finally {
    executing.value = false
  }
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleString('zh-CN')
}

onMounted(() => {
  loadDatabases()
})
</script>
