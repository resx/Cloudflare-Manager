use crate::models::*;
use reqwest::{header, Client};
use serde_json::json;

const CLOUDFLARE_API_BASE: &str = "https://api.cloudflare.com/client/v4";

pub struct CloudflareClient {
    client: Client,
    email: String,
    api_key: String,
}

impl CloudflareClient {
    pub fn new(credentials: &CloudflareCredentials) -> Self {
        CloudflareClient {
            client: Client::new(),
            email: credentials.email.clone(),
            api_key: credentials.api_key.clone(),
        }
    }

    fn get_headers(&self) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Auth-Email", header::HeaderValue::from_str(&self.email).unwrap());
        headers.insert("X-Auth-Key", header::HeaderValue::from_str(&self.api_key).unwrap());
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
        headers
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
        let url = format!("{}/zones/{}/dns_records", CLOUDFLARE_API_BASE, record.zone_id);

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
        let url = format!("{}/zones/{}/dns_records/{}", CLOUDFLARE_API_BASE, record.zone_id, record_id);

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

    // 获取 Workers 列表
    pub async fn list_workers(&self, account_id: &str) -> Result<Vec<String>, String> {
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

        let workers: Vec<String> = json["result"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|w| w["id"].as_str().map(|s| s.to_string()))
            .collect();

        Ok(workers)
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

        Ok(script_name.to_string())
    }
}
