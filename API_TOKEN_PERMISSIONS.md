# Cloudflare API Token 权限要求

本文档列出了 Cloudflare 管理平台所有功能所需的完整权限。

## 必需权限清单

### Account 级别权限

| 资源 | 权限级别 | 功能模块 | 说明 |
|-----|---------|---------|-----|
| **Account Settings** | **Read** | 账户管理 | 获取账户列表和信息 |
| **Workers Scripts** | **Edit** | Workers 管理 | 部署、查看、删除 Worker 脚本 |
| **Workers KV Storage** | **Edit** | Workers KV | 创建/删除 Namespace，读写键值对 |
| **Workers Routes** | **Edit** | Workers 路由 | 创建、删除 Worker 路由绑定 |
| **D1** | **Edit** | D1 数据库 | 创建/删除数据库，执行所有 SQL 操作（包含 Read + Write） |

### Zone 级别权限

| 资源 | 权限级别 | 功能模块 | 说明 |
|-----|---------|---------|-----|
| **Zone** | **Read** | 域名管理 | 查看域名列表和详情 |
| **Zone Settings** | **Edit** | 域名设置、自动优化 | 修改域名配置，执行优化操作 |
| **DNS** | **Edit** | DNS 记录管理 | 创建、更新、删除 DNS 记录 |
| **Cache Purge** | **Purge** | 缓存管理 | 清除缓存 |
| **SSL and Certificates** | **Edit** | SSL 证书管理 | 上传、删除自定义证书 |
| **Page Rules** | **Edit** | 页面规则 | 创建、更新、删除页面规则 |
| **Firewall Services** | **Edit** | 防火墙规则 | 创建、更新、删除防火墙规则 |
| **WAF** | **Edit** | WAF 管理 | 修改 WAF 规则和包配置 |
| **Zone Analytics** | **Read** | 统计分析 | 查看流量和性能数据 |

## 功能详细说明

### 1. 账户管理 (Account Management)
- **功能**: 获取 Cloudflare 账户信息
- **API 调用**: `GET /accounts`
- **所需权限**: Account Settings - Read

### 2. 域名管理 (Zones)
- **功能**: 查看和管理域名列表
- **API 调用**: `GET /zones`
- **所需权限**: Zone - Read

### 3. DNS 记录管理 (DNS Records)
- **功能**:
  - 查看 DNS 记录
  - 创建新的 DNS 记录
  - 更新现有记录
  - 删除记录
- **API 调用**:
  - `GET /zones/{zone_id}/dns_records`
  - `POST /zones/{zone_id}/dns_records`
  - `PUT /zones/{zone_id}/dns_records/{record_id}`
  - `DELETE /zones/{zone_id}/dns_records/{record_id}`
- **所需权限**: DNS - Edit

### 4. Workers 管理
- **功能**:
  - 部署 Worker 脚本
  - 查看 Worker 列表
  - 获取 Worker 代码
  - 删除 Worker
- **API 调用**:
  - `PUT /accounts/{account_id}/workers/scripts/{script_name}`
  - `GET /accounts/{account_id}/workers/scripts`
  - `GET /accounts/{account_id}/workers/scripts/{script_name}`
  - `DELETE /accounts/{account_id}/workers/scripts/{script_name}`
- **所需权限**: Workers Scripts - Edit

### 5. Workers 路由 (Worker Routes)
- **功能**:
  - 查看 Worker 路由
  - 创建路由绑定
  - 删除路由绑定
- **API 调用**:
  - `GET /zones/{zone_id}/workers/routes`
  - `POST /zones/{zone_id}/workers/routes`
  - `DELETE /zones/{zone_id}/workers/routes/{route_id}`
- **所需权限**: Workers Routes - Edit

### 6. Workers KV 存储
- **功能**:
  - 列出 KV Namespace
  - 创建/删除 Namespace
  - 读写键值对
