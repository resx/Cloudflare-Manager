<template>
  <div class="animate-in space-y-8">
    <!-- Header -->
    <header class="flex flex-col md:flex-row justify-between items-start md:items-end gap-4 px-1">
      <div>
        <h1 class="text-3xl font-bold text-foreground tracking-tight">{{ t('accounts.title') }}</h1>
        <p class="text-sm text-muted-foreground mt-1 font-medium italic">{{ t('accounts.subtitle') }}</p>
      </div>
      <div class="flex flex-wrap gap-2">
        <IslandButton variant="ghost" size="small" @click="exportAccounts">
          <template #icon><component :is="CloudUploadOutline" class="w-4 h-4" /></template>
          {{ t('accounts.export') }}
        </IslandButton>
        <IslandButton variant="ghost" size="small" @click="showImportModal = true">
          <template #icon><component :is="CloudDownloadOutline" class="w-4 h-4" /></template>
          {{ t('accounts.import') }}
        </IslandButton>
        <IslandButton @click="showAddModal = true">
          <template #icon><component :is="AddOutline" class="w-4 h-4" /></template>
          {{ t('accounts.addAccount') }}
        </IslandButton>
      </div>
    </header>

    <!-- Privacy Alert Card -->
    <GlassCard class="bg-primary/5 border-primary/20" :padding="6">
      <div class="flex gap-5 items-start">
        <div class="w-12 h-12 rounded-2xl bg-primary/20 flex items-center justify-center text-primary shrink-0 shadow-inner">
          <component :is="ShieldCheckmarkOutline" class="w-6 h-6" />
        </div>
        <div class="space-y-1">
          <h3 class="font-bold text-primary">{{ t('accounts.privacyTitle') }}</h3>
          <p class="text-sm text-muted-foreground leading-relaxed" v-html="t('accounts.privacyDesc')">
          </p>
          <div class="flex gap-4 mt-3">
            <div class="flex items-center gap-1.5 text-[10px] font-bold text-emerald-600 dark:text-emerald-400">
              <div class="w-1.5 h-1.5 rounded-full bg-current"></div>
              {{ t('accounts.privacyRiskFree') }}
            </div>
            <div class="flex items-center gap-1.5 text-[10px] font-bold text-emerald-600 dark:text-emerald-400">
              <div class="w-1.5 h-1.5 rounded-full bg-current"></div>
              {{ t('accounts.privacyControl') }}
            </div>
          </div>
        </div>
      </div>
    </GlassCard>

    <!-- Accounts Grid -->
    <div v-if="accountStore.accounts.length > 0" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
      <GlassCard 
        v-for="account in accountStore.accounts" 
        :key="account.id"
        :class="[
          'group relative overflow-hidden transition-all duration-500 hover:scale-[1.02]',
          accountStore.currentAccount?.id === account.id ? 'ring-2 ring-primary border-primary/50 bg-primary/[0.02]' : 'hover:border-primary/30'
        ]"
        :padding="0"
        @click="switchAccount(account.id)"
      >
        <!-- Selection Glow -->
        <div v-if="accountStore.currentAccount?.id === account.id" class="absolute -top-12 -right-12 w-24 h-24 bg-primary/20 blur-3xl rounded-full"></div>

        <div class="p-6">
          <div class="flex items-start justify-between mb-6">
            <div 
              class="w-14 h-14 rounded-2xl flex items-center justify-center text-xl font-black shadow-lg transition-transform group-hover:rotate-3" 
              :style="{ background: `linear-gradient(135deg, ${getAccountColor(account.id)}, ${getAccountColor(account.id)}dd)`, color: '#fff' }"
            >
              {{ account.alias[0].toUpperCase() }}
            </div>
            <div class="flex gap-1">
              <button 
                @click.stop="editAccount(account)"
                class="p-2 text-muted-foreground hover:text-primary hover:bg-primary/10 rounded-lg transition-all"
                :title="t('common.edit')"
              >
                <component :is="BuildOutline" class="w-4 h-4" />
              </button>
              <button 
                @click.stop="copyId(account.accountId)"
                class="p-2 text-muted-foreground hover:text-primary hover:bg-primary/10 rounded-lg transition-all"
                :title="t('accounts.copyId')"
              >
                <component :is="CopyOutline" class="w-4 h-4" />
              </button>
              <button 
                @click.stop="deleteAccount(account.id)"
                class="p-2 text-muted-foreground hover:text-danger hover:bg-danger/10 rounded-lg transition-all"
                :title="t('accounts.remove')"
              >
                <component :is="TrashOutline" class="w-4 h-4" />
              </button>
            </div>
          </div>

          <div class="space-y-1">
            <h3 class="font-bold text-lg text-foreground truncate pr-8">{{ account.alias }}</h3>
            <p class="text-xs text-muted-foreground font-mono truncate opacity-60">{{ account.accountId || t('accounts.idNotLoaded') }}</p>
          </div>

          <div class="mt-6 pt-6 border-t border-border/30 space-y-3">
            <div class="flex items-center justify-between text-xs">
              <span class="text-muted-foreground font-medium">{{ t('accounts.authMethod') }}</span>
              <span class="text-foreground font-mono bg-foreground/5 px-2 py-0.5 rounded">API Token</span>
            </div>
            <div class="flex items-center justify-between text-xs">
              <span class="text-muted-foreground font-medium">{{ t('accounts.tokenMask') }}</span>
              <span class="text-foreground font-mono tracking-wider">{{ maskToken(account.apiToken) }}</span>
            </div>
          </div>

          <div class="mt-6">
            <div v-if="accountStore.currentAccount?.id === account.id" class="flex items-center justify-center gap-2 py-2.5 bg-primary text-primary-foreground rounded-xl text-xs font-bold shadow-lg shadow-primary/20">
              <component :is="CheckmarkCircleOutline" class="w-4 h-4" />
              {{ t('accounts.active') }}
            </div>
            <IslandButton v-else variant="ghost" class="w-full" @click.stop="switchAccount(account.id)">
              {{ t('accounts.switchTo') }}
            </IslandButton>
          </div>
        </div>
      </GlassCard>
    </div>

    <!-- Empty State -->
    <div v-else class="py-32 flex flex-col items-center text-center">
      <GlassCard class="p-12 max-w-md">
        <div class="w-20 h-20 rounded-3xl bg-muted flex items-center justify-center text-muted-foreground mb-6 mx-auto">
          <component :is="PersonOutline" class="w-10 h-10" />
        </div>
        <h3 class="text-2xl font-bold">{{ t('accounts.noAccounts') }}</h3>
        <p class="text-sm text-muted-foreground mt-3 leading-relaxed">
          {{ t('accounts.noAccountsDesc') }}
        </p>
        <IslandButton class="mt-8 w-full" @click="showAddModal = true">
          {{ t('accounts.getStarted') }}
        </IslandButton>
      </GlassCard>
    </div>

    <!-- Modals -->
    <n-modal v-model:show="showModal">
      <GlassCard class="w-[500px] max-w-[95vw]" :padding="0">
        <div class="p-6 border-b border-border/50 flex justify-between items-center">
          <h2 class="text-xl font-bold">{{ editingId ? t('accounts.editTitle') : t('accounts.addTitle') }}</h2>
          <button @click="closeModal" class="text-muted-foreground hover:text-foreground p-1">
            <component :is="CloseOutline" class="w-5 h-5" />
          </button>
        </div>
        
        <div class="p-8 space-y-6">
          <div class="p-4 bg-amber-500/10 border border-amber-500/20 rounded-2xl text-amber-600 dark:text-amber-400 text-xs leading-relaxed flex gap-3">
            <component :is="ShieldOutline" class="w-5 h-5 shrink-0" />
            <div>
              <strong>{{ t('accounts.securityTipTitle') }}</strong> 
              {{ t('accounts.securityTipDesc') }}
            </div>
          </div>

          <div class="space-y-4">
            <div class="space-y-2">
              <label class="text-xs font-bold ml-1 uppercase tracking-widest text-muted-foreground">API Token</label>
              <input
                v-model="accountForm.apiToken"
                type="password"
                class="w-full bg-foreground/5 border border-border/50 rounded-xl px-4 py-3 text-sm focus:ring-2 focus:ring-primary/30 outline-none transition-all"
                placeholder="Cloudflare API Token"
              />
            </div>

            <div class="space-y-2">
              <label class="text-xs font-bold ml-1 uppercase tracking-widest text-muted-foreground">{{ t('accounts.alias') }}</label>
              <input
                v-model="accountForm.alias"
                class="w-full bg-foreground/5 border border-border/50 rounded-xl px-4 py-3 text-sm focus:ring-2 focus:ring-primary/30 outline-none transition-all"
                :placeholder="t('accounts.aliasPlaceholder')"
              />
            </div>
          </div>
        </div>

        <div class="p-6 border-t border-border/50 flex justify-end gap-3">
          <IslandButton variant="secondary" @click="closeModal">{{ t('common.cancel') }}</IslandButton>
          <IslandButton @click="handleSaveAccount" :loading="saving">
            {{ editingId ? t('common.save') : t('common.confirm') }}
          </IslandButton>
        </div>
      </GlassCard>
    </n-modal>

    <n-modal v-model:show="showImportModal">
      <GlassCard class="w-[600px] max-w-[95vw]" :padding="0">
        <div class="p-6 border-b border-border/50 flex justify-between items-center">
          <h2 class="text-xl font-bold">{{ t('accounts.importTitle') }}</h2>
          <button @click="showImportModal = false" class="text-muted-foreground hover:text-foreground">
            <component :is="CloseOutline" class="w-5 h-5" />
          </button>
        </div>
        
        <div class="p-8">
          <label class="text-xs font-bold mb-3 block text-muted-foreground uppercase tracking-widest">{{ t('accounts.backupJson') }}</label>
          <textarea
            v-model="importText"
            class="w-full h-64 bg-slate-950 text-emerald-500 border border-border/50 rounded-2xl p-6 font-mono text-xs focus:outline-none custom-scrollbar shadow-inner"
            :placeholder="t('accounts.importPlaceholder')"
          ></textarea>
          <p class="mt-4 text-[10px] text-muted-foreground italic">
            * {{ t('accounts.importNote') }}
          </p>
        </div>

        <div class="p-6 border-t border-border/50 flex justify-end gap-3">
          <IslandButton variant="secondary" @click="showImportModal = false">{{ t('common.cancel') }}</IslandButton>
          <IslandButton @click="handleImport">{{ t('accounts.startImport') }}</IslandButton>
        </div>
      </GlassCard>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NModal } from 'naive-ui'
