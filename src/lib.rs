#![feature(plugin, custom_derive, extern_prelude)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate harsh;
extern crate rusqlite;
extern crate serde_json;
extern crate toml;
extern crate rocket_cors;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate r2d2;

mod auth;
pub mod config;
mod db;
pub mod errors;
mod routes;
mod responses;
mod r2d2_sqlite;

use config::Config;

fn setup_server() -> Result<rocket::Rocket, errors::Error> {
    let config: Config = match Config::load("config.toml") {
        Ok(c) => c,
        Err(e) => return Err(errors::Error::ConfigError(e)),
    };

    // TODO: use a config var to set the level
    simple_logger::init()?;

    let db = db::pool::get_database_pool("scribbler.db");
    db::setup::initialize_tables(&db)?;

    let harsh = harsh::HarshBuilder::new()
        .alphabet(config.hashids.alphabet.clone().as_bytes())
        .salt(config.hashids.salt.clone().as_bytes())
        .length(config.hashids.min_length)
        .init()?;

    let cors = rocket_cors::Cors::default();

    Ok(rocket::ignite()
        .manage(config)
        .manage(db)
        .manage(harsh)
        .attach(cors)
        .mount("/auth", routes![routes::auth::params, routes::auth::sign_in, routes::auth::refresh, routes::auth::sign_up])
        .catch(catchers![routes::errs::not_found, routes::errs::unauthorized, routes::errs::internal_server_error])
    )
}

pub fn run_server() -> Result<(), errors::Error> {
    let server = setup_server()?;
    server.launch();
    Ok(())
}
