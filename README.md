# Cloudflare 可视化管理平台

<div align="center">

**🚀 功能强大 | 🎨 界面精美 | 🔒 安全可靠**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Docker](https://img.shields.io/badge/docker-ready-brightgreen.svg)](Dockerfile)
[![Vue 3](https://img.shields.io/badge/vue-3.x-green.svg)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

</div>

---

## 📌 项目简介

这是一款专为 Cloudflare 用户打造的**全功能可视化管理平台**，采用全新的 **GitLab Island Theme** 设计风格，将复杂的 CDN 配置变得简单直观。无需在 Cloudflare 官方后台来回切换，所有核心功能都集中在一个优雅的界面中。

> 💡 **设计理念**：以最简单、最直观的方式呈现 Cloudflare 强大功能，让 CDN 配置不再是专业人士的专利。

### ✨ 全新 UI 设计

采用 **GitLab Island Theme** 风格重构界面：

- 🏝️ **浮动岛屿设计** - 白色内容区域浮于淡紫色背景之上
- 🎨 **柔和配色方案** - 淡紫色 (#f1f3f9) 背景 + 浅蓝色 (#dbeafe) 激活状态
- 📐 **大圆角卡片** - 16px 圆角营造现代感
- ✨ **扁平化设计** - 细边框 + 极淡阴影，清爽简洁
- 🌈 **渐变横幅** - 粉紫渐变 (#FDF2F3 → #F1F0FB) 欢迎信息
- 📱 **响应式布局** - 完美适配所有设备

---

## ✨ 核心功能

### 🔐 多账户管理
- 安全的本地存储（API Token 仅存于浏览器）
- 一键切换多个 Cloudflare 账户
- 账户导入/导出功能

### 🚀 一键加速部署
- 30秒完成 Worker 快速部署
- 内置优选节点配置
- 自定义缓存时间设置
- 授权码验证机制

### 🤖 智能优化
- **安全优先模式** - 一键配置高安全防护
- **性能优先模式** - 一键优化加载速度
- **30+ 精细配置** - 专业用户完全掌控

### 🌐 DNS 管理
- 支持所有记录类型（A、AAAA、CNAME、MX、TXT等）
- 批量操作支持
- 一键代理开关
- 实时生效

### 🛡️ 防火墙规则
- 图形化规则编辑
- 6种常用模板
- 多种拦截动作
- 规则状态切换

### 📝 操作历史
- 完整审计追踪
- 操作日志记录
- 快速搜索过滤

---

## 🛠️ 技术架构

### 前端
- **框架**: Vue 3 + TypeScript
- **构建工具**: Vite
- **UI 框架**: Tailwind CSS (Island Theme)
- **状态管理**: Pinia
- **HTTP 客户端**: Axios

### 后端
- **语言**: Rust
- **Web 框架**: Actix-web 4.4
- **内存占用**: ~10-20MB
- **CPU 占用**: < 1%

### 容器化
- **统一镜像**: Frontend + Backend + Nginx
- **多阶段构建**: 优化镜像大小
- **Supervisor**: 进程管理

---

## 📦 快速部署

### 方式一: Docker Compose（推荐）

**使用统一镜像部署**

```bash
# 拉取并启动
docker-compose -f docker-compose.unified.yml up -d

# 访问应用
http://localhost:3000
```

### 方式二: 本地开发

**后端开发**
```bash
cd backend
cargo run
# 运行在 http://localhost:8080
```

**前端开发**
```bash
cd frontend
npm install
npm run dev
# 运行在 http://localhost:5173
```

---

## 🎨 Island Theme 设计特点

### 核心设计原则

1. **统一背景** - 整个页面使用 #f1f3f9 浅灰紫色
2. **白色岛屿** - 内容区域为纯白色，16px 大圆角
3. **透明侧边栏** - 融入背景，无独立背景色
4. **浅蓝激活** - 导航选中使用 #dbeafe 浅蓝色
5. **扁平设计** - 细边框 (#e5e7eb)，极淡阴影
6. **留白充足** - 大量空白，内容呼吸感强

### 颜色方案

```css
--background: #f1f3f9;     /* 全局背景 */
--card: #ffffff;           /* 岛屿/卡片 */
--primary: #1d4ed8;        /* 主要颜色（蓝色）*/
--accent: #dbeafe;         /* 激活状态 */
--border: #e5e7eb;         /* 边框颜色 */
--success: #d1fae5;        /* 成功状态 */
--banner-from: #FDF2F3;    /* 横幅渐变起始 */
--banner-to: #F1F0FB;      /* 横幅渐变结束 */
```

---

## 📊 资源占用

| 组件 | 内存 | CPU | 磁盘 |
|------|------|-----|------|
| Rust 后端 | ~15MB | < 1% | ~15MB |
| 前端(Nginx) | ~10MB | < 1% | ~25MB |
| Supervisor | ~5MB | < 1% | ~5MB |
| **总计** | **~30MB** | **< 2%** | **~50MB** |

---

## 🔒 安全说明

✅ **完全本地化** - API Token 仅存储在浏览器
✅ **零数据收集** - 不收集任何用户数据
✅ **直连 API** - 直接连接 Cloudflare 官方 API
✅ **端到端加密** - HTTPS 加密传输

---

## 📚 详细文档

- 📖 [快速入门](QUICK_START_GUIDE.md)
- 🚀 [部署指南](DEPLOYMENT.md)
- 🔧 [贡献指南](CONTRIBUTING.md)
- 📝 [API 权限](API_TOKEN_PERMISSIONS.md)

---

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 提交 Pull Request

---

## 📄 许可证

本项目采用 [MIT License](LICENSE)

---

## ⭐ 技术栈

- **前端**: Vue 3, Vite, Tailwind CSS, TypeScript, Pinia
- **后端**: Rust, Actix-web, Reqwest, Serde
- **容器**: Docker, Docker Compose, Nginx, Supervisor
- **CI/CD**: GitLab CI with Docker-in-Docker

---

**🌟 如果这个项目对你有帮助，请给个 Star ⭐**
