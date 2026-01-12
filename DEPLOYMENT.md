# 统一镜像部署指南

本项目已配置为构建单一的统一 Docker 镜像，包含前端、后端和 Nginx 反向代理。

## 架构说明

### 统一镜像架构
```
┌─────────────────────────────────────┐
│        Docker Container             │
│                                     │
│  ┌──────────────────────────────┐  │
│  │   Nginx (Port 80)            │  │
│  │   - 静态文件服务（前端）       │  │
│  │   - 反向代理 /api -> :8080   │  │
│  └──────────────────────────────┘  │
│              ▼                      │
│  ┌──────────────────────────────┐  │
│  │   Rust Backend (Port 8080)   │  │
│  │   - Cloudflare API 处理      │  │
│  └──────────────────────────────┘  │
│                                     │
│  ┌──────────────────────────────┐  │
│  │   Supervisor                 │  │
│  │   - 进程管理                 │  │
│  └──────────────────────────────┘  │
└─────────────────────────────────────┘
         对外暴露端口: 80
```

## 本地开发与测试

### 1. 本地构建镜像
```bash
# 构建统一镜像
docker build -t cf-manager:local .

# 查看镜像大小
docker images | grep cf-manager
```

### 2. 本地运行容器
```bash
# 使用 docker-compose
docker-compose -f docker-compose.unified.yml up -d

# 或直接运行
docker run -d \
  -p 3000:80 \
  --name cf-manager \
  cf-manager:local
```

### 3. 访问应用
```
前端: http://localhost:3000
API: http://localhost:3000/api/
```

### 4. 查看日志
```bash
# 查看所有日志
docker logs cf-manager

# 实时查看日志
docker logs -f cf-manager

# 进入容器查看详细日志
docker exec -it cf-manager sh
tail -f /var/log/supervisor/backend-stdout.log
tail -f /var/log/supervisor/nginx-stdout.log
```

## 镜像优化

### 多阶段构建优势
1. **前端构建阶段** - 使用 Node.js 20 构建 Vue 3 应用
2. **后端构建阶段** - 使用 Rust 1.83 编译 Actix-web 应用
3. **运行时阶段** - 使用 Nginx Alpine（体积小，安全性高）

### 镜像大小对比
```
传统方式（两个镜像）:
  - frontend: ~150MB
  - backend: ~50MB
  总计: ~200MB

统一镜像:
  - 总大小: ~180MB（共享基础层）
```

## 故障排查

### 容器无法启动
```bash
# 检查容器状态
docker ps -a | grep cf-manager

# 查看启动日志
docker logs cf-manager

# 检查健康状态
docker inspect cf-manager | grep Health -A 10
```

### 后端 API 无法访问
```bash
# 进入容器检查后端进程
docker exec -it cf-manager sh
ps aux | grep backend

# 检查端口监听
netstat -tlnp | grep 8080

# 查看后端日志
tail -f /var/log/supervisor/backend-stdout.log
```

### Nginx 配置问题
```bash
# 测试 Nginx 配置
docker exec -it cf-manager nginx -t

# 重新加载 Nginx
docker exec -it cf-manager nginx -s reload
```

## 环境变量配置

### 可配置的环境变量
- `RUST_LOG`: 后端日志级别（debug/info/warn/error）
- `HOST`: 后端监听地址（默认 127.0.0.1）
- `PORT`: 后端监听端口（默认 8080）

## 安全建议

1. 生产环境使用 HTTPS（在 Nginx 前面配置 TLS）
2. 定期更新镜像以获取安全补丁
3. 使用 `stable` 标签部署生产环境
4. 设置日志轮转避免磁盘占满
5. 配置防火墙规则限制访问

## 性能优化

### Nginx 缓存配置
统一镜像中的 Nginx 已配置：
- 静态资源缓存（1年）
- Gzip 压缩
- 合理的超时设置

### 后端性能
- Rust 编译时优化（release 模式）
- 异步 I/O 处理
- 高效的内存管理

## 监控建议

### 日志收集
```bash
# 使用 docker logs 驱动
docker run -d \
  --log-driver=json-file \
  --log-opt max-size=10m \
  --log-opt max-file=3 \
  registry.example.com/your-group/cloudflare-manager:latest
```

### 健康检查
容器已配置健康检查，可通过以下命令查看：
```bash
docker inspect cf-manager | grep -A 10 Health
```

## 支持

如有问题，请查看：
- 容器日志：`docker logs cf-manager`
- Supervisor 日志：`/var/log/supervisor/`
- Nginx 日志：`/var/log/nginx/`
- Backend 日志：`/var/log/supervisor/backend-*.log`