- **API 调用**:
  - `GET /accounts/{account_id}/storage/kv/namespaces`
  - `POST /accounts/{account_id}/storage/kv/namespaces`
  - `DELETE /accounts/{account_id}/storage/kv/namespaces/{namespace_id}`
  - `GET /accounts/{account_id}/storage/kv/namespaces/{namespace_id}/keys`
  - `GET /accounts/{account_id}/storage/kv/namespaces/{namespace_id}/values/{key}`
  - `PUT /accounts/{account_id}/storage/kv/namespaces/{namespace_id}/values/{key}`
  - `DELETE /accounts/{account_id}/storage/kv/namespaces/{namespace_id}/values/{key}`
- **所需权限**: Workers KV Storage - Edit

### 7. D1 数据库
- **功能**:
  - 列出数据库
  - 创建/删除数据库
  - 执行 SQL 查询（包括 SELECT, INSERT, UPDATE, DELETE 等所有操作）
- **API 调用**:
  - `GET /accounts/{account_id}/d1/database`
  - `POST /accounts/{account_id}/d1/database`
  - `DELETE /accounts/{account_id}/d1/database/{database_id}`
  - `POST /accounts/{account_id}/d1/database/{database_id}/query`
- **所需权限**: D1 - Edit（必须包含 Read + Write 权限，以支持所有类型的 SQL 操作）
- **注意事项**:
  - SELECT 查询需要 D1 Read 权限
  - INSERT, UPDATE, DELETE 等写操作需要 D1 Write 权限
  - 使用 Edit 权限可同时满足两者

### 8. 域名设置 (Zone Settings)
- **功能**:
  - 查看域名配置
  - 修改安全、性能等设置
  - 自动优化配置
- **API 调用**:
  - `GET /zones/{zone_id}/settings`
  - `PATCH /zones/{zone_id}/settings`
- **所需权限**: Zone Settings - Edit

### 9. 防火墙规则 (Firewall Rules)
- **功能**:
  - 查看防火墙规则
  - 创建、更新、删除规则
- **API 调用**:
  - `GET /zones/{zone_id}/firewall/rules`
  - `POST /zones/{zone_id}/firewall/rules`
  - `PUT /zones/{zone_id}/firewall/rules/{rule_id}`
  - `DELETE /zones/{zone_id}/firewall/rules/{rule_id}`
- **所需权限**: Firewall Services - Edit

### 10. 缓存管理 (Cache Purge)
- **功能**: 清除缓存（全部或指定文件/标签）
- **API 调用**: `POST /zones/{zone_id}/purge_cache`
- **所需权限**: Cache Purge - Purge

### 11. SSL 证书管理
- **功能**:
  - 查看 SSL 证书
  - 上传自定义证书
  - 删除自定义证书
- **API 调用**:
  - `GET /zones/{zone_id}/ssl/certificate_packs`
  - `GET /zones/{zone_id}/custom_certificates`
  - `POST /zones/{zone_id}/custom_certificates`
  - `DELETE /zones/{zone_id}/custom_certificates/{certificate_id}`
- **所需权限**: SSL and Certificates - Edit

### 12. 页面规则 (Page Rules)
- **功能**: 创建、更新、删除页面规则
- **API 调用**:
  - `GET /zones/{zone_id}/pagerules`
  - `POST /zones/{zone_id}/pagerules`
  - `PATCH /zones/{zone_id}/pagerules/{rule_id}`
  - `DELETE /zones/{zone_id}/pagerules/{rule_id}`
- **所需权限**: Page Rules - Edit

### 13. WAF 管理
- **功能**:
  - 查看 WAF 包和规则
  - 修改规则模式和包配置
- **API 调用**:
  - `GET /zones/{zone_id}/firewall/waf/packages`
  - `GET /zones/{zone_id}/firewall/waf/packages/{package_id}/rules`
  - `PATCH /zones/{zone_id}/firewall/waf/packages/{package_id}/rules/{rule_id}`
  - `PATCH /zones/{zone_id}/firewall/waf/packages/{package_id}`
