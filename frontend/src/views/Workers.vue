<template>
  <n-space vertical :size="24">
    <n-card title="Workers 管理">
      <template #header-extra>
        <n-button type="primary" @click="loadWorkers">
          <template #icon>
            <n-icon><RefreshOutline /></n-icon>
          </template>
          刷新列表
        </n-button>
      </template>

      <n-alert v-if="!currentAccount" type="warning" style="margin-bottom: 24px">
        请先配置账户信息并确保填写了 Account ID
      </n-alert>

      <n-spin :show="loading">
        <n-data-table
          :columns="workerColumns"
          :data="workers"
          :pagination="false"
          :bordered="false"
        />
      </n-spin>
    </n-card>

    <n-card v-if="currentZone" title="Worker 路由">
      <template #header-extra>
        <n-button type="primary" @click="showRouteModal = true">
          <template #icon>
            <n-icon><AddOutline /></n-icon>
          </template>
          添加路由
        </n-button>
      </template>

      <n-spin :show="routesLoading">
        <n-data-table
          :columns="routeColumns"
          :data="routes"
          :pagination="false"
          :bordered="false"
        />
      </n-spin>
    </n-card>

    <!-- 查看 Worker 脚本模态框 -->
    <n-modal
      v-model:show="showScriptModal"
      preset="card"
      title="Worker 脚本内容"
      style="width: 800px"
      :bordered="false"
      :segmented="{
        content: 'soft',
        footer: 'soft'
      }"
    >
      <n-code :code="currentScript" language="javascript" word-wrap />
    </n-modal>

    <!-- 添加路由模态框 -->
    <n-modal
      v-model:show="showRouteModal"
      preset="card"
      title="添加 Worker 路由"
      style="width: 600px"
      :bordered="false"
      :segmented="{
        content: 'soft',
        footer: 'soft'
      }"
    >
      <n-form
        ref="routeFormRef"
        :model="routeForm"
        label-placement="left"
        label-width="100"
      >
        <n-form-item label="路由模式" path="pattern">
          <n-input
            v-model:value="routeForm.pattern"
            placeholder="例如: *example.com/*"
          />
        </n-form-item>
        <n-form-item label="Worker 名称" path="scriptName">
          <n-select
            v-model:value="routeForm.scriptName"
            :options="workerOptions"
            placeholder="选择 Worker"
          />
        </n-form-item>
      </n-form>
      <template #footer>
        <n-space justify="end">
          <n-button @click="showRouteModal = false">取消</n-button>
          <n-button type="primary" :loading="routeCreating" @click="handleCreateRoute">
            创建
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, inject, computed, h, type Ref } from 'vue'
import { NButton, NSpace, NTag, NIcon, useMessage, useDialog } from 'naive-ui'
import { RefreshOutline, AddOutline, TrashOutline, CodeOutline } from '@vicons/ionicons5'
import { cloudflareApi, type Zone, type Worker, type WorkerRoute } from '@/api'
import { useAccountStore } from '@/stores/account'

const message = useMessage()
const dialog = useDialog()
const accountStore = useAccountStore()

// 从 Layout 获取当前域名
const currentZone = inject<Ref<Zone | null>>('currentZone')

const loading = ref(false)
const routesLoading = ref(false)
const workers = ref<Worker[]>([])
const routes = ref<WorkerRoute[]>([])
const showScriptModal = ref(false)
const showRouteModal = ref(false)
const currentScript = ref('')
const routeCreating = ref(false)

const routeForm = ref({
  pattern: '',
  scriptName: ''
})

// 获取当前账户
const currentAccount = computed(() => accountStore.currentAccount)

// Worker 选项
const workerOptions = computed(() => {
  return workers.value.map(w => ({
    label: w.id,
    value: w.id
  }))
})

// Worker 表格列
const workerColumns = computed(() => [
  {
    title: 'Worker 名称',
    key: 'id',
    width: 200
  },
  {
    title: 'ETag',
    key: 'etag',
    width: 150,
    render: (row: Worker) => row.etag || '-'
  },
  {
    title: '创建时间',
    key: 'created_on',
    width: 180,
    render: (row: Worker) => {
      if (!row.created_on) return '-'
      return new Date(row.created_on).toLocaleString('zh-CN')
    }
  },
  {
    title: '修改时间',
    key: 'modified_on',
    width: 180,
    render: (row: Worker) => {
      if (!row.modified_on) return '-'
      return new Date(row.modified_on).toLocaleString('zh-CN')
    }
  },
  {
    title: '操作',
    key: 'actions',
    width: 200,
    render: (row: Worker) => {
      return h(NSpace, {}, {
        default: () => [
          h(
            NButton,
            {
              size: 'small',
              onClick: () => handleViewScript(row.id)
            },
            {
              default: () => '查看脚本',
              icon: () => h(NIcon, {}, { default: () => h(CodeOutline) })
            }
          ),
          h(
            NButton,
            {
              size: 'small',
              type: 'error',
              onClick: () => handleDeleteWorker(row.id)
            },
            {
              default: () => '删除',
              icon: () => h(NIcon, {}, { default: () => h(TrashOutline) })
            }
          )
        ]
      })
    }
  }
])

