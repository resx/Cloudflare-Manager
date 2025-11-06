<template>
  <n-space vertical :size="24">
    <!-- Universal SSL -->
    <n-card title="Universal SSL 证书">
      <template #header-extra>
        <n-button @click="loadSslCertificates" :loading="loading">
          <template #icon>
            <n-icon><RefreshOutline /></n-icon>
          </template>
          刷新
        </n-button>
      </template>

      <n-spin :show="loading">
        <n-descriptions :column="2" bordered>
          <n-descriptions-item label="证书状态">
            <n-tag :type="universalCert.status === 'active' ? 'success' : 'warning'">
              {{ universalCert.status }}
            </n-tag>
          </n-descriptions-item>
          <n-descriptions-item label="证书类型">
            {{ universalCert.type }}
          </n-descriptions-item>
          <n-descriptions-item label="颁发者">
            {{ universalCert.issuer }}
          </n-descriptions-item>
          <n-descriptions-item label="签名算法">
            {{ universalCert.signature }}
          </n-descriptions-item>
          <n-descriptions-item label="覆盖域名" :span="2">
            <span v-if="currentZone">
              *.{{ currentZone.name }}, {{ currentZone.name }}
            </span>
            <span v-else>请先在左侧菜单选择域名</span>
          </n-descriptions-item>
        </n-descriptions>
      </n-spin>
    </n-card>

    <!-- 自定义证书 -->
    <n-card title="自定义 SSL 证书">
      <template #header-extra>
        <n-space>
          <n-button @click="loadCustomCertificates" :loading="customLoading">
            <template #icon>
              <n-icon><RefreshOutline /></n-icon>
            </template>
            刷新
          </n-button>
          <n-button type="primary" @click="showUploadModal = true">
            <template #icon>
              <n-icon><CloudUploadOutline /></n-icon>
            </template>
            上传证书
          </n-button>
        </n-space>
      </template>

      <n-alert type="info" style="margin-bottom: 16px">
        自定义证书功能需要 Business 或 Enterprise 计划。免费版和 Pro 计划将返回错误。
      </n-alert>

      <n-spin :show="customLoading">
        <n-data-table
          v-if="customCertificates.length > 0"
          :columns="certColumns"
          :data="customCertificates"
          :pagination="false"
          :bordered="false"
        />
        <n-empty v-else description="暂无自定义证书" />
      </n-spin>
    </n-card>

    <!-- 上传证书弹窗 -->
    <n-modal
      v-model:show="showUploadModal"
      preset="card"
      title="上传自定义证书"
      style="width: 700px"
      :bordered="false"
      :segmented="{
        content: 'soft',
        footer: 'soft'
      }"
    >
      <n-form
        ref="uploadFormRef"
        :model="uploadForm"
        label-placement="top"
        require-mark-placement="left"
      >
        <n-alert type="warning" style="margin-bottom: 16px">
          仅支持 Business 和 Enterprise 计划。请确保证书和私钥均为 PEM 格式。
        </n-alert>

        <n-form-item label="证书内容 (PEM 格式)" path="certificate" required>
          <n-input
            v-model:value="uploadForm.certificate"
            type="textarea"
            placeholder="-----BEGIN CERTIFICATE-----&#10;...&#10;-----END CERTIFICATE-----"
            :rows="8"
          />
        </n-form-item>

        <n-form-item label="私钥 (PEM 格式)" path="privateKey" required>
          <n-input
            v-model:value="uploadForm.privateKey"
            type="textarea"
            placeholder="-----BEGIN PRIVATE KEY-----&#10;...&#10;-----END PRIVATE KEY-----"
            :rows="8"
          />
        </n-form-item>

        <n-form-item label="打包方法 (Bundle Method)" path="bundleMethod">
          <n-select
            v-model:value="uploadForm.bundleMethod"
            :options="bundleMethodOptions"
            placeholder="选择打包方法（可选）"
          />
          <template #feedback>
            <n-text depth="3" style="font-size: 12px">
              ubiquitous: 最大兼容性 | optimal: 最佳性能 | force: 仅使用上传的证书
            </n-text>
          </template>
        </n-form-item>
      </n-form>

      <template #footer>
        <n-space justify="end">
          <n-button @click="showUploadModal = false">取消</n-button>
          <n-button type="primary" :loading="uploading" @click="handleUpload">
            上传
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, inject, watch, computed, h, type Ref } from 'vue'
import { NButton, NSpace, NTag, NIcon, useMessage, useDialog } from 'naive-ui'
import { RefreshOutline, CloudUploadOutline, TrashOutline } from '@vicons/ionicons5'
import { cloudflareApi, type Zone, type CustomCertificate } from '@/api'

const message = useMessage()
const dialog = useDialog()

const loading = ref(false)
const customLoading = ref(false)
const uploading = ref(false)
const showUploadModal = ref(false)

// 从 Layout 获取当前域名
const currentZone = inject<Ref<Zone | null>>('currentZone')

// Universal SSL 信息
const universalCert = ref({
  status: 'active',
  type: 'Universal SSL',
  issuer: 'Let\'s Encrypt',
  signature: 'SHA256-RSA'
})

