#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
extern crate serde_json;
extern crate rocket_cors;
extern crate harsh;
#[macro_use]
extern crate log;
extern crate simple_logger;

mod auth;
pub mod config;
mod db;
pub mod errors;
mod routes;
mod responses;
mod util;

use config::Config;

fn setup_server() -> Result<rocket::Rocket, errors::Error> {
    let config: Config = match Config::load("config.toml") {
        Ok(c) => c,
        Err(e) => return Err(errors::Error::ConfigError(e)),
    };

    // TODO: use a config var to set the level
    simple_logger::init()?;

    let db = get_pool();
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
        .mount("/auth", routes![routes::auth::sign_in])
        .catch(errors![routes::errs::not_found, routes::errs::unauthorized, routes::errs::internal_server_error])
    )
}

pub fn run_server() -> Result<(), errors::Error> {
    let server = setup_server()?;
    server.launch();
    Ok(())
}

#[cfg(test)]
mod tests;