// 路由表格列
const routeColumns = computed(() => [
  {
    title: 'ID',
    key: 'id',
    width: 150
  },
  {
    title: '路由模式',
    key: 'pattern',
    width: 300
  },
  {
    title: 'Worker',
    key: 'script',
    width: 200,
    render: (row: WorkerRoute) => {
      return row.script
        ? h(NTag, { type: 'success' }, { default: () => row.script })
        : h(NTag, { type: 'default' }, { default: () => '未绑定' })
    }
  },
  {
    title: '操作',
    key: 'actions',
    width: 150,
    render: (row: WorkerRoute) => {
      return h(
        NButton,
        {
          size: 'small',
          type: 'error',
          onClick: () => handleDeleteRoute(row.id)
        },
        {
          default: () => '删除',
          icon: () => h(NIcon, {}, { default: () => h(TrashOutline) })
        }
      )
    }
  }
])

async function loadWorkers() {
  if (!currentAccount.value?.accountId) {
    message.warning('请先在账户设置中配置 Account ID')
    return
  }

  loading.value = true
  try {
    workers.value = await cloudflareApi.listWorkers(currentAccount.value.accountId)
    message.success(`成功加载 ${workers.value.length} 个 Workers`)
  } catch (error: any) {
    message.error(error?.message || '加载 Workers 失败')
    console.error('Load workers error:', error)
  } finally {
    loading.value = false
  }
}

async function loadRoutes() {
  if (!currentZone?.value?.id) {
    return
  }

  routesLoading.value = true
  try {
    routes.value = await cloudflareApi.getWorkerRoutes(currentZone.value.id)
  } catch (error: any) {
    message.error(error?.message || '加载路由失败')
    console.error('Load routes error:', error)
  } finally {
    routesLoading.value = false
  }
}

async function handleViewScript(scriptName: string) {
  if (!currentAccount.value?.accountId) {
    message.warning('请先在账户设置中配置 Account ID')
    return
  }

  try {
    currentScript.value = await cloudflareApi.getWorker(
      currentAccount.value.accountId,
      scriptName
    )
    showScriptModal.value = true
  } catch (error: any) {
    message.error(error?.message || '获取 Worker 脚本失败')
    console.error('Get worker error:', error)
  }
}

function handleDeleteWorker(scriptName: string) {
  dialog.warning({
    title: '确认删除',
    content: `确定要删除 Worker "${scriptName}" 吗？此操作不可撤销。`,
    positiveText: '删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      if (!currentAccount.value?.accountId) {
        message.warning('请先在账户设置中配置 Account ID')
        return
      }

      try {
        await cloudflareApi.deleteWorker(currentAccount.value.accountId, scriptName)
        message.success('Worker 删除成功')
        await loadWorkers()
      } catch (error: any) {
        message.error(error?.message || '删除 Worker 失败')
        console.error('Delete worker error:', error)
      }
    }
  })
}

async function handleCreateRoute() {
  if (!currentZone?.value?.id) {
    message.warning('请先选择一个域名')
    return
  }

  if (!routeForm.value.pattern || !routeForm.value.scriptName) {
    message.warning('请填写所有必填项')
    return
  }

  routeCreating.value = true
  try {
    await cloudflareApi.createWorkerRoute(
      currentZone.value.id,
      routeForm.value.pattern,
      routeForm.value.scriptName
    )
    message.success('路由创建成功')
    showRouteModal.value = false
    routeForm.value = { pattern: '', scriptName: '' }
    await loadRoutes()
  } catch (error: any) {
    message.error(error?.message || '创建路由失败')
    console.error('Create route error:', error)
  } finally {
    routeCreating.value = false
  }
}

function handleDeleteRoute(routeId: string) {
  dialog.warning({
    title: '确认删除',
    content: '确定要删除此路由吗？此操作不可撤销。',
    positiveText: '删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      if (!currentZone?.value?.id) {
        message.warning('请先选择一个域名')
        return
      }

      try {
        await cloudflareApi.deleteWorkerRoute(currentZone.value.id, routeId)
        message.success('路由删除成功')
        await loadRoutes()
      } catch (error: any) {
        message.error(error?.message || '删除路由失败')
        console.error('Delete route error:', error)
      }
    }
  })
}

onMounted(() => {
  loadWorkers()
  if (currentZone?.value) {
    loadRoutes()
  }
})
</script>
