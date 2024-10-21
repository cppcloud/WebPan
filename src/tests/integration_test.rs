#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_user_info_handler() {
        let app = test::init_service(App::new().service(user_info_handler)).await;
        let req = test::TestRequest::get().uri("/user/info").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
