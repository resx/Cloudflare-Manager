<template>
  <n-space vertical :size="24">
    <n-card title="域名列表">
      <template #header-extra>
        <n-button type="primary" disabled>
          添加域名
        </n-button>
      </template>

      <n-spin :show="loading">
        <n-data-table
          :columns="columns"
          :data="zones"
          :pagination="{ pageSize: 10 }"
          :bordered="false"
        />
      </n-spin>
    </n-card>
  </n-space>
</template>

<script setup lang="ts">
import { ref, onMounted, h } from 'vue'
import { useRouter } from 'vue-router'
import { useMessage, NButton, NSpace, NTag } from 'naive-ui'
import { cloudflareApi, type Zone } from '@/api'
import { useAccountStore } from '@/stores/account'

const router = useRouter()
const message = useMessage()
const accountStore = useAccountStore()

const loading = ref(false)
const zones = ref<Zone[]>([])

const columns = [
  { title: '域名', key: 'name', minWidth: 200 },
  {
    title: '状态',
    key: 'status',
    width: 100,
    render: (row: Zone) =>
      h(
        NTag,
        {
          type: row.status === 'active' ? 'success' : 'warning',
          size: 'small'
        },
        { default: () => (row.status === 'active' ? '活跃' : row.status) }
      )
  },
  {
    title: 'NS 服务器',
    key: 'name_servers',
    render: (row: Zone) =>
      h(
        'div',
        { style: { fontSize: '12px', color: '#666' } },
        row.name_servers.join(', ')
      )
  },
  {
    title: '操作',
    key: 'actions',
    width: 200,
    render: (row: Zone) =>
      h(
        NSpace,
        {},
        {
          default: () => [
            h(
              NButton,
              {
                size: 'small',
                onClick: () => {
                  // 设置当前域名
                  console.log('Setting currentZoneId to:', row.id, 'Zone name:', row.name)
                  localStorage.setItem('currentZoneId', row.id)
                  router.push('/dns')
                }
              },
              { default: () => 'DNS 记录' }
            ),
            h(
              NButton,
              {
                size: 'small',
                onClick: () => {
                  // 设置当前域名
                  console.log('Setting currentZoneId to:', row.id, 'Zone name:', row.name)
                  localStorage.setItem('currentZoneId', row.id)
                  router.push('/firewall')
                }
              },
              { default: () => '防火墙' }
            )
          ]
        }
      )
  }
]

async function loadZones() {
  if (!accountStore.currentAccount) {
    message.warning('请先添加账户')
    return
  }

  loading.value = true
  try {
    zones.value = await cloudflareApi.getZones()
    if (zones.value.length === 0) {
      message.info('当前账户下没有域名')
    }
  } catch (error: any) {
    message.error(error?.message || '加载域名列表失败')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadZones()
})
</script>
