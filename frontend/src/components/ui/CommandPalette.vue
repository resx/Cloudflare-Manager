<template>
  <Teleport to="body">
    <transition name="palette-fade">
      <div v-if="visible" class="fixed inset-0 z-[100] flex items-start justify-center pt-[15vh] px-4">
        <!-- Backdrop -->
        <div class="absolute inset-0 bg-background/40 backdrop-blur-sm" @click="close"></div>
        
        <!-- Palette Container -->
        <div class="relative w-full max-w-2xl bg-card/80 backdrop-blur-[32px] rounded-[28px] border border-border/50 shadow-[0_32px_128px_-12px_rgba(0,0,0,0.3)] overflow-hidden animate-palette-in">
          <!-- Search Input -->
          <div class="p-6 border-b border-border/30 flex items-center gap-4">
            <component :is="SearchOutline" class="w-6 h-6 text-primary" />
            <input
              ref="searchInput"
              v-model="query"
              class="flex-1 bg-transparent border-none outline-none text-xl font-bold placeholder:text-muted-foreground/40 text-foreground"
              :placeholder="t('commandPalette.placeholder')"
              @keydown.esc="close"
              @keydown.down="moveCursor(1)"
              @keydown.up="moveCursor(-1)"
              @keydown.enter="executeAction"
            />
            <div class="px-2 py-1 bg-foreground/5 rounded-md text-[10px] font-black text-muted-foreground uppercase tracking-widest border border-border/50">ESC</div>
          </div>
 
          <!-- Results Area -->
          <div class="max-h-[450px] overflow-y-auto custom-scrollbar p-3">
            <div v-if="filteredResults.length > 0" class="space-y-6">
              <div v-for="(group, gIndex) in groupedResults" :key="gIndex" class="space-y-1">
                <div class="px-4 py-2 text-[10px] font-black uppercase tracking-[0.2em] text-muted-foreground/40">{{ group.name }}</div>
                <div
                  v-for="(item, iIndex) in group.items"
                  :key="item.id"
                  :class="['flex items-center gap-4 p-4 rounded-2xl cursor-pointer transition-all group', 
                    isActive(item) ? 'bg-primary text-white shadow-lg shadow-primary/20 scale-[1.01]' : 'hover:bg-foreground/5 text-foreground']"
                  @click="handleSelect(item)"
                  @mouseenter="selectedIndex = item.flatIndex"
                >
                  <div :class="['w-10 h-10 rounded-xl flex items-center justify-center shrink-0 border transition-colors', 
                    isActive(item) ? 'bg-white/20 border-white/20 text-white' : 'bg-foreground/5 border-border/30 text-primary group-hover:border-primary/30']">
                    <component :is="item.icon" class="w-5 h-5" />
                  </div>
                  <div class="flex-1 min-w-0">
                    <div class="font-bold text-sm tracking-tight">{{ item.title }}</div>
                    <div :class="['text-[10px] font-medium uppercase tracking-widest mt-0.5 opacity-60', isActive(item) ? 'text-white/80' : 'text-muted-foreground']">
                      {{ item.desc }}
                    </div>
                  </div>
                  <component v-if="isActive(item)" :is="ReturnDownBackOutline" class="w-4 h-4 opacity-60" />
                </div>
              </div>
            </div>
 
            <!-- Empty State -->
            <div v-else-if="query" class="py-20 text-center space-y-4 opacity-40">
              <component :is="SearchOutline" class="w-12 h-12 mx-auto" />
              <div class="space-y-1">
                <p class="font-bold">{{ t('commandPalette.noResults') }}</p>
                <p class="text-xs uppercase tracking-widest font-black">{{ t('commandPalette.noResultsSub', { query }) }}</p>
              </div>
            </div>
 
            <!-- Default Help -->
            <div v-else class="py-12 px-6 grid grid-cols-2 gap-8 border-t border-border/10 mt-2">
              <div class="space-y-3">
                <p class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/40">{{ t('commandPalette.tips') }}</p>
                <div class="space-y-2">
                  <div class="flex items-center gap-2 text-xs font-bold text-muted-foreground">
                    <kbd class="px-1.5 py-0.5 bg-foreground/5 border border-border/50 rounded text-[9px]">↑↓</kbd> {{ t('commandPalette.tipNav') }}
                  </div>
                  <div class="flex items-center gap-2 text-xs font-bold text-muted-foreground">
                    <kbd class="px-1.5 py-0.5 bg-foreground/5 border border-border/50 rounded text-[9px]">ENTER</kbd> {{ t('commandPalette.tipExecute') }}
                  </div>
                </div>
              </div>
              <div class="space-y-3">
                <p class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/40">{{ t('commandPalette.examples') }}</p>
                <div class="space-y-2 italic text-xs text-primary/60 font-bold">
                  <div>{{ t('commandPalette.ex1') }}</div>
                  <div>{{ t('commandPalette.ex2') }}</div>
                </div>
              </div>
 
            </div>
          </div>
        </div>
      </div>
    </transition>
  </Teleport>
</template>
 
