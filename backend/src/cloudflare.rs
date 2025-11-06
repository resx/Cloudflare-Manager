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
        // 验证凭证 - 必须提供 email + api_key
        if credentials.email.is_none() || credentials.api_key.is_none() {
            return Err("Invalid credentials: email and api_key are required".to_string());
        }

        Ok(CloudflareClient {
            client: Client::new(),
            credentials: credentials.clone(),
        })
    }

    // REST API 使用 Email + Global API Key（主要认证方式）
    fn get_headers(&self) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();

        // REST API 始终使用 Email + API Key
        if let (Some(email), Some(api_key)) = (&self.credentials.email, &self.credentials.api_key) {
            headers.insert("X-Auth-Email", header::HeaderValue::from_str(email).unwrap());
            headers.insert("X-Auth-Key", header::HeaderValue::from_str(api_key).unwrap());
        }

        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
        headers
    }

    // GraphQL API 优先使用 API Token，回退到 Email + API Key
    fn get_graphql_headers(&self) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();

        // 优先使用 API Token（用于 Analytics）
        if let Some(token) = &self.credentials.api_token {
            log::info!("Using API Token for GraphQL authentication");
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap()
            );
        }
        // 回退到 Email + API Key
        else if let (Some(email), Some(api_key)) = (&self.credentials.email, &self.credentials.api_key) {
            log::info!("Using Email + API Key for GraphQL authentication (API Token not provided)");
            headers.insert("X-Auth-Email", header::HeaderValue::from_str(email).unwrap());
            headers.insert("X-Auth-Key", header::HeaderValue::from_str(api_key).unwrap());
        } else {
            log::warn!("No valid credentials available for GraphQL authentication");
        }

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

        let certificates: Vec<SslCertificate> = serde_json::from_value(json["result"].clone())
            .map_err(|e| format!("Failed to parse SSL certificates: {}", e))?;

        log::info!("Successfully fetched {} SSL certificates for zone {}", certificates.len(), zone_id);
        Ok(certificates)
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
}
