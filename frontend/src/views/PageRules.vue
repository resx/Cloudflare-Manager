<template>
  <n-space vertical :size="24">
    <n-card title="页面规则">
      <template #header-extra>
        <n-button type="primary" @click="showCreateModal = true">
          创建规则
        </n-button>
      </template>

      <n-alert type="info" style="margin-bottom: 16px">
        页面规则允许您为特定 URL 模式自定义 Cloudflare 设置。规则按从上到下的顺序执行。
      </n-alert>

      <n-spin :show="loading">
        <n-empty v-if="rules.length === 0" description="暂无页面规则">
          <template #extra>
            <n-button size="small" @click="showCreateModal = true">
              创建第一条规则
            </n-button>
          </template>
        </n-empty>

        <n-list v-else bordered>
          <n-list-item v-for="(rule, index) in rules" :key="rule.id">
            <template #prefix>
              <n-text strong>{{ index + 1 }}</n-text>
            </template>

            <n-space vertical :size="8">
              <n-space align="center">
                <n-tag type="primary">{{ rule.pattern }}</n-tag>
                <n-tag v-if="rule.status === 'active'" type="success">生效中</n-tag>
                <n-tag v-else type="default">已禁用</n-tag>
              </n-space>

              <n-space>
                <n-text depth="3" style="font-size: 12px">
                  设置: {{ rule.actions.join(', ') }}
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
    <n-modal v-model:show="showCreateModal" preset="dialog" title="创建页面规则" style="width: 700px">
      <n-form ref="formRef" :model="ruleForm" label-placement="left" label-width="120">
        <n-form-item label="URL 模式" required>
          <n-input
            v-model:value="ruleForm.pattern"
            placeholder="例如: example.com/* 或 *.example.com/path"
          />
        </n-form-item>

        <n-divider>规则设置</n-divider>

        <n-alert type="info" style="margin-bottom: 16px; font-size: 12px">
          该功能需要 Cloudflare 企业版计划支持，当前仅显示界面示例
        </n-alert>

        <n-checkbox-group v-model:value="ruleForm.selectedActions">
          <n-space vertical>
            <n-checkbox value="cache_level">缓存级别</n-checkbox>
            <n-checkbox value="ssl">SSL 模式</n-checkbox>
            <n-checkbox value="browser_cache_ttl">浏览器缓存 TTL</n-checkbox>
            <n-checkbox value="security_level">安全级别</n-checkbox>
            <n-checkbox value="forwarding_url">转发 URL</n-checkbox>
          </n-space>
        </n-checkbox-group>
      </n-form>

      <template #action>
        <n-space>
          <n-button @click="showCreateModal = false">取消</n-button>
          <n-button type="primary" @click="handleCreate" disabled>
            创建（企业版功能）
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useMessage } from 'naive-ui'

const message = useMessage()
const loading = ref(false)
const showCreateModal = ref(false)

const rules = ref<any[]>([])
const ruleForm = ref({
  pattern: '',
  selectedActions: []
})

async function loadPageRules() {
  loading.value = true
  try {
    // TODO: 调用 Cloudflare Page Rules API
    await new Promise(resolve => setTimeout(resolve, 500))
    rules.value = []
  } catch (error: any) {
    message.error(error?.message || '加载页面规则失败')
  } finally {
    loading.value = false
  }
}

function handleCreate() {
  message.info('此功能需要 Cloudflare 企业版计划')
}

function handleEdit(rule: any) {
  message.info('此功能需要 Cloudflare 企业版计划')
}

function handleDelete(rule: any) {
  message.info('此功能需要 Cloudflare 企业版计划')
}

onMounted(() => {
  loadPageRules()
})
</script>
