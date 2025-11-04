<template>
  <n-space vertical :size="24">
    <n-card title="DNS 记录管理">
      <template #header-extra>
        <n-button type="primary" @click="showAddModal = true">
          添加记录
        </n-button>
      </template>

      <n-form label-placement="left" label-width="100" style="margin-bottom: 16px">
        <n-form-item label="选择域名">
          <n-select
            v-model:value="selectedZone"
            :options="zoneOptions"
            placeholder="请选择域名"
            :loading="loadingZones"
            @update:value="loadDnsRecords"
          />
        </n-form-item>
      </n-form>

      <n-spin :show="loadingRecords">
        <n-data-table
          :columns="columns"
          :data="dnsRecords"
          :pagination="{ pageSize: 10 }"
          :bordered="false"
        />
      </n-spin>
    </n-card>

    <!-- 添加 DNS 记录弹窗 -->
    <n-modal v-model:show="showAddModal" preset="dialog" title="添加 DNS 记录" style="width: 600px">
      <n-form
        ref="formRef"
        :model="dnsForm"
        :rules="formRules"
        label-placement="left"
        label-width="100"
      >
        <n-form-item label="记录类型" path="type">
          <n-select
            v-model:value="dnsForm.type"
            :options="recordTypeOptions"
          />
        </n-form-item>

        <n-form-item label="名称" path="name">
          <n-input
            v-model:value="dnsForm.name"
            placeholder="例如: www 或 @ (根域名)"
          />
        </n-form-item>

        <n-form-item label="内容" path="content">
          <n-input
            v-model:value="dnsForm.content"
            placeholder="例如: 192.168.1.1 或 example.com"
          />
        </n-form-item>

        <n-form-item label="TTL" path="ttl">
          <n-select
            v-model:value="dnsForm.ttl"
            :options="ttlOptions"
            style="width: 100%"
          />
        </n-form-item>

        <n-form-item label="代理状态" path="proxied">
          <n-switch v-model:value="dnsForm.proxied">
            <template #checked>已代理</template>
            <template #unchecked>仅 DNS</template>
          </n-switch>
        </n-form-item>

        <n-form-item v-if="dnsForm.type === 'MX'" label="优先级" path="priority">
          <n-input-number
            v-model:value="dnsForm.priority"
            :min="0"
            :max="65535"
            style="width: 100%"
          />
        </n-form-item>
      </n-form>

      <template #action>
        <n-space>
          <n-button @click="showAddModal = false">取消</n-button>
          <n-button type="primary" :loading="submitting" @click="handleAddRecord">
            确认
          </n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 编辑 DNS 记录弹窗 -->
    <n-modal v-model:show="showEditModal" preset="dialog" title="编辑 DNS 记录" style="width: 600px">
      <n-form
        ref="editFormRef"
        :model="editForm"
        :rules="formRules"
        label-placement="left"
        label-width="100"
      >
        <n-form-item label="记录类型" path="type">
          <n-select
            v-model:value="editForm.type"
            :options="recordTypeOptions"
          />
        </n-form-item>

        <n-form-item label="名称" path="name">
          <n-input v-model:value="editForm.name" />
        </n-form-item>

        <n-form-item label="内容" path="content">
          <n-input v-model:value="editForm.content" />
        </n-form-item>

        <n-form-item label="TTL" path="ttl">
          <n-select
            v-model:value="editForm.ttl"
            :options="ttlOptions"
            style="width: 100%"
          />
        </n-form-item>

        <n-form-item label="代理状态" path="proxied">
          <n-switch v-model:value="editForm.proxied">
            <template #checked>已代理</template>
            <template #unchecked>仅 DNS</template>
          </n-switch>
        </n-form-item>
      </n-form>

      <template #action>
        <n-space>
          <n-button @click="showEditModal = false">取消</n-button>
          <n-button type="primary" :loading="submitting" @click="handleUpdateRecord">
            确认
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, h } from 'vue'
import { useMessage, NButton, NSpace, NTag, NSwitch } from 'naive-ui'
import { cloudflareApi, type Zone, type DnsRecord } from '@/api'
import { useAccountStore } from '@/stores/account'

