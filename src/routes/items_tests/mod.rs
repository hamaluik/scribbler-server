use rocket::http::{ContentType, Method, Status, Header};
use rocket::local::Client;
use serde_json;

use communication::SignInResponse;
use communication::ID;

#[test]
fn create_item() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    // sign up
    let details = ::communication::SignUpForm {
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
    let auth = Header::new("Authorization", format!("Bearer {}", token));

    // create it!
    let details = ::communication::Item {
        id: None,
        version: 123456,
        content: "The content that I am creating!".to_string(),
        nonce: "The nonce we used!".to_string()
    };
    let form = serde_json::to_string(&details).unwrap();
    let response = client.req(Method::Post, "/items/").header(ContentType::JSON).header(auth).body(form).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn update_item() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    // sign up
    let details = ::communication::SignUpForm {
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

    // create it!
    let auth = Header::new("Authorization", format!("Bearer {}", token));
    let details = ::communication::Item {
        id: None,
        version: 123456,
        content: "The content that I am creating!".to_string(),
        nonce: "The nonce we used!".to_string()
    };
    let form = serde_json::to_string(&details).unwrap();
    let mut response = client.req(Method::Post, "/items").header(ContentType::JSON).header(auth).body(form).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let item_id = match response.body_string() {
        Some(body) => {
            let response: ID = serde_json::from_str(&body).expect("parsing create response");
            response.id
        },
        None => {
            panic!("body shouldn't be empty!");
        }
    };

    // update it!
    let auth = Header::new("Authorization", format!("Bearer {}", token));
    let details = ::communication::Item {
        id: Some(item_id.clone()),
        version: 313213,
        content: "Some updated content".to_string(),
        nonce: "a new nonce".to_string()
    };
    let form = serde_json::to_string(&details).unwrap();
    let response = client.req(Method::Patch, format!("/items/{}", item_id)).header(ContentType::JSON).header(auth).body(form).dispatch();
    assert_eq!(response.status(), Status::Ok);
}