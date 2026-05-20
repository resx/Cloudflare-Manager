import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Layout',
    component: () => import('@/views/Layout.vue'),
    redirect: '/dashboard',
    children: [
      {
        path: '/dashboard',
        name: 'Dashboard',
        component: () => import('@/views/Dashboard.vue'),
        meta: { title: 'common.dashboard' }
      },
      {
        path: '/zones',
        name: 'Zones',
        component: () => import('@/views/Zones.vue'),
        meta: { title: 'common.zones' }
      },
      {
        path: '/quick-deploy',
        name: 'QuickDeploy',
        component: () => import('@/views/QuickDeploy.vue'),
        meta: { title: 'quickDeploy.title' }
      },
      {
        path: '/workers',
        name: 'Workers',
        component: () => import('@/views/Workers.vue'),
        meta: { title: 'common.workers' }
      },
      {
        path: '/worker-templates',
        name: 'WorkerTemplates',
        component: () => import('@/views/WorkerTemplates.vue'),
        meta: { title: 'workerTemplates.title' }
      },
      {
        path: '/workers-kv',
        name: 'WorkersKV',
        component: () => import('@/views/WorkersKV.vue'),
        meta: { title: 'kv.title' }
      },
      {
        path: '/d1',
        name: 'D1',
        component: () => import('@/views/D1.vue'),
        meta: { title: 'd1.title' }
      },
      {
        path: '/optimize',
        name: 'Optimize',
        component: () => import('@/views/Optimize.vue'),
        meta: { title: 'common.optimize' }
      },
      {
        path: '/history',
        name: 'History',
        component: () => import('@/views/History.vue'),
        meta: { title: 'common.history' }
      },
      {
        path: '/dns',
        name: 'DNS',
        component: () => import('@/views/DNS.vue'),
        meta: { title: 'common.dns' }
      },
      {
        path: '/ssl',
        name: 'SSL',
        component: () => import('@/views/SSL.vue'),
        meta: { title: 'common.ssl' }
      },
      {
        path: '/cache',
        name: 'Cache',
        component: () => import('@/views/Cache.vue'),
        meta: { title: 'common.cache' }
      },
      {
        path: '/firewall',
        name: 'Firewall',
        component: () => import('@/views/Firewall.vue'),
        meta: { title: 'common.firewall' }
      },
      {
        path: '/waf',
        name: 'WAF',
        component: () => import('@/views/WAF.vue'),
        meta: { title: 'common.waf' }
      },
      {
        path: '/rate-limits',
        name: 'RateLimits',
        component: () => import('@/views/RateLimits.vue'),
        meta: { title: 'rateLimits.title' }
      },
      {
        path: '/analytics',
        name: 'Analytics',
        component: () => import('@/views/Analytics.vue'),
        meta: { title: 'common.analytics' }
      },
      {
        path: '/page-rules',
        name: 'PageRules',
        component: () => import('@/views/PageRules.vue'),
        meta: { title: 'common.pageRules' }
      },
      {
        path: '/certificates',
        name: 'Certificates',
        component: () => import('@/views/Certificates.vue'),
        meta: { title: 'certificates.title' }
      },
      {
        path: '/accounts',
        name: 'Accounts',
        component: () => import('@/views/Accounts.vue'),
        meta: { title: 'common.accounts' }
      },
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
