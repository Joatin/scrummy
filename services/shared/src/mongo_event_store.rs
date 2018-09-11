

use event_store::EventStore;
use Event;
use bson::to_bson;
use mongodb::Client;
use mongodb::db::Database;
use mongodb::coll::Collection;
use mongodb::ThreadedClient;
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::IndexOptions;
use serde::Serialize;
use bson::from_bson;
use bson::Bson;
use serde::Deserialize;
use std::marker::PhantomData;
use error::Error;


pub struct MongoEventStore<T: Serialize + for<'a> Deserialize<'a> + Sized> {
  client: Client,
  database: Database,
  collection: Collection,
  resource_type: PhantomData<T>,
}

impl<T: Serialize + for<'a> Deserialize<'a> + Sized> MongoEventStore<T> {

  pub fn new(_uri: &str) -> MongoEventStore<T> {
    let client = Client::with_uri(_uri).unwrap();
    let db = client.db("scrummy-organization");
    let coll = db.collection("event-store");
    coll.create_index(doc!{ "aggregateId": 1, "index": 1 }, Some(IndexOptions { unique: Some(true), ..Default::default() })).unwrap();

    return MongoEventStore {
      client,
      database: db,
      collection: coll,
      resource_type: PhantomData
    }
  }
}

impl<T: Serialize + for<'a> Deserialize<'a> + Sized> EventStore<T> for MongoEventStore<T> {

  fn store(&self, _events: Vec<Event<T>>) -> Result<(), Error> {
    let _mapped = _events.iter().map(|ref x| to_bson::<Event<T>>(&x).unwrap());

    let _docs = _mapped.map(|ref x | x.as_document().unwrap().clone()).collect::<Vec<_>>();

    if _docs.len() > 1 {
      println!("no elements");
    }
    match self.collection.insert_many(_docs, None) {
      Ok(o) => {
        match o.bulk_write_exception {
          Some(t) => {
            // Err(t.message.to_string())
            Err(Error::unknown())
          }
          None => {
            Ok(())
          }
        }
      }
      Err(e) => {
        //Err(e.description().to_string())
        Err(Error::unknown())
      }
    }
  }

  fn get_events(&self, _aggregate_id: &str) -> Result<Box<Iterator<Item = Result<Event<T>, Error>>>, Error> {
    let collect = match self.collection.find(Some(doc!{"aggregateId": _aggregate_id}), None) {
      Ok(o) => {
        let mapped = o.map(|  ref x | match x {
          Ok(t) => {
            let doc = from_bson::<Event<T>>(Bson::Document(t.clone()));
            match doc {
              Ok(d) => {
                Ok(d)
              }
              Err(e) => {
                Err(Error::unknown())
              }
            }
          }
          Err(e) => {
            Err(Error::unknown())
          }
        }).into_iter();
        Ok(Box::new(mapped) as Box<Iterator<Item = Result<Event<T>, Error>>>)
      }
      Err(e) => {
        //Err(e.description().to_string())
        Err(Error::unknown())
      }
    };
    collect
  }
}
