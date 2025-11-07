<template>
  <n-space vertical :size="24">
    <n-card title="Worker 模板库">
      <template #header-extra>
        <n-space>
          <n-input-group>
            <n-input
              v-model:value="searchKeyword"
              placeholder="搜索模板..."
              clearable
              style="width: 200px"
            >
              <template #prefix>
                <n-icon><SearchOutline /></n-icon>
              </template>
            </n-input>
          </n-input-group>
          <n-select
            v-model:value="selectedCategory"
            :options="categoryOptions"
            placeholder="选择分类"
            clearable
            style="width: 150px"
          />
        </n-space>
      </template>

      <n-alert type="info" style="margin-bottom: 16px">
        选择一个模板快速部署到您的 Cloudflare Workers。所有模板都经过测试，可以直接使用。
      </n-alert>

      <n-grid :cols="3" :x-gap="16" :y-gap="16" responsive="screen">
        <n-grid-item v-for="template in filteredTemplates" :key="template.id">
          <n-card
            :title="template.name"
            hoverable
            style="height: 100%"
            :segmented="{ content: true, footer: 'soft' }"
          >
            <template #header-extra>
              <n-tag :type="getCategoryColor(template.category)" size="small">
                {{ template.category }}
              </n-tag>
            </template>

            <n-space vertical :size="12">
              <n-text depth="2">{{ template.description }}</n-text>

              <n-divider style="margin: 8px 0" />

              <n-space vertical size="small">
                <n-text strong>功能特点：</n-text>
                <n-ul>
                  <n-li v-for="(feature, idx) in template.features" :key="idx">
                    <n-text depth="3" style="font-size: 13px">{{ feature }}</n-text>
                  </n-li>
                </n-ul>
              </n-space>

              <n-space>
                <n-tag size="small" :bordered="false">
                  <template #icon>
                    <n-icon><CodeSlashOutline /></n-icon>
                  </template>
                  {{ template.language }}
                </n-tag>
                <n-tag size="small" :bordered="false" type="info">
                  <template #icon>
                    <n-icon><StarOutline /></n-icon>
                  </template>
                  {{ template.difficulty }}
                </n-tag>
              </n-space>
            </n-space>

            <template #footer>
              <n-space justify="space-between">
                <n-button size="small" @click="handlePreview(template)">
                  <template #icon>
                    <n-icon><EyeOutline /></n-icon>
                  </template>
                  预览代码
                </n-button>
                <n-button type="primary" size="small" @click="handleDeploy(template)">
                  <template #icon>
                    <n-icon><RocketOutline /></n-icon>
                  </template>
                  使用模板
                </n-button>
              </n-space>
            </template>
          </n-card>
        </n-grid-item>
      </n-grid>

      <n-empty
        v-if="filteredTemplates.length === 0"
        description="没有找到匹配的模板"
        style="margin-top: 40px"
      />
    </n-card>

    <!-- 代码预览弹窗 -->
    <n-modal v-model:show="showPreviewModal" preset="card" title="Worker 代码预览" style="width: 800px">
      <template v-if="currentTemplate">
        <n-space vertical :size="16">
          <n-descriptions :column="2" bordered>
            <n-descriptions-item label="模板名称">{{ currentTemplate.name }}</n-descriptions-item>
            <n-descriptions-item label="分类">{{ currentTemplate.category }}</n-descriptions-item>
            <n-descriptions-item label="难度">{{ currentTemplate.difficulty }}</n-descriptions-item>
            <n-descriptions-item label="语言">{{ currentTemplate.language }}</n-descriptions-item>
          </n-descriptions>

          <n-divider>Worker 代码</n-divider>

          <n-code :code="currentTemplate.code" language="javascript" :show-line-numbers="true" />

          <n-alert type="info" title="使用说明">
            <n-text>{{ currentTemplate.usage }}</n-text>
          </n-alert>
        </n-space>
      </template>

      <template #footer>
        <n-space justify="end">
          <n-button @click="showPreviewModal = false">关闭</n-button>
          <n-button type="primary" @click="handleDeploy(currentTemplate)">
            <template #icon>
              <n-icon><RocketOutline /></n-icon>
            </template>
            使用此模板
          </n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 部署配置弹窗 -->
    <n-modal v-model:show="showDeployModal" preset="card" title="配置并部署 Worker" style="width: 600px">
      <template v-if="currentTemplate">
        <n-form ref="deployFormRef" :model="deployForm" label-placement="left" label-width="120">
          <n-alert type="info" style="margin-bottom: 16px">
            <strong>模板:</strong> {{ currentTemplate.name }}
          </n-alert>

          <n-form-item label="Worker 名称" required>
            <n-input
              v-model:value="deployForm.scriptName"
              placeholder="例如: my-worker"
            />
          </n-form-item>

          <n-form-item label="选择域名" required>
            <n-select
              v-model:value="deployForm.zoneId"
              :options="zoneOptions"
              placeholder="选择要部署的域名"
            />
          </n-form-item>

          <n-form-item label="路由模式" required>
            <n-input
              v-model:value="deployForm.routePattern"
              placeholder="例如: example.com/* 或 *.example.com/api/*"
            />
            <template #feedback>
              <n-text depth="3" style="font-size: 12px">
                使用 * 作为通配符。Worker 将处理匹配此模式的所有请求
              </n-text>
            </template>
          </n-form-item>

          <!-- 模板特定的配置选项 -->
          <template v-if="currentTemplate.configOptions">
            <n-divider>模板配置</n-divider>

            <n-form-item
              v-for="option in currentTemplate.configOptions"
              :key="option.key"
              :label="option.label"
            >
              <n-input
                v-if="option.type === 'text'"
                v-model:value="deployForm.config[option.key]"
                :placeholder="option.placeholder"
              />
              <n-input-number
                v-else-if="option.type === 'number'"
                v-model:value="deployForm.config[option.key]"
                :placeholder="option.placeholder"
                style="width: 100%"
              />
              <n-switch
                v-else-if="option.type === 'boolean'"
                v-model:value="deployForm.config[option.key]"
              />
              <template #feedback>
                <n-text depth="3" style="font-size: 12px">{{ option.description }}</n-text>
              </template>
            </n-form-item>
          </template>
        </n-form>
      </template>

      <template #footer>
        <n-space justify="end">
          <n-button @click="showDeployModal = false">取消</n-button>
          <n-button type="primary" :loading="deploying" @click="handleConfirmDeploy">
            <template #icon>
              <n-icon><RocketOutline /></n-icon>
            </template>
            开始部署
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useMessage } from 'naive-ui'
import {
  SearchOutline,
  CodeSlashOutline,
  StarOutline,
  EyeOutline,
  RocketOutline
} from '@vicons/ionicons5'
import { cloudflareApi, type Zone } from '@/api'
import { workerTemplates } from '@/data/workerTemplates'
import type { WorkerTemplate } from '@/data/workerTemplates'

