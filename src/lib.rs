#![feature(plugin, custom_derive, extern_prelude)]
#![plugin(rocket_codegen)]
// TODO: disable only for tests?
//#![allow(dead_code, unused_imports)]

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
mod communication;
pub mod r2d2_sqlite;

use config::{Config, JWTConfig, HashIDSConfig};

#[cfg(not(test))]
fn get_rocket_config(_config: &Config) -> rocket::Config {
    rocket::config::ConfigBuilder::new(rocket::config::Environment::Development)
        .finalize()
        .unwrap()
}

#[cfg(test)]
fn get_rocket_config(_config: &Config) -> rocket::Config {
    rocket::config::ConfigBuilder::new(rocket::config::Environment::Development)
        .log_level(rocket::config::LoggingLevel::Critical)
        .finalize()
        .unwrap()
}

#[cfg(not(test))]
fn initialize_logger() {
    // TODO: use a config var to set the level
    //simple_logger::init().expect("simple_logger init");
    simple_logger::init_with_level(log::Level::Info).expect("simple_logger init");
}

#[cfg(test)]
fn initialize_logger() {}

fn setup_server() -> Result<rocket::Rocket, errors::Error> {
    initialize_logger();

    let config: Config = match Config::load("config.toml") {
        Ok(c) => c,
        Err(e) => {
            // use a default config
            warn!("Using default config due to error: {:?}", e);
            Config {
                registration_key: "default_reg_key".to_string(),
                jwt: JWTConfig {
                    secret: "default_secret".to_string(),
                },
                hashids: HashIDSConfig {
                    salt: "sefault_salt".to_string(),
                    alphabet: "abcdefghijklmnopqrstuvwxyz012345679".to_string(),
                    min_length: 16,
                }
            }
            //return Err(errors::Error::ConfigError(e))
        },
    };

    let db = db::pool::get_database_pool("scribbler.db");
    db::setup::initialize_tables(&db)?;

    let harsh = harsh::HarshBuilder::new()
        .alphabet(config.hashids.alphabet.clone().as_bytes())
        .salt(config.hashids.salt.clone().as_bytes())
        .length(config.hashids.min_length)
        .init()?;

    let cors = rocket_cors::Cors::default();
    
    let rocket_config = get_rocket_config(&config);

    Ok(rocket::custom(rocket_config, false)
        .manage(config)
        .manage(db)
        .manage(harsh)
        .attach(cors)
        .mount("/auth", routes![routes::auth::params, routes::auth::sign_in, routes::auth::refresh, routes::auth::sign_up])
        .mount("/items", routes![routes::items::get_all_items, routes::items::create_item, routes::items::update_item])
        .catch(catchers![routes::errs::not_found, routes::errs::unauthorized, routes::errs::internal_server_error])
    )
}

pub fn run_server() -> Result<(), errors::Error> {
    let server = setup_server()?;
    server.launch();
    Ok(())
}
