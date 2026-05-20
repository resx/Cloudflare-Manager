<template>
  <n-message-provider>
    <n-dialog-provider>
      <div class="min-h-screen relative">
        <div class="glass-gradient-bg"></div>
        <router-view />
      </div>
    </n-dialog-provider>
  </n-message-provider>
  
  <!-- Island Theme Toast Component -->
  <Toast ref="toastRef" />
</template>

<script setup lang="ts">
import { NMessageProvider, NDialogProvider } from 'naive-ui'
import Toast from '@/components/Toast.vue'
import { onMounted, ref } from 'vue'

const toastRef = ref()

onMounted(() => {
  // Expose toast globally
  if (toastRef.value) {
    (window as any).__toast__ = toastRef.value
  }
})
// Island Theme - Using Tailwind for styling, Naive UI for compatibility during migration
</script>

<style>
/* Base styles are now handled in styles/island-theme.css */
#app {
  min-height: 100vh;
}
</style>
