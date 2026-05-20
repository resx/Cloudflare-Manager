import { createI18n } from 'vue-i18n'
import zhCN from './locales/zh-CN.json'
import enUS from './locales/en-US.json'

// 获取初始语言：优先 localStorage，其次浏览器语言，最后默认中文
function getInitialLocale() {
  const saved = localStorage.getItem('cf_language')
  if (saved) return saved

  const browserLang = navigator.language
  if (browserLang.startsWith('en')) return 'en-US'
  return 'zh-CN'
}

const i18n = createI18n({
  legacy: false, // 使用 Composition API
  locale: getInitialLocale(),
  fallbackLocale: 'en-US',
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS
  }
})

export default i18n
