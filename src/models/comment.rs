use wither::Model;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use chrono::prelude::{DateTime, Utc};
use std::option::Option;
use crate::models::user::UserModel;
use crate::models::post::PostModel;

/// This is the comment model.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="Comment")]
pub struct CommentModel {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename="id")]
    pub iid: u64,
    pub body: String,
    /// This is a reference to the UserModel
    #[model(index(embedded="targetField"))]
    pub user: UserModel,
    #[model(index(embedded="targetField"))]
    pub post: PostModel,
    pub dateCreated: DateTime<Utc>,
    /// Optional field
    pub lastModified: Option<DateTime<Utc>>,
    /// Optional field
    #[model(index(embedded="targetField"))]
    pub modifiedBy: Option<UserModel>
}
