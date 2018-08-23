extern crate jsonwebtoken as jwt;

use std;
use harsh::Harsh;
use self::jwt::{decode, encode, Header, Validation};

use super::AuthToken;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    aud: String,
    iat: u64,
    exp: u64
}

pub fn build_token(secret: &str, harsh: &Harsh, uid: u32) -> String {
    let hashed_id = match harsh.encode(&[uid as u64]) {
        Some(v) => v,
        None => String::new()
    };

    let iat = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards!")
        .as_secs();
    // expire every 5 minutes
    let exp = iat + 300;

    let claims = Claims {
        iss: "scribbler".to_owned(),
        aud: "scribbler".to_owned(),
        sub: hashed_id,
        iat,
        exp
    };

    let token = encode(&Header::default(), &claims, secret.as_bytes()).unwrap();
    token
}

pub fn validate_token(secret: &str, harsh: &Harsh, token: &str) -> Result<AuthToken, ()> {
    let mut validation = Validation {
        iss: Some("scribbler".to_string()),
        ..Default::default()
    };
    validation.set_audience(&"scribbler");
    let tok = decode::<Claims>(&token, secret.as_bytes(), &validation);

    match tok {
        Ok(t) => {
            let uid: u32 = match harsh.decode(t.claims.sub) {
                Some(uids) => uids[0] as u32,
                None => return Err(())
            };
            Ok(AuthToken{
                uid: uid
            })
        },
        Err(_) => Err(()),
    }
}
