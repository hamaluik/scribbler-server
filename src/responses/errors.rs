use rocket::response::Responder;
use rocket::Request;
use rocket::Response;
use rocket::http::Status;

#[derive(Debug)]
pub enum ErrorResponses {
    NotFound,
    Unauthorized,
    NotImplemented,
    InternalServerError,
    ServiceUnavailable,
    BadRequest,
    Conflict
}

impl<'r> Responder<'r> for ErrorResponses {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
        Response::build()
            .status(match self {
                ErrorResponses::NotFound => Status::NotFound,
                ErrorResponses::Unauthorized => Status::Unauthorized,
                ErrorResponses::NotImplemented => Status::NotImplemented,
                ErrorResponses::InternalServerError => Status::InternalServerError,
                ErrorResponses::ServiceUnavailable => Status::ServiceUnavailable,
                ErrorResponses::BadRequest => Status::BadRequest,
                ErrorResponses::Conflict => Status::Conflict
            })
            .ok()
    }
}