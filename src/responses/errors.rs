use rocket::response::Responder;
use rocket::Request;
use rocket::Response;
use rocket::http::Status;

#[derive(Debug)]
pub enum ErrorResponses {
    _NotFound,
    Unauthorized,
    _NotImplemented,
    InternalServerError,
    _ServiceUnavailable,
    _BadRequest,
    Conflict
}

impl<'r> Responder<'r> for ErrorResponses {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
        Response::build()
            .status(match self {
                ErrorResponses::_NotFound => Status::NotFound,
                ErrorResponses::Unauthorized => Status::Unauthorized,
                ErrorResponses::_NotImplemented => Status::NotImplemented,
                ErrorResponses::InternalServerError => Status::InternalServerError,
                ErrorResponses::_ServiceUnavailable => Status::ServiceUnavailable,
                ErrorResponses::_BadRequest => Status::BadRequest,
                ErrorResponses::Conflict => Status::Conflict
            })
            .ok()
    }
}