use harsh::Harsh;
use rocket::State;
use rocket_contrib::{Json, Value};

use auth::build_token;
use auth::AuthBasic;
use config::Config;

#[get("/params/<name>")]
pub fn params(name: &String, config: State<Config>, harsh: State<Harsh>) -> Result<Json<Value>, ErrorResponses> {
    Err(ErrorResponses::NotImplemented)
}

#[get("/")]
pub fn sign_in(config: State<Config>, harsh: State<Harsh>, auth: AuthBasic) -> Json<Value> {
    Json(json!({
        "token": build_token(&config.jwt.secret, &harsh, auth.uid)
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpForm {
    pub name: String,
    pub key: String,
    pub salt: String,
    pub registration_key: String,
}

#[post("/", data="<form>")]
pub fn sign_up(form: Json<SignUpForm>, config: State<Config>, conn: DbConn, harsh: State<Harsh>) -> Result<Json<Value>, ErrorResponses> {
    Err(ErrorResponses::NotImplemented)
}