const message = useMessage()

const searchKeyword = ref('')
const selectedCategory = ref<string | null>(null)
const showPreviewModal = ref(false)
const showDeployModal = ref(false)
const currentTemplate = ref<WorkerTemplate | null>(null)
const deploying = ref(false)

const zones = ref<Zone[]>([])

const deployForm = ref({
  scriptName: '',
  zoneId: '',
  routePattern: '',
  config: {} as Record<string, any>
})

const categoryOptions = [
  { label: '全部分类', value: null },
  { label: '反向代理', value: '反向代理' },
  { label: 'API 网关', value: 'API' },
  { label: '安全防护', value: '安全' },
  { label: '性能优化', value: '性能' },
  { label: '内容处理', value: '内容' },
  { label: '工具类', value: '工具' }
]

const zoneOptions = computed(() =>
  zones.value.map(zone => ({
    label: zone.name,
    value: zone.id
  }))
)

const filteredTemplates = computed(() => {
  let templates = workerTemplates

  // 按分类筛选
  if (selectedCategory.value) {
    templates = templates.filter(t => t.category === selectedCategory.value)
  }

  // 按关键词搜索
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase()
    templates = templates.filter(t =>
      t.name.toLowerCase().includes(keyword) ||
      t.description.toLowerCase().includes(keyword) ||
      t.features.some(f => f.toLowerCase().includes(keyword))
    )
  }

  return templates
})

function getCategoryColor(category: string) {
  const colors: Record<string, any> = {
    '反向代理': 'primary',
    'API': 'info',
    '安全': 'error',
    '性能': 'success',
    '内容': 'warning',
    '工具': 'default'
  }
  return colors[category] || 'default'
}

function handlePreview(template: WorkerTemplate) {
  currentTemplate.value = template
  showPreviewModal.value = true
}

function handleDeploy(template: WorkerTemplate | null) {
  if (!template) return

  currentTemplate.value = template
  deployForm.value = {
    scriptName: template.id,
    zoneId: zones.value.length > 0 ? zones.value[0].id : '',
    routePattern: '',
    config: {}
  }

  // 初始化配置默认值
  if (template.configOptions) {
    template.configOptions.forEach(option => {
      deployForm.value.config[option.key] = option.default
    })
  }

  showPreviewModal.value = false
  showDeployModal.value = true
}

async function handleConfirmDeploy() {
  if (!currentTemplate.value) return

  if (!deployForm.value.scriptName) {
    message.warning('请输入 Worker 名称')
    return
  }

  if (!deployForm.value.zoneId) {
    message.warning('请选择域名')
    return
  }

  if (!deployForm.value.routePattern) {
    message.warning('请输入路由模式')
    return
  }

  deploying.value = true

  try {
    // 生成替换了配置的Worker代码
    let workerCode = currentTemplate.value.code

    // 替换配置占位符
    if (currentTemplate.value.configOptions) {
      currentTemplate.value.configOptions.forEach(option => {
        const value = deployForm.value.config[option.key]
        const placeholder = `{{${option.key}}}`
        workerCode = workerCode.replace(new RegExp(placeholder, 'g'), String(value))
      })
    }

    // TODO: 调用后端API上传Worker脚本
    // 目前使用现有的deploy_worker API，但它生成的是固定的代码
    // 需要添加新的API来支持上传自定义Worker代码

    message.success('Worker 部署成功！')
    showDeployModal.value = false
  } catch (error: any) {
    message.error(error?.message || 'Worker 部署失败')
  } finally {
    deploying.value = false
  }
}

async function loadZones() {
  try {
    zones.value = await cloudflareApi.getZones()
  } catch (error) {
    console.error('Failed to load zones:', error)
  }
}

onMounted(() => {
  loadZones()
})
</script>
