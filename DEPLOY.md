# Cloudflare 可视化管理平台 - 部署文档

## 📋 项目概述

这是一款基于 **Rust + Vue 3** 构建的低占用 Cloudflare 可视化管理平台,通过 Docker Compose 实现一键部署。

### 技术栈

- **后端**: Rust + Actix-web (高性能、低内存占用)
- **前端**: Vue 3 + Vite + TypeScript + Naive UI
- **容器化**: Docker + Docker Compose
- **反向代理**: Nginx
- **数据存储**: 浏览器 LocalStorage (无需数据库)

### 资源占用

- **后端**: ~10-20MB 内存
- **前端**: ~5-10MB 内存 (Nginx)
- **总计**: ~20-30MB 内存占用
- **磁盘**: ~50MB

---

## 🚀 快速开始

### 前置要求

- Docker >= 20.10
- Docker Compose >= 2.0
- 2GB 可用内存
- 1GB 可用磁盘空间

### 一键部署

1. **克隆项目**

```bash
git clone <repository-url>
cd cloudflare-management-platform
```

2. **构建并启动服务**

```bash
docker-compose up -d --build
```

3. **访问应用**

打开浏览器访问: `http://localhost:3000`

4. **查看日志**

```bash
# 查看所有服务日志
docker-compose logs -f

# 查看特定服务日志
docker-compose logs -f backend
docker-compose logs -f frontend
docker-compose logs -f nginx
```

5. **停止服务**

```bash
docker-compose down
```

---

## 📦 本地开发

### 后端开发

```bash
cd backend

# 安装 Rust (如果未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 运行开发服务器
cargo run

# 构建 release 版本
cargo build --release
```

后端将运行在 `http://localhost:8080`

### 前端开发

```bash
cd frontend

# 安装依赖
npm install

# 运行开发服务器
npm run dev

# 构建生产版本
npm run build
```

前端将运行在 `http://localhost:5173`

---

## 🔧 配置说明

### 环境变量

#### 后端环境变量 (backend/.env)

```bash
RUST_LOG=info          # 日志级别: debug, info, warn, error
HOST=0.0.0.0          # 监听地址
PORT=8080             # 监听端口
```

#### 前端环境变量 (frontend/.env)

```bash
VITE_API_BASE_URL=/api  # API 基础路径
```

### 端口配置

在 `docker-compose.yml` 中修改端口映射:

```yaml
services:
  nginx:
    ports:
      - "3000:80"  # 修改为你想要的端口
```

---

## 🌐 生产部署

### 使用反向代理 (推荐)

如果你有自己的 Nginx 或 Traefik,可以代理到容器:

```nginx
# Nginx 配置示例
server {
    listen 80;
    server_name cf-manager.yourdomain.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 配置 HTTPS

1. **使用 Let's Encrypt**

```bash
# 安装 certbot
apt-get install certbot python3-certbot-nginx

# 获取证书
certbot --nginx -d cf-manager.yourdomain.com
```

2. **更新 Nginx 配置**

Certbot 会自动配置 SSL,或手动添加:

```nginx
server {
    listen 443 ssl http2;
    server_name cf-manager.yourdomain.com;

    ssl_certificate /etc/letsencrypt/live/cf-manager.yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/cf-manager.yourdomain.com/privkey.pem;

    location / {
        proxy_pass http://localhost:3000;
        # ... 其他配置
    }
}
```

---

## 🔐 安全说明

### 数据安全

- ✅ **API 凭证**: 仅存储在浏览器 LocalStorage,绝不上传到服务器
- ✅ **加密传输**: 所有 API 调用直连 Cloudflare 官方 API
- ✅ **无数据库**: 平台本身不存储任何用户数据
- ✅ **开源透明**: 所有代码开源,可审计

### 安全建议

1. **生产环境务必使用 HTTPS**
2. **限制服务器访问 IP (可选)**
3. **定期更新 Docker 镜像**
4. **使用强密码和 API Key**

---

## 📊 监控与维护

### 健康检查

```bash
# 检查后端健康状态
curl http://localhost:8080/health

# 检查前端是否正常
curl http://localhost:3000/health
```

### 查看容器状态

```bash
docker-compose ps
```

### 重启服务

```bash
# 重启所有服务
docker-compose restart

# 重启特定服务
docker-compose restart backend
```

### 清理日志

```bash
# 清理 Docker 日志
docker-compose down
docker system prune -a
```

---

## 🛠️ 故障排查

### 常见问题

#### 1. 端口被占用

```bash
# 查看端口占用
netstat -tulpn | grep :3000

# 修改 docker-compose.yml 中的端口映射
```

#### 2. 后端连接失败

```bash
# 检查后端日志
docker-compose logs backend

# 重启后端服务
docker-compose restart backend
```

#### 3. 前端无法访问

```bash
# 检查 Nginx 日志
docker-compose logs nginx

# 检查前端构建是否成功
docker-compose build frontend
```

#### 4. API 请求跨域问题

确保 `backend/src/main.rs` 中 CORS 配置正确:

```rust
let cors = Cors::default()
    .allow_any_origin()
    .allow_any_method()
    .allow_any_header()
    .max_age(3600);
```

---

## 📝 功能使用说明

### 1. 添加 Cloudflare 账户

1. 访问 [Cloudflare Dashboard](https://dash.cloudflare.com)
2. 点击右上角头像 → **我的个人资料** → **API 令牌**
3. 下拉到 **API 密钥** → 查看 **Global API Key**
4. 在平台中添加邮箱和 API Key

### 2. 一键加速部署

1. 选择要加速的域名
2. 填写 Worker 名称和目标网站
3. 配置缓存时间和 CDN 节点
4. 输入授权码 `1111`
5. 点击部署

### 3. 自动优化

- **安全优先模式**: 适合金融、政府网站
- **性能优先模式**: 适合电商、媒体网站

### 4. DNS 记录管理

支持添加、编辑、删除各类 DNS 记录:
- A / AAAA
- CNAME
- MX
- TXT
- SRV / NS

---

## 🔄 更新升级

### 拉取最新代码

```bash
git pull origin main
docker-compose down
docker-compose up -d --build
```

### 备份数据

由于数据存储在浏览器本地,建议定期导出账户信息:

1. 打开浏览器开发者工具 (F12)
2. 切换到 **Application** / **存储** 选项卡
3. 查看 **LocalStorage** → 导出 `cf_accounts` 键值

---

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request!

### 开发流程

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 提交 Pull Request

---

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

---

## 🙋 获取帮助

- **问题反馈**: [GitHub Issues](https://github.com/your-repo/issues)
- **功能建议**: [GitHub Discussions](https://github.com/your-repo/discussions)

---

## ⭐ 鸣谢

感谢以下开源项目:

- [Rust](https://www.rust-lang.org/)
- [Actix Web](https://actix.rs/)
- [Vue.js](https://vuejs.org/)
- [Naive UI](https://www.naiveui.com/)
- [Cloudflare API](https://api.cloudflare.com/)
