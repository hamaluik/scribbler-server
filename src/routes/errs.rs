use rocket::Request;

#[catch(404)]
pub fn not_found(_: &Request) -> String {
    String::from("")
}

#[catch(401)]
pub fn unauthorized(_: &Request) -> String {
    String::from("")
}

#[catch(500)]
pub fn internal_server_error(_: &Request) -> String {
    String::from("")
}
