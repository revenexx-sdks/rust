use serde::{Deserialize, Serialize};

/// Session
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    /// Session creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Session ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Session update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Client code name. View list of [available
    /// options](https://github.com/appwrite/appwrite/blob/master/docs/lists/clients.json).
    #[serde(rename = "clientCode", default)]
    pub client_code: String,
    /// Client engine name.
    #[serde(rename = "clientEngine", default)]
    pub client_engine: String,
    /// Client engine name.
    #[serde(rename = "clientEngineVersion", default)]
    pub client_engine_version: String,
    /// Client name.
    #[serde(rename = "clientName", default)]
    pub client_name: String,
    /// Client type.
    #[serde(rename = "clientType", default)]
    pub client_type: String,
    /// Client version.
    #[serde(rename = "clientVersion", default)]
    pub client_version: String,
    /// Country two-character ISO 3166-1 alpha code.
    #[serde(rename = "countryCode", default)]
    pub country_code: String,
    /// Country name.
    #[serde(rename = "countryName", default)]
    pub country_name: String,
    /// Returns true if this the current user session.
    #[serde(rename = "current", default)]
    pub current: bool,
    /// Device brand name.
    #[serde(rename = "deviceBrand", default)]
    pub device_brand: String,
    /// Device model name.
    #[serde(rename = "deviceModel", default)]
    pub device_model: String,
    /// Device name.
    #[serde(rename = "deviceName", default)]
    pub device_name: String,
    /// Session expiration date in ISO 8601 format.
    #[serde(rename = "expire", default)]
    pub expire: String,
    /// Returns a list of active session factors.
    #[serde(rename = "factors", default)]
    pub factors: Vec<String>,
    /// IP in use when the session was created.
    #[serde(rename = "ip", default)]
    pub ip: String,
    /// Most recent date in ISO 8601 format when the session successfully passed
    /// MFA challenge.
    #[serde(rename = "mfaUpdatedAt", default)]
    pub mfa_updated_at: String,
    /// Operating system code name. View list of [available
    /// options](https://github.com/appwrite/appwrite/blob/master/docs/lists/os.json).
    #[serde(rename = "osCode", default)]
    pub os_code: String,
    /// Operating system name.
    #[serde(rename = "osName", default)]
    pub os_name: String,
    /// Operating system version.
    #[serde(rename = "osVersion", default)]
    pub os_version: String,
    /// Session Provider.
    #[serde(rename = "provider", default)]
    pub provider: String,
    /// Session Provider Access Token.
    #[serde(rename = "providerAccessToken", default)]
    pub provider_access_token: String,
    /// The date of when the access token expires in ISO 8601 format.
    #[serde(rename = "providerAccessTokenExpiry", default)]
    pub provider_access_token_expiry: String,
    /// Session Provider Refresh Token.
    #[serde(rename = "providerRefreshToken", default)]
    pub provider_refresh_token: String,
    /// Session Provider User ID.
    #[serde(rename = "providerUid", default)]
    pub provider_uid: String,
    /// Secret used to authenticate the user. Only included if the request was made
    /// with an API key
    #[serde(rename = "secret", default)]
    pub secret: String,
    /// User ID.
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
