use ErrorType;
use rocket::response::Responder;
use rocket::Request;
use rocket::http::Status;
use rocket::Response;
use rocket::http::ContentType;
use std::io::Cursor;
use serde_json::ser::to_string;
use std::mem::replace;


#[derive(Debug)]
pub struct Error {
  error_type: ErrorType,
  message: String
}

impl Error {
  pub fn get_error_type(&self) -> &ErrorType {
    &self.error_type
  }
  pub fn get_message(&self) -> &str {
    &self.message
  }

  pub fn unknown() -> Error {
    Error {
      error_type: ErrorType::Unknown,
      message: "Unknown error occurred".to_string()
    }
  }

  pub fn aggregate_already_exists() -> Error {
    Error {
      error_type: ErrorType::AggregateExists,
      message: "An aggregate with this id already exist".to_string()
    }
  }
}

impl<'r> Responder<'r> for Error {
  fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
    Response::build()
      .header(ContentType::JSON)
      .sized_body(Cursor::new(json!({ "status": "fail", "error": to_string(self.get_error_type()).unwrap().replace("\"", ""), "message": self.get_message() }).to_string()))
      .status(Status::BadRequest)
      .ok()
  }
}
