use serde::{Deserialize, Serialize};

// Cloudflare API 凭证
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CloudflareCredentials {
    // 推荐使用 API Token（更安全）
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "apiToken")]  // 支持驼峰命名
    pub api_token: Option<String>,

    // 旧式认证方式（仅作向后兼容）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "apiKey")]  // 支持驼峰命名
    pub api_key: Option<String>,
}

impl CloudflareCredentials {
    // 验证凭证是否有效
    pub fn is_valid(&self) -> bool {
        // 必须提供 API Token 或者 Email + API Key
        self.api_token.is_some() || (self.email.is_some() && self.api_key.is_some())
    }
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
    #[serde(alias = "zoneId")]
    pub zone_id: String,
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
