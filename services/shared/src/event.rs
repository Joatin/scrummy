
extern crate bson;
extern crate mongodb;

use chrono::{DateTime, Utc};
use std::borrow::Borrow;
use serde::Serialize;
use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event<T: Serialize + Sized> {
  #[serde(rename = "_id")]
  id: bson::oid::ObjectId,
  index: i64,
  aggregate_id: String,
  timestamp: DateTime<Utc>,
  #[serde(flatten)]
  payload: T
}

impl<T: Serialize + for<'a> Deserialize<'a> + Sized> Event<T> {
  pub fn new(_index: i64, _aggregate_id: &str, _payload: T) -> Self where Self:Sized {
    Event {
      id: bson::oid::ObjectId::new().unwrap(),
      index: _index,
      aggregate_id: _aggregate_id.to_string(),
      timestamp:  Utc::now(),
      payload: _payload
    }
  }

  pub fn index(&self) -> i64 {
    return self.index;
  }

  pub fn aggregate_id(&self) -> &str {
    return self.aggregate_id.borrow();
  }

  pub fn timestamp(&self) -> &DateTime<Utc> {
    return self.timestamp.borrow();
  }

  pub fn payload(&self) -> &T {
    return self.payload.borrow();
  }
}
