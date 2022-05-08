mod routes;

use actix_web::{HttpServer, App};

pub async fn run() {
    let port = pick_port();
    
    let http = HttpServer::new(|| {
        let app = App::new()
            .service(routes::retrieve_theme);
            
        page_loader::route_directory!(app, "web/dist")

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