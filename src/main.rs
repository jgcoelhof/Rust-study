mod the_rust_programming_language;
mod rust_by_pratice;
mod actix_api;
mod teste_actix;
use actix_web::*;
use teste_actix::routes::ping;
#[actix_web::main]

pub async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new(|| App::new().route("/ping", web::get().to(ping)));
    let porta = 9091;
    let api = api
        .bind(format!("127.0.0.1: {}", porta))
        .expect("Não conseguiu conectar...");
    println!("Conectado com sucesso:\n https://localhost: {}/ping", porta);
    api.run().await
}
