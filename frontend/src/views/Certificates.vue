<template>
  <n-space vertical :size="24">
    <!-- Universal SSL -->
    <n-card title="Universal SSL 证书">
      <n-spin :show="loading">
        <n-descriptions :column="2" bordered>
          <n-descriptions-item label="证书状态">
            <n-tag type="success">有效</n-tag>
          </n-descriptions-item>
          <n-descriptions-item label="证书类型">
            Universal SSL
          </n-descriptions-item>
          <n-descriptions-item label="颁发者">
            Let's Encrypt
          </n-descriptions-item>
          <n-descriptions-item label="有效期">
            90 天（自动续期）
          </n-descriptions-item>
          <n-descriptions-item label="覆盖域名" :span="2">
            *.{{ currentDomain }}, {{ currentDomain }}
          </n-descriptions-item>
        </n-descriptions>
      </n-spin>
    </n-card>

    <!-- 自定义证书 -->
    <n-card title="自定义 SSL 证书">
      <template #header-extra>
        <n-button type="primary" @click="showUploadModal = true" disabled>
          上传证书
        </n-button>
      </template>

      <n-alert type="info" style="margin-bottom: 16px">
        自定义证书功能需要 Business 或 Enterprise 计划
      </n-alert>

      <n-empty description="暂无自定义证书" />
    </n-card>

    <!-- 证书验证设置 -->
    <n-card title="证书验证">
      <n-space vertical :size="16">
        <n-form-item label="CAA 记录检查">
          <n-switch disabled>
            <template #checked>已启用</template>
            <template #unchecked>已禁用</template>
          </n-switch>
          <template #feedback>
            <n-text depth="3" style="font-size: 12px">
              CAA (Certification Authority Authorization) 记录用于指定哪些证书颁发机构可以为您的域名颁发证书
            </n-text>
          </template>
        </n-form-item>

        <n-form-item label="证书透明度">
          <n-switch disabled checked>
            <template #checked>已启用</template>
            <template #unchecked>已禁用</template>
          </n-switch>
          <template #feedback>
            <n-text depth="3" style="font-size: 12px">
              证书透明度（Certificate Transparency）提高 SSL/TLS 证书的安全性
            </n-text>
          </template>
        </n-form-item>
      </n-space>
    </n-card>

    <!-- 上传证书弹窗 -->
    <n-modal v-model:show="showUploadModal" preset="dialog" title="上传自定义证书" style="width: 600px">
      <n-form label-placement="top">
        <n-form-item label="证书内容 (PEM 格式)">
          <n-input
            type="textarea"
            placeholder="-----BEGIN CERTIFICATE-----&#10;...&#10;-----END CERTIFICATE-----"
            :rows="6"
          />
        </n-form-item>

        <n-form-item label="私钥 (PEM 格式)">
          <n-input
            type="textarea"
            placeholder="-----BEGIN PRIVATE KEY-----&#10;...&#10;-----END PRIVATE KEY-----"
            :rows="6"
          />
        </n-form-item>

        <n-form-item label="证书链 (可选)">
          <n-input
            type="textarea"
            placeholder="-----BEGIN CERTIFICATE-----&#10;...&#10;-----END CERTIFICATE-----"
            :rows="6"
          />
        </n-form-item>
      </n-form>

      <template #action>
        <n-space>
          <n-button @click="showUploadModal = false">取消</n-button>
          <n-button type="primary" disabled>
            上传（Business 计划功能）
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useMessage } from 'naive-ui'

const message = useMessage()
const loading = ref(false)
const showUploadModal = ref(false)

const currentDomain = computed(() => {
  return 'example.com'
})

onMounted(() => {
  // 加载证书信息
})
</script>
