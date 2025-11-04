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
            .limit(4096)
            .error_handler(json_error_handler);

        App::new()
            .app_data(json_cfg)
            .wrap(cors)
            .wrap(Logger::default())
            // 健康检查
            .route("/health", web::get().to(handlers::health_check))
            // Cloudflare API 代理路由
            .service(
                web::scope("/cloudflare")
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
                    .route("/zone/settings", web::post().to(handlers::get_zone_settings))
                    .route("/zone/settings/update", web::post().to(handlers::update_zone_settings))
                    .route("/zone/optimize", web::post().to(handlers::optimize_zone))
                    .route("/analytics", web::post().to(handlers::get_analytics))
            )
    })
    .bind(&bind_addr)?
    .run()
    .await
}
