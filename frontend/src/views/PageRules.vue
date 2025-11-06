<template>
  <n-space vertical :size="24">
    <n-card title="页面规则">
      <template #header-extra>
        <n-space>
          <n-tag type="warning">免费计划限制: 最多 3 条规则</n-tag>
          <n-button type="primary" @click="handleOpenCreateModal" :disabled="rules.length >= 3">
            创建规则
          </n-button>
        </n-space>
      </template>

      <n-alert type="info" style="margin-bottom: 16px">
        页面规则允许您为特定 URL 模式自定义 Cloudflare 设置。规则按优先级从高到低执行（1 优先级最高）。
      </n-alert>

      <n-alert type="warning" style="margin-bottom: 16px" title="Token 权限要求">
        页面规则 API 需要<strong>用户级别</strong>的 API Token，不支持账户级别的 Token。请确保您的 Token 具有正确的权限范围。
      </n-alert>

      <n-spin :show="loading">
        <n-empty v-if="rules.length === 0" description="暂无页面规则">
          <template #extra>
            <n-button size="small" @click="handleOpenCreateModal">
              创建第一条规则
            </n-button>
          </template>
        </n-empty>

        <n-list v-else bordered>
          <n-list-item v-for="(rule, index) in rules" :key="rule.id">
            <template #prefix>
              <n-text strong>优先级 {{ rule.priority || index + 1 }}</n-text>
            </template>

            <n-space vertical :size="8">
              <n-space align="center">
                <n-tag type="primary">{{ getUrlPattern(rule) }}</n-tag>
                <n-tag v-if="rule.status === 'active'" type="success">生效中</n-tag>
                <n-tag v-else type="default">已禁用</n-tag>
              </n-space>

              <n-space>
                <n-text depth="3" style="font-size: 12px">
                  操作: {{ formatActions(rule.actions) }}
                </n-text>
              </n-space>
            </n-space>

            <template #suffix>
              <n-space>
                <n-button size="small" @click="handleEdit(rule)">
                  编辑
                </n-button>
                <n-button size="small" type="error" @click="handleDelete(rule)">
                  删除
                </n-button>
              </n-space>
            </template>
          </n-list-item>
        </n-list>
      </n-spin>
    </n-card>

    <!-- 创建/编辑规则弹窗 -->
    <n-modal v-model:show="showRuleModal" preset="card" :title="editingRule ? '编辑页面规则' : '创建页面规则'" style="width: 700px">
      <n-form ref="formRef" :model="ruleForm" label-placement="left" label-width="120">
        <n-form-item label="URL 模式" required>
          <n-input
            v-model:value="ruleForm.urlPattern"
            placeholder="例如: example.com/* 或 *.example.com/path/*"
          />
          <template #feedback>
            <n-text depth="3" style="font-size: 12px">
              使用 * 作为通配符。示例: example.com/blog/* 匹配所有博客页面
            </n-text>
          </template>
        </n-form-item>

        <n-divider>规则操作</n-divider>

        <!-- 缓存级别 -->
        <n-form-item label="缓存级别">
          <n-select
            v-model:value="ruleForm.cacheLevel"
            :options="cacheLevelOptions"
            clearable
            placeholder="选择缓存级别"
          />
        </n-form-item>

        <!-- 浏览器缓存 TTL -->
        <n-form-item label="浏览器缓存 TTL">
          <n-select
            v-model:value="ruleForm.browserCacheTtl"
            :options="browserCacheTtlOptions"
            clearable
            placeholder="选择浏览器缓存时长"
          />
        </n-form-item>

        <!-- SSL 模式 -->
        <n-form-item label="SSL 模式">
          <n-select
            v-model:value="ruleForm.ssl"
            :options="sslOptions"
            clearable
            placeholder="选择 SSL 模式"
          />
        </n-form-item>

        <!-- 安全级别 -->
        <n-form-item label="安全级别">
          <n-select
            v-model:value="ruleForm.securityLevel"
            :options="securityLevelOptions"
            clearable
            placeholder="选择安全级别"
          />
        </n-form-item>

        <!-- 规则状态 -->
        <n-form-item label="规则状态">
          <n-switch
            v-model:value="ruleForm.active"
            checked-value="active"
            unchecked-value="disabled"
          >
            <template #checked>生效</template>
            <template #unchecked>禁用</template>
          </n-switch>
        </n-form-item>
      </n-form>

      <template #footer>
        <n-space justify="end">
          <n-button @click="showRuleModal = false">取消</n-button>
          <n-button type="primary" :loading="submitting" @click="handleSubmit">
            {{ editingRule ? '保存' : '创建' }}
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, inject, watch, type Ref } from 'vue'
import { useMessage, useDialog } from 'naive-ui'
import { cloudflareApi, type Zone, type PageRule, type PageRuleAction } from '@/api'

const message = useMessage()
const dialog = useDialog()

const loading = ref(false)
const submitting = ref(false)
const showRuleModal = ref(false)
const editingRule = ref<PageRule | null>(null)

// 从 Layout 获取当前域名
const currentZone = inject<Ref<Zone | null>>('currentZone')

const rules = ref<PageRule[]>([])

// 表单
const ruleForm = ref({
  urlPattern: '',
  cacheLevel: null as string | null,
  browserCacheTtl: null as number | null,
  ssl: null as string | null,
  securityLevel: null as string | null,
  active: 'active'
})

// 选项
const cacheLevelOptions = [
  { label: '绕过 (Bypass)', value: 'bypass' },
  { label: '不查询字符串 (No Query String)', value: 'basic' },
  { label: '忽略查询字符串 (Ignore Query String)', value: 'simplified' },
  { label: '标准 (Standard)', value: 'aggressive' },
  { label: '缓存一切 (Cache Everything)', value: 'cache_everything' }
]

const browserCacheTtlOptions = [
  { label: '30 分钟', value: 1800 },
  { label: '1 小时', value: 3600 },
  { label: '2 小时', value: 7200 },
  { label: '4 小时', value: 14400 },
  { label: '1 天', value: 86400 },
  { label: '1 周', value: 604800 },
  { label: '1 个月', value: 2678400 },
  { label: '1 年', value: 31536000 }
]

const sslOptions = [
  { label: '关闭 (Off)', value: 'off' },
  { label: '灵活 (Flexible)', value: 'flexible' },
  { label: '完全 (Full)', value: 'full' },
  { label: '完全(严格) (Full Strict)', value: 'strict' }
]

const securityLevelOptions = [
  { label: '基本关闭 (Essentially Off)', value: 'essentially_off' },
  { label: '低 (Low)', value: 'low' },
  { label: '中 (Medium)', value: 'medium' },
  { label: '高 (High)', value: 'high' },
  { label: 'I\'m Under Attack!', value: 'under_attack' }
]

function getUrlPattern(rule: PageRule): string {
  if (rule.targets && rule.targets.length > 0) {
    return rule.targets[0].constraint.value
  }
  return '未知'
}

function formatActions(actions: PageRuleAction[]): string {
  if (!actions || actions.length === 0) return '无'

  const actionNames: Record<string, string> = {
    cache_level: '缓存级别',
    browser_cache_ttl: '浏览器缓存TTL',
    ssl: 'SSL模式',
    security_level: '安全级别',
    forwarding_url: 'URL转发'
  }

  return actions.map(a => actionNames[a.id] || a.id).join(', ')
}

async function loadPageRules() {
  if (!currentZone?.value?.id) {
    console.log('No currentZone available for page rules')
    return
  }

  console.log('Loading page rules for zone:', currentZone.value.name)
  loading.value = true
  try {
    rules.value = await cloudflareApi.getPageRules(currentZone.value.id)
  } catch (error: any) {
    console.error('Load page rules error:', error)

    // 检查是否是Token类型限制
    if (error.message && error.message.includes('does not support account owned tokens')) {
      message.warning('页面规则功能需要用户级别的 API Token，账户级别的Token不支持此功能')
    } else if (error.isFeatureLimited) {
      message.warning('页面规则功能需要特定权限或付费套餐')
    } else {
      message.error(error?.message || '加载页面规则失败')
    }
  } finally {
    loading.value = false
  }
}

function handleOpenCreateModal() {
  if (rules.value.length >= 3) {
    message.warning('免费计划最多支持 3 条页面规则')
    return
  }

  editingRule.value = null
  ruleForm.value = {
    urlPattern: '',
    cacheLevel: null,
    browserCacheTtl: null,
    ssl: null,
    securityLevel: null,
    active: 'active'
  }
  showRuleModal.value = true
}

function handleEdit(rule: PageRule) {
  editingRule.value = rule

  // 填充表单
  ruleForm.value.urlPattern = getUrlPattern(rule)
  ruleForm.value.active = rule.status || 'active'
  ruleForm.value.cacheLevel = null
  ruleForm.value.browserCacheTtl = null
  ruleForm.value.ssl = null
  ruleForm.value.securityLevel = null

  // 解析现有的 actions
  rule.actions.forEach(action => {
    switch (action.id) {
      case 'cache_level':
        ruleForm.value.cacheLevel = action.value as string
        break
      case 'browser_cache_ttl':
        ruleForm.value.browserCacheTtl = action.value as number
        break
      case 'ssl':
        ruleForm.value.ssl = action.value as string
        break
      case 'security_level':
        ruleForm.value.securityLevel = action.value as string
        break
    }
  })

  showRuleModal.value = true
}

async function handleSubmit() {
  if (!currentZone?.value?.id) {
    message.error('未选择域名')
    return
  }

  if (!ruleForm.value.urlPattern) {
    message.warning('请输入 URL 模式')
    return
  }

  // 构建 actions
  const actions: PageRuleAction[] = []

  if (ruleForm.value.cacheLevel) {
    actions.push({ id: 'cache_level', value: ruleForm.value.cacheLevel })
  }
  if (ruleForm.value.browserCacheTtl) {
    actions.push({ id: 'browser_cache_ttl', value: ruleForm.value.browserCacheTtl })
  }
  if (ruleForm.value.ssl) {
    actions.push({ id: 'ssl', value: ruleForm.value.ssl })
  }
  if (ruleForm.value.securityLevel) {
    actions.push({ id: 'security_level', value: ruleForm.value.securityLevel })
  }

  if (actions.length === 0) {
    message.warning('请至少选择一个操作')
    return
  }

  // 构建规则
  const rule: PageRule = {
    targets: [{
      target: 'url',
      constraint: {
        operator: 'matches',
        value: ruleForm.value.urlPattern
      }
    }],
    actions: actions,
    status: ruleForm.value.active
  }

  submitting.value = true
  try {
    if (editingRule.value?.id) {
      // 更新规则
      await cloudflareApi.updatePageRule(currentZone.value.id, editingRule.value.id, rule)
      message.success('规则已更新')
    } else {
      // 创建规则
      await cloudflareApi.createPageRule(currentZone.value.id, rule)
      message.success('规则已创建')
    }

    showRuleModal.value = false
    await loadPageRules()
  } catch (error: any) {
    message.error(error?.message || '操作失败')
  } finally {
    submitting.value = false
  }
}

function handleDelete(rule: PageRule) {
  if (!currentZone?.value?.id || !rule.id) return

  dialog.warning({
    title: '确认删除',
    content: `确定要删除规则 "${getUrlPattern(rule)}" 吗？此操作无法撤销。`,
    positiveText: '删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await cloudflareApi.deletePageRule(currentZone.value!.id, rule.id!)
        message.success('规则已删除')
        await loadPageRules()
      } catch (error: any) {
        message.error(error?.message || '删除失败')
      }
    }
  })
}

onMounted(() => {
  loadPageRules()
})

// 监听 currentZone 变化，自动重新加载页面规则
watch(() => currentZone?.value?.id, () => {
  loadPageRules()
})
</script>
