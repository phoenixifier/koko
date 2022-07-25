#[macro_use]
extern crate diesel;

mod core;
mod schema;
mod users;

use crate::core::helper::{db_target, target};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let manager = ConnectionManager::<PgConnection>::new(db_target());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    log::info!("Running server on http://{}", target());

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(core::routes::index)
            .service(users::routes::get_user)
            .service(users::routes::add_user)
            .default_service(web::route().to(core::not_found))
    })
    .bind(target())?
    .run()
    .await
}
