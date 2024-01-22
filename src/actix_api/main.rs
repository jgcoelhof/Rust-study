use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize};
use crate::actix_api;


#[derive(Serialize)]
pub struct Response {
    pub message: String,
    pub status: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        status: "failed".to_string(),
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

// será chamada se o cliente solicitar um recurso que não esteja registrado no servidor.
async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        status: "success".to_string(),
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
pub(crate) async fn main() -> std::io::Result<()> {
    //Cria um novo servidor usando a estrutura HttpServer.
    // A estrutura HttpServer usa um encerramento para atender quaisquer solicitações recebidas usando a instância App.
    // A instância App é usada para registrar todas as rotas que o servidor deve manipular.
    let todo_db = actix_api::repository::database::Database::new();
    let app_data = web::Data::new(todo_db);

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .configure(actix_api::api::api::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

