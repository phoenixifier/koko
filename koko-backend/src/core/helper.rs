use std::env;

pub fn target() -> String {
    // Define the target of host
    let target = format!(
        "{}:{}",
        match env::var("HOST") {
            Ok(host) => host,
            Err(_) => "127.0.0.1".to_owned(),
        },
        match env::var("PORT") {
            Ok(port) => port,
            Err(_) => 9000.to_string(),
        }
    );
    target
}

pub fn db_target() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL")
}
