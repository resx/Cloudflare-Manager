# 项目结构

```
cloudflare-management-platform/
├── backend/                      # Rust 后端服务
│   ├── src/
│   │   ├── main.rs              # 主入口,HTTP 服务器配置
│   │   ├── models.rs            # 数据模型定义
│   │   ├── handlers.rs          # API 处理器
│   │   └── cloudflare.rs        # Cloudflare API 客户端
│   ├── Cargo.toml               # Rust 依赖配置
│   ├── Dockerfile               # 后端 Docker 镜像
│   └── .env.example             # 环境变量示例
│
├── frontend/                     # Vue 3 前端应用
│   ├── src/
│   │   ├── main.ts              # Vue 应用入口
│   │   ├── App.vue              # 根组件
│   │   ├── router/
│   │   │   └── index.ts         # 路由配置
│   │   ├── stores/
│   │   │   └── account.ts       # 账户状态管理
│   │   ├── api/
│   │   │   └── index.ts         # API 接口封装
│   │   └── views/               # 页面组件
│   │       ├── Layout.vue       # 主布局
│   │       ├── Dashboard.vue    # 控制台
│   │       ├── Accounts.vue     # 多账户管理
│   │       ├── QuickDeploy.vue  # 一键加速部署
│   │       ├── Optimize.vue     # 自动优化
│   │       ├── DNS.vue          # DNS 管理
│   │       ├── Firewall.vue     # 防火墙规则
│   │       └── History.vue      # 操作历史
│   ├── package.json             # Node 依赖配置
│   ├── vite.config.ts           # Vite 构建配置
│   ├── tsconfig.json            # TypeScript 配置
│   ├── Dockerfile               # 前端 Docker 镜像
│   └── .env.example             # 环境变量示例
│
├── nginx/                        # Nginx 配置
│   ├── nginx.conf               # Nginx 主配置
│   └── conf.d/
│       └── default.conf         # 站点配置
│
├── docker-compose.yml            # Docker Compose 编排
├── start.sh                      # Linux/Mac 启动脚本
├── start.bat                     # Windows 启动脚本
├── README.md                     # 项目说明
├── DEPLOY.md                     # 部署文档
├── LICENSE                       # 开源许可证
└── .gitignore                    # Git 忽略配置
```

## 核心文件说明

### 后端 (Rust)

**`backend/src/main.rs`**
- HTTP 服务器初始化
- 路由配置
- CORS 中间件
- 日志配置

**`backend/src/models.rs`**
- API 请求/响应数据结构
- Zone、DNS、Firewall 等模型定义
- 序列化/反序列化配置

**`backend/src/handlers.rs`**
- API 端点处理函数
- 请求验证
- 错误处理

**`backend/src/cloudflare.rs`**
- Cloudflare API 客户端封装
- HTTP 请求发送
- 响应解析

### 前端 (Vue 3)

**`frontend/src/main.ts`**
- Vue 应用创建
- 插件注册 (Pinia, Router, Naive UI)

**`frontend/src/router/index.ts`**
- 路由定义
- 页面组件懒加载

**`frontend/src/stores/account.ts`**
- 账户状态管理
- LocalStorage 持久化
- 账户切换逻辑

**`frontend/src/api/index.ts`**
- Axios 配置
- API 接口封装
- 请求/响应拦截器

**`frontend/src/views/`**
- 各功能页面组件
- UI 交互逻辑
- 数据展示

### 配置文件

**`docker-compose.yml`**
- 服务编排
- 网络配置
- 卷挂载

**`nginx/conf.d/default.conf`**
- 反向代理配置
- 静态资源服务
- API 路由转发

## 数据流向

```
用户浏览器
    │
    ├── LocalStorage (存储账户凭证)
    │
    ▼
Vue 3 前端 (http://localhost:3000)
    │
    ├── 读取本地凭证
    ├── 发送 API 请求 (/api/*)
    │
    ▼
Nginx 反向代理
    │
    ├── 静态资源: 直接返回
    ├── API 请求: 转发到后端
    │
    ▼
Rust 后端 (http://localhost:8080)
    │
    ├── 解析请求
    ├── 提取凭证
    ├── 调用 Cloudflare API
    │
    ▼
Cloudflare API (https://api.cloudflare.com)
    │
    ├── 处理请求
    ├── 返回结果
    │
    ▼
Rust 后端
    │
    ├── 解析响应
    ├── 返回给前端
    │
    ▼
Vue 3 前端
    │
    └── 更新 UI
```

## 技术亮点

### 1. 低内存占用
- Rust 编译后二进制文件小 (~5MB)
- 运行时内存占用低 (~10-20MB)
- Alpine Linux 基础镜像

### 2. 高性能
- Actix-web 异步处理
- Vue 3 Composition API
- Vite 快速构建

### 3. 安全设计
- 凭证仅存储在浏览器
- 直连 Cloudflare API
- 无中间服务器存储

### 4. 易于部署
- Docker 容器化
- 一键启动脚本
- 零配置运行

### 5. 开发友好
- TypeScript 类型检查
- 热重载开发服务器
- 清晰的代码结构
