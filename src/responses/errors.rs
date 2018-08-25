use rocket::response::Responder;
use rocket::Request;
use rocket::Response;
use rocket::http::Status;

#[derive(Debug)]
pub enum ErrorResponses {
    Unauthorized,
    InternalServerError,
    BadRequest,
}

impl<'r> Responder<'r> for ErrorResponses {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
        Response::build()
            .status(match self {
                ErrorResponses::Unauthorized => Status::Unauthorized,
                ErrorResponses::InternalServerError => Status::InternalServerError,
                ErrorResponses::BadRequest => Status::BadRequest,
            })
            .ok()
    }
}