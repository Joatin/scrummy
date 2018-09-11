use Event;
use serde::Serialize;
use serde::Deserialize;
use error::Error;

pub trait Aggregate {
  type Item: Serialize + for<'a> Deserialize<'a> + Sized;

  fn id(&self) -> &str;
  fn version(&self) -> i64;
  fn new() -> Self where Self:Sized;
  fn apply(self, evt: &Event<Self::Item>) -> Self where Self:Sized;

  fn new_from_events(events: &mut Iterator<Item = Result<Event<Self::Item>, Error>>) -> Self where Self:Sized {
    let init = Self::new();
    let agg = events.fold(init, | acc, ref x| {
      match x {
        Ok(ref t) => {
          acc.apply(t)
        }
        _ => {
          acc
        }
      }
    });
    agg
  }
}