const accountStore = useAccountStore()
const message = useMessage()

const loadingZones = ref(false)
const loadingRecords = ref(false)
const submitting = ref(false)
const showAddModal = ref(false)
const showEditModal = ref(false)

const zones = ref<Zone[]>([])
const selectedZone = ref('')
const dnsRecords = ref<DnsRecord[]>([])

const dnsForm = ref({
  type: 'A',
  name: '',
  content: '',
  ttl: 1,
  proxied: true,
  priority: 10
})

const editForm = ref<DnsRecord>({
  zone_id: '',
  type: 'A',
  name: '',
  content: '',
  ttl: 1,
  proxied: true
})

const formRules = {
  name: { required: true, message: '请输入名称', trigger: 'blur' },
  content: { required: true, message: '请输入内容', trigger: 'blur' }
}

const recordTypeOptions = [
  { label: 'A', value: 'A' },
  { label: 'AAAA', value: 'AAAA' },
  { label: 'CNAME', value: 'CNAME' },
  { label: 'MX', value: 'MX' },
  { label: 'TXT', value: 'TXT' },
  { label: 'SRV', value: 'SRV' },
  { label: 'NS', value: 'NS' },
  { label: 'CAA', value: 'CAA' },
  { label: 'CERT', value: 'CERT' },
  { label: 'DNSKEY', value: 'DNSKEY' },
  { label: 'DS', value: 'DS' },
  { label: 'HTTPS', value: 'HTTPS' },
  { label: 'LOC', value: 'LOC' },
  { label: 'NAPTR', value: 'NAPTR' },
  { label: 'PTR', value: 'PTR' },
  { label: 'SMIMEA', value: 'SMIMEA' },
  { label: 'SPF', value: 'SPF' },
  { label: 'SSHFP', value: 'SSHFP' },
  { label: 'SVCB', value: 'SVCB' },
  { label: 'TLSA', value: 'TLSA' },
  { label: 'URI', value: 'URI' }
]

// TTL 选项（参照 Cloudflare 标准）
const ttlOptions = [
  { label: '自动', value: 1 },
  { label: '2 分钟', value: 120 },
  { label: '5 分钟', value: 300 },
  { label: '10 分钟', value: 600 },
  { label: '15 分钟', value: 900 },
  { label: '30 分钟', value: 1800 },
  { label: '1 小时', value: 3600 },
  { label: '2 小时', value: 7200 },
  { label: '5 小时', value: 18000 },
  { label: '12 小时', value: 43200 },
  { label: '1 天', value: 86400 }
]

const zoneOptions = computed(() =>
  zones.value.map(zone => ({
    label: zone.name,
    value: zone.id
  }))
)

