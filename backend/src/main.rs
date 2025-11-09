use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer, error};
use std::env;

mod cloudflare;
mod handlers;
mod models;

// 自定义 JSON 错误处理器
fn json_error_handler(err: error::JsonPayloadError, _req: &actix_web::HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    log::error!("JSON payload error: {}", detail);

    let resp = match &err {
        JsonPayloadError::Deserialize(e) => {
            log::error!("Deserialization error: {}", e);
            actix_web::HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": format!("Invalid JSON format: {}", e)
            }))
        }
        _ => actix_web::HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": detail
        }))
    };

    error::InternalError::from_response(err, resp).into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("{}:{}", host, port);

    log::info!("🚀 Starting Cloudflare Manager API Server");
    log::info!("📡 Listening on: http://{}", bind_addr);

    HttpServer::new(|| {
        // CORS 配置
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        // JSON 配置 - 增加大小限制和自定义错误处理
        let json_cfg = web::JsonConfig::default()
            .limit(1024 * 1024)  // 1MB - 足够大以支持包含 API Token 的请求
            .error_handler(json_error_handler);

        // Payload 配置 - 设置更大的限制以支持 String 提取
        let payload_cfg = web::PayloadConfig::default()
            .limit(1024 * 1024); // 1MB

        App::new()
            .app_data(json_cfg)
            .app_data(payload_cfg)
            .wrap(cors)
            .wrap(Logger::default())
            // 健康检查
            .route("/health", web::get().to(handlers::health_check))
            // Cloudflare API 代理路由
            .service(
                web::scope("/cloudflare")
                    .route("/accounts", web::post().to(handlers::get_accounts))
                    .route("/zones", web::post().to(handlers::get_zones))
                    .route("/dns/records", web::post().to(handlers::get_dns_records))
                    .route("/dns/records/create", web::post().to(handlers::create_dns_record))
                    .route("/dns/records/update", web::post().to(handlers::update_dns_record))
                    .route("/dns/records/delete", web::post().to(handlers::delete_dns_record))
                    .route("/firewall/rules", web::post().to(handlers::get_firewall_rules))
                    .route("/firewall/rules/create", web::post().to(handlers::create_firewall_rule))
                    .route("/firewall/rules/update", web::post().to(handlers::update_firewall_rule))
                    .route("/firewall/rules/delete", web::post().to(handlers::delete_firewall_rule))
                    .route("/workers/deploy", web::post().to(handlers::deploy_worker))
                    .route("/workers/list", web::post().to(handlers::list_workers))
                    .route("/workers/get", web::post().to(handlers::get_worker))
                    .route("/workers/delete", web::post().to(handlers::delete_worker))
                    .route("/workers/upload", web::post().to(handlers::upload_worker))
                    .route("/workers/routes", web::post().to(handlers::get_worker_routes))
                    .route("/workers/routes/create", web::post().to(handlers::create_worker_route))
                    .route("/workers/routes/delete", web::post().to(handlers::delete_worker_route))
                    .route("/zone/settings", web::post().to(handlers::get_zone_settings))
                    .route("/zone/settings/update", web::post().to(handlers::update_zone_settings))
                    .route("/zone/optimize", web::post().to(handlers::optimize_zone))
                    .route("/analytics", web::post().to(handlers::get_analytics))
                    .route("/cache/purge", web::post().to(handlers::purge_cache))
                    .route("/ssl/certificates", web::post().to(handlers::get_ssl_certificates))
                    .route("/ssl/custom", web::post().to(handlers::get_custom_certificates))
                    .route("/ssl/custom/upload", web::post().to(handlers::upload_custom_certificate))
                    .route("/ssl/custom/delete", web::post().to(handlers::delete_custom_certificate))
                    .route("/pagerules", web::post().to(handlers::get_page_rules))
                    .route("/pagerules/create", web::post().to(handlers::create_page_rule))
                    .route("/pagerules/update", web::post().to(handlers::update_page_rule))
                    .route("/pagerules/delete", web::post().to(handlers::delete_page_rule))
                    .route("/waf/packages", web::post().to(handlers::get_waf_packages))
                    .route("/waf/rules", web::post().to(handlers::get_waf_rules))
                    .route("/waf/rules/update", web::post().to(handlers::update_waf_rule))
                    .route("/waf/packages/update", web::post().to(handlers::update_waf_package))
                    .route("/ratelimits", web::post().to(handlers::get_rate_limits))
                    .route("/ratelimits/create", web::post().to(handlers::create_rate_limit))
                    .route("/ratelimits/update", web::post().to(handlers::update_rate_limit))
                    .route("/ratelimits/delete", web::post().to(handlers::delete_rate_limit))
                    // Workers KV routes
                    .route("/kv/namespaces", web::post().to(handlers::list_kv_namespaces))
                    .route("/kv/namespaces/create", web::post().to(handlers::create_kv_namespace))
                    .route("/kv/namespaces/delete", web::post().to(handlers::delete_kv_namespace))
                    .route("/kv/keys", web::post().to(handlers::list_kv_keys))
                    .route("/kv/read", web::post().to(handlers::read_kv_value))
                    .route("/kv/write", web::post().to(handlers::write_kv_value))
                    .route("/kv/delete", web::post().to(handlers::delete_kv_key))
                    // D1 Database routes
                    .route("/d1/databases", web::post().to(handlers::list_d1_databases))
                    .route("/d1/databases/create", web::post().to(handlers::create_d1_database))
                    .route("/d1/databases/delete", web::post().to(handlers::delete_d1_database))
                    .route("/d1/query", web::post().to(handlers::execute_d1_query))
            )
    })
    .bind(&bind_addr)?
    .run()
    .await
}
