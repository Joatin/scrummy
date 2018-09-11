use Event;
use serde::Serialize;
use serde::Deserialize;
use error::Error;

pub trait EventStore<T: Serialize + for<'a> Deserialize<'a> + Sized> {
  fn store(&self, _events: Vec<Event<T>>) -> Result<(), Error>;
  fn get_events(&self, _aggregate_id: &str) -> Result<Box<Iterator<Item = Result<Event<T>, Error>>>, Error>;
}
