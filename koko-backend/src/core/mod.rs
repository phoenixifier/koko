use actix_web::http::StatusCode;
use actix_web::HttpResponse;

pub mod helper;
pub mod routes;

// pub static MAIN_URL: &str = "koko.moe";

pub async fn not_found() -> HttpResponse {
    HttpResponse::build(StatusCode::FOUND)
        .append_header(("Location", "https://api.maid.uz/404"))
        .finish()
}
