use serde::{Deserialize, Serialize};

/// JWT
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Jwt {
    /// JWT encoded string.
    #[serde(rename = "jwt", default)]
    pub jwt: String,
}
