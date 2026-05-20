# 贡献指南

感谢你关注 Cloudflare 可视化管理平台。本文档说明当前仓库的开发、验证和提交约定。

## 开发环境

建议版本：

- Node.js 18+（Docker 构建使用 Node 20）
- npm 9+
- Rust stable（Docker 构建使用 Rust 1.88）
- Docker 与 Docker Compose v2

## 本地启动

后端：

```bash
cd backend
cargo run
```

前端：

```bash
cd frontend
npm install
npm run dev
```

前端默认运行在 `http://localhost:5173`，Vite 会把 `/api` 转发到 `http://localhost:8080`。

统一容器：

```bash
docker compose -f docker-compose.unified.yml up -d --build
```

访问 `http://localhost:3000`。

## 项目结构

```text
.
├── backend/                 # Rust Actix Web 后端
│   ├── src/main.rs          # 服务启动、CORS、路由注册
│   ├── src/handlers.rs      # HTTP handler
│   ├── src/cloudflare.rs    # Cloudflare API 客户端
│   └── src/models.rs        # 请求、响应与业务模型
├── frontend/                # Vue 3 前端
│   ├── src/api/             # Axios 客户端与接口封装
│   ├── src/views/           # 页面组件
│   ├── src/components/      # 复用组件
│   ├── src/stores/          # Pinia 状态
│   ├── src/i18n/            # 中英文翻译
│   └── src/data/            # Worker 模板等静态数据
├── Dockerfile               # 前后端统一镜像构建
├── docker-compose.unified.yml
├── docker-compose.prod.yml
├── nginx-unified.conf
└── supervisord.conf
```

## 常用验证

后端：

```bash
cd backend
cargo fmt --check
cargo test
```

前端：

```bash
cd frontend
npm run build:check
```

说明：`frontend/package.json` 中目前有 `npm run lint` 脚本，但仓库依赖没有声明 ESLint 相关包。除非本次变更同时补齐 lint 依赖和配置，否则不要把 `npm run lint` 作为必跑检查。

容器构建：

```bash
docker build -t cf-manager:local .
```

## 提交规范

Commit Message 推荐格式：

```text
<type>(<scope>): <subject>
```

常用 `type`：

- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式，不改变行为
- `refactor`: 重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建、依赖或工具维护

示例：

```bash
git commit -m "docs(readme): align feature list with current routes"
```

## Pull Request 要求

PR 描述建议包含：

- 变更目的
- 主要修改点
- 已执行的验证命令
- 影响范围和兼容性说明
- UI 变更截图或录屏（如适用）

提交前请确认：

- 代码或文档与当前实现一致
- 没有提交 API Token、`.env`、构建产物或本地缓存
- 新增页面同步更新路由、菜单和 i18n
- 新增 Cloudflare 功能同步更新 `API_TOKEN_PERMISSIONS.md`

## 代码风格

### Rust

- 使用 `cargo fmt`
- 公共模型和 handler 命名保持当前 `snake_case` / `PascalCase` 风格
- Cloudflare API 调用集中放在 `backend/src/cloudflare.rs`
- HTTP handler 放在 `backend/src/handlers.rs`
- 新路由在 `backend/src/main.rs` 的 `/cloudflare` scope 下注册

### TypeScript / Vue

- 使用 Vue 3 `<script setup lang="ts">`
- 页面放在 `frontend/src/views/`
- 复用 UI 组件放在 `frontend/src/components/`
- API 封装集中在 `frontend/src/api/index.ts`
- 账户状态使用 `frontend/src/stores/account.ts`
- 新增用户可见文本时同步更新 `frontend/src/i18n/locales/zh-CN.json` 和 `frontend/src/i18n/locales/en-US.json`

## 添加新的 Cloudflare API 功能

推荐顺序：

1. 在 `backend/src/models.rs` 添加请求和响应模型。
2. 在 `backend/src/cloudflare.rs` 添加 Cloudflare API 调用方法。
3. 在 `backend/src/handlers.rs` 添加 HTTP handler。
4. 在 `backend/src/main.rs` 注册 `/cloudflare/*` 路由。
5. 在 `frontend/src/api/index.ts` 添加前端调用封装。
6. 在 `frontend/src/views/` 或复用组件中接入界面。
7. 同步更新 i18n 和 `API_TOKEN_PERMISSIONS.md`。
8. 运行后端、前端和必要的容器构建验证。

## 添加新页面

1. 在 `frontend/src/views/` 创建 Vue 组件。
2. 在 `frontend/src/router/index.ts` 添加路由。
3. 在 `frontend/src/views/Layout.vue` 添加菜单入口。
4. 在 `frontend/src/components/ui/CommandPalette.vue` 添加搜索入口（如适用）。
5. 同步中英文翻译键。

## 调试

后端详细日志：

```bash
cd backend
RUST_LOG=debug cargo run
```

前端调试：

- 浏览器开发者工具
- Vue DevTools
- Network 面板检查 `/api` 请求和后端错误信息

容器调试：

```bash
docker logs -f cf-manager-app
docker exec -it cf-manager-app sh
```

## 安全注意事项

- 不要提交 Cloudflare API Token、Global API Key 或账户敏感信息。
- 不要把 Token 写入 Compose 文件、Dockerfile、前端源码或文档示例。
- 需要新增权限时，按最小权限原则更新 `API_TOKEN_PERMISSIONS.md`。
- 涉及认证、授权、Token 存储或生产部署策略的变更，需要在 PR 中单独说明风险。
