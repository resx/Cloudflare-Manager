<template>
  <n-space vertical :size="24">
    <!-- Workers 列表 -->
    <n-card title="Workers 脚本管理">
      <template #header-extra>
        <n-space>
          <n-button @click="loadWorkers">
            <template #icon>
              <n-icon><RefreshOutline /></n-icon>
            </template>
            刷新
          </n-button>
          <n-button type="primary" @click="handleCreateWorker">
            <template #icon>
              <n-icon><AddOutline /></n-icon>
            </template>
            上传 Worker
          </n-button>
        </n-space>
      </template>

      <n-alert v-if="!currentAccount?.accountId" type="warning" style="margin-bottom: 16px">
        请先添加账户，系统会自动获取 Account ID
      </n-alert>

      <n-spin :show="loading">
        <n-empty v-if="workers.length === 0" description="暂无 Worker 脚本">
          <template #extra>
            <n-button size="small" @click="handleCreateWorker">
              上传第一个 Worker
            </n-button>
          </template>
        </n-empty>

        <n-grid v-else :cols="2" :x-gap="16" :y-gap="16">
          <n-grid-item v-for="worker in workers" :key="worker.id">
            <n-card
              :title="worker.id"
              hoverable
              :segmented="{ content: true, footer: 'soft' }"
            >
              <template #header-extra>
                <n-dropdown :options="getWorkerActions(worker)" @select="(key) => handleWorkerAction(key, worker)">
                  <n-button size="small" circle quaternary>
                    <template #icon>
                      <n-icon><EllipsisVerticalOutline /></n-icon>
                    </template>
                  </n-button>
                </n-dropdown>
              </template>

              <n-space vertical size="small">
                <n-descriptions :column="1" size="small">
                  <n-descriptions-item v-if="worker.etag" label="ETag">
                    <n-text code style="font-size: 11px">{{ worker.etag?.substring(0, 16) }}...</n-text>
                  </n-descriptions-item>
                  <n-descriptions-item v-if="worker.created_on" label="创建时间">
                    {{ formatDate(worker.created_on) }}
                  </n-descriptions-item>
                  <n-descriptions-item v-if="worker.modified_on" label="修改时间">
                    {{ formatDate(worker.modified_on) }}
                  </n-descriptions-item>
                </n-descriptions>

                <!-- 显示绑定的路由 -->
                <n-divider style="margin: 8px 0" />
                <n-space vertical size="small">
                  <n-text depth="3" style="font-size: 12px">绑定路由:</n-text>
                  <n-space v-if="getWorkerRoutes(worker.id).length > 0" vertical size="small">
                    <n-tag v-for="route in getWorkerRoutes(worker.id)" :key="route.id" size="small" type="info">
                      {{ route.pattern }}
                    </n-tag>
                  </n-space>
                  <n-text v-else depth="3" style="font-size: 12px">未绑定任何路由</n-text>
                </n-space>
              </n-space>

              <template #footer>
                <n-space justify="space-between">
                  <n-button size="small" @click="handleViewScript(worker)">
                    <template #icon>
                      <n-icon><CodeSlashOutline /></n-icon>
                    </template>
                    查看代码
                  </n-button>
                  <n-button size="small" @click="handleEditWorker(worker)">
                    <template #icon>
                      <n-icon><CreateOutline /></n-icon>
                    </template>
                    编辑
                  </n-button>
                </n-space>
              </template>
            </n-card>
          </n-grid-item>
        </n-grid>
      </n-spin>
    </n-card>

    <!-- Worker 路由 -->
    <n-card v-if="currentZone" title="Worker 路由绑定">
      <template #header-extra>
        <n-button type="primary" @click="handleCreateRoute">
          <template #icon>
            <n-icon><AddOutline /></n-icon>
          </template>
          添加路由
        </n-button>
      </template>

      <n-alert type="info" style="margin-bottom: 16px">
        为当前域名 <strong>{{ currentZone.name }}</strong> 配置 Worker 路由，使 Worker 能够处理特定URL的请求。
      </n-alert>

      <n-spin :show="routesLoading">
        <n-list v-if="routes.length > 0" bordered>
          <n-list-item v-for="route in routes" :key="route.id">
            <n-space align="center" justify="space-between" style="width: 100%">
              <n-space vertical size="small">
                <n-text strong>{{ route.pattern }}</n-text>
                <n-tag v-if="route.script" type="success" size="small">
                  Worker: {{ route.script }}
                </n-tag>
                <n-tag v-else type="default" size="small">
                  未绑定 Worker
                </n-tag>
              </n-space>
              <n-button size="small" type="error" @click="handleDeleteRoute(route)">
                <template #icon>
                  <n-icon><TrashOutline /></n-icon>
                </template>
                删除
              </n-button>
            </n-space>
          </n-list-item>
        </n-list>
        <n-empty v-else description="暂无路由配置" />
      </n-spin>
    </n-card>

    <!-- 上传/编辑 Worker 弹窗 -->
    <n-modal v-model:show="showWorkerModal" preset="card" :title="editingWorker ? '编辑 Worker' : '上传 Worker'" style="width: 900px">
      <n-form ref="workerFormRef" :model="workerForm" label-placement="left" label-width="120">
        <n-form-item label="Worker 名称" required>
          <n-input
            v-model:value="workerForm.name"
            placeholder="例如: my-worker"
            :disabled="!!editingWorker"
          />
          <template #feedback>
            <n-text depth="3" style="font-size: 12px">
              Worker 名称只能包含字母、数字、连字符和下划线
            </n-text>
          </template>
        </n-form-item>

        <n-form-item label="Worker 代码" required>
          <n-input
            v-model:value="workerForm.script"
            type="textarea"
            placeholder="在此输入或粘贴 Worker JavaScript 代码"
            :rows="20"
            :input-props="{ spellcheck: false }"
          />
        </n-form-item>

        <n-collapse style="margin-top: 16px">
          <n-collapse-item title="代码示例（点击展开）" name="examples">
            <n-tabs type="segment">
              <n-tab-pane name="hello" tab="Hello World">
                <n-code :code="exampleHelloWorld" language="javascript" :show-line-numbers="true" />
                <n-button size="small" style="margin-top: 8px" @click="workerForm.script = exampleHelloWorld">
                  使用此示例
                </n-button>
              </n-tab-pane>
              <n-tab-pane name="proxy" tab="反向代理">
                <n-code :code="exampleProxy" language="javascript" :show-line-numbers="true" />
                <n-button size="small" style="margin-top: 8px" @click="workerForm.script = exampleProxy">
                  使用此示例
                </n-button>
              </n-tab-pane>
            </n-tabs>
          </n-collapse-item>
        </n-collapse>
      </n-form>

      <template #footer>
        <n-space justify="end">
          <n-button @click="showWorkerModal = false">取消</n-button>
          <n-button type="primary" :loading="uploading" @click="handleConfirmWorker">
            {{ editingWorker ? '保存' : '上传' }}
          </n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 查看代码弹窗 -->
    <n-modal v-model:show="showScriptModal" preset="card" :title="`Worker 代码 - ${viewingWorker?.id}`" style="width: 900px">
      <n-spin :show="loadingScript">
        <n-code v-if="currentScript" :code="currentScript" language="javascript" :show-line-numbers="true" />
      </n-spin>

      <template #footer>
        <n-space justify="end">
          <n-button @click="showScriptModal = false">关闭</n-button>
          <n-button type="primary" @click="handleCopyScript">
            <template #icon>
              <n-icon><CopyOutline /></n-icon>
            </template>
            复制代码
          </n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 添加路由弹窗 -->
    <n-modal v-model:show="showRouteModal" preset="card" title="添加 Worker 路由" style="width: 600px">
      <n-form ref="routeFormRef" :model="routeForm" label-placement="left" label-width="120">
        <n-form-item label="路由模式" required>
          <n-input
            v-model:value="routeForm.pattern"
            placeholder="例如: *example.com/* 或 example.com/api/*"
          />
          <template #feedback>
            <n-text depth="3" style="font-size: 12px">
              使用 * 作为通配符匹配任意字符
            </n-text>
          </template>
        </n-form-item>

        <n-form-item label="Worker 脚本" required>
          <n-select
            v-model:value="routeForm.scriptName"
            :options="workerOptions"
            placeholder="选择要绑定的 Worker"
          />
        </n-form-item>
      </n-form>

      <template #footer>
        <n-space justify="end">
          <n-button @click="showRouteModal = false">取消</n-button>
          <n-button type="primary" :loading="routeCreating" @click="handleConfirmRoute">
            创建
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, inject, watch, type Ref } from 'vue'
import { useMessage, useDialog } from 'naive-ui'
import {
  RefreshOutline,
  AddOutline,
  CodeSlashOutline,
  CreateOutline,
  TrashOutline,
  CopyOutline,
  EllipsisVerticalOutline
} from '@vicons/ionicons5'
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

