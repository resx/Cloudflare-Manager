use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::cloudflare::CloudflareClient;
use crate::models::*;

// 健康检查
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "cloudflare-manager-api",
        "version": "0.1.0"
    }))
}

// 获取 Cloudflare 账户列表（自动获取 Account ID）
pub async fn get_accounts(payload: web::Bytes, _req: HttpRequest) -> impl Responder {
    let payload_str = match String::from_utf8(payload.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to convert payload to string: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid UTF-8 in request body".to_string()));
        }
    };

    log::debug!("get_accounts received payload: {}", payload_str);

    let req: CloudflareRequest<serde_json::Value> = match serde_json::from_str(&payload_str) {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to parse get_accounts request: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(format!("Invalid request format: {}", e)));
        }
    };

    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_accounts().await {
        Ok(accounts) => HttpResponse::Ok().json(ApiResponse::success(accounts)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取所有 Zone
pub async fn get_zones(payload: web::Bytes, _req: HttpRequest) -> impl Responder {
    let payload_str = match String::from_utf8(payload.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to convert payload to string: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid UTF-8 in request body".to_string()));
        }
    };

    log::debug!("get_zones received payload: {}", payload_str);

    let req: CloudflareRequest<serde_json::Value> = match serde_json::from_str(&payload_str) {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to parse get_zones request: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(format!("Invalid request format: {}", e)));
        }
    };

    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_zones().await {
        Ok(zones) => HttpResponse::Ok().json(ApiResponse::success(zones)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取 DNS 记录
pub async fn get_dns_records(payload: web::Bytes, _req: HttpRequest) -> impl Responder {
    let payload_str = match String::from_utf8(payload.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to convert payload to string: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid UTF-8 in request body".to_string()));
        }
    };

    log::debug!("get_dns_records received payload: {}", payload_str);

    let req: CloudflareRequest<GetDnsRecordsRequest> = match serde_json::from_str(&payload_str) {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to parse get_dns_records request: {}",  e);
            log::error!("Raw payload was: {}", payload_str);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(format!("Invalid request format: {}", e)));
        }
    };

    log::debug!("Parsed zone_id: {}", req.data.zone_id);

    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to create CloudflareClient: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e));
        }
    };

    match client.get_dns_records(&req.data.zone_id).await {
        Ok(records) => {
            log::info!("Successfully fetched {} DNS records for zone {}", records.len(), req.data.zone_id);
            HttpResponse::Ok().json(ApiResponse::success(records))
        }
        Err(e) => {
            log::error!("Failed to get DNS records: {}", e);
            HttpResponse::BadRequest().json(ApiResponse::<()>::error(e))
        }
    }
}

