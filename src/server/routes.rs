use actix_web::{Responder, HttpResponse, get, web};

#[get("/theme/{theme_name}")]
pub(super) async fn retrieve_theme(path: web::Path<String>) -> impl Responder {
    use std::fs::read_to_string;
    
    let theme_name = path.into_inner() + ".css";

    println!("{}", theme_name);


    let content = match read_to_string(theme_name) {
        Ok(data) => data,
        Err(_) => include_str!("../../themes/midnight.css").to_string(),
    };


    HttpResponse::Ok()
        .content_type("text/css")
        .body(content)
}