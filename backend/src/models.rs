use serde::{Deserialize, Serialize};

// Cloudflare API 凭证（仅使用 API Token，更安全）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CloudflareCredentials {
    // API Token（必需） - 用户需要创建具有所需权限的 API Token
    #[serde(alias = "apiToken")]  // 支持驼峰命名
    pub api_token: String,
}

impl CloudflareCredentials {
    // 验证凭证是否有效
    pub fn is_valid(&self) -> bool {
        // 必须提供 API Token 且不为空
        !self.api_token.is_empty()
    }
}

// Cloudflare Account 信息（从 API 自动获取）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloudflareAccount {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

// 通用请求结构
#[derive(Debug, Deserialize)]
pub struct CloudflareRequest<T> {
    pub credentials: CloudflareCredentials,
    #[serde(flatten)]
    pub data: T,
}

// Zone 相关
#[derive(Debug, Serialize, Deserialize)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub status: String,
    pub name_servers: Vec<String>,
}

// DNS 记录
#[derive(Debug, Serialize, Deserialize)]
pub struct DnsRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "zoneId")]
    pub zone_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(alias = "recordType")]
    pub record_type: String,
    pub name: String,
    pub content: String,
    #[serde(default = "default_ttl")]
    pub ttl: u32,
    #[serde(default)]
    pub proxied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u16>,
}

fn default_ttl() -> u32 {
    1
}

#[derive(Debug, Deserialize)]
pub struct GetDnsRecordsRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRecordRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "recordId")]
    pub record_id: String,
}

// 防火墙规则
#[derive(Debug, Serialize, Deserialize)]
pub struct FirewallRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub filter: FirewallFilter,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "default_false")]
    pub paused: bool,
}

fn default_false() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirewallFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub expression: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetFirewallRulesRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteFirewallRuleRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "ruleId")]
    pub rule_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateFirewallRuleRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    pub rule: FirewallRule,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFirewallRuleRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "ruleId")]
    pub rule_id: String,
    pub rule: FirewallRule,
}

// Worker 部署
#[derive(Debug, Deserialize)]
pub struct DeployWorkerRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "scriptName")]
    pub script_name: String,
    #[serde(alias = "targetUrl")]
    pub target_url: String,
    #[serde(alias = "accessDomain")]
    pub access_domain: String,
    #[serde(alias = "cacheTtl")]
    pub cache_ttl: u32,
    #[serde(alias = "cdnNode")]
    pub cdn_node: String,
}

#[derive(Debug, Deserialize)]
pub struct ListWorkersRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Worker {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetWorkerRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
    #[serde(alias = "scriptName")]
    pub script_name: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteWorkerRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
    #[serde(alias = "scriptName")]
    pub script_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadWorkerRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
    #[serde(alias = "scriptName")]
    pub script_name: String,
    #[serde(alias = "scriptContent")]
    pub script_content: String,
}

#[derive(Debug, Deserialize)]
pub struct GetWorkerRoutesRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkerRoute {
    pub id: String,
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkerRouteRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    pub pattern: String,
    #[serde(alias = "scriptName")]
    pub script_name: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteWorkerRouteRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "routeId")]
    pub route_id: String,
}

// Zone 设置
#[derive(Debug, Deserialize)]
pub struct GetZoneSettingsRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneSetting {
    pub id: String,
    pub value: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "modifiedOn")]
    pub modified_on: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateZoneSettingsRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    pub settings: Vec<UpdateSetting>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSetting {
    pub id: String,
    pub value: serde_json::Value,
}

// 自动优化
#[derive(Debug, Deserialize)]
pub struct OptimizeZoneRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    pub mode: OptimizeMode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OptimizeMode {
    Security,
    Performance,
}

// Analytics 相关
#[derive(Debug, Deserialize)]
pub struct GetAnalyticsRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "timeRange")]
    pub time_range: String, // "24h", "7d", "30d"
}

#[derive(Debug, Serialize)]
pub struct AnalyticsData {
    pub stats: AnalyticsStats,
    pub timeseries: Vec<TimeseriesPoint>,
    #[serde(rename = "statusCodes")]
    pub status_codes: Vec<StatusCodeStat>,
    pub countries: Vec<CountryStat>,
    pub content: Vec<ContentStat>,
}

#[derive(Debug, Serialize)]
pub struct AnalyticsStats {
    #[serde(rename = "totalRequests")]
    pub total_requests: u64,
    #[serde(rename = "cacheHitRate")]
    pub cache_hit_rate: f64,
    pub bandwidth: f64,
    pub threats: u64,
}

#[derive(Debug, Serialize)]
pub struct TimeseriesPoint {
    pub timestamp: String,
    pub requests: u64,
    pub cached: u64,
    pub uncached: u64,
}

#[derive(Debug, Serialize)]
pub struct StatusCodeStat {
    pub code: String,
    pub description: String,
    pub count: u64,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct CountryStat {
    pub rank: u32,
    pub country: String,
    pub requests: u64,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct ContentStat {
    pub rank: u32,
    pub url: String,
    pub requests: u64,
    pub bandwidth: String,
}

// 缓存清除相关
#[derive(Debug, Deserialize)]
pub struct PurgeCacheRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "purgeEverything")]
    pub purge_everything: Option<bool>,
    pub files: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct PurgeCacheResponse {
    pub id: String,
}

