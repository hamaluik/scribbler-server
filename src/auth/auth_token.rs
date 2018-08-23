use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use harsh::Harsh;

use ::config::Config;
use ::auth::validate_token;

#[derive(Debug)]
pub struct AuthToken {
    pub uid: u32,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthToken, ()> {
        let auths: Vec<_> = request.headers().get("Authorization").collect();
        if auths.len() != 1 {
            warn!("No authorization header");
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        let auth: Vec<&str> = auths[0].split(' ').collect();
        if auth.len() != 2 || auth[0] != "Bearer" {
            warn!("Auth is not a bearer: is '{}'", auth[0]);
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        let token = auth[1];

        let config = request.guard::<State<Config>>()?;
        let harsh = request.guard::<State<Harsh>>()?;
        let token = match validate_token(&config.jwt.secret, &harsh, &token) {
            Ok(tok) => tok,
            Err(_) => {
                println!("Invalid token: {}", token);
                return Outcome::Failure((Status::Unauthorized, ()));
            }
        };

        Outcome::Success(token)
    }
}