- **所需权限**: WAF - Edit

### 14. 速率限制 (Rate Limiting)
- **功能**: 创建、更新、删除速率限制规则
- **API 调用**:
  - `GET /zones/{zone_id}/rate_limits`
  - `POST /zones/{zone_id}/rate_limits`
  - `PUT /zones/{zone_id}/rate_limits/{rate_limit_id}`
  - `DELETE /zones/{zone_id}/rate_limits/{rate_limit_id}`
- **所需权限**: Firewall Services - Edit

### 15. 统计分析 (Analytics)
- **功能**: 查看流量、缓存、性能等统计数据
- **API 调用**: GraphQL Analytics API
- **所需权限**: Zone Analytics - Read

## 创建 API Token 步骤

1. 登录 Cloudflare Dashboard
2. 进入 **My Profile** → **API Tokens**
3. 点击 **Create Token**
4. 选择 **Custom Token** 或从模板开始
5. 配置权限（参考上面的表格）
6. 选择适用的账户和域名（建议选择所有账户和域名）
7. （可选）设置 IP 地址限制和 TTL
8. 创建并保存 Token

## 最小权限配置（推荐）

如果您只需要基本功能，可以使用以下最小权限集：

**Account 权限：**
- Account Settings - Read
- Workers Scripts - Edit
- Workers KV Storage - Edit
- D1 - Edit

**Zone 权限：**
- Zone - Read
- DNS - Edit
- Cache Purge - Purge

## 完整功能配置

要使用本平台的所有功能，请配置以下权限：

**Account 权限：**
- Account Settings - Read
- Workers Scripts - Edit
- Workers Routes - Edit
- Workers KV Storage - Edit
- D1 - Edit

**Zone 权限：**
- Zone - Read
- Zone Settings - Edit
- DNS - Edit
- Cache Purge - Purge
- SSL and Certificates - Edit
- Page Rules - Edit
- Firewall Services - Edit
- WAF - Edit
- Zone Analytics - Read

## 权限验证

您可以通过以下方式验证 Token 权限：

```bash
curl -X GET "https://api.cloudflare.com/client/v4/user/tokens/verify" \
     -H "Authorization: Bearer YOUR_API_TOKEN" \
     -H "Content-Type: application/json"
```

## 常见问题

### Q: 为什么需要 Edit 权限而不是 Read？
A: 大多数管理功能需要创建、更新或删除资源，因此需要 Edit 权限。如果您只需要查看信息，可以降级为 Read 权限。

### Q: Workers Routes 权限在哪里？
A: Workers Routes 是一个独立的权限项，与 Workers Scripts 分开。确保两者都配置为 Edit。

### Q: 为什么 D1 查询需要 Edit 权限？
A: 因为系统支持执行所有类型的 SQL 语句，包括 INSERT、UPDATE、DELETE，这些操作需要写入权限。Edit 权限包含了 Read 和 Write，确保所有查询都能正常执行。

**重要提示**：在 Cloudflare Dashboard 中配置时：
- 对于 D1，选择 **Edit** 权限（这会自动包含 Read 和 Write）
- 如果只选择 Read，则只能执行 SELECT 查询
- 如果只选择 Write，可能无法读取数据

### Q: 可以使用 Global API Key 吗？
A: 不推荐。本系统专为 API Token 设计，更安全且支持细粒度权限控制。

## 安全建议

1. **使用最小权限原则**：仅授予所需的最小权限
2. **定期轮换 Token**：建议每 90 天更换一次
3. **设置 IP 限制**：限制 Token 只能从特定 IP 使用
4. **监控使用情况**：定期检查 Token 的使用日志
5. **妥善保管**：不要将 Token 提交到版本控制系统

## 故障排查

如果遇到权限错误（Authentication error, code 10000），请：

1. 确认 Token 包含所需的所有权限
2. 检查 Token 是否已过期
3. 验证账户和域名范围设置
4. 使用上面的验证命令测试 Token
