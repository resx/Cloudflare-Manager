<template>
  <div class="animate-in space-y-6">
    <!-- Header -->
    <header class="flex justify-between items-end px-1">
      <div>
        <h1 class="text-3xl font-bold text-foreground">{{ t('dns.title') }}</h1>
        <p class="text-sm text-muted-foreground mt-1 font-medium">{{ t('dns.subtitle') }}</p>
      </div>
      <div class="flex gap-3">
        <IslandButton variant="secondary" @click="showBatchImportModal = true">
          <template #icon><component :is="DocumentTextOutline" class="w-4 h-4" /></template>
          {{ t('dns.batchImport') }}
        </IslandButton>
        <IslandButton @click="openAddModal">
          <template #icon><component :is="AddOutline" class="w-4 h-4" /></template>
          {{ t('dns.addRecord') }}
        </IslandButton>
      </div>
    </header>

    <!-- DNS Records Area -->
    <GlassCard :padding="0" class="overflow-hidden">
      <!-- Loading State -->
      <div v-if="loadingRecords" class="py-24 flex flex-col items-center justify-center">
        <div class="w-10 h-10 border-4 border-primary/20 border-t-primary rounded-full animate-spin"></div>
        <p class="mt-4 text-sm text-muted-foreground font-medium">{{ t('dns.loadingRecords') }}</p>
      </div>

      <!-- Records Table -->
      <div v-else-if="dnsRecords.length > 0" class="overflow-x-auto">
        <table class="w-full">
          <thead>
            <tr class="bg-foreground/5 text-muted-foreground text-xs font-bold uppercase tracking-wider border-b border-border/50">
              <th class="text-left py-4 px-6">{{ t('dns.type') }}</th>
              <th class="text-left py-4 px-6">{{ t('dns.name') }}</th>
              <th class="text-left py-4 px-6">{{ t('dns.content') }}</th>
              <th class="text-left py-4 px-6">{{ t('dns.ttl') }}</th>
              <th class="text-left py-4 px-6">{{ t('dns.proxyStatus') }}</th>
              <th class="text-right py-4 px-6">{{ t('dns.actions') }}</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-border/50">
            <tr 
              v-for="record in dnsRecords" 
              :key="record.id"
              class="hover:bg-foreground/[0.02] transition-colors group text-sm"
            >
              <td class="py-4 px-6">
                <GlassBadge :variant="getBadgeVariant(record.type)" class="font-mono">
                  {{ record.type }}
                </GlassBadge>
              </td>
              <td class="py-4 px-6 font-bold text-foreground">{{ record.name }}</td>
              <td class="py-4 px-6 max-w-xs">
                <div class="truncate text-muted-foreground font-mono text-xs" :title="record.content">
                  {{ formatContent(record) }}
                </div>
              </td>
              <td class="py-4 px-6 text-muted-foreground whitespace-nowrap">
                {{ formatTTL(record.ttl) }}
              </td>
              <td class="py-4 px-6">
                <div class="flex items-center gap-2">
                  <div 
                    :class="[
                      'w-2.5 h-2.5 rounded-full transition-all duration-500',
                      record.proxied ? 'bg-orange-500 shadow-[0_0_8px_rgba(249,115,22,0.6)]' : 'bg-slate-400 opacity-50'
                    ]"
                  ></div>
                  <span :class="record.proxied ? 'text-orange-500 font-bold' : 'text-muted-foreground font-medium'">
                    {{ record.proxied ? t('dns.proxied') : t('dns.dnsOnly') }}
                  </span>
                </div>
              </td>
              <td class="py-4 px-6 text-right">
                <div class="flex justify-end gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                  <IslandButton size="small" variant="ghost" @click="handleEdit(record)">{{ t('common.edit') }}</IslandButton>
                  <IslandButton size="small" variant="danger" @click="handleDelete(record)">{{ t('common.delete') }}</IslandButton>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Empty State -->
      <div v-else class="py-32 flex flex-col items-center text-center">
        <div class="w-16 h-16 rounded-2xl bg-muted flex items-center justify-center text-muted-foreground mb-4">
          <component :is="BuildOutline" class="w-8 h-8" />
        </div>
        <h3 class="text-xl font-bold">{{ t('dns.noRecords') }}</h3>
        <p class="text-sm text-muted-foreground mt-2 max-w-xs">
          {{ t('dns.noRecordsDesc') }}
        </p>
      </div>
    </GlassCard>

    <!-- Modals -->
    <n-modal v-model:show="showBatchImportModal" transform-origin="center">
      <GlassCard class="w-[800px] max-w-[95vw]" :padding="0">
        <div class="p-6 border-b border-border/50 flex justify-between items-center">
          <h2 class="text-xl font-bold">{{ t('dns.batchImportTitle') }}</h2>
          <button @click="showBatchImportModal = false" class="text-muted-foreground hover:text-foreground">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
          </button>
        </div>
        <div class="p-8 space-y-6 overflow-y-auto max-h-[70vh] custom-scrollbar">
          <div class="p-4 bg-primary/5 border border-primary/10 rounded-xl text-xs space-y-2">
            <div class="font-bold text-primary">{{ t('dns.formatHelp') }}</div>
            <p class="text-muted-foreground">{{ t('dns.formatDesc') }}</p>
            <div class="bg-foreground/5 p-3 rounded-lg font-mono text-muted-foreground leading-relaxed">
              A, www, 1.1.1.1, 3600, true<br/>
              CNAME, blog, example.com, 1, false
            </div>
          </div>

          <div class="space-y-4">
            <div class="flex gap-4">
              <button 
                v-for="m in [{id:'paste', label:t('dns.pasteText')}, {id:'file', label:t('dns.uploadFile')}]" 
                :key="m.id"
                @click="importMethod = m.id"
                :class="['px-4 py-2 rounded-lg text-sm font-bold transition-all', importMethod === m.id ? 'bg-primary text-white shadow-lg shadow-primary/20' : 'bg-muted text-muted-foreground hover:bg-muted/80']"
              >
                {{ m.label }}
              </button>
            </div>

            <textarea
              v-if="importMethod === 'paste'"
              v-model="batchImportText"
              class="w-full h-48 bg-foreground/5 border border-border/50 rounded-xl p-4 font-mono text-xs focus:ring-2 focus:ring-primary/20 outline-none"
              :placeholder="t('dns.pastePlaceholder')"
            ></textarea>

            <div v-if="importMethod === 'file'" class="border-2 border-dashed border-border/50 rounded-2xl p-12 text-center hover:border-primary/50 transition-colors">
              <input type="file" class="hidden" ref="fileInput" @change="handleFileUpload" accept=".csv,.txt">
              <div @click="$refs.fileInput?.click()" class="cursor-pointer space-y-2">
                <div class="text-primary font-bold">{{ t('dns.clickUpload') }}</div>
                <div class="text-xs text-muted-foreground">{{ uploadedFileName || t('dns.noFileSelected') }}</div>
              </div>
            </div>
          </div>

          <div v-if="parsedRecords.length > 0" class="space-y-3">
            <div class="text-sm font-bold flex items-center gap-2">
              <div class="w-1.5 h-4 bg-primary rounded-full"></div>
              {{ t('dns.preview', { count: parsedRecords.length }) }}
            </div>
            <div class="border border-border/50 rounded-xl overflow-hidden text-xs">
              <table class="w-full">
                <thead class="bg-foreground/5 text-muted-foreground">
                  <tr>
                    <th class="py-2 px-3 text-left">{{ t('dns.type') }}</th>
                    <th class="py-2 px-3 text-left">{{ t('dns.name') }}</th>
                    <th class="py-2 px-3 text-left">{{ t('dns.content') }}</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-border/30">
                  <tr v-for="(r, i) in parsedRecords.slice(0, 5)" :key="i">
                    <td class="py-2 px-3">{{ r.type }}</td>
                    <td class="py-2 px-3">{{ r.name }}</td>
                    <td class="py-2 px-3 truncate max-w-[200px]">{{ r.content }}</td>
                  </tr>
                </tbody>
              </table>
              <div v-if="parsedRecords.length > 5" class="p-2 bg-foreground/5 text-center text-muted-foreground italic">
                ... {{ t('dns.andMore', { count: parsedRecords.length - 5 }) }}
              </div>
            </div>
          </div>
        </div>
        <div class="p-6 border-t border-border/50 flex justify-end gap-3">
          <IslandButton variant="secondary" @click="handleCancelBatchImport">{{ t('common.cancel') }}</IslandButton>
          <IslandButton variant="ghost" @click="handleParseBatchImport" :disabled="!batchImportText">{{ t('dns.parse') }}</IslandButton>
          <IslandButton :loading="batchImporting" :disabled="parsedRecords.length === 0" @click="handleConfirmBatchImport">
            {{ t('dns.confirmImport') }}
          </IslandButton>
        </div>
      </GlassCard>
    </n-modal>

    <!-- Add/Edit Modal -->
    <n-modal
      v-model:show="showFormModal"
      transform-origin="center"
      :auto-focus="false"
      @after-enter="focusRecordTypeSelect"
    >
      <GlassCard
        class="w-[500px]"
        :padding="0"
        role="dialog"
        aria-modal="true"
        :aria-label="isEditing ? t('dns.editRecord') : t('dns.addRecord')"
      >
        <div class="p-6 border-b border-border/50 flex justify-between items-center">
          <h2 class="text-xl font-bold">{{ isEditing ? t('dns.editRecord') : t('dns.addRecord') }}</h2>
          <button @click="showFormModal = false" class="text-muted-foreground hover:text-foreground">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
          </button>
        </div>
        <div class="p-8 space-y-5">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <label class="text-xs font-bold ml-1">{{ t('dns.type') }}</label>
              <select ref="recordTypeSelectRef" v-model="dnsForm.type" class="w-full bg-foreground/5 border border-border/50 rounded-lg px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-primary/20">
                <option v-for="opt in recordTypeOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
              </select>
            </div>
            <div class="space-y-2">
              <label class="text-xs font-bold ml-1">{{ t('dns.ttl') }}</label>
              <select v-model="dnsForm.ttl" class="w-full bg-foreground/5 border border-border/50 rounded-lg px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-primary/20">
                <option v-for="opt in ttlOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
              </select>
            </div>
          </div>
          <div class="space-y-2">
            <label class="text-xs font-bold ml-1">{{ t('dns.name') }}</label>
            <input v-model="dnsForm.name" class="w-full" :placeholder="t('dns.namePlaceholder')">
          </div>
          <div class="space-y-2">
            <label class="text-xs font-bold ml-1">{{ t('dns.content') }}</label>
            <textarea v-model="dnsForm.content" class="w-full h-24 bg-foreground/5 border border-border/50 rounded-xl p-4 text-sm outline-none focus:ring-2 focus:ring-primary/20" :placeholder="t('dns.contentPlaceholder')"></textarea>
          </div>
          <div class="flex items-center justify-between p-4 bg-foreground/5 rounded-xl border border-border/50">
            <div class="flex items-center gap-3">
              <div :class="['w-10 h-6 rounded-full relative transition-colors duration-300 cursor-pointer', dnsForm.proxied ? 'bg-orange-500' : 'bg-slate-300']" @click="dnsForm.proxied = !dnsForm.proxied">
                <div :class="['w-4 h-4 bg-white rounded-full absolute top-1 transition-all duration-300', dnsForm.proxied ? 'left-5' : 'left-1']"></div>
              </div>
              <div>
                <div class="text-sm font-bold">{{ t('dns.proxy') }}</div>
                <div class="text-[10px] text-muted-foreground">{{ t('dns.proxyDesc') }}</div>
              </div>
            </div>
            <component :is="CloudOutline" :class="['w-6 h-6', dnsForm.proxied ? 'text-orange-500' : 'text-muted-foreground opacity-30']" />
          </div>
        </div>
        <div class="p-6 border-t border-border/50 flex justify-end gap-3">
          <IslandButton variant="secondary" @click="showFormModal = false">{{ t('common.cancel') }}</IslandButton>
          <IslandButton :loading="submitting" @click="handleSaveRecord">{{ t('common.save') }}</IslandButton>
        </div>
      </GlassCard>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, inject, type Ref, computed, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { NModal } from 'naive-ui'
