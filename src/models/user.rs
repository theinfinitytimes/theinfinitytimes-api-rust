use wither::Model;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use chrono::prelude::{DateTime, Utc};
use url_serde::SerdeUrl;
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::theinfinitytimes_lib::{PreSaveMut};
use std::ops::Deref;

/// This is the user model, which is the base object
/// used to store user data and is used to authenticate
/// users and give the necessary permission to access
/// some front-end functionality based.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="User")]
pub struct UserModel {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub givenName: String,
    pub familyName: String,
    pub age: u32,
    pub gender: String,
    #[model(index(index="asc", unique="true"))]
    /// The nickname of the users can also be
    /// used to sign in and should be unique
    pub nickname: String,
    #[model(index(index="asc", unique="true"))]
    pub email: String,
    /// Only the hash of the password is saved
    /// and is saved in a string. We are using
    /// bcrypt to generate a hash and then during
    /// login comparing the hash of the password
    /// with the saved password
    pub userPassword: String,
    pub verifiedEmail: bool,
    pub memberSince: DateTime<Utc>,
    /// We are storing the profile pictures in a
    /// different server so this is the url pointing
    /// to that location
    pub profilePicture: SerdeUrl
}

impl PreSaveMut for UserModel{
    fn pre_save(&mut self){
        let user = self;
        // As in the trait, I am passing a mutable reference of the object
        // there is not need to check if `user` exists and isn't null
        // or if it is a null pointer. This would raise an error in Rust
        match hash(&user.userPassword, DEFAULT_COST){
            Ok(v) => user.userPassword = v,
            Err(e) => panic!(e)
        }
    }
}
