use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelDefaults {
    /// Channel codes created by this call.
    #[serde(rename = "created", default)]
    pub created: Vec<String>,
    /// Default channel codes that already existed.
    #[serde(rename = "existing", default)]
    pub existing: Vec<String>,
}
