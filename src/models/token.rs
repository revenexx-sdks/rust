use serde::{Deserialize, Serialize};

/// Token
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Token {
    /// Token creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Token ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Token expiration date in ISO 8601 format.
    #[serde(rename = "expire", default)]
    pub expire: String,
    /// Security phrase of a token. Empty if security phrase was not requested when
    /// creating a token. It includes randomly generated phrase which is also sent
    /// in the external resource such as email.
    #[serde(rename = "phrase", default)]
    pub phrase: String,
    /// Token secret key. This will return an empty string unless the response is
    /// returned using an API key or as part of a webhook payload.
    #[serde(rename = "secret", default)]
    pub secret: String,
    /// User ID.
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