import { cloudflareApi, type Zone, type DnsRecord } from '@/api'
import { toast } from '@/utils/toast'
import { logHistory } from '@/utils/history'
import GlassCard from '@/components/ui/GlassCard.vue'
import IslandButton from '@/components/ui/IslandButton.vue'
import GlassBadge from '@/components/ui/GlassBadge.vue'
import {
  AddOutline,
  DocumentTextOutline,
  BuildOutline,
  CloudOutline,
} from '@vicons/ionicons5'

const { t, locale } = useI18n()
const currentZone = inject<Ref<Zone | null>>('currentZone')

const loadingRecords = ref(false)
const submitting = ref(false)
const showFormModal = ref(false)
const isEditing = ref(false)
const editingId = ref<string | null>(null)
const recordTypeSelectRef = ref<HTMLSelectElement | null>(null)

// Forms
const dnsForm = ref({
  type: 'A',
  name: '',
  content: '',
  ttl: 1,
  proxied: true,
  priority: 10
})

// Batch Import
const showBatchImportModal = ref(false)
const batchImporting = ref(false)
const batchImportText = ref('')
const importMethod = ref('paste')
const uploadedFileName = ref('')
const parsedRecords = ref<DnsRecord[]>([])
const fileInput = ref<HTMLInputElement | null>(null)

