use std::env;

use actix_web::{
    App,
    HttpServer,
    middleware::Logger,
};

extern crate env_logger;

mod action;

fn init_logger()
{
    //logger setting
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    let args: Vec<String> = env::args().collect();

    let host = &args[1];
    let port = &args[2];

    init_logger();

    let _ = listenfd::ListenFd::from_env();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(r#"requests:"%r" status:%s elapsed:%Dms"#))
            .service(action::create_doc)
            .service(action::read_doc)
            .service(actix_files::Files::new("/", "/static").show_files_listing())
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}