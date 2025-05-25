use rocket::{
    Request, http,
    outcome::Outcome::{Error, Success},
    request::{self, FromRequest},
};
use std::env;

pub struct ApiKey(String);

#[derive(Debug)]
pub enum ApiKeyError {
    Invalid,
    NotPresent,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match env::var_os("HAPPIAPI_API_KEY") {
            Some(key) => match req.headers().get_one("Authorization") {
                Some(req_key) => {
                    if req_key == key {
                        Success(ApiKey(req_key.to_owned()))
                    } else {
                        Error((http::Status::Unauthorized, ApiKeyError::Invalid))
                    }
                }
                None => Error((http::Status::Unauthorized, ApiKeyError::NotPresent)),
            },
            None => Error((http::Status::InternalServerError, ApiKeyError::NotPresent)),
        }
    }
}
