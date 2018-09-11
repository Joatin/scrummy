#![feature(associated_type_defaults)]

extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate mongodb;
#[macro_use]
extern crate bson;
extern crate rocket;


mod aggregate;
mod event;
mod event_store;
mod mongo_event_store;
mod api_key;
mod aggregate_factory;
mod error;

pub use aggregate::Aggregate;
pub use event::Event;
pub use mongo_event_store::MongoEventStore;
pub use event_store::EventStore;
pub use api_key::ApiKey;
pub use aggregate_factory::AggregateFactory;
pub use error::Error;
pub use error::ErrorType;
