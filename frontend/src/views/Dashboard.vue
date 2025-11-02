<template>
  <n-space vertical :size="24">
    <n-card title="欢迎使用 Cloudflare 可视化管理平台">
      <n-space vertical>
        <n-text>
          这是一款专为 Cloudflare 用户打造的全功能可视化管理平台,让复杂的 CDN 配置变得简单直观。
        </n-text>
        <n-divider />
        <n-grid :cols="3" :x-gap="24">
          <n-gi>
            <n-statistic label="管理的域名" :value="zones.length" />
          </n-gi>
          <n-gi>
            <n-statistic label="DNS 记录" :value="totalDnsRecords" />
          </n-gi>
          <n-gi>
            <n-statistic label="账户数量" :value="accountStore.accounts.length" />
          </n-gi>
        </n-grid>
      </n-space>
    </n-card>

    <n-card title="域名列表">
      <n-spin :show="loading">
        <n-data-table
          :columns="columns"
          :data="zones"
          :pagination="false"
          :bordered="false"
        />
      </n-spin>
    </n-card>

    <n-card title="快速操作">
      <n-space>
        <n-button type="primary" @click="$router.push('/quick-deploy')">
          一键加速部署
        </n-button>
        <n-button type="info" @click="$router.push('/optimize')">
          自动优化配置
        </n-button>
        <n-button @click="$router.push('/dns')">
          管理 DNS 记录
        </n-button>
        <n-button @click="$router.push('/firewall')">
          配置防火墙
        </n-button>
      </n-space>
    </n-card>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, h } from 'vue'
import { NTag } from 'naive-ui'
import { cloudflareApi, type Zone } from '@/api'
import { useAccountStore } from '@/stores/account'

const accountStore = useAccountStore()
const loading = ref(false)
const zones = ref<Zone[]>([])
const totalDnsRecords = ref(0)

const columns = [
  { title: '域名', key: 'name' },
  {
    title: '状态',
    key: 'status',
    render: (row: Zone) =>
      h(
        NTag,
        {
          type: row.status === 'active' ? 'success' : 'warning'
        },
        { default: () => row.status }
      )
  },
  {
    title: 'Name Servers',
    key: 'name_servers',
    render: (row: Zone) => row.name_servers?.join(', ') || '-'
  }
]

async function loadZones() {
  if (!accountStore.currentAccount) return

  loading.value = true
  try {
    zones.value = await cloudflareApi.getZones()

    // 统计 DNS 记录总数
    let total = 0
    for (const zone of zones.value) {
      const records = await cloudflareApi.getDnsRecords(zone.id)
      total += records.length
    }
    totalDnsRecords.value = total
  } catch (error) {
    console.error('Failed to load zones:', error)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadZones()
})
</script>
