pub mod emptyok;
pub mod errors;

pub use self::emptyok::EmptyOK;
pub use self::errors::ErrorResponses;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthParams {
    pub salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpForm {
    pub name: String,
    pub server_key: String,
    pub salt: String,
    pub registration_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: Option<String>,
    pub version: u64,
    pub content: String,
    pub nonce: String
}
