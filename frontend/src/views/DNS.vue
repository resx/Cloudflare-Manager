<template>
  <n-space vertical :size="24">
    <n-card title="DNS 记录管理">
      <template #header-extra>
        <n-space>
          <n-button @click="showBatchImportModal = true">
            批量导入
          </n-button>
          <n-button type="primary" @click="showAddModal = true">
            添加记录
          </n-button>
        </n-space>
      </template>

      <n-spin :show="loadingRecords">
        <n-data-table
          :columns="columns"
          :data="dnsRecords"
          :pagination="{ pageSize: 10 }"
          :bordered="false"
        />
      </n-spin>
    </n-card>

    <!-- 批量导入弹窗 -->
    <n-modal v-model:show="showBatchImportModal" preset="card" title="批量导入 DNS 记录" style="width: 800px">
      <n-space vertical :size="16">
        <n-alert type="info">
          <template #header>导入格式说明</template>
          支持 CSV 格式，每行一条记录。格式：类型,名称,内容,TTL,是否代理,优先级<br/>
          示例：<br/>
          <n-code :code="csvExample" language="csv" style="margin-top: 8px" />
        </n-alert>

        <n-form-item label="导入方式">
          <n-radio-group v-model:value="importMethod">
            <n-space>
              <n-radio value="paste">粘贴文本</n-radio>
              <n-radio value="file">上传文件</n-radio>
            </n-space>
          </n-radio-group>
        </n-form-item>

        <n-form-item v-if="importMethod === 'paste'" label="CSV 数据">
          <n-input
            v-model:value="batchImportText"
            type="textarea"
            placeholder="粘贴 CSV 数据，每行一条记录"
            :rows="10"
          />
        </n-form-item>

        <n-form-item v-if="importMethod === 'file'" label="选择文件">
          <n-upload
            :max="1"
            accept=".csv,.txt"
            :on-change="handleFileUpload"
            :show-file-list="false"
          >
            <n-button>选择 CSV 文件</n-button>
          </n-upload>
          <n-text v-if="uploadedFileName" depth="3" style="margin-left: 8px">
            已选择: {{ uploadedFileName }}
          </n-text>
        </n-form-item>

        <n-alert v-if="parseErrors.length > 0" type="error" title="解析错误">
          <ul style="margin: 0; padding-left: 20px">
            <li v-for="(error, index) in parseErrors" :key="index">{{ error }}</li>
          </ul>
        </n-alert>

        <n-alert v-if="parsedRecords.length > 0" type="success" :title="`已解析 ${parsedRecords.length} 条记录`">
          <n-data-table
            :columns="previewColumns"
            :data="parsedRecords"
            :pagination="false"
            max-height="300"
            size="small"
          />
        </n-alert>
      </n-space>

      <template #footer>
        <n-space justify="end">
          <n-button @click="handleCancelBatchImport">取消</n-button>
          <n-button @click="handleParseBatchImport" :disabled="!batchImportText">
            解析
          </n-button>
          <n-button
            type="primary"
            :loading="batchImporting"
            :disabled="parsedRecords.length === 0"
            @click="handleConfirmBatchImport"
          >
            导入 {{ parsedRecords.length }} 条记录
          </n-button>
        </n-space>
      </template>
    </n-modal>

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
import { ref, onMounted, computed, h, watch, inject, type Ref } from 'vue'
import { NButton, NSpace, NTag, NSwitch } from 'naive-ui'
import { cloudflareApi, type Zone, type DnsRecord } from '@/api'
import { toast } from '@/utils/toast'
import { logHistory } from '@/utils/history'

// 从 Layout 获取当前域名
const currentZone = inject<Ref<Zone | null>>('currentZone')

const loadingRecords = ref(false)
const submitting = ref(false)
const showAddModal = ref(false)
const showEditModal = ref(false)

// 批量导入相关
const showBatchImportModal = ref(false)
const batchImporting = ref(false)
const batchImportText = ref('')
const importMethod = ref('paste')
const uploadedFileName = ref('')
const parsedRecords = ref<DnsRecord[]>([])
const parseErrors = ref<string[]>([])

const csvExample = `A,www,192.168.1.1,3600,true
AAAA,www,2001:db8::1,3600,true
CNAME,blog,example.com,1,false
MX,@,mail.example.com,3600,false,10
TXT,@,"v=spf1 include:_spf.example.com ~all",1,false`

const previewColumns = [
  { title: '类型', key: 'type', width: 80 },
  { title: '名称', key: 'name', width: 120 },
  { title: '内容', key: 'content', ellipsis: { tooltip: true } },
  { title: 'TTL', key: 'ttl', width: 80 },
  { title: '代理', key: 'proxied', width: 60, render: (row: any) => row.proxied ? '是' : '否' }
]

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

async function loadDnsRecords() {
  if (!currentZone?.value?.id) {
    console.log('No currentZone available')
    return
  }

  console.log('Loading DNS records for zone:', currentZone.value.name)
  loadingRecords.value = true
  try {
    const records = await cloudflareApi.getDnsRecords(currentZone.value.id)
    // 为每条记录添加 zone_id，因为 Cloudflare API 返回的记录不包含此字段
    dnsRecords.value = records.map(record => ({
      ...record,
      zone_id: currentZone.value!.id
    }))
  } catch (error) {
    toast.error('加载 DNS 记录失败')
  } finally {
    loadingRecords.value = false
  }
}

