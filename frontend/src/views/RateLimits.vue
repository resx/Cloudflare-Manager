<template>
  <n-space vertical :size="24">
    <n-card title="速率限制规则">
      <template #header-extra>
        <n-space>
          <n-button @click="loadRateLimits" :loading="loading">
            <template #icon>
              <n-icon><RefreshOutline /></n-icon>
            </template>
            刷新
          </n-button>
          <n-button type="primary" @click="showCreateModal = true">
            <template #icon>
              <n-icon><AddOutline /></n-icon>
            </template>
            创建规则
          </n-button>
        </n-space>
      </template>

      <n-alert type="info" style="margin-bottom: 16px">
        速率限制可以防止恶意流量和 DDoS 攻击。根据 URL 模式和阈值限制请求频率。
        免费计划每个域名最多 1 条规则。
      </n-alert>

      <n-spin :show="loading">
        <n-data-table
          v-if="rateLimits.length > 0"
          :columns="columns"
          :data="rateLimits"
          :pagination="false"
          :bordered="false"
        />
        <n-empty v-else description="暂无速率限制规则" />
      </n-spin>
    </n-card>

    <!-- 创建/编辑规则模态框 -->
    <n-modal
      v-model:show="showCreateModal"
      preset="card"
      :title="editingRule ? '编辑速率限制规则' : '创建速率限制规则'"
      style="width: 700px"
      :bordered="false"
      :segmented="{
        content: 'soft',
        footer: 'soft'
      }"
    >
      <n-form
        ref="formRef"
        :model="ruleForm"
        label-placement="left"
        label-width="120"
        require-mark-placement="left"
      >
        <n-form-item label="规则描述" path="description" required>
          <n-input
            v-model:value="ruleForm.description"
            placeholder="例如: 限制登录接口访问频率"
          />
        </n-form-item>

        <n-form-item label="URL 模式" path="url" required>
          <n-input
            v-model:value="ruleForm.url"
            placeholder="例如: example.com/api/login"
          />
          <template #feedback>
            <n-text depth="3" style="font-size: 12px">
              支持通配符，如 *.example.com/api/*
            </n-text>
          </template>
        </n-form-item>

        <n-form-item label="HTTP 方法" path="methods">
          <n-select
            v-model:value="ruleForm.methods"
            :options="methodOptions"
            multiple
            placeholder="选择 HTTP 方法（留空表示所有）"
          />
        </n-form-item>

        <n-form-item label="协议" path="schemes">
          <n-select
            v-model:value="ruleForm.schemes"
            :options="schemeOptions"
            multiple
            placeholder="选择协议（留空表示所有）"
          />
        </n-form-item>

        <n-divider>限制条件</n-divider>

        <n-form-item label="阈值" path="threshold" required>
          <n-input-number
            v-model:value="ruleForm.threshold"
            :min="1"
            :max="1000000"
            style="width: 100%"
          >
            <template #suffix>次请求</template>
          </n-input-number>
        </n-form-item>

        <n-form-item label="时间窗口" path="period" required>
          <n-input-number
            v-model:value="ruleForm.period"
            :min="10"
            :max="86400"
            style="width: 100%"
          >
            <template #suffix>秒</template>
          </n-input-number>
          <template #feedback>
            <n-text depth="3" style="font-size: 12px">
              在时间窗口内超过阈值将触发限制动作
            </n-text>
          </template>
        </n-form-item>

        <n-divider>动作设置</n-divider>

        <n-form-item label="动作类型" path="actionMode" required>
          <n-select
            v-model:value="ruleForm.actionMode"
            :options="actionModeOptions"
          />
        </n-form-item>

        <n-form-item
          v-if="ruleForm.actionMode === 'ban'"
          label="封禁时长"
          path="timeout"
        >
          <n-input-number
            v-model:value="ruleForm.timeout"
            :min="10"
            :max="86400"
            style="width: 100%"
          >
            <template #suffix>秒</template>
          </n-input-number>
        </n-form-item>

        <n-form-item label="启用规则" path="disabled">
          <n-switch v-model:value="ruleForm.enabled">
            <template #checked>已启用</template>
            <template #unchecked>已禁用</template>
          </n-switch>
        </n-form-item>
      </n-form>

      <template #footer>
        <n-space justify="end">
          <n-button @click="handleCancel">取消</n-button>
          <n-button type="primary" :loading="creating" @click="handleSubmit">
            {{ editingRule ? '更新' : '创建' }}
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, inject, watch, computed, h, type Ref } from 'vue'
import { NButton, NSpace, NTag, NIcon, NSwitch, useMessage, useDialog } from 'naive-ui'
import { RefreshOutline, AddOutline, TrashOutline, CreateOutline } from '@vicons/ionicons5'
import { cloudflareApi, type Zone, type RateLimit, type CreateRateLimitRequest } from '@/api'

const message = useMessage()
const dialog = useDialog()

const loading = ref(false)
const creating = ref(false)
const showCreateModal = ref(false)
const rateLimits = ref<RateLimit[]>([])
const editingRule = ref<RateLimit | null>(null)

// 从 Layout 获取当前域名
const currentZone = inject<Ref<Zone | null>>('currentZone')

// 表单数据
const ruleForm = ref({
  description: '',
  url: '',
  methods: [] as string[],
  schemes: [] as string[],
  threshold: 100,
  period: 60,
  actionMode: 'simulate',
  timeout: 3600,
  enabled: true
})

const methodOptions = [
  { label: 'GET', value: 'GET' },
  { label: 'POST', value: 'POST' },
  { label: 'PUT', value: 'PUT' },
  { label: 'DELETE', value: 'DELETE' },
  { label: 'PATCH', value: 'PATCH' },
  { label: 'HEAD', value: 'HEAD' },
  { label: 'OPTIONS', value: 'OPTIONS' }
]

const schemeOptions = [
  { label: 'HTTP', value: 'HTTP' },
  { label: 'HTTPS', value: 'HTTPS' }
]

const actionModeOptions = [
  { label: '模拟 (记录但不阻止)', value: 'simulate' },
  { label: '封禁', value: 'ban' },
  { label: 'CAPTCHA 质询', value: 'challenge' },
  { label: 'JavaScript 质询', value: 'js_challenge' }
]

// 表格列
const columns = computed(() => [
  {
    title: '描述',
    key: 'description',
    width: 200,
    ellipsis: {
      tooltip: true
    }
  },
  {
    title: 'URL 模式',
    key: 'match_request',
    width: 250,
    render: (row: RateLimit) => row.match_request.url
  },
  {
    title: '限制条件',
    key: 'threshold',
    width: 150,
    render: (row: RateLimit) => `${row.threshold} 次 / ${row.period} 秒`
  },
  {
    title: '动作',
    key: 'action',
    width: 120,
    render: (row: RateLimit) => {
      const modeMap: Record<string, string> = {
        simulate: '模拟',
        ban: '封禁',
        challenge: 'CAPTCHA',
        js_challenge: 'JS 质询'
      }
      const type = row.action.mode === 'simulate' ? 'warning'
                 : row.action.mode === 'ban' ? 'error'
                 : 'info'
      return h(NTag, { type, size: 'small' }, {
        default: () => modeMap[row.action.mode] || row.action.mode
      })
    }
  },
  {
    title: '状态',
    key: 'disabled',
    width: 100,
    render: (row: RateLimit) => {
      return h(NSwitch, {
        value: !row.disabled,
        onUpdateValue: (val: boolean) => handleToggle(row, val)
      })
    }
  },
  {
    title: '操作',
    key: 'actions',
    width: 150,
    render: (row: RateLimit) => {
      return h(NSpace, {}, {
        default: () => [
          h(
            NButton,
            {
              size: 'small',
              onClick: () => handleEdit(row)
            },
            {
              default: () => '编辑',
              icon: () => h(NIcon, {}, { default: () => h(CreateOutline) })
            }
          ),
          h(
            NButton,
            {
              size: 'small',
              type: 'error',
              onClick: () => handleDelete(row.id)
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

async function loadRateLimits() {
  if (!currentZone?.value?.id) {
    return
  }

  loading.value = true
  try {
    rateLimits.value = await cloudflareApi.getRateLimits(currentZone.value.id)
    message.success(`成功加载 ${rateLimits.value.length} 条速率限制规则`)
  } catch (error: any) {
    message.warning(error?.message || '加载速率限制规则失败')
    console.error('Load rate limits error:', error)
    rateLimits.value = []
  } finally {
    loading.value = false
  }
}

function handleEdit(rule: RateLimit) {
  editingRule.value = rule
  ruleForm.value = {
    description: rule.description,
    url: rule.match_request.url,
    methods: rule.match_request.methods || [],
    schemes: rule.match_request.schemes || [],
    threshold: rule.threshold,
    period: rule.period,
    actionMode: rule.action.mode,
    timeout: rule.action.timeout || 3600,
    enabled: !rule.disabled
  }
  showCreateModal.value = true
}

function handleCancel() {
  showCreateModal.value = false
  editingRule.value = null
  ruleForm.value = {
    description: '',
    url: '',
    methods: [],
    schemes: [],
    threshold: 100,
    period: 60,
    actionMode: 'simulate',
    timeout: 3600,
    enabled: true
  }
}

async function handleSubmit() {
  if (!currentZone?.value?.id) {
    message.warning('请先选择一个域名')
    return
  }

  if (!ruleForm.value.description || !ruleForm.value.url) {
    message.warning('请填写所有必填项')
    return
  }

  creating.value = true
  try {
    const request: CreateRateLimitRequest = {
      zone_id: currentZone.value.id,
      disabled: !ruleForm.value.enabled,
      description: ruleForm.value.description,
      match_request: {
        url: ruleForm.value.url,
        methods: ruleForm.value.methods.length > 0 ? ruleForm.value.methods : undefined,
        schemes: ruleForm.value.schemes.length > 0 ? ruleForm.value.schemes : undefined
      },
      threshold: ruleForm.value.threshold,
      period: ruleForm.value.period,
      action: {
        mode: ruleForm.value.actionMode,
        timeout: ruleForm.value.actionMode === 'ban' ? ruleForm.value.timeout : undefined
      }
    }

    if (editingRule.value) {
      await cloudflareApi.updateRateLimit(currentZone.value.id, editingRule.value.id, request)
      message.success('速率限制规则已更新')
    } else {
      await cloudflareApi.createRateLimit(request)
      message.success('速率限制规则已创建')
    }

    handleCancel()
    await loadRateLimits()
  } catch (error: any) {
    message.error(error?.message || '操作失败')
    console.error('Create/update rate limit error:', error)
  } finally {
    creating.value = false
  }
}

async function handleToggle(rule: RateLimit, enabled: boolean) {
  if (!currentZone?.value?.id) {
    message.warning('请先选择一个域名')
    return
  }

  try {
    const request: CreateRateLimitRequest = {
      zone_id: currentZone.value.id,
      disabled: !enabled,
      description: rule.description,
      match_request: rule.match_request,
      threshold: rule.threshold,
      period: rule.period,
      action: rule.action
    }

    await cloudflareApi.updateRateLimit(currentZone.value.id, rule.id, request)
    rule.disabled = !enabled
    message.success(`规则已${enabled ? '启用' : '禁用'}`)
  } catch (error: any) {
    message.error(error?.message || '更新规则状态失败')
    console.error('Toggle rate limit error:', error)
  }
}

function handleDelete(ruleId: string) {
  dialog.warning({
    title: '确认删除',
    content: '确定要删除此速率限制规则吗？此操作不可撤销。',
    positiveText: '删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      if (!currentZone?.value?.id) {
        message.warning('请先选择一个域名')
        return
      }

      try {
        await cloudflareApi.deleteRateLimit(currentZone.value.id, ruleId)
        message.success('速率限制规则已删除')
        await loadRateLimits()
      } catch (error: any) {
        message.error(error?.message || '删除规则失败')
        console.error('Delete rate limit error:', error)
      }
    }
  })
}

onMounted(() => {
  loadRateLimits()
})

// 监听 currentZone 变化
watch(() => currentZone?.value?.id, () => {
  rateLimits.value = []
  loadRateLimits()
})
</script>
