use harsh::Harsh;
use rocket::State;
use rocket_contrib::{Json, Value};

use auth::build_token;
use auth::AuthBasic;
use auth::AuthToken;
use config::Config;
use communication::EmptyOK;
use communication::ErrorResponses;
use communication::SignUpForm;
use db::DbConn;

#[get("/params/<name>")]
pub fn params(name: String, conn: DbConn) -> Result<Json<Value>, ErrorResponses> {
    let mut stmt = conn.prepare("select salt from users where name=?1")
        .expect("prepare select");
    let salt: String = match stmt.query_row(&[&name], |row| {
        row.get(0)
    }) {
        Ok(data) => data,
        Err(e) => {
            debug!("failed to find user: {:?}", e);
            return Err(ErrorResponses::Unauthorized);
        }
    };
    Ok(Json(json!({
        "salt": salt
    })))
}

#[get("/")]
pub fn sign_in(config: State<Config>, harsh: State<Harsh>, auth: AuthBasic) -> Json<Value> {
    Json(json!({
        "token": build_token(&config.jwt.secret, &harsh, auth.uid)
    }))
}

#[get("/refresh")]
pub fn refresh(config: State<Config>, harsh: State<Harsh>, auth: AuthToken) -> Json<Value> {
    Json(json!({
        "token": build_token(&config.jwt.secret, &harsh, auth.uid)
    }))
}

#[post("/", data="<form>")]
pub fn sign_up(form: Json<SignUpForm>, config: State<Config>, conn: DbConn) -> Result<EmptyOK, ErrorResponses> {
    if form.name.trim().len() == 0 {
        warn!("attempted to sign up with empty name");
        return Err(ErrorResponses::BadRequest);
    }

    if form.server_key.trim().len() == 0 {
        warn!("attempted to sign up with empty server_key");
        return Err(ErrorResponses::BadRequest);
    }

    if form.salt.trim().len() == 0 {
        warn!("attempted to sign up with empty salt");
        return Err(ErrorResponses::BadRequest);
    }

    if form.registration_key.trim().len() == 0 {
        warn!("attempted to sign up with empty registration_key");
        return Err(ErrorResponses::BadRequest);
    }

    // make sure the registration key is valid
    if form.registration_key != config.registration_key {
        warn!("Invalid registration key");
        return Err(ErrorResponses::Unauthorized);
    }

    // make sure that name doesn't already exist
    let mut stmt = conn.prepare("select count(id) from users where name=?1")
        .expect("prepare select");
    let count: u32 = match stmt.query_row(&[&form.name], |row| {
        row.get(0)
    }) {
        Ok(data) => data,
        Err(e) => {
            error!("failed to query database: {:?}", e);
            return Err(ErrorResponses::InternalServerError);
        }
    };

    if count > 0 {
        warn!("User already exists!");
        return Err(ErrorResponses::Unauthorized);
    }

    // TODO: validate that the data is base64!

    // ok, name is good, let's create the user
    let mut stmt = conn.prepare("insert into users(name, server_key, salt) values(?1, ?2, ?3)")
        .expect("prepare statement");
    match stmt.execute(&[&form.name, &form.server_key, &form.salt]) {
        Ok(affected_rows) => {
            if affected_rows != 1 {
                warn!("Failed to register user! (No changed rows)");
                return Err(ErrorResponses::InternalServerError);
            }
        },
        Err(e) => {
            warn!("Failed to register user: {:?}", e);
            return Err(ErrorResponses::InternalServerError);
        }
    };

    // great, we inserted!
    Ok(EmptyOK())
}
