use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn main() {
    info!("Started web server thread");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(fs::Files::new("/", "./public_html").show_files_listing())
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
