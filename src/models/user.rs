use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    #[serde(rename = "expires_at")]
    pub expires_at: i64,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub aud: String,
    pub role: String,
    pub email: String,
    #[serde(rename = "email_confirmed_at")]
    pub email_confirmed_at: String,
    pub phone: String,
    #[serde(rename = "last_sign_in_at")]
    pub last_sign_in_at: String,
    #[serde(rename = "app_metadata")]
    pub app_metadata: AppMetadata,
    #[serde(rename = "user_metadata")]
    pub user_metadata: UserMetadata,
    pub identities: Vec<Identity>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "is_anonymous")]
    pub is_anonymous: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMetadata {
    pub provider: String,
    pub providers: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMetadata {
    pub email: String,
    #[serde(rename = "email_verified")]
    pub email_verified: bool,
    #[serde(rename = "phone_verified")]
    pub phone_verified: bool,
    pub sub: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    #[serde(rename = "identity_id")]
    pub identity_id: String,
    pub id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "identity_data")]
    pub identity_data: IdentityData,
    pub provider: String,
    #[serde(rename = "last_sign_in_at")]
    pub last_sign_in_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityData {
    pub email: String,
    #[serde(rename = "email_verified")]
    pub email_verified: bool,
    #[serde(rename = "phone_verified")]
    pub phone_verified: bool,
    pub sub: String,
}
