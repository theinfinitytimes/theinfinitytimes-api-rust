use wither::Model;
use serde::{Serialize, Deserialize};
use chrono::prelude::{DateTime, Utc, Local};
use mongodb::bson::oid::ObjectId;
use crate::models::user::UserModel;
use crate::theinfinitytimes_lib::{PreSaveMut};

/// This is the account model.
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="Account")]
pub struct AccountModel {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    /// This is a reference to the UserModel
    #[model(index(embedded="targetField"))]
    pub user: UserModel,
    pub isEnabled: bool,
    #[model(index(index="asc", unique="true"))]
    pub userID: u64,
    pub accountType: String,
    pub lastLogon: DateTime<Utc>,
    pub logonCount: u64
}

impl PreSaveMut for AccountModel{
    fn pre_save(&mut self) {
        let account = self;
        if !account.accountType.is_empty() {
            account.isEnabled = account.user.verifiedEmail;
            account.accountType == "admin";
            account.lastLogon = DateTime::from(Local::now());
            account.logonCount = 0;
        }
    }
}
