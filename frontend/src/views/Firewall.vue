<template>
  <n-space vertical :size="24">
    <n-card title="防火墙规则管理">
      <template #header-extra>
        <n-button type="primary" @click="showAddModal = true">
          添加规则
        </n-button>
      </template>

      <n-spin :show="loadingRules">
        <n-data-table
          :columns="columns"
          :data="firewallRules"
          :pagination="{ pageSize: 10 }"
          :bordered="false"
        />
      </n-spin>
    </n-card>

    <!-- 常用规则模板 -->
    <n-card title="常用规则模板">
      <n-grid :cols="2" :x-gap="16" :y-gap="16">
        <n-gi v-for="template in ruleTemplates" :key="template.id">
          <n-card :title="template.name" size="small" hoverable>
            <n-space vertical>
              <n-text>{{ template.description }}</n-text>
              <n-code :code="template.expression" language="javascript" />
              <n-button size="small" @click="useTemplate(template)">
                使用此模板
              </n-button>
            </n-space>
          </n-card>
        </n-gi>
      </n-grid>
    </n-card>

    <!-- 添加规则弹窗 -->
    <n-modal v-model:show="showAddModal" preset="dialog" title="添加防火墙规则" style="width: 700px">
      <n-form
        ref="formRef"
        :model="ruleForm"
        :rules="formRules"
        label-placement="left"
        label-width="120"
      >
        <n-form-item label="规则描述" path="description">
          <n-input
            v-model:value="ruleForm.description"
            placeholder="例如: 阻止非中国大陆访问"
          />
        </n-form-item>

        <n-form-item label="过滤表达式" path="expression">
          <n-input
            v-model:value="ruleForm.expression"
            type="textarea"
            :rows="4"
            placeholder="例如: (ip.geoip.country ne &quot;CN&quot;)"
          />
          <template #feedback>
            <n-text depth="3">
              使用 Cloudflare 表达式语法,参考模板或
              <n-a href="https://developers.cloudflare.com/ruleset-engine/rules-language/" target="_blank">
                官方文档
              </n-a>
            </n-text>
          </template>
        </n-form-item>

        <n-form-item label="动作" path="action">
          <n-select
            v-model:value="ruleForm.action"
            :options="actionOptions"
          />
        </n-form-item>

        <n-form-item label="规则状态" path="paused">
          <n-switch v-model:value="ruleForm.paused">
            <template #checked>已暂停</template>
            <template #unchecked>已启用</template>
          </n-switch>
        </n-form-item>
      </n-form>

      <template #action>
        <n-space>
          <n-button @click="showAddModal = false">取消</n-button>
          <n-button type="primary" :loading="submitting" @click="handleAddRule">
            确认
          </n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 编辑规则弹窗 -->
    <n-modal v-model:show="showEditModal" preset="dialog" title="编辑防火墙规则" style="width: 700px">
      <n-form
        ref="editFormRef"
        :model="editForm"
        :rules="formRules"
        label-placement="left"
        label-width="120"
      >
        <n-form-item label="规则描述" path="description">
          <n-input v-model:value="editForm.description" />
        </n-form-item>

        <n-form-item label="过滤表达式" path="expression">
          <n-input
            v-model:value="editForm.filter.expression"
            type="textarea"
            :rows="4"
            disabled
          />
          <template #feedback>
            <n-text depth="3">表达式创建后无法修改,如需更改请删除后重新创建</n-text>
          </template>
        </n-form-item>

        <n-form-item label="动作" path="action">
          <n-select
            v-model:value="editForm.action"
            :options="actionOptions"
          />
        </n-form-item>

        <n-form-item label="规则状态" path="paused">
          <n-switch v-model:value="editForm.paused">
            <template #checked>已暂停</template>
            <template #unchecked>已启用</template>
          </n-switch>
        </n-form-item>
      </n-form>

      <template #action>
        <n-space>
          <n-button @click="showEditModal = false">取消</n-button>
          <n-button type="primary" :loading="submitting" @click="handleUpdateRule">
            确认
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, h, watch, inject, type Ref } from 'vue'
import { useMessage, NButton, NSpace, NTag, NCode } from 'naive-ui'
import { cloudflareApi, type FirewallRule, type Zone } from '@/api'
import { toast } from '@/utils/toast'
import { logHistory } from '@/utils/history'
import { useAccountStore } from '@/stores/account'

const accountStore = useAccountStore()
const message = useMessage()

// 从 Layout 获取当前域名
const currentZone = inject<Ref<Zone | null>>('currentZone')

const loadingRules = ref(false)
const submitting = ref(false)
const showAddModal = ref(false)
const showEditModal = ref(false)

const firewallRules = ref<FirewallRule[]>([])

const ruleForm = ref({
  description: '',
  expression: '',
  action: 'block',
  paused: false
})

const editForm = ref<FirewallRule>({
  filter: {
    expression: '',
    description: ''
  },
  action: 'block',
  description: '',
  paused: false
})

const formRules = {
  description: { required: true, message: '请输入规则描述', trigger: 'blur' },
  expression: { required: true, message: '请输入过滤表达式', trigger: 'blur' },
  action: { required: true, message: '请选择动作', trigger: 'change' }
}

const actionOptions = [
  { label: '阻止 (Block)', value: 'block' },
  { label: 'JS 质询 (JS Challenge)', value: 'js_challenge' },
  { label: '托管质询 (Managed Challenge)', value: 'managed_challenge' },
  { label: '允许 (Allow)', value: 'allow' },
  { label: '记录 (Log)', value: 'log' }
]

