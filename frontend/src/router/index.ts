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
        meta: { title: '控制台' }
      },
      {
        path: '/accounts',
        name: 'Accounts',
        component: () => import('@/views/Accounts.vue'),
        meta: { title: '多账户管理' }
      },
      {
        path: '/quick-deploy',
        name: 'QuickDeploy',
        component: () => import('@/views/QuickDeploy.vue'),
        meta: { title: '一键加速部署' }
      },
      {
        path: '/optimize',
        name: 'Optimize',
        component: () => import('@/views/Optimize.vue'),
        meta: { title: '自动优化' }
      },
      {
        path: '/dns',
        name: 'DNS',
        component: () => import('@/views/DNS.vue'),
        meta: { title: 'DNS 记录管理' }
      },
      {
        path: '/firewall',
        name: 'Firewall',
        component: () => import('@/views/Firewall.vue'),
        meta: { title: '防火墙规则' }
      },
      {
        path: '/history',
        name: 'History',
        component: () => import('@/views/History.vue'),
        meta: { title: '操作历史' }
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
