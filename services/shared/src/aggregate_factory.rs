use Aggregate;
use rocket::request::FromRequest;
use rocket::Request;
use rocket::Outcome;
use rocket::http::Status;
use rocket::State;
use EventStore;
use std::marker::PhantomData;

pub struct AggregateFactory<T: Aggregate, R: EventStore<T::Item> + Send + Sync + 'static> {
  aggregate: T,
  store_type: PhantomData<R>
}

impl<T: Aggregate, R: EventStore<T::Item> + Send + Sync + 'static> AggregateFactory<T, R> {
  pub fn aggregate(&self) -> &T {
    &self.aggregate
  }
}

impl<'a, 'r, T: Aggregate, R: EventStore<T::Item> + Send + Sync + 'static> FromRequest<'a, 'r> for AggregateFactory<T, R> {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, <Self as FromRequest<'a, 'r>>::Error), ()> {
    let store = request.guard::<State<R>>().unwrap();
    let mut events = store.get_events("").unwrap();
    let aggregate = T::new_from_events(&mut events);

    let fac = AggregateFactory {
      aggregate,
      store_type: PhantomData
    };
    Outcome::Success(fac)
  }
}
