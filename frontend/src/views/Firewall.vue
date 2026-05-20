<template>
  <div class="animate-in space-y-8 pb-12">
    <!-- Header -->
    <header class="flex flex-col md:flex-row justify-between items-start md:items-end gap-4 px-1">
      <div>
        <h1 class="text-3xl font-bold text-foreground tracking-tight">{{ t('firewall.title') }}</h1>
        <p class="text-sm text-muted-foreground mt-1 font-medium italic">
          {{ currentZone?.name || t('zones.notSelected') }} · {{ t('firewall.subtitle') }}
        </p>
      </div>
      <IslandButton @click="showAddModal = true">
        <template #icon><component :is="AddOutline" class="w-4 h-4" /></template>
        {{ t('firewall.addRule') }}
      </IslandButton>
    </header>

    <!-- Content Area -->
    <div class="grid grid-cols-1 xl:grid-cols-4 gap-8">
      <!-- Main Rules Table -->
      <div class="xl:col-span-3 space-y-6">
        <GlassCard :padding="0" class="overflow-hidden">
          <div v-if="loadingRules" class="py-24 flex flex-col items-center justify-center">
            <div class="w-10 h-10 border-4 border-primary/20 border-t-primary rounded-full animate-spin"></div>
            <p class="mt-4 text-xs font-bold text-muted-foreground uppercase tracking-widest">{{ t('firewall.syncing') }}</p>
          </div>

          <div v-else-if="firewallRules.length > 0" class="overflow-x-auto">
            <table class="w-full text-left">
              <thead>
                <tr class="bg-foreground/[0.03]">
                  <th class="py-4 px-6 text-xs font-bold uppercase tracking-widest text-muted-foreground">{{ t('firewall.colDescription') }}</th>
                  <th class="py-4 px-6 text-xs font-bold uppercase tracking-widest text-muted-foreground">{{ t('firewall.colExpression') }}</th>
                  <th class="py-4 px-6 text-xs font-bold uppercase tracking-widest text-muted-foreground">{{ t('firewall.colAction') }}</th>
                  <th class="py-4 px-6 text-xs font-bold uppercase tracking-widest text-muted-foreground">{{ t('firewall.colStatus') }}</th>
                  <th class="py-4 px-6 text-right text-xs font-bold uppercase tracking-widest text-muted-foreground">{{ t('common.actions') }}</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border/30">
                <tr v-for="rule in firewallRules" :key="rule.id" class="group hover:bg-primary/[0.02] transition-colors">
                  <td class="py-4 px-6">
                    <div class="font-bold text-sm text-foreground truncate max-w-[200px]" :title="rule.description">
                      {{ rule.description || t('firewall.unnamedRule') }}
                    </div>
                  </td>
                  <td class="py-4 px-6">
                    <div class="font-mono text-[10px] bg-foreground/5 px-2 py-1 rounded max-w-[300px] truncate opacity-80 group-hover:opacity-100 transition-opacity">
                      {{ rule.filter?.expression }}
                    </div>
                  </td>
                  <td class="py-4 px-6">
                    <GlassBadge :variant="getActionVariant(rule.action)">
                      {{ rule.action.toUpperCase() }}
                    </GlassBadge>
                  </td>
                  <td class="py-4 px-6">
                    <div class="flex items-center gap-2">
                      <div class="w-2 h-2 rounded-full" :class="rule.paused ? 'bg-amber-500' : 'bg-emerald-500'"></div>
                      <span class="text-xs font-bold">{{ rule.paused ? t('firewall.statusPaused') : t('firewall.statusActive') }}</span>
                    </div>
                  </td>
                  <td class="py-4 px-6 text-right">
                    <div class="flex justify-end gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                      <button @click="handleEdit(rule)" class="p-2 hover:bg-primary/10 rounded-lg text-primary transition-all">
                        <component :is="BuildOutline" class="w-4 h-4" />
                      </button>
                      <button @click="handleDelete(rule)" class="p-2 hover:bg-danger/10 rounded-lg text-danger transition-all">
                        <component :is="TrashOutline" class="w-4 h-4" />
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <!-- Empty State -->
          <div v-else class="py-24 flex flex-col items-center text-center px-6">
            <div class="w-16 h-16 rounded-3xl bg-muted flex items-center justify-center text-muted-foreground mb-6 shadow-inner">
              <component :is="ShieldCheckmarkOutline" class="w-8 h-8" />
            </div>
            <h3 class="text-xl font-bold">{{ currentZone ? t('firewall.noRules') : t('zones.notSelected') }}</h3>
            <p class="text-sm text-muted-foreground max-w-xs mt-3 leading-relaxed">
              {{ currentZone ? t('firewall.noRulesDesc') : t('firewall.selectZoneDesc') }}
            </p>
            <IslandButton v-if="currentZone" class="mt-8" @click="showAddModal = true">
              {{ t('firewall.createFirstRule') }}
            </IslandButton>
          </div>
        </GlassCard>
      </div>

      <!-- Rule Templates Sidebar -->
      <div class="space-y-6">
        <h3 class="text-xs font-bold uppercase tracking-widest text-muted-foreground px-1">{{ t('firewall.templates') }}</h3>
        <div class="space-y-4">
          <GlassCard 
            v-for="template in ruleTemplates" 
            :key="template.id" 
            :padding="5"
            class="group cursor-pointer hover:border-primary/50 transition-all active:scale-95"
            @click="useTemplate(template)"
          >
            <div class="flex items-center gap-3 mb-2">
              <div class="w-8 h-8 rounded-xl bg-primary/10 flex items-center justify-center text-primary">
                <component :is="FlashOutline" class="w-4 h-4" />
              </div>
              <h4 class="font-bold text-sm group-hover:text-primary transition-colors">{{ t(`firewall.templateName_${template.id}`) }}</h4>
            </div>
            <p class="text-[11px] text-muted-foreground leading-relaxed mb-3">{{ t(`firewall.templateDesc_${template.id}`) }}</p>
            <div class="font-mono text-[9px] bg-slate-950 text-emerald-500/80 p-2 rounded-lg truncate">
              {{ template.expression }}
            </div>
          </GlassCard>
        </div>
      </div>
    </div>

    <!-- Modals -->
    <n-modal v-model:show="showAddModal">
      <GlassCard class="w-[600px] max-w-[95vw]" :padding="0">
        <div class="p-6 border-b border-border/50 flex justify-between items-center">
          <h2 class="text-xl font-bold">{{ t('firewall.createTitle') }}</h2>
          <button @click="showAddModal = false" class="text-muted-foreground hover:text-foreground">
            <component :is="CloseOutline" class="w-5 h-5" />
          </button>
        </div>
        
        <div class="p-8 space-y-6">
          <div class="space-y-4">
            <div class="space-y-2">
              <label class="text-xs font-bold ml-1 uppercase tracking-widest text-muted-foreground">{{ t('firewall.colDescription') }}</label>
              <input
                v-model="ruleForm.description"
                class="w-full bg-foreground/5 border border-border/50 rounded-xl px-4 py-3 text-sm focus:ring-2 focus:ring-primary/30 outline-none transition-all"
                :placeholder="t('firewall.descPlaceholder')"
              />
            </div>

            <div class="space-y-2">
              <label class="text-xs font-bold ml-1 uppercase tracking-widest text-muted-foreground">{{ t('firewall.colExpression') }}</label>
              <textarea
                v-model="ruleForm.expression"
                class="w-full h-32 bg-slate-950 text-emerald-500 border border-border/50 rounded-xl p-4 font-mono text-xs focus:outline-none shadow-inner"
                placeholder='(ip.geoip.country ne "CN")'
              ></textarea>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div class="space-y-2">
                <label class="text-xs font-bold ml-1 uppercase tracking-widest text-muted-foreground">{{ t('firewall.colAction') }}</label>
                <select v-model="ruleForm.action" class="w-full bg-foreground/5 border border-border/50 rounded-xl px-4 py-3 text-sm outline-none appearance-none cursor-pointer">
                  <option v-for="opt in actionOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
                </select>
              </div>
              <div class="space-y-2">
                <label class="text-xs font-bold ml-1 uppercase tracking-widest text-muted-foreground">{{ t('firewall.initialStatus') }}</label>
                <div class="flex items-center gap-3 h-[46px] px-4 bg-foreground/5 border border-border/50 rounded-xl">
                  <n-switch v-model:value="ruleForm.paused" size="small" />
                  <span class="text-xs font-bold">{{ ruleForm.paused ? t('firewall.statusPaused') : t('firewall.statusEnabled') }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="p-6 border-t border-border/50 flex justify-end gap-3">
          <IslandButton variant="secondary" @click="showAddModal = false">{{ t('common.cancel') }}</IslandButton>
          <IslandButton @click="handleAddRule" :loading="submitting">{{ t('common.save') }}</IslandButton>
        </div>
      </GlassCard>
    </n-modal>

    <n-modal v-model:show="showEditModal">
      <GlassCard class="w-[600px] max-w-[95vw]" :padding="0">
        <div class="p-6 border-b border-border/50 flex justify-between items-center">
          <h2 class="text-xl font-bold">{{ t('firewall.editTitle') }}</h2>
          <button @click="showEditModal = false" class="text-muted-foreground hover:text-foreground">
            <component :is="CloseOutline" class="w-5 h-5" />
          </button>
        </div>
        
        <div class="p-8 space-y-6">
          <div class="space-y-4">
            <div class="space-y-2">
              <label class="text-xs font-bold ml-1 uppercase tracking-widest text-muted-foreground">{{ t('firewall.colDescription') }}</label>
              <input
                v-model="editForm.description"
                class="w-full bg-foreground/5 border border-border/50 rounded-xl px-4 py-3 text-sm focus:ring-2 focus:ring-primary/30 outline-none transition-all"
              />
            </div>

            <div class="space-y-2 opacity-60">
              <label class="text-xs font-bold ml-1 uppercase tracking-widest text-muted-foreground text-danger">{{ t('firewall.expressionReadOnly') }}</label>
              <div class="w-full bg-slate-900 text-slate-500 border border-border/50 rounded-xl p-4 font-mono text-xs shadow-inner">
                {{ editForm.filter.expression }}
              </div>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div class="space-y-2">
                <label class="text-xs font-bold ml-1 uppercase tracking-widest text-muted-foreground">{{ t('firewall.colAction') }}</label>
                <select v-model="editForm.action" class="w-full bg-foreground/5 border border-border/50 rounded-xl px-4 py-3 text-sm outline-none appearance-none cursor-pointer">
                  <option v-for="opt in actionOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
                </select>
              </div>
              <div class="space-y-2">
                <label class="text-xs font-bold ml-1 uppercase tracking-widest text-muted-foreground">{{ t('firewall.colStatus') }}</label>
                <div class="flex items-center gap-3 h-[46px] px-4 bg-foreground/5 border border-border/50 rounded-xl">
                  <n-switch v-model:value="editForm.paused" size="small" />
                  <span class="text-xs font-bold">{{ editForm.paused ? t('firewall.statusPaused') : t('firewall.statusActive') }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="p-6 border-t border-border/50 flex justify-end gap-3">
          <IslandButton variant="secondary" @click="showEditModal = false">{{ t('common.cancel') }}</IslandButton>
          <IslandButton @click="handleUpdateRule" :loading="submitting">{{ t('common.update') }}</IslandButton>
        </div>
      </GlassCard>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, inject, type Ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { NModal, NSwitch } from 'naive-ui'
import { 
  AddOutline, 
  ShieldCheckmarkOutline, 
  TrashOutline, 
  BuildOutline, 
  CloseOutline,
  FlashOutline 
} from '@vicons/ionicons5'
import { cloudflareApi, type FirewallRule, type Zone } from '@/api'
import { toast } from '@/utils/toast'
import { logHistory } from '@/utils/history'
import { useAccountStore } from '@/stores/account'
import GlassCard from '@/components/ui/GlassCard.vue'
import IslandButton from '@/components/ui/IslandButton.vue'
import GlassBadge from '@/components/ui/GlassBadge.vue'

const { t } = useI18n()
const accountStore = useAccountStore()
const currentZone = inject<Ref<Zone | null>>('currentZone')

const loadingRules = ref(false)
const submitting = ref(false)
const showAddModal = ref(false)
const showEditModal = ref(false)

const firewallRules = ref<FirewallRule[]>([])

const ruleForm = ref({
  description: '',
  expression: '',
  action: 'block',
  paused: false
})

const editForm = ref<FirewallRule>({
  filter: {
    expression: '',
    description: ''
  },
  action: 'block',
  description: '',
  paused: false
})

const actionOptions = computed(() => [
  { label: `${t('firewall.actionBlock')} (Block)`, value: 'block' },
  { label: `${t('firewall.actionJSChallenge')} (JS Challenge)`, value: 'js_challenge' },
  { label: `${t('firewall.actionManagedChallenge')} (Managed Challenge)`, value: 'managed_challenge' },
  { label: `${t('firewall.actionAllow')} (Allow)`, value: 'allow' },
  { label: `${t('firewall.actionLog')} (Log)`, value: 'log' }
])

const ruleTemplates = [
  { id: 1, expression: '(ip.geoip.country eq "XX")' },
  { id: 2, expression: '(ip.geoip.country ne "CN")' },
  { id: 3, expression: '(http.user_agent contains "bot" and not http.user_agent contains "Googlebot")' },
  { id: 4, expression: '(http.request.uri.path contains "/api/" and rate(ip.src, 100/1m))' },
  { id: 5, expression: '(ip.geoip.country ne "CN" and http.request.uri.path eq "/admin")' },
  { id: 6, expression: '(ip.src in {192.168.1.1 192.168.1.0/24})' }
]

function getActionVariant(action: string): 'danger' | 'success' | 'info' | 'warning' {
  const map: Record<string, any> = {
    block: 'danger',
    allow: 'success',
    log: 'info',
    js_challenge: 'warning',
    managed_challenge: 'warning'
  }
  return map[action] || 'info'
}

async function loadFirewallRules() {
  if (!currentZone?.value?.id || !accountStore.currentAccount) {
    firewallRules.value = []
    return
  }

  loadingRules.value = true
  try {
    firewallRules.value = await cloudflareApi.getFirewallRules(currentZone.value.id)
  } catch (error: any) {
    if (!error.silent) {
      toast.error(t('firewall.loadFailed'))
    }
  } finally {
    loadingRules.value = false
  }
}

function useTemplate(template: any) {
  ruleForm.value.expression = template.expression
  ruleForm.value.description = t(`firewall.templateName_${template.id}`)
  showAddModal.value = true
}

async function handleAddRule() {
  if (!currentZone?.value?.id) return

  submitting.value = true
  try {
    await cloudflareApi.createFirewallRule(currentZone.value.id, {
      filter: {
        expression: ruleForm.value.expression,
        description: ruleForm.value.description
      },
      action: ruleForm.value.action,
      description: ruleForm.value.description,
      paused: ruleForm.value.paused
    })

    logHistory.firewall('创建规则', ruleForm.value.description || '新规则')
    toast.success(t('firewall.createSuccess'))
    showAddModal.value = false
    ruleForm.value = {
      description: '',
      expression: '',
      action: 'block',
      paused: false
    }
    await loadFirewallRules()
  } catch (error: any) {
    toast.error(error?.message || t('common.updateFailed'))
  } finally {
    submitting.value = false
  }
}

function handleEdit(rule: FirewallRule) {
  editForm.value = JSON.parse(JSON.stringify(rule))
  showEditModal.value = true
}

async function handleUpdateRule() {
  if (!currentZone?.value?.id) return

  submitting.value = true
  try {
    await cloudflareApi.updateFirewallRule(
      currentZone.value.id,
      editForm.value.id!,
      editForm.value
    )

    toast.success(t('common.updateSuccess'))
    showEditModal.value = false
    await loadFirewallRules()
  } catch (error: any) {
    toast.error(error?.message || t('common.updateFailed'))
  } finally {
    submitting.value = false
  }
}

async function handleDelete(rule: FirewallRule) {
  if (!currentZone?.value?.id) return
  if (!confirm(t('firewall.deleteConfirm', { name: rule.description || t('firewall.unnamedRule') }))) return

  try {
    await cloudflareApi.deleteFirewallRule(currentZone.value.id, rule.id!)
    toast.success(t('common.deleteSuccess'))
    await loadFirewallRules()
  } catch (error: any) {
    toast.error(error?.message || t('common.deleteFailed'))
  }
}

onMounted(() => {
  loadFirewallRules()
})

watch(() => currentZone?.value?.id, () => {
  loadFirewallRules()
})
</script>
