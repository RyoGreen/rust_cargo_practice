use actix_web::{web, App, HttpServer};

mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(server::handler::hello)
            .service(server::handler::echo)
            .route("/hey", web::get().to(server::handler::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
