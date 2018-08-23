use rocket::response::Responder;
use rocket::Request;
use rocket::Response;
use rocket::http::Status;

#[derive(Debug)]
pub struct EmptyOK();

impl<'r> Responder<'r> for EmptyOK {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
        Response::build()
            .ok()
    }
}