const showWorkerModal = ref(false)
const showScriptModal = ref(false)
const showRouteModal = ref(false)

const editingWorker = ref<Worker | null>(null)
const viewingWorker = ref<Worker | null>(null)
const uploading = ref(false)
const loadingScript = ref(false)
const currentScript = ref('')
const routeCreating = ref(false)

const workerForm = ref({
  name: '',
  script: ''
})

const routeForm = ref({
  pattern: '',
  scriptName: ''
})

// 示例代码
const exampleHelloWorld = `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  return new Response('Hello World!', {
    headers: { 'content-type': 'text/plain' }
  })
}`

const exampleProxy = `addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
  const originUrl = 'https://example.com' // 修改为你的源站地址
  const url = new URL(request.url)

  // 修改URL指向源站
  url.hostname = new URL(originUrl).hostname
  url.protocol = new URL(originUrl).protocol

  // 转发请求
  const modifiedRequest = new Request(url.toString(), {
    method: request.method,
    headers: request.headers,
    body: request.body
  })

  try {
    return await fetch(modifiedRequest)
  } catch (error) {
    return new Response('源站无法访问', { status: 502 })
  }
}`

// 获取当前账户
const currentAccount = computed(() => accountStore.currentAccount)

// Worker 选项
const workerOptions = computed(() => {
  return workers.value.map(w => ({
    label: w.id,
    value: w.id
  }))
})