<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { 
  SearchOutline, 
  HomeOutline, 
  GlobeOutline, 
  ShieldOutline, 
  SettingsOutline, 
  RocketOutline, 
  FlashOutline, 
  TimeOutline,
  ReturnDownBackOutline,
  BuildOutline,
  LockClosedOutline
} from '@vicons/ionicons5'
 
const { t } = useI18n()
const visible = ref(false)
const query = ref('')
const selectedIndex = ref(0)
const searchInput = ref<HTMLInputElement | null>(null)
const router = useRouter()
 
interface PaletteItem {
  id: string
  title: string
  desc: string
  path?: string
  action?: () => void
  icon: any
  category: string
  flatIndex?: number
}
 
const staticResults = computed<PaletteItem[]>(() => [
  { id: 'dash', title: t('commandPalette.dash.title'), desc: t('commandPalette.dash.desc'), path: '/dashboard', icon: HomeOutline, category: t('commandPalette.cat.navigation') },
  { id: 'zones', title: t('commandPalette.zones.title'), desc: t('commandPalette.zones.desc'), path: '/zones', icon: GlobeOutline, category: t('commandPalette.cat.navigation') },
  { id: 'quickDeploy', title: t('commandPalette.quickDeploy.title'), desc: t('commandPalette.quickDeploy.desc'), path: '/quick-deploy', icon: RocketOutline, category: t('commandPalette.cat.navigation') },
  { id: 'dns', title: t('commandPalette.dns.title'), desc: t('commandPalette.dns.desc'), path: '/dns', icon: BuildOutline, category: t('commandPalette.cat.dns') },
  { id: 'ssl', title: t('commandPalette.ssl.title'), desc: t('commandPalette.ssl.desc'), path: '/ssl', icon: LockClosedOutline, category: t('commandPalette.cat.dns') },
  { id: 'firewall', title: t('commandPalette.firewall.title'), desc: t('commandPalette.firewall.desc'), path: '/firewall', icon: ShieldOutline, category: t('commandPalette.cat.dns') },
  { id: 'optimize', title: t('commandPalette.optimize.title'), desc: t('commandPalette.optimize.desc'), path: '/optimize', icon: FlashOutline, category: t('commandPalette.cat.navigation') },
  { id: 'history', title: t('commandPalette.history.title'), desc: t('commandPalette.history.desc'), path: '/history', icon: TimeOutline, category: t('commandPalette.cat.navigation') },
  { id: 'accounts', title: t('commandPalette.accounts.title'), desc: t('commandPalette.accounts.desc'), path: '/accounts', icon: SettingsOutline, category: t('commandPalette.cat.navigation') },
])
 
const filteredResults = computed(() => {
  const q = query.value.toLowerCase()
  if (!q) return staticResults.value
  return staticResults.value.filter(item => 
    item.title.toLowerCase().includes(q) || 
    item.desc.toLowerCase().includes(q) ||
    item.category.toLowerCase().includes(q)
  )
})
 
const groupedResults = computed(() => {
  const groups: Record<string, PaletteItem[]> = {}
  let flatIdx = 0
  
  filteredResults.value.forEach(item => {
    if (!groups[item.category]) groups[item.category] = []
    const itemWithIdx = { ...item, flatIndex: flatIdx++ }
    groups[item.category].push(itemWithIdx)
  })
  
  return Object.entries(groups).map(([name, items]) => ({ name, items }))
})
 
const totalItems = computed(() => filteredResults.value.length)
 
function open() {
  visible.value = true
  query.value = ''
  selectedIndex.value = 0
  nextTick(() => searchInput.value?.focus())
}
 
function close() {
  visible.value = false
}
 
function moveCursor(delta: number) {
  if (totalItems.value === 0) return
  selectedIndex.value = (selectedIndex.value + delta + totalItems.value) % totalItems.value
}
 
function isActive(item: PaletteItem) {
  return item.flatIndex === selectedIndex.value
}
 
function executeAction() {
  const activeItem = filteredResults.value.find(item => item.flatIndex === selectedIndex.value)
  if (activeItem) {
    handleSelect(activeItem)
  }
}
 
function handleSelect(item: PaletteItem) {
  if (item.path) {
    router.push(item.path)
    close()
  } else if (item.action) {
    item.action()
    close()
  }
}
 
// Expose to window for global access
if (typeof window !== 'undefined') {
  (window as any).commandPalette = { 
    open,
    close,
    toggle: () => visible.value ? close() : open()
  }
}
 
function handleKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault()
    if (visible.value) close()
    else open()
  }
}
 
onMounted(() => window.addEventListener('keydown', handleKeydown))
onUnmounted(() => window.removeEventListener('keydown', handleKeydown))
</script>
 
<style scoped>
.palette-fade-enter-active, .palette-fade-leave-active {
  transition: opacity 0.2s ease;
}
.palette-fade-enter-from, .palette-fade-leave-to {
  opacity: 0;
}
 
@keyframes paletteIn {
  from { opacity: 0; transform: scale(0.95) translateY(-20px); }
  to { opacity: 1; transform: scale(1) translateY(0); }
}
.animate-palette-in {
  animation: paletteIn 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}
 
input::placeholder {
  font-weight: 700;
}
</style>
