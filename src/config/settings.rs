// src/config/settings.rs

use std::env;

pub fn get_credentials() -> (String, String) {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");

    log::debug!("Loaded CLIENT_ID: {}", client_id);
    log::debug!("Loaded CLIENT_SECRET: {}", client_secret);

    (client_id, client_secret)
}
