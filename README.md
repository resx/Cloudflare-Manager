# Cloudflare 可视化管理平台

<div align="center">

**Cloudflare 账户、域名、Workers、DNS、安全、缓存与分析的一体化管理界面**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Docker](https://img.shields.io/badge/docker-ready-brightgreen.svg)](Dockerfile)
[![Vue 3](https://img.shields.io/badge/vue-3.x-green.svg)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/rust-1.88+-orange.svg)](https://www.rust-lang.org/)

</div>

---

## 项目简介

Cloudflare 可视化管理平台将常用 Cloudflare 操作集中到一个本地运行的 Web 控制台中。前端负责账户切换、操作界面和本地历史记录，后端作为 Cloudflare API 代理执行实际管理请求。

项目默认不落库保存 Cloudflare 凭证。API Token 由浏览器端账户配置保存并随请求发送给后端，后端仅转发到 Cloudflare API。

## 核心功能

### 账户与域名

- 多 Cloudflare 账户管理与快速切换
- 自动拉取 Account 与 Zone 列表
- 账户配置导入、导出与本地保存

### DNS 与流量管理

- DNS 记录查看、创建、编辑、删除
- 缓存清除，支持全站、指定文件与标签
- Page Rules 创建、更新、删除
- Zone 设置读取与安全/性能模式自动优化

### Workers 生态

- Workers 脚本列表、查看、上传、更新与删除
- Workers 路由绑定管理
- Worker 模板库，内置反向代理、URL 重定向、API 网关、防盗链、缓存优化、A/B 测试、自定义响应头、JSON API Mock 等模板
- Workers KV Namespace 与键值管理
- D1 数据库列表、创建、删除与 SQL 查询执行

### 安全与证书

- Firewall Rules 图形化管理
- WAF 包与规则模式配置
- Rate Limiting 规则管理
- SSL 证书与自定义证书查看、上传、删除

### 一键优选部署

- Worker 反向代理部署向导，支持源站、路由与缓存 TTL 配置
- SaaS 优选模式，覆盖回退源、Custom Hostname、验证记录与优选 DNS 配置流程

### 统计与体验

- Analytics 流量、缓存命中、带宽、威胁与国家/状态码分布统计
- 操作历史记录与快速检索
- 中英文界面，基于 Vue I18n
- Island Theme UI，支持响应式布局

## 技术架构

### 前端

- Vue 3 + TypeScript + Vite
- Vue Router + Pinia + Vue I18n
- Tailwind CSS + Naive UI + `@vicons/ionicons5`
- Axios API 客户端
- ECharts / Vue ECharts 数据可视化

### 后端

- Rust 2021
- Actix Web 4 + Actix CORS
- Reqwest 调用 Cloudflare REST / GraphQL API
- Serde / Serde JSON 数据序列化
- 环境变量控制监听地址、端口与日志级别

### 容器化

- 单一 Docker 镜像整合前端、后端与 Nginx
- Nginx 提供静态资源与 `/api` 反向代理
- Supervisor 同容器管理 Nginx 与后端进程
- Dockerfile 使用 Node 20 Alpine、Rust 1.88 Alpine 与 Nginx Alpine 多阶段构建

## 快速运行

### Docker Compose 本地构建

```bash
docker compose -f docker-compose.unified.yml up -d --build
```

访问：

```text
http://localhost:3000
```

### 使用预构建生产镜像

```bash
docker compose -f docker-compose.prod.yml up -d
```

生产镜像地址见 [docker-compose.prod.yml](docker-compose.prod.yml)。

### 本地开发

后端：

```bash
cd backend
cargo run
```

默认监听 `http://localhost:8080`。

前端：

```bash
cd frontend
npm install
npm run dev
```

默认访问 `http://localhost:5173`。Vite 会将 `/api` 代理到 `http://localhost:8080`。

## 常用检查

```bash
cd backend
cargo fmt --check
cargo test
```

```bash
cd frontend
npm run build:check
```

说明：当前 `frontend/package.json` 中保留了 `npm run lint` 脚本，但仓库依赖尚未声明 ESLint 相关包；在补齐 lint 依赖前，前端以 `npm run build:check` 作为主要类型与构建验证。

## API 路由

浏览器端请求统一使用 `/api` 前缀：

```text
Frontend -> /api/* -> Nginx -> Rust backend -> Cloudflare API
```

后端内部路由以 `/cloudflare/*` 为主，例如：

- `/cloudflare/accounts`
- `/cloudflare/zones`
- `/cloudflare/dns/records`
- `/cloudflare/workers/list`
- `/cloudflare/kv/namespaces`
- `/cloudflare/d1/databases`
- `/cloudflare/custom-hostnames`

容器内健康检查可通过前端根路径确认，也可访问：

```text
http://localhost:3000/api/health
```

## 安全说明

- API Token 不提交到仓库，不写入后端文件或数据库
- 后端只作为当前请求的 Cloudflare API 代理
- 建议使用 Cloudflare API Token，不建议使用 Global API Key
- 生产环境请在入口网关或反向代理层启用 HTTPS
- Token 权限按最小权限原则配置，详见 [API_TOKEN_PERMISSIONS.md](API_TOKEN_PERMISSIONS.md)

## 详细文档

- [部署指南](DEPLOYMENT.md)
- [贡献指南](CONTRIBUTING.md)
- [API Token 权限要求](API_TOKEN_PERMISSIONS.md)

## 许可证

本项目采用 [MIT License](LICENSE)。
