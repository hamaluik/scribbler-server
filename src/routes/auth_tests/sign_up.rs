use rocket::http::{ContentType, Method, Status};
use rocket::local::Client;
use serde_json;

#[test]
fn sign_up() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    let details = ::communication::SignUpForm {
        name: "kenton".to_string(),
        server_key: "I am a server key!".to_string(),
        salt: "I am the salt!".to_string(),
        registration_key: "default_reg_key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();
    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn sign_up_missing_name() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    #[derive(Debug, Serialize, Deserialize)]
    struct MissingDetails {
        server_key: String,
        salt: String,
        registration_key: String
    };
    let details = MissingDetails {
        server_key: "server key!!".to_string(),
        salt: "I am the salt!".to_string(),
        registration_key: "a completely, totally invalid registration key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();

    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn sign_up_missing_server_key() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    #[derive(Debug, Serialize, Deserialize)]
    struct MissingDetails {
        name: String,
        salt: String,
        registration_key: String
    };
    let details = MissingDetails {
        name: "kenton".to_string(),
        salt: "I am the salt!".to_string(),
        registration_key: "a completely, totally invalid registration key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();

    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn sign_up_missing_salt() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    #[derive(Debug, Serialize, Deserialize)]
    struct MissingDetails {
        name: String,
        server_key: String,
        registration_key: String
    };
    let details = MissingDetails {
        name: "kenton".to_string(),
        server_key: "I am a server key!".to_string(),
        registration_key: "a completely, totally invalid registration key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();

    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn sign_up_invalid_registration_code() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    let details = ::communication::SignUpForm {
        name: "kenton".to_string(),
        server_key: "I am a server key!".to_string(),
        salt: "I am the salt!".to_string(),
        registration_key: "a completely, totally invalid registration key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();

    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn sign_up_empty_name() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    let details = ::communication::SignUpForm {
        name: "".to_string(),
        server_key: "server_key".to_string(),
        salt: "salt".to_string(),
        registration_key: "default_reg_key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();

    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn sign_up_empty_server_key() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    let details = ::communication::SignUpForm {
        name: "kenton".to_string(),
        server_key: " ".to_string(),
        salt: "salt".to_string(),
        registration_key: "default_reg_key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();

    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn sign_up_empty_salt() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    let details = ::communication::SignUpForm {
        name: "kenton".to_string(),
        server_key: "server_key".to_string(),
        salt: "  ".to_string(),
        registration_key: "default_reg_key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();

    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn sign_up_already_exists() {
    // setup the server and client
    let rocket = ::setup_server().expect("server setup");
    let client = Client::new(rocket).unwrap();

    // submit the first time
    let details = ::communication::SignUpForm {
        name: "kenton".to_string(),
        server_key: "I am a server key!".to_string(),
        salt: "I am the salt!".to_string(),
        registration_key: "default_reg_key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();
    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::Ok);

    // resubmit!
    let details = ::communication::SignUpForm {
        name: "kenton".to_string(),
        server_key: "A different key".to_string(),
        salt: "But the same salt".to_string(),
        registration_key: "default_reg_key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();
    let response = client.req(Method::Post, "/auth/").header(ContentType::JSON).body(form).dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}
