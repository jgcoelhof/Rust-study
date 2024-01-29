use actix_web::*;
use routes::{info, ping,ctlg};

mod routes;

#[actix_web::main]

async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .route("/info", web::get().to(info))
            .route("/cat",web::get().to(ctlg))
    });
    let porta: i32 = 9091;
    let api = api
        .bind(format!("127.0.0.1:{}", porta))
        .expect("Não conseguiu conectar...");

    println!("Conectado com sucesso!\n http://localhost:{}/ping", porta);
    
    api.run().await
}