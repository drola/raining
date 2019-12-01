use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index(data: web::Data<super::SharedAppState>) -> impl Responder {
    HttpResponse::Ok()
        .header(actix_web::http::header::CONTENT_TYPE, "text/html; charset=UTF-8")
        .body((*data.index_html.read().unwrap()).clone())
}

pub fn main(shared_app_state: super::SharedAppState) {
    info!("Started web server thread");
    let app_data = web::Data::new(shared_app_state.clone());
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Compress::default())
            .register_data(app_data.clone())
            .route("/", web::get().to(index))
            .service(fs::Files::new("/", "./public_html").show_files_listing())
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
    shared_app_state.running.store(false, std::sync::atomic::Ordering::Relaxed);
}
