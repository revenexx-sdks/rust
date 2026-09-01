use serde::{Deserialize, Serialize};

/// The market that was read from, resolved — so a caller who passed a code
/// back gets the uuid, and one who passed a uuid gets the code the rest of the
/// platform stores.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketRef {
    /// The source market's code — the value other apps scope by.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The source market's primary key.
    #[serde(rename = "id", default)]
    pub id: String,
}
