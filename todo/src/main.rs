#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;

use std::{env, io};

use actix_files as fs;
use actix_redis::RedisSession;
// use actix_session::CookieSession;
use actix_web::middleware::{errhandlers::ErrorHandlers, Logger};
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use tera::Tera;

mod api;
mod db;
mod model;
mod schema;
mod session;

// static SESSION_SIGNING_KEY: &[u8] = &[0; 32];

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    env::set_var(
        "RUST_LOG",
        "actix_todo=debug,actix_web=info,actix_redis=info",
    );
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).expect("Failed to create pool");

    let app = move || {
        debug!("Constructing the App");

        let templates: Tera = compile_templates!("templates/**/*");

        // let session_store = CookieSession::signed(SESSION_SIGNING_KEY).secure(false);
        let redis_host = env::var("REDIS_HOST").unwrap();
        let redis_port = env::var("REDIS_PORT").unwrap();
        let redis_url = format!("{}:{}", redis_host, redis_port);
        let session_store = RedisSession::new(redis_url, &[0; 32]);
        let error_handlers = ErrorHandlers::new()
            .handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                api::internal_server_error,
            )
            .handler(http::StatusCode::BAD_REQUEST, api::bad_request)
            .handler(http::StatusCode::NOT_FOUND, api::not_found);

        App::new()
            .data(templates)
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(session_store)
            .wrap(error_handlers)
            .service(web::resource("/").route(web::get().to(api::index)))
            .service(web::resource("/todo").route(web::post().to(api::create)))
            .service(web::resource("/todo/{id}").route(web::post().to(api::update)))
            .service(fs::Files::new("/static", "static/"))
    };

    debug!("Starting server");
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(app);
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        println!("starting server");
        server.listen(l).unwrap()
    } else {
        let port = env::var("PORT").unwrap();
        let bind_config = format!("0.0.0.0:{}", port);
        println!("listening on {}", bind_config);
        server.bind(bind_config).unwrap()
    };
    server.start().await
}
