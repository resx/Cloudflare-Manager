# 快速开始指南

本指南将帮助你在 **5 分钟内**完成 Cloudflare 管理平台的部署和使用。

## 📋 准备工作 (3 分钟)

### 1. 安装 Docker

**Windows**
- 下载安装 [Docker Desktop for Windows](https://www.docker.com/products/docker-desktop/)
- 安装完成后重启电脑

**macOS**
- 下载安装 [Docker Desktop for Mac](https://www.docker.com/products/docker-desktop/)

**Linux**
```bash
# Ubuntu/Debian
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# 启动 Docker 服务
sudo systemctl start docker
sudo systemctl enable docker
```

### 2. 获取 Cloudflare API Key

1. 登录 [Cloudflare Dashboard](https://dash.cloudflare.com)
2. 点击右上角头像 → **我的个人资料**
3. 选择 **API 令牌** 标签
4. 向下滚动到 **API 密钥** 部分
5. 点击 **Global API Key** 旁边的 **查看**
6. 输入密码确认,复制 API Key
7. 记录你的 Cloudflare 邮箱和 API Key

## 🚀 部署平台 (1 分钟)

### 方式一: 一键脚本 (推荐)

**Windows**
1. 双击运行 `start.bat`
2. 等待构建完成 (~2-3 分钟)

**Linux/macOS**
```bash
chmod +x start.sh
./start.sh
```

### 方式二: 手动命令

```bash
# 进入项目目录
cd cloudflare-management-platform

# 启动所有服务
docker-compose up -d --build

# 查看服务状态
docker-compose ps
```

### 验证部署

打开浏览器访问: **http://localhost:3000**

如果看到登录界面,说明部署成功! ✅

## 🎯 开始使用 (1 分钟)

### 第一步: 添加账户

1. 首次访问会自动弹出 **添加账户** 窗口
2. 填写信息:
   - **邮箱**: 你的 Cloudflare 账户邮箱
   - **API Key**: 刚才获取的 Global API Key
   - **别名**: 可选,例如 "我的主账户"
3. 点击 **确认**

系统会自动验证凭证并加载你的域名列表。

### 第二步: 探索功能

**控制台页面**
- 查看域名数量和 DNS 记录统计
- 快速访问各功能模块

**功能列表**

| 图标 | 功能 | 说明 |
|------|------|------|
| 🏠 | 控制台 | 域名概览和快捷操作 |
| 👥 | 多账户管理 | 添加、切换多个 CF 账户 |
| 🚀 | 一键加速 | 30 秒部署 Worker CDN |
| ⚡ | 自动优化 | 一键应用最佳配置 |
| 🌐 | DNS 管理 | 增删改查 DNS 记录 |
| 🛡️ | 防火墙 | 管理安全规则 |
| 📝 | 操作历史 | 查看操作记录 |

## 🎮 常用操作示例

### 示例 1: 添加 DNS 记录

1. 点击左侧 **DNS 记录管理**
2. 选择域名
3. 点击 **添加记录**
4. 填写信息:
   - 类型: A
   - 名称: www
   - 内容: 192.168.1.1
   - TTL: 1 (自动)
   - 代理状态: 开启
5. 点击 **确认**

完成! DNS 记录已添加并实时生效。

### 示例 2: 一键加速部署

1. 点击左侧 **一键加速部署**
2. 填写表单:
   - 选择域名: example.com
   - Worker 名称: my-cdn
   - 目标网站: https://www.example.com
   - 访问域名: cdn.example.com
   - CDN 节点: cdns.doon.eu.org
   - 缓存时间: 3600 (1小时)
   - 授权码: 1111
3. 点击 **一键部署**

30 秒后,Worker 部署完成! 🎉

### 示例 3: 自动优化域名

1. 点击左侧 **自动优化**
2. 选择域名
3. 根据需求选择模式:
   - **安全优先**: 金融、政府、企业网站
   - **性能优先**: 电商、媒体、博客
4. 点击 **应用配置**

配置自动应用,域名性能/安全立即提升! 🚀

## 🔧 常见问题

### Q1: 端口 3000 被占用怎么办?

修改 `docker-compose.yml`:

```yaml
services:
  nginx:
    ports:
      - "8080:80"  # 改为其他端口
```

然后重新启动:
```bash
docker-compose down
docker-compose up -d
```

### Q2: 如何查看日志?

```bash
# 查看所有日志
docker-compose logs -f

# 查看后端日志
docker-compose logs -f backend

# 查看前端日志
docker-compose logs -f frontend
```

### Q3: 如何停止服务?

```bash
docker-compose down
```

### Q4: 忘记 API Key 怎么办?

1. 点击右上角账户下拉框
2. 选择 **多账户管理**
3. 删除旧账户
4. 重新添加新账户

### Q5: 数据存储在哪里?

所有数据存储在浏览器 LocalStorage,可以:
- 按 F12 打开开发者工具
- 切换到 **Application** 标签
- 查看 **LocalStorage** → `http://localhost:3000`

## 📚 下一步

- 📖 阅读 [完整文档](README.md)
- 🔧 查看 [部署指南](DEPLOY.md)
- 💡 查看 [项目结构](PROJECT_STRUCTURE.md)

## 🆘 获取帮助

- 🐛 问题反馈: [GitHub Issues](https://github.com/yourusername/cloudflare-management-platform/issues)
- 💬 技术交流: [GitHub Discussions](https://github.com/yourusername/cloudflare-management-platform/discussions)

---

**🎉 恭喜! 你已经完成了 Cloudflare 管理平台的部署和基本使用!**
