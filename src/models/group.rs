use wither::Model;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::models::account::AccountModel;

/// This is the group model.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="Group")]
pub struct GroupModel {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename="id")]
    pub identification: u64,
    pub name: String,
    /// This is a reference to the AccountModel
    #[model(index(embedded="targetField"))]
    pub members: Vec<AccountModel>
}
