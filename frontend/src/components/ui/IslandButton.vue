<template>
  <button
    :class="[
      'inline-flex items-center justify-center gap-2 px-4 py-2 rounded-lg font-medium transition-all duration-200 cursor-pointer active:scale-95',
      variantClass
    ]"
    :disabled="loading || disabled"
  >
    <div v-if="loading" class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin"></div>
    <slot name="icon" v-if="!loading" />
    <slot />
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost'
  loading?: boolean
  disabled?: boolean
}>(), {
  variant: 'primary'
})

const variantClass = computed(() => {
  switch (props.variant) {
    case 'secondary': return 'btn-island-secondary'
    case 'danger': return 'bg-red-500/10 text-red-500 border border-red-500/20 hover:bg-red-500/20'
    case 'ghost': return 'bg-transparent hover:bg-foreground/5 text-foreground'
    default: return 'btn-island-primary'
  }
})
</script>
