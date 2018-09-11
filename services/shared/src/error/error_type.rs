
#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorType {
  AggregateExists,
  DuplicatedEvent,
  Unknown
}