async function handleAddRecord() {
  if (!currentZone?.value?.id) {
    toast.error('未选择域名')
    return
  }

  submitting.value = true
  try {
    const recordToAdd = {
      zone_id: currentZone.value.id,
      ...dnsForm.value
    }

    // 为 TXT 记录自动添加引号（如果需要）
    if (recordToAdd.type === 'TXT' &&
        !recordToAdd.content.startsWith('"') &&
        !recordToAdd.content.endsWith('"')) {
      recordToAdd.content = `"${recordToAdd.content}"`
    }

    await cloudflareApi.createDnsRecord(recordToAdd)
    logHistory.dns('添加 DNS 记录', `${dnsForm.value.type} 记录：${dnsForm.value.name} → ${dnsForm.value.content}`)
    toast.success('DNS 记录已创建')
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
    toast.error(error?.message || '添加失败')
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
    logHistory.dns('更新 DNS 记录', `${editForm.value.type} 记录：${editForm.value.name}`)
    toast.success('DNS 记录已更新')
    showEditModal.value = false
    await loadDnsRecords()
  } catch (error: any) {
    toast.error(error?.message || '更新失败')
  } finally {
    submitting.value = false
  }
}

async function handleDelete(record: DnsRecord) {
  try {
    // 确保 zone_id 和 id 存在
    if (!record.zone_id || !record.id) {
      toast.error('记录信息不完整，无法删除')
      return
    }

    await cloudflareApi.deleteDnsRecord(currentZone.value!.id, record.id!)
    logHistory.dns('删除 DNS 记录', `${record.type} 记录：${record.name}`)
    toast.success('DNS 记录已删除')
    await loadDnsRecords()
  } catch (error: any) {
    toast.error(error?.message || '删除失败')
  }
}

// 批量导入相关函数
function handleFileUpload(options: any) {
  const file = options.file.file
  if (file) {
    uploadedFileName.value = file.name
    const reader = new FileReader()
    reader.onload = (e) => {
      batchImportText.value = e.target?.result as string
    }
    reader.readAsText(file)
  }
}

function handleParseBatchImport() {
  parseErrors.value = []
  parsedRecords.value = []

  if (!batchImportText.value.trim()) {
    parseErrors.value.push('请输入要导入的数据')
    return
  }

  const lines = batchImportText.value.trim().split('\n')

  lines.forEach((line, index) => {
    const lineNum = index + 1
    line = line.trim()

    if (!line || line.startsWith('#')) {
      return // 跳过空行和注释行
    }

    try {
      // 处理CSV格式：类型,名称,内容,TTL,是否代理,优先级
      const parts = line.split(',').map(p => p.trim())

      if (parts.length < 3) {
        parseErrors.value.push(`第 ${lineNum} 行: 格式不正确，至少需要类型、名称和内容`)
        return
      }

      const recordType = parts[0].toUpperCase()
      const name = parts[1]
      let content = parts[2]
      const ttl = parts[3] ? parseInt(parts[3]) : 1
      const proxied = parts[4] === 'true' || parts[4] === '1'
      const priority = parts[5] ? parseInt(parts[5]) : undefined

      // 处理TXT记录的引号
      if (recordType === 'TXT') {
        // 移除外部引号（如果有）
        if (content.startsWith('"') && content.endsWith('"')) {
          content = content.slice(1, -1)
        }
        // 确保有引号
        if (!content.startsWith('"')) {
          content = `"${content}"`
        }
      }

      // 验证记录类型
      const validTypes = ['A', 'AAAA', 'CNAME', 'MX', 'TXT', 'SRV', 'NS', 'CAA', 'PTR']
      if (!validTypes.includes(recordType)) {
        parseErrors.value.push(`第 ${lineNum} 行: 不支持的记录类型 "${recordType}"`)
        return
      }

      const record: DnsRecord = {
        type: recordType,
        name: name,
        content: content,
        ttl: ttl,
        proxied: proxied
      }

      if (priority !== undefined && (recordType === 'MX' || recordType === 'SRV')) {
        record.priority = priority
      }

      parsedRecords.value.push(record)
    } catch (error: any) {
      parseErrors.value.push(`第 ${lineNum} 行: 解析失败 - ${error.message}`)
    }
  })

  if (parsedRecords.value.length === 0 && parseErrors.value.length === 0) {
    parseErrors.value.push('没有找到有效的 DNS 记录')
  }
}

async function handleConfirmBatchImport() {
  if (!currentZone?.value?.id) {
    message.error('未选择域名')
    return
  }

  if (parsedRecords.value.length === 0) {
    message.warning('没有要导入的记录')
    return
  }

  batchImporting.value = true
  let successCount = 0
  let failCount = 0
  const errors: string[] = []

  try {
    for (const record of parsedRecords.value) {
      try {
        const recordWithZone = { ...record, zone_id: currentZone.value.id }
        await cloudflareApi.createDnsRecord(recordWithZone)
        successCount++
      } catch (error: any) {
        failCount++
        errors.push(`${record.name} (${record.type}): ${error.message || '导入失败'}`)
      }
    }

    if (successCount > 0) {
      message.success(`成功导入 ${successCount} 条记录${failCount > 0 ? `，失败 ${failCount} 条` : ''}`)
      await loadDnsRecords()
    }

    if (failCount > 0 && errors.length > 0) {
      console.error('批量导入错误:', errors)
      if (failCount === parsedRecords.value.length) {
        message.error(`所有记录导入失败，请检查格式和权限`)
      }
    }

    if (successCount > 0) {
      handleCancelBatchImport()
    }
  } catch (error: any) {
    message.error(error?.message || '批量导入失败')
  } finally {
    batchImporting.value = false
  }
}

function handleCancelBatchImport() {
  showBatchImportModal.value = false
  batchImportText.value = ''
  uploadedFileName.value = ''
  parsedRecords.value = []
  parseErrors.value = []
  importMethod.value = 'paste'
}

onMounted(() => {
  loadDnsRecords()
})

// 监听 currentZone 变化，自动重新加载 DNS 记录
watch(() => currentZone?.value?.id, () => {
  loadDnsRecords()
})
</script>
