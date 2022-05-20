extern crate diesel;

use actix_web::{web, App, HttpServer};
use messenger::{handlers, db, Config};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();
    let pool = db::get_connection_pool(config.db_host);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::list_users)
            .service(handlers::inner_messages)
            .service(handlers::create_message)
            .route("/ws/", web::get().to(handlers::websocket))
    })
        .bind((config.host, config.port))?
        .run()
        .await
}
