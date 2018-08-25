use rocket::http::{ContentType, Method, Status, Header};
use rocket::local::Client;
use serde_json;

mod sign_up;

#[test]
fn get_params() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    // sign up
    let details = ::routes::auth::SignUpForm {
        name: "kenton".to_string(),
        server_key: "secret password".to_string(),
        salt: "nacl".to_string(),
        registration_key: "default_reg_key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();
    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::Ok);

    // query the parameters!
    let mut response = client.req(Method::Get, "/auth/params/kenton").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("{\"salt\":\"nacl\"}".to_string()));
}

#[test]
fn sign_in() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    // sign up
    let details = ::routes::auth::SignUpForm {
        name: "kenton".to_string(),
        server_key: "secret password".to_string(),
        salt: "nacl".to_string(),
        registration_key: "default_reg_key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();
    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::Ok);

    // query the parameters!
    let mut response = client.req(Method::Get, "/auth/params/kenton").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("{\"salt\":\"nacl\"}".to_string()));

    // sign in
    let mut response = client.req(Method::Get, "/auth/").header(Header::new("Authorization", "Basic a2VudG9uOnNlY3JldCBwYXNzd29yZA==")).dispatch();
    assert_eq!(response.status(), Status::Ok);
    match response.body_string() {
        Some(body) => {
            assert_eq!(body.starts_with("{\"token\":\""), true);
        },
        None => {
            panic!("Body shouldn't be empty!");
        }
    }
}

#[test]
fn refresh() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    // sign up
    let details = ::routes::auth::SignUpForm {
        name: "kenton".to_string(),
        server_key: "secret password".to_string(),
        salt: "nacl".to_string(),
        registration_key: "default_reg_key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();
    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::Ok);

    // query the parameters!
    let mut response = client.req(Method::Get, "/auth/params/kenton").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("{\"salt\":\"nacl\"}".to_string()));

    // sign in
    #[derive(Debug, Serialize, Deserialize)]
    struct SignInResponse {
        token: String
    }
    let mut response = client.req(Method::Get, "/auth/").header(Header::new("Authorization", "Basic a2VudG9uOnNlY3JldCBwYXNzd29yZA==")).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let token = match response.body_string() {
        Some(body) => {
            assert_eq!(body.starts_with("{\"token\":\""), true);
            let response: SignInResponse = serde_json::from_str(&body).expect("parsing sign-in response");
            response.token
        },
        None => {
            panic!("body shouldn't be empty!");
        }
    };

    // refresh!
    let mut response = client.req(Method::Get, "/auth/refresh").header(Header::new("Authorization", format!("Bearer {}", token))).dispatch();
    assert_eq!(response.status(), Status::Ok);
    match response.body_string() {
        Some(body) => {
            assert_eq!(body.starts_with("{\"token\":\""), true);
            let response: SignInResponse = serde_json::from_str(&body).expect("parsing sign-in response");
            response.token
        },
        None => {
            panic!("body shouldn't be empty!");
        }
    };
}