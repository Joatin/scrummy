extern crate shared;

use organization_event::OrganizationEvent;
use organization_event::OrganizationCreateEventData;
use self::shared::Event;
use organization_event::OrganizationEvent::Create;
use organization_event::OrganizationEvent::ChangeName;
use shared::Aggregate;
use shared::Error;


#[derive(Debug)]
pub struct OrganizationAggregate {
  id: String,
  version: i64,
  name: String,
  description: String,
}

impl OrganizationAggregate {

  pub fn create(&self, _name: &str) -> Result<Vec<Event<OrganizationEvent>>, Error> {
    if self.version != -1 {
      Ok(vec![
        Event::new(
          0,
          _name,
          OrganizationEvent::Create(
            OrganizationCreateEventData {
              name: _name.to_string(),
            }
          )
        )
      ])
    } else {
      Err(Error::aggregate_already_exists())
    }
  }

  pub fn change_name(&self, _name: &str) -> Result<Vec<Event<OrganizationEvent>>, String> {
    Ok(vec![
      Event::new(
        self.version + 1,
        _name,
        OrganizationEvent::Create(
          OrganizationCreateEventData {
            name: _name.to_string(),
          }
        )
      )
    ])
  }
}

impl Aggregate for OrganizationAggregate {
  type Item = OrganizationEvent;

  fn id(&self) -> &str {
    &self.id
  }

  fn version(&self) -> i64 {
    self.version
  }

  // Creates a new aggregate
  fn new() -> OrganizationAggregate {
    OrganizationAggregate {
      version: -1,
      id: "".to_string(),
      name: "".to_string(),
      description: "".to_string()
    }
  }

  fn apply(mut self, _event: &Event<OrganizationEvent>) -> OrganizationAggregate {
    self.version = _event.index();
    self.id = _event.aggregate_id().to_string();

    match _event.payload() {
      Create(d) => {
        self.name = d.name.to_string()
      }
      ChangeName(d) => {
        self.name = d.name.to_string()
      }
    }

    self
  }
}
