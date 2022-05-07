mod server;

#[actix_web::main]
async fn main() {
    server::run().await;
}
