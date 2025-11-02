<template>
  <n-config-provider :theme="currentTheme" :theme-overrides="themeOverrides">
    <n-global-style />
    <n-message-provider>
      <n-dialog-provider>
        <n-notification-provider>
          <router-view />
        </n-notification-provider>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { darkTheme, type GlobalThemeOverrides } from 'naive-ui'
import { useThemeStore } from '@/stores/theme'

const themeStore = useThemeStore()

// 根据 isDark 状态返回主题
const currentTheme = computed(() => themeStore.isDark ? darkTheme : null)

// 主题覆盖配置
const themeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: '#18a058',
    primaryColorHover: '#36ad6a',
    primaryColorPressed: '#0c7a43',
    primaryColorSuppl: '#36ad6a'
  }
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

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
