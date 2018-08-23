pub mod emptyok;
pub mod errors;

pub use self::emptyok::EmptyOK;
pub use self::errors::ErrorResponses;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthParams {
    pub salt: String,
}
