<template>
  <n-space vertical :size="24">
    <n-card title="一键加速部署">
      <n-alert type="info" style="margin-bottom: 24px">
        通过 Cloudflare Worker 为您的网站提供全球加速服务,30 秒完成部署!
      </n-alert>

      <n-form
        ref="formRef"
        :model="deployForm"
        :rules="formRules"
        label-placement="left"
        label-width="120"
        require-mark-placement="left"
      >
        <n-form-item label="选择域名" path="zoneId">
          <n-select
            v-model:value="deployForm.zoneId"
            :options="zoneOptions"
            placeholder="请选择要加速的域名"
            :loading="loadingZones"
          />
        </n-form-item>

        <n-form-item label="Worker 名称" path="scriptName">
          <n-input
            v-model:value="deployForm.scriptName"
            placeholder="例如: my-cdn-worker"
          />
        </n-form-item>

        <n-form-item label="目标网站" path="targetUrl">
          <n-input
            v-model:value="deployForm.targetUrl"
            placeholder="https://example.com"
          />
          <template #feedback>
            需要加速的源站地址
          </template>
        </n-form-item>

        <n-form-item label="访问域名" path="accessDomain">
          <n-input
            v-model:value="deployForm.accessDomain"
            placeholder="cdn.yourdomain.com"
          />
          <template #feedback>
            用户访问的域名(需提前配置 DNS)
          </template>
        </n-form-item>

        <n-form-item label="CDN 节点" path="cdnNode">
          <n-select
            v-model:value="deployForm.cdnNode"
            :options="cdnNodeOptions"
          />
        </n-form-item>

        <n-form-item label="缓存时间" path="cacheTtl">
          <n-input-number
            v-model:value="deployForm.cacheTtl"
            :min="0"
            :max="31536000"
            style="width: 100%"
          >
            <template #suffix>秒</template>
          </n-input-number>
          <template #feedback>
            0 表示不缓存,最大 31536000 秒(1年)
          </template>
        </n-form-item>

        <n-form-item label="授权码" path="authCode">
          <n-input
            v-model:value="deployForm.authCode"
            type="password"
            placeholder="请输入授权码"
          />
          <template #feedback>
            默认授权码: 1111
          </template>
        </n-form-item>

        <n-form-item>
          <n-space>
            <n-button
              type="primary"
              size="large"
              :loading="deploying"
              @click="handleDeploy"
            >
              一键部署
            </n-button>
            <n-button size="large" @click="handleReset">
              重置
            </n-button>
          </n-space>
        </n-form-item>
      </n-form>
    </n-card>

    <n-card title="部署说明">
      <n-steps vertical>
        <n-step title="准备工作">
          确保您已经将域名添加到 Cloudflare 并正确配置了 DNS 记录
        </n-step>
        <n-step title="填写信息">
          填写目标网站地址、访问域名等信息
        </n-step>
        <n-step title="一键部署">
          点击部署按钮,系统将自动创建 Worker 并完成配置
        </n-step>
        <n-step title="验证生效">
          访问您的加速域名,验证加速是否生效
        </n-step>
      </n-steps>
    </n-card>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useMessage } from 'naive-ui'
import { cloudflareApi, type Zone } from '@/api'
import { useAccountStore } from '@/stores/account'

const accountStore = useAccountStore()
const message = useMessage()

const loadingZones = ref(false)
const deploying = ref(false)
const zones = ref<Zone[]>([])

const deployForm = ref({
  zoneId: '',
  scriptName: '',
  targetUrl: '',
  accessDomain: '',
  cdnNode: 'cdns.doon.eu.org',
  cacheTtl: 3600,
  authCode: ''
})

const formRules = {
  zoneId: { required: true, message: '请选择域名', trigger: 'change' },
  scriptName: { required: true, message: '请输入 Worker 名称', trigger: 'blur' },
  targetUrl: { required: true, message: '请输入目标网站', trigger: 'blur' },
  accessDomain: { required: true, message: '请输入访问域名', trigger: 'blur' },
  cdnNode: { required: true, message: '请选择 CDN 节点', trigger: 'change' },
  authCode: { required: true, message: '请输入授权码', trigger: 'blur' }
}

const zoneOptions = computed(() =>
  zones.value.map(zone => ({
    label: zone.name,
    value: zone.id
  }))
)

const cdnNodeOptions = [
  { label: 'cdns.doon.eu.org (优选节点1)', value: 'cdns.doon.eu.org' },
  { label: 'cloudflare.182682.xyz (优选节点2)', value: 'cloudflare.182682.xyz' }
]

async function loadZones() {
  if (!accountStore.currentAccount) return

  loadingZones.value = true
  try {
    zones.value = await cloudflareApi.getZones()
  } catch (error) {
    message.error('加载域名列表失败')
  } finally {
    loadingZones.value = false
  }
}

async function handleDeploy() {
  // 验证授权码
  if (deployForm.value.authCode !== '1111') {
    message.error('授权码错误')
    return
  }

  deploying.value = true
  try {
    const result = await cloudflareApi.deployWorker({
      zone_id: deployForm.value.zoneId,
      script_name: deployForm.value.scriptName,
      target_url: deployForm.value.targetUrl,
      access_domain: deployForm.value.accessDomain,
      cdn_node: deployForm.value.cdnNode,
      cache_ttl: deployForm.value.cacheTtl
    })

    message.success('部署成功! Worker 已创建并配置完成')
    handleReset()
  } catch (error: any) {
    message.error(error?.message || '部署失败,请检查配置')
  } finally {
    deploying.value = false
  }
}

function handleReset() {
  deployForm.value = {
    zoneId: '',
    scriptName: '',
    targetUrl: '',
    accessDomain: '',
    cdnNode: 'cdns.doon.eu.org',
    cacheTtl: 3600,
    authCode: ''
  }
}

onMounted(() => {
  loadZones()
})
</script>
