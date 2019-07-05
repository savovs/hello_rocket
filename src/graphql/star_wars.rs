use juniper::FieldResult;
use std::fmt;
use std::time::SystemTime;

#[derive()]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub published: bool,
  pub publish_at: Option<SystemTime>,
  pub views: i32,
}

impl fmt::Debug for Post {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "Post {{ title: {}}}", self.title)
  }
}
