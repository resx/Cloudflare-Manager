<template>
  <GlassCard hoverable class="metric-card h-36 flex flex-col justify-between">
    <div class="flex justify-between items-start mb-3">
      <span class="text-sm text-muted-foreground font-medium">{{ title }}</span>
      <div class="p-2 rounded-lg bg-primary/10 text-primary">
        <slot name="icon" />
      </div>
    </div>
    <div>
      <div class="text-3xl font-bold text-foreground mb-1">{{ value }}</div>
      <div class="text-xs text-muted-foreground leading-relaxed">
        <slot name="footer" />
      </div>
    </div>
  </GlassCard>
</template>

<script setup lang="ts">
import GlassCard from './GlassCard.vue'

defineProps<{
  title: string
  value: string | number
}>()
</script>
