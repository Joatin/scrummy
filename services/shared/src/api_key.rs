use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};


pub struct ApiKey(String);

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str) -> bool {
    key == "valid_api_key"
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];
        if !is_valid(keys[0]) {
            return Outcome::Forward(());
        }

        return Outcome::Success(ApiKey(key.to_string()));
    }
}