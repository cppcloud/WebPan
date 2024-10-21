use actix_web::{web, App, HttpServer};
use WebPan::handlers::user_handler::user_info_handler;
use WebPan::api::auth::fetch_access_token;
use WebPan::config::settings::get_credentials;
use WebPan::utils::token_manager::set_access_token;
use dotenv::dotenv;
use log::{info, error};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let (client_id, client_secret) = get_credentials();
    match fetch_access_token(&client_id, &client_secret).await {
        Ok(token) => set_access_token(token),
        Err(err) => {
            error!("Failed to fetch access token: {}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to initialize access token"));
        }
    }

    info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(user_info_handler) // 注册用户信息接口
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
