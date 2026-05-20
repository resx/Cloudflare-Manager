<template>
  <span :class="['glass-badge', variantClass]">
    <slot />
  </span>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  variant?: 'success' | 'error' | 'warning' | 'info'
}>(), {
  variant: 'info'
})

const variantClass = computed(() => {
  return `glass-badge-${props.variant}`
})
</script>
