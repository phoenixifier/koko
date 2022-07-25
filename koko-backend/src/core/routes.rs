use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "https://api.maid.uz"))
        .finish()
}
