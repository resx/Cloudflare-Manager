# 统一镜像部署指南

本项目提供单一 Docker 镜像，镜像内同时包含前端静态站点、Rust 后端 API 与 Nginx 反向代理。

## 架构

```text
Browser
  |
  | http://localhost:3000
  v
Nginx :80
  |-- /            -> Vue 前端静态文件
  |-- /api/*       -> Rust backend :8080
                         |
                         v
                    Cloudflare API
```

容器内使用 Supervisor 同时管理 Nginx 与后端进程。后端默认监听 `127.0.0.1:8080`（由 Compose 文件设置），只通过 Nginx 暴露给浏览器。

## 镜像构建

Dockerfile 使用三阶段构建：

| 阶段 | 基础镜像 | 作用 |
|---|---|---|
| `frontend-builder` | `node:20-alpine` | 安装前端依赖并执行 `npm run build` |
| `backend-builder` | `rust:1.88-alpine` | 编译 Rust release 二进制 |
| runtime | `nginx:alpine` | 运行 Nginx、后端与 Supervisor |

## 本地部署

### Docker Compose 构建并启动

```bash
docker compose -f docker-compose.unified.yml up -d --build
```

如果本机只安装了旧版 Compose，也可以使用：

```bash
docker-compose -f docker-compose.unified.yml up -d --build
```

访问地址：

```text
http://localhost:3000
```

### 本地手动构建镜像

```bash
docker build -t cf-manager:local .
docker run -d --name cf-manager -p 3000:80 cf-manager:local
```

## 生产镜像部署

当前生产 Compose 文件使用以下镜像：

```text
registry.sao.im/moriarty/cloudflare-management-platform:latest
```

启动：

```bash
docker compose -f docker-compose.prod.yml up -d
```

单容器运行示例：

```bash
docker run -d \
  --name cf-manager-app \
  --restart unless-stopped \
  -p 3000:80 \
  -e RUST_LOG=info \
  -e HOST=127.0.0.1 \
  -e PORT=8080 \
  registry.sao.im/moriarty/cloudflare-management-platform:latest
```

生产环境建议在外层网关、反向代理或负载均衡器上配置 HTTPS。

## 环境变量

| 变量 | 默认值 | 说明 |
|---|---|---|
| `RUST_LOG` | `info` | 后端日志级别，支持 `debug`、`info`、`warn`、`error` |
| `HOST` | `0.0.0.0` | 后端监听地址；Compose 中设置为 `127.0.0.1` |
| `PORT` | `8080` | 后端监听端口 |

## 健康检查

Compose 文件默认通过容器内 Nginx 根路径检查：

```text
http://localhost/
```

对外可以用以下地址确认前端和 API 是否可达：

```bash
curl http://localhost:3000/
curl http://localhost:3000/api/health
```

`/api/health` 会经过 Nginx 代理到后端 `/health`。

## 日志与排障

查看容器状态：

```bash
docker ps -a --filter name=cf-manager-app
```

查看运行日志：

```bash
docker logs -f cf-manager-app
```

进入容器检查进程：

```bash
docker exec -it cf-manager-app sh
ps aux
```

常用日志位置：

```text
/var/log/nginx/
/var/log/supervisor/
/var/log/backend/
```

验证 Nginx 配置：

```bash
docker exec -it cf-manager-app nginx -t
```

## 更新镜像

```bash
docker compose -f docker-compose.prod.yml pull
docker compose -f docker-compose.prod.yml up -d
```

如需手动替换容器：

```bash
docker pull registry.sao.im/moriarty/cloudflare-management-platform:latest
docker stop cf-manager-app
docker rm cf-manager-app
docker run -d \
  --name cf-manager-app \
  --restart unless-stopped \
  -p 3000:80 \
  registry.sao.im/moriarty/cloudflare-management-platform:latest
```

## 安全建议

1. 使用 HTTPS 暴露生产入口。
2. 限制管理平台访问来源，例如内网、VPN、Zero Trust 或认证网关。
3. 不要把 Cloudflare API Token 写入镜像、Compose 文件或环境变量。
4. 定期更新镜像与基础运行环境。
5. 配置 Docker 日志轮转，避免日志占满磁盘。
