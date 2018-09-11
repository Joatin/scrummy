

#[derive(Serialize, Deserialize, Debug)]
pub struct OrganizationCreateEventData {
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrganizationChangeNameEventData {
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum OrganizationEvent {
  Create(OrganizationCreateEventData),
  ChangeName(OrganizationChangeNameEventData)
}
