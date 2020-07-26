use wither::Model;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use chrono::prelude::{DateTime, Utc};
use url_serde::SerdeUrl;
use std::option::Option;
use crate::models::author::AuthorModel;
use crate::models::comment::CommentModel;

/// This is the post model.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="Post")]
pub struct PostModel {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    #[serde(rename="_id")]
    #[model(index(index="asc", unique="true"))]
    pub iid: u64,
    pub body: String,
    pub summary: Option<String>,
    /// This is a reference to the UserModel
    #[model(index(embedded="targetField"))]
    pub author: AuthorModel,
    pub dateCreated: DateTime<Utc>,
    pub lastModified: Option<DateTime<Utc>>,
    pub modifiedBy: Option<AuthorModel>,
    pub picture: Option<SerdeUrl>,
    pub tags: Vec<u64>,
    #[model(index(embedded="targetField"))]
    pub comments: Option<Vec<CommentModel>>
}