const dnsRecords = ref<DnsRecord[]>([])

// Options
const recordTypeOptions = [
  { label: 'A', value: 'A' },
  { label: 'AAAA', value: 'AAAA' },
  { label: 'CNAME', value: 'CNAME' },
  { label: 'MX', value: 'MX' },
  { label: 'TXT', value: 'TXT' },
  { label: 'SRV', value: 'SRV' },
  { label: 'NS', value: 'NS' },
  { label: 'CAA', value: 'CAA' }
]

const ttlOptions = computed(() => [
  { label: t('dns.ttlAuto'), value: 1 },
  { label: `2 ${t('common.minutes')}`, value: 120 },
  { label: `5 ${t('common.minutes')}`, value: 300 },
  { label: `10 ${t('common.minutes')}`, value: 600 },
  { label: `15 ${t('common.minutes')}`, value: 900 },
  { label: `30 ${t('common.minutes')}`, value: 1800 },
  { label: `1 ${t('common.hours')}`, value: 3600 },
  { label: `1 ${t('common.days')}`, value: 86400 }
])

// Formatting Helpers
const getBadgeVariant = (type: string) => {
  const map: Record<string, any> = {
    'A': 'success',
    'AAAA': 'success',
    'CNAME': 'info',
    'MX': 'warning',
    'TXT': 'info',
  }
  return map[type] || 'info'
}

