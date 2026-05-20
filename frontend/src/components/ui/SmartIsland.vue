<template>
  <Teleport to="body">
    <div class="fixed top-6 left-1/2 -translate-x-1/2 z-[200] pointer-events-none">
      <transition name="island-pop">
        <div 
          v-if="active" 
          :class="['pointer-events-auto flex items-center gap-3 px-4 py-2 rounded-full border shadow-2xl transition-all duration-500 ease-out', 
            status === 'processing' ? 'bg-card/90 backdrop-blur-2xl border-primary/30 w-[280px]' : 'bg-emerald-500/90 backdrop-blur-xl border-emerald-400/30 w-auto']"
        >
          <!-- Icon / Spinner -->
          <div class="shrink-0 w-8 h-8 rounded-full flex items-center justify-center relative overflow-hidden">
            <component 
              :is="status === 'processing' ? SyncOutline : CheckmarkCircleOutline" 
              :class="['w-5 h-5', status === 'processing' ? 'animate-spin text-primary' : 'text-white']" 
            />
            <div v-if="status === 'processing'" class="absolute inset-0 bg-primary/10 animate-pulse"></div>
          </div>

          <!-- Content -->
          <div class="flex-1 min-w-0">
            <p :class="['text-xs font-black truncate tracking-tight', status === 'processing' ? 'text-foreground' : 'text-white']">
              {{ message }}
            </p>
            <div v-if="status === 'processing'" class="h-1 bg-foreground/10 rounded-full mt-1 overflow-hidden">
              <div class="h-full bg-primary animate-progress-indeterminate"></div>
            </div>
          </div>

          <!-- Success Marker -->
          <div v-if="status === 'success'" class="text-[10px] font-black text-white/80 uppercase tracking-widest ml-2 animate-in">DONE</div>
        </div>
      </transition>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { SyncOutline, CheckmarkCircleOutline } from '@vicons/ionicons5'

// Global Event Bus approach for simplicity in this demo
// In a real app, use a dedicated Store
const active = ref(false)
const message = ref('')
const status = ref<'processing' | 'success'>('processing')

function show(msg: string, duration = 3000) {
  message.value = msg
  status.value = 'processing'
  active.value = true

  // Simulate process
  setTimeout(() => {
    status.value = 'success'
    message.value = '操作已成功完成'
    
    setTimeout(() => {
      active.value = false
    }, duration)
  }, 2000)
}

// Expose to window for global access in this demo
if (typeof window !== 'undefined') {
  (window as any).smartIsland = { show }
}
</script>

<style scoped>
.island-pop-enter-active {
  animation: islandIn 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
}
.island-pop-leave-active {
  animation: islandIn 0.4s cubic-bezier(0.34, 1.56, 0.64, 1) reverse;
}

@keyframes islandIn {
  0% { transform: scale(0.5) translateY(-40px); opacity: 0; }
  100% { transform: scale(1) translateY(0); opacity: 1; }
}

.animate-progress-indeterminate {
  width: 50%;
  animation: progressIndeterminate 1.5s infinite ease-in-out;
}

@keyframes progressIndeterminate {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(200%); }
}
</style>
