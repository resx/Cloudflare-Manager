import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { GlobalThemeOverrides } from 'naive-ui'

export type ThemeMode = 'light' | 'dark' | 'auto'

export const useThemeStore = defineStore('theme', () => {
  const mode = ref<ThemeMode>('light')
  const isDark = ref(false)

  // 从 localStorage 加载主题
  function loadTheme() {
    try {
      const stored = localStorage.getItem('cf_theme_mode')
      if (stored) {
        mode.value = stored as ThemeMode
        applyTheme()
      }
    } catch (error) {
      console.error('Failed to load theme:', error)
    }
  }

  // 保存主题到 localStorage
  function saveTheme() {
    try {
      localStorage.setItem('cf_theme_mode', mode.value)
    } catch (error) {
      console.error('Failed to save theme:', error)
    }
  }

  // 切换主题
  function setTheme(newMode: ThemeMode) {
    mode.value = newMode
    applyTheme()
    saveTheme()
  }

  // 应用主题
  function applyTheme() {
    if (mode.value === 'auto') {
      // 自动检测系统主题
      isDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches
    } else {
      isDark.value = mode.value === 'dark'
    }

    // 更新 HTML 的 data-theme 属性
    document.documentElement.setAttribute('data-theme', isDark.value ? 'dark' : 'light')
  }

  // 监听系统主题变化
  function setupThemeListener() {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    mediaQuery.addEventListener('change', (e) => {
      if (mode.value === 'auto') {
        isDark.value = e.matches
        document.documentElement.setAttribute('data-theme', isDark.value ? 'dark' : 'light')
      }
    })
  }

  // 初始化
  loadTheme()
  setupThemeListener()

  return {
    mode,
    isDark,
    setTheme,
    applyTheme
  }
})
