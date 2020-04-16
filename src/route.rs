async fn return_json(result: &serde_json::value::Value) -> String {
    if result != "null" {
        return format!("{{
  \"success\": true,
  \"data\": {}
}}", result)
    }

    return format!("{{
  \"success\": false,
  \"data\": null
}}")
}

pub mod common {
    use actix_web::{ HttpResponse, web, get };
    use actix_web::http::header::{ CacheControl, CacheDirective };
    
    use crate::cache::Cache;

    #[get("/")]
    pub async fn get_all_menu(
        cache: web::Data<Cache>
    ) -> HttpResponse {
        let dreamin = &cache.data;

        HttpResponse::Ok()
            .content_type("application/json")
            .set(
                CacheControl(
                    vec![
                        CacheDirective::MaxAge(86400u32),
                        CacheDirective::Public
                    ]
                )
            )
            .body(
                super::return_json(dreamin).await
            )
    }
}

pub mod dynamic {
    use actix_web::{ HttpResponse, web, get };
    use actix_web::http::header::{ CacheControl, CacheDirective };

    use crate::cache::Cache;

    #[get("/{type}")]
    pub async fn get_menu_by_type(
        cache: web::Data<Cache>, 
        param: web::Path<String>
    ) -> HttpResponse {
        let dreamin = &cache.data[format!("{}", param)];

        HttpResponse::Ok()
            .content_type("application/json")
            .set(
                CacheControl(
                    vec![
                        CacheDirective::MaxAge(86400u32),
                        CacheDirective::Public
                    ]
                )
            )
            .body(
                super::return_json(dreamin).await
            )
    }

    #[get("/{type}/{menu}")]
    pub async fn get_menu_by_name(
        cache: web::Data<Cache>, 
        param: web::Path<(String, String)>
    ) -> HttpResponse {
        let dreamin = &cache.data
            [format!("{}", param.0)]
            [format!("{}", param.1)];

        HttpResponse::Ok()
            .content_type("application/json")
            .set(
                CacheControl(
                    vec![
                        CacheDirective::MaxAge(86400u32),
                        CacheDirective::Public
                    ]
                )
            )
            .body(
                super::return_json(dreamin).await
            )
    }
}