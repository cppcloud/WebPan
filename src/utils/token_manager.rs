use std::sync::Mutex;
use lazy_static::lazy_static;
use log::{debug, error};

lazy_static! {
    static ref ACCESS_TOKEN: Mutex<Option<String>> = Mutex::new(None);
}

pub fn set_access_token(token: String) {
    let mut access_token = ACCESS_TOKEN.lock().unwrap();
    debug!("Setting new access token: {}", token);
    *access_token = Some(token);
}

pub fn get_access_token() -> Option<String> {
    let access_token = ACCESS_TOKEN.lock().unwrap();
    if let Some(ref token) = *access_token {
        debug!("Retrieved access token: {}", token);
    } else {
        error!("Access token is missing");
    }
    access_token.clone()
}