const formatContent = (record: DnsRecord) => {
  if (record.type === 'TXT' && record.content.startsWith('"') && record.content.endsWith('"')) {
    return record.content.slice(1, -1)
  }
  return record.content
}

const formatTTL = (ttl: number) => {
  if (ttl === 1) return t('dns.ttlAuto')
  if (ttl < 60) return `${ttl}s`
  if (ttl < 3600) return `${Math.floor(ttl / 60)}m`
  if (ttl < 86400) return `${Math.floor(ttl / 3600)}h`
  return `${Math.floor(ttl / 86400)}d`
}

// Actions
async function loadDnsRecords() {
  if (!currentZone?.value?.id) return

  loadingRecords.value = true
  try {
    const records = await cloudflareApi.getDnsRecords(currentZone.value.id)
    dnsRecords.value = records.map(r => ({ ...r, zone_id: currentZone.value!.id }))
  } catch (error) {
    toast.error(t('dns.loadFailed'))
  } finally {
    loadingRecords.value = false
  }
}

function openAddModal() {
  isEditing.value = false
  editingId.value = null
  dnsForm.value = {
    type: 'A',
    name: '',
    content: '',
    ttl: 1,
    proxied: true,
    priority: 10
  }
  blurActiveElement()
  showFormModal.value = true
}

