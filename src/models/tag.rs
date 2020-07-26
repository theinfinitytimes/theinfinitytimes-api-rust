use wither::Model;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
/// This is the tag model.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="Tag")]
pub struct TagModel {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub _id: u64,
    #[serde(rename="id")]
    pub iid: u64,
    pub name: String
}