// 获取 Worker 的路由
function getWorkerRoutes(workerId: string): WorkerRoute[] {
  return routes.value.filter(r => r.script === workerId)
}

function getWorkerActions(worker: Worker) {
  return [
    {
      label: '删除 Worker',
      key: 'delete',
      props: {
        style: { color: 'red' }
      }
    }
  ]
}

function handleWorkerAction(key: string, worker: Worker) {
  if (key === 'delete') {
    handleDeleteWorker(worker)
  }
}

async function loadWorkers() {
  if (!currentAccount.value?.accountId) {
    message.warning('请先添加账户')
    return
  }

  loading.value = true
  try {
    workers.value = await cloudflareApi.listWorkers(currentAccount.value.accountId)
  } catch (error: any) {
    console.error('Load workers error:', error)
    message.error(error?.message || '加载 Workers 失败')
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
    console.error('Load routes error:', error)
    message.error(error?.message || '加载路由失败')
  } finally {
    routesLoading.value = false
  }
}

function handleCreateWorker() {
  editingWorker.value = null
  workerForm.value = {
    name: '',
    script: ''
  }
  showWorkerModal.value = true
}

function handleEditWorker(worker: Worker) {
  editingWorker.value = worker
  workerForm.value = {
    name: worker.id,
    script: ''
  }
  showWorkerModal.value = true

  // 加载 Worker 代码
  loadWorkerScript(worker.id)
}

