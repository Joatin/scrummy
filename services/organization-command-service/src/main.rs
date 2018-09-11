#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate bson;
extern crate mongodb;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate shared;
#[macro_use]
extern crate log;
extern crate simplelog;


use rocket_contrib::Json;
use shared::MongoEventStore;
use rocket::State;
use shared::EventStore;
use organization_event::OrganizationEvent;
use shared::Aggregate;
use shared::ApiKey;
use shared::AggregateFactory;
use simplelog::LevelFilter;
use simplelog::TermLogger;
use simplelog::Config;
use shared::Error;

mod organization_aggregate;
mod organization_event;
mod create_organization_payload;

type Store<'a> = State<'a, MongoEventStore<OrganizationEvent>>;
type Factory = AggregateFactory<organization_aggregate::OrganizationAggregate, MongoEventStore<OrganizationEvent>>;

#[get("/")]
fn index(_key: ApiKey) -> &'static str {
  "Hello, world!"
}

#[post("/new", format = "application/json", data = "<payload>")]
fn create_new_organization(
  payload: Json<create_organization_payload::CreateOrganizationPayload>,
  _store: Store,
  _factory: Factory,
) -> Result<String, Error> {
  let events = _factory.aggregate().create(&payload.name)?;
  _store.store(events)?;
  Ok(json!({
            "result": "ok"
          }).to_string())
}

fn main() {
  TermLogger::init(LevelFilter::Info, Config::default()).unwrap();
  let _store = MongoEventStore::<OrganizationEvent>::new("mongodb://localhost:27017");

  rocket::ignite()
    .manage(_store)
    .mount("/", routes![index, create_new_organization])
    .launch();
}
