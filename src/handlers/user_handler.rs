use actix_web::{get, HttpResponse, Responder};
use crate::api::user::get_user_info;
use crate::utils::token_manager::get_access_token;
use log::{debug, error};

#[get("/user/info")]
pub async fn user_info_handler() -> impl Responder {
    debug!("Handling /user/info request");

    match get_access_token() {
        Some(token) => {
            debug!("Using access token: {}", token);
            match get_user_info(&token).await {
                Ok(user_info) => {
                    debug!("Successfully fetched user info: {:?}", user_info);
                    HttpResponse::Ok().json(user_info)
                }
                Err(err) => {
                    error!("Error fetching user info: {}", err);
                    HttpResponse::InternalServerError().body(err.to_string())
                }
            }
        }
        None => {
            error!("Access token is missing or invalid");
            HttpResponse::Unauthorized().body("Missing or invalid access token")
        }
    }
}
