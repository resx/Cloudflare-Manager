# 多阶段构建 - Rust 后端
FROM rust:1.83-alpine as builder

# 安装必要的构建工具
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# 先复制依赖文件以利用 Docker 缓存
COPY Cargo.toml ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制源代码并构建
COPY src ./src
RUN touch src/main.rs && \
    cargo build --release

# 运行阶段 - 最小化镜像
FROM alpine:latest

RUN apk add --no-cache ca-certificates

WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/cloudflare-manager-backend .

# 创建非 root 用户
RUN addgroup -g 1000 cfmanager && \
    adduser -D -u 1000 -G cfmanager cfmanager && \
    chown -R cfmanager:cfmanager /app

USER cfmanager

EXPOSE 8080

CMD ["./cloudflare-manager-backend"]
