use axum::{routing::post, serve, Router};
use dotenv::{dotenv, var};
use http::HeaderValue;
use resume_api::common::request::get_resume;
use std::{net::SocketAddr, str::FromStr};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv().ok();
    // 启动日志记录
    tracing_subscriber::fmt::init();
    // 设置跨域
    let domain = var("DOMAIN").expect("DOMAIN must be set");
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list([domain.parse::<HeaderValue>().unwrap()]))
        .allow_methods(Any)
        .allow_headers(Any);
    // 构建路由
    let app: Router = Router::new()
        .route("/api/resume", post(get_resume))
        .layer(cors);
    // 服务端口
    let host_str = var("HOST").expect("HOST must be set");
    let port_str = var("PORT").expect("PORT must be set");
    let addr =
        SocketAddr::from_str(&format!("{}:{}", host_str, port_str)).expect("SocketAddr fail");
    // 启动服务
    let listener: TcpListener = TcpListener::bind(&addr).await.expect("listener fail");
    serve(listener, app).await.expect("serve start fail");
}
