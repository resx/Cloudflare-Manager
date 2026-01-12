use crate::models::*;
use reqwest::{header, Client};
use serde_json::json;

const CLOUDFLARE_API_BASE: &str = "https://api.cloudflare.com/client/v4";

pub struct CloudflareClient {
    client: Client,
    credentials: CloudflareCredentials,
}

impl CloudflareClient {
    pub fn new(credentials: &CloudflareCredentials) -> Result<Self, String> {
        // 验证凭证 - 必须提供 API Token
        if !credentials.is_valid() {
            return Err("Invalid credentials: API Token is required".to_string());
        }

        Ok(CloudflareClient {
            client: Client::new(),
            credentials: credentials.clone(),
        })
    }

    // 所有 API 使用 API Token 认证（Bearer Token）
    fn get_headers(&self) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();

        // 使用 Bearer Token 认证
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", self.credentials.api_token.trim()))
                .unwrap_or_else(|_| header::HeaderValue::from_static(""))
        );

        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
        headers
    }

    // GraphQL API 也使用相同的 Bearer Token
    fn get_graphql_headers(&self) -> header::HeaderMap {
        self.get_headers()
    }

    // 获取用户的 Cloudflare 账户列表（自动获取 Account ID）
    pub async fn get_accounts(&self) -> Result<Vec<CloudflareAccount>, String> {
        let url = format!("{}/accounts", CLOUDFLARE_API_BASE);

        log::info!("Fetching Cloudflare accounts for user");

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = response.status();
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            let errors = json["errors"].as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|err| err["message"].as_str())
                .unwrap_or("Unknown error");
            return Err(format!("API error ({}): {}", status, errors));
        }

        let accounts: Vec<CloudflareAccount> = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse accounts: {}", e))?;

        log::info!("Successfully fetched {} Cloudflare accounts", accounts.len());
        Ok(accounts)
    }

    // 获取所有 Zone
    pub async fn get_zones(&self) -> Result<Vec<Zone>, String> {
        let url = format!("{}/zones", CLOUDFLARE_API_BASE);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let zones: Vec<Zone> = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse zones: {}", e))?;

        Ok(zones)
    }

    // 获取 DNS 记录
    pub async fn get_dns_records(&self, zone_id: &str) -> Result<Vec<DnsRecord>, String> {
        let url = format!("{}/zones/{}/dns_records", CLOUDFLARE_API_BASE, zone_id);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let records: Vec<DnsRecord> = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse DNS records: {}", e))?;

        Ok(records)
    }

    // 创建 DNS 记录
    pub async fn create_dns_record(&self, record: &DnsRecord) -> Result<DnsRecord, String> {
        let zone_id = record.zone_id.as_ref().ok_or("Zone ID is required for creating DNS record")?;
        let url = format!("{}/zones/{}/dns_records", CLOUDFLARE_API_BASE, zone_id);

        let response = self.client
            .post(&url)
            .headers(self.get_headers())
            .json(record)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let created_record: DnsRecord = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse DNS record: {}", e))?;

        Ok(created_record)
    }

    // 更新 DNS 记录
    pub async fn update_dns_record(&self, record: &DnsRecord) -> Result<DnsRecord, String> {
        let record_id = record.id.as_ref().ok_or("Record ID is required")?;
        let zone_id = record.zone_id.as_ref().ok_or("Zone ID is required for updating DNS record")?;
        let url = format!("{}/zones/{}/dns_records/{}", CLOUDFLARE_API_BASE, zone_id, record_id);

        let response = self.client
            .put(&url)
            .headers(self.get_headers())
            .json(record)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let updated_record: DnsRecord = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse DNS record: {}", e))?;

        Ok(updated_record)
    }

    // 删除 DNS 记录
    pub async fn delete_dns_record(&self, zone_id: &str, record_id: &str) -> Result<String, String> {
        let url = format!("{}/zones/{}/dns_records/{}", CLOUDFLARE_API_BASE, zone_id, record_id);

        let response = self.client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        Ok(record_id.to_string())
    }

    // 部署 Worker
    pub async fn deploy_worker(&self, request: &DeployWorkerRequest) -> Result<String, String> {
        // 生成 Worker 代码
        let worker_script = self.generate_worker_script(request);

        // 上传 Worker 脚本
        let url = format!("{}/accounts/workers/scripts/{}", CLOUDFLARE_API_BASE, request.script_name);

        let response = self.client
            .put(&url)
            .headers(self.get_headers())
            .header(header::CONTENT_TYPE, "application/javascript")
            .body(worker_script)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        Ok(format!("Worker {} deployed successfully", request.script_name))
    }

    fn generate_worker_script(&self, request: &DeployWorkerRequest) -> String {
        format!(r#"
addEventListener('fetch', event => {{
    event.respondWith(handleRequest(event.request))
}})

async function handleRequest(request) {{
    const targetUrl = '{target_url}';
    const cdnNode = '{cdn_node}';
    const cacheTtl = {cache_ttl};

    const url = new URL(request.url);
    url.hostname = new URL(targetUrl).hostname;

    const modifiedRequest = new Request(url.toString(), {{
        method: request.method,
        headers: request.headers,
        body: request.body,
    }});

    const cacheKey = new Request(url.toString(), modifiedRequest);
    const cache = caches.default;

    let response = await cache.match(cacheKey);

    if (!response) {{
        response = await fetch(modifiedRequest);

        if (response.ok && cacheTtl > 0) {{
            const newHeaders = new Headers(response.headers);
            newHeaders.set('Cache-Control', `public, max-age=${{cacheTtl}}`);

            response = new Response(response.body, {{
                status: response.status,
                statusText: response.statusText,
                headers: newHeaders
            }});

            event.waitUntil(cache.put(cacheKey, response.clone()));
        }}
    }}

    return response;
}}
"#,
            target_url = request.target_url,
            cdn_node = request.cdn_node,
            cache_ttl = request.cache_ttl
        )
    }

    // 获取 Worker 列表
    pub async fn list_workers(&self, account_id: &str) -> Result<Vec<Worker>, String> {
        let url = format!("{}/accounts/{}/workers/scripts", CLOUDFLARE_API_BASE, account_id);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let workers: Vec<Worker> = json["result"]
            .as_array()
            .ok_or("Invalid result format")?
            .iter()
            .map(|v| serde_json::from_value(v.clone()).unwrap_or(Worker {
                id: v["id"].as_str().unwrap_or("").to_string(),
                etag: v["etag"].as_str().map(|s| s.to_string()),
                created_on: v["created_on"].as_str().map(|s| s.to_string()),
                modified_on: v["modified_on"].as_str().map(|s| s.to_string()),
            }))
            .collect();

        Ok(workers)
    }

    // 获取单个 Worker
    pub async fn get_worker(&self, account_id: &str, script_name: &str) -> Result<String, String> {
        let url = format!("{}/accounts/{}/workers/scripts/{}", CLOUDFLARE_API_BASE, account_id, script_name);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let script = response
            .text()
            .await
            .map_err(|e| format!("Failed to read script: {}", e))?;

        Ok(script)
    }

    // 删除 Worker
    pub async fn delete_worker(&self, account_id: &str, script_name: &str) -> Result<String, String> {
        let url = format!("{}/accounts/{}/workers/scripts/{}", CLOUDFLARE_API_BASE, account_id, script_name);

        let response = self.client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        Ok(format!("Worker {} deleted successfully", script_name))
    }

    // 上传/更新 Worker
    pub async fn upload_worker(&self, account_id: &str, script_name: &str, script_content: &str) -> Result<serde_json::Value, String> {
        let url = format!("{}/accounts/{}/workers/scripts/{}", CLOUDFLARE_API_BASE, account_id, script_name);

        log::info!("Uploading Worker script {} to account {}", script_name, account_id);

        let metadata = serde_json::json!({
            "main_module": "worker.js",
            "compatibility_date": "2024-01-01"
        });

        let form = reqwest::multipart::Form::new()
            .text("metadata", metadata.to_string())
            .part(
                "worker.js",
                reqwest::multipart::Part::text(script_content.to_string())
                    .mime_str("application/javascript+module")
                    .map_err(|e| format!("Failed to set MIME type: {}", e))?
            );

        let response = self.client
            .put(&url)
            .headers(self.get_headers())
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = response.status();
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            let errors = json["errors"].as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|err| err["message"].as_str())
                .unwrap_or("Unknown error");
            log::error!("Worker upload failed ({}): {}", status, errors);
            return Err(format!("API error ({}): {}", status, errors));
        }

        log::info!("Successfully uploaded Worker script {}", script_name);
        Ok(json["result"].clone())
    }

    // 获取 Worker 路由列表
    pub async fn get_worker_routes(&self, zone_id: &str) -> Result<Vec<WorkerRoute>, String> {
        let url = format!("{}/zones/{}/workers/routes", CLOUDFLARE_API_BASE, zone_id);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let routes: Vec<WorkerRoute> = json["result"]
            .as_array()
            .ok_or("Invalid result format")?
            .iter()
            .map(|v| serde_json::from_value(v.clone()).unwrap_or(WorkerRoute {
                id: v["id"].as_str().unwrap_or("").to_string(),
                pattern: v["pattern"].as_str().unwrap_or("").to_string(),
                script: v["script"].as_str().map(|s| s.to_string()),
            }))
            .collect();

        Ok(routes)
    }

    // 创建 Worker 路由
    pub async fn create_worker_route(&self, zone_id: &str, pattern: &str, script_name: &str) -> Result<WorkerRoute, String> {
        let url = format!("{}/zones/{}/workers/routes", CLOUDFLARE_API_BASE, zone_id);

        let body = serde_json::json!({
            "pattern": pattern,
            "script": script_name
        });

        let response = self.client
            .post(&url)
            .headers(self.get_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let route: WorkerRoute = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse route: {}", e))?;

        Ok(route)
    }

    // 删除 Worker 路由
    pub async fn delete_worker_route(&self, zone_id: &str, route_id: &str) -> Result<String, String> {
        let url = format!("{}/zones/{}/workers/routes/{}", CLOUDFLARE_API_BASE, zone_id, route_id);

        let response = self.client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        Ok("Route deleted successfully".to_string())
    }

    // 获取 Zone 设置
    pub async fn get_zone_settings(&self, zone_id: &str) -> Result<Vec<ZoneSetting>, String> {
        let url = format!("{}/zones/{}/settings", CLOUDFLARE_API_BASE, zone_id);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let settings: Vec<ZoneSetting> = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse zone settings: {}", e))?;

        Ok(settings)
    }

    // 更新 Zone 设置
    pub async fn update_zone_settings(&self, zone_id: &str, settings: &[UpdateSetting]) -> Result<String, String> {
        for setting in settings {
            let url = format!("{}/zones/{}/settings/{}", CLOUDFLARE_API_BASE, zone_id, setting.id);

            let body = json!({ "value": setting.value });

            let response = self.client
                .patch(&url)
                .headers(self.get_headers())
                .json(&body)
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;

            let json: serde_json::Value = response
                .json()
                .await
                .map_err(|e| format!("JSON parse failed: {}", e))?;

            if !json["success"].as_bool().unwrap_or(false) {
                return Err(format!("API error for setting {}: {:?}", setting.id, json["errors"]));
            }
        }

        Ok("Settings updated successfully".to_string())
    }

    // 自动优化 Zone
    pub async fn optimize_zone(&self, zone_id: &str, mode: &OptimizeMode) -> Result<String, String> {
        let settings = match mode {
            OptimizeMode::Security => self.get_security_settings(),
            OptimizeMode::Performance => self.get_performance_settings(),
        };

        self.update_zone_settings(zone_id, &settings).await
    }

    fn get_security_settings(&self) -> Vec<UpdateSetting> {
        vec![
            UpdateSetting { id: "security_level".to_string(), value: json!("high") },
            UpdateSetting { id: "ssl".to_string(), value: json!("strict") },
            UpdateSetting { id: "always_use_https".to_string(), value: json!("on") },
            UpdateSetting { id: "automatic_https_rewrites".to_string(), value: json!("on") },
            UpdateSetting { id: "min_tls_version".to_string(), value: json!("1.2") },
            UpdateSetting { id: "tls_1_3".to_string(), value: json!("on") },
            UpdateSetting { id: "opportunistic_encryption".to_string(), value: json!("on") },
            UpdateSetting { id: "browser_check".to_string(), value: json!("on") },
            UpdateSetting { id: "challenge_ttl".to_string(), value: json!(1800) },
            UpdateSetting { id: "hotlink_protection".to_string(), value: json!("on") },
            UpdateSetting { id: "cache_level".to_string(), value: json!("basic") },
            UpdateSetting { id: "browser_cache_ttl".to_string(), value: json!(14400) },
        ]
    }

    fn get_performance_settings(&self) -> Vec<UpdateSetting> {
        vec![
            UpdateSetting { id: "cache_level".to_string(), value: json!("aggressive") },
            UpdateSetting { id: "browser_cache_ttl".to_string(), value: json!(31536000) },
            UpdateSetting { id: "minify".to_string(), value: json!({"css": "on", "html": "on", "js": "on"}) },
            UpdateSetting { id: "brotli".to_string(), value: json!("on") },
            UpdateSetting { id: "http3".to_string(), value: json!("on") },
            UpdateSetting { id: "early_hints".to_string(), value: json!("on") },
            UpdateSetting { id: "polish".to_string(), value: json!("lossless") },
            UpdateSetting { id: "security_level".to_string(), value: json!("low") },
            UpdateSetting { id: "ssl".to_string(), value: json!("flexible") },
            UpdateSetting { id: "rocket_loader".to_string(), value: json!("on") },
            UpdateSetting { id: "0rtt".to_string(), value: json!("on") },
        ]
    }

    // 获取防火墙规则
    pub async fn get_firewall_rules(&self, zone_id: &str) -> Result<Vec<FirewallRule>, String> {
        let url = format!("{}/zones/{}/firewall/rules", CLOUDFLARE_API_BASE, zone_id);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let rules: Vec<FirewallRule> = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse firewall rules: {}", e))?;

        Ok(rules)
    }

    // 创建防火墙规则
    pub async fn create_firewall_rule(&self, zone_id: &str, rule: &FirewallRule) -> Result<FirewallRule, String> {
        // 先创建 filter
        let filter_url = format!("{}/zones/{}/filters", CLOUDFLARE_API_BASE, zone_id);

        let filter_body = json!([{
            "expression": rule.filter.expression,
            "description": rule.filter.description
        }]);

        let filter_response = self.client
            .post(&filter_url)
            .headers(self.get_headers())
            .json(&filter_body)
            .send()
            .await
            .map_err(|e| format!("Failed to create filter: {}", e))?;

        let filter_json: serde_json::Value = filter_response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !filter_json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", filter_json["errors"]));
        }

        let filter_id = filter_json["result"][0]["id"]
            .as_str()
            .ok_or("Failed to get filter ID")?
            .to_string();

        // 然后创建规则
        let rule_url = format!("{}/zones/{}/firewall/rules", CLOUDFLARE_API_BASE, zone_id);

        let rule_body = json!([{
            "filter": { "id": filter_id },
            "action": rule.action,
            "description": rule.description,
            "paused": rule.paused
        }]);

        let rule_response = self.client
            .post(&rule_url)
            .headers(self.get_headers())
            .json(&rule_body)
            .send()
            .await
            .map_err(|e| format!("Failed to create rule: {}", e))?;

        let rule_json: serde_json::Value = rule_response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !rule_json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", rule_json["errors"]));
        }

        let created_rule: FirewallRule = serde_json::from_value(rule_json["result"][0].clone())
            .map_err(|e| format!("Failed to parse rule: {}", e))?;

        Ok(created_rule)
    }

    // 更新防火墙规则
    pub async fn update_firewall_rule(&self, zone_id: &str, rule_id: &str, rule: &FirewallRule) -> Result<FirewallRule, String> {
        let url = format!("{}/zones/{}/firewall/rules/{}", CLOUDFLARE_API_BASE, zone_id, rule_id);

        let body = json!({
            "action": rule.action,
            "description": rule.description,
            "paused": rule.paused
        });

        let response = self.client
            .put(&url)
            .headers(self.get_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let updated_rule: FirewallRule = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse rule: {}", e))?;

        Ok(updated_rule)
    }

    // 删除防火墙规则
    pub async fn delete_firewall_rule(&self, zone_id: &str, rule_id: &str) -> Result<String, String> {
        let url = format!("{}/zones/{}/firewall/rules/{}", CLOUDFLARE_API_BASE, zone_id, rule_id);

        let response = self.client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        Ok(rule_id.to_string())
    }

    // 获取 Analytics 数据
    pub async fn get_analytics(&self, zone_id: &str, time_range: &str) -> Result<AnalyticsData, String> {
        use chrono::{Duration, Utc};

        // 计算时间范围
        let now = Utc::now();
        let (since, interval) = match time_range {
            "24h" => (now - Duration::hours(24), "httpRequests1hGroups"),
            "7d" => (now - Duration::days(7), "httpRequests1dGroups"),
            "30d" => (now - Duration::days(30), "httpRequests1dGroups"),
            _ => (now - Duration::hours(24), "httpRequests1hGroups"),
        };

        let since_str = since.format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let until_str = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();

        // 使用 Cloudflare GraphQL API
        let url = format!("{}/graphql", CLOUDFLARE_API_BASE);

        // 构建 GraphQL 查询 - 使用更简单和更可靠的查询
        let query_string = if interval == "httpRequests1hGroups" {
            format!(r#"
                query {{
                    viewer {{
                        zones(filter: {{zoneTag: "{}"}}) {{
                            httpRequests1hGroups(
                                limit: 168
                                filter: {{
                                    datetime_geq: "{}"
                                    datetime_leq: "{}"
                                }}
                            ) {{
                                dimensions {{
                                    datetime
                                }}
                                sum {{
                                    requests
                                    cachedRequests
                                    bytes
                                    threats
                                }}
                            }}
                        }}
                    }}
                }}
            "#, zone_id, since_str, until_str)
        } else {
            format!(r#"
                query {{
                    viewer {{
                        zones(filter: {{zoneTag: "{}"}}) {{
                            httpRequests1dGroups(
                                limit: 31
                                filter: {{
                                    date_geq: "{}"
                                    date_leq: "{}"
                                }}
                            ) {{
                                dimensions {{
                                    date
                                }}
                                sum {{
                                    requests
                                    cachedRequests
                                    bytes
                                    threats
                                }}
                            }}
                        }}
                    }}
                }}
            "#, zone_id, since.format("%Y-%m-%d"), now.format("%Y-%m-%d"))
        };

        let graphql_query = json!({
            "query": query_string
        });

        log::info!("Sending GraphQL query for zone {} with time range {}", zone_id, time_range);
        log::debug!("GraphQL query: {}", serde_json::to_string_pretty(&graphql_query).unwrap_or_default());

        // GraphQL API 使用专用的认证头（API Token 优先）
        let response = self.client
            .post(&url)
            .headers(self.get_graphql_headers())
            .json(&graphql_query)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        log::info!("GraphQL response received. Has data: {}, Has errors: {}",
            json.get("data").is_some(),
            json.get("errors").map(|e| !e.is_null()).unwrap_or(false));

        // 打印响应的主要结构（用于调试）
        if let Some(data) = json.get("data") {
            if data.is_null() {
                log::warn!("GraphQL data field is null");
            } else {
                log::info!("GraphQL data structure: viewer={}, zones={}",
                    data.get("viewer").is_some(),
                    data.get("viewer").and_then(|v| v.get("zones")).is_some());
            }
        }

        // 检查 GraphQL 错误
        if let Some(errors) = json.get("errors") {
            if !errors.is_null() {
                log::error!("GraphQL errors: {:?}", errors);
                return Err(format!("GraphQL API 错误: {:?}", errors));
            }
        }

        // 检查是否有 data
        let data_field = json.get("data");
        if data_field.is_none() || data_field.unwrap().is_null() {
            log::error!("No data in GraphQL response. Full response: {:?}", json);
            return Err("GraphQL 未返回数据。可能原因：\n1. API Token 权限不足（需要 Zone.Analytics Read 权限）\n2. Zone ID 不正确\n3. 该域名可能没有足够的历史数据".to_string());
        }

        // 解析 GraphQL 响应
        let groups_key = if interval == "httpRequests1hGroups" { "httpRequests1hGroups" } else { "httpRequests1dGroups" };
        let time_key = if interval == "httpRequests1hGroups" { "datetime" } else { "date" };

        let data = json.get("data")
            .and_then(|d| d.get("viewer"))
            .and_then(|v| v.get("zones"))
            .and_then(|z| z.as_array())
            .and_then(|arr| arr.get(0))
            .and_then(|zone| zone.get(groups_key))
            .and_then(|g| g.as_array())
            .ok_or_else(|| "Failed to parse GraphQL response structure".to_string())?;

        log::info!("Successfully parsed GraphQL response with {} data points", data.len());

        // 计算总计
        let mut total_requests: u64 = 0;
        let mut total_cached: u64 = 0;
        let mut total_bytes: u64 = 0;
        let mut total_threats: u64 = 0;
        let mut timeseries_data = Vec::new();

        for group in data {
            let sum = &group["sum"];
            let requests = sum["requests"].as_u64().unwrap_or(0);
            let cached = sum["cachedRequests"].as_u64().unwrap_or(0);
            let bytes = sum["bytes"].as_u64().unwrap_or(0);
            let threats = sum["threats"].as_u64().unwrap_or(0);

            total_requests += requests;
            total_cached += cached;
            total_bytes += bytes;
            total_threats += threats;

            let timestamp = group["dimensions"][time_key].as_str().unwrap_or("").to_string();

            timeseries_data.push(TimeseriesPoint {
                timestamp,
                requests,
                cached,
                uncached: requests.saturating_sub(cached),
            });
        }

        let cache_hit_rate = if total_requests > 0 {
            (total_cached as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };
        let bandwidth = total_bytes as f64 / 1_073_741_824.0; // 转换为 GB

        let stats = AnalyticsStats {
            total_requests,
            cache_hit_rate,
            bandwidth,
            threats: total_threats,
        };

        // 生成状态码统计（模拟数据，因为 Analytics API 不直接提供）
        let status_codes = vec![
            StatusCodeStat {
                code: "200".to_string(),
                description: "OK".to_string(),
                count: (total_requests as f64 * 0.833) as u64,
                percentage: 83.3,
            },
            StatusCodeStat {
                code: "304".to_string(),
                description: "Not Modified".to_string(),
                count: (total_requests as f64 * 0.1) as u64,
                percentage: 10.0,
            },
            StatusCodeStat {
                code: "404".to_string(),
                description: "Not Found".to_string(),
                count: (total_requests as f64 * 0.04) as u64,
                percentage: 4.0,
            },
            StatusCodeStat {
                code: "500".to_string(),
                description: "Internal Server Error".to_string(),
                count: (total_requests as f64 * 0.01) as u64,
                percentage: 1.0,
            },
            StatusCodeStat {
                code: "Other".to_string(),
                description: "其他".to_string(),
                count: (total_requests as f64 * 0.017) as u64,
                percentage: 1.7,
            },
        ];

        // 生成地域分布（模拟数据）
        let countries = vec![
            CountryStat { rank: 1, country: "中国".to_string(), requests: (total_requests as f64 * 0.4) as u64, percentage: 40.0 },
            CountryStat { rank: 2, country: "美国".to_string(), requests: (total_requests as f64 * 0.2) as u64, percentage: 20.0 },
            CountryStat { rank: 3, country: "日本".to_string(), requests: (total_requests as f64 * 0.1) as u64, percentage: 10.0 },
            CountryStat { rank: 4, country: "德国".to_string(), requests: (total_requests as f64 * 0.06) as u64, percentage: 6.0 },
            CountryStat { rank: 5, country: "英国".to_string(), requests: (total_requests as f64 * 0.05) as u64, percentage: 5.0 },
            CountryStat { rank: 6, country: "法国".to_string(), requests: (total_requests as f64 * 0.04) as u64, percentage: 4.0 },
            CountryStat { rank: 7, country: "加拿大".to_string(), requests: (total_requests as f64 * 0.03) as u64, percentage: 3.0 },
            CountryStat { rank: 8, country: "澳大利亚".to_string(), requests: (total_requests as f64 * 0.025) as u64, percentage: 2.5 },
            CountryStat { rank: 9, country: "韩国".to_string(), requests: (total_requests as f64 * 0.02) as u64, percentage: 2.0 },
            CountryStat { rank: 10, country: "新加坡".to_string(), requests: (total_requests as f64 * 0.015) as u64, percentage: 1.5 },
        ];

        // 生成热门内容（模拟数据）
        let content = vec![
            ContentStat { rank: 1, url: "/images/banner.jpg".to_string(), requests: (total_requests as f64 * 0.05) as u64, bandwidth: format!("{:.1} GB", bandwidth * 0.1) },
            ContentStat { rank: 2, url: "/css/style.css".to_string(), requests: (total_requests as f64 * 0.04) as u64, bandwidth: format!("{:.1} GB", bandwidth * 0.008) },
            ContentStat { rank: 3, url: "/js/app.js".to_string(), requests: (total_requests as f64 * 0.035) as u64, bandwidth: format!("{:.1} GB", bandwidth * 0.036) },
            ContentStat { rank: 4, url: "/index.html".to_string(), requests: (total_requests as f64 * 0.03) as u64, bandwidth: format!("{:.1} GB", bandwidth * 0.003) },
            ContentStat { rank: 5, url: "/api/data".to_string(), requests: (total_requests as f64 * 0.025) as u64, bandwidth: format!("{:.1} GB", bandwidth * 0.026) },
            ContentStat { rank: 6, url: "/images/logo.png".to_string(), requests: (total_requests as f64 * 0.02) as u64, bandwidth: format!("{:.1} GB", bandwidth * 0.004) },
            ContentStat { rank: 7, url: "/fonts/main.woff2".to_string(), requests: (total_requests as f64 * 0.018) as u64, bandwidth: format!("{:.1} GB", bandwidth * 0.018) },
            ContentStat { rank: 8, url: "/about.html".to_string(), requests: (total_requests as f64 * 0.015) as u64, bandwidth: format!("{:.1} GB", bandwidth * 0.002) },
            ContentStat { rank: 9, url: "/contact.html".to_string(), requests: (total_requests as f64 * 0.013) as u64, bandwidth: format!("{:.1} GB", bandwidth * 0.001) },
            ContentStat { rank: 10, url: "/products.html".to_string(), requests: (total_requests as f64 * 0.01) as u64, bandwidth: format!("{:.1} GB", bandwidth * 0.001) },
        ];

        Ok(AnalyticsData {
            stats,
            timeseries: timeseries_data,
            status_codes,
            countries,
            content,
        })
    }

    // 清除缓存
    pub async fn purge_cache(&self, request: &PurgeCacheRequest) -> Result<PurgeCacheResponse, String> {
        let url = format!("{}/zones/{}/purge_cache", CLOUDFLARE_API_BASE, request.zone_id);

        // 构建请求体
        let mut body = serde_json::Map::new();

        if let Some(true) = request.purge_everything {
            // 清除所有缓存
            body.insert("purge_everything".to_string(), json!(true));
            log::info!("Purging all cache for zone {}", request.zone_id);
        } else if let Some(ref files) = request.files {
            // 按 URL 清除
            if files.len() > 30 {
                return Err("Maximum 30 files allowed per request".to_string());
            }
            body.insert("files".to_string(), json!(files));
            log::info!("Purging {} files from cache for zone {}", files.len(), request.zone_id);
        } else if let Some(ref tags) = request.tags {
            // 按标签清除
            if tags.len() > 30 {
                return Err("Maximum 30 tags allowed per request".to_string());
            }
            body.insert("tags".to_string(), json!(tags));
            log::info!("Purging {} tags from cache for zone {}", tags.len(), request.zone_id);
        } else {
            return Err("Must specify purge_everything, files, or tags".to_string());
        }

        let response = self.client
            .post(&url)
            .headers(self.get_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = response.status();
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            let errors = json["errors"].as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|err| err["message"].as_str())
                .unwrap_or("Unknown error");
            return Err(format!("API error ({}): {}", status, errors));
        }

        let result = json["result"].clone();
        let purge_response = PurgeCacheResponse {
            id: result["id"].as_str().unwrap_or("success").to_string(),
        };

        log::info!("Successfully purged cache for zone {}", request.zone_id);
        Ok(purge_response)
    }

    // 获取 SSL 证书信息
    pub async fn get_ssl_certificates(&self, zone_id: &str) -> Result<Vec<SslCertificate>, String> {
        let url = format!("{}/zones/{}/ssl/certificate_packs", CLOUDFLARE_API_BASE, zone_id);

        log::info!("Fetching SSL certificates for zone {}", zone_id);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = response.status();
        log::info!("SSL certificates API response status: {}", status);

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        log::debug!("SSL certificates API response: {:?}", json);

        if !json["success"].as_bool().unwrap_or(false) {
            let errors = json["errors"].as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|err| err["message"].as_str())
                .unwrap_or("Unknown error");
            log::error!("SSL certificates API error ({}): {}, Full response: {:?}", status, errors, json);
            return Err(format!("API error ({}): {}", status, errors));
        }

        let certificates: Vec<SslCertificate> = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse SSL certificates: {}", e))?;

        log::info!("Successfully fetched {} SSL certificates for zone {}", certificates.len(), zone_id);
        Ok(certificates)
    }

    // 获取自定义 SSL 证书列表
    pub async fn get_custom_certificates(&self, zone_id: &str) -> Result<Vec<CustomCertificate>, String> {
        let url = format!("{}/zones/{}/custom_certificates", CLOUDFLARE_API_BASE, zone_id);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let certificates: Vec<CustomCertificate> = json["result"]
            .as_array()
            .ok_or("Invalid result format")?
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();

        Ok(certificates)
    }

    // 上传自定义 SSL 证书
    pub async fn upload_custom_certificate(
        &self,
        zone_id: &str,
        certificate: &str,
        private_key: &str,
        bundle_method: Option<&str>,
    ) -> Result<CustomCertificate, String> {
        let url = format!("{}/zones/{}/custom_certificates", CLOUDFLARE_API_BASE, zone_id);

        let mut body = serde_json::json!({
            "certificate": certificate,
            "private_key": private_key
        });

        if let Some(method) = bundle_method {
            body["bundle_method"] = serde_json::Value::String(method.to_string());
        }

        let response = self.client
            .post(&url)
            .headers(self.get_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let certificate: CustomCertificate = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse certificate: {}", e))?;

        Ok(certificate)
    }

    // 删除自定义 SSL 证书
    pub async fn delete_custom_certificate(&self, zone_id: &str, certificate_id: &str) -> Result<String, String> {
        let url = format!(
            "{}/zones/{}/custom_certificates/{}",
            CLOUDFLARE_API_BASE, zone_id, certificate_id
        );

        let response = self.client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        Ok("Certificate deleted successfully".to_string())
    }

    // 获取页面规则
    pub async fn get_page_rules(&self, zone_id: &str) -> Result<Vec<PageRule>, String> {
        let url = format!("{}/zones/{}/pagerules", CLOUDFLARE_API_BASE, zone_id);

        log::info!("Fetching page rules for zone {}", zone_id);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = response.status();
        log::info!("Page rules API response status: {}", status);

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        log::debug!("Page rules API response: {:?}", json);

        if !json["success"].as_bool().unwrap_or(false) {
            let errors = json["errors"].as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|err| err["message"].as_str())
                .unwrap_or("Unknown error");
            log::error!("Page rules API error ({}): {}, Full response: {:?}", status, errors, json);
            return Err(format!("API error ({}): {}", status, errors));
        }

        let rules: Vec<PageRule> = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse page rules: {}", e))?;

        log::info!("Successfully fetched {} page rules for zone {}", rules.len(), zone_id);
        Ok(rules)
    }

    // 创建页面规则
    pub async fn create_page_rule(&self, zone_id: &str, rule: &PageRule) -> Result<PageRule, String> {
        let url = format!("{}/zones/{}/pagerules", CLOUDFLARE_API_BASE, zone_id);

        log::info!("Creating page rule for zone {}", zone_id);

        let response = self.client
            .post(&url)
            .headers(self.get_headers())
            .json(rule)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = response.status();
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            let errors = json["errors"].as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|err| err["message"].as_str())
                .unwrap_or("Unknown error");
            return Err(format!("API error ({}): {}", status, errors));
        }

        let created_rule: PageRule = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse page rule: {}", e))?;

        log::info!("Successfully created page rule for zone {}", zone_id);
        Ok(created_rule)
    }

    // 更新页面规则
    pub async fn update_page_rule(&self, zone_id: &str, rule_id: &str, rule: &PageRule) -> Result<PageRule, String> {
        let url = format!("{}/zones/{}/pagerules/{}", CLOUDFLARE_API_BASE, zone_id, rule_id);

        log::info!("Updating page rule {} for zone {}", rule_id, zone_id);

        let response = self.client
            .patch(&url)
            .headers(self.get_headers())
            .json(rule)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = response.status();
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            let errors = json["errors"].as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|err| err["message"].as_str())
                .unwrap_or("Unknown error");
            return Err(format!("API error ({}): {}", status, errors));
        }

        let updated_rule: PageRule = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse page rule: {}", e))?;

        log::info!("Successfully updated page rule {} for zone {}", rule_id, zone_id);
        Ok(updated_rule)
    }

    // 删除页面规则
    pub async fn delete_page_rule(&self, zone_id: &str, rule_id: &str) -> Result<String, String> {
        let url = format!("{}/zones/{}/pagerules/{}", CLOUDFLARE_API_BASE, zone_id, rule_id);

        log::info!("Deleting page rule {} for zone {}", rule_id, zone_id);

        let response = self.client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = response.status();
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            let errors = json["errors"].as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|err| err["message"].as_str())
                .unwrap_or("Unknown error");
            return Err(format!("API error ({}): {}", status, errors));
        }

        log::info!("Successfully deleted page rule {} for zone {}", rule_id, zone_id);
        Ok(rule_id.to_string())
    }

    // ===== WAF 规则管理 =====

    // 获取 WAF 包列表
    pub async fn get_waf_packages(&self, zone_id: &str) -> Result<Vec<WafPackage>, String> {
        let url = format!("{}/zones/{}/firewall/waf/packages", CLOUDFLARE_API_BASE, zone_id);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let packages: Vec<WafPackage> = json["result"]
            .as_array()
            .ok_or("Invalid result format")?
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();

        Ok(packages)
    }

    // 获取 WAF 规则列表
    pub async fn get_waf_rules(&self, zone_id: &str, package_id: &str) -> Result<Vec<WafRule>, String> {
        let url = format!(
            "{}/zones/{}/firewall/waf/packages/{}/rules",
            CLOUDFLARE_API_BASE, zone_id, package_id
        );

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let rules: Vec<WafRule> = json["result"]
            .as_array()
            .ok_or("Invalid result format")?
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();

        Ok(rules)
    }

    // 更新 WAF 规则
    pub async fn update_waf_rule(
        &self,
        zone_id: &str,
        package_id: &str,
        rule_id: &str,
        mode: &str,
    ) -> Result<WafRule, String> {
        let url = format!(
            "{}/zones/{}/firewall/waf/packages/{}/rules/{}",
            CLOUDFLARE_API_BASE, zone_id, package_id, rule_id
        );

        let body = serde_json::json!({
            "mode": mode
        });

        let response = self.client
            .patch(&url)
            .headers(self.get_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let rule: WafRule = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse WAF rule: {}", e))?;

        Ok(rule)
    }

    // 更新 WAF 包设置
    pub async fn update_waf_package(
        &self,
        zone_id: &str,
        package_id: &str,
        sensitivity: Option<&str>,
        action_mode: Option<&str>,
    ) -> Result<WafPackage, String> {
        let url = format!(
            "{}/zones/{}/firewall/waf/packages/{}",
            CLOUDFLARE_API_BASE, zone_id, package_id
        );

        let mut body = serde_json::Map::new();
        if let Some(s) = sensitivity {
            body.insert("sensitivity".to_string(), serde_json::Value::String(s.to_string()));
        }
        if let Some(a) = action_mode {
            body.insert("action_mode".to_string(), serde_json::Value::String(a.to_string()));
        }

        let response = self.client
            .patch(&url)
            .headers(self.get_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let package: WafPackage = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse WAF package: {}", e))?;

        Ok(package)
    }

    // ===== Rate Limiting =====

    // 获取速率限制规则列表
    pub async fn get_rate_limits(&self, zone_id: &str) -> Result<Vec<RateLimit>, String> {
        let url = format!("{}/zones/{}/rate_limits", CLOUDFLARE_API_BASE, zone_id);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let rate_limits: Vec<RateLimit> = json["result"]
            .as_array()
            .ok_or("Invalid result format")?
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();

        Ok(rate_limits)
    }

    // 创建速率限制规则
    pub async fn create_rate_limit(&self, zone_id: &str, rate_limit: &CreateRateLimitRequest) -> Result<RateLimit, String> {
        let url = format!("{}/zones/{}/rate_limits", CLOUDFLARE_API_BASE, zone_id);

        let body = serde_json::json!({
            "disabled": rate_limit.disabled,
            "description": rate_limit.description,
            "match": {
                "request": {
                    "url": rate_limit.match_request.url,
                    "methods": rate_limit.match_request.methods,
                    "schemes": rate_limit.match_request.schemes
                }
            },
            "threshold": rate_limit.threshold,
            "period": rate_limit.period,
            "action": rate_limit.action
        });

        let response = self.client
            .post(&url)
            .headers(self.get_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let rate_limit: RateLimit = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse rate limit: {}", e))?;

        Ok(rate_limit)
    }

    // 更新速率限制规则
    pub async fn update_rate_limit(
        &self,
        zone_id: &str,
        rate_limit_id: &str,
        rate_limit: &UpdateRateLimitRequest,
    ) -> Result<RateLimit, String> {
        let url = format!("{}/zones/{}/rate_limits/{}", CLOUDFLARE_API_BASE, zone_id, rate_limit_id);

        let body = serde_json::json!({
            "disabled": rate_limit.disabled,
            "description": rate_limit.description,
            "match": {
                "request": {
                    "url": rate_limit.match_request.url,
                    "methods": rate_limit.match_request.methods,
                    "schemes": rate_limit.match_request.schemes
                }
            },
            "threshold": rate_limit.threshold,
            "period": rate_limit.period,
            "action": rate_limit.action
        });

        let response = self.client
            .put(&url)
            .headers(self.get_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let rate_limit: RateLimit = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse rate limit: {}", e))?;

        Ok(rate_limit)
    }

    // 删除速率限制规则
    pub async fn delete_rate_limit(&self, zone_id: &str, rate_limit_id: &str) -> Result<String, String> {
        let url = format!("{}/zones/{}/rate_limits/{}", CLOUDFLARE_API_BASE, zone_id, rate_limit_id);

        let response = self.client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        Ok("Rate limit deleted successfully".to_string())
    }

    // ==================== Workers KV ====================

    // 列出 KV Namespaces
    pub async fn list_kv_namespaces(&self, account_id: &str) -> Result<Vec<serde_json::Value>, String> {
        let url = format!("{}/accounts/{}/storage/kv/namespaces", CLOUDFLARE_API_BASE, account_id);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let namespaces: Vec<serde_json::Value> = json["result"].as_array()
            .map(|arr| arr.clone())
            .unwrap_or_default();

        Ok(namespaces)
    }

    // 创建 KV Namespace
    pub async fn create_kv_namespace(&self, account_id: &str, title: &str) -> Result<serde_json::Value, String> {
        let url = format!("{}/accounts/{}/storage/kv/namespaces", CLOUDFLARE_API_BASE, account_id);

        let body = serde_json::json!({
            "title": title
        });

        let response = self.client
            .post(&url)
            .headers(self.get_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        Ok(json["result"].clone())
    }

    // 删除 KV Namespace
    pub async fn delete_kv_namespace(&self, account_id: &str, namespace_id: &str) -> Result<String, String> {
        let url = format!("{}/accounts/{}/storage/kv/namespaces/{}", CLOUDFLARE_API_BASE, account_id, namespace_id);

        let response = self.client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        Ok("KV namespace deleted successfully".to_string())
    }

    // 列出 KV 键
    pub async fn list_kv_keys(&self, account_id: &str, namespace_id: &str, prefix: Option<&str>) -> Result<Vec<serde_json::Value>, String> {
        let mut url = format!("{}/accounts/{}/storage/kv/namespaces/{}/keys", CLOUDFLARE_API_BASE, account_id, namespace_id);

        if let Some(p) = prefix {
            url = format!("{}?prefix={}", url, urlencoding::encode(p));
        }

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let keys: Vec<serde_json::Value> = json["result"].as_array()
            .map(|arr| arr.clone())
            .unwrap_or_default();

        Ok(keys)
    }

    // 读取 KV 值
    pub async fn read_kv_value(&self, account_id: &str, namespace_id: &str, key: &str) -> Result<String, String> {
        let url = format!("{}/accounts/{}/storage/kv/namespaces/{}/values/{}",
            CLOUDFLARE_API_BASE, account_id, namespace_id, urlencoding::encode(key));

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: HTTP {}", response.status()));
        }

        let value = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        Ok(value)
    }

    // 写入 KV 值
    pub async fn write_kv_value(
        &self,
        account_id: &str,
        namespace_id: &str,
        key: &str,
        value: &str,
        expiration_ttl: Option<u64>,
        metadata: Option<serde_json::Value>,
    ) -> Result<String, String> {
        let mut url = format!("{}/accounts/{}/storage/kv/namespaces/{}/values/{}",
            CLOUDFLARE_API_BASE, account_id, namespace_id, urlencoding::encode(key));

        // Add query parameters if provided
        let mut params = vec![];
        if let Some(ttl) = expiration_ttl {
            params.push(format!("expiration_ttl={}", ttl));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let mut request_builder = self.client
            .put(&url)
            .headers(self.get_headers())
            .header("Content-Type", "text/plain")
            .body(value.to_string());

        // Add metadata if provided
        if let Some(meta) = metadata {
            request_builder = request_builder.header("metadata", meta.to_string());
        }

        let response = request_builder
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API error: {}", error_text));
        }

        Ok("KV value written successfully".to_string())
    }

    // 删除 KV 键
    pub async fn delete_kv_key(&self, account_id: &str, namespace_id: &str, key: &str) -> Result<String, String> {
        let url = format!("{}/accounts/{}/storage/kv/namespaces/{}/values/{}",
            CLOUDFLARE_API_BASE, account_id, namespace_id, urlencoding::encode(key));

        let response = self.client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API error: {}", error_text));
        }

        Ok("KV key deleted successfully".to_string())
    }

    // ==================== D1 Database ====================

    // 列出 D1 数据库
    pub async fn list_d1_databases(&self, account_id: &str) -> Result<Vec<serde_json::Value>, String> {
        let url = format!("{}/accounts/{}/d1/database", CLOUDFLARE_API_BASE, account_id);

        let response = self.client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        let databases: Vec<serde_json::Value> = json["result"].as_array()
            .map(|arr| arr.clone())
            .unwrap_or_default();

        Ok(databases)
    }

    // 创建 D1 数据库
    pub async fn create_d1_database(&self, account_id: &str, name: &str) -> Result<serde_json::Value, String> {
        let url = format!("{}/accounts/{}/d1/database", CLOUDFLARE_API_BASE, account_id);

        let body = serde_json::json!({
            "name": name
        });

        let response = self.client
            .post(&url)
            .headers(self.get_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        Ok(json["result"].clone())
    }

    // 删除 D1 数据库
    pub async fn delete_d1_database(&self, account_id: &str, database_id: &str) -> Result<String, String> {
        let url = format!("{}/accounts/{}/d1/database/{}", CLOUDFLARE_API_BASE, account_id, database_id);

        let response = self.client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        Ok("D1 database deleted successfully".to_string())
    }

    // 执行 D1 查询
    pub async fn execute_d1_query(&self, account_id: &str, database_id: &str, query: &str) -> Result<serde_json::Value, String> {
        let url = format!("{}/accounts/{}/d1/database/{}/query", CLOUDFLARE_API_BASE, account_id, database_id);

        let body = serde_json::json!({
            "sql": query
        });

        println!("D1 Query URL: {}", url);
        println!("D1 Query Body: {}", serde_json::to_string_pretty(&body).unwrap_or_default());

        let response = self.client
            .post(&url)
            .headers(self.get_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = response.status();
        println!("D1 Query Response Status: {}", status);

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        println!("D1 Query Response: {}", serde_json::to_string_pretty(&json).unwrap_or_default());

        if !json["success"].as_bool().unwrap_or(false) {
            return Err(format!("API error: {:?}", json["errors"]));
        }

        // Return the first result object from the array
        if let Some(results) = json["result"].as_array() {
            if let Some(first_result) = results.first() {
                return Ok(first_result.clone());
            }
        }

        Err("No query results returned".to_string())
    }
}
