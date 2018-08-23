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

/*mod auth;
pub mod config;
mod db;
pub mod errors;
mod routes;
mod responses;
mod util;

use config::Config;*/

fn setup_server() -> Result<rocket::Rocket, errors::Error> {
    /*let config: Config = match Config::load("config.toml") {
        Ok(c) => c,
        Err(e) => return Err(errors::Error::ConfigError(e)),
    };

    let db = get_pool();
    db::setup::initialize_tables(&db)?;

    let cors = rocket_cors::Cors::default();*/

    Ok(rocket::ignite()
        /*.manage(config)
        .manage(db)
        .manage(harsh)
        .attach(cors)
        .mount("/auth", routes![routes::auth::sign_in])
        .mount("/profile", routes![routes::profile::get_profile, routes::profile::update_name, routes::profile::update_picture, routes::profile::update_pass])
        .mount("/timetracking", routes![routes::timetracking::start_tracking, routes::timetracking::stop_tracking, routes::timetracking::list_running_timers])
        .mount("/trackedtimes", routes![routes::trackedtimes::get_tracked_times])
        .mount("/projects", routes![routes::projects::create_project, routes::projects::get_projects])
        .mount("/project", routes![routes::projects::get_project, routes::projects::update_project, routes::projects::delete_project])
        .catch(errors![routes::errs::not_found, routes::errs::unauthorized, routes::errs::internal_server_error])*/
    )
}

pub fn run_server() -> Result<(), errors::Error> {
    let server = setup_server()?;
    server.launch();
    Ok(())
}

pub fn create_user(email: &str, name: &str, pass: &str) -> Result<u32, errors::Error> {
    let db = get_pool();
    db::setup::initialize_tables(&db)?;
    db::setup::register_user(&db, email, name, pass)
}

#[cfg(test)]
mod tests;