mod routes;

use actix_web::{HttpServer, App};

pub async fn run() {
    let port = pick_port();

    let http = HttpServer::new(|| {
        App::new()
        .service(routes::serve_index)
        .service(routes::serve_script)
        .service(routes::serve_style)
        .service(routes::serve_favicon)
    })
    .bind(("127.0.0.1", port)).unwrap();

    println!("Server running on http://localhost:{}", port);
    http.run().await.unwrap()
}

fn pick_port() -> u16 {
    if portpicker::is_free(8080) {
        return 8080
    }
    else {
        portpicker::pick_unused_port().unwrap()
    }
}