import { 
  CloudUploadOutline, 
  CloudDownloadOutline, 
  KeyOutline, 
  FingerPrintOutline, 
  PersonOutline,
  ShieldOutline,
  ShieldCheckmarkOutline,
  AddOutline,
  CopyOutline,
  TrashOutline,
  CheckmarkCircleOutline,
  CloseOutline,
  BuildOutline
} from '@vicons/ionicons5'
import { useAccountStore } from '@/stores/account'
import { toast } from '@/utils/toast'
import GlassCard from '@/components/ui/GlassCard.vue'
import IslandButton from '@/components/ui/IslandButton.vue'
import GlassBadge from '@/components/ui/GlassBadge.vue'

const { t } = useI18n()
const accountStore = useAccountStore()
const showAddModal = ref(false)
const editingId = ref<string | null>(null)
const showImportModal = ref(false)
const saving = ref(false)
const importText = ref('')

const showModal = computed({
  get: () => showAddModal.value || !!editingId.value,
  set: (val) => { if(!val) closeModal() }
})

const accountForm = ref({
  apiToken: '',
  alias: ''
})

const accountColors = [
  '#3b82f6', // blue
  '#8b5cf6', // violet
  '#10b981', // emerald
  '#f43f5e', // rose
  '#f59e0b', // amber
  '#06b6d4', // cyan
]

