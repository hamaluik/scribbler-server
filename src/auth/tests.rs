use rocket;
use rocket::http::{Accept, ContentType, Header, MediaType, Method, Status};
use rocket::local::Client;
use serde_json;

fn test<H>(method: Method, uri: &str, header: H, expected_status: Status, expected_body: String)
where
    H: Into<Header<'static>>,
{
    let rocket = ::setup_server().expect("server setup");

    let client = Client::new(rocket).unwrap();
    let mut response = client.req(method, uri).header(header).dispatch();
    assert_eq!(response.status(), status);
    assert_eq!(response.body_string(), Some(expected_body));
}

fn test_form<H, B>(method: Method, uri: &str, header: H, body: B, expected_status: Status, expected_body: String)
where
    H: Into<Header<'static>>,
{
    let rocket = ::setup_server().expect("server setup");

    let client = Client::new(rocket).unwrap();
    let mut response = client.req(method, uri).header(header).set_body(body).dispatch();
    assert_eq!(response.status(), status);
    assert_eq!(response.body_string(), Some(expected_body));
}

#[test]
fn sign_up_invalid_registration_code() {
    let details = ::routes::auth::SignUpForm {
        name: "kenton".to_string(),
        server_key: "I am a server key!".to_string(),
        salt: "I am the salt!".to_string(),
        registration_key: "a completely, totally invalid registration key".to_string(),
    };
    let form = serde_json::to_string(&details).unwrap();
    test_form(
        Method::Post,
        "/auth/",
        ContentType::JSON,
        form,
        Status::Unauthorized,
        "".to_string(),
    );
}