function handleEdit(record: DnsRecord) {
  isEditing.value = true
  editingId.value = record.id || null
  dnsForm.value = {
    type: record.type,
    name: record.name,
    content: formatContent(record),
    ttl: record.ttl,
    proxied: record.proxied,
    priority: record.priority || 10
  }
  blurActiveElement()
  showFormModal.value = true
}

function blurActiveElement() {
  const activeElement = document.activeElement
  if (activeElement instanceof HTMLElement) {
    activeElement.blur()
  }
}

async function focusRecordTypeSelect() {
  await nextTick()
  recordTypeSelectRef.value?.focus({ preventScroll: true })
}

async function handleSaveRecord() {
  if (!currentZone?.value?.id) return
  if (!dnsForm.value.name || !dnsForm.value.content) {
    toast.error(t('common.fillRequired'))
    return
  }

  submitting.value = true
  try {
    const payload: any = {
      ...dnsForm.value,
      zone_id: currentZone.value.id
    }
    
    if (isEditing.value) {
      payload.id = editingId.value
      await cloudflareApi.updateDnsRecord(payload)
      toast.success(t('common.updateSuccess'))
    } else {
      await cloudflareApi.createDnsRecord(payload)
      toast.success(t('common.createSuccess'))
    }
    
    showFormModal.value = false
    await loadDnsRecords()
  } catch (error: any) {
    toast.error(error.message || t('common.updateFailed'))
  } finally {
    submitting.value = false
  }
}

async function handleDelete(record: DnsRecord) {
  if (!confirm(t('dns.deleteConfirm', { type: record.type, name: record.name }))) return
  
  try {
    await cloudflareApi.deleteDnsRecord(currentZone.value!.id, record.id!)
    toast.success(t('common.deleteSuccess'))
    await loadDnsRecords()
  } catch (error: any) {
    toast.error(error.message || t('common.deleteFailed'))
  }
}

// Batch Logic
function handleFileUpload(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0]
  if (file) {
    uploadedFileName.value = file.name
    const reader = new FileReader()
    reader.onload = (ev) => batchImportText.value = ev.target?.result as string
    reader.readAsText(file)
  }
}

function handleParseBatchImport() {
  const lines = batchImportText.value.trim().split('\n')
  parsedRecords.value = lines.map(line => {
    const parts = line.split(',').map(p => p.trim())
    return {
      type: parts[0] || 'A',
      name: parts[1] || '',
      content: parts[2] || '',
      ttl: parseInt(parts[3]) || 1,
      proxied: parts[4] === 'true'
    }
  }).filter(r => r.name && r.content)
}

async function handleConfirmBatchImport() {
  batchImporting.value = true
  try {
    for (const r of parsedRecords.value) {
      await cloudflareApi.createDnsRecord({ ...r, zone_id: currentZone.value!.id })
    }
    toast.success(t('dns.importSuccess', { count: parsedRecords.value.length }))
    showBatchImportModal.value = false
    await loadDnsRecords()
  } catch (error: any) {
    toast.error(t('dns.importFailed'))
  } finally {
    batchImporting.value = false
  }
}

function handleCancelBatchImport() {
  showBatchImportModal.value = false
  batchImportText.value = ''
  parsedRecords.value = []
}

onMounted(loadDnsRecords)
watch(() => currentZone?.value?.id, loadDnsRecords)
</script>
