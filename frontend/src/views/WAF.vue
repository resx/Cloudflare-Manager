<template>
  <n-space vertical :size="24">
    <n-card title="WAF 规则包管理">
      <template #header-extra>
        <n-button @click="loadWafPackages" :loading="loading">
          <template #icon>
            <n-icon><RefreshOutline /></n-icon>
          </template>
          刷新
        </n-button>
      </template>

      <n-alert type="info" style="margin-bottom: 16px">
        Web Application Firewall (WAF) 通过托管规则集保护您的网站免受常见攻击。
        Pro 计划及以上可用。
      </n-alert>

      <n-spin :show="loading">
        <n-space vertical :size="16">
          <n-card
            v-for="pkg in packages"
            :key="pkg.id"
            :title="pkg.name"
            size="small"
            hoverable
          >
            <template #header-extra>
              <n-tag :type="pkg.detection_mode === 'anomaly' ? 'success' : 'default'">
                {{ pkg.detection_mode }}
              </n-tag>
            </template>

            <n-descriptions :column="2" size="small">
              <n-descriptions-item label="描述">
                {{ pkg.description }}
              </n-descriptions-item>
              <n-descriptions-item label="敏感度">
                <n-select
                  v-model:value="pkg.sensitivity"
                  :options="sensitivityOptions"
                  size="small"
                  style="width: 150px"
                  @update:value="(val) => handleUpdatePackage(pkg.id, val, pkg.action_mode)"
                />
              </n-descriptions-item>
            </n-descriptions>

            <n-divider style="margin: 12px 0" />

            <n-space vertical>
              <n-space justify="space-between">
                <n-text strong>规则列表</n-text>
                <n-button
                  size="small"
                  @click="toggleRules(pkg.id)"
                >
                  {{ expandedPackages.has(pkg.id) ? '收起' : '展开' }} ({{ getRuleCount(pkg.id) }})
                </n-button>
              </n-space>

              <n-collapse-transition :show="expandedPackages.has(pkg.id)">
                <n-spin :show="rulesLoading[pkg.id]">
                  <n-data-table
                    v-if="packageRules[pkg.id]?.length > 0"
                    :columns="ruleColumns"
                    :data="packageRules[pkg.id]"
                    :pagination="{ pageSize: 10 }"
                    :max-height="400"
                    size="small"
                    :bordered="false"
                  />
                  <n-empty v-else description="暂无规则" />
                </n-spin>
              </n-collapse-transition>
            </n-space>
          </n-card>
        </n-space>

        <n-empty v-if="packages.length === 0 && !loading" description="暂无 WAF 规则包" />
      </n-spin>
    </n-card>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, inject, watch, computed, h, type Ref } from 'vue'
import { NButton, NTag, NSelect, useMessage } from 'naive-ui'
import { RefreshOutline } from '@vicons/ionicons5'
import { cloudflareApi, type Zone, type WafPackage, type WafRule } from '@/api'

const message = useMessage()

const loading = ref(false)
const packages = ref<WafPackage[]>([])
const packageRules = ref<Record<string, WafRule[]>>({})
const rulesLoading = ref<Record<string, boolean>>({})
const expandedPackages = ref<Set<string>>(new Set())

// 从 Layout 获取当前域名
const currentZone = inject<Ref<Zone | null>>('currentZone')

const sensitivityOptions = [
  { label: '关闭 (Off)', value: 'off' },
  { label: '低 (Low)', value: 'low' },
  { label: '中 (Medium)', value: 'medium' },
  { label: '高 (High)', value: 'high' }
]

const ruleModeOptions = [
  { label: '默认 (Default)', value: 'default' },
  { label: '禁用 (Disable)', value: 'disable' },
  { label: '模拟 (Simulate)', value: 'simulate' },
  { label: '阻止 (Block)', value: 'block' },
  { label: '质询 (Challenge)', value: 'challenge' }
]

