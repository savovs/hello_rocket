use crate::db::schema::posts;

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fmt;

#[derive(Queryable, Identifiable, Serialize)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub published: bool,
  pub publish_at: DateTime<Utc>,
  pub views: i32,
}

impl fmt::Debug for Post {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "Post {{ title: {}}}", self.title)
  }
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
  pub title: &'a str,
  pub body: &'a str,
}
