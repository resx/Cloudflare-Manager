# ============================================
# 第一阶段：构建前端
# ============================================
FROM node:20-alpine as frontend-builder

WORKDIR /frontend

# 复制前端依赖文件
COPY frontend/package*.json ./

# 安装前端依赖
RUN npm install --production=false

# 复制前端源代码
COPY frontend/ ./

# 构建前端应用
RUN npm run build

# ============================================
# 第二阶段：构建后端
# ============================================
FROM rust:1.83-alpine as backend-builder

# 安装必要的构建工具
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /backend

# 先复制依赖文件以利用 Docker 缓存
COPY backend/Cargo.toml ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制后端源代码并构建
COPY backend/src ./src
RUN touch src/main.rs && \
    cargo build --release

# ============================================
# 第三阶段：运行时镜像（整合前后端 + nginx）
# ============================================
FROM nginx:alpine

# 安装 supervisor 用于管理多个进程
RUN apk add --no-cache ca-certificates supervisor

# 复制前端构建产物到 nginx 目录
COPY --from=frontend-builder /frontend/dist /usr/share/nginx/html

# 复制后端可执行文件
COPY --from=backend-builder /backend/target/release/cloudflare-manager-backend /app/backend

# 复制 nginx 配置文件（反向代理配置）
COPY nginx-unified.conf /etc/nginx/conf.d/default.conf

# 复制 supervisor 配置文件
COPY supervisord.conf /etc/supervisord.conf

# 创建日志目录
RUN mkdir -p /var/log/supervisor /var/log/backend

# 暴露端口（只需要一个端口）
EXPOSE 80

# 使用 supervisor 启动 nginx 和后端服务
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
