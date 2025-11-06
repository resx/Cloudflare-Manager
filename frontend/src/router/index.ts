import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Layout',
    component: () => import('@/views/Layout.vue'),
    redirect: '/zones',
    children: [
      {
        path: '/zones',
        name: 'Zones',
        component: () => import('@/views/Zones.vue'),
        meta: { title: '域名管理' }
      },
      {
        path: '/quick-deploy',
        name: 'QuickDeploy',
        component: () => import('@/views/QuickDeploy.vue'),
        meta: { title: '一键加速' }
      },
      {
        path: '/workers',
        name: 'Workers',
        component: () => import('@/views/Workers.vue'),
        meta: { title: 'Workers 管理' }
      },
      {
        path: '/optimize',
        name: 'Optimize',
        component: () => import('@/views/Optimize.vue'),
        meta: { title: '自动优化' }
      },
      {
        path: '/history',
        name: 'History',
        component: () => import('@/views/History.vue'),
        meta: { title: '操作历史' }
      },
      {
        path: '/dns',
        name: 'DNS',
        component: () => import('@/views/DNS.vue'),
        meta: { title: 'DNS 记录' }
      },
      {
        path: '/ssl',
        name: 'SSL',
        component: () => import('@/views/SSL.vue'),
        meta: { title: 'SSL/TLS' }
      },
      {
        path: '/cache',
        name: 'Cache',
        component: () => import('@/views/Cache.vue'),
        meta: { title: '缓存管理' }
      },
      {
        path: '/firewall',
        name: 'Firewall',
        component: () => import('@/views/Firewall.vue'),
        meta: { title: '防火墙' }
      },
      {
        path: '/waf',
        name: 'WAF',
        component: () => import('@/views/WAF.vue'),
        meta: { title: 'WAF 规则' }
      },
      {
        path: '/rate-limits',
        name: 'RateLimits',
        component: () => import('@/views/RateLimits.vue'),
        meta: { title: '速率限制' }
      },
      {
        path: '/analytics',
        name: 'Analytics',
        component: () => import('@/views/Analytics.vue'),
        meta: { title: '统计分析' }
      },
      {
        path: '/page-rules',
        name: 'PageRules',
        component: () => import('@/views/PageRules.vue'),
        meta: { title: '页面规则' }
      },
      {
        path: '/certificates',
        name: 'Certificates',
        component: () => import('@/views/Certificates.vue'),
        meta: { title: '证书管理' }
      },
      {
        path: '/accounts',
        name: 'AccountManagement',
        component: () => import('@/views/AccountManagement.vue'),
        meta: { title: '账户管理' }
      },
      // 原有的 Dashboard 页面保留以兼容
      {
        path: '/dashboard',
        redirect: '/zones'
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