function getAccountColor(accountId: string): string {
  const index = accountId ? (accountId.charCodeAt(accountId.length - 1) % accountColors.length) : 0
  return accountColors[index]
}

function maskToken(token: string): string {
  if (!token) return ''
  return token.substring(0, 8) + ' •••• ' + token.substring(token.length - 4)
}

function switchAccount(accountId: string) {
  accountStore.switchAccount(accountId)
  toast.success(t('accounts.switched'))
}

function editAccount(account: any) {
  editingId.value = account.id
  accountForm.value = {
    apiToken: account.apiToken,
    alias: account.alias
  }
}

function closeModal() {
  showAddModal.value = false
  editingId.value = null
  accountForm.value = { apiToken: '', alias: '' }
}

function copyId(id?: string) {
  if (!id) return
  navigator.clipboard.writeText(id)
  toast.success(t('accounts.idCopied'))
}

async function deleteAccount(accountId: string) {
  if (confirm(t('accounts.deleteConfirm'))) {
    accountStore.removeAccount(accountId)
    toast.success(t('accounts.removed'))
  }
}

async function handleSaveAccount() {
  if (!accountForm.value.apiToken.trim() || !accountForm.value.alias.trim()) {
    toast.warning(t('accounts.invalidInput'))
    return
  }

  saving.value = true
  try {
    if (editingId.value) {
      await accountStore.updateAccount(editingId.value, {
        apiToken: accountForm.value.apiToken,
        alias: accountForm.value.alias
      })
      toast.success(t('accounts.updated'))
    } else {
      await accountStore.addAccount({
        apiToken: accountForm.value.apiToken,
        alias: accountForm.value.alias
      })
      toast.success(t('accounts.linked'))
    }
    closeModal()
  } catch (e) {
    toast.error(t('accounts.saveFailed'))
  } finally {
    saving.value = false
  }
}

function exportAccounts() {
  const data = JSON.stringify(accountStore.accounts, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `cf-manager-backup-${new Date().toISOString().split('T')[0]}.json`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
  toast.success(t('accounts.exportSuccess'))
}

function handleImport() {
  if (!importText.value.trim()) return

  try {
    const accounts = JSON.parse(importText.value)
    if (!Array.isArray(accounts)) {
      toast.error(t('accounts.importInvalid'))
      return
    }
    
    let imported = 0
    accounts.forEach(acc => {
      if (acc.apiToken && acc.alias) {
        const exists = accountStore.accounts.some(existing => existing.apiToken === acc.apiToken)
        if (!exists) {
          accountStore.addAccount(acc)
          imported++
        }
      }
    })
    
    showImportModal.value = false
    importText.value = ''
    toast.success(t('accounts.importCount', { count: imported }))
  } catch (error) {
    toast.error(t('accounts.importParseFailed'))
  }
}
</script>
