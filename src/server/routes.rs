use actix_web::{HttpRequest, Responder, HttpResponse};

#[actix_web::get("/")]
pub(super) async fn serve_index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body(include_str!("../../labs/public/index.html"))
}

#[actix_web::get("/bundle.js")]
pub(super) async fn serve_script(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body(include_str!("../../labs/public/bundle.js"))
}

#[actix_web::get("/bundle.css")]
pub(super) async fn serve_style(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body(include_str!("../../labs/public/bundle.css"))
}

#[actix_web::get("/assets/favicon.png")]
pub(super) async fn serve_favicon(_req: HttpRequest) -> impl Responder {
    const ICON: &[u8] = include_bytes!("../../labs/public/assets/favicon.png");
    HttpResponse::Ok().body(ICON)
}