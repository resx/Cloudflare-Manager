use actix_web::{web, HttpResponse, Responder};
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

// 获取所有 Zone
pub async fn get_zones(req: web::Json<CloudflareRequest<serde_json::Value>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.get_zones().await {
        Ok(zones) => HttpResponse::Ok().json(ApiResponse::success(zones)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取 DNS 记录
pub async fn get_dns_records(req: web::Json<CloudflareRequest<GetDnsRecordsRequest>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.get_dns_records(&req.data.zone_id).await {
        Ok(records) => HttpResponse::Ok().json(ApiResponse::success(records)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 创建 DNS 记录
pub async fn create_dns_record(req: web::Json<CloudflareRequest<DnsRecord>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.create_dns_record(&req.data).await {
        Ok(record) => HttpResponse::Ok().json(ApiResponse::success(record)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 更新 DNS 记录
pub async fn update_dns_record(req: web::Json<CloudflareRequest<DnsRecord>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.update_dns_record(&req.data).await {
        Ok(record) => HttpResponse::Ok().json(ApiResponse::success(record)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 删除 DNS 记录
pub async fn delete_dns_record(req: web::Json<CloudflareRequest<DeleteRecordRequest>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.delete_dns_record(&req.data.zone_id, &req.data.record_id).await {
        Ok(id) => HttpResponse::Ok().json(ApiResponse::success(id)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 获取防火墙规则
pub async fn get_firewall_rules(req: web::Json<CloudflareRequest<GetFirewallRulesRequest>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.get_firewall_rules(&req.data.zone_id).await {
        Ok(rules) => HttpResponse::Ok().json(ApiResponse::success(rules)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 创建防火墙规则
pub async fn create_firewall_rule(req: web::Json<CloudflareRequest<CreateFirewallRuleRequest>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.create_firewall_rule(&req.data.zone_id, &req.data.rule).await {
        Ok(rule) => HttpResponse::Ok().json(ApiResponse::success(rule)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 更新防火墙规则
pub async fn update_firewall_rule(req: web::Json<CloudflareRequest<UpdateFirewallRuleRequest>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.update_firewall_rule(&req.data.zone_id, &req.data.rule_id, &req.data.rule).await {
        Ok(rule) => HttpResponse::Ok().json(ApiResponse::success(rule)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 删除防火墙规则
pub async fn delete_firewall_rule(req: web::Json<CloudflareRequest<DeleteFirewallRuleRequest>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.delete_firewall_rule(&req.data.zone_id, &req.data.rule_id).await {
        Ok(id) => HttpResponse::Ok().json(ApiResponse::success(id)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 部署 Worker
pub async fn deploy_worker(req: web::Json<CloudflareRequest<DeployWorkerRequest>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.deploy_worker(&req.data).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 列出 Workers
pub async fn list_workers(req: web::Json<CloudflareRequest<ListWorkersRequest>>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success(Vec::<String>::new()))
}

// 获取 Zone 设置
pub async fn get_zone_settings(req: web::Json<CloudflareRequest<GetZoneSettingsRequest>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.get_zone_settings(&req.data.zone_id).await {
        Ok(settings) => HttpResponse::Ok().json(ApiResponse::success(settings)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 更新 Zone 设置
pub async fn update_zone_settings(req: web::Json<CloudflareRequest<UpdateZoneSettingsRequest>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.update_zone_settings(&req.data.zone_id, &req.data.settings).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

// 自动优化 Zone
pub async fn optimize_zone(req: web::Json<CloudflareRequest<OptimizeZoneRequest>>) -> impl Responder {
    let client = CloudflareClient::new(&req.credentials);

    match client.optimize_zone(&req.data.zone_id, &req.data.mode).await {
        Ok(message) => HttpResponse::Ok().json(ApiResponse::success(message)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}
