use rocket::Request;

#[error(404)]
pub fn not_found(_: &Request) -> String {
    String::from("")
}

#[error(401)]
pub fn unauthorized(_: &Request) -> String {
    String::from("")
}

#[error(500)]
pub fn internal_server_error(_: &Request) -> String {
    String::from("")
}
