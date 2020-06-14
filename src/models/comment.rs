extern crate chrono;
use chrono::{prelude::* , DateTime, Utc, Local};
extern crate mongodb;
use mongodb::{ObjectId};
use std::option::Option;

/// This is the comment model.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="Comment")]
pub struct CommentModel {
    pub _id: Option<ObjectId>,
    pub id: u64,
    pub body: String,
    /// This is a reference to the UserModel
    #[model(index(embedded="targetField"))]
    pub user: UserModel,
    #[model(index(embedded="targetField"))]
    pub post: PostModel,
    pub dateCreated: DateTime,
    /// Optional field
    pub lastModified: Option<DateTime>,
    /// Optional field
    #[model(index(embedded="targetField"))]
    pub modifiedBy: Option<UserModel>
}
