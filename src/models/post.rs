extern crate chrono;
use chrono::{prelude::* , DateTime, Utc, Local};
extern crate url;
use url::{Url, ParseError};
extern crate mongodb;
use mongodb::{ObjectId};
use std::option::Option;

/// This is the post model.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="Post")]
pub struct PostModel {
    pub _id: Option<ObjectId>,
    pub title: String,
    #[model(index(index="asc", unique="true"))]
    pub id: u64,
    pub body: String,
    pub summary: Option<String>,
    /// This is a reference to the UserModel
    #[model(index(embedded="targetField"))]
    pub author: AuthorModel,
    pub dateCreated: DateTime,
    pub lastModified: Option<DateTime>,
    pub modifiedBy: Option<Author>,
    pub picture: Option<Url>,
    pub tags: Vec<u64>,
    #[model(index(embedded="targetField"))]
    pub comments: Option<Vec<CommentModel>>
}
