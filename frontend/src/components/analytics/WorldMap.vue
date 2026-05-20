<template>
  <div class="relative w-full aspect-[2/1] bg-card/40 backdrop-blur-[20px] rounded-[32px] border border-border/50 overflow-hidden group">
    <!-- Map Header -->
    <div class="absolute top-6 left-8 z-10">
      <div class="text-[10px] text-primary font-black uppercase tracking-[0.2em] mb-1">{{ t('analytics.globalCoverage') }}</div>
      <h3 class="text-xl font-black text-foreground tracking-tight">{{ t('analytics.trafficDistribution') }}</h3>
    </div>

    <!-- Stats Floating Card -->
    <div class="absolute top-6 right-8 z-10 flex gap-4">
      <div class="px-4 py-2 bg-foreground/5 backdrop-blur-md rounded-xl border border-border/30 text-center">
        <div class="text-lg font-black text-primary tracking-tighter">310+</div>
        <div class="text-[8px] text-muted-foreground font-black uppercase tracking-widest">{{ t('analytics.dataCenters') }}</div>
      </div>
      <div class="px-4 py-2 bg-foreground/5 backdrop-blur-md rounded-xl border border-border/30 text-center">
        <div class="text-lg font-black text-emerald-500 tracking-tighter">100%</div>
        <div class="text-[8px] text-muted-foreground font-black uppercase tracking-widest">{{ t('analytics.uptime') }}</div>
      </div>
    </div>

    <!-- Simplified SVG Map -->
    <div class="absolute inset-0 flex items-center justify-center p-12 opacity-20 dark:opacity-40 transition-opacity group-hover:opacity-30 dark:group-hover:opacity-50">
      <svg viewBox="0 0 1000 500" class="w-full h-full text-foreground fill-current">
        <!-- Abstract World Map (Simplified paths) -->
        <path d="M150,150 Q250,100 350,150 T550,150 T750,150 T900,200" fill="none" stroke="currentColor" stroke-width="1" stroke-dasharray="4 4" />
        <circle cx="200" cy="180" r="4" class="animate-pulse text-primary" />
        <circle cx="450" cy="220" r="4" class="animate-pulse text-primary" style="animation-delay: 0.5s" />
        <circle cx="750" cy="160" r="4" class="animate-pulse text-primary" style="animation-delay: 1s" />
        <circle cx="850" cy="300" r="4" class="animate-pulse text-primary" style="animation-delay: 1.5s" />
        <!-- More dots to represent Cloudflare Edge -->
        <circle v-for="n in 20" :key="n" 
          :cx="Math.random() * 800 + 100" 
          :cy="Math.random() * 300 + 100" 
          r="2" 
          class="text-primary/40 opacity-50" 
        />
      </svg>
    </div>

    <!-- Live Traffic Lines (CSS Animation) -->
    <div class="absolute inset-0 pointer-events-none">
      <div v-for="n in 5" :key="n" class="traffic-line" :style="lineStyles[n-1]"></div>
    </div>

    <!-- Bottom Legend -->
    <div class="absolute bottom-6 left-8 flex items-center gap-6">
      <div class="flex items-center gap-2">
        <div class="w-2 h-2 rounded-full bg-primary animate-pulse"></div>
        <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-widest">{{ t('analytics.liveRequestSources') }}</span>
      </div>
      <div class="flex items-center gap-2">
        <div class="w-2 h-2 rounded-full bg-emerald-500"></div>
        <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-widest">{{ t('analytics.edgeNodesOnline') }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const lineStyles = [
  { top: '40%', left: '20%', width: '150px', transform: 'rotate(15deg)', animationDelay: '0s' },
  { top: '60%', left: '45%', width: '200px', transform: 'rotate(-10deg)', animationDelay: '1s' },
  { top: '35%', left: '70%', width: '120px', transform: 'rotate(45deg)', animationDelay: '2s' },
  { top: '75%', left: '15%', width: '180px', transform: 'rotate(-25deg)', animationDelay: '1.5s' },
  { top: '50%', left: '75%', width: '100px', transform: 'rotate(-15deg)', animationDelay: '0.5s' },
]
</script>

<style scoped>
.traffic-line {
  position: absolute;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--color-primary), transparent);
  opacity: 0;
  animation: trafficFlow 4s infinite linear;
}

@keyframes trafficFlow {
  0% { opacity: 0; transform: translateX(-100%) rotate(inherit); }
  20% { opacity: 0.6; }
  80% { opacity: 0.6; }
  100% { opacity: 0; transform: translateX(200%) rotate(inherit); }
}
</style>