async function loadWorkerScript(scriptName: string) {
  if (!currentAccount.value?.accountId) return

  uploading.value = true
  try {
    const script = await cloudflareApi.getWorker(currentAccount.value.accountId, scriptName)
    workerForm.value.script = script
  } catch (error: any) {
    message.error(error?.message || '加载 Worker 代码失败')
  } finally {
    uploading.value = false
  }
}

async function handleConfirmWorker() {
  if (!currentAccount.value?.accountId) {
    message.error('未找到账户 ID')
    return
  }

  if (!workerForm.value.name) {
    message.warning('请输入 Worker 名称')
    return
  }

  if (!workerForm.value.script) {
    message.warning('请输入 Worker 代码')
    return
  }

  uploading.value = true
  try {
    // Cloudflare API 需要通过 PUT 请求上传 Worker 脚本
    // 由于当前API结构限制，我们需要添加上传Worker的API
    message.warning('Worker 上传功能需要后端支持，正在开发中...')
    // TODO: 调用上传 Worker API
  } catch (error: any) {
    message.error(error?.message || '操作失败')
  } finally {
    uploading.value = false
  }
}

async function handleViewScript(worker: Worker) {
  if (!currentAccount.value?.accountId) {
    message.warning('请先添加账户')
    return
  }

  viewingWorker.value = worker
  showScriptModal.value = true
  loadingScript.value = true
  currentScript.value = ''

  try {
    currentScript.value = await cloudflareApi.getWorker(
      currentAccount.value.accountId,
      worker.id
    )
  } catch (error: any) {
    message.error(error?.message || '获取 Worker 脚本失败')
    currentScript.value = '加载失败'
  } finally {
    loadingScript.value = false
  }
}

function handleDeleteWorker(worker: Worker) {
  dialog.warning({
    title: '确认删除',
    content: `确定要删除 Worker "${worker.id}" 吗？此操作将同时解除所有路由绑定，且无法撤销。`,
    positiveText: '删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      if (!currentAccount.value?.accountId) return

      try {
        await cloudflareApi.deleteWorker(currentAccount.value.accountId, worker.id)
        message.success('Worker 已删除')
        await loadWorkers()
        if (currentZone?.value) {
          await loadRoutes()
        }
      } catch (error: any) {
        message.error(error?.message || '删除失败')
      }
    }
  })
}

function handleCreateRoute() {
  if (workers.value.length === 0) {
    message.warning('请先上传 Worker 脚本')
    return
  }

  routeForm.value = {
    pattern: currentZone.value ? `${currentZone.value.name}/*` : '',
    scriptName: ''
  }
  showRouteModal.value = true
}

async function handleConfirmRoute() {
  if (!currentZone?.value?.id) {
    message.warning('请先选择域名')
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
    await loadRoutes()
  } catch (error: any) {
    message.error(error?.message || '创建路由失败')
  } finally {
    routeCreating.value = false
  }
}

function handleDeleteRoute(route: WorkerRoute) {
  dialog.warning({
    title: '确认删除',
    content: `确定要删除路由 "${route.pattern}" 吗？此操作无法撤销。`,
    positiveText: '删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      if (!currentZone?.value?.id) return

      try {
        await cloudflareApi.deleteWorkerRoute(currentZone.value.id, route.id)
        message.success('路由已删除')
        await loadRoutes()
      } catch (error: any) {
        message.error(error?.message || '删除失败')
      }
    }
  })
}

async function handleCopyScript() {
  try {
    await navigator.clipboard.writeText(currentScript.value)
    message.success('已复制到剪贴板')
  } catch (error) {
    message.error('复制失败')
  }
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleString('zh-CN')
}

onMounted(() => {
  loadWorkers()
  if (currentZone?.value) {
    loadRoutes()
  }
})

// 监听域名变化
watch(() => currentZone?.value?.id, () => {
  if (currentZone?.value) {
    loadRoutes()
  }
})
</script>
