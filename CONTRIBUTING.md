# 贡献指南

感谢你对 Cloudflare 管理平台的关注! 本文档将帮助你了解如何为项目做出贡献。

## 📋 目录

- [行为准则](#行为准则)
- [如何贡献](#如何贡献)
- [开发环境搭建](#开发环境搭建)
- [提交规范](#提交规范)
- [代码风格](#代码风格)
- [测试](#测试)

## 行为准则

请遵守以下基本准则:

- 尊重所有贡献者
- 保持友好和建设性的讨论
- 接受有益的批评
- 关注对社区最有利的事情

## 如何贡献

### 报告 Bug

如果你发现了 Bug,请:

1. 在 [Issues](https://github.com/yourusername/cloudflare-management-platform/issues) 中搜索是否已有相关问题
2. 如果没有,创建新 Issue,包含:
   - 清晰的标题
   - 详细的问题描述
   - 复现步骤
   - 期望行为
   - 实际行为
   - 环境信息 (操作系统、浏览器、Docker 版本等)
   - 截图或日志 (如果适用)

### 建议新功能

1. 在 Issues 中创建 Feature Request
2. 清楚描述功能需求和使用场景
3. 说明为什么这个功能对项目有价值

### 提交代码

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 进行开发
4. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
5. 推送到分支 (`git push origin feature/AmazingFeature`)
6. 创建 Pull Request

## 开发环境搭建

### 后端开发 (Rust)

**1. 安装 Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**2. 安装依赖并运行**

```bash
cd backend
cargo build
cargo run
```

后端将运行在 `http://localhost:8080`

**3. 开发工具**

推荐使用:
- VS Code + rust-analyzer 插件
- IntelliJ IDEA + Rust 插件

### 前端开发 (Vue 3)

**1. 安装 Node.js**

确保 Node.js >= 18.0

```bash
node --version
```

**2. 安装依赖并运行**

```bash
cd frontend
npm install
npm run dev
```

前端将运行在 `http://localhost:5173`

**3. 开发工具**

推荐使用:
- VS Code + Volar 插件
- WebStorm

### 完整开发环境

同时运行前后端:

```bash
# 终端 1: 后端
cd backend
cargo run

# 终端 2: 前端
cd frontend
npm run dev
```

访问 `http://localhost:5173` 进行开发。

## 提交规范

### Commit Message 格式

使用以下格式:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Type 类型:**

- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式 (不影响功能)
- `refactor`: 重构
- `perf`: 性能优化
- `test`: 测试
- `chore`: 构建/工具相关

**示例:**

```bash
feat(dns): 添加批量删除 DNS 记录功能

- 支持多选 DNS 记录
- 添加批量删除确认对话框
- 优化删除性能

Closes #123
```

### Pull Request 规范

**标题格式:**

```
[Type] Brief description
```

示例: `[Feature] Add batch DNS record deletion`

**PR 描述应包含:**

- 功能/修复说明
- 相关 Issue 编号
- 测试步骤
- 截图 (如果适用)

**PR 模板:**

```markdown
## 类型
- [ ] 新功能
- [ ] Bug 修复
- [ ] 文档更新
- [ ] 性能优化
- [ ] 其他

## 描述
简要描述此 PR 的目的

## 相关 Issue
Closes #xxx

## 测试步骤
1. ...
2. ...

## 截图
(如果适用)

## Checklist
- [ ] 代码遵循项目风格
- [ ] 已进行自测
- [ ] 已更新相关文档
- [ ] 提交信息符合规范
```

## 代码风格

### Rust 代码风格

使用 `rustfmt`:

```bash
cargo fmt
```

使用 `clippy` 检查:

```bash
cargo clippy
```

**规范:**

- 使用 4 空格缩进
- 函数名使用 snake_case
- 类型名使用 PascalCase
- 常量使用 SCREAMING_SNAKE_CASE
- 添加必要的注释

**示例:**

```rust
/// 获取所有 Zone
pub async fn get_zones(&self) -> Result<Vec<Zone>, String> {
    let url = format!("{}/zones", CLOUDFLARE_API_BASE);

    let response = self.client
        .get(&url)
        .headers(self.get_headers())
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    // 解析响应...
}
```

### TypeScript/Vue 代码风格

使用 ESLint:

```bash
cd frontend
npm run lint
```

**规范:**

- 使用 2 空格缩进
- 使用单引号
- 组件名使用 PascalCase
- 函数名使用 camelCase
- 使用 TypeScript 类型注解

**Vue 组件示例:**

```vue
<template>
  <n-card :title="title">
    <n-space>
      <!-- 内容 -->
    </n-space>
  </n-card>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  title: string
}

const props = defineProps<Props>()
const count = ref(0)

function handleClick() {
  count.value++
}
</script>

<style scoped>
/* 样式 */
</style>
```

## 测试

### 后端测试

```bash
cd backend
cargo test
```

### 前端测试

```bash
cd frontend
npm run test
```

### 集成测试

```bash
docker-compose -f docker-compose.test.yml up
```

## 项目结构

详见 [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md)

## 发布流程

项目维护者负责发布新版本:

1. 更新版本号
2. 更新 CHANGELOG.md
3. 创建 Git Tag
4. 构建 Docker 镜像
5. 发布到 Docker Hub
6. 创建 GitHub Release

## 常见问题

### Q: 如何添加新的 API 端点?

1. 在 `backend/src/models.rs` 添加数据模型
2. 在 `backend/src/cloudflare.rs` 添加 API 调用方法
3. 在 `backend/src/handlers.rs` 添加处理函数
4. 在 `backend/src/main.rs` 注册路由
5. 在 `frontend/src/api/index.ts` 添加前端接口

### Q: 如何添加新的页面?

1. 在 `frontend/src/views/` 创建新的 Vue 组件
2. 在 `frontend/src/router/index.ts` 添加路由
3. 在 `frontend/src/views/Layout.vue` 添加菜单项

### Q: 如何调试?

**后端:**
```bash
RUST_LOG=debug cargo run
```

**前端:**
- 使用浏览器开发者工具
- 使用 Vue DevTools 扩展

## 联系方式

- GitHub Issues: 技术问题和 Bug 报告
- GitHub Discussions: 功能讨论和问答
- Email: your-email@example.com

---

**感谢你的贡献! 🎉**
