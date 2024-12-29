use actix_web::{App, HttpServer};
use handler::handler::ping_handler;
 
mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let server = HttpServer::new(|| App::new().service(ping_handler))
        .bind(("127.0.0.1", 8080))?
        .run();
 
    println!("Server running at http://localhost:8080 :3");
 
    server.await
}