const columns = [
  { title: '类型', key: 'type', width: 80 },
  { title: '名称', key: 'name' },
  {
    title: '内容',
    key: 'content',
    render: (row: DnsRecord) => {
      // 移除 TXT 记录值外部的双引号
      if (row.type === 'TXT' && row.content.startsWith('"') && row.content.endsWith('"')) {
        return row.content.slice(1, -1)
      }
      return row.content
    }
  },
  {
    title: 'TTL',
    key: 'ttl',
    width: 100,
    render: (row: DnsRecord) => {
      // Cloudflare 的 TTL 显示格式
      if (row.ttl === 1) return '自动'
      if (row.ttl < 60) return `${row.ttl} 秒`
      if (row.ttl < 3600) return `${Math.floor(row.ttl / 60)} 分钟`
      if (row.ttl < 86400) return `${Math.floor(row.ttl / 3600)} 小时`
      return `${Math.floor(row.ttl / 86400)} 天`
    }
  },
  {
    title: '代理状态',
    key: 'proxied',
    width: 120,
    render: (row: DnsRecord) =>
      h(
        NTag,
        { type: row.proxied ? 'success' : 'default', size: 'small' },
        { default: () => (row.proxied ? '已代理' : '仅 DNS') }
      )
  },
  {
    title: '操作',
    key: 'actions',
    width: 150,
    render: (row: DnsRecord) =>
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

async function loadZones() {
  if (!accountStore.currentAccount) return

  loadingZones.value = true
  try {
    zones.value = await cloudflareApi.getZones()
    if (zones.value.length > 0) {
      selectedZone.value = zones.value[0].id
      await loadDnsRecords()
    }
  } catch (error) {
    message.error('加载域名列表失败')
  } finally {
    loadingZones.value = false
  }
}

async function loadDnsRecords() {
  if (!selectedZone.value) return

  loadingRecords.value = true
  try {
    const records = await cloudflareApi.getDnsRecords(selectedZone.value)
    // 为每条记录添加 zone_id，因为 Cloudflare API 返回的记录不包含此字段
    dnsRecords.value = records.map(record => ({
      ...record,
      zone_id: selectedZone.value
    }))
  } catch (error) {
    message.error('加载 DNS 记录失败')
  } finally {
    loadingRecords.value = false
  }
}

async function handleAddRecord() {
  submitting.value = true
  try {
    const recordToAdd = {
      zone_id: selectedZone.value,
      ...dnsForm.value
    }

    // 为 TXT 记录自动添加引号（如果需要）
    if (recordToAdd.type === 'TXT' &&
        !recordToAdd.content.startsWith('"') &&
        !recordToAdd.content.endsWith('"')) {
      recordToAdd.content = `"${recordToAdd.content}"`
    }

    await cloudflareApi.createDnsRecord(recordToAdd)

    message.success('DNS 记录添加成功')
    showAddModal.value = false
    dnsForm.value = {
      type: 'A',
      name: '',
      content: '',
      ttl: 1,
      proxied: true,
      priority: 10
    }
    await loadDnsRecords()
  } catch (error: any) {
    message.error(error?.message || '添加失败')
  } finally {
    submitting.value = false
  }
}

function handleEdit(record: DnsRecord) {
  editForm.value = { ...record }

  // 规范化 TTL 值到最接近的预设选项
  const validTtls = [1, 120, 300, 600, 900, 1800, 3600, 7200, 18000, 43200, 86400]
  if (!validTtls.includes(editForm.value.ttl)) {
    // 找到最接近的 TTL 值
    editForm.value.ttl = validTtls.reduce((prev, curr) =>
      Math.abs(curr - editForm.value.ttl) < Math.abs(prev - editForm.value.ttl) ? curr : prev
    )
  }

  // 移除 TXT 记录值的外部引号，方便编辑
  if (editForm.value.type === 'TXT' &&
      editForm.value.content.startsWith('"') &&
      editForm.value.content.endsWith('"')) {
    editForm.value.content = editForm.value.content.slice(1, -1)
  }
  showEditModal.value = true
}

async function handleUpdateRecord() {
  submitting.value = true
  try {
    const recordToUpdate = { ...editForm.value }

    // 为 TXT 记录自动添加引号（如果需要）
    if (recordToUpdate.type === 'TXT' &&
        !recordToUpdate.content.startsWith('"') &&
        !recordToUpdate.content.endsWith('"')) {
      recordToUpdate.content = `"${recordToUpdate.content}"`
    }

    await cloudflareApi.updateDnsRecord(recordToUpdate)

    message.success('DNS 记录更新成功')
    showEditModal.value = false
    await loadDnsRecords()
  } catch (error: any) {
    message.error(error?.message || '更新失败')
  } finally {
    submitting.value = false
  }
}

async function handleDelete(record: DnsRecord) {
  try {
    // 确保 zone_id 和 id 存在
    if (!record.zone_id || !record.id) {
      message.error('记录信息不完整，无法删除')
      return
    }

    await cloudflareApi.deleteDnsRecord(record.zone_id, record.id)
    message.success('DNS 记录删除成功')
    await loadDnsRecords()
  } catch (error: any) {
    message.error(error?.message || '删除失败')
  }
}

onMounted(() => {
  loadZones()
})
</script>