// 自定义证书列表
const customCertificates = ref<CustomCertificate[]>([])

// 上传表单
const uploadForm = ref({
  certificate: '',
  privateKey: '',
  bundleMethod: 'ubiquitous'
})

const bundleMethodOptions = [
  { label: 'Ubiquitous - 最大兼容性', value: 'ubiquitous' },
  { label: 'Optimal - 最佳性能', value: 'optimal' },
  { label: 'Force - 仅使用上传的证书', value: 'force' }
]

// 证书表格列
const certColumns = computed(() => [
  {
    title: 'ID',
    key: 'id',
    width: 150,
    ellipsis: {
      tooltip: true
    }
  },
  {
    title: '状态',
    key: 'status',
    width: 100,
    render: (row: CustomCertificate) => {
      const type = row.status === 'active' ? 'success' : row.status === 'pending' ? 'warning' : 'error'
      return h(NTag, { type }, { default: () => row.status })
    }
  },
  {
    title: '颁发者',
    key: 'issuer',
    width: 200
  },
  {
    title: '覆盖域名',
    key: 'hosts',
    width: 250,
    render: (row: CustomCertificate) => {
      return row.hosts.join(', ')
    }
  },
  {
    title: '过期时间',
    key: 'expires_on',
    width: 180,
    render: (row: CustomCertificate) => {
      return new Date(row.expires_on).toLocaleString('zh-CN')
    }
  },
  {
    title: '上传时间',
    key: 'uploaded_on',
    width: 180,
    render: (row: CustomCertificate) => {
      return new Date(row.uploaded_on).toLocaleString('zh-CN')
    }
  },
  {
    title: '操作',
    key: 'actions',
    width: 120,
    render: (row: CustomCertificate) => {
      return h(
        NButton,
        {
          size: 'small',
          type: 'error',
          onClick: () => handleDeleteCertificate(row.id)
        },
        {
          default: () => '删除',
          icon: () => h(NIcon, {}, { default: () => h(TrashOutline) })
        }
      )
    }
  }
])

async function loadSslCertificates() {
  if (!currentZone?.value?.id) {
    return
  }

  loading.value = true
  try {
    const certificates = await cloudflareApi.getSslCertificates(currentZone.value.id)
    if (certificates && certificates.length > 0) {
      const cert = certificates[0]
      universalCert.value = {
        status: cert.status,
        type: cert.type,
        issuer: cert.certificates?.[0]?.issuer || 'Let\'s Encrypt',
        signature: cert.certificates?.[0]?.signature || 'SHA256-RSA'
      }
    }
  } catch (error: any) {
    console.error('Load SSL certificates error:', error)
    // Fail silently, keep default values
  } finally {
    loading.value = false
  }
}

async function loadCustomCertificates() {
  if (!currentZone?.value?.id) {
    return
  }

  customLoading.value = true
  try {
    customCertificates.value = await cloudflareApi.getCustomCertificates(currentZone.value.id)
    message.success(`成功加载 ${customCertificates.value.length} 个自定义证书`)
  } catch (error: any) {
    message.warning(error?.message || '加载自定义证书失败（可能需要 Business 计划）')
    console.error('Load custom certificates error:', error)
    customCertificates.value = []
  } finally {
    customLoading.value = false
  }
}

async function handleUpload() {
  if (!currentZone?.value?.id) {
    message.warning('请先选择一个域名')
    return
  }

  if (!uploadForm.value.certificate || !uploadForm.value.privateKey) {
    message.warning('请填写证书和私钥')
    return
  }

  uploading.value = true
  try {
    await cloudflareApi.uploadCustomCertificate({
      zone_id: currentZone.value.id,
      certificate: uploadForm.value.certificate,
      private_key: uploadForm.value.privateKey,
      bundle_method: uploadForm.value.bundleMethod || undefined
    })
    message.success('证书上传成功')
    showUploadModal.value = false
    uploadForm.value = {
      certificate: '',
      privateKey: '',
      bundleMethod: 'ubiquitous'
    }
    await loadCustomCertificates()
  } catch (error: any) {
    message.error(error?.message || '证书上传失败')
    console.error('Upload certificate error:', error)
  } finally {
    uploading.value = false
  }
}

function handleDeleteCertificate(certificateId: string) {
  dialog.warning({
    title: '确认删除',
    content: '确定要删除此证书吗？此操作不可撤销。',
    positiveText: '删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      if (!currentZone?.value?.id) {
        message.warning('请先选择一个域名')
        return
      }

      try {
        await cloudflareApi.deleteCustomCertificate(currentZone.value.id, certificateId)
        message.success('证书删除成功')
        await loadCustomCertificates()
      } catch (error: any) {
        message.error(error?.message || '删除证书失败')
        console.error('Delete certificate error:', error)
      }
    }
  })
}

onMounted(() => {
  loadSslCertificates()
  loadCustomCertificates()
})

// 监听 currentZone 变化
watch(() => currentZone?.value?.id, () => {
  loadSslCertificates()
  loadCustomCertificates()
})
</script>
