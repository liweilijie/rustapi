use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::Model as WitherModel;
use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use bson::serde_helpers::serialize_object_id_as_hex_string;

use crate::database::Database;
use crate::models::ModelExt;
use crate::lib::date;
use crate::lib::date::Date;

#[derive(Clone)]
pub struct Model {
  pub db: Database,
}

impl Model {
  pub fn new(db: Database) -> Self {
    Self { db }
  }
}

impl ModelExt for Model {
  type T = Cat;
  fn get_database(&self) -> &Database {
    &self.db
  }
}

#[derive(WitherModel, Debug, Clone, Serialize, Deserialize)]
#[model(index(keys = r#"doc!{ "user": 1 }"#))]
pub struct Cat {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub user: ObjectId,
  pub name: String,
  pub updated_at: Date,
  pub created_at: Date,
}

impl Cat {
    pub fn new (name: String) -> Self {
      let now = date::now();
      Self {
        id: None,
        user: ObjectId::new(), // Temp
        name,
        updated_at: now,
        created_at: now,
      }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicCat {
  #[serde(alias = "_id", serialize_with = "serialize_object_id_as_hex_string")]
  pub id: ObjectId,
  #[serde(serialize_with = "serialize_object_id_as_hex_string")]
  pub user: ObjectId,
  pub name: String,
  #[serde(with = "bson_datetime_as_rfc3339_string")]
  pub updated_at: Date,
  #[serde(with = "bson_datetime_as_rfc3339_string")]
  pub created_at: Date,
}

impl From<Cat> for PublicCat {
  fn from(cat: Cat) -> Self {
    Self {
      id: cat.id.unwrap(),
      user: cat.user,
      name: cat.name.clone(),
      updated_at: cat.updated_at,
      created_at: cat.created_at,
    }
  }
}