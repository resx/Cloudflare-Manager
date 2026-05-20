<template>
  <!-- Page Rules - Island Theme -->
  <div class="animate-in">
    <div class="mb-6">
      <h1 class="text-2xl font-semibold">{{ t('pageRules.title') }}</h1>
      <p class="text-sm text-muted-foreground mt-1">
        {{ t('zones.managing') }}: <span class="font-medium">{{ currentZone?.name || t('common.loading') }}</span>
      </p>
    </div>

    <div class="p-4 bg-primary/5 border border-primary/20 rounded-2xl text-xs text-primary font-medium leading-relaxed mb-6">
      <strong>{{ t('common.tip') }}：</strong>{{ t('pageRules.explain') }}
    </div>

    <div class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">📄</div>
      <h3 class="font-semibold mb-2">{{ t('pageRules.title') }}</h3>
      <p class="text-sm text-muted-foreground mb-4">
        {{ t('pageRules.paidFeature') }}
      </p>
      <button class="btn-island-primary" disabled>
        {{ t('pageRules.createRule') }}
      </button>
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
