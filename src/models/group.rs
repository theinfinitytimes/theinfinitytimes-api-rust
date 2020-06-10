extern crate mongodb;
use mongodb::{ObjectId};

/// This is the group model.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="Group")]
pub struct GroupModel {
    pub _id: Option<ObjectId>,
    pub id: u64,
    pub name: String,
    /// This is a reference to the AccountModel
    #[model(index(embedded="targetField"))]
    pub members: Vec<AccountModel>
}