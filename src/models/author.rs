extern crate mongodb;
use mongodb::{ObjectId};

/// This is the author model.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="Author")]
pub struct AuthorModel {
    pub _id: Option<ObjectId>,
    /// This is a reference to the Account model
    #[model(index(unique="true", embedded="targetField"))]
    pub account: AccountModel,
    pub description: String,
    /// This is an array of posts. only references
    #[model(index(embedded="targetField"))]
    pub posts: Vec<PostModel>
}
