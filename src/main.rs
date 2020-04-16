use std::{io};
use std::fs::File;
use std::io::Read;

use actix_web::{ HttpServer, App, middleware, web };

mod cache;
mod route;
mod error;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        let mut file = File::open("static/dreamin.json").expect("File not found");
        let mut dreamin = String::new();
    
        file.read_to_string(&mut dreamin).expect("Unable to read file");

        let data: serde_json::Value = serde_json::from_str(&dreamin.to_string()).expect("Not a valid json");

        App::new()
            .wrap(
                middleware::Compress::default()
            )
            .data(cache::Cache {
                data: data
            })
            .service(route::common::get_all_menu)
            .service(route::dynamic::get_menu_by_type)
            .service(route::dynamic::get_menu_by_name)
            .default_service(
                web::resource("").to(error::http_error::not_found)
            )
    })
    .bind("0.0.0.0:80")
    .expect("Can't start server")
    .run()
    .await
}