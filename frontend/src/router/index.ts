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
        path: '/firewall',
        name: 'Firewall',
        component: () => import('@/views/Firewall.vue'),
        meta: { title: '防火墙' }
      },
      // 原有的 Dashboard 和 Accounts 页面保留以兼容
      {
        path: '/dashboard',
        redirect: '/zones'
      },
      {
        path: '/accounts',
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
