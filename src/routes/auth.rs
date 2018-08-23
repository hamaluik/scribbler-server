use harsh::Harsh;
use rocket::State;
use rocket_contrib::{Json, Value};

use auth::build_token;
use auth::AuthBasic;
use config::Config;

#[get("/")]
pub fn sign_in(config: State<Config>, harsh: State<Harsh>, auth: AuthBasic) -> Json<Value> {
    Json(json!({
        "token": build_token(&config.jwt.secret, &harsh, auth.uid)
    }))
}
