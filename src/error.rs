async fn return_error() -> String {
    format!("{{
\"success\": false,
\"data\": null
}}")
}

pub mod http_error {
    use actix_web::{ HttpResponse };

    pub async fn not_found() -> HttpResponse {
        HttpResponse::NotFound()
            .content_type("application/json")
            .body(
                super::return_error().await
            )
    }
}