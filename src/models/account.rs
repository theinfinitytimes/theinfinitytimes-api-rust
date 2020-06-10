extern crate chrono;
use chrono::{prelude::* , DateTime, Utc, Local};
extern crate url;
use url::{Url, ParseError};
extern crate mongodb;
use mongodb::{ObjectId};
use UserModel;

/// This is the account model. Ths mo
#[derive(Model, Serialize, Deserialize)]
#[model(collection_name="Account")]
pub struct AccountModel {
    pub _id: Option<ObjectId>,
    /// This is a reference to the UserModel
    #[model(index(embedded="targetField"))]
    pub user: UserModel,
    pub isEnabled: bool,
    #[model(index(index="asc", unique="true"))]
    pub userID: u64,
    pub accountType: String,
    pub lastLogon: DateTime,
    pub logonCount: u64
}

impl PreSave for AccountModel{
    fn pre_save(&self) {
        let account = self;
        if account && account.accountType {
            *account.isEnabled = *account.user.verifiedEmail;
            *account.accountType = "admin";
            *account.lastLogon = Local::now();
            *account.logonCount = 0;
        }
    }
}
