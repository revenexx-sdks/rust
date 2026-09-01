use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelDefaults {
    /// Channel codes created by this call.
    #[serde(rename = "created", default)]
    pub created: Vec<String>,
    /// Default channel codes that already existed.
    #[serde(rename = "existing", default)]
    pub existing: Vec<String>,
    /// The same answer for the channel types, which are seeded first because the
    /// seeded channel carries one.
    #[serde(rename = "types", default)]
    pub types: crate::models::ChannelTypeDefaults,
}
