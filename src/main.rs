use actix_web::{web, App, HttpServer};
use error_chain::error_chain;
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use tracing_subscriber::{filter, fmt, prelude::*, reload};

mod common;
mod controller;
mod model;

use crate::controller::{openapi, user_controller};

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    let filter = filter::LevelFilter::DEBUG;
    let (filter, _reload_handle) = reload::Layer::new(filter);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();

    let rb = Rbatis::new();
    rb.init(MysqlDriver {}, "mysql://root:root@localhost:3306/trauma")
        .unwrap();

    let state = common::AppState { pool: rb };

    tracing::info!("Server is running on http://{}:{}", "127.0.0.1", 9991);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(user_controller::register_routes)
            .configure(openapi::init)
    })
    .workers(5)
    .bind(("127.0.0.1", 9991))?
    .run();

    _ = server.await;

    Ok(())
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(controller::api_routes());
}
