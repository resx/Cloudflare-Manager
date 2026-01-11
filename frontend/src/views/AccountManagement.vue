<template>
  <!-- Account Management - Island Theme (Redirect to Accounts) -->
  <div class="animate-in">
    <div class="metric-card p-12 text-center">
      <div class="text-5xl mb-4">👤</div>
      <h3 class="font-semibold mb-2">账户管理</h3>
      <p class="text-sm text-muted-foreground mb-4">
        账户管理功能已迁移到新页面
      </p>
      <button class="btn-island-primary" @click="$router.push('/accounts')">
        前往账户管理
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
// Redirect to /accounts page
</script>
