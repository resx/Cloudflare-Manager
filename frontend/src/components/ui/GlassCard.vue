<template>
  <div
    :class="[
      'island-container transition-all duration-300',
      hoverable ? 'hover:shadow-island-hover hover:-translate-y-0.5' : '',
      padding ? `p-${padding}` : ''
    ]"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
defineProps<{
  hoverable?: boolean
  padding?: string | number
}>()
</script>