const ruleTemplates = [
  {
    id: 1,
    name: '阻止特定国家',
    description: '阻止来自特定国家的访问',
    expression: '(ip.geoip.country eq "XX")'
  },
  {
    id: 2,
    name: '仅允许中国大陆',
    description: '仅允许来自中国大陆的访问',
    expression: '(ip.geoip.country ne "CN")'
  },
  {
    id: 3,
    name: '阻止恶意爬虫',
    description: '阻止包含bot关键字的UA,但允许Googlebot',
    expression: '(http.user_agent contains "bot" and not http.user_agent contains "Googlebot")'
  },
  {
    id: 4,
    name: 'API频率限制',
    description: '限制API路径的访问频率',
    expression: '(http.request.uri.path contains "/api/" and rate(ip.src, 100/1m))'
  },
  {
    id: 5,
    name: '保护管理后台',
    description: '仅允许中国访问管理后台',
    expression: '(ip.geoip.country ne "CN" and http.request.uri.path eq "/admin")'
  },
  {
    id: 6,
    name: '阻止特定IP',
    description: '阻止特定IP地址或IP段',
    expression: '(ip.src in {192.168.1.1 192.168.1.0/24})'
  }
]

const columns = [
  {
    title: '描述',
    key: 'description',
    ellipsis: { tooltip: true }
  },
  {
    title: '表达式',
    key: 'filter.expression',
    render: (row: FirewallRule) =>
      h(
        NCode,
        { code: row.filter.expression, language: 'javascript', style: 'font-size: 12px' }
      ),
    ellipsis: { tooltip: true }
  },
  {
    title: '动作',
    key: 'action',
    width: 150,
    render: (row: FirewallRule) => {
      const typeMap: Record<string, any> = {
        block: 'error',
        allow: 'success',
        log: 'info',
        js_challenge: 'warning',
        managed_challenge: 'warning'
      }
      return h(
        NTag,
        { type: typeMap[row.action] || 'default', size: 'small' },
        { default: () => row.action }
      )
    }
  },
  {
    title: '状态',
    key: 'paused',
    width: 100,
    render: (row: FirewallRule) =>
      h(
        NTag,
        { type: row.paused ? 'warning' : 'success', size: 'small' },
        { default: () => (row.paused ? '已暂停' : '已启用') }
      )
  },
  {
    title: '操作',
    key: 'actions',
    width: 150,
    render: (row: FirewallRule) =>
      h(
        NSpace,
        {},
        {
          default: () => [
            h(
              NButton,
              {
                size: 'small',
                onClick: () => handleEdit(row)
              },
              { default: () => '编辑' }
            ),
            h(
              NButton,
              {
                size: 'small',
                type: 'error',
                secondary: true,
                onClick: () => handleDelete(row)
              },
              { default: () => '删除' }
            )
          ]
        }
      )
  }
]

async function loadFirewallRules() {
  if (!currentZone?.value?.id) {
    console.log('No currentZone available')
    return
  }

  console.log('Loading firewall rules for zone:', currentZone.value.name)
  loadingRules.value = true
  try {
    firewallRules.value = await cloudflareApi.getFirewallRules(currentZone.value.id)
  } catch (error) {
    message.error('加载防火墙规则失败')
  } finally {
    loadingRules.value = false
  }
}

function useTemplate(template: any) {
  ruleForm.value.expression = template.expression
  ruleForm.value.description = template.name
  showAddModal.value = true
}

async function handleAddRule() {
  if (!currentZone?.value?.id) {
    message.error('未选择域名')
    return
  }

  submitting.value = true
  try {
    await cloudflareApi.createFirewallRule(currentZone.value.id, {
      filter: {
        expression: ruleForm.value.expression,
        description: ruleForm.value.description
      },
      action: ruleForm.value.action,
      description: ruleForm.value.description,
      paused: ruleForm.value.paused
    })

    logHistory.firewall('创建防火墙规则', ruleForm.value.description || '新规则')
    toast.success('防火墙规则已创建')
    showAddModal.value = false
    ruleForm.value = {
      description: '',
      expression: '',
      action: 'block',
      paused: false
    }
    await loadFirewallRules()
  } catch (error: any) {
    message.error(error?.message || '删除失败')
  } finally {
    submitting.value = false
  }
}

function handleEdit(rule: FirewallRule) {
  editForm.value = { ...rule }
  showEditModal.value = true
}

async function handleUpdateRule() {
  if (!currentZone?.value?.id) {
    message.error('未选择域名')
    return
  }

  submitting.value = true
  try {
    await cloudflareApi.updateFirewallRule(
      currentZone.value.id,
      editForm.value.id!,
      editForm.value
    )

    message.success('防火墙规则更新成功')
    showEditModal.value = false
    await loadFirewallRules()
  } catch (error: any) {
    message.error(error?.message || '更新失败')
  } finally {
    submitting.value = false
  }
}

async function handleDelete(rule: FirewallRule) {
  if (!currentZone?.value?.id) {
    message.error('未选择域名')
    return
  }

  try {
    await cloudflareApi.deleteFirewallRule(currentZone.value.id, rule.id!)
    message.success('防火墙规则删除成功')
    await loadFirewallRules()
  } catch (error: any) {
    message.error(error?.message || '删除失败')
  }
}

onMounted(() => {
  loadFirewallRules()
})

// 监听 currentZone 变化，自动重新加载防火墙规则
watch(() => currentZone?.value?.id, () => {
  loadFirewallRules()
})
</script>
