<template>
  <n-message-provider>
    <n-dialog-provider>
      <div class="min-h-screen bg-background">
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
/* Island Theme base styles are imported from island-theme.css */

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  transition: background-color 0.3s, color 0.3s;
}

#app {
  min-height: 100vh;
}

/* 亮色主题 */
[data-theme='light'] #app {
  background-color: #f5f7fa;
  color: #333;
}

/* 暗色主题 */
[data-theme='dark'] #app {
  background-color: #18181c;
  color: #f5f7fa;
}
</style>
