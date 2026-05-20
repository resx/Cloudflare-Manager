<template>
  <!-- Rate Limits - Island Theme -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">{{ t('rateLimits.title') }}</h1>
      <p class="text-sm text-muted-foreground mt-1">
        {{ t('common.managing') }}: <span class="font-medium">{{ currentZone?.name || t('common.notSelected') }}</span>
      </p>
    </div>

    <div class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">⏱️</div>
      <h3 class="font-semibold mb-2">{{ t('rateLimits.rulesTitle') }}</h3>
      <p class="text-sm text-muted-foreground mb-4">
        {{ t('rateLimits.rulesDesc') }}
      </p>
      <p class="text-xs text-muted-foreground">
        {{ t('rateLimits.tip') }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, type Ref } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Zone } from '@/api'

const { t } = useI18n()
const currentZone = inject<Ref<Zone | null>>('currentZone')
</script>
