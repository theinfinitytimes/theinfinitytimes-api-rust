extern crate chrono;
use chrono::{DateTime, Utc};
extern crate url;
use url::{Url, ParseError};
extern crate mongodb;
use mongodb::{ObjectId};
extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};
use theinfinitytimes_lib::*;

/// This is the user model, which is the base object
/// used to store user data and is used to authenticate
/// users and give the necessary permission to access
/// some front-end functionality based.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="User")]
pub struct UserModel {
    pub _id: Option<ObjectId>,
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
    pub memberSince: DateTime,
    /// We are storing the profile pictures in a
    /// different server so this is the url pointing
    /// to that location
    pub profilePicture: Url
}

impl PreSave for UserModel{
    fn pre_save(&self){
        let user = self;
        if user && user.userPassword && user.userPassword != null {
            return
        }
        user.userPassword = bcrypt.hash(user.userPassword, DEFAULT_COST)?;
    }
}
