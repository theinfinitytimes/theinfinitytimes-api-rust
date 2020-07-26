use wither::Model;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::models::account::AccountModel;
use crate::models::post::PostModel;

/// This is the author model.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="Author")]
pub struct AuthorModel {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    /// This is a reference to the Account model
    #[model(index(unique="true", embedded="targetField"))]
    pub account: AccountModel,
    pub description: String,
    /// This is an array of posts. only references
    #[model(index(embedded="targetField"))]
    pub posts: Vec<PostModel>
}
