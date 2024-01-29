use actix_web::*;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("Conectado...")
}
pub async fn info() -> HttpResponse {
    
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(format!("versão: {}<br> name: {}",VERSION,NAME))
}
pub async fn ctlg() -> HttpResponse{
    HttpResponse::Ok()
    .content_type("application/json; charset=utf-8")
    .body(r#"[
        {"mensagem" :"Olá", "author":"email@gmail.com"}        
        {"mensagem" :"Olá mundo", "author":"outroemail@gmail.com"}
    ]"#)

}