// 创建 DNS 记录
pub async fn create_dns_record(req: web::Json<CloudflareRequest<DnsRecord>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.create_dns_record(&req.data).await {
        Ok(record) => HttpResponse::Ok().json(ApiResponse::success(record)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 更新 DNS 记录
pub async fn update_dns_record(req: web::Json<CloudflareRequest<DnsRecord>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.update_dns_record(&req.data).await {
        Ok(record) => HttpResponse::Ok().json(ApiResponse::success(record)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 删除 DNS 记录
pub async fn delete_dns_record(req: web::Json<CloudflareRequest<DeleteRecordRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.delete_dns_record(&req.data.zone_id, &req.data.record_id).await {
        Ok(id) => HttpResponse::Ok().json(ApiResponse::success(id)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取防火墙规则
pub async fn get_firewall_rules(req: web::Json<CloudflareRequest<GetFirewallRulesRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_firewall_rules(&req.data.zone_id).await {
        Ok(rules) => HttpResponse::Ok().json(ApiResponse::success(rules)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 创建防火墙规则
pub async fn create_firewall_rule(req: web::Json<CloudflareRequest<CreateFirewallRuleRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.create_firewall_rule(&req.data.zone_id, &req.data.rule).await {
        Ok(rule) => HttpResponse::Ok().json(ApiResponse::success(rule)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 更新防火墙规则
pub async fn update_firewall_rule(req: web::Json<CloudflareRequest<UpdateFirewallRuleRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.update_firewall_rule(&req.data.zone_id, &req.data.rule_id, &req.data.rule).await {
        Ok(rule) => HttpResponse::Ok().json(ApiResponse::success(rule)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 删除防火墙规则
pub async fn delete_firewall_rule(req: web::Json<CloudflareRequest<DeleteFirewallRuleRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.delete_firewall_rule(&req.data.zone_id, &req.data.rule_id).await {
        Ok(id) => HttpResponse::Ok().json(ApiResponse::success(id)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 部署 Worker
pub async fn deploy_worker(req: web::Json<CloudflareRequest<DeployWorkerRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.deploy_worker(&req.data).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 列出 Workers
pub async fn list_workers(req: web::Json<CloudflareRequest<ListWorkersRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.list_workers(&req.data.account_id).await {
        Ok(workers) => HttpResponse::Ok().json(ApiResponse::success(workers)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取单个 Worker
pub async fn get_worker(req: web::Json<CloudflareRequest<GetWorkerRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_worker(&req.data.account_id, &req.data.script_name).await {
        Ok(script) => HttpResponse::Ok().json(ApiResponse::success(script)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 删除 Worker
pub async fn delete_worker(req: web::Json<CloudflareRequest<DeleteWorkerRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.delete_worker(&req.data.account_id, &req.data.script_name).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 上传/更新 Worker
pub async fn upload_worker(req: web::Json<CloudflareRequest<UploadWorkerRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.upload_worker(&req.data.account_id, &req.data.script_name, &req.data.script_content).await {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取 Worker 路由列表
pub async fn get_worker_routes(req: web::Json<CloudflareRequest<GetWorkerRoutesRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_worker_routes(&req.data.zone_id).await {
        Ok(routes) => HttpResponse::Ok().json(ApiResponse::success(routes)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 创建 Worker 路由
pub async fn create_worker_route(req: web::Json<CloudflareRequest<CreateWorkerRouteRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.create_worker_route(&req.data.zone_id, &req.data.pattern, &req.data.script_name).await {
        Ok(route) => HttpResponse::Ok().json(ApiResponse::success(route)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 删除 Worker 路由
pub async fn delete_worker_route(req: web::Json<CloudflareRequest<DeleteWorkerRouteRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.delete_worker_route(&req.data.zone_id, &req.data.route_id).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取 Zone 设置
pub async fn get_zone_settings(req: web::Json<CloudflareRequest<GetZoneSettingsRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_zone_settings(&req.data.zone_id).await {
        Ok(settings) => HttpResponse::Ok().json(ApiResponse::success(settings)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 更新 Zone 设置
pub async fn update_zone_settings(req: web::Json<CloudflareRequest<UpdateZoneSettingsRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.update_zone_settings(&req.data.zone_id, &req.data.settings).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 自动优化 Zone
pub async fn optimize_zone(req: web::Json<CloudflareRequest<OptimizeZoneRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.optimize_zone(&req.data.zone_id, &req.data.mode).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取 Analytics 数据
pub async fn get_analytics(req: web::Json<CloudflareRequest<GetAnalyticsRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_analytics(&req.data.zone_id, &req.data.time_range).await {
        Ok(analytics) => HttpResponse::Ok().json(ApiResponse::success(analytics)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 清除缓存
pub async fn purge_cache(req: web::Json<CloudflareRequest<PurgeCacheRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.purge_cache(&req.data).await {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取 SSL 证书信息
pub async fn get_ssl_certificates(payload: web::Bytes, _req: HttpRequest) -> impl Responder {
    let payload_str = match String::from_utf8(payload.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to convert payload to string: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid UTF-8 in request body".to_string()));
        }
    };

    log::debug!("get_ssl_certificates received payload: {}", payload_str);

    let req: CloudflareRequest<GetSslCertificatesRequest> = match serde_json::from_str(&payload_str) {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to parse get_ssl_certificates request: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(format!("Invalid request format: {}", e)));
        }
    };

    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_ssl_certificates(&req.data.zone_id).await {
        Ok(certificates) => HttpResponse::Ok().json(ApiResponse::success(certificates)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取自定义 SSL 证书列表
pub async fn get_custom_certificates(payload: web::Bytes, _req: HttpRequest) -> impl Responder {
    let payload_str = match String::from_utf8(payload.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to convert payload to string: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid UTF-8 in request body".to_string()));
        }
    };

    log::debug!("get_custom_certificates received payload: {}", payload_str);

    let req: CloudflareRequest<GetCustomCertificatesRequest> = match serde_json::from_str(&payload_str) {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to parse get_custom_certificates request: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(format!("Invalid request format: {}", e)));
        }
    };

    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_custom_certificates(&req.data.zone_id).await {
        Ok(certificates) => HttpResponse::Ok().json(ApiResponse::success(certificates)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 上传自定义 SSL 证书
pub async fn upload_custom_certificate(req: web::Json<CloudflareRequest<UploadCustomCertificateRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.upload_custom_certificate(
        &req.data.zone_id,
        &req.data.certificate,
        &req.data.private_key,
        req.data.bundle_method.as_deref(),
    ).await {
        Ok(certificate) => HttpResponse::Ok().json(ApiResponse::success(certificate)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 删除自定义 SSL 证书
pub async fn delete_custom_certificate(req: web::Json<CloudflareRequest<DeleteCustomCertificateRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.delete_custom_certificate(&req.data.zone_id, &req.data.certificate_id).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取页面规则
pub async fn get_page_rules(payload: web::Bytes, _req: HttpRequest) -> impl Responder {
    let payload_str = match String::from_utf8(payload.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to convert payload to string: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid UTF-8 in request body".to_string()));
        }
    };

    log::debug!("get_page_rules received payload: {}", payload_str);

    let req: CloudflareRequest<GetPageRulesRequest> = match serde_json::from_str(&payload_str) {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to parse get_page_rules request: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(format!("Invalid request format: {}", e)));
        }
    };

    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_page_rules(&req.data.zone_id).await {
        Ok(rules) => HttpResponse::Ok().json(ApiResponse::success(rules)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 创建页面规则
pub async fn create_page_rule(req: web::Json<CloudflareRequest<CreatePageRuleRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.create_page_rule(&req.data.zone_id, &req.data.rule).await {
        Ok(rule) => HttpResponse::Ok().json(ApiResponse::success(rule)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 更新页面规则
pub async fn update_page_rule(req: web::Json<CloudflareRequest<UpdatePageRuleRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.update_page_rule(&req.data.zone_id, &req.data.rule_id, &req.data.rule).await {
        Ok(rule) => HttpResponse::Ok().json(ApiResponse::success(rule)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 删除页面规则
pub async fn delete_page_rule(req: web::Json<CloudflareRequest<DeletePageRuleRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.delete_page_rule(&req.data.zone_id, &req.data.rule_id).await {
        Ok(id) => HttpResponse::Ok().json(ApiResponse::success(id)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// ===== WAF 规则管理 =====

// 获取 WAF 包列表
pub async fn get_waf_packages(req: web::Json<CloudflareRequest<GetWafPackagesRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_waf_packages(&req.data.zone_id).await {
        Ok(packages) => HttpResponse::Ok().json(ApiResponse::success(packages)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取 WAF 规则列表
pub async fn get_waf_rules(req: web::Json<CloudflareRequest<GetWafRulesRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_waf_rules(&req.data.zone_id, &req.data.package_id).await {
        Ok(rules) => HttpResponse::Ok().json(ApiResponse::success(rules)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 更新 WAF 规则
pub async fn update_waf_rule(req: web::Json<CloudflareRequest<UpdateWafRuleRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.update_waf_rule(
        &req.data.zone_id,
        &req.data.package_id,
        &req.data.rule_id,
        &req.data.mode,
    ).await {
        Ok(rule) => HttpResponse::Ok().json(ApiResponse::success(rule)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 更新 WAF 包设置
pub async fn update_waf_package(req: web::Json<CloudflareRequest<UpdateWafPackageRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.update_waf_package(
        &req.data.zone_id,
        &req.data.package_id,
        req.data.sensitivity.as_deref(),
        req.data.action_mode.as_deref(),
    ).await {
        Ok(package) => HttpResponse::Ok().json(ApiResponse::success(package)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// ===== Rate Limiting =====

// 获取速率限制规则列表
pub async fn get_rate_limits(req: web::Json<CloudflareRequest<GetRateLimitsRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.get_rate_limits(&req.data.zone_id).await {
        Ok(rate_limits) => HttpResponse::Ok().json(ApiResponse::success(rate_limits)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 创建速率限制规则
pub async fn create_rate_limit(req: web::Json<CloudflareRequest<CreateRateLimitRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.create_rate_limit(&req.data.zone_id, &req.data).await {
        Ok(rate_limit) => HttpResponse::Ok().json(ApiResponse::success(rate_limit)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 更新速率限制规则
pub async fn update_rate_limit(req: web::Json<CloudflareRequest<UpdateRateLimitRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.update_rate_limit(&req.data.zone_id, &req.data.rate_limit_id, &req.data).await {
        Ok(rate_limit) => HttpResponse::Ok().json(ApiResponse::success(rate_limit)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 删除速率限制规则
pub async fn delete_rate_limit(req: web::Json<CloudflareRequest<DeleteRateLimitRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.delete_rate_limit(&req.data.zone_id, &req.data.rate_limit_id).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// ==================== Workers KV ====================

// 列出 KV Namespaces
pub async fn list_kv_namespaces(req: web::Json<CloudflareRequest<ListKVNamespacesRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.list_kv_namespaces(&req.data.account_id).await {
        Ok(namespaces) => HttpResponse::Ok().json(ApiResponse::success(namespaces)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 创建 KV Namespace
pub async fn create_kv_namespace(req: web::Json<CloudflareRequest<CreateKVNamespaceRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.create_kv_namespace(&req.data.account_id, &req.data.title).await {
        Ok(namespace) => HttpResponse::Ok().json(ApiResponse::success(namespace)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 删除 KV Namespace
pub async fn delete_kv_namespace(req: web::Json<CloudflareRequest<DeleteKVNamespaceRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.delete_kv_namespace(&req.data.account_id, &req.data.namespace_id).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 列出 KV 键
pub async fn list_kv_keys(req: web::Json<CloudflareRequest<ListKVKeysRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.list_kv_keys(&req.data.account_id, &req.data.namespace_id, req.data.prefix.as_deref()).await {
        Ok(keys) => HttpResponse::Ok().json(ApiResponse::success(keys)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 读取 KV 值
pub async fn read_kv_value(req: web::Json<CloudflareRequest<ReadKVValueRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.read_kv_value(&req.data.account_id, &req.data.namespace_id, &req.data.key).await {
        Ok(value) => HttpResponse::Ok().json(ApiResponse::success(value)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 写入 KV 值
pub async fn write_kv_value(req: web::Json<CloudflareRequest<WriteKVValueRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.write_kv_value(
        &req.data.account_id,
        &req.data.namespace_id,
        &req.data.key,
        &req.data.value,
        req.data.expiration_ttl,
        req.data.metadata.clone(),
    ).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 删除 KV 键
pub async fn delete_kv_key(req: web::Json<CloudflareRequest<DeleteKVKeyRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.delete_kv_key(&req.data.account_id, &req.data.namespace_id, &req.data.key).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// ==================== D1 Database ====================

// 列出 D1 数据库
pub async fn list_d1_databases(req: web::Json<CloudflareRequest<ListD1DatabasesRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.list_d1_databases(&req.data.account_id).await {
        Ok(databases) => HttpResponse::Ok().json(ApiResponse::success(databases)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 创建 D1 数据库
pub async fn create_d1_database(req: web::Json<CloudflareRequest<CreateD1DatabaseRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.create_d1_database(&req.data.account_id, &req.data.name).await {
        Ok(database) => HttpResponse::Ok().json(ApiResponse::success(database)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 删除 D1 数据库
pub async fn delete_d1_database(req: web::Json<CloudflareRequest<DeleteD1DatabaseRequest>>) -> impl Responder {
    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    };

    match client.delete_d1_database(&req.data.account_id, &req.data.database_id).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 执行 D1 查询
pub async fn execute_d1_query(req: web::Json<CloudflareRequest<ExecuteD1QueryRequest>>) -> impl Responder {
    log::info!("=== Execute D1 Query Request ===");
    log::info!("Account ID: {}", req.data.account_id);
    log::info!("Database ID: {}", req.data.database_id);
    log::info!("Query: {}", req.data.query);

    let client = match CloudflareClient::new(&req.credentials) {
        Ok(c) => c,
        Err(e) => {
            log::error!("Client creation error: {}", e);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e));
        }
    };

    match client.execute_d1_query(&req.data.account_id, &req.data.database_id, &req.data.query).await {
        Ok(result) => HttpResponse::Ok().json(ApiResponse::success(result)),
        Err(e) => {
            log::error!("D1 query error: {}", e);
            HttpResponse::BadRequest().json(ApiResponse::<()>::error(e))
        }
    }
}
