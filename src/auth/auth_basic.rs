use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use base64;
use bcrypt;

use ::db::DatabasePool;

#[derive(Debug)]
pub struct AuthBasic {
    pub uid: u32,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthBasic {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let auths: Vec<_> = request.headers().get("Authorization").collect();
        if auths.len() != 1 {
            debug!("too many auth headers!");
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        debug!("auth: {}", auths[0]);
        let auth: Vec<&str> = auths[0].split(' ').collect();
        if auth.len() != 2 || auth[0] != "Basic" {
            debug!("not basic auth");
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let auth = auth[1];
        let auth = match base64::decode(&auth) {
            Ok(a) => a,
            Err(_) => {
                debug!("failure to decode base64");
                return Outcome::Failure((Status::Unauthorized, ()));
            }
        };
        let auth: String = match String::from_utf8(auth) {
            Ok(a) => a,
            Err(_) => {
                debug!("failed to decode utf8 version");
                return Outcome::Failure((Status::Unauthorized, ()));
            }
        };

        debug!("auth: {}", auth);
        let auth: Vec<&str> = auth.split(':').collect();
        if auth.len() < 2 {
            debug!("auth isn't of form name:server_key");
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        let name: &str = auth[0];
        let server_key: &str = &auth[1..].join(":");

        debug!("name: {}, server_key: {}", name, server_key);
        let pool = request.guard::<State<DatabasePool>>()?;
        let conn = match pool.get() {
            Ok(c) => c,
            Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ()))
        };

        let mut stmt = conn.prepare("select id from users where name=?1 and server_key=?2")
            .expect("prepare select");
        let uid: u32 = match stmt.query_row(&[&name], |row| {
            row.get(0)
        }) {
            Ok(data) => data,
            Err(e) => {
                debug!("failed to find user: {:?}", e);
                return Outcome::Failure((Status::Unauthorized, ()));
            }
        };
        Outcome::Success(AuthBasic { uid })
    }
}