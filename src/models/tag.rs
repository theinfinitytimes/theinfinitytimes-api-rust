
/// This is the tag model.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="Tag")]
pub struct TagModel {
    pub _id: u64,
    pub id: u64,
    pub name: String
}