// 规则表格列
const ruleColumns = computed(() => [
  {
    title: 'ID',
    key: 'id',
    width: 100,
    ellipsis: {
      tooltip: true
    }
  },
  {
    title: '描述',
    key: 'description',
    width: 300,
    ellipsis: {
      tooltip: true
    }
  },
  {
    title: '优先级',
    key: 'priority',
    width: 80
  },
  {
    title: '分组',
    key: 'group',
    width: 150,
    render: (row: WafRule) => row.group.name
  },
  {
    title: '模式',
    key: 'mode',
    width: 150,
    render: (row: WafRule) => {
      return h(NSelect, {
        value: row.mode,
        options: ruleModeOptions.filter(opt => row.allowed_modes.includes(opt.value)),
        size: 'small',
        onUpdateValue: (val: string) => handleUpdateRule(row, val)
      })
    }
  },
  {
    title: '状态',
    key: 'status',
    width: 100,
    render: (row: WafRule) => {
      const type = row.mode === 'on' || row.mode === 'block' ? 'success'
                 : row.mode === 'simulate' ? 'warning'
                 : 'default'
      return h(NTag, { type, size: 'small' }, { default: () => row.mode })
    }
  }
])

async function loadWafPackages() {
  if (!currentZone?.value?.id) {
    return
  }

  loading.value = true
  try {
    packages.value = await cloudflareApi.getWafPackages(currentZone.value.id)
    message.success(`成功加载 ${packages.value.length} 个 WAF 规则包`)
  } catch (error: any) {
    message.warning(error?.message || '加载 WAF 规则包失败（可能需要 Pro 计划）')
    console.error('Load WAF packages error:', error)
    packages.value = []
  } finally {
    loading.value = false
  }
}

async function toggleRules(packageId: string) {
  if (expandedPackages.value.has(packageId)) {
    expandedPackages.value.delete(packageId)
  } else {
    expandedPackages.value.add(packageId)
    if (!packageRules.value[packageId]) {
      await loadWafRules(packageId)
    }
  }
}

async function loadWafRules(packageId: string) {
  if (!currentZone?.value?.id) {
    return
  }

  rulesLoading.value[packageId] = true
  try {
    const rules = await cloudflareApi.getWafRules(currentZone.value.id, packageId)
    packageRules.value[packageId] = rules
  } catch (error: any) {
    message.error(error?.message || '加载 WAF 规则失败')
    console.error('Load WAF rules error:', error)
    packageRules.value[packageId] = []
  } finally {
    rulesLoading.value[packageId] = false
  }
}

function getRuleCount(packageId: string): string {
  const count = packageRules.value[packageId]?.length
  return count !== undefined ? count.toString() : '?'
}

async function handleUpdatePackage(packageId: string, sensitivity?: string, actionMode?: string) {
  if (!currentZone?.value?.id) {
    message.warning('请先选择一个域名')
    return
  }

  try {
    await cloudflareApi.updateWafPackage(currentZone.value.id, packageId, sensitivity, actionMode)
    message.success('WAF 规则包设置已更新')
    await loadWafPackages()
  } catch (error: any) {
    message.error(error?.message || '更新 WAF 规则包失败')
    console.error('Update WAF package error:', error)
  }
}

async function handleUpdateRule(rule: WafRule, mode: string) {
  if (!currentZone?.value?.id) {
    message.warning('请先选择一个域名')
    return
  }

  // 找到规则所属的包
  let packageId = ''
  for (const [pkgId, rules] of Object.entries(packageRules.value)) {
    if (rules.some(r => r.id === rule.id)) {
      packageId = pkgId
      break
    }
  }

  if (!packageId) {
    message.error('无法找到规则所属的包')
    return
  }

  try {
    await cloudflareApi.updateWafRule(currentZone.value.id, packageId, rule.id, mode)
    message.success('WAF 规则已更新')
    // 更新本地数据
    rule.mode = mode
  } catch (error: any) {
    message.error(error?.message || '更新 WAF 规则失败')
    console.error('Update WAF rule error:', error)
  }
}

onMounted(() => {
  loadWafPackages()
})

// 监听 currentZone 变化
watch(() => currentZone?.value?.id, () => {
  packages.value = []
  packageRules.value = {}
  expandedPackages.value.clear()
  loadWafPackages()
})
</script>
