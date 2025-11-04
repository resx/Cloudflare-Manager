# 域名统一管理修复总结

## 已完成修复的页面

### ✅ DNS.vue
- 移除域名选择器
- 使用 `inject<Ref<Zone | null>>('currentZone')`
- 添加 `watch(() => currentZone?.value?.id)` 监听

### ✅ Firewall.vue
- 移除域名选择器
- 使用 `inject<Ref<Zone | null>>('currentZone')`
- 添加 `watch(() => currentZone?.value?.id)` 监听

### ✅ SSL.vue
- 移除 `currentZoneId` computed 属性
- 使用 `inject<Ref<Zone | null>>('currentZone')`
- 所有函数使用 `currentZone.value.id`
- 添加 `watch(() => currentZone?.value?.id)` 监听

### ✅ Cache.vue
- 移除 `currentZoneId` computed 属性
- 使用 `inject<Ref<Zone | null>>('currentZone')`
- 所有函数使用 `currentZone.value.id`
- 添加 `watch(() => currentZone?.value?.id)` 监听

## 已完成修复的其他页面

### ✅ Optimize.vue
- 移除域名选择器
- 使用 `inject<Ref<Zone | null>>('currentZone')`
- 显示当前域名信息
- `handleOptimize` 函数使用 `currentZone.value.id`

### ✅ QuickDeploy.vue
- 移除域名选择器
- 使用 `inject<Ref<Zone | null>>('currentZone')`
- 显示当前域名（只读）
- Deploy 函数使用 `currentZone.value.id`

### ✅ Analytics.vue
- 移除 `currentZoneId` computed（localStorage 方式）
- 使用 `inject<Ref<Zone | null>>('currentZone')`
- `loadAnalytics` 函数使用 `currentZone.value.id`
- 添加 `watch(() => currentZone?.value?.id)` 监听

### ✅ Certificates.vue
- 使用 `inject<Ref<Zone | null>>('currentZone')`
- 显示 `currentZone.value.name` 在证书覆盖域名
- 添加 `watch(() => currentZone?.value?.id)` 监听

### ✅ PageRules.vue
- 使用 `inject<Ref<Zone | null>>('currentZone')`
- `loadPageRules` 函数使用 `currentZone.value.id`
- 添加 `watch(() => currentZone?.value?.id)` 监听

## 统一修复模式

```typescript
// 1. Import
import { ref, onMounted, watch, inject, type Ref } from 'vue'
import { cloudflareApi, type Zone } from '@/api'

// 2. Inject currentZone
const currentZone = inject<Ref<Zone | null>>('currentZone')

// 3. 在函数中使用
async function loadData() {
  if (!currentZone?.value?.id) {
    console.log('No currentZone available')
    return
  }

  const data = await cloudflareApi.someMethod(currentZone.value.id)
}

// 4. 添加监听
watch(() => currentZone?.value?.id, () => {
  loadData()
})
```