// 页面规则相关
#[derive(Debug, Deserialize)]
pub struct GetPageRulesRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub targets: Vec<PageRuleTarget>,
    pub actions: Vec<PageRuleAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageRuleTarget {
    pub target: String,  // "url"
    pub constraint: PageRuleConstraint,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageRuleConstraint {
    pub operator: String,  // "matches"
    pub value: String,  // URL pattern
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageRuleAction {
    pub id: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct CreatePageRuleRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    pub rule: PageRule,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePageRuleRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "ruleId")]
    pub rule_id: String,
    pub rule: PageRule,
}

#[derive(Debug, Deserialize)]
pub struct DeletePageRuleRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "ruleId")]
    pub rule_id: String,
}

// SSL 证书相关
#[derive(Debug, Deserialize)]
pub struct GetSslCertificatesRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SslCertificate {
    pub id: String,
    #[serde(rename = "type")]
    pub cert_type: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<CertificateDetail>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateDetail {
    pub id: String,
    pub status: String,
    pub issuer: String,
    pub signature: String,
    pub serial_number: String,
    pub expires_on: String,
    pub uploaded_on: String,
}

// 自定义 SSL 证书管理
#[derive(Debug, Deserialize)]
pub struct UploadCustomCertificateRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    pub certificate: String,  // PEM format
    pub private_key: String,  // PEM format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_method: Option<String>,  // ubiquitous, optimal, force
}

#[derive(Debug, Deserialize)]
pub struct GetCustomCertificatesRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomCertificate {
    pub id: String,
    pub status: String,
    pub issuer: String,
    pub signature: String,
    pub expires_on: String,
    pub uploaded_on: String,
    pub modified_on: String,
    pub hosts: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_method: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteCustomCertificateRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "certificateId")]
    pub certificate_id: String,
}

// WAF 规则管理
#[derive(Debug, Deserialize)]
pub struct GetWafPackagesRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WafPackage {
    pub id: String,
    pub name: String,
    pub description: String,
    pub detection_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_mode: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetWafRulesRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "packageId")]
    pub package_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WafRule {
    pub id: String,
    pub description: String,
    pub priority: String,
    pub group: WafRuleGroup,
    pub mode: String,
    pub allowed_modes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WafRuleGroup {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWafRuleRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "packageId")]
    pub package_id: String,
    #[serde(alias = "ruleId")]
    pub rule_id: String,
    pub mode: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWafPackageRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "packageId")]
    pub package_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_mode: Option<String>,
}

// Rate Limiting
#[derive(Debug, Deserialize)]
pub struct GetRateLimitsRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimit {
    pub id: String,
    pub disabled: bool,
    pub description: String,
    pub match_request: MatchRequest,
    pub threshold: u32,
    pub period: u32,
    pub action: RateLimitAction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchRequest {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub methods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimitAction {
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<RateLimitResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimitResponse {
    pub content_type: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRateLimitRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    pub disabled: bool,
    pub description: String,
    pub match_request: MatchRequest,
    pub threshold: u32,
    pub period: u32,
    pub action: RateLimitAction,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRateLimitRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "rateLimitId")]
    pub rate_limit_id: String,
    pub disabled: bool,
    pub description: String,
    pub match_request: MatchRequest,
    pub threshold: u32,
    pub period: u32,
    pub action: RateLimitAction,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRateLimitRequest {
    #[serde(alias = "zoneId")]
    pub zone_id: String,
    #[serde(alias = "rateLimitId")]
    pub rate_limit_id: String,
}

// Workers KV 相关
#[derive(Debug, Deserialize)]
pub struct ListKVNamespacesRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateKVNamespaceRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteKVNamespaceRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
    #[serde(alias = "namespaceId")]
    pub namespace_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ListKVKeysRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
    #[serde(alias = "namespaceId")]
    pub namespace_id: String,
    pub prefix: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReadKVValueRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
    #[serde(alias = "namespaceId")]
    pub namespace_id: String,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct WriteKVValueRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
    #[serde(alias = "namespaceId")]
    pub namespace_id: String,
    pub key: String,
    pub value: String,
    #[serde(alias = "expirationTtl")]
    pub expiration_ttl: Option<u64>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteKVKeyRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
    #[serde(alias = "namespaceId")]
    pub namespace_id: String,
    pub key: String,
}

// D1 Database 相关
#[derive(Debug, Deserialize)]
pub struct ListD1DatabasesRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateD1DatabaseRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteD1DatabaseRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
    #[serde(alias = "databaseId")]
    pub database_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteD1QueryRequest {
    #[serde(alias = "accountId")]
    pub account_id: String,
    #[serde(alias = "databaseId")]
    pub database_id: String,
    pub query: String,
}

// API 响应
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> ApiResponse<T> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}
